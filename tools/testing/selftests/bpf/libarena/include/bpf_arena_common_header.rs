/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

/*
 * C header dependencies intentionally left external:
 * __ksym, __weak, SEC, struct bpf_arena, __PAGE_SIZE, bpf_addr_space_cast,
 * and BPF address-space attributes.
 */

#[inline]
pub unsafe fn WRITE_ONCE<T>(x: *mut T, val: T) -> T
where
    T: Copy,
{
    unsafe {
        ptr::write_volatile(x, val);
    }
    val
}

pub const NUMA_NO_NODE: c_int = -1;

#[macro_export]
macro_rules! arena_container_of {
    ($ptr:expr, $type:ty, $member:ident) => {{
        let __mptr = $ptr as *mut core::ffi::c_void as *mut u8;
        __mptr.sub(core::mem::offset_of!($type, $member)) as *mut $type
    }};
}

/* when compiled as bpf program */
#[cfg(__BPF__)]
pub const PAGE_SIZE: usize = __PAGE_SIZE as usize;

/*
 * for older kernels try sizeof(struct genradix_node)
 * or flexible:
 * static inline long __bpf_page_size(void) {
 *   return bpf_core_enum_value(enum page_size_enum___l, __PAGE_SIZE___l) ?: sizeof(struct genradix_node);
 * }
 * but generated code is not great.
 */

/*
 * C conditional:
 * #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) && !defined(BPF_ARENA_FORCE_ASM)
 *   __arena is address_space(1) with btf_type_tag("arena")
 *   __arena_global is address_space(1)
 *   cast_kern(ptr) and cast_user(ptr) are LLVM-emitted no-ops
 * #else
 *   __arena is btf_type_tag("arena")
 *   __arena_global is SEC(".addr_space.1")
 *   cast_kern(ptr) is bpf_addr_space_cast(ptr, 0, 1)
 *   cast_user(ptr) is bpf_addr_space_cast(ptr, 1, 0)
 * #endif
 */

#[cfg(all(__BPF__, __BPF_FEATURE_ADDR_SPACE_CAST, not(BPF_ARENA_FORCE_ASM)))]
#[inline]
pub unsafe fn cast_kern<T>(ptr: *mut T) -> *mut T {
    ptr
}

#[cfg(all(__BPF__, __BPF_FEATURE_ADDR_SPACE_CAST, not(BPF_ARENA_FORCE_ASM)))]
#[inline]
pub unsafe fn cast_user<T>(ptr: *mut T) -> *mut T {
    ptr
}

#[cfg(all(__BPF__, any(not(__BPF_FEATURE_ADDR_SPACE_CAST), BPF_ARENA_FORCE_ASM)))]
#[inline]
pub unsafe fn cast_kern<T>(ptr: *mut T) -> *mut T {
    unsafe { bpf_addr_space_cast(ptr as *mut c_void, 0, 1) as *mut T }
}

#[cfg(all(__BPF__, any(not(__BPF_FEATURE_ADDR_SPACE_CAST), BPF_ARENA_FORCE_ASM)))]
#[inline]
pub unsafe fn cast_user<T>(ptr: *mut T) -> *mut T {
    unsafe { bpf_addr_space_cast(ptr as *mut c_void, 1, 0) as *mut T }
}

#[cfg(__BPF__)]
unsafe extern "C" {
    pub fn bpf_addr_space_cast(ptr: *mut c_void, from: c_int, to: c_int) -> *mut c_void;

    pub fn bpf_arena_alloc_pages(
        map: *mut c_void,
        addr: *mut c_void,
        page_cnt: u32,
        node_id: c_int,
        flags: u64,
    ) -> *mut c_void;
    pub fn bpf_arena_reserve_pages(map: *mut c_void, addr: *mut c_void, page_cnt: u32) -> c_int;
    pub fn bpf_arena_free_pages(map: *mut c_void, ptr: *mut c_void, page_cnt: u32);
}

#[cfg(__BPF__)]
#[inline]
pub unsafe fn arena_base(map: *mut c_void) -> *mut c_void {
    unsafe { (*(map as *mut bpf_arena)).user_vm_start as *mut c_void }
}

/* when compiled as user space code */
#[cfg(not(__BPF__))]
#[inline]
pub unsafe fn cast_kern<T>(ptr: *mut T) -> *mut T {
    ptr
}

#[cfg(not(__BPF__))]
#[inline]
pub unsafe fn cast_user<T>(ptr: *mut T) -> *mut T {
    ptr
}

#[cfg(not(__BPF__))]
unsafe extern "C" {
    /* C declaration: __weak char arena[1]; */
    pub static mut arena: [c_char; 1];
}

#[cfg(not(__BPF__))]
#[inline]
pub const fn offsetof<T>(offset: usize) -> usize {
    offset
}

#[cfg(not(__BPF__))]
#[inline]
pub unsafe fn bpf_arena_alloc_pages(
    _map: *mut c_void,
    _addr: *mut c_void,
    _page_cnt: u32,
    _node_id: c_int,
    _flags: u64,
) -> *mut c_void {
    ptr::null_mut()
}

#[cfg(not(__BPF__))]
#[inline]
pub unsafe fn bpf_arena_free_pages(_map: *mut c_void, _ptr: *mut c_void, _page_cnt: u32) {}
