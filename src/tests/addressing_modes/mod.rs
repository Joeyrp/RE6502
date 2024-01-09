
#![allow(dead_code, non_snake_case)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers};

#[test]
fn IMP()
{

}

#[test]
fn ACM()
{

}

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
fn ZP0() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x08 into memory in the zero page
    bus.write(0x000A, 0x08);

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xA5); // LDA - Zero Page mode
    bus.write(addr + 1, 0x0A);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the A register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::A));
}

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

    // Manually put 0x08 into memory in the zero page
    bus.write(0x000A, 0x08);

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xB5); // LDA - Zero Page, X mode
    bus.write(addr + 1, 0x04);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x06);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the A register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::A));
}

#[test]
fn ZPY()
{

}

#[test]
fn REL()
{

}


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

    // Manually put 0x08 into memory in the zero page
    bus.write(0x010A, 0x08);

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xAD); // LDA - Absolute mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the A register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::A));
}

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

    // Manually put 0x08 into memory in the zero page
    bus.write(0x010B, 0x08);

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xBD); // LDA - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the A register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::A));
}

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
    bus.write(0x010B, 0x08);

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xB9); // LDA - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::Y, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the A register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::A));
}

#[test]
fn IND()
{
    
}

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

    // Manually put 0x08 into memory
    bus.write(0x010B, 0x08);

    // Manuall put 0x010B into the Zero page at 0x000A
    bus.write(0x000A, 0x0B);    // Pointer lo byte
    bus.write(0x000B, 0x01);    // Pointer hi byte

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xA1); // LDA - Indirect, X mode
    bus.write(addr + 1, 0x09);  // Argument - gets added to X reg

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the A register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::A));
}

#[test]
fn IZY() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x08 into memory
    bus.write(0x020B, 0x08);

    // Manuall put 0x01FC into the Zero page at 0x000A
    // This will be added to the Y register (which will store 0x0F)
    bus.write(0x000A, 0xFC);    // Pointer lo byte
    bus.write(0x000B, 0x01);    // Pointer hi byte

    // Program to load 0x08 into the accumulator
    bus.write(addr, 0xB1); // LDA - Indirect, Y mode
    bus.write(addr + 1, 0x0A);  // Argument - Pointer into the Zero Page

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::Y, 0x0F); // Offset of the value at the zero page address

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x08 in the A register?
    assert_eq!(0x08, cpu.debug_get_reg(Registers::A));
}