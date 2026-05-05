use crate::column::XuguColumn;
use crate::meta_data::XuguMetaData;
use crate::types::Decode;
use crate::value::XuguValue;
use bytes::Bytes;
use rbdc::db::MetaData;
use rbdc::ext::ustr::UStr;
use rbdc::Error;
use rbs::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
#[allow(dead_code)]
pub struct XuguRow {
    pub(crate) row: Arc<Vec<Bytes>>,
    pub(crate) columns: Arc<Vec<XuguColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl XuguRow {
    fn try_take(&mut self, index: usize) -> Option<XuguValue> {
        let column = &self.columns[index];
        let value = self.row.get(index).cloned();

        if is_null(value.as_deref()) {
            return None;
        }

        Some(XuguValue {
            type_info: column.type_info.clone(),
            value,
        })
    }
}

impl rbdc::db::Row for XuguRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        Box::new(XuguMetaData {
            columns: self.columns.clone(),
        })
    }

    fn get(&mut self, i: usize) -> Result<Value, Error> {
        match self.try_take(i) {
            None => Ok(Value::Null),
            Some(v) => Value::decode(v),
        }
    }
}

fn is_null(value: Option<&[u8]>) -> bool {
    if let Some(value) = value {
        if value.is_empty() {
            return true;
        }
    }

    value.is_none()
}
