use halo2_proofs::{circuit::AssignedCell, plonk::Assigned};
use halo2curves::ff::PrimeField;

#[derive(Debug, Clone)]
pub(crate) struct ACell<F: PrimeField>(pub AssignedCell<Assigned<F>, F>);
