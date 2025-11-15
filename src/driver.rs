use crate::options::XuguConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection, Driver, Placeholder};
use rbdc::Error;

#[derive(Debug)]
pub struct XuguDriver;

impl Driver for XuguDriver {
    fn name(&self) -> &str {
        "xugu"
    }

    fn connect(&self, url: &str) -> BoxFuture<'_, Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let mut opt = self.default_option();
            opt.set_uri(&url)?;
            if let Some(opt) = opt.downcast_ref::<XuguConnectOptions>() {
                let conn = opt.connect().await?;
                Ok(Box::new(conn) as Box<dyn Connection>)
            } else {
                Err(Error::from("downcast_ref failure"))
            }
        })
    }

    fn connect_opt<'a>(
        &'a self,
        opt: &'a dyn ConnectOptions,
    ) -> BoxFuture<'a, Result<Box<dyn Connection>, Error>> {
        let opt: &XuguConnectOptions = opt.downcast_ref().unwrap();
        Box::pin(async move {
            let conn = opt.connect().await?;
            Ok(conn)
        })
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(XuguConnectOptions::default())
    }
}

impl Placeholder for XuguDriver {
    fn exchange(&self, sql: &str) -> String {
        sql.to_string()
    }
}
