
#![allow(dead_code, non_snake_case)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers, Flags};

#[test]
fn basic()
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
    assert_eq!(0x0A, cpu.debug_get_reg(Registers::A), "Wrong answer");
    assert_eq!(0, cpu.check_flag(Flags::V), "Overflow bit should not be set");
    assert_eq!(0, cpu.check_flag(Flags::C), "Carry bit should not be set");

}

#[test]
fn with_carry()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

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
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A), "Wrong answer");
    assert_eq!(0, cpu.check_flag(Flags::V), "Overflow bit should not be set");

}

#[test]
fn with_overflow()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

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