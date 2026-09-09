/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of linux/irq.h. Included C headers and build-time
 * configuration are supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct irq_common_data {
    pub state_use_accessors: u32,
    pub node: u32,
    pub handler_data: *mut core::ffi::c_void,
    pub msi_desc: *mut msi_desc,
    pub affinity: cpumask_var_t,
    pub effective_affinity: cpumask_var_t,
    pub ipi_offset: u32,
}

#[repr(C)]
pub struct irq_data {
    pub mask: u32,
    pub irq: u32,
    pub hwirq: irq_hw_number_t,
    pub common: *mut irq_common_data,
    pub chip: *mut irq_chip,
    pub domain: *mut irq_domain,
    pub parent_data: *mut irq_data,
    pub chip_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const core::ffi::c_char,
    pub irq_startup: Option<unsafe extern "C" fn(*mut irq_data) -> u32>,
    pub irq_shutdown: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_enable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_set_affinity: Option<unsafe extern "C" fn(*mut irq_data, *const cpumask, bool) -> i32>,
    pub irq_pre_redirect: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_retrigger: Option<unsafe extern "C" fn(*mut irq_data) -> i32>,
    pub irq_set_type: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
    pub irq_set_wake: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
    pub irq_bus_lock: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_bus_sync_unlock: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_cpu_online: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_cpu_offline: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_suspend: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_resume: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_pm_shutdown: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_calc_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_print_chip: Option<unsafe extern "C" fn(*mut irq_data, *mut seq_file)>,
    pub irq_request_resources: Option<unsafe extern "C" fn(*mut irq_data) -> i32>,
    pub irq_release_resources: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_compose_msi_msg: Option<unsafe extern "C" fn(*mut irq_data, *mut msi_msg)>,
    pub irq_write_msi_msg: Option<unsafe extern "C" fn(*mut irq_data, *mut msi_msg)>,
    pub irq_get_irqchip_state: Option<unsafe extern "C" fn(*mut irq_data, irqchip_irq_state, bool) -> i32>,
    pub irq_set_irqchip_state: Option<unsafe extern "C" fn(*mut irq_data, irqchip_irq_state, bool) -> i32>,
    pub irq_set_vcpu_affinity: Option<unsafe extern "C" fn(*mut irq_data, *mut core::ffi::c_void) -> i32>,
    pub ipi_send_single: Option<unsafe extern "C" fn(*mut irq_data, u32)>,
    pub ipi_send_mask: Option<unsafe extern "C" fn(*mut irq_data, *const cpumask)>,
    pub irq_nmi_setup: Option<unsafe extern "C" fn(*mut irq_data) -> i32>,
    pub irq_nmi_teardown: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_force_complete_move: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub flags: usize,
}

