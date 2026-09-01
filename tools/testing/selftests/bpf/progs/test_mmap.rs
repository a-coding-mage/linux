// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_F_RDONLY_PROG: u32 = 1 << 7;
const BPF_F_MMAPABLE: u32 = 1 << 10;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct RdonlyMapDef {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut rdonly_map: RdonlyMapDef = RdonlyMapDef {
    type_: BPF_MAP_TYPE_ARRAY,
    map_flags: BPF_F_MMAPABLE | BPF_F_RDONLY_PROG,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<i8>() as u32,
};

#[repr(C)]
pub struct DataMapDef {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut data_map: DataMapDef = DataMapDef {
    type_: BPF_MAP_TYPE_ARRAY,
    map_flags: BPF_F_MMAPABLE,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
};

#[no_mangle]
pub static mut in_val: u64 = 0;
#[no_mangle]
pub static mut out_val: u64 = 0;

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_mmap(ctx: *mut core::ffi::c_void) -> i32 {
    let zero: i32 = 0;
    let one: i32 = 1;
    let two: i32 = 2;
    let far: i32 = 1500;
    let mut val: u64;
    let mut p: *mut u64;

    let _ = ctx;

    out_val = in_val;

    /* data_map[2] = in_val; */
    bpf_map_update_elem(
        &raw mut data_map as *mut _ as *mut core::ffi::c_void,
        &two as *const _ as *const core::ffi::c_void,
        &raw const in_val as *const _ as *const core::ffi::c_void,
        0,
    );

    /* data_map[1] = data_map[0] * 2; */
    p = bpf_map_lookup_elem(
        &raw mut data_map as *mut _ as *mut core::ffi::c_void,
        &zero as *const _ as *const core::ffi::c_void,
    ) as *mut u64;
    if !p.is_null() {
        val = (*p).wrapping_mul(2);
        bpf_map_update_elem(
            &raw mut data_map as *mut _ as *mut core::ffi::c_void,
            &one as *const _ as *const core::ffi::c_void,
            &val as *const _ as *const core::ffi::c_void,
            0,
        );
    }

    /* data_map[far] = in_val * 3; */
    val = in_val.wrapping_mul(3);
    bpf_map_update_elem(
        &raw mut data_map as *mut _ as *mut core::ffi::c_void,
        &far as *const _ as *const core::ffi::c_void,
        &val as *const _ as *const core::ffi::c_void,
        0,
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
