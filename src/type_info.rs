use std::fmt::{self, Display, Formatter};

use crate::protocol::text::{ColumnDefinition, ColumnFlags, ColumnType};

/// Type information for a Xugu type.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XuguTypeInfo {
    pub(crate) r#type: ColumnType,
    pub(crate) flags: ColumnFlags,
}

impl XuguTypeInfo {
    pub(crate) const fn null() -> Self {
        Self {
            r#type: ColumnType::NULL,
            flags: ColumnFlags::empty(),
        }
    }

    pub(crate) const fn from_type(ty: ColumnType) -> Self {
        Self {
            r#type: ty,
            flags: ColumnFlags::empty(),
        }
    }

    pub(crate) const fn binary(ty: ColumnType) -> Self {
        Self {
            r#type: ty,
            flags: ColumnFlags::IS_LOB,
        }
    }

    #[doc(hidden)]
    pub fn __type_feature_gate(&self) -> Option<&'static str> {
        match self.r#type {
            ColumnType::DATE
            | ColumnType::TIME
            | ColumnType::DATETIME
            | ColumnType::TIME_TZ
            | ColumnType::DATETIME_TZ => Some("time"),

            ColumnType::JSON => Some("json"),
            ColumnType::NUMERIC => Some("numeric"),

            _ => None,
        }
    }

    pub(crate) fn from_column(column: &ColumnDefinition) -> Self {
        Self {
            r#type: column.r#type,
            flags: column.flags,
        }
    }
}

impl Display for XuguTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.name())
    }
}

impl XuguTypeInfo {
    fn name(&self) -> &str {
        self.r#type.name()
    }
}

impl PartialEq<XuguTypeInfo> for XuguTypeInfo {
    fn eq(&self, other: &XuguTypeInfo) -> bool {
        if self.r#type != other.r#type {
            return false;
        }

        true
    }
}

impl Eq for XuguTypeInfo {}
