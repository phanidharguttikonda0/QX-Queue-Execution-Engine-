use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub name: String,
    pub value: String // the value to be passed after parsing into string
}