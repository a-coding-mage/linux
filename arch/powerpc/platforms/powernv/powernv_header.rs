/* SPDX-License-Identifier: GPL-2.0 */

/*
 * There's various hacks scattered throughout the generic powerpc arch code
 * that needs to call into powernv platform stuff. The prototypes for those
 * functions are in asm/powernv.h
 *
 * Dependency: asm/powernv.h
 */

#[cfg(CONFIG_SMP)]
unsafe extern "C" {
    pub fn pnv_smp_init();
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub fn pnv_smp_init() {}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pnv_platform_error_reboot(regs: *mut pt_regs, msg: *const core::ffi::c_char) -> !;
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[cfg(CONFIG_PCI)]
unsafe extern "C" {
    pub fn pnv_pci_init();
    pub fn pnv_pci_shutdown();
}

#[cfg(not(CONFIG_PCI))]
#[inline]
pub fn pnv_pci_init() {}

#[cfg(not(CONFIG_PCI))]
#[inline]
pub fn pnv_pci_shutdown() {}

unsafe extern "C" {
    pub fn pnv_get_supported_cpuidle_states() -> u32;
    pub fn pnv_lpc_init();
    pub fn opal_handle_events();
    pub fn opal_have_pending_events() -> bool;
    pub fn opal_event_shutdown();
    pub fn cpu_core_split_required() -> bool;
}

#[repr(C)]
pub struct memcons {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn memcons_copy(
        mc: *mut memcons,
        to: *mut core::ffi::c_char,
        pos: i64,
        count: usize,
    ) -> isize;

    /* __init */
    pub fn memcons_get_size(mc: *mut memcons) -> u32;

    /* __init */
    pub fn memcons_init(
        node: *mut device_node,
        mc_prop_name: *const core::ffi::c_char,
    ) -> *mut memcons;

    pub fn pnv_rng_init();
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
