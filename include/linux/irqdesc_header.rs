/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are referenced here.

pub struct irq_affinity_notify;
pub struct proc_dir_entry;
pub struct module;
pub struct irq_desc;
pub struct irq_domain;
pub struct pt_regs;

#[repr(C)]
pub struct irqstat {
    pub cnt: core::ffi::c_uint,
    // CONFIG_GENERIC_IRQ_STAT_SNAPSHOT
    #[cfg(feature = "CONFIG_GENERIC_IRQ_STAT_SNAPSHOT")]
    pub ref_: core::ffi::c_uint,
}

#[repr(C)]
pub struct irq_redirect {
    pub work: irq_work,
    pub target_cpu: core::ffi::c_uint,
}

#[repr(C)]
pub struct irq_desc {
    pub irq_common_data: irq_common_data,
    pub irq_data: irq_data,
    pub kstat_irqs: *mut irqstat,
    pub handle_irq: irq_flow_handler_t,
    pub action: *mut irqaction,
    pub status_use_accessors: core::ffi::c_uint,
    pub core_internal_state__do_not_mess_with_it: core::ffi::c_uint,
    pub depth: core::ffi::c_uint,
    pub wake_depth: core::ffi::c_uint,
    pub tot_count: core::ffi::c_ulong,
    pub last_unhandled: core::ffi::c_ulong,
    pub irq_count: core::ffi::c_uint,
    pub irqs_unhandled: core::ffi::c_uint,
    pub threads_handled: atomic_t,
    pub threads_handled_last: core::ffi::c_int,
    pub lock: raw_spinlock_t,
    pub percpu_enabled: *mut cpumask,
    // CONFIG_SMP
    #[cfg(feature = "CONFIG_SMP")]
    pub redirect: irq_redirect,
    #[cfg(feature = "CONFIG_SMP")]
    pub affinity_hint: *const cpumask,
    #[cfg(feature = "CONFIG_SMP")]
    pub affinity_notify: *mut irq_affinity_notify,
    // CONFIG_GENERIC_PENDING_IRQ
    #[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_GENERIC_PENDING_IRQ"))]
    pub pending_mask: cpumask_var_t,
    pub threads_oneshot: core::ffi::c_ulong,
    pub threads_active: atomic_t,
    pub wait_for_threads: wait_queue_head_t,
    // CONFIG_PM_SLEEP
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub nr_actions: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub no_suspend_depth: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub cond_suspend_depth: core::ffi::c_uint,
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub force_resume_depth: core::ffi::c_uint,
    // CONFIG_PROC_FS
    #[cfg(feature = "CONFIG_PROC_FS")]
    pub dir: *mut proc_dir_entry,
    // CONFIG_GENERIC_IRQ_DEBUGFS
    #[cfg(feature = "CONFIG_GENERIC_IRQ_DEBUGFS")]
    pub debugfs_file: *mut dentry,
    #[cfg(feature = "CONFIG_GENERIC_IRQ_DEBUGFS")]
    pub dev_name: *const core::ffi::c_char,
    pub refcnt: rcuref_t,
    // CONFIG_SPARSE_IRQ
    #[cfg(feature = "CONFIG_SPARSE_IRQ")]
    pub rcu: rcu_head,
    #[cfg(feature = "CONFIG_SPARSE_IRQ")]
    pub kobj: kobject,
    pub request_mutex: mutex,
    pub parent_irq: core::ffi::c_int,
    pub owner: *mut module,
    pub name: *const core::ffi::c_char,
    // CONFIG_HARDIRQS_SW_RESEND
    #[cfg(feature = "CONFIG_HARDIRQS_SW_RESEND")]
    pub resend_node: hlist_node,
}

// CONFIG_SPARSE_IRQ
#[cfg(feature = "CONFIG_SPARSE_IRQ")]
pub fn irq_lock_sparse();
#[cfg(feature = "CONFIG_SPARSE_IRQ")]
pub fn irq_unlock_sparse();
#[cfg(not(feature = "CONFIG_SPARSE_IRQ"))]
pub fn irq_lock_sparse() {}
#[cfg(not(feature = "CONFIG_SPARSE_IRQ"))]
pub fn irq_unlock_sparse() {}
#[cfg(not(feature = "CONFIG_SPARSE_IRQ"))]
pub static mut irq_desc: [irq_desc; NR_IRQS] = unsafe { core::mem::zeroed() };

