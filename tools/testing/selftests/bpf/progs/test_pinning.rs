// SPDX-License-Identifier: GPL-2.0

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct pinmap {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    pub type_: u32,
    // __uint(max_entries, 1);
    pub max_entries: u32,
    // __type(key, __u32);
    pub key: __u32,
    // __type(value, __u64);
    pub value: __u64,
    // __uint(pinning, LIBBPF_PIN_BY_NAME);
    pub pinning: u32,
}

#[unsafe(link_section = ".maps")]
pub static mut pinmap: pinmap = pinmap {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key: 0,
    value: 0,
    pinning: LIBBPF_PIN_BY_NAME,
};

#[repr(C)]
pub struct nopinmap {
    // __uint(type, BPF_MAP_TYPE_HASH);
    pub type_: u32,
    // __uint(max_entries, 1);
    pub max_entries: u32,
    // __type(key, __u32);
    pub key: __u32,
    // __type(value, __u64);
    pub value: __u64,
}

#[unsafe(link_section = ".maps")]
pub static mut nopinmap: nopinmap = nopinmap {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct nopinmap2 {
    // __uint(type, BPF_MAP_TYPE_HASH);
    pub type_: u32,
    // __uint(max_entries, 1);
    pub max_entries: u32,
    // __type(key, __u32);
    pub key: __u32,
    // __type(value, __u64);
    pub value: __u64,
    // __uint(pinning, LIBBPF_PIN_NONE);
    pub pinning: u32,
}

#[unsafe(link_section = ".maps")]
pub static mut nopinmap2: nopinmap2 = nopinmap2 {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key: 0,
    value: 0,
    pinning: LIBBPF_PIN_NONE,
};

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];
