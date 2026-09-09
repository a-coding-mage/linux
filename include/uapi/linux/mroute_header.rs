/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the Linux UAPI mroute.h header. */

use core::ffi::{c_int, c_ulong};

pub const MRT_BASE: c_int = 200;
pub const MRT_INIT: c_int = MRT_BASE;
pub const MRT_DONE: c_int = MRT_BASE + 1;
pub const MRT_ADD_VIF: c_int = MRT_BASE + 2;
pub const MRT_DEL_VIF: c_int = MRT_BASE + 3;
pub const MRT_ADD_MFC: c_int = MRT_BASE + 4;
pub const MRT_DEL_MFC: c_int = MRT_BASE + 5;
pub const MRT_VERSION: c_int = MRT_BASE + 6;
pub const MRT_ASSERT: c_int = MRT_BASE + 7;
pub const MRT_PIM: c_int = MRT_BASE + 8;
pub const MRT_TABLE: c_int = MRT_BASE + 9;
pub const MRT_ADD_MFC_PROXY: c_int = MRT_BASE + 10;
pub const MRT_DEL_MFC_PROXY: c_int = MRT_BASE + 11;
pub const MRT_FLUSH: c_int = MRT_BASE + 12;
pub const MRT_MAX: c_int = MRT_BASE + 12;

/* SIOCPROTOPRIVATE is supplied by linux/sockios.h. */
pub const SIOCGETVIFCNT: c_int = SIOCPROTOPRIVATE;
pub const SIOCGETSGCNT: c_int = SIOCPROTOPRIVATE + 1;
pub const SIOCGETRPF: c_int = SIOCPROTOPRIVATE + 2;

pub const MRT_FLUSH_MFC: c_int = 1;
pub const MRT_FLUSH_MFC_STATIC: c_int = 2;
pub const MRT_FLUSH_VIFS: c_int = 4;
pub const MRT_FLUSH_VIFS_STATIC: c_int = 8;

pub const MAXVIFS: usize = 32;
pub type vifbitmap_t = c_ulong;
pub type vifi_t = u16;
pub const ALL_VIFS: vifi_t = !0;

#[inline]
pub fn VIFM_SET(n: usize, m: &mut vifbitmap_t) { *m |= (1 as vifbitmap_t).wrapping_shl(n as u32); }
#[inline]
pub fn VIFM_CLR(n: usize, m: &mut vifbitmap_t) { *m &= !(1 as vifbitmap_t).wrapping_shl(n as u32); }
#[inline]
pub fn VIFM_ISSET(n: usize, m: vifbitmap_t) -> vifbitmap_t { m & (1 as vifbitmap_t).wrapping_shl(n as u32) }
#[inline]
pub fn VIFM_CLRALL(m: &mut vifbitmap_t) { *m = 0; }
#[inline]
pub fn VIFM_COPY(mfrom: vifbitmap_t, mto: &mut vifbitmap_t) { *mto = mfrom; }
#[inline]
pub fn VIFM_SAME(m1: vifbitmap_t, m2: vifbitmap_t) -> bool { m1 == m2 }

#[repr(C)]
pub union vifctl_lcl_addr {
    pub vifc_lcl_addr: in_addr,
    pub vifc_lcl_ifindex: c_int,
}

#[repr(C)]
pub struct vifctl {
    pub vifc_vifi: vifi_t,
    pub vifc_flags: u8,
    pub vifc_threshold: u8,
    pub vifc_rate_limit: u32,
    pub vifc_lcl_addr: vifctl_lcl_addr,
    pub vifc_rmt_addr: in_addr,
}

pub const VIFF_TUNNEL: u32 = 0x1;
pub const VIFF_SRCRT: u32 = 0x2;
pub const VIFF_REGISTER: u32 = 0x4;
pub const VIFF_USE_IFINDEX: u32 = 0x8;

#[repr(C)]
pub struct mfcctl {
    pub mfcc_origin: in_addr,
    pub mfcc_mcastgrp: in_addr,
    pub mfcc_parent: vifi_t,
    pub mfcc_ttls: [u8; MAXVIFS],
    pub mfcc_pkt_cnt: u32,
    pub mfcc_byte_cnt: u32,
    pub mfcc_wrong_if: u32,
    pub mfcc_expire: c_int,
}

