// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_LRU_HASH: __u32 = 9;
const BPF_ANY: __u64 = 0;

#[repr(C)]
pub struct lru_map_def {
    // __uint(type, BPF_MAP_TYPE_LRU_HASH);
    pub type_: __u32,
    // __uint(max_entries, 64);
    pub max_entries: __u32,
    // __type(key, __u32);
    // __type(value, __u64);
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut lru_map: lru_map_def = lru_map_def {
    type_: BPF_MAP_TYPE_LRU_HASH,
    max_entries: 64,
};

#[unsafe(no_mangle)]
pub static mut hits: i32 = 0;

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> __u32;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i64;
}

#[unsafe(link_section = "perf_event")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oncpu(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /*
     * Key range deliberately wider than max_entries to force LRU
     * eviction on every other update.
     */
    let key: __u32 = unsafe { bpf_get_prandom_u32() } % 128;
    let do_update: bool = (unsafe { bpf_get_prandom_u32() } & 1) != 0;
    let val: __u64 = 1;

    if do_update {
        unsafe {
            bpf_map_update_elem(
                core::ptr::addr_of_mut!(lru_map).cast::<core::ffi::c_void>(),
                core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
                core::ptr::addr_of!(val).cast::<core::ffi::c_void>(),
                BPF_ANY,
            );
        }
    } else {
        unsafe {
            bpf_map_delete_elem(
                core::ptr::addr_of_mut!(lru_map).cast::<core::ffi::c_void>(),
                core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
            );
        }
    }
    unsafe {
        core::intrinsics::atomic_xadd_acqrel(core::ptr::addr_of_mut!(hits), 1);
    }
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
