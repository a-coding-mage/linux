// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __s32 = i32;
type __s64 = i64;

const BPF_MAP_TYPE_ARENA: __u32 = 0;
const BPF_F_MMAPABLE: __u32 = 0;
const NUMA_NO_NODE: __s32 = -1;

#[repr(C)]
pub struct arena_map_def {
    pub type_: __u32,
    pub map_flags: __u32,
    pub max_entries: __u32,
    pub map_extra: __u64,
}

// Original C used SEC(".maps") and BPF helper macros:
// __uint(type, BPF_MAP_TYPE_ARENA);
// __uint(map_flags, BPF_F_MMAPABLE);
// __uint(max_entries, 10); /* number of pages */
// map_extra is 0x1ull << 32 on __TARGET_ARCH_arm64, otherwise 0x1ull << 44.
#[no_mangle]
#[link_section = ".maps"]
pub static mut arena: arena_map_def = arena_map_def {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 10,
    #[cfg(__TARGET_ARCH_arm64)]
    map_extra: 0x1u64 << 32,
    #[cfg(not(__TARGET_ARCH_arm64))]
    map_extra: 0x1u64 << 44,
};

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_arena_alloc_pages(
        arena: *mut arena_map_def,
        addr: *mut core::ffi::c_void,
        page_cnt: __u64,
        node: __s32,
        flags: __u64,
    ) -> *mut core::ffi::c_void;
    fn bpf_arena_free_pages(arena: *mut arena_map_def, ptr: *mut core::ffi::c_void, page_cnt: __u64);
}

#[cfg(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST))]
#[no_mangle]
#[link_section = ".data"]
pub static mut skip_all_tests: bool = false;
#[cfg(not(all(ENABLE_ATOMICS_TESTS, __BPF_FEATURE_ADDR_SPACE_CAST)))]
#[no_mangle]
pub static mut skip_all_tests: bool = true;

#[cfg(all(
    ENABLE_ATOMICS_TESTS,
    __BPF_FEATURE_ADDR_SPACE_CAST,
    any(
        __TARGET_ARCH_arm64,
        __TARGET_ARCH_x86,
        all(__TARGET_ARCH_riscv, target_pointer_width = "64"),
        __TARGET_ARCH_s390
    )
))]
#[no_mangle]
#[link_section = ".data"]
pub static mut skip_lacq_srel_tests: bool = false;
#[cfg(not(all(
    ENABLE_ATOMICS_TESTS,
    __BPF_FEATURE_ADDR_SPACE_CAST,
    any(
        __TARGET_ARCH_arm64,
        __TARGET_ARCH_x86,
        all(__TARGET_ARCH_riscv, target_pointer_width = "64"),
        __TARGET_ARCH_s390
    )
)))]
#[no_mangle]
pub static mut skip_lacq_srel_tests: bool = true;

#[no_mangle]
pub static mut pid: __u32 = 0;

#[no_mangle]
pub static mut add64_value: __u64 = 1;
#[no_mangle]
pub static mut add64_result: __u64 = 0;
#[no_mangle]
pub static mut add32_value: __u32 = 1;
#[no_mangle]
pub static mut add32_result: __u32 = 0;
#[no_mangle]
pub static mut add_stack_value_copy: __u64 = 0;
#[no_mangle]
pub static mut add_stack_result: __u64 = 0;
#[no_mangle]
pub static mut add_noreturn_value: __u64 = 1;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn add(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        let mut add_stack_value: __u64 = 1;

        add64_result = core::intrinsics::atomic_xadd_seqcst(&raw mut add64_value, 2);
        add32_result = core::intrinsics::atomic_xadd_seqcst(&raw mut add32_value, 2);
        add_stack_result = core::intrinsics::atomic_xadd_seqcst(&mut add_stack_value, 2);
        add_stack_value_copy = add_stack_value;
        core::intrinsics::atomic_xadd_seqcst(&raw mut add_noreturn_value, 2);
    }

    0
}

