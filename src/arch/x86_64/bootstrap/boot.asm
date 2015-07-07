; when grub starts this file the processor is still in 32-bit mode
BITS 32

; the entry point to our kernel (specified in the linker)
global start
; used by rust code
global tss

; the code for setting up paging (in setup_paging.asm)
extern setup_paging
; called after paging setup
extern long_mode_init


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
    ; Now we have switched to our new gdt32. The next step is to enable
    ; long mode (64 bit). Therefor we must setup paging.
    call setup_paging

; now we need a 64-bit GDT
.load_gdt64:
    mov eax, gdt64_pointer
    lgdt [eax]

.switch_to_64bit_code:
    push 0x08
    push long_mode_init
    retf


; section for read only data
section .rodata
; the 32-bit GDT
gdt32:
    DQ  0x0000000000000000
    DQ  0x00CF9A000000FFFF
    DQ  0x00CF92000000FFFF
gdt32_pointer:
    DW  23
    DD  gdt32

; the 64-bit GDT
gdt64:
    DQ  0x0000000000000000
    DQ  0x00A09A0000000000  ;ring 0 code
    DQ  0x00A0920000000000  ;ring 0 data
    DQ  0x00A0FA0000000000  ;ring 3 code
    DQ  0x00A0F20000000000  ;ring 3 data
    ; tss (16 byte big in 64bit mode)
    DW 0x0067 ; limit
    DW (tss-0x200000) ; the warning "word data exceeds bounds" is ok here...
    DB 0x20 ;base middle
    DB 0x89 ;present + (type = `non busy tss`)
    DW 0x0  ;upper byte: base high; lower byte: flags + limit middle
    DQ 0x0  ;base high
    DQ 0x0  ;reserved

gdt64_pointer:
    DW  55
    DD  gdt64
    DD  0


; special section for tss to ensure a low address (see linker.ld)
section .tss
tss:
    DD 0            ; reserved
    times 3 DQ 0    ; rsp {0,1,2}
    DQ 0            ; reserved
    times 7 DQ 0    ; IST{1-7}
    DQ 0            ; reserved
    DW 0            ; reserved
    DW 0            ; io map base address


; section for read/write data
section .data
; a small stack (only for bootstrap)
stack_bottom:
times 0x1000 db 0
stack:
