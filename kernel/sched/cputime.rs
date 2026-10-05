// SPDX-License-Identifier: GPL-2.0-only
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// Reconciled against 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
//! CPU accounting algorithms continued from the existing Rust source.
//!
//! Native configured headers own all C layouts, macros and synchronization.
//! Source-only checkpoint: this owner is not admitted to a build.

#[cfg(CONFIG_RUST_SCHED_CPUTIME)]
compile_error!("SOURCE ONLY HOLD: scheduler cputime is not admitted");

use core::ptr::{addr_of, addr_of_mut};
use kernel::bindings::cputime_native::*;

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must satisfy the native static-key update context and serialization rules.
#[no_mangle]
pub unsafe extern "C" fn enable_sched_clock_irqtime() {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        lupos_cputime_irqtime_enable();
    }
}
#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must satisfy the native static-key update context and serialization rules.
#[no_mangle]
pub unsafe extern "C" fn disable_sched_clock_irqtime() {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if lupos_cputime_irqtime_enabled() { lupos_cputime_irqtime_disable(); }
    }
}

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
unsafe fn irqtime_account_delta(q: *mut irqtime, delta: u64, idx: cpu_usage_stat) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let cpustat = addr_of_mut!((*lupos_cputime_this_cpustat()).cpustat).cast::<u64>();
        lupos_cputime_u64_stats_update_begin(addr_of_mut!((*q).sync));
        *cpustat.add(idx.0 as usize) = (*cpustat.add(idx.0 as usize)).wrapping_add(delta);
        (*q).total = ((*q).total).wrapping_add(delta);
        if !lupos_cputime_kcpustat_idle_dyntick() {
            (*q).tick_delta = (*q).tick_delta.wrapping_add(delta);
        }
        lupos_cputime_u64_stats_update_end(addr_of_mut!((*q).sync));
    }
}

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn irqtime_account_irq(curr: *mut task_struct, offset: u32) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let q = lupos_cputime_this_irqtime();
        if !lupos_cputime_irqtime_enabled() {
            return;
        }
        let cpu = lupos_cputime_smp_processor_id();
        let delta = lupos_cputime_sched_clock_cpu(cpu).wrapping_sub((*q).irq_start_time);
        (*q).irq_start_time = (*q).irq_start_time.wrapping_add(delta);
        let pc = lupos_cputime_irq_count().wrapping_sub(offset);
        if pc & (LUPOS_CPUTIME_HARDIRQ_MASK as u32) != 0 { irqtime_account_delta(q, delta, CPUTIME_IRQ); }
        else if pc & (LUPOS_CPUTIME_SOFTIRQ_OFFSET as u32) != 0 && curr != lupos_cputime_this_cpu_ksoftirqd() {
            irqtime_account_delta(q, delta, CPUTIME_SOFTIRQ);
        }
    }
}

#[cfg(all(CONFIG_IRQ_TIME_ACCOUNTING, not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)))]
unsafe fn irqtime_tick_accounted(maxtime: u64) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let q = lupos_cputime_this_irqtime();
        let delta = core::cmp::min((*q).tick_delta, maxtime);
        (*q).tick_delta -= delta; delta
    }
}
#[cfg(all(not(CONFIG_IRQ_TIME_ACCOUNTING), CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
fn irqtime_tick_accounted(_: u64) -> u64 {
    0
}

unsafe fn task_group_account_field(p: *mut task_struct, index: i32, tmp: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        lupos_cputime_root_add(index, tmp);
        lupos_cputime_cgroup_account_cputime_field(p, index, tmp);
    }
}

/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn account_user_time(p: *mut task_struct, cputime: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        (*p).utime = ((*p).utime).wrapping_add(cputime); lupos_cputime_account_group_user_time(p, cputime);
        let index = if lupos_cputime_task_nice(p) > 0 { CPUTIME_NICE } else { CPUTIME_USER };
        task_group_account_field(p, index.0 as i32, cputime); lupos_cputime_acct_account_cputime(p);
    }
}

/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn account_guest_time(p: *mut task_struct, cputime: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let cpustat = addr_of_mut!((*lupos_cputime_this_cpustat()).cpustat).cast::<u64>();
        (*p).utime = ((*p).utime).wrapping_add(cputime); lupos_cputime_account_group_user_time(p, cputime); (*p).gtime = ((*p).gtime).wrapping_add(cputime);
        if lupos_cputime_task_nice(p) > 0 {
            task_group_account_field(p, CPUTIME_NICE.0 as i32, cputime);
            let guest = cpustat.add(CPUTIME_GUEST_NICE.0 as usize);
            *guest = (*guest).wrapping_add(cputime);
        } else {
            task_group_account_field(p, CPUTIME_USER.0 as i32, cputime);
            let guest = cpustat.add(CPUTIME_GUEST.0 as usize);
            *guest = (*guest).wrapping_add(cputime);
        }
    }
}

