/*
 * vga_switcheroo.h - Support for laptop with dual GPU using one set of outputs
 *
 * Copyright (c) 2010 Red Hat Inc.
 * Author : Dave Airlie <airlied@redhat.com>
 *
 * Copyright (c) 2015 Lukas Wunner <lukas@wunner.de>
 *
 * Translated from the corresponding C header.
 */

use core::ffi::c_int;

// Supplied by the Linux framebuffer and device headers.
pub enum pci_dev {}
pub enum fb_info {}
pub enum device {}
pub enum dev_pm_domain {}

/** Handler flags bitmask. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vga_switcheroo_handler_flags_t {
    VGA_SWITCHEROO_CAN_SWITCH_DDC = 1 << 0,
    VGA_SWITCHEROO_NEEDS_EDP_CONFIG = 1 << 1,
}

/** Client power state. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vga_switcheroo_state {
    VGA_SWITCHEROO_OFF = 0,
    VGA_SWITCHEROO_ON,
    VGA_SWITCHEROO_NOT_FOUND,
}

/** Client identifier. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vga_switcheroo_client_id {
    VGA_SWITCHEROO_UNKNOWN_ID = 0x1000,
    VGA_SWITCHEROO_IGD = 0,
    VGA_SWITCHEROO_DIS,
    VGA_SWITCHEROO_MAX_CLIENTS,
}

/** Handler callbacks. */
#[repr(C)]
pub struct vga_switcheroo_handler {
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub switchto: Option<unsafe extern "C" fn(vga_switcheroo_client_id) -> c_int>,
    pub switch_ddc: Option<unsafe extern "C" fn(vga_switcheroo_client_id) -> c_int>,
    pub power_state:
        Option<unsafe extern "C" fn(vga_switcheroo_client_id, vga_switcheroo_state) -> c_int>,
    pub get_client_id:
        Option<unsafe extern "C" fn(*mut pci_dev) -> vga_switcheroo_client_id>,
}

/** Client callbacks. */
#[repr(C)]
pub struct vga_switcheroo_client_ops {
    pub set_gpu_state: Option<unsafe extern "C" fn(*mut pci_dev, vga_switcheroo_state)>,
    pub reprobe: Option<unsafe extern "C" fn(*mut pci_dev)>,
    pub can_switch: Option<unsafe extern "C" fn(*mut pci_dev) -> bool>,
    pub gpu_bound: Option<unsafe extern "C" fn(*mut pci_dev, vga_switcheroo_client_id)>,
}

// The following declarations are available when CONFIG_VGA_SWITCHEROO is enabled.
#[cfg(CONFIG_VGA_SWITCHEROO)]
extern "C" {
    pub fn vga_switcheroo_unregister_client(dev: *mut pci_dev);
    pub fn vga_switcheroo_register_client(
        dev: *mut pci_dev,
        ops: *const vga_switcheroo_client_ops,
        driver_power_control: bool,
    ) -> c_int;
    pub fn vga_switcheroo_register_audio_client(
        pdev: *mut pci_dev,
        ops: *const vga_switcheroo_client_ops,
        vga_dev: *mut pci_dev,
    ) -> c_int;
    pub fn vga_switcheroo_client_fb_set(dev: *mut pci_dev, info: *mut fb_info);
    pub fn vga_switcheroo_register_handler(
        handler: *const vga_switcheroo_handler,
        handler_flags: vga_switcheroo_handler_flags_t,
    ) -> c_int;
    pub fn vga_switcheroo_unregister_handler();
    pub fn vga_switcheroo_handler_flags() -> vga_switcheroo_handler_flags_t;
    pub fn vga_switcheroo_lock_ddc(pdev: *mut pci_dev) -> c_int;
    pub fn vga_switcheroo_unlock_ddc(pdev: *mut pci_dev) -> c_int;
    pub fn vga_switcheroo_process_delayed_switch() -> c_int;
    pub fn vga_switcheroo_client_probe_defer(pdev: *mut pci_dev) -> bool;
    pub fn vga_switcheroo_get_client_state(dev: *mut pci_dev) -> vga_switcheroo_state;
    pub fn vga_switcheroo_init_domain_pm_ops(dev: *mut device, domain: *mut dev_pm_domain) -> c_int;
    pub fn vga_switcheroo_fini_domain_pm_ops(dev: *mut device);
}

// CONFIG_VGA_SWITCHEROO disabled: preserve the C header's inline no-op behavior.
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_unregister_client(_dev: *mut pci_dev) {}
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_register_client(
    _dev: *mut pci_dev,
    _ops: *const vga_switcheroo_client_ops,
    _driver_power_control: bool,
) -> c_int { 0 }
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_client_fb_set(_dev: *mut pci_dev, _info: *mut fb_info) {}
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_register_handler(
    _handler: *const vga_switcheroo_handler,
    _handler_flags: vga_switcheroo_handler_flags_t,
) -> c_int { 0 }
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_register_audio_client(
    _pdev: *mut pci_dev,
    _ops: *const vga_switcheroo_client_ops,
    _vga_dev: *mut pci_dev,
) -> c_int { 0 }
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_unregister_handler() {}
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_handler_flags() -> vga_switcheroo_handler_flags_t {
    vga_switcheroo_handler_flags_t::VGA_SWITCHEROO_CAN_SWITCH_DDC
}
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_lock_ddc(_pdev: *mut pci_dev) -> c_int { -crate::ENODEV }
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_unlock_ddc(_pdev: *mut pci_dev) -> c_int { -crate::ENODEV }
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_process_delayed_switch() -> c_int { 0 }
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_client_probe_defer(_pdev: *mut pci_dev) -> bool { false }
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_get_client_state(_dev: *mut pci_dev) -> vga_switcheroo_state {
    vga_switcheroo_state::VGA_SWITCHEROO_ON
}
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_init_domain_pm_ops(
    _dev: *mut device,
    _domain: *mut dev_pm_domain,
) -> c_int { -crate::EINVAL }
#[cfg(not(CONFIG_VGA_SWITCHEROO))]
pub unsafe fn vga_switcheroo_fini_domain_pm_ops(_dev: *mut device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
