/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header guard __PPC_FSL_SOC_H.
// The declarations below correspond to the original __KERNEL__-only API.

// Dependency supplied externally: phys_addr_t, u32, and the kernel ABI types.

use core::ffi::c_char;

pub enum spi_device {}

extern "C" {
    pub fn get_immrbase() -> phys_addr_t;
}

// Original condition: defined(CONFIG_CPM) || defined(CONFIG_QUICC_ENGINE).
#[cfg(any(feature = "CONFIG_CPM", feature = "CONFIG_QUICC_ENGINE"))]
extern "C" {
    pub fn get_brgfreq() -> u32;
    pub fn get_baudrate() -> u32;
}

#[cfg(not(any(feature = "CONFIG_CPM", feature = "CONFIG_QUICC_ENGINE")))]
#[inline]
pub fn get_brgfreq() -> u32 {
    u32::MAX
}

#[cfg(not(any(feature = "CONFIG_CPM", feature = "CONFIG_QUICC_ENGINE")))]
#[inline]
pub fn get_baudrate() -> u32 {
    u32::MAX
}

extern "C" {
    pub fn fsl_get_sys_freq() -> u32;
}

pub enum spi_board_info {}
pub enum device_node {}

/* The different ports that the DIU can be connected to */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fsl_diu_monitor_port {
    FSL_DIU_PORT_DVI,
    FSL_DIU_PORT_LVDS,
    FSL_DIU_PORT_DLVDS,
}

#[repr(C)]
pub struct platform_diu_data_ops {
    pub get_pixel_format:
        Option<unsafe extern "C" fn(port: fsl_diu_monitor_port, bpp: u32) -> u32>,
    pub set_gamma_table:
        Option<unsafe extern "C" fn(port: fsl_diu_monitor_port, gamma_table_base: *mut c_char)>,
    pub set_monitor_port: Option<unsafe extern "C" fn(port: fsl_diu_monitor_port)>,
    pub set_pixel_clock: Option<unsafe extern "C" fn(pixclock: u32)>,
    pub valid_monitor_port: Option<
        unsafe extern "C" fn(port: fsl_diu_monitor_port) -> fsl_diu_monitor_port,
    >,
    pub release_bootmem: Option<unsafe extern "C" fn()>,
}

extern "C" {
    pub static mut diu_ops: platform_diu_data_ops;

    pub fn fsl_hv_restart(cmd: *mut c_char) -> !;
    pub fn fsl_hv_halt() -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
