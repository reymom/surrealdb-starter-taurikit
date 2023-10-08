use crate::model::types::{Name, Person};
use crate::model::{take_bool, take_object, take_string};
use crate::{Error, Result};
use surrealdb::sql::{Array, Object, Value};

// Generic Wrapper tuple struct for newtype pattern, mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

impl TryFrom<Object> for Name {
    type Error = Error;

    fn try_from(mut val: Object) -> Result<Name> {
        let name: Name = Name {
            first: take_string(val.clone(), "first")?,
            last: take_string(val, "last")?,
        };

        Ok(name)
    }
}

impl TryFrom<Object> for Person {
    type Error = Error;

    fn try_from(mut val: Object) -> Result<Person> {
        let name: Result<Name> = take_object(val.clone(), "name")?.try_into();
        let task = Person {
            id: take_string(val.clone(), "id")?,
            title: take_string(val.clone(), "title")?,
            name: name?,
            marketing: take_bool(val, "marketing")?,
        };

        Ok(task)
    }
}

impl TryFrom<W<Value>> for Object {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Object> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(Error::XValueNotOfType("Object")),
        }
    }
}

impl TryFrom<W<Value>> for Array {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Array> {
        match val.0 {
            Value::Array(obj) => Ok(obj),
            _ => Err(Error::XValueNotOfType("Array")),
        }
    }
}

impl TryFrom<W<Value>> for String {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<String> {
        match val.0 {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(Error::XValueNotOfType("String")),
        }
    }
}

impl TryFrom<W<Value>> for i64 {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<i64> {
        match val.0 {
            Value::Number(obj) => Ok(obj.as_int()),
            _ => Err(Error::XValueNotOfType("i64")),
        }
    }
}

impl TryFrom<W<Value>> for bool {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<bool> {
        match val.0 {
            Value::Bool(true) => Ok(true),
            Value::Bool(false) => Ok(false),
            _ => Err(Error::XValueNotOfType("bool")),
        }
    }
}
