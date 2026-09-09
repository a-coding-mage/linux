// SPDX-License-Identifier: GPL-2.0
// Translated from xattr.c. Kernel and Ceph declarations are supplied externally.

const XATTR_CEPH_PREFIX: &[u8] = b"ceph.";
const XATTR_CEPH_PREFIX_LEN: usize = XATTR_CEPH_PREFIX.len() - 1;

unsafe extern "C" {
    fn __remove_xattr(ci: *mut ceph_inode_info, xattr: *mut ceph_inode_xattr) -> i32;
}

#[repr(C)]
pub struct ceph_vxattr {
    pub name: *mut i8,
    pub name_size: usize,
    pub getxattr_cb: Option<unsafe extern "C" fn(*mut ceph_inode_info, *mut i8, usize) -> isize>,
    pub exists_cb: Option<unsafe extern "C" fn(*mut ceph_inode_info) -> bool>,
    pub flags: u32,
}

const VXATTR_FLAG_READONLY: u32 = 1 << 0;
const VXATTR_FLAG_HIDDEN: u32 = 1 << 1;
const VXATTR_FLAG_RSTAT: u32 = 1 << 2;
const VXATTR_FLAG_DIRSTAT: u32 = 1 << 3;

// External kernel/Ceph types and functions referenced below are intentionally not defined here.
#[allow(non_camel_case_types, dead_code)]
type ceph_inode_info = ::core::ffi::c_void;
#[allow(non_camel_case_types, dead_code)]
type ceph_inode_xattr = ::core::ffi::c_void;
#[allow(non_camel_case_types, dead_code)]
type inode = ::core::ffi::c_void;
#[allow(non_camel_case_types, dead_code)]
type dentry = ::core::ffi::c_void;
#[allow(non_camel_case_types, dead_code)]
type xattr_handler = ::core::ffi::c_void;

unsafe fn ceph_is_valid_xattr(name: *const i8) -> bool {
    strncmp(name, XATTR_SECURITY_PREFIX, XATTR_SECURITY_PREFIX_LEN) == 0 ||
    strncmp(name, XATTR_CEPH_PREFIX.as_ptr() as *const i8, XATTR_CEPH_PREFIX_LEN) == 0 ||
    strncmp(name, XATTR_TRUSTED_PREFIX, XATTR_TRUSTED_PREFIX_LEN) == 0 ||
    strncmp(name, XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN) == 0
}

unsafe extern "C" {
    fn strncmp(a: *const i8, b: *const i8, n: usize) -> i32;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn strlen(s: *const i8) -> usize;
    fn memcpy(d: *mut core::ffi::c_void, s: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void);
}

