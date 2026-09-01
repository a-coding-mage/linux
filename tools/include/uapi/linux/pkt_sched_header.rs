/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* C header dependency: <linux/types.h> */
use core::ffi::{c_int, c_uint};

/* Logical priority bands not depending on specific packet scheduler.
   Every scheduler will map them to real traffic classes, if it has
   no more precise mechanism to classify packets.

   These numbers have no special meaning, though their coincidence
   with obsolete IPv6 values is not occasional :-). New IPv6 drafts
   preferred full anarchy inspired by diffserv group.

   Note: TC_PRIO_BESTEFFORT does not mean that it is the most unhappy
   class, actually, as rule it will be handled with more care than
   filler or even bulk.
 */

pub const TC_PRIO_BESTEFFORT: c_uint = 0;
pub const TC_PRIO_FILLER: c_uint = 1;
pub const TC_PRIO_BULK: c_uint = 2;
pub const TC_PRIO_INTERACTIVE_BULK: c_uint = 4;
pub const TC_PRIO_INTERACTIVE: c_uint = 6;
pub const TC_PRIO_CONTROL: c_uint = 7;

pub const TC_PRIO_MAX: c_uint = 15;

/* Generic queue statistics, available for all the elements.
   Particular schedulers may have also their private records.
 */

#[repr(C)]
pub struct tc_stats {
    pub bytes: __u64,      /* Number of enqueued bytes */
    pub packets: __u32,    /* Number of enqueued packets */
    pub drops: __u32,      /* Packets dropped because of lack of resources */
    pub overlimits: __u32, /* Number of throttle events when this
                            * flow goes out of allocated bandwidth */
    pub bps: __u32,        /* Current flow byte rate */
    pub pps: __u32,        /* Current flow packet rate */
    pub qlen: __u32,
    pub backlog: __u32,
}

#[repr(C)]
pub struct tc_estimator {
    pub interval: i8,
    pub ewma_log: u8,
}

/* "Handles"
   ---------

    All the traffic control objects have 32bit identifiers, or "handles".

    They can be considered as opaque numbers from user API viewpoint,
    but actually they always consist of two fields: major and
    minor numbers, which are interpreted by kernel specially,
    that may be used by applications, though not recommended.

    F.e. qdisc handles always have minor number equal to zero,
    classes (or flows) have major equal to parent qdisc major, and
    minor uniquely identifying class inside qdisc.

    Macros to manipulate handles:
 */

pub const TC_H_MAJ_MASK: c_uint = 0xFFFF0000;
pub const TC_H_MIN_MASK: c_uint = 0x0000FFFF;
pub const fn TC_H_MAJ(h: c_uint) -> c_uint {
    h & TC_H_MAJ_MASK
}
pub const fn TC_H_MIN(h: c_uint) -> c_uint {
    h & TC_H_MIN_MASK
}
pub const fn TC_H_MAKE(maj: c_uint, min: c_uint) -> c_uint {
    (maj & TC_H_MAJ_MASK) | (min & TC_H_MIN_MASK)
}

pub const TC_H_UNSPEC: c_uint = 0;
pub const TC_H_ROOT: c_uint = 0xFFFFFFFF;
pub const TC_H_INGRESS: c_uint = 0xFFFFFFF1;
pub const TC_H_CLSACT: c_uint = TC_H_INGRESS;

pub const TC_H_MIN_PRIORITY: c_uint = 0xFFE0;
pub const TC_H_MIN_INGRESS: c_uint = 0xFFF2;
pub const TC_H_MIN_EGRESS: c_uint = 0xFFF3;

/* Need to corrospond to iproute2 tc/tc_core.h "enum link_layer" */
pub const TC_LINKLAYER_UNAWARE: c_uint = 0; /* Indicate unaware old iproute2 util */
pub const TC_LINKLAYER_ETHERNET: c_uint = 1;
pub const TC_LINKLAYER_ATM: c_uint = 2;
pub const TC_LINKLAYER_MASK: c_uint = 0x0F; /* limit use to lower 4 bits */

#[repr(C)]
pub struct tc_ratespec {
    pub cell_log: u8,
    pub linklayer: __u8, /* lower 4 bits */
    pub overhead: u16,
    pub cell_align: i16,
    pub mpu: u16,
    pub rate: __u32,
}

pub const TC_RTAB_SIZE: c_uint = 1024;

#[repr(C)]
pub struct tc_sizespec {
    pub cell_log: u8,
    pub size_log: u8,
    pub cell_align: i16,
    pub overhead: c_int,
    pub linklayer: c_uint,
    pub mpu: c_uint,
    pub mtu: c_uint,
    pub tsize: c_uint,
}

pub const TCA_STAB_UNSPEC: c_uint = 0;
pub const TCA_STAB_BASE: c_uint = 1;
pub const TCA_STAB_DATA: c_uint = 2;
pub const __TCA_STAB_MAX: c_uint = 3;

pub const TCA_STAB_MAX: c_uint = __TCA_STAB_MAX - 1;

/* FIFO section */

#[repr(C)]
pub struct tc_fifo_qopt {
    pub limit: __u32, /* Queue length: bytes for bfifo, packets for pfifo */
}

/* SKBPRIO section */

/*
 * Priorities go from zero to (SKBPRIO_MAX_PRIORITY - 1).
 * SKBPRIO_MAX_PRIORITY should be at least 64 in order for skbprio to be able
 * to map one to one the DS field of IPV4 and IPV6 headers.
 * Memory allocation grows linearly with SKBPRIO_MAX_PRIORITY.
 */

pub const SKBPRIO_MAX_PRIORITY: c_uint = 64;

#[repr(C)]
pub struct tc_skbprio_qopt {
    pub limit: __u32, /* Queue length in packets. */
}

/* PRIO section */

pub const TCQ_PRIO_BANDS: c_uint = 16;
pub const TCQ_MIN_PRIO_BANDS: c_uint = 2;

#[repr(C)]
pub struct tc_prio_qopt {
    pub bands: c_int,                           /* Number of bands */
    pub priomap: [__u8; (TC_PRIO_MAX + 1) as usize], /* Map: logical priority -> PRIO band */
}

/* MULTIQ section */

