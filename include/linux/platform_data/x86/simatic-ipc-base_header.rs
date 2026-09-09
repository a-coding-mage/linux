/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Siemens SIMATIC IPC drivers
 *
 * Copyright (c) Siemens AG, 2018-2023
 *
 * Authors:
 *  Henning Schild <henning.schild@siemens.com>
 *  Gerd Haeussler <gerd.haeussler.ext@siemens.com>
 */

// Dependency equivalent of <linux/types.h>: u8.

pub const SIMATIC_IPC_DEVICE_NONE: i32 = 0;
pub const SIMATIC_IPC_DEVICE_227D: i32 = 1;
pub const SIMATIC_IPC_DEVICE_427E: i32 = 2;
pub const SIMATIC_IPC_DEVICE_127E: i32 = 3;
pub const SIMATIC_IPC_DEVICE_227E: i32 = 4;
pub const SIMATIC_IPC_DEVICE_227G: i32 = 5;
pub const SIMATIC_IPC_DEVICE_BX_21A: i32 = 6;
pub const SIMATIC_IPC_DEVICE_BX_39A: i32 = 7;
pub const SIMATIC_IPC_DEVICE_BX_59A: i32 = 8;

#[repr(C)]
pub struct simatic_ipc_platform {
    pub devmode: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
