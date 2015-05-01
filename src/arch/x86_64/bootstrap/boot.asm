; when grub starts this file the processor is still in 32-bit mode
BITS 32

; the entry point to our kernel (specified in the linker)
global start


; Declare the multiboot 2 header. The linker will ensure that this
; section is at the start of the final executable. See the multiboot
; specification at [1] for details.
; [1] http://nongnu.askapache.com/grub/phcoder/multiboot.pdf
section .multiboot
align 8
header_start:
    dd 0xe85250d6                   ; magic number (multiboot 2)
    dd 0                            ; architecture 0 (protected mode i386)
    dd header_end - header_start    ; header length
    ; checksum
    dd 0xffffffff - (0xe85250d6 + 0 + (header_end - header_start)) + 1   

    ; insert multiboot tags here if needed (but remember to update header 
    ; lenght and checksum)

    ; special end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:


; the standard code section, but still in 32-bit mode
section .text
; the entry point that will be called by grub
start:
    ; load a new GDT as we cannot trust grub's GDT
    mov eax, gdt32_pointer
    lgdt [eax]    
    mov esp, stack      ; load stack pointer
    push 0x8            ; code segment in the gdt
    push .gdt32_code_segment_ready 
    retf                ; jump using code segment in gdt32

.gdt32_code_segment_ready:
    ; reload segment registers
    mov eax, 0x10   ; data gdt segment
    mov ds, ax
    mov ss, ax

.gdt32_ready:
    ; now we have switched to our new gdt32
    hlt


; section for read only data
section .rodata
gdt32:
    DQ  0x0000000000000000
    DQ  0x00CF9A000000FFFF
    DQ  0x00CF92000000FFFF
gdt32_pointer:
    DW  23
    DD  gdt32


; section for read/write data
section .data
; stack 
stack_bottom:
times 0x1000 db 0 
stack: