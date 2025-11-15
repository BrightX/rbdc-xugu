use crate::arguments::XuguArgumentValue;
use crate::types::{Decode, Encode, IsNull};
use crate::value::XuguValue;
use rbdc::{Date, DateTime, Time, Timestamp};
use rbs::Error;

impl Encode for fastdate::Date {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error> {
        let days: i64 = fastdate::DateTime::from(self).unix_timestamp() / 86400;

        (days as i32).encode(buf)
    }
}

impl Decode for fastdate::Date {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        let days = <i32 as Decode>::decode(value)?;
        let t = fastdate::DateTime::from_timestamp(days as i64 * 86400);

        Ok(Self::from(t))
    }
}

impl Encode for Date {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error> {
        self.0.encode(buf)
    }
}

impl Decode for Date {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        Ok(Self(fastdate::Date::decode(value)?))
    }
}

impl Encode for fastdate::Time {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error> {
        // TIME is encoded as the milliseconds since midnight
        // milliseconds
        let millis: u32 = self.get_micro() / 1000
            + ((self.hour as u32 * 60 + self.minute as u32) * 60 + self.sec as u32) * 1000;

        millis.encode(buf)
    }
}

impl Decode for fastdate::Time {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        let millis: u32 = <u32 as Decode>::decode(value)?;
        let hour = ((millis / 1000 / 60 / 60) % 24) as u8;
        let minute = ((millis / 1000 / 60) % 60) as u8;
        let sec = ((millis / 1000) % 60) as u8;
        let nano = (millis % 1000) * 1_000_000;

        Ok(Self {
            hour,
            minute,
            sec,
            nano,
        })
    }
}

impl Encode for Time {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error> {
        self.0.encode(buf)
    }
}

impl Decode for Time {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        Ok(Self(fastdate::Time::decode(value)?))
    }
}

impl Encode for fastdate::DateTime {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error> {
        let micros: i64 = self.unix_timestamp_micros();

        micros.encode(buf)
    }
}

impl Decode for fastdate::DateTime {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        let micros = <i64 as Decode>::decode(value)?;
        let t = fastdate::DateTime::from_timestamp_micros(micros);

        Ok(t)
    }
}

impl Encode for DateTime {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error> {
        self.0.encode(buf)
    }
}

impl Decode for DateTime {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        Ok(Self(fastdate::DateTime::decode(value)?))
    }
}

impl Encode for Timestamp {
    fn encode(self, buf: &mut Vec<XuguArgumentValue>) -> Result<IsNull, Error> {
        // Timestamp(timestamp_millis:u64)
        let micros: i64 = self.0 * 1000;

        micros.encode(buf)
    }
}

impl Decode for Timestamp {
    fn decode(value: XuguValue) -> Result<Self, Error> {
        let micros = <i64 as Decode>::decode(value)?;
        let millis = micros / 1000;

        // Timestamp(timestamp_millis:u64)
        Ok(Self(millis))
    }
}
