// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

pub type __u32 = u32;

pub const MAX_ENTRIES: usize = 100000000;

#[repr(C)]
pub struct trie_key {
    pub prefixlen: __u32,
    pub data: __u32,
}

// Direct translation of libbpf map-definition macros:
// __uint(name, val) -> int (*name)[val]
// __type(name, val) -> typeof(val) *name
#[repr(C)]
pub struct trie_free_map_def {
    pub r#type: *mut [i32; BPF_MAP_TYPE_LPM_TRIE],
    pub key: *mut trie_key,
    pub value: *mut __u32,
    pub map_flags: *mut [i32; BPF_F_NO_PREALLOC],
    pub max_entries: *mut [i32; MAX_ENTRIES],
}

pub const BPF_MAP_TYPE_LPM_TRIE: usize = 11;
pub const BPF_F_NO_PREALLOC: usize = 1;

#[no_mangle]
#[link_section = ".maps"]
pub static mut trie_free_map: trie_free_map_def = trie_free_map_def {
    r#type: core::ptr::null_mut(),
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
    map_flags: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
