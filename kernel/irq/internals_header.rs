/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from internals.h; included dependencies and configuration are supplied elsewhere. */

#[cfg(feature = "CONFIG_SPARSE_IRQ")]
pub const MAX_SPARSE_IRQS: i32 = i32::MAX;
#[cfg(not(feature = "CONFIG_SPARSE_IRQ"))]
pub const MAX_SPARSE_IRQS: u32 = NR_IRQS;

pub const IRQ_RESEND: bool = true;
pub const IRQ_NORESEND: bool = false;
pub const IRQ_START_FORCE: bool = true;
pub const IRQ_START_COND: bool = false;

pub const IRQTF_RUNTHREAD: u32 = 0;
pub const IRQTF_WARNED: u32 = 1;
pub const IRQTF_AFFINITY: u32 = 2;
pub const IRQTF_FORCED_THREAD: u32 = 3;
pub const IRQTF_READY: u32 = 4;

pub const IRQS_AUTODETECT: u32 = 0x00000001;
pub const IRQS_SPURIOUS_DISABLED: u32 = 0x00000002;
pub const IRQS_POLL_INPROGRESS: u32 = 0x00000008;
pub const IRQS_ONESHOT: u32 = 0x00000020;
pub const IRQS_REPLAY: u32 = 0x00000040;
pub const IRQS_WAITING: u32 = 0x00000080;
pub const IRQS_PENDING: u32 = 0x00000200;
pub const IRQS_SUSPENDED: u32 = 0x00000800;
pub const IRQS_TIMINGS: u32 = 0x00001000;
pub const IRQS_NMI: u32 = 0x00002000;
pub const IRQS_SYSFS: u32 = 0x00004000;

pub const _IRQ_DESC_CHECK: u32 = 1 << 0;
pub const _IRQ_DESC_PERCPU: u32 = 1 << 1;
pub const IRQ_GET_DESC_CHECK_GLOBAL: u32 = _IRQ_DESC_CHECK;
pub const IRQ_GET_DESC_CHECK_PERCPU: u32 = _IRQ_DESC_CHECK | _IRQ_DESC_PERCPU;

extern "C" {
    pub static mut noirqdebug: bool;
    pub static mut irq_poll_cpu: i32;
    pub static mut total_nr_irqs: u32;
    pub static mut chained_action: irqaction;

    pub fn __irq_set_trigger(desc: *mut irq_desc, flags: u64) -> i32;
    pub fn __disable_irq(desc: *mut irq_desc);
    pub fn __enable_irq(desc: *mut irq_desc);
    pub fn irq_activate(desc: *mut irq_desc) -> i32;
    pub fn irq_activate_and_startup(desc: *mut irq_desc, resend: bool) -> i32;
    pub fn irq_startup(desc: *mut irq_desc, resend: bool, force: bool) -> i32;
    pub fn irq_startup_managed(desc: *mut irq_desc);
    pub fn irq_shutdown(desc: *mut irq_desc);
    pub fn irq_shutdown_and_deactivate(desc: *mut irq_desc);
    pub fn irq_disable(desc: *mut irq_desc);
    pub fn irq_percpu_enable(desc: *mut irq_desc, cpu: u32);
    pub fn irq_percpu_disable(desc: *mut irq_desc, cpu: u32);
    pub fn mask_irq(desc: *mut irq_desc);
    pub fn unmask_irq(desc: *mut irq_desc);
    pub fn unmask_threaded_irq(desc: *mut irq_desc);
    pub fn irq_desc_free_rcu(desc: *mut irq_desc);
    pub fn irq_mark_irq(irq: u32);
    pub fn __handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t;
    pub fn handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t;
    pub fn handle_irq_event(desc: *mut irq_desc) -> irqreturn_t;
    pub fn check_irq_resend(desc: *mut irq_desc, inject: bool) -> i32;
    pub fn clear_irq_resend(desc: *mut irq_desc);
    pub fn irq_resend_init(desc: *mut irq_desc);
    pub fn __irq_wake_thread(desc: *mut irq_desc, action: *mut irqaction);
    pub fn wake_threads_waitq(desc: *mut irq_desc);
    pub fn irq_find_desc_at_or_after(offset: u32) -> *mut irq_desc;
    pub fn irq_can_set_affinity_usr(irq: u32) -> bool;
    pub fn irq_do_set_affinity(data: *mut irq_data, dest: *const cpumask, force: bool) -> i32;
    pub fn irq_affinity_schedule_notify_work(desc: *mut irq_desc);
    pub fn __irq_get_desc_lock(irq: u32, flags: *mut u64, bus: bool, check: u32) -> *mut irq_desc;
    pub fn __irq_put_desc_unlock(desc: *mut irq_desc, flags: u64, bus: bool);
}

