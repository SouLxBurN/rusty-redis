use std::sync::Arc;
use anyhow::anyhow;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Response {
    Empty, // 0
    Error(String), // 1
    String(String), // 2
    Int(i32), // 3
    Float(f32), // 4
    Array(Arc<Vec<String>>), // 5
    Data(Arc<Vec<u8>>), // 6
}

impl Response {
    pub fn deserialize(buffer: &[u8]) -> Result<Response, anyhow::Error> {
        let response_type: &[u8; 4] = &buffer[0..4].try_into()?;

        match u32::from_le_bytes(*response_type) {
            0 => Ok(Response::Empty),
            1 => { // Response::Error
                let len_buf: &[u8; 4] = &buffer[4..8].try_into()?;
                let msg_size = u32::from_le_bytes(*len_buf) as usize;
                let remaining = &buffer[8..msg_size+8];
                Ok(Response::Error(std::str::from_utf8(remaining)?.to_string()))
            },
            2 => { // Response::String
                let len_buf: &[u8; 4] = &buffer[4..8].try_into()?;
                let val_size = u32::from_le_bytes(*len_buf) as usize;
                let val_bytes = &buffer[8..8+val_size];
                Ok(Response::String(std::str::from_utf8(val_bytes)?.to_string()))
            },
            3 => { // Response::Float
                Ok(Response::Float(f32::from_le_bytes(buffer[4..8].try_into()?)))
            },
            4 => { // Response:: Int
                Ok(Response::Int(i32::from_le_bytes(buffer[4..8].try_into()?)))
            },
            5 => { // Response::Array
                let size_buf: &[u8; 4] = &buffer[4..8].try_into()?;
                let array_size = u32::from_le_bytes(*size_buf) as usize;

                let mut arr = Vec::new();
                let mut cur = 8;
                for _ in 0..array_size {
                    let len_buf = &buffer[cur..cur+4].try_into()?;
                    let val_size = u32::from_le_bytes(*len_buf) as usize;
                    let val_bytes = &buffer[cur+4..cur+4+val_size];
                    let st = std::str::from_utf8(val_bytes)?;
                    arr.push(st.to_owned());
                    cur = cur+4+val_size;
                };
                Ok(Response::Array(Arc::new(arr)))
            },
            6 => { // Response::Data
                let len_buf: &[u8; 4] = &buffer[4..8].try_into()?;
                let msg_size = u32::from_le_bytes(*len_buf) as usize;
                let data = &buffer[8..msg_size+8+4];
                Ok(Response::Data(Arc::new(data.to_vec())))
            },
            _ => Err(anyhow!(String::from("Unrecognized Response Code")))
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Response::Empty => 0u32.to_le_bytes().to_vec(),
            Response::Error(msg) => {
                let mut vv = 1u32.to_le_bytes().to_vec();
                vv.extend_from_slice(&(msg.len() as u32).to_le_bytes());
                vv.extend_from_slice(msg.as_bytes());
                vv
            },
            Response::String(value) => {
                let mut vv = 2u32.to_le_bytes().to_vec();
                vv.extend_from_slice(&(value.len() as u32).to_le_bytes());
                vv.extend_from_slice(value.as_bytes());
                vv
            },
            Response::Int(value) => {
                let mut vv = 3u32.to_le_bytes().to_vec();
                vv.extend_from_slice(&value.to_le_bytes());
                vv
            },
            Response::Float(value) => {
                let mut vv = 4u32.to_le_bytes().to_vec();
                vv.extend_from_slice(&value.to_le_bytes());
                vv
            },
            Response::Array(arr) => {
                let mut vv = 5u32.to_le_bytes().to_vec();
                vv.extend_from_slice(&(arr.len() as u32).to_le_bytes());
                let values = arr.iter().fold(Vec::new(), |mut acc, s| -> Vec<u8> {
                    acc.extend_from_slice(&(s.len() as u32).to_le_bytes());
                    acc.extend_from_slice(s.as_bytes());
                    acc
                });
                vv.extend_from_slice(&values);
                vv
            },
            Response::Data(data) => {
                let mut vv = 6u32.to_le_bytes().to_vec();
                vv.extend_from_slice(&(data.len() as u32).to_le_bytes());
                vv.extend_from_slice(data);
                vv
            },
        }
    }
}

