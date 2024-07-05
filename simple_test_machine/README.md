
# The Simple Test Machine
The Simple Test Machine is a very basic virtual machine that uses the RE6502 as the cpu and a custom Console for simple I/O operations. There are example programs in the programs directory for reference/testing. The important memory addresses (I/O buffers, bit flags, etc) are listed in the src/machine.rs file.

# Running the Simple Test Machine
To build the project you can `cd simple_test_machine` then `cargo build` or `cargo run`. If you run it on it's own it will just run some basic internal test programs. To run a specific progam you can pass the name of the binary to load when the machine starts up. To do this with cargo try `cargo run -- path/to/the/program` (ex. `cargo run -- programs/bin/hello.rw`).

# Assembling Programs for the Simple Test Machine
Program binaries are not included in this repo but you can build them from the .asm files in the programs subdirectory. I've tested building these programs with the `win2c64` assembler (it also has linux `lin2c64` and mac `mac2c64` versions) that can be found here: https://www.aartbik.com/retro.php. 

To use the assembler you can invoke it directly or use the `assemble.bat` script in the programs subdirectory. 

## Using the assemble.bat script
To use the assemble.bat script you'll need to cd into the programs directory: `cd programs` (or, if you're in the project root: `cd simple_test_machine/programs`) then you can run the script with `.\assemble.bat hello.asm`. The script will move the resulting binary into the bin directory.

## Using win2c64 directly
You can invoke win2c64 directly with `win2c64.exe -R my_program.asm` The `-R` option is necessary to have the assembler output the binary in the correct format.

