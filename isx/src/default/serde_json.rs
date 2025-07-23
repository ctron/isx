use super::IsDefault;

impl IsDefault for serde_json::Value {
    fn is_default(&self) -> bool {
        matches!(self, serde_json::Value::Null)
    }
}
