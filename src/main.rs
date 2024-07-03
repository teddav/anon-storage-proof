use std::fs::File;

use axiom_eth::{
    halo2_base::{
        gates::circuit::{BaseCircuitParams, CircuitBuilderStage},
        utils::fs::gen_srs,
        utils::halo2::KeygenCircuitIntent,
    },
    halo2_proofs::dev::MockProver,
    halo2curves::bn256::Fr,
    rlc::circuit::RlcCircuitParams,
    snark_verifier_sdk::CircuitExt,
    snark_verifier_sdk::{
        halo2::{aggregation::AggregationConfigParams, gen_snark_shplonk},
        SHPLONK,
    },
    storage::circuit::{EthBlockStorageCircuit, EthBlockStorageInput, EthStorageInput},
    utils::build_utils::pinning::PinnableCircuit,
    utils::{
        component::promise_loader::single::PromiseLoaderParams, eth_circuit::create_circuit,
        snark_verifier::create_universal_aggregation_circuit,
    },
};
use axiom_query::{
    components::subqueries::{
        account::circuit::CoreParamsAccountSubquery, storage::circuit::CoreParamsStorageSubquery,
    },
    keygen::shard::{
        ShardIntentAccount, ShardIntentHeader, ShardIntentResultsRoot, ShardIntentStorage,
    },
};
use ethers_core::types::Chain;

mod utils;
use utils::test_input;