/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn account_system_index_time(p: *mut task_struct, cputime: u64, index: cpu_usage_stat) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        (*p).stime = ((*p).stime).wrapping_add(cputime); lupos_cputime_account_group_system_time(p, cputime);
        task_group_account_field(p, index.0 as i32, cputime); lupos_cputime_acct_account_cputime(p);
    }
}

/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn account_system_time(p: *mut task_struct, hardirq_offset: i32, cputime: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if (*p).flags & LUPOS_CPUTIME_PF_VCPU != 0 && lupos_cputime_irq_count().wrapping_sub(hardirq_offset as u32) == 0 { account_guest_time(p, cputime); return; }
        let index = if lupos_cputime_hardirq_count().wrapping_sub(hardirq_offset as u32) != 0 { CPUTIME_IRQ }
            else if lupos_cputime_in_serving_softirq() { CPUTIME_SOFTIRQ } else { CPUTIME_SYSTEM };
        account_system_index_time(p, cputime, index);
    }
}

/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn account_steal_time(cputime: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let kc = lupos_cputime_this_cpustat();
        (*kc).cpustat[CPUTIME_STEAL.0 as usize] =
            (*kc).cpustat[CPUTIME_STEAL.0 as usize].wrapping_add(cputime);
    }
}
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn account_idle_time(cputime: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let s = addr_of_mut!((*lupos_cputime_this_cpustat()).cpustat).cast::<u64>(); let rq = lupos_cputime_this_rq();
        if lupos_cputime_atomic_read(addr_of!((*rq).nr_iowait)) > 0 { *s.add(CPUTIME_IOWAIT.0 as usize) = (*s.add(CPUTIME_IOWAIT.0 as usize)).wrapping_add(cputime); }
        else { *s.add(CPUTIME_IDLE.0 as usize) = (*s.add(CPUTIME_IDLE.0 as usize)).wrapping_add(cputime); }
    }
}

#[cfg(CONFIG_SCHED_CORE)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn __account_forceidle_time(p: *mut task_struct, delta: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        lupos_cputime_forceidle_add(p, delta);
        task_group_account_field(p, CPUTIME_FORCEIDLE.0 as i32, delta);
    }
}

#[cfg(any(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE), all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE))))]
unsafe fn steal_account_process_time(maxtime: u64) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    #[cfg(CONFIG_PARAVIRT)]
    unsafe {
        if lupos_cputime_steal_enabled() {
            let rq = lupos_cputime_this_rq();
            let mut steal = lupos_cputime_steal_clock(lupos_cputime_smp_processor_id())
                .wrapping_sub((*rq).prev_steal_time);
            steal = core::cmp::min(steal, maxtime);
            account_steal_time(steal);
            (*rq).prev_steal_time = (*rq).prev_steal_time.wrapping_add(steal);
            return steal;
        }
    }
    #[cfg(not(CONFIG_PARAVIRT))]
    let _ = maxtime;
    0
}
#[cfg(any(CONFIG_VIRT_CPU_ACCOUNTING_GEN, all(CONFIG_IRQ_TIME_ACCOUNTING, not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))))]
unsafe fn account_other_time(max: u64) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        lupos_cputime_lockdep_assert_irqs_disabled();
        let mut accounted = steal_account_process_time(max);
        if accounted < max {
            accounted += irqtime_tick_accounted(max - accounted);
        }
        accounted
    }
}