pub const IRQ_TYPE_NONE: u32 = 0;
pub const IRQ_TYPE_EDGE_RISING: u32 = 1;
pub const IRQ_TYPE_EDGE_FALLING: u32 = 2;
pub const IRQ_TYPE_EDGE_BOTH: u32 = IRQ_TYPE_EDGE_FALLING | IRQ_TYPE_EDGE_RISING;
pub const IRQ_TYPE_LEVEL_HIGH: u32 = 4;
pub const IRQ_TYPE_LEVEL_LOW: u32 = 8;
pub const IRQ_TYPE_LEVEL_MASK: u32 = IRQ_TYPE_LEVEL_LOW | IRQ_TYPE_LEVEL_HIGH;
pub const IRQ_TYPE_SENSE_MASK: u32 = 0xf;
pub const IRQ_TYPE_DEFAULT: u32 = IRQ_TYPE_SENSE_MASK;
pub const IRQ_TYPE_PROBE: u32 = 0x10;
pub const IRQ_LEVEL: u32 = 1 << 8;
pub const IRQ_PER_CPU: u32 = 1 << 9;
pub const IRQ_NOPROBE: u32 = 1 << 10;
pub const IRQ_NOREQUEST: u32 = 1 << 11;
pub const IRQ_NOAUTOEN: u32 = 1 << 12;
pub const IRQ_NO_BALANCING: u32 = 1 << 13;
pub const IRQ_NESTED_THREAD: u32 = 1 << 15;
pub const IRQ_NOTHREAD: u32 = 1 << 16;
pub const IRQ_PER_CPU_DEVID: u32 = 1 << 17;
pub const IRQ_IS_POLLED: u32 = 1 << 18;
pub const IRQ_DISABLE_UNLAZY: u32 = 1 << 19;
pub const IRQ_HIDDEN: u32 = 1 << 20;
pub const IRQ_NO_DEBUG: u32 = 1 << 21;
pub const IRQ_RESERVED: u32 = 1 << 22;
pub const IRQF_MODIFY_MASK: u32 = IRQ_TYPE_SENSE_MASK | IRQ_NOPROBE | IRQ_NOREQUEST | IRQ_NOAUTOEN | IRQ_LEVEL | IRQ_NO_BALANCING | IRQ_PER_CPU | IRQ_NESTED_THREAD | IRQ_NOTHREAD | IRQ_PER_CPU_DEVID | IRQ_IS_POLLED | IRQ_DISABLE_UNLAZY | IRQ_HIDDEN;
pub const IRQ_NO_BALANCING_MASK: u32 = IRQ_PER_CPU | IRQ_NO_BALANCING;

pub const IRQ_SET_MASK_OK: i32 = 0;
pub const IRQ_SET_MASK_OK_NOCOPY: i32 = 1;
pub const IRQ_SET_MASK_OK_DONE: i32 = 2;

pub const IRQD_TRIGGER_MASK: u32 = 0xf;
pub const IRQD_SETAFFINITY_PENDING: u32 = 1 << 8;
pub const IRQD_ACTIVATED: u32 = 1 << 9;
pub const IRQD_NO_BALANCING: u32 = 1 << 10;
pub const IRQD_PER_CPU: u32 = 1 << 11;
pub const IRQD_AFFINITY_SET: u32 = 1 << 12;
pub const IRQD_LEVEL: u32 = 1 << 13;
pub const IRQD_WAKEUP_STATE: u32 = 1 << 14;
pub const IRQD_IRQ_DISABLED: u32 = 1 << 16;
pub const IRQD_IRQ_MASKED: u32 = 1 << 17;
pub const IRQD_IRQ_INPROGRESS: u32 = 1 << 18;
pub const IRQD_WAKEUP_ARMED: u32 = 1 << 19;
pub const IRQD_FORWARDED_TO_VCPU: u32 = 1 << 20;
pub const IRQD_AFFINITY_MANAGED: u32 = 1 << 21;
pub const IRQD_IRQ_STARTED: u32 = 1 << 22;
pub const IRQD_MANAGED_SHUTDOWN: u32 = 1 << 23;
pub const IRQD_SINGLE_TARGET: u32 = 1 << 24;
pub const IRQD_DEFAULT_TRIGGER_SET: u32 = 1 << 25;
pub const IRQD_CAN_RESERVE: u32 = 1 << 26;
pub const IRQD_HANDLE_ENFORCE_IRQCTX: u32 = 1 << 27;
pub const IRQD_AFFINITY_ON_ACTIVATE: u32 = 1 << 28;
pub const IRQD_IRQ_ENABLED_ON_SUSPEND: u32 = 1 << 29;
pub const IRQD_RESEND_WHEN_IN_PROGRESS: u32 = 1 << 30;

