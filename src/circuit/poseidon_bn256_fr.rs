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
const L: usize = 2;

#[derive(Debug, Clone)]
pub(super) struct PoseidonBN256FrConfig {
    pow5config: Pow5Config<Fr, WIDTH, RATE>,
}

pub(super) struct PoseidonBN256FrSynthesisOutput {
    pub(super) message: AssignedCell<Fr, Fr>,
    pub(super) output: AssignedCell<Fr, Fr>,
}

impl PoseidonBN256FrConfig {
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
        msg_val: Fr,
    ) -> Result<PoseidonBN256FrSynthesisOutput, Error> {
        let chip = Pow5Chip::construct(self.pow5config.clone());

        let mut msg_arr = [Fr::zero(); L];
        msg_arr[0] = msg_val;

        let message: [AssignedCell<Fr, Fr>; L] = layouter.assign_region(
            || "load message",
            |mut region| {
                let message_word = |i: usize| {
                    region.assign_advice(
                        || format!("load message_{}", i),
                        self.pow5config.state[i],
                        0,
                        || Value::known(msg_arr[i]),
                    )
                };

                let message: Result<Vec<_>, Error> = (0..L).map(message_word).collect();
                Ok(message?.try_into().unwrap())
            },
        )?;

        let hasher = Hash::<_, _, P128Pow5T3, ConstantLength<L>, WIDTH, RATE>::init(
            chip,
            layouter.namespace(|| "poseidon init"),
        )?;

        let output = hasher.hash(layouter.namespace(|| "do poseidon hash"), message.clone())?;

        Ok(PoseidonBN256FrSynthesisOutput {
            message: message[0].clone(),
            output,
        })
    }
}
