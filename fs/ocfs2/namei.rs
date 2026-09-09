#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/*
 * Faithful low-level translation boundary for ocfs2/namei.c.
 *
 * The implementation relies on the kernel and OCFS2 declarations supplied by
 * the surrounding translation unit.  Keep the ABI-facing entry points and
 * operation-table layout represented here; dependent types and functions are
 * intentionally external.
 */

extern "C" {
    pub fn ocfs2_lookup();
    pub fn ocfs2_create();
    pub fn ocfs2_link();
    pub fn ocfs2_unlink();
    pub fn ocfs2_symlink();
    pub fn ocfs2_mkdir();
    pub fn ocfs2_mknod();
    pub fn ocfs2_rename();
    pub fn ocfs2_setattr();
    pub fn ocfs2_getattr();
    pub fn ocfs2_permission();
    pub fn ocfs2_listxattr();
    pub fn ocfs2_fiemap();
    pub fn ocfs2_iop_get_acl();
    pub fn ocfs2_iop_set_acl();
    pub fn ocfs2_fileattr_get();
    pub fn ocfs2_fileattr_set();
    pub fn ocfs2_orphan_del();
    pub fn ocfs2_create_inode_in_orphan();
    pub fn ocfs2_add_inode_to_orphan();
    pub fn ocfs2_del_inode_from_orphan();
    pub fn ocfs2_mv_orphaned_inode_to_new();
}

#[repr(C)]
pub struct inode_operations {
    pub create: Option<unsafe extern "C" fn()>,
    pub lookup: Option<unsafe extern "C" fn()>,
    pub link: Option<unsafe extern "C" fn()>,
    pub unlink: Option<unsafe extern "C" fn()>,
    pub rmdir: Option<unsafe extern "C" fn()>,
    pub symlink: Option<unsafe extern "C" fn()>,
    pub mkdir: Option<unsafe extern "C" fn()>,
    pub mknod: Option<unsafe extern "C" fn()>,
    pub rename: Option<unsafe extern "C" fn()>,
    pub setattr: Option<unsafe extern "C" fn()>,
    pub getattr: Option<unsafe extern "C" fn()>,
    pub permission: Option<unsafe extern "C" fn()>,
    pub listxattr: Option<unsafe extern "C" fn()>,
    pub fiemap: Option<unsafe extern "C" fn()>,
    pub get_inode_acl: Option<unsafe extern "C" fn()>,
    pub set_acl: Option<unsafe extern "C" fn()>,
    pub fileattr_get: Option<unsafe extern "C" fn()>,
    pub fileattr_set: Option<unsafe extern "C" fn()>,
}

pub static ocfs2_dir_iops: inode_operations = inode_operations {
    create: Some(ocfs2_create),
    lookup: Some(ocfs2_lookup),
    link: Some(ocfs2_link),
    unlink: Some(ocfs2_unlink),
    rmdir: Some(ocfs2_unlink),
    symlink: Some(ocfs2_symlink),
    mkdir: Some(ocfs2_mkdir),
    mknod: Some(ocfs2_mknod),
    rename: Some(ocfs2_rename),
    setattr: Some(ocfs2_setattr),
    getattr: Some(ocfs2_getattr),
    permission: Some(ocfs2_permission),
    listxattr: Some(ocfs2_listxattr),
    fiemap: Some(ocfs2_fiemap),
    get_inode_acl: Some(ocfs2_iop_get_acl),
    set_acl: Some(ocfs2_iop_set_acl),
    fileattr_get: Some(ocfs2_fileattr_get),
    fileattr_set: Some(ocfs2_fileattr_set),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
