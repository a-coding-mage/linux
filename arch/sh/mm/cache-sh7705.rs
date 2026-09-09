/*
 * arch/sh/mm/cache-sh7705.c
 *
 * Copyright (C) 1999, 2000  Niibe Yutaka
 * Copyright (C) 2004  Alex Song
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Linux and architecture headers supplying these declarations are external dependencies. */

#[repr(C)]
pub struct CacheInfo {
    pub ways: ::core::ffi::c_ulong,
    pub sets: ::core::ffi::c_ulong,
    pub entry_shift: ::core::ffi::c_ulong,
    pub linesz: ::core::ffi::c_ulong,
    pub way_incr: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct CpuData {
    pub dcache: CacheInfo,
}

#[repr(C)]
pub struct FlusherData {
    pub addr1: ::core::ffi::c_ulong,
    pub addr2: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct Folio {
    pub flags: FolioFlags,
}

#[repr(C)]
pub struct FolioFlags {
    pub f: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct AddressSpace;

extern "C" {
    static mut current_cpu_data: CpuData;
    static mut local_flush_icache_range: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>;
    static mut local_flush_dcache_folio: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>;
    static mut local_flush_cache_all: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>;
    static mut local_flush_cache_mm: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>;
    static mut local_flush_cache_dup_mm: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>;
    static mut local_flush_cache_range: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>;
    static mut local_flush_cache_page: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>;
    static mut local_flush_icache_folio: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>;

    fn __raw_readl(addr: ::core::ffi::c_ulong) -> u32;
    fn __raw_writel(value: u32, addr: ::core::ffi::c_ulong);
    fn __flush_wback_region(addr: *mut ::core::ffi::c_void, size: ::core::ffi::c_ulong);
    fn __flush_purge_region(addr: *mut ::core::ffi::c_void, size: ::core::ffi::c_ulong);
    fn local_irq_save(flags: *mut ::core::ffi::c_ulong);
    fn local_irq_restore(flags: ::core::ffi::c_ulong);
    fn jump_to_uncached();
    fn back_to_cached();
    fn folio_flush_mapping(folio: *mut Folio) -> *mut AddressSpace;
    fn mapping_mapped(mapping: *mut AddressSpace) -> bool;
    fn clear_bit(bit: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong);
    fn folio_pfn(folio: *mut Folio) -> ::core::ffi::c_ulong;
    fn folio_nr_pages(folio: *mut Folio) -> u32;
    fn folio_address(folio: *mut Folio) -> *mut ::core::ffi::c_void;
    fn folio_size(folio: *mut Folio) -> ::core::ffi::c_ulong;
}

const CACHE_OC_ADDRESS_ARRAY: ::core::ffi::c_ulong = 0;
const SH_CACHE_UPDATED: u32 = 0;
const SH_CACHE_VALID: u32 = 0;
const PAGE_SIZE: ::core::ffi::c_ulong = 4096;
const PAGE_SHIFT: u32 = 12;
const PG_DCACHE_CLEAN: ::core::ffi::c_ulong = 0;

unsafe fn cache_wback_all() {
    let mut ways = current_cpu_data.dcache.ways;
    let mut waysize = current_cpu_data.dcache.sets;
    waysize <<= current_cpu_data.dcache.entry_shift;
    let mut addrstart = CACHE_OC_ADDRESS_ARRAY;

    loop {
        let mut addr = addrstart;
        while addr < addrstart + waysize {
            let data = __raw_readl(addr);
            let v = SH_CACHE_UPDATED | SH_CACHE_VALID;
            if (data & v) == v {
                __raw_writel(data & !v, addr);
            }
            addr += current_cpu_data.dcache.linesz;
        }
        addrstart += current_cpu_data.dcache.way_incr;
        ways = ways.wrapping_sub(1);
        if ways == 0 { break; }
    }
}

unsafe extern "C" fn sh7705_flush_icache_range(args: *mut ::core::ffi::c_void) {
    let data = args as *mut FlusherData;
    let start = (*data).addr1;
    let end = (*data).addr2;
    __flush_wback_region(start as *mut ::core::ffi::c_void, end - start);
}

unsafe fn __flush_dcache_page(mut phys: ::core::ffi::c_ulong) {
    let mut ways = current_cpu_data.dcache.ways;
    let mut waysize = current_cpu_data.dcache.sets;
    let mut flags: ::core::ffi::c_ulong = 0;
    phys |= SH_CACHE_VALID as ::core::ffi::c_ulong;
    local_irq_save(&mut flags);
    jump_to_uncached();
    waysize <<= current_cpu_data.dcache.entry_shift;
    let mut addrstart = CACHE_OC_ADDRESS_ARRAY;
    loop {
        let mut addr = addrstart;
        while addr < addrstart + waysize {
            let mut data = (__raw_readl(addr) as ::core::ffi::c_ulong) & (0x1ffffC00 | SH_CACHE_VALID as ::core::ffi::c_ulong);
            if data == phys {
                data &= !(SH_CACHE_VALID as ::core::ffi::c_ulong | SH_CACHE_UPDATED as ::core::ffi::c_ulong);
                __raw_writel(data as u32, addr);
            }
            addr += current_cpu_data.dcache.linesz;
        }
        addrstart += current_cpu_data.dcache.way_incr;
        ways = ways.wrapping_sub(1);
        if ways == 0 { break; }
    }
    back_to_cached();
    local_irq_restore(flags);
}

unsafe extern "C" fn sh7705_flush_dcache_folio(arg: *mut ::core::ffi::c_void) {
    let folio = arg as *mut Folio;
    let mapping = folio_flush_mapping(folio);
    if !mapping.is_null() && !mapping_mapped(mapping) {
        clear_bit(PG_DCACHE_CLEAN, &mut (*folio).flags.f);
    } else {
        let pfn = folio_pfn(folio);
        let nr = folio_nr_pages(folio);
        for i in 0..nr { __flush_dcache_page((pfn + i as u64) * PAGE_SIZE); }
    }
}

unsafe extern "C" fn sh7705_flush_cache_all(_args: *mut ::core::ffi::c_void) {
    let mut flags: ::core::ffi::c_ulong = 0;
    local_irq_save(&mut flags);
    jump_to_uncached();
    cache_wback_all();
    back_to_cached();
    local_irq_restore(flags);
}

unsafe extern "C" fn sh7705_flush_cache_page(args: *mut ::core::ffi::c_void) {
    let data = args as *mut FlusherData;
    __flush_dcache_page((*data).addr2 << PAGE_SHIFT);
}

unsafe extern "C" fn sh7705_flush_icache_folio(arg: *mut ::core::ffi::c_void) {
    let folio = arg as *mut Folio;
    __flush_purge_region(folio_address(folio), folio_size(folio));
}

pub unsafe extern "C" fn sh7705_cache_init() {
    local_flush_icache_range = Some(sh7705_flush_icache_range);
    local_flush_dcache_folio = Some(sh7705_flush_dcache_folio);
    local_flush_cache_all = Some(sh7705_flush_cache_all);
    local_flush_cache_mm = Some(sh7705_flush_cache_all);
    local_flush_cache_dup_mm = Some(sh7705_flush_cache_all);
    local_flush_cache_range = Some(sh7705_flush_cache_all);
    local_flush_cache_page = Some(sh7705_flush_cache_page);
    local_flush_icache_folio = Some(sh7705_flush_icache_folio);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
