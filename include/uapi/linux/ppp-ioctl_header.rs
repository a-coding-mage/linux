/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ppp-ioctl.h - PPP ioctl definitions.
 *
 * Copyright 1999-2002 Paul Mackerras.
 *
 *  This program is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU General Public License
 *  version 2 as published by the Free Software Foundation.
 */

/* C dependencies: linux/types.h, linux/compiler.h, linux/ppp_defs.h. */

/* Bit definitions for flags argument to PPPIOCGFLAGS/PPPIOCSFLAGS. */
pub const SC_COMP_PROT: u32 = 0x00000001; /* protocol compression (output) */
pub const SC_COMP_AC: u32 = 0x00000002; /* header compression (output) */
pub const SC_COMP_TCP: u32 = 0x00000004; /* TCP (VJ) compression (output) */
pub const SC_NO_TCP_CCID: u32 = 0x00000008; /* disable VJ connection-id comp. */
pub const SC_REJ_COMP_AC: u32 = 0x00000010; /* reject adrs/ctrl comp. on input */
pub const SC_REJ_COMP_TCP: u32 = 0x00000020; /* reject TCP (VJ) comp. on input */
pub const SC_CCP_OPEN: u32 = 0x00000040; /* Look at CCP packets */
pub const SC_CCP_UP: u32 = 0x00000080; /* May send/recv compressed packets */
pub const SC_ENABLE_IP: u32 = 0x00000100; /* IP packets may be exchanged */
pub const SC_LOOP_TRAFFIC: u32 = 0x00000200; /* send traffic to pppd */
pub const SC_MULTILINK: u32 = 0x00000400; /* do multilink encapsulation */
pub const SC_MP_SHORTSEQ: u32 = 0x00000800; /* use short MP sequence numbers */
pub const SC_COMP_RUN: u32 = 0x00001000; /* compressor has been inited */
pub const SC_DECOMP_RUN: u32 = 0x00002000; /* decompressor has been inited */
pub const SC_MP_XSHORTSEQ: u32 = 0x00004000; /* transmit short MP seq numbers */
pub const SC_DEBUG: u32 = 0x00010000; /* enable debug messages */
pub const SC_LOG_INPKT: u32 = 0x00020000; /* log contents of good pkts recvd */
pub const SC_LOG_OUTPKT: u32 = 0x00040000; /* log contents of pkts sent */
pub const SC_LOG_RAWIN: u32 = 0x00080000; /* log all chars received */
pub const SC_LOG_FLUSH: u32 = 0x00100000; /* log all chars flushed */
pub const SC_SYNC: u32 = 0x00200000; /* synchronous serial mode */
pub const SC_MUST_COMP: u32 = 0x00400000; /* no uncompressed packets may be sent or received */
pub const SC_MASK: u32 = 0x0f600fff; /* bits that user can change */

/* state bits */
pub const SC_XMIT_BUSY: u32 = 0x10000000; /* (used by isdn_ppp?) */
pub const SC_RCV_ODDP: u32 = 0x08000000; /* have rcvd char with odd parity */
pub const SC_RCV_EVNP: u32 = 0x04000000; /* have rcvd char with even parity */
pub const SC_RCV_B7_1: u32 = 0x02000000; /* have rcvd char with bit 7 = 1 */
pub const SC_RCV_B7_0: u32 = 0x01000000; /* have rcvd char with bit 7 = 0 */
pub const SC_DC_FERROR: u32 = 0x00800000; /* fatal decomp error detected */
pub const SC_DC_ERROR: u32 = 0x00400000; /* non-fatal decomp error detected */

/* Used with PPPIOCGNPMODE/PPPIOCSNPMODE */
#[repr(C)]
pub struct npioctl {
    pub protocol: ::core::ffi::c_int, /* PPP protocol, e.g. PPP_IP */
    pub mode: NPmode,
}

/* Structure describing a CCP configuration option, for PPPIOCSCOMPRESS */
#[repr(C)]
pub struct ppp_option_data {
    pub ptr: *mut __u8,
    pub length: __u32,
    pub transmit: ::core::ffi::c_int,
}

