// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/raw_stack.c */

// C includes translated as dependency intent:
// <linux/bpf.h>
// <bpf/bpf_helpers.h>
// "bpf_misc.h"

extern "C" {
    fn bpf_skb_load_bytes();
}

#[link_section = "socket"]
// __description("raw_stack: no skb_load_bytes")
// __success
// __failure_unpriv __msg_unpriv("invalid read from stack R6 off=-8 size=8")
#[no_mangle]
pub unsafe extern "C" fn stack_no_skb_load_bytes() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -8",
        "r3 = r6",
        "r4 = 8",
        "/* Call to skb_load_bytes() omitted. */",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, negative len")
// __failure __msg("R4 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn skb_load_bytes_negative_len() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -8",
        "r3 = r6",
        "r4 = -8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, negative len 2")
// __failure __msg("R4 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_negative_len_2() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -8",
        "r3 = r6",
        "r4 = {imm_0}",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        imm_0 = const !0u64,
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, zero len")
// __failure __msg("R4 invalid zero-sized read: u64=[0,0]")
#[no_mangle]
pub unsafe extern "C" fn skb_load_bytes_zero_len() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -8",
        "r3 = r6",
        "r4 = 0",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, no init")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn skb_load_bytes_no_init() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -8",
        "r3 = r6",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, init")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn stack_skb_load_bytes_init() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -8",
        "r3 = 0xcafe",
        "*(u64*)(r6 + 0) = r3",
        "r3 = r6",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, spilled regs around bounds")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn bytes_spilled_regs_around_bounds() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -16",
        "*(u64*)(r6 - 8) = r1",
        "*(u64*)(r6 + 8) = r1",
        "r3 = r6",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 - 8)",
        "r2 = *(u64*)(r6 + 8)",
        "r0 = *(u32*)(r0 + {sk_buff_mark})",
        "r2 = *(u32*)(r2 + {sk_buff_priority})",
        "r0 += r2",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        sk_buff_mark = const 0,     // offsetof(struct __sk_buff, mark)
        sk_buff_priority = const 0, // offsetof(struct __sk_buff, priority)
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, spilled regs corruption")
// __failure __msg("R0 invalid mem access 'scalar'")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn load_bytes_spilled_regs_corruption() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -8",
        "*(u64*)(r6 + 0) = r1",
        "r3 = r6",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "r0 = *(u32*)(r0 + {sk_buff_mark})",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        sk_buff_mark = const 0, // offsetof(struct __sk_buff, mark)
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, spilled regs corruption 2")
// __failure __msg("R3 invalid mem access 'scalar'")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn bytes_spilled_regs_corruption_2() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -16",
        "*(u64*)(r6 - 8) = r1",
        "*(u64*)(r6 + 0) = r1",
        "*(u64*)(r6 + 8) = r1",
        "r3 = r6",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 - 8)",
        "r2 = *(u64*)(r6 + 8)",
        "r3 = *(u64*)(r6 + 0)",
        "r0 = *(u32*)(r0 + {sk_buff_mark})",
        "r2 = *(u32*)(r2 + {sk_buff_priority})",
        "r0 += r2",
        "r3 = *(u32*)(r3 + {sk_buff_pkt_type})",
        "r0 += r3",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        sk_buff_mark = const 0,     // offsetof(struct __sk_buff, mark)
        sk_buff_pkt_type = const 0, // offsetof(struct __sk_buff, pkt_type)
        sk_buff_priority = const 0, // offsetof(struct __sk_buff, priority)
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, spilled regs + data")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_bytes_spilled_regs_data() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -16",
        "*(u64*)(r6 - 8) = r1",
        "*(u64*)(r6 + 0) = r1",
        "*(u64*)(r6 + 8) = r1",
        "r3 = r6",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 - 8)",
        "r2 = *(u64*)(r6 + 8)",
        "r3 = *(u64*)(r6 + 0)",
        "r0 = *(u32*)(r0 + {sk_buff_mark})",
        "r2 = *(u32*)(r2 + {sk_buff_priority})",
        "r0 += r2",
        "r0 += r3",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        sk_buff_mark = const 0,     // offsetof(struct __sk_buff, mark)
        sk_buff_priority = const 0, // offsetof(struct __sk_buff, priority)
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, invalid access 1")
// __failure __msg("invalid write to stack R3 off=-513 size=8")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_1() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -513",
        "r3 = r6",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, invalid access 2")
// __failure __msg("invalid write to stack R3 off=-1 size=8")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_2() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -1",
        "r3 = r6",
        "r4 = 8",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, invalid access 3")
// __failure __msg("R4 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_3() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += 0xffffffff",
        "r3 = r6",
        "r4 = 0xffffffff",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, invalid access 4")
// __failure
// __msg("R4 unbounded memory access, use 'var &= const' or 'if (var < const)'")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_4() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -1",
        "r3 = r6",
        "r4 = 0x7fffffff",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, invalid access 5")
// __failure
// __msg("R4 unbounded memory access, use 'var &= const' or 'if (var < const)'")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_5() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -512",
        "r3 = r6",
        "r4 = 0x7fffffff",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, invalid access 6")
// __failure __msg("invalid zero-sized read")
#[no_mangle]
pub unsafe extern "C" fn load_bytes_invalid_access_6() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -512",
        "r3 = r6",
        "r4 = 0",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "tc"]
// __description("raw_stack: skb_load_bytes, large access")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn skb_load_bytes_large_access() {
    core::arch::asm!(
        "r2 = 4",
        "r6 = r10",
        "r6 += -512",
        "r3 = r6",
        "r4 = 512",
        "call {bpf_skb_load_bytes}",
        "r0 = *(u64*)(r6 + 0)",
        "exit",
        bpf_skb_load_bytes = sym bpf_skb_load_bytes,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
