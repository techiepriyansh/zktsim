#[derive(Default, Debug)]
pub struct BooleanCircuitGateIo {
    pub gate: u64,
    pub l_idx: u64,
    pub r_idx: u64,
    pub o_idx: u64,
}

#[derive(Default, Debug)]
pub struct BooleanCircuit {
    pub gates: Vec<BooleanCircuitGateIo>,
    pub wires: Vec<bool>,
}
