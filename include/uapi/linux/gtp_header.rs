/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const GTP_GENL_MCGRP_NAME: &str = "gtp";

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gtp_genl_cmds {
    GTP_CMD_NEWPDP = 0,
    GTP_CMD_DELPDP,
    GTP_CMD_GETPDP,
    GTP_CMD_ECHOREQ,

    GTP_CMD_MAX,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gtp_version {
    GTP_V0 = 0,
    GTP_V1,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gtp_attrs {
    GTPA_UNSPEC = 0,
    GTPA_LINK,
    GTPA_VERSION,
    GTPA_TID, // for GTPv0 only
    GTPA_PEER_ADDRESS, // Remote GSN peer, either SGSN or GGSN
    GTPA_MS_ADDRESS,
    GTPA_FLOW,
    GTPA_NET_NS_FD,
    GTPA_I_TEI, // for GTPv1 only
    GTPA_O_TEI, // for GTPv1 only
    GTPA_PAD,
    GTPA_PEER_ADDR6,
    GTPA_MS_ADDR6,
    GTPA_FAMILY,
    __GTPA_MAX,
}

pub const GTPA_SGSN_ADDRESS: gtp_attrs = gtp_attrs::GTPA_PEER_ADDRESS;
pub const GTPA_MAX: u32 = gtp_attrs::__GTPA_MAX as u32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
