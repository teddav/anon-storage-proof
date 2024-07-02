use axiom_eth::{
    halo2_base::gates::circuit::{BaseCircuitParams, CircuitBuilderStage},
    halo2_proofs::dev::MockProver,
    halo2curves::bn256::Fr,
    rlc::circuit::RlcCircuitParams,
    storage::circuit::{EthBlockStorageCircuit, EthBlockStorageInput},
    utils::eth_circuit::create_circuit,
};
use ethers_core::types::Chain;

mod utils;
use utils::test_input;

#[tokio::main]
async fn main() {
    let input = test_input().await.expect("input");

    let block_storage_input = EthBlockStorageInput {
        block: input.block,
        block_hash: input.block_hash,
        block_number: input.block_number,
        storage: input.eth_storage_input,
        block_header: input.header_rlp,
    };
    let circuit = EthBlockStorageCircuit::<Fr>::new(block_storage_input, Chain::Gnosis);

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
    // circuit.calculate_params();

    let instances = circuit.instances();
    println!("instances {instances:?}");

    let prover = MockProver::run(20, &circuit, instances).unwrap();

    println!("{:?}", prover.verify());
}

// fn get_circuit() {
//     EthBlockStorageCircuit::from_provider(provider, block_number, address, slots, acct_pf_max_depth, storage_pf_max_depth, network)
// }
