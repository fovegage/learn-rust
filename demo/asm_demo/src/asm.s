.text ; 我们编写的代码在.text节中
;.code64 ; 使用的是64位汇编代码

.global nop_func ; 需要导出的函数名
.p2align 4 ; .p2align 4 意思为在16字节边界上对齐 具体定义可参考 https://sourceware.org/binutils/docs/as/P2align.html#P2align
nop_func:
    nop ; 不做任何操作，空指令
    ret ; 函数返回
