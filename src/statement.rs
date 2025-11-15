use crate::column::XuguColumn;
use crate::protocol::statement::ParameterDef;
use crate::type_info::XuguTypeInfo;
use either::Either;
use rbdc::ext::ustr::UStr;
use rbdc::Error;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct XuguStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) metadata: XuguStatementMetadata,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct XuguStatementMetadata {
    pub(crate) columns: Arc<Vec<XuguColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
    pub(crate) parameters: Arc<Vec<ParameterDef>>,
}

#[allow(dead_code)]
impl XuguStatement<'_> {
    fn to_owned(&self) -> XuguStatement<'static> {
        XuguStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            metadata: self.metadata.clone(),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    /// 获取此语句的预期参数。
    ///
    /// 返回的信息取决于驱动程序提供的信息。SQLite 只能告诉我们参数的数量。PostgreSQL 可以为我们提供完整的类型信息。
    fn parameters(&self) -> Option<Either<&[XuguTypeInfo], usize>> {
        Some(Either::Right(self.metadata.parameters.len()))
    }

    fn columns(&self) -> &[XuguColumn] {
        &self.metadata.columns
    }
}

#[allow(dead_code)]
impl XuguStatement<'_> {
    fn index(&self, index: &str) -> Result<usize, Error> {
        self.metadata
            .column_names
            .get(index)
            .or_else(|| {
                // 列名忽略大小写时，列名大写检索
                self.metadata.column_names.get(&*index.to_uppercase())
            })
            .ok_or_else(|| Error::from(format!("ColumnNotFound {}", index)))
            .copied()
    }
}
