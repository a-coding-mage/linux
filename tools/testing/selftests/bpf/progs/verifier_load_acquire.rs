// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Google LLC. */

// C dependencies removed from executable Rust:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>,
// "../../../include/linux/filter.h", and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use core::arch::asm;

pub type __u64 = u64;

unsafe extern "C" {
    fn bpf_rdonly_cast(ptr: *const core::ffi::c_void, flags: u64) -> *mut core::ffi::c_void;
}

// The following tests are enabled in C only when CAN_USE_LOAD_ACQ_STORE_REL is
// defined by the BPF test build environment.

// SEC("socket")
// __description("load-acquire, 8-bit")
// __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_8() {
    unsafe {
        asm!(
            "r0 = 0;",
            "w1 = 0xfe;",
            "*(u8 *)(r10 - 1) = w1;",
            ".8byte {load_acquire_insn}", // w2 = load_acquire((u8 *)(r10 - 1));
            "if r2 == r1 goto 1f;",
            "r0 = 1;",
            "1:",
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -1),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("load-acquire, 16-bit")
// __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_16() {
    unsafe {
        asm!(
            "r0 = 0;",
            "w1 = 0xfedc;",
            "*(u16 *)(r10 - 2) = w1;",
            ".8byte {load_acquire_insn}", // w2 = load_acquire((u16 *)(r10 - 2));
            "if r2 == r1 goto 1f;",
            "r0 = 1;",
            "1:",
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_H, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -2),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("load-acquire, 32-bit")
// __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_32() {
    unsafe {
        asm!(
            "r0 = 0;",
            "w1 = 0xfedcba09;",
            "*(u32 *)(r10 - 4) = w1;",
            ".8byte {load_acquire_insn}", // w2 = load_acquire((u32 *)(r10 - 4));
            "if r2 == r1 goto 1f;",
            "r0 = 1;",
            "1:",
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_W, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -4),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("load-acquire, 64-bit")
// __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_64() {
    unsafe {
        asm!(
            "r0 = 0;",
            "r1 = 0xfedcba0987654321 ll;",
            "*(u64 *)(r10 - 8) = r1;",
            ".8byte {load_acquire_insn}", // r2 = load_acquire((u64 *)(r10 - 8));
            "if r2 == r1 goto 1f;",
            "r0 = 1;",
            "1:",
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_10, -8),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("load-acquire with uninitialized src_reg")
// __failure __failure_unpriv __msg("R2 !read_ok")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_with_uninitialized_src_reg() {
    unsafe {
        asm!(
            ".8byte {load_acquire_insn}", // r0 = load_acquire((u64 *)(r2 + 0));
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_2, 0),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("load-acquire with non-pointer src_reg")
// __failure __failure_unpriv __msg("R1 invalid mem access 'scalar'")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_with_non_pointer_src_reg() {
    unsafe {
        asm!(
            "r1 = 0;",
            ".8byte {load_acquire_insn}", // r0 = load_acquire((u64 *)(r1 + 0));
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_1, 0),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("misaligned load-acquire")
// __failure __failure_unpriv __msg("misaligned stack access off")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn load_acquire_misaligned() {
    unsafe {
        asm!(
            "r1 = 0;",
            "*(u64 *)(r10 - 8) = r1;",
            ".8byte {load_acquire_insn}", // w0 = load_acquire((u32 *)(r10 - 5));
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_W, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_10, -5),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("load-acquire from ctx pointer")
// __failure __failure_unpriv __msg("BPF_ATOMIC loads from R1 ctx is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_ctx_pointer() {
    unsafe {
        asm!(
            ".8byte {load_acquire_insn}", // w0 = load_acquire((u8 *)(r1 + 0));
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_1, 0),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("load-acquire from ctx pointer, same dst and src register")
// __failure __failure_unpriv __msg("BPF_ATOMIC loads from R6 ctx is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_ctx_same_dst_src() {
    unsafe {
        asm!(
            "r6 = r1;",
            ".8byte {load_acquire_insn}", // w6 = load_acquire((u32 *)(r6 + 0));
            "r0 = 0;",
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_W, BPF_LOAD_ACQ, BPF_REG_6, BPF_REG_6, 0),
            options(noreturn)
        );
    }
}

// SEC("xdp")
// __description("load-acquire from pkt pointer")
// __failure __msg("BPF_ATOMIC loads from R2 pkt is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_pkt_pointer() {
    unsafe {
        asm!(
            "r2 = *(u32 *)(r1 + {xdp_md_data});",
            "r3 = *(u32 *)(r1 + {xdp_md_data_end});",
            "r1 = r2;",
            "r1 += 8;",
            "if r1 >= r3 goto l0_0;",
            ".8byte {load_acquire_insn}", // w0 = load_acquire((u8 *)(r2 + 0));
            "l0_0:  r0 = 0;",
            "exit;",
            xdp_md_data = const offset_of!(xdp_md, data),
            xdp_md_data_end = const offset_of!(xdp_md, data_end),
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_2, 0),
            options(noreturn)
        );
    }
}

// SEC("flow_dissector")
// __description("load-acquire from flow_keys pointer")
// __failure __msg("BPF_ATOMIC loads from R2 flow_keys is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_flow_keys_pointer() {
    unsafe {
        asm!(
            "r2 = *(u64 *)(r1 + {__sk_buff_flow_keys});",
            ".8byte {load_acquire_insn}", // w0 = load_acquire((u8 *)(r2 + 0));
            "exit;",
            __sk_buff_flow_keys = const offset_of!(__sk_buff, flow_keys),
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_2, 0),
            options(noreturn)
        );
    }
}

// SEC("sk_reuseport")
// __description("load-acquire from sock pointer")
// __failure __msg("BPF_ATOMIC loads from R2 sock is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_sock_pointer() {
    unsafe {
        asm!(
            "r2 = *(u64 *)(r1 + {sk_reuseport_md_sk});",
            // w0 = load_acquire((u8 *)(r2 + offsetof(struct bpf_sock, family)));
            ".8byte {load_acquire_insn};",
            "exit;",
            sk_reuseport_md_sk = const offset_of!(sk_reuseport_md, sk),
            load_acquire_insn = const BPF_ATOMIC_OP(
                BPF_B,
                BPF_LOAD_ACQ,
                BPF_REG_0,
                BPF_REG_2,
                offset_of!(bpf_sock, family)
            ),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("load-acquire from rdonly_untrusted_mem pointer")
// __failure __msg("BPF_ATOMIC loads from R{{[0-9]+}} rdonly_untrusted_mem is not allowed")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_from_rdonly_untrusted_mem(ctx: *mut core::ffi::c_void) -> i32 {
    let mut val: __u64 = 0;
    let p: *mut core::ffi::c_void;

    /*
     * bpf_rdonly_cast(x, 0) yields PTR_TO_MEM | MEM_RDONLY | PTR_UNTRUSTED.
     * A regular BPF_LDX from it is rewritten to BPF_PROBE_MEM, but a
     * load-acquire is not, so it must be rejected, otherwise the JIT emits
     * a plain load with no exception table entry and a fault would crash
     * the kernel.
     */
    p = unsafe { bpf_rdonly_cast((&mut val as *mut __u64).cast(), 0) };
    unsafe {
        asm!(
            "r1 = {p};",
            ".8byte {load_acquire_insn}", // r0 = load_acquire((u64 *)(r1 + 0));
            p = in(reg) p,
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_0, BPF_REG_1, 0),
            out("r0") _,
            out("r1") _,
        );
    }
    0
}

// SEC("socket")
// __description("load-acquire with invalid register R15")
// __failure __failure_unpriv __msg("R15 is invalid")
#[no_mangle]
pub unsafe extern "C" fn load_acquire_with_invalid_reg() {
    unsafe {
        asm!(
            ".8byte {load_acquire_insn}", // r0 = load_acquire((u64 *)(r15 + 0));
            "exit;",
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_0, 15 /* invalid reg */, 0),
            options(noreturn)
        );
    }
}

// Else branch for C builds without CAN_USE_LOAD_ACQ_STORE_REL:
// SEC("socket")
// __description("Clang version < 18, ENABLE_ATOMICS_TESTS not defined, and/or JIT doesn't support load-acquire, use a dummy test")
// __success
#[no_mangle]
pub extern "C" fn dummy_test() -> i32 {
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
