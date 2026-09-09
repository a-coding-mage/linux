/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Cache operations for the cache instruction.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/*
 * Most cache ops are split into a 3 bit field identifying the cache, and a 2
 * bit field identifying the cache operation.
 */
pub const CACHE_OP_CACHE: u32 = 0x07;
pub const CACHE_OP_OP: u32 = 0x18;

pub const CACHE_LEAF0: u32 = 0x00;
pub const CACHE_LEAF1: u32 = 0x01;
pub const CACHE_LEAF2: u32 = 0x02;
pub const CACHE_LEAF3: u32 = 0x03;
pub const CACHE_LEAF4: u32 = 0x04;
pub const CACHE_LEAF5: u32 = 0x05;

pub const INDEX_INVALIDATE: u32 = 0x08;
pub const INDEX_WRITEBACK_INV: u32 = 0x08;
pub const HIT_INVALIDATE: u32 = 0x10;
pub const HIT_WRITEBACK_INV: u32 = 0x10;
pub const CACHE_OP_USER_DEFINED: u32 = 0x18;

pub const INDEX_WRITEBACK_INV_LEAF0: u32 = CACHE_LEAF0 | INDEX_WRITEBACK_INV;
pub const INDEX_WRITEBACK_INV_LEAF1: u32 = CACHE_LEAF1 | INDEX_WRITEBACK_INV;
pub const INDEX_WRITEBACK_INV_LEAF2: u32 = CACHE_LEAF2 | INDEX_WRITEBACK_INV;
pub const INDEX_WRITEBACK_INV_LEAF3: u32 = CACHE_LEAF3 | INDEX_WRITEBACK_INV;
pub const INDEX_WRITEBACK_INV_LEAF4: u32 = CACHE_LEAF4 | INDEX_WRITEBACK_INV;
pub const INDEX_WRITEBACK_INV_LEAF5: u32 = CACHE_LEAF5 | INDEX_WRITEBACK_INV;
pub const HIT_WRITEBACK_INV_LEAF0: u32 = CACHE_LEAF0 | HIT_WRITEBACK_INV;
pub const HIT_WRITEBACK_INV_LEAF1: u32 = CACHE_LEAF1 | HIT_WRITEBACK_INV;
pub const HIT_WRITEBACK_INV_LEAF2: u32 = CACHE_LEAF2 | HIT_WRITEBACK_INV;
pub const HIT_WRITEBACK_INV_LEAF3: u32 = CACHE_LEAF3 | HIT_WRITEBACK_INV;
pub const HIT_WRITEBACK_INV_LEAF4: u32 = CACHE_LEAF4 | HIT_WRITEBACK_INV;
pub const HIT_WRITEBACK_INV_LEAF5: u32 = CACHE_LEAF5 | HIT_WRITEBACK_INV;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
