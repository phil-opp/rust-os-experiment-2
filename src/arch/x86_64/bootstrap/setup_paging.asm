; we are still in 32bit-mode
BITS 32

; will be called by boot.asm
global setup_paging

; identity map first 8*2=16 Megabyte
%define p1_tables 8

; declare page table %1
%macro create_page_table 1
    align 4096
    %1:
    times 4096 db 0
%endmacro

; link entry %2 of page table %1 to page table %3
%macro link_page_table_entry 3
    mov eax, %3
    or eax, 1       ; present bit
    mov [%1 + %2 * 8], eax
%endmacro


section .text

setup_paging:
    call link_page_tables
    call fill_p1_tables
    call unmap_null_page
    call recursive_map
    call map_page_frame_stack_tables
    call enable_paging
    ret                     ; to boot.asm

link_page_tables:
    ; link P4[0] to P3
    link_page_table_entry P4, 0, P3
    ; link P3[0] to P2
    link_page_table_entry P3, 0, P2
    ; link P2[i] to P1_i
    %assign i 0
    %rep p1_tables
        link_page_table_entry P2, i, P1_%[i]
    %assign i i+1
    %endrep
    ret

; identity map the first 8*4 Kb
fill_p1_tables:
    %assign i 0
    %rep p1_tables
        mov edi, P1_%[i]
        mov eax, 0x200000 * i   ; 2MB per P1 table
        call fill_p1_table
    %assign i i+1
    %endrep
    ret

; map the P1 table in edi to the 2MB following the address in eax
fill_p1_table:
    mov ecx, 512                ; loop variable: 512 page table entries
    or eax, 0x3                 ; present + read/write flags
.loop:
    mov dword [edi], eax        ; map current entry to address in eax
    mov dword [edi + 4], 0      ; higher bits of the address are 0
    add edi, 8                  ; current entry <- next entry
    add eax, 4096               ; current address <- next address (current+4kB)
    sub ecx, 1                  ; decrement loop variable
    jnz .loop
.done:
    ret

; unmap the page containing 0 to catch null pointer dereferences
unmap_null_page:
    mov dword [P1_0], 0
    mov dword [P1_0 + 4], 0
    ret

; map the page tables to the highest addresses for easier access
recursive_map:
    mov eax, P4
    or eax, 1
    mov [P4 + 511*8], eax
    ret

; map the page tables required for the page frame stack
map_page_frame_stack_tables:
    link_page_table_entry P3, 1, page_frame_stack_P2
    link_page_table_entry page_frame_stack_P2, 0, page_frame_stack_P1

enable_paging:
    ; load P4 to cr3 register (cpu uses this to access the first page table)
    mov eax, P4
    mov cr3, eax

    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; enable long mode in the respective MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret


section .data
; page tables (P4=PML4, P3=PDPT, P2=PD, P1=PT)
create_page_table P4
create_page_table P3
create_page_table P2

%assign i 0
%rep p1_tables
    create_page_table P1_%[i]
%assign i i+1
%endrep

create_page_table page_frame_stack_P2;
create_page_table page_frame_stack_P1;
