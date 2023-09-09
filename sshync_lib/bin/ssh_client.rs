use anyhow::Result;
use clap::Parser;
use env_logger::{Builder, Target};
use log::info;
use sshync::{Client, Sshync};
use std::net::TcpStream;

// arg might look like
// not_rsync knara@localhost:src/
//
// rsync ... SRC ... [USER@]HOST:DEST # synchronize a remote file with local
// rsync ... [USER@]HOST:SRC ... DEST # synchronize a local file with remote
//
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    src: String,
    #[arg(short, long)]
    dest: String,
    #[arg(long)]
    ssh: bool,
    #[arg(short, long)]
    verbose: bool,
}

/// Runs the not_rsync client for syncing a file to a server.
fn main() -> Result<()> {
    // parse command line args
    let args = Args::parse();

    let mut builder = Builder::from_default_env();
    builder
        .target(if args.verbose {
            Target::Stdout
        } else {
            Target::Stderr
        })
        .init();

    info!("starting client!");
    // check if source or path is dir
    // can't sync a file to a directionary
    // can sync a file a file to a file
    // can sync a directory to a directory

    Ok(())
}

const LAUNCH_CMD: &str = "cd /home/knara/dev/rust/not-rsync/ && RUST_LOG=info cargo run --bin remote_server 2> /home/knara/dev/rust/not-rsync/logs/remote_server.txt &";
const SERVER_PORT: u16 = 50051;

pub struct RemoteClient {
    session_channel: Option<ssh2::Channel>,
    forwarding_channel: Option<ssh2::Channel>,
    server_pid: Option<u32>,
    username: String,
    hostname: String,
}

impl RemoteClient {
    pub fn new(username: String, hostname: String) -> RemoteClient {
        let sess = self.start_ssh_session()?;
        info!("launching server!");
        self.session_channel = Some(sess.channel_session()?);

        if let Some(session_channel) = &mut self.session_channel {
            session_channel.exec(LAUNCH_CMD)?;

            info!("checing server pid");
            let mut server_ack_pid: &mut [u8] = &mut [0; 5];
            session_channel.read_exact(&mut server_ack_pid)?;
            let server_pid = String::from_utf8_lossy(&server_ack_pid)
                .parse::<u32>()
                .map(|s| Some(s))
                .unwrap_or(None);

            self.server_pid = server_pid;
            match self.server_pid.map(|pid| pid.to_string()) {
                Some(pid) => info!("Server is running at {} with pid {}", SERVER_PORT, pid),
                None => warn!("Server is running at {}, UNABLE TO DECODE PID", SERVER_PORT),
            }

            self.forwarding_channel =
                Some(sess.channel_direct_tcpip("localhost", SERVER_PORT, None)?);
        }

        RemoteClient {
            session_channel: None,
            forwarding_channel: None,
            server_pid: None,
            username,
            hostname,
        }
    }

    fn start_ssh_session(&self) -> Result<ssh2::Session> {
        info!("attempting to start ssh session");
        let mut sess: ssh2::Session = ssh2::Session::new()?;

        let tcp = TcpStream::connect(format!("{}:22", self.hostname)).unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        sess.userauth_agent(&self.username)?; // TODO: automatically determine remote username
        assert!(sess.authenticated());
        info!("session authenticated");

        Ok(sess)
    }
}

impl Client for RemoteClient {
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

