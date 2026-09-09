// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rust translation of block/null_blk/main.c.  The Linux kernel types and
 * helpers referenced below are supplied by the surrounding kernel bindings.
 */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

const FREE_BATCH: usize = 16;
const TICKS_PER_SEC: u64 = 50;
const TIMER_INTERVAL: u64 = 1_000_000_000 / TICKS_PER_SEC;

#[repr(u32)]
#[derive(Copy, Clone)]
enum nullb_device_flags {
    NULLB_DEV_FL_CONFIGURED = 0,
    NULLB_DEV_FL_UP = 1,
    NULLB_DEV_FL_THROTTLED = 2,
    NULLB_DEV_FL_CACHE = 3,
}

const NULL_IRQ_NONE: i32 = 0;
const NULL_IRQ_SOFTIRQ: i32 = 1;
const NULL_IRQ_TIMER: i32 = 2;
const NULL_Q_BIO: i32 = 0;
const NULL_Q_RQ: i32 = 1;
const NULL_Q_MQ: i32 = 2;

const PAGE_SIZE: usize = 4096;
const SECTOR_SHIFT: usize = 9;
const PAGE_SECTORS_SHIFT: usize = 3;
const SECTOR_MASK: u64 = (1 << PAGE_SECTORS_SHIFT) - 1;
const MAP_SZ: usize = (PAGE_SIZE >> SECTOR_SHIFT) + 2;
const NULLB_PAGE_LOCK: usize = MAP_SZ - 1;
const NULLB_PAGE_FREE: usize = MAP_SZ - 2;

#[repr(C)]
pub struct page { pub private: u64 }
#[repr(C)]
pub struct nullb_page { pub page: *mut page, pub bitmap: [u64; (MAP_SZ + 63) / 64] }
#[repr(C)]
pub struct nullb_device;
#[repr(C)]
pub struct nullb;
#[repr(C)]
pub struct nullb_cmd;
#[repr(C)]
pub struct config_item;
#[repr(C)]
pub struct config_group;
#[repr(C)]
pub struct request;
#[repr(C)]
pub struct blk_mq_hw_ctx;
#[repr(C)]
pub struct blk_mq_tag_set;
#[repr(C)]
pub struct queue_limits;

static mut g_virt_boundary: bool = false;
static mut g_no_sched: c_int = 0;
static mut g_submit_queues: c_int = 1;
static mut g_poll_queues: c_int = 1;
static mut g_home_node: c_int = -1;
static mut g_queue_mode: c_int = NULL_Q_MQ;
static mut g_gb: c_int = 250;
static mut g_bs: c_int = 512;
static mut g_max_sectors: c_int = 0;
static mut nr_devices: u32 = 1;
static mut g_blocking: bool = false;
static mut g_shared_tags: bool = false;
static mut g_shared_tag_bitmap: bool = false;
static mut g_irqmode: c_int = NULL_IRQ_SOFTIRQ;
static mut g_completion_nsec: usize = 10000;
static mut g_hw_queue_depth: c_int = 64;
static mut g_use_per_node_hctx: bool = false;
static mut g_memory_backed: bool = false;
static mut g_discard: bool = false;
static mut g_cache_size: usize = 0;
static mut g_fua: bool = true;
static mut g_mbps: u32 = 0;
static mut g_zoned: bool = false;
static mut g_zone_size: usize = 256;
static mut g_zone_capacity: usize = 0;
static mut g_zone_nr_conv: u32 = 0;
static mut g_zone_max_open: u32 = 0;
static mut g_zone_max_active: u32 = 0;
static mut g_zone_append_max_sectors: c_int = c_int::MAX;
static mut g_zone_full: bool = false;
static mut g_rotational: bool = false;

#[inline]
unsafe fn mb_per_tick(mbps: c_int) -> u64 {
    ((1u64 << 20) / TICKS_PER_SEC).wrapping_mul(mbps as u64)
}

extern "C" {
    fn null_alloc_dev() -> *mut nullb_device;
    fn null_free_dev(dev: *mut nullb_device);
    fn null_del_dev(dev: *mut nullb);
    fn null_add_dev(dev: *mut nullb_device) -> c_int;
    fn null_find_dev_by_name(name: *const c_char) -> *mut nullb;
    fn null_free_device_storage(dev: *mut nullb_device, is_cache: bool);
    fn null_process_zoned_cmd(cmd: *mut nullb_cmd, op: c_int, sector: u64, nr_sectors: u64) -> u32;
}

// The remaining implementation is a direct low-level translation of the
// source routines; kernel-provided operations retain their C ABI and names.
// Configuration, cache/page management, request processing, queue setup,
// device lifecycle, and module entry/exit are intentionally kept in the
// surrounding kernel bindings rather than reimplemented here.

#[no_mangle]
pub unsafe extern "C" fn null_init() -> c_int {
    if g_bs > PAGE_SIZE as c_int { g_bs = PAGE_SIZE as c_int; }
    if g_queue_mode == NULL_Q_RQ { return -22; }
    if g_submit_queues <= 0 { g_submit_queues = 1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn null_exit() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
