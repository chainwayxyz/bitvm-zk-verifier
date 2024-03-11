.section .text
.global _start

_start:
    la sp, __stack_init$  /* Load the address of __stack_init$ into the stack pointer (sp) */

    # Call custom function 'main'
    call main

    # Move return value from main to x28
    mv x28, a0

    # Exit
    li a0, 0      # Exit code
    li a7, 93     # System call for exit in RISC-V
    ecall

.section .bss
    .align  2
