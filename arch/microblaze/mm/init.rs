/*
 * Copyright (C) 2007-2008 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies are supplied by the surrounding kernel translation.

/* Use for MMU and noMMU because of PCI generic code */
pub static mut mem_init_done: i32 = 0;

pub static mut klimit: *mut u8 = unsafe { _end as *mut u8 };

/*
 * Initialize the bootmem system and give it all the memory we
 * have available.
 */
#[no_mangle]
pub static mut memory_start: c_ulong = 0;
#[no_mangle]
pub static mut memory_size: c_ulong = 0;
pub static mut lowmem_size: c_ulong = 0;

#[cfg(feature = "CONFIG_HIGHMEM")]
unsafe fn highmem_init() {
    pr_debug!("%x\n", PKMAP_BASE as u32);
    map_page(PKMAP_BASE, 0, 0); /* XXX gross */
    pkmap_page_table = virt_to_kpte(PKMAP_BASE);
}

#[no_mangle]
pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) {
    #[cfg(feature = "CONFIG_HIGHMEM")]
    {
        *max_zone_pfns.add(ZONE_DMA as usize) = max_low_pfn;
        *max_zone_pfns.add(ZONE_HIGHMEM as usize) = max_pfn;
    }
    #[cfg(not(feature = "CONFIG_HIGHMEM"))]
    {
        *max_zone_pfns.add(ZONE_DMA as usize) = max_pfn;
    }
}

/* paging_init() sets up the page tables - in fact we've already done this. */
unsafe fn paging_init() {
    let mut idx: i32 = 0;
    while idx < __end_of_fixed_addresses {
        clear_fixmap(idx);
        idx += 1;
    }

    #[cfg(feature = "CONFIG_HIGHMEM")]
    highmem_init();
}

#[no_mangle]
pub unsafe extern "C" fn setup_memory() {
    min_low_pfn = memory_start >> PAGE_SHIFT;
    max_low_pfn = ((memory_start as u64 + lowmem_size as u64) >> PAGE_SHIFT) as c_ulong;
    max_pfn = ((memory_start as u64 + memory_size as u64) >> PAGE_SHIFT) as c_ulong;

    pr_info!("%s: min_low_pfn: %#lx\n", "setup_memory", min_low_pfn);
    pr_info!("%s: max_low_pfn: %#lx\n", "setup_memory", max_low_pfn);
    pr_info!("%s: max_pfn: %#lx\n", "setup_memory", max_pfn);

    paging_init();
}

#[no_mangle]
pub unsafe extern "C" fn mem_init() {
    mem_init_done = 1;
}

#[no_mangle]
pub unsafe extern "C" fn page_is_ram(pfn: c_ulong) -> i32 {
    (pfn < max_low_pfn) as i32
}

/* Check for command-line options that affect what MMU_init will do. */
unsafe fn mm_cmdline_setup() {
    let mut maxmem: c_ulong = 0;
    let mut p: *mut u8 = cmd_line;

    p = strstr(cmd_line, b"mem=\0".as_ptr());
    if !p.is_null() {
        p = p.add(4);
        maxmem = memparse(p, &mut p);
        if maxmem != 0 && memory_size > maxmem {
            memory_size = maxmem;
            (*memblock.memory.regions).size = memory_size;
        }
    }
}

/* MMU_init_hw does the chip-specific initialization of the MMU hardware. */
unsafe fn mmu_init_hw() {
    core::arch::asm!("ori r11, r0, 0x10000000; mts rzpr, r11", out("r11") _);
}

/* MMU_init sets up the basic memory mappings for the kernel. */
#[no_mangle]
pub unsafe extern "C" fn mmu_init() {
    let kstart: c_uint;
    let ksize: c_uint;

    if (memblock.memory.regions[0].size as u32) < 0x400000 {
        pr_emerg!("Memory must be greater than 4MB\n");
        machine_restart(core::ptr::null_mut());
    }
    if (memblock.memory.regions[0].size as u32) < kernel_tlb {
        pr_emerg!("Kernel size is greater than memory node\n");
        machine_restart(core::ptr::null_mut());
    }

    memory_start = memblock.memory.regions[0].base as u32 as c_ulong;
    lowmem_size = memblock.memory.regions[0].size as u32 as c_ulong;
    memory_size = lowmem_size;

    if lowmem_size > CONFIG_LOWMEM_SIZE {
        lowmem_size = CONFIG_LOWMEM_SIZE;
        #[cfg(not(feature = "CONFIG_HIGHMEM"))]
        { memory_size = lowmem_size; }
    }

    mm_cmdline_setup();
    kstart = __pa(CONFIG_KERNEL_START);
    ksize = PAGE_ALIGN((_end as u32).wrapping_sub(CONFIG_KERNEL_START as u32));
    memblock_reserve(kstart as c_ulong, ksize as c_ulong);

    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    if initrd_start != 0 {
        let size = initrd_end - initrd_start;
        memblock_reserve(__virt_to_phys(initrd_start), size);
    }

    mmu_init_hw();
    mapin_ram();

    #[cfg(feature = "CONFIG_HIGHMEM")]
    { ioremap_base = PKMAP_BASE; ioremap_bot = PKMAP_BASE; }
    #[cfg(not(feature = "CONFIG_HIGHMEM"))]
    { ioremap_base = FIXADDR_START; ioremap_bot = FIXADDR_START; }

    mmu_context_init();
    memblock_set_current_limit(memory_start + lowmem_size - 1);
    parse_early_param();
    early_init_fdt_scan_reserved_mem();
    dma_contiguous_reserve(memory_start + lowmem_size - 1);
    memblock_dump_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
