/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding kernel translation. */
pub type time64_t = i64;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_controller_ops {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pas_get_boot_time() -> time64_t;
    pub fn pas_pci_init();
    pub fn pas_pci_dma_dev_setup(dev: *mut pci_dev);

    pub fn pasemi_pci_getcfgaddr(dev: *mut pci_dev, offset: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_void;

    pub fn pasemi_map_registers();

    /* Power savings modes, implemented in asm */
    pub fn idle_spin();
    pub fn idle_doze();
}

/* Restore astate to last set */
#[cfg(feature = "CONFIG_PPC_PASEMI_CPUFREQ")]
unsafe extern "C" {
    pub fn check_astate() -> ::core::ffi::c_int;
    pub fn restore_astate(cpu: ::core::ffi::c_int);
}

#[cfg(not(feature = "CONFIG_PPC_PASEMI_CPUFREQ"))]
#[inline]
pub fn check_astate() -> ::core::ffi::c_int {
    /* Always return >0 so we never power save */
    1
}

#[cfg(not(feature = "CONFIG_PPC_PASEMI_CPUFREQ"))]
#[inline]
pub fn restore_astate(_cpu: ::core::ffi::c_int) {}

unsafe extern "C" {
    pub static mut pasemi_pci_controller_ops: pci_controller_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
