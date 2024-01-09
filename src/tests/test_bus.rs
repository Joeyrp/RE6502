use crate::r6502::Bus;

// All-RAM bus for testing
pub struct RAMBus
{
    ram: [u8; 64 * 1024]
}

impl RAMBus
{
    pub fn new() -> RAMBus
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