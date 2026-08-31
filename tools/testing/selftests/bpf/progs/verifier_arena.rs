// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C source dependencies:
// #define BPF_NO_KFUNC_PROTOTYPES
// #include <vmlinux.h>
// #include <errno.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "bpf_experimental.h"
// #include <bpf_arena_common.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_void = core::ffi::c_void;
type c_int = i32;
type u32 = u32;
type u64 = u64;
type __u64 = u64;

const BPF_MAP_TYPE_ARENA: u32 = 0;
const BPF_F_MMAPABLE: u32 = 0;
const NUMA_NO_NODE: c_int = -1;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;

extern "C" {
    static __PAGE_SIZE: u64;
    fn bpf_arena_alloc_pages(
        arena: *mut bpf_map,
        addr: *mut c_void,
        page_cnt: u64,
        node: c_int,
        flags: u64,
    ) -> *mut c_void;
    fn bpf_arena_free_pages(arena: *mut bpf_map, addr: *mut c_void, page_cnt: u64);
    fn bpf_arena_reserve_pages(arena: *mut bpf_map, addr: *mut c_void, page_cnt: u64) -> c_int;
    fn arena_base(arena: *mut bpf_map) -> *mut i8;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_get_prandom_u32() -> u32;
}

#[repr(C)]
pub struct bpf_map {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub map_extra: u64,
    pub inner_map_meta: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_spin_lock {
    val: u32,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct bpf_iter__bpf_map {
    pub meta: *mut bpf_iter_meta,
    pub map: *mut bpf_map,
}

// #define private(name) SEC(".bss." #name) __hidden __attribute__((aligned(8)))

#[cfg(target_arch = "aarch64")]
const fn arena_vm_start(page_size: u64) -> u64 {
    (1u64 << 32) | ((!0u32 as u64).wrapping_sub(page_size.wrapping_mul(2)).wrapping_add(1))
}

#[cfg(not(target_arch = "aarch64"))]
const fn arena_vm_start(page_size: u64) -> u64 {
    (1u64 << 44) | ((!0u32 as u64).wrapping_sub(page_size.wrapping_mul(2)).wrapping_add(1))
}

// struct arena SEC(".maps"):
#[no_mangle]
#[link_section = ".maps"]
pub static mut arena: bpf_map = bpf_map {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 2, /* arena of two pages close to 32-bit boundary*/
    map_extra: 0,  /* start of mmap() region: ARENA_VM_START, depends on __PAGE_SIZE */
    inner_map_meta: core::ptr::null_mut(),
};

// SEC("socket")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc1_nosleep(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        let mut page1: *mut c_int;
        let mut page2: *mut c_int;
        let no_page: *mut c_int;

        page1 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut c_int;
        if page1.is_null() {
            return 1;
        }
        core::ptr::write_volatile(page1, 1);
        page2 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut c_int;
        if page2.is_null() {
            return 2;
        }
        core::ptr::write_volatile(page2, 2);
        no_page = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut c_int;
        if !no_page.is_null() {
            return 3;
        }
        if core::ptr::read_volatile(page1) != 1 {
            return 4;
        }
        if core::ptr::read_volatile(page2) != 2 {
            return 5;
        }
        bpf_arena_free_pages(&raw mut arena, page2 as *mut c_void, 1);
        if core::ptr::read_volatile(page1) != 1 {
            return 6;
        }
        if core::ptr::read_volatile(page2) != 0 && core::ptr::read_volatile(page2) != 2 {
            /* use-after-free should return 0 or the stored value */
            return 7;
        }
    }
    0
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc1(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        let mut page1: *mut c_int;
        let mut page2: *mut c_int;
        let no_page: *mut c_int;
        let page3: *mut c_int;

        page1 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut c_int;
        if page1.is_null() {
            return 1;
        }
        core::ptr::write_volatile(page1, 1);
        page2 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut c_int;
        if page2.is_null() {
            return 2;
        }
        core::ptr::write_volatile(page2, 2);
        no_page = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut c_int;
        if !no_page.is_null() {
            return 3;
        }
        if core::ptr::read_volatile(page1) != 1 {
            return 4;
        }
        if core::ptr::read_volatile(page2) != 2 {
            return 5;
        }
        bpf_arena_free_pages(&raw mut arena, page2 as *mut c_void, 1);
        if core::ptr::read_volatile(page1) != 1 {
            return 6;
        }
        if core::ptr::read_volatile(page2) != 0 {
            /* use-after-free should return 0 */
            return 7;
        }
        page3 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut c_int;
        if page3.is_null() {
            return 8;
        }
        core::ptr::write_volatile(page3, 3);
        if page2 != page3 {
            return 9;
        }
        if core::ptr::read_volatile(page1) != 1 {
            return 10;
        }
    }
    0
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn free_scalar_below_arena(_ctx: *mut c_void) -> c_int {
    let page1: *mut c_void;
    let page2: *mut c_void;
    let mut page3: *mut c_void;
    let bad_addr: __u64 = arena_vm_start(__PAGE_SIZE).wrapping_sub(__PAGE_SIZE);

    page1 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if page1.is_null() {
        return 1;
    }

    page2 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if page2.is_null() {
        return 2;
    }

    page3 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if !page3.is_null() {
        return 3;
    }

    bpf_arena_free_pages(&raw mut arena, bad_addr as *mut c_void, 1);

    page3 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if !page3.is_null() {
        return 4;
    }

    0
}

unsafe fn basic_alloc2_impl(nosleep: bool) -> c_int {
    let mut page1: *mut i8;
    let page2: *mut i8;
    let page3: *mut i8;
    let page4: *mut i8;

    page1 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 2, NUMA_NO_NODE, 0) as *mut i8;
    if page1.is_null() {
        return 1;
    }
    page2 = page1.add(__PAGE_SIZE as usize);
    page3 = page1.add((__PAGE_SIZE * 2) as usize);
    page4 = page1.sub(__PAGE_SIZE as usize);
    core::ptr::write_volatile(page1, 1);
    core::ptr::write_volatile(page2, 2);
    core::ptr::write_volatile(page3, 3);
    core::ptr::write_volatile(page4, 4);
    if core::ptr::read_volatile(page1) != 1 {
        return 1;
    }
    if core::ptr::read_volatile(page2) != 2 {
        return 2;
    }
    if core::ptr::read_volatile(page3) != 0 {
        return 3;
    }
    if core::ptr::read_volatile(page4) != 0 {
        return 4;
    }
    bpf_arena_free_pages(&raw mut arena, page1 as *mut c_void, 2);
    if nosleep {
        if core::ptr::read_volatile(page1) != 0 && core::ptr::read_volatile(page1) != 1 {
            return 5;
        }
        if core::ptr::read_volatile(page2) != 0 && core::ptr::read_volatile(page2) != 2 {
            return 6;
        }
    } else {
        if core::ptr::read_volatile(page1) != 0 {
            return 5;
        }
        if core::ptr::read_volatile(page2) != 0 {
            return 6;
        }
    }
    if core::ptr::read_volatile(page3) != 0 {
        return 7;
    }
    if core::ptr::read_volatile(page4) != 0 {
        return 8;
    }
    0
}

