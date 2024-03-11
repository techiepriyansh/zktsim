use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, Value},
    plonk::{
        Advice, Assigned, Column, ConstraintSystem, Error,
        Expression::{self, Constant},
        Fixed,
    },
    poly::Rotation,
};

use halo2curves::ff::PrimeField;

use super::common::*;

#[derive(Debug, Clone)]
pub(super) struct Mimc7CbcCipherConfig<F: PrimeField, const N: usize> {
    pub(super) enable_cipher: Column<Fixed>,
    pub(super) k: Column<Advice>,
    pub(super) iv: Column<Advice>,
    pub(super) x_in: Column<Advice>,
    pub(super) x: [Column<Advice>; 92],
    pub(super) x_out: Column<Advice>,

    c: [F; 91],

    _marker: PhantomData<F>,
}

pub(super) struct Mimc7CbcCipherParams<F: PrimeField> {
    pub(super) enable_cipher: Column<Fixed>,
    pub(super) x_in: Column<Advice>,
    pub(super) c: [F; 91], // round constants
}

impl<F: PrimeField, const N: usize> Mimc7CbcCipherConfig<F, N> {
    pub(super) fn configure(
        meta: &mut ConstraintSystem<F>,
        params: Mimc7CbcCipherParams<F>,
    ) -> Self {
        let k = meta.advice_column();
        let iv = meta.advice_column();
        let x: [Column<Advice>; 92] = (0..92)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let x_out = meta.advice_column();

        meta.enable_equality(k);
        meta.enable_equality(iv);
        meta.enable_equality(x_out);

        meta.create_gate("MiMC7 CBC encryption round function", |meta| {
            let e_c = meta.query_fixed(params.enable_cipher, Rotation::cur());
            let k_ = meta.query_advice(k, Rotation::cur());
            let x_ = (0..92)
                .map(|i| meta.query_advice(x[i], Rotation::cur()))
                .collect::<Vec<Expression<F>>>();

            let c = params.c;

            let pow7 = |a: Expression<F>| {
                let a2 = a.clone().square();
                let a4 = a2.clone().square();
                a4 * a2 * a
            };

            (0..91)
                .map(|i| {
                    let e_c = e_c.clone();
                    let k__ = k_.clone();
                    let x_i = x_[i].clone();
                    let x_ip1 = x_[i + 1].clone();

                    e_c * (pow7(x_i + Constant(c[i]) + k__) - x_ip1)
                })
                .collect::<Vec<_>>()
        });

        meta.create_gate("MiMC7 CBC encryption cipher input", |meta| {
            let e_c = meta.query_fixed(params.enable_cipher, Rotation::cur());
            let iv_ = meta.query_advice(iv, Rotation::cur());
            let x_in_ = meta.query_advice(params.x_in, Rotation::cur());
            let x_0_ = meta.query_advice(x[0], Rotation::cur());

            vec![e_c * (x_in_ + iv_ - x_0_)]
        });

        meta.create_gate("MiMC7 CBC encryption cipher output", |meta| {
            let e_c = meta.query_fixed(params.enable_cipher, Rotation::cur());
            let k_ = meta.query_advice(k, Rotation::cur());
            let x_91_ = meta.query_advice(x[91], Rotation::cur());
            let x_out_ = meta.query_advice(x_out, Rotation::cur());

            vec![e_c * (x_91_ + k_ - x_out_)]
        });

        Self {
            enable_cipher: params.enable_cipher,
            k,
            iv,
            x_in: params.x_in,
            x,
            x_out,
            c: params.c,
            _marker: PhantomData,
        }
    }

    pub(super) fn synthesize(
        &self,
        mut layouter: impl Layouter<F>,
        x_in_vals: Vec<F>,
        k_val: F,
    ) -> Result<(), Error> {
        assert!(x_in_vals.len() <= N);

        let mut x_in_vals = x_in_vals;
        x_in_vals.extend((0..(N - x_in_vals.len())).map(|_| F::ZERO));

        let va = |v: F| Value::known(Assigned::from(v));

        let mut iv_val = F::ZERO;
        let mut prev_k_acell: Option<ACell<F>> = None;
        let mut prev_x_out_acell: Option<ACell<F>> = None;

        for (row, x_in_val) in x_in_vals.iter().enumerate() {
            let mut x_vals = [F::ZERO; 92];
            x_vals[0] = *x_in_val + iv_val;

            for i in 0..91 {
                x_vals[i + 1] = (x_vals[i] + self.c[i] + k_val).pow([7u64]);
            }

            let x_out_val = x_vals[91] + k_val;

            (prev_k_acell, prev_x_out_acell) = layouter.assign_region(
                || format!("MiMC7 CBC assignment for row {}", row),
                |mut region| {
                    let k_acell = prev_k_acell.clone().map_or(
                        Some(
                            region
                                .assign_advice(|| "k", self.k, 0, || va(k_val))
                                .map(ACell)?,
                        ),
                        |prev_k_acell_val| {
                            prev_k_acell_val
                                .0
                                .copy_advice(|| "k", &mut region, self.k, 0)
                                .map(ACell)
                                .ok()
                        },
                    );

                    match prev_x_out_acell.clone() {
                        Some(prev_x_out_acell_val) => {
                            prev_x_out_acell_val
                                .0
                                .copy_advice(|| "iv", &mut region, self.iv, 0)?;
                        }
                        None => {
                            region.assign_advice(|| "iv", self.iv, 0, || va(iv_val))?;
                        }
                    };

                    region.assign_advice(|| "x_in", self.x_in, 0, || va(*x_in_val))?;

                    for i in 0..92 {
                        region.assign_advice(
                            || format!("x_{}", i),
                            self.x[i],
                            0,
                            || va(x_vals[i]),
                        )?;
                    }

                    let x_out_acell = Some(
                        region
                            .assign_advice(|| "x_out", self.x_out, 0, || va(x_out_val))
                            .map(ACell)?,
                    );

                    Ok((k_acell, x_out_acell))
                },
            )?;

            iv_val = x_out_val;
        }

        Ok(())
    }
}

pub(super) fn Mimc7DefaultConstants<F: PrimeField>() -> [F; 91] {
    let mut c = [F::ZERO; 91];
    for i in 1..91 {
        c[i] = F::from(i as u64);
    }
    c
}
