// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Tessares SA <http://www.tessares.net> */

/* Dependencies from the original C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 */

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_PERCPU_HASH: __u32 = 5;
const BPF_NOEXIST: __u64 = 1;

#[no_mangle]
pub static mut inKey: __u64 = 0;
#[no_mangle]
pub static mut inValue: __u64 = 0;
#[no_mangle]
pub static mut inPid: __u32 = 0;

#[repr(C)]
pub struct hashmap1_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut hashmap1: hashmap1_def = hashmap1_def {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    max_entries: 2,
    key_size: core::mem::size_of::<__u64>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
}

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_getpgid"]
pub unsafe extern "C" fn sysenter_getpgid(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;

    /* Just do it for once, when called from our own test prog. This
     * ensures the map value is only updated for a single CPU.
     */
    let cur_pid: i32 = (bpf_get_current_pid_tgid() >> 32) as i32;

    if cur_pid == inPid as i32 {
        bpf_map_update_elem(
            &mut hashmap1 as *mut _ as *mut core::ffi::c_void,
            &inKey as *const _ as *const core::ffi::c_void,
            &inValue as *const _ as *const core::ffi::c_void,
            BPF_NOEXIST,
        );
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
