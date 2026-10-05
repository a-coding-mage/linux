// SPDX-License-Identifier: GPL-2.0-only
//
// Existing translation of kernel/sched/debug.c, continued against
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. SOURCE ONLY, not build-admitted.
// Kernel-provided layouts and primitives remain authoritative.
//! Scheduler debug owners, continued with configured native interfaces.
#![no_std]
compile_error!("Lupos scheduler debug source proposal is not build-admitted; qualification is incomplete");

use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use kernel::bindings::sched_debug_native as b;
use b::{cfs_rq, dl_rq, file, pid_namespace, rq, rt_rq, sched_domain,
        seq_file, task_struct};
#[cfg(CONFIG_CGROUP_SCHED)]
use b::task_group;

macro_rules! cstr {
    ($s:expr) => { concat!($s, "\0").as_ptr().cast::<c_char>() };
}
macro_rules! out {
    ($m:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        b::lupos_debug_printf($m, cstr!($fmt) $(, $arg)*)
    };
}
macro_rules! ns {
    ($m:expr, $fmt:expr, $label:expr, $value:expr) => {{
        let value = $value as u64;
        out!($m, $fmt, $label, nsec_high(value), nsec_low(value));
    }};
}
macro_rules! stat_value {
    ($stats:expr, $field:ident) => {{
        #[cfg(CONFIG_SCHEDSTATS)] { (*$stats).$field }
        #[cfg(not(CONFIG_SCHEDSTATS))] { 0u64 }
    }};
}

// Preserve the existing quotient/remainder helpers and signed-u64 behavior.
fn nsec_high(mut nsec: u64) -> i64 {
    if (nsec as i64) < 0 { nsec = nsec.wrapping_neg(); -((nsec / 1_000_000) as i64) }
    else { (nsec / 1_000_000) as i64 }
}
fn nsec_low(mut nsec: u64) -> c_ulong {
    if (nsec as i64) < 0 { nsec = nsec.wrapping_neg(); }
    (nsec % 1_000_000) as c_ulong
}

#[export_name = "lupos_debug_feat_show"]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn sched_feat_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    for i in 0..b::LUPOS_DEBUG_SCHED_FEAT_NR as usize {
        if (b::sysctl_sched_features as c_ulong) & (1 as c_ulong).wrapping_shl(i as u32) == 0 {
            b::lupos_debug_seq_puts(m, cstr!("NO_"));
        }
        out!(m, "%s ", b::lupos_debug_feat_name(i as c_int));
    }
    b::lupos_debug_seq_puts(m, cstr!("\n"));
    0

    }
}

#[export_name = "lupos_debug_scaling_show"]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn sched_scaling_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    out!(m, "%d\n", b::sysctl_sched_tunable_scaling as c_int);
    0

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn proc_sched_set_task(p: *mut task_struct) {
    #[cfg(CONFIG_SCHEDSTATS)]
    // SAFETY: The original proc entry point supplies a live task and permits
    // resetting its native scheduler statistics storage to the all-zero state.
    unsafe { core::ptr::write_bytes(addr_of_mut!((*p).stats), 0, 1); }
    #[cfg(not(CONFIG_SCHEDSTATS))]
    let _ = p;
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn sysrq_sched_debug_show() {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    sched_debug_header(null_mut());
    each_configured_cpu(b::lupos_debug_online_mask(), |cpu| {
        b::lupos_debug_touch_nmi_watchdog();
        b::lupos_debug_touch_all_softlockup_watchdogs();
        print_cpu(null_mut(), cpu as c_int);
    });

    }
}

// for_each_possible_cpu/for_each_online_cpu ignore masks when NR_CPUS == 1.
// Generic for_each_cpu (domain dirtiness) deliberately does not use this helper.
unsafe fn each_configured_cpu(mask: *const b::cpumask, mut visit: impl FnMut(c_uint)) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if b::LUPOS_DEBUG_NR_CPUS == 1 {
        visit(0);
        return;
    }
    let mut cpu = b::lupos_debug_mask_first(mask);
    while cpu < b::lupos_debug_mask_bound() {
        visit(cpu);
        cpu = b::lupos_debug_mask_next(cpu as c_int, mask);
    }

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn resched_latency_warn(cpu: c_int, latency: u64) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    // Native storage retains DEFINE_RATELIMIT_STATE and its native lock/init.
    if !b::lupos_debug_latency_ratelimit() { return; }
    let ticks = (*b::lupos_debug_cpu_rq(cpu)).ticks_without_resched;
    b::lupos_debug_error(cstr!("sched: CPU %d need_resched set for > %llu ns (%d ticks) without schedule\n"), cpu, latency, ticks);
    b::lupos_debug_dump_stack();

    }
}

include!("sched_debug_controls.rs");
include!("sched_debug_print.rs");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
