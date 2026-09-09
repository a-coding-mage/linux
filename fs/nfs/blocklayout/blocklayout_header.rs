/*
 *  linux/fs/nfs/blocklayout/blocklayout.h
 *
 *  Module for the NFSv4.1 pNFS block layout driver.
 *
 *  Copyright (c) 2006 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Andy Adamson <andros@citi.umich.edu>
 *  Fred Isaman <iisaman@umich.edu>
 *
 * Permission and warranty notice retained from the original header.
 */

// C dependencies: linux/device-mapper.h, linux/nfs_fs.h,
// linux/sunrpc/rpc_pipe_fs.h, ../nfs4_fs.h, ../pnfs.h, and ../netns.h.

pub const PAGE_CACHE_SECTORS: usize = PAGE_SIZE >> SECTOR_SHIFT;
pub const PAGE_CACHE_SECTOR_SHIFT: usize = PAGE_SHIFT - SECTOR_SHIFT;
pub const SECTOR_SIZE: usize = 1usize << SECTOR_SHIFT;

pub struct pnfs_block_dev;

pub const PNFS_BLOCK_MAX_UUIDS: usize = 4;
pub const PNFS_BLOCK_MAX_DEVICES: usize = 64;
pub const PNFS_BLOCK_UUID_LEN: usize = 128;

#[repr(C)]
pub struct pnfs_block_volume_simple_sig {
    pub offset: u64,
    pub sig_len: u32,
    pub sig: [u8; PNFS_BLOCK_UUID_LEN],
}

#[repr(C)]
pub struct pnfs_block_volume_simple {
    pub len: ::std::os::raw::c_int,
    pub nr_sigs: ::std::os::raw::c_int,
    pub sigs: [pnfs_block_volume_simple_sig; PNFS_BLOCK_MAX_UUIDS],
}

#[repr(C)]
pub struct pnfs_block_volume_slice { pub start: u64, pub len: u64, pub volume: u32 }

#[repr(C)]
pub struct pnfs_block_volume_concat {
    pub volumes_count: u32,
    pub volumes: [u32; PNFS_BLOCK_MAX_DEVICES],
}

#[repr(C)]
pub struct pnfs_block_volume_stripe {
    pub chunk_size: u64,
    pub volumes_count: u32,
    pub volumes: [u32; PNFS_BLOCK_MAX_DEVICES],
}

#[repr(C)]
pub struct pnfs_block_volume_scsi {
    pub code_set: scsi_code_set,
    pub designator_type: scsi_designator_type,
    pub designator_len: ::std::os::raw::c_int,
    pub designator: [u8; 256],
    pub pr_key: u64,
}

#[repr(C)]
pub union pnfs_block_volume_data {
    pub simple: ::std::mem::ManuallyDrop<pnfs_block_volume_simple>,
    pub slice: ::std::mem::ManuallyDrop<pnfs_block_volume_slice>,
    pub concat: ::std::mem::ManuallyDrop<pnfs_block_volume_concat>,
    pub stripe: ::std::mem::ManuallyDrop<pnfs_block_volume_stripe>,
    pub scsi: ::std::mem::ManuallyDrop<pnfs_block_volume_scsi>,
}

#[repr(C)]
pub struct pnfs_block_volume {
    pub type_: pnfs_block_volume_type,
    pub data: pnfs_block_volume_data,
}

#[repr(C)]
pub struct pnfs_block_dev_map {
    pub start: u64,
    pub len: u64,
    pub disk_offset: u64,
    pub bdev: *mut block_device,
}

#[repr(C)]
pub struct pnfs_block_dev {
    pub node: nfs4_deviceid_node,
    pub start: u64,
    pub len: u64,
    pub type_: pnfs_block_volume_type,
    pub nr_children: u32,
    pub children: *mut pnfs_block_dev,
    pub chunk_size: u64,
    pub bdev_file: *mut file,
    pub disk_offset: u64,
    pub flags: ::std::os::raw::c_ulong,
    pub pr_key: u64,
    pub map: Option<unsafe extern "C" fn(*mut pnfs_block_dev, u64, *mut pnfs_block_dev_map) -> bool>,
}

