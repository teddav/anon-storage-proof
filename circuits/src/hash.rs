use halo2_proofs::halo2curves::pasta::Fp as Fr;
use pse_poseidon::Poseidon;

const R_F: usize = 8;
const R_P: usize = 56;

pub fn run() {
    let mut poseidon = Poseidon::<Fr, 3, 2>::new(R_F, R_P);
    let inputs = vec![Fr::from(5), Fr::from(7)];

    poseidon.update(&inputs);
    let result = poseidon.squeeze();
    println!("result: {result:?}");
}
