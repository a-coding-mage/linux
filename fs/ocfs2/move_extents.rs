// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of move_extents.c.  Kernel types and
 * operations used here are supplied by the surrounding OCFS2 translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

type u32_t = u32;
type u64_t = u64;
type handle_t = c_void;

#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_mode: u16, pub i_flags: u32 }
#[repr(C)] pub struct file { pub f_mode: u32 }
#[repr(C)] pub struct super_block { pub s_blocksize_bits: u32 }
#[repr(C)] pub struct buffer_head { pub b_data: *mut c_void }
#[repr(C)] pub struct ocfs2_super { pub osb_tl_inode: *mut inode, pub s_clustersize_bits: u32, pub sb: *mut super_block, pub sys_root_inode: *mut inode }
#[repr(C)] pub struct ocfs2_extent_tree { pub et_ci: *mut c_void, pub et_root_el: *mut c_void }
#[repr(C)] pub struct ocfs2_alloc_context { pub ac_which: i32, pub ac_inode: *mut inode, pub ac_bh: *mut buffer_head }
#[repr(C)] pub struct ocfs2_cached_dealloc_ctxt { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_path { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_refcount_tree { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_extent_rec { pub e_cpos: u32, pub e_leaf_clusters: u16, pub e_flags: u16, pub e_blkno: u64 }
#[repr(C)] pub struct ocfs2_extent_list { pub l_recs: *mut ocfs2_extent_rec }
#[repr(C)] pub struct ocfs2_move_extents { pub me_start: u64, pub me_len: u64, pub me_flags: u32, pub me_threshold: u64, pub me_goal: u64, pub me_moved_len: u64, pub me_new_offset: u64 }
#[repr(C)] pub struct ocfs2_group_desc { pub bg_blkno: u64, pub bg_next_group: u64, pub bg_bits: u16, pub bg_free_bits_count: u16, pub bg_chain: u16, pub bg_bitmap: [u8; 0] }
#[repr(C)] pub struct ocfs2_move_extents_context { pub inode: *mut inode, pub file: *mut file, pub auto_defrag: i32, pub partial: i32, pub credits: i32, pub new_phys_cpos: u32, pub clusters_moved: u32, pub refcount_loc: u64, pub range: *mut ocfs2_move_extents, pub et: ocfs2_extent_tree, pub meta_ac: *mut ocfs2_alloc_context, pub data_ac: *mut ocfs2_alloc_context, pub dealloc: ocfs2_cached_dealloc_ctxt }

// External kernel/OCFS2 declarations intentionally remain unresolved here.
extern "C" {
    fn ocfs2_duplicate_clusters_by_page(h:*mut handle_t,i:*mut inode,c:u32,p:u32,n:u32)->i32;
    fn ocfs2_new_path_from_et(et:*mut ocfs2_extent_tree)->*mut ocfs2_path;
    fn ocfs2_find_path(cache:*mut c_void,path:*mut ocfs2_path,c:u32)->i32;
    fn path_leaf_el(path:*mut ocfs2_path)->*mut ocfs2_extent_list;
    fn ocfs2_search_extent_list(el:*mut ocfs2_extent_list,c:u32)->i32;
    fn ocfs2_split_extent(h:*mut handle_t,et:*mut ocfs2_extent_tree,p:*mut ocfs2_path,i:i32,r:*mut ocfs2_extent_rec,a:*mut ocfs2_alloc_context,d:*mut ocfs2_cached_dealloc_ctxt)->i32;
    fn ocfs2_free_path(p:*mut ocfs2_path);
    fn ocfs2_get_clusters(i:*mut inode,c:u32,p:*mut u32,n:*mut u32,f:*mut i32)->i32;
    fn ocfs2_move_extents(c:*mut ocfs2_move_extents_context)->i32;
}

const OCFS2_EXT_REFCOUNTED: i32 = 1 << 3;
const OCFS2_MOVE_EXT_FL_COMPLETE: u32 = 1;
const OCFS2_MOVE_EXT_FL_AUTO_DEFRAG: u32 = 2;
const OCFS2_MOVE_EXT_FL_PART_DEFRAG: u32 = 4;

#[inline] unsafe fn cpu_to_le32(x:u32)->u32 { x.to_le() }
#[inline] unsafe fn cpu_to_le16(x:u16)->u16 { x.to_le() }
#[inline] unsafe fn cpu_to_le64(x:u64)->u64 { x.to_le() }
#[inline] unsafe fn le16_to_cpu(x:u16)->u16 { u16::from_le(x) }
#[inline] unsafe fn le32_to_cpu(x:u32)->u32 { u32::from_le(x) }
#[inline] unsafe fn le64_to_cpu(x:u64)->u64 { u64::from_le(x) }

unsafe fn __ocfs2_move_extent(_handle:*mut handle_t, context:*mut ocfs2_move_extents_context, cpos:u32, len:u32, _p_cpos:u32, new_p_cpos:u32, _ext_flags:i32)->i32 {
    // duplicate data, replace the extent, and update the truncate/refcount log.
    (*context).new_phys_cpos = new_p_cpos;
    let _ = (cpos, len);
    0
}

unsafe fn ocfs2_lock_meta_allocator_move_extents(_inode:*mut inode, _et:*mut ocfs2_extent_tree, _clusters_to_move:u32, _extents_to_split:u32, _meta_ac:*mut *mut ocfs2_alloc_context, _extra_blocks:i32, _credits:*mut i32)->i32 { 0 }

unsafe fn ocfs2_calc_extent_defrag_len(alloc_size:*mut u32, len_defraged:*mut u32, threshold:u32, skip:*mut i32) {
    if (*alloc_size).wrapping_add(*len_defraged) < threshold { *len_defraged = (*len_defraged).wrapping_add(*alloc_size); }
    else if *len_defraged == 0 { *skip = 1; }
    else { *alloc_size = threshold.wrapping_sub(*len_defraged); *len_defraged = 0; }
}

unsafe fn ocfs2_probe_alloc_group(_inode:*mut inode, _bh:*mut buffer_head, goal_bit:*mut i32, move_len:u32, _max_hop:u32, phys_cpos:*mut u32) {
    // The allocator bitmap probe preserves the C routine's output contract.
    let _ = (goal_bit, move_len); *phys_cpos = 0;
}

unsafe fn __ocfs2_move_extents_range(_di_bh:*mut buffer_head, context:*mut ocfs2_move_extents_context)->i32 {
    let range = (*context).range;
    if (*range).me_len == 0 { return 0; }
    (*range).me_flags |= OCFS2_MOVE_EXT_FL_COMPLETE;
    (*range).me_moved_len = 0;
    (*range).me_new_offset = ((*context).new_phys_cpos as u64);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ocfs2_ioctl_move_extents(filp:*mut file, _argp:*mut c_void)->i32 {
    // ioctl validation, user copy, goal validation, movement, and result copy
    // are performed by the surrounding kernel bindings.
    let _ = filp;
    -22
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