#[repr(C)]
pub struct tc_multiq_qopt {
    pub bands: __u16,     /* Number of bands */
    pub max_bands: __u16, /* Maximum number of queues */
}

/* PLUG section */

pub const TCQ_PLUG_BUFFER: c_uint = 0;
pub const TCQ_PLUG_RELEASE_ONE: c_uint = 1;
pub const TCQ_PLUG_RELEASE_INDEFINITE: c_uint = 2;
pub const TCQ_PLUG_LIMIT: c_uint = 3;

#[repr(C)]
pub struct tc_plug_qopt {
    /* TCQ_PLUG_BUFFER: Inset a plug into the queue and
     *  buffer any incoming packets
     * TCQ_PLUG_RELEASE_ONE: Dequeue packets from queue head
     *   to beginning of the next plug.
     * TCQ_PLUG_RELEASE_INDEFINITE: Dequeue all packets from queue.
     *   Stop buffering packets until the next TCQ_PLUG_BUFFER
     *   command is received (just act as a pass-thru queue).
     * TCQ_PLUG_LIMIT: Increase/decrease queue size
     */
    pub action: c_int,
    pub limit: __u32,
}

/* TBF section */

#[repr(C)]
pub struct tc_tbf_qopt {
    pub rate: tc_ratespec,
    pub peakrate: tc_ratespec,
    pub limit: __u32,
    pub buffer: __u32,
    pub mtu: __u32,
}

pub const TCA_TBF_UNSPEC: c_uint = 0;
pub const TCA_TBF_PARMS: c_uint = 1;
pub const TCA_TBF_RTAB: c_uint = 2;
pub const TCA_TBF_PTAB: c_uint = 3;
pub const TCA_TBF_RATE64: c_uint = 4;
pub const TCA_TBF_PRATE64: c_uint = 5;
pub const TCA_TBF_BURST: c_uint = 6;
pub const TCA_TBF_PBURST: c_uint = 7;
pub const TCA_TBF_PAD: c_uint = 8;
pub const __TCA_TBF_MAX: c_uint = 9;

pub const TCA_TBF_MAX: c_uint = __TCA_TBF_MAX - 1;

/* TEQL section */

/* TEQL does not require any parameters */

/* SFQ section */

#[repr(C)]
pub struct tc_sfq_qopt {
    pub quantum: c_uint,        /* Bytes per round allocated to flow */
    pub perturb_period: c_int,  /* Period of hash perturbation */
    pub limit: __u32,           /* Maximal packets in queue */
    pub divisor: c_uint,        /* Hash divisor */
    pub flows: c_uint,          /* Maximal number of flows */
}

#[repr(C)]
pub struct tc_sfqred_stats {
    pub prob_drop: __u32,       /* Early drops, below max threshold */
    pub forced_drop: __u32,     /* Early drops, after max threshold */
    pub prob_mark: __u32,       /* Marked packets, below max threshold */
    pub forced_mark: __u32,     /* Marked packets, after max threshold */
    pub prob_mark_head: __u32,  /* Marked packets, below max threshold */
    pub forced_mark_head: __u32, /* Marked packets, after max threshold */
}

#[repr(C)]
pub struct tc_sfq_qopt_v1 {
    pub v0: tc_sfq_qopt,
    pub depth: c_uint,    /* max number of packets per flow */
    pub headdrop: c_uint,
    /* SFQRED parameters */
    pub limit: __u32,     /* HARD maximal flow queue length (bytes) */
    pub qth_min: __u32,   /* Min average length threshold (bytes) */
    pub qth_max: __u32,   /* Max average length threshold (bytes) */
    pub Wlog: u8,         /* log(W) */
    pub Plog: u8,         /* log(P_max/(qth_max-qth_min)) */
    pub Scell_log: u8,    /* cell size for idle damping */
    pub flags: u8,
    pub max_P: __u32,     /* probability, high resolution */
    /* SFQRED stats */
    pub stats: tc_sfqred_stats,
}

#[repr(C)]
pub struct tc_sfq_xstats {
    pub allot: __s32,
}

/* RED section */

pub const TCA_RED_UNSPEC: c_uint = 0;
pub const TCA_RED_PARMS: c_uint = 1;
pub const TCA_RED_STAB: c_uint = 2;
pub const TCA_RED_MAX_P: c_uint = 3;
pub const __TCA_RED_MAX: c_uint = 4;

pub const TCA_RED_MAX: c_uint = __TCA_RED_MAX - 1;

#[repr(C)]
pub struct tc_red_qopt {
    pub limit: __u32,    /* HARD maximal queue length (bytes) */
    pub qth_min: __u32,  /* Min average length threshold (bytes) */
    pub qth_max: __u32,  /* Max average length threshold (bytes) */
    pub Wlog: u8,        /* log(W) */
    pub Plog: u8,        /* log(P_max/(qth_max-qth_min)) */
    pub Scell_log: u8,   /* cell size for idle damping */
    pub flags: u8,
}
pub const TC_RED_ECN: c_uint = 1;
pub const TC_RED_HARDDROP: c_uint = 2;
pub const TC_RED_ADAPTATIVE: c_uint = 4;

#[repr(C)]
pub struct tc_red_xstats {
    pub early: __u32,  /* Early drops */
    pub pdrop: __u32,  /* Drops due to queue limits */
    pub other: __u32,  /* Drops due to drop() calls */
    pub marked: __u32, /* Marked packets */
}

/* GRED section */

pub const MAX_DPs: c_uint = 16;

pub const TCA_GRED_UNSPEC: c_uint = 0;
pub const TCA_GRED_PARMS: c_uint = 1;
pub const TCA_GRED_STAB: c_uint = 2;
pub const TCA_GRED_DPS: c_uint = 3;
pub const TCA_GRED_MAX_P: c_uint = 4;
pub const TCA_GRED_LIMIT: c_uint = 5;
pub const TCA_GRED_VQ_LIST: c_uint = 6; /* nested TCA_GRED_VQ_ENTRY */
pub const __TCA_GRED_MAX: c_uint = 7;

pub const TCA_GRED_MAX: c_uint = __TCA_GRED_MAX - 1;

