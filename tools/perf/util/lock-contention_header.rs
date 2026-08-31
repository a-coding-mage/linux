// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/lock-contention.h.
// C includes removed: <linux/list.h>, <linux/rbtree.h>.
// The referenced kernel/perf types are expected to be supplied by surrounding
// translated code.

#[repr(C)]
pub struct lock_filter {
    pub nr_types: ::std::os::raw::c_int,
    pub nr_addrs: ::std::os::raw::c_int,
    pub nr_syms: ::std::os::raw::c_int,
    pub nr_cgrps: ::std::os::raw::c_int,
    pub nr_slabs: ::std::os::raw::c_int,
    pub types: *mut ::std::os::raw::c_uint,
    pub addrs: *mut ::std::os::raw::c_ulong,
    pub syms: *mut *mut ::std::os::raw::c_char,
    pub cgrps: *mut u64,
    pub slabs: *mut *mut ::std::os::raw::c_char,
}

#[repr(C)]
pub struct lock_delay {
    pub sym: *mut ::std::os::raw::c_char,
    pub addr: ::std::os::raw::c_ulong,
    pub time: ::std::os::raw::c_ulong,
}

#[repr(C)]
pub union lock_stat__bindgen_ty_1 {
    pub nr_readlock: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct lock_stat {
    pub hash_entry: hlist_node,
    pub rb: rb_node, /* used for sorting */

    pub addr: u64, /* address of lockdep_map, used as ID */
    pub name: *mut ::std::os::raw::c_char, /* for strcpy(), we cannot use const */
    pub callstack: *mut u64,

    pub nr_acquire: ::std::os::raw::c_uint,
    pub nr_acquired: ::std::os::raw::c_uint,
    pub nr_contended: ::std::os::raw::c_uint,
    pub nr_release: ::std::os::raw::c_uint,

    pub u: lock_stat__bindgen_ty_1,
    pub nr_trylock: ::std::os::raw::c_uint,

    /* these times are in nano sec. */
    pub avg_wait_time: u64,
    pub wait_time_total: u64,
    pub wait_time_min: u64,
    pub wait_time_max: u64,

    pub broken: ::std::os::raw::c_int, /* flag of blacklist */
    pub combined: ::std::os::raw::c_int,
}

/*
 * States of lock_seq_stat
 *
 * UNINITIALIZED is required for detecting first event of acquire.
 * As the nature of lock events, there is no guarantee
 * that the first event for the locks are acquire,
 * it can be acquired, contended or release.
 */
pub const SEQ_STATE_UNINITIALIZED: ::std::os::raw::c_int = 0; /* initial state */
pub const SEQ_STATE_RELEASED: ::std::os::raw::c_int = 1;
pub const SEQ_STATE_ACQUIRING: ::std::os::raw::c_int = 2;
pub const SEQ_STATE_ACQUIRED: ::std::os::raw::c_int = 3;
pub const SEQ_STATE_READ_ACQUIRED: ::std::os::raw::c_int = 4;
pub const SEQ_STATE_CONTENDED: ::std::os::raw::c_int = 5;

/*
 * MAX_LOCK_DEPTH
 * Imported from include/linux/sched.h.
 * Should this be synchronized?
 */
pub const MAX_LOCK_DEPTH: ::std::os::raw::c_int = 48;

/* based on kernel/lockdep.c */
pub const LOCKHASH_BITS: ::std::os::raw::c_int = 12;
pub const LOCKHASH_SIZE: ::std::os::raw::c_ulong = 1_u64.wrapping_shl(LOCKHASH_BITS as u32) as ::std::os::raw::c_ulong;

extern "C" {
    pub static mut lockhash_table: *mut hlist_head;
}

/*
 * struct lock_seq_stat:
 * Place to put on state of one lock sequence
 * 1) acquire -> acquired -> release
 * 2) acquire -> contended -> acquired -> release
 * 3) acquire (with read or try) -> release
 * 4) Are there other patterns?
 */
#[repr(C)]
pub struct lock_seq_stat {
    pub list: list_head,
    pub state: ::std::os::raw::c_int,
    pub prev_event_time: u64,
    pub addr: u64,

