use rusqlite::types::{FromSql, ToSql, ToSqlOutput, Value};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatGroup {
    Miscellaneous,
    Image,
    Video,
}

impl FormatGroup {
    pub fn new(format: &str) -> Self {
        match format {
            "jpg" => FormatGroup::Image,
            "jpeg" => FormatGroup::Image,
            "png" => FormatGroup::Image,
            "webp" => FormatGroup::Image,

            "gif" => FormatGroup::Video,
            "webm" => FormatGroup::Video,
            "mp4" => FormatGroup::Video,

            _ => FormatGroup::Miscellaneous,
        }
    }
}

impl FromSql for FormatGroup {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let i: i64 = <i64 as FromSql>::column_result(value)?;

        let fg = match i {
            0 => FormatGroup::Miscellaneous,
            1 => FormatGroup::Image,
            2 => FormatGroup::Video,
            _ => return Err(rusqlite::types::FromSqlError::OutOfRange(i)),
        };
        Ok(fg)
    }
}

impl ToSql for FormatGroup {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let i: i64 = match self {
            FormatGroup::Miscellaneous => 0,
            FormatGroup::Image => 1,
            FormatGroup::Video => 2,
        };

        Ok(ToSqlOutput::Owned(Value::Integer(i)))
    }
}

impl Display for FormatGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatGroup::Miscellaneous => write!(f, "Miscellaneous"),
            FormatGroup::Image => write!(f, "Image"),
            FormatGroup::Video => write!(f, "Video"),
        }
    }
}
