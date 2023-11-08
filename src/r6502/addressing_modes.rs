
#![allow(unused_variables, dead_code, non_snake_case)]

use super::{R6502, Bus};

// Instruction decoding:
// https://llx.com/Neil/a2/opcodes.html

// GROUP ONE ADDRESS MODES
// 000	(zero page,X)       IZX
// 001	zero page           ZP0
// 010	#immediate          IMM
// 011	absolute            ABS
// 100	(zero page),Y       IZY
// 101	zero page,X         ZPX
// 110	absolute,Y          ABY
// 111	absolute,X          ABX

// GROUP TWO ADDRESS MODES
// 000	#immediate          IMM
// 001	zero page           ZP0
// 010	accumulator         IMP
// 011	absolute            ABS
// 101	zero page,X         ZPX
// 111	absolute,X          ABX

pub struct AddressingModes;
impl AddressingModes
{
    pub const GROUP_ONE_ADDRS: [fn(&mut R6502, &mut dyn Bus); 8] = [
        AddressingModes::IZX, 
        AddressingModes::ZP0,
        AddressingModes::IMM,
        AddressingModes::ABS,
        AddressingModes::IZY,
        AddressingModes::ZPX,
        AddressingModes::ABY,
        AddressingModes::ABX,
        ];


}

impl AddressingModes
{
    // This is also the accumulator mode
    pub fn IMP(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.working_data = cpu.a as u16;
    }

    pub fn IMM(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.working_data = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
    }

    pub fn ZP0(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let working_addr = bus.read(cpu.pc) as u16 & 0x00FF;
        cpu.pc += 1;

        cpu.working_data = bus.read(working_addr) as u16;
    }

    pub fn ZPX(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let mut working_addr = bus.read(cpu.pc) as u16 & 0x00FF;
        working_addr += cpu.x as u16;
        cpu.pc += 1; 

        cpu.working_data = bus.read(working_addr) as u16;
    }

    pub fn ZPY(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let mut working_addr = bus.read(cpu.pc) as u16 & 0x00FF;
        working_addr += cpu.y as u16;
        cpu.pc += 1; 
        
        cpu.working_data = bus.read(working_addr) as u16;
    }

    pub fn REL(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        // NOTE: Not sure if we can use the working_data variable for this.
        //          if any instruction using this address mode needs extra data read
        //          then we need another variable to store this address
        cpu.working_data = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
    }

    pub fn ABS(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.working_data = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.working_data |= (bus.read(cpu.pc) as u16) << 8;
        cpu.pc += 1;

        cpu.working_data = bus.read(cpu.working_data) as u16 & 0x00FF;
    }

    pub fn ABX(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.working_data = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.working_data |= (bus.read(cpu.pc) as u16) << 8;
        cpu.pc += 1;

        cpu.working_data += cpu.x as u16;

        cpu.working_data = bus.read(cpu.working_data) as u16 & 0x00FF;
    }

    pub fn ABY(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.working_data = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.working_data |= (bus.read(cpu.pc) as u16) << 8;
        cpu.pc += 1;

        cpu.working_data += cpu.y as u16;

        cpu.working_data = bus.read(cpu.working_data) as u16 & 0x00FF;
    }

    pub fn IND(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }

    pub fn IZX(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }

    pub fn IZY(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }
}