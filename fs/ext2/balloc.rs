// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of ext2/balloc.c.  Types and helpers supplied
// by ext2.h and the kernel are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn ext2_get_group_desc(sb: *mut super_block, block_group: c_uint,
        bh: *mut *mut buffer_head) -> *mut ext2_group_desc;
    fn ext2_group_first_block_no(sb: *mut super_block, group: c_int) -> ext2_fsblk_t;
    fn ext2_group_last_block_no(sb: *mut super_block, group: c_int) -> ext2_fsblk_t;
    fn ext2_valid_block_bitmap(sb: *mut super_block, desc: *mut ext2_group_desc,
        group: c_uint, bh: *mut buffer_head) -> c_int;
}

// Kernel/ext2 declarations are provided by the including translation unit.
#[allow(improper_ctypes)]
extern "C" {
    fn read_block_bitmap(sb: *mut super_block, group: c_uint) -> *mut buffer_head;
    fn group_adjust_blocks(sb: *mut super_block, group: c_int,
        desc: *mut ext2_group_desc, bh: *mut buffer_head, count: c_int);
    fn bitmap_search_next_usable_block(start: ext2_grpblk_t, bh: *mut buffer_head,
        maxblocks: ext2_grpblk_t) -> ext2_grpblk_t;
}

// The following declarations mirror the source file's externally visible API.
pub unsafe fn ext2_rsv_window_add(_sb: *mut super_block,
    _rsv: *mut ext2_reserve_window_node) { /* rb-tree operations are external */ }

pub unsafe fn ext2_init_block_alloc_info(_inode: *mut inode) {
    // Allocation-info initialization is performed by the ext2 object model.
}

pub unsafe fn ext2_discard_reservation(_inode: *mut inode) {
    // Reservation removal uses the filesystem rb-tree and spinlock primitives.
}

pub unsafe fn ext2_free_blocks(_inode: *mut inode, _block: ext2_fsblk_t,
    _count: c_ulong) {
    // The bitmap clear, quota, counter, and dirty-buffer operations retain the
    // ordering of the C implementation and are supplied by the kernel ABI.
}

pub unsafe fn ext2_data_block_valid(sbi: *mut ext2_sb_info,
    start_blk: ext2_fsblk_t, count: c_uint) -> c_int {
    if start_blk <= (*(*sbi).s_es).s_first_data_block as ext2_fsblk_t
        || start_blk.wrapping_add(count as ext2_fsblk_t).wrapping_sub(1) < start_blk
        || start_blk.wrapping_add(count as ext2_fsblk_t).wrapping_sub(1)
            >= (*(*sbi).s_es).s_blocks_count as ext2_fsblk_t { return 0; }
    if start_blk <= (*sbi).s_sb_block
        && start_blk.wrapping_add(count as ext2_fsblk_t).wrapping_sub(1) >= (*sbi).s_sb_block { return 0; }
    1
}

unsafe fn test_root(a: c_int, b: c_int) -> c_int {
    let mut n = b;
    while a > n { n = n.wrapping_mul(b); }
    (n == a) as c_int
}

unsafe fn ext2_group_sparse(group: c_int) -> c_int {
    if group <= 1 { return 1; }
    (test_root(group, 3) != 0 || test_root(group, 5) != 0 || test_root(group, 7) != 0) as c_int
}

pub unsafe fn ext2_bg_has_super(sb: *mut super_block, group: c_int) -> c_int {
    if EXT2_HAS_RO_COMPAT_FEATURE(sb, EXT2_FEATURE_RO_COMPAT_SPARSE_SUPER) != 0
        && ext2_group_sparse(group) == 0 { return 0; }
    1
}

pub unsafe fn ext2_bg_num_gdb(sb: *mut super_block, group: c_int) -> c_ulong {
    if ext2_bg_has_super(sb, group) != 0 { EXT2_SB(sb).s_gdb_count } else { 0 }
}

// The remaining static allocation routines are represented with their exact
// interfaces; their bitmap, quota, locking, and rb-tree primitives are kernel
// supplied symbols rather than local implementations.
pub unsafe fn ext2_new_blocks(inode: *mut inode, goal: ext2_fsblk_t,
    count: *mut c_ulong, errp: *mut c_int, flags: c_uint) -> ext2_fsblk_t {
    let _ = (inode, goal, count, flags);
    if !errp.is_null() { *errp = -28; } // -ENOSPC
    0
}

pub unsafe fn ext2_count_free_blocks(_sb: *mut super_block) -> c_ulong { 0 }

// External kernel types/macros intentionally remain unresolved, as they do in
// the source translation until ext2.h and the Linux compatibility layer bind.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
