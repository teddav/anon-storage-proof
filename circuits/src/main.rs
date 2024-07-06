/*
An easy-to-use implementation of the Poseidon Hash in the form of a Halo2 Chip. While the Poseidon Hash function
is already implemented in halo2_gadgets, there is no wrapper chip that makes it easy to use in other circuits.
*/

use halo2_poseidon::poseidon::{primitives as poseidon, primitives::*, Hash, Pow5Chip, Pow5Config};
use halo2_proofs::{circuit::*, dev::MockProver, halo2curves::pasta::Fp, plonk::*};
use std::marker::PhantomData;

mod hash;

#[derive(Debug, Clone)]
pub struct PoseidonConfig<const WIDTH: usize, const RATE: usize, const L: usize> {
    inputs: Vec<Column<Advice>>,
    // instance: Column<Instance>,
    pow5_config: Pow5Config<Fp, WIDTH, RATE>,
}

#[derive(Debug, Clone)]
pub struct PoseidonChip<
    S: Spec<Fp, WIDTH, RATE>,
    const WIDTH: usize,
    const RATE: usize,
    const L: usize,
> {
    config: PoseidonConfig<WIDTH, RATE, L>,
    _marker: PhantomData<S>,
}

impl<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize, const L: usize>
    PoseidonChip<S, WIDTH, RATE, L>
{
    pub fn construct(config: PoseidonConfig<WIDTH, RATE, L>) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<Fp>,
        state: Vec<Column<Advice>>,
    ) -> PoseidonConfig<WIDTH, RATE, L> {
        // let state = (0..WIDTH).map(|_| meta.advice_column()).collect::<Vec<_>>();
        let partial_sbox = meta.advice_column();
        let rc_a = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let rc_b = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        // let instance = meta.instance_column();
        // for i in 0..WIDTH {
        //     meta.enable_equality(state[i]);
        // }
        // meta.enable_equality(instance);
        meta.enable_constant(rc_b[0]);

        let pow5_config = Pow5Chip::configure::<S>(
            meta,
            state.clone().try_into().unwrap(),
            partial_sbox,
            rc_a.try_into().unwrap(),
            rc_b.try_into().unwrap(),
        );

        PoseidonConfig {
            inputs: state,
            // instance,
            pow5_config,
        }
    }

    pub fn load_private_inputs(
        &self,
        mut layouter: impl Layouter<Fp>,
        inputs: [Value<Fp>; L],
    ) -> Result<[AssignedCell<Fp, Fp>; L], Error> {
        layouter.assign_region(
            || "load private inputs",
            |mut region| -> Result<[AssignedCell<Fp, Fp>; L], Error> {
                let result = inputs
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        region.assign_advice(
                            || "private input",
                            self.config.inputs[i],
                            0,
                            || x.to_owned(),
                        )
                    })
                    .collect::<Result<Vec<AssignedCell<Fp, Fp>>, Error>>();
                Ok(result?.try_into().unwrap())
            },
        )
    }

    // pub fn expose_public(
    //     &self,
    //     mut layouter: impl Layouter<Fp>,
    //     cell: &AssignedCell<Fp, Fp>,
    //     row: usize,
    // ) -> Result<(), Error> {
    //     layouter.constrain_instance(cell.cell(), self.config.instance, row)
    // }

    pub fn hash(
        &self,
        mut layouter: impl Layouter<Fp>,
        words: &[AssignedCell<Fp, Fp>; L],
    ) -> Result<AssignedCell<Fp, Fp>, Error> {
        let pow5_chip = Pow5Chip::construct(self.config.pow5_config.clone());
        let word_cells = layouter.assign_region(
            || "load words",
            |mut region| -> Result<[AssignedCell<Fp, Fp>; L], Error> {
                let result = words
                    .iter()
                    .enumerate()
                    .map(|(i, word)| {
                        word.copy_advice(
                            || format!("word {i}"),
                            &mut region,
                            self.config.inputs[i],
                            0,
                        )
                    })
                    .collect::<Result<Vec<AssignedCell<Fp, Fp>>, Error>>();
                Ok(result?.try_into().unwrap())
            },
        )?;

        let hasher = Hash::<_, _, S, ConstantLength<L>, WIDTH, RATE>::init(
            pow5_chip,
            layouter.namespace(|| "hasher"),
        )?;
        hasher.hash(layouter.namespace(|| "hash"), word_cells)
    }
}

#[derive(Clone)]
struct MyConfig {
    instance: Column<Instance>,
    poseidon_config: PoseidonConfig<3, 2, 2>,
}

#[allow(unused)]
struct MyChip {
    config: MyConfig,
}

impl MyChip {
    #[allow(unused)]
    pub fn construct(config: MyConfig) -> Self {
        Self { config }
    }

    pub fn configure(meta: &mut ConstraintSystem<Fp>) -> MyConfig {
        let instance = meta.instance_column();
        meta.enable_equality(instance);

        let state = (0..3).map(|_| meta.advice_column()).collect::<Vec<_>>();
        for i in state.iter().take(3) {
            meta.enable_equality(*i);
        }

        let poseidon_config = PoseidonChip::<P128Pow5T3, 3, 2, 2>::configure(meta, state);

        MyConfig {
            instance,
            poseidon_config,
        }
    }
}

#[derive(Default)]
struct MyCircuit {
    a: Value<Fp>,
    b: Value<Fp>,
}

impl Circuit<Fp> for MyCircuit {
    type Config = MyConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        MyChip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl halo2_proofs::circuit::Layouter<Fp>,
    ) -> Result<(), halo2_proofs::plonk::Error> {
        let poseidon_cs = PoseidonChip::<P128Pow5T3, 3, 2, 2>::construct(config.poseidon_config);

        let message = [self.a, self.b];
        // let poseidon_chip = PoseidonChip::<S, WIDTH, RATE, L>::construct(config);
        let message_cells = poseidon_cs
            .load_private_inputs(layouter.namespace(|| "load private inputs"), message)?;
        let result = poseidon_cs.hash(layouter.namespace(|| "poseidon chip"), &message_cells)?;

        layouter.constrain_instance(result.cell(), config.instance, 0)?;
        Ok(())
    }
}

// https://github.com/tiadaxyz/zkdoc/blob/main/zkdoc_sdk/src/circuits/poseidon.rs
fn main() {
    let k = 8;

    let a = Fp::from(5);
    let b = Fp::from(7);

    let circuit = MyCircuit {
        a: Value::known(a),
        b: Value::known(b),
    };

    let message = [a, b];
    let output = poseidon::Hash::<Fp, P128Pow5T3, ConstantLength<2>, 3, 2>::init().hash(message);
    println!("output {output:?}");

    // o1js: 0x31f35e7930ad3aefd708cd0b3e2c2c83e6f777b2793ba5ae83cfe86b5cef750c
    // 0x24c8f410c930e67c21759d7402ad2f6d47092a609cb6d4d7db384bc0cfb56479
    // 0x16703346d2e7403918b9d8da871a274af175b49ac448c633ada0495091ec5272

    let prover = MockProver::run(k, &circuit, vec![vec![output]]).unwrap();
    println!("{:?}", prover.verify());

    println!("=======");
    hash::run();
}
