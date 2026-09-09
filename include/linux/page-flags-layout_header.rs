/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/numa.h and generated/bounds.h

/*
 * When a memory allocation must conform to specific limitations (such
 * as being suitable for DMA) the caller will pass in hints to the
 * allocator in the gfp_mask, in the zone modifier bits.  These bits are
 * used to select a priority ordered list of memory zones which match the
 * requested limits. See gfp_zone() in include/linux/gfp.h
 */
pub const ZONES_SHIFT: usize = if MAX_NR_ZONES < 2 {
    0
} else if MAX_NR_ZONES <= 2 {
    1
} else if MAX_NR_ZONES <= 4 {
    2
} else if MAX_NR_ZONES <= 8 {
    3
} else {
    // C preprocessing emits an error when too many zones are configured.
    0
};
pub const ZONES_WIDTH: usize = ZONES_SHIFT;

// CONFIG_SPARSEMEM selects the following nonzero section shift.
#[cfg(CONFIG_SPARSEMEM)]
pub const SECTIONS_SHIFT: usize = MAX_PHYSMEM_BITS - SECTION_SIZE_BITS;
#[cfg(not(CONFIG_SPARSEMEM))]
pub const SECTIONS_SHIFT: usize = 0;

// BUILD_VDSO32_64 excludes the page flag layout definitions below.
#[cfg(not(BUILD_VDSO32_64))]
pub mod page_flags_layout {
    // CONFIG_SPARSEMEM without CONFIG_SPARSEMEM_VMEMMAP selects this width.
    #[cfg(all(CONFIG_SPARSEMEM, not(CONFIG_SPARSEMEM_VMEMMAP)))]
    pub const SECTIONS_WIDTH: usize = super::SECTIONS_SHIFT;
    #[cfg(not(all(CONFIG_SPARSEMEM, not(CONFIG_SPARSEMEM_VMEMMAP))))]
    pub const SECTIONS_WIDTH: usize = 0;

    pub const NODES_WIDTH: usize = if ZONES_WIDTH + LRU_GEN_WIDTH + SECTIONS_WIDTH
        <= BITS_PER_LONG - NR_PAGEFLAGS
    {
        NODES_SHIFT
    } else {
        // CONFIG_SPARSEMEM_VMEMMAP emits a preprocessing error here.
        0
    };

    // Defined only when the node field cannot fit in page flags.
    #[cfg(all(NODES_SHIFT != 0, NODES_WIDTH == 0))]
    pub const NODE_NOT_IN_PAGE_FLAGS: usize = 1;

    #[cfg(CONFIG_KASAN_SW_TAGS)]
    pub const KASAN_TAG_WIDTH: usize = 8;
    #[cfg(all(not(CONFIG_KASAN_SW_TAGS), CONFIG_KASAN_HW_TAGS))]
    pub const KASAN_TAG_WIDTH: usize = 4;
    #[cfg(all(not(CONFIG_KASAN_SW_TAGS), not(CONFIG_KASAN_HW_TAGS)))]
    pub const KASAN_TAG_WIDTH: usize = 0;

    #[cfg(CONFIG_NUMA_BALANCING)]
    pub const LAST__PID_SHIFT: usize = 8;
    #[cfg(CONFIG_NUMA_BALANCING)]
    pub const LAST__PID_MASK: usize = (1usize << LAST__PID_SHIFT) - 1;
    #[cfg(CONFIG_NUMA_BALANCING)]
    pub const LAST__CPU_SHIFT: usize = NR_CPUS_BITS;
    #[cfg(CONFIG_NUMA_BALANCING)]
    pub const LAST__CPU_MASK: usize = (1usize << LAST__CPU_SHIFT) - 1;
    #[cfg(CONFIG_NUMA_BALANCING)]
    pub const LAST_CPUPID_SHIFT: usize = LAST__PID_SHIFT + LAST__CPU_SHIFT;
    #[cfg(not(CONFIG_NUMA_BALANCING))]
    pub const LAST_CPUPID_SHIFT: usize = 0;

    pub const LAST_CPUPID_WIDTH: usize = if ZONES_WIDTH + LRU_GEN_WIDTH + SECTIONS_WIDTH
        + NODES_WIDTH + KASAN_TAG_WIDTH + LAST_CPUPID_SHIFT
        <= BITS_PER_LONG - NR_PAGEFLAGS
    {
        LAST_CPUPID_SHIFT
    } else {
        0
    };

    // Defined only when the last-cpupid field cannot fit in page flags.
    #[cfg(all(LAST_CPUPID_SHIFT != 0, LAST_CPUPID_WIDTH == 0))]
    pub const LAST_CPUPID_NOT_IN_PAGE_FLAGS: bool = true;

    /* see the comment on MAX_NR_TIERS */
    pub const LRU_REFS_WIDTH: usize = {
        let available = BITS_PER_LONG - NR_PAGEFLAGS - ZONES_WIDTH - LRU_GEN_WIDTH
            - SECTIONS_WIDTH - NODES_WIDTH - KASAN_TAG_WIDTH - LAST_CPUPID_WIDTH;
        if __LRU_REFS_WIDTH < available { __LRU_REFS_WIDTH } else { available }
    };

    pub const NR_NON_PAGEFLAG_BITS: usize = SECTIONS_WIDTH + NODES_WIDTH + ZONES_WIDTH
        + LAST_CPUPID_SHIFT + KASAN_TAG_WIDTH + LRU_GEN_WIDTH + LRU_REFS_WIDTH;
    pub const NR_UNUSED_PAGEFLAG_BITS: usize =
        BITS_PER_LONG - (NR_NON_PAGEFLAG_BITS + NR_PAGEFLAGS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
