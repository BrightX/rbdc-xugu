use crate::connection::XuguConnection;
use crate::options::XuguConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection};
use rbdc::Error;
use std::str::FromStr;

impl ConnectOptions for XuguConnectOptions {
    fn connect(&self) -> BoxFuture<'_, Result<Box<dyn Connection>, Error>> {
        Box::pin(async {
            let conn = XuguConnection::establish(self).await?;

            let r: Box<dyn Connection> = Box::new(conn);
            Ok(r)
        })
    }

    fn set_uri(&mut self, uri: &str) -> Result<(), Error> {
        *self = XuguConnectOptions::from_str(uri).map_err(|e| Error::from(e.to_string()))?;
        Ok(())
    }
}
