use crate::client::Client;
use std::io::prelude::*;

pub struct SshClient {
    session: ssh2::Session,
    server_session_channel: Option<ssh2::Channel>,
    forwarding_channel: Option<ssh2::Channel>,
}

impl SshClient {
    pub fn init_from_session(session: ssh2::Session) -> Self {
        Self {
            session,
            server_session_channel: None,
            forwarding_channel: None,
        }
    }
}

impl Client for SshClient {
    fn request(&mut self, request: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}

fn read_message_len_header<T>(buf: &mut T) -> anyhow::Result<u32>
where
    T: Read,
{
    let mut request_len_bytes: [u8; 4] = [0u8; 4];
    buf.read_exact(&mut request_len_bytes)?;
    let request_len = u32::from_be_bytes(request_len_bytes);
    log::trace!("received request length: {} bytes", request_len);

    Ok(request_len)
}

fn read_message<T>(buf: &mut T, message_len: usize) -> anyhow::Result<Vec<u8>>
where
    T: Read,
{
    let mut read_buf = vec![0u8; message_len];
    buf.read_exact(&mut read_buf)?;

    Ok(read_buf)
}

fn write_message_len<T>(buf: &mut T, message: &Vec<u8>) -> anyhow::Result<()>
where
    T: Write,
{
    buf.write(&(message.len() as u32).to_be_bytes())?;
    log::trace!("sending: {}", message.len() as u32);

    Ok(())
}

fn write_message<T>(buf: &mut T, message: Vec<u8>) -> anyhow::Result<()>
where
    T: Write,
{
    buf.write(&message)?;
    Ok(())
}
