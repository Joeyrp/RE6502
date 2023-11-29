
#![allow(unused_variables, dead_code, non_snake_case)]

mod addressing_modes;
mod instructions;

use addressing_modes::{AddressingModes, ModeID};
use instructions::Instructions;

pub trait Bus
{
    fn read(&self, addr: u16) -> u8; 
    fn write(&mut self, addr: u16, value: u8);
}

// impl Sized for Bus
// {
    
// }


#[derive(Copy, Clone, Debug)]
pub enum Flags
{
    C = (1 << 0),   // Carry Flag
    Z = (1 << 1),   // Zero Flag      
    I = (1 << 2),   // Interrupt Disable
    D = (1 << 3),   // Decimal Mode Flag
    B = (1 << 4),   // Break Command
    U = (1 << 5),   // Unused
    V = (1 << 6),   // Overflow Flag
    N = (1 << 7),   // Negative Flag
}

pub enum Registers
{
    A,
    X,
    Y,
    PC,
    SP,
    STATUS,
}

#[derive(Clone, Copy, PartialEq)]
pub struct R6502
{
    a: u8,      // Accumulator
    x: u8,      // X Register
    y: u8,      // Y Register

    pc: u16,    // Program Counter
    sp: u8,     // Stack Pointer
    status: u8, // Status Flags

    cycles: u32, // Track cycles

    // Helper Vars
    addr_mode: ModeID,
    working_data: u16,   // value fetched for the ALU
    working_addr: u16,
}

impl R6502
{
    // constructor
    pub fn new() -> R6502
    {
        R6502 { a: 0, x: 0, y: 0, pc: 0, sp: 0, status: 0, cycles: 0, addr_mode: ModeID::IMP, working_data: 0, working_addr: 0 }
    }

    // Debug Access
    pub fn debug_get_reg(&self, reg: Registers) -> u16
    {
        match reg
        {
            Registers::A => self.a as u16,
            Registers::X => self.x as u16,
            Registers::Y => self.y as u16,

            Registers::PC => self.pc,
            Registers::SP => self.sp as u16,

            Registers::STATUS => self.status as u16,
        }
    }

    pub fn debug_set_reg(&mut self, reg: Registers, value: u16)
    {
        match reg
        {
            Registers::A => self.a = value as u8,
            Registers::X => self.x = value as u8,
            Registers::Y => self.y = value as u8,

            Registers::PC => self.pc = value,
            Registers::SP => self.sp  = value as u8,

            Registers::STATUS => self.status = value as u8,
        }
    }

    // signals
    pub fn clock(&mut self, bus: &mut dyn Bus)
    {
        // TODO: Track instructions cycles
        // if self.cycles == 0
        //{
            let opcode = bus.read(self.pc);
            self.pc += 1;

            execute(opcode, self, bus);
            //self.pc += 1;
        //}

        self.cycles -= 1;
    }

    pub fn reset(&mut self, bus: &mut dyn Bus)
    {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFF;     // stack actually starts at 0x01FF but we only need the low byte since the end of the stack is at 0x0100
        self.status = 0;
        self.set_flag(Flags::U);

        let lo: u16 = bus.read(0xFFFC) as u16;
        let hi: u16 = bus.read(0xFFFD) as u16;

        self.pc = (hi << 8) | lo;

        // internal helper variables
        self.working_data = 0;
        // self.working_addr = 0;

        self.cycles = 8;
    }

    pub fn irq(&mut self, bus: &mut impl Bus)
    {

    }

    pub fn nmi(&mut self, bus: &mut impl Bus)
    {
        
    }

    // helpers
    pub fn set_flag(&mut self, bit: Flags)
    {
        self.status |= bit as u8;
    }

    pub fn clear_flag(&mut self, bit: Flags)
    {
        self.status &= !(bit as u8);
    }

    pub fn check_flag(&mut self, bit: Flags) -> u8
    {
        if self.status & (bit as u8) > 0
        {
            return 1;
        }

        return 0;
    }
}



fn execute(instruction: u8, cpu: &mut R6502, bus: &mut dyn Bus)
{
    // Instruction decoding:
    // https://llx.com/Neil/a2/opcodes.html
    
    let group_code = instruction & 0x03; // group one has a bit pattern of xxxxxx01
    match group_code
    {
        0x01 => exe_group_one(instruction, cpu, bus),
        0x02  => exe_group_two(instruction, cpu, bus),

        // TODO: Conditionals and specially formatted instructions

        _ => panic!("UNKNOWN INSTRUCTION ADDRESS MODE: {}", group_code)
    }
}

fn exe_group_one(instruction: u8, cpu: &mut R6502, bus: &mut dyn Bus)
{
    let addr_mask = (instruction & 0x1C) >> 2;
    let op_mask = (instruction & 0xE0) >> 5;
    
    cpu.addr_mode = AddressingModes::GROUP_ONE_ADDRS[addr_mask as usize](cpu, bus);
    Instructions::GROUP_ONE_OPS[op_mask as usize](cpu, bus);
}

fn exe_group_two(instruction: u8, cpu: &mut R6502, bus: &mut dyn Bus)
{
    let addr_mask = (instruction & 0x1C) >> 2;
    let op_mask = (instruction & 0xE0) >> 5;

    // With STX and LDX, "zero page,X" addressing becomes "zero page,Y", and with LDX, "absolute,X" becomes "absolute,Y".
    const STX_ZPX: u8 = 0x96;
    const LDX_ZPX: u8 = 0xB6;
    const LDX_ABX: u8 = 0xBE;


    match instruction
    {
        STX_ZPX => 
        {
            cpu.addr_mode = AddressingModes::ZPY(cpu, bus);
            Instructions::STX(cpu, bus);
        },

        LDX_ZPX => 
        {
            cpu.addr_mode = AddressingModes::ZPY(cpu, bus);
            Instructions::LDX(cpu, bus);
        }

        LDX_ABX => 
        {
            
            cpu.addr_mode = AddressingModes::ABY(cpu, bus);
            Instructions::LDX(cpu, bus);
        }

        _ =>
        {
            cpu.addr_mode = AddressingModes::GROUP_TWO_ADDRS[addr_mask as usize](cpu, bus);
            Instructions::GROUP_TWO_OPS[op_mask as usize](cpu, bus);
        }
    }
}

