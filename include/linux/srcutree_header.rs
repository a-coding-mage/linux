/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Sleepable Read-Copy Update mechanism for mutual exclusion,
 * tree variant.
 *
 * Rust translation of srcutree.h.
 */

use core::ffi::c_ulong;

/* Supplied by linux/rcu_node_tree.h and linux/completion.h. */
pub struct raw_spinlock_t;
pub struct rcu_segcblist;
pub struct timer_list;
pub struct work_struct;
pub struct rcu_head;
pub struct mutex;
pub struct completion;
pub struct delayed_work;
pub struct irq_work;
pub struct lockdep_map;
pub struct atomic_long_t;
pub struct atomic_t;

pub const RCU_NUM_LVLS: usize = 0; // Supplied by the dependency header.
pub const RCU_SEQ_CTR_SHIFT: usize = 0; // Supplied by the dependency header.

pub struct srcu_node;
pub struct srcu_struct;

/* One element of the srcu_data srcu_ctrs array. */
#[repr(C)]
pub struct srcu_ctr {
    pub srcu_locks: atomic_long_t,
    pub srcu_unlocks: atomic_long_t,
}

/* Per-CPU structure feeding into leaf srcu_node, similar in function to rcu_node. */
#[repr(C)]
pub struct srcu_data {
    pub srcu_ctrs: [srcu_ctr; 2],
    pub srcu_reader_flavor: core::ffi::c_int,
    pub lock: raw_spinlock_t,
    pub srcu_cblist: rcu_segcblist,
    pub srcu_gp_seq_needed: c_ulong,
    pub srcu_gp_seq_needed_exp: c_ulong,
    pub srcu_cblist_invoking: bool,
    pub delay_work: timer_list,
    pub work: work_struct,
    pub srcu_barrier_head: rcu_head,
    pub srcu_ec_head: rcu_head,
    pub srcu_ec_state: core::ffi::c_int,
    pub mynode: *mut srcu_node,
    pub grpmask: c_ulong,
    pub cpu: core::ffi::c_int,
    pub ssp: *mut srcu_struct,
}

#[repr(C)]
pub struct srcu_node {
    pub lock: raw_spinlock_t,
    pub srcu_have_cbs: [c_ulong; 4],
    pub srcu_data_have_cbs: [c_ulong; 4],
    pub srcu_gp_seq_needed_exp: c_ulong,
    pub srcu_parent: *mut srcu_node,
    pub grplo: core::ffi::c_int,
    pub grphi: core::ffi::c_int,
}

#[repr(C)]
pub struct srcu_usage {
    pub node: *mut srcu_node,
    pub level: [*mut srcu_node; RCU_NUM_LVLS + 1],
    pub srcu_size_state: core::ffi::c_int,
    pub srcu_cb_mutex: mutex,
    pub lock: raw_spinlock_t,
    pub srcu_gp_mutex: mutex,
    pub srcu_gp_seq: c_ulong,
    pub srcu_gp_seq_needed: c_ulong,
    pub srcu_gp_seq_needed_exp: c_ulong,
    pub srcu_gp_start: c_ulong,
    pub srcu_last_gp_end: c_ulong,
    pub srcu_size_jiffies: c_ulong,
    pub srcu_n_lock_retries: c_ulong,
    pub srcu_n_exp_nodelay: c_ulong,
    pub sda_is_static: bool,
    pub srcu_barrier_seq: c_ulong,
    pub srcu_barrier_mutex: mutex,
    pub srcu_barrier_completion: completion,
    pub srcu_barrier_cpu_cnt: atomic_t,
    pub reschedule_jiffies: c_ulong,
    pub reschedule_count: c_ulong,
    pub work: delayed_work,
    pub irq_work: irq_work,
    pub srcu_ssp: *mut srcu_struct,
}

#[repr(C)]
pub struct srcu_struct {
    pub srcu_ctrp: *mut srcu_ctr,
    pub sda: *mut srcu_data,
    pub srcu_reader_flavor: u8,
    pub dep_map: lockdep_map,
    pub srcu_sup: *mut srcu_usage,
}

