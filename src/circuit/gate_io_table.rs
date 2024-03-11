use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, Value},
    plonk::{Advice, Assigned, Column, ConstraintSystem, Error, Fixed},
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

    #[allow(clippy::too_many_arguments)]
    pub(super) fn assign_gate(
        &self,
        mut layouter: impl Layouter<F>,
        gate: Value<Assigned<F>>,
        l_idx: Value<Assigned<F>>,
        l_val: Value<Assigned<F>>,
        r_idx: Value<Assigned<F>>,
        r_val: Value<Assigned<F>>,
        o_idx: Value<Assigned<F>>,
        o_val: Value<Assigned<F>>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "assign gate",
            |mut region| {
                region.assign_advice(
                    || "enable_gate",
                    self.enable_gate,
                    0,
                    || Value::known(F::ONE),
                )?;
                region.assign_advice(|| "gate", self.gate, 0, || gate)?;
                region.assign_advice(|| "l_idx", self.l_idx, 0, || l_idx)?;
                region.assign_advice(|| "l_val", self.l_val, 0, || l_val)?;
                region.assign_advice(|| "r_idx", self.r_idx, 0, || r_idx)?;
                region.assign_advice(|| "r_val", self.r_val, 0, || r_val)?;
                region.assign_advice(|| "o_idx", self.o_idx, 0, || o_idx)?;
                region.assign_advice(|| "o_val", self.o_val, 0, || o_val)?;

                Ok(())
            },
        )
    }
}
