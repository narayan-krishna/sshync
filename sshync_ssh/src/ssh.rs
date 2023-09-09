use anyhow::{anyhow, Result};
use log::{info, warn};
use ssh2::{Channel, Session};
use std::net::TcpStream;
use crate::{read_message, read_message_len_header, write_message, write_message_len};
use std::io::prelude::*;

const LAUNCH_CMD: &str = "cd /home/knara/dev/rust/sshync/ && RUST_LOG=info cargo run --bin ssh_server 2> /home/knara/dev/rust/sshync/sshync_ssh/ssh_server.log &";
const SERVER_PORT: u16 = 50051;

pub struct SshClient {
    _server_pid: Option<u32>,
    _session_channel: Option<Channel>,
    forwarding_channel: Option<Channel>,
}

impl SshClient {
    pub fn init(session: Session) -> anyhow::Result<SshClient> {
        let mut session_channel = Some(session.channel_session()?);
        let mut forwarding_channel = None;
        let mut server_pid = None;

        if let Some(session_channel) = &mut session_channel {
            info!("launching server!");
            session_channel.exec(LAUNCH_CMD)?;

            info!("checing server pid");
            let mut server_ack_pid: &mut [u8] = &mut [0; 5];
            session_channel.read_exact(&mut server_ack_pid)?;
            server_pid = String::from_utf8_lossy(&server_ack_pid)
                .parse::<u32>()
                .map(|s| Some(s))
                .unwrap_or(None);

            match server_pid.map(|pid| pid.to_string()) {
                Some(pid) => info!("Server is running at {} with pid {}", SERVER_PORT, pid),
                None => warn!("Server is running at {}, UNABLE TO DECODE PID", SERVER_PORT),
            }

            forwarding_channel =
                Some(session.channel_direct_tcpip("localhost", SERVER_PORT, None)?);
        }

        Ok(SshClient {
            _server_pid: server_pid,
            _session_channel: session_channel,
            forwarding_channel,
        })
    }

}

impl sshync_lib::Client for SshClient {
    /// Get response from the server by sending a request
    fn request(&mut self, request: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        if let Some(channel) = &mut self.forwarding_channel {
            write_message_len(channel, &request)?;
            write_message(channel, request)?;
            let response_len = read_message_len_header(channel)?;
            return Ok(read_message(channel, response_len as usize)?);
        }

        Err(anyhow!("connection not established"))
    }
}

pub struct SshServer {
    tcp_stream: TcpStream,
}

impl SshServer {
    pub fn new(tcp_stream: TcpStream) -> SshServer {
        SshServer { tcp_stream }
    }
}

impl sshync_lib::Server for SshServer {
    fn receive(&mut self) -> Result<Vec<u8>> {
        let request_len = read_message_len_header(&mut self.tcp_stream)?;
        let request = read_message(&mut self.tcp_stream, request_len as usize)?;

        Ok(request)
    }

    fn send(&mut self, response: Vec<u8>) -> Result<()> {
        write_message_len(&mut self.tcp_stream, &response)?;
        write_message(&mut self.tcp_stream, response)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
}