// SEC("socket")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc2_nosleep(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return basic_alloc2_impl(true);
    }
    0
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc2(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return basic_alloc2_impl(false);
    }
    0
}

#[repr(C)]
pub struct bpf_arena___l {
    pub map: bpf_map,
} /* __attribute__((preserve_access_index)) */

unsafe fn basic_alloc3_impl(_ctx: *mut c_void) -> c_int {
    let ar: *mut bpf_arena___l = (&raw mut arena).cast::<bpf_arena___l>();
    let pages: *mut i8;

    pages = bpf_arena_alloc_pages(
        &mut (*ar).map,
        core::ptr::null_mut(),
        (*ar).map.max_entries as u64,
        NUMA_NO_NODE,
        0,
    ) as *mut i8;
    if pages.is_null() {
        return 1;
    }
    0
}

// SEC("socket")
// __success __retval(0) __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc3_nosleep(ctx: *mut c_void) -> c_int {
    basic_alloc3_impl(ctx)
}

// SEC("syscall")
// __success __retval(0) __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc3(ctx: *mut c_void) -> c_int {
    basic_alloc3_impl(ctx)
}

unsafe fn basic_reserve1_impl() -> c_int {
    let mut page: *mut i8;
    let mut ret: c_int;

    page = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut i8;
    if page.is_null() {
        return 1;
    }

    page = page.add(__PAGE_SIZE as usize);

    /* Reserve the second page */
    ret = bpf_arena_reserve_pages(&raw mut arena, page as *mut c_void, 1);
    if ret != 0 {
        return 2;
    }

    /* Try to explicitly allocate the reserved page. */
    page = bpf_arena_alloc_pages(&raw mut arena, page as *mut c_void, 1, NUMA_NO_NODE, 0) as *mut i8;
    if !page.is_null() {
        return 3;
    }

    /* Try to implicitly allocate the page (since there's only 2 of them). */
    page = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut i8;
    if !page.is_null() {
        return 4;
    }
    0
}