/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn thread_group_cputime(tsk: *mut task_struct, times: *mut task_cputime) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let sig = (*tsk).signal;
        let curr = lupos_cputime_current();
        if lupos_cputime_same_thread_group(curr, tsk) {
            let _ = task_sched_runtime(curr);
        }
        lupos_cputime_rcu_read_lock();
        let mut seq = 0;
        loop {
            let flags = lupos_cputime_group_read_begin(sig, &mut seq);
            (*times).utime = (*sig).utime; (*times).stime = (*sig).stime; (*times).sum_exec_runtime = (*sig).sum_sched_runtime;
            let mut t = lupos_cputime_thread_first(sig);
            while !t.is_null() {
                let (mut u, mut s) = (0, 0);
                task_cputime(t, &mut u, &mut s);
                (*times).utime = ((*times).utime).wrapping_add(u);
                (*times).stime = ((*times).stime).wrapping_add(s);
                (*times).sum_exec_runtime = ((*times).sum_exec_runtime).wrapping_add(read_sum_exec_runtime(t));
                t = lupos_cputime_thread_next(sig, t);
            }
            let retry = lupos_cputime_group_read_retry(sig, seq);
            lupos_cputime_group_read_end(sig, seq, flags);
            if !retry { break; }
            // Match scoped_seqlock_read(..., ss_lock_irqsave): one lockless
            // attempt, then an IRQ-saving locked read if a writer raced us.
            seq = 1;
        }
        lupos_cputime_rcu_read_unlock();
    }
}

#[cfg(CONFIG_64BIT)]
unsafe fn read_sum_exec_runtime(t: *mut task_struct) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        (*t).se.sum_exec_runtime
    }
}

