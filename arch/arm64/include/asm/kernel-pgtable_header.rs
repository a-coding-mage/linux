/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Kernel page table mapping
 *
 * Copyright (C) 2015 ARM Ltd.
 */

// Dependencies supplied by the corresponding architecture headers are
// intentionally left as external names.

/*
 * The physical and virtual addresses of the start of the kernel image are
 * equal modulo 2 MiB (per the arm64 booting.txt requirements). Hence we can
 * use section mapping with 4K (section size = 2M) but not with 16K (section
 * size = 32M) or 64K (section size = 512M).
 */
pub const SWAPPER_BLOCK_SHIFT: usize = if PMD_SIZE <= MIN_KIMG_ALIGN {
    PMD_SHIFT
} else {
    PAGE_SHIFT
};
pub const SWAPPER_SKIP_LEVEL: usize = if PMD_SIZE <= MIN_KIMG_ALIGN { 1 } else { 0 };
pub const SWAPPER_BLOCK_SIZE: usize = 1usize << SWAPPER_BLOCK_SHIFT;

pub const SWAPPER_PGTABLE_LEVELS: usize = CONFIG_PGTABLE_LEVELS - SWAPPER_SKIP_LEVEL;
pub const INIT_IDMAP_PGTABLE_LEVELS: usize = IDMAP_LEVELS - SWAPPER_SKIP_LEVEL;

pub const IDMAP_VA_BITS: usize = 48;
pub const IDMAP_LEVELS: usize = ARM64_HW_PGTABLE_LEVELS(IDMAP_VA_BITS);
pub const IDMAP_ROOT_LEVEL: usize = 4 - IDMAP_LEVELS;

/* A relocatable kernel may require an additional page table at each level. */
// __is_defined(CONFIG_RELOCATABLE), represented as a build configuration.
pub const EXTRA_PAGE: usize = if cfg!(feature = "CONFIG_RELOCATABLE") { 1 } else { 0 };

pub const fn span_nr_entries(vstart: usize, vend: usize, shift: usize) -> usize {
    ((vend - 1) >> shift) - (vstart >> shift) + 1
}

pub const fn early_entries(lvl: usize, vstart: usize, vend: usize) -> usize {
    span_nr_entries(vstart, vend, SWAPPER_BLOCK_SHIFT + lvl * PTDESC_TABLE_SHIFT)
}

pub const fn early_level(
    lvl: usize,
    lvls: usize,
    vstart: usize,
    vend: usize,
    add: usize,
) -> usize {
    if lvls > lvl {
        early_entries(lvl, vstart, vend) + add
    } else {
        0
    }
}

pub const fn early_pages(lvls: usize, vstart: usize, vend: usize, add: usize) -> usize {
    1 /* PGDIR page */
        + early_level(3, lvls, vstart, vend, add)
        + early_level(2, lvls, vstart, vend, add)
        + early_level(1, lvls, vstart, vend, add)
}

pub const INIT_DIR_SIZE: usize = PAGE_SIZE
    * (early_pages(SWAPPER_PGTABLE_LEVELS, KIMAGE_VADDR, _end, EXTRA_PAGE)
        + EARLY_SEGMENT_EXTRA_PAGES);

pub const INIT_IDMAP_DIR_PAGES: usize =
    early_pages(INIT_IDMAP_PGTABLE_LEVELS, KIMAGE_VADDR, kimage_limit, 1);
pub const INIT_IDMAP_DIR_SIZE: usize =
    (INIT_IDMAP_DIR_PAGES + EARLY_IDMAP_EXTRA_PAGES) * PAGE_SIZE;

pub const INIT_IDMAP_FDT_PAGES: usize =
    early_pages(INIT_IDMAP_PGTABLE_LEVELS, 0, MAX_FDT_SIZE, 1) - 1;
pub const INIT_IDMAP_FDT_SIZE: usize =
    (INIT_IDMAP_FDT_PAGES + EARLY_IDMAP_EXTRA_FDT_PAGES) * PAGE_SIZE;

/* The number of segments in the kernel image (text, rodata, inittext, initdata, data+bss) */
pub const KERNEL_SEGMENT_COUNT: usize = 5;

/* These values depend on the build-time SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN condition. */
pub const EARLY_SEGMENT_EXTRA_PAGES: usize = if SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN {
    KERNEL_SEGMENT_COUNT + 2
} else {
    0
};
pub const EARLY_IDMAP_EXTRA_PAGES: usize = if SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN { 3 } else { 0 };
pub const EARLY_IDMAP_EXTRA_FDT_PAGES: usize =
    if SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN { 2 } else { 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
