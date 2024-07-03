

use std::str;

//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
//				USED MEMORY ADDRESSES
//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
const PROGRAM_START_ADDR: u16 = 0x0200;     // Program code starts on page 2
const CPU_RESET_START_ADDR: u16 = 0xFFFC;   // This is where the cpu looks for the address to start executing code at

pub const OUTPUT_BUF_ADDR: u16 = 0x1100;    // Output buffer -- Put values to be printed at this location!
pub const INPUT_BUF_ADDR: u16 = 0x1200;     // Input buffer

pub const CONSOLE_FLAGS_ADDR: u16 = 0x009A; // Grouping all of the console flags into a single byte

pub const PRINT_BYTE_FLAG: u16 = 0x009F;    // Then set one of these flags to trigger the print
pub const PRINT_STR_FLAG: u16 = 0x009E;     //      and indicate what type is being printed.

pub const READ_KB_FLAG: u16 = 0x009D;       // Set this address to 1 to request user input from the keyboard

/////////////////////////////////////////////////////////////////////
//				BUS
/////////////////////////////////////////////////////////////////////
use re6502::r6502::{R6502, Bus, Flags};
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
//				CONSOLE OUTPUT
//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||


struct OutputConsole
{

}

impl OutputConsole
{
    // fn new() -> Console
    // {
    //     Console { }
    // }

    fn clock(_cpu: &mut R6502, bus: &mut TBus )
    {
        // Check for a string to print
        let mut value = bus.read(PRINT_STR_FLAG);
        if value != 0
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
            bus.write(PRINT_STR_FLAG, 0);

            println!("{}", str::from_utf8(&msg).unwrap());
        }

        // Check for byte to print
        let flag = bus.read(PRINT_BYTE_FLAG);
        if flag != 0
        {
            let byte = bus.read(OUTPUT_BUF_ADDR);
            bus.write(PRINT_BYTE_FLAG, 0);
            println!("{}", byte as u8);
        }

    }
}

//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
//				CONSOLE INPUT
//|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||


struct InputConsole {}

impl InputConsole
{
    fn clock(_cpu: &mut R6502, bus: &mut TBus)
    {

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
            OutputConsole::clock(&mut self.cpu, &mut self.bus);

            // TODO: Check if the input flag is set (need to choose a memory location for it)
            //          if it's set, prompt for input and copy the input into memory (need an address for that too)
            //  ACTUALLY: Move that logic into a new module (InputConsole) and just call clock() (like OutputConsole)
        }
    }
}