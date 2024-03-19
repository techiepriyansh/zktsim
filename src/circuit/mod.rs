use halo2_proofs::{
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    plonk::{
        Advice, Assigned, Circuit, Column, ConstraintSystem, Constraints, Error, Expression, Fixed,
        Instance, Selector,
    },
    poly::Rotation,
};

use ff::Field;
use halo2curves::bn256::Fr as F;

use crate::boolean_circuit::BooleanCircuitInstance;

mod common;
use common::*;

mod gate_io_table;
use gate_io_table::{GateIoTableAdvice, GateIoTableConfig};

mod gate_definition_table;
use gate_definition_table::GateDefinitionTableConfig;

mod wire_assignment_table;
use wire_assignment_table::{WireAssignmentTableAdvice, WireAssignmentTableConfig};

mod expected_io_table;
use expected_io_table::{ExpectedIoTableConfig, ExpectedIoTableInstance};

mod mimc7_cbc_cipher;
use mimc7_cbc_cipher::{Mimc7CbcCipherConfig, Mimc7CbcCipherParams, Mimc7DefaultConstants};

mod poseidon_bn256_fr;
use poseidon_bn256_fr::{PoseidonBN256FrConfig, PoseidonBN256FrSynthesisOutput};

#[derive(Debug, Clone)]
struct ZktSimConfig<const G: usize, const W: usize> {
    gate_io_table: GateIoTableConfig<F, G>,
    wire_assignment_table: WireAssignmentTableConfig<F, W>,
    gate_definition_table: GateDefinitionTableConfig<F>,
    expected_io_table: ExpectedIoTableConfig<F>,
    mimc7_cbc_cipher: Mimc7CbcCipherConfig<F, G>,
    poseidon_bn256_fr: PoseidonBN256FrConfig,
}

impl<const G: usize, const W: usize> ZktSimConfig<G, W> {
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        gate_io_table_advice: GateIoTableAdvice,
        wire_assignment_table_advice: WireAssignmentTableAdvice,
        expected_io_table_instance: ExpectedIoTableInstance,
        mimc7_cbc_cipher_params: Mimc7CbcCipherParams<F>,
    ) -> Self {
        let gio = GateIoTableConfig::configure(meta, gate_io_table_advice);
        let wa = WireAssignmentTableConfig::configure(meta, wire_assignment_table_advice);
        let gdef = GateDefinitionTableConfig::configure(meta);
        let eio = ExpectedIoTableConfig::configure(meta, expected_io_table_instance);
        let mcc = Mimc7CbcCipherConfig::<F, G>::configure(meta, mimc7_cbc_cipher_params);
        let psd = PoseidonBN256FrConfig::configure(meta);

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

        meta.create_gate("input/output constraints satisfaction", |meta| {
            let e_i_o = meta.query_instance(eio.enable_i_o, Rotation::cur());
            let i_o_val = meta.query_instance(eio.i_o_val, Rotation::cur());
            let val = meta.query_advice(wa.val, Rotation::cur());

            vec![e_i_o * (val - i_o_val)]
        });

        meta.create_gate("input encoding for circuit netlist encryption", |meta| {
            let s = meta.query_fixed(mcc.s, Rotation::cur());
            let x_in = meta.query_advice(mcc.x_in, Rotation::cur());

            let g = meta.query_advice(gio.gate, Rotation::cur());
            let l_idx = meta.query_advice(gio.l_idx, Rotation::cur());
            let r_idx = meta.query_advice(gio.r_idx, Rotation::cur());
            let o_idx = meta.query_advice(gio.o_idx, Rotation::cur());
            let limb0 = g
                + l_idx * F::from(1 << 3u64)
                + r_idx * F::from(1 << 23u64)
                + o_idx * F::from(1 << 43u64);

            let gp1 = meta.query_advice(gio.gate, Rotation(1));
            let l_idx_p1 = meta.query_advice(gio.l_idx, Rotation(1));
            let r_idx_p1 = meta.query_advice(gio.r_idx, Rotation(1));
            let o_idx_p1 = meta.query_advice(gio.o_idx, Rotation(1));
            let limb1 = gp1
                + l_idx_p1 * F::from(1 << 3u64)
                + r_idx_p1 * F::from(1 << 23u64)
                + o_idx_p1 * F::from(1 << 43u64);

            let gp2 = meta.query_advice(gio.gate, Rotation(2));
            let l_idx_p2 = meta.query_advice(gio.l_idx, Rotation(2));
            let r_idx_p2 = meta.query_advice(gio.r_idx, Rotation(2));
            let o_idx_p2 = meta.query_advice(gio.o_idx, Rotation(2));
            let limb2 = gp2
                + l_idx_p2 * F::from(1 << 3u64)
                + r_idx_p2 * F::from(1 << 23u64)
                + o_idx_p2 * F::from(1 << 43u64);

            let gp3 = meta.query_advice(gio.gate, Rotation(3));
            let l_idx_p3 = meta.query_advice(gio.l_idx, Rotation(3));
            let r_idx_p3 = meta.query_advice(gio.r_idx, Rotation(3));
            let o_idx_p3 = meta.query_advice(gio.o_idx, Rotation(3));
            let limb3 = gp3
                + l_idx_p3 * F::from(1 << 3u64)
                + r_idx_p3 * F::from(1 << 23u64)
                + o_idx_p3 * F::from(1 << 43u64);

            vec![
                s * (((limb3 * F::from(1 << 63u64) + limb2) * F::from(1 << 63u64) + limb1)
                    * F::from(1 << 63u64)
                    + limb0
                    - x_in),
            ]
        });

        Self {
            gate_io_table: gio,
            wire_assignment_table: wa,
            gate_definition_table: gdef,
            expected_io_table: eio,
            mimc7_cbc_cipher: mcc,
            poseidon_bn256_fr: psd,
        }
    }
}

