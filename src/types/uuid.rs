use crate::arguments::XuguArgumentValue;
use crate::protocol::text::{ColumnFlags, ColumnType};
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use rbdc::uuid::Uuid;
use rbdc::Error;
use std::borrow::Cow;

impl Encode for Uuid {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        args.push(XuguArgumentValue::Str(Cow::Owned(self.0)));

        Ok(IsNull::No)
    }
}

impl Decode for Uuid {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        Ok(Self(value.as_str().unwrap_or_default().to_string()))
    }
}

impl TypeInfo for Uuid {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo {
            r#type: ColumnType::CHAR,
            flags: ColumnFlags::empty(),
        }
    }
}