#[no_mangle]
pub static mut sub64_value: __s64 = 1;
#[no_mangle]
pub static mut sub64_result: __s64 = 0;
#[no_mangle]
pub static mut sub32_value: __s32 = 1;
#[no_mangle]
pub static mut sub32_result: __s32 = 0;
#[no_mangle]
pub static mut sub_stack_value_copy: __s64 = 0;
#[no_mangle]
pub static mut sub_stack_result: __s64 = 0;
#[no_mangle]
pub static mut sub_noreturn_value: __s64 = 1;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn sub(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        let mut sub_stack_value: __u64 = 1;

        sub64_result = core::intrinsics::atomic_xsub_seqcst(&raw mut sub64_value, 2);
        sub32_result = core::intrinsics::atomic_xsub_seqcst(&raw mut sub32_value, 2);
        sub_stack_result = core::intrinsics::atomic_xsub_seqcst(&mut sub_stack_value, 2) as __s64;
        sub_stack_value_copy = sub_stack_value as __s64;
        core::intrinsics::atomic_xsub_seqcst(&raw mut sub_noreturn_value, 2);
    }

    0
}

#[no_mangle]
pub static mut and64_value: __u64 = 0x110u64 << 32;
#[no_mangle]
pub static mut and32_value: __u32 = 0x110;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn and(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        core::intrinsics::atomic_and_relaxed(&raw mut and64_value, 0x011u64 << 32);
        core::intrinsics::atomic_and_relaxed(&raw mut and32_value, 0x011);
    }

    0
}

#[no_mangle]
pub static mut or32_value: __u32 = 0x110;
#[no_mangle]
pub static mut or64_value: __u64 = 0x110u64 << 32;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn or(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        core::intrinsics::atomic_or_relaxed(&raw mut or64_value, 0x011u64 << 32);
        core::intrinsics::atomic_or_relaxed(&raw mut or32_value, 0x011);
    }

    0
}

#[no_mangle]
pub static mut xor64_value: __u64 = 0x110u64 << 32;
#[no_mangle]
pub static mut xor32_value: __u32 = 0x110;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn xor(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        core::intrinsics::atomic_xor_relaxed(&raw mut xor64_value, 0x011u64 << 32);
        core::intrinsics::atomic_xor_relaxed(&raw mut xor32_value, 0x011);
    }

    0
}

#[no_mangle]
pub static mut cmpxchg32_value: __u32 = 1;
#[no_mangle]
pub static mut cmpxchg32_result_fail: __u32 = 0;
#[no_mangle]
pub static mut cmpxchg32_result_succeed: __u32 = 0;
#[no_mangle]
pub static mut cmpxchg64_value: __u64 = 1;
#[no_mangle]
pub static mut cmpxchg64_result_fail: __u64 = 0;
#[no_mangle]
pub static mut cmpxchg64_result_succeed: __u64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn cmpxchg(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        cmpxchg64_result_fail =
            core::intrinsics::atomic_cxchg_seqcst_seqcst(&raw mut cmpxchg64_value, 0, 3).0;
        cmpxchg64_result_succeed =
            core::intrinsics::atomic_cxchg_seqcst_seqcst(&raw mut cmpxchg64_value, 1, 2).0;

        cmpxchg32_result_fail =
            core::intrinsics::atomic_cxchg_seqcst_seqcst(&raw mut cmpxchg32_value, 0, 3).0;
        cmpxchg32_result_succeed =
            core::intrinsics::atomic_cxchg_seqcst_seqcst(&raw mut cmpxchg32_value, 1, 2).0;
    }

    0
}

#[no_mangle]
pub static mut xchg64_value: __u64 = 1;
#[no_mangle]
pub static mut xchg64_result: __u64 = 0;
#[no_mangle]
pub static mut xchg32_value: __u32 = 1;
#[no_mangle]
pub static mut xchg32_result: __u32 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn xchg(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        let val64: __u64 = 2;
        let val32: __u32 = 2;

        xchg64_result = core::intrinsics::atomic_xchg_seqcst(&raw mut xchg64_value, val64);
        xchg32_result = core::intrinsics::atomic_xchg_seqcst(&raw mut xchg32_value, val32);
    }

    0
}

