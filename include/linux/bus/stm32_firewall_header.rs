/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023, STMicroelectronics - All Rights Reserved
 */

//! Rust translation of `stm32_firewall.h`.
//!
//! C header dependencies are expected to be supplied by the surrounding
//! translation unit.

use core::ffi::c_void;

// Opaque types supplied by the corresponding kernel headers.
pub enum device {}
pub enum list_head {}

/// STM32_PERIPHERAL_FIREWALL: This type of firewall protects peripherals.
pub const STM32_PERIPHERAL_FIREWALL: u32 = 1u32 << 1;
/// STM32_MEMORY_FIREWALL: This type of firewall protects memories/subsets of memory zones.
pub const STM32_MEMORY_FIREWALL: u32 = 1u32 << 2;
/// STM32_NOTYPE_FIREWALL: Undefined firewall type.
pub const STM32_NOTYPE_FIREWALL: u32 = 1u32 << 3;

/// Information on firewall controller supplying services.
#[repr(C)]
pub struct stm32_firewall_controller {
    /// Name of the firewall controller.
    pub name: *const core::ffi::c_char,
    /// Device reference of the firewall controller.
    pub dev: *mut device,
    /// Base address of the firewall controller.
    pub mmio: *mut c_void,
    /// List entry of the firewall controller list.
    pub entry: list_head,
    /// Type of firewall.
    pub type_: u32,
    /// Number of entries covered by the firewall.
    pub max_entries: u32,

    /// Callback used to grant access for a device access against a firewall controller.
    pub grant_access:
        Option<unsafe extern "C" fn(ctrl: *mut stm32_firewall_controller, id: u32) -> i32>,
    /// Callback used to release resources taken by a device when access was granted.
    pub release_access:
        Option<unsafe extern "C" fn(ctrl: *mut stm32_firewall_controller, id: u32)>,
    /// Callback used to grant access for a device to a given memory region.
    pub grant_memory_range_access: Option<
        unsafe extern "C" fn(
            ctrl: *mut stm32_firewall_controller,
            paddr: u64,
            size: usize,
        ) -> i32,
    >,
}

/// Register a firewall controller to the STM32 firewall framework.
pub unsafe extern "C" fn stm32_firewall_controller_register(
    firewall_controller: *mut stm32_firewall_controller,
) -> i32;

/// Unregister a firewall controller from the STM32 firewall framework.
pub unsafe extern "C" fn stm32_firewall_controller_unregister(
    firewall_controller: *mut stm32_firewall_controller,
);

/// Populate device tree nodes that have a correct firewall configuration.
pub unsafe extern "C" fn stm32_firewall_populate_bus(
    firewall_controller: *mut stm32_firewall_controller,
) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
