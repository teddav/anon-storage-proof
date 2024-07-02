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
use ethers_core::types::Chain;
// use halo2_base::halo2_proofs::dev::MockProver;

mod utils;
use utils::test_input;

#[tokio::main]
async fn main() {
    // let data = r#"
    // {
    //     "number": "2",
    //     "hash": "0xc63b43bdd07923e355885e663bbf13e71cf87ced46ae5ffb148fdc32b4c29e7e",
    //     "timestamp": "1719922128",
    //     "parentHash": "0xab529d9298f4b6db7ee7822eeef84d55abb3ecc2fe728deacd077936297f6180",
    //     "nonce": "0x0000000000000000",
    //     "difficulty": "0",
    //     "gasLimit": "30000000",
    //     "gasUsed": "43724",
    //     "stateRoot": "0x827050a7d4faffb4182f3fd71adb1e093ba0e527eb4cb9d3909d852ae93555e1",
    //     "receiptsRoot": "0xa24dca9baad04af628a769453229b3c30eaa257b845fd9d318e6211faadaf59e",
    //     "blobGasUsed": "0",
    //     "excessBlobGas": "0",
    //     "miner": "0x0000000000000000000000000000000000000000",
    //     "prevRandao": "0x0000000000000000000000000000000000000000000000000000000000000000",
    //     "extraData": "0x00",
    //     "baseFeePerGas": "876047475"
    // }
    // "#;
    // let block: Block<H256> = serde_json::from_str(data).unwrap();
    // println!("block {block:#?}");

    // pub struct Halo2MultisigInput {
    //     pub eth_storage_input: EthStorageInput,
    //     pub state_root: H256,
    //     pub storage_root: H256,
    //     pub storage_key: H256,
    //     pub address: H160,
    //     pub block_number: u32,
    //     pub block_hash: H256,
    //     pub header_rlp: Vec<u8>,
    // }

    let input = test_input().await.expect("input");

    // let storage_input = json_to_mpt_input(
    //     serde_json::from_str(r#"
    //     {
    //         "address": "0x8438ad1c834623cff278ab6829a248e37c2d7e3f",
    //         "balance": "0x0",
    //         "codeHash": "0xf44576296d62ef60f9ac62f6d50c94fe68205c66f8af838fe8721b2536ed2d4f",
    //         "nonce": "0x1",
    //         "storageHash": "0x943c02f2499cc8d3f202090ee5224a2253a50714ef5b111f04c2f3f1a301adb8",
    //         "accountProof": [
    //             "0xf90131a0b91a8b7a7e9d3eab90afd81da3725030742f663c6ed8c26657bf00d842a9f4aaa01689b2a5203afd9ea0a0ca3765e4a538c7176e53eac1f8307a344ffc3c6176558080a037658e942168bc0b2a5e6b7278ff4aadd9e919f7ae5768dc98b36eb3761508c1a043eb836cd6058fa897d90bddcfd23dd0f419f42df80d642bf6ba81a49610c3708080a0bbdac19951f8f0b2f270e1395755ae8e19931d9290156ac725b0ed6ab0a397b7a0d0a1bfe5b45d2d863a794f016450a4caca04f3b599e8d1652afca8b752935fd880a05410ec435061d16364193283507c4e01ea618ff1c53310fc425c5e27dd07f50b8080a08c7e065496685639a948bd8ea2af427a385f2f0e4e2ae075bf356d083db99853a0e5c557a0ce3894afeb44c37f3d24247f67dc76a174d8cacc360c1210eef60a7680",
    //             "0xf871808080a0bf54f61f44d15f14ddfc8575113a5f76981a37c1ed63edb131e20d94dfc2086b80a0aabfb1441169c3379f428df147ba34658049e31ab75bca31dcea5ea3513408a7808080a09e483f60ca0aeb12c5034571e1761a0e0f0299ba941380385139a006cc670abc80808080808080",
    //             "0xf869a020b75c2b94a85e5830c64dab57ec38404030395f4b2e7a7cb6c5b35a151455c7b846f8440180a0943c02f2499cc8d3f202090ee5224a2253a50714ef5b111f04c2f3f1a301adb8a0f44576296d62ef60f9ac62f6d50c94fe68205c66f8af838fe8721b2536ed2d4f"
    //         ],
    //         "storageProof": [ { "key": "0x0", "value": "0xc", "proof": ["0xe3a120290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e5630c"] } ]
    //     }
    //     "#).unwrap(),
    //     100,
    //     100,
    // );

    let block_storage_input = EthBlockStorageInput {
        block: input.block,// block.clone(),
        block_hash: input.block_hash,//block.hash.unwrap(),
        block_number:input.block_number ,//block.number.unwrap().try_into().unwrap(),
        storage: input.eth_storage_input,//storage_input,
        block_header: input.header_rlp,
    };
    let circuit = EthBlockStorageCircuit::<Fr>::new(block_storage_input, Chain::Gnosis);
    // println!("{circuit:?}");

    // let base_params = serde_json::from_str(
    //     r#"
    //     {
    //     "base": {
    //         "k": 20,
    //         "num_advice_per_phase": [45, 24],
    //         "num_fixed": 1,
    //         "num_lookup_advice_per_phase": [1, 1, 0],
    //         "lookup_bits": 8,
    //         "num_instance_columns": 1
    //     },
    //     "num_rlc_columns": 3
    //     }
    //     "#,
    // )
    // .unwrap();
    let base_params = BaseCircuitParams {
        k: 20,
        num_advice_per_phase: vec![45, 24],
        num_lookup_advice_per_phase: vec![1, 1, 0],
        num_fixed: 1,
        lookup_bits: Some(19),
        num_instance_columns: 1,
    };
    let rlc_params = RlcCircuitParams {
        base: base_params,
        num_rlc_columns: 3,
    };
    let mut circuit = create_circuit(CircuitBuilderStage::Mock, rlc_params, circuit);
    circuit.mock_fulfill_keccak_promises(None);
    circuit.calculate_params();
    let instances = circuit.instances();
    println!("instances {instances:?}");

    let prover = MockProver::run(20, &circuit, instances).unwrap();

    println!("{:?}", prover.verify());
}

// fn get_circuit() {
//     EthBlockStorageCircuit::from_provider(provider, block_number, address, slots, acct_pf_max_depth, storage_pf_max_depth, network)
// }