#[repr(C)]
pub struct sioc_sg_req {
    pub src: in_addr,
    pub grp: in_addr,
    pub pktcnt: c_ulong,
    pub bytecnt: c_ulong,
    pub wrong_if: c_ulong,
}

#[repr(C)]
pub struct sioc_vif_req {
    pub vifi: vifi_t,
    pub icount: c_ulong,
    pub ocount: c_ulong,
    pub ibytes: c_ulong,
    pub obytes: c_ulong,
}

#[repr(C)]
pub struct igmpmsg {
    pub unused1: __u32,
    pub unused2: __u32,
    pub im_msgtype: u8,
    pub im_mbz: u8,
    pub im_vif: u8,
    pub im_vif_hi: u8,
    pub im_src: in_addr,
    pub im_dst: in_addr,
}

pub const IPMRA_TABLE_UNSPEC: c_int = 0;
pub const IPMRA_TABLE_ID: c_int = 1;
pub const IPMRA_TABLE_CACHE_RES_QUEUE_LEN: c_int = 2;
pub const IPMRA_TABLE_MROUTE_REG_VIF_NUM: c_int = 3;
pub const IPMRA_TABLE_MROUTE_DO_ASSERT: c_int = 4;
pub const IPMRA_TABLE_MROUTE_DO_PIM: c_int = 5;
pub const IPMRA_TABLE_VIFS: c_int = 6;
pub const IPMRA_TABLE_MROUTE_DO_WRVIFWHOLE: c_int = 7;
pub const __IPMRA_TABLE_MAX: c_int = 8;
pub const IPMRA_TABLE_MAX: c_int = __IPMRA_TABLE_MAX - 1;

pub const IPMRA_VIF_UNSPEC: c_int = 0;
pub const IPMRA_VIF: c_int = 1;
pub const __IPMRA_VIF_MAX: c_int = 2;
pub const IPMRA_VIF_MAX: c_int = __IPMRA_VIF_MAX - 1;

pub const IPMRA_VIFA_UNSPEC: c_int = 0;
pub const IPMRA_VIFA_IFINDEX: c_int = 1;
pub const IPMRA_VIFA_VIF_ID: c_int = 2;
pub const IPMRA_VIFA_FLAGS: c_int = 3;
pub const IPMRA_VIFA_BYTES_IN: c_int = 4;
pub const IPMRA_VIFA_BYTES_OUT: c_int = 5;
pub const IPMRA_VIFA_PACKETS_IN: c_int = 6;
pub const IPMRA_VIFA_PACKETS_OUT: c_int = 7;
pub const IPMRA_VIFA_LOCAL_ADDR: c_int = 8;
pub const IPMRA_VIFA_REMOTE_ADDR: c_int = 9;
pub const IPMRA_VIFA_PAD: c_int = 10;
pub const __IPMRA_VIFA_MAX: c_int = 11;
pub const IPMRA_VIFA_MAX: c_int = __IPMRA_VIFA_MAX - 1;

pub const IPMRA_CREPORT_UNSPEC: c_int = 0;
pub const IPMRA_CREPORT_MSGTYPE: c_int = 1;
pub const IPMRA_CREPORT_VIF_ID: c_int = 2;
pub const IPMRA_CREPORT_SRC_ADDR: c_int = 3;
pub const IPMRA_CREPORT_DST_ADDR: c_int = 4;
pub const IPMRA_CREPORT_PKT: c_int = 5;
pub const IPMRA_CREPORT_TABLE: c_int = 6;
pub const __IPMRA_CREPORT_MAX: c_int = 7;
pub const IPMRA_CREPORT_MAX: c_int = __IPMRA_CREPORT_MAX - 1;

/* HZ is supplied by the kernel headers/build configuration. */
pub const MFC_ASSERT_THRESH: c_int = 3 * HZ;

pub const IGMPMSG_NOCACHE: c_int = 1;
pub const IGMPMSG_WRONGVIF: c_int = 2;
pub const IGMPMSG_WHOLEPKT: c_int = 3;
pub const IGMPMSG_WRVIFWHOLE: c_int = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