#[derive(Default)]
struct ZktSimCircuit<const G: usize, const W: usize> {
    boolean_circuit_instance: BooleanCircuitInstance,
    encryption_key: F,
}

impl<const G: usize, const W: usize> Circuit<F> for ZktSimCircuit<G, W> {
    type Config = ZktSimConfig<G, W>;
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
        let expected_io_table_instance = ExpectedIoTableInstance {
            enable_i_o: meta.instance_column(),
            i_o_val: meta.instance_column(),
        };
        let mimc7_cbc_cipher_params = Mimc7CbcCipherParams {
            x_in: meta.advice_column(),
            c: Mimc7DefaultConstants(),
        };

        ZktSimConfig::configure(
            meta,
            gate_io_table_advice,
            wire_assignment_table_advice,
            expected_io_table_instance,
            mimc7_cbc_cipher_params,
        )
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        config.gate_io_table.load_fixed(&mut layouter)?;
        config.wire_assignment_table.load_fixed(&mut layouter)?;
        config.gate_definition_table.load(&mut layouter)?;

        for wire in self.boolean_circuit_instance.assn.wires.iter() {
            let wire_val = if *wire {
                Value::known(Assigned::from(F::ONE))
            } else {
                Value::known(Assigned::from(F::ZERO))
            };
            config
                .wire_assignment_table
                .assign_wire(layouter.namespace(|| "assign wire"), wire_val)?;
        }
        // Check if we need to explicity assign the zero wire in the last row (where internal_enable_wire is zero)?

        let mut x_in_quarter_vals = Vec::<F>::new();

