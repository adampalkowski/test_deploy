use crate::models::{to_field_elements_agreement, DeployResult, InputData, ProfileData};
use clap::Parser;
use starknet::accounts::ExecutionEncoding;
use starknet::contract::ContractFactory;

use crate::errors::RunnerError;
use starknet::core::types::InvokeTransactionResult;
use starknet::providers::Provider;
use starknet::signers::Signer;
use starknet::{
    accounts::{Account, Call, SingleOwnerAccount},
    core::types::contract::{CompiledClass, SierraClass},
    core::types::{BlockId, BlockTag, FieldElement},
    macros::selector,
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
    signers::{LocalWallet, SigningKey},
};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use url::Url;
mod errors;
mod models;
use starknet::macros::felt;

pub const SIERRA_STR: &str = include_str!("../../agreement_version_2/target/dev/agreement_version_2_AgreementVersion2.compiled_contract_class.json");
// We can store only the class_hash and thus te casm_str would not be needed but for now it is
pub const CASM_STR: &str = include_str!(
    "../../agreement_version_2/target/dev/agreement_version_2_AgreementVersion2.contract_class.json"
);

#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let address = felt!("0x4ad77233a32945d633558939989ca6abcc87a51ccc9d22587528f937c0956cd");
    let class_hash = felt!("0x26c4d6961674f8c33c55d2f7c9e78c32d00e73552bd0c1df8652db0b42bdd9c");
    let salt = felt!("0x679ca888a3102b4");
    let udc_address = felt!("0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF");
    let chain_id = felt!("0x534e5f5345504f4c4941");
    let private_key = felt!("0x1c39c193193ee90f688703409a1a0a0f63d933c771cac9230e75ce4dad21ab7");
    let client_public_key =
        felt!("0xe5f5c0f64f7d753a3094d012a62d714f0fe3ca320df466cee03bf393d352f");

    let server_public_key =
        felt!("0x70bf7cc40c6ea06a861742fa98c2a22e077672a1dd9ed2aa025ec2f8258a2e5");
    let parsed = Url::parse("http://localhost:5050/rpc")?;

    let profile_data = ProfileData {
        salt,
        udc_address,
        class_hash,
    };

    let prefunded_account = get_account(parsed, chain_id, address, private_key);

    let deployment_result = deploy_contract(
        prefunded_account,
        client_public_key,
        server_public_key,
        profile_data,
    )
    .await?;

    Ok(())
}

pub async fn declare_contract<P, S>(
    prefunded_account: &SingleOwnerAccount<P, S>,
) -> Result<FieldElement, RunnerError>
where
    P: Provider + Send + Sync,
    S: Signer + Send + Sync,
{
    let contract_artifact: SierraClass = serde_json::from_str(SIERRA_STR).unwrap();
    let compiled_class: CompiledClass = serde_json::from_str(CASM_STR).unwrap();
    let casm_class_hash = compiled_class.class_hash()?;
    let flattened_class = contract_artifact.clone().flatten()?;

    let result = prefunded_account
        .declare(Arc::new(flattened_class), casm_class_hash)
        .send()
        .await
        .unwrap();
    Ok(result.class_hash)
}

pub async fn deploy_contract(
    prefunded_account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    client_public_key: FieldElement,
    server_public_key: FieldElement,
    profile_data: ProfileData,
) -> Result<DeployResult, RunnerError> {
    let contract_factory: ContractFactory<
        SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    > = ContractFactory::new_with_udc(
        profile_data.class_hash,
        prefunded_account,
        profile_data.udc_address,
    );

    let deployment = contract_factory.deploy(vec![], profile_data.salt, true);
    let deployed_address = deployment.deployed_address();

    let InvokeTransactionResult { transaction_hash } =
        deployment.send().await.expect("Unable to deploy contract");
    let result = DeployResult {
        deployed_address,
        transaction_hash,
    };
    Ok(result)
}

pub fn read_data() -> Result<InputData, RunnerError> {
    let mut file = File::open("resources/json_generator_out/in.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: InputData = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn get_account(
    rpc_url: Url,
    chain_id: FieldElement,
    address: FieldElement,
    private_key: FieldElement,
) -> SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> {
    let provider = JsonRpcClient::new(HttpTransport::new(rpc_url));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id,
        ExecutionEncoding::Legacy,
    );
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    account
}

pub struct AgreementConstructor {
    pub client_balance: FieldElement,
    pub server_balance: FieldElement,
    pub client_public_key: FieldElement,
    pub server_public_key: FieldElement,
    pub a: FieldElement,
    pub b: FieldElement,
}