// SEC("socket")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_reserve1_nosleep(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return basic_reserve1_impl();
    }
    0
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_reserve1(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return basic_reserve1_impl();
    }
    0
}

unsafe fn basic_reserve2_impl() -> c_int {
    let mut page: *mut i8;
    let mut ret: c_int;

    page = arena_base(&raw mut arena);
    ret = bpf_arena_reserve_pages(&raw mut arena, page as *mut c_void, 1);
    if ret != 0 {
        return 1;
    }

    page = bpf_arena_alloc_pages(&raw mut arena, page as *mut c_void, 1, NUMA_NO_NODE, 0) as *mut i8;
    if page as u64 != 0 {
        return 2;
    }
    0
}

// SEC("socket")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_reserve2_nosleep(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return basic_reserve2_impl();
    }
    0
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_reserve2(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return basic_reserve2_impl();
    }
    0
}

/* Reserve the same page twice, should return -EBUSY. */
unsafe fn reserve_twice_impl() -> c_int {
    let page: *mut i8;
    let mut ret: c_int;

    page = arena_base(&raw mut arena);

    ret = bpf_arena_reserve_pages(&raw mut arena, page as *mut c_void, 1);
    if ret != 0 {
        return 1;
    }

    ret = bpf_arena_reserve_pages(&raw mut arena, page as *mut c_void, 1);
    if ret != -EBUSY {
        return 2;
    }
    0
}

// SEC("socket")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn reserve_twice_nosleep(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return reserve_twice_impl();
    }
    0
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn reserve_twice(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return reserve_twice_impl();
    }
    0
}

/* Try to reserve past the end of the arena. */
unsafe fn reserve_invalid_region_impl() -> c_int {
    let page: *mut i8;
    let mut ret: c_int;

    /* Try a NULL pointer. */
    ret = bpf_arena_reserve_pages(&raw mut arena, core::ptr::null_mut(), 3);
    if ret != -EINVAL {
        return 1;
    }

    page = arena_base(&raw mut arena);

    ret = bpf_arena_reserve_pages(&raw mut arena, page as *mut c_void, 3);
    if ret != -EINVAL {
        return 2;
    }

    ret = bpf_arena_reserve_pages(&raw mut arena, page as *mut c_void, 4096);
    if ret != -EINVAL {
        return 3;
    }

    ret = bpf_arena_reserve_pages(&raw mut arena, page as *mut c_void, (1u64 << 32) - 1);
    if ret != -EINVAL {
        return 4;
    }
    0
}

// SEC("socket")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn reserve_invalid_region_nosleep(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return reserve_invalid_region_impl();
    }
    0
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn reserve_invalid_region(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        return reserve_invalid_region_impl();
    }
    0
}

// SEC("iter.s/bpf_map")
// __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn iter_maps1(ctx: *mut bpf_iter__bpf_map) -> c_int {
    let map: *mut bpf_map = (*ctx).map;

    if map.is_null() {
        return 0;
    }
    bpf_arena_alloc_pages(map, core::ptr::null_mut(), (*map).max_entries as u64, 0, 0);
    0
}

// SEC("iter.s/bpf_map")
// __failure __msg("expected pointer to STRUCT bpf_map")
#[no_mangle]
pub unsafe extern "C" fn iter_maps2(ctx: *mut bpf_iter__bpf_map) -> c_int {
    let seq: *mut seq_file = (*(*ctx).meta).seq;

    bpf_arena_alloc_pages(seq as *mut bpf_map, core::ptr::null_mut(), 1, 0, 0);
    0
}

// SEC("iter.s/bpf_map")
// __failure __msg("untrusted_ptr_bpf_map")
#[no_mangle]
pub unsafe extern "C" fn iter_maps3(ctx: *mut bpf_iter__bpf_map) -> c_int {
    let map: *mut bpf_map = (*ctx).map;

    if map.is_null() {
        return 0;
    }
    bpf_arena_alloc_pages((*map).inner_map_meta, core::ptr::null_mut(), (*map).max_entries as u64, 0, 0);
    0
}

