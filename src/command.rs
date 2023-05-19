use std::collections::VecDeque;
use std::io::{ErrorKind, Error};

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

    // TODO: Clean up the unwraps, and handle bad command data gracefully.
    pub fn parse(mut cmd_str: VecDeque<Vec<u8>>) -> Result<Self, anyhow::Error> {
        let cmd = cmd_str.pop_front();
        if let Some(cmd) = cmd {
            match String::from_utf8(cmd.to_vec())?.as_str() {
                "get" => Ok(Command::GET(String::from_utf8(cmd_str.pop_front().unwrap())?.to_string())),
                "keys" => Ok(Command::KEYS),
                "del" => Ok(Command::DELETE(String::from_utf8(cmd_str.pop_front().unwrap())?.to_string())),
                "set" => Ok(Command::SET(String::from_utf8(cmd_str.pop_front().unwrap())?.to_string(), cmd_str.pop_front().unwrap())),
                _s => Err(Error::new(ErrorKind::Unsupported, format!("unsupported command: {}", _s)).into()),
            }
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "Failed to parse command").into())
        }
    }
}
