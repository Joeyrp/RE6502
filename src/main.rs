


mod r6502;
use r6502::{R6502, Flags, Bus};

// All-RAM bus for testing
struct RAMBus
{
    ram: [u8; 64 * 1024]
}

impl RAMBus
{
    fn new() -> RAMBus
    {
        RAMBus { ram: [0; 64 * 1024] }
    }
}

impl Bus for RAMBus
{
    fn read(&self, addr: u16) -> u8 
    {
        self.ram[addr as usize]    
    }

    fn write(&mut self, addr: u16, value: u8) 
    {
        self.ram[addr as usize] = value;    
    }
}

fn main() 
{
    let mut cpu = R6502::new();

    println!("\nStatus flag testing...");
    cpu.set_flag(Flags::I);
    println!("\nI Flag is: {}", cpu.check_flag(Flags::I));
    
    cpu.clear_flag(Flags::I);
    println!("\nI Flag is: {}", cpu.check_flag(Flags::I));

    println!("\nRunning a very simple test program:\n\tLDA #9\n\tORA #2\n Result should be 11 in the A register");

    //////////////////////////
    // Setup Bus with program
    //////////////////////////

    let mut bus = RAMBus::new();

    // program address
    let addr = 0x0020 as u16;

    // Set the program counter address
    bus.write(0xFFFC, (addr & 0x00FF) as u8);  // low byte
    bus.write(0xFFFD, ((addr & 0xFF00) >> 8) as u8);  // high byte

    ///////////////////////////////
    // write the program to memory
    //////////////////////////////
    
    // LDA #9
    bus.write(addr, 0xA9); // LDA - Immediate mode
    bus.write(addr + 1, 0x09);  // Argument

    // ORA #2
    bus.write(addr + 2, 0x09); // ORA - Immediate mode
    bus.write(addr + 3, 0x02);  // Argument

    ////////////////////
    // Run the program!
    ////////////////////
    
    // Restart cpu
    cpu.reset(&mut bus);

    // Clock the cpu twice (Clock essentially runs one full instruction)
    cpu.clock(&mut bus);
    cpu.clock(&mut bus);

    println!("\nProgram result, A register: {}", cpu.a);

    println!("\nFinished.");
}
