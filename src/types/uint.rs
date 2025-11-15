use crate::arguments::XuguArgumentValue;
use crate::protocol::text::ColumnType;
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;
use std::borrow::Cow;

impl TypeInfo for u8 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::TINYINT)
    }
}

impl TypeInfo for u16 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::SMALLINT)
    }
}

impl TypeInfo for u32 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::INTEGER)
    }
}

impl TypeInfo for u64 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::BIGINT)
    }
}

impl Encode for u8 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

impl Encode for u16 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

impl Encode for u32 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

impl Encode for u64 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

fn uint_decode(value: XuguValue) -> Result<u64, Error> {
    if value.type_info.r#type == ColumnType::CHAR {
        let s = value.as_str()?;
        return Ok(u64::from_str_radix(s, 10)?);
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
        return <bool as Decode>::decode(value).map(|x| x as u64);
    }

    Ok(BigEndian::read_uint(buf, buf.len()))
}

impl Decode for u8 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        uint_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode for u16 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        uint_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode for u32 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        uint_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode for u64 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        uint_decode(value)
    }
}