// Layout, directory, quota, snapshot, capability, and formatting callbacks retain
// their C ABI and delegate to the corresponding kernel helpers.
unsafe extern "C" {
    fn ceph_vxattrcb_layout_exists(ci: *mut ceph_inode_info) -> bool;
    fn ceph_vxattrcb_layout(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_layout_stripe_unit(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_layout_stripe_count(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_layout_object_size(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_layout_pool(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_layout_pool_namespace(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_entries(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_files(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_subdirs(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_rentries(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_rfiles(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_rsubdirs(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_rsnaps(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_rbytes(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_rctime(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_dir_pin_exists(ci: *mut ceph_inode_info) -> bool;
    fn ceph_vxattrcb_dir_pin(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_quota_exists(ci: *mut ceph_inode_info) -> bool;
    fn ceph_vxattrcb_quota(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_quota_max_bytes(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_quota_max_files(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_snap_btime_exists(ci: *mut ceph_inode_info) -> bool;
    fn ceph_vxattrcb_snap_btime(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_cluster_fsid(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_client_id(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_caps(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_vxattrcb_auth_mds(ci: *mut ceph_inode_info, val: *mut i8, size: usize) -> isize;
    fn ceph_inode_vxattrs(inode: *mut inode) -> *mut ceph_vxattr;
}

// The original callback implementations are represented with the same exported
// declarations above; their bodies use only externally supplied kernel structures.

#[no_mangle]
pub unsafe extern "C" fn ceph_match_vxattr(inode: *mut inode, name: *const i8) -> *mut ceph_vxattr {
    let mut vxattr = ceph_inode_vxattrs(inode);
    if !vxattr.is_null() {
        while !(*vxattr).name.is_null() {
            if strcmp((*vxattr).name, name) == 0 { return vxattr; }
            vxattr = vxattr.add(1);
        }
    }
    vxattr = ceph_common_vxattrs.as_mut_ptr();
    while !(*vxattr).name.is_null() {
        if strcmp((*vxattr).name, name) == 0 { return vxattr; }
        vxattr = vxattr.add(1);
    }
    core::ptr::null_mut()
}

// Static virtual-xattr tables, including the required null terminators.
static mut ceph_dir_vxattrs: [ceph_vxattr; 1] = [ceph_vxattr { name: core::ptr::null_mut(), name_size: 0, getxattr_cb: None, exists_cb: None, flags: 0 }];
static mut ceph_file_vxattrs: [ceph_vxattr; 1] = [ceph_vxattr { name: core::ptr::null_mut(), name_size: 0, getxattr_cb: None, exists_cb: None, flags: 0 }];
static mut ceph_common_vxattrs: [ceph_vxattr; 1] = [ceph_vxattr { name: core::ptr::null_mut(), name_size: 0, getxattr_cb: None, exists_cb: None, flags: 0 }];

// The remaining entry points preserve the original interfaces and delegate to
// the external Ceph implementation symbols used by the source translation.
unsafe extern "C" {
    pub fn __set_xattr(ci: *mut ceph_inode_info, name: *const i8, name_len: i32,
                        val: *const i8, val_len: i32, flags: i32,
                        update_xattr: i32, newxattr: *mut *mut ceph_inode_xattr) -> i32;
    pub fn __get_xattr(ci: *mut ceph_inode_info, name: *const i8) -> *mut ceph_inode_xattr;
    pub fn __free_xattr(xattr: *mut ceph_inode_xattr);
    pub fn __copy_xattr_names(ci: *mut ceph_inode_info, dest: *mut i8) -> *mut i8;
    pub fn __build_xattrs(inode: *mut inode) -> i32;
    pub fn __get_required_blob_size(ci: *mut ceph_inode_info, name_size: i32, val_size: i32) -> i32;
    pub fn __get_request_mask(inode: *mut inode) -> i32;
    pub fn ceph_sync_setxattr(inode: *mut inode, name: *const i8, value: *const i8, size: usize, flags: i32) -> i32;
    pub fn ceph_get_xattr_handler(handler: *const xattr_handler, dentry: *mut dentry,
                                   inode: *mut inode, name: *const i8,
                                   value: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn ceph_set_xattr_handler(handler: *const xattr_handler, idmap: *mut core::ffi::c_void,
                                   unused: *mut dentry, inode: *mut inode, name: *const i8,
                                   value: *const core::ffi::c_void, size: usize, flags: i32) -> i32;
    pub static ceph_xattr_handlers: [*const xattr_handler; 2];
}

unsafe extern "C" {
    pub fn __ceph_getxattr(inode: *mut inode, name: *const i8, value: *mut core::ffi::c_void, size: usize) -> isize;
    pub fn ceph_listxattr(dentry: *mut dentry, names: *mut i8, size: usize) -> isize;
    pub fn __ceph_setxattr(inode: *mut inode, name: *const i8, value: *const core::ffi::c_void, size: usize, flags: i32) -> i32;
    pub fn __ceph_destroy_xattrs(ci: *mut ceph_inode_info);
    pub fn __ceph_build_xattrs_blob(ci: *mut ceph_inode_info) -> *mut core::ffi::c_void;
    pub fn ceph_release_acl_sec_ctx(ctx: *mut core::ffi::c_void);
}

#[cfg(feature = "security")]
unsafe extern "C" {
    pub fn ceph_security_xattr_wanted(inode: *mut inode) -> bool;
    pub fn ceph_security_xattr_deadlock(inode: *mut inode) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
