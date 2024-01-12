

#![allow(dead_code, non_snake_case)]

use super::{R6502, Bus, Flags, addressing_modes::{AddressingModes, ModeID}};

// Instruction decoding:
// https://llx.com/Neil/a2/opcodes.html

// GROUP ONE
// 000	ORA
// 001	AND
// 010	EOR
// 011	ADC
// 100	STA
// 101	LDA
// 110	CMP
// 111	SBC

pub struct Instructions;

impl Instructions
{
    pub const GROUP_ONE_OPS: [fn(&mut R6502, &mut dyn Bus); 8] = [
            Instructions::ORA, 
            Instructions::AND,
            Instructions::EOR,
            Instructions::ADC,
            Instructions::STA,
            Instructions::LDA,
            Instructions::CMP,
            Instructions::SBC,
        ];

        pub const GROUP_TWO_OPS: [fn(&mut R6502, &mut dyn Bus); 8] = [
            Instructions::ASL, // 000	
            Instructions::ROL, // 001	
            Instructions::LSR, // 010	
            Instructions::ROR, // 011	
            Instructions::STX, // 100	
            Instructions::LDX, // 101	
            Instructions::DEC, // 110	
            Instructions::INC, // 111	
        ];

        pub const GROUP_THREE_OPS: [fn(&mut R6502, &mut dyn Bus); 8] = [
            Instructions::ERR,
            Instructions::BIT,  // 001	BIT
            Instructions::JMP,  // 010	JMP
            Instructions::JMP,  // 011	JMP (abs)
            Instructions::STY,  // 100	STY
            Instructions::LDY,  // 101	LDY
            Instructions::CPY,  // 110	CPY
            Instructions::CPX,  // 111	CPX
        ];

        pub const GROUP_BRANCHING_OPS: [fn(&mut R6502, &mut dyn Bus); 8] = [
            Instructions::BPL,	
            Instructions::BMI,	
            Instructions::BVC,	
            Instructions::BVS,	
            Instructions::BCC,	
            Instructions::BCS,
            Instructions::BNE,
            Instructions::BEQ,
        ];
       
       
        
}

impl Instructions
{
    pub fn ERR(_cpu: &mut R6502, _bus: &mut dyn Bus)
    {
        // TODO: Better error handling
        println!("ERROR: Invalid Instruction");
    }

