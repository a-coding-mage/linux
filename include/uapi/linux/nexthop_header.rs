/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Dependency intent: types correspond to linux/types.h (__u8, __u16, __u32). */

#[repr(C)]
pub struct nhmsg {
    pub nh_family: u8,
    pub nh_scope: u8,     /* return only */
    pub nh_protocol: u8,  /* Routing protocol that installed nh */
    pub resvd: u8,
    pub nh_flags: u32,    /* RTNH_F flags */
}

/* entry in a nexthop group */
#[repr(C)]
pub struct nexthop_grp {
    pub id: u32,          /* nexthop id - must exist */
    pub weight: u8,      /* weight of this nexthop */
    pub weight_high: u8, /* high order bits of weight */
    pub resvd2: u16,
}

pub unsafe fn nexthop_grp_weight(entry: *const nexthop_grp) -> u16 {
    (((*entry).weight_high as u16).wrapping_shl(8) | (*entry).weight as u16).wrapping_add(1)
}

pub const NEXTHOP_GRP_TYPE_MPATH: i32 = 0;
pub const NEXTHOP_GRP_TYPE_RES: i32 = 1;
pub const __NEXTHOP_GRP_TYPE_MAX: i32 = 2;

pub const NEXTHOP_GRP_TYPE_MAX: i32 = __NEXTHOP_GRP_TYPE_MAX - 1;

pub const NHA_OP_FLAG_DUMP_STATS: u32 = 1u32 << 0;
pub const NHA_OP_FLAG_DUMP_HW_STATS: u32 = 1u32 << 1;

/* Response OP_FLAGS. */
pub const NHA_OP_FLAG_RESP_GRP_RESVD_0: u32 = 1u32 << 31; /* Dump clears resvd fields. */

pub const NHA_UNSPEC: i32 = 0;
pub const NHA_ID: i32 = 1; /* u32; id for nexthop. id == 0 means auto-assign */

pub const NHA_GROUP: i32 = 2; /* array of nexthop_grp */
pub const NHA_GROUP_TYPE: i32 = 3; /* u16 one of NEXTHOP_GRP_TYPE */
/* if NHA_GROUP attribute is added, no other attributes can be set */

pub const NHA_BLACKHOLE: i32 = 4; /* flag; nexthop used to blackhole packets */
/* if NHA_BLACKHOLE is added, OIF, GATEWAY, ENCAP can not be set */

pub const NHA_OIF: i32 = 5; /* u32; nexthop device */
pub const NHA_GATEWAY: i32 = 6; /* be32 (IPv4) or in6_addr (IPv6) gw address */
pub const NHA_ENCAP_TYPE: i32 = 7; /* u16; lwt encap type */
pub const NHA_ENCAP: i32 = 8; /* lwt encap data */

/* NHA_OIF can be appended to dump request to return only
 * nexthops using given device
 */
pub const NHA_GROUPS: i32 = 9; /* flag; only return nexthop groups in dump */
pub const NHA_MASTER: i32 = 10; /* u32;  only return nexthops with given master dev */

pub const NHA_FDB: i32 = 11; /* flag; nexthop belongs to a bridge fdb */
/* if NHA_FDB is added, OIF, BLACKHOLE, ENCAP cannot be set */

/* nested; resilient nexthop group attributes */
pub const NHA_RES_GROUP: i32 = 12;
/* nested; nexthop bucket attributes */
pub const NHA_RES_BUCKET: i32 = 13;

/* u32; operation-specific flags */
pub const NHA_OP_FLAGS: i32 = 14;
/* nested; nexthop group stats */
pub const NHA_GROUP_STATS: i32 = 15;
/* u32; nexthop hardware stats enable */
pub const NHA_HW_STATS_ENABLE: i32 = 16;
/* u32; read-only; whether any driver collects HW stats */
pub const NHA_HW_STATS_USED: i32 = 17;
/* be16; UDP destination port for an fdb nexthop (e.g. VXLAN) */
pub const NHA_DST_PORT: i32 = 18;
pub const __NHA_MAX: i32 = 19;
pub const NHA_MAX: i32 = __NHA_MAX - 1;

pub const NHA_RES_GROUP_UNSPEC: i32 = 0;
/* Pad attribute for 64-bit alignment. */
pub const NHA_RES_GROUP_PAD: i32 = NHA_RES_GROUP_UNSPEC;
/* u16; number of nexthop buckets in a resilient nexthop group */
pub const NHA_RES_GROUP_BUCKETS: i32 = 1;
/* clock_t as u32; nexthop bucket idle timer (per-group) */
pub const NHA_RES_GROUP_IDLE_TIMER: i32 = 2;
/* clock_t as u32; nexthop unbalanced timer */
pub const NHA_RES_GROUP_UNBALANCED_TIMER: i32 = 3;
/* clock_t as u64; nexthop unbalanced time */
pub const NHA_RES_GROUP_UNBALANCED_TIME: i32 = 4;
pub const __NHA_RES_GROUP_MAX: i32 = 5;
pub const NHA_RES_GROUP_MAX: i32 = __NHA_RES_GROUP_MAX - 1;

pub const NHA_RES_BUCKET_UNSPEC: i32 = 0;
/* Pad attribute for 64-bit alignment. */
pub const NHA_RES_BUCKET_PAD: i32 = NHA_RES_BUCKET_UNSPEC;
/* u16; nexthop bucket index */
pub const NHA_RES_BUCKET_INDEX: i32 = 1;
/* clock_t as u64; nexthop bucket idle time */
pub const NHA_RES_BUCKET_IDLE_TIME: i32 = 2;
/* u32; nexthop id assigned to the nexthop bucket */
pub const NHA_RES_BUCKET_NH_ID: i32 = 3;
pub const __NHA_RES_BUCKET_MAX: i32 = 4;
pub const NHA_RES_BUCKET_MAX: i32 = __NHA_RES_BUCKET_MAX - 1;

pub const NHA_GROUP_STATS_UNSPEC: i32 = 0;
/* nested; nexthop group entry stats */
pub const NHA_GROUP_STATS_ENTRY: i32 = 1;
pub const __NHA_GROUP_STATS_MAX: i32 = 2;
pub const NHA_GROUP_STATS_MAX: i32 = __NHA_GROUP_STATS_MAX - 1;

pub const NHA_GROUP_STATS_ENTRY_UNSPEC: i32 = 0;
/* u32; nexthop id of the nexthop group entry */
pub const NHA_GROUP_STATS_ENTRY_ID: i32 = 1;
/* uint; number of packets forwarded via the nexthop group entry */
pub const NHA_GROUP_STATS_ENTRY_PACKETS: i32 = 2;
/* uint; number of packets forwarded via the nexthop group entry in
 * hardware
 */
pub const NHA_GROUP_STATS_ENTRY_PACKETS_HW: i32 = 3;
pub const __NHA_GROUP_STATS_ENTRY_MAX: i32 = 4;
pub const NHA_GROUP_STATS_ENTRY_MAX: i32 = __NHA_GROUP_STATS_ENTRY_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
