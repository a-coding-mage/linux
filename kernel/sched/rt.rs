// SPDX-License-Identifier: GPL-2.0
// Full source owner proposal for kernel/sched/rt.c.
// Authority: e1d84f501551943a11f4c5271e9f5c85d7e15168.
// Native types/constants are generated solely from rt_native_bindings.h.
// SOURCE ONLY: no generated binding, compilation, policy or runtime acceptance.
//! Recovered RT scheduler owner with native configured interfaces.
//!
//! Caller contracts below describe the required native lifetime and locking
//! conditions. They do not qualify generated layouts, instrumentation, unlocked
//! accesses or cross-language memory-model behavior. Build admission stays closed.
#![no_std]
#[cfg(any(CONFIG_RT_GROUP_SCHED, CONFIG_UCLAMP_TASK, CONFIG_POSIX_TIMERS))]
use core::cmp::min;
#[cfg(CONFIG_RT_GROUP_SCHED)]
use core::mem::MaybeUninit;
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_uint, c_ulong};
#[cfg(CONFIG_RT_GROUP_SCHED)]
use kernel::ffi::c_long;
#[cfg(any(CONFIG_RT_GROUP_SCHED, CONFIG_SYSCTL))]
use kernel::ffi::c_void;
use kernel::bindings::sched_rt_native as b;
// Native configured offsets; raw pointers never create references to shared data.
macro_rules! container_of {
    ($p:expr, $ty:ty, $field:ident) => {
        ($p as *mut u8).sub(core::mem::offset_of!($ty, $field)) as *mut $ty
    };
}
// Rust owns iteration control. Each closure returns to this advancement, making
// an early closure return the precise counterpart of C's loop continue.
/// Visits CPUs in the supplied native mask.
///
/// # Safety
/// The mask must remain readable for native cpumask operations throughout the
/// iteration. The caller must satisfy the supplied callback's lifetime and locking
/// requirements for every visited CPU.
unsafe fn each_cpu(mask: *const b::cpumask, mut f: impl FnMut(c_int)) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut cpu = b::rust_rt_cpumask_first(mask) as c_int;
        while (cpu as c_uint) < b::rust_rt_nr_cpu_ids() {
            f(cpu);
            cpu = b::rust_rt_cpumask_next(cpu, mask) as c_int;
        }
    }
}
include!("rt_group.rs");
include!("rt_queue.rs");
include!("rt_balance.rs");
include!("rt_control.rs");

#[no_mangle]
/// Round-robin quantum in native scheduler ticks.
pub static mut sched_rr_timeslice: c_int = b::RUST_RT_RR_TIMESLICE as c_int;
#[no_mangle]
/// Global RT bandwidth period in microseconds.
pub static mut sysctl_sched_rt_period: c_int = 1000000;
#[no_mangle]
/// Global RT runtime limit in microseconds, with the native unlimited sentinel.
pub static mut sysctl_sched_rt_runtime: c_int = 1000000;
#[cfg(any(CONFIG_RT_GROUP_SCHED, CONFIG_SYSCTL))]
const MAX_RT_RUNTIME: u64 = b::RUST_RT_MAX_BW as u64;
#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
/// Millisecond-facing sysctl storage for the RR quantum.
pub static mut rust_rt_sysctl_sched_rr_timeslice: c_int =
    (b::RUST_RT_MSEC_PER_SEC as c_int * b::RUST_RT_RR_TIMESLICE as c_int) / b::RUST_RT_HZ as c_int;

