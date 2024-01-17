use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, Value, Region},
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed},
};

use halo2curves::ff::PrimeField;

#[derive(Debug, Clone)]
pub(super) struct GateDefinitionTableConfig<F: PrimeField> {
    pub(super) internal_enable_gate_def: Column<Fixed>,
    pub(super) gate_def: Column<Fixed>,
    pub(super) l_def: Column<Fixed>,
    pub(super) r_def: Column<Fixed>,
    pub(super) o_def: Column<Fixed>,

    _marker: PhantomData<F>,
}

impl<F: PrimeField> GateDefinitionTableConfig<F> {
    pub(super) fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let internal_enable_gate_def = meta.fixed_column();
        let gate_def = meta.fixed_column();
        let l_def = meta.fixed_column();
        let r_def = meta.fixed_column();
        let o_def = meta.fixed_column();

        Self {
            internal_enable_gate_def,
            gate_def,
            l_def,
            r_def,
            o_def,
            _marker: PhantomData,
        }
    }

    pub(super) fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_region(
            || "load gate-definition table",
            |mut region| {
                let mut offset = 0;

                // AND gate
                let and_gate = 0;
                self.declare_gate_io(&mut region, &mut offset, and_gate, 0, 0, 0)?;
                self.declare_gate_io(&mut region, &mut offset, and_gate, 0, 1, 0)?;
                self.declare_gate_io(&mut region, &mut offset, and_gate, 1, 0, 0)?;
                self.declare_gate_io(&mut region, &mut offset, and_gate, 1, 1, 1)?;

                // OR gate
                let or_gate = 1;
                self.declare_gate_io(&mut region, &mut offset, or_gate, 0, 0, 0)?;
                self.declare_gate_io(&mut region, &mut offset, or_gate, 0, 1, 1)?;
                self.declare_gate_io(&mut region, &mut offset, or_gate, 1, 0, 1)?;
                self.declare_gate_io(&mut region, &mut offset, or_gate, 1, 1, 1)?;

                // NOT gate (right input is ignored; set it same as left input)
                let not_gate = 2;
                self.declare_gate_io(&mut region, &mut offset, not_gate, 0, 0, 1)?;
                self.declare_gate_io(&mut region, &mut offset, not_gate, 0, 1, 1)?;
                self.declare_gate_io(&mut region, &mut offset, not_gate, 1, 0, 0)?;
                self.declare_gate_io(&mut region, &mut offset, not_gate, 1, 1, 0)?;

                // XOR gate
                let xor_gate = 3;
                self.declare_gate_io(&mut region, &mut offset, xor_gate, 0, 0, 0)?;
                self.declare_gate_io(&mut region, &mut offset, xor_gate, 0, 1, 1)?;
                self.declare_gate_io(&mut region, &mut offset, xor_gate, 1, 0, 1)?;
                self.declare_gate_io(&mut region, &mut offset, xor_gate, 1, 1, 0)?;

                // NAND gate
                let nand_gate = 4;
                self.declare_gate_io(&mut region, &mut offset, nand_gate, 0, 0, 1)?;
                self.declare_gate_io(&mut region, &mut offset, nand_gate, 0, 1, 1)?;
                self.declare_gate_io(&mut region, &mut offset, nand_gate, 1, 0, 1)?;
                self.declare_gate_io(&mut region, &mut offset, nand_gate, 1, 1, 0)?;

                // NOR gate
                let nor_gate = 5;
                self.declare_gate_io(&mut region, &mut offset, nor_gate, 0, 0, 1)?;
                self.declare_gate_io(&mut region, &mut offset, nor_gate, 0, 1, 0)?;
                self.declare_gate_io(&mut region, &mut offset, nor_gate, 1, 0, 0)?;
                self.declare_gate_io(&mut region, &mut offset, nor_gate, 1, 1, 0)?;

                // For when internal_enable_gate_def is disabled
                self.load_zero_row(&mut region, offset)?;

                Ok(())
            },
        )
    }

    fn declare_gate_io(
        &self,
        region: &mut Region<F>,
        offset: &mut usize,
        gate: u64,
        l: u64,
        r: u64,
        o: u64,
    ) -> Result<(), Error> {
        region.assign_fixed(
            || format!("i_e_g_def[{}]", *offset),
            self.internal_enable_gate_def,
            *offset,
            || Value::known(F::ONE),
        )?;
        region.assign_fixed(
            || format!("g_def[{}]", *offset),
            self.gate_def,
            *offset,
            || Value::known(F::from(gate)),
        )?;
        region.assign_fixed(
            || format!("l_def[{}]", *offset),
            self.l_def,
            *offset,
            || Value::known(F::from(l)),
        )?;
        region.assign_fixed(
            || format!("r_def[{}]", *offset),
            self.r_def,
            *offset,
            || Value::known(F::from(r)),
        )?;
        region.assign_fixed(
            || format!("o_def[{}]", *offset),
            self.o_def,
            *offset,
            || Value::known(F::from(o)),
        )?;

        *offset += 1;

        Ok(())
    }

    fn load_zero_row(&self, region: &mut Region<F>, offset: usize) -> Result<(), Error> {
        region.assign_fixed(
            || format!("i_e_g_def[{}]", offset),
            self.internal_enable_gate_def,
            offset,
            || Value::known(F::ZERO),
        )?;
        region.assign_fixed(
            || format!("g_def[{}]", offset),
            self.gate_def,
            offset,
            || Value::known(F::ZERO),
        )?;
        region.assign_fixed(
            || format!("l_def[{}]", offset),
            self.l_def,
            offset,
            || Value::known(F::ZERO),
        )?;
        region.assign_fixed(
            || format!("r_def[{}]", offset),
            self.r_def,
            offset,
            || Value::known(F::ZERO),
        )?;
        region.assign_fixed(
            || format!("o_def[{}]", offset),
            self.o_def,
            offset,
            || Value::known(F::ZERO),
        )?;

        Ok(())
    }
}
