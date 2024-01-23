
; Multiply 7 with 10 and print the result
; For testing this program was assembled with c64 (using the -R option):
; https://www.aartbik.com/MISC/c64.html

        ; FAST MULTIPLY program from:
        ; http://6502.org/source/integers/fastx10.htm
main    LDA #7      ; load 7 into the accumulator
        ASL         ;multiply by 2
        STA TEMP    ;temp store in TEMP
        ASL         ;again multiply by 2 (*4)
        ASL         ;again multiply by 2 (*8)
        CLC
        ADC TEMP    ;as result, A = x*8 + x*2

        ; PRINT RESULT
        STA $A0          ; 0x85, 0xA0, ; store A into the console output address
        LDX #0           ; 0xA2, 0x00, ; null terminator
        STX $A1          ; 0x86, 0xA1, ; store null terminator to output addr + 1

        ; Set flag to do the print
        LDX #1           ; 0xA2, 0x01,  
        STX $9F          ; 0x86, 0x9F, ; Print byte flag is at 0x9F

        ; End the program
        RTS            ; 0x60 

        ; Variables
        TEMP    .byte 0
