/* SPDX-License-Identifier: GPL-2.0 */

// UltraSparc-III Data Cache Unit Control Register
pub const DCU_CP: u64 = 0x0002_0000_0000_0000; // Phys Cache Enable w/o mmu
pub const DCU_CV: u64 = 0x0001_0000_0000_0000; // Virt Cache Enable w/o mmu
pub const DCU_ME: u64 = 0x0000_8000_0000_0000; // NC-store Merging Enable
pub const DCU_RE: u64 = 0x0000_4000_0000_0000; // RAW bypass Enable
pub const DCU_PE: u64 = 0x0000_2000_0000_0000; // PCache Enable
pub const DCU_HPE: u64 = 0x0000_1000_0000_0000; // HW prefetch Enable
pub const DCU_SPE: u64 = 0x0000_0800_0000_0000; // SW prefetch Enable
pub const DCU_SL: u64 = 0x0000_0400_0000_0000; // Secondary ld-steering Enab
pub const DCU_WE: u64 = 0x0000_0200_0000_0000; // WCache enable
pub const DCU_PM: u64 = 0x0000_01fe_0000_0000; // PA Watchpoint Byte Mask
pub const DCU_VM: u64 = 0x0000_0001_fe00_0000; // VA Watchpoint Byte Mask
pub const DCU_PR: u64 = 0x0000_0000_0100_0000; // PA Watchpoint Read Enable
pub const DCU_PW: u64 = 0x0000_0000_0080_0000; // PA Watchpoint Write Enable
pub const DCU_VR: u64 = 0x0000_0000_0040_0000; // VA Watchpoint Read Enable
pub const DCU_VW: u64 = 0x0000_0000_0020_0000; // VA Watchpoint Write Enable
pub const DCU_DM: u64 = 0x0000_0000_0000_0008; // DMMU Enable
pub const DCU_IM: u64 = 0x0000_0000_0000_0004; // IMMU Enable
pub const DCU_DC: u64 = 0x0000_0000_0000_0002; // Data Cache Enable
pub const DCU_IC: u64 = 0x0000_0000_0000_0001; // Instruction Cache Enable

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
