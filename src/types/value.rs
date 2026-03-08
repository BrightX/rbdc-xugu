use crate::arguments::XuguArgumentValue;
use crate::protocol::text::ColumnType;
use crate::type_info::XuguTypeInfo;
use crate::types::{Decode, Encode, IsNull, TypeInfo};
use crate::value::XuguValue;
use rbdc::{Date, DateTime, Decimal, Error, Time, Timestamp, Uuid};
use rbs::Value;
use std::str::FromStr;

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
            ColumnType::ROWVERSION => Value::String(Decode::decode(v)?),
            ColumnType::BIT => Value::Binary(Decode::decode(v)?),
            ColumnType::VARBIT => Value::Binary(Decode::decode(v)?),
            ColumnType::XML => Value::String(Decode::decode(v)?),
            ColumnType::JSON => Value::String(Decode::decode(v)?),
            ColumnType::GUID => <Uuid as Decode>::decode(v)?.into(),
            ColumnType::DATE => <Date as Decode>::decode(v)?.into(),
            ColumnType::TIME => <Time as Decode>::decode(v)?.into(),
            ColumnType::DATETIME => <DateTime as Decode>::decode(v)?.into(),
            ColumnType::INTERVAL_Y
            | ColumnType::INTERVAL_Y2M
            | ColumnType::INTERVAL_M
            | ColumnType::INTERVAL_D
            | ColumnType::INTERVAL_D2H
            | ColumnType::INTERVAL_H
            | ColumnType::INTERVAL_D2M
            | ColumnType::INTERVAL_H2M
            | ColumnType::INTERVAL_MI
            | ColumnType::INTERVAL_D2S
            | ColumnType::INTERVAL_H2S
            | ColumnType::INTERVAL_M2S
            | ColumnType::INTERVAL_S => super::time::interval::decode(v)?,
            // 几何类型 按字符串编解码
            ColumnType::POINT => Value::Ext("Point", Box::new(Value::String(Decode::decode(v)?))),
            ColumnType::LSEG => Value::Ext("Lseg", Box::new(Value::String(Decode::decode(v)?))),
            ColumnType::PATH => Value::Ext("Path", Box::new(Value::String(Decode::decode(v)?))),
            ColumnType::BOX => Value::Ext("Box", Box::new(Value::String(Decode::decode(v)?))),
            ColumnType::POLYGON => {
                Value::Ext("Polygon", Box::new(Value::String(Decode::decode(v)?)))
            }
            ColumnType::LINE => Value::Ext("Line", Box::new(Value::String(Decode::decode(v)?))),
            ColumnType::CIRCLE => Value::Ext("Circle", Box::new(Value::String(Decode::decode(v)?))),
            ColumnType::GEOMETRY => {
                Value::Ext("Geometry", Box::new(Value::String(Decode::decode(v)?)))
            }
            ColumnType::GEOGRAPHY => {
                Value::Ext("Geography", Box::new(Value::String(Decode::decode(v)?)))
            }
            ColumnType::BOX2D => Value::Ext("Box2d", Box::new(Value::String(Decode::decode(v)?))),
            ColumnType::BOX3D => Value::Ext("Box3d", Box::new(Value::String(Decode::decode(v)?))),
            ColumnType::SPHEROID => {
                Value::Ext("Spheroid", Box::new(Value::String(Decode::decode(v)?)))
            }
            ColumnType::RASTER => Value::Ext("Raster", Box::new(Value::String(Decode::decode(v)?))),

            _ => {
                // TODO 其他类型暂不支持
                Value::Binary(Decode::decode(v)?)
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
                "Date" => Date::from_str(&v.into_string().unwrap_or_default())
                    .map_err(|e| Error::from(e.to_string()))?
                    .encode(buf),
                "Time" => Time::from_str(&v.into_string().unwrap_or_default())
                    .map_err(|e| Error::from(e.to_string()))?
                    .encode(buf),
                "DateTime" => DateTime::from_str(&v.into_string().unwrap_or_default())
                    .map_err(|e| Error::from(e.to_string()))?
                    .encode(buf),
                "Timestamp" => Timestamp(v.as_i64().unwrap_or_default()).encode(buf),
                "Interval" => v.as_f64().unwrap_or_default().encode(buf),

                // 几何类型 按字符串编解码
                "Point" => v.into_string().unwrap_or_default().encode(buf),
                "Lseg" => v.into_string().unwrap_or_default().encode(buf),
                "Path" => v.into_string().unwrap_or_default().encode(buf),
                "Box" => v.into_string().unwrap_or_default().encode(buf),
                "Polygon" => v.into_string().unwrap_or_default().encode(buf),
                "Line" => v.into_string().unwrap_or_default().encode(buf),
                "Circle" => v.into_string().unwrap_or_default().encode(buf),
                "Geometry" => v.into_string().unwrap_or_default().encode(buf),
                "Geography" => v.into_string().unwrap_or_default().encode(buf),
                "Box2d" => v.into_string().unwrap_or_default().encode(buf),
                "Box3d" => v.into_string().unwrap_or_default().encode(buf),
                "Spheroid" => v.into_string().unwrap_or_default().encode(buf),
                "Raster" => v.into_string().unwrap_or_default().encode(buf),

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
                "Interval" => XuguTypeInfo::from_type(ColumnType::DOUBLE),

                // 几何类型 按字符串编解码
                "Point" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Lseg" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Path" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Box" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Polygon" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Line" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Circle" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Geometry" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Geography" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Box2d" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Box3d" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Spheroid" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "Raster" => XuguTypeInfo::from_type(ColumnType::CHAR),

                "Varchar" => XuguTypeInfo::from_type(ColumnType::CHAR),
                "DATETIME_TZ" => XuguTypeInfo::from_type(ColumnType::DATETIME_TZ),

                _ => XuguTypeInfo::null(),
            },
        }
    }
}
