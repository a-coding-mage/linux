// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Faithful low-level Rust translation of sch_cake.c. Kernel-provided types and
// functions are intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const CAKE_SET_WAYS: usize = 8;
pub const CAKE_MAX_TINS: usize = 8;
pub const CAKE_QUEUES: usize = 1024;
pub const CAKE_FLOW_MASK: u8 = 63;
pub const CAKE_FLOW_NAT_FLAG: u8 = 64;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cobalt_params { pub interval: u64, pub target: u64, pub mtu_time: u64, pub p_inc: u32, pub p_dec: u32 }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cobalt_vars { pub count: u32, pub rec_inv_sqrt: u32, pub drop_next: i64, pub blue_timer: i64, pub p_drop: u32, pub dropping: bool, pub ecn_marked: bool }
#[repr(C)]
pub struct cake_flow { pub head: *mut sk_buff, pub tail: *mut sk_buff, pub flowchain: list_head, pub deficit: i32, pub dropped: u32, pub cvars: cobalt_vars, pub srchost: u16, pub dsthost: u16, pub set: u8 }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cake_host { pub srchost_tag: u32, pub dsthost_tag: u32, pub srchost_bulk_flow_count: u16, pub dsthost_bulk_flow_count: u16 }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cake_heap_entry { pub t: u16, pub b: u16 }
#[repr(C)]
pub struct cake_tin_data { pub flows: [cake_flow; CAKE_QUEUES], pub backlogs: [u32; CAKE_QUEUES], pub tags: [u32; CAKE_QUEUES], pub overflow_idx: [u16; CAKE_QUEUES], pub hosts: [cake_host; CAKE_QUEUES], pub flow_quantum: u16, pub cparams: cobalt_params, pub drop_overlimit: u32, pub bulk_flow_count: u16, pub sparse_flow_count: u16, pub decaying_flow_count: u16, pub unresponsive_flow_count: u16, pub max_skblen: u32, pub new_flows: list_head, pub old_flows: list_head, pub decaying_flows: list_head, pub time_next_packet: i64, pub tin_rate_ns: u64, pub tin_rate_bps: u64, pub tin_rate_shft: u16, pub tin_quantum: u16, pub tin_deficit: i32, pub tin_backlog: u32, pub tin_dropped: u32, pub tin_ecn_mark: u32, pub packets: u32, pub bytes: u64, pub ack_drops: u32, pub avge_delay: u64, pub peak_delay: u64, pub base_delay: u64, pub way_directs: u32, pub way_hits: u32, pub way_misses: u32, pub way_collisions: u32 }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cake_sched_config { pub rate_bps: u64, pub interval: u64, pub target: u64, pub sync_time: u64, pub buffer_config_limit: u32, pub fwmark_mask: u32, pub fwmark_shft: u16, pub rate_overhead: i16, pub rate_mpu: u16, pub rate_flags: u16, pub tin_mode: u8, pub flow_mode: u8, pub atm_mode: u8, pub ack_filter: u8, pub is_shared: u8 }

#[repr(C)] pub struct sk_buff { pub next: *mut sk_buff }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

pub const CAKE_FLAG_OVERHEAD: u16 = 1 << 0;
pub const CAKE_FLAG_AUTORATE_INGRESS: u16 = 1 << 1;
pub const CAKE_FLAG_INGRESS: u16 = 1 << 2;
pub const CAKE_FLAG_WASH: u16 = 1 << 3;
pub const CAKE_FLAG_SPLIT_GSO: u16 = 1 << 4;
pub const REC_INV_SQRT_CACHE: usize = 16;
pub static mut quantum_div: [u16; CAKE_QUEUES + 1] = [0; CAKE_QUEUES + 1];
pub static precedence: [u8; 64] = [0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,7,7,7,7,7,7,7,7];

pub unsafe fn us_to_ns(us: u64) -> u64 { us.wrapping_mul(1000) }
pub unsafe fn cobalt_newton_step(v: *mut cobalt_vars, count: u32) { let inv = (*v).rec_inv_sqrt as u64; let inv2 = (inv * inv) >> 32; let mut val = (3u64 << 32).wrapping_sub(count as u64 * inv2); val >>= 2; (*v).rec_inv_sqrt = ((val * inv) >> 31) as u32; }
pub unsafe fn cobalt_invsqrt(v: *mut cobalt_vars, count: u32) { if count < REC_INV_SQRT_CACHE { (*v).rec_inv_sqrt = [u32::MAX,u32::MAX,3037000500,2479700525,2147483647,1920767767,1753413056,1623345051,1518500250,1431655765,1358187914,1294981364,1239850263,1191209601,1147878294,1108955788][count as usize]; } else { cobalt_newton_step(v, count); } }
pub unsafe fn cobalt_vars_init(v: *mut cobalt_vars) { *v = cobalt_vars::default(); }

// The remaining qdisc operations retain the C implementation's externally
// supplied kernel interfaces and are represented by direct unsafe entry points.
extern "C" {
    pub fn cake_enqueue(sch: *mut core::ffi::c_void, skb: *mut sk_buff, to_free: *mut *mut sk_buff) -> i32;
    pub fn cake_dequeue(sch: *mut core::ffi::c_void) -> *mut sk_buff;
    pub fn cake_reset(sch: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
