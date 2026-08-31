/* SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause) */

// C dependencies translated from:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_HASH: __u32 = 1;
pub const BPF_MAP_TYPE_ARRAY: __u32 = 2;

#[repr(C)]
pub struct HtabDef {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

#[repr(C)]
pub struct ArrayDef {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut htab: HtabDef = HtabDef {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<core::ffi::c_long>() as __u32,
    max_entries: 2,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut array: ArrayDef = ArrayDef {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<core::ffi::c_long>() as __u32,
    max_entries: 2,
};

/* Sample program which should always load for testing control paths. */
#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn func() -> core::ffi::c_int {
    let key64: __u64 = 0;
    let key: __u32 = 0;
    let mut value: *mut core::ffi::c_long;

    value = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(htab).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
    )
    .cast::<core::ffi::c_long>();
    if value.is_null() {
        return 1;
    }
    value = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(array).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(key64).cast::<core::ffi::c_void>(),
    )
    .cast::<core::ffi::c_long>();
    if value.is_null() {
        return 1;
    }

    return 0;
}