    ///////////////////////////////////////////////////////////
    // GROUP ONE
    pub fn ORA(cpu: &mut R6502, _bus: &mut dyn Bus)
    {
        let data = cpu.working_data as u8;
        cpu.a = cpu.a | data;
        if cpu.a == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.a & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn AND(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let data = cpu.working_data as u8;
        cpu.a = cpu.a & data;
        if cpu.a == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.a & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn EOR(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let data = cpu.working_data as u8;
        cpu.a = cpu.a ^ data;
        if cpu.a == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.a & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    // Using a technique written javidx9
    // The code in this function falls under the License (OLC-3) SEE LICENSE FILE
    // https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp#L659
    pub fn ADC(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let carry = cpu.check_flag(Flags::C) as u16;

        // 16 bit addition to capture the carry easier
        let temp: u16 = cpu.a as u16 + cpu.working_data + carry;

        if temp > 255
        {
            cpu.set_flag(Flags::C);
        }
        
        if temp == 0
        {
            cpu.set_flag(Flags::Z);
        }

        let did_overflow = (!((cpu.a as u16) ^ (cpu.working_data as u16)) & ((cpu.a as u16) ^ temp)) & 0x0080;
        cpu.clear_flag(Flags::V);
        if did_overflow > 0
        {
            cpu.set_flag(Flags::V);
        }

        cpu.clear_flag(Flags::N);
        if temp & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

        cpu.a = (temp & 0x00FF) as u8;
    }

    pub fn STA(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        bus.write(cpu.working_addr, cpu.a);
    }

    pub fn LDA(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let data = cpu.working_data as u8;
        cpu.a = data;

        if cpu.a == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.a & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn CMP(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let data = cpu.working_data as u8;
        if cpu.a >= data
        {
            cpu.set_flag(Flags::C);
        }
        else
        {
            cpu.clear_flag(Flags::C);
        }

        if cpu.a == data
        {
            cpu.set_flag(Flags::Z);
        }
        else
        {
            cpu.clear_flag(Flags::Z);
        }

        if cpu.a < data
        {
            cpu.set_flag(Flags::N);
        }
        else
        {
            cpu.clear_flag(Flags::N);
        }
    }

    // Using a technique written javidx9
    // The code in this function falls under the License (OLC-3) SEE LICENSE FILE
    // https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp#L714
    //
    // More info about the carry bit:
    // http://forum.6502.org/viewtopic.php?t=18
    pub fn SBC(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let value = cpu.working_data ^ 0x00FF;
        let carry = cpu.check_flag(Flags::C) as u16;

        let temp: u16 = cpu.a as u16 + value + carry;

        cpu.clear_flag(Flags::C);
        if temp > 255
        {
            cpu.set_flag(Flags::C);
        }
        
        cpu.clear_flag(Flags::Z);
        if temp == 0
        {
            cpu.set_flag(Flags::Z);
        }

        let did_overflow = (!((cpu.a as u16) ^ (value)) & ((cpu.a as u16) ^ temp)) & 0x0080;
        cpu.clear_flag(Flags::V);
        if did_overflow > 0
        {
            cpu.set_flag(Flags::V);
        }

        cpu.clear_flag(Flags::N);
        if temp & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

        cpu.a = (temp & 0x00FF) as u8;
    }

    ///////////////////////////////////////////////////////////
    // GROUP TWO
    pub fn ASL(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.clear_flag(Flags::C);
        if cpu.working_data as u8 & 0x80 > 0
        {
            cpu.set_flag(Flags::C);
        }

        let result = cpu.working_data << 1;

        cpu.clear_flag(Flags::Z);
        if result == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if result as u8 & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

        if cpu.addr_mode == ModeID::ACM
        {
            cpu.a = result as u8;
        }
        else
        {
            bus.write(cpu.working_addr, result as u8);
        }
    }

    pub fn ROL(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let old_bit_7 = (cpu.working_data & 0x80) > 0;
        let carry = cpu.check_flag(Flags::C) as u16;
        let result = (cpu.working_data << 1) ^ carry;

        cpu.clear_flag(Flags::C);
        if old_bit_7
        {
            cpu.set_flag(Flags::C);
        }

        cpu.clear_flag(Flags::N);
        if result & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

        cpu.clear_flag(Flags::Z);
        if result == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.addr_mode == ModeID::ACM
        {
            cpu.a = result as u8;
        }
        else
        {
            bus.write(cpu.working_addr, result as u8);
        }
        
    }

    pub fn LSR(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let old_bit_0 = (cpu.working_data & 0x01) > 0;
        let carry = cpu.check_flag(Flags::C) as u16;
        let result = cpu.working_data >> 1;

        cpu.clear_flag(Flags::C);
        if old_bit_0
        {
            cpu.set_flag(Flags::C);
        }

        cpu.clear_flag(Flags::N);
        if result & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

        cpu.clear_flag(Flags::Z);
        if result == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.addr_mode == ModeID::ACM
        {
            cpu.a = result as u8;
        }
        else
        {
            bus.write(cpu.working_addr, result as u8);
        }
    }

    pub fn ROR(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let old_bit_0 = (cpu.working_data & 0x01) > 0;
        let carry = cpu.check_flag(Flags::C) as u16;
        let temp = carry << 7;
        let result = (cpu.working_data >> 1) ^ (carry << 7);

        cpu.clear_flag(Flags::C);
        if old_bit_0
        {
            cpu.set_flag(Flags::C);
        }

        cpu.clear_flag(Flags::N);
        if result & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

        cpu.clear_flag(Flags::Z);
        if result == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.addr_mode == ModeID::ACM
        {
            cpu.a = result as u8;
        }
        else
        {
            bus.write(cpu.working_addr, result as u8);
        }
    }

    pub fn STX(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        bus.write(cpu.working_addr, cpu.x);
    }

    pub fn LDX(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let data = cpu.working_data as u8;
        cpu.x = data;

        if cpu.x == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.x & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn DEC(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let dec_val = bus.read(cpu.working_addr) - 1;
        bus.write(cpu.working_addr, dec_val);

        cpu.clear_flag(Flags::Z);
        if dec_val == 0
        {
            cpu.set_flag(Flags::Z);
        }

        cpu.clear_flag(Flags::N);
        if dec_val & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

    }

    pub fn INC(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let dec_val = bus.read(cpu.working_addr) + 1;
        bus.write(cpu.working_addr, dec_val);

        cpu.clear_flag(Flags::Z);
        if dec_val == 0
        {
            cpu.set_flag(Flags::Z);
        }

        cpu.clear_flag(Flags::N);
        if dec_val & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    ///////////////////////////////////////////////////////////
    // GROUP THREE
    pub fn BIT(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.set_flag(Flags::Z);
        if cpu.a & (cpu.working_data as u8) > 0
        {
            cpu.clear_flag(Flags::Z);
        }

        cpu.clear_flag(Flags::V);
        if cpu.working_data & 0x0040 > 0
        {
            cpu.set_flag(Flags::V);
        }

        cpu.clear_flag(Flags::N);
        if cpu.working_data & 0x0080 > 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn JMP(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.pc = cpu.working_addr;
    }

    // JMP (abs)
    // pub fn JPA(cpu: &mut R6502, bus: &mut dyn Bus)
    // {
    //     cpu.pc = cpu.working_addr;

    // }

    pub fn STY(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        bus.write(cpu.working_addr, cpu.y);
    }

    pub fn LDY(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        let data = cpu.working_data as u8;
        cpu.y = data;

        if cpu.y == 0
        {
            cpu.set_flag(Flags::Z);
        }

        if cpu.y & 0x80 != 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn CPY(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.clear_flag(Flags::C);
        if cpu.y as u16 >= cpu.working_data
        {
            cpu.set_flag(Flags::C);
        }

        cpu.clear_flag(Flags::Z);
        if cpu.y == cpu.working_data as u8
        {
            cpu.set_flag(Flags::Z);
        }

        cpu.clear_flag(Flags::N);
        if (cpu.y as u16 - cpu.working_data) & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }
    }

    pub fn CPX(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        cpu.clear_flag(Flags::C);
        if cpu.x as u16 >= cpu.working_data
        {
            cpu.set_flag(Flags::C);
        }

        cpu.clear_flag(Flags::Z);
        if cpu.x == cpu.working_data as u8
        {
            cpu.set_flag(Flags::Z);
        }

        cpu.clear_flag(Flags::N);
        if (cpu.x as u16 - cpu.working_data) & 0x80 > 0
        {
            cpu.set_flag(Flags::N);
        }

    }

    ///////////////////////////////////////////////////////////
    // BRANCHING
    fn BPL(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        if cpu.check_flag(Flags::N) == 0
        {
            cpu.pc += cpu.working_data;
        }
    }	

    fn BMI(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        if cpu.check_flag(Flags::N) != 0
        {
            cpu.pc += cpu.working_data;
        }
    }	

    fn BVC(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        if cpu.check_flag(Flags::V) == 0
        {
            cpu.pc += cpu.working_data;
        }
    }	

    fn BVS(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        if cpu.check_flag(Flags::V) != 0
        {
            cpu.pc += cpu.working_data;
        }
    }	

    fn BCC(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        if cpu.check_flag(Flags::C) == 0
        {
            cpu.pc += cpu.working_data;
        }
    }	

    fn BCS(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        if cpu.check_flag(Flags::C) != 0
        {
            cpu.pc += cpu.working_data;
        }
    } 

    fn BNE(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        if cpu.check_flag(Flags::Z) == 0
        {
            cpu.pc += cpu.working_data;
        }
    }	

    fn BEQ(cpu: &mut R6502, bus: &mut dyn Bus)
    {
        if cpu.check_flag(Flags::Z) != 0
        {
            cpu.pc += cpu.working_data;
        }
    }

    ///////////////////////////////////////////////////////////
    // INTERRUPT AND SUBROUTINE

    ///////////////////////////////////////////////////////////
    // SINGLE BYTE

}