
#![allow(dead_code)]

mod addressing_modes;
mod instructions;

use addressing_modes::AddressingModes;
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

pub struct R6502
{
    pub a: u8,      // Accumulator
    x: u8,      // X Register
    y: u8,      // Y Register

    pc: u16,    // Program Counter
    sp: u8,     // Stack Pointer
    status: u8, // Status Flags

    cycles: u32, // Track cycles

    // Helper Vars
    working_data: u8,   // value fetched for the ALU
    addr_abs: u16,
    addr_rel: u16,
}

impl R6502
{
    // constructor
    pub fn new() -> R6502
    {
        R6502 { a: 0, x: 0, y: 0, pc: 0, sp: 0, status: 0, cycles: 0, working_data: 0, addr_abs: 0, addr_rel: 0 }
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
        self.addr_abs = 0;
        self.addr_rel = 0;

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

    pub fn check_flag(&mut self, bit: Flags) -> bool
    {
        self.status & (bit as u8) > 0
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
    
    AddressingModes::GROUP_ONE_ADDRS[addr_mask as usize](cpu, bus);
    Instructions::GROUP_ONE_OPS[op_mask as usize](cpu, bus);
}

fn exe_group_two(instruction: u8, cpu: &mut R6502, bus: &mut dyn Bus)
{
    
}

