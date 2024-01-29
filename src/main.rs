use zktsim::boolean_circuit::{BooleanCircuit, BooleanCircuitInstance};
use zktsim::circuit::run_zktsim;

fn test_zktsim() {
    let ckt = BooleanCircuit::from_netlist("examples/cla_adder_6b.zkt").unwrap();
    let inputs = [
        true, true, true, true, true, false, // a
        true, false, true, false, false, false, // b
        false, // c_in
    ];

    let inst = BooleanCircuitInstance::from_ckt_and_inputs(&ckt, &inputs);
    let w = &inst.wires;
    println!(
        "{} {} {} {} {} {} {}",
        u8::from(w[8]),
        u8::from(w[7]),
        u8::from(w[6]),
        u8::from(w[5]),
        u8::from(w[4]),
        u8::from(w[3]),
        u8::from(w[2]),
    );

    run_zktsim(inst);

    println!("zktsim works!");
}

fn main() {
    test_zktsim();
}
