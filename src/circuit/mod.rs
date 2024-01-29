use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    plonk::{
        Advice, Assigned, Circuit, Column, ConstraintSystem, Constraints, Error, Expression, Fixed,
        Instance, Selector,
    },
    poly::Rotation,
};

use halo2curves::ff::PrimeField;

use crate::boolean_circuit::BooleanCircuit;

mod gate_io_table;
use gate_io_table::{GateIoTableAdvice, GateIoTableConfig};

mod gate_definition_table;
use gate_definition_table::GateDefinitionTableConfig;

mod wire_assignment_table;
use wire_assignment_table::{WireAssignmentTableAdvice, WireAssignmentTableConfig};

#[derive(Debug, Clone)]
struct ACell<F: PrimeField>(AssignedCell<Assigned<F>, F>);

#[derive(Debug, Clone)]
struct ZktSimConfig<F: PrimeField, const G: usize, const W: usize> {
    gate_io_table: GateIoTableConfig<F, G>,
    wire_assignment_table: WireAssignmentTableConfig<F, W>,
    gate_definition_table: GateDefinitionTableConfig<F>,
}

impl<F: PrimeField, const G: usize, const W: usize> ZktSimConfig<F, G, W> {
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        gate_io_table_advice: GateIoTableAdvice,
        wire_assignment_table_advice: WireAssignmentTableAdvice,
    ) -> Self {
        let gio = GateIoTableConfig::configure(meta, gate_io_table_advice);
        let wa = WireAssignmentTableConfig::configure(meta, wire_assignment_table_advice);
        let gdef = GateDefinitionTableConfig::configure(meta);

        meta.lookup_any("logic gates satisfaction", |meta| {
            let i_e_g = meta.query_fixed(gio.internal_enable_gate, Rotation::cur());
            let e_g = meta.query_advice(gio.enable_gate, Rotation::cur());
            let g = meta.query_advice(gio.gate, Rotation::cur());
            let l_val = meta.query_advice(gio.l_val, Rotation::cur());
            let r_val = meta.query_advice(gio.r_val, Rotation::cur());
            let o_val = meta.query_advice(gio.o_val, Rotation::cur());

            let i_e_g_def = meta.query_fixed(gdef.internal_enable_gate_def, Rotation::cur());
            let g_def = meta.query_fixed(gdef.gate_def, Rotation::cur());
            let l_def = meta.query_fixed(gdef.l_def, Rotation::cur());
            let r_def = meta.query_fixed(gdef.r_def, Rotation::cur());
            let o_def = meta.query_fixed(gdef.o_def, Rotation::cur());

            vec![
                (i_e_g * e_g, i_e_g_def),
                (g, g_def),
                (l_val, l_def),
                (r_val, r_def),
                (o_val, o_def),
            ]
        });

        meta.lookup_any("wire assignments satisfaction L", |meta| {
            let i_e_g = meta.query_fixed(gio.internal_enable_gate, Rotation::cur());
            let e_g = meta.query_advice(gio.enable_gate, Rotation::cur());
            let l_idx = meta.query_advice(gio.l_idx, Rotation::cur());
            let l_val = meta.query_advice(gio.l_val, Rotation::cur());

            let i_e_w = meta.query_fixed(wa.internal_enable_wire, Rotation::cur());
            let idx = meta.query_fixed(wa.idx, Rotation::cur());
            let val = meta.query_advice(wa.val, Rotation::cur());

            vec![(i_e_g * e_g, i_e_w), (l_idx, idx), (l_val, val)]
        });

        meta.lookup_any("wire assignments satisfaction R", |meta| {
            let i_e_g = meta.query_fixed(gio.internal_enable_gate, Rotation::cur());
            let e_g = meta.query_advice(gio.enable_gate, Rotation::cur());
            let r_idx = meta.query_advice(gio.r_idx, Rotation::cur());
            let r_val = meta.query_advice(gio.r_val, Rotation::cur());

            let i_e_w = meta.query_fixed(wa.internal_enable_wire, Rotation::cur());
            let idx = meta.query_fixed(wa.idx, Rotation::cur());
            let val = meta.query_advice(wa.val, Rotation::cur());

            vec![(i_e_g * e_g, i_e_w), (r_idx, idx), (r_val, val)]
        });

        meta.lookup_any("wire assignments satisfaction O", |meta| {
            let i_e_g = meta.query_fixed(gio.internal_enable_gate, Rotation::cur());
            let e_g = meta.query_advice(gio.enable_gate, Rotation::cur());
            let o_idx = meta.query_advice(gio.o_idx, Rotation::cur());
            let o_val = meta.query_advice(gio.o_val, Rotation::cur());

            let i_e_w = meta.query_fixed(wa.internal_enable_wire, Rotation::cur());
            let idx = meta.query_fixed(wa.idx, Rotation::cur());
            let val = meta.query_advice(wa.val, Rotation::cur());

            vec![(i_e_g * e_g, i_e_w), (o_idx, idx), (o_val, val)]
        });

        Self {
            gate_io_table: gio,
            wire_assignment_table: wa,
            gate_definition_table: gdef,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn assign_gate(
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
                    self.gate_io_table.enable_gate,
                    0,
                    || Value::known(F::ONE),
                )?;
                region.assign_advice(|| "gate", self.gate_io_table.gate, 0, || gate)?;
                region.assign_advice(|| "l_idx", self.gate_io_table.l_idx, 0, || l_idx)?;
                region.assign_advice(|| "l_val", self.gate_io_table.l_val, 0, || l_val)?;
                region.assign_advice(|| "r_idx", self.gate_io_table.r_idx, 0, || r_idx)?;
                region.assign_advice(|| "r_val", self.gate_io_table.r_val, 0, || r_val)?;
                region.assign_advice(|| "o_idx", self.gate_io_table.o_idx, 0, || o_idx)?;
                region.assign_advice(|| "o_val", self.gate_io_table.o_val, 0, || o_val)?;

                Ok(())
            },
        )
    }

    pub fn assign_wire(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<Assigned<F>>,
    ) -> Result<ACell<F>, Error> {
        layouter.assign_region(
            || "assign wire",
            |mut region| {
                region
                    .assign_advice(|| "wire value", self.wire_assignment_table.val, 0, || value)
                    .map(ACell)
            },
        )
    }
}