#[tokio::main]
async fn main() {
    env_logger::init();
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let storage_pinning_path =
        format!("{cargo_manifest_dir}/artifacts/storage_circuit_pinning.json");
    let storage_pk_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.pk");
    let storage_vk_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.vk");
    let storage_circuit_path = format!("{cargo_manifest_dir}/artifacts/storage_circuit.shplonk");
    let account_pinning_path =
        format!("{cargo_manifest_dir}/artifacts/account_circuit_pinning.json");
    let account_pk_path = format!("{cargo_manifest_dir}/artifacts/account_circuit.pk");
    let account_vk_path = format!("{cargo_manifest_dir}/artifacts/account_circuit.vk");
    let account_circuit_path = format!("{cargo_manifest_dir}/artifacts/account_circuit.shplonk");
    let input = test_input().await.expect("input");
    let acc_input = input.clone();

    let block_storage_input = EthBlockStorageInput {
        block: input.block,
        block_hash: input.block_hash,
        block_number: input.block_number,
        storage: input.eth_storage_input,
        block_header: input.header_rlp,
    };

    let block_account_input = EthStorageInput {
        addr: acc_input.address,
        acct_pf: acc_input.eth_storage_input.acct_pf,
        acct_state: acc_input.eth_storage_input.acct_state,
        storage_pfs: acc_input.eth_storage_input.storage_pfs,
    };

    let mut account_block_storage_proof = block_storage_input.clone();
    account_block_storage_proof.storage = block_account_input;

    let storage_circuit = EthBlockStorageCircuit::<Fr>::new(block_storage_input, Chain::Gnosis);

    let account_circuit =
        EthBlockStorageCircuit::<Fr>::new(account_block_storage_proof, Chain::Gnosis);

    let base_params = BaseCircuitParams {
        k: 20,
        num_advice_per_phase: vec![45, 24],
        num_lookup_advice_per_phase: vec![1, 1, 0],
        num_fixed: 1,
        lookup_bits: Some(19),
        num_instance_columns: 1,
    };
    let storage_rlc_params = RlcCircuitParams {
        base: base_params,
        num_rlc_columns: 3,
    };

    let acc_rlc_params = storage_rlc_params.clone();

    let mut storage_circuit = create_circuit(
        CircuitBuilderStage::Mock,
        storage_rlc_params,
        storage_circuit,
    );
    storage_circuit.mock_fulfill_keccak_promises(None);
    // circuit.calculate_params();

    // let instances = storage_circuit.instances();
    // println!("instances {instances:?}");

    // let storage_prover = MockProver::run(20, &storage_circuit, instances).unwrap();

    // println!("{:?}", storage_prover.verify());

    let mut account_circuit =
        create_circuit(CircuitBuilderStage::Mock, acc_rlc_params, account_circuit);
    account_circuit.mock_fulfill_keccak_promises(None);

    // let instances = account_circuit.instances();
    // println!("instances {instances:?}");

    // let account_prover = MockProver::run(20, &account_circuit, instances).unwrap();

    // println!("{:?}", account_prover.verify());

    ////////////////////////////////////////////////WIP

    let k = 20;
    let aggr_params = AggregationConfigParams {
        degree: k,
        lookup_bits: (k - 1) as usize,
        num_advice: 19,
        num_lookup_advice: 3,
        num_fixed: 3, //USER_FIXED_COLS,
    };
    let kzg_params = gen_srs(k as u32);

    let agg_vkey_hash_indices = vec![None, None];

    //TODO
    let storage_pk = {
        let core_params = CoreParamsStorageSubquery {
            capacity: 1,
            max_trie_depth: 13,
        };
        let loader_params = (
            PromiseLoaderParams::new_for_one_shard(200),
            PromiseLoaderParams::new_for_one_shard(1),
        );
        let storage_intent = ShardIntentStorage {
            core_params: core_params.clone(),
            loader_params: loader_params.clone(),
            k: 20,
            lookup_bits: 19,
        };
        let keygen_circuit = storage_intent.build_keygen_circuit();
        let (storage_pk, storage_pinning) = keygen_circuit
            .create_pk(&kzg_params, &storage_pk_path, &storage_pinning_path)
            .expect("strg pk and pinning");
        let mut vk_file = File::create(&storage_vk_path).expect("strg vk bin file");
        storage_pk
            .get_vk()
            .write(&mut vk_file, axiom_eth::halo2_proofs::SerdeFormat::RawBytes)
            .expect("strg vk bin write");

        storage_pk
    };

    let account_pk = {
        let core_params = CoreParamsAccountSubquery {
            capacity: 1,
            max_trie_depth: 14,
        };
        let loader_params = (
            PromiseLoaderParams::new_for_one_shard(200),
            PromiseLoaderParams::new_for_one_shard(1), //132), //HEADER_CAPACITY),
        );
        let account_intent = ShardIntentAccount {
            core_params: core_params.clone(),
            loader_params: loader_params.clone(),
            k: 20,
            lookup_bits: 19,
        };
        let keygen_circuit = account_intent.build_keygen_circuit();
        let (account_pk, account_pinning) = keygen_circuit
            .create_pk(&kzg_params, &account_pk_path, &account_pinning_path)
            .expect("acnt pk and pinning");
        let mut vk_file = File::create(&account_vk_path).expect("acnt vk bin file");
        account_pk
            .get_vk()
            .write(&mut vk_file, axiom_eth::halo2_proofs::SerdeFormat::RawBytes)
            .expect("acnt vk bin write");

        account_pk
    };

    log::info!("✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞ generating storage snark");
    let snark_storage = gen_snark_shplonk(
        &kzg_params,
        &storage_pk,
        storage_circuit,
        Some(&storage_circuit_path),
    );
    log::info!("✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞ generating account snark");
    let snark_account = gen_snark_shplonk(
        &kzg_params,
        &account_pk,
        account_circuit,
        Some(&account_circuit_path),
    );
    let snarks = vec![snark_storage, snark_account];

    let (mut aggr_circuit, previous_instances, agg_vkey_hash) =
        create_universal_aggregation_circuit::<SHPLONK>(
            CircuitBuilderStage::Prover,
            aggr_params,
            &kzg_params,
            snarks,
            agg_vkey_hash_indices,
        );

    let aggr_instances = aggr_circuit.instances();
    // println!("instances {aggr_instances:?}");
    log::info!("✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞ instances {:?}", aggr_instances);

    let aggr_prover = MockProver::run(20, &aggr_circuit, aggr_instances).unwrap();

    // println!("{:?}", aggr_prover.verify());
    log::info!("✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞✞ {:?}", aggr_prover.verify());
}

// fn get_circuit() {
//     EthBlockStorageCircuit::from_provider(provider, block_number, address, slots, acct_pf_max_depth, storage_pf_max_depth, network)
// }
