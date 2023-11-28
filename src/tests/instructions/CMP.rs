
#![allow(dead_code, non_snake_case)]

use crate::tests::test_bus::RAMBus;
use crate::r6502::{R6502, Bus, Registers, Flags};

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

    
    ///////////////////////
    // Parameter is less than A reg

    // Manually put 0x05 into memory in the zero page
    bus.write(0x000A, 0x05);

    // Program to compare 0x10 with 0x05 (0x05 will be the argument)
    bus.write(addr, 0xC5); // CMP - Zero Page mode
    bus.write(addr + 1, 0x0A);  // Argument

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

    
    ///////////////////////
    // Parameter is equal to the A reg

    // Manually put 0x05 into memory in the zero page
    bus.write(0x000A, 0x10);

    // Program to compare 0x10 with 0x10
    bus.write(addr, 0xC5); // CMP - Zero Page mode
    bus.write(addr + 1, 0x0A);  // Argument

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
    
    ///////////////////////
    // Parameter is greater than A reg

     // Manually put 0x05 into memory in the zero page
     bus.write(0x000A, 0x10);

     // Program to compare 0x10 with 0x10
     bus.write(addr, 0xC5); // CMP - Zero Page mode
     bus.write(addr + 1, 0x0A);  // Argument
 
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

        
    ///////////////////////
    // Parameter is less than A reg

    // Manually put 0x05 into memory in the zero page
    bus.write(0x000A, 0x05);

    // Program to compare 0x05 and 0x10
    bus.write(addr, 0xD5); // CMP - Zero Page, X mode
    bus.write(addr + 1, 0x04);  // Argument

     // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x10);
    cpu.debug_set_reg(Registers::X, 0x06);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Flag should be 1, Z N Flags should be 0
    assert_eq!(1, cpu.check_flag(Flags::C));
    assert_eq!(0, cpu.check_flag(Flags::Z));
    assert_eq!(0, cpu.check_flag(Flags::N));

    ///////////////////////
    // Parameter is equal to the A reg

    // Manually put 0x10 into memory in the zero page
    bus.write(0x000A, 0x10);

    // Program to compare 0x05 and 0x10
    bus.write(addr, 0xD5); // CMP - Zero Page, X mode
    bus.write(addr + 1, 0x04);  // Argument

        // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x10);
    cpu.debug_set_reg(Registers::X, 0x06);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Z Flags should be 1, N Flag should be 0
    assert_eq!(1, cpu.check_flag(Flags::C));
    assert_eq!(1, cpu.check_flag(Flags::Z));
    assert_eq!(0, cpu.check_flag(Flags::N));
    
    ///////////////////////
    // Parameter is greater than A reg

    // Manually put 0x10 into memory in the zero page
    bus.write(0x000A, 0x10);

    // Program to compare 0x05 and 0x10
    bus.write(addr, 0xD5); // CMP - Zero Page, X mode
    bus.write(addr + 1, 0x04);  // Argument

        // Restart cpu
    cpu.reset(&mut bus);

    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::A, 0x05);
    cpu.debug_set_reg(Registers::X, 0x06);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Z Flags should be 0, N Flag should be 1
    assert_eq!(0, cpu.check_flag(Flags::C));
    assert_eq!(0, cpu.check_flag(Flags::Z));
    assert_eq!(1, cpu.check_flag(Flags::N));
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



    ///////////////////////
    // Parameter is less than A reg

    // Manually put 0x05 into memory
    bus.write(0x010A, 0x05);

    // Program to compare 0x05 and 0x10
    bus.write(addr, 0xCD); // CMP - Absolute mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

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

    ///////////////////////
    // Parameter is equal to the A reg

    // Manually put 0x10 into memory
    bus.write(0x010A, 0x10);

    // Program to compare 0x10 and 0x10
    bus.write(addr, 0xCD); // LDA - Absolute mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

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
    
    ///////////////////////
    // Parameter is greater than A reg

    // Manually put 0x10 into memory
    bus.write(0x010A, 0x10);

    // Program to compare 0x05 and 0x10
    bus.write(addr, 0xCD); // LDA - Absolute mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

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

    
    ///////////////////////
    // Parameter is less than A reg

    // Manually put 0x05 into memory
    bus.write(0x010B, 0x05);

    // Program to compare 0x05 to 0x10
    bus.write(addr, 0xDD); // CMP - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);
    cpu.debug_set_reg(Registers::A, 0x10);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Flag should be 1, Z N Flags should be 0
    assert_eq!(1, cpu.check_flag(Flags::C));
    assert_eq!(0, cpu.check_flag(Flags::Z));
    assert_eq!(0, cpu.check_flag(Flags::N));

    ///////////////////////
    // Parameter is equal to the A reg

    
    // Manually put 0x10 into memory
    bus.write(0x010B, 0x10);

    // Program to compare 0x05 to 0x10
    bus.write(addr, 0xDD); // CMP - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);
    cpu.debug_set_reg(Registers::A, 0x10);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Z Flags should be 1, N Flag should be 0
    assert_eq!(1, cpu.check_flag(Flags::C));
    assert_eq!(1, cpu.check_flag(Flags::Z));
    assert_eq!(0, cpu.check_flag(Flags::N));
    
    ///////////////////////
    // Parameter is greater than A reg

  
    // Manually put 0x05 into memory
    bus.write(0x010B, 0x10);

    // Program to compare 0x05 to 0x10
    bus.write(addr, 0xDD); // CMP - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::X, 0x01);
    cpu.debug_set_reg(Registers::A, 0x05);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Z Flags should be 0, N Flag should be 1
    assert_eq!(0, cpu.check_flag(Flags::C));
    assert_eq!(0, cpu.check_flag(Flags::Z));
    assert_eq!(1, cpu.check_flag(Flags::N));
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

    
    ///////////////////////
    // Parameter is less than A reg

    // Manually put 0x05 into memory
    bus.write(0x010B, 0x05);

    // Program to compare 0x05 to 0x10
    bus.write(addr, 0xD9); // CMP - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::Y, 0x01);
    cpu.debug_set_reg(Registers::A, 0x10);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Flag should be 1, Z N Flags should be 0
    assert_eq!(1, cpu.check_flag(Flags::C));
    assert_eq!(0, cpu.check_flag(Flags::Z));
    assert_eq!(0, cpu.check_flag(Flags::N));

    ///////////////////////
    // Parameter is equal to the A reg

    
    // Manually put 0x10 into memory
    bus.write(0x010B, 0x10);

    // Program to compare 0x05 to 0x10
    bus.write(addr, 0xD9); // CMP - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::Y, 0x01);
    cpu.debug_set_reg(Registers::A, 0x10);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Z Flags should be 1, N Flag should be 0
    assert_eq!(1, cpu.check_flag(Flags::C));
    assert_eq!(1, cpu.check_flag(Flags::Z));
    assert_eq!(0, cpu.check_flag(Flags::N));
    
    ///////////////////////
    // Parameter is greater than A reg

  
    // Manually put 0x05 into memory
    bus.write(0x010B, 0x10);

    // Program to compare 0x05 to 0x10
    bus.write(addr, 0xD9); // CMP - Absolute, X mode
    bus.write(addr + 1, 0x0A);  // Argument lo word
    bus.write(addr + 2, 0x01);  // Argument hi word

     // Restart cpu
    cpu.reset(&mut bus);
    
    // manually setup the cpu registers
    cpu.debug_set_reg(Registers::Y, 0x01);
    cpu.debug_set_reg(Registers::A, 0x05);

    // Clock the cpu to run the program (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);

    // C Z Flags should be 0, N Flag should be 1
    assert_eq!(0, cpu.check_flag(Flags::C));
    assert_eq!(0, cpu.check_flag(Flags::Z));
    assert_eq!(1, cpu.check_flag(Flags::N));
}