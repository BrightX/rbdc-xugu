use crate::arguments::XuguArgumentValue;
use crate::protocol::text::{ColumnFlags, ColumnType};
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use rbdc::Error;
use std::borrow::Cow;

impl TypeInfo for String {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo {
            r#type: ColumnType::CHAR,
            flags: ColumnFlags::empty(),
        }
    }
}

impl Encode for String {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        args.push(XuguArgumentValue::Str(Cow::Owned(self)));

        Ok(IsNull::No)
    }
}

impl Decode for String {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        let ty = value.type_info.r#type;
        if matches!(
            ty,
            ColumnType::TINYINT | ColumnType::SMALLINT | ColumnType::INTEGER | ColumnType::BIGINT
        ) {
            let num = <i64 as Decode>::decode(value)?;
            return Ok(num.to_string());
        }

        value.as_str().map(ToOwned::to_owned)
    }
}
