// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2020 Google LLC.
 */

// Dependencies from the C source:
// <errno.h>, <linux/bpf.h>, <linux/ip.h>, <linux/udp.h>,
// <bpf/bpf_helpers.h>, and "progs/cg_storage_multi.h".

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

pub type __u32 = u32;
pub type __u64 = u64;

// Provided by "progs/cg_storage_multi.h" in the original repository.
#[repr(C)]
pub struct cgroup_value {
    pub egress_pkts: __u64,
    pub ingress_pkts: __u64,
}

const BPF_MAP_TYPE_CGROUP_STORAGE: __u32 = 19;

#[repr(C)]
pub struct cgroup_storage_map {
    pub type_: __u32,
    pub key: __u64,
    pub value: cgroup_value,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut cgroup_storage: cgroup_storage_map = cgroup_storage_map {
    type_: BPF_MAP_TYPE_CGROUP_STORAGE,
    key: 0,
    value: cgroup_value {
        egress_pkts: 0,
        ingress_pkts: 0,
    },
};

#[no_mangle]
pub static mut invocations: __u32 = 0;

extern "C" {
    fn bpf_get_local_storage(map: *mut cgroup_storage_map, flags: __u64) -> *mut cgroup_value;
}

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn egress1(skb: *mut __sk_buff) -> i32 {
    let ptr_cg_storage: *mut cgroup_value = bpf_get_local_storage(&mut cgroup_storage, 0);

    core::sync::atomic::AtomicU64::from_ptr(&mut (*ptr_cg_storage).egress_pkts)
        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    core::sync::atomic::AtomicU32::from_ptr(&mut invocations)
        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);

    1
}

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn egress2(skb: *mut __sk_buff) -> i32 {
    let ptr_cg_storage: *mut cgroup_value = bpf_get_local_storage(&mut cgroup_storage, 0);

    core::sync::atomic::AtomicU64::from_ptr(&mut (*ptr_cg_storage).egress_pkts)
        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    core::sync::atomic::AtomicU32::from_ptr(&mut invocations)
        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);

    1
}

#[link_section = "cgroup_skb/ingress"]
#[no_mangle]
pub unsafe extern "C" fn ingress(skb: *mut __sk_buff) -> i32 {
    let ptr_cg_storage: *mut cgroup_value = bpf_get_local_storage(&mut cgroup_storage, 0);

    core::sync::atomic::AtomicU64::from_ptr(&mut (*ptr_cg_storage).ingress_pkts)
        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    core::sync::atomic::AtomicU32::from_ptr(&mut invocations)
        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
