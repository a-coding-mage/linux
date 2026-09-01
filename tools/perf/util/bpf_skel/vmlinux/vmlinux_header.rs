// Translated from vmlinux.h.
// C includes removed: linux/stddef.h, linux/bpf.h, linux/types.h,
// linux/perf_event.h, stdbool.h.

use core::ffi::{c_char, c_int, c_long, c_void};

// non-UAPI kernel data structures, used in the .bpf.c BPF tool component.

// Just the fields used in these tools preserving the access index so that
// libbpf can fixup offsets with the ones used in the kernel when loading the
// BPF bytecode, if they differ from what is used here.

pub type s32 = i32;
pub type s64 = i64;

pub type pid_t = c_int;

pub type time64_t = s64;

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: time64_t,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub enum cgroup_subsys_id {
    perf_event_cgrp_id = 8,
}

pub const HI_SOFTIRQ: c_int = 0;
pub const TIMER_SOFTIRQ: c_int = 1;
pub const NET_TX_SOFTIRQ: c_int = 2;
pub const NET_RX_SOFTIRQ: c_int = 3;
pub const BLOCK_SOFTIRQ: c_int = 4;
pub const IRQ_POLL_SOFTIRQ: c_int = 5;
pub const TASKLET_SOFTIRQ: c_int = 6;
pub const SCHED_SOFTIRQ: c_int = 7;
pub const HRTIMER_SOFTIRQ: c_int = 8;
pub const RCU_SOFTIRQ: c_int = 9; /* Preferable RCU should always be the last softirq */

pub const NR_SOFTIRQS: c_int = 10;

#[repr(C)]
pub struct atomic64_t {
    pub counter: s64,
}

pub type atomic_long_t = atomic64_t;

#[repr(C)]
pub struct raw_spinlock {
    pub rawlock: c_int,
}

pub type raw_spinlock_t = raw_spinlock;

#[repr(C)]
pub struct spinlock_t {
    pub rlock: raw_spinlock,
}

#[repr(C)]
pub struct sighand_struct {
    pub siglock: spinlock_t,
}

#[repr(C)]
pub struct rw_semaphore {
    pub owner: atomic_long_t,
}

#[repr(C)]
pub struct mutex {
    pub owner: atomic_long_t,
}

#[repr(C)]
pub struct kernfs_node {
    pub id: u64,
}

#[repr(C)]
pub struct cgroup {
    pub kn: *mut kernfs_node,
    pub level: c_int,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    pub cgroup: *mut cgroup,
}

#[repr(C)]
pub struct css_set {
    pub subsys: [*mut cgroup_subsys_state; 13],
    pub dfl_cgrp: *mut cgroup,
}

#[repr(C)]
pub struct mm_struct {
    pub mmap_lock: rw_semaphore,
}

#[repr(C)]
pub struct task_struct {
    pub flags: u32,
    pub mm: *mut mm_struct,
    pub pid: pid_t,
    pub tgid: pid_t,
    pub comm: [c_char; 16],
    pub sighand: *mut sighand_struct,
    pub cgroups: *mut css_set,
}

#[repr(C)]
pub struct trace_entry {
    pub type_: u16,
    pub flags: u8,
    pub preempt_count: u8,
    pub pid: c_int,
}

#[repr(C)]
pub struct trace_event_raw_irq_handler_entry {
    pub ent: trace_entry,
    pub irq: c_int,
    pub __data_loc_name: u32,
    pub __data: [c_char; 0],
}

#[repr(C)]
pub struct trace_event_raw_irq_handler_exit {
    pub ent: trace_entry,
    pub irq: c_int,
    pub ret: c_int,
    pub __data: [c_char; 0],
}

#[repr(C)]
pub struct trace_event_raw_softirq {
    pub ent: trace_entry,
    pub vec: u32,
    pub __data: [c_char; 0],
}

#[repr(C)]
pub struct trace_event_raw_workqueue_execute_start {
    pub ent: trace_entry,
    pub work: *mut c_void,
    pub function: *mut c_void,
    pub __data: [c_char; 0],
}

#[repr(C)]
pub struct trace_event_raw_workqueue_execute_end {
    pub ent: trace_entry,
    pub work: *mut c_void,
    pub function: *mut c_void,
    pub __data: [c_char; 0],
}

#[repr(C)]
pub struct trace_event_raw_workqueue_activate_work {
    pub ent: trace_entry,
    pub work: *mut c_void,
    pub __data: [c_char; 0],
}

// Types supplied by the removed linux/perf_event.h include:
// perf_sample_weight, perf_mem_data_src.

#[repr(C)]
pub struct perf_sample_data__tid_entry {
    pub pid: u32,
    pub tid: u32,
}

#[repr(C)]
pub struct perf_sample_data__cpu_entry {
    pub cpu: u32,
}

#[repr(C, align(64))]
pub struct perf_sample_data {
    pub addr: u64,
    pub period: u64,
    pub weight: perf_sample_weight,
    pub txn: u64,
    pub data_src: perf_mem_data_src,
    pub ip: u64,
    pub tid_entry: perf_sample_data__tid_entry,
    pub time: u64,
    pub id: u64,
    pub cpu_entry: perf_sample_data__cpu_entry,
    pub phys_addr: u64,
    pub cgroup: u64,
    pub data_page_size: u64,
    pub code_page_size: u64,
}

#[repr(C)]
pub struct perf_event {
    pub parent: *mut perf_event,
    pub id: u64,
}

#[repr(C)]
pub struct bpf_perf_event_data_kern {
    pub data: *mut perf_sample_data,
    pub event: *mut perf_event,
}

/*
 * If 'struct rq' isn't defined for lock_contention.bpf.c, for the sake of
 * rq___old and rq___new, then the type for the 'runqueue' variable ends up
 * being a forward declaration (BTF_KIND_FWD) while the kernel has it defined
 * (BTF_KIND_STRUCT). The definition appears in vmlinux.h rather than
 * lock_contention.bpf.c for consistency with a generated vmlinux.h.
 */
#[repr(C)]
pub struct rq {}

#[repr(C)]
pub struct kmem_cache {
    pub name: *const c_char,
}

#[repr(C)]
pub struct bpf_iter__kmem_cache {
    pub s: *mut kmem_cache,
}

#[repr(C)]
pub struct zone {
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct pglist_data {
    pub node_zones: [zone; 6], /* value for all possible config */
    pub nr_zones: c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
