/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

/*
 * high memory space
 *
 * in loongson2e, starts from 512M
 * in loongson2f, starts from 2G 256M
 *
 * The original C definition selects this value using CONFIG_CPU_LOONGSON2E.
 */
#[cfg(CONFIG_CPU_LOONGSON2E)]
pub const LOONGSON_HIGHMEM_START: u32 = 0x20000000;
#[cfg(not(CONFIG_CPU_LOONGSON2E))]
pub const LOONGSON_HIGHMEM_START: u32 = 0x90000000;

/*
 * the peripheral registers(MMIO):
 *
 * On the Lemote Loongson 2e system, reside between 0x1000:0000 and 0x2000:0000.
 * On the Lemote Loongson 2f system, reside between 0x1000:0000 and 0x8000:0000.
 */

pub const LOONGSON_MMIO_MEM_START: u32 = 0x10000000;

#[cfg(CONFIG_CPU_LOONGSON2E)]
pub const LOONGSON_MMIO_MEM_END: u32 = 0x20000000;
#[cfg(not(CONFIG_CPU_LOONGSON2E))]
pub const LOONGSON_MMIO_MEM_END: u32 = 0x80000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
