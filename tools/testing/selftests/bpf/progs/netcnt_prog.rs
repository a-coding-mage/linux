// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/progs/netcnt_prog.c.
// Original dependencies: <linux/bpf.h>, <linux/version.h>,
// <bpf/bpf_helpers.h>, "netcnt_common.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u64 = u64;
type __u32 = u32;

const MAX_BPS: __u64 = 3 * 1024 * 1024;

const REFRESH_TIME_NS: __u64 = 100000000;
const NS_PER_SEC: __u64 = 1000000000;

extern "C" {
    static BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: __u32;
    static BPF_MAP_TYPE_CGROUP_STORAGE: __u32;
    static MAX_PERCPU_PACKETS: __u64;
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
}

#[repr(C)]
pub union percpu_net_cnt {
    pub packets: __u64,
    pub bytes: __u64,
    pub prev_ts: __u64,
    pub prev_packets: __u64,
    pub prev_bytes: __u64,
}

#[repr(C)]
pub union net_cnt {
    pub packets: __u64,
    pub bytes: __u64,
}

#[repr(C)]
pub struct bpf_map_def_percpu_netcnt {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct bpf_map_def_netcnt {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut percpu_netcnt: bpf_map_def_percpu_netcnt = bpf_map_def_percpu_netcnt {
    type_: 0,
    key_size: core::mem::size_of::<bpf_cgroup_storage_key>() as __u32,
    value_size: core::mem::size_of::<percpu_net_cnt>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut netcnt: bpf_map_def_netcnt = bpf_map_def_netcnt {
    type_: 0,
    key_size: core::mem::size_of::<bpf_cgroup_storage_key>() as __u32,
    value_size: core::mem::size_of::<net_cnt>() as __u32,
};

extern "C" {
    fn bpf_get_local_storage(map: *mut core::ffi::c_void, flags: __u64) -> *mut core::ffi::c_void;
    fn bpf_ktime_get_ns() -> __u64;
}

#[link_section = "cgroup/skb"]
#[no_mangle]
pub unsafe extern "C" fn bpf_nextcnt(skb: *mut __sk_buff) -> i32 {
    let mut cnt: *mut net_cnt;
    let mut percpu_cnt: *mut percpu_net_cnt;
    let mut ts: __u64;
    let mut dt: __u64;
    let mut ret: i32;

    cnt = bpf_get_local_storage(
        &mut netcnt as *mut bpf_map_def_netcnt as *mut core::ffi::c_void,
        0,
    ) as *mut net_cnt;
    percpu_cnt = bpf_get_local_storage(
        &mut percpu_netcnt as *mut bpf_map_def_percpu_netcnt as *mut core::ffi::c_void,
        0,
    ) as *mut percpu_net_cnt;

    (*percpu_cnt).packets = (*percpu_cnt).packets.wrapping_add(1);
    (*percpu_cnt).bytes = (*percpu_cnt).bytes.wrapping_add((*skb).len as __u64);

    if (*percpu_cnt).packets > MAX_PERCPU_PACKETS {
        core::intrinsics::atomic_xadd_seqcst(&mut (*cnt).packets, (*percpu_cnt).packets);
        (*percpu_cnt).packets = 0;

        core::intrinsics::atomic_xadd_seqcst(&mut (*cnt).bytes, (*percpu_cnt).bytes);
        (*percpu_cnt).bytes = 0;
    }

    ts = bpf_ktime_get_ns();
    dt = ts.wrapping_sub((*percpu_cnt).prev_ts);

    dt = dt.wrapping_mul(MAX_BPS);
    dt /= NS_PER_SEC;

    if (*cnt)
        .bytes
        .wrapping_add((*percpu_cnt).bytes)
        .wrapping_sub((*percpu_cnt).prev_bytes)
        < dt
    {
        ret = 1;
    } else {
        ret = 0;
    }

    if dt > REFRESH_TIME_NS {
        (*percpu_cnt).prev_ts = ts;
        (*percpu_cnt).prev_packets = (*cnt).packets;
        (*percpu_cnt).prev_bytes = (*cnt).bytes;
    }

    (ret != 0) as i32
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
