use std::collections::VecDeque;
use std::io::{ErrorKind, Error};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Command {
    GET(String),
    KEYS,
    SET(String, Vec<u8>, u64),
    DELETE(String)
}

impl Command {
    pub fn encode(&self) -> Vec<u8> {
        let mut command: Vec<u8> = vec![];
        match self {
            Command::KEYS => {
                // [NumV][LNV][V]
                command.extend_from_slice(&1u32.to_le_bytes());
                command.extend_from_slice(&4u32.to_le_bytes());
                command.extend_from_slice(b"keys");
            },
            Command::GET(key) => {
                // [NumV][LNV][V][LNV][V]
                command.extend_from_slice(&2u32.to_le_bytes());
                command.extend_from_slice(&3u32.to_le_bytes());
                command.extend_from_slice(b"get");
                command.extend_from_slice(&(key.len() as u32).to_le_bytes());
                command.extend_from_slice(key.as_bytes());
            },
            Command::SET(key, value, ttl) => {
                // [NumV][LNV][V][LNV][V][LNV][V]
                command.extend_from_slice(&4u32.to_le_bytes());
                command.extend_from_slice(&3u32.to_le_bytes());
                command.extend_from_slice(b"set");
                command.extend_from_slice(&(key.len() as u32).to_le_bytes());
                command.extend_from_slice(key.as_bytes());
                command.extend_from_slice(&(value.len() as u32).to_le_bytes());
                command.extend_from_slice(&value);
                command.extend_from_slice(&8u32.to_le_bytes());
                command.extend_from_slice(&ttl.to_le_bytes());
            },
            Command::DELETE(key) => {
                // [NumV][LNV][V][LNV][V]
                command.extend_from_slice(&2u32.to_le_bytes());
                command.extend_from_slice(&3u32.to_le_bytes());
                command.extend_from_slice(b"del");
                command.extend_from_slice(&(key.len() as u32).to_le_bytes());
                command.extend_from_slice(key.as_bytes());
            },
        }
    command
    }

    pub fn parse(mut cmd_str: VecDeque<Vec<u8>>) -> anyhow::Result<Self> {
        let cmd = cmd_str.pop_front();
        if let Some(cmd) = cmd {
            match String::from_utf8(cmd.to_vec())?.as_str() {
                "keys" => Ok(Command::KEYS),
                "get" => {
                    let key_bytes = cmd_str.pop_front().ok_or(Error::new(ErrorKind::UnexpectedEof, "Expected cache key after get"))?;
                    Ok(Command::GET(String::from_utf8(key_bytes)?.to_string()))
                }
                "del" => {
                    let key_bytes = cmd_str.pop_front().ok_or(Error::new(ErrorKind::UnexpectedEof, "Expected cache key after del"))?;
                    Ok(Command::DELETE(String::from_utf8(key_bytes)?.to_string()))
                },
                "set" => {
                    let key_bytes = cmd_str.pop_front()
                        .ok_or(Error::new(ErrorKind::UnexpectedEof, "Expected cache key after set"))?;
                    let value_bytes = cmd_str.pop_front()
                        .ok_or(Error::new(ErrorKind::UnexpectedEof, "Expected byte value after cache key"))?;
                    let ttl_bytes = cmd_str.pop_front()
                        .ok_or(Error::new(ErrorKind::UnexpectedEof, "Expected ttl after byte value"))?;
                    let ttl = u64::from_le_bytes(ttl_bytes.try_into()
                        .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid TTL value encountered"))?);
                    Ok(Command::SET(String::from_utf8(key_bytes)?.to_string(), value_bytes, ttl))
                }
                _s => Err(Error::new(ErrorKind::Unsupported, format!("unsupported command: {}", _s)).into()),
            }
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "Failed to parse command").into())
        }
    }
}
