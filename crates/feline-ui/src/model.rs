#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChatAuthor {
    Assistant,
    User,
}

pub struct ChatMessage {
    pub author: ChatAuthor,
    pub text: String,
}

impl ChatMessage {
    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            author: ChatAuthor::Assistant,
            text: text.into(),
        }
    }

    pub fn user(text: impl Into<String>) -> Self {
        Self {
            author: ChatAuthor::User,
            text: text.into(),
        }
    }
}

pub fn demo_messages() -> Vec<ChatMessage> {
    vec![
        ChatMessage::assistant(
            "Hey, I am Feline. I can sit over your desktop and keep the conversation compact.",
        ),
        ChatMessage::user("Nice. Can you keep the UI close to the sketch?"),
        ChatMessage::assistant(
            "Yes. I am using a tall glass panel, a simple title bar, alternating chat bubbles, and a pinned composer.",
        ),
        ChatMessage::assistant(
            "The messages are hardcoded for now, but the render path is ready for real chat state.",
        ),
        ChatMessage::user("Perfect. Next step will be wiring the model backend."),
    ]
}

pub fn canned_reply(input: &str) -> String {
    format!("Stub reply received: \"{input}\". Backend wiring can replace this later.")
}
