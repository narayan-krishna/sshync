use crate::{client::Client, server::Server, servicer::Servicer};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct ShmClient {
    server_t: Option<thread::JoinHandle<()>>,
    p_send: Option<mpsc::Sender<Vec<u8>>>,
    p_recv: Option<mpsc::Receiver<Vec<u8>>>,
}

impl ShmClient {
    pub fn init() -> ShmClient {
        let (p_send, c_recv) = mpsc::channel::<Vec<u8>>();
        let (c_send, p_recv) = mpsc::channel::<Vec<u8>>();

        let server_t = thread::spawn(move || {
            let mut local_server = ShmServer::new(c_send, c_recv);
            local_server.run().unwrap();
        });

        ShmClient {
            server_t: Some(server_t),
            p_send: Some(p_send),
            p_recv: Some(p_recv),
        }
    }
}

impl Client for ShmClient {
    fn request(&mut self, request: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        match (&self.p_send, &self.p_recv) {
            (Some(p_send), Some(p_recv)) => {
                log::trace!("client sending {}", String::from_utf8(request.to_vec())?);
                p_send.send(request)?;
                let response = p_recv.recv_timeout(Duration::from_secs(10))?;
                log::trace!(
                    "client got response: {}",
                    String::from_utf8(response.to_vec())
                        .unwrap_or("response can't be decoded".to_string())
                );

                log::trace!("returning after client got response");
                return Ok(response);
            }
            _ => {
                return {
                    let err_string: &str = "sender/receiver not initalized";
                    log::error!("{}", err_string);
                    Err(anyhow::anyhow!(err_string))
                }
            }
        }
    }
}

impl Drop for ShmClient {
    fn drop(&mut self) {
        log::info!("server dropping ShmClient, joining server thread");
        // self.server_t.take().unwrap().join().expect("failed to join server thread.");
    }
}

struct ShmServer {
    c_send: mpsc::Sender<Vec<u8>>,
    c_recv: mpsc::Receiver<Vec<u8>>,
}

impl ShmServer {
    pub fn new(c_send: mpsc::Sender<Vec<u8>>, c_recv: mpsc::Receiver<Vec<u8>>) -> ShmServer {
        ShmServer { c_send, c_recv }
    }
}

impl Server for ShmServer {
    fn run(&mut self) -> anyhow::Result<()> {
        let mut servicer = Servicer::new(self);
        servicer.handle()?;
        log::info!("finished handling connection");
        Ok(())
    }

    fn send(&mut self, response: Vec<u8>) -> anyhow::Result<()> {
        self.c_send.send(response)?;
        Ok(())
    }

    fn receive(&mut self) -> anyhow::Result<Vec<u8>> {
        Ok(self.c_recv.recv_timeout(Duration::from_secs(5))?)
    }
}
