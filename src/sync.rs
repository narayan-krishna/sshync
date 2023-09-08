use crate::Client;
use crate::comms::*;
use crate::Args;
use fast_rsync;
use std::fs::File;
use std::io::prelude::*;

pub struct Sshync {
    client: Box<dyn Client>,
    default_args: Option<Args>,
}

impl Sshync {
    pub fn init(client: Box<dyn Client>, default_args: Option<Args>) -> Self {
        log::info!("running shared memory client");
        Self {
            client,
            default_args,
        }
    }

    pub fn default_args(mut self, args: Args) -> Self {
        self.default_args = Some(args);
        self
    }

    pub fn sync(
        &mut self,
        src_filepath: &str,
        target_filepath: &str,
        args: Option<Args>,
    ) -> anyhow::Result<()> {
        self.verify_src_filepath(src_filepath)?;
        log::info!("Requesting signatures from server");
        let signatures: Vec<FileSignature> = self.request_signatures(target_filepath)?;
        log::info!("{:?}", signatures);
        log::info!("Requesting server patch");
        let _patch_ok = self.request_patch(src_filepath, signatures)?;
        log::info!("Requesting shutdown");
        let _shutdown_ok = self.request_shutdown()?;

        Ok(())
    }

    pub fn sync_dir(&self, args: Args) -> anyhow::Result<()> {
        todo!()
    }

    fn verify_src_filepath(&self, filepath: &str) -> anyhow::Result<()> {
        return Ok(());
        unimplemented!()
    }

    fn request_signatures(&mut self, filepath: &str) -> anyhow::Result<Vec<FileSignature>> {
        let req: SignatureRequest = SignatureRequest {
            filepaths: vec![filepath.to_string()],
        };

        let client_msg: ClientMessage = ClientMessage {
            message: client_message::Message::SignatureRequest(req),
        };

        let response_bytes: Vec<u8> = self.client.request(bincode::serialize(&client_msg)?)?;
        let response: SignatureResponse =
            bincode::deserialize::<SignatureResponse>(&response_bytes)?;

        Ok(response.signatures)
    }

    fn request_patch(
        &mut self,
        filepath: &str,
        signatures: Vec<FileSignature>,
    ) -> anyhow::Result<()> {
        let req: PatchRequest = PatchRequest {
            deltas: signatures
                .into_iter()
                .map(|fs: FileSignature| Delta {
                    filepath: fs.filepath.clone(),
                    content: Self::calculate_delta(
                        &filepath,
                        fast_rsync::Signature::deserialize(fs.content).unwrap(),
                    )
                    .unwrap(),
                })
                .collect(),
        };

        let client_msg: ClientMessage = ClientMessage {
            message: client_message::Message::PatchRequest(req),
        };

        let response_bytes: Vec<u8> = self.client.request(bincode::serialize(&client_msg)?)?;
        log::trace!("attempting to deserialize request");
        let _response: PatchResponse = bincode::deserialize::<PatchResponse>(&response_bytes)?;

        log::info!("successful patch");
        Ok(())
    }

    fn request_shutdown(&mut self) -> anyhow::Result<()> {
        let req = ShutdownRequest {};

        let client_msg = ClientMessage {
            message: client_message::Message::ShutdownRequest(req),
        };

        let response_bytes: Vec<u8> = self.client.request(bincode::serialize(&client_msg)?)?;
        let _response: ShutdownResponse =
            bincode::deserialize::<ShutdownResponse>(&response_bytes)?;

        Ok(())
    }

    /// calculate delta of a file
    fn calculate_delta(
        base_filepath: &str,
        signature: fast_rsync::Signature,
    ) -> anyhow::Result<Vec<u8>> {
        let mut delta = vec![];
        let mut file_bytes: Vec<u8> = Vec::new();

        let mut file = File::open(base_filepath.clone())?;
        file.read_to_end(&mut file_bytes)?;
        fast_rsync::diff(&signature.index(), &file_bytes, &mut delta)?;

        return Ok(delta);
    }
}
