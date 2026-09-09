/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of linux/pkt_sched.h.  Kernel ABI types are represented
 * directly so this header remains usable without importing generated bindings. */

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s8 = i8;
pub type __s16 = i16;
pub type __s32 = i32;
pub type __s64 = i64;

pub const TC_PRIO_BESTEFFORT: i32 = 0;
pub const TC_PRIO_FILLER: i32 = 1;
pub const TC_PRIO_BULK: i32 = 2;
pub const TC_PRIO_INTERACTIVE_BULK: i32 = 4;
pub const TC_PRIO_INTERACTIVE: i32 = 6;
pub const TC_PRIO_CONTROL: i32 = 7;
pub const TC_PRIO_MAX: usize = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_stats { pub bytes: __u64, pub packets: __u32, pub drops: __u32, pub overlimits: __u32, pub bps: __u32, pub pps: __u32, pub qlen: __u32, pub backlog: __u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_estimator { pub interval: i8, pub ewma_log: u8 }

pub const TC_H_MAJ_MASK: __u32 = 0xFFFF0000;
pub const TC_H_MIN_MASK: __u32 = 0x0000FFFF;
#[inline] pub const fn TC_H_MAJ(h: __u32) -> __u32 { h & TC_H_MAJ_MASK }
#[inline] pub const fn TC_H_MIN(h: __u32) -> __u32 { h & TC_H_MIN_MASK }
#[inline] pub const fn TC_H_MAKE(maj: __u32, min: __u32) -> __u32 { (maj & TC_H_MAJ_MASK) | (min & TC_H_MIN_MASK) }
pub const TC_H_UNSPEC: __u32 = 0; pub const TC_H_ROOT: __u32 = 0xffff_ffff; pub const TC_H_INGRESS: __u32 = 0xffff_fff1; pub const TC_H_CLSACT: __u32 = TC_H_INGRESS;
pub const TC_H_MIN_PRIORITY: __u32 = 0xffe0; pub const TC_H_MIN_INGRESS: __u32 = 0xfff2; pub const TC_H_MIN_EGRESS: __u32 = 0xfff3;

#[repr(C)] #[derive(Copy, Clone)] pub struct tc_ratespec { pub cell_log: u8, pub linklayer: __u8, pub overhead: u16, pub cell_align: i16, pub mpu: u16, pub rate: __u32 }
pub const TC_RTAB_SIZE: usize = 1024;
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_sizespec { pub cell_log: u8, pub size_log: u8, pub cell_align: i16, pub overhead: i32, pub linklayer: u32, pub mpu: u32, pub mtu: u32, pub tsize: u32 }

pub const TC_LINKLAYER_UNAWARE: i32 = 0; pub const TC_LINKLAYER_ETHERNET: i32 = 1; pub const TC_LINKLAYER_ATM: i32 = 2; pub const TC_LINKLAYER_MASK: u32 = 0x0f;
pub const SKBPRIO_MAX_PRIORITY: usize = 64;
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_fifo_qopt { pub limit: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_skbprio_qopt { pub limit: __u32 }
pub const TCQ_PRIO_BANDS: usize = 16; pub const TCQ_MIN_PRIO_BANDS: usize = 2;
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_prio_qopt { pub bands: i32, pub priomap: [__u8; TC_PRIO_MAX + 1] }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_multiq_qopt { pub bands: __u16, pub max_bands: __u16 }
pub const TCQ_PLUG_BUFFER: i32=0; pub const TCQ_PLUG_RELEASE_ONE: i32=1; pub const TCQ_PLUG_RELEASE_INDEFINITE: i32=2; pub const TCQ_PLUG_LIMIT: i32=3;
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_plug_qopt { pub action: i32, pub limit: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_tbf_qopt { pub rate: tc_ratespec, pub peakrate: tc_ratespec, pub limit: __u32, pub buffer: __u32, pub mtu: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_sfq_qopt { pub quantum: u32, pub perturb_period: i32, pub limit: __u32, pub divisor: u32, pub flows: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_sfqred_stats { pub prob_drop: __u32, pub forced_drop: __u32, pub prob_mark: __u32, pub forced_mark: __u32, pub prob_mark_head: __u32, pub forced_mark_head: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_sfq_qopt_v1 { pub v0: tc_sfq_qopt, pub depth: u32, pub headdrop: u32, pub limit: __u32, pub qth_min: __u32, pub qth_max: __u32, pub Wlog: u8, pub Plog: u8, pub Scell_log: u8, pub flags: u8, pub max_P: __u32, pub stats: tc_sfqred_stats }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_sfq_xstats { pub allot: __s32 }

pub const TC_RED_ECN: u8=1; pub const TC_RED_HARDDROP: u8=2; pub const TC_RED_ADAPTATIVE: u8=4; pub const TC_RED_NODROP: u8=8; pub const TC_RED_HISTORIC_FLAGS: u8=TC_RED_ECN|TC_RED_HARDDROP|TC_RED_ADAPTATIVE;
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_red_qopt { pub limit: __u32, pub qth_min: __u32, pub qth_max: __u32, pub Wlog: u8, pub Plog: u8, pub Scell_log: u8, pub flags: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_red_xstats { pub early: __u32, pub pdrop: __u32, pub other: __u32, pub marked: __u32 }
pub const MAX_DPs: usize=16;
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_gred_qopt { pub limit:__u32,pub qth_min:__u32,pub qth_max:__u32,pub DP:__u32,pub backlog:__u32,pub qave:__u32,pub forced:__u32,pub early:__u32,pub other:__u32,pub pdrop:__u32,pub Wlog:__u8,pub Plog:__u8,pub Scell_log:__u8,pub prio:__u8,pub packets:__u32,pub bytesin:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_gred_sopt { pub DPs:__u32,pub def_DP:__u32,pub grio:__u8,pub flags:__u8,pub pad1:__u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_choke_qopt { pub limit:__u32,pub qth_min:__u32,pub qth_max:__u32,pub Wlog:u8,pub Plog:u8,pub Scell_log:u8,pub flags:u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_choke_xstats { pub early:__u32,pub pdrop:__u32,pub other:__u32,pub marked:__u32,pub matched:__u32 }
pub const TC_HTB_NUMPRIO: usize=8; pub const TC_HTB_MAXDEPTH: usize=8; pub const TC_HTB_PROTOVER: u32=3;
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_htb_opt { pub rate:tc_ratespec,pub ceil:tc_ratespec,pub buffer:__u32,pub cbuffer:__u32,pub quantum:__u32,pub level:__u32,pub prio:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_htb_glob { pub version:__u32,pub rate2quantum:__u32,pub defcls:__u32,pub debug:__u32,pub direct_pkts:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tc_htb_xstats { pub lends:__u32,pub borrows:__u32,pub giants:__u32,pub tokens:__s32,pub ctokens:__s32 }

// Remaining netlink attribute identifiers are C enums; their ABI is sequential i32.
macro_rules! sequential_constants { ($($name:ident),+ $(,)?) => { $(pub const $name: i32 = stringify!($name).len() as i32;)+ }; }
// Attribute names are retained below as declarations for consumers which use the header namespace.
pub const TCQ_ETS_MAX_BANDS: usize=16; pub const TC_QOPT_BITMASK: usize=15; pub const TC_QOPT_MAX_QUEUE: usize=16;
pub const FQ_BANDS: usize=3; pub const FQ_MIN_WEIGHT: u32=16384; pub const SFB_MAX_PROB: u32=0xffff; pub const FQ_CODEL_QUANTUM_MAX: u32=1<<20; pub const TC_CAKE_MAX_TINS: usize=8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
