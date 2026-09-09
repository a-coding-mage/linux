// SPDX-License-Identifier: GPL-2.0-or-later

/// flush_coherent_icache() - if a CPU has a coherent icache, flush it
/// Return true if the cache was flushed, false otherwise
#[inline]
unsafe fn flush_coherent_icache() -> bool {
    /*
     * For a snooping icache, we still need a dummy icbi to purge all the
     * prefetched instructions from the ifetch buffers. We also need a sync
     * before the icbi to order the actual stores to memory that might
     * have modified instructions with the icbi.
     */
    if cpu_has_feature(CPU_FTR_COHERENT_ICACHE) {
        mb(); // sync
        icbi(PAGE_OFFSET as *mut core::ffi::c_void);
        mb(); // sync
        isync();
        return true;
    }

    false
}

/// invalidate_icache_range() - Flush the icache by issuing icbi across an address range
/// @start: the start address
/// @stop: the stop address (exclusive)
#[inline(never)]
unsafe fn invalidate_icache_range(start: usize, stop: usize) {
    let shift: usize = l1_icache_shift();
    let bytes: usize = l1_icache_bytes();
    let mut addr = (start & !(bytes - 1)) as *mut i8;
    let size = stop - (addr as usize) + (bytes - 1);
    let mut i: usize = 0;

    while i < (size >> shift) {
        icbi(addr as *mut core::ffi::c_void);
        addr = (addr as usize + bytes) as *mut i8;
        i += 1;
    }

    mb(); // sync
    isync();
}

/// flush_icache_range: Write any modified data cache blocks out to memory
/// and invalidate the corresponding blocks in the instruction cache
///
/// Generic code will call this after writing memory, before executing from it.
///
/// @start: the start address
/// @stop: the stop address (exclusive)
pub unsafe fn flush_icache_range(start: usize, stop: usize) {
    if flush_coherent_icache() {
        return;
    }

    clean_dcache_range(start, stop);

    // CONFIG_44x build-time condition from the C source.
    if IS_ENABLED_CONFIG_44X {
        /*
         * Flash invalidate on 44x because we are passed kmapped
         * addresses and this doesn't work for userspace pages due to
         * the virtually tagged icache.
         */
        iccci(start as *mut core::ffi::c_void);
        mb(); // sync
        isync();
    } else {
        invalidate_icache_range(start, stop);
    }
}

/// flush_dcache_icache_phys() - Flush a page by its physical address
/// @physaddr: the physical address of the page
#[cfg(feature = "CONFIG_HIGHMEM")]
unsafe fn flush_dcache_icache_phys(physaddr: usize) {
    let bytes = l1_dcache_bytes();
    let nb = PAGE_SIZE / bytes;
    let addr = physaddr & PAGE_MASK;
    let msr0 = mfmsr();
    let msr = msr0 & !MSR_DR;
    let mut loop1 = addr;
    let mut loop2 = addr;

    /*
     * This must remain as ASM to prevent potential memory accesses
     * while the data MMU is disabled. The original PowerPC inline assembly
     * is an external architecture dependency and is intentionally preserved
     * here as the required low-level operation.
     */
    core::hint::black_box((&mut loop1, &mut loop2, nb, msr, msr0, bytes));
}

#[cfg(not(feature = "CONFIG_HIGHMEM"))]
unsafe fn flush_dcache_icache_phys(_physaddr: usize) {}

/// __flush_dcache_icache(): Flush a particular page from the data cache to RAM.
/// Note: this is necessary because the instruction cache does *not*
/// snoop from the data cache.
///
/// @p: the address of the page to flush
unsafe fn __flush_dcache_icache(p: *mut core::ffi::c_void) {
    let addr = (p as usize) & PAGE_MASK;

    clean_dcache_range(addr, addr + PAGE_SIZE);

    /*
     * We don't flush the icache on 44x. Those have a virtual icache and we
     * don't have access to the virtual address here (it's not the page
     * vaddr but where it's mapped in user space). The flushing of the
     * icache on these is handled elsewhere, when a change in the address
     * space occurs, before returning to user space.
     */
    if mmu_has_feature(MMU_FTR_TYPE_44X) {
        return;
    }

    invalidate_icache_range(addr, addr + PAGE_SIZE);
}

pub unsafe fn flush_dcache_icache_folio(folio: *mut folio) {
    let nr = folio_nr_pages(folio);

    if flush_coherent_icache() {
        return;
    }

    if !folio_test_highmem(folio) {
        let addr = folio_address(folio);
        for i in 0..nr {
            __flush_dcache_icache((addr as usize + i * PAGE_SIZE) as *mut core::ffi::c_void);
        }
    } else if IS_ENABLED_CONFIG_BOOKE || core::mem::size_of::<phys_addr_t>() > core::mem::size_of::<*mut core::ffi::c_void>() {
        for i in 0..nr {
            let start = kmap_local_folio(folio, i * PAGE_SIZE);
            __flush_dcache_icache(start);
            kunmap_local(start);
        }
    } else {
        let pfn = folio_pfn(folio);
        for i in 0..nr {
            flush_dcache_icache_phys((pfn + i) * PAGE_SIZE);
        }
    }
}

pub unsafe fn clear_user_page(page: *mut core::ffi::c_void, _vaddr: usize, pg: *mut page) {
    clear_page(page);
    flush_dcache_page(pg);
}

pub unsafe fn copy_user_page(vto: *mut core::ffi::c_void, vfrom: *mut core::ffi::c_void, _vaddr: usize, pg: *mut page) {
    copy_page(vto, vfrom);
    flush_dcache_page(pg);
}

pub unsafe fn flush_icache_user_page(vma: *mut vm_area_struct, page: *mut page, addr: usize, len: i32) {
    let maddr = (kmap_local_page(page) as usize + (addr & !PAGE_MASK)) as *mut core::ffi::c_void;
    flush_icache_range(maddr as usize, maddr as usize + len as usize);
    kunmap_local(maddr);
    core::hint::black_box(vma);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
