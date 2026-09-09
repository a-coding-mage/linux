// SPDX-License-Identifier: GPL-2.0-only
/*
 * Memory subsystem initialization for Hexagon
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

// Define a startpg just past the end of the kernel image and a lastpg
// that corresponds to the end of real or simulated platform memory.
// #define bootmem_startpg (PFN_UP(((unsigned long) _end) - PAGE_OFFSET + PHYS_OFFSET))

pub static mut bootmem_lastpg: c_ulong = 0; // Should be set by platform code
pub static mut __phys_offset: c_ulong = 0; // physical kernel offset >> 12

// Set as variable to limit PMD copies
pub static mut max_kernel_seg: c_int = 0x303;

// indicate pfn's of high memory
pub static mut highstart_pfn: c_ulong = 0;
pub static mut highend_pfn: c_ulong = 0;

// Default cache attribute for newly created page tables
pub static mut _dflt_cache_att: c_ulong = CACHEDEF;

/*
 * The current "generation" of kernel map, which should not roll
 * over until Hell freezes over.  Actual bound in years needs to be
 * calculated to confirm.
 */
pub static mut kmap_gen_lock: spinlock_t = DEFINE_SPINLOCK();

// checkpatch says don't init this to 0.
pub static mut kmap_generation: c_ulonglong = 0;

pub unsafe fn sync_icache_dcache(pte: pte_t) {
    let page: *mut page = pte_page(pte);
    let addr: c_ulong = page_address(page) as c_ulong;

    __vmcache_idsync(addr, PAGE_SIZE);
}

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) {
    /*
     * This is not particularly well documented anywhere, but
     * give ZONE_NORMAL all the memory, including the big holes
     * left by the kernel+bootmem_map which are already left as reserved
     * in the bootmem_map; free_area_init should see those bits and
     * adjust accordingly.
     */
    *max_zone_pfns.add(ZONE_NORMAL) = max_low_pfn;
}

unsafe fn paging_init() {
    /*
     * Set the init_mm descriptors "context" value to point to the
     * initial kernel segment table's physical address.
     */
    init_mm.context.ptbase = __pa(init_mm.pgd);
}

// #ifndef DMA_RESERVE
pub const DMA_RESERVE: c_ulong = 4;
// #endif

pub const DMA_CHUNKSIZE: c_ulong = 1 << 22;
pub const DMA_RESERVED_BYTES: c_ulong = DMA_RESERVE * DMA_CHUNKSIZE;

/*
 * Pick out the memory size.  We look for mem=size,
 * where size is "size[KkMm]"
 */
unsafe fn early_mem(p: *mut c_char) -> c_int {
    let mut endp: *mut c_char = core::ptr::null_mut();
    let size: c_ulong = memparse(p, &mut endp);

    bootmem_lastpg = PFN_DOWN(size);

    0
}

// early_param("mem", early_mem);

pub static mut hexagon_coherent_pool_size: usize = (DMA_RESERVE << 22) as usize;

pub unsafe fn setup_arch_memory() {
    /* XXX Todo: this probably should be cleaned up */
    let mut segtable: *mut u32 = &mut swapper_pg_dir[0] as *mut _ as *mut u32;
    let segtable_end: *mut u32;

    /*
     * Set up boot memory allocator
     *
     * The Gorman book also talks about these functions.
     * This needs to change for highmem setups.
     */

    // Prior to this, bootmem_lastpg is actually mem size
    bootmem_lastpg = bootmem_lastpg.wrapping_add(ARCH_PFN_OFFSET);

    // Memory size needs to be a multiple of 16M
    bootmem_lastpg = PFN_DOWN((bootmem_lastpg << PAGE_SHIFT) & !(BIG_KERNEL_PAGE_SIZE - 1));

    memblock_add(PHYS_OFFSET, (bootmem_lastpg - ARCH_PFN_OFFSET) << PAGE_SHIFT);

    // Reserve kernel text/data/bss
    memblock_reserve(PHYS_OFFSET, (bootmem_startpg - ARCH_PFN_OFFSET) << PAGE_SHIFT);

    /*
     * Reserve the top DMA_RESERVE bytes of RAM for DMA (uncached)
     * memory allocation
     */
    max_low_pfn = bootmem_lastpg - PFN_DOWN(DMA_RESERVED_BYTES);
    min_low_pfn = ARCH_PFN_OFFSET;
    memblock_reserve(PFN_PHYS(max_low_pfn), DMA_RESERVED_BYTES);

    printk(KERN_INFO, "bootmem_startpg:  0x%08lx\n", bootmem_startpg);
    printk(KERN_INFO, "bootmem_lastpg:  0x%08lx\n", bootmem_lastpg);
    printk(KERN_INFO, "min_low_pfn:  0x%08lx\n", min_low_pfn);
    printk(KERN_INFO, "max_low_pfn:  0x%08lx\n", max_low_pfn);

    // This is pointer arithmetic; each entry covers 4MB.
    segtable = segtable.add(PAGE_OFFSET >> 22);
    // This actually only goes to the end of the first gig.
    segtable_end = segtable.add(1 << (30 - 22));

    // Move forward to the start of empty pages; take into account phys_offset shift.
    segtable = segtable.add(((bootmem_lastpg - ARCH_PFN_OFFSET) >> (22 - PAGE_SHIFT)) as usize);
    {
        for i in 1..=DMA_RESERVE as usize {
            let p = segtable.sub(i);
            *p = (*p & __HVM_PTE_PGMASK_4MB)
                | __HVM_PTE_R | __HVM_PTE_W | __HVM_PTE_X
                | (__HEXAGON_C_UNC << 6) | __HVM_PDE_S_4MB;
        }
    }

    printk(KERN_INFO, "clearing segtable from %p to %p\n", segtable, segtable_end);
    while segtable < segtable_end.sub(8) {
        *segtable = __HVM_PDE_S_INVALID;
        segtable = segtable.add(1);
    }
    // stop the pointer at the device I/O 4MB page

    printk(KERN_INFO, "segtable = %p (should be equal to _K_io_map)\n", segtable);

    /*
    #if 0
    // Other half of the early device table from vm_init_segtable.
    printk(KERN_INFO, "&_K_init_devicetable = 0x%08x\n",
        (unsigned long) _K_init_devicetable-PAGE_OFFSET);
    *segtable = ((u32) (unsigned long) _K_init_devicetable-PAGE_OFFSET) |
        __HVM_PDE_S_4KB;
    printk(KERN_INFO, "*segtable = 0x%08x\n", *segtable);
    #endif
    */

    printk(KERN_INFO, "PAGE_SIZE=%lu\n", PAGE_SIZE);
    paging_init();
}

pub static protection_map: [pgprot_t; 16] = [
    __pgprot(_PAGE_PRESENT | _PAGE_USER | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_READ | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_READ | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_EXECUTE | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_EXECUTE | _PAGE_READ | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_EXECUTE | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_EXECUTE | _PAGE_READ | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_READ | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_WRITE | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_READ | _PAGE_WRITE | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_EXECUTE | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_EXECUTE | _PAGE_READ | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_EXECUTE | _PAGE_WRITE | CACHEDEF),
    __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_READ | _PAGE_EXECUTE | _PAGE_WRITE | CACHEDEF),
];

// DECLARE_VM_GET_PAGE_PROT

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
