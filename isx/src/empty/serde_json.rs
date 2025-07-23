use super::IsEmpty;
use serde_json::Value;

/// A [`serde_json::Value`] is empty if it is any of:
/// * [`Value::Null`]
impl IsEmpty for Value {
    fn is_empty(&self) -> bool {
        match self {
            Value::Null => true,
            Value::Array(array) => array.is_empty(),
            Value::Object(map) => map.is_empty(),
            Value::String(string) => string.is_empty(),
            _ => false,
        }
    }
}
