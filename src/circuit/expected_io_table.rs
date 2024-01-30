use std::marker::PhantomData;

use halo2_proofs::plonk::{Column, ConstraintSystem, Instance};

use halo2curves::ff::PrimeField;

#[derive(Debug, Clone)]
pub(super) struct ExpectedIoTableConfig<F: PrimeField> {
    pub(super) enable_i_o: Column<Instance>,
    pub(super) i_o_val: Column<Instance>,

    _marker: PhantomData<F>,
}

pub(super) struct ExpectedIoTableInstance {
    pub(super) enable_i_o: Column<Instance>,
    pub(super) i_o_val: Column<Instance>,
}

impl<F: PrimeField> ExpectedIoTableConfig<F> {
    pub(super) fn configure(
        meta: &mut ConstraintSystem<F>,
        instance: ExpectedIoTableInstance,
    ) -> Self {
        Self {
            enable_i_o: instance.enable_i_o,
            i_o_val: instance.i_o_val,
            _marker: PhantomData,
        }
    }
}
