/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * FS Locations
 */

pub const MAX_FS_LOCATIONS: usize = 128;

#[repr(C)]
pub struct nfsd4_fs_location {
    pub hosts: *mut core::ffi::c_char, /* colon separated list of hosts */
    pub path: *mut core::ffi::c_char,  /* slash separated list of path components */
}

#[repr(C)]
pub struct nfsd4_fs_locations {
    pub locations_count: u32,
    pub locations: *mut nfsd4_fs_location,
    /* If we're not actually serving this data ourselves (only providing a
     * list of replicas that do serve it) then we set "migrated": */
    pub migrated: core::ffi::c_int,
}

/*
 * We keep an array of pseudoflavors with the export, in order from most
 * to least preferred.  For the foreseeable future, we don't expect more
 * than the eight pseudoflavors null, unix, krb5, krb5i, krb5p, skpm3,
 * spkm3i, and spkm3p (and using all 8 at once should be rare).
 */
pub const MAX_SECINFO_LIST: usize = 8;
pub const EX_UUID_LEN: usize = 16;

#[repr(C)]
pub struct exp_flavor_info {
    pub pseudoflavor: u32,
    pub flags: u32,
}

/* Per-export stats */
pub const EXP_STATS_FH_STALE: usize = 0;
pub const EXP_STATS_IO_READ: usize = 1;
pub const EXP_STATS_IO_WRITE: usize = 2;
pub const EXP_STATS_COUNTERS_NUM: usize = 3;

#[repr(C)]
pub struct export_stats {
    pub start_time: time64_t,
    pub counter: [percpu_counter; EXP_STATS_COUNTERS_NUM],
}

#[repr(C)]
pub struct svc_export {
    pub h: cache_head,
    pub ex_client: *mut auth_domain,
    pub ex_flags: core::ffi::c_int,
    pub ex_fsid: core::ffi::c_int,
    pub ex_path: path,
    pub ex_anon_uid: kuid_t,
    pub ex_anon_gid: kgid_t,
    pub ex_uuid: *mut u8, /* 16 byte fsid */
    pub ex_fslocs: nfsd4_fs_locations,
    pub ex_nflavors: u32,
    pub ex_flavors: [exp_flavor_info; MAX_SECINFO_LIST],
    pub ex_layout_types: u32,
    pub ex_devid_map: *mut nfsd4_deviceid_map,
    pub cd: *mut cache_detail,
    pub ex_rcu: rcu_head,
    pub ex_xprtsec_modes: usize,
    pub ex_stats: *mut export_stats,
}

/* an "export key" (expkey) maps a filehandlefragement to an
 * svc_export for a given client.  There can be several per export,
 * for the different fsid types.
 */
#[repr(C)]
pub struct svc_expkey {
    pub h: cache_head,
    pub ek_client: *mut auth_domain,
    pub ek_fsidtype: u8,
    pub ek_fsid: [u32; 6],
    pub ek_path: path,
    pub ek_rcu: rcu_head,
}

#[inline]
pub unsafe fn EX_ISSYNC(exp: *const svc_export) -> bool {
    !((*exp).ex_flags & NFSEXP_ASYNC != 0)
}

#[inline]
pub unsafe fn EX_NOHIDE(exp: *const svc_export) -> core::ffi::c_int {
    (*exp).ex_flags & NFSEXP_NOHIDE
}

#[inline]
pub unsafe fn EX_WGATHER(exp: *const svc_export) -> core::ffi::c_int {
    (*exp).ex_flags & NFSEXP_GATHERED_WRITES
}

extern "C" {
    pub fn nfsexp_flags(cred: *mut svc_cred, exp: *mut svc_export) -> core::ffi::c_int;
    pub fn check_xprtsec_policy(exp: *mut svc_export, rqstp: *mut svc_rqst) -> __be32;
    pub fn check_security_flavor(
        exp: *mut svc_export,
        rqstp: *mut svc_rqst,
        may_bypass_gss: bool,
    ) -> __be32;
    pub fn check_nfsd_access(
        exp: *mut svc_export,
        rqstp: *mut svc_rqst,
        may_bypass_gss: bool,
    ) -> __be32;

    /* Function declarations */
    pub fn nfsd_export_init(net: *mut net) -> core::ffi::c_int;
    pub fn nfsd_export_shutdown(net: *mut net);
    pub fn nfsd_export_flush(net: *mut net);
    pub fn rqst_exp_get_by_name(rqstp: *mut svc_rqst, path: *const path) -> *mut svc_export;
    pub fn rqst_exp_parent(rqstp: *mut svc_rqst, path: *mut path) -> *mut svc_export;
    pub fn rqst_find_fsidzero_export(rqstp: *mut svc_rqst) -> *mut svc_export;
    pub fn exp_rootfh(
        net: *mut net,
        cl: *mut auth_domain,
        path: *mut core::ffi::c_char,
        fh: *mut knfsd_fh,
        maxsize: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn exp_pseudoroot(rqstp: *mut svc_rqst, fh: *mut svc_fh) -> __be32;
    pub fn rqst_exp_find(
        reqp: *mut cache_req,
        net: *mut net,
        cl: *mut auth_domain,
        gsscl: *mut auth_domain,
        fsid_type: core::ffi::c_int,
        fsidv: *mut u32,
    ) -> *mut svc_export;
}

#[inline]
pub unsafe fn exp_put(exp: *mut svc_export) {
    cache_put(&mut (*exp).h, (*exp).cd);
}

#[inline]
pub unsafe fn exp_get(exp: *mut svc_export) -> *mut svc_export {
    cache_get(&mut (*exp).h);
    exp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
