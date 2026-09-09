/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020 ARM Ltd.
 */

// The original header guard was: __ASM_MTE_DEF_H

pub const MTE_GRANULE_SIZE: usize = 16;
pub const MTE_GRANULE_MASK: usize = !(MTE_GRANULE_SIZE - 1);
pub const MTE_GRANULES_PER_PAGE: usize = PAGE_SIZE / MTE_GRANULE_SIZE;
pub const MTE_TAG_SHIFT: usize = 56;
pub const MTE_TAG_SIZE: usize = 4;
pub const MTE_TAG_MASK: u64 = (((1u64 << (MTE_TAG_SHIFT + (MTE_TAG_SIZE - 1) + 1)) - 1)
    & !((1u64 << MTE_TAG_SHIFT) - 1));
pub const MTE_PAGE_TAG_STORAGE: usize = MTE_GRANULES_PER_PAGE * MTE_TAG_SIZE / 8;

pub const __MTE_PREAMBLE: &str = concat!(ARM64_ASM_PREAMBLE, ".arch_extension memtag\n");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
