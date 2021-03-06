
use ::TryInto;

use std::fmt;

/// Parses a String into a Value, first attempting bool, i64, and f64.
///
/// Examples:
/// ```
/// match parse_into_value("32.3") {
///     Value::Float(f) => println!("value is float: {}", f),
///     _ => panic!("it's broke yo"),
/// }
///
/// match parse_into_value("something silly") {
///     Value::String(s) => println!("none of the above: {}", s),
///     _ => panic!("it's really broke yo")
/// }
/// ```
pub fn parse_into_value(into: String) -> Value {
    match into.parse::<bool>() {
        Ok(b) => return Value::Bool(b),
        Err(_) => {},
    }

    match into.parse::<i64>() {
        Ok(i) => return Value::Integer(i),
        Err(_) => {},
    }

    match into.parse::<f64>() {
        Ok(f) => return Value::Float(f),
        Err(_) => {},
    }

    Value::String(into)
}

#[derive(Debug, PartialEq, Clone)]
/// Wrapper type for possible types in nccl configuration.
pub enum Value {
    String(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
}

impl Value {
    fn into_string(self) -> Result<String, ()> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(())
        }
    }

    fn into_bool(self) -> Result<bool, ()> {
        match self {
            Value::Bool(b) => Ok(b),
            _ => Err(())
        }
    }

    fn into_integer(self) -> Result<i64, ()> {
        match self {
            Value::Integer(i) => Ok(i),
            _ => Err(())
        }
    }

    fn into_float(self) -> Result<f64, ()> {
        match self {
            Value::Float(f) => Ok(f),
            _ => Err(())
        }
    }
}

impl TryInto<String> for Value {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        self.into_string()
    }
}

impl TryInto<bool> for Value {
    type Error = ();
    fn try_into(self) -> Result<bool, Self::Error> {
        self.into_bool()
    }
}

impl TryInto<i64> for Value {
    type Error = ();
    fn try_into(self) -> Result<i64, Self::Error> {
        self.into_integer()
    }
}

impl TryInto<f64> for Value {
    type Error = ();
    fn try_into(self) -> Result<f64, Self::Error> {
        self.into_float()
    }
}

impl<'a> From<&'a Value> for Value {
    fn from(v: &'a Value) -> Self {
        v.clone()
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl<'a> From<&'a String> for Value {
    fn from(s: &'a String) -> Self {
        Value::String(s.to_owned())
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::String(s.to_owned())
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        self.into_string().unwrap()
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        self.into_bool().unwrap()
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        self.into_integer().unwrap()
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        self.into_float().unwrap()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Bool(b) => write!(f, "{}", b),
            &Value::String(ref s) => write!(f, "{}", s),
            &Value::Float(fl) => write!(f, "{}", fl),
            &Value::Integer(i) => write!(f, "{}", i),
        }
    }
}

