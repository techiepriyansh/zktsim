use halo2_proofs::{
    circuit::{AssignedCell, Layouter, Region, Value},
    plonk::{
        Advice, Assigned, Column, ConstraintSystem, Error,
        Expression::{self, Constant},
        Fixed, Selector,
    },
    poly::Rotation,
};
use halo2curves::bn256::Fr;

use crate::gadgets::poseidon::{
    primitives::{ConstantLength, P128Pow5T3},
    Hash, Pow5Chip, Pow5Config,
};

const WIDTH: usize = 3;
const RATE: usize = 2;
const L: usize = 1;

#[derive(Debug, Clone)]
pub(super) struct PoseidonHashConfig {
    pow5config: Pow5Config<Fr, WIDTH, RATE>,
}

impl PoseidonHashConfig {
    pub(super) fn configure(meta: &mut ConstraintSystem<Fr>) -> Self {
        let state = (0..WIDTH).map(|_| meta.advice_column()).collect::<Vec<_>>();
        let partial_sbox = meta.advice_column();

        let rc_a = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let rc_b = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();

        meta.enable_constant(rc_b[0]);

        let pow5config = Pow5Chip::configure::<P128Pow5T3>(
            meta,
            state.try_into().unwrap(),
            partial_sbox,
            rc_a.try_into().unwrap(),
            rc_b.try_into().unwrap(),
        );

        Self { pow5config }
    }

    pub(super) fn synthesize(
        &self,
        mut layouter: impl Layouter<Fr>,
        message: Fr,
    ) -> Result<AssignedCell<Fr, Fr>, Error> {
        let chip = Pow5Chip::construct(self.pow5config.clone());

        let message = layouter.assign_region(
            || "load message for poseidon hash",
            |mut region| {
                let message = region.assign_advice(
                    || "poseidon hash message",
                    self.pow5config.state[0],
                    0,
                    || Value::known(message),
                );
                Ok(message.map(|m| [m]).unwrap())
            },
        )?;

        let hasher = Hash::<_, _, P128Pow5T3, ConstantLength<L>, WIDTH, RATE>::init(
            chip,
            layouter.namespace(|| "poseidon init"),
        )?;

        hasher.hash(layouter.namespace(|| "do poseidon hash"), message)
    }
}
