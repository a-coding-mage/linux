/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// Translated from mmu_v1_0.h. The original C header guard is omitted.

pub const MMU_V1_0_HOP0_MASK: u64 = 0x3000000000000u64;
pub const MMU_V1_0_HOP1_MASK: u64 = 0x0FF8000000000u64;
pub const MMU_V1_0_HOP2_MASK: u64 = 0x0007FC0000000u64;
pub const MMU_V1_0_HOP3_MASK: u64 = 0x000003FE00000u64;
pub const MMU_V1_0_HOP4_MASK: u64 = 0x00000001FF000u64;

pub const MMU_V1_0_HOP0_SHIFT: u32 = 48;
pub const MMU_V1_0_HOP1_SHIFT: u32 = 39;
pub const MMU_V1_0_HOP2_SHIFT: u32 = 30;
pub const MMU_V1_0_HOP3_SHIFT: u32 = 21;
pub const MMU_V1_0_HOP4_SHIFT: u32 = 12;

pub const MMU_HOP0_PA43_12: u32 = 0x490004;
pub const MMU_HOP0_PA49_44: u32 = 0x490008;
pub const MMU_ASID_BUSY: u32 = 0x490000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
