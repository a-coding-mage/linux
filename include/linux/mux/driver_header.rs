/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mux/driver.h - definitions for the multiplexer driver interface
 *
 * Copyright (C) 2017 Axentia Technologies AB
 *
 * Author: Peter Rosin <peda@axentia.se>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub struct mux_chip;

pub struct mux_control;

/**
 * struct mux_control_ops - Mux controller operations for a mux chip.
 * @set: Set the state of the given mux controller.
 */
#[repr(C)]
pub struct mux_control_ops {
    pub set: Option<unsafe extern "C" fn(mux: *mut mux_control, state: ::core::ffi::c_int) -> ::core::ffi::c_int>,
}

/**
 * struct mux_control - Represents a mux controller.
 * @lock: Protects the mux controller state.
 * @chip: The mux chip that is handling this mux controller.
 * @cached_state: The current mux controller state, or -1 if none.
 * @states: The number of mux controller states.
 * @idle_state: The mux controller state to use when inactive, or one
 *              of MUX_IDLE_AS_IS and MUX_IDLE_DISCONNECT.
 * @last_change: Timestamp of last change
 *
 * Mux drivers may only change @states and @idle_state, and may only do so
 * between allocation and registration of the mux controller. Specifically,
 * @cached_state is internal to the mux core and should never be written by
 * mux drivers.
 */
#[repr(C)]
pub struct mux_control {
    pub lock: semaphore,
    pub chip: *mut mux_chip,
    pub cached_state: ::core::ffi::c_int,
    pub states: ::core::ffi::c_uint,
    pub idle_state: ::core::ffi::c_int,
    pub last_change: ktime_t,
}

/**
 * struct mux_chip - Represents a chip holding mux controllers.
 * @controllers: Number of mux controllers handled by the chip.
 * @dev: Device structure.
 * @id: Used to identify the device internally.
 * @ops: Mux controller operations.
 * @mux: Array of mux controllers that are handled.
 */
#[repr(C)]
pub struct mux_chip {
    pub controllers: ::core::ffi::c_uint,
    pub dev: device,
    pub id: ::core::ffi::c_int,
    pub ops: *const mux_control_ops,
    pub mux: [mux_control; 0],
}

#[macro_export]
macro_rules! to_mux_chip {
    ($x:expr) => {
        container_of!($x, $crate::mux_chip, dev)
    };
}

/**
 * mux_chip_priv() - Get the extra memory reserved by mux_chip_alloc().
 * @mux_chip: The mux-chip to get the private memory from.
 *
 * Return: Pointer to the private memory reserved by the allocator.
 */
#[inline]
pub unsafe fn mux_chip_priv(mux_chip: *mut mux_chip) -> *mut ::core::ffi::c_void {
    (*mux_chip).mux.as_mut_ptr().add((*mux_chip).controllers as usize) as *mut ::core::ffi::c_void
}

unsafe extern "C" {
    pub fn mux_chip_alloc(
        dev: *mut device,
        controllers: ::core::ffi::c_uint,
        sizeof_priv: usize,
    ) -> *mut mux_chip;
    pub fn mux_chip_register(mux_chip: *mut mux_chip) -> ::core::ffi::c_int;
    pub fn mux_chip_unregister(mux_chip: *mut mux_chip);
    pub fn mux_chip_free(mux_chip: *mut mux_chip);

    pub fn devm_mux_chip_alloc(
        dev: *mut device,
        controllers: ::core::ffi::c_uint,
        sizeof_priv: usize,
    ) -> *mut mux_chip;
    pub fn devm_mux_chip_register(dev: *mut device, mux_chip: *mut mux_chip) -> ::core::ffi::c_int;
}

/**
 * mux_control_get_index() - Get the index of the given mux controller
 * @mux: The mux-control to get the index for.
 *
 * Return: The index of the mux controller within the mux chip the mux
 * controller is a part of.
 */
#[inline]
pub unsafe fn mux_control_get_index(mux: *mut mux_control) -> ::core::ffi::c_uint {
    mux.offset_from((*mux).chip.as_ref().unwrap().mux.as_ptr()) as ::core::ffi::c_uint
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
