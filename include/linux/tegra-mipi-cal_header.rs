/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __TEGRA_MIPI_CAL_H_

use core::ffi::c_int;

// External types supplied by other headers/dependencies.
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tegra_mipi_device {
    pub ops: *const tegra_mipi_ops,
    pub pdev: *mut platform_device,
    pub pads: usize,
}

/**
 * Operations for Tegra MIPI calibration device
 */
#[repr(C)]
pub struct tegra_mipi_ops {
    /**
     * @enable:
     *
     * Enable MIPI calibration device
     */
    pub enable: Option<unsafe extern "C" fn(device: *mut tegra_mipi_device) -> c_int>,

    /**
     * @disable:
     *
     * Disable MIPI calibration device
     */
    pub disable: Option<unsafe extern "C" fn(device: *mut tegra_mipi_device) -> c_int>,

    /**
     * @start_calibration:
     *
     * Start MIPI calibration
     */
    pub start_calibration:
        Option<unsafe extern "C" fn(device: *mut tegra_mipi_device) -> c_int>,

    /**
     * @finish_calibration:
     *
     * Finish MIPI calibration
     */
    pub finish_calibration:
        Option<unsafe extern "C" fn(device: *mut tegra_mipi_device) -> c_int>,
}

unsafe extern "C" {
    pub fn devm_tegra_mipi_add_provider(
        device: *mut device,
        np: *mut device_node,
        ops: *const tegra_mipi_ops,
    ) -> c_int;

    pub fn tegra_mipi_request(
        device: *mut device,
        np: *mut device_node,
    ) -> *mut tegra_mipi_device;
    pub fn tegra_mipi_free(device: *mut tegra_mipi_device);

    pub fn tegra_mipi_enable(device: *mut tegra_mipi_device) -> c_int;
    pub fn tegra_mipi_disable(device: *mut tegra_mipi_device) -> c_int;
    pub fn tegra_mipi_start_calibration(device: *mut tegra_mipi_device) -> c_int;
    pub fn tegra_mipi_finish_calibration(device: *mut tegra_mipi_device) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