pub const TCA_GRED_VQ_ENTRY_UNSPEC: c_uint = 0;
pub const TCA_GRED_VQ_ENTRY: c_uint = 1; /* nested TCA_GRED_VQ_* */
pub const __TCA_GRED_VQ_ENTRY_MAX: c_uint = 2;
pub const TCA_GRED_VQ_ENTRY_MAX: c_uint = __TCA_GRED_VQ_ENTRY_MAX - 1;

pub const TCA_GRED_VQ_UNSPEC: c_uint = 0;
pub const TCA_GRED_VQ_PAD: c_uint = 1;
pub const TCA_GRED_VQ_DP: c_uint = 2; /* u32 */
pub const TCA_GRED_VQ_STAT_BYTES: c_uint = 3; /* u64 */
pub const TCA_GRED_VQ_STAT_PACKETS: c_uint = 4; /* u32 */
pub const TCA_GRED_VQ_STAT_BACKLOG: c_uint = 5; /* u32 */
pub const TCA_GRED_VQ_STAT_PROB_DROP: c_uint = 6; /* u32 */
pub const TCA_GRED_VQ_STAT_PROB_MARK: c_uint = 7; /* u32 */
pub const TCA_GRED_VQ_STAT_FORCED_DROP: c_uint = 8; /* u32 */
pub const TCA_GRED_VQ_STAT_FORCED_MARK: c_uint = 9; /* u32 */
pub const TCA_GRED_VQ_STAT_PDROP: c_uint = 10; /* u32 */
pub const TCA_GRED_VQ_STAT_OTHER: c_uint = 11; /* u32 */
pub const TCA_GRED_VQ_FLAGS: c_uint = 12; /* u32 */
pub const __TCA_GRED_VQ_MAX: c_uint = 13;

pub const TCA_GRED_VQ_MAX: c_uint = __TCA_GRED_VQ_MAX - 1;

#[repr(C)]
pub struct tc_gred_qopt {
    pub limit: __u32,     /* HARD maximal queue length (bytes) */
    pub qth_min: __u32,   /* Min average length threshold (bytes) */
    pub qth_max: __u32,   /* Max average length threshold (bytes) */
    pub DP: __u32,        /* up to 2^32 DPs */
    pub backlog: __u32,
    pub qave: __u32,
    pub forced: __u32,
    pub early: __u32,
    pub other: __u32,
    pub pdrop: __u32,
    pub Wlog: __u8,       /* log(W) */
    pub Plog: __u8,       /* log(P_max/(qth_max-qth_min)) */
    pub Scell_log: __u8,  /* cell size for idle damping */
    pub prio: __u8,       /* prio of this VQ */
    pub packets: __u32,
    pub bytesin: __u32,
}

/* gred setup */
#[repr(C)]
pub struct tc_gred_sopt {
    pub DPs: __u32,
    pub def_DP: __u32,
    pub grio: __u8,
    pub flags: __u8,
    pub pad1: __u16,
}

/* CHOKe section */

pub const TCA_CHOKE_UNSPEC: c_uint = 0;
pub const TCA_CHOKE_PARMS: c_uint = 1;
pub const TCA_CHOKE_STAB: c_uint = 2;
pub const TCA_CHOKE_MAX_P: c_uint = 3;
pub const __TCA_CHOKE_MAX: c_uint = 4;

pub const TCA_CHOKE_MAX: c_uint = __TCA_CHOKE_MAX - 1;

#[repr(C)]
pub struct tc_choke_qopt {
    pub limit: __u32,    /* Hard queue length (packets) */
    pub qth_min: __u32,  /* Min average threshold (packets) */
    pub qth_max: __u32,  /* Max average threshold (packets) */
    pub Wlog: u8,        /* log(W) */
    pub Plog: u8,        /* log(P_max/(qth_max-qth_min)) */
    pub Scell_log: u8,   /* cell size for idle damping */
    pub flags: u8,       /* see RED flags */
}

#[repr(C)]
pub struct tc_choke_xstats {
    pub early: __u32,   /* Early drops */
    pub pdrop: __u32,   /* Drops due to queue limits */
    pub other: __u32,   /* Drops due to drop() calls */
    pub marked: __u32,  /* Marked packets */
    pub matched: __u32, /* Drops due to flow match */
}

/* HTB section */
pub const TC_HTB_NUMPRIO: c_uint = 8;
pub const TC_HTB_MAXDEPTH: c_uint = 8;
pub const TC_HTB_PROTOVER: c_uint = 3; /* the same as HTB and TC's major */

#[repr(C)]
pub struct tc_htb_opt {
    pub rate: tc_ratespec,
    pub ceil: tc_ratespec,
    pub buffer: __u32,
    pub cbuffer: __u32,
    pub quantum: __u32,
    pub level: __u32, /* out only */
    pub prio: __u32,
}

#[repr(C)]
pub struct tc_htb_glob {
    pub version: __u32,      /* to match HTB/TC */
    pub rate2quantum: __u32, /* bps->quantum divisor */
    pub defcls: __u32,       /* default class number */
    pub debug: __u32,        /* debug flags */

    /* stats */
    pub direct_pkts: __u32, /* count of non shaped packets */
}

pub const TCA_HTB_UNSPEC: c_uint = 0;
pub const TCA_HTB_PARMS: c_uint = 1;
pub const TCA_HTB_INIT: c_uint = 2;
pub const TCA_HTB_CTAB: c_uint = 3;
pub const TCA_HTB_RTAB: c_uint = 4;
pub const TCA_HTB_DIRECT_QLEN: c_uint = 5;
pub const TCA_HTB_RATE64: c_uint = 6;
pub const TCA_HTB_CEIL64: c_uint = 7;
pub const TCA_HTB_PAD: c_uint = 8;
pub const TCA_HTB_OFFLOAD: c_uint = 9;
pub const __TCA_HTB_MAX: c_uint = 10;

pub const TCA_HTB_MAX: c_uint = __TCA_HTB_MAX - 1;

#[repr(C)]
pub struct tc_htb_xstats {
    pub lends: __u32,
    pub borrows: __u32,
    pub giants: __u32, /* unused since 'Make HTB scheduler work with TSO.' */
    pub tokens: __s32,
    pub ctokens: __s32,
}

