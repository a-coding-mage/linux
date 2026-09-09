/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct ndmsg {
    pub ndm_family: __u8,
    pub ndm_pad1: __u8,
    pub ndm_pad2: __u16,
    pub ndm_ifindex: __s32,
    pub ndm_state: __u16,
    pub ndm_flags: __u8,
    pub ndm_type: __u8,
}

pub const NDA_UNSPEC: i32 = 0;
pub const NDA_DST: i32 = 1;
pub const NDA_LLADDR: i32 = 2;
pub const NDA_CACHEINFO: i32 = 3;
pub const NDA_PROBES: i32 = 4;
pub const NDA_VLAN: i32 = 5;
pub const NDA_PORT: i32 = 6;
pub const NDA_VNI: i32 = 7;
pub const NDA_IFINDEX: i32 = 8;
pub const NDA_MASTER: i32 = 9;
pub const NDA_LINK_NETNSID: i32 = 10;
pub const NDA_SRC_VNI: i32 = 11;
pub const NDA_PROTOCOL: i32 = 12; /* Originator of entry */
pub const NDA_NH_ID: i32 = 13;
pub const NDA_FDB_EXT_ATTRS: i32 = 14;
pub const NDA_FLAGS_EXT: i32 = 15;
pub const NDA_NDM_STATE_MASK: i32 = 16;
pub const NDA_NDM_FLAGS_MASK: i32 = 17;
pub const __NDA_MAX: i32 = 18;
pub const NDA_MAX: i32 = __NDA_MAX - 1;

/* Neighbor Cache Entry Flags */
pub const NTF_USE: u32 = 1 << 0;
pub const NTF_SELF: u32 = 1 << 1;
pub const NTF_MASTER: u32 = 1 << 2;
pub const NTF_PROXY: u32 = 1 << 3; /* == ATF_PUBL */
pub const NTF_EXT_LEARNED: u32 = 1 << 4;
pub const NTF_OFFLOADED: u32 = 1 << 5;
pub const NTF_STICKY: u32 = 1 << 6;
pub const NTF_ROUTER: u32 = 1 << 7;
/* Extended flags under NDA_FLAGS_EXT: */
pub const NTF_EXT_MANAGED: u32 = 1 << 0;
pub const NTF_EXT_LOCKED: u32 = 1 << 1;
pub const NTF_EXT_EXT_VALIDATED: u32 = 1 << 2;

/* Neighbor Cache Entry States. */
pub const NUD_INCOMPLETE: u32 = 0x01;
pub const NUD_REACHABLE: u32 = 0x02;
pub const NUD_STALE: u32 = 0x04;
pub const NUD_DELAY: u32 = 0x08;
pub const NUD_PROBE: u32 = 0x10;
pub const NUD_FAILED: u32 = 0x20;
/* Dummy states */
pub const NUD_NOARP: u32 = 0x40;
pub const NUD_PERMANENT: u32 = 0x80;
pub const NUD_NONE: u32 = 0x00;

/* NUD_NOARP and NUD_PERMANENT are pseudostates; they never change and make no
 * address resolution or NUD. */

#[repr(C)]
pub struct nda_cacheinfo {
    pub ndm_confirmed: __u32,
    pub ndm_used: __u32,
    pub ndm_updated: __u32,
    pub ndm_refcnt: __u32,
}

#[repr(C)]
pub struct ndt_stats {
    pub ndts_allocs: __u64,
    pub ndts_destroys: __u64,
    pub ndts_hash_grows: __u64,
    pub ndts_res_failed: __u64,
    pub ndts_lookups: __u64,
    pub ndts_hits: __u64,
    pub ndts_rcv_probes_mcast: __u64,
    pub ndts_rcv_probes_ucast: __u64,
    pub ndts_periodic_gc_runs: __u64,
    pub ndts_forced_gc_runs: __u64,
    pub ndts_table_fulls: __u64,
}

