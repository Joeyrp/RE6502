
#![allow(dead_code, non_snake_case)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers, Flags};

#[test]
fn less_than() 
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    ///////////////////////
    // Parameter is less than A reg

    // Program to compare 0x10 with 0x05 (0x05 will be the argument)
    bus.write(addr, 0xC9); // CMP - Immediate mode
    bus.write(addr + 1, 0x05);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x10);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Flag should be 1, Z N Flags should be 0
    assert_eq!(1, cpu.check_flag(Flags::C));
    assert_eq!(0, cpu.check_flag(Flags::Z));
    assert_eq!(0, cpu.check_flag(Flags::N));
}

#[test]
fn equal_to()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    ///////////////////////
    // Parameter is equal to the A reg

    // Program to compare 0x10 with 0x10
    bus.write(addr, 0xC9); // CMP - Immediate mode
    bus.write(addr + 1, 0x10);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x10);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Z Flags should be 1, N Flag should be 0
    assert_eq!(1, cpu.check_flag(Flags::C));
    assert_eq!(1, cpu.check_flag(Flags::Z));
    assert_eq!(0, cpu.check_flag(Flags::N));
}

#[test]
fn greater_than()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte
    
    ///////////////////////
    // Parameter is greater than A reg

    // Program to compare 0x05 with 0x10
    bus.write(addr, 0xC9); // CMP - Immediate mode
    bus.write(addr + 1, 0x10);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x05);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Z Flags should be 0, N Flag should be 1
    assert_eq!(0, cpu.check_flag(Flags::C));
    assert_eq!(0, cpu.check_flag(Flags::Z));
    assert_eq!(1, cpu.check_flag(Flags::N));
}