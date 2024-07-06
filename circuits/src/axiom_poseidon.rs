use halo2_base::{
    gates::{circuit::builder::BaseCircuitBuilder, GateChip, RangeChip},
    halo2_proofs::{
        circuit::{SimpleFloorPlanner, Value},
        halo2curves::bn256::Fr as Fp,
        plonk::Circuit,
    },
    poseidon::{
        hasher::{spec::OptimizedPoseidonSpec, PoseidonHasher},
        PoseidonChip, PoseidonInstructions,
    },
    safe_types::{FixLenBytes, SafeByte},
    utils::BigPrimeField,
    AssignedValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
struct MyConfig;

#[derive(Debug, Clone, Default)]
struct MyParams;

#[derive(Debug, Clone, Default)]
struct MyCircuit;

impl Circuit<Fp> for MyCircuit {
    type Config = MyConfig;
    type Params = MyParams;
    type FloorPlanner = SimpleFloorPlanner;

    fn configure(meta: &mut halo2_base::halo2_proofs::plonk::ConstraintSystem<Fp>) -> Self::Config {
        MyConfig
    }

    fn configure_with_params(
        meta: &mut halo2_base::halo2_proofs::plonk::ConstraintSystem<Fp>,
        _params: Self::Params,
    ) -> Self::Config {
        MyConfig
    }

    fn params(&self) -> Self::Params {
        MyParams
    }

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn synthesize(
        &self,
        config: Self::Config,
        layouter: impl halo2_base::halo2_proofs::circuit::Layouter<Fp>,
    ) -> Result<(), halo2_base::halo2_proofs::plonk::Error> {
        Ok(())
    }
}

const T: usize = 3;
const RATE: usize = 2;
const R_F: usize = 8;
const R_P: usize = 57;

// https://github.com/axiom-crypto/pse-poseidon/blob/main/src/poseidon.rs
// https://github.com/privacy-scaling-explorations/snark-verifier/blob/main/snark-verifier-sdk/src/halo2.rs#L72
// https://github.com/axiom-crypto/halo2-scaffold/blob/main/examples/poseidon.rs#L10

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub inputs: [String; 2], // two field elements, but as strings for easier deserialization
}

fn hash_two<F: BigPrimeField>(
    builder: &mut BaseCircuitBuilder<F>,
    inp: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    // `Context` can roughly be thought of as a single-threaded execution trace of a program we want to ZK prove. We do some post-processing on `Context` to optimally divide the execution trace into multiple columns in a PLONKish arithmetization
    let ctx = builder.main(0);
    // More advanced usage with multi-threaded witness generation is possible, but we do not explain it here

    // first we load a private input `x` (let's not worry about public inputs for now)
    let [x, y] = inp
        .inputs
        .map(|x| ctx.load_witness(F::from_str_vartime(&x).unwrap()));
    make_public.extend([x, y]);

    // create a Gate chip that contains methods for basic arithmetic operations
    let gate = GateChip::<F>::default();
    let mut poseidon =
        PoseidonHasher::<F, T, RATE>::new(OptimizedPoseidonSpec::new::<R_F, R_P, 0>());
    poseidon.initialize_consts(ctx, &gate);
    let hash = poseidon.hash_fix_len_array(ctx, &gate, &[x, y]);
    make_public.push(hash);
    println!(
        "x: {:?}, y: {:?}, poseidon(x): {:?}",
        x.value(),
        y.value(),
        hash.value()
    );
}

fn main() {
    let mut builder = BaseCircuitBuilder::<Fp>::new(false);
    let inputs = CircuitInput {
        inputs: ["5".into(), "7".into()],
    };
    let mut make_public = vec![];
    hash_two(&mut builder, inputs, &mut make_public);

    // let mut builder = BaseCircuitBuilder::<Fp>::new(false);
    // let range_chip = builder.range_chip();
    // let ctx = builder.main(0);

    // let spec = OptimizedPoseidonSpec::<Fp, 5, 4>::new::<R_F, R_P, 0>();
    // let chip = PoseidonChip::new(ctx, spec, &range_chip);

    // let input = [0; 32];

    // // FixLenBytes::new(bytes)

    // let f = Value::known(Fp::one());
    // let b = SafeByte(f);
    // // chip.hash_fix_len_bytes(ctx, input.into());
}
