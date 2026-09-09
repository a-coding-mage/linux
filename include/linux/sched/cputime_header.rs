/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/sched/cputime.h.  The included kernel declarations are
// intentionally left as external dependencies.

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
extern "C" {
    pub fn task_cputime(t: *mut task_struct, utime: *mut u64, stime: *mut u64) -> bool;
    pub fn task_gtime(t: *mut task_struct) -> u64;
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn task_cputime(t: *mut task_struct, utime: *mut u64, stime: *mut u64) -> bool {
    *utime = (*t).utime;
    *stime = (*t).stime;
    false
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn task_gtime(t: *mut task_struct) -> u64 {
    (*t).gtime
}

#[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)]
#[inline]
pub unsafe fn task_cputime_scaled(t: *mut task_struct, utimescaled: *mut u64, stimescaled: *mut u64) {
    *utimescaled = (*t).utimescaled;
    *stimescaled = (*t).stimescaled;
}

#[cfg(not(CONFIG_ARCH_HAS_SCALED_CPUTIME))]
#[inline]
pub unsafe fn task_cputime_scaled(t: *mut task_struct, utimescaled: *mut u64, stimescaled: *mut u64) {
    task_cputime(t, utimescaled, stimescaled);
}

extern "C" {
    pub fn task_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64);
    pub fn thread_group_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64);
    pub fn cputime_adjust(curr: *mut task_cputime, prev: *mut prev_cputime, ut: *mut u64, st: *mut u64);
    pub fn thread_group_cputime(tsk: *mut task_struct, times: *mut task_cputime);
    pub fn thread_group_sample_cputime(tsk: *mut task_struct, samples: *mut u64);
}

#[cfg(CONFIG_POSIX_TIMERS)]
#[inline]
pub unsafe fn get_running_cputimer(tsk: *mut task_struct) -> *mut thread_group_cputimer {
    let cputimer = &mut (*(*tsk).signal).cputimer as *mut thread_group_cputimer;
    if !READ_ONCE((*(*tsk).signal).posix_cputimers.timers_active) {
        return core::ptr::null_mut();
    }
    if (*tsk).sighand.is_null() {
        return core::ptr::null_mut();
    }
    cputimer
}

#[cfg(not(CONFIG_POSIX_TIMERS))]
#[inline]
pub unsafe fn get_running_cputimer(_tsk: *mut task_struct) -> *mut thread_group_cputimer {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn account_group_user_time(tsk: *mut task_struct, cputime: u64) {
    let cputimer = get_running_cputimer(tsk);
    if cputimer.is_null() { return; }
    atomic64_add(cputime, &mut (*cputimer).cputime_atomic.utime);
}

#[inline]
pub unsafe fn account_group_system_time(tsk: *mut task_struct, cputime: u64) {
    let cputimer = get_running_cputimer(tsk);
    if cputimer.is_null() { return; }
    atomic64_add(cputime, &mut (*cputimer).cputime_atomic.stime);
}

#[inline]
pub unsafe fn account_group_exec_runtime(tsk: *mut task_struct, ns: u64) {
    let cputimer = get_running_cputimer(tsk);
    if cputimer.is_null() { return; }
    atomic64_add(ns, &mut (*cputimer).cputime_atomic.sum_exec_runtime);
}

#[inline]
pub unsafe fn prev_cputime_init(prev: *mut prev_cputime) {
    #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
    {
        (*prev).utime = 0;
        (*prev).stime = 0;
        raw_spin_lock_init(&mut (*prev).lock);
    }
}

extern "C" {
    pub fn task_sched_runtime(task: *mut task_struct) -> u64;
}

#[cfg(CONFIG_PARAVIRT)]
extern "C" {
    pub static mut paravirt_steal_enabled: static_key;
    pub static mut paravirt_steal_rq_enabled: static_key;
}

#[cfg(all(CONFIG_PARAVIRT, CONFIG_HAVE_PV_STEAL_CLOCK_GEN))]
extern "C" {
    pub fn dummy_steal_clock(cpu: i32) -> u64;
    // DECLARE_STATIC_CALL(pv_steal_clock, dummy_steal_clock)
    pub fn static_call_pv_steal_clock(cpu: i32) -> u64;
}

#[cfg(all(CONFIG_PARAVIRT, CONFIG_HAVE_PV_STEAL_CLOCK_GEN))]
#[inline]
pub unsafe fn paravirt_steal_clock(cpu: i32) -> u64 {
    static_call_pv_steal_clock(cpu)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
