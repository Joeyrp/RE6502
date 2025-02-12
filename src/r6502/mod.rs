
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
    sp: u16,     // Stack Pointer
    status: u8, // Status Flags

    cycles: u32, // Track cycles

    // Helper Vars
    addr_mode: ModeID,
    working_data: u16,   // value fetched for the ALU
    working_addr: u16,

    program_stopped: bool,
}

impl R6502
{
    // constructor
    pub fn new() -> R6502
    {
        R6502 { a: 0, x: 0, y: 0, pc: 0, sp: 0, status: 0, cycles: 0, addr_mode: ModeID::IMP, 
                    working_data: 0, working_addr: 0, program_stopped: true }
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
            Registers::SP => self.sp  = value,

            Registers::STATUS => self.status = value as u8,
        }
    }

    pub fn is_program_stopped(&self) -> bool
    {
        self.program_stopped
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

            // Addressing modes increment pc instead
            //self.pc += 1; 
        //}

        // self.cycles -= 1;
    }

    pub fn reset(&mut self, bus: &mut dyn Bus)
    {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0x01FF; 
        self.status = 0;
        self.set_flag(Flags::U);

        let lo: u16 = bus.read(0xFFFC) as u16;
        let hi: u16 = bus.read(0xFFFD) as u16;

        self.pc = (hi << 8) | lo;

        // internal helper variables
        self.working_data = 0;
        // self.working_addr = 0;
        self.program_stopped = false;

        self.cycles = 8;
    }

    pub fn irq(&mut self, bus: &mut impl Bus)
    {
        if self.check_flag(Flags::I) != 0
        {
            return;
        }

        let pc_hi = ((self.pc & 0xFF00) >> 8) as u8; 
        let pc_lo = (self.pc & 0x00FF) as u8; 
        stack_push(pc_hi, self, bus, );
        stack_push(pc_lo, self, bus, );

        stack_push(self.status, self, bus);

        self.set_flag(Flags::I);

        let addr_lo = bus.read(0xFFFE) as u16;
        let addr_hi = bus.read(0xFFFF) as u16;
        self.pc = (addr_hi >> 8) | addr_lo;
    }

    pub fn nmi(&mut self, bus: &mut impl Bus)
    {
        let pc_hi = ((self.pc & 0xFF00) >> 8) as u8; 
        let pc_lo = (self.pc & 0x00FF) as u8; 
        stack_push(pc_hi, self, bus, );
        stack_push(pc_lo, self, bus, );

        stack_push(self.status, self, bus);

        let addr_lo = bus.read(0xFFFA) as u16;
        let addr_hi = bus.read(0xFFFB) as u16;
        self.pc = (addr_hi >> 8) | addr_lo;
    }

    // helpers
    pub fn set_zn_flags(&mut self, val: u8)
    {
        self.clear_flag(Flags::Z);
        if val == 0
        {
            self.set_flag(Flags::Z);
        }

        self.clear_flag(Flags::N);
        if val & 0x80 != 0
        {
            self.set_flag(Flags::N);
        }
    }

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


pub(crate) fn stack_push(value: u8, cpu: &mut R6502, bus: &mut dyn Bus)
{
    // TODO: Check for out of bounds errors
    bus.write(cpu.sp, value);
    cpu.sp -= 1;
}

pub(crate) fn stack_pop(cpu: &mut R6502, bus: &mut dyn Bus) -> u8
{
    cpu.sp += 1;
    bus.read(cpu.sp)
}


