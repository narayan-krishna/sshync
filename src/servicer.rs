//! Utils for the server, whether on a remote machine (SSH), or an adjacent thread for local transport.

use crate::server::Server;
use crate::comms::*;
use anyhow::Result;
use fast_rsync::{apply, Signature, SignatureOptions};
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, SeekFrom};
use log::{error, info};

pub struct Servicer<'a, T>
where
    T: Server,
{
    server: &'a mut T,
}

impl<'a, T> Servicer<'a, T>
where
    T: Server,
{
    pub fn new(server: &mut T) -> Servicer<T> {
        Servicer { server }
    }

    pub fn handle(&mut self) -> Result<()> {
        info!("proceeding to handler");
        let mut shutdown: bool = false;
        while !shutdown {
            let req: ClientMessage = bincode::deserialize(self.server.receive()?.as_slice())?;

            match req.message {
                client_message::Message::SignatureRequest(req) => {
                    info!("signature request");
                    let res = self.get_file_signatures(req)?;
                    self.server.send(bincode::serialize(&res)?)?;
                }
                client_message::Message::PatchRequest(req) => {
                    info!("patch request");
                    let res = self.patch_files(req)?;
                    self.server.send(bincode::serialize(&res)?)?;
                }
                client_message::Message::ShutdownRequest(_) => {
                    info!("shutdown request");
                    shutdown = true;
                    self.server
                        .send(bincode::serialize(&ShutdownRequest{})?)?;
                }
            };
        }

        Ok(())
    }

    fn get_file_signatures(&self, req: SignatureRequest) -> Result<SignatureResponse> {
        let mut res = SignatureResponse {
            signatures: vec![]
        };

        // parallelize
        for fp in req.filepaths.into_iter() {
            info!("calculating signature for {}", fp);
            let mut sig: FileSignature = FileSignature {
                filepath: fp.clone(),
                content: Self::calculate_signature(fp, None)?.serialized().to_vec()
            };
            res.signatures.push(sig);
        }

        Ok(res)
    }

    // make this atomic
    fn patch_files(&self, req: PatchRequest) -> Result<PatchResponse> {
        for delta in req.deltas.into_iter() {
            Self::patch(delta.filepath, delta.content)?;
        }

        return Ok(PatchResponse { ok: true });
    }

    fn calculate_signature(
        filepath: String,
        sig_opts: Option<SignatureOptions>,
    ) -> Result<Signature> {
        let sig_opts = sig_opts.unwrap_or_else(|| SignatureOptions {
            block_size: 4,
            crypto_hash_size: 8,
        });

        let mut file_bytes: Vec<u8> = Vec::new();
        let mut file = File::open(filepath)?;
        if let Err(e) = file.read_to_end(&mut file_bytes) {
            file_bytes.clear();
            error!("{}", e);
        }

        return Ok(Signature::calculate(&file_bytes, sig_opts));
    }

    fn patch(filepath: String, delta: Vec<u8>) -> Result<()> {
        let mut patched_out = vec![];

        info!("attempting to patch {} with {:?}", filepath, delta);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(filepath)?;
        let mut file_bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut file_bytes)?;
        apply(&file_bytes, &delta, &mut patched_out)?;

        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)
            .unwrap_or_else(|e| error!("some whack error{e}"));
        file.write_all(&patched_out)
            .unwrap_or_else(|e| error!("couldn't write to file: {e}"));

        Ok(())
    }
}

#[cfg(test)]
mod servicer_tests {
    // TODO: add servicer tests
}
