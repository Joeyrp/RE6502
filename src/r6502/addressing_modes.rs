
#![allow(unused_variables, dead_code, non_snake_case)]

use super::{R6502, Bus};

#[derive(Clone, Copy, PartialEq)]
pub enum ModeID
{
    IMP,    // Implied
    ACM,    // Accumulator
    IMM,    // Immediate
    ZP0,    // Zero Page
    ZPX,    // Zero Page, X
    ZPY,    // Zero Page, Y
    REL,    // Relative
    ABS,    // Absolute
    ABX,    // Absolute, X
    ABY,    // Aboslute, Y
    IND,    // Indirect
    IZX,    // Indirect, X
    IZY,    // Indirect, Y    
    ERR,    // Error mode - this is an invalid mode

}

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
// 010	accumulator         ACM
// 011	absolute            ABS
// 100  NONE                ERR
// 101	zero page,X         ZPX
// 110  NONE                ERR
// 111	absolute,X          ABX

// GROUP THREE ADDRES MODES
// 000	#immediate          IMM
// 001	zero page           ZP0
// 011	absolute            ABS
// 101	zero page,X         ZPX
// 111	absolute,X          ABX

pub struct AddressingModes;
impl AddressingModes
{
    pub const GROUP_ONE_ADDRS: [fn(&mut R6502, &mut dyn Bus) -> ModeID; 8] = [
        AddressingModes::IZX, 
        AddressingModes::ZP0,
        AddressingModes::IMM,
        AddressingModes::ABS,
        AddressingModes::IZY,
        AddressingModes::ZPX,
        AddressingModes::ABY,
        AddressingModes::ABX,
        ];

    pub const GROUP_TWO_ADDRS: [fn(&mut R6502, &mut dyn Bus) -> ModeID; 8] = [
        AddressingModes::IMM,
        AddressingModes::ZP0,
        AddressingModes::ACM,
        AddressingModes::ABS,
        AddressingModes::ERR,
        AddressingModes::ZPX,
        AddressingModes::ERR,
        AddressingModes::ABX,
        ];

    pub const GROUP_THREE_ADDRS: [fn(&mut R6502, &mut dyn Bus) -> ModeID; 8] = [
        AddressingModes::IMM,
        AddressingModes::ZP0,
        AddressingModes::ERR,
        AddressingModes::ABS,
        AddressingModes::ERR,
        AddressingModes::ZPX,
        AddressingModes::ERR,
        AddressingModes::ABX,
        ];
}

impl AddressingModes
{

    pub fn ERR(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        // TODO: Better error handling
        ModeID::ERR
    }

    pub fn IMP(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        ModeID::IMP
    }

    pub fn ACM(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_data = cpu.a as u16;
        ModeID::ACM
    }

    pub fn IMM(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_data = bus.read(cpu.pc) as u16;
        cpu.pc += 1;

        ModeID::IMM
    }

    pub fn ZP0(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_addr = bus.read(cpu.pc) as u16 & 0x00FF;
        cpu.pc += 1;

        cpu.working_data = bus.read(cpu.working_addr) as u16;

        ModeID::ZP0
    }

    pub fn ZPX(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_addr = bus.read(cpu.pc) as u16 & 0x00FF;
        cpu.working_addr += cpu.x as u16;
        cpu.pc += 1; 

        cpu.working_data = bus.read(cpu.working_addr) as u16;

        ModeID::ZPX
    }

    pub fn ZPY(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_addr = bus.read(cpu.pc) as u16 & 0x00FF;
        cpu.working_addr += cpu.y as u16;
        cpu.pc += 1; 
        
        cpu.working_data = bus.read(cpu.working_addr) as u16;

        ModeID::ZPY
    }

    pub fn REL(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_data = bus.read(cpu.pc) as u16;
        cpu.pc += 1;

        ModeID::REL
    }

    pub fn ABS(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_addr = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.working_addr |= (bus.read(cpu.pc) as u16) << 8;
        cpu.pc += 1;

        cpu.working_data = bus.read(cpu.working_addr) as u16 & 0x00FF;

        ModeID::ABS
    }

    pub fn ABX(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_addr = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.working_addr |= (bus.read(cpu.pc) as u16) << 8;
        cpu.pc += 1;

        cpu.working_addr += cpu.x as u16;

        cpu.working_data = bus.read(cpu.working_addr) as u16 & 0x00FF;

        ModeID::ABX
    }

