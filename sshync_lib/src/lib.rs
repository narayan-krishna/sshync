//! Sshync is a libary quickly implementing delta-compression file synchronization for any client-server
//! that can send and receive bytes.

mod comms;
mod servicer;
mod sync;
mod msg;

pub use servicer::Servicer;
pub use sync::Sshync;

pub trait Client {
    fn request(&mut self, request: Vec<u8>) -> anyhow::Result<Vec<u8>>;
}

pub trait Server: Sized {
    /// receive a request from the client and send a response
    fn run(&mut self) -> anyhow::Result<()> {
        log::info!("attempting to handle connection...");
        let mut servicer = servicer::Servicer::new(self);
        servicer.handle()?;
        log::info!("finished handling connection...");
        Ok(())
    }

    fn receive(&mut self) -> anyhow::Result<Vec<u8>>;
    fn send(&mut self, response: Vec<u8>) -> anyhow::Result<()>;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Clone, Copy)]
pub struct Args {
    verbose: bool,
    quiet: bool,
    backup: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            verbose: false,
            quiet: false,
            backup: false,
        }
    }
}

impl Args {
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    pub fn backup(mut self, backup: bool) -> Self {
        self.backup = backup;
        self
    }
}
