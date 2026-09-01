// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependency intent:
// #define BPF_NO_KFUNC_PROTOTYPES
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "bpf_experimental.h"
// #include <bpf_arena_common.h>

pub const ARENA_PAGES: usize = 32;

unsafe extern "C" {
    static __PAGE_SIZE: u64;
    static PAGE_SIZE: usize;

    fn arena_base(map: *mut ArenaMapDef) -> *mut core::ffi::c_void;
    fn bpf_arena_reserve_pages(
        map: *mut ArenaMapDef,
        addr: *mut core::ffi::c_void,
        page_cnt: u64,
    ) -> i32;
}

pub const BPF_MAP_TYPE_ARENA: u32 = 1; // External BPF enum value supplied by headers in C.
pub const BPF_F_MMAPABLE: u32 = 1; // External BPF flag value supplied by headers in C.

#[repr(C)]
pub struct ArenaMapDef {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub map_extra: u64,
}

// Original C places this object in SEC(".maps").
// On __TARGET_ARCH_arm64:
//   map_extra = (1ull << 32) | (~0u - __PAGE_SIZE * ARENA_PAGES + 1)
// Otherwise:
//   map_extra = (1ull << 44) | (~0u - __PAGE_SIZE * ARENA_PAGES + 1)
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut arena: ArenaMapDef = ArenaMapDef {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: ARENA_PAGES as u32,
    // Requires the external C header constant __PAGE_SIZE. Kept as zero here
    // because Rust static initializers cannot call or read an extern static.
    map_extra: 0,
};

/*
 * Fill the entire arena with global data.
 * The offset into the arena should be 0.
 */
// Original C declaration: char __arena global_data[ARENA_PAGES][PAGE_SIZE];
// PAGE_SIZE and the __arena address space are provided by BPF headers.
#[unsafe(no_mangle)]
pub static mut global_data: [[i8; 0]; ARENA_PAGES] = [[0; 0]; ARENA_PAGES];

// Original C attributes: SEC("syscall") __success __retval(0)
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_reserve2(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    // #if defined(__BPF_FEATURE_ADDR_SPACE_CAST)
    #[cfg(__BPF_FEATURE_ADDR_SPACE_CAST)]
    {
        let guard: *mut core::ffi::c_void;
        let ret: i32;

        guard = unsafe { arena_base(&raw mut arena) };

        /* Make sure the data at offset 0 case is properly handled. */
        ret = unsafe { bpf_arena_reserve_pages(&raw mut arena, guard, 1) };
        if ret == 0 {
            return 1;
        }
    }

    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
