/* SPDX-License-Identifier: GPL-2.0 */

/*
 * include/linux/platform_data/ams-delta-fiq.h
 *
 * Taken from the original Amstrad modifications to fiq.h
 *
 * Copyright (c) 2004 Amstrad Plc
 * Copyright (c) 2006 Matt Callow
 * Copyright (c) 2010 Janusz Krzysztofik
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

/*
 * These are the offsets from the beginning of the fiq_buffer. They are put here
 * since the buffer and header need to be accessed by drivers servicing devices
 * which generate GPIO interrupts - e.g. keyboard, modem, hook switch.
 */
pub const FIQ_MASK: usize = 0;
pub const FIQ_STATE: usize = 1;
pub const FIQ_KEYS_CNT: usize = 2;
pub const FIQ_TAIL_OFFSET: usize = 3;
pub const FIQ_HEAD_OFFSET: usize = 4;
pub const FIQ_BUF_LEN: usize = 5;
pub const FIQ_KEY: usize = 6;
pub const FIQ_MISSED_KEYS: usize = 7;
pub const FIQ_BUFFER_START: usize = 8;
pub const FIQ_GPIO_INT_MASK: usize = 9;
pub const FIQ_KEYS_HICNT: usize = 10;
pub const FIQ_IRQ_PEND: usize = 11;
pub const FIQ_SIR_CODE_L1: usize = 12;
pub const IRQ_SIR_CODE_L2: usize = 13;

pub const FIQ_CNT_INT_00: usize = 14;
pub const FIQ_CNT_INT_KEY: usize = 15;
pub const FIQ_CNT_INT_MDM: usize = 16;
pub const FIQ_CNT_INT_03: usize = 17;
pub const FIQ_CNT_INT_HSW: usize = 18;
pub const FIQ_CNT_INT_05: usize = 19;
pub const FIQ_CNT_INT_06: usize = 20;
pub const FIQ_CNT_INT_07: usize = 21;
pub const FIQ_CNT_INT_08: usize = 22;
pub const FIQ_CNT_INT_09: usize = 23;
pub const FIQ_CNT_INT_10: usize = 24;
pub const FIQ_CNT_INT_11: usize = 25;
pub const FIQ_CNT_INT_12: usize = 26;
pub const FIQ_CNT_INT_13: usize = 27;
pub const FIQ_CNT_INT_14: usize = 28;
pub const FIQ_CNT_INT_15: usize = 29;

pub const FIQ_CIRC_BUFF: usize = 30; /*Start of circular buffer */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