#[cfg(not(CONFIG_64BIT))]
unsafe fn read_sum_exec_runtime(t: *mut task_struct) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
        let rq = lupos_cputime_task_rq_lock(t, rf.as_mut_ptr());
        let runtime = (*t).se.sum_exec_runtime;
        lupos_cputime_task_rq_unlock(rq, t, rf.as_mut_ptr());
        runtime
    }
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn cputime_adjust(curr: *mut task_cputime, prev: *mut prev_cputime, ut: *mut u64, st: *mut u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let mut stime; let mut utime;
        let flags = lupos_cputime_prev_lock(prev);
        let rtime = (*curr).sum_exec_runtime;
        if (*prev).stime.wrapping_add((*prev).utime) >= rtime {
            *ut = (*prev).utime;
            *st = (*prev).stime;
            lupos_cputime_prev_unlock(prev, flags);
            return;
        }
        stime = (*curr).stime; utime = (*curr).utime;
        if stime != 0 {
            if utime == 0 { stime = rtime; }
            else { stime = lupos_cputime_mul_u64_u64_div_u64(stime, rtime, stime.wrapping_add(utime)); }
        }
        if stime < (*prev).stime { stime = (*prev).stime; } utime = rtime.wrapping_sub(stime);
        if utime < (*prev).utime { utime = (*prev).utime; stime = rtime.wrapping_sub(utime); }
        (*prev).stime = stime;
        (*prev).utime = utime;
        *ut = (*prev).utime;
        *st = (*prev).stime;
        lupos_cputime_prev_unlock(prev, flags);
    }
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn task_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let mut c = task_cputime {
            sum_exec_runtime: (*p).se.sum_exec_runtime,
            utime: 0,
            stime: 0,
        };
        if task_cputime(p, &mut c.utime, &mut c.stime) { c.sum_exec_runtime = task_sched_runtime(p); }
        cputime_adjust(&mut c, addr_of_mut!((*p).prev_cputime), ut, st);
    }
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn thread_group_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let mut c = task_cputime {
            sum_exec_runtime: 0,
            utime: 0,
            stime: 0,
        };
        thread_group_cputime(p, &mut c);
        cputime_adjust(&mut c, addr_of_mut!((*(*p).signal).prev_cputime), ut, st);
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn cputime_adjust(curr: *mut task_cputime, _prev: *mut prev_cputime, ut: *mut u64, st: *mut u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        *ut = (*curr).utime;
        *st = (*curr).stime;
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn task_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        *ut = (*p).utime;
        *st = (*p).stime;
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn thread_group_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let mut c = task_cputime {
            sum_exec_runtime: 0,
            utime: 0,
            stime: 0,
        };
        thread_group_cputime(p, &mut c);
        *ut = c.utime;
        *st = c.stime;
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn vtime_account_irq(tsk: *mut task_struct, offset: u32) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let pc = lupos_cputime_irq_count().wrapping_sub(offset);
        if pc & (LUPOS_CPUTIME_HARDIRQ_OFFSET as u32) != 0 {
            lupos_cputime_vtime_account_hardirq(tsk);
        } else if pc & (LUPOS_CPUTIME_SOFTIRQ_OFFSET as u32) != 0 {
            lupos_cputime_vtime_account_softirq(tsk);
        } else if !lupos_cputime_kcpustat_idle_dyntick() {
            #[cfg(not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE))]
            if lupos_cputime_is_idle_task(tsk) {
                lupos_cputime_vtime_account_idle(tsk);
                return;
            }
            lupos_cputime_arch_account_kernel(tsk);
        } else {
            lupos_cputime_vtime_reset();
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn vtime_delta(v: *mut vtime) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let c = lupos_cputime_sched_clock();
        if c < (*v).starttime { 0 } else { c - (*v).starttime }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn get_vtime_delta(v: *mut vtime) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let delta = vtime_delta(v);
        let other = account_other_time(delta);
        lupos_cputime_warn_vtime_inactive(v);
        (*v).starttime = (*v).starttime.wrapping_add(delta);
        delta - other
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn vtime_account_system(t: *mut task_struct, v: *mut vtime) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        (*v).stime = (*v).stime.wrapping_add(get_vtime_delta(v));
        if (*v).stime >= lupos_cputime_tick_nsec() {
            account_system_time(t, lupos_cputime_irq_count() as i32, (*v).stime);
            (*v).stime = 0;
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn vtime_account_guest(t: *mut task_struct, v: *mut vtime) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        (*v).gtime = (*v).gtime.wrapping_add(get_vtime_delta(v));
        if (*v).gtime >= lupos_cputime_tick_nsec() {
            account_guest_time(t, (*v).gtime);
            (*v).gtime = 0;
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn __vtime_account_kernel(t: *mut task_struct, v: *mut vtime) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if (*v).state == VTIME_GUEST {
            vtime_account_guest(t, v);
        } else {
            vtime_account_system(t, v);
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn vtime_account_kernel(tsk: *mut task_struct) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*tsk).vtime);
        if vtime_delta(v) == 0 { return; }
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*v).seqcount));
        __vtime_account_kernel(tsk, v);
        lupos_cputime_write_seqcount_end(addr_of_mut!((*v).seqcount));
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn vtime_user_enter(t: *mut task_struct) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*t).vtime);
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*v).seqcount));
        vtime_account_system(t, v);
        (*v).state = VTIME_USER;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*v).seqcount));
    }
}
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn vtime_user_exit(t: *mut task_struct) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v=addr_of_mut!((*t).vtime);
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*v).seqcount));
        (*v).utime = (*v).utime.wrapping_add(get_vtime_delta(v));
        if (*v).utime >= lupos_cputime_tick_nsec() {
            account_user_time(t, (*v).utime);
            (*v).utime = 0;
        }
        (*v).state=VTIME_SYS;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*v).seqcount));
    }
}
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn vtime_guest_enter(t: *mut task_struct) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*t).vtime);
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*v).seqcount));
        vtime_account_system(t, v);
        (*t).flags |= LUPOS_CPUTIME_PF_VCPU;
        (*v).state = VTIME_GUEST;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*v).seqcount));
    }
}
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn vtime_guest_exit(t: *mut task_struct) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*t).vtime);
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*v).seqcount));
        vtime_account_guest(t, v);
        (*t).flags &= !LUPOS_CPUTIME_PF_VCPU;
        (*v).state = VTIME_SYS;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*v).seqcount));
    }
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn account_process_tick(p: *mut task_struct, user_tick: i32) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if lupos_cputime_vtime_accounting_enabled_this_cpu() || lupos_cputime_kcpustat_idle_dyntick() {
            return;
        }
        #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
        if lupos_cputime_irqtime_enabled() { irqtime_account_process_tick(p, user_tick, 1); return; }
        let mut cputime = lupos_cputime_tick_nsec(); let steal = steal_account_process_time(lupos_cputime_ulong_max());
        if steal >= cputime { return; } cputime -= steal;
        if user_tick != 0 { account_user_time(p, cputime); }
        else if p != (*lupos_cputime_this_rq()).idle || lupos_cputime_irq_count() != LUPOS_CPUTIME_HARDIRQ_OFFSET as u32 { account_system_time(p, LUPOS_CPUTIME_HARDIRQ_OFFSET as i32, cputime); }
        else { account_idle_time(cputime); }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn task_gtime(t: *mut task_struct) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*t).vtime);
        if !lupos_cputime_vtime_accounting_enabled() {
            return (*t).gtime;
        }
        loop {
            let seq=lupos_cputime_read_seqcount_begin(addr_of!((*v).seqcount));
            let mut g=(*t).gtime;
            if (*v).state==VTIME_GUEST { g = g.wrapping_add((*v).gtime).wrapping_add(vtime_delta(v)); }
            if !lupos_cputime_read_seqcount_retry(addr_of!((*v).seqcount),seq) { return g; }
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// Task and accounting pointers must remain live; output pointers must be writable.
/// The native scheduler lifetime and locking rules apply to each supplied object.
#[no_mangle]
pub unsafe extern "C" fn task_cputime(t: *mut task_struct, ut: *mut u64, st: *mut u64) -> bool {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*t).vtime);
        if !lupos_cputime_vtime_accounting_enabled() {
            *ut = (*t).utime;
            *st = (*t).stime;
            return false;
        }
        loop { let seq=lupos_cputime_read_seqcount_begin(addr_of!((*v).seqcount)); *ut=(*t).utime; *st=(*t).stime;
            let active = (*v).state.0 >= VTIME_SYS.0;
            if active { let d=vtime_delta(v); if (*v).state==VTIME_SYS { *st = (*st).wrapping_add((*v).stime).wrapping_add(d); } else { *ut = (*ut).wrapping_add((*v).utime).wrapping_add(d); } }
            if !lupos_cputime_read_seqcount_retry(addr_of!((*v).seqcount),seq) { return active; }
        }
    }
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
unsafe fn task_cputime(t: *mut task_struct, ut: *mut u64, st: *mut u64) -> bool {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        *ut = (*t).utime;
        *st = (*t).stime;
        false
    }
}

