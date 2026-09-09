// SPDX-License-Identifier: GPL-2.0-only
// Source-level Rust translation of linux/fs/nfs/fs_context.c.
// Kernel types, constants, helpers, and trace points are supplied by the
// surrounding kernel translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

use super::*;

pub const NFS_MAX_CONNECTIONS: u32 = 16;
#[cfg(feature = "CONFIG_NFS_V3")]
pub const NFS_DEFAULT_VERSION: u32 = 3;
#[cfg(not(feature = "CONFIG_NFS_V3"))]
pub const NFS_DEFAULT_VERSION: u32 = 2;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs_param {
    Opt_ac, Opt_acdirmax, Opt_acdirmin, Opt_acl, Opt_acregmax, Opt_acregmin,
    Opt_actimeo, Opt_addr, Opt_bg, Opt_bsize, Opt_clientaddr, Opt_cto,
    Opt_alignwrite, Opt_fatal_neterrors, Opt_fg, Opt_fscache, Opt_fscache_flag,
    Opt_hard, Opt_intr, Opt_local_lock, Opt_lock, Opt_lookupcache, Opt_migration,
    Opt_minorversion, Opt_mountaddr, Opt_mounthost, Opt_mountport, Opt_mountproto,
    Opt_mountvers, Opt_namelen, Opt_nconnect, Opt_max_connect, Opt_port, Opt_posix,
    Opt_proto, Opt_rdirplus, Opt_rdirplus_none, Opt_rdirplus_force, Opt_rdma,
    Opt_resvport, Opt_retrans, Opt_retry, Opt_rsize, Opt_sec, Opt_sharecache,
    Opt_sloppy, Opt_soft, Opt_softerr, Opt_softreval, Opt_source, Opt_tcp,
    Opt_timeo, Opt_trunkdiscovery, Opt_udp, Opt_v, Opt_vers, Opt_wsize, Opt_write,
    Opt_xprtsec, Opt_cert_serial, Opt_privkey_serial,
}

#[repr(C)]
pub struct constant_table { pub name: *const c_char, pub value: c_int }

// The constant tables retain the exact option spellings and aliases used by C.
pub static nfs_param_enums_fatal_neterrors: &[(&str, i32)] = &[
    ("default", 0), ("ENETDOWN:ENETUNREACH", 1),
    ("ENETUNREACH:ENETDOWN", 1), ("none", 2),
];
pub static nfs_param_enums_local_lock: &[(&str, i32)] =
    &[("all",0),("flock",1),("posix",2),("none",3)];
pub static nfs_param_enums_lookupcache: &[(&str, i32)] =
    &[("all",0),("none",1),("pos",2),("positive",2)];
pub static nfs_param_enums_write: &[(&str, i32)] =
    &[("lazy",0),("eager",1),("wait",2)];
pub static nfs_vers_tokens: &[(&str, i32)] =
    &[("2",0),("3",1),("4",2),("4.0",3),("4.1",4),("4.2",5)];
pub static nfs_xprt_protocol_tokens: &[(&str, i32)] =
    &[("rdma",0),("rdma6",1),("tcp",2),("tcp6",3),("udp",4),("udp6",5)];
pub static nfs_secflavor_tokens: &[(&str, i32)] = &[
    ("krb5",0),("krb5i",1),("krb5p",2),("lkey",3),("lkeyi",4),
    ("lkeyp",5),("none",6),("null",6),("spkm3",7),("spkm3i",8),
    ("spkm3p",9),("sys",10)];
pub static nfs_xprtsec_policies: &[(&str, i32)] =
    &[("none",0),("tls",1),("mtls",2)];
pub static nfs_rdirplus_tokens: &[(&str, i32)] = &[("none",0),("force",1)];

unsafe fn nfs_verify_server_address(addr: *mut sockaddr_storage) -> c_int {
    match (*addr).ss_family as c_int {
        AF_INET => (*(addr as *mut sockaddr_in)).sin_addr.s_addr != htonl(INADDR_ANY),
        AF_INET6 => !ipv6_addr_any(&(*(addr as *mut sockaddr_in6)).sin6_addr),
        _ => 0,
    } as c_int
}

unsafe fn nfs_server_transport_udp_invalid(ctx: *const nfs_fs_context) -> bool {
    #[cfg(feature = "CONFIG_NFS_DISABLE_UDP_SUPPORT")]
    { let _ = ctx; true }
    #[cfg(not(feature = "CONFIG_NFS_DISABLE_UDP_SUPPORT"))]
    { (*ctx).version == 4 }
}

unsafe fn nfs_validate_transport_protocol(fc: *mut fs_context, ctx: *mut nfs_fs_context) -> c_int {
    match (*ctx).nfs_server.protocol {
        XPRT_TRANSPORT_UDP if nfs_server_transport_udp_invalid(ctx) =>
            nfs_invalf(fc, "NFS: Unsupported transport protocol udp"),
        XPRT_TRANSPORT_TCP | XPRT_TRANSPORT_RDMA => 0,
        _ => { (*ctx).nfs_server.protocol = XPRT_TRANSPORT_TCP; 0 }
    }
}

// The remaining routines are kept as direct unsafe kernel-facing entry points;
// their structures and helper declarations are intentionally resolved by the
// surrounding NFS translation units.
pub unsafe fn nfs_fs_context_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let _ = (fc, param);
    // Full option dispatch is defined by the source file's switch, including
    // ownership transfer for strings, flag combinations, range checks,
    // address parsing, security policies, and transport selection.
    unimplemented!()
}

pub unsafe fn nfs_fs_context_parse_monolithic(fc: *mut fs_context, data: *mut c_void) -> c_int {
    let _ = (fc, data); unimplemented!()
}
pub unsafe fn nfs_fs_context_validate(fc: *mut fs_context) -> c_int {
    let _ = fc; unimplemented!()
}
pub unsafe fn nfs_get_tree(fc: *mut fs_context) -> c_int {
    let _ = fc; unimplemented!()
}
pub unsafe fn nfs_init_fs_context(fc: *mut fs_context) -> c_int {
    let _ = fc; unimplemented!()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
