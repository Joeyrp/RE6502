#![allow(dead_code, non_snake_case)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers};



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
    //bus.write(0x000B, 0x0A);

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

    // STA
    bus.write(addr, 0x95); // STA - Zero Page, X mode
    bus.write(addr + 1, 0x04);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x06);
    cpu.debug_set_reg(Registers::A, 0x0F);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0F at memory address 0x0A?
    assert_eq!(0x0F, bus.read(0x0A));

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

    // Manually put 0x02 into memory in the zero page
    bus.write(0x010A, 0x02);

    // STA
    bus.write(addr, 0x8D); // STA - Absolute mode
    bus.write(addr + 1, 0x0A);  // Argument
    bus.write(addr + 2, 0x01);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    // cpu.debug_set_reg(Registers::X, 0x06);
    cpu.debug_set_reg(Registers::A, 0x0F);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0F at memory address 0x010A?
    assert_eq!(0x0F, bus.read(0x010A));

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

    // STA
    bus.write(addr, 0x9D); // STA - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument
    bus.write(addr + 2, 0x01);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);
    cpu.debug_set_reg(Registers::A, 0x0F);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0F at memory address 0x010B?
    assert_eq!(0x0F, bus.read(0x010B));

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

    // Manually put 0x02 into memory in the zero page
    bus.write(0x010B, 0x02);

    // STA
    bus.write(addr, 0x99); // STA - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument
    bus.write(addr + 2, 0x01);  // Argument

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::Y, 0x01);
    cpu.debug_set_reg(Registers::A, 0x0F);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0F at memory address 0x010B?
    assert_eq!(0x0F, bus.read(0x010B));

}