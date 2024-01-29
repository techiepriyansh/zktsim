use zktsim::boolean_circuit::{BooleanCircuit, BooleanCircuitGateIo, BooleanCircuitInstance};
use zktsim::circuit::run_zktsim;

fn test_zktsim() {
    let ckt = BooleanCircuitInstance {
        gates: vec![BooleanCircuitGateIo {
            gate: 1,
            l_idx: 0,
            r_idx: 1,
            o_idx: 2,
        }],
        wires: vec![true, true, true],
    };

    run_zktsim(ckt);

    println!("zktsim works!");
}

fn test_parse_boolean_circut() {
    let ckt = BooleanCircuit::from_netlist("examples/cla_adder_6b.zkt").unwrap();
    println!("ckt: {:?}", ckt);
}

fn main() {
    test_zktsim();
    test_parse_boolean_circut();
}
