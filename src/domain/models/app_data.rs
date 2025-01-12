use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Message for target application
///
/// Contains application name, operation ID and payload
///
/// for example: `{"app":"bank","operation":"transfer","payload":{"receiver":"addr"}}`
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct AppData {
    /// app name; ex: `bank`
    pub app: String,
    /// app operation; ex: `transfer`
    pub operation: String,
    /// app payload; ex: `{ "receiver": "addr" }`
    pub payload: Value
}


#[cfg(test)]
mod tests {
    #[test]
    fn serialize_deserialize_app_data() {
        let original_data = super::AppData {
            app: "bank".to_string(),
            operation: "transfer".to_string(),
            payload: serde_json::json!({ "receiver": "addr" })
        };

        let serialized_data = serde_json::to_string(&original_data).unwrap();
        let deserialized_data: super::AppData = serde_json::from_str(&serialized_data).unwrap();

        assert_eq!(original_data, deserialized_data);
    }
}
