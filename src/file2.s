.intel_syntax noprefix

.text
        .global _asm_function, asm_function

asm_function:
_asm_function:
        lea     rdi, [rip + message]
        sub     rsp, 8
        call    _puts
        add     rsp, 8
        ret

.data
message:
    .asciz "asm function"
