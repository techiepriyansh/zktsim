use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{AssignedCell, Layouter, Value},
    plonk::{
        Advice, Assigned, Column, ConstraintSystem, Constraints, Error, Expression, Fixed,
        Instance, Selector,
    },
    poly::Rotation,
};

use halo2curves::ff::PrimeField;

mod gate_definition_table;
use gate_definition_table::GateDefinitionTableConfig;

mod wire_assignment_table;
use wire_assignment_table::WireAssignmentTableConfig;

#[derive(Debug, Clone)]
struct ACell<F: PrimeField>(AssignedCell<Assigned<F>, F>);

#[derive(Debug, Clone)]
struct ZktSimConfig<F: PrimeField> {
    // Gate inputs and output subtable
    internal_enable_gate: Column<Fixed>,
    enable_gate: Column<Advice>,
    gate: Column<Advice>,
    l_idx: Column<Advice>,
    l_val: Column<Advice>,
    r_idx: Column<Advice>,
    r_val: Column<Advice>,
    o_idx: Column<Advice>,
    o_val: Column<Advice>,

    // Wire assignments subtable
    wire_assignment_table: WireAssignmentTableConfig<F>,

    // Gate definitions subtable
    gate_definition_table: GateDefinitionTableConfig<F>,

    // Expected input and output subtable
    enable_io: Column<Instance>,
    io: Column<Instance>,
}

#[derive(Debug, Clone)]
struct ZktSimAdvice {
    enable_gate: Column<Advice>,
    gate: Column<Advice>,
    l_idx: Column<Advice>,
    l_val: Column<Advice>,
    r_idx: Column<Advice>,
    r_val: Column<Advice>,
    o_idx: Column<Advice>,
    o_val: Column<Advice>,

    val: Column<Advice>,
}

#[derive(Debug, Clone)]
struct ZktSimInstance {
    enable_io: Column<Instance>,
    io: Column<Instance>,
}

impl<F: PrimeField> ZktSimConfig<F> {
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: ZktSimAdvice,
        instance: ZktSimInstance,
    ) -> Self {
        todo!()
    }
}