    pub fn ABY(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        cpu.working_addr = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.working_addr |= (bus.read(cpu.pc) as u16) << 8;
        cpu.pc += 1;

        cpu.working_addr += cpu.y as u16;

        cpu.working_data = bus.read(cpu.working_addr) as u16 & 0x00FF;

        ModeID::ABY
    }

    // https://www.nesdev.org/obelisk-6502-guide/addressing.html#IND
    // JMP is the only 6502 instruction to support indirection. 
    // The instruction contains a 16 bit address which identifies the location of 
    // the least significant byte of another 16 bit memory address which is the real 
    // target of the instruction.

    // For example if location $0120 contains $FC and location $0121 contains $BA then 
    // the instruction JMP ($0120) will cause the next instruction execution to occur at 
    // $BAFC (e.g. the contents of $0120 and $0121).

    pub fn IND(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
        // NOTE: An original 6502 does not correctly fetch the target address 
        // if the indirect vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF). 
        // In this case it fetches the LSB from $xxFF as expected but takes the MSB from $xx00.

        let ptr_lo = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
        let ptr_hi = bus.read(cpu.pc) as u16;
        cpu.pc += 1;

        let ptr = (ptr_hi << 8) | ptr_lo;
        
        let addr_lo = bus.read(ptr) as u16;
        let mut addr_hi = bus.read(ptr + 1) as u16;

        // Emulate the bug
        if ptr_lo == 0xFF
        {
            addr_hi = bus.read(ptr & 0xFF00) as u16;
        }

        cpu.working_addr = (addr_hi << 8) | addr_lo;

        ModeID::IND
    }


    // Indexed Indirect Addressing (IND, X)
    // In indexed indirect addressing (referred to as (Indirect, X)), the second byte of the
    // instruction is added to the contents of the X register, discarding the carry.
    // The result of the addition points to a memory location on the Zero Page which contains
    // the low order byte of the effective address. The next memory location in page zero,
    // contains the high order byte of the effective address. Both memory locations specifying
    // the effective address must be in the Zero Page.
    //
    // Info from:
    // https://web.archive.org/web/20221112231348if_/http://archive.6502.org/datasheets/rockwell_r650x_r651x.pdf
    pub fn IZX(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        let offset = bus.read(cpu.pc) as u16;
        cpu.pc += 1;
        let mut pointer = cpu.x as u16 + offset;

        // discard the carry and wrap
        // If the addition goes beyond the Zero Page
        // it should wrap around back to the beginning
        pointer = pointer & 0x00FF;

        let lo_byte = bus.read(pointer) as u16;
        let hi_byte = bus.read(pointer + 1) as u16;
        cpu.working_addr = (hi_byte << 0x08) | lo_byte;

        cpu.working_data = bus.read(cpu.working_addr) as u16 & 0x00FF;

        ModeID::IZX
    }


    // Indirect Indexed Addressing (IND), Y
    // In indirect indexed addressing, the second byte of the instruction points to
    // a memory location in page zero. The contents of this memory location are added to
    // the contents of the Y register. The result is the low order byte of the effective address.
    // The carry from this addition is added to the contents of the next page zero memory
    // location, to form the high order byte of the effective address.
    //
    // Info from:
    // https://web.archive.org/web/20221112231348if_/http://archive.6502.org/datasheets/rockwell_r650x_r651x.pdf
    pub fn IZY(cpu: &mut R6502, bus: &mut dyn Bus) -> ModeID
    {
        // zp_pointer points to a location in zero page
        let zp_pointer = bus.read(cpu.pc) as u16;
        cpu.pc += 1;

        // The value at zp_pointer is added to the Y register
        let zp_value = bus.read(zp_pointer) as u16;
        let sum = zp_value + cpu.y as u16;

        // The sum with the carry discarded is the lo byte
        let lo_byte = sum & 0x00FF;

        // The carry plus the value at the next zero page address is the hi byte
        let zp_next = bus.read(zp_pointer + 1) as u16;
        let temp = (sum & 0xFF00) >> 0x08;
        let temp2 = temp + zp_next;
        let hi_byte: u8 = (((sum & 0xFF00) >> 0x08) + zp_next) as u8;

        // Store the final address and read the data
        cpu.working_addr = ((hi_byte as u16) << 0x08) | lo_byte;
        cpu.working_data = bus.read(cpu.working_addr) as u16;

        ModeID::IZY
    }
}