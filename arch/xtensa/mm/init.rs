/*
 * arch/xtensa/mm/init.c
 *
 * Derived from MIPS, PPC.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 * Copyright (C) 2014 - 2016 Cadence Design Systems Inc.
 *
 * Chris Zankel\t<chris@zankel.net>
 * Joe Taylor\t<joe@tensilica.com, joetylr@yahoo.com>
 * Marc Gauthier
 * Kevin Chea
 */

// Linux and Xtensa dependencies supplied by other translation units.

/*
 * Initialize the bootmem system and give it all low memory we have available.
 */
pub unsafe fn bootmem_init() {
    /* Reserve all memory below PHYS_OFFSET, as memory
     * accounting doesn't work for pages below that address.
     *
     * If PHYS_OFFSET is zero reserve page at address 0:
     * successfull allocations should never return NULL.
     */
    memblock_reserve(0, if PHYS_OFFSET != 0 { PHYS_OFFSET } else { 1 });

    early_init_fdt_scan_reserved_mem();

    if memblock_phys_mem_size() == 0 {
        panic!("No memory found!\n");
    }

    min_low_pfn = PFN_UP(memblock_start_of_DRAM());
    min_low_pfn = max(min_low_pfn, PFN_UP(PHYS_OFFSET));
    max_pfn = PFN_DOWN(memblock_end_of_DRAM());
    max_low_pfn = min(max_pfn, MAX_LOW_PFN);

    early_memtest(
        (min_low_pfn as phys_addr_t) << PAGE_SHIFT,
        (max_low_pfn as phys_addr_t) << PAGE_SHIFT,
    );

    memblock_set_current_limit(PFN_PHYS(max_low_pfn));
    dma_contiguous_reserve(PFN_PHYS(max_low_pfn));

    memblock_dump_all();
}

unsafe fn print_vm_layout() {
    pr_info!("virtual kernel memory layout:\n");

    #[cfg(CONFIG_KASAN)]
    pr_info!(
        "    kasan   : 0x{:08x} - 0x{:08x}  ({:5} MB)\n",
        KASAN_SHADOW_START,
        KASAN_SHADOW_START + KASAN_SHADOW_SIZE,
        KASAN_SHADOW_SIZE >> 20,
    );

    #[cfg(CONFIG_MMU)]
    pr_info!(
        "    vmalloc : 0x{:08x} - 0x{:08x}  ({:5} MB)\n",
        VMALLOC_START,
        VMALLOC_END,
        (VMALLOC_END - VMALLOC_START) >> 20,
    );

    #[cfg(CONFIG_HIGHMEM)]
    {
        pr_info!(
            "    pkmap   : 0x{:08x} - 0x{:08x}  ({:5} kB)\n",
            PKMAP_BASE,
            PKMAP_BASE + LAST_PKMAP * PAGE_SIZE,
            (LAST_PKMAP * PAGE_SIZE) >> 10,
        );
        pr_info!(
            "    fixmap  : 0x{:08x} - 0x{:08x}  ({:5} kB)\n",
            FIXADDR_START,
            FIXADDR_END,
            (FIXADDR_END - FIXADDR_START) >> 10,
        );
    }

    #[cfg(CONFIG_MMU)]
    let lowmem_start = PAGE_OFFSET;
    #[cfg(CONFIG_MMU)]
    let lowmem_end = PAGE_OFFSET + (max_low_pfn - min_low_pfn) * PAGE_SIZE;
    #[cfg(not(CONFIG_MMU))]
    let lowmem_start = min_low_pfn * PAGE_SIZE;
    #[cfg(not(CONFIG_MMU))]
    let lowmem_end = max_low_pfn * PAGE_SIZE;

    pr_info!(
        "    lowmem  : 0x{:08x} - 0x{:08x}  ({:5} MB)\n",
        lowmem_start,
        lowmem_end,
        ((max_low_pfn - min_low_pfn) * PAGE_SIZE) >> 20,
    );
    pr_info!("    .text   : 0x{:08x} - 0x{:08x}  ({:5} kB)\n", _text as usize, _etext as usize, (_etext as usize - _text as usize) >> 10);
    pr_info!("    .rodata : 0x{:08x} - 0x{:08x}  ({:5} kB)\n", __start_rodata as usize, __end_rodata as usize, (__end_rodata as usize - __start_rodata as usize) >> 10);
    pr_info!("    .data   : 0x{:08x} - 0x{:08x}  ({:5} kB)\n", _sdata as usize, _edata as usize, (_edata as usize - _sdata as usize) >> 10);
    pr_info!("    .init   : 0x{:08x} - 0x{:08x}  ({:5} kB)\n", __init_begin as usize, __init_end as usize, (__init_end as usize - __init_begin as usize) >> 10);
    pr_info!("    .bss    : 0x{:08x} - 0x{:08x}  ({:5} kB)\n", __bss_start as usize, __bss_stop as usize, (__bss_stop as usize - __bss_start as usize) >> 10);
}

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) {
    *max_zone_pfns.add(ZONE_NORMAL) = max_low_pfn;
    #[cfg(CONFIG_HIGHMEM)]
    {
        *max_zone_pfns.add(ZONE_HIGHMEM) = max_pfn;
    }
}

