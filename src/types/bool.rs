use crate::arguments::XuguArgumentValue;
use crate::protocol::text::{ColumnFlags, ColumnType};
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use rbdc::Error;
use std::borrow::Cow;

impl TypeInfo for bool {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo {
            r#type: ColumnType::BOOLEAN,
            flags: ColumnFlags::empty(),
        }
    }
}

impl Encode for bool {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = if self { b"\x01" } else { b"\x00" };
        args.push(XuguArgumentValue::Bin(Cow::Borrowed(buf)));

        Ok(IsNull::No)
    }
}

impl Decode for bool {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        // 整数类型 转 bool
        match value.type_info.r#type {
            ColumnType::TINYINT
            | ColumnType::SMALLINT
            | ColumnType::INTEGER
            | ColumnType::BIGINT => return <i64 as Decode>::decode(value).map(|x| x != 0),
            _ => {}
        }

        match value.as_bytes()?[0] {
            b'T' | b't' => Ok(true),
            b'1' | 0x01 => Ok(true),
            b'U' | b'u' => Ok(true),

            b'F' | b'f' => Ok(false),
            b'0' | 0x00 => Ok(false),
            s => Err(format!("unexpected value '{}' for boolean", s as char).into()),
        }
    }
}