#[cfg(all(CONFIG_IRQ_TIME_ACCOUNTING, not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)))]
unsafe fn irqtime_account_process_tick(p: *mut task_struct, user_tick: i32, ticks: i32) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let mut cputime = lupos_cputime_tick_nsec().wrapping_mul(ticks as u64);
        let other = account_other_time(lupos_cputime_ulong_max());
        if other >= cputime { return; }
        cputime -= other;

        if lupos_cputime_this_cpu_ksoftirqd() == p {
            account_system_index_time(p, cputime, CPUTIME_SOFTIRQ);
        } else if user_tick != 0 {
            account_user_time(p, cputime);
        } else if p == (*lupos_cputime_this_rq()).idle {
            account_idle_time(cputime);
        } else if (*p).flags & LUPOS_CPUTIME_PF_VCPU != 0 {
            account_guest_time(p, cputime);
        } else {
            account_system_index_time(p, cputime, CPUTIME_SYSTEM);
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn vtime_task_switch_generic(prev: *mut task_struct) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*prev).vtime);
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*v).seqcount));
        if (*v).state == VTIME_IDLE {
            account_idle_time(get_vtime_delta(v));
        } else {
            __vtime_account_kernel(prev, v);
        }
        (*v).state = VTIME_INACTIVE;
        (*v).cpu = -1i32 as u32;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*v).seqcount));

        let curr = lupos_cputime_current();
        let v = addr_of_mut!((*curr).vtime);
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*v).seqcount));
        (*v).state = if lupos_cputime_is_idle_task(curr) {
            VTIME_IDLE
        } else if (*curr).flags & LUPOS_CPUTIME_PF_VCPU != 0 {
            VTIME_GUEST
        } else {
            VTIME_SYS
        };
        (*v).starttime = lupos_cputime_sched_clock();
        (*v).cpu = lupos_cputime_smp_processor_id() as u32;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*v).seqcount));
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn vtime_init_idle(t: *mut task_struct, cpu: i32) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*t).vtime);
        let flags = lupos_cputime_local_irq_save();
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*v).seqcount));
        (*v).state = VTIME_IDLE;
        (*v).starttime = lupos_cputime_sched_clock();
        (*v).cpu = cpu as u32;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*v).seqcount));
        lupos_cputime_local_irq_restore(flags);
    }
}


#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
unsafe fn kcpustat_idle_stop(kc: *mut kernel_cpustat, now: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if !(*kc).idle_elapse { return; }
        let iowait = (lupos_cputime_nr_iowait_cpu(lupos_cputime_smp_processor_id()) > 0) as usize;
        let mut delta = now.wrapping_sub((*kc).idle_entrytime);
        let steal = steal_account_process_time(delta);

        lupos_cputime_write_seqcount_begin(addr_of_mut!((*kc).idle_sleeptime_seq));
        // Subtract only previously deferred steal so concurrent readers cannot
        // move backward when the current update learns about newly stolen time.
        let steal_delta = core::cmp::min((*kc).idle_stealtime[iowait], delta);
        delta -= steal_delta;
        (*kc).idle_stealtime[iowait] -= steal_delta;
        let index = if iowait != 0 { CPUTIME_IOWAIT } else { CPUTIME_IDLE };
        (*kc).cpustat[index.0 as usize] = (*kc).cpustat[index.0 as usize].wrapping_add(delta);
        (*kc).idle_stealtime[iowait] = (*kc).idle_stealtime[iowait].wrapping_add(steal);
        (*kc).idle_entrytime = now;
        (*kc).idle_elapse = false;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*kc).idle_sleeptime_seq));
    }
}