pub unsafe fn zones_init() {
    print_vm_layout();
}

unsafe fn parse_memmap_one(mut p: *mut c_char) {
    let mut oldp: *mut c_char;
    let mut start_at: c_ulong;
    let mut mem_size: c_ulong;

    if p.is_null() {
        return;
    }

    oldp = p;
    mem_size = memparse(p, &mut p);
    if p == oldp {
        return;
    }

    match *p as u8 {
        b'@' => {
            start_at = memparse(p.add(1), &mut p);
            memblock_add(start_at, mem_size);
        }
        b'$' => {
            start_at = memparse(p.add(1), &mut p);
            memblock_reserve(start_at, mem_size);
        }
        0 => {
            memblock_reserve(mem_size, 0u64.wrapping_sub(mem_size));
        }
        _ => {
            pr_warn!("Unrecognized memmap syntax: %s\n", p);
        }
    }
}

unsafe fn parse_memmap_opt(mut str_: *mut c_char) -> c_int {
    while !str_.is_null() {
        let mut k = strchr(str_, b',' as c_int);

        if !k.is_null() {
            *k = 0;
            k = k.add(1);
        }

        parse_memmap_one(str_);
        str_ = k;
    }

    0
}

// early_param("memmap", parse_memmap_opt);

#[cfg(CONFIG_MMU)]
static protection_map: [pgprot_t; 16] = [
    [VM_NONE] = PAGE_NONE,
    [VM_READ] = PAGE_READONLY,
    [VM_WRITE] = PAGE_COPY,
    [VM_WRITE | VM_READ] = PAGE_COPY,
    [VM_EXEC] = PAGE_READONLY_EXEC,
    [VM_EXEC | VM_READ] = PAGE_READONLY_EXEC,
    [VM_EXEC | VM_WRITE] = PAGE_COPY_EXEC,
    [VM_EXEC | VM_WRITE | VM_READ] = PAGE_COPY_EXEC,
    [VM_SHARED] = PAGE_NONE,
    [VM_SHARED | VM_READ] = PAGE_READONLY,
    [VM_SHARED | VM_WRITE] = PAGE_SHARED,
    [VM_SHARED | VM_WRITE | VM_READ] = PAGE_SHARED,
    [VM_SHARED | VM_EXEC] = PAGE_READONLY_EXEC,
    [VM_SHARED | VM_EXEC | VM_READ] = PAGE_READONLY_EXEC,
    [VM_SHARED | VM_EXEC | VM_WRITE] = PAGE_SHARED_EXEC,
    [VM_SHARED | VM_EXEC | VM_WRITE | VM_READ] = PAGE_SHARED_EXEC,
];

// DECLARE_VM_GET_PAGE_PROT

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
