/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/*
 * Translated from C header bpf_arena_common.bpf.h.
 *
 * C-only include guards, pragmas, attributes, and preprocessor conditionals are
 * preserved here as Rust items or comments where Rust has no direct equivalent.
 */

/*
 * #ifndef PAGE_SIZE
 * #define PAGE_SIZE __PAGE_SIZE
 *
 * for older kernels try sizeof(struct genradix_node)
 * or flexible:
 * static inline long __bpf_page_size(void) {
 *   return bpf_core_enum_value(enum page_size_enum___l, __PAGE_SIZE___l) ?: sizeof(struct genradix_node);
 * }
 * but generated code is not great.
 */
pub const PAGE_SIZE: usize = __PAGE_SIZE as usize;

/*
 * If __BPF_FEATURE_ADDR_SPACE_CAST is defined and BPF_ARENA_FORCE_ASM is not
 * defined, C uses address_space(1) attributes for __arena and __arena_global,
 * and cast_kern()/cast_user() are nops emitted by LLVM.
 *
 * Otherwise, C emits a BPF_ADDR_SPACE_CAST instruction with inline assembly.
 * Rust does not have a direct stable spelling for these C address-space
 * attributes or asm-goto macros in a header translation, so pointer address
 * space intent is preserved with raw pointers and the cast helpers below.
 */

#[inline(always)]
pub unsafe fn bpf_addr_space_cast<T>(var: *mut T, dst_as: i32, src_as: i32) -> *mut T {
    let mut var = var;
    unsafe {
        core::arch::asm!(
            ".byte 0xBF",
            ".short {off}",
            ".long {addr_space}",
            off = const BPF_ADDR_SPACE_CAST,
            addr_space = const ((dst_as << 16) | src_as),
            inout(reg) var,
            options(nostack, preserves_flags),
        );
    }
    var
}

#[inline(always)]
pub unsafe fn cast_kern<T>(ptr: *mut T) -> *mut T {
    unsafe { bpf_addr_space_cast(ptr, 0, 1) }
}

#[inline(always)]
pub unsafe fn cast_user<T>(ptr: *mut T) -> *mut T {
    unsafe { bpf_addr_space_cast(ptr, 1, 0) }
}

unsafe extern "C" {
    pub fn bpf_arena_alloc_pages(
        map: *mut core::ffi::c_void,
        addr: *mut core::ffi::c_void,
        page_cnt: __u32,
        node_id: core::ffi::c_int,
        flags: __u64,
    ) -> *mut core::ffi::c_void;

    pub fn bpf_arena_free_pages(
        map: *mut core::ffi::c_void,
        ptr: *mut core::ffi::c_void,
        page_cnt: __u32,
    );

    pub fn bpf_arena_reserve_pages(
        map: *mut core::ffi::c_void,
        ptr: *mut core::ffi::c_void,
        page_cnt: __u32,
    ) -> core::ffi::c_int;
}

/*
 * Note that cond_break can only be portably used in the body of a breakable
 * construct, whereas can_loop can be used anywhere.
 *
 * In C, SCX_BPF_UNITTEST maps can_loop to true and __cond_break(expr) to expr.
 * Otherwise, the implementation uses __BPF_FEATURE_MAY_GOTO when available, or
 * emits endian-specific asm-goto bytes for older toolchains.
 */

#[cfg(SCX_BPF_UNITTEST)]
pub const can_loop: bool = true;

#[cfg(not(SCX_BPF_UNITTEST))]
#[inline(always)]
pub unsafe fn can_loop() -> bool {
    /*
     * C version:
     *   __label__ l_break, l_continue;
     *   bool ret = true;
     *   asm volatile goto("may_goto %l[l_break]" :::: l_break);
     *   goto l_continue;
     *   l_break: ret = false;
     *   l_continue:;
     *   ret;
     *
     * Without __BPF_FEATURE_MAY_GOTO, C emits the may_goto instruction bytes
     * directly. The immediate differs by byte order:
     *   little endian: ((%l[l_break] - 1b - 8) / 8) & 0xffff
     *   big endian:    (((%l[l_break] - 1b - 8) / 8) & 0xffff) << 16
     */
    true
}

#[cfg(SCX_BPF_UNITTEST)]
#[macro_export]
macro_rules! __cond_break {
    ($expr:expr) => {
        $expr
    };
}

#[cfg(not(SCX_BPF_UNITTEST))]
#[macro_export]
macro_rules! __cond_break {
    ($expr:expr) => {{
        /*
         * C version uses asm volatile goto to branch to l_break and execute
         * expr, then continues at l_continue. Rust has no direct equivalent
         * for this C macro form in a source-level header translation.
         */
        let _ = || {
            $expr
        };
    }};
}

#[macro_export]
macro_rules! cond_break {
    () => {
        $crate::__cond_break!(break)
    };
}

#[macro_export]
macro_rules! cond_break_label {
    ($label:lifetime) => {
        $crate::__cond_break!(break $label)
    };
}

unsafe extern "C" {
    pub fn bpf_preempt_disable();
    pub fn bpf_preempt_enable();
    pub fn bpf_arena_mapping_nr_pages(p__map: *mut core::ffi::c_void) -> ssize_t;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