#[no_mangle]
pub static mut uaf_sink: __u64 = 0;
#[no_mangle]
pub static mut uaf_recovery_fails: __u64 = 0;

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn uaf(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(all(
        ENABLE_ATOMICS_TESTS,
        not(__TARGET_ARCH_arm64),
        not(__TARGET_ARCH_x86)
    ))]
    {
        let page = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
        bpf_arena_free_pages(&raw mut arena, page, 1);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, 24);

        let page32 = page as *mut __u32;
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_xadd_seqcst(page32, 1) as __u64);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_xadd_seqcst(page32, 1).wrapping_add(1);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_xsub_seqcst(page32, 1) as __u64);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_xsub_seqcst(page32, 1).wrapping_sub(1);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_and_seqcst(page32, 1) as __u64);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_and_seqcst(page32, 1) & 1;
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_or_seqcst(page32, 1) as __u64);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_or_seqcst(page32, 1) | 1;
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_xor_seqcst(page32, 1) as __u64);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_xor_seqcst(page32, 1) ^ 1;
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_cxchg_seqcst_seqcst(page32, 0, 1).0 as __u64);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_xchg_seqcst(page32, 1) as __u64);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));

        let page64 = page as *mut __u64;
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_xadd_seqcst(page64, 1));
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_xadd_seqcst(page64, 1).wrapping_add(1);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_xsub_seqcst(page64, 1));
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_xsub_seqcst(page64, 1).wrapping_sub(1);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_and_seqcst(page64, 1));
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_and_seqcst(page64, 1) & 1;
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_or_seqcst(page64, 1));
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_or_seqcst(page64, 1) | 1;
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_xor_seqcst(page64, 1));
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        core::intrinsics::atomic_xor_seqcst(page64, 1) ^ 1;
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_cxchg_seqcst_seqcst(page64, 0, 1).0);
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
        uaf_sink = uaf_sink.wrapping_add(core::intrinsics::atomic_xchg_seqcst(page64, 1));
        core::ptr::write_volatile(&raw mut uaf_recovery_fails, core::ptr::read_volatile(&raw const uaf_recovery_fails).wrapping_sub(1));
    }

    0
}

#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut load_acquire8_value: __u8 = 0x12;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut load_acquire16_value: __u16 = 0x1234;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut load_acquire32_value: __u32 = 0x12345678;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut load_acquire64_value: __u64 = 0x1234567890abcdef;

#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut load_acquire8_result: __u8 = 0;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut load_acquire16_result: __u16 = 0;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut load_acquire32_result: __u32 = 0;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut load_acquire64_result: __u64 = 0;

// clang-17 crashes if the .addr_space.1 ELF section has holes. Work around
// this issue by defining the below variables as 64-bit.
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut load_acquire8_value: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut load_acquire16_value: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut load_acquire32_value: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut load_acquire64_value: __u64 = 0;

