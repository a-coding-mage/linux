// SPDX-License-Identifier: GPL-2.0

// Dependencies from <linux/bpf.h> and <bpf/bpf_helpers.h>:
// BPF_MAP_TYPE_DEVMAP, LIBBPF_PIN_BY_NAME, SEC, __uint, __type, __u32.

pub type __u32 = u32;

extern "C" {
    pub static BPF_MAP_TYPE_DEVMAP: __u32;
    pub static LIBBPF_PIN_BY_NAME: __u32;
}

#[repr(C)]
pub struct pinmap1 {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key: __u32,
    pub value: __u32,
    pub pinning: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static pinmap1: pinmap1 = pinmap1 {
    type_: unsafe { BPF_MAP_TYPE_DEVMAP },
    max_entries: 1,
    key: 0,
    value: 0,
    pinning: unsafe { LIBBPF_PIN_BY_NAME },
};

#[repr(C)]
pub struct pinmap2 {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key: __u32,
    pub value: __u32,
    pub pinning: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static pinmap2: pinmap2 = pinmap2 {
    type_: unsafe { BPF_MAP_TYPE_DEVMAP },
    max_entries: 2,
    key: 0,
    value: 0,
    pinning: unsafe { LIBBPF_PIN_BY_NAME },
};
