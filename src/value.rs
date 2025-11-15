use crate::type_info::XuguTypeInfo;
use bytes::Bytes;
use rbdc::Error;
use std::str::from_utf8;

#[derive(Clone)]
pub struct XuguValue {
    pub(crate) value: Option<Bytes>,
    pub(crate) type_info: XuguTypeInfo,
}

impl XuguValue {
    pub(crate) fn as_bytes(&self) -> Result<&[u8], Error> {
        match &self.value {
            Some(v) => Ok(v),
            None => Err(Error::protocol("UnexpectedNull")),
        }
    }

    pub(crate) fn as_str(&self) -> Result<&str, Error> {
        Ok(from_utf8(self.as_bytes()?)?)
    }
}
