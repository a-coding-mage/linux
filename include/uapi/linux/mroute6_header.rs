/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the Linux UAPI mroute6.h header.
// Dependencies supplied by the surrounding UAPI translation are intentionally
// referenced but not redefined here.

/*
 * Based on the MROUTING 3.5 defines primarily to keep source compatibility
 * with BSD. See the pim6sd code for the original history.
 * Protocol Independent Multicast (PIM) data structures included
 * Carlos Picoto (cap@di.fc.ul.pt)
 */

pub const MRT6_BASE: i32 = 200;
pub const MRT6_INIT: i32 = MRT6_BASE;
pub const MRT6_DONE: i32 = MRT6_BASE + 1;
pub const MRT6_ADD_MIF: i32 = MRT6_BASE + 2;
pub const MRT6_DEL_MIF: i32 = MRT6_BASE + 3;
pub const MRT6_ADD_MFC: i32 = MRT6_BASE + 4;
pub const MRT6_DEL_MFC: i32 = MRT6_BASE + 5;
pub const MRT6_VERSION: i32 = MRT6_BASE + 6;
pub const MRT6_ASSERT: i32 = MRT6_BASE + 7;
pub const MRT6_PIM: i32 = MRT6_BASE + 8;
pub const MRT6_TABLE: i32 = MRT6_BASE + 9;
pub const MRT6_ADD_MFC_PROXY: i32 = MRT6_BASE + 10;
pub const MRT6_DEL_MFC_PROXY: i32 = MRT6_BASE + 11;
pub const MRT6_FLUSH: i32 = MRT6_BASE + 12;
pub const MRT6_MAX: i32 = MRT6_BASE + 12;

// SIOCPROTOPRIVATE is supplied by linux/sockios.h.
pub const SIOCGETMIFCNT_IN6: i32 = SIOCPROTOPRIVATE;
pub const SIOCGETSGCNT_IN6: i32 = SIOCPROTOPRIVATE + 1;
pub const SIOCGETRPF: i32 = SIOCPROTOPRIVATE + 2;

pub const MRT6_FLUSH_MFC: u32 = 1;
pub const MRT6_FLUSH_MFC_STATIC: u32 = 2;
pub const MRT6_FLUSH_MIFS: u32 = 4;
pub const MRT6_FLUSH_MIFS_STATIC: u32 = 8;

pub const MAXMIFS: usize = 32;
pub type mifbitmap_t = usize;
pub type mifi_t = u16;
pub const ALL_MIFS: mifi_t = mifi_t::MAX;

pub const IF_SETSIZE: usize = 256;
pub type if_mask = u32;
pub const NIFBITS: usize = core::mem::size_of::<if_mask>() * 8;

#[repr(C)]
pub struct if_set {
    pub ifs_bits: [if_mask; (IF_SETSIZE + NIFBITS - 1) / NIFBITS],
}

#[macro_export]
macro_rules! IF_SET {
    ($n:expr, $p:expr) => {
        ($p).ifs_bits[($n) / $crate::NIFBITS] |= 1 << (($n) % $crate::NIFBITS)
    };
}
#[macro_export]
macro_rules! IF_CLR {
    ($n:expr, $p:expr) => {
        ($p).ifs_bits[($n) / $crate::NIFBITS] &= !(1 << (($n) % $crate::NIFBITS))
    };
}
#[macro_export]
macro_rules! IF_ISSET {
    ($n:expr, $p:expr) => {
        ($p).ifs_bits[($n) / $crate::NIFBITS] & (1 << (($n) % $crate::NIFBITS))
    };
}
// IF_COPY and IF_ZERO depend on the C bcopy/bzero interfaces.

#[repr(C)]
pub struct mif6ctl {
    pub mif6c_mifi: mifi_t,
    pub mif6c_flags: u8,
    pub vifc_threshold: u8,
    pub mif6c_pifi: u16,
    pub vifc_rate_limit: u32,
}

pub const MIFF_REGISTER: u32 = 0x1;

#[repr(C)]
pub struct mf6cctl {
    pub mf6cc_origin: sockaddr_in6,
    pub mf6cc_mcastgrp: sockaddr_in6,
    pub mf6cc_parent: mifi_t,
    pub mf6cc_ifset: if_set,
}

#[repr(C)]
pub struct sioc_sg_req6 {
    pub src: sockaddr_in6,
    pub grp: sockaddr_in6,
    pub pktcnt: usize,
    pub bytecnt: usize,
    pub wrong_if: usize,
}

#[repr(C)]
pub struct sioc_mif_req6 {
    pub mifi: mifi_t,
    pub icount: usize,
    pub ocount: usize,
    pub ibytes: usize,
    pub obytes: usize,
}

#[repr(C)]
pub struct mrt6msg {
    pub im6_mbz: u8,
    pub im6_msgtype: u8,
    pub im6_mif: u16,
    pub im6_pad: u32,
    pub im6_src: in6_addr,
    pub im6_dst: in6_addr,
}

pub const MRT6MSG_NOCACHE: u32 = 1;
pub const MRT6MSG_WRONGMIF: u32 = 2;
pub const MRT6MSG_WHOLEPKT: u32 = 3;
pub const MRT6MSG_WRMIFWHOLE: u32 = 4;

pub const IP6MRA_CREPORT_UNSPEC: u32 = 0;
pub const IP6MRA_CREPORT_MSGTYPE: u32 = 1;
pub const IP6MRA_CREPORT_MIF_ID: u32 = 2;
pub const IP6MRA_CREPORT_SRC_ADDR: u32 = 3;
pub const IP6MRA_CREPORT_DST_ADDR: u32 = 4;
pub const IP6MRA_CREPORT_PKT: u32 = 5;
pub const __IP6MRA_CREPORT_MAX: u32 = 6;
pub const IP6MRA_CREPORT_MAX: u32 = __IP6MRA_CREPORT_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
