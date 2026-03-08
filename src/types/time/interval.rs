use crate::protocol::text::ColumnType;
use crate::value::XuguValue;
use bytes::Buf;
use rbs::{Error, Value};

const HOURS_PER_DAY_F: f64 = 24.0;
const MINUTES_PER_DAY_F: f64 = 1440.0;
/// The number of microseconds per in days.
const MICROS_PER_DAY_F: f64 = 86_400_000_000.0;

pub(crate) fn decode(v: XuguValue) -> Result<Value, Error> {
    let type_info = v.type_info.r#type;
    let mut buf = v.as_bytes()?;

    let value = match type_info {
        ColumnType::INTERVAL_Y => todo!(),
        ColumnType::INTERVAL_Y2M | ColumnType::INTERVAL_M => todo!(),
        // 精确到天
        ColumnType::INTERVAL_D => {
            let day: i32 = buf.get_i32();
            Value::Ext("Interval", Box::new(Value::F64(day as f64)))
        }
        // 精确到小时
        ColumnType::INTERVAL_D2H | ColumnType::INTERVAL_H => {
            let h: i32 = buf.get_i32();
            let day: f64 = h as f64 / HOURS_PER_DAY_F;
            Value::Ext("Interval", Box::new(Value::F64(day)))
        }
        // 精确到分钟
        ColumnType::INTERVAL_D2M | ColumnType::INTERVAL_H2M | ColumnType::INTERVAL_MI => {
            let min: i32 = buf.get_i32();
            let day: f64 = min as f64 / MINUTES_PER_DAY_F;
            Value::Ext("Interval", Box::new(Value::F64(day)))
        }
        // 精确到秒
        ColumnType::INTERVAL_D2S
        | ColumnType::INTERVAL_H2S
        | ColumnType::INTERVAL_M2S
        | ColumnType::INTERVAL_S => {
            let us: i64 = buf.get_i64();
            let day: f64 = us as f64 / MICROS_PER_DAY_F;
            Value::Ext("Interval", Box::new(Value::F64(day)))
        }
        _ => {
            return Err(Error::from(
                "[E50044] Resultset: Required type conversion not allowed",
            ))
        }
    };

    Ok(value)
}
