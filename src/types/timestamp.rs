use std::fmt::Display;
use chrono::{DateTime, Local, prelude::*};
use rusqlite::{ToSql, types::FromSql, types::ToSqlOutput};

pub struct TimeStamp {
    stamp: i64,
}

impl TimeStamp {

    pub fn now() -> Self {
        Self::new(chrono::Utc::now().timestamp())
    }

    pub fn new(stamp: i64) -> Self {
        Self { stamp }
    }
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: the unwrap here is not good
        let date: DateTime<Local> = match DateTime::from_timestamp(self.stamp, 0) {
            Some(date) => date,
            None => return Err(std::fmt::Error {}), // TODO: I don't understand this line
        }
        .into();

        write!(
            f,
            "<{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}>",
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute()
        )?;

        Ok(())
    }
}

impl ToSql for TimeStamp {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.stamp.to_sql()
    }
}

impl FromSql for TimeStamp {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let stamp = <i64 as FromSql>::column_result(value)?;
        Ok(Self::new(stamp))
    }
}
