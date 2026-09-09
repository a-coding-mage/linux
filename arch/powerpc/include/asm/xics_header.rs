/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common definitions across all variants of ICP and ICS interrupt
 * controllers.
 */

/* Dependency: linux/interrupt.h */

pub const XICS_IPI: ::core::ffi::c_uint = 2;
pub const XICS_IRQ_SPURIOUS: ::core::ffi::c_uint = 0;

/* Want a priority other than 0.  Various HW issues require this. */
pub const DEFAULT_PRIORITY: u8 = 5;

/*
 * Mark IPIs as higher priority so we can take them inside interrupts
 * FIXME: still true now?
 */
pub const IPI_PRIORITY: u8 = 4;

/* The least favored priority */
pub const LOWEST_PRIORITY: u8 = 0xFF;

/* The number of priorities defined above */
pub const MAX_NUM_PRIORITIES: usize = 3;

/* Native ICP */
#[cfg(CONFIG_PPC_ICP_NATIVE)]
extern "C" {
    pub fn icp_native_init() -> ::core::ffi::c_int;
    pub fn icp_native_flush_interrupt();
}
#[cfg(not(CONFIG_PPC_ICP_NATIVE))]
#[inline]
pub fn icp_native_init() -> ::core::ffi::c_int { -ENODEV }

/* PAPR ICP */
#[cfg(CONFIG_PPC_ICP_HV)]
extern "C" {
    pub fn icp_hv_init() -> ::core::ffi::c_int;
}
#[cfg(not(CONFIG_PPC_ICP_HV))]
#[inline]
pub fn icp_hv_init() -> ::core::ffi::c_int { -ENODEV }

#[cfg(CONFIG_PPC_POWERNV)]
extern "C" {
    pub fn icp_opal_init() -> ::core::ffi::c_int;
    pub fn icp_opal_flush_interrupt();
}
#[cfg(not(CONFIG_PPC_POWERNV))]
#[inline]
pub fn icp_opal_init() -> ::core::ffi::c_int { -ENODEV }

/* ICP ops */
#[repr(C)]
pub struct icp_ops {
    pub get_irq: Option<unsafe extern "C" fn() -> ::core::ffi::c_uint>,
    pub eoi: Option<unsafe extern "C" fn(d: *mut irq_data)>,
    pub set_priority: Option<unsafe extern "C" fn(prio: u8)>,
    pub teardown_cpu: Option<unsafe extern "C" fn()>,
    pub flush_ipi: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_SMP)]
    pub cause_ipi: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_int)>,
    #[cfg(CONFIG_SMP)]
    pub ipi_action: irq_handler_t,
}

extern "C" {
    pub static mut icp_ops: *const icp_ops;
}

/* Native ICS */
#[cfg(CONFIG_PPC_ICS_NATIVE)]
extern "C" { pub fn ics_native_init() -> ::core::ffi::c_int; }
#[cfg(not(CONFIG_PPC_ICS_NATIVE))]
#[inline]
pub fn ics_native_init() -> ::core::ffi::c_int { -ENODEV }

/* RTAS ICS */
#[cfg(CONFIG_PPC_ICS_RTAS)]
extern "C" { pub fn ics_rtas_init() -> ::core::ffi::c_int; }
#[cfg(not(CONFIG_PPC_ICS_RTAS))]
#[inline]
pub fn ics_rtas_init() -> ::core::ffi::c_int { -ENODEV }

/* HAL ICS */
#[cfg(CONFIG_PPC_POWERNV)]
extern "C" { pub fn ics_opal_init() -> ::core::ffi::c_int; }
#[cfg(not(CONFIG_PPC_POWERNV))]
#[inline]
pub fn ics_opal_init() -> ::core::ffi::c_int { -ENODEV }

