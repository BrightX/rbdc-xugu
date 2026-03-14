use super::{StatementId, XuguConnection};
use crate::arguments::XuguArguments;
use crate::io::AsyncStreamExt;
use crate::protocol::message::*;
use crate::protocol::statement::{Execute as StatementExecute, Prepare, StmtClose};
use crate::protocol::text::{OkPacket, Query};
use crate::protocol::ServerContext;
use crate::query_result::XuguQueryResult;
use crate::row::XuguRow;
use crate::statement::{XuguStatement, XuguStatementMetadata};
use crate::type_info::XuguTypeInfo;
use crate::XuguDatabaseError;
use either::Either;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_core::Stream;
use futures_util::TryStreamExt;
use log::Level;
use rbdc::error::Error;
use rbdc::try_stream;
use rbs::Value;
use std::collections::HashMap;
use std::{borrow::Cow, pin::pin, sync::Arc};

impl XuguConnection {
    async fn prepare_statement<'c>(
        &mut self,
        sql: &str,
    ) -> Result<(StatementId, XuguStatementMetadata), Error> {
        // flush and wait until we are re-ready
        self.wait_until_ready().await?;

        let id = self.inner.gen_st_id();
        self.inner
            .stream
            .send_packet(Prepare {
                query: sql,
                st_id: id,
            })
            .await?;

        let mut error = None;
        let mut columns = Vec::new();
        let mut column_names = HashMap::new();
        let mut params = Vec::new();

        loop {
            let message: ReceivedMessage = self.inner.stream.recv().await?;
            let cnt = ServerContext::new(self.inner.stream.server_version);
            match message.format {
                BackendMessageFormat::ErrorResponse => {
                    let err: ErrorResponse = message.decode(&mut self.inner.stream, cnt).await?;
                    error = Some(err.error);
                }
                BackendMessageFormat::MessageResponse => {
                    // 读到服务器端返回消息用对话框抛出
                    // 警告和信息
                    let notice: MessageResponse =
                        message.decode(&mut self.inner.stream, cnt).await?;
                    let log_level = Level::Info;
                    let log_is_enabled = log::log_enabled!(
                        target: "xugu::notice",
                        log_level
                    );
                    if log_is_enabled {
                        log::logger().log(
                            &log::Record::builder()
                                .args(format_args!("{}", &notice.msg))
                                .level(log_level)
                                .module_path_static(Some("xugu::notice"))
                                .target("xugu::notice")
                                .file_static(Some(file!()))
                                .line(Some(line!()))
                                .build(),
                        );
                    }
                }
                BackendMessageFormat::ReadyForQuery => {
                    let _: ReadyForQuery = message.decode(&mut self.inner.stream, cnt).await?;
                    break;
                }
                BackendMessageFormat::RowDescription => {
                    let row_columns: RowDescription =
                        message.decode(&mut self.inner.stream, cnt).await?;
                    (columns, column_names) = row_columns.convert_columns()?;
                }
                BackendMessageFormat::ParameterDescription => {
                    let param_def: ParameterDescription =
                        message.decode(&mut self.inner.stream, cnt).await?;
                    params = param_def.params;
                }
                _ => {
                    break;
                }
            }
        }

        if let Some(err) = error {
            return Err(XuguDatabaseError::from_str(&err).into());
        }

        let metadata = XuguStatementMetadata {
            parameters: Arc::new(params),
            columns: Arc::new(columns),
            column_names: Arc::new(column_names),
        };

        Ok((id, metadata))
    }

    async fn get_or_prepare_statement<'c>(
        &mut self,
        sql: &str,
    ) -> Result<(StatementId, XuguStatementMetadata), Error> {
        if let Some(statement) = self.inner.cache_statement.get_mut(sql) {
            // <XuguStatementMetadata> is internally reference-counted
            return Ok((*statement).clone());
        }

        let (id, metadata) = self.prepare_statement(sql).await?;

        // in case of the cache being full, close the least recently used statement
        if let Some((id, _)) = self
            .inner
            .cache_statement
            .insert(sql, (id, metadata.clone()))
        {
            // flush and wait until we are re-ready
            self.wait_until_ready().await?;
            self.inner.stream.send_packet(StmtClose(id)).await?;
            // for StmtClose
            let _ok: OkPacket = self.inner.stream.recv().await?;
        }

        Ok((id, metadata))
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `sql`:
    /// * `arguments`:
    /// * `persistent`: sql 语句是否需要被缓存
    ///
    #[allow(clippy::needless_lifetimes)]
    pub(crate) async fn run<'e, 'q: 'e>(
        &'e mut self,
        sql: &'q str,
        arguments: Option<XuguArguments<'q>>,
        persistent: bool,
    ) -> Result<impl Stream<Item = Result<Either<XuguQueryResult, XuguRow>, Error>> + 'e, Error>
    {
        self.wait_until_ready().await?;

        // make a slot for the shared column data
        // as long as a reference to a row is not held past one iteration, this enables us
        // to re-use this memory freely between result sets
        let (mut column_names, mut columns, mut needs_metadata) = if let Some(arguments) = arguments
        {
            if persistent && self.inner.cache_statement.is_enabled() {
                let (id, metadata) = self.get_or_prepare_statement(sql).await?;

                self.inner
                    .stream
                    .send_packet(StatementExecute {
                        st_id: id,
                        arguments: &arguments,
                        params: &metadata.parameters,
                    })
                    .await?;

                let needs_metadata = metadata.column_names.is_empty();
                (metadata.column_names, metadata.columns, needs_metadata)
            } else {
                let (id, metadata) = self.prepare_statement(sql).await?;

                self.inner
                    .stream
                    .send_packet(StatementExecute {
                        st_id: id,
                        arguments: &arguments,
                        params: &metadata.parameters,
                    })
                    .await?;

                self.inner.stream.send_packet(StmtClose(id)).await?;
                // for StmtClose
                self.inner.pending_ready_for_query_count += 1;

                let needs_metadata = metadata.column_names.is_empty();
                (metadata.column_names, metadata.columns, needs_metadata)
            }
        } else {
            self.inner.stream.send_packet(Query(sql)).await?;

            (Arc::default(), Arc::default(), true)
        };

        self.inner.pending_ready_for_query_count += 1;

        let mut error = None;

        let mut num_columns = 0;

        Ok(try_stream! {
            loop {
                let message: ReceivedMessage = self.inner.stream.recv().await?;
                let cnt = ServerContext::new(self.inner.stream.server_version);
                match message.format {
                    BackendMessageFormat::ErrorResponse => {
                        let err: ErrorResponse = message.decode(&mut self.inner.stream, cnt).await?;
                        error = Some(err.error);
                    },
                    BackendMessageFormat::MessageResponse => {
                        // 读到服务器端返回消息用对话框抛出
                        // 警告和信息
                        let notice: MessageResponse =
                            message.decode(&mut self.inner.stream, cnt).await?;
                        let log_level = Level::Info;
                        let log_is_enabled = log::log_enabled!(
                            target: "xugu::notice",
                            log_level
                        );
                        if log_is_enabled {
                            log::logger().log(
                                &log::Record::builder()
                                    .args(format_args!("{}", &notice.msg))
                                    .level(log_level)
                                    .module_path_static(Some("xugu::notice"))
                                    .target("xugu::notice")
                                    .file_static(Some(file!()))
                                    .line(Some(line!()))
                                    .build(),
                            );
                        }
                    },
                    BackendMessageFormat::ReadyForQuery => {
                        //命令结束 / 错误结束
                        let _: ReadyForQuery = message.decode(&mut self.inner.stream, cnt).await?;
                        self.handle_ready_for_query().await?;
                        break;
                    },
                    BackendMessageFormat::InsertResponse => {
                        let res: InsertResponse = message.decode(&mut self.inner.stream, cnt).await?;
                        let rows_affected = 1;
                        let done = XuguQueryResult {
                            rows_affected,
                            last_insert_id: Some(res.rowid),
                        };
                        r#yield!(Either::Left(done));
                    },
                    BackendMessageFormat::DeleteResponse => {
                        let res: DeleteResponse = message.decode(&mut self.inner.stream, cnt).await?;
                        let rows_affected = res.rows_affected as u64;
                        let done = XuguQueryResult {
                            rows_affected,
                            last_insert_id: None,
                        };
                        r#yield!(Either::Left(done));
                    },
                    BackendMessageFormat::UpdateResponse => {
                        let res: UpdateResponse = message.decode(&mut self.inner.stream, cnt).await?;
                        let rows_affected = res.rows_affected as u64;
                        let done = XuguQueryResult {
                            rows_affected,
                            last_insert_id: None,
                        };
                        r#yield!(Either::Left(done));
                    },
                    BackendMessageFormat::RowDescription => {
                        // 接收列数据
                        let row_columns: RowDescription = message.decode(&mut self.inner.stream, cnt).await?;
                        num_columns = row_columns.fields.len();
                        self.inner.last_num_columns = num_columns;
                        if needs_metadata {
                            let (columns_c, column_names_c) = row_columns.convert_columns()?;
                            columns = Arc::new(columns_c);
                            column_names = Arc::new(column_names_c);
                        } else {
                            // next time we hit here, it'll be a new result set and we'll need the
                            // full metadata
                            needs_metadata = true;
                        }
                    },
                    BackendMessageFormat::ParameterDescription => {
                        let _: ParameterDescription = message.decode(&mut self.inner.stream, cnt).await?;
                    },
                    BackendMessageFormat::DataRow => {
                        // 接收行数据
                        let _: DataRow = message.decode(&mut self.inner.stream, cnt).await?;
                        let mut row = Vec::with_capacity(num_columns);
                        for _ in 0..num_columns {
                            let len = self.inner.stream.read_i32().await?;
                            let buf = self.inner.stream.read_bytes(len as usize).await?;
                            row.push(buf);
                        }
                        let row = Arc::new(row);

                        let v = Either::Right(XuguRow {
                            row,
                            columns: Arc::clone(&columns),
                            column_names: Arc::clone(&column_names),
                        });

                        r#yield!(v);
                    }
                }
            }

            if let Some(err) = error {
                return Err(XuguDatabaseError::from_str(&err).into());
            }

            return Ok(());
        })
    }
}

