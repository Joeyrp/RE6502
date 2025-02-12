

use std::{io, str};

//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
//				MEMORY ADDRESSES AND FLAG MASKS
//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||

// CPU Addresses
const PROGRAM_START_ADDR:   u16 = 0x0200;     // Program code starts on page 2
const CPU_RESET_START_ADDR: u16 = 0xFFFC;     // This is where the cpu looks for the address to start executing code at

// Console Addresses
pub const OUTPUT_BUF_ADDR:  u16 = 0x1100;     // Output buffer -- Put values to be printed at this location!
pub const INPUT_BUF_ADDR:   u16 = 0x1200;     // Input buffer
pub const INPUT_BUF_SIZE:   u16 = 0x00FF;     // Input buffer is 255 bytes
pub const CONSOLE_FLAGS_ADDR:   u16 = 0x009A; // Grouping all of the console flags into a single byte

// Console Flag Masks
pub const PRINT_BYTE_FLAG:      u8 = 0x01;    // Trigger printing a single byte
pub const PRINT_STR_FLAG:       u8 = 0x02;    // Trigger printing a string (continues printing bytes until a null byte is encountered)
pub const READ_LINE_FLAG:       u8 = 0x10;    // Set this flag to request user input from the keyboard
pub const READ_OVERFLOW_FLAG:   u8 = 0x20;    // This flag is set after reading input if there is too much input for the buffer

/////////////////////////////////////////////////////////////////////
//				BUS
/////////////////////////////////////////////////////////////////////

use re6502::r6502::{R6502, Bus, Flags};

// The Bus is how you connect other components to the RE6502 cpu.
// At minimium the read() and write() traits must be implement for the Bus.
// The following is a very basic Test Bus implementation.

struct TBus
{
    memory: [u8; 64 * 1024]
}

impl TBus
{
    pub fn new() -> TBus
    {
        TBus { memory: [0; 64 * 1024] }
    }

    pub fn clear_memory(&mut self)
    {
        for i in 0..self.memory.len()
        {
            self.memory[i] = 0;
        }
    }

    pub fn load_into_memory(&mut self, data: &[u8], at: u16)
    {
        for i in 0..data.len()
        {
            self.write(at + i as u16, data[i]);
        }
    }
}

impl Bus for TBus
{
    fn read(&self, addr: u16) -> u8 
    {
        self.memory[addr as usize]    
    }

    fn write(&mut self, addr: u16, value: u8) 
    {
        self.memory[addr as usize] = value;    
    }
}

//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
//				CONSOLE
//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
struct Console {}

impl Console
{
    fn clock(_cpu: &mut R6502, bus: &mut TBus )
    {
        Self::clock_output(_cpu, bus);
        Self::clock_input(_cpu, bus);
    }

    fn clock_output(_cpu: &mut R6502, bus: &mut TBus )
    {
        // Check for a string to print
        let mut value = bus.read(CONSOLE_FLAGS_ADDR);
        if (value & PRINT_STR_FLAG) != 0
        {
            let mut msg: Vec<u8> = Vec::new();
            
            let mut idx = 0;
            while value != 0
            {
                value = bus.read(OUTPUT_BUF_ADDR + idx);
                msg.push(value);
                idx += 1;
            }

            // Mark the string as empty again
            value &= !(PRINT_STR_FLAG);
            bus.write(CONSOLE_FLAGS_ADDR, value);

            println!("{}", str::from_utf8(&msg).unwrap());
        }

        // Check for byte to print
        else if (value & PRINT_BYTE_FLAG)!= 0
        {
            // read the byte
            let byte = bus.read(OUTPUT_BUF_ADDR);

            // reset the flag
            value &= !(PRINT_BYTE_FLAG);
            bus.write(CONSOLE_FLAGS_ADDR, value);

            // print the byte
            println!("{}", byte as u8);
        }

    }

    fn clock_input(_cpu: &mut R6502, bus: &mut TBus)
    {
        // Check input request flag
        let mut value = bus.read(CONSOLE_FLAGS_ADDR);
        if (value & READ_LINE_FLAG) != 0
        {
            // reset the flag
            value &= !(READ_LINE_FLAG);

            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buffer).expect("Failed to read input from the console");

            // Make sure the string will fit in the input buffer
            if (buffer.len() + 1) as u16 >= INPUT_BUF_SIZE
            { 
                // The input is too large for the buffer so
                //  set the overflow flag
                value |= READ_OVERFLOW_FLAG;
            }

            // Store input in the input buffer
            for (i, byte) in buffer.chars().enumerate()
            {
                // Truncate the input if it's too large for the buffer
                if i as u16 >= INPUT_BUF_SIZE
                {
                    break;
                }

                bus.write(INPUT_BUF_ADDR + (i as u16), byte as u8);
            }

            // Add the null byte to the end
            bus.write(INPUT_BUF_ADDR + buffer.len() as u16, 0);

            // store the flags back into memory
            bus.write(CONSOLE_FLAGS_ADDR, value);

        }
    }
}

/////////////////////////////////////////////////////////////////////
//				MACHINE
/////////////////////////////////////////////////////////////////////

pub struct TestMachine
{
    bus: TBus,
    cpu: R6502,

}

impl TestMachine
{
    pub fn new() -> TestMachine
    {
        TestMachine { bus: TBus::new(), cpu: R6502::new() }
    }

    pub fn reset(&mut self)
    {
        self.cpu.reset(&mut self.bus);
    }

    pub fn load_program(&mut self, program: &[u8])
    {
        self.bus.clear_memory();

        // Load program starting at page 2
        self.bus.load_into_memory(program, PROGRAM_START_ADDR);

        // Set the cpu program start address
        self.bus.write(CPU_RESET_START_ADDR, (PROGRAM_START_ADDR & 0x00FF) as u8);
        self.bus.write(CPU_RESET_START_ADDR + 1, ((PROGRAM_START_ADDR & 0xFF00) >> 8) as u8);

        self.reset();
    }

    pub fn run_program(&mut self)
    {
        // Program should run until the cpu detects that it has stopped
        // or when the program hits a BRK instruction
        while !self.cpu.is_program_stopped() && self.cpu.check_flag(Flags::B) == 0
        {
            self.cpu.clock(&mut self.bus);
            Console::clock(&mut self.cpu, &mut self.bus);
        }
    }
}