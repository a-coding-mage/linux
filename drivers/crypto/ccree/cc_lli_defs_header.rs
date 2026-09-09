/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

// Dependency supplied by the surrounding kernel/Rust environment:
// `dma_addr_t`.

pub const DLLI_SIZE_BIT_SIZE: u32 = 0x18;

pub const CC_MAX_MLLI_ENTRY_SIZE: u32 = 0xFFFF;

pub const LLI_MAX_NUM_OF_DATA_ENTRIES: usize = 128;
pub const LLI_MAX_NUM_OF_ASSOC_DATA_ENTRIES: usize = 8;
pub const MLLI_TABLE_MIN_ALIGNMENT: usize = 4; // 32 bit alignment
pub const MAX_NUM_OF_BUFFERS_IN_MLLI: usize = 4;
pub const MAX_NUM_OF_TOTAL_MLLI_ENTRIES: usize =
    2 * LLI_MAX_NUM_OF_DATA_ENTRIES + LLI_MAX_NUM_OF_ASSOC_DATA_ENTRIES;

/* Size of entry */
pub const LLI_ENTRY_WORD_SIZE: usize = 2;
pub const LLI_ENTRY_BYTE_SIZE: usize = LLI_ENTRY_WORD_SIZE * core::mem::size_of::<u32>();

/* Word0[31:0] = ADDR[31:0] */
pub const LLI_WORD0_OFFSET: usize = 0;
pub const LLI_LADDR_BIT_OFFSET: u32 = 0;
pub const LLI_LADDR_BIT_SIZE: u32 = 32;
/* Word1[31:16] = ADDR[47:32]; Word1[15:0] = SIZE */
pub const LLI_WORD1_OFFSET: usize = 1;
pub const LLI_SIZE_BIT_OFFSET: u32 = 0;
pub const LLI_SIZE_BIT_SIZE: u32 = 16;
pub const LLI_HADDR_BIT_OFFSET: u32 = 16;
pub const LLI_HADDR_BIT_SIZE: u32 = 16;

pub const LLI_SIZE_MASK: u32 = 0xffff;
pub const LLI_HADDR_MASK: u32 = 0xffff0000;

#[inline]
pub unsafe fn cc_lli_set_addr(lli_p: *mut u32, addr: dma_addr_t) {
    *lli_p.add(LLI_WORD0_OFFSET) = (addr & u32::MAX as dma_addr_t) as u32;

    // CONFIG_ARCH_DMA_ADDR_T_64BIT is a build-time kernel condition.
    #[cfg(CONFIG_ARCH_DMA_ADDR_T_64BIT)]
    {
        let word1 = lli_p.add(LLI_WORD1_OFFSET);
        *word1 &= !LLI_HADDR_MASK;
        *word1 |= ((addr >> 32) as u32) << LLI_HADDR_BIT_OFFSET;
    }
}

#[inline]
pub unsafe fn cc_lli_set_size(lli_p: *mut u32, size: u16) {
    let word1 = lli_p.add(LLI_WORD1_OFFSET);
    *word1 &= !LLI_SIZE_MASK;
    *word1 |= (size as u32) << LLI_SIZE_BIT_OFFSET;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
