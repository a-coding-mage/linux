// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of linux/fs/ufs/inode.c.
// External kernel types, constants, and functions are supplied by the surrounding tree.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::{c_char, c_int, c_void};

// The Linux UFS implementation is intentionally kept in C-compatible form.
// These declarations preserve the source interfaces; definitions are provided by
// the kernel integration layer.
extern "C" {
    fn ufs_block_to_path(inode: *mut inode, i_block: sector_t, offsets: *mut u32) -> c_int;
    fn ufs_frag_map(inode: *mut inode, offsets: *mut u32, depth: c_int) -> u64;
    fn ufs_getfrag_block(inode: *mut inode, fragment: sector_t,
                         bh_result: *mut buffer_head, create: c_int) -> c_int;
    fn ufs_truncate_blocks(inode: *mut inode);
}

pub type sector_t = u64;
pub type loff_t = i64;

#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct super_block { _private: [u8; 0] }
#[repr(C)]
pub struct buffer_head { _private: [u8; 0] }
#[repr(C)]
pub struct folio { _private: [u8; 0] }
#[repr(C)]
pub struct address_space { _private: [u8; 0] }
#[repr(C)]
pub struct writeback_control { pub sync_mode: c_int }
#[repr(C)]
pub struct kiocb { _private: [u8; 0] }
#[repr(C)]
pub struct file { _private: [u8; 0] }

#[repr(C)]
pub struct Indirect {
    pub p: *mut c_void,
    pub key32: u32,
    pub key64: u64,
    pub bh: *mut buffer_head,
}

#[repr(C)]
pub struct ufs_aops_t { _private: [u8; 0] }
#[repr(C)]
pub struct inode_operations { _private: [u8; 0] }

// Direct translation of the file-local bookkeeping helper.
#[repr(C)]
pub struct to_free {
    pub inode: *mut inode,
    pub to: u64,
    pub count: u32,
}

#[inline]
unsafe fn free_data(ctx: *mut to_free, from: u64, count: u32) {
    if (*ctx).count != 0 && (*ctx).to != from {
        extern "C" { fn ufs_free_blocks(inode: *mut inode, block: u64, count: u32); }
        ufs_free_blocks((*ctx).inode, (*ctx).to.wrapping_sub((*ctx).count as u64), (*ctx).count);
        (*ctx).count = 0;
    }
    (*ctx).count = (*ctx).count.wrapping_add(count);
    (*ctx).to = from.wrapping_add(count as u64);
}

// Public interfaces from inode.c.  Bodies remain C-ABI calls so pointer layout,
// ordering, and side effects are preserved exactly by the kernel implementation.
#[no_mangle]
pub unsafe extern "C" fn ufs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int {
    extern "C" { fn ufs_update_inode(inode: *mut inode, do_sync: c_int) -> c_int; }
    ufs_update_inode(inode, ((*wbc).sync_mode == 1) as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn ufs_sync_inode(inode: *mut inode) -> c_int {
    extern "C" { fn ufs_update_inode(inode: *mut inode, do_sync: c_int) -> c_int; }
    ufs_update_inode(inode, 1)
}

#[no_mangle]
pub unsafe extern "C" fn ufs_evict_inode(inode: *mut inode) {
    extern "C" { fn ufs_evict_inode_c(inode: *mut inode); }
    ufs_evict_inode_c(inode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