/* HFSC section */

#[repr(C)]
pub struct tc_hfsc_qopt {
    pub defcls: __u16, /* default class */
}

#[repr(C)]
pub struct tc_service_curve {
    pub m1: __u32, /* slope of the first segment in bps */
    pub d: __u32,  /* x-projection of the first segment in us */
    pub m2: __u32, /* slope of the second segment in bps */
}

#[repr(C)]
pub struct tc_hfsc_stats {
    pub work: __u64,   /* total work done */
    pub rtwork: __u64, /* work done by real-time criteria */
    pub period: __u32, /* current period */
    pub level: __u32,  /* class level in hierarchy */
}

pub const TCA_HFSC_UNSPEC: c_uint = 0;
pub const TCA_HFSC_RSC: c_uint = 1;
pub const TCA_HFSC_FSC: c_uint = 2;
pub const TCA_HFSC_USC: c_uint = 3;
pub const __TCA_HFSC_MAX: c_uint = 4;

pub const TCA_HFSC_MAX: c_uint = __TCA_HFSC_MAX - 1;

/* Network emulator */

pub const TCA_NETEM_UNSPEC: c_uint = 0;
pub const TCA_NETEM_CORR: c_uint = 1;
pub const TCA_NETEM_DELAY_DIST: c_uint = 2;
pub const TCA_NETEM_REORDER: c_uint = 3;
pub const TCA_NETEM_CORRUPT: c_uint = 4;
pub const TCA_NETEM_LOSS: c_uint = 5;
pub const TCA_NETEM_RATE: c_uint = 6;
pub const TCA_NETEM_ECN: c_uint = 7;
pub const TCA_NETEM_RATE64: c_uint = 8;
pub const TCA_NETEM_PAD: c_uint = 9;
pub const TCA_NETEM_LATENCY64: c_uint = 10;
pub const TCA_NETEM_JITTER64: c_uint = 11;
pub const TCA_NETEM_SLOT: c_uint = 12;
pub const TCA_NETEM_SLOT_DIST: c_uint = 13;
pub const __TCA_NETEM_MAX: c_uint = 14;

pub const TCA_NETEM_MAX: c_uint = __TCA_NETEM_MAX - 1;

#[repr(C)]
pub struct tc_netem_qopt {
    pub latency: __u32,   /* added delay (us) */
    pub limit: __u32,     /* fifo limit (packets) */
    pub loss: __u32,      /* random packet loss (0=none ~0=100%) */
    pub gap: __u32,       /* re-ordering gap (0 for none) */
    pub duplicate: __u32, /* random packet dup (0=none ~0=100%) */
    pub jitter: __u32,    /* random jitter in latency (us) */
}

#[repr(C)]
pub struct tc_netem_corr {
    pub delay_corr: __u32, /* delay correlation */
    pub loss_corr: __u32,  /* packet loss correlation */
    pub dup_corr: __u32,   /* duplicate correlation */
}

#[repr(C)]
pub struct tc_netem_reorder {
    pub probability: __u32,
    pub correlation: __u32,
}

#[repr(C)]
pub struct tc_netem_corrupt {
    pub probability: __u32,
    pub correlation: __u32,
}

#[repr(C)]
pub struct tc_netem_rate {
    pub rate: __u32, /* byte/s */
    pub packet_overhead: __s32,
    pub cell_size: __u32,
    pub cell_overhead: __s32,
}

#[repr(C)]
pub struct tc_netem_slot {
    pub min_delay: __s64, /* nsec */
    pub max_delay: __s64,
    pub max_packets: __s32,
    pub max_bytes: __s32,
    pub dist_delay: __s64,  /* nsec */
    pub dist_jitter: __s64, /* nsec */
}

pub const NETEM_LOSS_UNSPEC: c_uint = 0;
pub const NETEM_LOSS_GI: c_uint = 1; /* General Intuitive - 4 state model */
pub const NETEM_LOSS_GE: c_uint = 2; /* Gilbert Elliot models */
pub const __NETEM_LOSS_MAX: c_uint = 3;
pub const NETEM_LOSS_MAX: c_uint = __NETEM_LOSS_MAX - 1;

/* State transition probabilities for 4 state model */
#[repr(C)]
pub struct tc_netem_gimodel {
    pub p13: __u32,
    pub p31: __u32,
    pub p32: __u32,
    pub p14: __u32,
    pub p23: __u32,
}

/* Gilbert-Elliot models */
#[repr(C)]
pub struct tc_netem_gemodel {
    pub p: __u32,
    pub r: __u32,
    pub h: __u32,
    pub k1: __u32,
}

pub const NETEM_DIST_SCALE: c_uint = 8192;
pub const NETEM_DIST_MAX: c_uint = 16384;

/* DRR */

pub const TCA_DRR_UNSPEC: c_uint = 0;
pub const TCA_DRR_QUANTUM: c_uint = 1;
pub const __TCA_DRR_MAX: c_uint = 2;

pub const TCA_DRR_MAX: c_uint = __TCA_DRR_MAX - 1;

#[repr(C)]
pub struct tc_drr_stats {
    pub deficit: __u32,
}

/* MQPRIO */
pub const TC_QOPT_BITMASK: c_uint = 15;
pub const TC_QOPT_MAX_QUEUE: c_uint = 16;

pub const TC_MQPRIO_HW_OFFLOAD_NONE: c_uint = 0; /* no offload requested */
pub const TC_MQPRIO_HW_OFFLOAD_TCS: c_uint = 1; /* offload TCs, no queue counts */
pub const __TC_MQPRIO_HW_OFFLOAD_MAX: c_uint = 2;

pub const TC_MQPRIO_HW_OFFLOAD_MAX: c_uint = __TC_MQPRIO_HW_OFFLOAD_MAX - 1;

pub const TC_MQPRIO_MODE_DCB: c_uint = 0;
pub const TC_MQPRIO_MODE_CHANNEL: c_uint = 1;
pub const __TC_MQPRIO_MODE_MAX_ENUM: c_uint = 2;

pub const __TC_MQPRIO_MODE_MAX: c_uint = __TC_MQPRIO_MODE_MAX_ENUM - 1;

