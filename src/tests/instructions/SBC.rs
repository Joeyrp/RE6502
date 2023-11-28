
#![allow(dead_code, non_snake_case)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers};

//////////////////////////////////////////////////////////////////////////////
///     IMM     IMM     IMM     IMM     IMM     IMM     IMM     IMM     IMM
//////////////////////////////////////////////////////////////////////////////

#[test]
fn IMM() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Program to subtract 0x06 from 0x0A
    bus.write(addr, 0xE9); // SBC - Immediate mode
    bus.write(addr + 1, 0x06);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x04 in the A register?
    assert_eq!(0x04, cpu.debug_get_reg(Registers::A));
}


//////////////////////////////////////////////////////////////////////////////
///     ZP0     ZP0     ZP0     ZP0     ZP0     ZP0     ZP0     ZP0     ZP0
//////////////////////////////////////////////////////////////////////////////

#[test]
fn ZP0() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x06 into memory in the zero page
    bus.write(0x000A, 0x06);

    // Program to 
    bus.write(addr, 0xE5); // SBC - Zero Page mode
    bus.write(addr + 1, 0x0A);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x04 in the A register?
    assert_eq!(0x04, cpu.debug_get_reg(Registers::A));
}


//////////////////////////////////////////////////////////////////////////////
///     ZPX     ZPX     ZPX     ZPX     ZPX     ZPX     ZPX     ZPX     ZPX
//////////////////////////////////////////////////////////////////////////////

#[test]
fn ZPX() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x06 into memory in the zero page
    bus.write(0x000A, 0x06);

    // Program to 
    bus.write(addr, 0xF5); //  - Zero Page, X mode
    bus.write(addr + 1, 0x04);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x06);
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x04 in the A register?
    assert_eq!(0x04, cpu.debug_get_reg(Registers::A));
}


//////////////////////////////////////////////////////////////////////////////
///     ABS     ABS     ABS     ABS     ABS     ABS     ABS     ABS     ABS
//////////////////////////////////////////////////////////////////////////////

#[test]
fn ABS() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x06 into memory in the zero page
    bus.write(0x010A, 0x06);

    // Program to 
    bus.write(addr, 0xED); // SBC - Absolute mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);

    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x04 in the A register?
    assert_eq!(0x04, cpu.debug_get_reg(Registers::A));
}


//////////////////////////////////////////////////////////////////////////////
///     ABX     ABX     ABX     ABX     ABX     ABX     ABX     ABX     ABX
//////////////////////////////////////////////////////////////////////////////

#[test]
fn ABX() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x06 into memory in the zero page
    bus.write(0x010B, 0x06);

    // Program to 
    bus.write(addr, 0xFD); // SBC - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x04 in the A register?
    assert_eq!(0x04, cpu.debug_get_reg(Registers::A));
}

//////////////////////////////////////////////////////////////////////////////
///     ABY     ABY     ABY     ABY     ABY     ABY     ABY     ABY     ABY
//////////////////////////////////////////////////////////////////////////////

#[test]
fn ABY() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x08 into memory in the zero page
    bus.write(0x010B, 0x06);

    // Program to 
    bus.write(addr, 0xF9); //  - Absolute, Y mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::Y, 0x01);
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x04 in the A register?
    assert_eq!(0x04, cpu.debug_get_reg(Registers::A));
}

//////////////////////////////////////////////////////////////////////////////
///     IZX     IZX     IZX     IZX     IZX     IZX     IZX     IZX     IZX
//////////////////////////////////////////////////////////////////////////////

#[test]
fn IZX() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x08 into memory in the zero page
    bus.write(0x010B, 0x06);

    // Manually put 0x010B into the Zero page
    bus.write(0x000B, 0x0B);
    bus.write(0x000C, 0x01);

    // Program to 
    bus.write(addr, 0xE1); //  - Indirect, X mode
    bus.write(addr + 1, 0x0A);  // Argument - Pointer into the Zero Page

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01); // Zero Page Pointer offset
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x04 in the A register?
    assert_eq!(0x04, cpu.debug_get_reg(Registers::A));
}