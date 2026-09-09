/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2019 Andes Technology Corporation */

/* C header guard: __ASM_KASAN_H */
/* C-only assembler exclusion: __ASSEMBLER__ */

/*
 * The following comment was copied from arm64:
 * KASAN_SHADOW_START: beginning of the kernel virtual addresses.
 * KASAN_SHADOW_END: KASAN_SHADOW_START + 1/N of kernel virtual addresses,
 * where N = (1 << KASAN_SHADOW_SCALE_SHIFT).
 *
 * KASAN_SHADOW_OFFSET:
 * This value is used to map an address to the corresponding shadow
 * address by the following formula:
 *     shadow_addr = (address >> KASAN_SHADOW_SCALE_SHIFT) + KASAN_SHADOW_OFFSET
 *
 * (1 << (64 - KASAN_SHADOW_SCALE_SHIFT)) shadow addresses that lie in range
 * [KASAN_SHADOW_OFFSET, KASAN_SHADOW_END) cover all 64-bits of virtual
 * addresses. So KASAN_SHADOW_OFFSET should satisfy the following equation:
 *      KASAN_SHADOW_OFFSET = KASAN_SHADOW_END -
 *                              (1ULL << (64 - KASAN_SHADOW_SCALE_SHIFT))
 */
pub const KASAN_SHADOW_SCALE_SHIFT: usize = 3;

/* External dependency supplied by the surrounding kernel translation. */
pub const KASAN_SHADOW_SIZE: usize =
    (1usize << ((VA_BITS - 1) - KASAN_SHADOW_SCALE_SHIFT));

/*
 * Depending on the size of the virtual address space, the region may not be
 * aligned on PGDIR_SIZE, so force its alignment to ease its population.
 */
pub const KASAN_SHADOW_START: usize =
    (KASAN_SHADOW_END - KASAN_SHADOW_SIZE) & PGDIR_MASK;
pub const KASAN_SHADOW_END: usize = MODULES_LOWEST_VADDR;

/* CONFIG_KASAN conditional declarations preserved from the C header. */
#[cfg(CONFIG_KASAN)]
pub const KASAN_SHADOW_OFFSET: usize = CONFIG_KASAN_SHADOW_OFFSET;

#[cfg(CONFIG_KASAN)]
unsafe extern "C" {
    pub fn kasan_init();
    /* `asmlinkage` is an ABI annotation in the source kernel environment. */
    pub fn kasan_early_init();
    pub fn kasan_swapper_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
