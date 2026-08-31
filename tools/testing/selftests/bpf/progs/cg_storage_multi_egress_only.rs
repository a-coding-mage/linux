// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2020 Google LLC.
 */

// C dependencies translated as external Rust dependencies:
// errno.h, linux/bpf.h, linux/ip.h, linux/udp.h, bpf/bpf_helpers.h
// and "progs/cg_storage_multi.h".

extern "C" {
    fn bpf_get_local_storage(map: *mut core::ffi::c_void, flags: u64) -> *mut cgroup_value;
}

type __u32 = u32;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup_value {
    pub egress_pkts: __u32,
}

// struct {
//     __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
//     __type(key, struct bpf_cgroup_storage_key);
//     __type(value, struct cgroup_value);
// } cgroup_storage SEC(".maps");
#[repr(C)]
pub struct cgroup_storage_def {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut cgroup_storage: cgroup_storage_def = cgroup_storage_def { _private: [] };

#[no_mangle]
pub static mut invocations: __u32 = 0;

#[no_mangle]
#[link_section = "cgroup_skb/egress"]
pub unsafe extern "C" fn egress(skb: *mut __sk_buff) -> i32 {
    let ptr_cg_storage: *mut cgroup_value =
        bpf_get_local_storage(&raw mut cgroup_storage as *mut core::ffi::c_void, 0);

    core::intrinsics::atomic_xadd_relaxed(&raw mut (*ptr_cg_storage).egress_pkts, 1);
    core::intrinsics::atomic_xadd_relaxed(&raw mut invocations, 1);

    1
}
