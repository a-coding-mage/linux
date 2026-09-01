// SPDX-License-Identifier: GPL-2.0

// C dependencies originally included:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct storage_map {
    pub type_: __uint_type_BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE,
    pub key: __type_key_struct_bpf_cgroup_storage_key,
    pub value: __type_value___u64,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut storage_map: storage_map = storage_map {
    type_: __uint_type_BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE,
    key: __type_key_struct_bpf_cgroup_storage_key,
    value: __type_value___u64,
};

#[repr(C)]
pub struct prog_array {
    pub type_: __uint_type_BPF_MAP_TYPE_PROG_ARRAY,
    pub max_entries: __uint_max_entries_1,
    pub key_size: __uint_key_size_sizeof___u32,
    pub value_size: __uint_value_size_sizeof___u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut prog_array: prog_array = prog_array {
    type_: __uint_type_BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: __uint_max_entries_1,
    key_size: __uint_key_size_sizeof___u32,
    value_size: __uint_value_size_sizeof___u32,
};

extern "C" {
    fn bpf_get_local_storage(map: *mut core::ffi::c_void, flags: __u64) -> *mut core::ffi::c_void;
    fn bpf_tail_call(ctx: *mut __sk_buff, prog_array_map: *mut core::ffi::c_void, index: __u32);
}

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn prog_array_owner(skb: *mut __sk_buff) -> core::ffi::c_int {
    let mut storage: *mut __u64;

    storage = bpf_get_local_storage(
        core::ptr::addr_of_mut!(storage_map) as *mut core::ffi::c_void,
        0,
    ) as *mut __u64;
    if !storage.is_null() {
        *storage = 1;
    }

    bpf_tail_call(
        skb,
        core::ptr::addr_of_mut!(prog_array) as *mut core::ffi::c_void,
        0,
    );
    return 1;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
