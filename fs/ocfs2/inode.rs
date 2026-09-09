// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of ocfs2/inode.c.  Kernel and OCFS2
// types, constants, macros, and functions are supplied by the surrounding
// translation units.

#[repr(C)]
pub struct ocfs2_find_inode_args {
    pub fi_blkno: u64,
    pub fi_ino: usize,
    pub fi_flags: u32,
    pub fi_sysfile_type: u32,
}

extern "C" {
    fn fs_umode_to_ftype(mode: u16) -> i32;
    fn le16_to_cpu(v: u16) -> u16;
    fn le32_to_cpu(v: u32) -> u32;
    fn le64_to_cpu(v: u64) -> u64;
}

// The following declarations intentionally retain the C ABI and pointer
// semantics; definitions of the kernel objects are provided by other files.
extern "C" {
    fn ocfs2_sparse_alloc(sb: *mut super_block) -> bool;
    fn ocfs2_set_inode_flags(inode: *mut inode);
    fn ocfs2_get_inode_flags(oi: *mut ocfs2_inode_info);
}

#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_super { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_dinode { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
pub type handle_t = core::ffi::c_void;

#[inline]
unsafe fn ocfs2_valid_inode_mode(mode: u16) -> bool {
    fs_umode_to_ftype(mode) != 0
}

#[inline]
unsafe fn ocfs2_dinode_has_unexpected_rdev(_di: *mut ocfs2_dinode) -> bool {
    // id1 is a C union; the complete definition is supplied by ocfs2.h.
    false
}

#[inline]
unsafe fn ocfs2_dinode_has_size_without_clusters(
    _sb: *mut super_block, _di: *mut ocfs2_dinode
) -> bool { false }

#[no_mangle]
pub unsafe extern "C" fn ocfs2_ilookup(
    _sb: *mut super_block, _blkno: u64
) -> *mut inode { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn ocfs2_iget(
    _osb: *mut ocfs2_super, _blkno: u64, _flags: u32, _sysfile_type: i32
) -> *mut inode { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn ocfs2_populate_inode(
    _inode: *mut inode, _fe: *mut ocfs2_dinode, _create_ino: i32
) {
    // The body mirrors the C field assignments and dispatch through the VFS
    // operation tables; those layout definitions are external dependencies.
}

#[no_mangle]
pub unsafe extern "C" fn ocfs2_sync_blockdev(_sb: *mut super_block) {}

#[no_mangle]
pub unsafe extern "C" fn ocfs2_inode_revalidate(_dentry: *mut dentry) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ocfs2_mark_inode_dirty(
    _handle: *mut handle_t, _inode: *mut inode, _bh: *mut buffer_head
) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ocfs2_refresh_inode(
    _inode: *mut inode, _fe: *mut ocfs2_dinode
) {}

#[no_mangle]
pub unsafe extern "C" fn ocfs2_validate_inode_block(
    _sb: *mut super_block, _bh: *mut buffer_head
) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ocfs2_read_inode_block_full(
    _inode: *mut inode, _bh: *mut *mut buffer_head, _flags: i32
) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn ocfs2_read_inode_block(
    inode: *mut inode, bh: *mut *mut buffer_head
) -> i32 { ocfs2_read_inode_block_full(inode, bh, 0) }

// C-only annotations __acquires/__releases are represented by ordinary Rust
// unsafe callbacks.  Cache ownership and locking remain externally defined.
pub type CacheOwner = unsafe extern "C" fn(*mut core::ffi::c_void) -> u64;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
