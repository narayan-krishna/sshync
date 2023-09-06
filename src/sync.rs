use crate::client::Client;
use crate::Args;
use crate::comms;

pub struct Sshync {
    client: Box<dyn Client>,
    default_args: Option<Args>,
}

impl Sshync {
    pub fn from_shm() -> Self {
        Self {
            client: Box::new(crate::shm::ShmClient::new()),
            default_args: None,
        }
    }

    pub fn from_ssh() -> Self {
        Self {
            client: Box::new(crate::shm::ShmClient::new()),
            default_args: None,
        }
    }

    pub fn default_args(mut self, args: Args) -> Self {
        self.default_args = Some(args);
        self
    }

    pub fn sync(&mut self, src_filepath: &str, target_filepath: &str, args: Args) -> anyhow::Result<()> {
        self.verify_src_filepath(src_filepath);
        log::info!("Requesting signatures from server");
        let signatures: Vec<comms::FileSignature> = self.request_signatures(src_filepath)?;
        log::info!("Requesting server patch");
        let _patch_ok: bool = self.request_patch(src_filepath, signatures)?;
        log::info!("Requesting shutdown");
        let _shutdown_ok: bool = self.request_shutdown()?;

        Ok(())
    }

    pub fn sync_dir(&self, args: Args) -> anyhow::Result<()> {
        todo!()
    }

    fn verify_src_filepath(&self, filepath: &str) {
        todo!()
    }

    fn request_signatures(&self, filepath: &str) -> anyhow::Result<Vec<comms::FileSignature>> {
        todo!()
    }

    fn request_patch(&self, filepath: &str, signatures: Vec<comms::FileSignature>) -> anyhow::Result<bool> {
        todo!()
    }

    fn request_shutdown(&self) -> anyhow::Result<bool> {
        todo!()
    }
}

