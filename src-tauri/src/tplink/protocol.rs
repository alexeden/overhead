use super::error::{TpError, TpResult};
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use log::*;
use std::{
    convert::TryInto,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

pub fn encrypt(plain: &str) -> Result<Vec<u8>, TpError> {
    let len = plain.len();
    let msgbytes = plain.as_bytes();
    let mut cipher = vec![];
    cipher.write_u32::<BigEndian>(len as u32)?;
    let mut key = 0xAB;
    let mut payload: Vec<u8> = Vec::with_capacity(len);

    for i in 0..len {
        payload.push(msgbytes[i] ^ key);
        key = payload[i];
    }

    for i in &payload {
        cipher.write_u8(*i).unwrap();
    }

    Ok(cipher)
}

pub fn decrypt(cipher: &mut [u8]) -> String {
    let len = cipher.len();
    let mut key = 0xAB;
    let mut next: u8;

    for item in cipher.iter_mut().take(len) {
        next = *item;
        *item ^= key;
        key = next;
    }

    String::from_utf8_lossy(cipher).into_owned()
}

pub fn send(ip: SocketAddr, msg: &str) -> Result<String, TpError> {
    debug!("Sending to IP {:?} this message: {:?}", ip, msg);
    let payload = encrypt(msg)?;
    let mut stream = TcpStream::connect(ip)?;

    stream.set_read_timeout(Some(Duration::new(5, 0)))?;
    stream.write_all(&payload)?;

    let mut resp = vec![];
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut length: Option<u32> = None;

    loop {
        if let Ok(read) = stream.read(&mut buffer) {
            if length.is_none() {
                length = Some(BigEndian::read_u32(&buffer[0..4]));
            }
            resp.extend_from_slice(&buffer[0..read]);
            let lval: u32 = length.unwrap();
            if lval > 0 && resp.len() >= (lval + 4).try_into().unwrap() || read == 0 {
                break;
            }
        }
    }
    if resp.len() < 4 {
        Err(TpError::from("response not big enough to decrypt"))
    } else {
        let result = decrypt(&mut resp.split_off(4));
        debug!("Decrypted response:\n{}", result);
        Ok(result)
    }
}

/// Check the error code of a standard command
pub(crate) fn validate_response_code(value: &serde_json::Value, pointer: &str) -> TpResult<()> {
    if let Some(err_code) = value.pointer(pointer) {
        if err_code == 0 {
            Ok(())
        } else {
            Err(TpError::from(format!("Invalid error code {}", err_code)))
        }
    } else {
        Err(TpError::from(format!("Invalid response format: {}", value)))
    }
}
