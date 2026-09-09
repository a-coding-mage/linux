/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated Linux interval-tree declarations.

#[repr(C)]
pub struct vhost_iotlb_map {
    pub rb: rb_node,
    pub link: list_head,
    pub start: u64,
    pub last: u64,
    pub size: u64,
    pub addr: u64,
    pub perm: u32,
    pub flags_padding: u32,
    pub __subtree_last: u64,
    pub opaque: *mut core::ffi::c_void,
}

pub const VHOST_MAP_RO: u32 = 0x1;
pub const VHOST_MAP_WO: u32 = 0x2;
pub const VHOST_MAP_RW: u32 = 0x3;

pub const VHOST_IOTLB_FLAG_RETIRE: u32 = 0x1;

#[repr(C)]
pub struct vhost_iotlb {
    pub root: rb_root_cached,
    pub list: list_head,
    pub limit: core::ffi::c_uint,
    pub nmaps: core::ffi::c_uint,
    pub flags: core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn vhost_iotlb_add_range_ctx(
        iotlb: *mut vhost_iotlb,
        start: u64,
        last: u64,
        addr: u64,
        perm: core::ffi::c_uint,
        opaque: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn vhost_iotlb_add_range(
        iotlb: *mut vhost_iotlb,
        start: u64,
        last: u64,
        addr: u64,
        perm: core::ffi::c_uint,
    ) -> core::ffi::c_int;

    pub fn vhost_iotlb_del_range(iotlb: *mut vhost_iotlb, start: u64, last: u64);

    pub fn vhost_iotlb_init(
        iotlb: *mut vhost_iotlb,
        limit: core::ffi::c_uint,
        flags: core::ffi::c_uint,
    );

    pub fn vhost_iotlb_alloc(
        limit: core::ffi::c_uint,
        flags: core::ffi::c_uint,
    ) -> *mut vhost_iotlb;

    pub fn vhost_iotlb_free(iotlb: *mut vhost_iotlb);
    pub fn vhost_iotlb_reset(iotlb: *mut vhost_iotlb);

    pub fn vhost_iotlb_itree_first(
        iotlb: *mut vhost_iotlb,
        start: u64,
        last: u64,
    ) -> *mut vhost_iotlb_map;

    pub fn vhost_iotlb_itree_next(
        map: *mut vhost_iotlb_map,
        start: u64,
        last: u64,
    ) -> *mut vhost_iotlb_map;

    pub fn vhost_iotlb_map_free(iotlb: *mut vhost_iotlb, map: *mut vhost_iotlb_map);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