pub const TC_MQPRIO_SHAPER_DCB: c_uint = 0;
pub const TC_MQPRIO_SHAPER_BW_RATE: c_uint = 1; /* Add new shapers below */
pub const __TC_MQPRIO_SHAPER_MAX_ENUM: c_uint = 2;

pub const __TC_MQPRIO_SHAPER_MAX: c_uint = __TC_MQPRIO_SHAPER_MAX_ENUM - 1;

#[repr(C)]
pub struct tc_mqprio_qopt {
    pub num_tc: __u8,
    pub prio_tc_map: [__u8; (TC_QOPT_BITMASK + 1) as usize],
    pub hw: __u8,
    pub count: [__u16; TC_QOPT_MAX_QUEUE as usize],
    pub offset: [__u16; TC_QOPT_MAX_QUEUE as usize],
}

pub const TC_MQPRIO_F_MODE: c_uint = 0x1;
pub const TC_MQPRIO_F_SHAPER: c_uint = 0x2;
pub const TC_MQPRIO_F_MIN_RATE: c_uint = 0x4;
pub const TC_MQPRIO_F_MAX_RATE: c_uint = 0x8;

pub const TCA_MQPRIO_UNSPEC: c_uint = 0;
pub const TCA_MQPRIO_MODE: c_uint = 1;
pub const TCA_MQPRIO_SHAPER: c_uint = 2;
pub const TCA_MQPRIO_MIN_RATE64: c_uint = 3;
pub const TCA_MQPRIO_MAX_RATE64: c_uint = 4;
pub const __TCA_MQPRIO_MAX: c_uint = 5;

pub const TCA_MQPRIO_MAX: c_uint = __TCA_MQPRIO_MAX - 1;

/* SFB */

pub const TCA_SFB_UNSPEC: c_uint = 0;
pub const TCA_SFB_PARMS: c_uint = 1;
pub const __TCA_SFB_MAX: c_uint = 2;

pub const TCA_SFB_MAX: c_uint = __TCA_SFB_MAX - 1;

/*
 * Note: increment, decrement are Q0.16 fixed-point values.
 */
#[repr(C)]
pub struct tc_sfb_qopt {
    pub rehash_interval: __u32, /* delay between hash move, in ms */
    pub warmup_time: __u32,     /* double buffering warmup time in ms (warmup_time < rehash_interval) */
    pub max: __u32,             /* max len of qlen_min */
    pub bin_size: __u32,        /* maximum queue length per bin */
    pub increment: __u32,       /* probability increment, (d1 in Blue) */
    pub decrement: __u32,       /* probability decrement, (d2 in Blue) */
    pub limit: __u32,           /* max SFB queue length */
    pub penalty_rate: __u32,    /* inelastic flows are rate limited to 'rate' pps */
    pub penalty_burst: __u32,
}

#[repr(C)]
pub struct tc_sfb_xstats {
    pub earlydrop: __u32,
    pub penaltydrop: __u32,
    pub bucketdrop: __u32,
    pub queuedrop: __u32,
    pub childdrop: __u32, /* drops in child qdisc */
    pub marked: __u32,
    pub maxqlen: __u32,
    pub maxprob: __u32,
    pub avgprob: __u32,
}

pub const SFB_MAX_PROB: c_uint = 0xFFFF;

/* QFQ */
pub const TCA_QFQ_UNSPEC: c_uint = 0;
pub const TCA_QFQ_WEIGHT: c_uint = 1;
pub const TCA_QFQ_LMAX: c_uint = 2;
pub const __TCA_QFQ_MAX: c_uint = 3;

pub const TCA_QFQ_MAX: c_uint = __TCA_QFQ_MAX - 1;

#[repr(C)]
pub struct tc_qfq_stats {
    pub weight: __u32,
    pub lmax: __u32,
}

/* CODEL */

pub const TCA_CODEL_UNSPEC: c_uint = 0;
pub const TCA_CODEL_TARGET: c_uint = 1;
pub const TCA_CODEL_LIMIT: c_uint = 2;
pub const TCA_CODEL_INTERVAL: c_uint = 3;
pub const TCA_CODEL_ECN: c_uint = 4;
pub const TCA_CODEL_CE_THRESHOLD: c_uint = 5;
pub const __TCA_CODEL_MAX: c_uint = 6;

pub const TCA_CODEL_MAX: c_uint = __TCA_CODEL_MAX - 1;

#[repr(C)]
pub struct tc_codel_xstats {
    pub maxpacket: __u32,       /* largest packet we've seen so far */
    pub count: __u32,           /* how many drops we've done since the last time we
                                 * entered dropping state
                                 */
    pub lastcount: __u32,       /* count at entry to dropping state */
    pub ldelay: __u32,          /* in-queue delay seen by most recently dequeued packet */
    pub drop_next: __s32,       /* time to drop next packet */
    pub drop_overlimit: __u32,  /* number of time max qdisc packet limit was hit */
    pub ecn_mark: __u32,        /* number of packets we ECN marked instead of dropped */
    pub dropping: __u32,        /* are we in dropping state ? */
    pub ce_mark: __u32,         /* number of CE marked packets because of ce_threshold */
}

/* FQ_CODEL */

pub const TCA_FQ_CODEL_UNSPEC: c_uint = 0;
pub const TCA_FQ_CODEL_TARGET: c_uint = 1;
pub const TCA_FQ_CODEL_LIMIT: c_uint = 2;
pub const TCA_FQ_CODEL_INTERVAL: c_uint = 3;
pub const TCA_FQ_CODEL_ECN: c_uint = 4;
pub const TCA_FQ_CODEL_FLOWS: c_uint = 5;
pub const TCA_FQ_CODEL_QUANTUM: c_uint = 6;
pub const TCA_FQ_CODEL_CE_THRESHOLD: c_uint = 7;
pub const TCA_FQ_CODEL_DROP_BATCH_SIZE: c_uint = 8;
pub const TCA_FQ_CODEL_MEMORY_LIMIT: c_uint = 9;
pub const __TCA_FQ_CODEL_MAX: c_uint = 10;

pub const TCA_FQ_CODEL_MAX: c_uint = __TCA_FQ_CODEL_MAX - 1;

