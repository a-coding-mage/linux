/* SPDX-License-Identifier: GPL-2.0 */
/*
  File: linux/posix_acl.h

  (C) 2002 Andreas Gruenbacher, <a.gruenbacher@computer.org>
*/

// Dependencies supplied by the surrounding kernel translation.

pub struct UserNamespace;

#[repr(C)]
pub union PosixAclEntryId {
    pub e_uid: kuid_t,
    pub e_gid: kgid_t,
}

#[repr(C)]
pub struct PosixAclEntry {
    pub e_tag: i16,
    pub e_perm: u16,
    pub id: PosixAclEntryId,
}

#[repr(C)]
pub struct PosixAclHdr {
    pub a_refcount: refcount_t,
    pub a_count: u32,
    pub a_rcu: rcu_head,
}

#[repr(C)]
pub struct PosixAcl {
    pub a_refcount: refcount_t,
    pub a_count: u32,
    pub a_rcu: rcu_head,
    pub a_entries: [PosixAclEntry; 0],
}

// FOREACH_ACL_ENTRY(pa, acl, pe):
// for (pa = (acl)->a_entries, pe = pa + (acl)->a_count; pa < pe; pa++)

/* Duplicate an ACL handle. */
#[inline]
pub unsafe fn posix_acl_dup(acl: *mut PosixAcl) -> *mut PosixAcl {
    if !acl.is_null() {
        refcount_inc(&mut (*acl).a_refcount);
    }
    acl
}

/* Free an ACL handle. */
#[inline]
pub unsafe fn posix_acl_release(acl: *mut PosixAcl) {
    if !acl.is_null() && refcount_dec_and_test(&mut (*acl).a_refcount) {
        kfree_rcu(acl, a_rcu);
    }
}

/* posix_acl.c */
extern "C" {
    pub fn posix_acl_init(acl: *mut PosixAcl, count: i32);
    pub fn posix_acl_alloc(count: u32, flags: gfp_t) -> *mut PosixAcl;
    pub fn posix_acl_from_mode(mode: umode_t, flags: gfp_t) -> *mut PosixAcl;
    pub fn posix_acl_equiv_mode(acl: *const PosixAcl, mode_p: *mut umode_t) -> i32;
    pub fn __posix_acl_create(acl: *mut *mut PosixAcl, flags: gfp_t, mode: *mut umode_t) -> i32;
    pub fn __posix_acl_chmod(acl: *mut *mut PosixAcl, flags: gfp_t, mode: umode_t) -> i32;

    pub fn get_posix_acl(inode: *mut inode, type_: i32) -> *mut PosixAcl;
    pub fn set_posix_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, type_: i32, acl: *mut PosixAcl) -> i32;
    pub fn get_cached_acl_rcu(inode: *mut inode, type_: i32) -> *mut PosixAcl;
    pub fn posix_acl_clone(acl: *const PosixAcl, flags: gfp_t) -> *mut PosixAcl;
}

#[cfg(CONFIG_FS_POSIX_ACL)]
extern "C" {
    pub fn posix_acl_chmod(idmap: *mut mnt_idmap, dentry: *mut dentry, mode: umode_t) -> i32;
    pub fn posix_acl_create(inode: *mut inode, mode: *mut umode_t,
                            default_acl: *mut *mut PosixAcl, acl: *mut *mut PosixAcl) -> i32;
    pub fn posix_acl_update_mode(idmap: *mut mnt_idmap, inode: *mut inode, mode: *mut umode_t,
                                 acl: *mut *mut PosixAcl) -> i32;
    pub fn simple_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut PosixAcl, type_: i32) -> i32;
    pub fn simple_acl_create(dir: *mut inode, inode: *mut inode) -> i32;
    pub fn get_cached_acl(inode: *mut inode, type_: i32) -> *mut PosixAcl;
    pub fn set_cached_acl(inode: *mut inode, type_: i32, acl: *mut PosixAcl);
    pub fn forget_cached_acl(inode: *mut inode, type_: i32);
    pub fn forget_all_cached_acls(inode: *mut inode);
    pub fn posix_acl_valid(user_ns: *mut UserNamespace, acl: *const PosixAcl) -> i32;
    pub fn posix_acl_permission(idmap: *mut mnt_idmap, inode: *mut inode, acl: *const PosixAcl, mask: i32) -> i32;
    pub fn vfs_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl_name: *const i8, kacl: *mut PosixAcl) -> i32;
    pub fn vfs_get_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl_name: *const i8) -> *mut PosixAcl;
    pub fn vfs_remove_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl_name: *const i8) -> i32;
    pub fn posix_acl_listxattr(inode: *mut inode, buffer: *mut *mut i8, remaining_size: *mut ssize_t) -> i32;
}

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn posix_acl_chmod(_idmap: *mut mnt_idmap, _dentry: *mut dentry, _mode: umode_t) -> i32 { 0 }

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn simple_acl_create(_dir: *mut inode, _inode: *mut inode) -> i32 { 0 }

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn cache_no_acl(inode: *mut inode) {
    (*inode).i_acl = core::ptr::null_mut();
    (*inode).i_default_acl = core::ptr::null_mut();
}

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn posix_acl_create(_inode: *mut inode, _mode: *mut umode_t,
                               default_acl: *mut *mut PosixAcl, acl: *mut *mut PosixAcl) -> i32 {
    *default_acl = core::ptr::null_mut();
    *acl = core::ptr::null_mut();
    0
}

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn forget_all_cached_acls(_inode: *mut inode) {}

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn vfs_set_acl(_idmap: *mut mnt_idmap, _dentry: *mut dentry, _name: *const i8, _acl: *mut PosixAcl) -> i32 { -EOPNOTSUPP }

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn vfs_get_acl(_idmap: *mut mnt_idmap, _dentry: *mut dentry, _acl_name: *const i8) -> *mut PosixAcl { ERR_PTR(-EOPNOTSUPP) }

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn vfs_remove_acl(_idmap: *mut mnt_idmap, _dentry: *mut dentry, _acl_name: *const i8) -> i32 { -EOPNOTSUPP }

#[cfg(not(CONFIG_FS_POSIX_ACL))]
#[inline]
pub unsafe fn posix_acl_listxattr(_inode: *mut inode, _buffer: *mut *mut i8, _remaining_size: *mut ssize_t) -> i32 { 0 }

extern "C" {
    pub fn get_inode_acl(inode: *mut inode, type_: i32) -> *mut PosixAcl;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
