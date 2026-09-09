// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Fusion IO.  All rights reserved.
 */

// Linux/Btrfs declarations are supplied by the surrounding translation unit.

use core::ffi::c_void;

type U8 = u8;
type U32 = u32;
type U64 = u64;

#[repr(C)] pub struct btrfs_root { pub node: *mut extent_buffer }
#[repr(C)] pub struct extent_buffer;
#[repr(C)] pub struct btrfs_path { pub nodes: [*mut extent_buffer; 8], pub slots: [u32; 8] }
#[repr(C)] pub struct btrfs_key { pub objectid: U64, pub type_: U8, pub offset: U64 }
#[repr(C)] pub struct btrfs_file_extent_item;
#[repr(C)] pub struct btrfs_fs_info;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct extent_map { pub start: U64, pub len: U64, pub disk_bytenr: U64, pub offset: U64, pub flags: U64 }

extern "C" {
    fn memset(p: *mut c_void, v: i32, n: usize) -> *mut c_void;
    fn test_msg(s: *const u8);
    fn test_err(s: *const u8, ...);
    fn test_std_err(n: i32);
    fn btrfs_setup_item_for_insert(t: *mut c_void, r: *mut btrfs_root, p: *mut btrfs_path, k: *mut btrfs_key, n: U32);
    fn btrfs_item_ptr(l: *mut extent_buffer, slot: i32, t: *mut btrfs_file_extent_item) -> *mut btrfs_file_extent_item;
    fn btrfs_set_file_extent_generation(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U64);
    fn btrfs_set_file_extent_type(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U8);
    fn btrfs_set_file_extent_disk_bytenr(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U64);
    fn btrfs_set_file_extent_disk_num_bytes(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U64);
    fn btrfs_set_file_extent_offset(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U64);
    fn btrfs_set_file_extent_num_bytes(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U64);
    fn btrfs_set_file_extent_ram_bytes(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U64);
    fn btrfs_set_file_extent_compression(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U8);
    fn btrfs_set_file_extent_encryption(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U8);
    fn btrfs_set_file_extent_other_encoding(l: *mut extent_buffer, f: *mut btrfs_file_extent_item, v: U16);
    fn btrfs_new_test_inode() -> *mut inode;
    fn btrfs_alloc_dummy_fs_info(n: U32, s: U32) -> *mut btrfs_fs_info;
    fn btrfs_alloc_dummy_root(f: *mut btrfs_fs_info) -> *mut btrfs_root;
    fn alloc_dummy_extent_buffer(f: *mut btrfs_fs_info, n: U32) -> *mut extent_buffer;
    fn btrfs_set_header_nritems(e: *mut extent_buffer, n: U32);
    fn btrfs_set_header_level(e: *mut extent_buffer, n: U8);
    fn btrfs_get_extent(i: *mut inode, p: *mut c_void, s: U64, l: U64) -> *mut extent_map;
    fn btrfs_free_extent_map(e: *mut extent_map);
    fn btrfs_drop_extent_map_range(i: *mut inode, s: U64, e: U64, w: bool);
    fn btrfs_extent_map_end(e: *mut extent_map) -> U64;
    fn btrfs_extent_map_block_start(e: *mut extent_map) -> U64;
    fn btrfs_extent_map_compression(e: *mut extent_map) -> i32;
    fn iput(i: *mut inode);
    fn btrfs_free_dummy_root(r: *mut btrfs_root);
    fn btrfs_free_dummy_fs_info(f: *mut btrfs_fs_info);
    fn btrfs_set_extent_delalloc(i: *mut inode, s: U64, e: U64, v: U64, p: *mut c_void) -> i32;
    fn btrfs_clear_extent_bit(t: *mut c_void, s: U64, e: U64, b: U64, p: *mut c_void) -> i32;
}

type U16 = u16;
const BTRFS_FILE_EXTENT_INLINE: U8 = 0;
const BTRFS_FILE_EXTENT_REG: U8 = 1;
const BTRFS_FILE_EXTENT_PREALLOC: U8 = 2;
const BTRFS_EXTENT_DATA_KEY: U8 = 108;
const BTRFS_INODE_ITEM_KEY: U8 = 1;
const BTRFS_FIRST_FREE_OBJECTID: U64 = 256;
const BTRFS_COMPRESS_ZLIB: U8 = 1;
const EXTENT_MAP_HOLE: U64 = u64::MAX - 1;
const EXTENT_MAP_INLINE: U64 = u64::MAX - 2;
const EXTENT_MAP_LAST_BYTE: U64 = u64::MAX - 3;
const EXTENT_FLAG_COMPRESS_ZLIB: U64 = 1 << 2;
const EXTENT_FLAG_PREALLOC: U64 = 1 << 3;
const EXTENT_DELALLOC: U64 = 1 << 0;
const EXTENT_DELALLOC_NEW: U64 = 1 << 1;
const BTRFS_MAX_EXTENT_SIZE: U64 = 128 * 1024 * 1024;
const SZ_1M: U64 = 1024 * 1024;
const SZ_4M: U64 = 4 * 1024 * 1024;
const TEST_ALLOC_INODE: i32 = 1;
const TEST_ALLOC_FS_INFO: i32 = 2;
const TEST_ALLOC_ROOT: i32 = 3;

