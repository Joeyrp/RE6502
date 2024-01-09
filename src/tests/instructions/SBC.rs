
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

    // Program to subtract 0x06 from 0x0A
    bus.write(addr, 0xE9); // SBC - Immediate mode
    bus.write(addr + 1, 0x06);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu state
    cpu.set_flag(Flags::C);
    cpu.debug_set_reg(Registers::A, 0x0A);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x04 in the A register?
    assert_eq!(0x04, cpu.debug_get_reg(Registers::A), "wrong answer");
    assert_eq!(1, cpu.check_flag(Flags::C), "Carry bit should be set");
    assert_eq!(0, cpu.check_flag(Flags::V), "Overflow bit should not be set");
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

    // Program to subtract 0x09 from 0x08
    bus.write(addr, 0xE9); // SBC - Immediate mode
    bus.write(addr + 1, 0x09);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu state
    cpu.set_flag(Flags::C);
    cpu.debug_set_reg(Registers::A, 0x08);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is -1 in the A register?
    assert_eq!(0xFF as u16, cpu.debug_get_reg(Registers::A), "Wrong answer");
    assert_eq!(0, cpu.check_flag(Flags::C), "Carry bit should not be set");
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

    // Program to subtract 0x7E from 0xFB (-5 - 126)
    bus.write(addr, 0xE9); // SBC - Immediate mode
    bus.write(addr + 1, 0x7E);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu state
    cpu.set_flag(Flags::C);
    cpu.debug_set_reg(Registers::A, 0xFB);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);


    assert_eq!(0x7D, cpu.debug_get_reg(Registers::A), "Wrong answer");
    assert_eq!(1, cpu.check_flag(Flags::C), "Carry bit should be set");
    assert_eq!(1, cpu.check_flag(Flags::V), "Overflow bit should be set");
}
