

#![allow(dead_code, non_snake_case)]

use super::{R6502, Bus, Flags};

// Instruction decoding:
// https://llx.com/Neil/a2/opcodes.html

// GROUP ONE
// 000	ORA
// 001	AND
// 010	EOR
// 011	ADC
// 100	STA
// 101	LDA
// 110	CMP
// 111	SBC

pub struct Instructions;

impl Instructions
{
    pub const GROUP_ONE_OPS: [fn(&mut R6502, &mut dyn Bus); 8] = [
        Instructions::ORA, 
        Instructions::AND,
        Instructions::EOR,
        Instructions::ADC,
        Instructions::STA,
        Instructions::LDA,
        Instructions::CMP,
        Instructions::SBC,
        ];
}

impl Instructions
{
    ///////////////////////////////////////////////////////////
    // GROUP ONE
    pub fn ORA(cpu: &mut R6502, _bus: &mut dyn Bus)
    {
        cpu.a = cpu.a | cpu.working_data;
        if cpu.a == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.a & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn AND(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.a = cpu.a & cpu.working_data;
        if cpu.a == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.a & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn EOR(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.a = cpu.a ^ cpu.working_data;
        if cpu.a == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.a & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    // Using a technique written javidx9
    // The code in this function falls under the License (OLC-3) SEE LICENSE FILE
    // https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp#L659
    pub fn ADC(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        // 16 bit addition to capture the carry easier
        let temp: u16 = cpu.a as u16 + cpu.working_data as u16;

        if temp > 255
        {
            cpu.set_flag(Flags::C);
        }
        
        if temp == 0
        {
            cpu.set_flag(Flags::Z);
        }

        let did_overflow = (!((cpu.a as u16) ^ (cpu.working_data as u16)) & ((cpu.a as u16) ^ temp)) & 0x0080;
        cpu.clear_flag(Flags::V);
        if did_overflow > 0
        {
            cpu.set_flag(Flags::V);
        }

        cpu.clear_flag(Flags::N);
        if temp & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

        cpu.a = (temp & 0x00FF) as u8;
    }

    pub fn STA(cpu: &mut R6502, bus: &mut dyn Bus)
    {

    }

    pub fn LDA(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.a = cpu.working_data;

        if cpu.a == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.a & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn CMP(cpu: &mut R6502, bus: &mut dyn Bus)
    {

    }

    pub fn SBC(cpu: &mut R6502, bus: &mut dyn Bus)
    {

    }

    ///////////////////////////////////////////////////////////
    // GROUP TWO
}