use axiom_eth::{
    halo2_base::gates::circuit::{BaseCircuitParams, CircuitBuilderStage},
    halo2_proofs::dev::MockProver,
    halo2curves::bn256::Fr,
    providers::{block::get_block_rlp, storage::json_to_mpt_input},
    rlc::circuit::RlcCircuitParams,
    storage::circuit::{EthBlockStorageCircuit, EthBlockStorageInput},
    utils::eth_circuit::create_circuit,
    zkevm_hashes::util::eth_types::{Block, H256},
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{BlockId, BlockNumber, EIP1186ProofResponse},
};
use ethers_core::types::Chain;
use std::error::Error;

pub async fn get_proof() -> Result<(Block<H256>, EIP1186ProofResponse), Box<dyn Error>> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
    let latest = provider.get_block_number().await?;
    let block = provider.get_block(latest).await?.unwrap();
    println!("{block:#?}");

    let slot0: H256 = [0; 32].into();

    // let storage = provider
    //     .get_storage_at(
    //         "0x663F3ad617193148711d28f5334eE4Ed07016602",
    //         slot0,
    //         Some(BlockId::Number(BlockNumber::Latest)),
    //     )
    //     .await?;
    // println!("{:?}", storage);

    let proof = provider
        .get_proof(
            "0x663F3ad617193148711d28f5334eE4Ed07016602",
            vec![slot0],
            Some(BlockId::Number(BlockNumber::Latest)),
        )
        .await?;
    // println!("{proof:#?}");

    // EthBlockStorageCircuit::from_provider(
    //     &provider,
    //     0,
    //     "0x663F3ad617193148711d28f5334eE4Ed07016602".into(),
    //     slots,
    //     acct_pf_max_depth,
    //     storage_pf_max_depth,
    //     network,
    // );

    Ok((block, proof))
}

// pub async fn fetch_input(rpc: &str, safe_address: Address, msg_hash: H256) -> Result<Halo2MultisigInput> {
//     let storage_key = keccak256(&concat_bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

//     let provider = Provider::try_from(rpc)?;
//     let latest = provider.get_block_number().await?;
//     let block = provider.get_block(latest).await?.context("no such block")?;
//     let proof = provider.get_proof(safe_address, vec![storage_key.into()], Some(latest.into())).await?;

//     let storage_hash = if proof.storage_hash.is_zero() {
//         // RPC provider may give zero storage hash for empty account, but the correct storage hash should be the null root = keccak256(0x80)
//         H256::from_slice(&KECCAK_RLP_EMPTY_STRING)
//     } else {
//         proof.storage_hash
//     };
//     let block_number: u32 = block.number.unwrap().try_into().unwrap();
//     let block_hash = block.hash.expect("block hash");
//     let state_root = block.state_root;
//     let header_rlp = get_block_rlp(&block);

//     Ok(Halo2MultisigInput {
//         eth_storage_input: json_to_input(block, proof),
//         state_root,
//         storage_root: storage_hash.into(),
//         storage_key: H256::from(storage_key),
//         address: safe_address,
//         block_number,
//         block_hash,
//         header_rlp,
//     })
// }
