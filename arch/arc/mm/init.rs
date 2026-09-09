// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Linux and ARC headers provide the types, constants, macros, and external
// symbols referenced below.

#[repr(C)]
pub struct pgd_t {
    _private: [u8; 0],
}

#[no_mangle]
#[repr(align(4096))]
pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD] = [pgd_t { _private: [] }; PTRS_PER_PGD];

static low_mem_start: ::core::ffi::c_ulong = CONFIG_LINUX_RAM_BASE;
static mut low_mem_sz: ::core::ffi::c_ulong = 0;

#[cfg(feature = "CONFIG_HIGHMEM")]
static mut min_high_pfn: ::core::ffi::c_ulong = 0;
#[cfg(feature = "CONFIG_HIGHMEM")]
static mut max_high_pfn: ::core::ffi::c_ulong = 0;
#[cfg(feature = "CONFIG_HIGHMEM")]
static mut high_mem_start: phys_addr_t = 0;
#[cfg(feature = "CONFIG_HIGHMEM")]
static mut high_mem_sz: phys_addr_t = 0;
#[cfg(feature = "CONFIG_HIGHMEM")]
#[no_mangle]
pub static mut arch_pfn_offset: ::core::ffi::c_ulong = 0;

pub unsafe extern "C" fn arc_get_mem_sz() -> ::core::ffi::c_long {
    low_mem_sz as ::core::ffi::c_long
}

/* User can over-ride above with "mem=nnn[KkMm]" in cmdline */
unsafe extern "C" fn setup_mem_sz(str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    low_mem_sz = memparse(str_, core::ptr::null_mut()) & PAGE_MASK;

    /* early console might not be setup yet - it will show up later */
    pr_info(c"\"mem=%s\": mem sz set to %ldM\n", str_, TO_MB(low_mem_sz));

    0
}

/* early_param("mem", setup_mem_sz); */

pub unsafe extern "C" fn early_init_dt_add_memory_arch(base: u64, size: u64) {
    let mut in_use: ::core::ffi::c_int = 0;

    if low_mem_sz == 0 {
        if base != low_mem_start as u64 {
            panic(c"CONFIG_LINUX_RAM_BASE != DT memory { }");
        }

        low_mem_sz = size as ::core::ffi::c_ulong;
        in_use = 1;
        memblock_add_node(base, size, 0, MEMBLOCK_NONE);
    } else {
        #[cfg(feature = "CONFIG_HIGHMEM")]
        {
            high_mem_start = base as phys_addr_t;
            high_mem_sz = size as phys_addr_t;
            in_use = 1;
            memblock_add_node(base, size, 1, MEMBLOCK_NONE);
            memblock_reserve(base, size);
        }
    }

    pr_info(c"Memory @ %llx [%lldM] %s\n", base, TO_MB(size), if in_use == 0 { c"Not used" } else { c"" });
}

pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfn: *mut ::core::ffi::c_ulong) {
    /*----------------- node/zones setup --------------------------*/
    *max_zone_pfn.add(ZONE_NORMAL) = max_low_pfn;

    #[cfg(feature = "CONFIG_HIGHMEM")]
    {
        /*
         * max_high_pfn should be ok here for both HIGHMEM and HIGHMEM+PAE.
         * For HIGHMEM without PAE max_high_pfn should be less than
         * min_low_pfn to guarantee that these two regions don't overlap.
         * For PAE case highmem is greater than lowmem, so it is natural
         * to use max_high_pfn.
         *
         * In both cases, holes should be handled by pfn_valid().
         */
        *max_zone_pfn.add(ZONE_HIGHMEM) = max_high_pfn;
    }
}

/*
 * First memory setup routine called from setup_arch()
 * 1. setup swapper's mm @init_mm
 * 2. Count the pages we have and setup bootmem allocator
 * 3. zone setup
 */
pub unsafe extern "C" fn setup_arch_memory() {
    setup_initial_init_mm(_text, _etext, _edata, _end);

    /* first page of system - kernel .vector starts here */
    min_low_pfn = virt_to_pfn(CONFIG_LINUX_RAM_BASE as *mut ::core::ffi::c_void);

    /* Last usable page of low mem */
    max_low_pfn = PFN_DOWN(low_mem_start + low_mem_sz);
    max_pfn = max_low_pfn;

    /*------------- bootmem allocator setup -----------------------*/

    /*
     * seed the bootmem allocator after any DT memory node parsing or
     * "mem=xxx" cmdline overrides have potentially updated @arc_mem_sz
     *
     * Only low mem is added, otherwise we have crashes when allocating
     * mem_map[] itself. NO_BOOTMEM allocates mem_map[] at the end of
     * avail memory, ending in highmem with a > 32-bit address. However
     * it then tries to memset it with a truncaed 32-bit handle, causing
     * the crash
     */

    memblock_reserve(CONFIG_LINUX_LINK_BASE, __pa(_end) - CONFIG_LINUX_LINK_BASE);

    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    if phys_initrd_size != 0 {
        memblock_reserve(phys_initrd_start, phys_initrd_size);
        initrd_start = __va(phys_initrd_start) as ::core::ffi::c_ulong;
        initrd_end = initrd_start + phys_initrd_size;
    }

    early_init_fdt_reserve_self();
    early_init_fdt_scan_reserved_mem();

    memblock_dump_all();

    #[cfg(feature = "CONFIG_HIGHMEM")]
    {
        /*
         * On ARC (w/o PAE) HIGHMEM addresses are actually smaller (0 based)
         * than addresses in normal aka low memory (0x8000_0000 based).
         * Even with PAE, the huge peripheral space hole would waste a lot of
         * mem with single contiguous mem_map[].
         * Thus when HIGHMEM on ARC is enabled the memory map corresponding
         * to the hole is freed and ARC specific version of pfn_valid()
         * handles the hole in the memory map.
         */

        min_high_pfn = PFN_DOWN(high_mem_start);
        max_high_pfn = PFN_DOWN(high_mem_start + high_mem_sz);

        arch_pfn_offset = min(min_low_pfn, min_high_pfn);
        kmap_init();
    }
}

pub unsafe extern "C" fn arch_mm_preinit() {
    #[cfg(feature = "CONFIG_HIGHMEM")]
    memblock_phys_free(high_mem_start, high_mem_sz);

    BUILD_BUG_ON!((PTRS_PER_PGD * core::mem::size_of::<pgd_t>()) > PAGE_SIZE);
    BUILD_BUG_ON!((PTRS_PER_PUD * core::mem::size_of::<pud_t>()) > PAGE_SIZE);
    BUILD_BUG_ON!((PTRS_PER_PMD * core::mem::size_of::<pmd_t>()) > PAGE_SIZE);
    BUILD_BUG_ON!((PTRS_PER_PTE * core::mem::size_of::<pte_t>()) > PAGE_SIZE);
}

#[cfg(feature = "CONFIG_HIGHMEM")]
pub unsafe extern "C" fn pfn_valid(pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    ((pfn >= min_high_pfn && pfn <= max_high_pfn)
        || (pfn >= min_low_pfn && pfn <= max_low_pfn)) as ::core::ffi::c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
