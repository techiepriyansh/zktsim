use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed},
};

use halo2curves::ff::PrimeField;

#[derive(Debug, Clone)]
pub(super) struct GateIoTableConfig<F: PrimeField, const G: usize> {
    pub(super) internal_enable_gate: Column<Fixed>,
    pub(super) enable_gate: Column<Advice>,
    pub(super) gate: Column<Advice>,
    pub(super) l_idx: Column<Advice>,
    pub(super) l_val: Column<Advice>,
    pub(super) r_idx: Column<Advice>,
    pub(super) r_val: Column<Advice>,
    pub(super) o_idx: Column<Advice>,
    pub(super) o_val: Column<Advice>,

    _marker: PhantomData<F>,
}

#[derive(Debug, Clone)]
pub(super) struct GateIoTableAdvice {
    pub(super) enable_gate: Column<Advice>,
    pub(super) gate: Column<Advice>,
    pub(super) l_idx: Column<Advice>,
    pub(super) l_val: Column<Advice>,
    pub(super) r_idx: Column<Advice>,
    pub(super) r_val: Column<Advice>,
    pub(super) o_idx: Column<Advice>,
    pub(super) o_val: Column<Advice>,
}

impl<F: PrimeField, const G: usize> GateIoTableConfig<F, G> {
    pub(super) fn configure(meta: &mut ConstraintSystem<F>, advice: GateIoTableAdvice) -> Self {
        let internal_enable_gate = meta.fixed_column();

        Self {
            internal_enable_gate,
            enable_gate: advice.enable_gate,
            gate: advice.gate,
            l_idx: advice.l_idx,
            l_val: advice.l_val,
            r_idx: advice.r_idx,
            r_val: advice.r_val,
            o_idx: advice.o_idx,
            o_val: advice.o_val,
            _marker: PhantomData,
        }
    }

    pub(super) fn load_fixed(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_region(
            || "load gate-i/o table fixed part",
            |mut region| {
                for offset in 0..G {
                    region.assign_fixed(
                        || format!("i_e_g[{}]", offset),
                        self.internal_enable_gate,
                        offset,
                        || Value::known(F::ONE),
                    )?;
                }

                Ok(())
            },
        )
    }
}
