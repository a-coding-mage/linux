/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ppp_defs.h - PPP definitions.
 *
 * Copyright 1994-2000 Paul Mackerras.
 *
 *  This program is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU General Public License
 *  version 2 as published by the Free Software Foundation.
 */
// Dependency supplied by linux/types.h is referenced directly below.

/* The basic PPP frame. */
pub const PPP_HDRLEN: usize = 4; /* octets for standard ppp header */
pub const PPP_FCSLEN: usize = 2; /* octets for FCS */
pub const PPP_MRU: usize = 1500; /* default MRU = max length of info field */

#[macro_export]
macro_rules! PPP_ADDRESS {
    ($p:expr) => { unsafe { (*($p as *const __u8).add(0)) } };
}
#[macro_export]
macro_rules! PPP_CONTROL {
    ($p:expr) => { unsafe { (*($p as *const __u8).add(1)) } };
}
#[macro_export]
macro_rules! PPP_PROTOCOL {
    ($p:expr) => {
        (((unsafe { (*($p as *const __u8).add(2)) }) as u16) << 8)
            .wrapping_add(unsafe { (*($p as *const __u8).add(3)) } as u16)
    };
}

/* Significant octet values. */
pub const PPP_ALLSTATIONS: u8 = 0xff; /* All-Stations broadcast address */
pub const PPP_UI: u8 = 0x03; /* Unnumbered Information */
pub const PPP_FLAG: u8 = 0x7e; /* Flag Sequence */
pub const PPP_ESCAPE: u8 = 0x7d; /* Asynchronous Control Escape */
pub const PPP_TRANS: u8 = 0x20; /* Asynchronous transparency modifier */

/* Protocol field values. */
pub const PPP_IP: u16 = 0x21; /* Internet Protocol */
pub const PPP_AT: u16 = 0x29; /* AppleTalk Protocol */
pub const PPP_IPX: u16 = 0x2b; /* IPX protocol */
pub const PPP_VJC_COMP: u16 = 0x2d; /* VJ compressed TCP */
pub const PPP_VJC_UNCOMP: u16 = 0x2f; /* VJ uncompressed TCP */
pub const PPP_MP: u16 = 0x3d; /* Multilink protocol */
pub const PPP_IPV6: u16 = 0x57; /* Internet Protocol Version 6 */
pub const PPP_COMPFRAG: u16 = 0xfb; /* fragment compressed below bundle */
pub const PPP_COMP: u16 = 0xfd; /* compressed packet */
pub const PPP_MPLS_UC: u16 = 0x0281; /* Multi Protocol Label Switching - Unicast */
pub const PPP_MPLS_MC: u16 = 0x0283; /* Multi Protocol Label Switching - Multicast */
pub const PPP_IPCP: u16 = 0x8021; /* IP Control Protocol */
pub const PPP_ATCP: u16 = 0x8029; /* AppleTalk Control Protocol */
pub const PPP_IPXCP: u16 = 0x802b; /* IPX Control Protocol */
pub const PPP_IPV6CP: u16 = 0x8057; /* IPv6 Control Protocol */
pub const PPP_CCPFRAG: u16 = 0x80fb; /* CCP at link level (below MP bundle) */
pub const PPP_CCP: u16 = 0x80fd; /* Compression Control Protocol */
pub const PPP_MPLSCP: u16 = 0x80fd; /* MPLS Control Protocol */
pub const PPP_LCP: u16 = 0xc021; /* Link Control Protocol */
pub const PPP_PAP: u16 = 0xc023; /* Password Authentication Protocol */
pub const PPP_LQR: u16 = 0xc025; /* Link Quality Report protocol */
pub const PPP_CHAP: u16 = 0xc223; /* Cryptographic Handshake Auth. Protocol */
pub const PPP_CBCP: u16 = 0xc029; /* Callback Control Protocol */

/* Values for FCS calculations. */
pub const PPP_INITFCS: u16 = 0xffff; /* Initial FCS value */
pub const PPP_GOODFCS: u16 = 0xf0b8; /* Good final FCS value */

/* Extended asyncmap - allows any character to be escaped. */
pub type ext_accm = [__u32; 8];

/* What to do with network protocol (NP) packets. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NPmode {
    NPMODE_PASS, /* pass the packet through */
    NPMODE_DROP, /* silently drop the packet */
    NPMODE_ERROR, /* return an error */
    NPMODE_QUEUE, /* save it up for later. */
}

#[repr(C)]
pub struct pppstat {
    pub ppp_discards: __u32,
    pub ppp_ibytes: __u32,
    pub ppp_ioctects: __u32,
    pub ppp_ipackets: __u32,
    pub ppp_ierrors: __u32,
    pub ppp_ilqrs: __u32,
    pub ppp_obytes: __u32,
    pub ppp_ooctects: __u32,
    pub ppp_opackets: __u32,
    pub ppp_oerrors: __u32,
    pub ppp_olqrs: __u32,
}

#[repr(C)]
pub struct vjstat {
    pub vjs_packets: __u32,
    pub vjs_compressed: __u32,
    pub vjs_searches: __u32,
    pub vjs_misses: __u32,
    pub vjs_uncompressedin: __u32,
    pub vjs_compressedin: __u32,
    pub vjs_errorin: __u32,
    pub vjs_tossed: __u32,
}

#[repr(C)]
pub struct compstat {
    pub unc_bytes: __u32,
    pub unc_packets: __u32,
    pub comp_bytes: __u32,
    pub comp_packets: __u32,
    pub inc_bytes: __u32,
    pub inc_packets: __u32,
    pub in_count: __u32,
    pub bytes_out: __u32,
    pub ratio: f64,
}

#[repr(C)]
pub struct ppp_stats { pub p: pppstat, pub vj: vjstat }

#[repr(C)]
pub struct ppp_comp_stats { pub c: compstat, pub d: compstat }

#[repr(C)]
pub struct ppp_idle {
    pub xmit_idle: __kernel_old_time_t,
    pub recv_idle: __kernel_old_time_t,
}

#[repr(C)]
pub struct ppp_idle32 { pub xmit_idle: __s32, pub recv_idle: __s32 }

#[repr(C)]
pub struct ppp_idle64 { pub xmit_idle: __s64, pub recv_idle: __s64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
