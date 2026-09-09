/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Simple Reset Controller ops
 *
 * Based on Allwinner SoCs Reset Controller driver
 *
 * Copyright 2013 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/io.h, linux/reset-controller.h, and linux/spinlock.h

/**
 * struct reset_simple_data - driver data for simple reset controllers
 * @lock: spinlock to protect registers during read-modify-write cycles
 * @membase: memory mapped I/O register range
 * @rcdev: reset controller device base structure
 * @active_low: if true, bits are cleared to assert the reset. Otherwise, bits
 *              are set to assert the reset. Note that this says nothing about
 *              the voltage level of the actual reset line.
 * @status_active_low: if true, bits read back as cleared while the reset is
 *                     asserted. Otherwise, bits read back as set while the
 *                     reset is asserted.
 * @reset_us: Minimum delay in microseconds needed that needs to be
 *            waited for between an assert and a deassert to reset the
 *            device. If multiple consumers with different delay
 *            requirements are connected to this controller, it must be
 *            the largest minimum delay. 0 means that such a delay is
 *            unknown and the reset operation is unsupported.
 */
#[repr(C)]
pub struct reset_simple_data {
    pub lock: spinlock_t,
    pub membase: *mut core::ffi::c_void,
    pub rcdev: reset_controller_dev,
    pub active_low: bool,
    pub status_active_low: bool,
    pub reset_us: core::ffi::c_uint,
}

extern "C" {
    pub static reset_simple_ops: reset_control_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
