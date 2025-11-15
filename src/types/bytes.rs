use crate::arguments::XuguArgumentValue;
use crate::protocol::text::ColumnType;
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use bytes::Bytes;
use rbdc::Error;
use std::borrow::Cow;

impl TypeInfo for Vec<u8> {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::BLOB)
    }
}

impl Encode for Vec<u8> {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        args.push(XuguArgumentValue::Bin(Cow::Owned(self)));

        Ok(IsNull::No)
    }
}

impl Decode for Vec<u8> {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        value.as_bytes().map(ToOwned::to_owned)
    }
}

impl TypeInfo for Bytes {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::BLOB)
    }
}

impl Encode for Bytes {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        args.push(XuguArgumentValue::Bytes(self));

        Ok(IsNull::No)
    }
}

impl Decode for Bytes {
    fn decode(v: XuguValue) -> Result<Self, Error> {
        match v.value {
            Some(v) => Ok(v),
            None => Ok(Bytes::new()),
        }
    }
}
