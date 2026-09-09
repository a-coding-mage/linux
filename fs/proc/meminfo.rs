// SPDX-License-Identifier: GPL-2.0
// C headers and symbols are supplied by the surrounding kernel translation.

#[allow(dead_code)]
pub unsafe extern "C" fn arch_report_meminfo(m: *mut seq_file) {}

unsafe fn show_val_kb(m: *mut seq_file, s: *const core::ffi::c_char, num: c_ulong) {
    seq_put_decimal_ull_width(m, s, num << (PAGE_SHIFT - 10), 8);
    seq_write(m, b" kB\n".as_ptr() as *const core::ffi::c_char, 4);
}

unsafe fn meminfo_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> c_int {
    let mut i: sysinfo = core::mem::zeroed();
    let mut committed: c_ulong;
    let mut cached: c_long;
    let mut available: c_long;
    let mut pages: [c_ulong; NR_LRU_LISTS as usize] = [0; NR_LRU_LISTS as usize];
    let (mut sreclaimable, mut sunreclaim): (c_ulong, c_ulong);
    let mut lru: c_int;

    si_meminfo(&mut i);
    si_swapinfo(&mut i);
    committed = vm_memory_committed();

    cached = global_node_page_state(NR_FILE_PAGES) - total_swapcache_pages() as c_long
        - i.bufferram as c_long;
    if cached < 0 {
        cached = 0;
    }

    lru = LRU_BASE;
    while lru < NR_LRU_LISTS {
        pages[lru as usize] = global_node_page_state(NR_LRU_BASE + lru);
        lru += 1;
    }

    available = si_mem_available();
    sreclaimable = global_node_page_state_pages(NR_SLAB_RECLAIMABLE_B);
    sunreclaim = global_node_page_state_pages(NR_SLAB_UNRECLAIMABLE_B);

    show_val_kb(m, c"MemTotal:       ".as_ptr(), i.totalram);
    show_val_kb(m, c"MemFree:        ".as_ptr(), i.freeram);
    show_val_kb(m, c"MemAvailable:   ".as_ptr(), available as c_ulong);
    show_val_kb(m, c"Buffers:        ".as_ptr(), i.bufferram);
    show_val_kb(m, c"Cached:         ".as_ptr(), cached as c_ulong);
    show_val_kb(m, c"SwapCached:     ".as_ptr(), total_swapcache_pages());
    show_val_kb(m, c"Active:         ".as_ptr(), pages[LRU_ACTIVE_ANON as usize] + pages[LRU_ACTIVE_FILE as usize]);
    show_val_kb(m, c"Inactive:       ".as_ptr(), pages[LRU_INACTIVE_ANON as usize] + pages[LRU_INACTIVE_FILE as usize]);
    show_val_kb(m, c"Active(anon):   ".as_ptr(), pages[LRU_ACTIVE_ANON as usize]);
    show_val_kb(m, c"Inactive(anon): ".as_ptr(), pages[LRU_INACTIVE_ANON as usize]);
    show_val_kb(m, c"Active(file):   ".as_ptr(), pages[LRU_ACTIVE_FILE as usize]);
    show_val_kb(m, c"Inactive(file): ".as_ptr(), pages[LRU_INACTIVE_FILE as usize]);
    show_val_kb(m, c"Unevictable:    ".as_ptr(), pages[LRU_UNEVICTABLE as usize]);
    show_val_kb(m, c"Mlocked:        ".as_ptr(), global_zone_page_state(NR_MLOCK));

    // CONFIG_HIGHMEM
    #[cfg(CONFIG_HIGHMEM)]
    {
        show_val_kb(m, c"HighTotal:      ".as_ptr(), i.totalhigh);
        show_val_kb(m, c"HighFree:       ".as_ptr(), i.freehigh);
        show_val_kb(m, c"LowTotal:       ".as_ptr(), i.totalram - i.totalhigh);
        show_val_kb(m, c"LowFree:        ".as_ptr(), i.freeram - i.freehigh);
    }

    // !CONFIG_MMU
    #[cfg(not(CONFIG_MMU))]
    show_val_kb(m, c"MmapCopy:       ".as_ptr(), atomic_long_read(&mmap_pages_allocated) as c_ulong);

    show_val_kb(m, c"SwapTotal:      ".as_ptr(), i.totalswap);
    show_val_kb(m, c"SwapFree:       ".as_ptr(), i.freeswap);
    // CONFIG_ZSWAP
    #[cfg(CONFIG_ZSWAP)]
    {
        show_val_kb(m, c"Zswap:          ".as_ptr(), zswap_total_pages());
        seq_printf(m, c"Zswapped:       %8lu kB\n".as_ptr(), (atomic_long_read(&zswap_stored_pages) as c_ulong) << (PAGE_SHIFT - 10));
    }
    show_val_kb(m, c"Dirty:          ".as_ptr(), global_node_page_state(NR_FILE_DIRTY));
    show_val_kb(m, c"Writeback:      ".as_ptr(), global_node_page_state(NR_WRITEBACK));
    show_val_kb(m, c"AnonPages:      ".as_ptr(), global_node_page_state(NR_ANON_MAPPED));
    show_val_kb(m, c"Mapped:         ".as_ptr(), global_node_page_state(NR_FILE_MAPPED));
    show_val_kb(m, c"Shmem:          ".as_ptr(), i.sharedram);
    show_val_kb(m, c"KReclaimable:   ".as_ptr(), sreclaimable + global_node_page_state(NR_KERNEL_MISC_RECLAIMABLE));
    show_val_kb(m, c"Slab:           ".as_ptr(), sreclaimable + sunreclaim);
    show_val_kb(m, c"SReclaimable:   ".as_ptr(), sreclaimable);
    show_val_kb(m, c"SUnreclaim:     ".as_ptr(), sunreclaim);
    seq_printf(m, c"KernelStack:    %8lu kB\n".as_ptr(), global_node_page_state(NR_KERNEL_STACK_KB));
    // CONFIG_SHADOW_CALL_STACK
    #[cfg(CONFIG_SHADOW_CALL_STACK)]
    seq_printf(m, c"ShadowCallStack:%8lu kB\n".as_ptr(), global_node_page_state(NR_KERNEL_SCS_KB));
    show_val_kb(m, c"PageTables:     ".as_ptr(), global_node_page_state(NR_PAGETABLE));
    show_val_kb(m, c"SecPageTables:  ".as_ptr(), global_node_page_state(NR_SECONDARY_PAGETABLE));
    show_val_kb(m, c"NFS_Unstable:   ".as_ptr(), 0);
    show_val_kb(m, c"Bounce:         ".as_ptr(), 0);
    show_val_kb(m, c"WritebackTmp:   ".as_ptr(), 0);
    show_val_kb(m, c"CommitLimit:    ".as_ptr(), vm_commit_limit());
    show_val_kb(m, c"Committed_AS:   ".as_ptr(), committed);
    seq_printf(m, c"VmallocTotal:   %8lu kB\n".as_ptr(), (VMALLOC_TOTAL as c_ulong) >> 10);
    show_val_kb(m, c"VmallocUsed:    ".as_ptr(), global_node_page_state(NR_VMALLOC));
    show_val_kb(m, c"VmallocChunk:   ".as_ptr(), 0);
    show_val_kb(m, c"Percpu:         ".as_ptr(), pcpu_nr_pages());
    memtest_report_meminfo(m);
    // CONFIG_MEMORY_FAILURE
    #[cfg(CONFIG_MEMORY_FAILURE)]
    seq_printf(m, c"HardwareCorrupted: %5lu kB\n".as_ptr(), (atomic_long_read(&num_poisoned_pages) as c_ulong) << (PAGE_SHIFT - 10));
    // CONFIG_TRANSPARENT_HUGEPAGE
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    {
        show_val_kb(m, c"AnonHugePages:  ".as_ptr(), global_node_page_state(NR_ANON_THPS));
        show_val_kb(m, c"ShmemHugePages: ".as_ptr(), global_node_page_state(NR_SHMEM_THPS));
        show_val_kb(m, c"ShmemPmdMapped: ".as_ptr(), global_node_page_state(NR_SHMEM_PMDMAPPED));
        show_val_kb(m, c"FileHugePages:  ".as_ptr(), global_node_page_state(NR_FILE_THPS));
        show_val_kb(m, c"FilePmdMapped:  ".as_ptr(), global_node_page_state(NR_FILE_PMDMAPPED));
    }
    // CONFIG_CMA
    #[cfg(CONFIG_CMA)]
    {
        show_val_kb(m, c"CmaTotal:       ".as_ptr(), totalcma_pages);
        show_val_kb(m, c"CmaFree:        ".as_ptr(), global_zone_page_state(NR_FREE_CMA_PAGES));
    }
    // CONFIG_UNACCEPTED_MEMORY
    #[cfg(CONFIG_UNACCEPTED_MEMORY)]
    show_val_kb(m, c"Unaccepted:     ".as_ptr(), global_zone_page_state(NR_UNACCEPTED));
    show_val_kb(m, c"Balloon:        ".as_ptr(), global_node_page_state(NR_BALLOON_PAGES));
    show_val_kb(m, c"GPUActive:      ".as_ptr(), global_node_page_state(NR_GPU_ACTIVE));
    show_val_kb(m, c"GPUReclaim:     ".as_ptr(), global_node_page_state(NR_GPU_RECLAIM));
    hugetlb_report_meminfo(m);
    arch_report_meminfo(m);
    0
}

unsafe fn proc_meminfo_init() -> c_int {
    let mut pde: *mut proc_dir_entry;
    pde = proc_create_single(c"meminfo".as_ptr(), 0, core::ptr::null_mut(), meminfo_proc_show);
    pde_make_permanent(pde);
    0
}

// fs_initcall(proc_meminfo_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
