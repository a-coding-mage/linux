/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2006 IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_uint};

/* C dependencies: linux/interrupt.h, asm/rtas.h, and linux/of.h. */

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct property {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pseries_hp_errorlog {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_host_bridge {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_controller_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_controller {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iommu_group {
    _private: [u8; 0],
}

/* irq_handler_t is supplied by linux/interrupt.h. */
pub type irq_handler_t = unsafe extern "C" fn();

extern "C" {
    pub fn request_event_sources_irqs(
        np: *mut device_node,
        handler: irq_handler_t,
        name: *const c_char,
    );

    pub fn pSeries_system_reset_exception(regs: *mut pt_regs) -> c_int;
    pub fn pSeries_machine_check_exception(regs: *mut pt_regs) -> c_int;
    pub fn pseries_machine_check_realmode(regs: *mut pt_regs) -> c_long;
    pub fn pSeries_machine_check_log_err();

    #[cfg(feature = "CONFIG_SMP")]
    pub fn smp_init_pseries();

    #[cfg(feature = "CONFIG_SMP")]
    pub fn smp_query_cpu_stopped(pcpu: c_uint) -> c_int;

    pub fn pseries_kexec_cpu_down(crash_shutdown: c_int, secondary: c_int);
    pub fn pSeries_final_fixup();

    pub static mut rtas_poweron_auto: c_ulong;

    pub fn dlpar_free_cc_nodes(node: *mut device_node);
    pub fn dlpar_free_cc_property(property: *mut property);
    pub fn dlpar_configure_connector(be32_value: u32, node: *mut device_node) -> *mut device_node;
    pub fn dlpar_attach_node(child: *mut device_node, parent: *mut device_node) -> c_int;
    pub fn dlpar_detach_node(node: *mut device_node) -> c_int;
    pub fn dlpar_acquire_drc(drc_index: u32) -> c_int;
    pub fn dlpar_release_drc(drc_index: u32) -> c_int;
    pub fn dlpar_unisolate_drc(drc_index: u32) -> c_int;
    pub fn post_mobility_fixup();

    pub fn queue_hotplug_event(hp_errlog: *mut pseries_hp_errorlog);
    pub fn handle_dlpar_errorlog(hp_errlog: *mut pseries_hp_errorlog) -> c_int;

    #[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
    pub fn dlpar_memory(hp_elog: *mut pseries_hp_errorlog) -> c_int;
    #[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
    pub fn dlpar_hp_pmem(hp_elog: *mut pseries_hp_errorlog) -> c_int;

    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub fn dlpar_cpu(hp_elog: *mut pseries_hp_errorlog) -> c_int;
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub fn pseries_cpu_hotplug_init();

    pub fn pseries_root_bridge_prepare(bridge: *mut pci_host_bridge) -> c_int;
    pub static mut pseries_pci_controller_ops: pci_controller_ops;
    pub fn pseries_msi_allocate_domains(phb: *mut pci_controller) -> c_int;
    pub fn pseries_msi_free_domains(phb: *mut pci_controller);

    pub static mut CMO_PrPSP: c_int;
    pub static mut CMO_SecPSP: c_int;
    pub static mut CMO_PageSize: c_ulong;

    pub fn dlpar_workqueue_init() -> c_int;
    pub static mut pseries_security_flavor: u32;
    pub fn pseries_setup_security_mitigations();

    #[cfg(feature = "CONFIG_PPC_64S_HASH_MMU")]
    pub fn pseries_lpar_read_hblkrm_characteristics();

    pub fn pseries_rng_init();

    #[cfg(feature = "CONFIG_SPAPR_TCE_IOMMU")]
    pub fn pSeries_pci_device_group(
        hose: *mut pci_controller,
        pdev: *mut pci_dev,
    ) -> *mut iommu_group;
}

pub const QCSS_STOPPED: c_int = 0;
pub const QCSS_STOPPING: c_int = 1;
pub const QCSS_NOT_STOPPED: c_int = 2;
pub const QCSS_HARDWARE_ERROR: c_int = -1;
pub const QCSS_HARDWARE_BUSY: c_int = -2;

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub fn smp_init_pseries() {}

#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub unsafe fn dlpar_memory(_hp_elog: *mut pseries_hp_errorlog) -> c_int {
    -95 /* -EOPNOTSUPP */
}

#[cfg(not(feature = "CONFIG_MEMORY_HOTPLUG"))]
#[inline]
pub unsafe fn dlpar_hp_pmem(_hp_elog: *mut pseries_hp_errorlog) -> c_int {
    -95 /* -EOPNOTSUPP */
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub unsafe fn dlpar_cpu(_hp_elog: *mut pseries_hp_errorlog) -> c_int {
    -95 /* -EOPNOTSUPP */
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn pseries_cpu_hotplug_init() {}

#[inline]
pub unsafe fn cmo_get_primary_psp() -> c_int {
    CMO_PrPSP
}

#[inline]
pub unsafe fn cmo_get_secondary_psp() -> c_int {
    CMO_SecPSP
}

#[inline]
pub unsafe fn cmo_get_page_size() -> c_ulong {
    CMO_PageSize
}

#[cfg(not(feature = "CONFIG_PPC_64S_HASH_MMU"))]
#[inline]
pub fn pseries_lpar_read_hblkrm_characteristics() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
