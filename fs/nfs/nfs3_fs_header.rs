/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2014 Anna Schumaker.
 *
 * NFSv3-specific filesystem definitions and declarations
 */

/*
 * nfs3acl.c
 */
#[cfg(CONFIG_NFS_V3_ACL)]
extern "C" {
    pub fn nfs3_get_acl(inode: *mut inode, r#type: i32, rcu: bool) -> *mut posix_acl;
    pub fn nfs3_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        r#type: i32,
    ) -> i32;
    pub fn nfs3_proc_setacls(
        inode: *mut inode,
        acl: *mut posix_acl,
        dfacl: *mut posix_acl,
    ) -> i32;
    pub fn nfs3_listxattr(dentry: *mut dentry, buffer: *mut core::ffi::c_char, size: usize) -> isize;
}

#[cfg(not(CONFIG_NFS_V3_ACL))]
pub unsafe fn nfs3_proc_setacls(
    _inode: *mut inode,
    _acl: *mut posix_acl,
    _dfacl: *mut posix_acl,
) -> i32 {
    0
}

#[cfg(not(CONFIG_NFS_V3_ACL))]
pub const nfs3_listxattr: Option<
    unsafe extern "C" fn(*mut dentry, *mut core::ffi::c_char, usize) -> isize,
> = None;

/* nfs3client.c */
extern "C" {
    pub fn nfs3_create_server(fs_context: *mut fs_context) -> *mut nfs_server;
    pub fn nfs3_clone_server(
        server: *mut nfs_server,
        fh: *mut nfs_fh,
        fattr: *mut nfs_fattr,
        authflavor: rpc_authflavor_t,
    ) -> *mut nfs_server;
}

/* nfs3super.c */
extern "C" {
    pub static mut nfs_v3: nfs_subversion;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