/* For PPPIOCGL2TPSTATS */
#[repr(C)]
pub struct pppol2tp_ioc_stats {
    pub tunnel_id: __u16, /* redundant */
    pub session_id: __u16, /* if zero, get tunnel stats */
    pub using_ipsec: __u32, /* C bit-field: using_ipsec:1 */
    pub tx_packets: __aligned_u64,
    pub tx_bytes: __aligned_u64,
    pub tx_errors: __aligned_u64,
    pub rx_packets: __aligned_u64,
    pub rx_bytes: __aligned_u64,
    pub rx_seq_discards: __aligned_u64,
    pub rx_oos_packets: __aligned_u64,
    pub rx_errors: __aligned_u64,
}

/* Ioctl definitions. */
pub const PPPIOCGFLAGS: _IOR = _IOR('t', 90, ::core::ffi::c_int); /* get configuration flags */
pub const PPPIOCSFLAGS: _IOW = _IOW('t', 89, ::core::ffi::c_int); /* set configuration flags */
pub const PPPIOCGASYNCMAP: _IOR = _IOR('t', 88, ::core::ffi::c_int); /* get async map */
pub const PPPIOCSASYNCMAP: _IOW = _IOW('t', 87, ::core::ffi::c_int); /* set async map */
pub const PPPIOCGUNIT: _IOR = _IOR('t', 86, ::core::ffi::c_int); /* get ppp unit number */
pub const PPPIOCGNPMODE: _IOWR = _IOWR('t', 76, npioctl); /* get NP mode */
pub const PPPIOCSNPMODE: _IOW = _IOW('t', 75, npioctl); /* set NP mode */
pub const PPPIOCGMRU: _IOR = _IOR('t', 83, ::core::ffi::c_int);
pub const PPPIOCSMRU: _IOW = _IOW('t', 82, ::core::ffi::c_int);
pub const PPPIOCSMAXCID: _IOW = _IOW('t', 81, ::core::ffi::c_int);
pub const PPPIOCGXASYNCMAP: _IOR = _IOR('t', 80, ext_accm);
pub const PPPIOCSXASYNCMAP: _IOW = _IOW('t', 79, ext_accm);
pub const PPPIOCXFERUNIT: _IO = _IO('t', 78); /* transfer PPP unit */
pub const PPPIOCSCOMPRESS: _IOW = _IOW('t', 77, ppp_option_data);
pub const PPPIOCSPASS: _IOW = _IOW('t', 71, sock_fprog);
pub const PPPIOCSACTIVE: _IOW = _IOW('t', 70, sock_fprog);
pub const PPPIOCGDEBUG: _IOR = _IOR('t', 65, ::core::ffi::c_int);
pub const PPPIOCSDEBUG: _IOW = _IOW('t', 64, ::core::ffi::c_int);
pub const PPPIOCGIDLE: _IOR = _IOR('t', 63, ppp_idle);
pub const PPPIOCGIDLE32: _IOR = _IOR('t', 63, ppp_idle32);
pub const PPPIOCGIDLE64: _IOR = _IOR('t', 63, ppp_idle64);
pub const PPPIOCNEWUNIT: _IOWR = _IOWR('t', 62, ::core::ffi::c_int);
pub const PPPIOCATTACH: _IOW = _IOW('t', 61, ::core::ffi::c_int);
pub const PPPIOCDETACH: _IOW = _IOW('t', 60, ::core::ffi::c_int); /* obsolete, do not use */
pub const PPPIOCSMRRU: _IOW = _IOW('t', 59, ::core::ffi::c_int);
pub const PPPIOCCONNECT: _IOW = _IOW('t', 58, ::core::ffi::c_int);
pub const PPPIOCDISCONN: _IO = _IO('t', 57);
pub const PPPIOCATTCHAN: _IOW = _IOW('t', 56, ::core::ffi::c_int);
pub const PPPIOCGCHAN: _IOR = _IOR('t', 55, ::core::ffi::c_int);
pub const PPPIOCGL2TPSTATS: _IOR = _IOR('t', 54, pppol2tp_ioc_stats);
pub const PPPIOCBRIDGECHAN: _IOW = _IOW('t', 53, ::core::ffi::c_int);
pub const PPPIOCUNBRIDGECHAN: _IO = _IO('t', 52);

pub const SIOCGPPPSTATS: _SIO = SIOCDEVPRIVATE + 0; 
pub const SIOCGPPPVER: _SIO = SIOCDEVPRIVATE + 1; /* NEVER change this!! */
pub const SIOCGPPPCSTATS: _SIO = SIOCDEVPRIVATE + 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