        for gate_io in self.boolean_circuit_instance.ckt.gates.iter() {
            let va = |val: u64| Value::known(Assigned::from(F::from(val)));
            let wire_va = |idx: u64| {
                if self.boolean_circuit_instance.assn.wires[idx as usize] {
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

            config.gate_io_table.assign_gate(
                layouter.namespace(|| "assign gate"),
                gate,
                l_idx,
                l_val,
                r_idx,
                r_val,
                o_idx,
                o_val,
            )?;

            let g = F::from(gate_io.gate);
            let l_idx = F::from(gate_io.l_idx);
            let r_idx = F::from(gate_io.r_idx);
            let o_idx = F::from(gate_io.o_idx);
            x_in_quarter_vals.push(
                g + l_idx * F::from(1 << 3u64)
                    + r_idx * F::from(1 << 23u64)
                    + o_idx * F::from(1 << 43u64),
            );
        }

        let poseidon_synth_out = config.poseidon_bn256_fr.synthesize(
            layouter.namespace(|| "Poseidon hash of encryption key"),
            self.encryption_key,
        )?;

        config.mimc7_cbc_cipher.synthesize(
            layouter.namespace(|| "Circuit netlist encryption"),
            x_in_quarter_vals,
            self.encryption_key,
            poseidon_synth_out.message.cell(),
        )?;

        Ok(())
    }
}

pub fn run_mock_prover(ckt: BooleanCircuitInstance) {
    use halo2_proofs::dev::MockProver;

    #[allow(non_upper_case_globals)]
    const k: u32 = 12;
    const G: usize = 1 << (k - 1);
    const W: usize = 1 << (k - 1);

    let circuit = ZktSimCircuit::<G, W> {
        boolean_circuit_instance: ckt,
        encryption_key: F::from(1337u64),
    };

    let bckt = &circuit.boolean_circuit_instance.ckt;
    let bckt_assn = &circuit.boolean_circuit_instance.assn;

    let mut inst_enable_i_o = vec![F::zero(); bckt_assn.wires.len()];
    let mut inst_i_o_val = inst_enable_i_o.clone();

    bckt.inputs.iter().for_each(|&i| {
        inst_enable_i_o[i as usize] = F::one();
        inst_i_o_val[i as usize] = F::from(bckt_assn.wires[i as usize]);
    });
    bckt.outputs.iter().for_each(|&o| {
        inst_enable_i_o[o as usize] = F::one();
        inst_i_o_val[o as usize] = F::from(bckt_assn.wires[o as usize]);
    });

    let instance = vec![inst_enable_i_o, inst_i_o_val];

    let prover = MockProver::run(k, &circuit, instance).unwrap();
    prover.assert_satisfied();
}

pub fn run_prover_kzg(ckt: BooleanCircuitInstance) {
    use halo2_proofs::{
        plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, ProvingKey},
        poly::{
            kzg::{
                commitment::{KZGCommitmentScheme, ParamsKZG},
                multiopen::{ProverGWC, VerifierGWC},
                strategy::SingleStrategy,
            },
            Rotation,
        },
        transcript::{
            Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
        },
        SerdeFormat,
    };
    use halo2curves::bn256::{Bn256, G1Affine};
    use rand_core::OsRng;

    use std::time::Instant;

    #[allow(non_upper_case_globals)]
    const k: u32 = 12;
    const G: usize = 1 << (k - 1);
    const W: usize = 1 << (k - 1);

    let circuit = ZktSimCircuit::<G, W> {
        boolean_circuit_instance: ckt,
        encryption_key: F::from(1337u64),
    };

    let bckt = &circuit.boolean_circuit_instance.ckt;
    let bckt_assn = &circuit.boolean_circuit_instance.assn;

    let mut inst_enable_i_o = vec![F::zero(); bckt_assn.wires.len()];
    let mut inst_i_o_val = inst_enable_i_o.clone();

    bckt.inputs.iter().for_each(|&i| {
        inst_enable_i_o[i as usize] = F::one();
        inst_i_o_val[i as usize] = F::from(bckt_assn.wires[i as usize]);
    });
    bckt.outputs.iter().for_each(|&o| {
        inst_enable_i_o[o as usize] = F::one();
        inst_i_o_val[o as usize] = F::from(bckt_assn.wires[o as usize]);
    });

    println!("Creating parameters...");

    let params = ParamsKZG::<Bn256>::setup(k, OsRng);

    let vk = keygen_vk(&params, &circuit).expect("vk should not fail");
    let pk = keygen_pk(&params, vk, &circuit).expect("pk should not fail");

    let instance: &[&[F]] = &[&inst_enable_i_o, &inst_i_o_val];
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);

    println!("Generating proof...");
    let proof_start_time = Instant::now();

    create_proof::<
        KZGCommitmentScheme<Bn256>,
        ProverGWC<'_, Bn256>,
        Challenge255<G1Affine>,
        _,
        Blake2bWrite<Vec<u8>, G1Affine, Challenge255<_>>,
        _,
    >(
        &params,
        &pk,
        &[circuit],
        &[instance],
        OsRng,
        &mut transcript,
    )
    .expect("prover should not fail");
    let proof = transcript.finalize();

    println!("Proof generated!");
    let proof_end_time = Instant::now();
    println!(
        "Proof generation time: {}ms",
        proof_end_time.duration_since(proof_start_time).as_millis()
    );

    let strategy = SingleStrategy::new(&params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

    println!("Verifying proof...");
    let verification_start_time = Instant::now();

    assert!(verify_proof::<
        KZGCommitmentScheme<Bn256>,
        VerifierGWC<'_, Bn256>,
        Challenge255<G1Affine>,
        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
        SingleStrategy<'_, Bn256>,
    >(&params, pk.get_vk(), strategy, &[instance], &mut transcript)
    .is_ok());

    println!("Proof verified!");
    let verification_end_time = Instant::now();
    println!(
        "Proof verification time: {}ms",
        verification_end_time
            .duration_since(verification_start_time)
            .as_millis()
    );
}