// private(ARENA_TESTS) struct bpf_spin_lock arena_bpf_test_lock;
#[no_mangle]
#[link_section = ".bss.ARENA_TESTS"]
pub static mut arena_bpf_test_lock: bpf_spin_lock = bpf_spin_lock { val: 0 };

/* Use the arena kfunc API while under a BPF lock. */
// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_kfuncs_under_bpf_lock(_ctx: *mut c_void) -> c_int {
    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        let mut page: *mut i8;
        let mut ret: c_int;

        bpf_spin_lock(&raw mut arena_bpf_test_lock);

        /* Get a separate region of the arena. */
        page = arena_base(&raw mut arena);
        ret = bpf_arena_reserve_pages(&raw mut arena, page as *mut c_void, 1);
        if ret != 0 {
            bpf_spin_unlock(&raw mut arena_bpf_test_lock);
            return 1;
        }

        bpf_arena_free_pages(&raw mut arena, page as *mut c_void, 1);

        page = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut i8;
        if page.is_null() {
            bpf_spin_unlock(&raw mut arena_bpf_test_lock);
            return 2;
        }

        bpf_arena_free_pages(&raw mut arena, page as *mut c_void, 1);

        bpf_spin_unlock(&raw mut arena_bpf_test_lock);
    }

    0
}

// #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)

/*
 * Test that scalar += PTR_TO_ARENA correctly upgrades the
 * destination register to a PTR_TO_ARENA.
 */
// SEC("syscall")
// __success __retval(0)
#[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
#[no_mangle]
pub unsafe extern "C" fn scalar_add_arena_ptr(_ctx: *mut c_void) -> c_int {
    let mut scalar: *mut u32;
    let mut arena_ptr: *mut u32;

    let base: *mut i8 = arena_base(&raw mut arena);

    core::arch::asm!(
        "{arena_ptr} = 8192;",
        "{arena_ptr} = addr_space_cast({arena_ptr}, 0x0, 0x1);",
        "{scalar} = 12;",
        "{scalar} += {arena_ptr};",
        scalar = out(reg) scalar,
        arena_ptr = out(reg) arena_ptr,
        in(reg) base,
    );
    0
}

/*
 * Tests that PTR_TO_ARENA + PTR_TO_ARENA is allowed.
 */
// SEC("syscall")
// __success __retval(0)
#[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
#[no_mangle]
pub unsafe extern "C" fn arena_ptr_add_arena_ptr(_ctx: *mut c_void) -> c_int {
    let mut arena_ptr2: *mut u32;
    let mut arena_ptr1: *mut u32;

    /* Needed for the verifier to link the arena to the subprog. */
    let base: *mut i8 = arena_base(&raw mut arena);

    core::arch::asm!(
        "{arena_ptr1} = 8192;",
        "{arena_ptr1} = addr_space_cast({arena_ptr1}, 0x0, 0x1);",
        "{arena_ptr2} = 4096;",
        "{arena_ptr2} = addr_space_cast({arena_ptr2}, 0x0, 0x1);",
        "{arena_ptr2} += {arena_ptr1};",
        arena_ptr2 = out(reg) arena_ptr2,
        arena_ptr1 = out(reg) arena_ptr1,
        in(reg) base,
    );
    0
}

// SEC("syscall")
// __success __retval(0)
#[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
#[no_mangle]
pub unsafe extern "C" fn scalar_xor_arena_ptr(_ctx: *mut c_void) -> c_int {
    let mut scalar: *mut u32;
    let mut arena_ptr: *mut u32;

    let base: *mut i8 = arena_base(&raw mut arena);

    core::arch::asm!(
        "{arena_ptr} = 8192;",
        "{arena_ptr} = addr_space_cast({arena_ptr}, 0x0, 0x1);",
        "{scalar} = 12;",
        "{scalar} ^= {arena_ptr};",
        scalar = out(reg) scalar,
        arena_ptr = out(reg) arena_ptr,
        in(reg) base,
    );
    0
}

/*
 * Tests that PTR_TO_ARENA and non-arena pointers can be added.
 */