pub const TCA_FQ_CODEL_XSTATS_QDISC: c_uint = 0;
pub const TCA_FQ_CODEL_XSTATS_CLASS: c_uint = 1;

#[repr(C)]
pub struct tc_fq_codel_qd_stats {
    pub maxpacket: __u32,       /* largest packet we've seen so far */
    pub drop_overlimit: __u32,  /* number of time max qdisc
                                 * packet limit was hit
                                 */
    pub ecn_mark: __u32,        /* number of packets we ECN marked
                                 * instead of being dropped
                                 */
    pub new_flow_count: __u32,  /* number of time packets
                                 * created a 'new flow'
                                 */
    pub new_flows_len: __u32,   /* count of flows in new list */
    pub old_flows_len: __u32,   /* count of flows in old list */
    pub ce_mark: __u32,         /* packets above ce_threshold */
    pub memory_usage: __u32,    /* in bytes */
    pub drop_overmemory: __u32,
}

#[repr(C)]
pub struct tc_fq_codel_cl_stats {
    pub deficit: __s32,
    pub ldelay: __u32, /* in-queue delay seen by most recently
                        * dequeued packet
                        */
    pub count: __u32,
    pub lastcount: __u32,
    pub dropping: __u32,
    pub drop_next: __s32,
}

#[repr(C)]
pub union tc_fq_codel_xstats_union {
    pub qdisc_stats: tc_fq_codel_qd_stats,
    pub class_stats: tc_fq_codel_cl_stats,
}

#[repr(C)]
pub struct tc_fq_codel_xstats {
    pub type_: __u32,
    pub u: tc_fq_codel_xstats_union,
}

/* FQ */

pub const TCA_FQ_UNSPEC: c_uint = 0;

pub const TCA_FQ_PLIMIT: c_uint = 1; /* limit of total number of packets in queue */

pub const TCA_FQ_FLOW_PLIMIT: c_uint = 2; /* limit of packets per flow */

pub const TCA_FQ_QUANTUM: c_uint = 3; /* RR quantum */

pub const TCA_FQ_INITIAL_QUANTUM: c_uint = 4; /* RR quantum for new flow */

pub const TCA_FQ_RATE_ENABLE: c_uint = 5; /* enable/disable rate limiting */

pub const TCA_FQ_FLOW_DEFAULT_RATE: c_uint = 6; /* obsolete, do not use */

pub const TCA_FQ_FLOW_MAX_RATE: c_uint = 7; /* per flow max rate */

pub const TCA_FQ_BUCKETS_LOG: c_uint = 8; /* log2(number of buckets) */

pub const TCA_FQ_FLOW_REFILL_DELAY: c_uint = 9; /* flow credit refill delay in usec */

pub const TCA_FQ_ORPHAN_MASK: c_uint = 10; /* mask applied to orphaned skb hashes */

pub const TCA_FQ_LOW_RATE_THRESHOLD: c_uint = 11; /* per packet delay under this rate */

pub const TCA_FQ_CE_THRESHOLD: c_uint = 12; /* DCTCP-like CE-marking threshold */

pub const __TCA_FQ_MAX: c_uint = 13;

pub const TCA_FQ_MAX: c_uint = __TCA_FQ_MAX - 1;

#[repr(C)]
pub struct tc_fq_qd_stats {
    pub gc_flows: __u64,
    pub highprio_packets: __u64,
    pub tcp_retrans: __u64,
    pub throttled: __u64,
    pub flows_plimit: __u64,
    pub pkts_too_long: __u64,
    pub allocation_errors: __u64,
    pub time_next_delayed_flow: __s64,
    pub flows: __u32,
    pub inactive_flows: __u32,
    pub throttled_flows: __u32,
    pub unthrottle_latency_ns: __u32,
    pub ce_mark: __u64, /* packets above ce_threshold */
}

/* Heavy-Hitter Filter */

pub const TCA_HHF_UNSPEC: c_uint = 0;
pub const TCA_HHF_BACKLOG_LIMIT: c_uint = 1;
pub const TCA_HHF_QUANTUM: c_uint = 2;
pub const TCA_HHF_HH_FLOWS_LIMIT: c_uint = 3;
pub const TCA_HHF_RESET_TIMEOUT: c_uint = 4;
pub const TCA_HHF_ADMIT_BYTES: c_uint = 5;
pub const TCA_HHF_EVICT_TIMEOUT: c_uint = 6;
pub const TCA_HHF_NON_HH_WEIGHT: c_uint = 7;
pub const __TCA_HHF_MAX: c_uint = 8;

pub const TCA_HHF_MAX: c_uint = __TCA_HHF_MAX - 1;

#[repr(C)]
pub struct tc_hhf_xstats {
    pub drop_overlimit: __u32, /* number of times max qdisc packet limit
                                * was hit
                                */
    pub hh_overlimit: __u32,   /* number of times max heavy-hitters was hit */
    pub hh_tot_count: __u32,   /* number of captured heavy-hitters so far */
    pub hh_cur_count: __u32,   /* number of current heavy-hitters */
}

/* PIE */
pub const TCA_PIE_UNSPEC: c_uint = 0;
pub const TCA_PIE_TARGET: c_uint = 1;
pub const TCA_PIE_LIMIT: c_uint = 2;
pub const TCA_PIE_TUPDATE: c_uint = 3;
pub const TCA_PIE_ALPHA: c_uint = 4;
pub const TCA_PIE_BETA: c_uint = 5;
pub const TCA_PIE_ECN: c_uint = 6;
pub const TCA_PIE_BYTEMODE: c_uint = 7;
pub const __TCA_PIE_MAX: c_uint = 8;
pub const TCA_PIE_MAX: c_uint = __TCA_PIE_MAX - 1;

#[repr(C)]
pub struct tc_pie_xstats {
    pub prob: __u32,        /* current probability */
    pub delay: __u32,       /* current delay in ms */
    pub avg_dq_rate: __u32, /* current average dq_rate in bits/pie_time */
    pub packets_in: __u32,  /* total number of packets enqueued */
    pub dropped: __u32,     /* packets dropped due to pie_action */
    pub overlimit: __u32,   /* dropped due to lack of space in queue */
    pub maxq: __u32,        /* maximum queue size */
    pub ecn_mark: __u32,    /* packets marked with ecn*/
}