impl XuguConnection {
    /// 执行多个查询，并将生成的结果作为每个查询的流返回。
    pub(crate) fn fetch_many(
        &mut self,
        sql: &str,
        params: Vec<Value>,
        persistent: bool,
    ) -> BoxStream<'_, Result<Either<XuguQueryResult, XuguRow>, Error>> {
        let sql = sql.to_owned();
        Box::pin(try_stream! {
            let arguments = if params.is_empty() { None } else {
                Some(XuguArguments::from_args(params)?)
            };
            let mut s = pin!(self.run(&sql, arguments, persistent).await?);

            while let Some(v) = s.try_next().await? {
                r#yield!(v);
            }

            Ok(())
        })
    }

    /// 执行查询并最多返回一行。
    #[allow(dead_code)]
    fn fetch_optional(
        &mut self,
        sql: &str,
        params: Vec<Value>,
        persistent: bool,
    ) -> BoxFuture<'_, Result<Option<XuguRow>, Error>> {
        let mut s = self.fetch_many(sql, params, persistent);
        Box::pin(async move {
            while let Some(v) = s.try_next().await? {
                if let Either::Right(r) = v {
                    return Ok(Some(r));
                }
            }

            Ok(None)
        })
    }

    /// 准备 SQL 查询，其中包含参数类型信息，以检查有关其参数和结果的类型信息。
    ///
    /// 只有某些数据库驱动程序（PostgreSQL、MSSQL）可以利用此额外信息来影响参数类型推断。
    pub(crate) fn prepare_with<'e, 'q: 'e>(
        &'e mut self,
        sql: &'q str,
        _parameters: &'e [XuguTypeInfo],
    ) -> BoxFuture<'e, Result<XuguStatement<'q>, Error>> {
        Box::pin(async move {
            self.wait_until_ready().await?;

            let metadata = if self.inner.cache_statement.is_enabled() {
                self.get_or_prepare_statement(sql).await?.1
            } else {
                let (id, metadata) = self.prepare_statement(sql).await?;

                self.inner.stream.send_packet(StmtClose(id)).await?;

                // for StmtClose
                let _ok: OkPacket = self.inner.stream.recv().await?;

                metadata
            };

            Ok(XuguStatement {
                sql: Cow::Borrowed(sql),
                // metadata has internal Arcs for expensive data structures
                metadata: metadata.clone(),
            })
        })
    }
}
