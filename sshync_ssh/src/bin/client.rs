use anyhow::Result;
use clap::Parser;
use env_logger::{Builder, Target};
use log::info;
use sshync_lib;
use sshync_ssh::SshClient;

// arg might look like
// not_rsync knara@localhost:src/
//
// rsync ... SRC ... [USER@]HOST:DEST # synchronize a remote file with local
// rsync ... [USER@]HOST:SRC ... DEST # synchronize a local file with remote

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

    let (_src_user, _src_host, src_fp) = from_arg(args.src);
    let (dest_user, dest_host, dest_fp) = from_arg(args.dest);
    log::info!("{}, {}, {}", _src_user, _src_host, src_fp);
    log::info!("{}, {}, {}", dest_user, dest_host, dest_fp);

    info!("attempting to create ssh session");
    let mut sess: ssh2::Session = ssh2::Session::new()?;

    let tcp = std::net::TcpStream::connect(format!("{}:22", dest_host)).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_agent(&dest_user)?; // TODO: automatically determine remote username
    assert!(sess.authenticated());
    info!("session authenticated");

    let ssh_client = Box::new(SshClient::init(sess)?);
    let mut snc = sshync_lib::Sshync::init(ssh_client, None);

    snc.sync(&src_fp, &dest_fp, None)?;

    Ok(())
}

// return username, hostname, and filepath from argument
pub fn from_arg(arg: String) -> (String, String, String) {
    let items: Vec<&str> = arg.split(['@', ':']).collect();
    assert_eq!(items.len(), 3);
    (String::from(items[0]), String::from(items[1]), String::from(items[2]))
}
