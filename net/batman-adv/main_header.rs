/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

pub const BATADV_DRIVER_AUTHOR: &str = "Marek Lindner <marek.lindner@mailbox.org>, Simon Wunderlich <sw@simonwunderlich.de>";
pub const BATADV_DRIVER_DESC: &str = "B.A.T.M.A.N. advanced";
pub const BATADV_DRIVER_DEVICE: &str = "batman-adv";

pub const BATADV_TQ_MAX_VALUE: u32 = 255;
pub const BATADV_THROUGHPUT_MAX_VALUE: u32 = 0xFFFF_FFFF;
pub const BATADV_JITTER: u32 = 20;
pub const BATADV_MAX_MTU: usize = ETH_MAX_MTU - batadv_max_header_len() as usize;
pub const BATADV_TTL: u32 = 50;
pub const BATADV_BCAST_MAX_AGE: u32 = 64;
pub const BATADV_PURGE_TIMEOUT: u32 = 200000;
pub const BATADV_TT_LOCAL_TIMEOUT: u32 = 600000;
pub const BATADV_TT_CLIENT_ROAM_TIMEOUT: u32 = 600000;
pub const BATADV_TT_CLIENT_TEMP_TIMEOUT: u32 = 600000;
pub const BATADV_TT_WORK_PERIOD: u32 = 5000;
pub const BATADV_ORIG_WORK_PERIOD: u32 = 1000;
pub const BATADV_MCAST_WORK_PERIOD: u32 = 500;
pub const BATADV_DAT_ENTRY_TIMEOUT: u32 = 5 * 60000;
pub const BATADV_TQ_LOCAL_WINDOW_SIZE: u32 = 64;
pub const BATADV_TT_REQUEST_TIMEOUT: u32 = 3000;
pub const BATADV_TQ_GLOBAL_WINDOW_SIZE: u32 = 5;
pub const BATADV_TQ_LOCAL_BIDRECT_SEND_MINIMUM: u32 = 1;
pub const BATADV_TQ_LOCAL_BIDRECT_RECV_MINIMUM: u32 = 1;
pub const BATADV_TQ_TOTAL_BIDRECT_LIMIT: u32 = 1;
pub const BATADV_THROUGHPUT_DEFAULT_VALUE: u32 = 10;
pub const BATADV_ELP_PROBES_PER_NODE: u32 = 2;
pub const BATADV_ELP_MIN_PROBE_SIZE: u32 = 200;
pub const BATADV_ELP_PROBE_MAX_TX_DIFF: u32 = 100;
pub const BATADV_ELP_MAX_AGE: u32 = 64;
pub const BATADV_OGM_MAX_ORIGDIFF: u32 = 5;
pub const BATADV_OGM_MAX_AGE: u32 = 64;
pub const BATADV_TT_OGM_APPEND_MAX: u32 = 3;
pub const BATADV_ROAMING_MAX_TIME: u32 = 20000;
pub const BATADV_ROAMING_MAX_COUNT: u32 = 5;
pub const BATADV_NO_FLAGS: u32 = 0;
pub const BATADV_NULL_IFINDEX: u32 = 0;
pub const BATADV_NO_MARK: u32 = 0;
pub const BATADV_BCAST_QUEUE_LEN: u32 = 256;
pub const BATADV_BATMAN_QUEUE_LEN: u32 = 256;
pub const BATADV_NUM_BCASTS_DEFAULT: u32 = 1;
pub const BATADV_NUM_BCASTS_WIRELESS: u32 = 3;
pub const BATADV_TP_PACKET_LEN: usize = ETH_DATA_LEN;
pub const ARP_REQ_DELAY: u32 = 250;
pub const BATADV_DAT_CANDIDATES_NUM: u32 = 3;
pub const BATADV_TQ_SIMILARITY_THRESHOLD: u32 = 50;
pub const BATADV_MAX_AGGREGATION_PACKETS: u32 = 32;
pub const BATADV_MAX_AGGREGATION_BYTES: u32 = 512;
pub const BATADV_MAX_AGGREGATION_MS: u32 = 100;
pub const BATADV_BLA_PERIOD_LENGTH: u32 = 10000;
pub const BATADV_BLA_BACKBONE_TIMEOUT: u32 = BATADV_BLA_PERIOD_LENGTH * 6;
pub const BATADV_BLA_CLAIM_TIMEOUT: u32 = BATADV_BLA_PERIOD_LENGTH * 10;
pub const BATADV_BLA_WAIT_PERIODS: u32 = 3;
pub const BATADV_BLA_LOOPDETECT_PERIODS: u32 = 6;
pub const BATADV_BLA_LOOPDETECT_TIMEOUT: u32 = 3000;
pub const BATADV_DUPLIST_SIZE: u32 = 16;
pub const BATADV_DUPLIST_TIMEOUT: u32 = 500;
pub const BATADV_RESET_PROTECTION_MS: u32 = 30000;
pub const BATADV_EXPECTED_SEQNO_RANGE: u32 = 65536;
pub const BATADV_GW_THRESHOLD: u32 = 50;
pub const BATADV_FRAG_BUFFER_COUNT: u32 = 8;
pub const BATADV_FRAG_MAX_FRAGMENTS: u32 = 16;
pub const BATADV_FRAG_MAX_FRAG_SIZE: u32 = 1280;
pub const BATADV_FRAG_TIMEOUT: u32 = 10000;
pub const BATADV_DAT_CANDIDATE_NOT_FOUND: u32 = 0;
pub const BATADV_DAT_CANDIDATE_ORIG: u32 = 1;
pub const BATADV_TP_MAX_NUM: u32 = 5;
pub const BATADV_LOG_BUF_LEN: u32 = 8192;
pub const BATADV_NUM_WORDS: usize = BITS_TO_LONGS(BATADV_TQ_LOCAL_WINDOW_SIZE);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum batadv_mesh_state { BATADV_MESH_INACTIVE, BATADV_MESH_ACTIVE, BATADV_MESH_DEACTIVATING }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum batadv_uev_action { BATADV_UEV_ADD = 0, BATADV_UEV_DEL, BATADV_UEV_CHANGE, BATADV_UEV_LOOPDETECT }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum batadv_uev_type { BATADV_UEV_GW = 0, BATADV_UEV_BLA }

