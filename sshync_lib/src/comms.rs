use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMessage {
    pub message: client_message::Message,
}

pub mod client_message {
    #[derive(Debug, super::Serialize, super::Deserialize)]
    pub enum Message {
        SignatureRequest(super::SignatureRequest),
        PatchRequest(super::PatchRequest),
        ShutdownRequest(super::ShutdownRequest),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureRequest {
    pub filepaths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureResponse {
    pub signatures: Vec<FileSignature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchRequest {
    pub deltas: Vec<Delta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchResponse {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSignature {
    pub filepath: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    pub filepath: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShutdownRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShutdownResponse {}