#[inline] pub unsafe fn irqd_get_trigger_type(d: *mut irq_data) -> u32 { (*(*d).common).state_use_accessors & IRQD_TRIGGER_MASK }
#[inline] pub unsafe fn irqd_is_setaffinity_pending(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_SETAFFINITY_PENDING != 0 }
#[inline] pub unsafe fn irqd_is_per_cpu(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_PER_CPU != 0 }
#[inline] pub unsafe fn irqd_can_balance(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & (IRQD_PER_CPU | IRQD_NO_BALANCING) == 0 }
#[inline] pub unsafe fn irqd_affinity_was_set(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_AFFINITY_SET != 0 }
#[inline] pub unsafe fn irqd_mark_affinity_was_set(d: *mut irq_data) { (*(*d).common).state_use_accessors |= IRQD_AFFINITY_SET; }
#[inline] pub unsafe fn irqd_set_trigger_type(d: *mut irq_data, ty: u32) { (*(*d).common).state_use_accessors = ((*(*d).common).state_use_accessors & !IRQD_TRIGGER_MASK) | (ty & IRQD_TRIGGER_MASK) | IRQD_DEFAULT_TRIGGER_SET; }
#[inline] pub unsafe fn irqd_is_level_type(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_LEVEL != 0 }
#[inline] pub unsafe fn irqd_is_single_target(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_SINGLE_TARGET != 0 }
#[inline] pub unsafe fn irqd_set_single_target(d: *mut irq_data) { (*(*d).common).state_use_accessors |= IRQD_SINGLE_TARGET; }
#[inline] pub unsafe fn irqd_is_activated(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_ACTIVATED != 0 }
#[inline] pub unsafe fn irqd_set_activated(d: *mut irq_data) { (*(*d).common).state_use_accessors |= IRQD_ACTIVATED; }
#[inline] pub unsafe fn irqd_clr_activated(d: *mut irq_data) { (*(*d).common).state_use_accessors &= !IRQD_ACTIVATED; }
#[inline] pub unsafe fn irqd_is_wakeup_set(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_WAKEUP_STATE != 0 }
#[inline] pub unsafe fn irqd_irq_disabled(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_IRQ_DISABLED != 0 }
#[inline] pub unsafe fn irqd_irq_masked(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_IRQ_MASKED != 0 }
#[inline] pub unsafe fn irqd_irq_inprogress(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_IRQ_INPROGRESS != 0 }
#[inline] pub unsafe fn irqd_is_wakeup_armed(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_WAKEUP_ARMED != 0 }
#[inline] pub unsafe fn irqd_is_forwarded_to_vcpu(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_FORWARDED_TO_VCPU != 0 }
#[inline] pub unsafe fn irqd_set_forwarded_to_vcpu(d: *mut irq_data) { (*(*d).common).state_use_accessors |= IRQD_FORWARDED_TO_VCPU; }
#[inline] pub unsafe fn irqd_clr_forwarded_to_vcpu(d: *mut irq_data) { (*(*d).common).state_use_accessors &= !IRQD_FORWARDED_TO_VCPU; }
#[inline] pub unsafe fn irqd_affinity_is_managed(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_AFFINITY_MANAGED != 0 }
#[inline] pub unsafe fn irqd_is_started(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_IRQ_STARTED != 0 }
#[inline] pub unsafe fn irqd_is_managed_and_shutdown(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_MANAGED_SHUTDOWN != 0 }
#[inline] pub unsafe fn irqd_can_reserve(d: *mut irq_data) -> bool { (*(*d).common).state_use_accessors & IRQD_CAN_RESERVE != 0 }
#[inline] pub unsafe fn irqd_set_can_reserve(d: *mut irq_data) { (*(*d).common).state_use_accessors |= IRQD_CAN_RESERVE; }
#[inline] pub unsafe fn irqd_clr_can_reserve(d: *mut irq_data) { (*(*d).common).state_use_accessors &= !IRQD_CAN_RESERVE; }
#[inline] pub unsafe fn irqd_to_hwirq(d: *mut irq_data) -> irq_hw_number_t { (*d).hwirq }

