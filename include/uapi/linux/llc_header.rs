/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */
/*
 * IEEE 802.2 User Interface SAPs for Linux, data structures and indicators.
 *
 * Copyright (c) 2001 by Jay Schulist <jschlst@samba.org>
 */

// Dependencies supplied by the surrounding Linux UAPI translation:
// `__kernel_sa_family_t` and `IFHWADDRLEN`.

pub const __LLC_SOCK_SIZE__: usize = 16; /* sizeof(sockaddr_llc), word align. */

#[repr(C)]
pub struct sockaddr_llc {
    pub sllc_family: __kernel_sa_family_t, /* AF_LLC */
    pub sllc_arphrd: __kernel_sa_family_t, /* ARPHRD_ETHER */
    pub sllc_test: u8,
    pub sllc_xid: u8,
    pub sllc_ua: u8, /* UA data, only for SOCK_STREAM. */
    pub sllc_sap: u8,
    pub sllc_mac: [u8; IFHWADDRLEN],
    pub __pad: [u8; __LLC_SOCK_SIZE__
        - core::mem::size_of::<__kernel_sa_family_t>() * 2
        - core::mem::size_of::<u8>() * 4
        - IFHWADDRLEN],
}

/* sockopt definitions. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum llc_sockopts {
    LLC_OPT_UNKNOWN = 0,
    LLC_OPT_RETRY,      /* max retrans attempts. */
    LLC_OPT_SIZE,       /* max PDU size (octets). */
    LLC_OPT_ACK_TMR_EXP, /* ack expire time (secs). */
    LLC_OPT_P_TMR_EXP,  /* pf cycle expire time (secs). */
    LLC_OPT_REJ_TMR_EXP, /* rej sent expire time (secs). */
    LLC_OPT_BUSY_TMR_EXP, /* busy state expire time (secs). */
    LLC_OPT_TX_WIN,     /* tx window size. */
    LLC_OPT_RX_WIN,     /* rx window size. */
    LLC_OPT_PKTINFO,    /* ancillary packet information. */
    LLC_OPT_MAX,
}

pub const LLC_OPT_MAX_RETRY: i32 = 100;
pub const LLC_OPT_MAX_SIZE: i32 = 4196;
pub const LLC_OPT_MAX_WIN: i32 = 127;
pub const LLC_OPT_MAX_ACK_TMR_EXP: i32 = 60;
pub const LLC_OPT_MAX_P_TMR_EXP: i32 = 60;
pub const LLC_OPT_MAX_REJ_TMR_EXP: i32 = 60;
pub const LLC_OPT_MAX_BUSY_TMR_EXP: i32 = 60;

/* LLC SAP types. */
pub const LLC_SAP_NULL: u8 = 0x00; /* NULL SAP. */
pub const LLC_SAP_LLC: u8 = 0x02; /* LLC Sublayer Management. */
pub const LLC_SAP_SNA: u8 = 0x04; /* SNA Path Control. */
pub const LLC_SAP_PNM: u8 = 0x0E; /* Proway Network Management. */
pub const LLC_SAP_IP: u8 = 0x06; /* TCP/IP. */
pub const LLC_SAP_BSPAN: u8 = 0x42; /* Bridge Spanning Tree Proto */
pub const LLC_SAP_MMS: u8 = 0x4E; /* Manufacturing Message Srv. */
pub const LLC_SAP_8208: u8 = 0x7E; /* ISO 8208 */
pub const LLC_SAP_3COM: u8 = 0x80; /* 3COM. */
pub const LLC_SAP_PRO: u8 = 0x8E; /* Proway Active Station List */
pub const LLC_SAP_SNAP: u8 = 0xAA; /* SNAP. */
pub const LLC_SAP_BANYAN: u8 = 0xBC; /* Banyan. */
pub const LLC_SAP_IPX: u8 = 0xE0; /* IPX/SPX. */
pub const LLC_SAP_NETBEUI: u8 = 0xF0; /* NetBEUI. */
pub const LLC_SAP_LANMGR: u8 = 0xF4; /* LanManager. */
pub const LLC_SAP_IMPL: u8 = 0xF8; /* IMPL */
pub const LLC_SAP_DISC: u8 = 0xFC; /* Discovery */
pub const LLC_SAP_OSI: u8 = 0xFE; /* OSI Network Layers. */
pub const LLC_SAP_LAR: u8 = 0xDC; /* LAN Address Resolution */
pub const LLC_SAP_RM: u8 = 0xD4; /* Resource Management */
pub const LLC_SAP_GLOBAL: u8 = 0xFF; /* Global SAP. */

#[repr(C)]
pub struct llc_pktinfo {
    pub lpi_ifindex: i32,
    pub lpi_sap: u8,
    pub lpi_mac: [u8; IFHWADDRLEN],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
