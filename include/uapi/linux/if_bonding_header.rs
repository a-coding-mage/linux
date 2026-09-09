/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */
/*
 * Bond several ethernet interfaces into a Cisco, running 'Etherchannel'.
 *
 * This is a source-level Rust translation of the Linux UAPI header.
 */

/* Dependencies supplied by the surrounding UAPI translation. */

/* userland - kernel ABI version (2003/05/08) */
pub const BOND_ABI_VERSION: i32 = 2;

/*
 * We can remove these ioctl definitions in 2.5. People should use the
 * SIOC*** versions of them instead
 */
pub const BOND_ENSLAVE_OLD: _ = SIOCDEVPRIVATE;
pub const BOND_RELEASE_OLD: _ = SIOCDEVPRIVATE + 1;
pub const BOND_SETHWADDR_OLD: _ = SIOCDEVPRIVATE + 2;
pub const BOND_SLAVE_INFO_QUERY_OLD: _ = SIOCDEVPRIVATE + 11;
pub const BOND_INFO_QUERY_OLD: _ = SIOCDEVPRIVATE + 12;
pub const BOND_CHANGE_ACTIVE_OLD: _ = SIOCDEVPRIVATE + 13;

pub const BOND_CHECK_MII_STATUS: _ = SIOCGMIIPHY;

pub const BOND_MODE_ROUNDROBIN: i32 = 0;
pub const BOND_MODE_ACTIVEBACKUP: i32 = 1;
pub const BOND_MODE_XOR: i32 = 2;
pub const BOND_MODE_BROADCAST: i32 = 3;
pub const BOND_MODE_8023AD: i32 = 4;
pub const BOND_MODE_TLB: i32 = 5;
pub const BOND_MODE_ALB: i32 = 6; /* TLB + RLB (receive load balancing) */

/* each slave's link has 4 states */
pub const BOND_LINK_UP: i32 = 0; /* link is up and running */
pub const BOND_LINK_FAIL: i32 = 1; /* link has just gone down */
pub const BOND_LINK_DOWN: i32 = 2; /* link has been down for too long time */
pub const BOND_LINK_BACK: i32 = 3; /* link is going back */

/* each slave has several states */
pub const BOND_STATE_ACTIVE: i32 = 0; /* link is active */
pub const BOND_STATE_BACKUP: i32 = 1; /* link is backup */

pub const BOND_DEFAULT_MAX_BONDS: i32 = 1; /* Default maximum number of devices to support */
pub const BOND_DEFAULT_TX_QUEUES: i32 = 16; /* Default number of tx queues per device */
pub const BOND_DEFAULT_RESEND_IGMP: i32 = 1; /* Default number of IGMP membership reports */

/* hashing types */
pub const BOND_XMIT_POLICY_LAYER2: i32 = 0; /* layer 2 (MAC only), default */
pub const BOND_XMIT_POLICY_LAYER34: i32 = 1; /* layer 3+4 (IP ^ (TCP || UDP)) */
pub const BOND_XMIT_POLICY_LAYER23: i32 = 2; /* layer 2+3 (IP ^ MAC) */
pub const BOND_XMIT_POLICY_ENCAP23: i32 = 3; /* encapsulated layer 2+3 */
pub const BOND_XMIT_POLICY_ENCAP34: i32 = 4; /* encapsulated layer 3+4 */
pub const BOND_XMIT_POLICY_VLAN_SRCMAC: i32 = 5; /* vlan + source MAC */

/* 802.3ad port state definitions (43.4.2.2 in the 802.3ad standard) */
pub const LACP_STATE_LACP_ACTIVITY: u8 = 0x1;
pub const LACP_STATE_LACP_TIMEOUT: u8 = 0x2;
pub const LACP_STATE_AGGREGATION: u8 = 0x4;
pub const LACP_STATE_SYNCHRONIZATION: u8 = 0x8;
pub const LACP_STATE_COLLECTING: u8 = 0x10;
pub const LACP_STATE_DISTRIBUTING: u8 = 0x20;
pub const LACP_STATE_DEFAULTED: u8 = 0x40;
pub const LACP_STATE_EXPIRED: u8 = 0x80;

#[repr(C)]
pub struct ifbond {
    pub bond_mode: i32,
    pub num_slaves: i32,
    pub miimon: i32,
}

#[repr(C)]
pub struct ifslave {
    pub slave_id: i32, /* Used as an IN param to the BOND_SLAVE_INFO_QUERY ioctl */
    pub slave_name: [::core::ffi::c_char; IFNAMSIZ],
    pub link: i8,
    pub state: i8,
    pub link_failure_count: u32,
}

#[repr(C)]
pub struct ad_info {
    pub aggregator_id: u16,
    pub ports: u16,
    pub actor_key: u16,
    pub partner_key: u16,
    pub partner_system: [u8; ETH_ALEN],
}

/* Embedded inside LINK_XSTATS_TYPE_BOND */
#[repr(u32)]
pub enum BondXstats {
    BOND_XSTATS_UNSPEC,
    BOND_XSTATS_3AD,
    __BOND_XSTATS_MAX,
}
pub const BOND_XSTATS_MAX: u32 = BondXstats::__BOND_XSTATS_MAX as u32 - 1;

/* Embedded inside BOND_XSTATS_3AD */
#[repr(u32)]
pub enum Bond3adStat {
    BOND_3AD_STAT_LACPDU_RX,
    BOND_3AD_STAT_LACPDU_TX,
    BOND_3AD_STAT_LACPDU_UNKNOWN_RX,
    BOND_3AD_STAT_LACPDU_ILLEGAL_RX,
    BOND_3AD_STAT_MARKER_RX,
    BOND_3AD_STAT_MARKER_TX,
    BOND_3AD_STAT_MARKER_RESP_RX,
    BOND_3AD_STAT_MARKER_RESP_TX,
    BOND_3AD_STAT_MARKER_UNKNOWN_RX,
    BOND_3AD_STAT_PAD,
    __BOND_3AD_STAT_MAX,
}
pub const BOND_3AD_STAT_MAX: u32 = Bond3adStat::__BOND_3AD_STAT_MAX as u32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
