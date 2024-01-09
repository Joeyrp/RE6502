
#![allow(dead_code, non_snake_case)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers, Flags};

/////////////////////////////////////////////////////////////////////
//				GROUP ONE
/////////////////////////////////////////////////////////////////////

#[test]
fn ORA()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // ORA #2
    bus.write(addr, 0x09); // ORA - Immediate mode
    bus.write(addr + 1, 0x02);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x09);

    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0B in the A register?
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A));

}

#[cfg(test)]
mod ADC;

#[test]
fn AND()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // AND #3
    bus.write(addr, 0x29); // AND - Immediate mode
    bus.write(addr + 1, 0x03);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x02 in the A register?
    assert_eq!(0x02, cpu.debug_get_reg(Registers::A));
}

#[test]
fn EOR() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // AND #3
    bus.write(addr, 0x49); // EOR - Immediate mode
    bus.write(addr + 1, 0x03);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x09 in the A register?
    assert_eq!(0x09, cpu.debug_get_reg(Registers::A));
}


#[test]
fn LDA() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xA9); // LDA - Immediate mode
    bus.write(addr + 1, 0x08);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the A register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::A));
}

#[test]
fn STA()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte


    // STA $0A
    bus.write(addr, 0x85); // STA - Zero Page mode
    bus.write(addr + 1, 0x0A);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0F);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0F at memory address 0x0A?
    assert_eq!(0x0F, bus.read(0x0A));

}

#[cfg(test)]
mod CMP;

#[cfg(test)]
mod SBC;

/////////////////////////////////////////////////////////////////////
//				GROUP TWO
/////////////////////////////////////////////////////////////////////

#[test]
fn ASL()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Program to left shift the value in the accumulator
    bus.write(addr, 0x0A); // ASL - Accumulator mode

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x98);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x30 in the A register?
    assert_eq!(0x30, cpu.debug_get_reg(Registers::A));
    assert_eq!(0, cpu.check_flag(Flags::Z), "Zero flag should not be set");
    assert_eq!(1, cpu.check_flag(Flags::C), "Carry flag should be set");
    assert_eq!(0, cpu.check_flag(Flags::N), "Negative flag should not be set");
}

#[test]
fn ASL_ZP0()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Write the value into memory
    bus.write(0x05, 0x98);

    // Program to left shift the value in the accumulator
    bus.write(addr, 0x06); // ASL - Zero Page mode
    bus.write(addr + 1, 0x05); // Zero Page pointer

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x05);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x30 in memory?
    assert_eq!(0x30, bus.read(0x05));
    assert_eq!(0, cpu.check_flag(Flags::Z), "Zero flag should not be set");
    assert_eq!(1, cpu.check_flag(Flags::C), "Carry flag should be set");
    assert_eq!(0, cpu.check_flag(Flags::N), "Negative flag should not be set");
}

#[test]
fn ROL()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Program to left shift the value in the accumulator
    bus.write(addr, 0x2A); // ROL - Accumulator mode

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x80);
    cpu.set_flag(Flags::C);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x01 in the A register?
    assert_eq!(0x01, cpu.debug_get_reg(Registers::A));
    assert_eq!(0, cpu.check_flag(Flags::Z), "Zero flag should not be set");
    assert_eq!(1, cpu.check_flag(Flags::C), "Carry flag should be set");
    assert_eq!(0, cpu.check_flag(Flags::N), "Negative flag should not be set");
}


#[test]
fn LSR()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Program to left shift the value in the accumulator
    bus.write(addr, 0x4A); // LSR - Accumulator mode

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x81);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x40 in the A register?
    assert_eq!(0x40, cpu.debug_get_reg(Registers::A));
    assert_eq!(0, cpu.check_flag(Flags::Z), "Zero flag should not be set");
    assert_eq!(1, cpu.check_flag(Flags::C), "Carry flag should be set");
    assert_eq!(0, cpu.check_flag(Flags::N), "Negative flag should not be set");
}

#[test]
fn ROR()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Program to left shift the value in the accumulator
    bus.write(addr, 0x6A); // ROR - Accumulator mode

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x81);
    cpu.set_flag(Flags::C);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0xC0 in the A register?
    assert_eq!(0xC0, cpu.debug_get_reg(Registers::A));
    assert_eq!(0, cpu.check_flag(Flags::Z), "Zero flag should not be set");
    assert_eq!(1, cpu.check_flag(Flags::C), "Carry flag should be set");
    assert_eq!(1, cpu.check_flag(Flags::N), "Negative flag should be set");
}

#[test]
fn STX()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte


    // STX $0A
    bus.write(addr, 0x86); // STX - Zero Page mode
    bus.write(addr + 1, 0x0A);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x0F);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0F at memory address 0x0A?
    assert_eq!(0x0F, bus.read(0x0A));

}

#[test]
fn LDX() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xA2); // LDX - Immediate mode
    bus.write(addr + 1, 0x08);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the X register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::X));
}

#[test]
fn DEC()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // put value to decrement into memory

    bus.write(0x08, 0x10);

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xC6); // DEC - Zero Page
    bus.write(addr + 1, 0x08);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0F in memory at 0x08?
    assert_eq!(0x0F, bus.read(0x08));
}

#[test]
fn INC()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Put value to decrement into memory
    bus.write(0x08, 0x10);

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xE6); // INC - Zero Page
    bus.write(addr + 1, 0x08);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x11 in memory at 0x08?
    assert_eq!(0x11, bus.read(0x08));
}


/////////////////////////////////////////////////////////////////////
//				GROUP THREE
/////////////////////////////////////////////////////////////////////
#[test]
fn BIT()
{
    // TODO: Could add more tests for this instruction
    //      Maybe move into it's own file and test
    //      every result pattern.

    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Put value to test into memory
    bus.write(0x08, 0x10);

    // BIT test program
    bus.write(addr, 0x24); // BIT - Zero Page
    bus.write(addr + 1, 0x08);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // Preload A register with bit mask
    cpu.debug_set_reg(Registers::A, 0x05);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is the Z flag set?
    assert_eq!(1, cpu.check_flag(Flags::Z));

    // Is the N flag and V flag not set?
    assert_eq!(0, cpu.check_flag(Flags::V));
    assert_eq!(0, cpu.check_flag(Flags::N));
}

#[test]
fn JMP()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Put jump location into memory
    bus.write(0x08, 0x34);
    bus.write(0x09, 0x12);

    // JMP test program
    bus.write(addr, 0x6C); // JMP - IND
    bus.write(addr + 1, 0x08);  // LO Argument
    bus.write(addr + 2, 0x00);  // HI Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is the program counter now 0x1234?
    assert_eq!(0x1234, cpu.debug_get_reg(Registers::PC));
}