    pub read_count: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct thread_stat {
    pub rb: rb_node,

    pub tid: u32,
    pub seq_list: list_head,
}

/*
 * CONTENTION_STACK_DEPTH
 * Number of stack trace entries to find callers
 */
pub const CONTENTION_STACK_DEPTH: ::std::os::raw::c_int = 8;

/*
 * CONTENTION_STACK_SKIP
 * Number of stack trace entries to skip when finding callers.
 * The first few entries belong to the locking implementation itself.
 */
pub const CONTENTION_STACK_SKIP: ::std::os::raw::c_int = 4;

/*
 * flags for lock:contention_begin
 * Imported from include/trace/events/lock.h.
 */
pub const LCB_F_SPIN: ::std::os::raw::c_uint = 1_u32 << 0;
pub const LCB_F_READ: ::std::os::raw::c_uint = 1_u32 << 1;
pub const LCB_F_WRITE: ::std::os::raw::c_uint = 1_u32 << 2;
pub const LCB_F_RT: ::std::os::raw::c_uint = 1_u32 << 3;
pub const LCB_F_PERCPU: ::std::os::raw::c_uint = 1_u32 << 4;
pub const LCB_F_MUTEX: ::std::os::raw::c_uint = 1_u32 << 5;

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct target {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct lock_contention_fails {
    pub task: ::std::os::raw::c_int,
    pub stack: ::std::os::raw::c_int,
    pub time: ::std::os::raw::c_int,
    pub data: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct lock_contention {
    pub evlist: *mut evlist,
    pub target: *mut target,
    pub machine: *mut machine,
    pub result: *mut hlist_head,
    pub filters: *mut lock_filter,
    pub delays: *mut lock_delay,
    pub fails: lock_contention_fails,
    pub cgroups: rb_root,
    pub btf: *mut ::std::os::raw::c_void,
    pub map_nr_entries: ::std::os::raw::c_ulong,
    pub max_stack: ::std::os::raw::c_int,
    pub stack_skip: ::std::os::raw::c_int,
    pub aggr_mode: ::std::os::raw::c_int,
    pub owner: ::std::os::raw::c_int,
    pub nr_filtered: ::std::os::raw::c_int,
    pub nr_delays: ::std::os::raw::c_int,
    pub save_callstack: bool,
}

#[repr(C)]
pub struct option {
    _unused: [u8; 0],
}

extern "C" {
    pub fn parse_call_stack(
        opt: *const option,
        str_: *const ::std::os::raw::c_char,
        unset: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn needs_callstack() -> bool;

    pub fn lock_stat_find(addr: u64) -> *mut lock_stat;
    pub fn lock_stat_findnew(
        addr: u64,
        name: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> *mut lock_stat;

    pub fn match_callstack_filter(
        machine: *mut machine,
        callstack: *mut u64,
        max_stack_depth: ::std::os::raw::c_int,
    ) -> bool;
}

// C conditional: #ifdef HAVE_BPF_SKEL
#[cfg(HAVE_BPF_SKEL)]
extern "C" {
    pub fn lock_contention_prepare(con: *mut lock_contention) -> ::std::os::raw::c_int;
    pub fn lock_contention_start() -> ::std::os::raw::c_int;
    pub fn lock_contention_stop() -> ::std::os::raw::c_int;
    pub fn lock_contention_read(con: *mut lock_contention) -> ::std::os::raw::c_int;
    pub fn lock_contention_finish(con: *mut lock_contention) -> ::std::os::raw::c_int;

    pub fn pop_owner_stack_trace(con: *mut lock_contention) -> *mut lock_stat;
}

// C conditional: #else /* !HAVE_BPF_SKEL */
#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn lock_contention_prepare(_con: *mut lock_contention) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn lock_contention_start() -> ::std::os::raw::c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn lock_contention_stop() -> ::std::os::raw::c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn lock_contention_finish(_con: *mut lock_contention) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn lock_contention_read(_con: *mut lock_contention) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn pop_owner_stack_trace(_con: *mut lock_contention) -> *mut lock_stat {
    ::std::ptr::null_mut()
}
