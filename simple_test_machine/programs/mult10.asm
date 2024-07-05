
; Multiply 7 with 10 and print the result
; For testing this program was assembled with c64 (using the -R option):
; https://www.aartbik.com/MISC/c64.html

strout          .equ    $1100   ; console output address
con_flags       .equ    $009A   ; console flags address
prt_str_flag    .equ    $0001   ; print string flag

        ; FAST MULTIPLY program from:
        ; http://6502.org/source/integers/fastx10.htm
main    LDA #7      ; load 7 into the accumulator
        ASL         ; multiply by 2
        STA TEMP    ; temp store in TEMP
        ASL         ; again multiply by 2 (*4)
        ASL         ; again multiply by 2 (*8)
        CLC
        ADC TEMP    ; as result, A = x*8 + x*2

        ; print result
        STA strout      ; store A into the console output address
        LDX #0          ; 0xA2, 0x00, ; null terminator
        STX strout+1    ; store null terminator to output addr + 1

        ; Set flag to do the print
        LDA con_flags     ; load the current console flag set
        ORA #prt_str_flag ; turn on the print string flag
        STA con_flags     ; store the flags back in memory

        ; End the program
        RTS            ; 0x60 

        ; Variables
        TEMP    .byte 0
