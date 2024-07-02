use axiom_eth::{
    halo2_base::gates::circuit::{BaseCircuitParams, CircuitBuilderStage},
    halo2_proofs::dev::MockProver,
    halo2curves::bn256::Fr,
    rlc::circuit::RlcCircuitParams,
    storage::circuit::{EthBlockStorageCircuit, EthBlockStorageInput,EthStorageInput},
    utils::eth_circuit::create_circuit,
};
use ethers_core::types::Chain;

mod utils;
use utils::test_input;

#[tokio::main]
async fn main() {
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
        acct_pf : acc_input.eth_storage_input.acct_pf,
        acct_state: acc_input.eth_storage_input.acct_state,
        storage_pfs: acc_input.eth_storage_input.storage_pfs,
    };

  let mut account_block_storage_proof = block_storage_input.clone();
  account_block_storage_proof.storage = block_account_input;


    let storage_circuit = EthBlockStorageCircuit::<Fr>::new(block_storage_input, Chain::Gnosis);

    let account_circuit = EthBlockStorageCircuit::<Fr>::new(account_block_storage_proof, Chain::Gnosis);

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

    let mut storage_circuit = create_circuit(CircuitBuilderStage::Mock, storage_rlc_params, storage_circuit);
    storage_circuit.mock_fulfill_keccak_promises(None);
    // circuit.calculate_params();

    let instances = storage_circuit.instances();
    println!("instances {instances:?}");

    let storage_prover = MockProver::run(20, &storage_circuit, instances).unwrap();

    println!("{:?}", storage_prover.verify());

    let mut account_circuit = create_circuit(CircuitBuilderStage::Mock, acc_rlc_params, account_circuit);
    account_circuit.mock_fulfill_keccak_promises(None);


    let instances = account_circuit.instances();
    println!("instances {instances:?}");

    let account_prover = MockProver::run(20, &account_circuit, instances).unwrap();

    println!("{:?}", account_prover.verify());
}

// fn get_circuit() {
//     EthBlockStorageCircuit::from_provider(provider, block_number, address, slots, acct_pf_max_depth, storage_pf_max_depth, network)
// }