// SEC("syscall")
// __success __retval(0)
#[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
#[no_mangle]
pub unsafe extern "C" fn arena_ptr_add_to_non_arena_ptr(ctx: *mut c_void) -> c_int {
    let mut arena_ptr: *mut u32;
    let mut dst: *mut c_void;

    let base: *mut i8 = arena_base(&raw mut arena);

    core::arch::asm!(
        "{arena_ptr} = 8192;",
        "{arena_ptr} = addr_space_cast({arena_ptr}, 0x0, 0x1);",
        "{dst} = {ctx};",
        "{dst} += {arena_ptr};",
        arena_ptr = out(reg) arena_ptr,
        dst = out(reg) dst,
        ctx = in(reg) ctx,
        in(reg) base,
    );

    let _ = ctx;

    0
}

// SEC("syscall")
// __success __retval(0)
#[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
#[no_mangle]
pub unsafe extern "C" fn non_arena_ptr_add_to_arena_ptr(ctx: *mut c_void) -> c_int {
    let mut arena_ptr: *mut u32;
    let mut src: *mut c_void;

    let base: *mut i8 = arena_base(&raw mut arena);

    core::arch::asm!(
        "{arena_ptr} = 8192;",
        "{arena_ptr} = addr_space_cast({arena_ptr}, 0x0, 0x1);",
        "{src} = {ctx};",
        "{arena_ptr} += {src};",
        arena_ptr = out(reg) arena_ptr,
        src = out(reg) src,
        ctx = in(reg) ctx,
        in(reg) base,
    );

    let _ = ctx;

    0
}

// SEC("socket")
// __description("arena and stack atomic at the same instruction")
// __failure __msg("same insn cannot be used with different pointers")
// __arch_x86_64
// __load_if_JITed()
// __naked
#[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
#[no_mangle]
pub unsafe extern "C" fn mixed_arena_stack_atomic() {
    core::arch::asm!(
        "r1 = {arena} ll;",
        "r6 = r10;",
        "r6 += -8;",
        "r9 = 0;",
        "*(u64 *)(r6 + 0) = r9;",
        "r7 = 8192;",
        "r7 = addr_space_cast(r7, 0, 1);",
        "call {bpf_get_prandom_u32};",
        "if w0 != 0 goto 1f;",
        "r8 = r6;",
        "goto 2f;",
        "1:",
        "r8 = r7;",
        "2:",
        "r9 = 1;",
        "lock *(u64 *)(r8 + 0) += r9;",
        "r0 = 0;",
        "exit;",
        arena = sym arena,
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn),
    );
}

// #endif /* defined(__BPF_FEATURE_ADDR_SPACE_CAST) */

// static __noinline
unsafe fn check_arena_arg_nonglobal(arg: *mut u32) -> *mut u32 {
    let val: u32 = core::ptr::read_volatile(arg);

    core::ptr::write_volatile(arg, val.wrapping_add(1));

    arg
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn check_arena_arg_global(arg: *mut u32) -> *mut u32 {
    let val: u32 = core::ptr::read_volatile(arg);

    core::ptr::write_volatile(arg, val.wrapping_add(1));

    arg
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn check_arena_arg_quals1(arg1: *mut u32, arg2: *mut u32) -> *mut u32 {
    core::ptr::write_volatile(arg1, core::ptr::read_volatile(arg1).wrapping_add(1));
    core::ptr::write_volatile(arg2, core::ptr::read_volatile(arg1).wrapping_add(1));

    arg2
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn check_arena_arg_quals2(arg1: *mut u32, arg2: *mut u32) -> *mut u32 {
    core::ptr::write_volatile(arg1, core::ptr::read_volatile(arg1).wrapping_add(1));
    core::ptr::write_volatile(arg2, core::ptr::read_volatile(arg2).wrapping_add(1));

    arg2
}

// SEC("syscall")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn check_arena_arg_ret(_ctx: *mut c_void) -> c_int {
    let page: *mut u32 = bpf_arena_alloc_pages(&raw mut arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0) as *mut u32;
    let mut arg: *mut u32 = page;
    let arg1: *mut u32;
    let ret1: *mut u32;
    let arg2: *mut u32;
    let ret2: *mut u32;

    if arg.is_null() {
        return 1;
    }

    /* Make sure we use {arg, ret}{1, 2}. */

    arg = check_arena_arg_nonglobal(page);
    arg = check_arena_arg_global(arg);

    arg1 = page;
    arg2 = page;
    ret1 = check_arena_arg_quals1(arg1, arg2);
    ret2 = check_arena_arg_quals2(arg1, arg2);

    if !(core::ptr::read_volatile(ret1) != 0 || core::ptr::read_volatile(ret2) != 0) {
        return -EINVAL;
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
