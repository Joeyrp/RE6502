
#![allow(dead_code, non_snake_case)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers, Flags};

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

    // Write the program to memory
    // ADC 6
    bus.write(addr, 0x69); // ADC - Immediate mode
    bus.write(addr + 1, 0x06);   // Argument

    // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x04);

    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0A in the A register?
    assert_eq!(0x0A, cpu.debug_get_reg(Registers::A), "Failed basic addition");

    ///////////////////////////////////////////
    // EXTRA TESTING

    // Add with carry in
    // Write the program to memory
    // ADC 6
    bus.write(addr, 0x69); // ADC - Immediate mode
    bus.write(addr + 1, 0x06);   // Argument

    // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu internals
    cpu.debug_set_reg(Registers::A, 0x04);
    cpu.set_flag(Flags::C);

    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0B in the A register?
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A), "Failed addition with carry");

    // Add with overflow
    // Write the program to memory
    // ADC 126
    bus.write(addr, 0x69); // ADC - Immediate mode
    bus.write(addr + 1, 126);   // Argument

    // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu internals
    cpu.debug_set_reg(Registers::A, 0x04);

    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 130 in the A register?
    assert_eq!(130, cpu.debug_get_reg(Registers::A));

    // Is the overflow bit set?
    assert_eq!(1, cpu.check_flag(Flags::V), "Failed addition with overflow");

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

    // Manually put 0x04 into memory in the zero page
    bus.write(0x000B, 0x04);

    // ADC #6
    bus.write(addr, 0x65); // ADC - Zero Page mode
    bus.write(addr + 1, 0x0B);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x06);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0A in the A register?
    assert_eq!(0x0A, cpu.debug_get_reg(Registers::A));
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

    // Manually put 0x0A into memory in the zero page
    bus.write(0x000B, 0x04);

    // ADC #A
    bus.write(addr, 0x75); // ADC - Zero Page, X mode
    bus.write(addr + 1, 0x0A);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x06);
    cpu.debug_set_reg(Registers::X, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0A in the A register?
    assert_eq!(0x0A, cpu.debug_get_reg(Registers::A));
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

    // Manually put 0x0A into memory in the zero page
    bus.write(0x010B, 0x06);

    // AND $10B
    bus.write(addr, 0x6D); // ADC - Absolute mode
    bus.write(addr + 1, 0x0B);  // Argument
    bus.write(addr + 2, 0x01);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x04);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x09 in the A register?
    assert_eq!(0x0A, cpu.debug_get_reg(Registers::A));
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

    // Manually put 0x0A into memory
    bus.write(0x010C, 0x04);

    // ADC $10C
    bus.write(addr, 0x7D); // ADC - Absolute, X mode
    bus.write(addr + 1, 0x0B);  // Argument
    bus.write(addr + 2, 0x01);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x06);
    cpu.debug_set_reg(Registers::X, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0A in the A register?
    assert_eq!(0x0A, cpu.debug_get_reg(Registers::A));
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

    // Manually put 0x04 into memory
    bus.write(0x010C, 0x04);

    // Manually put 0x010C into the Zero Page
    bus.write(0x00B, 0x0C);
    bus.write(0x00C, 0x01);

    // ADC $10C
    bus.write(addr, 0x61); // ADC - Indirect, X mode
    bus.write(addr + 1, 0x0A);  // Argument pointer into the Zero Page 
    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x06);
    cpu.debug_set_reg(Registers::X, 0x01);  // Zero Page pointer offset

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0A in the A register?
    assert_eq!(0x0A, cpu.debug_get_reg(Registers::A));
}