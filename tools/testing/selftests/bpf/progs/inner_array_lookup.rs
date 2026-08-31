// SPDX-License-Identifier: GPL-2.0-only

// Translated from C includes:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_MAP_TYPE_HASH_OF_MAPS: u32 = 12;

#[repr(C)]
pub struct inner_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key: core::marker::PhantomData<i32>,
    pub value: core::marker::PhantomData<i32>,
}

#[used]
#[link_section = ".maps"]
pub static mut inner_map1: inner_map = inner_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 5,
    key: core::marker::PhantomData,
    value: core::marker::PhantomData,
};

#[repr(C)]
pub struct outer_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key: core::marker::PhantomData<i32>,
    pub values: [*mut inner_map; 3],
}

#[used]
#[link_section = ".maps"]
pub static mut outer_map1: outer_map = outer_map {
    type_: BPF_MAP_TYPE_HASH_OF_MAPS,
    max_entries: 3,
    key: core::marker::PhantomData,
    values: [
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        core::ptr::addr_of_mut!(inner_map1),
    ],
};

extern "C" {
    pub fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handle__sys_enter(ctx: *mut core::ffi::c_void) -> i32 {
    let mut outer_key: i32 = 2;
    let mut inner_key: i32 = 3;
    let mut val: *mut i32;
    let mut map: *mut core::ffi::c_void;

    map = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(outer_map1) as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(outer_key) as *const core::ffi::c_void,
    );
    if map.is_null() {
        return 1;
    }

    val = bpf_map_lookup_elem(
        map,
        core::ptr::addr_of_mut!(inner_key) as *const core::ffi::c_void,
    ) as *mut i32;
    if val.is_null() {
        return 1;
    }

    if *val == 1 {
        *val = 2;
    }

    0
}

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