fn execute(instruction: u8, cpu: &mut R6502, bus: &mut dyn Bus)
{
    // Instruction decoding:
    // https://llx.com/Neil/a2/opcodes.html

    // Check if this is a branch instruction
    // The conditional branch instructions all have the form xxy10000
    // 0x1F == 11111
    // 0x10 == 10000
    const BRANCH_OP: u8 = 0x10;
    const BRANCH_MASK: u8 = 0x1F;
    if instruction & BRANCH_MASK == BRANCH_OP
    {
        exe_branch(instruction, cpu, bus);
        return;
    }

    // Interrupt and Subroutine
    const BRK: u8 = 0x00;
    const JSR: u8 = 0x20;
    const RTI: u8 = 0x40;
    const RTS: u8 = 0x60;

    match instruction
    {
        BRK => {Instructions::BRK(cpu, bus); return; }
        JSR => {Instructions::JSR(cpu, bus); return; }
        RTI => {Instructions::RTI(cpu, bus); return; }
        RTS => 
        {
            // Use the stack pointer to detect if this is the end of the program
            if cpu.sp == 0x01FF
            {
                cpu.program_stopped = true;
            }
            else
            {
                Instructions::RTS(cpu, bus); 
            }

            return; 
        }

        _ => ()
    }

    // Single byte instructions
    if exe_single_byte(instruction, cpu, bus)
    {
        return;
    }
    
    // Instructions with arguments
    const GROUP_ONE_OP:   u8 = 0x01;
    const GROUP_TWO_OP:   u8 = 0x02;
    const GROUP_THREE_OP: u8 = 0x00;

    let group_code = instruction & 0x03; // group one has a bit pattern of xxxxxx01
    match group_code
    {
        GROUP_ONE_OP => exe_group_one(instruction, cpu, bus),
        GROUP_TWO_OP => exe_group_two(instruction, cpu, bus),
        GROUP_THREE_OP => exe_group_three(instruction, cpu, bus),

        _ => panic!("UNKNOWN INSTRUCTION: {:#02X}", instruction)
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

fn exe_group_three(instruction: u8, cpu: &mut R6502, bus: &mut dyn Bus)
{
    let addr_mask = (instruction & 0x1C) >> 2;
    let op_mask = (instruction & 0xE0) >> 5;

    // SPECIAL CASE FOR JMP (abs)
    const JMP_IND: u8 = 0x6C;
    if instruction == JMP_IND
    {
        cpu.addr_mode = AddressingModes::IND(cpu, bus);
    }
    else
    {
        cpu.addr_mode = AddressingModes::GROUP_THREE_ADDRS[addr_mask as usize](cpu, bus);
    }

    Instructions::GROUP_THREE_OPS[op_mask as usize](cpu, bus);
}

fn exe_branch(instruction: u8, cpu: &mut R6502, bus: &mut dyn Bus)
{
    let pc_offset = AddressingModes::REL(cpu, bus);

    // Decode instruction
    // Need to map: 10	30	50	70	90	B0 	D0	F0 - op code
    // to:          0   1   2   3   4   5   6   7  - method index

    // ChatGPT says: Index = (Value−16​) / 32

    let idx = ((instruction - 16) / 32) as usize;

    Instructions::GROUP_BRANCHING_OPS[idx](cpu, bus);
}

// Returns true if the insruction was handled
fn exe_single_byte(instruction: u8, cpu: &mut R6502, bus: &mut dyn Bus) -> bool
{
    // Group of 8's
    // PHP CLC PLP SEC PHA CLI PLA SEI DEY TYA TAY CLV INY CLD INX SED
    //  08  18  28  38  48  58  68 	78  88 	98  A8  B8 	C8  D8  E8  F8 
    // Index = (Value-8) / 16
    const EIGHT_MASK: u8 = 0x0F;
    if instruction & EIGHT_MASK == 0x08
    {
        let i = ((instruction - 0x08) / 0x10) as usize;
        Instructions::GROUP_SB1_OPS[i](cpu, bus);
        return true;
    }
    
    // Group of A's
    // TXA 	TXS  TAX  TSX  DEX 	NOP
    // 8A 	9A 	  AA   BA 	CA 	 EA	
    // Index = (Value-8A) /	16
    if instruction < 0x8A
    {
        return false;
    }

    const A_MASK: u8 = 0x05;
    if instruction & A_MASK == 0xA
    {
        let i = ((instruction - 0x8A) / 0x10) as usize;
        Instructions::GROUP_SB2_OPS[i](cpu, bus);
        return true;
    }

    false
}