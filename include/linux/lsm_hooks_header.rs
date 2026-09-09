/*
 * Linux Security Module interfaces
 *
 * Copyright (C) 2001 WireX Communications, Inc <chris@wirex.com>
 * Copyright (C) 2001 Greg Kroah-Hartman <greg@kroah.com>
 * Copyright (C) 2001 Networks Associates Technology, Inc <ssmalley@nai.com>
 * Copyright (C) 2001 James Morris <jmorris@intercode.com.au>
 * Copyright (C) 2001 Silicon Graphics, Inc. (Trust Technology Group)
 * Copyright (C) 2015 Intel Corporation.
 * Copyright (C) 2015 Casey Schaufler <casey@schaufler-ca.com>
 * Copyright (C) 2016 Mellanox Techonologies
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

// C dependencies: uapi/linux/lsm.h, linux/security.h, linux/init.h,
// linux/rculist.h, linux/xattr.h, linux/static_call.h, linux/unroll.h,
// linux/jump_label.h, linux/lsm_count.h.

// The C union contains one function-pointer member for each LSM_HOOK in
// lsm_hook_defs.h. Those externally generated members are intentionally
// represented by the raw address member here; dependent code supplies the
// hook-specific declarations.
#[repr(C)]
pub union security_list_options {
    pub lsm_func_addr: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct lsm_static_call {
    pub key: *mut static_call_key,
    pub trampoline: *mut core::ffi::c_void,
    pub hl: *mut security_hook_list,
    pub active: *mut static_key_false,
}

#[repr(C, packed)]
pub struct lsm_static_calls_table {
    // C expands one field per LSM_HOOK as `NAME: [lsm_static_call; MAX_LSM_COUNT]`.
}

#[repr(C)]
pub struct lsm_id {
    pub name: *const core::ffi::c_char,
    pub id: u64,
}

#[repr(C)]
pub struct security_hook_list {
    pub scalls: *mut lsm_static_call,
    pub hook: security_list_options,
    pub lsmid: *const lsm_id,
}

#[repr(C)]
pub struct lsm_blob_sizes {
    pub lbs_cred: core::ffi::c_uint,
    pub lbs_file: core::ffi::c_uint,
    pub lbs_backing_file: core::ffi::c_uint,
    pub lbs_ib: core::ffi::c_uint,
    pub lbs_inode: core::ffi::c_uint,
    pub lbs_sock: core::ffi::c_uint,
    pub lbs_superblock: core::ffi::c_uint,
    pub lbs_ipc: core::ffi::c_uint,
    pub lbs_key: core::ffi::c_uint,
    pub lbs_msg_msg: core::ffi::c_uint,
    pub lbs_perf_event: core::ffi::c_uint,
    pub lbs_task: core::ffi::c_uint,
    pub lbs_xattr_count: core::ffi::c_uint,
    pub lbs_tun_dev: core::ffi::c_uint,
    pub lbs_bdev: core::ffi::c_uint,
    pub lbs_bpf_map: core::ffi::c_uint,
    pub lbs_bpf_prog: core::ffi::c_uint,
    pub lbs_bpf_token: core::ffi::c_uint,
}

// LSM_RET_VOID is the default value for void LSM hooks.
pub const LSM_RET_VOID: () = ();

// LSM_HOOK_INIT(NAME, HOOK) initializes the common security hook-list fields.
// The C designated-union initializer is expressed at each call site in Rust.

extern "C" {
    pub fn security_add_hooks(
        hooks: *mut security_hook_list,
        count: core::ffi::c_int,
        lsmid: *const lsm_id,
    );
}

pub const LSM_FLAG_LEGACY_MAJOR: core::ffi::c_ulong = 1u64 << 0;
pub const LSM_FLAG_EXCLUSIVE: core::ffi::c_ulong = 1u64 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lsm_order {
    LSM_ORDER_FIRST = -1,
    LSM_ORDER_MUTABLE = 0,
    LSM_ORDER_LAST = 1,
}

#[repr(C)]
pub struct lsm_info {
    pub id: *const lsm_id,
    pub order: lsm_order,
    pub flags: core::ffi::c_ulong,
    pub blobs: *mut lsm_blob_sizes,
    pub enabled: *mut core::ffi::c_int,
    pub init: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub initcall_pure: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub initcall_early: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub initcall_core: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub initcall_subsys: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub initcall_fs: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub initcall_device: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub initcall_late: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub initcall_late_sync: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
}

// DEFINE_LSM and DEFINE_EARLY_LSM emit static lsm_info objects into linker
// sections; their section/alignment attributes are supplied by the build.

// DO NOT tamper with this variable outside of the LSM framework.
extern "C" {
    pub static mut static_calls_table: lsm_static_calls_table;
}

#[inline]
pub unsafe fn lsm_get_xattr_slot(
    xattrs: *mut xattr,
    xattr_count: *mut core::ffi::c_int,
) -> *mut xattr {
    if xattrs.is_null() {
        return core::ptr::null_mut();
    }
    let slot = xattrs.offset(*xattr_count as isize);
    *xattr_count += 1;
    slot
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