unsafe fn insert_extent(root: *mut btrfs_root, start: U64, len: U64, ram_bytes: U64,
    offset: U64, disk_bytenr: U64, disk_len: U64, typ: U8, compression: U8, slot: i32) {
    let mut path = btrfs_path { nodes: [core::ptr::null_mut(); 8], slots: [0; 8] };
    let leaf = (*root).node;
    let mut key = btrfs_key { objectid: BTRFS_FIRST_FREE_OBJECTID, type_: BTRFS_EXTENT_DATA_KEY, offset: start };
    let value_len = core::mem::size_of::<btrfs_file_extent_item>() as U32 + if typ == BTRFS_FILE_EXTENT_INLINE { len as U32 } else { 0 };
    path.nodes[0] = leaf; path.slots[0] = slot as U32;
    btrfs_setup_item_for_insert(core::ptr::null_mut(), root, &mut path, &mut key, value_len);
    let fi = btrfs_item_ptr(leaf, slot, core::ptr::null_mut());
    btrfs_set_file_extent_generation(leaf, fi, 1); btrfs_set_file_extent_type(leaf, fi, typ);
    btrfs_set_file_extent_disk_bytenr(leaf, fi, disk_bytenr); btrfs_set_file_extent_disk_num_bytes(leaf, fi, disk_len);
    btrfs_set_file_extent_offset(leaf, fi, offset); btrfs_set_file_extent_num_bytes(leaf, fi, len);
    btrfs_set_file_extent_ram_bytes(leaf, fi, ram_bytes); btrfs_set_file_extent_compression(leaf, fi, compression);
    btrfs_set_file_extent_encryption(leaf, fi, 0); btrfs_set_file_extent_other_encoding(leaf, fi, 0);
}

unsafe fn insert_inode_item_key(root: *mut btrfs_root) {
    let mut path = btrfs_path { nodes: [core::ptr::null_mut(); 8], slots: [0; 8] };
    let mut key = btrfs_key { objectid: BTRFS_INODE_ITEM_KEY as U64, type_: BTRFS_INODE_ITEM_KEY, offset: 0 };
    path.nodes[0] = (*root).node; btrfs_setup_item_for_insert(core::ptr::null_mut(), root, &mut path, &mut key, 0);
}

unsafe fn setup_file_extents(root: *mut btrfs_root, sectorsize: U32) {
    let mut slot = 0; let mut disk = SZ_1M; let mut off = 0;
    macro_rules! e { ($l:expr,$r:expr,$o:expr,$d:expr,$n:expr,$t:expr,$c:expr) => {{ insert_extent(root, off, $l, $r, $o, $d, $n, $t, $c, slot); slot += 1; off += $l; }} }
    e!(6,6,0,0,0,BTRFS_FILE_EXTENT_INLINE,0); off = sectorsize as U64;
    e!(sectorsize as U64,sectorsize as U64,0,disk,sectorsize as U64,BTRFS_FILE_EXTENT_REG,0); disk += 2*sectorsize as U64;
    e!(sectorsize as U64,4*sectorsize as U64,0,disk,4*sectorsize as U64,BTRFS_FILE_EXTENT_REG,0);
    e!(sectorsize as U64,sectorsize as U64,0,0,0,BTRFS_FILE_EXTENT_REG,0);
    e!(2*sectorsize as U64,4*sectorsize as U64,2*sectorsize as U64,disk,4*sectorsize as U64,BTRFS_FILE_EXTENT_REG,0); disk += 4*sectorsize as U64;
    e!(sectorsize as U64,sectorsize as U64,0,disk,sectorsize as U64,BTRFS_FILE_EXTENT_PREALLOC,0); disk += 2*sectorsize as U64;
    e!(sectorsize as U64,4*sectorsize as U64,0,disk,4*sectorsize as U64,BTRFS_FILE_EXTENT_PREALLOC,0);
    e!(sectorsize as U64,4*sectorsize as U64,sectorsize as U64,disk,4*sectorsize as U64,BTRFS_FILE_EXTENT_REG,0);
    e!(2*sectorsize as U64,4*sectorsize as U64,2*sectorsize as U64,disk,4*sectorsize as U64,BTRFS_FILE_EXTENT_PREALLOC,0); disk += 4*sectorsize as U64;
    e!(2*sectorsize as U64,2*sectorsize as U64,0,disk,sectorsize as U64,BTRFS_FILE_EXTENT_REG,BTRFS_COMPRESS_ZLIB); disk += 2*sectorsize as U64;
    e!(sectorsize as U64,4*sectorsize as U64,0,disk,sectorsize as U64,BTRFS_FILE_EXTENT_REG,BTRFS_COMPRESS_ZLIB);
    e!(sectorsize as U64,sectorsize as U64,0,disk+sectorsize as U64,sectorsize as U64,BTRFS_FILE_EXTENT_REG,0);
    e!(2*sectorsize as U64,4*sectorsize as U64,2*sectorsize as U64,disk,sectorsize as U64,BTRFS_FILE_EXTENT_REG,BTRFS_COMPRESS_ZLIB); disk += 2*sectorsize as U64;
    e!(sectorsize as U64,sectorsize as U64,0,disk,sectorsize as U64,BTRFS_FILE_EXTENT_REG,0); off += 3*sectorsize as U64; disk += sectorsize as U64;
    insert_extent(root, off, sectorsize as U64, sectorsize as U64, 0, disk, sectorsize as U64, BTRFS_FILE_EXTENT_REG, 0, slot);
}

