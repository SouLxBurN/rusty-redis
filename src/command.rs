#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Command {
    GET(String),
    KEYS,
    SET(String, Vec<u8>),
    DELETE(String)
}

impl Command {
    pub fn encode(&self) -> Vec<u8> {
        let mut command: Vec<u8> = vec![];
        match self {
            Command::KEYS => {
                command.extend_from_slice(&1u32.to_le_bytes());
                command.extend_from_slice(&4u32.to_le_bytes());
                command.extend_from_slice(b"keys");
            },
            Command::GET(key) => {
                command.extend_from_slice(&2u32.to_le_bytes());
                command.extend_from_slice(&3u32.to_le_bytes());
                command.extend_from_slice(b"get");
                command.extend_from_slice(&(key.len() as u32).to_le_bytes());
                command.extend_from_slice(key.as_bytes());
            },
            Command::SET(key, value) => {
                command.extend_from_slice(&3u32.to_le_bytes());
                command.extend_from_slice(&3u32.to_le_bytes());
                command.extend_from_slice(b"set");
                command.extend_from_slice(&(key.len() as u32).to_le_bytes());
                command.extend_from_slice(key.as_bytes());
                command.extend_from_slice(&(value.len() as u32).to_le_bytes());
                command.extend_from_slice(&value);
            },
            Command::DELETE(key) => {
                command.extend_from_slice(&2u32.to_le_bytes());
                command.extend_from_slice(&3u32.to_le_bytes());
                command.extend_from_slice(b"del");
                command.extend_from_slice(&(key.len() as u32).to_le_bytes());
                command.extend_from_slice(key.as_bytes());
            },
        }
    command
    }
}
