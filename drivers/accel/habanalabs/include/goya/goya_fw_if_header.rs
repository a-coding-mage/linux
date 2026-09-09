/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// Header guard: GOYA_FW_IF_H

pub const GOYA_EVENT_QUEUE_MSIX_IDX: u32 = 5;

pub const CPU_BOOT_ADDR: u64 = 0x7FF8040000u64;

pub const UBOOT_FW_OFFSET: u32 = 0x100000; // 1MB in SRAM
pub const LINUX_FW_OFFSET: u32 = 0x800000; // 8MB in DDR

pub const GOYA_PLL_FREQ_LOW: u32 = 50000000; // 50 MHz

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
