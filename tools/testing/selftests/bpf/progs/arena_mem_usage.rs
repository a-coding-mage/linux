// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external symbols/constants expected from the
// surrounding BPF build environment:
// <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_arena_common.h"

extern "C" {
    fn bpf_arena_alloc_pages(
        arena: *mut ArenaMap,
        pgoff: *mut core::ffi::c_void,
        page_cnt: i32,
        node: i32,
        flags: u64,
    ) -> *mut core::ffi::c_void;

    fn bpf_arena_free_pages(arena: *mut ArenaMap, ptr: *mut u8, page_cnt: i32);
}

const BPF_MAP_TYPE_ARENA: u32 = 33;
const BPF_F_MMAPABLE: u32 = 1024;
const NUMA_NO_NODE: i32 = -1;

#[repr(C)]
pub struct ArenaMap {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub map_extra: u64,
}

#[cfg(target_arch = "aarch64")]
const ARENA_MAP_EXTRA: u64 = 0x1_u64 << 32; /* start of mmap() region */

#[cfg(not(target_arch = "aarch64"))]
const ARENA_MAP_EXTRA: u64 = 0x1_u64 << 44; /* start of mmap() region */

#[link_section = ".maps"]
#[no_mangle]
pub static mut arena: ArenaMap = ArenaMap {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 1000, /* number of pages */
    map_extra: ARENA_MAP_EXTRA,
};

#[no_mangle]
pub static mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();

#[no_mangle]
pub static mut alloc_cnt: i32 = 0; /* in:  pages to allocate */

#[no_mangle]
pub static mut free_byte_off: i64 = 0; /* in:  byte offset within ptr to start freeing */

#[no_mangle]
pub static mut free_cnt: i32 = 0; /* in:  pages to free */

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn alloc(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    ptr = bpf_arena_alloc_pages(
        &mut arena,
        core::ptr::null_mut(),
        alloc_cnt,
        NUMA_NO_NODE,
        0,
    );
    /* Success/failure is checked from user space via skel->bss->ptr. */
    0
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn free_pages(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    if ptr.is_null() {
        return 1;
    }
    bpf_arena_free_pages(
        &mut arena,
        (ptr as *mut u8).offset(free_byte_off as isize),
        free_cnt,
    );
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
