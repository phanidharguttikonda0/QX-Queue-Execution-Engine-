use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub queue_name: String,
    pub message: String // the value to be passed after parsing into string
}

#[derive(Debug, Deserialize)]
pub struct Queue {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeadLetterQueue {
    pub dead_letter_queue: VecDeque<Message>,
}