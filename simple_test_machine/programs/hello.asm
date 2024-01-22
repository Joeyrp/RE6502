        # Load string into memory at the output address
        0xA2, b'H',             # LDX H
        0x86, output_addr,      # STX
        0xA2, b'e',             # LDX e
        0x86, output_addr + 1,  # STX
        0xA2, b'l',             # LDX l
        0x86, output_addr + 2,  # STX
        0xA2, b'l',             # LDX l
        0x86, output_addr + 3,  # STX
        0xA2, b'o',             # LDX o
        0x86, output_addr + 4,  # STX

        0xA2, b' ',             # LDX ' '
        0x86, output_addr + 5,  # STX

        0xA2, b'w',             # LDX w
        0x86, output_addr + 6,  # STX
        0xA2, b'o',             # LDX o
        0x86, output_addr + 7,  # STX
        0xA2, b'r',             # LDX r
        0x86, output_addr + 8,  # STX
        0xA2, b'l',             # LDX l
        0x86, output_addr + 9,  # STX
        0xA2, b'd',             # LDX d
        0x86, output_addr + 10, # STX
        0xA2, b'!',             # LDX !
        0x86, output_addr + 11, # STX

        0xA2, 0x00,             # LDX 0
        0x86, output_addr + 12, # STX

        # Set flag to do the print
        0xA2, 0x01,             # LDX 1
        0x86, print_flag_addr,  # STX

        # End the program
        0x60                    # RTS