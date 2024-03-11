use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, Value},
    plonk::{
        Advice, Column, ConstraintSystem, Error,
        Expression::{self, Constant},
        Fixed,
    },
    poly::Rotation,
};

use halo2curves::ff::PrimeField;

#[derive(Debug, Clone)]
pub(super) struct Mimc7CbcCipherConfig<F: PrimeField> {
    pub(super) enable_cipher: Column<Fixed>,
    pub(super) k: Column<Advice>,
    pub(super) iv: Column<Advice>,
    pub(super) x: [Column<Advice>; 92],

    _marker: PhantomData<F>,
}

pub(super) struct Mimc7CbcCipherParams<F: PrimeField> {
    pub(super) enable_cipher: Column<Fixed>,
    pub(super) k: F,
    pub(super) x_in: Column<Advice>,
    pub(super) c: [F; 91], // round constants
}

impl<F: PrimeField> Mimc7CbcCipherConfig<F> {
    pub(super) fn configure(
        meta: &mut ConstraintSystem<F>,
        params: Mimc7CbcCipherParams<F>,
    ) -> Self {
        let k = meta.advice_column();
        let iv = meta.advice_column();

        let mut x = vec![params.x_in];
        x.extend_from_slice(&[meta.advice_column(); 91]);
        let x: [Column<Advice>; 92] = x.try_into().unwrap();

        meta.create_gate("MiMC7 CBC encryption round function", |meta| {
            let e_c = meta.query_fixed(params.enable_cipher, Rotation::cur());
            let iv = meta.query_advice(iv, Rotation::cur());
            let k = meta.query_advice(k, Rotation::cur());

            let c = params.c;

            let pow7 = |a: Expression<F>| {
                let a2 = a.clone().square();
                let a4 = a2.clone().square();
                a4 * a2 * a
            };

            (0..91)
                .map(|i| {
                    let x_i = meta.query_advice(x[i], Rotation::cur());
                    let x_ip1 = meta.query_advice(x[i + 1], Rotation::cur());

                    let e_c = e_c.clone();
                    let iv = iv.clone();
                    let k = k.clone();

                    e_c * (pow7(x_i + iv + Constant(c[i]) + k) - x_ip1)
                })
                .collect::<Vec<_>>()
        });

        Self {
            enable_cipher: params.enable_cipher,
            k,
            iv,
            x,
            _marker: PhantomData,
        }
    }
}
