/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2020 Google LLC.
 */

/* External Linux dependencies are supplied by the surrounding translation. */

#[cfg(feature = "CONFIG_BPF_LSM")]
extern "C" {
    pub static mut bpf_lsm_initialized: bool;
}

/* The C LSM_HOOK macro expands declarations from linux/lsm_hook_defs.h. */

#[repr(C)]
pub struct bpf_storage_blob {
    pub storage: *mut bpf_local_storage,
}

#[cfg(feature = "CONFIG_BPF_LSM")]
extern "C" {
    pub static mut bpf_lsm_blob_sizes: lsm_blob_sizes;

    pub fn bpf_lsm_verify_prog(
        vlog: *mut bpf_verifier_log,
        prog: *const bpf_prog,
    ) -> ::core::ffi::c_int;

    pub fn bpf_lsm_is_sleepable_hook(btf_id: u32) -> bool;
    pub fn bpf_lsm_is_trusted(prog: *const bpf_prog) -> bool;

    pub static bpf_inode_storage_get_proto: bpf_func_proto;
    pub static bpf_inode_storage_delete_proto: bpf_func_proto;
    pub fn bpf_inode_storage_free(inode: *mut inode);

    pub fn bpf_lsm_find_cgroup_shim(prog: *const bpf_prog, bpf_func: *mut bpf_func_t);

    pub fn bpf_lsm_get_retval_range(
        prog: *const bpf_prog,
        range: *mut bpf_retval_range,
    ) -> ::core::ffi::c_int;
    pub fn bpf_set_dentry_xattr_locked(
        dentry: *mut dentry,
        name__str: *const ::core::ffi::c_char,
        value_p: *const bpf_dynptr,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn bpf_remove_dentry_xattr_locked(
        dentry: *mut dentry,
        name__str: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn bpf_lsm_has_d_inode_locked(prog: *const bpf_prog) -> bool;
    pub fn bpf_lsm_hook_returns_errno(btf_id: u32) -> bool;
}

#[cfg(feature = "CONFIG_BPF_LSM")]
#[inline]
pub unsafe fn bpf_inode(inode: *const inode) -> *mut bpf_storage_blob {
    if (*inode).i_security.is_null() {
        return core::ptr::null_mut();
    }

    (*inode).i_security.add((*core::ptr::addr_of!(bpf_lsm_blob_sizes)).lbs_inode)
        as *mut bpf_storage_blob
}

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
pub const bpf_lsm_initialized: bool = false;

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_lsm_is_sleepable_hook(_btf_id: u32) -> bool { false }

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_lsm_is_trusted(_prog: *const bpf_prog) -> bool { false }

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_lsm_verify_prog(
    _vlog: *mut bpf_verifier_log,
    _prog: *const bpf_prog,
) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_inode(_inode: *const inode) -> *mut bpf_storage_blob {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_inode_storage_free(_inode: *mut inode) {}

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_lsm_find_cgroup_shim(_prog: *const bpf_prog, _bpf_func: *mut bpf_func_t) {}

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_lsm_get_retval_range(
    _prog: *const bpf_prog,
    _range: *mut bpf_retval_range,
) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_set_dentry_xattr_locked(
    _dentry: *mut dentry,
    _name__str: *const ::core::ffi::c_char,
    _value_p: *const bpf_dynptr,
    _flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_remove_dentry_xattr_locked(
    _dentry: *mut dentry,
    _name__str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_lsm_has_d_inode_locked(_prog: *const bpf_prog) -> bool { false }

#[cfg(not(feature = "CONFIG_BPF_LSM"))]
#[inline]
pub unsafe fn bpf_lsm_hook_returns_errno(_btf_id: u32) -> bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
