
; hello world example
; for win2c64 by Aart Bik
; http://www.aartbik.com/

; Adapted for the RE6502 emulator simple test machine
; compile with win2c64 using the -R option

strout          .equ    $1100   ; console output address
con_flags       .equ    $009A   ; console flags address
prt_str_flag    .equ    $0002

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
        lda con_flags
        ora #prt_str_flag 
        sta con_flags   ; 0x86, 0x9A, ; Print string flag is at 0x9A

        ; End the program
        rts            ; 0x60 

        ; Variables
text            .byte  "HELLO WORLD"