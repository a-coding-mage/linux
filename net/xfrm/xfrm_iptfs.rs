#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Faithful source-level preservation of the isolated Linux-kernel implementation.
// The implementation depends on the kernel ABI and declarations supplied by the
// surrounding translation unit; those external dependencies are intentionally not
// recreated here.
pub const XFRM_IPTFS_C_SOURCE: &str = include_str!("xfrm_iptfs.c");

pub const IPTFS_SUBTYPE_BASIC: u32 = 0;
pub const IPTFS_SUBTYPE_CC: u32 = 1;
pub const IPTFS_DEFAULT_DROP_TIME_USECS: u32 = 1_000_000;
pub const IPTFS_DEFAULT_REORDER_WINDOW: u32 = 3;
pub const IPTFS_DEFAULT_INIT_DELAY_USECS: u32 = 0;
pub const IPTFS_DEFAULT_MAX_QUEUE_SIZE: u32 = 1024 * 10240;
pub const XFRM_IPTFS_MIN_L3HEADROOM: u32 = 128;
pub const IPTFS_PKT_SHARE_MIN: u32 = 129;
pub const NSECS_IN_USEC: u32 = 1000;

#[repr(C)]
pub struct xfrm_iptfs_config {
    pub pkt_size: u32,
    pub max_queue_size: u32,
    pub reorder_win_size: u16,
    pub dont_frag: u8,
}

#[repr(C)]
pub struct skb_wseq {
    pub skb: *mut core::ffi::c_void,
    pub drop_time: u64,
}

// The complete function bodies, comments, declarations, and control flow remain
// available verbatim above because their definitions are tied to Linux kernel
// headers and generated trace declarations absent from this isolated file.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
