use machine::{OUTPUT_ADDR, PRINT_STR_FLAG, PRINT_BYTE_FLAG, TestMachine};


mod machine;

fn main()
{
    hello_world_test();
    println!();
    fast_mult_by_10();
}

fn hello_world_test()
{
    let print_flag_addr = (PRINT_STR_FLAG & 0x00FF) as u8;
    let output_addr = (OUTPUT_ADDR & 0x00FF) as u8;
    let program = 
    [
        // Load string into memory at the output address
        0xA2, b'H',             // LDX H
        0x86, output_addr,      // STX
        0xA2, b'e',             // LDX e
        0x86, output_addr + 1,  // STX
        0xA2, b'l',             // LDX l
        0x86, output_addr + 2,  // STX
        0xA2, b'l',             // LDX l
        0x86, output_addr + 3,  // STX
        0xA2, b'o',             // LDX o
        0x86, output_addr + 4,  // STX

        0xA2, b' ',             // LDX ' '
        0x86, output_addr + 5,  // STX

        0xA2, b'w',             // LDX w
        0x86, output_addr + 6,  // STX
        0xA2, b'o',             // LDX o
        0x86, output_addr + 7,  // STX
        0xA2, b'r',             // LDX r
        0x86, output_addr + 8,  // STX
        0xA2, b'l',             // LDX l
        0x86, output_addr + 9,  // STX
        0xA2, b'd',             // LDX d
        0x86, output_addr + 10, // STX
        0xA2, b'!',             // LDX !
        0x86, output_addr + 11, // STX

        0xA2, 0x00,             // LDX 0
        0x86, output_addr + 12, // STX

        // Set flag to do the print
        0xA2, 0x01,             // LDX 1
        0x86, print_flag_addr,  // STX

        // End the program
        0x60                    // RTS
    ];

    let mut vm = TestMachine::new();

    vm.load_program(&program);
    vm.reset();
    vm.run_program();

    println!("Program stopped");


}

fn fast_mult_by_10()
{
    // Program from:
    // http://6502.org/source/integers/fastx10.htm
    
    // MULT10  
    //     ASL         ;multiply by 2
    //     STA TEMP    ;temp store in TEMP
    //     ASL         ;again multiply by 2 (*4)
    //     ASL         ;again multiply by 2 (*8)
    //     CLC
    //     ADC TEMP    ;as result, A = x*8 + x*2
    //     RTS
    //
    //     TEMP    .byte 0

    let print_flag_addr = (PRINT_BYTE_FLAG & 0x00FF) as u8;
    let output_addr = (OUTPUT_ADDR & 0x00FF) as u8;
    let temp_addr: u8 = 0xB0;

    let program = 
    [
        0xA9, 0x07,         // LDA 7    - The value we want to multiply

        // START OF MULT10 FUNCTION
        0x0A,               // ASL ;multiply by 2
        0x85, temp_addr,    // STA TEMP ;temp store in TEMP
        0x0A,               // ASL ;again multiply by 2 (*4)
        0x0A,               // ASL ;again multiply by 2 (*8)
        0x18,               // CLC
        0x65, temp_addr,    // ADC TEMP ;as result, A = x*8 + x*2

        // PRINT RESULT
        0x85, output_addr,      // STA output addr        
        0xA2, 0x00,             // LDX 0 - null terminator
        0x86, output_addr + 1,  // STX

        // Set flag to do the print
        0xA2, 0x01,             // LDX 1
        0x86, print_flag_addr,  // STX

        // End the program
        0x60                    // RTS

    ];

    let mut vm = TestMachine::new();

    vm.load_program(&program);
    vm.reset();
    vm.run_program();

    println!("Program stopped");

}