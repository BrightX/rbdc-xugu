use crate::protocol::text::ColumnFlags;
use crate::type_info::XuguTypeInfo;
use rbdc::ext::ustr::UStr;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XuguColumn {
    pub(crate) name: UStr,
    pub(crate) type_info: XuguTypeInfo,

    pub(crate) flags: Option<ColumnFlags>,
}