/* Build-time/kernel dependencies supplied by other translation units. */
extern "C" {
    pub static mut batadv_event_workqueue: *mut workqueue_struct;
    pub fn batadv_mesh_init(mesh_iface: *mut net_device) -> i32;
    pub fn batadv_mesh_free(mesh_iface: *mut net_device);
    pub fn batadv_is_my_mac(bat_priv: *mut batadv_priv, addr: *const u8) -> bool;
    pub fn batadv_max_header_len() -> i32;
    pub fn batadv_skb_set_priority(skb: *mut sk_buff, offset: i32);
    pub fn batadv_batman_skb_recv(skb: *mut sk_buff, dev: *mut net_device, ptype: *mut packet_type, orig_dev: *mut net_device) -> i32;
    pub fn batadv_recv_handler_register(packet_type: u8, recv_handler: Option<unsafe extern "C" fn(*mut sk_buff, *mut batadv_hard_iface) -> i32>) -> i32;
    pub fn batadv_recv_handler_unregister(packet_type: u8);
    pub fn batadv_get_vid(skb: *mut sk_buff, header_len: usize) -> u16;
    pub fn batadv_vlan_ap_isola_get(bat_priv: *mut batadv_priv, vid: u16) -> bool;
    pub fn batadv_throw_uevent(bat_priv: *mut batadv_priv, type_: batadv_uev_type, action: batadv_uev_action, data: *const i8) -> i32;
}

pub type c_void = core::ffi::c_void;
#[allow(non_camel_case_types)] pub enum workqueue_struct {}
#[allow(non_camel_case_types)] pub enum net_device {}
#[allow(non_camel_case_types)] pub enum sk_buff {}
#[allow(non_camel_case_types)] pub enum packet_type {}
#[allow(non_camel_case_types)] pub enum batadv_priv {}
#[allow(non_camel_case_types)] pub enum batadv_hard_iface {}

#[inline]
pub unsafe fn batadv_print_vid(vid: u16) -> i32 {
    if vid & BATADV_VLAN_HAS_TAG != 0 { (vid & VLAN_VID_MASK) as i32 } else { -1 }
}

#[inline]
pub unsafe fn batadv_compare_eth(data1: *const c_void, data2: *const c_void) -> bool {
    ether_addr_equal_unaligned(data1, data2)
}

#[inline]
pub unsafe fn batadv_has_timed_out(timestamp: usize, timeout: u32) -> bool {
    time_is_before_jiffies(timestamp.wrapping_add(msecs_to_jiffies(timeout)))
}

#[inline]
pub const fn batadv_smallest_signed_int<T>() -> T { panic!("type-level C integer macro requires a concrete integer type") }

/* The C macro preserves the operand's integer type and uses wrapping arithmetic. */
#[macro_export]
macro_rules! batadv_seq_before { ($x:expr, $y:expr) => {{ let d = ($x).wrapping_sub($y); d > (1 as _ << (7 + 8 * (core::mem::size_of_val(&d) - 1))) }} }
#[macro_export]
macro_rules! batadv_seq_after { ($x:expr, $y:expr) => { $crate::batadv_seq_before!($y, $x) } }

/* Kernel macros retained as source-level Rust hooks. */
#[macro_export] macro_rules! batadv_atomic_dec_not_zero { ($v:expr) => { atomic_add_unless($v, -1, 0) } }
#[macro_export] macro_rules! batadv_inc_counter { ($b:expr, $i:expr) => { batadv_add_counter($b, $i, 1) } }
#[macro_export] macro_rules! BATADV_SKB_CB { ($skb:expr) => { &mut (*$skb).cb[0] as *mut _ } }

pub unsafe fn batadv_add_counter(_bat_priv: *mut batadv_priv, _idx: usize, _count: usize) {
    /* this_cpu_add(bat_priv->bat_counters[idx], count); */
    unimplemented!()
}

extern "C" {
    fn ether_addr_equal_unaligned(data1: *const c_void, data2: *const c_void) -> bool;
    fn time_is_before_jiffies(value: usize) -> bool;
    fn msecs_to_jiffies(timeout: u32) -> usize;
    fn atomic_add_unless(v: *mut c_void, a: i32, unless: i32) -> bool;
}

/* External constants/macros from kernel and packet headers. */
extern "C" { static BATADV_VLAN_HAS_TAG: u16; static VLAN_VID_MASK: u16; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
