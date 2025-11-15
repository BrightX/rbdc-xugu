use crate::arguments::XuguArgumentValue;
use crate::protocol::text::ColumnType;
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;
use std::borrow::Cow;
use std::str::FromStr;

impl TypeInfo for f32 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::FLOAT)
    }
}

impl TypeInfo for f64 {
    fn type_info(&self) -> XuguTypeInfo {
        XuguTypeInfo::binary(ColumnType::DOUBLE)
    }
}

impl Encode for f32 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

impl Encode for f64 {
    fn encode(self, args: &mut Vec<XuguArgumentValue<'_>>) -> Result<IsNull, Error> {
        let buf = self.to_be_bytes().to_vec();
        args.push(XuguArgumentValue::Bin(Cow::Owned(buf)));

        Ok(IsNull::No)
    }
}

impl Decode for f32 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        if value.type_info.r#type == ColumnType::CHAR {
            let s = value.as_str()?;
            return Ok(f32::from_str(s)?);
        }

        let buf = value.as_bytes()?;

        Ok(match buf.len() {
            // These functions panic if `buf` is not exactly the right size.
            4 => BigEndian::read_f32(buf),
            // Xugu can return 8-byte DOUBLE values for a FLOAT
            // We take and truncate to f32 as that's the same behavior as *in* Xugu,
            #[allow(clippy::cast_possible_truncation)]
            8 => BigEndian::read_f64(buf) as f32,
            other => {
                // Users may try to decode a DECIMAL as floating point;
                // inform them why that's a bad idea.
                return Err(format!(
                    "expected a FLOAT as 4 or 8 bytes, got {other} bytes; \
                             note that decoding DECIMAL as `f32` is not supported \
                             due to differing semantics"
                )
                .into());
            }
        })
    }
}

impl Decode for f64 {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        if value.type_info.r#type == ColumnType::CHAR {
            let s = value.as_str()?;
            return Ok(f64::from_str(s)?);
        }

        let buf = value.as_bytes()?;

        // The `read_*` functions panic if `buf` is not exactly the right size.
        Ok(match buf.len() {
            // Allow implicit widening here
            4 => BigEndian::read_f32(buf) as f64,
            8 => BigEndian::read_f64(buf),
            other => {
                // Users may try to decode a DECIMAL as floating point;
                // inform them why that's a bad idea.
                return Err(format!(
                    "expected a DOUBLE as 4 or 8 bytes, got {other} bytes; \
                             note that decoding DECIMAL as `f64` is not supported \
                             due to differing semantics"
                )
                .into());
            }
        })
    }
}
