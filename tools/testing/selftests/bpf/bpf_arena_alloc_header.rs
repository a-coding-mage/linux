// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
//
// C header dependency intent: #include <bpf_arena_common.h>

// C macro fallback:
// #ifndef __round_mask
// #define __round_mask(x, y) ((__typeof__(x))((y)-1))
// #endif
#[inline(always)]
pub const fn __round_mask_u32(_x: u32, y: u32) -> u32 {
    y.wrapping_sub(1)
}

// C macro fallback:
// #ifndef round_up
// #define round_up(x, y) ((((x)-1) | __round_mask(x, y))+1)
// #endif
#[inline(always)]
pub const fn round_up_u32(x: u32, y: u32) -> u32 {
    (x.wrapping_sub(1) | __round_mask_u32(x, y)).wrapping_add(1)
}

use core::mem::size_of;

// The following items translate the #ifdef __BPF__ branch. Names such as
// cpumask, PAGE_SIZE, NUMA_NO_NODE, arena, bpf_get_smp_processor_id,
// bpf_arena_alloc_pages, bpf_arena_free_pages, and cast_kern are supplied by
// the Rust translation of <bpf_arena_common.h> or other BPF dependencies.

unsafe extern "C" {
    pub static mut arena: bpf_arena;

    pub fn bpf_get_smp_processor_id() -> u32;
    pub fn bpf_arena_alloc_pages(
        arena: *mut bpf_arena,
        addr: *mut core::ffi::c_void,
        page_cnt: u32,
        node: i32,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_arena_free_pages(
        arena: *mut bpf_arena,
        addr: *mut core::ffi::c_void,
        page_cnt: u32,
    );
    pub fn cast_kern(addr: *mut core::ffi::c_void);
}

pub const NR_CPUS: usize = size_of::<cpumask>() * 8;

pub static mut page_frag_cur_page: [*mut core::ffi::c_void; NR_CPUS] =
    [core::ptr::null_mut(); NR_CPUS];
pub static mut page_frag_cur_offset: [i32; NR_CPUS] = [0; NR_CPUS];

// Simple page_frag allocator
#[inline(always)]
pub unsafe fn bpf_alloc(mut size: u32) -> *mut core::ffi::c_void {
    let mut obj_cnt: *mut u64;
    let cpu: u32 = unsafe { bpf_get_smp_processor_id() };
    let mut page: *mut core::ffi::c_void = unsafe { page_frag_cur_page[cpu as usize] };
    let cur_offset: *mut i32 = unsafe { &raw mut page_frag_cur_offset[cpu as usize] };
    let mut offset: i32;

    size = round_up_u32(size, 8);
    if size >= (PAGE_SIZE - 8) as u32 {
        return core::ptr::null_mut();
    }
    if page.is_null() {
        loop {
            page = unsafe {
                bpf_arena_alloc_pages(
                    &raw mut arena,
                    core::ptr::null_mut(),
                    1,
                    NUMA_NO_NODE,
                    0,
                )
            };
            if page.is_null() {
                return core::ptr::null_mut();
            }
            unsafe {
                cast_kern(page);
                page_frag_cur_page[cpu as usize] = page;
                *cur_offset = PAGE_SIZE - 8;
            }
            obj_cnt = unsafe { (page as *mut u8).add((PAGE_SIZE - 8) as usize) as *mut u64 };
            unsafe {
                *obj_cnt = 0;
            }

            offset = unsafe { *cur_offset } - size as i32;
            if offset >= 0 {
                break;
            }
        }
    } else {
        unsafe {
            cast_kern(page);
        }
        obj_cnt = unsafe { (page as *mut u8).add((PAGE_SIZE - 8) as usize) as *mut u64 };

        offset = unsafe { *cur_offset } - size as i32;
        if offset < 0 {
            loop {
                page = unsafe {
                    bpf_arena_alloc_pages(
                        &raw mut arena,
                        core::ptr::null_mut(),
                        1,
                        NUMA_NO_NODE,
                        0,
                    )
                };
                if page.is_null() {
                    return core::ptr::null_mut();
                }
                unsafe {
                    cast_kern(page);
                    page_frag_cur_page[cpu as usize] = page;
                    *cur_offset = PAGE_SIZE - 8;
                }
                obj_cnt =
                    unsafe { (page as *mut u8).add((PAGE_SIZE - 8) as usize) as *mut u64 };
                unsafe {
                    *obj_cnt = 0;
                }

                offset = unsafe { *cur_offset } - size as i32;
                if offset >= 0 {
                    break;
                }
            }
        }
    }

    unsafe {
        *obj_cnt = (*obj_cnt).wrapping_add(1);
        *cur_offset = offset;
    }
    unsafe { (page as *mut u8).add(offset as usize) as *mut core::ffi::c_void }
}

#[inline(always)]
pub unsafe fn bpf_free(mut addr: *mut core::ffi::c_void) {
    let obj_cnt: *mut u64;

    addr = ((addr as isize) & !((PAGE_SIZE - 1) as isize)) as *mut core::ffi::c_void;
    obj_cnt = unsafe { (addr as *mut u8).add((PAGE_SIZE - 8) as usize) as *mut u64 };
    unsafe {
        *obj_cnt = (*obj_cnt).wrapping_sub(1);
        if *obj_cnt == 0 {
            bpf_arena_free_pages(&raw mut arena, addr, 1);
        }
    }
}

// C #else branch:
// static inline void __arena* bpf_alloc(unsigned int size) { return NULL; }
// static inline void bpf_free(void __arena *addr) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
