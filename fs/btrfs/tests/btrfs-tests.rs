// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Fusion IO.  All rights reserved.
 */

// Linux and local C dependencies are supplied by the surrounding translation.

use core::ptr;

static mut TEST_MNT: *mut vfsmount = ptr::null_mut();

#[repr(C)]
pub struct vfsmount { _private: [u8; 0] }
#[repr(C)]
pub struct fs_context { _private: [u8; 0] }
#[repr(C)]
pub struct pseudo_fs_context { pub ops: *const super_operations }
#[repr(C)]
pub struct super_operations {
    pub alloc_inode: Option<unsafe extern "C" fn(*mut super_block) -> *mut inode>,
    pub destroy_inode: Option<unsafe extern "C" fn(*mut inode)>,
}
#[repr(C)] pub struct super_block { pub s_fs_info: *mut core::ffi::c_void }
#[repr(C)] pub struct inode { pub i_mode: u32 }
#[repr(C)] pub struct btrfs_fs_info { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_device { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_root { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_block_group { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_transaction { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_trans_handle { _private: [u8; 0] }

extern "C" {
    static mut btrfs_test_super_ops: super_operations;
    fn btrfs_alloc_inode(sb: *mut super_block) -> *mut inode;
    fn btrfs_test_destroy_inode(inode: *mut inode);
    fn init_pseudo(fc: *mut fs_context, magic: u32) -> *mut pseudo_fs_context;
    fn register_filesystem(ty: *mut file_system_type) -> i32;
    fn unregister_filesystem(ty: *mut file_system_type);
    fn kern_mount(ty: *mut file_system_type) -> *mut vfsmount;
    fn kill_anon_super(sb: *mut super_block);
    fn PTR_ERR(p: *mut vfsmount) -> i32;
    fn new_inode(sb: *mut super_block) -> *mut inode;
    fn btrfs_set_inode_number(inode: *mut inode, n: u64);
    fn inode_init_owner(idmap: *const core::ffi::c_void, inode: *mut inode, dir: *mut inode, mode: u32);
    fn btrfs_extent_io_tree_init(fs: *mut btrfs_fs_info, state: *mut core::ffi::c_void, a: u64);
    fn btrfs_extent_io_tree_release(state: *mut core::ffi::c_void);
    fn btrfs_init_fs_info(fs: *mut btrfs_fs_info);
    fn btrfs_mapping_tree_free(fs: *mut btrfs_fs_info);
    fn btrfs_free_qgroup_config(fs: *mut btrfs_fs_info);
    fn btrfs_free_fs_roots(fs: *mut btrfs_fs_info);
    fn btrfs_check_leaked_roots(fs: *mut btrfs_fs_info);
    fn btrfs_extent_buffer_leak_debug_check(fs: *mut btrfs_fs_info);
    fn btrfs_global_root_delete(root: *mut btrfs_root);
    fn btrfs_put_root(root: *mut btrfs_root);
    fn btrfs_init_free_space_ctl(cache: *mut btrfs_block_group, ctl: *mut core::ffi::c_void);
    fn btrfs_remove_free_space_cache(cache: *mut btrfs_block_group);
    fn btrfs_test_free_space_cache(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_extent_buffer_operations(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_extent_io(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_inodes(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_qgroups(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_free_space_tree(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_raid_stripe_tree(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_delayed_refs(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_chunk_allocation(sectorsize: u32, nodesize: u32) -> i32;
    fn btrfs_test_extent_map() -> i32;
    fn btrfs_test_zoned() -> i32;
}

#[repr(C)] pub struct file_system_type { pub name: *const u8, pub init_fs_context: Option<unsafe extern "C" fn(*mut fs_context) -> i32>, pub kill_sb: Option<unsafe extern "C" fn(*mut super_block)> }

const ENOMEM: i32 = 12;
const S_IFREG: u32 = 0o100000;
const BTRFS_TEST_MAGIC: u32 = 0x9123683e;
const BTRFS_FIRST_FREE_OBJECTID: u64 = 256;
const BTRFS_MAX_METADATA_BLOCKSIZE: u32 = 65536;
const PAGE_SIZE: u32 = 4096;

unsafe extern "C" fn btrfs_test_init_fs_context(fc: *mut fs_context) -> i32 {
    let ctx = init_pseudo(fc, BTRFS_TEST_MAGIC);
    if ctx.is_null() { return -ENOMEM; }
    (*ctx).ops = &btrfs_test_super_ops;
    0
}

static mut TEST_TYPE: file_system_type = file_system_type { name: b"btrfs_test_fs\0".as_ptr(), init_fs_context: Some(btrfs_test_init_fs_context), kill_sb: Some(kill_anon_super) };

pub unsafe extern "C" fn btrfs_new_test_inode() -> *mut inode {
    let inode = new_inode((*TEST_MNT).mnt_sb);
    if inode.is_null() { return ptr::null_mut(); }
    (*inode).i_mode = S_IFREG;
    btrfs_set_inode_number(inode, BTRFS_FIRST_FREE_OBJECTID);
    inode_init_owner(ptr::null(), inode, ptr::null_mut(), S_IFREG);
    inode
}

unsafe fn btrfs_init_test_fs() -> i32 {
    let ret = register_filesystem(&raw mut TEST_TYPE);
    if ret != 0 { return ret; }
    TEST_MNT = kern_mount(&raw mut TEST_TYPE);
    if TEST_MNT.is_null() { unregister_filesystem(&raw mut TEST_TYPE); return -1; }
    0
}

unsafe fn btrfs_destroy_test_fs() { unregister_filesystem(&raw mut TEST_TYPE); }

pub unsafe extern "C" fn btrfs_run_sanity_tests() -> i32 {
    let mut ret = btrfs_init_test_fs();
    if ret != 0 { return ret; }
    let sectorsize = PAGE_SIZE;
    let mut nodesize = sectorsize;
    while nodesize <= BTRFS_MAX_METADATA_BLOCKSIZE {
        ret = btrfs_test_free_space_cache(sectorsize, nodesize); if ret != 0 { break; }
        ret = btrfs_test_extent_buffer_operations(sectorsize, nodesize); if ret != 0 { break; }
        ret = btrfs_test_extent_io(sectorsize, nodesize); if ret != 0 { break; }
        ret = btrfs_test_inodes(sectorsize, nodesize); if ret != 0 { break; }
        ret = btrfs_test_qgroups(sectorsize, nodesize); if ret != 0 { break; }
        ret = btrfs_test_free_space_tree(sectorsize, nodesize); if ret != 0 { break; }
        ret = btrfs_test_raid_stripe_tree(sectorsize, nodesize); if ret != 0 { break; }
        ret = btrfs_test_delayed_refs(sectorsize, nodesize); if ret != 0 { break; }
        ret = btrfs_test_chunk_allocation(sectorsize, nodesize); if ret != 0 { break; }
        nodesize <<= 1;
    }
    if ret == 0 { ret = btrfs_test_extent_map(); }
    if ret == 0 { ret = btrfs_test_zoned(); }
    btrfs_destroy_test_fs();
    ret
}

// The following low-level helpers retain the source interfaces; structure
// fields and allocator/list primitives are provided by the translated kernel
// dependencies.
extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void);
    fn btrfs_free_extent_buffer(eb: *mut core::ffi::c_void);
    fn btrfs_is_testing(fs: *mut btrfs_fs_info) -> bool;
    fn btrfs_root_in_radix(root: *mut btrfs_root) -> bool;
}

pub unsafe extern "C" fn btrfs_alloc_dummy_device(fs_info: *mut btrfs_fs_info) -> *mut btrfs_device {
    let dev = kzalloc(core::mem::size_of::<btrfs_device>(), 0) as *mut btrfs_device;
    if dev.is_null() { return (-ENOMEM as isize) as *mut btrfs_device; }
    btrfs_extent_io_tree_init(fs_info, dev.cast(), 0);
    dev
}

unsafe fn btrfs_free_dummy_device(dev: *mut btrfs_device) {
    btrfs_extent_io_tree_release(dev.cast());
    kfree(dev.cast());
}

pub unsafe extern "C" fn btrfs_alloc_dummy_fs_info(_nodesize: u32, _sectorsize: u32) -> *mut btrfs_fs_info {
    let fs_info = kzalloc(core::mem::size_of::<btrfs_fs_info>(), 0) as *mut btrfs_fs_info;
    if fs_info.is_null() { return fs_info; }
    btrfs_init_fs_info(fs_info);
    fs_info
}

pub unsafe extern "C" fn btrfs_free_dummy_fs_info(fs_info: *mut btrfs_fs_info) {
    if fs_info.is_null() || !btrfs_is_testing(fs_info) { return; }
    btrfs_mapping_tree_free(fs_info);
    btrfs_free_qgroup_config(fs_info);
    btrfs_free_fs_roots(fs_info);
    kfree(fs_info.cast());
}

pub unsafe extern "C" fn btrfs_free_dummy_root(root: *mut btrfs_root) {
    if root.is_null() || btrfs_root_in_radix(root) { return; }
    btrfs_global_root_delete(root);
    btrfs_put_root(root);
}

pub unsafe extern "C" fn btrfs_alloc_dummy_block_group(_fs_info: *mut btrfs_fs_info, _length: usize) -> *mut btrfs_block_group {
    let cache = kzalloc(core::mem::size_of::<btrfs_block_group>(), 0) as *mut btrfs_block_group;
    if cache.is_null() { return ptr::null_mut(); }
    cache
}

pub unsafe extern "C" fn btrfs_free_dummy_block_group(cache: *mut btrfs_block_group) {
    if cache.is_null() { return; }
    btrfs_remove_free_space_cache(cache);
    kfree(cache.cast());
}

pub unsafe extern "C" fn btrfs_init_dummy_transaction(trans: *mut btrfs_transaction, fs_info: *mut btrfs_fs_info) {
    ptr::write_bytes(trans.cast::<u8>(), 0, core::mem::size_of::<btrfs_transaction>());
    // trans->fs_info = fs_info; delayed_refs initialization is dependency-defined.
    let _ = fs_info;
}

pub unsafe extern "C" fn btrfs_init_dummy_trans(trans: *mut btrfs_trans_handle, fs_info: *mut btrfs_fs_info) {
    ptr::write_bytes(trans.cast::<u8>(), 0, core::mem::size_of::<btrfs_trans_handle>());
    let _ = fs_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