pub const PNFS_BDEV_REGISTERED: usize = 0;

#[repr(C)]
pub union pnfs_block_extent_link {
    pub be_node: rb_node,
    pub be_list: list_head,
}

#[repr(C)]
pub struct pnfs_block_extent {
    pub link: pnfs_block_extent_link,
    pub be_device: *mut nfs4_deviceid_node,
    pub be_f_offset: sector_t,
    pub be_length: sector_t,
    pub be_v_offset: sector_t,
    pub be_state: pnfs_block_extent_state,
    pub be_tag: ::std::os::raw::c_uint,
}

pub const EXTENT_WRITTEN: ::std::os::raw::c_uint = 1;
pub const EXTENT_COMMITTING: ::std::os::raw::c_uint = 2;

#[repr(C)]
pub struct pnfs_block_layout {
    pub bl_layout: pnfs_layout_hdr,
    pub bl_ext_rw: rb_root,
    pub bl_ext_ro: rb_root,
    pub bl_ext_lock: spinlock_t,
    pub bl_scsi_layout: bool,
    pub bl_lwb: u64,
}

#[inline]
pub unsafe fn BLK_LO2EXT(lo: *mut pnfs_layout_hdr) -> *mut pnfs_block_layout {
    (lo as *mut u8).sub(::std::mem::offset_of!(pnfs_block_layout, bl_layout)) as *mut pnfs_block_layout
}

#[inline]
pub unsafe fn BLK_LSEG2EXT(lseg: *mut pnfs_layout_segment) -> *mut pnfs_block_layout {
    BLK_LO2EXT((*lseg).pls_layout)
}

#[repr(C)]
pub struct bl_pipe_msg { pub msg: rpc_pipe_msg, pub bl_wq: *mut wait_queue_head_t }

#[repr(C)]
pub struct bl_msg_hdr { pub type_: u8, pub totallen: u16 }

pub const BL_DEVICE_UMOUNT: u32 = 0x0;
pub const BL_DEVICE_MOUNT: u32 = 0x1;
pub const BL_DEVICE_REQUEST_INIT: u32 = 0x0;
pub const BL_DEVICE_REQUEST_PROC: u32 = 0x1;
pub const BL_DEVICE_REQUEST_ERR: u32 = 0x2;

extern "C" {
    pub fn bl_register_dev(d: *mut pnfs_block_dev) -> bool;
    pub fn bl_alloc_deviceid_node(server: *mut nfs_server, pdev: *mut pnfs_device, gfp_mask: gfp_t) -> *mut nfs4_deviceid_node;
    pub fn bl_free_deviceid_node(d: *mut nfs4_deviceid_node);
    pub fn ext_tree_insert(bl: *mut pnfs_block_layout, new_: *mut pnfs_block_extent) -> ::std::os::raw::c_int;
    pub fn ext_tree_remove(bl: *mut pnfs_block_layout, rw: bool, start: sector_t, end: sector_t) -> ::std::os::raw::c_int;
    pub fn ext_tree_mark_written(bl: *mut pnfs_block_layout, start: sector_t, len: sector_t, lwb: u64) -> ::std::os::raw::c_int;
    pub fn ext_tree_lookup(bl: *mut pnfs_block_layout, isect: sector_t, ret: *mut pnfs_block_extent, rw: bool) -> bool;
    pub fn ext_tree_prepare_commit(arg: *mut nfs4_layoutcommit_args) -> ::std::os::raw::c_int;
    pub fn ext_tree_mark_committed(arg: *mut nfs4_layoutcommit_args, status: ::std::os::raw::c_int);
    pub fn bl_resolve_deviceid(server: *mut nfs_server, b: *mut pnfs_block_volume, gfp_mask: gfp_t) -> dev_t;
    pub fn bl_init_pipefs() -> ::std::os::raw::c_int;
    pub fn bl_cleanup_pipefs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
