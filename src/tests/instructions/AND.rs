
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
fn ZP0()
{
    let mut cpu = R6502::new();
    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    // Manually put 0x0A into memory in the zero page
    bus.write(0x000B, 0x03);

    // AND #3
    bus.write(addr, 0x25); // AND - Zero Page mode
    bus.write(addr + 1, 0x0B);  // Argument

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
    bus.write(0x000B, 0x03);

    // AND #3
    bus.write(addr, 0x35); // AND - Zero Page, X mode
    bus.write(addr + 1, 0x0A);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0A);
    cpu.debug_set_reg(Registers::X, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x02 in the A register?
    assert_eq!(0x02, cpu.debug_get_reg(Registers::A));
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
    bus.write(0x010B, 0x03);

    // AND #3
    bus.write(addr, 0x2D); // AND - Absolute mode
    bus.write(addr + 1, 0x0B);  // Argument
    bus.write(addr + 2, 0x01);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0A);
    //cpu.debug_set_reg(Registers::X, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x02 in the A register?
    assert_eq!(0x02, cpu.debug_get_reg(Registers::A));
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

    // Manually put 0x0A into memory in the zero page
    bus.write(0x010C, 0x03);

    // AND #3
    bus.write(addr, 0x3D); // AND - Absolute, X mode
    bus.write(addr + 1, 0x0B);  // Argument
    bus.write(addr + 2, 0x01);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0A);
    cpu.debug_set_reg(Registers::X, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x02 in the A register?
    assert_eq!(0x02, cpu.debug_get_reg(Registers::A));
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

    // Manually put 0x0A into memory in the zero page
    bus.write(0x010C, 0x03);

    // AND #3
    bus.write(addr, 0x39); // AND - Absolute, X mode
    bus.write(addr + 1, 0x0B);  // Argument
    bus.write(addr + 2, 0x01);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x0A);
    cpu.debug_set_reg(Registers::Y, 0x01);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x02 in the A register?
    assert_eq!(0x02, cpu.debug_get_reg(Registers::A));
}