/* ICS instance, hooked up to chip_data of an irq */
#[repr(C)]
pub struct ics {
    pub link: list_head,
    pub check: Option<unsafe extern "C" fn(ics: *mut ics, hwirq: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub mask_unknown: Option<unsafe extern "C" fn(ics: *mut ics, vec: ::core::ffi::c_ulong)>,
    pub get_server: Option<unsafe extern "C" fn(ics: *mut ics, vec: ::core::ffi::c_ulong) -> ::core::ffi::c_long>,
    pub host_match: Option<unsafe extern "C" fn(ics: *mut ics, node: *mut device_node) -> ::core::ffi::c_int>,
    pub chip: *mut irq_chip,
    pub data: [::core::ffi::c_char; 0],
}

/* Commons */
extern "C" {
    pub static mut xics_default_server: ::core::ffi::c_uint;
    pub static mut xics_default_distrib_server: ::core::ffi::c_uint;
    pub static mut xics_interrupt_server_size: ::core::ffi::c_uint;
    pub static mut xics_host: *mut irq_domain;
}

#[repr(C)]
pub struct xics_cppr {
    pub stack: [u8; MAX_NUM_PRIORITIES],
    pub index: ::core::ffi::c_int,
}

/* DECLARE_PER_CPU(struct xics_cppr, xics_cppr); */

#[inline]
pub unsafe fn xics_push_cppr(vec: ::core::ffi::c_uint) {
    let os_cppr: *mut xics_cppr = this_cpu_ptr(&mut xics_cppr);

    if WARN_ON((*os_cppr).index >= (MAX_NUM_PRIORITIES as ::core::ffi::c_int) - 1) {
        return;
    }

    (*os_cppr).index += 1;
    (*os_cppr).stack[(*os_cppr).index as usize] = if vec == XICS_IPI {
        IPI_PRIORITY
    } else {
        DEFAULT_PRIORITY
    };
}

#[inline]
pub unsafe fn xics_pop_cppr() -> u8 {
    let os_cppr: *mut xics_cppr = this_cpu_ptr(&mut xics_cppr);

    if WARN_ON((*os_cppr).index < 1) {
        return LOWEST_PRIORITY;
    }

    (*os_cppr).index -= 1;
    (*os_cppr).stack[(*os_cppr).index as usize]
}

#[inline]
pub unsafe fn xics_set_base_cppr(cppr: u8) {
    let os_cppr: *mut xics_cppr = this_cpu_ptr(&mut xics_cppr);

    /* we only really want to set the priority when there's
     * just one cppr value on the stack
     */
    WARN_ON((*os_cppr).index != 0);

    (*os_cppr).stack[0] = cppr;
}

#[inline]
pub unsafe fn xics_cppr_top() -> u8 {
    let os_cppr: *mut xics_cppr = this_cpu_ptr(&mut xics_cppr);

    (*os_cppr).stack[(*os_cppr).index as usize]
}

/* DECLARE_PER_CPU_SHARED_ALIGNED(unsigned long, xics_ipi_message); */

extern "C" {
    pub fn xics_init();
    pub fn xics_setup_cpu();
    pub fn xics_update_irq_servers();
    pub fn xics_set_cpu_giq(gserver: ::core::ffi::c_uint, join: ::core::ffi::c_uint);
    pub fn xics_mask_unknown_vec(vec: ::core::ffi::c_uint);
    pub fn xics_smp_probe();
    pub fn xics_register_ics(ics: *mut ics);
    pub fn xics_teardown_cpu();
    pub fn xics_kexec_teardown_cpu(secondary: ::core::ffi::c_int);
    pub fn xics_migrate_irqs_away();
    pub fn icp_native_eoi(d: *mut irq_data);
    pub fn xics_set_irq_type(d: *mut irq_data, flow_type: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn xics_retrigger(data: *mut irq_data) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn xics_get_irq_server(virq: ::core::ffi::c_uint, cpumask: *const cpumask,
                               strict_check: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn xics_get_irq_server(_virq: ::core::ffi::c_uint, _cpumask: *const cpumask,
                                  _strict_check: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    xics_default_server
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
