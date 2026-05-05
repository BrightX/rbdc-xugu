use crate::arguments::XuguArgumentValue;
use crate::protocol::text::ColumnType;
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;
use std::borrow::Cow;

impl TypeInfo for i8 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::TINYINT)
    }
}

impl TypeInfo for i16 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::SMALLINT)
    }
}

impl TypeInfo for i32 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::INTEGER)
    }
}

impl TypeInfo for i64 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::BIGINT)
    }
}

impl Encode for i8 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

impl Encode for i16 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

impl Encode for i32 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

impl Encode for i64 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

fn int_decode(value: XuguValue) -> Result<i64, Error> {
    if value.type_info.r#type == ColumnType::CHAR {
        let s = value.as_str()?;
        return Ok(s.parse::<i64>()?);
    }

    let buf = value.as_bytes()?;

    // Check conditions that could cause `read_int()` to panic.
    if buf.is_empty() {
        return Err("empty buffer".into());
    }

    if buf.len() > 8 {
        return Err(format!(
            "expected no more than 8 bytes for integer value, got {}",
            buf.len()
        )
        .into());
    }

    if value.type_info.r#type == ColumnType::BOOLEAN {
        return <bool as Decode>::decode(value).map(|x| x as i64);
    }

    Ok(BigEndian::read_int(buf, buf.len()))
}

impl Decode for i8 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        int_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode for i16 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        int_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode for i32 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        int_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode for i64 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        int_decode(value)
    }
}
