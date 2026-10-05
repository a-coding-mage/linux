// SPDX-License-Identifier: GPL-2.0-only
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// Continued from the existing Rust owner against baseline 126a30fa.
// SOURCE ONLY: no ABI, native protection, configuration or runtime acceptance.
// C types/constants are generated from sched_syscalls_bindings.h, never invented.
#![no_std]

use core::mem::{size_of, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut};
use kernel::bindings::sched_syscalls_native::*;
use kernel::ffi::{c_int, c_long, c_uint, c_ulong};

const MAX_DL_PRIO: c_int = LUPOS_SYS_MAX_DL_PRIO as c_int;
const MAX_RT_PRIO: c_int = LUPOS_SYS_MAX_RT_PRIO as c_int;
const MIN_NICE: c_int = LUPOS_SYS_MIN_NICE as c_int;
const MAX_NICE: c_int = LUPOS_SYS_MAX_NICE as c_int;
const NICE_WIDTH: c_int = LUPOS_SYS_NICE_WIDTH as c_int;
const CAP_SYS_NICE: c_int = LUPOS_SYS_CAP_SYS_NICE as c_int;
const EPERM: c_int = LUPOS_SYS_EPERM as c_int;
const ESRCH: c_int = LUPOS_SYS_ESRCH as c_int;
const EINVAL: c_int = LUPOS_SYS_EINVAL as c_int;
const ENOMEM: c_int = LUPOS_SYS_ENOMEM as c_int;
const EFAULT: c_int = LUPOS_SYS_EFAULT as c_int;
const E2BIG: c_int = LUPOS_SYS_E2BIG as c_int;
const EBUSY: c_int = LUPOS_SYS_EBUSY as c_int;
#[cfg(not(CONFIG_UCLAMP_TASK))]
const EOPNOTSUPP: c_int = LUPOS_SYS_EOPNOTSUPP as c_int;
// Private sentinel defined by the original C owner, not a native ABI constant.
const SETPARAM_POLICY: c_int = -1;

// The native helpers retain RCU instrumentation and the C task-ref primitives.
// These private guards may only be constructed under the kernel call contracts.
struct Rcu;
impl Rcu {
    unsafe fn lock() -> Self {
        unsafe { lupos_syscalls_rcu_lock(); }
        Self
    }
}
impl Drop for Rcu {
    fn drop(&mut self) {
        unsafe { lupos_syscalls_rcu_unlock(); }
    }
}
struct TaskRef(*mut task_struct);
impl Drop for TaskRef {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() { lupos_syscalls_put_task(self.0); }
        }
    }
}

