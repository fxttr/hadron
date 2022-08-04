;; Copyright (c) 2022, Florian Büstgens
;; All rights reserved.
;;
;; Redistribution and use in source and binary forms, with or without
;; modification, are permitted provided that the following conditions are met:
;;     1. Redistributions of source code must retain the above copyright
;;        notice, this list of conditions and the following disclaimer.
;;
;;     2. Redistributions in binary form must reproduce the above copyright notice,
;;        this list of conditions and the following disclaimer in the
;;        documentation and/or other materials provided with the distribution.
;;
;; THIS SOFTWARE IS PROVIDED BY Florian Büstgens ''AS IS'' AND ANY
;; EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
;; WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
;; DISCLAIMED. IN NO EVENT SHALL Florian Büstgens BE LIABLE FOR ANY
;; DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
;; (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
;; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
;; ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
;; (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
;; SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
                    
	
BASE equ 0xFFFFFFFF80000000 ; Relocate at -2GB
	
;; TEXT ==++
segment .itext exec
[BITS 16]
global _start
_start:
	;; Enable A20 line
	pushf
	push ds
	push es
	push di
	push si

	cli
	
	xor ax, ax
	mov es, ax		; Clear extra segment

	not ax
	mov ds, ax		; Setting data segment to 0xFFFF

	mov di, 0x0500
	mov si, 0x0510

	mov al, byte[es:di]
	push ax

	mov al, byte[ds:si]
	push ax

	mov byte [es:di], 0x00
	mov byte [ds:si], 0xFF
	
	cmp byte [es:di], 0xFF
	
	pop ax
	mov byte [ds:si], al
	
	pop ax
	mov byte [es:di], al
	
	mov ax, 0		; if ax is 0 the a20 line is disabled, else it is already enabled
	jne .exit		; If ax is not 0, jump to exit, we are done here.
	
.fast_a20_gate:
	in al, 0x92
	or al, 2
	out 0x92, al
.exit:
	pop si
	pop di
	pop es
	pop ds
	popf

	jmp _32_start
	
[BITS 32]
_32_start:
	mov eax, 0x80000000	; Prepare protected mode
	cpuid
	cmp eax, 0x80000001	; Check for CPUID
	jbe .error		; Our CPU doesn't support 64 Bit. uxOS does not support anything else yet.

	mov eax, 0x80000001	; Check for CPUID IA-32e.
	cpuid
	test edx, 0x20000000
	jz .error

	mov eax, cr4		; Setting PGE, PAE, PSE state
	or eax, (0x80|0x20|0x10)
	mov cr4, eax

	mov eax, (ipml4 - BASE)	; init page tables
	mov cr3, eax

	mov ecx, 0xC0000080	; Enable IA-32e (+ Syscall, NX)
	rdmsr
	or eax, ((1 << 11)|(1 << 8)|(1 << 0))
	wrmsr
	

	mov eax, cr0 		; Enable Paging
	and eax, 01111111111111111111111111111111b
	mov cr0, eax

	lgdt [rel GDT_bottom - BASE]
	jmp _init_long_start
	
.error:
	jmp .error

[bits 64]
global _init_long_start
_init_long_start:
;; Prepare 64 bit
	lgdt [rel GDT_high]
	jmp _long_start

segment .text
extern _kmain
global _long_start
_long_start:
	mov rax, 0
	mov [rel ipml4], rax

	mov ax, 0x10
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	lea rsp, [rel init_stack]

	call _kmain
	
	cli			 ; Disable interrupts, preparing to go down.
.hang:
	hlt			 ; Infinite loop
	jmp .hang
.end:				 ; Bye


;; TEXT ==++

;; PADATA ==++
segment .padata write
;; Here we'll init paging.
ipml4:
	dq pdpt_bottom - BASE + 3
%rep 512 - 3
	dq 0
%endrep
	dq 0
	dq ipdpt - BASE + 3
	
pdpt_bottom:
	dq ipd - BASE + 3
%rep 512 - 1
	dq 0
%endrep

ipdpt:
%rep 512 - 2
	dq 0
%endrep
	dq ipd - BASE + 3
	dq 0

ipd:
	dq 0x000000 + 0x80 + 3
	dq 0x200000 + 0x80 + 3
%rep 521 - 2
	dq 0
%endrep

init_stack_b:
%rep 0x1000 * 2
	db 0
%endrep
init_stack:
;; PADATA ==++

;; DATA ==++
	; Align our stack.
segment .data 

GDT_bottom:			; Global Descriptor Table
	dw GDTEnd - GDT - 1
	dq GDT - BASE
GDT_high:
	dw GDTEnd - GDT -1
	dq GDT
global GDT
GDT:
	dd 0, 0
        dd 0x00000000, 0x00209A00 ; 64 Bit Code
        dd 0x00000000, 0x00009200 ; 64 Bit Data
        dd 0x00000000, 0x0040FA00 ; 32 Bit Userspace
        dd 0x00000000, 0x0040F200 ; Userspace
        dd 0x00000000, 0x0020FA00 ; 64 Bit Userspace
        dd 0x00000000, 0x0000F200
GDTEnd:
	
;; DATA ==++
