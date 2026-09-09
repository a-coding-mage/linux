/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/*
 * Apple Silicon hardware tunable support
 *
 * Each tunable is a list with each entry containing a offset into the MMIO
 * region, a mask of bits to be cleared and a set of bits to be set. These
 * tunables are passed along by the previous boot stages and vary from device
 * to device such that they cannot be hardcoded in the individual drivers.
 *
 * Copyright (C) The Asahi Linux Contributors
 */

// C dependencies: linux/device.h and linux/types.h.

/**
 * Struct to store an Apple Silicon hardware tunable.
 *
 * Each tunable is a list with each entry containing a offset into the MMIO
 * region, a mask of bits to be cleared and a set of bits to be set. These
 * tunables are passed along by the previous boot stages and vary from device
 * to device such that they cannot be hardcoded in the individual drivers.
 *
 * @param sz Number of [offset, mask, value] tuples stored in values.
 * @param values [offset, mask, value] array.
 */
#[repr(C)]
pub struct AppleTunable {
    pub sz: usize,
    // Flexible array member, counted by `sz` in the C definition.
    pub values: [AppleTunableValue; 0],
}

#[repr(C)]
pub struct AppleTunableValue {
    pub offset: u32,
    pub mask: u32,
    pub value: u32,
}

// Opaque types supplied by the corresponding Linux dependencies.
#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Resource {
    _private: [u8; 0],
}

/**
 * Parse an array of hardware tunables from the device tree.
 *
 * @dev: Device node used for devm_kzalloc internally.
 * @np: Device node which contains the tunable array.
 * @name: Name of the device tree property which contains the tunables.
 * @res: Resource to which the tunables will be applied, used for bound checking
 *
 * @return: devres allocated struct on success or PTR_ERR on failure.
 */
unsafe extern "C" {
    pub fn devm_apple_tunable_parse(
        dev: *mut Device,
        np: *mut DeviceNode,
        name: *const core::ffi::c_char,
        res: *mut Resource,
    ) -> *mut AppleTunable;
}

/**
 * Apply a previously loaded hardware tunable.
 *
 * @param regs: MMIO to which the tunable will be applied.
 * @param tunable: Pointer to the tunable.
 */
unsafe extern "C" {
    pub fn apple_tunable_apply(regs: *mut core::ffi::c_void, tunable: *mut AppleTunable);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
