
; hello world example
; for win2c64 by Aart Bik
; http://www.aartbik.com/

; Adapted for the RE6502 emulator simple test machine
; compile with win2c64 using the -R option

; This example shows how to print a string to the console

strout          .equ    $1100   ; console output address
con_flags       .equ    $009A   ; console flags address
prt_str_flag    .equ    $0002   ; the print flag bitmask

main    .org   $0200   ; program load address for the simple test machine

        ; Copy the string into the console output buffer
        ldx    #0           ; initialize the offset counter
loop    lda    text,x       ; load byte from the text buffer, offset by x register   
        sta    strout,x     ; store byte into the output buffer, offset by x register
        inx                 ; increment the offset counter
        cpx    #11          ; check if we've reached the end of the string - text is 11 bytes long
        bne    loop         ; branch if we haven't reached the end yet

        ; null terminate the string
        lda #0           
        sta strout,x          

        ; Set flag to do the print
        lda con_flags           ; load flags into accumulator
        ora #prt_str_flag       ; set the print string flag
        sta con_flags           ; store flags back into memory - this will trigger the print

        ; End the program
        rts            ; 0x60 

        ; Variables
text            .byte  "HELLO WORLD"