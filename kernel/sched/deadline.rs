// SPDX-License-Identifier: GPL-2.0
// Incremental repair of the pinned deadline.rs skeleton. The original C owner
// remains the semantic oracle; none of its missing routines are C fallbacks.
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// RECONCILED-SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
// SOURCE ONLY: incomplete owner, inactive admission. See deadline review inventory.
#![no_std]
use core::ptr::{addr_of, addr_of_mut};
use kernel::ffi::{c_int, c_uint, c_ulong};
use kernel::bindings::sched_deadline as b;

#[no_mangle]
static mut rust_dl_period_max: c_uint = 1 << 22;
#[no_mangle]
static mut rust_dl_period_min: c_uint = 100;
#[no_mangle]
pub static mut dl_cookie: u64 = 0;

#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn sched_dl_sysctl_init() -> c_int {
    // SAFETY: Initcall registration uses the static table and its live Rust-owned sysctl scalars.
    unsafe {
        b::rust_dl_register_sysctl_init();
        0
    }
}

// Retained baseline helpers, repaired to use original-header ABI primitives.
#[inline]
unsafe fn rq_of_dl_rq(dl_rq: *mut b::dl_rq) -> *mut b::rq {
    // SAFETY: The input points to the dl member of a live native runqueue.
    unsafe {
        b::rust_dl_rq_from_dl_rq(dl_rq)
    }
}
#[inline]
unsafe fn rq_of_dl_se(dl_se: *mut b::sched_dl_entity) -> *mut b::rq {
    // SAFETY: The live entity is either a server with a valid rq or belongs to a live task.
    unsafe {
        if b::rust_dl_server(dl_se) { (*dl_se).rq }
        else { b::rust_dl_task_rq(b::rust_dl_task_of(dl_se)) }
    }
}
#[inline]
unsafe fn dl_rq_of_se(dl_se: *mut b::sched_dl_entity) -> *mut b::dl_rq {
    // SAFETY: The live entity's owning rq remains valid while its dl member is borrowed.
    unsafe {
        addr_of_mut!((*rq_of_dl_se(dl_se)).dl)
    }
}
#[inline]
unsafe fn on_dl_rq(dl_se: *mut b::sched_dl_entity) -> c_int {
    // SAFETY: The entity is live and its rb-node membership is serialized by its rq lock.
    unsafe {
        (!b::rust_dl_rb_empty_node(addr_of!((*dl_se).rb_node))) as c_int
    }
}
#[cfg(CONFIG_RT_MUTEXES)]
#[inline]
unsafe fn pi_of(dl_se: *mut b::sched_dl_entity) -> *mut b::sched_dl_entity {
    // SAFETY: The live entity's priority-inheritance pointer is serialized by its rq or PI lock.
    unsafe {
        (*dl_se).pi_se
    }
}
#[cfg(not(CONFIG_RT_MUTEXES))]
#[inline]
unsafe fn pi_of(dl_se: *mut b::sched_dl_entity) -> *mut b::sched_dl_entity { dl_se }
#[cfg(CONFIG_RT_MUTEXES)]
#[inline]
unsafe fn is_dl_boosted(dl_se: *mut b::sched_dl_entity) -> bool {
    // SAFETY: The live entity's priority-inheritance state is serialized by its rq or PI lock.
    unsafe { pi_of(dl_se) != dl_se }
}
#[cfg(not(CONFIG_RT_MUTEXES))]
#[inline]
unsafe fn is_dl_boosted(_dl_se: *mut b::sched_dl_entity) -> bool { false }
#[inline]
unsafe fn dl_get_type(dl_se: *mut b::sched_dl_entity, rq: *mut b::rq) -> u8 {
    // SAFETY: Both entity and rq are live; configured server members have their native layout.
    unsafe {
        if !b::rust_dl_server(dl_se) { return b::DL_TASK as u8; }
        if dl_se == addr_of_mut!((*rq).fair_server) { return b::DL_SERVER_FAIR as u8; }
        #[cfg(CONFIG_SCHED_CLASS_EXT)]
        if dl_se == addr_of_mut!((*rq).ext_server) { return b::DL_SERVER_EXT as u8; }
        b::DL_OTHER as u8
    }
}
#[inline]
unsafe fn __dl_bw_capacity(mask: *const b::cpumask) -> c_ulong {
    // SAFETY: The CPU mask is live and domain/CPU state has the caller's scheduler-RCU exclusion.
    unsafe {
        let mut cap: c_ulong = 0;
        let active = b::rust_dl_active_mask();
        let mut cpu = b::rust_dl_cpumask_first_and(mask, active);
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            cap = cap.wrapping_add(b::rust_dl_arch_scale_cpu_capacity(cpu as c_int));
            cpu = b::rust_dl_cpumask_next_and(cpu as c_int, mask, active);
        }
        cap
    }
}

include!("deadline_bandwidth.rs");
include!("deadline_entity.rs");
include!("deadline_runqueue.rs");
include!("deadline_migration.rs");
include!("deadline_lifecycle.rs");
