use crate::quote;
use std::{
    borrow::{Cow, ToOwned},
    ops::Deref,
};

#[cfg(feature = "chrono")]
pub mod chrono;

pub struct SqlArg(String);

impl ToString for SqlArg {
    fn to_string(&self) -> String {
        self.0
    }
}

impl Deref for SqlArg {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ToSqlArg {
    fn to_sql_arg(&self) -> SqlArg;
}

impl ToSqlArg for str {
    fn to_sql_arg(&self) -> String {
        SqlArg(quote(self))
    }
}

impl ToSqlArg for &str {
    fn to_sql_arg(&self) -> String {
        quote(self)
    }
}

impl ToSqlArg for Cow<'_, str> {
    fn to_sql_arg(&self) -> String {
        quote(self[..].to_owned())
    }
}

impl ToSqlArg for String {
    fn to_sql_arg(&self) -> String {
        quote(self)
    }
}

impl ToSqlArg for i8 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for u8 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for i16 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for u16 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for i32 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for u32 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for i64 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for u64 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for i128 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for u128 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for isize {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for usize {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for f32 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for f64 {
    fn to_sql_arg(&self) -> String {
        self.to_string()
    }
}

impl ToSqlArg for bool {
    fn to_sql_arg(&self) -> String {
        String::from(if *self { "TRUE" } else { "FALSE" })
    }
}

#[cfg(feature = "uuid")]
impl ToSqlArg for uuid::Uuid {
    fn to_sql_arg(&self) -> String {
        self.to_hyphenated().to_string()
    }
}

impl<T: ToSqlArg> ToSqlArg for Option<T> {
    fn to_sql_arg(&self) -> String {
        match &*self {
            Some(value) => value.to_sql_arg(),
            None => String::from("NULL"),
        }
    }
}

impl<T: ToSqlArg> ToSqlArg for &T {
    fn to_sql_arg(&self) -> String {
        (*self).to_sql_arg()
    }
}
