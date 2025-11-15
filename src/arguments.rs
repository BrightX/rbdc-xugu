use crate::type_info::XuguTypeInfo;
use crate::types::{Encode, IsNull, TypeInfo};
use bytes::Bytes;
use rbdc::Error;
use rbs::Value;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum XuguArgumentValue<'q> {
    Null,
    Str(Cow<'q, str>),
    Bin(Cow<'q, [u8]>),
    Bytes(Bytes),
}

#[derive(Debug, Default, Clone)]
pub struct XuguArguments<'q> {
    pub(crate) values: Vec<XuguArgumentValue<'q>>,
    pub(crate) types: Vec<XuguTypeInfo>,
}

impl<'q> XuguArguments<'q> {
    pub(crate) fn add<T>(&mut self, value: T) -> Result<(), Error>
    where
        T: Encode + TypeInfo,
    {
        let ty = value.produces().unwrap_or_else(|| value.type_info());

        let value_length_before_encoding = self.values.len();
        match value.encode(&mut self.values) {
            Ok(IsNull::Yes) => self.values.push(XuguArgumentValue::Null),
            Ok(IsNull::No) => {}
            Err(error) => {
                // reset the value buffer to its previous value if encoding failed so we don't leave a half-encoded value behind
                self.values.truncate(value_length_before_encoding);
                return Err(error);
            }
        };

        self.types.push(ty);

        Ok(())
    }
}

impl<'q> XuguArguments<'q> {
    pub fn from_args(args: Vec<Value>) -> Result<Self, Error> {
        let mut arg = Self {
            types: Vec::with_capacity(args.len()),
            values: Vec::with_capacity(args.len()),
        };
        for x in args {
            arg.add(x)?;
        }

        Ok(arg)
    }
}