pub const SRCU_SIZE_SMALL: i32 = 0;
pub const SRCU_SIZE_ALLOC: i32 = 1;
pub const SRCU_SIZE_WAIT_BARRIER: i32 = 2;
pub const SRCU_SIZE_WAIT_CALL: i32 = 3;
pub const SRCU_SIZE_WAIT_CBS1: i32 = 4;
pub const SRCU_SIZE_WAIT_CBS2: i32 = 5;
pub const SRCU_SIZE_WAIT_CBS3: i32 = 6;
pub const SRCU_SIZE_WAIT_CBS4: i32 = 7;
pub const SRCU_SIZE_BIG: i32 = 8;

pub const SRCU_STATE_IDLE: i32 = 0;
pub const SRCU_STATE_SCAN1: i32 = 1;
pub const SRCU_STATE_SCAN2: i32 = 2;

pub const SRCU_EC_IDLE: i32 = 0;
pub const SRCU_EC_PENDING: i32 = 1;
pub const SRCU_EC_REPOST: i32 = 2;

/* These values depend on the kernel's sequence-counter width. */
pub const SRCU_GP_SEQ_INITIAL_VAL: c_ulong = (0u64.wrapping_sub(100) << RCU_SEQ_CTR_SHIFT) as c_ulong;
pub const SRCU_GP_SEQ_INITIAL_VAL_WITH_STATE: c_ulong = SRCU_GP_SEQ_INITIAL_VAL.wrapping_sub(1);

#[macro_export]
macro_rules! __SRCU_USAGE_INIT {
    ($name:ident) => {
        srcu_usage {
            lock: unsafe { core::mem::zeroed() },
            srcu_gp_seq: SRCU_GP_SEQ_INITIAL_VAL,
            srcu_gp_seq_needed: SRCU_GP_SEQ_INITIAL_VAL_WITH_STATE,
            srcu_gp_seq_needed_exp: SRCU_GP_SEQ_INITIAL_VAL,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

#[macro_export]
macro_rules! __SRCU_STRUCT_INIT_COMMON {
    ($name:ident, $usage_name:ident, $fast:expr) => {
        srcu_sup: &mut $usage_name as *mut _,
        srcu_reader_flavor: $fast,
        ..unsafe { core::mem::zeroed() }
    };
}

#[macro_export]
macro_rules! __SRCU_STRUCT_INIT_MODULE {
    ($name:ident, $usage_name:ident, $fast:expr) => {
        srcu_struct { __SRCU_STRUCT_INIT_COMMON!($name, $usage_name, $fast) }
    };
}

#[macro_export]
macro_rules! __SRCU_STRUCT_INIT {
    ($name:ident, $usage_name:ident, $pcpu_name:ident, $fast:expr) => {
        srcu_struct {
            sda: &mut $pcpu_name as *mut _,
            srcu_ctrp: unsafe { &mut (*(&mut $pcpu_name as *mut _)).srcu_ctrs[0] },
            __SRCU_STRUCT_INIT_COMMON!($name, $usage_name, $fast)
        }
    };
}

/* Kernel build-time DEFINE_SRCU variants, represented as declaration macros. */
#[macro_export]
macro_rules! DEFINE_SRCU { ($name:ident) => { __DEFINE_SRCU!($name, 0, false) }; }
#[macro_export]
macro_rules! DEFINE_STATIC_SRCU { ($name:ident) => { __DEFINE_SRCU!($name, 0, true) }; }
#[macro_export]
macro_rules! DEFINE_SRCU_FAST { ($name:ident) => { __DEFINE_SRCU!($name, SRCU_READ_FLAVOR_FAST, false) }; }
#[macro_export]
macro_rules! DEFINE_STATIC_SRCU_FAST { ($name:ident) => { __DEFINE_SRCU!($name, SRCU_READ_FLAVOR_FAST, true) }; }
#[macro_export]
macro_rules! DEFINE_SRCU_FAST_UPDOWN { ($name:ident) => { __DEFINE_SRCU!($name, SRCU_READ_FLAVOR_FAST_UPDOWN, false) }; }
#[macro_export]
macro_rules! DEFINE_STATIC_SRCU_FAST_UPDOWN { ($name:ident) => { __DEFINE_SRCU!($name, SRCU_READ_FLAVOR_FAST_UPDOWN, true) }; }

extern "C" {
    pub fn __srcu_read_lock(ssp: *mut srcu_struct) -> core::ffi::c_int;
    pub fn synchronize_srcu_expedited(ssp: *mut srcu_struct);
    pub fn srcu_barrier(ssp: *mut srcu_struct);
    pub fn srcu_expedite_current(ssp: *mut srcu_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
