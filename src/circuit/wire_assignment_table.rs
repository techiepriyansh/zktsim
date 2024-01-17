use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed},
};

use halo2curves::ff::PrimeField;

#[derive(Debug, Clone)]
pub(super) struct WireAssignmentTableConfig<F: PrimeField> {
    pub(super) internal_enable_wire: Column<Fixed>,
    pub(super) idx: Column<Fixed>,
    pub(super) val: Column<Advice>,

    pub(super) size: usize,

    _marker: PhantomData<F>,
}

impl<F: PrimeField> WireAssignmentTableConfig<F> {
    pub(super) fn configure(
        meta: &mut ConstraintSystem<F>,
        val: Column<Advice>,
        size: usize,
    ) -> Self {
        let internal_enable_wire = meta.fixed_column();
        let idx = meta.fixed_column();

        Self {
            internal_enable_wire,
            idx,
            val,
            size,
            _marker: PhantomData,
        }
    }

    pub(super) fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_region(
            || "load wire-assignment table",
            |mut region| {
                let mut offset = 0;
                for value in 0..self.size {
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
                region.assign_advice(
                    || format!("val[{}]", offset),
                    self.val,
                    offset,
                    || Value::known(F::ZERO),
                )?; // does this advice assignment need to be here?

                Ok(())
            },
        )
    }
}