#[cfg(feature = "CONFIG_SPARSE_IRQ")]
#[inline(always)] pub unsafe fn irq_mark_irq_noop(_irq: u32) {}
#[cfg(feature = "CONFIG_SPARSE_IRQ")]
#[inline(always)] pub unsafe fn irq_desc_get_ref(_desc: *mut irq_desc) -> bool { true }
#[cfg(feature = "CONFIG_SPARSE_IRQ")]
#[inline(always)] pub unsafe fn irq_desc_put_ref(_desc: *mut irq_desc) {}
#[cfg(not(feature = "CONFIG_SPARSE_IRQ"))]
#[inline(always)] pub unsafe fn irq_desc_get_ref(_desc: *mut irq_desc) -> bool { true }
#[cfg(not(feature = "CONFIG_SPARSE_IRQ"))]
#[inline(always)] pub unsafe fn irq_desc_put_ref(_desc: *mut irq_desc) {}

#[inline] pub unsafe fn irq_desc_is_chained(desc: *mut irq_desc) -> bool { !(*desc).action.is_null() && (*desc).action == &mut chained_action }
#[inline] pub unsafe fn irq_is_nmi(desc: *mut irq_desc) -> bool { (*desc).istate & IRQS_NMI != 0 }

#[cfg(feature = "CONFIG_SMP")]
extern "C" { pub fn irq_setup_affinity(desc: *mut irq_desc) -> i32; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn irq_setup_affinity(_desc: *mut irq_desc) -> i32 { 0 }

#[inline] pub unsafe fn chip_bus_lock(desc: *mut irq_desc) {
    if !(*(*desc).irq_data.chip).irq_bus_lock.is_none() { ((*(*desc).irq_data.chip).irq_bus_lock.unwrap())(&mut (*desc).irq_data); }
}
#[inline] pub unsafe fn chip_bus_sync_unlock(desc: *mut irq_desc) {
    if !(*(*desc).irq_data.chip).irq_bus_sync_unlock.is_none() { ((*(*desc).irq_data.chip).irq_bus_sync_unlock.unwrap())(&mut (*desc).irq_data); }
}

#[inline] pub unsafe fn irq_state_set_disabled(desc: *mut irq_desc) { irqd_set(&mut (*desc).irq_data, IRQD_IRQ_DISABLED); }
#[inline] pub unsafe fn irq_state_set_masked(desc: *mut irq_desc) { irqd_set(&mut (*desc).irq_data, IRQD_IRQ_MASKED); }
#[inline] pub unsafe fn irqd_set(d: *mut irq_data, mask: u32) { (*d).state_use_accessors |= mask; }
#[inline] pub unsafe fn irqd_clear(d: *mut irq_data, mask: u32) { (*d).state_use_accessors &= !mask; }
#[inline] pub unsafe fn irqd_get(d: *mut irq_data) -> u32 { (*d).state_use_accessors }
#[inline] pub unsafe fn irqd_has_set(d: *mut irq_data, mask: u32) -> bool { (*d).state_use_accessors & mask != 0 }
#[inline] pub unsafe fn irqd_set_move_pending(d: *mut irq_data) { irqd_set(d, IRQD_SETAFFINITY_PENDING); }
#[inline] pub unsafe fn irqd_clr_move_pending(d: *mut irq_data) { irqd_clear(d, IRQD_SETAFFINITY_PENDING); }
#[inline] pub unsafe fn irqd_set_managed_shutdown(d: *mut irq_data) { irqd_set(d, IRQD_MANAGED_SHUTDOWN); }
#[inline] pub unsafe fn irqd_clr_managed_shutdown(d: *mut irq_data) { irqd_clear(d, IRQD_MANAGED_SHUTDOWN); }

#[cfg(feature = "CONFIG_PM_SLEEP")]
extern "C" { pub fn irq_pm_handle_wakeup(desc: *mut irq_desc); pub fn irq_pm_install_action(desc: *mut irq_desc, action: *mut irqaction); pub fn irq_pm_remove_action(desc: *mut irq_desc, action: *mut irqaction); }
#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
#[inline] pub unsafe fn irq_pm_handle_wakeup(_desc: *mut irq_desc) {}
#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
#[inline] pub unsafe fn irq_pm_install_action(_desc: *mut irq_desc, _action: *mut irqaction) {}
#[cfg(not(feature = "CONFIG_PM_SLEEP"))]
#[inline] pub unsafe fn irq_pm_remove_action(_desc: *mut irq_desc, _action: *mut irqaction) {}

#[inline] pub unsafe fn irqd_get_parent_data(irqd: *mut irq_data) -> *mut irq_data {
    #[cfg(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY")] { (*irqd).parent_data }
    #[cfg(not(feature = "CONFIG_IRQ_DOMAIN_HIERARCHY"))] { core::ptr::null_mut() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