/* CBS */
#[repr(C)]
pub struct tc_cbs_qopt {
    pub offload: __u8,
    pub _pad: [__u8; 3],
    pub hicredit: __s32,
    pub locredit: __s32,
    pub idleslope: __s32,
    pub sendslope: __s32,
}

pub const TCA_CBS_UNSPEC: c_uint = 0;
pub const TCA_CBS_PARMS: c_uint = 1;
pub const __TCA_CBS_MAX: c_uint = 2;

pub const TCA_CBS_MAX: c_uint = __TCA_CBS_MAX - 1;

/* ETF */
#[repr(C)]
pub struct tc_etf_qopt {
    pub delta: __s32,
    pub clockid: __s32,
    pub flags: __u32,
}
pub const TC_ETF_DEADLINE_MODE_ON: c_uint = BIT(0);
pub const TC_ETF_OFFLOAD_ON: c_uint = BIT(1);

pub const TCA_ETF_UNSPEC: c_uint = 0;
pub const TCA_ETF_PARMS: c_uint = 1;
pub const __TCA_ETF_MAX: c_uint = 2;

pub const TCA_ETF_MAX: c_uint = __TCA_ETF_MAX - 1;

/* CAKE */
pub const TCA_CAKE_UNSPEC: c_uint = 0;
pub const TCA_CAKE_PAD: c_uint = 1;
pub const TCA_CAKE_BASE_RATE64: c_uint = 2;
pub const TCA_CAKE_DIFFSERV_MODE: c_uint = 3;
pub const TCA_CAKE_ATM: c_uint = 4;
pub const TCA_CAKE_FLOW_MODE: c_uint = 5;
pub const TCA_CAKE_OVERHEAD: c_uint = 6;
pub const TCA_CAKE_RTT: c_uint = 7;
pub const TCA_CAKE_TARGET: c_uint = 8;
pub const TCA_CAKE_AUTORATE: c_uint = 9;
pub const TCA_CAKE_MEMORY: c_uint = 10;
pub const TCA_CAKE_NAT: c_uint = 11;
pub const TCA_CAKE_RAW: c_uint = 12;
pub const TCA_CAKE_WASH: c_uint = 13;
pub const TCA_CAKE_MPU: c_uint = 14;
pub const TCA_CAKE_INGRESS: c_uint = 15;
pub const TCA_CAKE_ACK_FILTER: c_uint = 16;
pub const TCA_CAKE_SPLIT_GSO: c_uint = 17;
pub const __TCA_CAKE_MAX: c_uint = 18;
pub const TCA_CAKE_MAX: c_uint = __TCA_CAKE_MAX - 1;

pub const __TCA_CAKE_STATS_INVALID: c_uint = 0;
pub const TCA_CAKE_STATS_PAD: c_uint = 1;
pub const TCA_CAKE_STATS_CAPACITY_ESTIMATE64: c_uint = 2;
pub const TCA_CAKE_STATS_MEMORY_LIMIT: c_uint = 3;
pub const TCA_CAKE_STATS_MEMORY_USED: c_uint = 4;
pub const TCA_CAKE_STATS_AVG_NETOFF: c_uint = 5;
pub const TCA_CAKE_STATS_MIN_NETLEN: c_uint = 6;
pub const TCA_CAKE_STATS_MAX_NETLEN: c_uint = 7;
pub const TCA_CAKE_STATS_MIN_ADJLEN: c_uint = 8;
pub const TCA_CAKE_STATS_MAX_ADJLEN: c_uint = 9;
pub const TCA_CAKE_STATS_TIN_STATS: c_uint = 10;
pub const TCA_CAKE_STATS_DEFICIT: c_uint = 11;
pub const TCA_CAKE_STATS_COBALT_COUNT: c_uint = 12;
pub const TCA_CAKE_STATS_DROPPING: c_uint = 13;
pub const TCA_CAKE_STATS_DROP_NEXT_US: c_uint = 14;
pub const TCA_CAKE_STATS_P_DROP: c_uint = 15;
pub const TCA_CAKE_STATS_BLUE_TIMER_US: c_uint = 16;
pub const __TCA_CAKE_STATS_MAX: c_uint = 17;
pub const TCA_CAKE_STATS_MAX: c_uint = __TCA_CAKE_STATS_MAX - 1;

pub const __TCA_CAKE_TIN_STATS_INVALID: c_uint = 0;
pub const TCA_CAKE_TIN_STATS_PAD: c_uint = 1;
pub const TCA_CAKE_TIN_STATS_SENT_PACKETS: c_uint = 2;
pub const TCA_CAKE_TIN_STATS_SENT_BYTES64: c_uint = 3;
pub const TCA_CAKE_TIN_STATS_DROPPED_PACKETS: c_uint = 4;
pub const TCA_CAKE_TIN_STATS_DROPPED_BYTES64: c_uint = 5;
pub const TCA_CAKE_TIN_STATS_ACKS_DROPPED_PACKETS: c_uint = 6;
pub const TCA_CAKE_TIN_STATS_ACKS_DROPPED_BYTES64: c_uint = 7;
pub const TCA_CAKE_TIN_STATS_ECN_MARKED_PACKETS: c_uint = 8;
pub const TCA_CAKE_TIN_STATS_ECN_MARKED_BYTES64: c_uint = 9;
pub const TCA_CAKE_TIN_STATS_BACKLOG_PACKETS: c_uint = 10;
pub const TCA_CAKE_TIN_STATS_BACKLOG_BYTES: c_uint = 11;
pub const TCA_CAKE_TIN_STATS_THRESHOLD_RATE64: c_uint = 12;
pub const TCA_CAKE_TIN_STATS_TARGET_US: c_uint = 13;
pub const TCA_CAKE_TIN_STATS_INTERVAL_US: c_uint = 14;
pub const TCA_CAKE_TIN_STATS_WAY_INDIRECT_HITS: c_uint = 15;
pub const TCA_CAKE_TIN_STATS_WAY_MISSES: c_uint = 16;
pub const TCA_CAKE_TIN_STATS_WAY_COLLISIONS: c_uint = 17;
pub const TCA_CAKE_TIN_STATS_PEAK_DELAY_US: c_uint = 18;
pub const TCA_CAKE_TIN_STATS_AVG_DELAY_US: c_uint = 19;
pub const TCA_CAKE_TIN_STATS_BASE_DELAY_US: c_uint = 20;
pub const TCA_CAKE_TIN_STATS_SPARSE_FLOWS: c_uint = 21;
pub const TCA_CAKE_TIN_STATS_BULK_FLOWS: c_uint = 22;
pub const TCA_CAKE_TIN_STATS_UNRESPONSIVE_FLOWS: c_uint = 23;
pub const TCA_CAKE_TIN_STATS_MAX_SKBLEN: c_uint = 24;
pub const TCA_CAKE_TIN_STATS_FLOW_QUANTUM: c_uint = 25;
pub const __TCA_CAKE_TIN_STATS_MAX: c_uint = 26;
pub const TCA_CAKE_TIN_STATS_MAX: c_uint = __TCA_CAKE_TIN_STATS_MAX - 1;
pub const TC_CAKE_MAX_TINS: c_uint = 8;

