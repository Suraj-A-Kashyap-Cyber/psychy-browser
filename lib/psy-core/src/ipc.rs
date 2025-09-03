use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;   // ← bring the alias into scope

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Request {
    NewTab { url: Option<String> },
    NewIdentity,
    RoutingIntent { mode: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok(JsonValue),
    Err(String),
}
