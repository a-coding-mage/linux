// SPDX-License-Identifier: GPL-2.0

// C dependency: <linux/types.h> provides __u8 and __u64.

pub const MAX_PERCPU_PACKETS: usize = 32;

/* sizeof(struct bpf_local_storage_elem):
 *
 * It is about 128 bytes on x86_64 and 512 bytes on s390x, but allocate more to
 * account for possible layout changes, different architectures, etc.
 * The kernel will wrap up to PAGE_SIZE internally anyway.
 */
pub const SIZEOF_BPF_LOCAL_STORAGE_ELEM: usize = 768;

/* Try to estimate kernel's BPF_LOCAL_STORAGE_MAX_VALUE_SIZE: */
pub const BPF_LOCAL_STORAGE_MAX_VALUE_SIZE: usize =
    0xFFFFusize - SIZEOF_BPF_LOCAL_STORAGE_ELEM;

pub const PCPU_MIN_UNIT_SIZE: usize = 32768;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_net_cnt__bindgen_ty_1 {
    pub packets: __u64,
    pub bytes: __u64,

    pub prev_ts: __u64,

    pub prev_packets: __u64,
    pub prev_bytes: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union percpu_net_cnt {
    pub __bindgen_anon_1: percpu_net_cnt__bindgen_ty_1,
    pub data: [__u8; PCPU_MIN_UNIT_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_cnt__bindgen_ty_1 {
    pub packets: __u64,
    pub bytes: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union net_cnt {
    pub __bindgen_anon_1: net_cnt__bindgen_ty_1,
    pub data: [__u8; BPF_LOCAL_STORAGE_MAX_VALUE_SIZE],
}
