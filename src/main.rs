use zktsim::boolean_circuit::{BooleanCircuit, BooleanCircuitInstance};
use zktsim::circuit::run_zktsim;

fn u64_to_bits_le(x: u64, n: usize) -> Vec<bool> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push((x >> i) & 1 == 1);
    }
    v
}

fn bits_le_to_u64(v: &[bool]) -> u64 {
    assert!(v.len() <= 64);
    let mut x = 0;
    for i in 0..v.len() {
        if v[i] {
            x |= 1 << i;
        }
    }
    x
}

fn test_zktsim_cla() {
    let ckt = BooleanCircuit::from_netlist("examples/cla_adder_6b.zkt").unwrap();

    let a = u64_to_bits_le(31, 6);
    let b = u64_to_bits_le(17, 6);
    let c_in = u64_to_bits_le(0, 1);
    let inputs = vec![a, b, c_in].concat();

    let inst = BooleanCircuitInstance::from_ckt_and_inputs(&ckt, &inputs);

    let outputs = ckt
        .outputs
        .iter()
        .map(|&i| inst.wires[i as usize])
        .collect::<Vec<_>>();
    let s = bits_le_to_u64(&outputs[0..6]);
    let c_out = bits_le_to_u64(&outputs[6..7]);

    println!("s = {}", s);
    println!("c_out = {}", c_out);

    run_zktsim(inst);

    println!("zktsim cla works!");
}

fn test_zktsim_c6288() {
    let ckt = BooleanCircuit::from_netlist("examples/c6288.zkt").unwrap();

    let mut a = u64_to_bits_le(11, 16);
    let mut b = u64_to_bits_le(13, 16);
    a.reverse();
    b.reverse();
    let inputs = vec![a, b].concat();

    let inst = BooleanCircuitInstance::from_ckt_and_inputs(&ckt, &inputs);

    let mut outputs = ckt
        .outputs
        .iter()
        .map(|&i| inst.wires[i as usize])
        .collect::<Vec<_>>();
    outputs.reverse();
    let p = bits_le_to_u64(&outputs);
    println!("p = {}", p);

    run_zktsim(inst);

    println!("zktsim c6288 works!");
}

fn main() {
    test_zktsim_c6288();
}
