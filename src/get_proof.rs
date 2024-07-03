use anyhow::Result;
use axiom_eth::{
    providers::block::get_block_rlp,
    zkevm_hashes::util::eth_types::{Block, H256},
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{BlockId, BlockNumber, EIP1186ProofResponse},
};

pub async fn get_proof() -> Result<(Block<H256>, EIP1186ProofResponse)> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
    let latest = provider.get_block_number().await?;
    let block = provider.get_block(latest).await?.unwrap();
    println!("{block:#?}");
    let block_header = get_block_rlp(&block);
    println!("{block_header:?}");

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
