use crate::column::XuguColumn;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct XuguMetaData {
    pub(crate) columns: Arc<Vec<XuguColumn>>,
}

impl rbdc::db::MetaData for XuguMetaData {
    fn column_len(&self) -> usize {
        self.columns.len()
    }

    fn column_name(&self, i: usize) -> String {
        if cfg!(feature = "lowercase_column_name") {
            self.columns[i].name.to_ascii_lowercase()
        } else {
            self.columns[i].name.to_string()
        }
    }

    fn column_type(&self, i: usize) -> String {
        self.columns[i].type_info.r#type.name().to_string()
    }
}