#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
unsafe fn kcpustat_idle_start(kc: *mut kernel_cpustat, now: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if (*kc).idle_elapse {
            return;
        }
        lupos_cputime_write_seqcount_begin(addr_of_mut!((*kc).idle_sleeptime_seq));
        (*kc).idle_entrytime = now;
        (*kc).idle_elapse = true;
        lupos_cputime_write_seqcount_end(addr_of_mut!((*kc).idle_sleeptime_seq));
    }
}

#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn kcpustat_dyntick_stop(now: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let kc = lupos_cputime_this_cpustat();
        if !lupos_cputime_vtime_generic_enabled_this_cpu() {
            lupos_cputime_warn_idle_dyntick(kc);
            kcpustat_idle_stop(kc, now);
            (*kc).idle_dyntick = false;
            lupos_cputime_vtime_dyntick_stop();
        }
    }
}

#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn kcpustat_dyntick_start(now: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let kc = lupos_cputime_this_cpustat();
        if !lupos_cputime_vtime_generic_enabled_this_cpu() {
            lupos_cputime_vtime_dyntick_start();
            (*kc).idle_dyntick = true;
            kcpustat_idle_start(kc, now);
        }
    }
}

#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn kcpustat_irq_enter(now: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if !lupos_cputime_vtime_generic_enabled_this_cpu()
            && (lupos_cputime_irqtime_enabled() || lupos_cputime_vtime_accounting_enabled_this_cpu())
        {
            kcpustat_idle_stop(lupos_cputime_this_cpustat(), now);
        }
    }
}

#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The caller must keep any task pointers live and hold the interrupt, preemption,
/// and scheduler synchronization required by the corresponding native accounting API.
#[no_mangle]
pub unsafe extern "C" fn kcpustat_irq_exit(now: u64) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if !lupos_cputime_vtime_generic_enabled_this_cpu()
            && (cfg!(CONFIG_IRQ_TIME_ACCOUNTING) || lupos_cputime_vtime_accounting_enabled_this_cpu())
        {
            kcpustat_idle_start(lupos_cputime_this_cpustat(), now);
        }
    }
}

#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
unsafe fn kcpustat_field_dyntick(cpu: i32, index: cpu_usage_stat, compute_delta: bool, now: u64) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let kc = lupos_cputime_cpu_cpustat(cpu);
        let iowait = (index == CPUTIME_IOWAIT) as usize;
        loop {
            let seq = lupos_cputime_read_seqcount_begin(addr_of!((*kc).idle_sleeptime_seq));
            let mut idle = (*kc).cpustat[index.0 as usize];
            if (*kc).idle_elapse && compute_delta && now > (*kc).idle_entrytime {
                let mut delta = now - (*kc).idle_entrytime;
                delta -= core::cmp::min((*kc).idle_stealtime[iowait], delta);
                idle = idle.wrapping_add(delta);
            }
            if !lupos_cputime_read_seqcount_retry(addr_of!((*kc).idle_sleeptime_seq), seq) {
                return idle;
            }
        }
    }
}

#[cfg(not(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE))))]
unsafe fn kcpustat_field_dyntick(cpu: i32, index: cpu_usage_stat, _compute_delta: bool, _now: u64) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        (*lupos_cputime_cpu_cpustat(cpu)).cpustat[index.0 as usize]
    }
}

#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The CPU index must identify a valid native per-CPU allocation. Any non-null
/// output pointer must be writable for its native type for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn kcpustat_field_idle(cpu: i32) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        kcpustat_field_dyntick(cpu, CPUTIME_IDLE, lupos_cputime_nr_iowait_cpu(cpu) == 0,
                             lupos_cputime_ktime_get() as u64)
    }
}

#[cfg(all(CONFIG_NO_HZ_COMMON, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The CPU index must identify a valid native per-CPU allocation. Any non-null
/// output pointer must be writable for its native type for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn kcpustat_field_iowait(cpu: i32) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        kcpustat_field_dyntick(cpu, CPUTIME_IOWAIT, lupos_cputime_nr_iowait_cpu(cpu) != 0,
                             lupos_cputime_ktime_get() as u64)
    }
}

