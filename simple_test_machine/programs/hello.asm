
; hello world example
; for win2c64 by Aart Bik
; http://www.aartbik.com/

; Adapted for the RE6502 emulator simple test machine

strout          .equ   $00A0   ; console output address
print_flag      .equ   $009E   ; console output address

main    .org   $0200   ; program load address for the simple test machine
        ldx    #0
loop    lda    text,x
        sta    strout,x    
        inx
        cpx    #11
        bne    loop 

        ; null terminate the string
        lda #0           
        sta strout,x          

        ; Set flag to do the print
        ldx #1           ; 0xA2, 0x01,  
        stx print_flag   ; 0x86, 0x9E, ; Print string flag is at 0x9E

        ; End the program
        rts            ; 0x60 

        ; Variables
text    .byte  "HELLO WORLD"