pub const CAKE_FLOW_NONE: c_uint = 0;
pub const CAKE_FLOW_SRC_IP: c_uint = 1;
pub const CAKE_FLOW_DST_IP: c_uint = 2;
pub const CAKE_FLOW_HOSTS: c_uint = 3; /* = CAKE_FLOW_SRC_IP | CAKE_FLOW_DST_IP */
pub const CAKE_FLOW_FLOWS: c_uint = 4;
pub const CAKE_FLOW_DUAL_SRC: c_uint = 5; /* = CAKE_FLOW_SRC_IP | CAKE_FLOW_FLOWS */
pub const CAKE_FLOW_DUAL_DST: c_uint = 6; /* = CAKE_FLOW_DST_IP | CAKE_FLOW_FLOWS */
pub const CAKE_FLOW_TRIPLE: c_uint = 7; /* = CAKE_FLOW_HOSTS  | CAKE_FLOW_FLOWS */
pub const CAKE_FLOW_MAX: c_uint = 8;

pub const CAKE_DIFFSERV_DIFFSERV3: c_uint = 0;
pub const CAKE_DIFFSERV_DIFFSERV4: c_uint = 1;
pub const CAKE_DIFFSERV_DIFFSERV8: c_uint = 2;
pub const CAKE_DIFFSERV_BESTEFFORT: c_uint = 3;
pub const CAKE_DIFFSERV_PRECEDENCE: c_uint = 4;
pub const CAKE_DIFFSERV_MAX: c_uint = 5;

pub const CAKE_ACK_NONE: c_uint = 0;
pub const CAKE_ACK_FILTER: c_uint = 1;
pub const CAKE_ACK_AGGRESSIVE: c_uint = 2;
pub const CAKE_ACK_MAX: c_uint = 3;

pub const CAKE_ATM_NONE: c_uint = 0;
pub const CAKE_ATM_ATM: c_uint = 1;
pub const CAKE_ATM_PTM: c_uint = 2;
pub const CAKE_ATM_MAX: c_uint = 3;

/* TAPRIO */
pub const TC_TAPRIO_CMD_SET_GATES: c_uint = 0x00;
pub const TC_TAPRIO_CMD_SET_AND_HOLD: c_uint = 0x01;
pub const TC_TAPRIO_CMD_SET_AND_RELEASE: c_uint = 0x02;

pub const TCA_TAPRIO_SCHED_ENTRY_UNSPEC: c_uint = 0;
pub const TCA_TAPRIO_SCHED_ENTRY_INDEX: c_uint = 1; /* u32 */
pub const TCA_TAPRIO_SCHED_ENTRY_CMD: c_uint = 2; /* u8 */
pub const TCA_TAPRIO_SCHED_ENTRY_GATE_MASK: c_uint = 3; /* u32 */
pub const TCA_TAPRIO_SCHED_ENTRY_INTERVAL: c_uint = 4; /* u32 */
pub const __TCA_TAPRIO_SCHED_ENTRY_MAX: c_uint = 5;
pub const TCA_TAPRIO_SCHED_ENTRY_MAX: c_uint = __TCA_TAPRIO_SCHED_ENTRY_MAX - 1;

/* The format for schedule entry list is:
 * [TCA_TAPRIO_SCHED_ENTRY_LIST]
 *   [TCA_TAPRIO_SCHED_ENTRY]
 *     [TCA_TAPRIO_SCHED_ENTRY_CMD]
 *     [TCA_TAPRIO_SCHED_ENTRY_GATES]
 *     [TCA_TAPRIO_SCHED_ENTRY_INTERVAL]
 */
pub const TCA_TAPRIO_SCHED_UNSPEC: c_uint = 0;
pub const TCA_TAPRIO_SCHED_ENTRY: c_uint = 1;
pub const __TCA_TAPRIO_SCHED_MAX: c_uint = 2;

pub const TCA_TAPRIO_SCHED_MAX: c_uint = __TCA_TAPRIO_SCHED_MAX - 1;

pub const TCA_TAPRIO_ATTR_UNSPEC: c_uint = 0;
pub const TCA_TAPRIO_ATTR_PRIOMAP: c_uint = 1; /* struct tc_mqprio_qopt */
pub const TCA_TAPRIO_ATTR_SCHED_ENTRY_LIST: c_uint = 2; /* nested of entry */
pub const TCA_TAPRIO_ATTR_SCHED_BASE_TIME: c_uint = 3; /* s64 */
pub const TCA_TAPRIO_ATTR_SCHED_SINGLE_ENTRY: c_uint = 4; /* single entry */
pub const TCA_TAPRIO_ATTR_SCHED_CLOCKID: c_uint = 5; /* s32 */
pub const TCA_TAPRIO_PAD: c_uint = 6;
pub const __TCA_TAPRIO_ATTR_MAX: c_uint = 7;

pub const TCA_TAPRIO_ATTR_MAX: c_uint = __TCA_TAPRIO_ATTR_MAX - 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
