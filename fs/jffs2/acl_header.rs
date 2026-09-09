/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2006  NEC Corporation
 *
 * Created by KaiGai Kohei <kaigai@ak.jp.nec.com>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 *
 */

#[repr(C)]
pub struct jffs2_acl_entry {
    pub e_tag: jint16_t,
    pub e_perm: jint16_t,
    pub e_id: jint32_t,
}

#[repr(C)]
pub struct jffs2_acl_entry_short {
    pub e_tag: jint16_t,
    pub e_perm: jint16_t,
}

#[repr(C)]
pub struct jffs2_acl_header {
    pub a_version: jint32_t,
    pub a_entries: [jffs2_acl_entry; 0],
}

// CONFIG_JFFS2_FS_POSIX_ACL is a build-time configuration condition.
#[cfg(feature = "CONFIG_JFFS2_FS_POSIX_ACL")]
extern "C" {
    pub fn jffs2_get_acl(inode: *mut inode, type_: core::ffi::c_int, rcu: bool) -> *mut posix_acl;
    pub fn jffs2_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn jffs2_init_acl_pre(
        _: *mut inode,
        _: *mut inode,
        _: *mut umode_t,
    ) -> core::ffi::c_int;
    pub fn jffs2_init_acl_post(_: *mut inode) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_JFFS2_FS_POSIX_ACL"))]
macro_rules! jffs2_get_acl {
    () => {
        core::ptr::null_mut::<posix_acl>()
    };
}

#[cfg(not(feature = "CONFIG_JFFS2_FS_POSIX_ACL"))]
macro_rules! jffs2_set_acl {
    ($($arg:tt)*) => {
        core::ptr::null_mut::<()>()
    };
}

#[cfg(not(feature = "CONFIG_JFFS2_FS_POSIX_ACL"))]
macro_rules! jffs2_init_acl_pre {
    ($($arg:tt)*) => {
        0
    };
}

#[cfg(not(feature = "CONFIG_JFFS2_FS_POSIX_ACL"))]
macro_rules! jffs2_init_acl_post {
    ($($arg:tt)*) => {
        0
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
