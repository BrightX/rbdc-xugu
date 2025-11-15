mod bool;
mod bytes;
mod decimal;
mod float;
mod int;
mod str;
mod uint;
mod uuid;
mod value;

use crate::arguments::XuguArgumentValue;
use crate::type_info::XuguTypeInfo;
use crate::value::XuguValue;
use rbdc::Error;

pub(crate) trait TypeInfo {
    fn type_info(&self) -> XuguTypeInfo;
}

/// The return type of [Encode::encode].
pub(crate) enum IsNull {
    /// The value is null; no data was written.
    Yes,

    /// The value is not null.
    ///
    /// This does not mean that data was written.
    No,
}

pub(crate) trait Decode: Sized {
    fn decode(value: XuguValue) -> Result<Self, Error>;
}

pub(crate) trait Encode {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error>;

    fn produces(&self) -> Option<XuguTypeInfo> {
        // `produces` is inherently a hook to allow database drivers to produce value-dependent
        // type information; if the driver doesn't need this, it can leave this as `None`
        None
    }
}
