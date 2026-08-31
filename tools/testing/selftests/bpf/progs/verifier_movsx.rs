// SPDX-License-Identifier: GPL-2.0

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

// Original C condition:
// #if (defined(__TARGET_ARCH_arm64) || defined(__TARGET_ARCH_x86) || \
//      (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) || \
//      defined(__TARGET_ARCH_arm) || defined(__TARGET_ARCH_s390) || \
//      defined(__TARGET_ARCH_loongarch)) && \
//      __clang_major__ >= 18

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_trace_printk() -> i32;
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S8\")"]
#[doc = "__success __success_unpriv __retval(0x23)"]
#[naked]
pub unsafe extern "C" fn mov32sx_s8() {
    core::arch::naked_asm!(
        "w0 = 0xff23",
        "w0 = (s8)w0",
        "exit",
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S16\")"]
#[doc = "__success __success_unpriv __retval(0xFFFFff23)"]
#[naked]
pub unsafe extern "C" fn mov32sx_s16() {
    core::arch::naked_asm!(
        "w0 = 0xff23",
        "w0 = (s16)w0",
        "exit",
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV64SX, S8\")"]
#[doc = "__success __success_unpriv __retval(-2)"]
#[naked]
pub unsafe extern "C" fn mov64sx_s8() {
    core::arch::naked_asm!(
        "r0 = 0x1fe",
        "r0 = (s8)r0",
        "exit",
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV64SX, S16\")"]
#[doc = "__success __success_unpriv __retval(0xf23)"]
#[naked]
pub unsafe extern "C" fn mov64sx_s16() {
    core::arch::naked_asm!(
        "r0 = 0xf0f23",
        "r0 = (s16)r0",
        "exit",
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV64SX, S32\")"]
#[doc = "__success __success_unpriv __retval(-1)"]
#[naked]
pub unsafe extern "C" fn mov64sx_s32() {
    core::arch::naked_asm!(
        "r0 = 0xfffffffe",
        "r0 = (s32)r0",
        "r0 >>= 1",
        "exit",
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S8, range_check\")"]
#[doc = "__success __success_unpriv __retval(1)"]
#[naked]
pub unsafe extern "C" fn mov32sx_s8_range() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "w1 = (s8)w0",
        "/* w1 with s8 range */",
        "if w1 s> 0x7f goto 0f",
        "if w1 s< -0x80 goto 0f",
        "r0 = 1",
        "1:",
        "exit",
        "0:",
        "r0 = 2",
        "goto 1b",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S16, range_check\")"]
#[doc = "__success __success_unpriv __retval(1)"]
#[naked]
pub unsafe extern "C" fn mov32sx_s16_range() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "w1 = (s16)w0",
        "/* w1 with s16 range */",
        "if w1 s> 0x7fff goto 0f",
        "if w1 s< -0x80ff goto 0f",
        "r0 = 1",
        "1:",
        "exit",
        "0:",
        "r0 = 2",
        "goto 1b",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S16, range_check 2\")"]
#[doc = "__success __success_unpriv __retval(1)"]
#[naked]
pub unsafe extern "C" fn mov32sx_s16_range_2() {
    core::arch::naked_asm!(
        "r1 = 65535",
        "w2 = (s16)w1",
        "r2 >>= 1",
        "if r2 != 0x7fffFFFF goto 0f",
        "r0 = 1",
        "1:",
        "exit",
        "0:",
        "r0 = 0",
        "goto 1b",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV64SX, S8, range_check\")"]
#[doc = "__success __success_unpriv __retval(1)"]
#[naked]
pub unsafe extern "C" fn mov64sx_s8_range() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "r1 = (s8)r0",
        "/* r1 with s8 range */",
        "if r1 s> 0x7f goto 0f",
        "if r1 s< -0x80 goto 0f",
        "r0 = 1",
        "1:",
        "exit",
        "0:",
        "r0 = 2",
        "goto 1b",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV64SX, S16, range_check\")"]
#[doc = "__success __success_unpriv __retval(1)"]
#[naked]
pub unsafe extern "C" fn mov64sx_s16_range() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "r1 = (s16)r0",
        "/* r1 with s16 range */",
        "if r1 s> 0x7fff goto 0f",
        "if r1 s< -0x8000 goto 0f",
        "r0 = 1",
        "1:",
        "exit",
        "0:",
        "r0 = 2",
        "goto 1b",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV64SX, S32, range_check\")"]
#[doc = "__success __success_unpriv __retval(1)"]
#[naked]
pub unsafe extern "C" fn mov64sx_s32_range() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "r1 = (s32)r0",
        "/* r1 with s32 range */",
        "if r1 s> 0x7fffffff goto 0f",
        "if r1 s< -0x80000000 goto 0f",
        "r0 = 1",
        "1:",
        "exit",
        "0:",
        "r0 = 2",
        "goto 1b",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV64SX, S16, R10 Sign Extension\")"]
#[doc = "__failure __msg(\"R1 type=scalar expected=fp, pkt, pkt_meta, map_key, map_value, mem, ringbuf_mem, buf, trusted_ptr_\")"]
#[doc = "__failure_unpriv __msg_unpriv(\"R10 sign-extension part of pointer\")"]
#[naked]
pub unsafe extern "C" fn mov64sx_s16_r10() {
    core::arch::naked_asm!(
        "r1 = 553656332",
        "*(u32 *)(r10 - 8) = r1",
        "r1 = (s16)r10",
        "r1 += -8",
        "r2 = 3",
        "if r2 <= r1 goto 0f",
        "0:",
        "call {bpf_trace_printk}",
        "r0 = 0",
        "exit",
        bpf_trace_printk = sym bpf_trace_printk,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S8, var_off u32_max\")"]
#[doc = "__failure __msg(\"infinite loop detected\")"]
#[doc = "__failure_unpriv __msg_unpriv(\"back-edge from insn 2 to 0\")"]
#[naked]
pub unsafe extern "C" fn mov64sx_s32_varoff_1() {
    core::arch::naked_asm!(
        "0:",
        "r3 = *(u8 *)(r10 -387)",
        "w7 = (s8)w3",
        "if w7 >= 0x2533823b goto 0b",
        "w0 = 0",
        "exit",
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S8, var_off not u32_max, positive after s8 extension\")"]
#[doc = "__success __retval(0)"]
#[doc = "__success_unpriv"]
#[cfg_attr(SPEC_V1, doc = "__xlated_unpriv(\"w0 = 0\")")]
#[cfg_attr(SPEC_V1, doc = "__xlated_unpriv(\"exit\")")]
#[cfg_attr(SPEC_V1, doc = "__xlated_unpriv(\"nospec\") /* inserted to prevent `frame pointer is read only` */")]
#[cfg_attr(SPEC_V1, doc = "__xlated_unpriv(\"goto pc-1\")")]
#[naked]
pub unsafe extern "C" fn mov64sx_s32_varoff_2() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "r3 = r0",
        "r3 &= 0xf",
        "w7 = (s8)w3",
        "if w7 s>= 16 goto 0f",
        "w0 = 0",
        "exit",
        "0:",
        "r10 = 1",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S8, var_off not u32_max, negative after s8 extension\")"]
#[doc = "__success __retval(0)"]
#[doc = "__success_unpriv"]
#[cfg_attr(SPEC_V1, doc = "__xlated_unpriv(\"w0 = 0\")")]
#[cfg_attr(SPEC_V1, doc = "__xlated_unpriv(\"exit\")")]
#[cfg_attr(SPEC_V1, doc = "__xlated_unpriv(\"nospec\") /* inserted to prevent `frame pointer is read only` */")]
#[cfg_attr(SPEC_V1, doc = "__xlated_unpriv(\"goto pc-1\")")]
#[naked]
pub unsafe extern "C" fn mov64sx_s32_varoff_3() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "r3 = r0",
        "r3 &= 0xf",
        "r3 |= 0x80",
        "w7 = (s8)w3",
        "if w7 s>= -5 goto 0f",
        "w0 = 0",
        "exit",
        "0:",
        "r10 = 1",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV64SX, S8, unsigned range_check\")"]
#[doc = "__success __retval(0)"]
#[naked]
pub unsafe extern "C" fn mov64sx_s8_range_check() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "r0 &= 0x1",
        "r0 += 0xfe",
        "r0 = (s8)r0",
        "if r0 < 0xfffffffffffffffe goto 0f",
        "r0 = 0",
        "exit",
        "0:",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[link_section = "socket"]
#[doc = "__description(\"MOV32SX, S8, unsigned range_check\")"]
#[doc = "__success __retval(0)"]
#[naked]
pub unsafe extern "C" fn mov32sx_s8_range_check() {
    core::arch::naked_asm!(
        "call {bpf_get_prandom_u32}",
        "w0 &= 0x1",
        "w0 += 0xfe",
        "w0 = (s8)w0",
        "if w0 < 0xfffffffe goto 0f",
        "r0 = 0",
        "exit",
        "0:",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
    );
}

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "loongarch64"
)))]
#[link_section = "socket"]
#[doc = "__description(\"cpuv4 is not supported by compiler or jit, use a dummy test\")"]
#[doc = "__success"]
pub extern "C" fn dummy_test() -> i32 {
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
