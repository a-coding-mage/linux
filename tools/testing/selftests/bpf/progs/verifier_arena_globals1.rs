// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies: vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h,
// bpf_experimental.h, bpf_arena_common.h, bpf_misc.h.
// #define BPF_NO_KFUNC_PROTOTYPES

type u8 = ::core::ffi::c_uchar;
type u64 = ::core::ffi::c_ulonglong;

const GLOBAL_PAGES: usize = 16;
const ARENA_PAGES: usize = 1usize << (32 - ffs_usize(__PAGE_SIZE) + 1);

const fn ffs_usize(value: usize) -> usize {
    let mut bit = 1usize;
    let mut v = value;

    while v != 0 {
        if (v & 1) != 0 {
            return bit;
        }
        v >>= 1;
        bit += 1;
    }

    0
}

#[repr(C)]
pub struct ArenaMap {
    type_: u32,
    map_flags: u32,
    max_entries: usize,
    map_extra: u64,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut arena: ArenaMap = ArenaMap {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: ARENA_PAGES,
    #[cfg(target_arch = "aarch64")]
    map_extra: ((1u64 << 32) | ((!0u32 as usize) - __PAGE_SIZE * ARENA_PAGES + 1) as u64),
    #[cfg(not(target_arch = "aarch64"))]
    map_extra: ((1u64 << 44) | ((!0u32 as usize) - __PAGE_SIZE * ARENA_PAGES + 1) as u64),
};

/*
 * Global data, to be placed at the end of the arena.
 *
 * C used the __arena address-space qualifier.
 */
#[no_mangle]
pub static mut global_data: [[::core::ffi::c_char; PAGE_SIZE]; GLOBAL_PAGES] =
    [[0; PAGE_SIZE]; GLOBAL_PAGES];

unsafe extern "C" {
    fn arena_base(map: *mut ArenaMap) -> u64;
    fn bpf_arena_reserve_pages(map: *mut ArenaMap, addr: *mut ::core::ffi::c_void, page_cnt: usize) -> i32;
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn check_reserve1(ctx: *mut ::core::ffi::c_void) -> i32 {
    let _ = ctx;

    // Original C condition: #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        const magic: u8 = 0x5a;
        let mut guard: *mut u8;
        let mut globals: *mut u8;
        let mut ptr: *mut ::core::ffi::c_char;
        let mut i: i32;
        let mut ret: i32;

        guard = arena_base(&mut arena) as *mut u8;
        globals = (arena_base(&mut arena) + ((ARENA_PAGES - GLOBAL_PAGES) * PAGE_SIZE) as u64) as *mut u8;

        /* Reserve the region we've offset the globals by. */
        ret = bpf_arena_reserve_pages(
            &mut arena,
            guard as *mut ::core::ffi::c_void,
            ARENA_PAGES - GLOBAL_PAGES,
        );
        if ret != 0 {
            return 1;
        }

        /* Make sure the globals are in the expected offset. */
        ret = bpf_arena_reserve_pages(&mut arena, globals as *mut ::core::ffi::c_void, 1);
        if ret == 0 {
            return 2;
        }

        /* Verify globals are properly mapped in by libbpf. */
        i = 0;
        while i < GLOBAL_PAGES as i32 {
            ptr = &mut global_data[i as usize][PAGE_SIZE / 2] as *mut ::core::ffi::c_char;

            ::core::ptr::write_volatile(ptr, magic as ::core::ffi::c_char);
            if ::core::ptr::read_volatile(ptr) != magic as ::core::ffi::c_char {
                return i + 3;
            }

            i += 1;
        }
    }

    0
}

/*
 * Relocation check by reading directly into the global data w/o using symbols.
 */
#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn check_relocation(ctx: *mut ::core::ffi::c_void) -> i32 {
    let _ = ctx;

    // Original C condition: #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        const magic: u8 = 0xfa;
        let ptr: *mut u8;

        global_data[GLOBAL_PAGES - 1][PAGE_SIZE / 2] = magic as ::core::ffi::c_char;
        ptr = (ARENA_PAGES * PAGE_SIZE - PAGE_SIZE / 2) as u64 as *mut u8;
        if *ptr != magic {
            return 1;
        }
    }

    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