#[no_mangle]
/// Initializes the RT portion of a native runqueue.
///
/// # Safety
/// The pointer must identify writable, aligned native rt_rq storage that is not
/// concurrently published. The allocation must contain all configured fields and its
/// list and lock initializers must not race with users.
pub unsafe extern "C" fn init_rt_rq(rt_rq: *mut b::rt_rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let array = addr_of_mut!((*rt_rq).active);
        for i in 0..b::RUST_RT_MAX_RT_PRIO as c_int {
            b::rust_rt_init_list_head(addr_of_mut!((*array).queue).cast::<b::list_head>().add(i as usize));
            b::rust_rt_clear_bit(i, addr_of_mut!((*array).bitmap).cast::<c_ulong>());
        }
        b::rust_rt_set_bit(b::RUST_RT_MAX_RT_PRIO as c_int, addr_of_mut!((*array).bitmap).cast::<c_ulong>());
        (*rt_rq).highest_prio.curr = b::RUST_RT_MAX_RT_PRIO as c_int - 1;
        (*rt_rq).highest_prio.next = b::RUST_RT_MAX_RT_PRIO as c_int - 1;
        (*rt_rq).overloaded = false;
        b::rust_rt_plist_head_init(addr_of_mut!((*rt_rq).pushable_tasks));
        (*rt_rq).rt_queued = 0;
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            (*rt_rq).rt_time = 0;
            (*rt_rq).rt_throttled = 0;
            (*rt_rq).rt_runtime = 0;
            b::rust_rt_init_rq_runtime_lock(rt_rq);
            (*rt_rq).tg = addr_of_mut!(b::root_task_group);
        }
    }
}
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn rt_entity_is_task(_rt_se: *mut b::sched_rt_entity) -> bool {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe { (*_rt_se).my_q.is_null() }
    #[cfg(not(CONFIG_RT_GROUP_SCHED))] { true }
}
/// Recovers the task containing a task RT entity.
///
/// # Safety
/// The entity must be the rt field of a live task_struct allocation. The group
/// diagnostic is not a substitute for this embedding requirement; a group entity or
/// list sentinel is not a valid argument.
unsafe fn rt_task_of(rt_se: *mut b::sched_rt_entity) -> *mut b::task_struct {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] b::rust_rt_warn_173(!rt_entity_is_task(rt_se));
        container_of!(rt_se, b::task_struct, rt)
    }
}
/// Accesses a native RT runqueue relationship or field.
///
/// # Safety
/// The supplied entity or runqueue and the configured objects it refers to must remain
/// live and initialized. The caller must retain the native lifetime and
/// synchronization protection for the relationship and fields being read. Non-group
/// container recovery requires the real containing rq allocation.
unsafe fn rq_of_rt_rq(rt_rq: *mut b::rt_rq) -> *mut b::rq {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            b::rust_rt_warn_181(!b::rust_rt_group_sched_enabled() && (*rt_rq).tg != addr_of_mut!(b::root_task_group));
            (*rt_rq).rq
        }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] { container_of!(rt_rq, b::rq, rt) }
    }
}
/// Accesses a native RT runqueue relationship or field.
///
/// # Safety
/// The supplied entity or runqueue and the configured objects it refers to must remain
/// live and initialized. The caller must retain the native lifetime and
/// synchronization protection for the relationship and fields being read. Non-group
/// container recovery requires the real containing rq allocation.
unsafe fn rt_rq_of_se(rt_se: *mut b::sched_rt_entity) -> *mut b::rt_rq {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            b::rust_rt_warn_187(!b::rust_rt_group_sched_enabled() && (*(*rt_se).rt_rq).tg != addr_of_mut!(b::root_task_group));
            (*rt_se).rt_rq
        }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] { addr_of_mut!((*rq_of_rt_se(rt_se)).rt) }
    }
}
/// Accesses a native RT runqueue relationship or field.
///
/// # Safety
/// The supplied entity or runqueue and the configured objects it refers to must remain
/// live and initialized. The caller must retain the native lifetime and
/// synchronization protection for the relationship and fields being read. Non-group
/// container recovery requires the real containing rq allocation.
unsafe fn rq_of_rt_se(rt_se: *mut b::sched_rt_entity) -> *mut b::rq {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            let rt_rq = (*rt_se).rt_rq;
            b::rust_rt_warn_195(!b::rust_rt_group_sched_enabled() && (*rt_rq).tg != addr_of_mut!(b::root_task_group));
            (*rt_rq).rq
        }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] { b::rust_rt_task_rq(rt_task_of(rt_se)) }
    }
}
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn rt_parent(_rt_se: *mut b::sched_rt_entity) -> *mut b::sched_rt_entity {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe { (*_rt_se).parent }
    #[cfg(not(CONFIG_RT_GROUP_SCHED))] { null_mut() }
}
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn group_rt_rq(_rt_se: *mut b::sched_rt_entity) -> *mut b::rt_rq {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe { (*_rt_se).my_q }
    #[cfg(not(CONFIG_RT_GROUP_SCHED))] { null_mut() }
}
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn on_rt_rq(rt_se: *mut b::sched_rt_entity) -> bool {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        (*rt_se).on_rq != 0
    }
}
/// Accesses a native RT runqueue relationship or field.
///
/// # Safety
/// The supplied entity or runqueue and the configured objects it refers to must remain
/// live and initialized. The caller must retain the native lifetime and
/// synchronization protection for the relationship and fields being read. Non-group
/// container recovery requires the real containing rq allocation.
unsafe fn rt_rq_throttled(_rt_rq: *mut b::rt_rq) -> bool {
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe { (*_rt_rq).rt_throttled != 0 && (*_rt_rq).rt_nr_boosted == 0 }
    #[cfg(not(CONFIG_RT_GROUP_SCHED))] { false }
}
/// Inspects an initialized RT scheduling entity.
///
/// # Safety
/// The entity must remain live with its native task or group embedding and hierarchy
/// initialized. The owning runqueue protocol must stabilize the accessed fields and
/// any referenced task or group queue. A task entity must actually belong to
/// task_struct, never a list sentinel.
unsafe fn rt_se_prio(rt_se: *mut b::sched_rt_entity) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            let rt_rq = group_rt_rq(rt_se);
            if !rt_rq.is_null() { return (*rt_rq).highest_prio.curr; }
        }
        (*rt_task_of(rt_se)).prio
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Reads the current runqueue's RT bandwidth span.
///
/// # Safety
/// The current CPU must remain pinned and its runqueue and root domain must remain
/// valid under the native timer/scheduler lifetime protocol.
unsafe fn sched_rt_period_mask() -> *const b::cpumask {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] { b::rust_rt_rd_span((*b::rust_rt_this_rq()).rd) }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] { b::rust_rt_cpu_online_mask() }
    }
}
#[cfg(CONFIG_RT_GROUP_SCHED)]
/// Finds a task group's RT runqueue for one CPU.
///
/// # Safety
/// The bandwidth pointer must be embedded in a live task_group. The CPU must index
/// that group's initialized native rt_rq array, and the group and array must remain
/// alive throughout the access.
unsafe fn sched_rt_period_rt_rq(_rt_b: *mut b::rt_bandwidth, cpu: c_int) -> *mut b::rt_rq {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            *(*container_of!(_rt_b, b::task_group, rt_bandwidth)).rt_rq.add(cpu as usize)
        }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] { addr_of_mut!((*b::rust_rt_cpu_rq(cpu)).rt) }
    }
}
// Matches for_each_rt_rq, including early termination on a NULL rq entry.
/// Traverses live RT task-group runqueues.
///
/// # Safety
/// The runqueue or task group must be live, with initialized list links and per-CPU
/// arrays. The caller must hold RCU read protection or the native equivalent that
/// prevents task-group reclamation, and satisfy the callback's additional locking
/// requirements.
unsafe fn each_rt_rq(rq: *mut b::rq, mut f: impl FnMut(*mut b::rt_rq)) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        #[cfg(CONFIG_RT_GROUP_SCHED)] {
            let mut tg = addr_of_mut!(b::root_task_group);
            while !tg.is_null() {
                let rr = *(*tg).rt_rq.add(b::rust_rt_cpu_of(rq) as usize);
                if rr.is_null() { break; }
                f(rr);
                tg = next_task_group(tg);
            }
        }
        #[cfg(not(CONFIG_RT_GROUP_SCHED))] { f(addr_of_mut!((*rq).rt)); }
    }
}
#[no_mangle]
/// Prints the RT runqueues associated with a CPU.
///
/// # Safety
/// The seq_file must be valid for native output and the CPU must identify an
/// initialized runqueue. The native debug-reader access protocol must permit the
/// statistics reads; this function supplies RCU lifetime protection for group
/// traversal.
pub unsafe extern "C" fn print_rt_stats(m: *mut b::seq_file, cpu: c_int) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_rcu_read_lock();
        each_rt_rq(b::rust_rt_cpu_rq(cpu), |rr| { b::print_rt_rq(m, cpu, rr); });
        b::rust_rt_rcu_read_unlock();
    }
}
