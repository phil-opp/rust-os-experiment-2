; finally 64bit mode
BITS 64

; will be called after paging setup
global long_mode_init

extern setup_interrupt_table

; reserved in the linker
extern kernel_stack_bottom
extern kernel_stack_top

; rust entry
extern main


section .text
long_mode_init:
    call reload_segment_registers
    call setup_SSE
    call remap_PIC
    call reprogram_timer
    call init_fs_and_gs
    call setup_interrupt_table

start_kernel:
    ; set stack limit (bottom + 20 * 1024 (red zone))
    ; see std/sys/common/stack.rs
    mov qword [fs:0x70], kernel_stack_bottom + 20*1024

    mov rdi, rbx                ; multiboot structure
    mov rsp, kernel_stack_top   ; switch to kernel stack

    call main

    ; kernel returned
    call clear_screen_green
.hang:
    cli
    hlt
    jmp .hang


reload_segment_registers:
    mov eax, 0x10   ; ring 0 data gdt segment
    mov ss, ax
    mov eax, 0x20   ; ring 3 data gdt segment
    mov ds, ax
    mov es, ax

    mov eax, 0x28   ; tss segment
    ltr ax          ; “load task register“

    ret

; enable SSE and the like
setup_SSE:
    mov rax, cr0
    and ax, 0xFFFB      ; clear coprocessor emulation CR0.EM
    or ax, 0x2          ; set coprocessor monitoring  CR0.MP
    mov cr0, rax
    mov rax, cr4
    or ax, 3 << 9       ; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
    mov cr4, rax

    ret

remap_PIC:
    in al, 0x21                   ; save pic1 mask
    mov cl, al
    in al, 0xA1                   ; save pic2 mask
    mov ch, al

    mov al, 0x11
    out 0x20, al                ; send initialize command to pic1
    out 0xA0, al                ; send initialize command to pic2

    mov al, 0x20
    out 0x21, al                ; set vector offset of pic1 to 0x20
    mov al, 0x28
    out 0xA1, al                ; set vector offset of pic2 to 0x28

    mov al, 4
    out 0x21, al                   ; tell pic1 that there is a slave PIC at IRQ2 (0000 0100)
    mov al, 2
    out 0xA1, al                   ; tell pic2 its cascade identity (0000 0010)

    mov al, 0x1
    out 0x21, al                 ; 8086 mode for pic1
    out 0xA1, al                 ; 8086 mode for pic2

    mov al, cl
    out 0x21, al                  ; restore pic1 mask
    mov al, ch
    out 0xA1, al                  ; restore pic2 mask

    ret

reprogram_timer:
    mov rcx, 1193180/100        ; divisor (100 Hz)

    mov al, 0x36
    out 0x43, al                ; set channel 0 data register + mode bits

    mov al, cl
    out 0x40, al                ; set low divisor byte

    mov al, ch
    out 0x40, al                ; set high divisor byte

    ret

clear_screen_green:
    mov edi, 0xB8000              ; Set the destination index to 0xB8000.
    mov rax, 0x2F202F202F202F20   ; Set the A-register to 0x1F201F201F201F20.
    mov ecx, 500                  ; Set the C-register to 500.
    rep stosq                     ; Clear the screen.

    ret

init_fs_and_gs:
    ; set fs
    mov edx, 0
    mov eax, fs_struct
    mov ecx, 0xC0000100
    wrmsr

    ; set gs
    mov edx, 0
    mov eax, gs_struct
    mov ecx, 0xC0000101
    wrmsr

    ret

section .data
align 4096
fs_struct:
    times 0x1000 db 0
gs_struct:
    times 0x1000 db 0
