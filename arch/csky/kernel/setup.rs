// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// C dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_BLK_DEV_INITRD)]
unsafe fn setup_initrd() {
    let mut size: libc::c_ulong;

    if initrd_start >= initrd_end {
        pr_err(c"initrd not found or empty");
        initrd_start = 0;
        initrd_end = 0;
        pr_err(c" - disabling initrd\n");
        return;
    }

    if __pa(initrd_end) > PFN_PHYS(max_low_pfn) {
        pr_err(c"initrd extends beyond end of memory");
        initrd_start = 0;
        initrd_end = 0;
        pr_err(c" - disabling initrd\n");
        return;
    }

    size = initrd_end - initrd_start;

    if memblock_is_region_reserved(__pa(initrd_start), size) {
        pr_err(c"INITRD: 0x%08lx+0x%08lx overlaps in-use memory region",
               __pa(initrd_start), size);
        initrd_start = 0;
        initrd_end = 0;
        pr_err(c" - disabling initrd\n");
        return;
    }

    memblock_reserve(__pa(initrd_start), size);

    pr_info(c"Initial ramdisk at: 0x%p (%lu bytes)\n",
            initrd_start as *mut libc::c_void, size);

    initrd_below_start_ok = 1;
}

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut libc::c_ulong) {
    *max_zone_pfns.add(ZONE_NORMAL as usize) = max_low_pfn;
    #[cfg(CONFIG_HIGHMEM)]
    {
        *max_zone_pfns.add(ZONE_HIGHMEM as usize) = max_pfn;
    }
}

unsafe fn csky_memblock_init() {
    let lowmem_size: libc::c_ulong = PFN_DOWN(LOWMEM_LIMIT - PHYS_OFFSET_OFFSET);
    let sseg_size: libc::c_ulong = PFN_DOWN(SSEG_SIZE - PHYS_OFFSET_OFFSET);
    let mut size: libc::c_long;

    memblock_reserve(__pa(_start), _end - _start);

    early_init_fdt_reserve_self();
    early_init_fdt_scan_reserved_mem();

    memblock_dump_all();

    min_low_pfn = PFN_UP(memblock_start_of_DRAM());
    max_low_pfn = max_pfn = PFN_DOWN(memblock_end_of_DRAM());

    size = (max_pfn - min_low_pfn) as libc::c_long;

    if size >= lowmem_size as libc::c_long {
        max_low_pfn = min_low_pfn + lowmem_size;
        #[cfg(CONFIG_PAGE_OFFSET_80000000)]
        write_mmu_msa1(read_mmu_msa0() + SSEG_SIZE);
    } else if size > sseg_size as libc::c_long {
        max_low_pfn = min_low_pfn + sseg_size;
    }

    #[cfg(CONFIG_BLK_DEV_INITRD)]
    setup_initrd();

    mmu_init(min_low_pfn, max_low_pfn);

    #[cfg(CONFIG_HIGHMEM)]
    {
        highstart_pfn = max_low_pfn;
        highend_pfn = max_pfn;
    }
    memblock_set_current_limit(PFN_PHYS(max_low_pfn));

    dma_contiguous_reserve(0);
}

pub unsafe fn setup_arch(cmdline_p: *mut *mut libc::c_char) {
    *cmdline_p = boot_command_line;

    console_verbose();

    pr_info(c"Phys. mem: %ldMB\n",
            memblock_phys_mem_size() / 1024 / 1024);

    setup_initial_init_mm(_start, _etext, _edata, _end);

    parse_early_param();

    csky_memblock_init();

    unflatten_and_copy_device_tree();

    #[cfg(CONFIG_SMP)]
    setup_smp();

    fixaddr_init();

    #[cfg(CONFIG_HIGHMEM)]
    kmap_init();
}

pub static mut va_pa_offset: libc::c_ulong = 0;

#[inline]
unsafe fn read_mmu_msa() -> libc::c_ulong {
    #[cfg(CONFIG_PAGE_OFFSET_80000000)]
    {
        return read_mmu_msa0();
    }
    #[cfg(CONFIG_PAGE_OFFSET_A0000000)]
    {
        return read_mmu_msa1();
    }
    0
}

pub unsafe extern "C" fn csky_start(_unused: libc::c_uint, dtb_start: *mut libc::c_void) {
    /* Clean up bss section */
    core::ptr::write_bytes(__bss_start, 0, (__bss_stop as usize) - (__bss_start as usize));

    va_pa_offset = read_mmu_msa() & !(SSEG_SIZE - 1);

    pre_trap_init();

    if dtb_start.is_null() {
        early_init_dt_scan(__dtb_start, __pa(dtb_start));
    } else {
        early_init_dt_scan(dtb_start, __pa(dtb_start));
    }

    start_kernel();

    core::arch::asm!("br .");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
