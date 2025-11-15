use crate::arguments::XuguArgumentValue;
use crate::protocol::text::{ColumnFlags, ColumnType};
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use rbdc::decimal::Decimal;
use rbdc::Error;
use std::borrow::Cow;
use std::str::FromStr;

impl Encode for Decimal {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        args.push(XuguArgumentValue::Str(Cow::Owned(self.0.to_string())));

        Ok(IsNull::No)
    }
}

impl Decode for Decimal {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        Decimal::from_str(value.as_str().unwrap_or("0"))
    }
}

impl TypeInfo for Decimal {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo {
            r#type: ColumnType::CHAR,
            flags: ColumnFlags::empty(),
        }
    }
}
