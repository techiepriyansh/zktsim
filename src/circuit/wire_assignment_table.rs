use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, Value},
    plonk::{Advice, Assigned, Column, ConstraintSystem, Error, Fixed},
};

use halo2curves::ff::PrimeField;

use super::common::*;

#[derive(Debug, Clone)]
pub(super) struct WireAssignmentTableConfig<F: PrimeField, const W: usize> {
    pub(super) internal_enable_wire: Column<Fixed>,
    pub(super) idx: Column<Fixed>,
    pub(super) val: Column<Advice>,

    _marker: PhantomData<F>,
}

pub(super) struct WireAssignmentTableAdvice {
    pub(super) val: Column<Advice>,
}

impl<F: PrimeField, const W: usize> WireAssignmentTableConfig<F, W> {
    pub(super) fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: WireAssignmentTableAdvice,
    ) -> Self {
        let internal_enable_wire = meta.fixed_column();
        let idx = meta.fixed_column();

        Self {
            internal_enable_wire,
            idx,
            val: advice.val,
            _marker: PhantomData,
        }
    }

    pub(super) fn load_fixed(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_region(
            || "load wire-assignment table fixed part",
            |mut region| {
                let mut offset = 0;
                for value in 0..W {
                    region.assign_fixed(
                        || format!("i_e_w[{}]", offset),
                        self.internal_enable_wire,
                        offset,
                        || Value::known(F::ONE),
                    )?;
                    region.assign_fixed(
                        || format!("idx[{}]", offset),
                        self.idx,
                        offset,
                        || Value::known(F::from(value as u64)),
                    )?;
                    offset += 1;
                }

                // For when internal_enable_wire is disabled
                region.assign_fixed(
                    || format!("i_e_w[{}]", offset),
                    self.internal_enable_wire,
                    offset,
                    || Value::known(F::ZERO),
                )?;
                region.assign_fixed(
                    || format!("idx[{}]", offset),
                    self.idx,
                    offset,
                    || Value::known(F::ZERO),
                )?;

                Ok(())
            },
        )
    }

    pub(super) fn assign_wire(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<Assigned<F>>,
    ) -> Result<ACell<F>, Error> {
        layouter.assign_region(
            || "assign wire",
            |mut region| {
                region
                    .assign_advice(|| "wire value", self.val, 0, || value)
                    .map(ACell)
            },
        )
    }
}
