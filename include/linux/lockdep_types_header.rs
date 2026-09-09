/* SPDX-License-Identifier: GPL-2.0 */
/* Runtime locking correctness validator. */
/* Translated from linux/lockdep_types.h; linux/types.h supplies dependent types. */

pub const MAX_LOCKDEP_SUBCLASSES: usize = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lockdep_wait_type {
    LD_WAIT_INV = 0,
    LD_WAIT_FREE,
    LD_WAIT_SPIN,
    #[cfg(feature = "CONFIG_PROVE_RAW_LOCK_NESTING")]
    LD_WAIT_CONFIG,
    #[cfg(not(feature = "CONFIG_PROVE_RAW_LOCK_NESTING"))]
    LD_WAIT_CONFIG = LD_WAIT_SPIN as isize,
    LD_WAIT_SLEEP,
    LD_WAIT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lockdep_lock_type {
    LD_LOCK_NORMAL = 0,
    LD_LOCK_PERCPU,
    LD_LOCK_WAIT_OVERRIDE,
    LD_LOCK_MAX,
}

pub const XXX_LOCK_USAGE_STATES: usize = 2;
pub const LOCK_TRACE_STATES: usize = XXX_LOCK_USAGE_STATES * 4 + 2;
pub const NR_LOCKDEP_CACHING_CLASSES: usize = 2;

#[repr(C, packed)]
pub struct lockdep_subclass_key {
    pub __one_byte: i8,
}

#[repr(C)]
pub union lock_class_key_union {
    pub hash_entry: hlist_node,
    pub subkeys: [lockdep_subclass_key; MAX_LOCKDEP_SUBCLASSES],
}

#[repr(C)]
pub struct lock_class_key {
    pub data: lock_class_key_union,
}

extern "C" {
    pub static mut __lockdep_no_validate__: lock_class_key;
    pub static mut __lockdep_no_track__: lock_class_key;
}

pub struct lock_trace;
pub const LOCKSTAT_POINTS: usize = 4;

pub struct lockdep_map;
pub type lock_cmp_fn = unsafe extern "C" fn(a: *const lockdep_map, b: *const lockdep_map) -> i32;
pub type lock_print_fn = unsafe extern "C" fn(map: *const lockdep_map);

#[repr(C)]
pub struct lock_class {
    pub hash_entry: hlist_node,
    pub lock_entry: list_head,
    pub locks_after: list_head,
    pub locks_before: list_head,
    pub key: *const lockdep_subclass_key,
    pub cmp_fn: Option<lock_cmp_fn>,
    pub print_fn: Option<lock_print_fn>,
    pub subclass: u32,
    pub dep_gen_id: u32,
    pub usage_mask: usize,
    pub usage_traces: [*const lock_trace; LOCK_TRACE_STATES],
    pub name: *const i8,
    pub name_version: i32,
    pub wait_type_inner: u8,
    pub wait_type_outer: u8,
    pub lock_type: u8,
    #[cfg(feature = "CONFIG_LOCK_STAT")]
    pub contention_point: [usize; LOCKSTAT_POINTS],
    #[cfg(feature = "CONFIG_LOCK_STAT")]
    pub contending_point: [usize; LOCKSTAT_POINTS],
}

#[cfg(feature = "CONFIG_LOCK_STAT")]
#[repr(C)]
pub struct lock_time { pub min: i64, pub max: i64, pub total: i64, pub nr: usize }

#[cfg(feature = "CONFIG_LOCK_STAT")]
#[repr(C)]
pub enum bounce_type {
    bounce_acquired_write,
    bounce_acquired_read,
    bounce_contended_write,
    bounce_contended_read,
    nr_bounce_types,
}

#[cfg(feature = "CONFIG_LOCK_STAT")]
pub const bounce_acquired: bounce_type = bounce_type::bounce_acquired_write;
#[cfg(feature = "CONFIG_LOCK_STAT")]
pub const bounce_contended: bounce_type = bounce_type::bounce_contended_write;

#[cfg(feature = "CONFIG_LOCK_STAT")]
#[repr(C)]
pub struct lock_class_stats {
    pub contention_point: [usize; LOCKSTAT_POINTS],
    pub contending_point: [usize; LOCKSTAT_POINTS],
    pub read_waittime: lock_time,
    pub write_waittime: lock_time,
    pub read_holdtime: lock_time,
    pub write_holdtime: lock_time,
    pub bounces: [usize; 4],
}

#[cfg(feature = "CONFIG_LOCK_STAT")]
extern "C" {
    pub fn lock_stats(class: *mut lock_class, stats: *mut lock_class_stats);
    pub fn clear_lock_stats(class: *mut lock_class);
}

#[repr(C)]
pub struct lockdep_map {
    pub key: *mut lock_class_key,
    pub class_cache: [*mut lock_class; NR_LOCKDEP_CACHING_CLASSES],
    pub name: *const i8,
    pub wait_type_outer: u8,
    pub wait_type_inner: u8,
    pub lock_type: u8,
    #[cfg(feature = "CONFIG_LOCK_STAT")]
    pub cpu: i32,
    #[cfg(feature = "CONFIG_LOCK_STAT")]
    pub ip: usize,
}

#[repr(C)]
pub struct pin_cookie { pub val: u32 }

pub const MAX_LOCKDEP_KEYS_BITS: u32 = 13;
pub const MAX_LOCKDEP_KEYS: usize = 1usize << MAX_LOCKDEP_KEYS_BITS;
pub const INITIAL_CHAIN_KEY: i32 = -1;

#[repr(C)]
pub struct held_lock {
    pub prev_chain_key: u64,
    pub acquire_ip: usize,
    pub instance: *mut lockdep_map,
    pub nest_lock: *mut lockdep_map,
    #[cfg(feature = "CONFIG_LOCK_STAT")]
    pub waittime_stamp: u64,
    #[cfg(feature = "CONFIG_LOCK_STAT")]
    pub holdtime_stamp: u64,
    /* C bit-fields are kept in their containing integer representation. */
    pub class_idx: u32,
    pub irq_context: u32,
    pub trylock: u32,
    pub read: u32,
    pub check: u32,
    pub hardirqs_off: u32,
    pub sync: u32,
    pub references: u32,
    pub pin_count: u32,
    pub seq_count: u32,
}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[repr(C)]
pub struct lock_class_key_disabled {}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[repr(C)]
pub struct lockdep_map_disabled {}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[repr(C)]
pub struct pin_cookie_disabled {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