unsafe fn get_cpu_sleep_time_us(cpu: i32, index: cpu_usage_stat, compute_delta: bool, last_update_time: *mut u64) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let now = lupos_cputime_ktime_get();
        let res = if lupos_cputime_vtime_generic_enabled_cpu(cpu) {
            #[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
            { kcpustat_field(index, cpu) }
            #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
            { lupos_cputime_field_default(index, cpu) }
        } else {
            kcpustat_field_dyntick(cpu, index, compute_delta, now as u64)
        };
        if !last_update_time.is_null() {
            *last_update_time = lupos_cputime_ktime_to_us(now) as u64;
        }
        res / LUPOS_CPUTIME_NSEC_PER_USEC as u64
    }
}

/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The CPU index must identify a valid native per-CPU allocation. Any non-null
/// output pointer must be writable for its native type for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn get_cpu_idle_time_us(cpu: i32, last_update_time: *mut u64) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        get_cpu_sleep_time_us(cpu, CPUTIME_IDLE, lupos_cputime_nr_iowait_cpu(cpu) == 0, last_update_time)
    }
}

/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The CPU index must identify a valid native per-CPU allocation. Any non-null
/// output pointer must be writable for its native type for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn get_cpu_iowait_time_us(cpu: i32, last_update_time: *mut u64) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        get_cpu_sleep_time_us(cpu, CPUTIME_IOWAIT, lupos_cputime_nr_iowait_cpu(cpu) != 0, last_update_time)
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn vtime_state_fetch(v: *mut vtime, cpu: i32) -> i32 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let state = lupos_cputime_vtime_state_read_once(v).0 as i32;
        if (*v).cpu != cpu as u32 && (*v).cpu != -1i32 as u32 {
            return -(LUPOS_CPUTIME_EAGAIN as i32);
        }
        if state == VTIME_INACTIVE.0 as i32 {
            return -(LUPOS_CPUTIME_EAGAIN as i32);
        }
        state
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn kcpustat_user_vtime(v: *mut vtime) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        if (*v).state == VTIME_USER {
            (*v).utime.wrapping_add(vtime_delta(v))
        } else if (*v).state == VTIME_GUEST {
            (*v).gtime.wrapping_add(vtime_delta(v))
        } else {
            0
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn kcpustat_field_vtime(cpustat: *const u64, tsk: *mut task_struct, usage: cpu_usage_stat, cpu: i32, val: *mut u64) -> i32 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*tsk).vtime);
        let rq = lupos_cputime_cpu_rq(cpu);
        loop {
            let seq = lupos_cputime_read_seqcount_begin(addr_of!((*v).seqcount));
            let state = vtime_state_fetch(v, cpu);
            if state < 0 { return state; }
            *val = *cpustat.add(usage.0 as usize);
            let delta = match usage {
                CPUTIME_SYSTEM if state == VTIME_SYS.0 as i32 => (*v).stime.wrapping_add(vtime_delta(v)),
                CPUTIME_USER if lupos_cputime_task_nice(tsk) <= 0 => kcpustat_user_vtime(v),
                CPUTIME_NICE if lupos_cputime_task_nice(tsk) > 0 => kcpustat_user_vtime(v),
                CPUTIME_GUEST if state == VTIME_GUEST.0 as i32 && lupos_cputime_task_nice(tsk) <= 0 => (*v).gtime.wrapping_add(vtime_delta(v)),
                CPUTIME_GUEST_NICE if state == VTIME_GUEST.0 as i32 && lupos_cputime_task_nice(tsk) > 0 => (*v).gtime.wrapping_add(vtime_delta(v)),
                CPUTIME_IDLE if state == VTIME_IDLE.0 as i32 && lupos_cputime_atomic_read(addr_of!((*rq).nr_iowait)) == 0 => vtime_delta(v),
                CPUTIME_IOWAIT if state == VTIME_IDLE.0 as i32 && lupos_cputime_atomic_read(addr_of!((*rq).nr_iowait)) > 0 => vtime_delta(v),
                _ => 0,
            };
            *val = (*val).wrapping_add(delta);
            if !lupos_cputime_read_seqcount_retry(addr_of!((*v).seqcount), seq) {
                return 0;
            }
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The CPU index must identify a valid native per-CPU allocation. Any non-null
/// output pointer must be writable for its native type for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn kcpustat_field(usage: cpu_usage_stat, cpu: i32) -> u64 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let cpustat = addr_of!((*lupos_cputime_cpu_cpustat(cpu)).cpustat).cast::<u64>();
        if !lupos_cputime_vtime_generic_enabled_cpu(cpu) {
            return lupos_cputime_field_default(usage, cpu);
        }
        loop {
            lupos_cputime_rcu_read_lock();
            let curr = lupos_cputime_rq_current(cpu);
            if lupos_cputime_warn_null_task(curr) {
                lupos_cputime_rcu_read_unlock();
                return *cpustat.add(usage.0 as usize);
            }
            let mut val = 0;
            let err = kcpustat_field_vtime(cpustat, curr, usage, cpu, &mut val);
            lupos_cputime_rcu_read_unlock();
            if err == 0 { return val; }
            lupos_cputime_cpu_relax();
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe fn kcpustat_cpu_fetch_vtime(dst: *mut kernel_cpustat, src: *const kernel_cpustat, tsk: *mut task_struct, cpu: i32) -> i32 {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let v = addr_of_mut!((*tsk).vtime);
        loop {
            let seq = lupos_cputime_read_seqcount_begin(addr_of!((*v).seqcount));
            let state = vtime_state_fetch(v, cpu);
            if state < 0 { return state; }
            core::ptr::copy(src, dst, 1);
            if state >= VTIME_IDLE.0 as i32 {
                let cpustat = addr_of_mut!((*dst).cpustat).cast::<u64>();
                let delta = vtime_delta(v);
                let (index, time) = match vtime_state(state as _) {
                    VTIME_SYS => (CPUTIME_SYSTEM, (*v).stime.wrapping_add(delta)),
                    VTIME_USER => {
                        let index = if lupos_cputime_task_nice(tsk) > 0 { CPUTIME_NICE } else { CPUTIME_USER };
                        (index, (*v).utime.wrapping_add(delta))
                    }
                    VTIME_GUEST => {
                        let nice = lupos_cputime_task_nice(tsk) > 0;
                        let guest_index = if nice { CPUTIME_GUEST_NICE } else { CPUTIME_GUEST };
                        let time = (*v).gtime.wrapping_add(delta);
                        let guest = cpustat.add(guest_index.0 as usize);
                        *guest = (*guest).wrapping_add(time);
                        (if nice { CPUTIME_NICE } else { CPUTIME_USER }, time)
                    }
                    VTIME_IDLE => {
                        let rq = lupos_cputime_cpu_rq(cpu);
                        let index = if lupos_cputime_atomic_read(addr_of!((*rq).nr_iowait)) > 0 { CPUTIME_IOWAIT } else { CPUTIME_IDLE };
                        (index, delta)
                    }
                    _ => {
                        lupos_cputime_warn_bad_vtime_state();
                        // No field is modified for the native switch's default.
                        if !lupos_cputime_read_seqcount_retry(addr_of!((*v).seqcount), seq) { return 0; }
                        continue;
                    }
                };
                let target = cpustat.add(index.0 as usize);
                *target = (*target).wrapping_add(time);
            }
            if !lupos_cputime_read_seqcount_retry(addr_of!((*v).seqcount), seq) { return 0; }
        }
    }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
/// Native CPU-accounting entry point.
///
/// # Safety
///
/// The CPU index must identify a valid native per-CPU allocation. Any non-null
/// output pointer must be writable for its native type for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn kcpustat_cpu_fetch(dst: *mut kernel_cpustat, cpu: i32) {
    // SAFETY: The native caller or enclosing accounting path supplies the
    // object lifetime and synchronization contract; raw pointers avoid claiming
    // exclusive Rust references to scheduler objects shared with native code.
    unsafe {
        let src = lupos_cputime_cpu_cpustat(cpu);
        if !lupos_cputime_vtime_generic_enabled_cpu(cpu) {
            lupos_cputime_cpu_fetch_default(dst, cpu);
            return;
        }
        loop {
            lupos_cputime_rcu_read_lock();
            let curr = lupos_cputime_rq_current(cpu);
            if lupos_cputime_warn_null_task_fetch(curr) {
                lupos_cputime_rcu_read_unlock();
                lupos_cputime_cpu_fetch_default(dst, cpu);
                return;
            }
            let err = kcpustat_cpu_fetch_vtime(dst, src, curr, cpu);
            lupos_cputime_rcu_read_unlock();
            if err == 0 { return; }
            lupos_cputime_cpu_relax();
        }
    }
}
