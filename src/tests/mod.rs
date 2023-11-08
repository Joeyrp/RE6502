
#![allow(dead_code, non_snake_case)]

mod test_bus;


#[cfg(test)]
mod instructions 
{
    use super::test_bus::RAMBus;
    use crate::r6502::{R6502, Bus, Registers};

    //////////////////////////////////////////////////////////////////
    /// ORA     ORA     ORA     ORA
    ////////////////////////////////////////////////////////////////// 

    #[test]
    fn ORA_IMM()
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
    fn ORA_ZP0()
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
    fn ORA_ZPX()
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
    fn ORA_ABS()
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
    fn ORA_ABX()
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
    fn ORA_ABY()
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

    //////////////////////////////////////////////////////////////////
    /// AND     AND     AND     AND
    ////////////////////////////////////////////////////////////////// 

    #[test]
    fn AND_IMM()
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
    fn AND_ZP0()
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
    fn AND_ZPX()
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
    fn AND_ABS()
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
    fn AND_ABX()
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
    fn AND_ABY()
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

    //////////////////////////////////////////////////////////////////
    /// LDA     LDA     LDA     LDA
    ////////////////////////////////////////////////////////////////// 

    #[test]
    fn LDA_IMM() 
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
    fn LDA_ZP0() 
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
    fn LDA_ZPX() 
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
    fn LDA_ABS() 
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
    fn LDA_ABX() 
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
    fn LDA_ABY() 
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
}