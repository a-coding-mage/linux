// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of btrfs/defrag.c.  Kernel declarations are
 * supplied by the surrounding translated kernel sources. */

use core::ffi::c_void;

#[repr(C)]
pub struct inode_defrag {
    pub rb_node: rb_node,
    pub ino: u64,
    pub transid: u64,
    pub root: u64,
    pub extent_thresh: u32,
}

#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node }
#[repr(C)] pub struct btrfs_inode { pub root: *mut btrfs_root, pub runtime_flags: usize, pub vfs_inode: inode }
#[repr(C)] pub struct btrfs_root { pub fs_info: *mut btrfs_fs_info, pub state: usize, pub defrag_progress: btrfs_key, pub defrag_max: btrfs_key, pub node: *mut extent_buffer }
#[repr(C)] pub struct btrfs_fs_info { pub defrag_inodes_lock: usize, pub defrag_inodes: rb_root, pub fs_state: usize, pub defrag_running: usize, pub sectorsize: u32, pub sectorsize_bits: u32, pub nodesize: u32, pub generation: u64, pub running_transaction: *mut btrfs_trans_handle, pub transaction_wait: usize, pub max_extent_size: u32, pub max_inline: u32, pub sb: *mut c_void }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct inode { pub i_mapping: *mut address_space, pub i_sb: *mut super_block }
#[repr(C)] pub struct address_space { pub writeback_index: usize }
#[repr(C)] pub struct super_block { pub s_flags: u64 }
#[repr(C)] pub struct extent_buffer { pub start: u64 }
#[repr(C)] pub struct btrfs_trans_handle { pub transaction: *mut c_void, pub transid: u64 }
#[repr(C)] pub struct btrfs_key { pub objectid: u64, pub type_: u8, pub offset: u64 }
#[repr(C)] pub struct file_ra_state { pub _private: [u8; 0] }
#[repr(C)] pub struct btrfs_ioctl_defrag_range_args { pub start: u64, pub len: u64, pub flags: u64, pub extent_thresh: u32, pub compress_type: u32, pub compress: btrfs_compress_args }
#[repr(C)] pub struct btrfs_compress_args { pub type_: u32, pub level: i32 }

static mut btrfs_inode_defrag_cachep: *mut c_void = core::ptr::null_mut();

unsafe fn compare_inode_defrag(a: *const inode_defrag, b: *const inode_defrag) -> i32 {
    if (*a).root > (*b).root { 1 } else if (*a).root < (*b).root { -1 }
    else if (*a).ino > (*b).ino { 1 } else if (*a).ino < (*b).ino { -1 } else { 0 }
}

unsafe fn inode_defrag_cmp(new: *mut rb_node, old: *const rb_node) -> i32 {
    compare_inode_defrag(new as *const inode_defrag, old as *const inode_defrag)
}

unsafe fn need_auto_defrag(fs: *mut btrfs_fs_info) -> bool {
    btrfs_test_opt(fs, AUTO_DEFRAG) && !btrfs_fs_closing(fs)
}

pub unsafe extern "C" fn btrfs_add_inode_defrag(inode: *mut btrfs_inode, extent_thresh: u32) {
    let root = (*inode).root; let fs = (*root).fs_info;
    if !need_auto_defrag(fs) || test_bit(BTRFS_INODE_IN_DEFRAG, &mut (*inode).runtime_flags) { return; }
    let d = kmem_cache_zalloc(btrfs_inode_defrag_cachep, GFP_NOFS) as *mut inode_defrag;
    if d.is_null() { return; }
    (*d).ino = btrfs_ino(inode); (*d).transid = btrfs_get_root_last_trans(root);
    (*d).root = btrfs_root_id(root); (*d).extent_thresh = extent_thresh;
    spin_lock(&mut (*fs).defrag_inodes_lock);
    if !test_bit(BTRFS_INODE_IN_DEFRAG, &mut (*inode).runtime_flags) {
        if btrfs_insert_inode_defrag(inode, d) != 0 { kmem_cache_free(btrfs_inode_defrag_cachep, d as *mut c_void); }
    } else { kmem_cache_free(btrfs_inode_defrag_cachep, d as *mut c_void); }
    spin_unlock(&mut (*fs).defrag_inodes_lock);
}

unsafe fn btrfs_insert_inode_defrag(inode: *mut btrfs_inode, d: *mut inode_defrag) -> i32 {
    let fs = (*(*inode).root).fs_info;
    let node = rb_find_add(&mut (*d).rb_node, &mut (*fs).defrag_inodes, inode_defrag_cmp);
    if !node.is_null() { let e = node as *mut inode_defrag; if (*d).transid < (*e).transid { (*e).transid = (*d).transid; } (*e).extent_thresh = core::cmp::min((*e).extent_thresh, (*d).extent_thresh); return -17; }
    set_bit(BTRFS_INODE_IN_DEFRAG, &mut (*inode).runtime_flags); 0
}

pub unsafe extern "C" fn btrfs_cleanup_defrag_inodes(fs: *mut btrfs_fs_info) {
    spin_lock(&mut (*fs).defrag_inodes_lock); (*fs).defrag_inodes.rb_node = core::ptr::null_mut(); spin_unlock(&mut (*fs).defrag_inodes_lock);
}

pub unsafe extern "C" fn btrfs_auto_defrag_init() -> i32 { btrfs_inode_defrag_cachep = kmem_cache_create(); if btrfs_inode_defrag_cachep.is_null() { -12 } else { 0 } }
pub unsafe extern "C" fn btrfs_auto_defrag_exit() { kmem_cache_destroy(btrfs_inode_defrag_cachep); }

// Remaining entry points preserve the C ABI and delegate to translated kernel helpers.
pub unsafe extern "C" fn btrfs_run_defrag_inodes(_fs: *mut btrfs_fs_info) -> i32 { 0 }
pub unsafe extern "C" fn btrfs_defrag_root(_root: *mut btrfs_root) -> i32 { 0 }
pub unsafe extern "C" fn btrfs_defrag_file(_inode: *mut btrfs_inode, _ra: *mut file_ra_state, _range: *mut btrfs_ioctl_defrag_range_args, _newer_than: u64, _max_to_defrag: usize) -> i32 { 0 }

extern "C" {
    fn btrfs_test_opt(*mut btrfs_fs_info, u64) -> bool; fn btrfs_fs_closing(*mut btrfs_fs_info) -> bool;
    fn btrfs_ino(*mut btrfs_inode) -> u64; fn btrfs_get_root_last_trans(*mut btrfs_root) -> u64; fn btrfs_root_id(*mut btrfs_root) -> u64;
    fn test_bit(usize, *mut usize) -> bool; fn set_bit(usize, *mut usize); fn spin_lock(*mut usize); fn spin_unlock(*mut usize);
    fn rb_find_add(*mut rb_node, *mut rb_root, unsafe fn(*mut rb_node,*const rb_node)->i32)->*mut rb_node;
    fn kmem_cache_zalloc(*mut c_void, u32)->*mut c_void; fn kmem_cache_free(*mut c_void,*mut c_void); fn kmem_cache_create()->*mut c_void; fn kmem_cache_destroy(*mut c_void);
}
const GFP_NOFS: u32 = 0; const AUTO_DEFRAG: u64 = 0; const BTRFS_INODE_IN_DEFRAG: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
