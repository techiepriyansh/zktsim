use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

#[derive(Default, Debug)]
pub struct BooleanCircuitGateIo {
    pub gate: u64,
    pub l_idx: u64,
    pub r_idx: u64,
    pub o_idx: u64,
}

#[derive(Default, Debug)]
pub struct BooleanCircuit {
    pub inputs: Vec<u64>,
    pub outputs: Vec<u64>,
    pub gates: Vec<BooleanCircuitGateIo>,
    pub max_wire_idx: u64,
}

pub struct BooleanCircuitAssignment {
    pub wires: Vec<bool>,
}

#[derive(Default, Debug)]
pub struct BooleanCircuitInstance {
    pub gates: Vec<BooleanCircuitGateIo>,
    pub wires: Vec<bool>,
}

impl BooleanCircuit {
    pub fn from_netlist(file_name: &str) -> Result<Self, Error> {
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        let mut gates = Vec::new();
        let mut max_wire_idx: u64 = 0;

        let mut section = 0;
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }

            line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("inputs") {
                section = 1;
                continue;
            } else if line.starts_with("outputs") {
                section = 2;
                continue;
            } else if line.starts_with("wirings") {
                section = 3;
                continue;
            }

            match section {
                1 => {
                    let input_vec = line.split_ascii_whitespace().collect::<Vec<&str>>();
                    let input_idx: u64 = input_vec[0].parse().map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidData,
                            format!("invalid input index {}", input_vec[0]),
                        )
                    })?;
                    inputs.push(input_idx);
                }
                2 => {
                    let output_vec = line.split_ascii_whitespace().collect::<Vec<&str>>();
                    let output_idx: u64 = output_vec[0].parse().map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidData,
                            format!("invalid output index {}", output_vec[0]),
                        )
                    })?;
                    outputs.push(output_idx);
                }
                3 => {
                    let wiring_vec = line.split_ascii_whitespace().collect::<Vec<&str>>();

                    let gate: u64 = match wiring_vec[0] {
                        "not" => 0,
                        "and" => 1,
                        "nand" => 2,
                        "or" => 3,
                        "nor" => 4,
                        "xor" => 5,
                        "xnor" => 6,
                        _ => Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("invalid gate {}", wiring_vec[0]),
                        ))?,
                    };

                    let l_idx: u64 = wiring_vec[1].parse().map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidData,
                            format!("invalid index {}", wiring_vec[1]),
                        )
                    })?;

                    let r_idx: u64 = wiring_vec[2].parse().map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidData,
                            format!("invalid index {}", wiring_vec[2]),
                        )
                    })?;

                    let o_idx: u64 = wiring_vec[3].parse().map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidData,
                            format!("invalid index {}", wiring_vec[3]),
                        )
                    })?;

                    gates.push(BooleanCircuitGateIo {
                        gate,
                        l_idx,
                        r_idx,
                        o_idx,
                    });

                    max_wire_idx = max_wire_idx.max(l_idx).max(r_idx).max(o_idx);
                }
                _ => {}
            }
        }

        Ok(BooleanCircuit {
            inputs,
            outputs,
            gates,
            max_wire_idx,
        })
    }
}
