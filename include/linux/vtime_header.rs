/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/context_tracking_state.h, linux/sched.h

/* Common vtime APIs. */
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING)]
unsafe extern "C" {
    pub fn vtime_account_kernel(tsk: *mut task_struct);
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)]
unsafe extern "C" {
    pub fn vtime_user_enter(tsk: *mut task_struct);
    pub fn vtime_user_exit(tsk: *mut task_struct);
    pub fn vtime_guest_enter(tsk: *mut task_struct);
    pub fn vtime_guest_exit(tsk: *mut task_struct);
    pub fn vtime_init_idle(tsk: *mut task_struct, cpu: core::ffi::c_int);
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_user_enter(_tsk: *mut task_struct) {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_user_exit(_tsk: *mut task_struct) {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_guest_enter(_tsk: *mut task_struct) {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_guest_exit(_tsk: *mut task_struct) {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_init_idle(_tsk: *mut task_struct, _cpu: core::ffi::c_int) {}

#[inline]
pub unsafe fn vtime_generic_enabled_cpu(cpu: core::ffi::c_int) -> bool {
    context_tracking_enabled_cpu(cpu)
}

#[inline]
pub unsafe fn vtime_generic_enabled_this_cpu() -> bool {
    context_tracking_enabled_this_cpu()
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
unsafe extern "C" {
    pub fn vtime_account_idle(tsk: *mut task_struct);
    pub fn vtime_account_irq(tsk: *mut task_struct, offset: core::ffi::c_uint);
    pub fn vtime_account_softirq(tsk: *mut task_struct);
    pub fn vtime_account_hardirq(tsk: *mut task_struct);
    pub fn vtime_flush(tsk: *mut task_struct);
}

#[cfg(all(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE))]
#[inline]
pub unsafe fn vtime_reset() {}
#[cfg(all(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE))]
#[inline]
pub unsafe fn vtime_dyntick_start() {}
#[cfg(all(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE))]
#[inline]
pub unsafe fn vtime_dyntick_stop() {}

#[cfg(all(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, not(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)))]
unsafe extern "C" {
    pub fn vtime_reset();
    pub fn vtime_dyntick_start();
    pub fn vtime_dyntick_stop();
}

#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
#[inline]
pub unsafe fn vtime_account_irq(_tsk: *mut task_struct, _offset: core::ffi::c_uint) {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
#[inline]
pub unsafe fn vtime_account_softirq(_tsk: *mut task_struct) {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
#[inline]
pub unsafe fn vtime_account_hardirq(_tsk: *mut task_struct) {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
#[inline]
pub unsafe fn vtime_flush(_tsk: *mut task_struct) {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
#[inline]
pub unsafe fn vtime_reset() {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
#[inline]
pub unsafe fn vtime_dyntick_start() {}
#[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
#[inline]
pub unsafe fn vtime_dyntick_stop() {}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
#[inline]
pub unsafe fn vtime_accounting_enabled_this_cpu() -> bool { true }

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
unsafe extern "C" {
    pub fn vtime_task_switch(prev: *mut task_struct);
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
#[inline(always)]
pub unsafe fn vtime_account_guest_enter() {
    vtime_account_kernel(current);
    (*current).flags |= PF_VCPU;
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
#[inline(always)]
pub unsafe fn vtime_account_guest_exit() {
    vtime_account_kernel(current);
    (*current).flags &= !PF_VCPU;
}

#[cfg(all(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE), CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_accounting_enabled() -> bool { context_tracking_enabled() }

#[cfg(all(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE), CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_accounting_enabled_cpu(cpu: core::ffi::c_int) -> bool {
    vtime_generic_enabled_cpu(cpu)
}

#[cfg(all(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE), CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_accounting_enabled_this_cpu() -> bool {
    vtime_generic_enabled_this_cpu()
}

#[cfg(all(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE), CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
unsafe extern "C" {
    pub fn vtime_task_switch_generic(prev: *mut task_struct);
}

#[cfg(all(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE), CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline]
pub unsafe fn vtime_task_switch(prev: *mut task_struct) {
    if vtime_accounting_enabled_this_cpu() {
        vtime_task_switch_generic(prev);
    }
}

#[cfg(all(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE), CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline(always)]
pub unsafe fn vtime_account_guest_enter() {
    if vtime_accounting_enabled_this_cpu() {
        vtime_guest_enter(current);
    } else {
        (*current).flags |= PF_VCPU;
    }
}

#[cfg(all(not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE), CONFIG_VIRT_CPU_ACCOUNTING_GEN))]
#[inline(always)]
pub unsafe fn vtime_account_guest_exit() {
    if vtime_accounting_enabled_this_cpu() {
        vtime_guest_exit(current);
    } else {
        (*current).flags &= !PF_VCPU;
    }
}

#[cfg(not(any(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, CONFIG_VIRT_CPU_ACCOUNTING_GEN)))]
#[inline]
pub unsafe fn vtime_accounting_enabled_this_cpu() -> bool { false }

#[cfg(not(any(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, CONFIG_VIRT_CPU_ACCOUNTING_GEN)))]
#[inline]
pub unsafe fn vtime_task_switch(_prev: *mut task_struct) {}

#[cfg(not(any(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, CONFIG_VIRT_CPU_ACCOUNTING_GEN)))]
#[inline(always)]
pub unsafe fn vtime_account_guest_enter() {
    (*current).flags |= PF_VCPU;
}

#[cfg(not(any(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, CONFIG_VIRT_CPU_ACCOUNTING_GEN)))]
#[inline(always)]
pub unsafe fn vtime_account_guest_exit() {
    (*current).flags &= !PF_VCPU;
}

#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
unsafe extern "C" {
    pub fn irqtime_account_irq(tsk: *mut task_struct, offset: core::ffi::c_uint);
}

#[cfg(not(CONFIG_IRQ_TIME_ACCOUNTING))]
#[inline]
pub unsafe fn irqtime_account_irq(_tsk: *mut task_struct, _offset: core::ffi::c_uint) {}

#[inline]
pub unsafe fn account_softirq_enter(tsk: *mut task_struct) {
    vtime_account_irq(tsk, SOFTIRQ_OFFSET);
    irqtime_account_irq(tsk, SOFTIRQ_OFFSET);
}

#[inline]
pub unsafe fn account_softirq_exit(tsk: *mut task_struct) {
    vtime_account_softirq(tsk);
    irqtime_account_irq(tsk, 0);
}

#[inline]
pub unsafe fn account_hardirq_enter(tsk: *mut task_struct) {
    vtime_account_irq(tsk, HARDIRQ_OFFSET);
    irqtime_account_irq(tsk, HARDIRQ_OFFSET);
}

#[inline]
pub unsafe fn account_hardirq_exit(tsk: *mut task_struct) {
    vtime_account_hardirq(tsk);
    irqtime_account_irq(tsk, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
