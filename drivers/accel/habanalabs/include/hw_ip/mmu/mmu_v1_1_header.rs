/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// Translation of INCLUDE_MMU_V1_1_H_.

pub const MMU_V1_1_HOP0_MASK: u64 = 0x3000000000000u64;
pub const MMU_V1_1_HOP1_MASK: u64 = 0x0FF8000000000u64;
pub const MMU_V1_1_HOP2_MASK: u64 = 0x0007FC0000000u64;
pub const MMU_V1_1_HOP3_MASK: u64 = 0x000003FE00000u64;
pub const MMU_V1_1_HOP4_MASK: u64 = 0x00000001FF000u64;

pub const MMU_V1_1_HOP0_SHIFT: u32 = 48;
pub const MMU_V1_1_HOP1_SHIFT: u32 = 39;
pub const MMU_V1_1_HOP2_SHIFT: u32 = 30;
pub const MMU_V1_1_HOP3_SHIFT: u32 = 21;
pub const MMU_V1_1_HOP4_SHIFT: u32 = 12;

pub const MMU_ASID: u32 = 0xC12004;
pub const MMU_HOP0_PA43_12: u32 = 0xC12008;
pub const MMU_HOP0_PA49_44: u32 = 0xC1200C;
pub const MMU_BUSY: u32 = 0xC12000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