#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut load_acquire8_result: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut load_acquire16_result: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut load_acquire32_result: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut load_acquire64_result: __u64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn load_acquire(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    #[cfg(all(
        ENABLE_ATOMICS_TESTS,
        __BPF_FEATURE_ADDR_SPACE_CAST,
        any(
            __TARGET_ARCH_arm64,
            __TARGET_ARCH_x86,
            all(__TARGET_ARCH_riscv, target_pointer_width = "64"),
            __TARGET_ARCH_s390
        )
    ))]
    {
        core::arch::asm!(
            "r1 = {src} ll",
            "r1 = addr_space_cast(r1, 0x0, 0x1)",
            ".8byte {load_acquire_insn}",
            "r3 = {dst} ll",
            "r3 = addr_space_cast(r3, 0x0, 0x1)",
            "*(u8 *)(r3 + 0) = r2",
            src = sym load_acquire8_value,
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_B, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_1, 0),
            dst = sym load_acquire8_result,
        );
        core::arch::asm!(
            "r1 = {src} ll",
            "r1 = addr_space_cast(r1, 0x0, 0x1)",
            ".8byte {load_acquire_insn}",
            "r3 = {dst} ll",
            "r3 = addr_space_cast(r3, 0x0, 0x1)",
            "*(u16 *)(r3 + 0) = r2",
            src = sym load_acquire16_value,
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_H, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_1, 0),
            dst = sym load_acquire16_result,
        );
        core::arch::asm!(
            "r1 = {src} ll",
            "r1 = addr_space_cast(r1, 0x0, 0x1)",
            ".8byte {load_acquire_insn}",
            "r3 = {dst} ll",
            "r3 = addr_space_cast(r3, 0x0, 0x1)",
            "*(u32 *)(r3 + 0) = r2",
            src = sym load_acquire32_value,
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_W, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_1, 0),
            dst = sym load_acquire32_result,
        );
        core::arch::asm!(
            "r1 = {src} ll",
            "r1 = addr_space_cast(r1, 0x0, 0x1)",
            ".8byte {load_acquire_insn}",
            "r3 = {dst} ll",
            "r3 = addr_space_cast(r3, 0x0, 0x1)",
            "*(u64 *)(r3 + 0) = r2",
            src = sym load_acquire64_value,
            load_acquire_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_LOAD_ACQ, BPF_REG_2, BPF_REG_1, 0),
            dst = sym load_acquire64_result,
        );
    }
    0
}

#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut store_release8_result: __u8 = 0;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut store_release16_result: __u16 = 0;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut store_release32_result: __u32 = 0;
#[cfg(__clang_major_ge_18)]
#[no_mangle]
pub static mut store_release64_result: __u64 = 0;

// clang-17 crashes if the .addr_space.1 ELF section has holes. Work around
// this issue by defining the below variables as 64-bit.
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut store_release8_result: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut store_release16_result: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut store_release32_result: __u64 = 0;
#[cfg(not(__clang_major_ge_18))]
#[no_mangle]
pub static mut store_release64_result: __u64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn store_release(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    #[cfg(all(
        ENABLE_ATOMICS_TESTS,
        __BPF_FEATURE_ADDR_SPACE_CAST,
        any(
            __TARGET_ARCH_arm64,
            __TARGET_ARCH_x86,
            all(__TARGET_ARCH_riscv, target_pointer_width = "64"),
            __TARGET_ARCH_s390
        )
    ))]
    {
        core::arch::asm!(
            "r1 = 0x12",
            "r2 = {dst} ll",
            "r2 = addr_space_cast(r2, 0x0, 0x1)",
            ".8byte {store_release_insn}",
            dst = sym store_release8_result,
            store_release_insn = const BPF_ATOMIC_OP(BPF_B, BPF_STORE_REL, BPF_REG_2, BPF_REG_1, 0),
        );
        core::arch::asm!(
            "r1 = 0x1234",
            "r2 = {dst} ll",
            "r2 = addr_space_cast(r2, 0x0, 0x1)",
            ".8byte {store_release_insn}",
            dst = sym store_release16_result,
            store_release_insn = const BPF_ATOMIC_OP(BPF_H, BPF_STORE_REL, BPF_REG_2, BPF_REG_1, 0),
        );
        core::arch::asm!(
            "r1 = 0x12345678",
            "r2 = {dst} ll",
            "r2 = addr_space_cast(r2, 0x0, 0x1)",
            ".8byte {store_release_insn}",
            dst = sym store_release32_result,
            store_release_insn = const BPF_ATOMIC_OP(BPF_W, BPF_STORE_REL, BPF_REG_2, BPF_REG_1, 0),
        );
        core::arch::asm!(
            "r1 = 0x1234567890abcdef ll",
            "r2 = {dst} ll",
            "r2 = addr_space_cast(r2, 0x0, 0x1)",
            ".8byte {store_release_insn}",
            dst = sym store_release64_result,
            store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, BPF_REG_2, BPF_REG_1, 0),
        );
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
