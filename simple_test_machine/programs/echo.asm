
; echo example - takes input and then prints it back out
; This is for testing I/O of the Simple Test Machine
; by Joey Pollack

; assemble with win2c64 (or lin2c64, or mac2c64) using the -R option
; win2c64 found here:
; https://www.aartbik.com/retro.php

; CONSTANTS
con_out         .equ    $1100   ; console output buffer address
con_in          .equ    $1200   ; console input buffer address
con_flags       .equ    $009A   ; console flags address
prt_str_flag    .equ    $0002   ; print string flag
prmp_input_flag .equ 	$0010   ; prompt for input flag

; MAIN
main    .org   $0200   ; program load address for the Simple Test Machine
        ldx    #0
loop    lda    text,x
        sta    con_out,x    
        inx
        cpx    #12		; length of text
        bne    loop 

        ; null terminate the string
        lda #0           
        sta con_out,x          

        ; Print the prompt message
        lda con_flags           ; load the console flags
        ora #prt_str_flag       ; set the print string flag
        sta con_flags           ; store the modified flags

        ; Prompt the user for input
        lda con_flags 		; load the console flags
        ora #prmp_input_flag	; set the prompt for input flag
        sta con_flags		; store the flags back to memory - this should trigger reading input

        ; Echo the input back to the console
        ; iterate the input buffer and move each byte to the output buffer
        ldx     #0              ; initialize the offset counter
loop2   lda     con_in,x        ; load byte from the input buffer, offset by x register
        sta     con_out,x       ; store byte into output buffer, offset by x register
        inx                     ; increment the offset
        cmp     #0              ; check for the null byte
        bne     loop2           ; branch back to loop2 if we are not at the null byte yet        

        lda con_flags           ; load the console flags
        ora #prt_str_flag       ; set the print string flag
        sta con_flags           ; store the modified flags

        ; End the program
        rts            ; 0x60 

; MEMORY
text            .byte  "Enter Text: "