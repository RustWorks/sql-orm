use super::SqlArg;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

impl SqlArg for NaiveDateTime {
    fn sql_arg(&self) -> String {
        format!("{}", self)
    }
}

impl SqlArg for NaiveDate {
    fn sql_arg(&self) -> String {
        format!("{}", self)
    }
}

impl SqlArg for NaiveTime {
    fn sql_arg(&self) -> String {
        format!("{}", self)
    }
}