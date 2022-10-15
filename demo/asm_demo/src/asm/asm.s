.global test_p_add
.p2align 4
test_p_add:
    mov (%rdi),%eax
    add (%rsi),%eax
    retq
