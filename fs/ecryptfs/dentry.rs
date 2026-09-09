// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * eCryptfs: Linux filesystem encryption layer
 *
 * Copyright (C) 1997-2003 Erez Zadok
 * Copyright (C) 2001-2003 Stony Brook University
 * Copyright (C) 2004-2006 International Business Machines Corp.
 *   Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

pub const LOOKUP_RCU: ::core::ffi::c_uint = 0x0000_0040;
pub const DCACHE_OP_REVALIDATE: ::core::ffi::c_uint = 0x0000_0010;
pub const ECHILD: ::core::ffi::c_int = 10;

#[repr(C)]
pub struct inode {
    pub i_nlink: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct qstr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct name_snapshot {
    pub name: qstr,
}

#[repr(C)]
pub struct dentry_operations {
    pub d_revalidate: Option<unsafe extern "C" fn(
        *mut inode,
        *const qstr,
        *mut dentry,
        ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int>,
    pub d_release: Option<unsafe extern "C" fn(*mut dentry)>,
}

#[repr(C)]
pub struct dentry {
    pub d_flags: ::core::ffi::c_uint,
    pub d_op: *const dentry_operations,
    pub d_fsdata: *mut ::core::ffi::c_void,
}

extern "C" {
    fn ecryptfs_dentry_to_lower(dentry: *mut dentry) -> *mut dentry;
    fn ecryptfs_inode_to_lower(inode: *mut inode) -> *mut inode;
    fn take_dentry_name_snapshot(snapshot: *mut name_snapshot, dentry: *mut dentry);
    fn release_dentry_name_snapshot(snapshot: *mut name_snapshot);
    fn d_really_is_positive(dentry: *mut dentry) -> bool;
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn fsstack_copy_attr_all(inode: *mut inode, lower_inode: *mut inode);
    fn dput(dentry: *mut ::core::ffi::c_void);
}

/**
 * ecryptfs_d_revalidate - revalidate an ecryptfs dentry
 * @dir: inode of expected parent
 * @name: expected name
 * @dentry: dentry to revalidate
 * @flags: lookup flags
 *
 * Called when the VFS needs to revalidate a dentry. This
 * is called whenever a name lookup finds a dentry in the
 * dcache. Most filesystems leave this as NULL, because all their
 * dentries in the dcache are valid.
 *
 * Returns 1 if valid, 0 otherwise.
 *
 */
unsafe extern "C" fn ecryptfs_d_revalidate(
    dir: *mut inode,
    _name: *const qstr,
    dentry: *mut dentry,
    flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let lower_dentry = ecryptfs_dentry_to_lower(dentry);
    let mut rc: ::core::ffi::c_int = 1;

    if flags & LOOKUP_RCU != 0 {
        return -ECHILD;
    }

    if (*lower_dentry).d_flags & DCACHE_OP_REVALIDATE != 0 {
        let lower_dir = ecryptfs_inode_to_lower(dir);
        let mut n = ::core::mem::MaybeUninit::<name_snapshot>::uninit();

        take_dentry_name_snapshot(n.as_mut_ptr(), lower_dentry);
        let snapshot = n.assume_init_mut();
        let d_revalidate = (*(*lower_dentry).d_op)
            .d_revalidate
            .expect("d_revalidate is required when DCACHE_OP_REVALIDATE is set");
        rc = d_revalidate(lower_dir, &snapshot.name, lower_dentry, flags);
        release_dentry_name_snapshot(snapshot as *mut name_snapshot);
    }

    if d_really_is_positive(dentry) {
        let inode = d_inode(dentry);

        fsstack_copy_attr_all(inode, ecryptfs_inode_to_lower(inode));
        if (*inode).i_nlink == 0 {
            return 0;
        }
    }
    rc
}

/**
 * ecryptfs_d_release
 * @dentry: The ecryptfs dentry
 *
 * Called when a dentry is really deallocated.
 */
unsafe extern "C" fn ecryptfs_d_release(dentry: *mut dentry) {
    dput((*dentry).d_fsdata);
}

#[no_mangle]
pub static ecryptfs_dops: dentry_operations = dentry_operations {
    d_revalidate: Some(ecryptfs_d_revalidate),
    d_release: Some(ecryptfs_d_release),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
