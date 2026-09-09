/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2016 Tom Haynes <loghyr@primarydata.com>
 */

// Dependency intent: the original header includes <linux/inet.h> and "xdr4.h".

pub const FF_FLAGS_NO_LAYOUTCOMMIT: u32 = 1;
pub const FF_FLAGS_NO_IO_THRU_MDS: u32 = 2;
pub const FF_FLAGS_NO_READ_IO: u32 = 4;

// Opaque declaration corresponding to `struct xdr_stream`.
#[repr(C)]
pub struct xdr_stream {
    _private: [u8; 0],
}

pub const FF_NETID_LEN: usize = 4;
pub const FF_ADDR_LEN: usize = INET6_ADDRSTRLEN + 8;

#[repr(C)]
pub struct pnfs_ff_netaddr {
    pub netid: [core::ffi::c_char; FF_NETID_LEN + 1],
    pub addr: [core::ffi::c_char; FF_ADDR_LEN + 1],
    pub netid_len: u32,
    pub addr_len: u32,
}

#[repr(C)]
pub struct pnfs_ff_device_addr {
    pub netaddr: pnfs_ff_netaddr,
    pub version: u32,
    pub minor_version: u32,
    pub rsize: u32,
    pub wsize: u32,
    pub tightly_coupled: bool,
}

#[repr(C)]
pub struct pnfs_ff_layout {
    pub flags: u32,
    pub stats_collect_hint: u32,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub deviceid: nfsd4_deviceid,
    pub stateid: stateid_t,
    pub fh: nfs_fh,
}

extern "C" {
    pub fn nfsd4_ff_encode_getdeviceinfo(
        xdr: *mut xdr_stream,
        gdp: *const nfsd4_getdeviceinfo,
    ) -> __be32;
    pub fn nfsd4_ff_encode_layoutget(
        xdr: *mut xdr_stream,
        lgp: *const nfsd4_layoutget,
    ) -> __be32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