#[inline]
unsafe fn __normal_prio(policy: c_int, rt_prio: c_int, nice: c_int) -> c_int {
    unsafe {
        if lupos_syscalls_dl_policy(policy) { MAX_DL_PRIO - 1 }
        else if lupos_syscalls_rt_policy(policy) { MAX_RT_PRIO - 1 - rt_prio }
        else { lupos_syscalls_nice_to_prio(nice) }
    }
}
#[inline]
unsafe fn normal_prio(p: *mut task_struct) -> c_int {
    unsafe {
        __normal_prio((*p).policy as c_int, (*p).rt_priority as c_int,
                      lupos_syscalls_prio_to_nice((*p).static_prio))
    }
}
unsafe fn effective_prio(p: *mut task_struct) -> c_int {
    unsafe {
        (*p).normal_prio = normal_prio(p);
        if !lupos_syscalls_rt_or_dl_prio((*p).prio) { (*p).normal_prio }
        else { (*p).prio }
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_user_nice(p: *mut task_struct, nice: c_long) {
    unsafe {
        // Validate the original long before narrowing it.
        if lupos_syscalls_task_nice(p) as c_long == nice ||
           nice < MIN_NICE as c_long || nice > MAX_NICE as c_long { return; }
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = lupos_syscalls_task_rq_lock(p, rf.as_mut_ptr());
        if lupos_syscalls_task_has_dl_policy(p) || lupos_syscalls_task_has_rt_policy(p) {
            (*p).static_prio = lupos_syscalls_nice_to_prio(nice as c_int);
        } else {
            let scope = sched_change_begin(p, LUPOS_SYS_DEQUEUE_SAVE as c_uint);
            (*p).static_prio = lupos_syscalls_nice_to_prio(nice as c_int);
            set_load_weight(p, true);
            (*p).prio = effective_prio(p);
            sched_change_end(scope);
        }
        lupos_syscalls_task_rq_unlock(rq, p, rf.as_mut_ptr());
    }
}
unsafe fn is_nice_reduction(p: *const task_struct, nice: c_int) -> bool {
    unsafe {
        lupos_syscalls_nice_to_rlimit(nice) as c_ulong <=
            lupos_syscalls_task_rlimit(p, LUPOS_SYS_RLIMIT_NICE as c_uint)
    }
}
#[no_mangle]
pub unsafe extern "C" fn can_nice(p: *const task_struct, nice: c_int) -> c_int {
    unsafe { (is_nice_reduction(p, nice) || lupos_syscalls_capable(CAP_SYS_NICE)) as c_int }
}
// The native registration shell applies __ARCH_WANT_SYS_NICE.
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_nice(increment: c_int) -> c_long {
    unsafe {
        let increment = core::cmp::min(core::cmp::max(increment, -NICE_WIDTH), NICE_WIDTH);
        let p = lupos_syscalls_current();
        let nice = core::cmp::min(core::cmp::max(lupos_syscalls_task_nice(p) + increment, MIN_NICE), MAX_NICE);
        if increment < 0 && can_nice(p, nice) == 0 { return -EPERM as c_long; }
        let ret = lupos_syscalls_security_setnice(p, nice as c_int);
        if ret != 0 { return ret as c_long; }
        set_user_nice(p, nice as c_long);
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn task_prio(p: *const task_struct) -> c_int {
    unsafe { (*p).prio - MAX_RT_PRIO }
}
#[no_mangle]
pub unsafe extern "C" fn idle_cpu(cpu: c_int) -> c_int {
    unsafe { lupos_syscalls_idle_rq(lupos_syscalls_cpu_rq(cpu)) as c_int }
}
#[no_mangle]
pub unsafe extern "C" fn idle_task(cpu: c_int) -> *mut task_struct {
    unsafe { (*lupos_syscalls_cpu_rq(cpu)).idle }
}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn sched_core_idle_cpu(cpu: c_int) -> c_int {
    unsafe {
        let rq = lupos_syscalls_cpu_rq(cpu);
        if lupos_syscalls_sched_core_enabled(rq) && lupos_syscalls_rq_curr(rq) == (*rq).idle { return 1; }
        idle_cpu(cpu)
    }
}
unsafe fn find_process_by_pid(pid: pid_t) -> *mut task_struct {
    unsafe { if pid != 0 { find_task_by_vpid(pid) } else { lupos_syscalls_current() } }
}
unsafe fn find_get_task(pid: pid_t) -> TaskRef {
    unsafe {
        let _rcu = Rcu::lock();
        let p = find_process_by_pid(pid);
        if lupos_syscalls_likely_find_task(!p.is_null()) { lupos_syscalls_get_task(p); }
        TaskRef(p)
    }
}
unsafe fn __setscheduler_params(p: *mut task_struct, attr: *const sched_attr) {
    unsafe {
        let mut policy = (*attr).sched_policy as c_int;
        if policy == SETPARAM_POLICY { policy = (*p).policy as c_int; }
        (*p).policy = policy as c_uint;
        if lupos_syscalls_dl_policy(policy) { __setparam_dl(p, attr); }
        else if lupos_syscalls_fair_policy(policy) { __setparam_fair(p, attr); }
        if lupos_syscalls_rt_or_dl_task_policy(p) { (*p).timer_slack_ns = 0; }
        else if (*p).timer_slack_ns == 0 {
            (*p).timer_slack_ns = (*p).default_timer_slack_ns;
        }
        (*p).rt_priority = (*attr).sched_priority;
        (*p).normal_prio = normal_prio(p);
        set_load_weight(p, true);
    }
}
unsafe fn check_same_owner(p: *mut task_struct) -> bool {
    unsafe {
        let cred = lupos_syscalls_current_cred();
        let _rcu = Rcu::lock();
        let pcred = lupos_syscalls_task_cred(p);
        lupos_syscalls_cred_euid_eq(cred, pcred) ||
            lupos_syscalls_cred_euid_uid_eq(cred, pcred)
    }
}
#[cfg(CONFIG_RT_MUTEXES)]
unsafe fn __setscheduler_dl_pi(newprio: c_int, policy: c_int,
                               p: *mut task_struct, scope: *mut sched_change_ctx) {
    unsafe {
        if lupos_syscalls_dl_prio(newprio) && !lupos_syscalls_dl_policy(policy) {
            let pi_task = lupos_syscalls_rt_mutex_get_top_task(p);
            if !pi_task.is_null() {
                (*p).dl.pi_se = (*pi_task).dl.pi_se;
                (*scope).flags |= LUPOS_SYS_ENQUEUE_REPLENISH as c_int;
            }
        }
    }
}

#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_validate(p: *mut task_struct, attr: *const sched_attr) -> c_int {
    unsafe {
        let req = addr_of_mut!((*p).uclamp_req).cast::<uclamp_se>();
        let mut util_min = lupos_syscalls_uclamp_value(req.add(UCLAMP_MIN as usize)) as c_int;
        let mut util_max = lupos_syscalls_uclamp_value(req.add(UCLAMP_MAX as usize)) as c_int;
        if (*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP_MIN as u64 != 0 {
            util_min = (*attr).sched_util_min as c_int;
            if util_min.wrapping_add(1) as c_long > SCHED_CAPACITY_SCALE as c_long + 1 {
                return -EINVAL;
            }
        }
        if (*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP_MAX as u64 != 0 {
            util_max = (*attr).sched_util_max as c_int;
            if util_max.wrapping_add(1) as c_long > SCHED_CAPACITY_SCALE as c_long + 1 {
                return -EINVAL;
            }
        }
        if util_min != -1 && util_max != -1 && util_min > util_max { return -EINVAL; }
        lupos_syscalls_uclamp_enable();
        0
    }
}
#[cfg(not(CONFIG_UCLAMP_TASK))]
unsafe fn uclamp_validate(_p: *mut task_struct, _attr: *const sched_attr) -> c_int {
    // Exact original !CONFIG_UCLAMP_TASK behavior, not a fallback implementation.
    -EOPNOTSUPP
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn uclamp_reset(attr: *const sched_attr, id: uclamp_id, uc: *mut uclamp_se) -> bool {
    unsafe {
        if lupos_syscalls_likely_uclamp_reset((*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP as u64 == 0) &&
            !lupos_syscalls_uclamp_user_defined(uc) { return true; }
        if id == UCLAMP_MIN && (*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP_MIN as u64 != 0 &&
            (*attr).sched_util_min == u32::MAX { return true; }
        if id == UCLAMP_MAX && (*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP_MAX as u64 != 0 &&
            (*attr).sched_util_max == u32::MAX { return true; }
        false
    }
}
#[cfg(CONFIG_UCLAMP_TASK)]
unsafe fn __setscheduler_uclamp(p: *mut task_struct, attr: *const sched_attr) {
    unsafe {
        let req = addr_of_mut!((*p).uclamp_req).cast::<uclamp_se>();
        for id in 0..UCLAMP_CNT {
            let uc = req.add(id as usize);
            if !uclamp_reset(attr, id, uc) { continue; }
            let value = if lupos_syscalls_unlikely_uclamp_rt_min(lupos_syscalls_rt_task(p) && id == UCLAMP_MIN) {
                sysctl_sched_uclamp_util_min_rt_default
            } else { lupos_syscalls_uclamp_none(id) };
            lupos_syscalls_uclamp_set(uc, value, false);
        }
        if lupos_syscalls_likely_uclamp_no_flags((*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP as u64 == 0) { return; }
        if (*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP_MIN as u64 != 0 &&
            (*attr).sched_util_min != u32::MAX {
            lupos_syscalls_uclamp_set(req.add(UCLAMP_MIN as usize), (*attr).sched_util_min, true);
        }
        if (*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP_MAX as u64 != 0 &&
            (*attr).sched_util_max != u32::MAX {
            lupos_syscalls_uclamp_set(req.add(UCLAMP_MAX as usize), (*attr).sched_util_max, true);
        }
    }
}

unsafe fn user_check_sched_setscheduler(p: *mut task_struct, attr: *const sched_attr,
                                       policy: c_int, reset_on_fork: bool) -> c_int {
    unsafe {
        // Preserve the single late lupos_syscalls_capable() check, including audit semantics.
        let requires_privilege = 'checks: {
            if lupos_syscalls_fair_policy(policy) &&
                (*attr).sched_nice < lupos_syscalls_task_nice(p) &&
                !is_nice_reduction(p, (*attr).sched_nice) { break 'checks true; }
            if lupos_syscalls_rt_policy(policy) {
                let limit = lupos_syscalls_task_rlimit(p, LUPOS_SYS_RLIMIT_RTPRIO as c_uint);
                if policy != (*p).policy as c_int && limit == 0 { break 'checks true; }
                if (*attr).sched_priority > (*p).rt_priority &&
                    (*attr).sched_priority as c_ulong > limit { break 'checks true; }
            }
            if lupos_syscalls_dl_policy(policy) { break 'checks true; }
            if lupos_syscalls_task_has_idle_policy(p) && !lupos_syscalls_idle_policy(policy) &&
                !is_nice_reduction(p, lupos_syscalls_task_nice(p)) { break 'checks true; }
            if !check_same_owner(p) { break 'checks true; }
            if lupos_syscalls_reset_on_fork(p) && !reset_on_fork { break 'checks true; }
            false
        };
        if requires_privilege && !lupos_syscalls_capable(CAP_SYS_NICE) { -EPERM } else { 0 }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __sched_setscheduler(p: *mut task_struct, attr: *const sched_attr,
                                            user: bool, pi: bool) -> c_int {
    unsafe {
        let mut oldpolicy = -1;
        let mut policy = (*attr).sched_policy as c_int;
        let mut cpuset_locked = false;
        let mut queue_flags = (LUPOS_SYS_DEQUEUE_SAVE | LUPOS_SYS_DEQUEUE_MOVE |
                               LUPOS_SYS_DEQUEUE_NOCLOCK) as c_int;
        lupos_syscalls_bug_pi_interrupt(pi);
        'recheck: loop {
            let reset_on_fork;
            if policy < 0 {
                reset_on_fork = lupos_syscalls_reset_on_fork(p);
                policy = (*p).policy as c_int;
                oldpolicy = policy;
            } else {
                reset_on_fork = (*attr).sched_flags & SCHED_FLAG_RESET_ON_FORK as u64 != 0;
                if !lupos_syscalls_valid_policy(policy) { return -EINVAL; }
            }
            if (*attr).sched_flags & !((SCHED_FLAG_ALL | SCHED_FLAG_SUGOV) as u64) != 0 {
                return -EINVAL;
            }
            if (*attr).sched_priority > (MAX_RT_PRIO - 1) as c_uint { return -EINVAL; }
            if (lupos_syscalls_dl_policy(policy) && !__checkparam_dl(attr)) ||
                (lupos_syscalls_rt_policy(policy) != ((*attr).sched_priority != 0)) {
                return -EINVAL;
            }
            if user {
                let ret = user_check_sched_setscheduler(p, attr, policy, reset_on_fork);
                if ret != 0 { return ret; }
                if (*attr).sched_flags & SCHED_FLAG_SUGOV as u64 != 0 { return -EINVAL; }
                let ret = lupos_syscalls_security_setscheduler(p);
                if ret != 0 { return ret; }
            }
            if (*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP as u64 != 0 {
                let ret = uclamp_validate(p, attr);
                if ret != 0 { return ret; }
            }
            if lupos_syscalls_dl_policy(policy) || lupos_syscalls_dl_policy((*p).policy as c_int) {
                cpuset_locked = true;
                lupos_syscalls_cpuset_lock();
            }
            let mut rf = MaybeUninit::<rq_flags>::uninit();
            let rq = lupos_syscalls_task_rq_lock(p, rf.as_mut_ptr());
            update_rq_clock(rq);
            let retval = 'locked: {
                if p == (*rq).stop { break 'locked -EINVAL; }
                let ret = lupos_syscalls_scx_check_setscheduler(p, policy);
                if ret != 0 { break 'locked ret; }
                if lupos_syscalls_unlikely_same_policy(policy == (*p).policy as c_int) {
                    let change = (lupos_syscalls_fair_policy(policy) &&
                        ((*attr).sched_nice != lupos_syscalls_task_nice(p) ||
                         (*attr).sched_runtime != (*p).se.slice)) ||
                        (lupos_syscalls_rt_policy(policy) &&
                         (*attr).sched_priority != (*p).rt_priority) ||
                        (lupos_syscalls_dl_policy(policy) && dl_param_changed(p, attr)) ||
                        ((*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP as u64 != 0);
                    if !change {
                        lupos_syscalls_set_reset_on_fork(p, reset_on_fork);
                        break 'locked 0;
                    }
                }
                if user {
                    #[cfg(CONFIG_RT_GROUP_SCHED)]
                    if lupos_syscalls_rt_group_sched_enabled() &&
                        lupos_syscalls_rt_bandwidth_enabled() && lupos_syscalls_rt_policy(policy) {
                        let tg = lupos_syscalls_task_group(p);
                        if (*tg).rt_bandwidth.rt_runtime == 0 &&
                            !lupos_syscalls_task_group_is_autogroup(tg) { break 'locked -EPERM; }
                    }
                    if lupos_syscalls_dl_bandwidth_enabled() && lupos_syscalls_dl_policy(policy) &&
                        (*attr).sched_flags & SCHED_FLAG_SUGOV as u64 == 0 {
                        // cpumask_var_t may be an inline array: native accessor owns it.
                        let span = lupos_syscalls_root_span(rq);
                        if !lupos_syscalls_mask_subset(span, (*p).cpus_ptr) ||
                            (*(*rq).rd).dl_bw.bw == 0 { break 'locked -EPERM; }
                    }
                }
                if lupos_syscalls_unlikely_policy_changed(oldpolicy != -1 && oldpolicy != (*p).policy as c_int) {
                    policy = -1;
                    oldpolicy = -1;
                    lupos_syscalls_task_rq_unlock(rq, p, rf.as_mut_ptr());
                    if cpuset_locked { lupos_syscalls_cpuset_unlock(); }
                    continue 'recheck;
                }
                if (lupos_syscalls_dl_policy(policy) || lupos_syscalls_dl_task(p)) &&
                    sched_dl_overflow(p, policy, attr) != 0 { break 'locked -EBUSY; }
                lupos_syscalls_set_reset_on_fork(p, reset_on_fork);
                let oldprio = (*p).prio;
                let mut newprio = __normal_prio(policy, (*attr).sched_priority as c_int,
                                                (*attr).sched_nice);
                if pi {
                    newprio = lupos_syscalls_rt_effective_prio(p, newprio);
                    if newprio == oldprio && !lupos_syscalls_dl_prio(newprio) {
                        queue_flags &= !(LUPOS_SYS_DEQUEUE_MOVE as c_int);
                    }
                }
                let prev_class = (*p).sched_class;
                let next_class = __setscheduler_class(policy, newprio);
                if prev_class != next_class { queue_flags |= LUPOS_SYS_DEQUEUE_CLASS as c_int; }
                let scope = sched_change_begin(p, queue_flags as c_uint);
                if (*attr).sched_flags & SCHED_FLAG_KEEP_PARAMS as u64 == 0 {
                    __setscheduler_params(p, attr);
                    (*p).sched_class = next_class;
                    (*p).prio = newprio;
                    #[cfg(CONFIG_RT_MUTEXES)]
                    __setscheduler_dl_pi(newprio, policy, p, scope);
                }
                #[cfg(CONFIG_UCLAMP_TASK)]
                __setscheduler_uclamp(p, attr);
                if (*scope).queued && oldprio < (*p).prio {
                    (*scope).flags |= LUPOS_SYS_ENQUEUE_HEAD as c_int;
                }
                sched_change_end(scope);
                lupos_syscalls_preempt_disable();
                let head = splice_balance_callbacks(rq);
                lupos_syscalls_task_rq_unlock(rq, p, rf.as_mut_ptr());
                if pi {
                    if cpuset_locked { lupos_syscalls_cpuset_unlock(); }
                    lupos_syscalls_rt_mutex_adjust_pi(p);
                }
                balance_callbacks(rq, head);
                lupos_syscalls_preempt_enable();
                return 0;
            };
            lupos_syscalls_task_rq_unlock(rq, p, rf.as_mut_ptr());
            if cpuset_locked { lupos_syscalls_cpuset_unlock(); }
            return retval;
        }
    }
}

unsafe fn _sched_setscheduler(p: *mut task_struct, mut policy: c_int,
                             param: *const sched_param, check: bool) -> c_int {
    unsafe {
        let mut attr = MaybeUninit::<sched_attr>::zeroed().assume_init();
        attr.sched_policy = policy as c_uint;
        attr.sched_priority = (*param).sched_priority as c_uint;
        attr.sched_nice = lupos_syscalls_prio_to_nice((*p).static_prio);
        if (*p).se.custom_slice != 0 { attr.sched_runtime = (*p).se.slice; }
        if policy != SETPARAM_POLICY && policy & SCHED_RESET_ON_FORK as c_int != 0 {
            attr.sched_flags |= SCHED_FLAG_RESET_ON_FORK as u64;
            policy &= !(SCHED_RESET_ON_FORK as c_int);
            attr.sched_policy = policy as c_uint;
        }
        __sched_setscheduler(p, &attr, check, true)
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_setscheduler(p: *mut task_struct, policy: c_int,
                                           param: *const sched_param) -> c_int {
    unsafe { _sched_setscheduler(p, policy, param, true) }
}
#[no_mangle]
pub unsafe extern "C" fn sched_setattr(p: *mut task_struct, attr: *const sched_attr) -> c_int {
    unsafe { __sched_setscheduler(p, attr, true, true) }
}
#[no_mangle]
pub unsafe extern "C" fn sched_setattr_nocheck(p: *mut task_struct, attr: *const sched_attr) -> c_int {
    unsafe { __sched_setscheduler(p, attr, false, true) }
}
#[no_mangle]
pub unsafe extern "C" fn sched_setscheduler_nocheck(p: *mut task_struct, policy: c_int,
                                                   param: *const sched_param) -> c_int {
    unsafe { _sched_setscheduler(p, policy, param, false) }
}
#[no_mangle]
pub unsafe extern "C" fn sched_set_fifo(p: *mut task_struct) {
    unsafe {
        let param = sched_param { sched_priority: MAX_RT_PRIO / 2 };
        lupos_syscalls_warn_fifo(sched_setscheduler_nocheck(p, SCHED_FIFO as c_int, &param));
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_set_fifo_low(p: *mut task_struct) {
    unsafe {
        let param = sched_param { sched_priority: 1 };
        lupos_syscalls_warn_fifo_low(sched_setscheduler_nocheck(p, SCHED_FIFO as c_int, &param));
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_set_fifo_secondary(p: *mut task_struct) {
    unsafe {
        let param = sched_param { sched_priority: MAX_RT_PRIO / 2 - 1 };
        lupos_syscalls_warn_fifo_secondary(sched_setscheduler_nocheck(p, SCHED_FIFO as c_int, &param));
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_set_normal(p: *mut task_struct, nice: c_int) {
    unsafe {
        let mut attr = MaybeUninit::<sched_attr>::zeroed().assume_init();
        attr.sched_policy = SCHED_NORMAL;
        attr.sched_nice = nice;
        lupos_syscalls_warn_normal(sched_setattr_nocheck(p, &attr));
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_task_check_affinity(p: *mut task_struct, mask: *const cpumask) -> c_int {
    unsafe {
        if !lupos_syscalls_task_has_dl_policy(p) || !lupos_syscalls_dl_bandwidth_enabled() {
            return 0;
        }
        if lupos_syscalls_dl_entity_is_special(addr_of_mut!((*p).dl)) { return 0; }
        let _rcu = Rcu::lock();
        if !lupos_syscalls_mask_subset(lupos_syscalls_root_span(lupos_syscalls_task_rq(p)), mask) {
            return -EBUSY;
        }
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn __sched_setaffinity(p: *mut task_struct,
                                           ctx: *mut affinity_context) -> c_int {
    unsafe {
        // Keep native cpumask_var_t storage in place: !OFFSTACK uses an array.
        let mut allowed_storage = MaybeUninit::<cpumask_var_t>::uninit();
        let mut mask_storage = MaybeUninit::<cpumask_var_t>::uninit();
        if !lupos_syscalls_alloc_mask(allowed_storage.as_mut_ptr(), false) { return -ENOMEM; }
        if !lupos_syscalls_alloc_mask(mask_storage.as_mut_ptr(), false) {
            lupos_syscalls_free_mask(allowed_storage.as_mut_ptr());
            return -ENOMEM;
        }
        let allowed = lupos_syscalls_mask_ptr(allowed_storage.as_mut_ptr());
        let new_mask = lupos_syscalls_mask_ptr(mask_storage.as_mut_ptr());
        lupos_syscalls_cpuset_cpus_allowed(p, allowed);
        lupos_syscalls_mask_and(new_mask, (*ctx).new_mask, allowed);
        (*ctx).new_mask = new_mask;
        (*ctx).flags |= LUPOS_SYS_SCA_CHECK as c_uint;
        let retval = 'apply: {
            let ret = dl_task_check_affinity(p, new_mask);
            if ret != 0 { break 'apply ret; }
            let ret = __set_cpus_allowed_ptr(p, ctx);
            if ret != 0 { break 'apply ret; }
            lupos_syscalls_cpuset_cpus_allowed(p, allowed);
            if !lupos_syscalls_mask_subset(new_mask, allowed) {
                lupos_syscalls_mask_copy(new_mask, allowed);
                if lupos_syscalls_unlikely_old_user_mask((*ctx).flags & LUPOS_SYS_SCA_USER as c_uint != 0 && !(*ctx).user_mask.is_null()) {
                    let empty = !lupos_syscalls_mask_and(new_mask, new_mask, (*ctx).user_mask);
                    if empty { lupos_syscalls_mask_copy(new_mask, allowed); }
                }
                // The second call restores the previous user mask; its return
                // is intentionally discarded in the original race-recovery path.
                __set_cpus_allowed_ptr(p, ctx);
                break 'apply -EINVAL;
            }
            ret
        };
        lupos_syscalls_free_mask(mask_storage.as_mut_ptr());
        lupos_syscalls_free_mask(allowed_storage.as_mut_ptr());
        retval
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_setaffinity(pid: pid_t, mask: *const cpumask) -> c_long {
    unsafe {
        let task = find_get_task(pid);
        let p = task.0;
        if p.is_null() { return -ESRCH as c_long; }
        if (*p).flags & LUPOS_SYS_PF_NO_SETAFFINITY as c_uint != 0 { return -EINVAL as c_long; }
        if !check_same_owner(p) {
            let _rcu = Rcu::lock();
            if !lupos_syscalls_ns_capable((*lupos_syscalls_task_cred(p)).user_ns, CAP_SYS_NICE) {
                return -EPERM as c_long;
            }
        }
        let ret = lupos_syscalls_security_setscheduler(p);
        if ret != 0 { return ret as c_long; }
        let user_mask = lupos_syscalls_alloc_user_mask();
        if user_mask.is_null() { return -ENOMEM as c_long; }
        lupos_syscalls_mask_copy(user_mask, mask);
        let mut ac = affinity_context {
            new_mask: mask, user_mask, flags: LUPOS_SYS_SCA_USER as c_uint,
        };
        let ret = __sched_setaffinity(p, &mut ac);
        kfree(ac.user_mask.cast());
        ret as c_long
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_getaffinity(pid: pid_t, mask: *mut cpumask) -> c_long {
    unsafe {
        let _rcu = Rcu::lock();
        let p = find_process_by_pid(pid);
        if p.is_null() { return -ESRCH as c_long; }
        let ret = lupos_syscalls_security_getscheduler(p);
        if ret != 0 { return ret as c_long; }
        lupos_syscalls_pi_lock(p);
        lupos_syscalls_mask_and(mask, addr_of!((*p).cpus_mask), lupos_syscalls_active_mask());
        lupos_syscalls_pi_unlock(p);
        0
    }
}

unsafe fn do_sched_setscheduler(pid: pid_t, policy: c_int, param: *mut sched_param) -> c_int {
    unsafe {
        if lupos_syscalls_unlikely_setparam_args(param.is_null() || pid < 0) { return -EINVAL; }
        let mut lparam = MaybeUninit::<sched_param>::uninit();
        if lupos_syscalls_copy_param_from_user(lparam.as_mut_ptr(), param) != 0 { return -EFAULT; }
        let task = find_get_task(pid);
        if task.0.is_null() { return -ESRCH; }
        sched_setscheduler(task.0, policy, lparam.as_ptr())
    }
}
unsafe fn sched_copy_attr(uattr: *mut sched_attr, attr: *mut sched_attr) -> c_int {
    unsafe {
        // Match memset over the entire UAPI object before the versioned copy.
        core::ptr::write_bytes(attr.cast::<u8>(), 0, size_of::<sched_attr>());
        let mut size = 0u32;
        let ret = lupos_syscalls_get_attr_size(uattr, &mut size);
        if ret != 0 { return ret; }
        if size == 0 { size = SCHED_ATTR_SIZE_VER0; }
        let ret = 'copy: {
            if size < SCHED_ATTR_SIZE_VER0 || size as c_ulong > lupos_syscalls_page_size() {
                break 'copy -E2BIG;
            }
            let ret = lupos_syscalls_copy_attr_from_user(attr, uattr, size);
            if ret != 0 { break 'copy ret; }
            if (*attr).sched_flags & SCHED_FLAG_UTIL_CLAMP as u64 != 0 && size < SCHED_ATTR_SIZE_VER1 {
                return -EINVAL;
            }
            (*attr).sched_nice = core::cmp::min(core::cmp::max((*attr).sched_nice, MIN_NICE), MAX_NICE);
            return 0;
        };
        if ret == -E2BIG {
            // The original ABI ignores a failed size writeback on this path.
            lupos_syscalls_put_attr_size(uattr, size_of::<sched_attr>() as u32);
        }
        ret
    }
}
unsafe fn get_params(p: *mut task_struct, attr: *mut sched_attr, flags: c_uint) {
    unsafe {
        if lupos_syscalls_task_has_dl_policy(p) { __getparam_dl(p, attr, flags); }
        else if lupos_syscalls_task_has_rt_policy(p) { (*attr).sched_priority = (*p).rt_priority; }
        else {
            (*attr).sched_nice = lupos_syscalls_task_nice(p);
            (*attr).sched_runtime = (*p).se.slice;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_setscheduler(pid: pid_t, policy: c_int,
                                                              param: *mut sched_param) -> c_long {
    unsafe {
        if policy < 0 { return -EINVAL as c_long; }
        do_sched_setscheduler(pid, policy, param) as c_long
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_setparam(pid: pid_t,
                                                          param: *mut sched_param) -> c_long {
    unsafe { do_sched_setscheduler(pid, SETPARAM_POLICY, param) as c_long }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_setattr(pid: pid_t, uattr: *mut sched_attr,
                                                         flags: c_uint) -> c_long {
    unsafe {
        if lupos_syscalls_unlikely_setattr_args(uattr.is_null() || pid < 0 || flags != 0) { return -EINVAL as c_long; }
        let mut attr = MaybeUninit::<sched_attr>::uninit();
        let ret = sched_copy_attr(uattr, attr.as_mut_ptr());
        if ret != 0 { return ret as c_long; }
        let mut attr = attr.assume_init();
        if (attr.sched_policy as c_int) < 0 { return -EINVAL as c_long; }
        if attr.sched_flags & SCHED_FLAG_KEEP_POLICY as u64 != 0 {
            attr.sched_policy = SETPARAM_POLICY as c_uint;
        }
        let task = find_get_task(pid);
        if task.0.is_null() { return -ESRCH as c_long; }
        if attr.sched_flags & SCHED_FLAG_KEEP_PARAMS as u64 != 0 {
            get_params(task.0, &mut attr, 0);
        }
        sched_setattr(task.0, &attr) as c_long
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_getscheduler(pid: pid_t) -> c_long {
    unsafe {
        if pid < 0 { return -EINVAL as c_long; }
        let _rcu = Rcu::lock();
        let p = find_process_by_pid(pid);
        if p.is_null() { return -ESRCH as c_long; }
        let mut ret = lupos_syscalls_security_getscheduler(p);
        if ret == 0 {
            ret = (*p).policy as c_int;
            if lupos_syscalls_reset_on_fork(p) { ret |= SCHED_RESET_ON_FORK as c_int; }
        }
        ret as c_long
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_getparam(pid: pid_t,
                                                          param: *mut sched_param) -> c_long {
    unsafe {
        if lupos_syscalls_unlikely_getparam_args(param.is_null() || pid < 0) { return -EINVAL as c_long; }
        let mut lp = sched_param { sched_priority: 0 };
        {
            let _rcu = Rcu::lock();
            let p = find_process_by_pid(pid);
            if p.is_null() { return -ESRCH as c_long; }
            let ret = lupos_syscalls_security_getscheduler(p);
            if ret != 0 { return ret as c_long; }
            if lupos_syscalls_task_has_rt_policy(p) { lp.sched_priority = (*p).rt_priority as c_int; }
        }
        if lupos_syscalls_copy_param_to_user(param, &lp) != 0 { -EFAULT as c_long } else { 0 }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_getattr(pid: pid_t, uattr: *mut sched_attr,
                                                         usize: c_uint, flags: c_uint) -> c_long {
    unsafe {
        if lupos_syscalls_unlikely_getattr_args(uattr.is_null() || pid < 0 ||
            usize as c_ulong > lupos_syscalls_page_size() || usize < SCHED_ATTR_SIZE_VER0) {
            return -EINVAL as c_long;
        }
        let mut kattr = MaybeUninit::<sched_attr>::zeroed().assume_init();
        {
            let _rcu = Rcu::lock();
            let p = find_process_by_pid(pid);
            if p.is_null() { return -ESRCH as c_long; }
            if flags != 0 && (!lupos_syscalls_task_has_dl_policy(p) ||
                              flags != SCHED_GETATTR_FLAG_DL_DYNAMIC) { return -EINVAL as c_long; }
            let ret = lupos_syscalls_security_getscheduler(p);
            if ret != 0 { return ret as c_long; }
            kattr.sched_policy = (*p).policy;
            if lupos_syscalls_reset_on_fork(p) { kattr.sched_flags |= SCHED_FLAG_RESET_ON_FORK as u64; }
            get_params(p, &mut kattr, flags);
            kattr.sched_flags &= SCHED_FLAG_ALL as u64;
            #[cfg(CONFIG_UCLAMP_TASK)] {
                let req = addr_of_mut!((*p).uclamp_req).cast::<uclamp_se>();
                kattr.sched_util_min = lupos_syscalls_uclamp_value(req.add(UCLAMP_MIN as usize));
                kattr.sched_util_max = lupos_syscalls_uclamp_value(req.add(UCLAMP_MAX as usize));
            }
        }
        kattr.size = core::cmp::min(usize, size_of::<sched_attr>() as c_uint);
        lupos_syscalls_copy_attr_to_user(uattr, usize, &kattr) as c_long
    }
}
unsafe fn get_user_cpu_mask(user_mask: *mut c_ulong, mut len: c_uint,
                            mask: *mut cpumask) -> c_int {
    unsafe {
        let size = lupos_syscalls_mask_size();
        if len < size { lupos_syscalls_mask_clear(mask); }
        else if len > size { len = size; }
        if lupos_syscalls_copy_mask_from_user(mask, user_mask, len) != 0 { -EFAULT } else { 0 }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_setaffinity(pid: pid_t, len: c_uint,
                                                             user_mask: *mut c_ulong) -> c_long {
    unsafe {
        let mut storage = MaybeUninit::<cpumask_var_t>::uninit();
        if !lupos_syscalls_alloc_mask(storage.as_mut_ptr(), false) { return -ENOMEM as c_long; }
        let mask = lupos_syscalls_mask_ptr(storage.as_mut_ptr());
        let mut ret = get_user_cpu_mask(user_mask, len, mask) as c_long;
        if ret == 0 { ret = sched_setaffinity(pid, mask); }
        lupos_syscalls_free_mask(storage.as_mut_ptr());
        ret
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_getaffinity(pid: pid_t, len: c_uint,
                                                             user_mask: *mut c_ulong) -> c_long {
    unsafe {
        if len.wrapping_mul(LUPOS_SYS_BITS_PER_BYTE as c_uint) < lupos_syscalls_nr_cpu_ids() ||
            len as usize & (size_of::<c_ulong>() - 1) != 0 { return -EINVAL as c_long; }
        let mut storage = MaybeUninit::<cpumask_var_t>::uninit();
        if !lupos_syscalls_alloc_mask(storage.as_mut_ptr(), true) { return -ENOMEM as c_long; }
        let mask = lupos_syscalls_mask_ptr(storage.as_mut_ptr());
        let mut ret = sched_getaffinity(pid, mask) as c_int;
        if ret == 0 {
            let retlen = core::cmp::min(len, lupos_syscalls_mask_size());
            if lupos_syscalls_copy_mask_to_user(user_mask, mask, retlen) != 0 { ret = -EFAULT; }
            else { ret = retlen as c_int; }
        }
        lupos_syscalls_free_mask(storage.as_mut_ptr());
        ret as c_long
    }
}

unsafe fn do_sched_yield() {
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = lupos_syscalls_this_rq_lock_irq(rf.as_mut_ptr());
        lupos_syscalls_yield_stat(rq);
        // yield_task is mandatory in a runnable scheduler class, as in C.
        ((*(*lupos_syscalls_rq_donor(rq)).sched_class).yield_task.unwrap_unchecked())(rq);
        lupos_syscalls_preempt_disable();
        lupos_syscalls_rq_unlock_irq(rq, rf.as_mut_ptr());
        lupos_syscalls_preempt_enable_no_resched();
        lupos_syscalls_schedule();
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_yield() -> c_long {
    unsafe { do_sched_yield(); }
    0
}
#[export_name = "yield"]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn yield_() {
    unsafe {
        lupos_syscalls_set_running();
        do_sched_yield();
    }
}
#[no_mangle]
#[link_section = ".sched.text"]
pub unsafe extern "C" fn yield_to(p: *mut task_struct, preempt: bool) -> c_int {
    unsafe {
        lupos_syscalls_pi_lock(p);
        let rq = lupos_syscalls_this_rq();
        let curr = lupos_syscalls_rq_donor(rq);
        let yielded = loop {
            let p_rq = lupos_syscalls_task_rq(p);
            if (*rq).nr_running == 1 && (*p_rq).nr_running == 1 { break -ESRCH; }
            lupos_syscalls_double_rq_lock(rq, p_rq);
            if lupos_syscalls_task_rq(p) != p_rq {
                lupos_syscalls_double_rq_unlock(rq, p_rq);
                continue;
            }
            let result = 'locked: {
                let Some(callback) = (*(*curr).sched_class).yield_to_task else { break 'locked 0; };
                if (*curr).sched_class != (*p).sched_class { break 'locked 0; }
                if lupos_syscalls_task_on_cpu(p_rq, p) || !lupos_syscalls_task_is_running(p) {
                    break 'locked 0;
                }
                let yielded = callback(rq, p);
                if yielded {
                    lupos_syscalls_yield_stat(rq);
                    if preempt && rq != p_rq { resched_curr(p_rq); }
                }
                yielded as c_int
            };
            lupos_syscalls_double_rq_unlock(rq, p_rq);
            break result;
        };
        lupos_syscalls_pi_unlock(p);
        // -ESRCH was an early guard return in the C source and never schedules.
        if yielded > 0 { lupos_syscalls_schedule(); }
        yielded
    }
}
#[no_mangle]
pub extern "C" fn lupos_syscalls_sys_sched_get_priority_max(policy: c_int) -> c_long {
    if policy == SCHED_FIFO as c_int || policy == SCHED_RR as c_int {
        (MAX_RT_PRIO - 1) as c_long
    } else if policy == SCHED_DEADLINE as c_int || policy == SCHED_NORMAL as c_int ||
              policy == SCHED_BATCH as c_int || policy == SCHED_IDLE as c_int ||
              policy == SCHED_EXT as c_int { 0 }
    else { -EINVAL as c_long }
}
#[no_mangle]
pub extern "C" fn lupos_syscalls_sys_sched_get_priority_min(policy: c_int) -> c_long {
    if policy == SCHED_FIFO as c_int || policy == SCHED_RR as c_int { 1 }
    else if policy == SCHED_DEADLINE as c_int || policy == SCHED_NORMAL as c_int ||
              policy == SCHED_BATCH as c_int || policy == SCHED_IDLE as c_int ||
              policy == SCHED_EXT as c_int { 0 }
    else { -EINVAL as c_long }
}
unsafe fn sched_rr_get_interval(pid: pid_t, t: *mut timespec64) -> c_int {
    unsafe {
        if pid < 0 { return -EINVAL; }
        let mut time_slice = 0;
        {
            let _rcu = Rcu::lock();
            let p = find_process_by_pid(pid);
            if p.is_null() { return -ESRCH; }
            let ret = lupos_syscalls_security_getscheduler(p);
            if ret != 0 { return ret; }
            let mut rf = MaybeUninit::<rq_flags>::uninit();
            let rq = lupos_syscalls_task_rq_lock(p, rf.as_mut_ptr());
            if let Some(callback) = (*(*p).sched_class).get_rr_interval { time_slice = callback(rq, p); }
            lupos_syscalls_task_rq_unlock(rq, p, rf.as_mut_ptr());
        }
        lupos_syscalls_jiffies_to_timespec64(time_slice, t);
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_rr_get_interval(pid: pid_t,
                                                                 interval: *mut __kernel_timespec) -> c_long {
    unsafe {
        let mut t = MaybeUninit::<timespec64>::uninit();
        let mut ret = sched_rr_get_interval(pid, t.as_mut_ptr());
        if ret == 0 { ret = put_timespec64(t.as_ptr(), interval); }
        ret as c_long
    }
}
#[cfg(CONFIG_COMPAT_32BIT_TIME)]
#[no_mangle]
pub unsafe extern "C" fn lupos_syscalls_sys_sched_rr_get_interval_time32(pid: pid_t,
                                                                       interval: *mut old_timespec32) -> c_long {
    unsafe {
        let mut t = MaybeUninit::<timespec64>::uninit();
        let mut ret = sched_rr_get_interval(pid, t.as_mut_ptr());
        if ret == 0 { ret = put_old_timespec32(t.as_ptr(), interval.cast()); }
        ret as c_long
    }
}
