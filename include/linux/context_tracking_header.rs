/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the corresponding scheduler, vtime, context
// tracking state, instrumentation, and architecture-specific dependencies.

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
extern "C" {
    pub fn ct_cpu_track_user(cpu: ::core::ffi::c_int);

    // Called with interrupts disabled.
    pub fn __ct_user_enter(state: ctx_state);
    pub fn __ct_user_exit(state: ctx_state);

    pub fn ct_user_enter(state: ctx_state);
    pub fn ct_user_exit(state: ctx_state);

    pub fn user_enter_callable();
    pub fn user_exit_callable();
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
#[inline]
pub unsafe fn user_enter() {
    if context_tracking_enabled() {
        ct_user_enter(CT_STATE_USER);
    }
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
#[inline]
pub unsafe fn user_exit() {
    if context_tracking_enabled() {
        ct_user_exit(CT_STATE_USER);
    }
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
#[inline(always)]
pub unsafe fn user_enter_irqoff() {
    if context_tracking_enabled() {
        __ct_user_enter(CT_STATE_USER);
    }
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
#[inline(always)]
pub unsafe fn user_exit_irqoff() {
    if context_tracking_enabled() {
        __ct_user_exit(CT_STATE_USER);
    }
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
#[inline]
pub unsafe fn exception_enter() -> ctx_state {
    if cfg!(CONFIG_HAVE_CONTEXT_TRACKING_USER_OFFSTACK) || !context_tracking_enabled() {
        return 0;
    }

    let prev_ctx = __ct_state();
    if prev_ctx != CT_STATE_KERNEL {
        ct_user_exit(prev_ctx);
    }
    prev_ctx
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
#[inline]
pub unsafe fn exception_exit(prev_ctx: ctx_state) {
    if !cfg!(CONFIG_HAVE_CONTEXT_TRACKING_USER_OFFSTACK) && context_tracking_enabled()
        && prev_ctx != CT_STATE_KERNEL
    {
        ct_user_enter(prev_ctx);
    }
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
#[inline(always)]
pub unsafe fn context_tracking_guest_enter() -> bool {
    if context_tracking_enabled() {
        __ct_user_enter(CT_STATE_GUEST);
    }
    context_tracking_enabled_this_cpu()
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
#[inline(always)]
pub unsafe fn context_tracking_guest_exit() -> bool {
    if context_tracking_enabled() {
        __ct_user_exit(CT_STATE_GUEST);
    }
    context_tracking_enabled_this_cpu()
}

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
macro_rules! CT_WARN_ON {
    ($cond:expr) => {
        WARN_ON(context_tracking_enabled() && ($cond))
    };
}

#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline]
pub fn user_enter() {}
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline]
pub fn user_exit() {}
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline]
pub fn user_enter_irqoff() {}
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline]
pub fn user_exit_irqoff() {}
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline]
pub fn exception_enter() -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline]
pub fn exception_exit(_prev_ctx: ctx_state) {}
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline]
pub fn ct_state() -> ::core::ffi::c_int { -1 }
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline]
pub fn __ct_state() -> ::core::ffi::c_int { -1 }
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline(always)]
pub fn context_tracking_guest_enter() -> bool { false }
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
#[inline(always)]
pub fn context_tracking_guest_exit() -> bool { false }
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
macro_rules! CT_WARN_ON { ($cond:expr) => {{ let _ = $cond; }}; }

#[cfg(CONFIG_CONTEXT_TRACKING_USER_FORCE)]
extern "C" { pub fn context_tracking_init(); }
#[cfg(not(CONFIG_CONTEXT_TRACKING_USER_FORCE))]
#[inline]
pub fn context_tracking_init() {}

#[cfg(CONFIG_CONTEXT_TRACKING_IDLE)]
extern "C" {
    pub fn ct_idle_enter();
    pub fn ct_idle_exit();
}

#[cfg(CONFIG_CONTEXT_TRACKING_IDLE)]
#[inline(always)]
pub unsafe fn rcu_is_watching_curr_cpu() -> bool {
    raw_atomic_read(this_cpu_ptr(&context_tracking.state)) & CT_RCU_WATCHING != 0
}

#[cfg(CONFIG_CONTEXT_TRACKING_IDLE)]
#[inline(always)]
pub unsafe fn ct_state_inc(incby: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
    raw_atomic_add_return(incby, this_cpu_ptr(&context_tracking.state))
}

#[cfg(CONFIG_CONTEXT_TRACKING_IDLE)]
#[inline(always)]
pub unsafe fn warn_rcu_enter() -> bool {
    let mut ret = false;
    preempt_disable_notrace();
    if !rcu_is_watching_curr_cpu() {
        ret = true;
        ct_state_inc(CT_RCU_WATCHING);
    }
    ret
}

#[cfg(CONFIG_CONTEXT_TRACKING_IDLE)]
#[inline(always)]
pub unsafe fn warn_rcu_exit(rcu: bool) {
    if rcu {
        ct_state_inc(CT_RCU_WATCHING);
    }
    preempt_enable_notrace();
}

#[cfg(not(CONFIG_CONTEXT_TRACKING_IDLE))]
#[inline]
pub fn ct_idle_enter() {}
#[cfg(not(CONFIG_CONTEXT_TRACKING_IDLE))]
#[inline]
pub fn ct_idle_exit() {}
#[cfg(not(CONFIG_CONTEXT_TRACKING_IDLE))]
#[inline(always)]
pub fn warn_rcu_enter() -> bool { false }
#[cfg(not(CONFIG_CONTEXT_TRACKING_IDLE))]
#[inline(always)]
pub fn warn_rcu_exit(_rcu: bool) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
