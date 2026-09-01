// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
/* C source defined BPF_NO_KFUNC_PROTOTYPES before including BPF headers. */
/* Dependencies from C includes: vmlinux.h, bpf_helpers.h, bpf_tracing.h,
 * bpf_core_read.h, bpf_experimental.h, bpf_arena_alloc.h, bpf_arena_list.h.
 */

#[repr(C)]
pub struct arena_list_node {
    _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct arena_list_head {
    _bindgen_opaque_blob: [u8; 0],
}

pub type __u64 = u64;

pub const BPF_MAP_TYPE_ARENA: u32 = 33;
pub const BPF_F_MMAPABLE: u32 = 1024;

#[repr(C)]
pub struct ArenaMapDef {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub map_extra: u64,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut arena: ArenaMapDef = ArenaMapDef {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 100, /* number of pages */
    /* C used 0x1ull << 32 for __TARGET_ARCH_arm64, otherwise 0x1ull << 44. */
    #[cfg(target_arch = "aarch64")]
    map_extra: 0x1_u64 << 32, /* start of mmap() region */
    #[cfg(not(target_arch = "aarch64"))]
    map_extra: 0x1_u64 << 44, /* start of mmap() region */
};

#[repr(C)]
pub struct elem {
    pub node: arena_list_node,
    pub value: __u64,
}

unsafe extern "C" {
    pub fn bpf_alloc(size: usize) -> *mut core::ffi::c_void;
    pub fn bpf_free(ptr: *mut core::ffi::c_void);
    pub fn list_add_head(node: *mut arena_list_node, head: *mut arena_list_head);
    pub fn list_del(node: *mut arena_list_node);
    pub fn bpf_rcu_read_lock();
    pub fn bpf_rcu_read_unlock();

    pub static mut can_loop: bool;
}

#[no_mangle]
pub static mut list_head: *mut arena_list_head = core::ptr::null_mut();
#[no_mangle]
pub static mut list_sum: i32 = 0;
#[no_mangle]
pub static mut cnt: i32 = 0;
#[no_mangle]
pub static mut skip: bool = false;
#[no_mangle]
pub static nonsleepable: bool = false;

/* C condition: #ifdef __BPF_FEATURE_ADDR_SPACE_CAST. */
#[cfg(feature = "__BPF_FEATURE_ADDR_SPACE_CAST")]
#[no_mangle]
pub static mut arena_sum: i64 = 0;
#[cfg(feature = "__BPF_FEATURE_ADDR_SPACE_CAST")]
#[no_mangle]
pub static mut test_val: i32 = 1;
#[cfg(feature = "__BPF_FEATURE_ADDR_SPACE_CAST")]
#[no_mangle]
pub static mut global_head: arena_list_head = arena_list_head {
    _bindgen_opaque_blob: [],
};

#[cfg(not(feature = "__BPF_FEATURE_ADDR_SPACE_CAST"))]
#[link_section = ".addr_space.1"]
#[no_mangle]
pub static mut arena_sum: i64 = 0;
#[cfg(not(feature = "__BPF_FEATURE_ADDR_SPACE_CAST"))]
#[link_section = ".addr_space.1"]
#[no_mangle]
pub static mut test_val: i32 = 0;

#[no_mangle]
pub static mut zero: i32 = 0;

#[cfg(feature = "__BPF_FEATURE_ADDR_SPACE_CAST")]
#[inline(always)]
unsafe fn arena_list_for_each_entry_and_delete(mut n: *mut elem, mut sum: i32) -> i32 {
    /* Translation placeholder for list_for_each_entry(n, list_head, node), whose
     * exact pointer-walking implementation is supplied by bpf_arena_list.h.
     */
    while !n.is_null() {
        sum = sum.wrapping_add((*n).value as i32);
        arena_sum = arena_sum.wrapping_add((*n).value as i64);
        list_del(&mut (*n).node as *mut arena_list_node);
        bpf_free(n as *mut core::ffi::c_void);
        n = core::ptr::null_mut();
    }
    sum
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn arena_list_add(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* C condition: #ifdef __BPF_FEATURE_ADDR_SPACE_CAST. */
    #[cfg(feature = "__BPF_FEATURE_ADDR_SPACE_CAST")]
    {
        let mut i: __u64;

        list_head = &mut global_head as *mut arena_list_head;

        i = zero as __u64;
        while i < cnt as __u64 && can_loop {
            let n: *mut elem = bpf_alloc(core::mem::size_of::<elem>()) as *mut elem;

            test_val = test_val.wrapping_add(1);
            (*n).value = i;
            arena_sum = arena_sum.wrapping_add(i as i64);
            list_add_head(&mut (*n).node as *mut arena_list_node, list_head);
            i = i.wrapping_add(1);
        }
    }

    #[cfg(not(feature = "__BPF_FEATURE_ADDR_SPACE_CAST"))]
    {
        skip = true;
    }

    0
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn arena_list_del(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* C condition: #ifdef __BPF_FEATURE_ADDR_SPACE_CAST. */
    #[cfg(feature = "__BPF_FEATURE_ADDR_SPACE_CAST")]
    {
        let n: *mut elem;
        let mut sum: i32 = 0;

        /* Take rcu_read_lock to test non-sleepable context */
        if nonsleepable {
            bpf_rcu_read_lock();
        }

        arena_sum = 0;
        n = core::ptr::null_mut();
        sum = arena_list_for_each_entry_and_delete(n, sum);
        list_sum = sum;

        if nonsleepable {
            bpf_rcu_read_unlock();
        }
    }

    #[cfg(not(feature = "__BPF_FEATURE_ADDR_SPACE_CAST"))]
    {
        skip = true;
    }

    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
