// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Christian Brauner <brauner@kernel.org> */

// Dependencies supplied by the surrounding kernel sources:
// linux/fs/super_types.h, linux/fs_context.h, linux/magic.h, and mount.h.

#[allow(non_camel_case_types)]
type c_int = i32;

#[repr(C)]
pub struct super_operations {
    pub statfs: Option<unsafe extern "C" fn(*mut super_block, *mut kstatfs) -> c_int>,
}

#[repr(C)]
pub struct super_block {
    pub s_maxbytes: i64,
    pub s_blocksize: u32,
    pub s_blocksize_bits: u32,
    pub s_magic: u32,
    pub s_op: *const super_operations,
    pub s_export_op: *const export_operations,
    pub s_xattr: *const xattr_handler,
    pub s_time_gran: u32,
    pub s_d_flags: u32,
    pub s_root: *mut dentry,
}

#[repr(C)]
pub struct fs_context {
    pub ops: *const fs_context_operations,
    pub sb_flags: u32,
    pub s_iflags: u32,
}

#[repr(C)]
pub struct inode {
    pub i_ino: u64,
    pub i_flags: u32,
}

#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct export_operations;
#[repr(C)]
pub struct xattr_handler;
#[repr(C)]
pub struct kstatfs;

#[repr(C)]
pub struct fs_context_operations {
    pub get_tree: Option<unsafe extern "C" fn(*mut fs_context) -> c_int>,
}

#[repr(C)]
pub struct file_system_type {
    pub name: *const u8,
    pub init_fs_context: Option<unsafe extern "C" fn(*mut fs_context) -> c_int>,
    pub kill_sb: Option<unsafe extern "C" fn(*mut super_block)>,
}

extern "C" {
    static simple_statfs: unsafe extern "C" fn(*mut super_block, *mut kstatfs) -> c_int;
    static MAX_LFS_FILESIZE: i64;
    static PAGE_SIZE: u32;
    static PAGE_SHIFT: u32;
    static NULL_FS_MAGIC: u32;
    static SB_NOUSER: u32;
    static SB_I_NOEXEC: u32;
    static SB_I_NODEV: u32;
    static S_IMMUTABLE: u32;

    fn new_inode(s: *mut super_block) -> *mut inode;
    fn make_empty_dir_inode(inode: *mut inode);
    fn simple_inode_init_ts(inode: *mut inode);
    fn d_make_root(inode: *mut inode) -> *mut dentry;
    fn get_tree_nodev(
        fc: *mut fs_context,
        fill_super: unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int,
    ) -> c_int;
    fn kill_anon_super(s: *mut super_block);
}

const ENOMEM: c_int = 12;

static NULLFS_SUPER_OPERATIONS: super_operations = super_operations {
    statfs: Some(simple_statfs),
};

unsafe extern "C" fn nullfs_fs_fill_super(
    s: *mut super_block,
    _fc: *mut fs_context,
) -> c_int {
    let inode: *mut inode;

    (*s).s_maxbytes = MAX_LFS_FILESIZE;
    (*s).s_blocksize = PAGE_SIZE;
    (*s).s_blocksize_bits = PAGE_SHIFT;
    (*s).s_magic = NULL_FS_MAGIC;
    (*s).s_op = &NULLFS_SUPER_OPERATIONS;
    (*s).s_export_op = core::ptr::null();
    (*s).s_xattr = core::ptr::null();
    (*s).s_time_gran = 1;
    (*s).s_d_flags = 0;

    inode = new_inode(s);
    if inode.is_null() {
        return -ENOMEM;
    }

    /* nullfs is permanently empty... */
    make_empty_dir_inode(inode);
    simple_inode_init_ts(inode);
    (*inode).i_ino = 1;
    /* ... and immutable. */
    (*inode).i_flags |= S_IMMUTABLE;

    (*s).s_root = d_make_root(inode);
    if (*s).s_root.is_null() {
        return -ENOMEM;
    }

    0
}

unsafe extern "C" fn nullfs_fs_get_tree(fc: *mut fs_context) -> c_int {
    get_tree_nodev(fc, nullfs_fs_fill_super)
}

static NULLFS_FS_CONTEXT_OPS: fs_context_operations = fs_context_operations {
    get_tree: Some(nullfs_fs_get_tree),
};

unsafe extern "C" fn nullfs_init_fs_context(fc: *mut fs_context) -> c_int {
    (*fc).ops = &NULLFS_FS_CONTEXT_OPS;
    (*fc).sb_flags |= SB_NOUSER;
    (*fc).s_iflags |= SB_I_NOEXEC | SB_I_NODEV;
    0
}

#[no_mangle]
pub static mut nullfs_fs_type: file_system_type = file_system_type {
    name: b"nullfs\0".as_ptr(),
    init_fs_context: Some(nullfs_init_fs_context),
    kill_sb: Some(kill_anon_super),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