pub const NDTPA_UNSPEC: i32 = 0;
pub const NDTPA_IFINDEX: i32 = 1; /* u32, unchangeable */
pub const NDTPA_REFCNT: i32 = 2; /* u32, read-only */
pub const NDTPA_REACHABLE_TIME: i32 = 3; /* u64, read-only, msecs */
pub const NDTPA_BASE_REACHABLE_TIME: i32 = 4; /* u64, msecs */
pub const NDTPA_RETRANS_TIME: i32 = 5; /* u64, msecs */
pub const NDTPA_GC_STALETIME: i32 = 6; /* u64, msecs */
pub const NDTPA_DELAY_PROBE_TIME: i32 = 7; /* u64, msecs */
pub const NDTPA_QUEUE_LEN: i32 = 8; /* u32 */
pub const NDTPA_APP_PROBES: i32 = 9; /* u32 */
pub const NDTPA_UCAST_PROBES: i32 = 10; /* u32 */
pub const NDTPA_MCAST_PROBES: i32 = 11; /* u32 */
pub const NDTPA_ANYCAST_DELAY: i32 = 12; /* u64, msecs */
pub const NDTPA_PROXY_DELAY: i32 = 13; /* u64, msecs */
pub const NDTPA_PROXY_QLEN: i32 = 14; /* u32 */
pub const NDTPA_LOCKTIME: i32 = 15; /* u64, msecs */
pub const NDTPA_QUEUE_LENBYTES: i32 = 16; /* u32 */
pub const NDTPA_MCAST_REPROBES: i32 = 17; /* u32 */
pub const NDTPA_PAD: i32 = 18;
pub const NDTPA_INTERVAL_PROBE_TIME_MS: i32 = 19; /* u64, msecs */
pub const __NDTPA_MAX: i32 = 20;
pub const NDTPA_MAX: i32 = __NDTPA_MAX - 1;

#[repr(C)]
pub struct ndtmsg {
    pub ndtm_family: __u8,
    pub ndtm_pad1: __u8,
    pub ndtm_pad2: __u16,
}

#[repr(C)]
pub struct ndt_config {
    pub ndtc_key_len: __u16,
    pub ndtc_entry_size: __u16,
    pub ndtc_entries: __u32,
    pub ndtc_last_flush: __u32, /* delta to now in msecs */
    pub ndtc_last_rand: __u32, /* delta to now in msecs */
    pub ndtc_hash_rnd: __u32,
    pub ndtc_hash_mask: __u32,
    pub ndtc_hash_chain_gc: __u32,
    pub ndtc_proxy_qlen: __u32,
}

pub const NDTA_UNSPEC: i32 = 0;
pub const NDTA_NAME: i32 = 1; /* char *, unchangeable */
pub const NDTA_THRESH1: i32 = 2; /* u32 */
pub const NDTA_THRESH2: i32 = 3; /* u32 */
pub const NDTA_THRESH3: i32 = 4; /* u32 */
pub const NDTA_CONFIG: i32 = 5; /* struct ndt_config, read-only */
pub const NDTA_PARMS: i32 = 6; /* nested TLV NDTPA_* */
pub const NDTA_STATS: i32 = 7; /* struct ndt_stats, read-only */
pub const NDTA_GC_INTERVAL: i32 = 8; /* u64, msecs */
pub const NDTA_PAD: i32 = 9;
pub const __NDTA_MAX: i32 = 10;
pub const NDTA_MAX: i32 = __NDTA_MAX - 1;

pub const FDB_NOTIFY_BIT: u32 = 1 << 0;
pub const FDB_NOTIFY_INACTIVE_BIT: u32 = 1 << 1;

pub const NFEA_UNSPEC: i32 = 0;
pub const NFEA_ACTIVITY_NOTIFY: i32 = 1;
pub const NFEA_DONT_REFRESH: i32 = 2;
pub const __NFEA_MAX: i32 = 3;
pub const NFEA_MAX: i32 = __NFEA_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