#[inline]
pub unsafe fn irq_desc_kstat_cpu(desc: *mut irq_desc, cpu: core::ffi::c_uint) -> core::ffi::c_uint {
    per_cpu((*(*desc).kstat_irqs).cnt, cpu)
}

#[inline]
pub unsafe fn irq_data_to_desc(data: *mut irq_data) -> *mut irq_desc {
    container_of((*data).common, irq_desc, irq_common_data)
}

#[inline]
pub unsafe fn irq_desc_get_irq(desc: *mut irq_desc) -> core::ffi::c_uint { (*desc).irq_data.irq }
#[inline]
pub unsafe fn irq_desc_get_irq_data(desc: *mut irq_desc) -> *mut irq_data { &mut (*desc).irq_data }
#[inline]
pub unsafe fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip { (*desc).irq_data.chip }
#[inline]
pub unsafe fn irq_desc_get_chip_data(desc: *mut irq_desc) -> *mut core::ffi::c_void { (*desc).irq_data.chip_data }
#[inline]
pub unsafe fn irq_desc_get_handler_data(desc: *mut irq_desc) -> *mut core::ffi::c_void { (*desc).irq_common_data.handler_data }

#[inline]
pub unsafe fn generic_handle_irq_desc(desc: *mut irq_desc) { ((*desc).handle_irq)(desc); }

pub fn handle_irq_desc(desc: *mut irq_desc) -> core::ffi::c_int;
pub fn generic_handle_irq(irq: core::ffi::c_uint) -> core::ffi::c_int;
pub fn generic_handle_irq_safe(irq: core::ffi::c_uint) -> core::ffi::c_int;

// CONFIG_IRQ_DOMAIN
#[cfg(feature = "CONFIG_IRQ_DOMAIN")]
pub fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_IRQ_DOMAIN")]
pub fn generic_handle_domain_irq_safe(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_IRQ_DOMAIN")]
pub fn generic_handle_domain_nmi(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> core::ffi::c_int;
#[cfg(feature = "CONFIG_IRQ_DOMAIN")]
pub fn generic_handle_demux_domain_irq(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> bool;

#[inline]
pub unsafe fn irq_desc_has_action(desc: *mut irq_desc) -> bool { !desc.is_null() && !(*desc).action.is_null() }

#[inline]
pub unsafe fn irq_set_handler_locked(data: *mut irq_data, handler: irq_flow_handler_t) {
    let desc = irq_data_to_desc(data);
    (*desc).handle_irq = handler;
}

#[inline]
pub unsafe fn irq_set_chip_handler_name_locked(data: *mut irq_data, chip: *const irq_chip, handler: irq_flow_handler_t, name: *const core::ffi::c_char) {
    let desc = irq_data_to_desc(data);
    (*desc).handle_irq = handler;
    (*desc).name = name;
    (*data).chip = chip as *mut irq_chip;
}

pub fn irq_check_status_bit(irq: core::ffi::c_uint, bitmask: core::ffi::c_uint) -> bool;
#[inline] pub fn irq_balancing_disabled(irq: core::ffi::c_uint) -> bool { irq_check_status_bit(irq, IRQ_NO_BALANCING_MASK) }
#[inline] pub fn irq_is_percpu(irq: core::ffi::c_uint) -> bool { irq_check_status_bit(irq, IRQ_PER_CPU) }
#[inline] pub fn irq_is_percpu_devid(irq: core::ffi::c_uint) -> bool { irq_check_status_bit(irq, IRQ_PER_CPU_DEVID) }

pub fn __irq_set_lockdep_class(irq: core::ffi::c_uint, lock_class: *mut lock_class_key, request_class: *mut lock_class_key);
#[inline]
pub unsafe fn irq_set_lockdep_class(irq: core::ffi::c_uint, lock_class: *mut lock_class_key, request_class: *mut lock_class_key) {
    if IS_ENABLED(CONFIG_LOCKDEP) { __irq_set_lockdep_class(irq, lock_class, request_class); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
