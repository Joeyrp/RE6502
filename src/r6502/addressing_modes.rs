
#![allow(dead_code, non_snake_case)]

use super::{R6502, Bus};


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

    }

    pub fn IMM(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.working_data = bus.read(cpu.pc);
        cpu.pc += 1;
    }

    pub fn ZP0(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }

    pub fn ZPX(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }

    pub fn ZPY(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }

    pub fn REL(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }

    pub fn ABS(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }

    pub fn ABX(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
    }

    pub fn ABY(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        
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