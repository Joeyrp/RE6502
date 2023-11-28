


#![allow(dead_code, non_snake_case)]

// mod test_bus;

// #[cfg(test)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers};

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

    // Manually put 0x02 into memory in the zero page
    bus.write(0x000A, 0x02);

    // ORA #2
    bus.write(addr, 0x05); // ORA - Zero Page mode
    bus.write(addr +1, 0x0A);  // Argument (memory address of the value we want to OR with)

    // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x09);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0B in the A register?
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A));

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

    // Manually put 0x02 into memory in the zero page
    bus.write(0x000A, 0x02);

    // ORA #2
    bus.write(addr, 0x15); // ORA - Zero Page, X mode
    bus.write(addr + 1, 0x04);  // Argument (memory address of the value we want to OR with)

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x06);
    cpu.debug_set_reg(Registers::A, 0x09);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0B in the A register?
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A));

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

    // ORA #2
    bus.write(addr, 0x0D); // ORA - Absolute mode
    bus.write(addr + 1, 0x0A);  // Argument (memory address of the value we want to OR with)
    bus.write(addr + 2, 0x01);  // Argument (memory address of the value we want to OR with)

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    // cpu.debug_set_reg(Registers::X, 0x06);
    cpu.debug_set_reg(Registers::A, 0x09);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0B in the A register?
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A));

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

    // Manually put 0x02 into memory in the zero page
    bus.write(0x010B, 0x02);

    // ORA #2
    bus.write(addr, 0x1D); // ORA - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument (memory address of the value we want to OR with)
    bus.write(addr + 2, 0x01);  // Argument (memory address of the value we want to OR with)

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);
    cpu.debug_set_reg(Registers::A, 0x09);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0B in the A register?
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A));

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

    // ORA #2
    bus.write(addr, 0x19); // ORA - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument (memory address of the value we want to OR with)
    bus.write(addr + 2, 0x01);  // Argument (memory address of the value we want to OR with)

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::Y, 0x01);
    cpu.debug_set_reg(Registers::A, 0x09);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0B in the A register?
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A));

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

    // Manually put 0x02 into memory in the zero page
    bus.write(0x010B, 0x02);

    // Manually put 0x010B into the Zero Page
    bus.write(0x000B, 0x0B);
    bus.write(0x000C, 0x01);

    // ORA #2
    bus.write(addr, 0x01); // ORA - Indirect, X mode
    bus.write(addr + 1, 0x0A);  // Argument - Pointer to Zero Page

    // Restart cpu
    cpu.reset(&mut bus);

    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);  // Zero page pointer offset
    cpu.debug_set_reg(Registers::A, 0x09);


    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // Is 0x0B in the A register?
    assert_eq!(0x0B, cpu.debug_get_reg(Registers::A));

}
