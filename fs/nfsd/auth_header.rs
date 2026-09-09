/* SPDX-License-Identifier: GPL-2.0 */
/*
 * nfsd-specific authentication stuff.
 *
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 */

// Forward declarations corresponding to the C declarations.
#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct svc_export {
    _private: [u8; 0],
}

#[repr(C)]
pub struct svc_rqst {
    _private: [u8; 0],
}

#[repr(C)]
pub struct svc_cred {
    _private: [u8; 0],
}

/*
 * Set the current process's fsuid/fsgid etc to those of the NFS
 * client user
 */
unsafe extern "C" {
    pub fn nfsd_setuser(cred: *mut svc_cred, exp: *mut svc_export) -> ::core::ffi::c_int;

    pub fn nfsd_user_namespace(rqstp: *const svc_rqst) -> *mut user_namespace;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