#[repr(C)] pub struct irq_chip_regs { pub enable: usize, pub disable: usize, pub mask: usize, pub ack: usize, pub eoi: usize, pub type_: usize }
#[repr(C)] pub struct irq_chip_type { pub chip: irq_chip, pub regs: irq_chip_regs, pub handler: irq_flow_handler_t, pub type_: u32, pub mask_cache_priv: u32, pub mask_cache: *mut u32 }
#[repr(C)] pub struct irq_chip_generic { pub lock: raw_spinlock_t, pub reg_base: *mut core::ffi::c_void, pub reg_readl: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> u32>, pub reg_writel: Option<unsafe extern "C" fn(u32, *mut core::ffi::c_void)>, pub suspend: Option<unsafe extern "C" fn(*mut irq_chip_generic)>, pub resume: Option<unsafe extern "C" fn(*mut irq_chip_generic)>, pub irq_base: u32, pub irq_cnt: u32, pub mask_cache: u32, pub wake_enabled: u32, pub wake_active: u32, pub num_ct: u32, pub private: *mut core::ffi::c_void, pub installed: usize, pub unused: usize, pub domain: *mut irq_domain, pub list: list_head }
#[repr(C)] pub struct irq_domain_chip_generic { pub irqs_per_chip: u32, pub num_chips: u32, pub irq_flags_to_clear: u32, pub irq_flags_to_set: u32, pub gc_flags: irq_gc_flags, pub exit: Option<unsafe extern "C" fn(*mut irq_chip_generic)>, pub gc: *mut *mut irq_chip_generic }
#[repr(C)] pub struct irq_domain_chip_generic_info { pub name: *const core::ffi::c_char, pub handler: irq_flow_handler_t, pub irqs_per_chip: u32, pub num_ct: u32, pub irq_flags_to_clear: u32, pub irq_flags_to_set: u32, pub gc_flags: irq_gc_flags, pub init: Option<unsafe extern "C" fn(*mut irq_chip_generic) -> i32>, pub exit: Option<unsafe extern "C" fn(*mut irq_chip_generic)> }
#[repr(u32)] pub enum irq_gc_flags { IRQ_GC_INIT_MASK_CACHE = 1, IRQ_GC_INIT_NESTED_LOCK = 2, IRQ_GC_MASK_CACHE_PER_TYPE = 4, IRQ_GC_NO_MASK = 8, IRQ_GC_BE_IO = 16 }

pub const INVALID_HWIRQ: usize = usize::MAX;
pub const IRQCHIP_SET_TYPE_MASKED: usize = 1 << 0;
pub const IRQCHIP_EOI_IF_HANDLED: usize = 1 << 1;
pub const IRQCHIP_MASK_ON_SUSPEND: usize = 1 << 2;
pub const IRQCHIP_ONOFFLINE_ENABLED: usize = 1 << 3;
pub const IRQCHIP_SKIP_SET_WAKE: usize = 1 << 4;
pub const IRQCHIP_ONESHOT_SAFE: usize = 1 << 5;
pub const IRQCHIP_EOI_THREADED: usize = 1 << 6;
pub const IRQCHIP_SUPPORTS_LEVEL_MSI: usize = 1 << 7;
pub const IRQCHIP_SUPPORTS_NMI: usize = 1 << 8;
pub const IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND: usize = 1 << 9;
pub const IRQCHIP_AFFINITY_PRE_STARTUP: usize = 1 << 10;
pub const IRQCHIP_IMMUTABLE: usize = 1 << 11;
pub const IRQCHIP_MOVE_DEFERRED: usize = 1 << 12;

