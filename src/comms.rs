use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub content: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    pub message: client_message::Message,
}

pub mod client_message {
    #[derive(super::Serialize, super::Deserialize)]
    pub enum Message {
        SignatureRequest(super::SignatureRequest),
        PatchRequest(super::PatchRequest),
        ShutdownRequest(super::ShutdownRequest),
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignatureRequest {
    pub filepaths: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct SignatureResponse {
    pub signatures: Vec<FileSignature>
}

#[derive(Serialize, Deserialize)]
pub struct PatchRequest {
    pub deltas: Vec<Delta>
}

#[derive(Serialize, Deserialize)]
pub struct PatchResponse {
    pub ok: bool
}

#[derive(Serialize, Deserialize)]
pub struct FileSignature {
    pub filepath: String,
    pub content: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Delta {
    pub filepath: String,
    pub content: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct ShutdownRequest {}

#[derive(Serialize, Deserialize)]
pub struct ShutdownResponse {}
