pub trait Server {
    /// receive a request from the client and send a response
    fn run(&mut self) -> anyhow::Result<()>;
    fn receive(&mut self) -> anyhow::Result<Vec<u8>>;
    fn send(&mut self, response: Vec<u8>) -> anyhow::Result<()>;
}
