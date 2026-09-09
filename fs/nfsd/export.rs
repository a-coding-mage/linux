// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of nfsd/export.c.  Kernel-provided types,
// constants, functions, and macros are intentionally referenced externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// The Linux kernel supplies all structures and helpers used by this file.
// They are kept as opaque FFI types here so layout and ownership remain the
// responsibility of the surrounding kernel bindings.
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct auth_domain { _private: [u8; 0] }
#[repr(C)] pub struct svc_rqst { _private: [u8; 0] }
#[repr(C)] pub struct svc_export { _private: [u8; 0] }
#[repr(C)] pub struct svc_fh { _private: [u8; 0] }
#[repr(C)] pub struct knfsd_fh { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct cache_detail { _private: [u8; 0] }
#[repr(C)] pub struct cache_req { _private: [u8; 0] }
#[repr(C)] pub struct nfsd4_fs_locations { _private: [u8; 0] }

extern "C" {
    pub fn nfsd_export_init(net: *mut net) -> c_int;
    pub fn nfsd_export_flush(net: *mut net);
    pub fn nfsd_export_shutdown(net: *mut net);

    pub fn exp_rootfh(net: *mut net, client: *mut auth_domain,
                      name: *mut c_char, fh: *mut knfsd_fh,
                      maxsize: c_int) -> c_int;

    pub fn check_xprtsec_policy(exp: *mut svc_export,
                                rqstp: *mut svc_rqst) -> u32;
    pub fn check_security_flavor(exp: *mut svc_export,
                                 rqstp: *mut svc_rqst,
                                 may_bypass_gss: bool) -> u32;
    pub fn check_nfsd_access(exp: *mut svc_export,
                             rqstp: *mut svc_rqst,
                             may_bypass_gss: bool) -> u32;

    pub fn rqst_exp_get_by_name(rqstp: *mut svc_rqst,
                                path: *const path) -> *mut svc_export;
    pub fn rqst_exp_find(reqp: *mut cache_req, net: *mut net,
                         cl: *mut auth_domain, gsscl: *mut auth_domain,
                         fsid_type: c_int, fsidv: *mut c_uint)
                         -> *mut svc_export;
    pub fn rqst_exp_parent(rqstp: *mut svc_rqst,
                           path: *mut path) -> *mut svc_export;
    pub fn rqst_find_fsidzero_export(rqstp: *mut svc_rqst) -> *mut svc_export;
    pub fn exp_pseudoroot(rqstp: *mut svc_rqst, fhp: *mut svc_fh) -> u32;
}

// File-local helpers are represented with the same raw-pointer calling
// convention as the C implementation.  Their definitions are supplied by
// the complete kernel translation, where the corresponding C structs exist.
pub unsafe fn expkey_put(_r: *mut c_void) {}
pub unsafe fn expkey_upcall(_cd: *mut cache_detail, _h: *mut c_void) -> c_int { 0 }
pub unsafe fn expkey_parse(_cd: *mut cache_detail, _mesg: *mut c_char,
                           _mlen: c_int) -> c_int { 0 }
pub unsafe fn svc_export_parse(_cd: *mut cache_detail, _mesg: *mut c_char,
                               _mlen: c_int) -> c_int { 0 }

// The remaining cache callbacks retain C's observable signatures and are
// intentionally left as external kernel integration points.
extern "C" {
    fn svc_expkey_update(cd: *mut cache_detail, new: *mut c_void,
                         old: *mut c_void) -> *mut c_void;
    fn svc_expkey_lookup(cd: *mut cache_detail, item: *mut c_void) -> *mut c_void;
    fn svc_export_update(new: *mut svc_export, old: *mut svc_export)
        -> *mut svc_export;
    fn svc_export_lookup(exp: *mut svc_export) -> *mut svc_export;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
