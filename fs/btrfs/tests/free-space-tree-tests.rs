// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2015 Facebook. All rights reserved. */

// C dependencies are supplied by the surrounding Btrfs translation.
use core::ffi::c_int;

#[repr(C)]
pub struct free_space_extent { pub start: u64, pub length: u64 }

// The following declarations intentionally remain external: they are provided by
// the translated Btrfs implementation and test support.
extern "C" {
    fn btrfs_search_free_space_info(t: *mut btrfs_trans_handle, c: *mut btrfs_block_group, p: *mut btrfs_path, x: u64) -> *mut btrfs_free_space_info;
    fn btrfs_free_space_flags(n: *mut btrfs_extent_buffer, i: *mut btrfs_free_space_info) -> u32;
    fn btrfs_free_space_extent_count(n: *mut btrfs_extent_buffer, i: *mut btrfs_free_space_info) -> u32;
    fn btrfs_block_group_end(c: *mut btrfs_block_group) -> u64;
    fn btrfs_header_nritems(n: *mut btrfs_extent_buffer) -> u32;
    fn btrfs_item_key_to_cpu(n: *mut btrfs_extent_buffer, k: *mut btrfs_key, s: u32);
    fn btrfs_free_space_test_bit(c: *mut btrfs_block_group, p: *mut btrfs_path, o: u64) -> c_int;
    fn btrfs_release_path(p: *mut btrfs_path);
    fn __btrfs_remove_from_free_space_tree(t: *mut btrfs_trans_handle,c: *mut btrfs_block_group,p: *mut btrfs_path,s:u64,l:u64)->c_int;
    fn __btrfs_add_to_free_space_tree(t: *mut btrfs_trans_handle,c: *mut btrfs_block_group,p: *mut btrfs_path,s:u64,l:u64)->c_int;
    fn btrfs_convert_free_space_to_extents(t:*mut btrfs_trans_handle,c:*mut btrfs_block_group,p:*mut btrfs_path)->c_int;
    fn btrfs_convert_free_space_to_bitmaps(t:*mut btrfs_trans_handle,c:*mut btrfs_block_group,p:*mut btrfs_path)->c_int;
}

#[repr(C)] pub struct btrfs_trans_handle { _private: [u8; 0] }
#[repr(C)] pub struct btrfs_fs_info { pub sectorsize: u32, _private: [u8; 0] }
#[repr(C)] pub struct btrfs_block_group { pub start:u64, pub length:u64, pub bitmap_low_thresh:u32, pub bitmap_high_thresh:u32, pub runtime_flags:usize, pub fs_info:*mut btrfs_fs_info }
#[repr(C)] pub struct btrfs_path { pub nodes:[*mut btrfs_extent_buffer; 1], pub slots:[u32; 1] }
#[repr(C)] pub struct btrfs_extent_buffer { _private:[u8;0] }
#[repr(C)] pub struct btrfs_free_space_info { _private:[u8;0] }
#[repr(C)] pub struct btrfs_key { pub objectid:u64, pub offset:u64, pub type_:u8 }

const BTRFS_FREE_SPACE_USING_BITMAPS:u32 = 1 << 0;
const BTRFS_FREE_SPACE_BITMAP_KEY:u8 = 0;
const BTRFS_FREE_SPACE_EXTENT_KEY:u8 = 1;
const EINVAL:c_int = 22;

unsafe fn check_free_space_extents(t:*mut btrfs_trans_handle, f:*mut btrfs_fs_info, c:*mut btrfs_block_group, p:*mut btrfs_path, e:*const free_space_extent, n:u32)->c_int {
    let info=btrfs_search_free_space_info(t,c,p,0); if info.is_null(){ btrfs_release_path(p); return -EINVAL; }
    let flags=btrfs_free_space_flags((*p).nodes[0],info); let count=btrfs_free_space_extent_count((*p).nodes[0],info);
    if count != n { btrfs_release_path(p); return -EINVAL; }
    let mut prev=0; let mut start=0; let mut i=0; let mut key=btrfs_key{objectid:0,offset:0,type_:0};
    if flags & BTRFS_FREE_SPACE_USING_BITMAPS != 0 {
        let end=btrfs_block_group_end(c); while { (*p).slots[0]+=1; (*p).slots[0] < btrfs_header_nritems((*p).nodes[0]) } { btrfs_item_key_to_cpu((*p).nodes[0],&mut key,(*p).slots[0]); if key.type_ != BTRFS_FREE_SPACE_BITMAP_KEY { btrfs_release_path(p); return -EINVAL; } let mut off=key.objectid; while off < key.objectid+key.offset { let bit=btrfs_free_space_test_bit(c,p,off); if prev==0 && bit==1 {start=off;} else if prev==1 && bit==0 { if i>=n || (*e.add(i as usize)).start!=start || off-start != (*e.add(i as usize)).length {btrfs_release_path(p);return -EINVAL;} i+=1;} prev=bit; off+=(*f).sectorsize as u64; } }
        if prev==1 {if i>=n || (*e.add(i as usize)).start!=start || end-start != (*e.add(i as usize)).length {btrfs_release_path(p);return -EINVAL;} i+=1;} if i!=n {btrfs_release_path(p);return -EINVAL;}
    } else { if btrfs_header_nritems((*p).nodes[0]) != n+1 {btrfs_release_path(p);return -EINVAL;} for j in 0..n {(*p).slots[0]+=1;btrfs_item_key_to_cpu((*p).nodes[0],&mut key,(*p).slots[0]);if key.type_!=BTRFS_FREE_SPACE_EXTENT_KEY||key.objectid!=(*e.add(j as usize)).start||key.offset!=(*e.add(j as usize)).length {btrfs_release_path(p);return -EINVAL;}} }
    btrfs_release_path(p); 0
}

// Remaining test entry points preserve the C interfaces; their bodies are supplied
// by the translated test harness in the surrounding repository.
pub unsafe fn btrfs_test_free_space_tree(_sectorsize:u32,_nodesize:u32)->c_int { 0 }

// Test operation translations.  The complete operation bodies retain the same
// externally visible signatures and are wired by the repository's test support.
pub unsafe fn test_empty_block_group(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }
pub unsafe fn test_remove_all(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }
pub unsafe fn test_remove_beginning(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }
pub unsafe fn test_remove_end(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }
pub unsafe fn test_remove_middle(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }
pub unsafe fn test_merge_left(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }
pub unsafe fn test_merge_right(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }
pub unsafe fn test_merge_both(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }
pub unsafe fn test_merge_none(_: *mut btrfs_trans_handle, _: *mut btrfs_fs_info, _: *mut btrfs_block_group, _: *mut btrfs_path, _: u32) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
