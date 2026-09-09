/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 - 2003, 06, 07 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2007 MIPS Technologies, Inc.
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

/* Cache operations. */
pub static mut flush_cache_all: Option<unsafe extern "C" fn()> = None;
pub static mut __flush_cache_all: Option<unsafe extern "C" fn()> = None;
pub static mut flush_cache_mm: Option<unsafe extern "C" fn(*mut mm_struct)> = None;
pub static mut flush_cache_range:
    Option<unsafe extern "C" fn(*mut vm_area_struct, c_ulong, c_ulong)> = None;
pub static mut flush_cache_page:
    Option<unsafe extern "C" fn(*mut vm_area_struct, c_ulong, c_ulong)> = None;
pub static mut flush_icache_range: Option<unsafe extern "C" fn(c_ulong, c_ulong)> = None;
pub static mut local_flush_icache_range: Option<unsafe extern "C" fn(c_ulong, c_ulong)> = None;
pub static mut __flush_icache_user_range: Option<unsafe extern "C" fn(c_ulong, c_ulong)> = None;
pub static mut __local_flush_icache_user_range:
    Option<unsafe extern "C" fn(c_ulong, c_ulong)> = None;

pub static mut __flush_cache_vmap: Option<unsafe extern "C" fn()> = None;
pub static mut __flush_cache_vunmap: Option<unsafe extern "C" fn()> = None;
pub static mut __flush_kernel_vmap_range:
    Option<unsafe extern "C" fn(c_ulong, c_int)> = None;

/* MIPS specific cache operations */
pub static mut flush_data_cache_page: Option<unsafe extern "C" fn(c_ulong)> = None;
pub static mut flush_icache_all: Option<unsafe extern "C" fn()> = None;

/* Dummy cache handling routine */
pub unsafe extern "C" fn cache_noop() {}

// CONFIG_BOARD_SCACHE conditionally supplies the board-cache operations.
// CONFIG_DMA_NONCOHERENT conditionally supplies the DMA cache operations.

pub static mut _dma_cache_wback_inv:
    Option<unsafe extern "C" fn(c_ulong, c_ulong)> = None;
pub static mut _dma_cache_wback: Option<unsafe extern "C" fn(c_ulong, c_ulong)> = None;
pub static mut _dma_cache_inv: Option<unsafe extern "C" fn(c_ulong, c_ulong)> = None;

/*
 * We could optimize the case where the cache argument is not BCACHE but
 * that seems very atypical use ...
 */
pub unsafe extern "C" fn cacheflush(addr: c_ulong, bytes: c_ulong, _cache: c_uint) -> c_long {
    if bytes == 0 {
        return 0;
    }
    if !access_ok(addr as *mut core::ffi::c_void, bytes) {
        return -EFAULT;
    }

    if let Some(flush) = __flush_icache_user_range {
        flush(addr, addr.wrapping_add(bytes));
    }

    0
}

pub unsafe fn __flush_dcache_folio_pages(
    folio: *mut folio,
    page: *mut page,
    nr: c_uint,
) {
    let mapping = folio_flush_mapping(folio);
    let mut addr: c_ulong;

    if !mapping.is_null() && !mapping_mapped(mapping) {
        folio_set_dcache_dirty(folio);
        return;
    }

    /*
     * We could delay the flush for the !folio_mapping case too.  But that
     * case is for exec env/arg pages and those are %99 certainly going to
     * get faulted into the tlb (and thus flushed) anyways.
     */
    for i in 0..nr {
        addr = kmap_local_page(page.add(i as usize)) as c_ulong;
        if let Some(flush) = flush_data_cache_page {
            flush(addr);
        }
        kunmap_local(addr as *mut core::ffi::c_void);
    }
}

pub unsafe fn __flush_anon_page(page: *mut page, vmaddr: c_ulong) {
    let addr = page_address(page) as c_ulong;
    let folio = page_folio(page);

    if pages_do_alias(addr, vmaddr) {
        if folio_mapped(folio) && !folio_test_dcache_dirty(folio) {
            let kaddr = kmap_coherent(page, vmaddr);
            if let Some(flush) = flush_data_cache_page {
                flush(kaddr as c_ulong);
            }
            kunmap_coherent();
        } else if let Some(flush) = flush_data_cache_page {
            flush(addr);
        }
    }
}

pub unsafe fn __update_cache(address: c_ulong, pte: pte_t) {
    let pfn = pte_pfn(pte);
    if !pfn_valid(pfn) {
        return;
    }

    let folio = page_folio(pfn_to_page(pfn));
    let mut address = (address & PAGE_MASK).wrapping_sub(offset_in_folio(folio, pfn << PAGE_SHIFT));
    let exec = !pte_no_exec(pte) && !cpu_has_ic_fills_f_dc;

    if folio_test_dcache_dirty(folio) {
        for i in 0..folio_nr_pages(folio) {
            let addr = kmap_local_folio(folio, i) as c_ulong;
            if exec || pages_do_alias(addr, address) {
                if let Some(flush) = flush_data_cache_page {
                    flush(addr);
                }
            }
            kunmap_local(addr as *mut core::ffi::c_void);
            address = address.wrapping_add(PAGE_SIZE);
        }
        folio_clear_dcache_dirty(folio);
    }
}

pub static mut _page_cachable_default: c_ulong = 0;

#[allow(non_upper_case_globals)]
pub static mut protection_map: [pgprot_t; 16] = [pgprot_t::default(); 16];

unsafe fn setup_protection_map() {
    protection_map[0] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_NO_EXEC | _PAGE_NO_READ));
    protection_map[1] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_NO_EXEC));
    protection_map[2] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_NO_EXEC | _PAGE_NO_READ));
    protection_map[3] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_NO_EXEC));
    protection_map[4] = __pgprot(_page_cachable_default | _PAGE_PRESENT);
    protection_map[5] = __pgprot(_page_cachable_default | _PAGE_PRESENT);
    protection_map[6] = __pgprot(_page_cachable_default | _PAGE_PRESENT);
    protection_map[7] = __pgprot(_page_cachable_default | _PAGE_PRESENT);
    protection_map[8] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_NO_EXEC | _PAGE_NO_READ));
    protection_map[9] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_NO_EXEC));
    protection_map[10] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_NO_EXEC | _PAGE_WRITE | _PAGE_NO_READ));
    protection_map[11] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_NO_EXEC | _PAGE_WRITE));
    protection_map[12] = __pgprot(_page_cachable_default | _PAGE_PRESENT);
    protection_map[13] = __pgprot(_page_cachable_default | _PAGE_PRESENT);
    protection_map[14] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_WRITE));
    protection_map[15] = __pgprot(_page_cachable_default | (_PAGE_PRESENT | _PAGE_WRITE));
}

pub unsafe fn cpu_cache_init() {
    if IS_ENABLED(CONFIG_CPU_R3000) && cpu_has_3k_cache {
        r3k_cache_init();
    }
    if (IS_ENABLED(CONFIG_CPU_R4K_CACHE_TLB) || IS_ENABLED(CONFIG_CPU_SB1)) && cpu_has_4k_cache {
        r4k_cache_init();
    }
    if IS_ENABLED(CONFIG_CPU_CAVIUM_OCTEON) && cpu_has_octeon_cache {
        octeon_cache_init();
    }
    setup_protection_map();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
