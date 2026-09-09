// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Samsung Electronics.
 * Kyungmin Park <kyungmin.park@samsung.com>
 * Tomasz Figa <t.figa@samsung.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// #include <linux/kernel.h>
// #include <linux/suspend.h>
// #include <asm/firmware.h>

#[repr(C)]
pub struct firmware_ops {
    _private: [u8; 0],
}

static default_firmware_ops: firmware_ops = firmware_ops { _private: [] };

pub static mut firmware_ops: *const firmware_ops = &default_firmware_ops;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
