// SPDX-License-Identifier: GPL-2.0-only
// Original C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

const TEST_COMM_LEN: usize = 16;

type __u32 = u32;
type u32 = core::ffi::c_uint;

const BPF_MAP_TYPE_CGROUP_ARRAY: u32 = 8;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub tgid: __u32,
    pub comm: [core::ffi::c_char; TEST_COMM_LEN],
}

#[repr(C)]
pub struct cgroup_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static cgroup_map: cgroup_map_def = cgroup_map_def {
    type_: BPF_MAP_TYPE_CGROUP_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
};

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

extern "C" {
    fn bpf_get_current_task() -> *mut core::ffi::c_void;
    fn bpf_probe_read_kernel(
        dst: *mut core::ffi::c_void,
        size: u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> core::ffi::c_long;
    fn bpf_probe_read_kernel_str(
        dst: *mut core::ffi::c_void,
        size: u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> core::ffi::c_long;
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_skb_helpers(skb: *mut __sk_buff) -> core::ffi::c_int {
    let mut task: *mut task_struct;
    let mut comm: [core::ffi::c_char; TEST_COMM_LEN] = [0; TEST_COMM_LEN];
    let mut tpid: __u32 = 0;

    let _ = skb;

    task = bpf_get_current_task() as *mut task_struct;
    bpf_probe_read_kernel(
        &mut tpid as *mut __u32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&tpid) as u32,
        &(*task).tgid as *const __u32 as *const core::ffi::c_void,
    );
    bpf_probe_read_kernel_str(
        &mut comm as *mut [core::ffi::c_char; TEST_COMM_LEN] as *mut core::ffi::c_void,
        core::mem::size_of_val(&comm) as u32,
        &(*task).comm as *const [core::ffi::c_char; TEST_COMM_LEN] as *const core::ffi::c_void,
    );
    return 0;
}