#[derive(Default)]
struct ZktSimCircuit<F: PrimeField, const G: usize, const W: usize> {
    boolean_circuit: BooleanCircuit,
    _marker: PhantomData<F>,
}

impl<F: PrimeField, const G: usize, const W: usize> Circuit<F> for ZktSimCircuit<F, G, W> {
    type Config = ZktSimConfig<F, G, W>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let gate_io_table_advice = GateIoTableAdvice {
            enable_gate: meta.advice_column(),
            gate: meta.advice_column(),
            l_idx: meta.advice_column(),
            l_val: meta.advice_column(),
            r_idx: meta.advice_column(),
            r_val: meta.advice_column(),
            o_idx: meta.advice_column(),
            o_val: meta.advice_column(),
        };
        let wire_assignment_table_advice = WireAssignmentTableAdvice {
            val: meta.advice_column(),
        };

        ZktSimConfig::configure(meta, gate_io_table_advice, wire_assignment_table_advice)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        config.gate_io_table.load_fixed(&mut layouter)?;
        config.wire_assignment_table.load_fixed(&mut layouter)?;
        config.gate_definition_table.load(&mut layouter)?;

        for wire in self.boolean_circuit.wires.iter() {
            let wire_val = if *wire {
                Value::known(Assigned::from(F::ONE))
            } else {
                Value::known(Assigned::from(F::ZERO))
            };
            config.assign_wire(layouter.namespace(|| "assign wire"), wire_val)?;
        }
        // Check if we need to explicity assign the zero wire in the last row (where internal_enable_wire is zero)?

        for gate_io in self.boolean_circuit.gates.iter() {
            let va = |val: u64| Value::known(Assigned::from(F::from(val)));
            let wire_va = |idx: u64| {
                if self.boolean_circuit.wires[idx as usize] {
                    Value::known(Assigned::from(F::ONE))
                } else {
                    Value::known(Assigned::from(F::ZERO))
                }
            };

            let gate = va(gate_io.gate);
            let l_idx = va(gate_io.l_idx);
            let l_val = wire_va(gate_io.l_idx);
            let r_idx = va(gate_io.r_idx);
            let r_val = wire_va(gate_io.r_idx);
            let o_idx = va(gate_io.o_idx);
            let o_val = wire_va(gate_io.o_idx);

            config.assign_gate(
                layouter.namespace(|| "assign gate"),
                gate,
                l_idx,
                l_val,
                r_idx,
                r_val,
                o_idx,
                o_val,
            )?;
        }

        Ok(())
    }
}

pub fn run_zktsim(ckt: BooleanCircuit) {
    use halo2_proofs::dev::MockProver;
    use halo2curves::pasta::Fp;

    let k = 8;
    const G: usize = 16;
    const W: usize = 16;

    let zktsim_circuit = ZktSimCircuit::<Fp, G, W> {
        boolean_circuit: ckt,
        _marker: PhantomData,
    };

    let prover = MockProver::run(k, &zktsim_circuit, vec![]).unwrap();
    prover.assert_satisfied();
}
