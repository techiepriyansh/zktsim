use zktsim::circuit::{BooleanCircuit, BooleanCircuitGateIo, run_zktsim};

fn main() {
    let ckt = BooleanCircuit {
        gates: vec![
            BooleanCircuitGateIo {
                gate: 0,
                l_idx: 0,
                r_idx: 1,
                o_idx: 2,
            },
        ],
        wires: vec![true, true, true],
    };

    run_zktsim(ckt);

    println!("Works!");
}
