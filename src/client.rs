pub trait Client {
    fn request(&mut self, request: Vec<u8>) -> anyhow::Result<Vec<u8>>;
}
