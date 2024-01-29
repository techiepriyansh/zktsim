use zktsim::boolean_circuit::{BooleanCircuit, BooleanCircuitInstance};
use zktsim::circuit::run_zktsim;

fn test_zktsim() {
    let ckt = BooleanCircuit::from_netlist("examples/cla_adder_6b.zkt").unwrap();
    let inputs = [
        true, true, true, true, true, false, // a
        true, false, false, false, false, false, // b
        false, // c_in
    ];

    run_zktsim(BooleanCircuitInstance::from_ckt_and_inputs(&ckt, &inputs));

    println!("zktsim works!");
}

fn main() {
    test_zktsim();
}
