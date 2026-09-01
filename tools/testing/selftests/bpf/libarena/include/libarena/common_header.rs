// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/*
 * C header dependency/conditional intent:
 *
 * Under __BPF__, this header includes vmlinux.h, bpf_arena_common.h,
 * bpf_arena_spin_lock.h, and asm-generic/errno.h, requires
 * __BPF_FEATURE_ADDR_SPACE_CAST, defines arena_stdout/arena_stderr in terms of
 * bpf_stream_printk, defines private(name) for BPF data sections, declares an
 * arena BPF_MAP_TYPE_ARENA map, and uses the __arena address-space qualifier.
 *
 * Under !__BPF__, stdint.h supplies the fixed-width integer types, __arena is
 * empty, and arena_spinlock_t is a dummy int definition for userspace.
 */

#[cfg(not(target_arch = "bpf"))]
pub type u8 = ::std::os::raw::c_uchar;
#[cfg(not(target_arch = "bpf"))]
pub type u16 = ::std::os::raw::c_ushort;
#[cfg(not(target_arch = "bpf"))]
pub type u32 = ::std::os::raw::c_uint;
#[cfg(not(target_arch = "bpf"))]
pub type u64 = ::std::os::raw::c_ulonglong;
#[cfg(not(target_arch = "bpf"))]
pub type s8 = ::std::os::raw::c_schar;
#[cfg(not(target_arch = "bpf"))]
pub type s16 = ::std::os::raw::c_short;
#[cfg(not(target_arch = "bpf"))]
pub type s32 = ::std::os::raw::c_int;
#[cfg(not(target_arch = "bpf"))]
pub type s64 = ::std::os::raw::c_longlong;

/* Dummy "definition" for userspace. */
#[cfg(not(target_arch = "bpf"))]
pub type arena_spinlock_t = ::std::os::raw::c_int;

/*
 * #define arena_stdout(fmt, ...) bpf_stream_printk(1, (fmt), ##__VA_ARGS__)
 * #define arena_stderr(fmt, ...) bpf_stream_printk(2, (fmt), ##__VA_ARGS__)
 * #define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
 */

/*
 * #define ARENA_PAGES (1UL << (32 - __builtin_ffs(__PAGE_SIZE) + 1))
 *
 * The exact value depends on the externally supplied __PAGE_SIZE build-time
 * constant and the C compiler builtin __builtin_ffs.
 */

#[cfg(target_arch = "bpf")]
unsafe extern "C" {
    /*
     * BPF map declaration from C:
     *
     * struct {
     *     __uint(type, BPF_MAP_TYPE_ARENA);
     *     __uint(map_flags, BPF_F_MMAPABLE);
     *     __uint(max_entries, ARENA_PAGES);
     * #if defined(__TARGET_ARCH_arm64) || defined(__aarch64__)
     *     __ulong(map_extra, (1ull << 32));
     * #else
     *     __ulong(map_extra, (1ull << 44));
     * #endif
     * } arena __weak SEC(".maps");
     */
    pub static mut arena: ArenaMap;
}

#[cfg(target_arch = "bpf")]
#[repr(C)]
pub struct ArenaMap {
    _private: [u8; 0],
}

/*
 * This is a variable used to aid verification. The may_goto directive
 * permits open-coded for loops, but requires that the index variable is
 * imprecise. To force the variable to be imprecise, initialize it with
 * the opaque volatile variable 0 instead of the constant 0.
 */
#[cfg(target_arch = "bpf")]
unsafe extern "C" {
    pub static mut zero: u32;
    pub static mut asan_violated: u64;
}

#[cfg(target_arch = "bpf")]
unsafe extern "C" {
    pub fn arena_fls(word: u64) -> i32;
    pub fn arena_malloc(size: usize) -> *mut ::core::ffi::c_void;
    pub fn arena_free(ptr: *mut ::core::ffi::c_void);
}

/*
 * The verifier associates arenas with programs by checking LD.IMM
 * instruction operands for an arena and populating the program state
 * with the first instance it finds. This requires accessing our global
 * arena variable, but subprogs do not necessarily do so while still
 * using pointers from that arena. Insert an LD.IMM instruction  to
 * access the arena and help the verifier.
 */
#[cfg(target_arch = "bpf")]
#[inline(always)]
pub unsafe fn arena_subprog_init() {
    unsafe {
        ::core::arch::asm!("", in("r0") &raw const arena, options(nostack, preserves_flags));
    }
}

#[repr(C)]
pub struct arena_get_info_args {
    pub arena_base: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct arena_alloc_reserve_args {
    pub nr_pages: u64,
}

/* Reasonable default number of pages reserved by arena_alloc_reserve. */
pub const ARENA_RESERVE_PAGES_DFL: ::std::os::raw::c_int = 8;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
