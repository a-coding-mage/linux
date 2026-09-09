/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated headers:
// linux/list.h, linux/mutex.h

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_reference_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_phandle_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

/**
 * struct reset_control_ops - reset controller driver callbacks
 *
 * @reset: for self-deasserting resets, does all necessary
 *         things to reset the device
 * @assert: manually assert the reset line, if supported
 * @deassert: manually deassert the reset line, if supported
 * @status: return the status of the reset line, if supported
 */
#[repr(C)]
pub struct reset_control_ops {
    pub reset: Option<unsafe extern "C" fn(rcdev: *mut reset_controller_dev, id: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub assert: Option<unsafe extern "C" fn(rcdev: *mut reset_controller_dev, id: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub deassert: Option<unsafe extern "C" fn(rcdev: *mut reset_controller_dev, id: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub status: Option<unsafe extern "C" fn(rcdev: *mut reset_controller_dev, id: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
}

/**
 * struct reset_controller_dev - reset controller entity that might
 *                               provide multiple reset controls
 * @ops: a pointer to device specific struct reset_control_ops
 * @owner: kernel module of the reset controller driver
 * @list: internal list of reset controller devices
 * @reset_control_head: head of internal list of requested reset controls
 * @dev: corresponding driver model device struct
 * @of_node: corresponding device tree node as phandle target
 * @of_reset_n_cells: number of cells in reset line specifiers
 * @of_xlate: translation function to translate from specifier as found in the
 *            device tree to id as given to the reset control ops
 * @fwnode: firmware node associated with this device
 * @fwnode_reset_n_cells: number of cells in reset line specifiers
 * @fwnode_xlate: translation function to translate from firmware specifier to
 *                id as given to the reset control ops, defaults to
 *                :c:func:`fwnode_reset_simple_xlate`
 * @nr_resets: number of reset controls in this reset controller device
 * @lock: protects the reset control list from concurrent access
 */
#[repr(C)]
pub struct reset_controller_dev {
    pub ops: *const reset_control_ops,
    pub owner: *mut module,
    pub list: list_head,
    pub reset_control_head: list_head,
    pub dev: *mut device,
    pub of_node: *mut device_node,
    pub of_reset_n_cells: ::core::ffi::c_int,
    pub of_xlate: Option<unsafe extern "C" fn(rcdev: *mut reset_controller_dev, reset_spec: *const of_phandle_args) -> ::core::ffi::c_int>,
    pub fwnode: *mut fwnode_handle,
    pub fwnode_reset_n_cells: ::core::ffi::c_int,
    pub fwnode_xlate: Option<unsafe extern "C" fn(rcdev: *mut reset_controller_dev, reset_spec: *const fwnode_reference_args) -> ::core::ffi::c_int>,
    pub nr_resets: ::core::ffi::c_uint,
    pub lock: mutex,
}

// #if IS_ENABLED(CONFIG_RESET_CONTROLLER)
#[cfg(feature = "CONFIG_RESET_CONTROLLER")]
unsafe extern "C" {
    pub fn reset_controller_register(rcdev: *mut reset_controller_dev) -> ::core::ffi::c_int;
    pub fn reset_controller_unregister(rcdev: *mut reset_controller_dev);
    pub fn devm_reset_controller_register(dev: *mut device, rcdev: *mut reset_controller_dev) -> ::core::ffi::c_int;
}
// #else
#[cfg(not(feature = "CONFIG_RESET_CONTROLLER"))]
pub unsafe fn reset_controller_register(_rcdev: *mut reset_controller_dev) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_RESET_CONTROLLER"))]
pub unsafe fn reset_controller_unregister(_rcdev: *mut reset_controller_dev) {}

#[cfg(not(feature = "CONFIG_RESET_CONTROLLER"))]
pub unsafe fn devm_reset_controller_register(
    _dev: *mut device,
    _rcdev: *mut reset_controller_dev,
) -> ::core::ffi::c_int {
    0
}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