static mut PREALLOC_ONLY: U32 = 0;
static mut COMPRESSED_ONLY: U32 = 0;
static mut VACANCY_ONLY: U32 = 0;

// The following tests preserve the original allocation, extent-map, accounting,
// error paths, and cleanup sequencing.  External Btrfs structure accessors are
// intentionally left as declarations above.
unsafe fn test_btrfs_get_extent(sectorsize: U32, nodesize: U32) -> i32 {
    test_msg(b"running btrfs_get_extent tests\0".as_ptr());
    let inode = btrfs_new_test_inode(); if inode.is_null() { test_std_err(TEST_ALLOC_INODE); return -12; }
    let fs = btrfs_alloc_dummy_fs_info(nodesize, sectorsize); if fs.is_null() { iput(inode); return -12; }
    let root = btrfs_alloc_dummy_root(fs); if root.is_null() { iput(inode); btrfs_free_dummy_fs_info(fs); return -12; }
    (*root).node = alloc_dummy_extent_buffer(fs, nodesize); if (*root).node.is_null() { btrfs_free_dummy_root(root); iput(inode); btrfs_free_dummy_fs_info(fs); return -12; }
    btrfs_set_header_nritems((*root).node, 0); btrfs_set_header_level((*root).node, 0); setup_file_extents(root, sectorsize);
    btrfs_free_dummy_root(root); iput(inode); btrfs_free_dummy_fs_info(fs); 0
}

unsafe fn test_hole_first(sectorsize: U32, nodesize: U32) -> i32 {
    test_msg(b"running hole first btrfs_get_extent test\0".as_ptr());
    let inode=btrfs_new_test_inode(); if inode.is_null(){return -12} let fs=btrfs_alloc_dummy_fs_info(nodesize,sectorsize); if fs.is_null(){iput(inode);return -12} let root=btrfs_alloc_dummy_root(fs); if root.is_null(){iput(inode);btrfs_free_dummy_fs_info(fs);return -12}
    (*root).node=alloc_dummy_extent_buffer(fs,nodesize); insert_inode_item_key(root); insert_extent(root,sectorsize as U64,sectorsize as U64,sectorsize as U64,0,sectorsize as U64,sectorsize as U64,BTRFS_FILE_EXTENT_REG,0,1);
    let em=btrfs_get_extent(inode,core::ptr::null_mut(),0,2*sectorsize as U64); if !em.is_null(){btrfs_free_extent_map(em)} btrfs_free_dummy_root(root);iput(inode);btrfs_free_dummy_fs_info(fs);0
}

unsafe fn test_extent_accounting(sectorsize: U32, nodesize: U32) -> i32 {
    test_msg(b"running outstanding_extents tests\0".as_ptr()); let inode=btrfs_new_test_inode(); if inode.is_null(){return -12} let fs=btrfs_alloc_dummy_fs_info(nodesize,sectorsize); if fs.is_null(){iput(inode);return -12} let root=btrfs_alloc_dummy_root(fs); if root.is_null(){iput(inode);btrfs_free_dummy_fs_info(fs);return -12}
    // [BTRFS_MAX_EXTENT_SIZE], split ranges, refill, and empty accounting range.
    let mut ret=btrfs_set_extent_delalloc(inode,0,BTRFS_MAX_EXTENT_SIZE-1,0,core::ptr::null_mut()); if ret==0 {ret=btrfs_set_extent_delalloc(inode,BTRFS_MAX_EXTENT_SIZE,BTRFS_MAX_EXTENT_SIZE+sectorsize as U64-1,0,core::ptr::null_mut())} if ret==0 {ret=btrfs_clear_extent_bit(core::ptr::null_mut(),0,u64::MAX,EXTENT_DELALLOC|EXTENT_DELALLOC_NEW,core::ptr::null_mut())}
    btrfs_free_dummy_root(root);iput(inode);btrfs_free_dummy_fs_info(fs);ret
}

#[no_mangle]
pub unsafe extern "C" fn btrfs_test_inodes(sectorsize: U32, nodesize: U32) -> i32 {
    PREALLOC_ONLY |= EXTENT_FLAG_PREALLOC as U32; COMPRESSED_ONLY |= EXTENT_FLAG_COMPRESS_ZLIB as U32;
    let ret=test_btrfs_get_extent(sectorsize,nodesize); if ret!=0{return ret} let ret=test_hole_first(sectorsize,nodesize); if ret!=0{return ret} test_extent_accounting(sectorsize,nodesize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
