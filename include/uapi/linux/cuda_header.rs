/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Definitions for talking to the CUDA.  The CUDA is a microcontroller
 * which controls the ADB, system power, RTC, and various other things.
 *
 * Copyright (C) 1996 Paul Mackerras.
 */

/* CUDA commands (2nd byte) */
pub const CUDA_WARM_START: u32 = 0;
pub const CUDA_AUTOPOLL: u32 = 1;
pub const CUDA_GET_6805_ADDR: u32 = 2;
pub const CUDA_GET_TIME: u32 = 3;
pub const CUDA_GET_PRAM: u32 = 7;
pub const CUDA_SET_6805_ADDR: u32 = 8;
pub const CUDA_SET_TIME: u32 = 9;
pub const CUDA_POWERDOWN: u32 = 0xa;
pub const CUDA_POWERUP_TIME: u32 = 0xb;
pub const CUDA_SET_PRAM: u32 = 0xc;
pub const CUDA_MS_RESET: u32 = 0xd;
pub const CUDA_SEND_DFAC: u32 = 0xe;
pub const CUDA_RESET_SYSTEM: u32 = 0x11;
pub const CUDA_SET_IPL: u32 = 0x12;
pub const CUDA_SET_AUTO_RATE: u32 = 0x14;
pub const CUDA_GET_AUTO_RATE: u32 = 0x16;
pub const CUDA_SET_DEVICE_LIST: u32 = 0x19;
pub const CUDA_GET_DEVICE_LIST: u32 = 0x1a;
pub const CUDA_GET_SET_IIC: u32 = 0x22;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
