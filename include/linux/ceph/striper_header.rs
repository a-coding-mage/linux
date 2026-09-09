/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header linux/ceph/striper.h.
// The declarations below depend on the corresponding Linux list/types definitions.

pub struct ceph_file_layout;

extern "C" {
    pub fn ceph_calc_file_object_mapping(
        l: *mut ceph_file_layout,
        off: u64,
        len: u64,
        objno: *mut u64,
        objoff: *mut u64,
        xlen: *mut u32,
    );
}

#[repr(C)]
pub struct ceph_object_extent {
    pub oe_item: list_head,
    pub oe_objno: u64,
    pub oe_off: u64,
    pub oe_len: u64,
}

extern "C" {
    pub fn INIT_LIST_HEAD(list: *mut list_head);
}

pub unsafe fn ceph_object_extent_init(ex: *mut ceph_object_extent) {
    INIT_LIST_HEAD(&mut (*ex).oe_item);
}

/*
 * Called for each mapped stripe unit.
 *
 * @bytes: number of bytes mapped, i.e. the minimum of the full length
 *         requested (file extent length) or the remainder of the stripe
 *         unit within an object
 */
pub type ceph_object_extent_fn_t =
    Option<unsafe extern "C" fn(ex: *mut ceph_object_extent, bytes: u32, arg: *mut core::ffi::c_void)>;

extern "C" {
    pub fn ceph_file_to_extents(
        l: *mut ceph_file_layout,
        off: u64,
        len: u64,
        object_extents: *mut list_head,
        alloc_fn: Option<unsafe extern "C" fn(arg: *mut core::ffi::c_void) -> *mut ceph_object_extent>,
        alloc_arg: *mut core::ffi::c_void,
        action_fn: ceph_object_extent_fn_t,
        action_arg: *mut core::ffi::c_void,
    ) -> i32;

    pub fn ceph_iterate_extents(
        l: *mut ceph_file_layout,
        off: u64,
        len: u64,
        object_extents: *mut list_head,
        action_fn: ceph_object_extent_fn_t,
        action_arg: *mut core::ffi::c_void,
    ) -> i32;
}

#[repr(C)]
pub struct ceph_file_extent {
    pub fe_off: u64,
    pub fe_len: u64,
}

pub unsafe fn ceph_file_extents_bytes(
    file_extents: *mut ceph_file_extent,
    num_file_extents: u32,
) -> u64 {
    let mut bytes: u64 = 0;
    let mut i: u32 = 0;

    while i < num_file_extents {
        bytes = bytes.wrapping_add((*file_extents.add(i as usize)).fe_len);
        i = i.wrapping_add(1);
    }

    bytes
}

extern "C" {
    pub fn ceph_extent_to_file(
        l: *mut ceph_file_layout,
        objno: u64,
        objoff: u64,
        objlen: u64,
        file_extents: *mut *mut ceph_file_extent,
        num_file_extents: *mut u32,
    ) -> i32;

    pub fn ceph_get_num_objects(l: *mut ceph_file_layout, size: u64) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
