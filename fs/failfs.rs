// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Christian Brauner <brauner@kernel.org> */

// Kernel headers and "internal.h" provide the types, constants, and functions
// referenced below.

static mut failfs_root_path: path = unsafe { core::mem::zeroed() };

unsafe fn failfs_get_root(path: *mut path) {
    *path = failfs_root_path;
    path_get(path);
}

unsafe fn failfs_mnt(mnt: *const vfsmount) -> bool {
    (*mnt).mnt_sb == failfs_root_path.mnt.as_ref().unwrap().mnt_sb
}

unsafe fn failfs_permission(_idmap: *mut mnt_idmap, _inode: *mut inode, _mask: i32) -> i32 {
    -EOPNOTSUPP
}

unsafe fn failfs_lookup(
    _dir: *mut inode,
    _dentry: *mut dentry,
    _flags: u32,
) -> *mut dentry {
    /* Unreachable: ->permission() already failed the walk. */
    ERR_PTR(-EOPNOTSUPP)
}

unsafe fn failfs_getattr(
    _idmap: *mut mnt_idmap,
    _path: *const path,
    _stat: *mut kstat,
    _request_mask: u32,
    _query_flags: u32,
) -> i32 {
    -EOPNOTSUPP
}

static failfs_dir_inode_operations: inode_operations = inode_operations {
    permission: Some(failfs_permission),
    lookup: Some(failfs_lookup),
    getattr: Some(failfs_getattr),
};

static failfs_dir_operations: file_operations = file_operations {};

unsafe fn failfs_d_weak_revalidate(_dentry: *mut dentry, _flags: u32) -> i32 {
    /*
     * The root is only ever reached as a path-walk terminal by jumping
     * to it: as "/" when it is the caller's root, or through a
     * /proc/<pid>/{root,cwd} magic link. ->permission() already fails
     * every walk of a component, but a jump lands on the root without
     * one. Refuse here too so the root cannot be pinned by an O_PATH
     * open or encoded into a file handle.
     */
    -EOPNOTSUPP
}

unsafe fn failfs_dname(_dentry: *mut dentry, buffer: *mut i8, buflen: i32) -> *mut i8 {
    dynamic_dname(buffer, buflen, "failfs:/")
}

static failfs_dentry_operations: dentry_operations = dentry_operations {
    d_dname: Some(failfs_dname),
    d_weak_revalidate: Some(failfs_d_weak_revalidate),
};

unsafe fn failfs_statfs(_dentry: *mut dentry, _buf: *mut kstatfs) -> i32 {
    -EOPNOTSUPP
}

static failfs_super_operations: super_operations = super_operations {
    statfs: Some(failfs_statfs),
};

unsafe fn failfs_fill_super(s: *mut super_block, _fc: *mut fs_context) -> i32 {
    let inode: *mut inode;

    (*s).s_maxbytes = MAX_LFS_FILESIZE;
    (*s).s_blocksize = PAGE_SIZE;
    (*s).s_blocksize_bits = PAGE_SHIFT;
    (*s).s_magic = FAIL_FS_MAGIC;
    (*s).s_op = &failfs_super_operations;
    (*s).s_export_op = core::ptr::null();
    (*s).s_xattr = core::ptr::null();
    (*s).s_time_gran = 1;
    (*s).s_d_flags = 0;

    inode = new_inode(s);
    if inode.is_null() {
        return -ENOMEM;
    }

    /* failfs supports no operations... */
    (*inode).i_mode = S_IFDIR;
    set_nlink(inode, 2);
    (*inode).i_op = &failfs_dir_inode_operations;
    (*inode).i_fop = &failfs_dir_operations;
    simple_inode_init_ts(inode);
    (*inode).i_ino = 1;
    /* ... and is immutable. */
    (*inode).i_flags |= S_IMMUTABLE;

    set_default_d_op(s, &failfs_dentry_operations);
    (*s).s_root = d_make_root(inode);
    if (*s).s_root == core::ptr::null_mut() {
        return -ENOMEM;
    }

    0
}

unsafe fn failfs_get_tree(fc: *mut fs_context) -> i32 {
    get_tree_single(fc, failfs_fill_super)
}

static failfs_context_ops: fs_context_operations = fs_context_operations {
    get_tree: Some(failfs_get_tree),
};

unsafe fn failfs_init_fs_context(fc: *mut fs_context) -> i32 {
    (*fc).ops = &failfs_context_ops;
    (*fc).global = true;
    (*fc).sb_flags |= SB_NOUSER;
    (*fc).s_iflags |= SB_I_NOEXEC | SB_I_NODEV;
    0
}

unsafe fn failfs_current_chdir() -> i32 {
    let mut path: path = core::mem::zeroed();

    failfs_get_root(&mut path);
    set_fs_pwd((*current).fs, &path);
    path_put(&mut path);
    0
}

static mut failfs_fs_type: file_system_type = file_system_type {
    name: "failfs",
    init_fs_context: Some(failfs_init_fs_context),
    kill_sb: Some(kill_anon_super),
};

unsafe fn failfs_init() {
    let mnt: *mut vfsmount;

    /* A single instance that is member of no mount namespace. */
    mnt = kern_mount(&mut failfs_fs_type);
    if IS_ERR(mnt) {
        panic!("VFS: Failed to create failfs");
    }

    failfs_root_path.mnt = mnt;
    failfs_root_path.dentry = (*mnt).mnt_root;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
