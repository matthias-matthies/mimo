#[derive(Debug, Clone, Copy)]
pub enum EventType {
    SystemEvent,
    UserEvent,
    HeartBeat
}

pub enum MessageRole {
    Ai,
    System,
    Tool,
    User,
}

pub struct Message {
    pub message_role: MessageRole,
    pub message_content: String
}

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub event_type: EventType,
}

#[derive(Debug)]
pub struct State {
    pub heart_beats_since_start: u32,
}
