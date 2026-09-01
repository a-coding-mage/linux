// SPDX-License-Identifier: GPL-2.0

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::arch::global_asm;

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
// #include <bpf_arena_common.h>
//
// Original build condition:
// (defined(__TARGET_ARCH_arm64) || defined(__TARGET_ARCH_x86) ||
//  (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) ||
//  defined(__TARGET_ARCH_arm) || defined(__TARGET_ARCH_s390) ||
//  defined(__TARGET_ARCH_loongarch)) && __clang_major__ >= 18

#[repr(C)]
pub struct arena {
    // __uint(type, BPF_MAP_TYPE_ARENA);
    // __uint(map_flags, BPF_F_MMAPABLE);
    // __uint(max_entries, 1);
    _unused: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static arena: arena = arena { _unused: [] };

// SEC("socket")
// __description("LDSX, S8")
// __success __success_unpriv __retval(-2)
// __naked void ldsx_s8(void)
global_asm!(
    r#"
    .section socket,"ax"
    .global ldsx_s8
ldsx_s8:
    r1 = 0x3fe;
    *(u64 *)(r10 - 8) = r1;
    /* little endian: r0 = *(s8 *)(r10 - 8); */
    /* big endian:    r0 = *(s8 *)(r10 - 1); */
    r0 = *(s8 *)(r10 - 8);
    exit;
"#
);

// SEC("socket")
// __description("LDSX, S16")
// __success __success_unpriv __retval(-2)
// __naked void ldsx_s16(void)
global_asm!(
    r#"
    .section socket,"ax"
    .global ldsx_s16
ldsx_s16:
    r1 = 0x3fffe;
    *(u64 *)(r10 - 8) = r1;
    /* little endian: r0 = *(s16 *)(r10 - 8); */
    /* big endian:    r0 = *(s16 *)(r10 - 2); */
    r0 = *(s16 *)(r10 - 8);
    exit;
"#
);

// SEC("socket")
// __description("LDSX, S32")
// __success __success_unpriv __retval(-1)
// __naked void ldsx_s32(void)
global_asm!(
    r#"
    .section socket,"ax"
    .global ldsx_s32
ldsx_s32:
    r1 = 0xfffffffe;
    *(u64 *)(r10 - 8) = r1;
    /* little endian: r0 = *(s32 *)(r10 - 8); */
    /* big endian:    r0 = *(s32 *)(r10 - 4); */
    r0 = *(s32 *)(r10 - 8);
    r0 >>= 1;
    exit;
"#
);

// SEC("socket")
// __description("LDSX, S8 range checking, privileged")
// __log_level(2) __success __retval(1)
// __msg("R1=scalar(smin=smin32=-128,smax=smax32=127)")
// __naked void ldsx_s8_range_priv(void)
global_asm!(
    r#"
    .section socket,"ax"
    .global ldsx_s8_range_priv
ldsx_s8_range_priv:
    call bpf_get_prandom_u32;
    *(u64 *)(r10 - 8) = r0;
    /* little endian: r1 = *(s8 *)(r10 - 8); */
    /* big endian:    r1 = *(s8 *)(r10 - 1); */
    r1 = *(s8 *)(r10 - 8);
    /* r1 with s8 range */
    if r1 s> 0x7f goto l0_ldsx_s8_range_priv;
    if r1 s< -0x80 goto l0_ldsx_s8_range_priv;
    r0 = 1;
l1_ldsx_s8_range_priv:
    exit;
l0_ldsx_s8_range_priv:
    r0 = 2;
    goto l1_ldsx_s8_range_priv;
"#
);

// SEC("socket")
// __description("LDSX, S16 range checking")
// __success __success_unpriv __retval(1)
// __naked void ldsx_s16_range(void)
global_asm!(
    r#"
    .section socket,"ax"
    .global ldsx_s16_range
ldsx_s16_range:
    call bpf_get_prandom_u32;
    *(u64 *)(r10 - 8) = r0;
    /* little endian: r1 = *(s16 *)(r10 - 8); */
    /* big endian:    r1 = *(s16 *)(r10 - 2); */
    r1 = *(s16 *)(r10 - 8);
    /* r1 with s16 range */
    if r1 s> 0x7fff goto l0_ldsx_s16_range;
    if r1 s< -0x8000 goto l0_ldsx_s16_range;
    r0 = 1;
l1_ldsx_s16_range:
    exit;
l0_ldsx_s16_range:
    r0 = 2;
    goto l1_ldsx_s16_range;
"#
);

// SEC("socket")
// __description("LDSX, S32 range checking")
// __success __success_unpriv __retval(1)
// __naked void ldsx_s32_range(void)
global_asm!(
    r#"
    .section socket,"ax"
    .global ldsx_s32_range
ldsx_s32_range:
    call bpf_get_prandom_u32;
    *(u64 *)(r10 - 8) = r0;
    /* little endian: r1 = *(s32 *)(r10 - 8); */
    /* big endian:    r1 = *(s32 *)(r10 - 4); */
    r1 = *(s32 *)(r10 - 8);
    /* r1 with s16 range */
    if r1 s> 0x7fffFFFF goto l0_ldsx_s32_range;
    if r1 s< -0x80000000 goto l0_ldsx_s32_range;
    r0 = 1;
l1_ldsx_s32_range:
    exit;
l0_ldsx_s32_range:
    r0 = 2;
    goto l1_ldsx_s32_range;
"#
);

// SEC("xdp")
// __description("LDSX, xdp s32 xdp_md->data")
// __failure __msg("invalid bpf_context access")
// __naked void ldsx_ctx_1(void)
global_asm!(
    r#"
    .section xdp,"ax"
    .global ldsx_ctx_1
ldsx_ctx_1:
    r2 = *(s32 *)(r1 + xdp_md_data);
    r0 = 0;
    exit;
"#
);

// SEC("xdp")
// __description("LDSX, xdp s32 xdp_md->data_end")
// __failure __msg("invalid bpf_context access")
// __naked void ldsx_ctx_2(void)
global_asm!(
    r#"
    .section xdp,"ax"
    .global ldsx_ctx_2
ldsx_ctx_2:
    r2 = *(s32 *)(r1 + xdp_md_data_end);
    r0 = 0;
    exit;
"#
);

// SEC("xdp")
// __description("LDSX, xdp s32 xdp_md->data_meta")
// __failure __msg("invalid bpf_context access")
// __naked void ldsx_ctx_3(void)
global_asm!(
    r#"
    .section xdp,"ax"
    .global ldsx_ctx_3
ldsx_ctx_3:
    r2 = *(s32 *)(r1 + xdp_md_data_meta);
    r0 = 0;
    exit;
"#
);

// SEC("tcx/ingress")
// __description("LDSX, tcx s32 __sk_buff->data")
// __failure __msg("invalid bpf_context access")
// __naked void ldsx_ctx_4(void)
global_asm!(
    r#"
    .section tcx/ingress,"ax"
    .global ldsx_ctx_4
ldsx_ctx_4:
    r2 = *(s32 *)(r1 + sk_buff_data);
    r0 = 0;
    exit;
"#
);

// SEC("tcx/ingress")
// __description("LDSX, tcx s32 __sk_buff->data_end")
// __failure __msg("invalid bpf_context access")
// __naked void ldsx_ctx_5(void)
global_asm!(
    r#"
    .section tcx/ingress,"ax"
    .global ldsx_ctx_5
ldsx_ctx_5:
    r2 = *(s32 *)(r1 + sk_buff_data_end);
    r0 = 0;
    exit;
"#
);

// SEC("tcx/ingress")
// __description("LDSX, tcx s32 __sk_buff->data_meta")
// __failure __msg("invalid bpf_context access")
// __naked void ldsx_ctx_6(void)
global_asm!(
    r#"
    .section tcx/ingress,"ax"
    .global ldsx_ctx_6
ldsx_ctx_6:
    r2 = *(s32 *)(r1 + sk_buff_data_meta);
    r0 = 0;
    exit;
"#
);

// SEC("flow_dissector")
// __description("LDSX, flow_dissector s32 __sk_buff->data")
// __failure __msg("invalid bpf_context access")
// __naked void ldsx_ctx_7(void)
global_asm!(
    r#"
    .section flow_dissector,"ax"
    .global ldsx_ctx_7
ldsx_ctx_7:
    r2 = *(s32 *)(r1 + sk_buff_data);
    r0 = 0;
    exit;
"#
);

// SEC("flow_dissector")
// __description("LDSX, flow_dissector s32 __sk_buff->data_end")
// __failure __msg("invalid bpf_context access")
// __naked void ldsx_ctx_8(void)
global_asm!(
    r#"
    .section flow_dissector,"ax"
    .global ldsx_ctx_8
ldsx_ctx_8:
    r2 = *(s32 *)(r1 + sk_buff_data_end);
    r0 = 0;
    exit;
"#
);

// SEC("syscall")
// __description("Arena LDSX Disasm")
// __success
// __arch_x86_64
// __jited("movslq	0x10(%rax,%r12), %r14")
// __jited("movswq	0x18(%rax,%r12), %r14")
// __jited("movsbq	0x20(%rax,%r12), %r14")
// __jited("movslq	0x10(%rdi,%r12), %r15")
// __jited("movswq	0x18(%rdi,%r12), %r15")
// __jited("movsbq	0x20(%rdi,%r12), %r15")
// __arch_arm64
// __jited("add	x11, x8, x28")
// __jited("ldrsw	x21, [x11, #0x10]")
// __jited("add	x11, x8, x28")
// __jited("ldrsh	x21, [x11, #0x18]")
// __jited("add	x11, x8, x28")
// __jited("ldrsb	x21, [x11, #0x20]")
// __jited("add	x11, x0, x28")
// __jited("ldrsw	x22, [x11, #0x10]")
// __jited("add	x11, x0, x28")
// __jited("ldrsh	x22, [x11, #0x18]")
// __jited("add	x11, x0, x28")
// __jited("ldrsb	x22, [x11, #0x20]")
// __naked void arena_ldsx_disasm(void *ctx)
global_asm!(
    r#"
    .section syscall,"ax"
    .global arena_ldsx_disasm
arena_ldsx_disasm:
    r1 = arena ll;
    r2 = 0;
    r3 = 1;
    r4 = NUMA_NO_NODE;
    r5 = 0;
    call bpf_arena_alloc_pages;
    r0 = addr_space_cast(r0, 0x0, 0x1);
    r1 = r0;
    r8 = *(s32 *)(r0 + 16);
    r8 = *(s16 *)(r0 + 24);
    r8 = *(s8  *)(r0 + 32);
    r9 = *(s32 *)(r1 + 16);
    r9 = *(s16 *)(r1 + 24);
    r9 = *(s8  *)(r1 + 32);
    r0 = 0;
    exit;
"#
);

// SEC("syscall")
// __description("Arena LDSX Exception")
// __success __retval(0)
// __arch_x86_64
// __arch_arm64
// __naked void arena_ldsx_exception(void *ctx)
global_asm!(
    r#"
    .section syscall,"ax"
    .global arena_ldsx_exception
arena_ldsx_exception:
    r1 = arena ll;
    r0 = 0xdeadbeef;
    r0 = addr_space_cast(r0, 0x0, 0x1);
    r1 = 0x3fe;
    *(u64 *)(r0 + 0) = r1;
    r0 = *(s8 *)(r0 + 0);
    exit;
"#
);

// SEC("syscall")
// __description("Arena LDSX, S8")
// __success __retval(-1)
// __arch_x86_64
// __arch_arm64
// __naked void arena_ldsx_s8(void *ctx)
global_asm!(
    r#"
    .section syscall,"ax"
    .global arena_ldsx_s8
arena_ldsx_s8:
    r1 = arena ll;
    r2 = 0;
    r3 = 1;
    r4 = NUMA_NO_NODE;
    r5 = 0;
    call bpf_arena_alloc_pages;
    r0 = addr_space_cast(r0, 0x0, 0x1);
    r1 = 0x3fe;
    *(u64 *)(r0 + 0) = r1;
    /* little endian: r0 = *(s8 *)(r0 + 0); */
    /* big endian:    r0 = *(s8 *)(r0 + 7); */
    r0 = *(s8 *)(r0 + 0);
    r0 >>= 1;
    exit;
"#
);

// SEC("syscall")
// __description("Arena LDSX, S16")
// __success __retval(-1)
// __arch_x86_64
// __arch_arm64
// __naked void arena_ldsx_s16(void *ctx)
global_asm!(
    r#"
    .section syscall,"ax"
    .global arena_ldsx_s16
arena_ldsx_s16:
    r1 = arena ll;
    r2 = 0;
    r3 = 1;
    r4 = NUMA_NO_NODE;
    r5 = 0;
    call bpf_arena_alloc_pages;
    r0 = addr_space_cast(r0, 0x0, 0x1);
    r1 = 0x3fffe;
    *(u64 *)(r0 + 0) = r1;
    /* little endian: r0 = *(s16 *)(r0 + 0); */
    /* big endian:    r0 = *(s16 *)(r0 + 6); */
    r0 = *(s16 *)(r0 + 0);
    r0 >>= 1;
    exit;
"#
);

// SEC("syscall")
// __description("Arena LDSX, S32")
// __success __retval(-1)
// __arch_x86_64
// __arch_arm64
// __naked void arena_ldsx_s32(void *ctx)
global_asm!(
    r#"
    .section syscall,"ax"
    .global arena_ldsx_s32
arena_ldsx_s32:
    r1 = arena ll;
    r2 = 0;
    r3 = 1;
    r4 = NUMA_NO_NODE;
    r5 = 0;
    call bpf_arena_alloc_pages;
    r0 = addr_space_cast(r0, 0x0, 0x1);
    r1 = 0xfffffffe;
    *(u64 *)(r0 + 0) = r1;
    /* little endian: r0 = *(s32 *)(r0 + 0); */
    /* big endian:    r0 = *(s32 *)(r0 + 4); */
    r0 = *(s32 *)(r0 + 0);
    r0 >>= 1;
    exit;
"#
);

unsafe extern "C" {
    fn bpf_arena_alloc_pages(arena: u64, page_cnt: u64, flags: u64, nid: u64, page: u64) -> u64;
}

/* to retain debug info for BTF generation */
#[no_mangle]
pub unsafe extern "C" fn kfunc_root() {
    unsafe {
        bpf_arena_alloc_pages(0, 0, 0, 0, 0);
    }
}

// Original #else fallback:
// SEC("socket")
// __description("cpuv4 is not supported by compiler or jit, use a dummy test")
// __success
#[no_mangle]
pub extern "C" fn dummy_test() -> i32 {
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
