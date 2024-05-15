use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agreement {
    pub quantity: String,
    pub nonce: String,
    pub price: String,
    pub server_signature_r: String,
    pub server_signature_s: String,
    pub client_signature_r: String,
    pub client_signature_s: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldElementAgreement {
    pub quantity: FieldElement,
    pub nonce: FieldElement,
    pub price: FieldElement,
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
    pub client_signature_r: FieldElement,
    pub client_signature_s: FieldElement,
}

pub struct ProfileData {
    pub salt: FieldElement,
    pub udc_address: FieldElement,
    pub class_hash: FieldElement,
}
