use crate::arguments::XuguArgumentValue;
use crate::protocol::text::ColumnType;
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use rbdc::{Decimal, Error, Uuid};
use rbs::Value;

impl Decode for Value {
    fn decode(v: XuguValue) -> Result<Self, Error> {
        let type_info = v.type_info.r#type;
        Ok(match type_info {
            ColumnType::NONE => Value::Null,
            ColumnType::NULL => Value::Null,
            ColumnType::BOOLEAN => Value::Bool(<bool as Decode>::decode(v)?),
            ColumnType::TINYINT => Value::I32(Decode::decode(v)?),
            ColumnType::SMALLINT => Value::I32(Decode::decode(v)?),
            ColumnType::INTEGER => Value::I32(Decode::decode(v)?),
            ColumnType::BIGINT => Value::I64(Decode::decode(v)?),
            ColumnType::FLOAT => Value::F32(Decode::decode(v)?),
            ColumnType::DOUBLE => Value::F64(Decode::decode(v)?),
            ColumnType::NUMERIC => <Decimal as Decode>::decode(v)?.into(),
            ColumnType::CHAR => Value::String(Decode::decode(v)?),
            ColumnType::NCHAR => Value::String(Decode::decode(v)?),
            ColumnType::CLOB => Value::String(Decode::decode(v)?),
            ColumnType::BINARY => Value::Binary(Decode::decode(v)?),
            ColumnType::BLOB => Value::Binary(Decode::decode(v)?),
            ColumnType::BLOB_I => Value::Binary(Decode::decode(v)?),
            ColumnType::BLOB_S => Value::Binary(Decode::decode(v)?),
            ColumnType::BLOB_M => Value::Binary(Decode::decode(v)?),
            ColumnType::BLOB_OM => Value::Binary(Decode::decode(v)?),
            ColumnType::ROWID => Value::String(Decode::decode(v)?),
            ColumnType::BIT => Value::Binary(Decode::decode(v)?),
            ColumnType::VARBIT => Value::Binary(Decode::decode(v)?),
            ColumnType::XML => Value::String(Decode::decode(v)?),
            ColumnType::JSON => Value::String(Decode::decode(v)?),
            ColumnType::GUID => <Uuid as Decode>::decode(v)?.into(),
            _ => {
                // TODO 其他类型暂不支持
                Value::String(Decode::decode(v)?)
            }
        })
    }
}

impl Encode for Value {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error> {
        match self {
            Value::Null => Ok(IsNull::Yes),
            Value::Bool(v) => v.encode(buf),
            Value::I32(v) => v.encode(buf),
            Value::I64(v) => v.encode(buf),
            Value::U32(v) => v.encode(buf),
            Value::U64(v) => v.encode(buf),
            Value::F32(v) => v.encode(buf),
            Value::F64(v) => v.encode(buf),
            Value::String(v) => v.encode(buf),
            Value::Binary(v) => v.encode(buf),
            Value::Array(_) => todo!(),
            Value::Map(_) => todo!(),
            Value::Ext(ext_type, v) => match ext_type {
                "Uuid" => v.into_string().unwrap_or_default().encode(buf),
                "Decimal" => v.into_string().unwrap_or_default().encode(buf),
                _ => todo!(),
            },
        }
    }
}

impl TypeInfo for Value {
    fn type_info(&self) -> XuguTypeInfo {
        match self {
            Value::Null => XuguTypeInfo::null(),
            Value::Bool(_) => XuguTypeInfo::from_type(ColumnType::BOOLEAN),
            Value::I32(_) => XuguTypeInfo::from_type(ColumnType::INTEGER),
            Value::I64(_) => XuguTypeInfo::from_type(ColumnType::BIGINT),
            Value::U32(_) => XuguTypeInfo::from_type(ColumnType::INTEGER),
            Value::U64(_) => XuguTypeInfo::from_type(ColumnType::BIGINT),
            Value::F32(_) => XuguTypeInfo::from_type(ColumnType::FLOAT),
            Value::F64(_) => XuguTypeInfo::from_type(ColumnType::DOUBLE),
            Value::String(_) => XuguTypeInfo::from_type(ColumnType::CHAR),
            Value::Binary(_) => XuguTypeInfo::from_type(ColumnType::BINARY),
            Value::Array(_) => XuguTypeInfo::from_type(ColumnType::ARRAY),
            Value::Map(_) => XuguTypeInfo::from_type(ColumnType::JSON),
            Value::Ext(ext_type, _) => match *ext_type {
                "Uuid" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Decimal" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Date" => XuguTypeInfo::from_type(ColumnType::DATE),
                "Time" => XuguTypeInfo::from_type(ColumnType::TIME),
                "Timestamp" => XuguTypeInfo::from_type(ColumnType::DATETIME),
                "DateTime" => XuguTypeInfo::from_type(ColumnType::DATETIME),
                "Bool" => XuguTypeInfo::from_type(ColumnType::BOOLEAN),
                "Char" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Json" => XuguTypeInfo::from_type(ColumnType::JSON),

                "Point" => XuguTypeInfo::from_type(ColumnType::POINT),
                "Lseg" => XuguTypeInfo::from_type(ColumnType::LSEG),
                "Path" => XuguTypeInfo::from_type(ColumnType::PATH),
                "Box" => XuguTypeInfo::from_type(ColumnType::BOX),
                "Polygon" => XuguTypeInfo::from_type(ColumnType::POLYGON),
                "Line" => XuguTypeInfo::from_type(ColumnType::LINE),
                "Circle" => XuguTypeInfo::from_type(ColumnType::CIRCLE),

                "Varchar" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "DATETIME_TZ" => XuguTypeInfo::from_type(ColumnType::DATETIME_TZ),

                _ => XuguTypeInfo::null(),
            },
        }
    }
}
