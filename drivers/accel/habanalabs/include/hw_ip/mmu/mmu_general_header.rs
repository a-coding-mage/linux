/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2020 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

pub const PAGE_SHIFT_4KB: u32 = 12;
pub const PAGE_SHIFT_64KB: u32 = 16;
pub const PAGE_SHIFT_2MB: u32 = 21;
pub const PAGE_SHIFT_16MB: u32 = 24;
pub const PAGE_SHIFT_64MB: u32 = 26;
pub const PAGE_SHIFT_1GB: u32 = 30;
pub const PAGE_SIZE_4KB: u64 = 1u64 << PAGE_SHIFT_4KB;
pub const PAGE_SIZE_64KB: u64 = 1u64 << PAGE_SHIFT_64KB;
pub const PAGE_SIZE_2MB: u64 = 1u64 << PAGE_SHIFT_2MB;
pub const PAGE_SIZE_16MB: u64 = 1u64 << PAGE_SHIFT_16MB;
pub const PAGE_SIZE_64MB: u64 = 1u64 << PAGE_SHIFT_64MB;
pub const PAGE_SIZE_1GB: u64 = 1u64 << PAGE_SHIFT_1GB;

pub const PAGE_PRESENT_MASK: u64 = 0x0000000000001;
pub const SWAP_OUT_MASK: u64 = 0x0000000000004;
pub const LAST_MASK: u64 = 0x0000000000800;
pub const FLAGS_MASK: u64 = 0x0000000000FFF;

pub const MMU_ARCH_3_HOPS: u32 = 3;
pub const MMU_ARCH_4_HOPS: u32 = 4;
pub const MMU_ARCH_5_HOPS: u32 = 5;
pub const MMU_ARCH_6_HOPS: u32 = 6;

pub const HOP_PHYS_ADDR_MASK: u64 = !FLAGS_MASK;

pub const HL_PTE_SIZE: usize = core::mem::size_of::<u64>();

/* definitions for HOP with 512 PTE entries */
pub const HOP_PTE_ENTRIES_512: usize = 512;
pub const HOP_TABLE_SIZE_512_PTE: usize = HOP_PTE_ENTRIES_512 * HL_PTE_SIZE;
pub const HOP0_512_PTE_TABLES_TOTAL_SIZE: usize = HOP_TABLE_SIZE_512_PTE * MAX_ASID;

pub const MMU_HOP0_PA43_12_SHIFT: u32 = 12;
pub const MMU_HOP0_PA49_44_SHIFT: u32 = 12 + 32;
pub const MMU_HOP0_PA63_44_SHIFT: u32 = 12 + 32;

pub const MMU_CONFIG_TIMEOUT_USEC: u32 = 2000; /* 2 ms */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mmu_hop_num {
    MMU_HOP0,
    MMU_HOP1,
    MMU_HOP2,
    MMU_HOP3,
    MMU_HOP4,
    MMU_HOP5,
    MMU_HOP_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