/* External declarations from the included kernel headers and this header. */
extern "C" {
    pub fn irq_get_irq_data(irq: u32) -> *mut irq_data;
    pub fn irq_set_chip_and_handler_name(irq: u32, chip: *const irq_chip, handle: irq_flow_handler_t, name: *const core::ffi::c_char);
    pub fn __irq_set_handler(irq: u32, handle: irq_flow_handler_t, is_chained: i32, name: *const core::ffi::c_char);
    pub fn irq_modify_status(irq: u32, clr: usize, set: usize);
    pub fn irq_set_chip(irq: u32, chip: *const irq_chip) -> i32;
    pub fn irq_set_handler_data(irq: u32, data: *mut core::ffi::c_void) -> i32;
    pub fn irq_set_chip_data(irq: u32, data: *mut core::ffi::c_void) -> i32;
    pub fn irq_set_irq_type(irq: u32, ty: u32) -> i32;
    pub fn irq_set_msi_desc(irq: u32, entry: *mut msi_desc) -> i32;
    pub fn irq_free_descs(irq: u32, cnt: u32);
    pub fn __irq_alloc_descs(irq: i32, from: u32, cnt: u32, node: i32, owner: *mut module, affinity: *const irq_affinity_desc) -> i32;
    pub fn arch_dynirq_lower_bound(from: u32) -> u32;
    pub fn irq_chip_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) -> i32;
    pub fn irq_chip_pm_get(data: *mut irq_data) -> i32;
    pub fn irq_chip_pm_put(data: *mut irq_data);
    pub fn machine_kexec_mask_interrupts();
    pub fn note_interrupt(desc: *mut irq_desc, action_ret: irqreturn_t);
    pub fn can_request_irq(irq: u32, irqflags: usize) -> bool;
}

#[inline] pub unsafe fn irq_set_chip_and_handler(irq: u32, chip: *const irq_chip, handle: irq_flow_handler_t) { irq_set_chip_and_handler_name(irq, chip, handle, core::ptr::null()); }
#[inline] pub unsafe fn irq_set_handler(irq: u32, handle: irq_flow_handler_t) { __irq_set_handler(irq, handle, 0, core::ptr::null()); }
#[inline] pub unsafe fn irq_set_chained_handler(irq: u32, handle: irq_flow_handler_t) { __irq_set_handler(irq, handle, 1, core::ptr::null()); }
#[inline] pub unsafe fn irq_set_status_flags(irq: u32, set: usize) { irq_modify_status(irq, 0, set); }
#[inline] pub unsafe fn irq_clear_status_flags(irq: u32, clr: usize) { irq_modify_status(irq, clr, 0); }
#[inline] pub unsafe fn irq_set_noprobe(irq: u32) { irq_set_status_flags(irq, IRQ_NOPROBE as usize); }
#[inline] pub unsafe fn irq_set_probe(irq: u32) { irq_clear_status_flags(irq, IRQ_NOPROBE as usize); }
#[inline] pub unsafe fn irq_set_nothread(irq: u32) { irq_set_status_flags(irq, IRQ_NOTHREAD as usize); }
#[inline] pub unsafe fn irq_set_thread(irq: u32) { irq_clear_status_flags(irq, IRQ_NOTHREAD as usize); }
#[inline] pub unsafe fn irq_set_nested_thread(irq: u32, nest: bool) { if nest { irq_set_status_flags(irq, IRQ_NESTED_THREAD as usize) } else { irq_clear_status_flags(irq, IRQ_NESTED_THREAD as usize) } }
#[inline] pub unsafe fn irq_free_desc(irq: u32) { irq_free_descs(irq, 1); }

/* Types supplied by other translated kernel headers. */
#[allow(non_camel_case_types)] pub type irq_hw_number_t = usize;
#[allow(non_camel_case_types)] pub type irq_flow_handler_t = Option<unsafe extern "C" fn(*mut irq_desc)>;
#[allow(non_camel_case_types)] pub type cpumask_var_t = *mut cpumask;
#[allow(non_camel_case_types)] pub type irqreturn_t = i32;
pub enum cpumask {}
pub enum irq_desc {}
pub enum irq_domain {}
pub enum msi_desc {}
pub enum msi_msg {}
pub enum module {}
pub enum irq_affinity_desc {}
pub enum seq_file {}
pub enum list_head {}
pub enum raw_spinlock_t {}
pub enum irqchip_irq_state {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
