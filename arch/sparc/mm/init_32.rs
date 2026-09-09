// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/sparc/mm/init.c
 *
 *  Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 *  Copyright (C) 1995 Eddie C. Dost (ecd@skynet.be)
 *  Copyright (C) 1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 *  Copyright (C) 2000 Anton Blanchard (anton@samba.org)
 */

// C headers and "mm_32.h" provide the external kernel symbols used below.

extern "C" {
    static mut sparc_valid_addr_bitmap: *mut usize;
    pub static mut phys_base: usize;
    pub static mut pfn_base: usize;
    pub static mut sp_banks: [sparc_phys_banks; SPARC_PHYS_BANKS + 1];
    pub static mut sparc_ramdisk_image: u32;
    pub static mut sparc_ramdisk_size: u32;
    pub static mut highstart_pfn: usize;
    pub static mut highend_pfn: usize;
    pub static mut last_valid_pfn: usize;

    static mut max_low_pfn: usize;
    static mut max_pfn: usize;
    static mut cmdline_memory_size: usize;
    static mut initrd_start: usize;
    static mut initrd_end: usize;

    fn memblock_set_bottom_up(bottom_up: bool);
    fn memblock_allow_resize();
    fn memblock_add(base: usize, size: usize);
    fn memblock_reserve(base: usize, size: usize);
    fn memblock_phys_mem_size() -> usize;
    fn memblock_reserved_size() -> usize;
    fn memblock_set_current_limit(limit: usize);
    fn memblock_alloc(size: usize, align: usize) -> *mut core::ffi::c_void;
    fn printk(fmt: *const u8, ...);
    fn prom_printf(fmt: *const u8, ...);
    fn prom_halt() -> !;
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize) -> *mut core::ffi::c_void;
    fn set_bit(nr: usize, addr: *mut usize);
    fn srmmu_paging_init();
    fn prom_build_devicetree();
    fn of_fill_in_cpu_data();
    fn device_scan();
    fn __flush_page_to_ram(vaddr: usize);
    fn page_address(page: *mut page) -> *mut core::ffi::c_void;
    fn folio_address(folio: *mut folio) -> *mut core::ffi::c_void;
    fn folio_nr_pages(folio: *mut folio) -> u32;
}

#[repr(C)]
pub struct sparc_phys_banks {
    pub base_addr: usize,
    pub num_bytes: usize,
}

#[repr(C)]
pub struct page;
#[repr(C)]
pub struct folio;
pub type pgprot_t = usize;

const PAGE_SHIFT: usize = 12;

pub unsafe fn calc_highpages() -> usize {
    let mut i = 0;
    let mut nr = 0;
    while sp_banks[i].num_bytes != 0 {
        let mut start_pfn = sp_banks[i].base_addr >> PAGE_SHIFT;
        let end_pfn = (sp_banks[i].base_addr + sp_banks[i].num_bytes) >> PAGE_SHIFT;
        if end_pfn <= max_low_pfn { i += 1; continue; }
        if start_pfn < max_low_pfn { start_pfn = max_low_pfn; }
        nr += end_pfn - start_pfn;
        i += 1;
    }
    nr
}

unsafe fn calc_max_low_pfn() -> usize {
    let mut i = 1;
    let mut tmp = pfn_base + (SRMMU_MAXMEM >> PAGE_SHIFT);
    let mut last_pfn = (sp_banks[0].base_addr + sp_banks[0].num_bytes) >> PAGE_SHIFT;
    while sp_banks[i].num_bytes != 0 {
        let curr_pfn = sp_banks[i].base_addr >> PAGE_SHIFT;
        if curr_pfn >= tmp {
            if last_pfn < tmp { tmp = last_pfn; }
            break;
        }
        last_pfn = (sp_banks[i].base_addr + sp_banks[i].num_bytes) >> PAGE_SHIFT;
        i += 1;
    }
    tmp
}

unsafe fn find_ramdisk(end_of_phys_memory: usize) {
    // CONFIG_BLK_DEV_INITRD conditional from the C source.
    if sparc_ramdisk_image != 0 {
        if sparc_ramdisk_image as usize >= (&_end as *const u8 as usize) - 2 * PAGE_SIZE {
            sparc_ramdisk_image = sparc_ramdisk_image.wrapping_sub(KERNBASE as u32);
        }
        initrd_start = sparc_ramdisk_image as usize + phys_base;
        initrd_end = initrd_start + sparc_ramdisk_size as usize;
        if initrd_end > end_of_phys_memory {
            printk(b"initrd extends beyond end of memory (0x%016lx > 0x%016lx)\ndisabling initrd\n\0".as_ptr(), initrd_end, end_of_phys_memory);
            initrd_start = 0;
        } else {
            let size = initrd_end - initrd_start;
            memblock_reserve(initrd_start, size);
            initrd_start = (initrd_start - phys_base) + PAGE_OFFSET;
            initrd_end = (initrd_end - phys_base) + PAGE_OFFSET;
        }
    }
}

pub unsafe fn bootmem_init(pages_avail: *mut usize) -> usize {
    let mut start_pfn;
    let mut bytes_avail = 0usize;
    let mut end_of_phys_memory = 0usize;
    let mut high_pages = 0usize;
    memblock_set_bottom_up(true);
    memblock_allow_resize();
    let mut i = 0;
    while sp_banks[i].num_bytes != 0 {
        end_of_phys_memory = sp_banks[i].base_addr + sp_banks[i].num_bytes;
        bytes_avail += sp_banks[i].num_bytes;
        if cmdline_memory_size != 0 && bytes_avail > cmdline_memory_size {
            let slack = bytes_avail - cmdline_memory_size;
            bytes_avail -= slack;
            end_of_phys_memory -= slack;
            sp_banks[i].num_bytes -= slack;
            if sp_banks[i].num_bytes == 0 {
                sp_banks[i].base_addr = 0xdeadbeef;
            } else {
                memblock_add(sp_banks[i].base_addr, sp_banks[i].num_bytes);
                sp_banks[i + 1].num_bytes = 0;
                sp_banks[i + 1].base_addr = 0xdeadbeef;
            }
            break;
        }
        memblock_add(sp_banks[i].base_addr, sp_banks[i].num_bytes);
        i += 1;
    }
    start_pfn = __pa(PAGE_ALIGN(&_end as *const u8 as usize)) >> PAGE_SHIFT;
    max_pfn = end_of_phys_memory >> PAGE_SHIFT;
    max_low_pfn = max_pfn;
    highstart_pfn = max_pfn;
    highend_pfn = max_pfn;
    if max_low_pfn > pfn_base + (SRMMU_MAXMEM >> PAGE_SHIFT) {
        highstart_pfn = pfn_base + (SRMMU_MAXMEM >> PAGE_SHIFT);
        max_low_pfn = calc_max_low_pfn();
        high_pages = calc_highpages();
        printk(b"%ldMB HIGHMEM available.\n\0".as_ptr(), high_pages >> (20 - PAGE_SHIFT));
    }
    find_ramdisk(end_of_phys_memory);
    let size = (start_pfn << PAGE_SHIFT) - phys_base;
    memblock_reserve(phys_base, size);
    memblock_add(phys_base, size);
    let size = memblock_phys_mem_size() - memblock_reserved_size();
    *pages_avail = (size >> PAGE_SHIFT) - high_pages;
    memblock_set_current_limit(max_low_pfn << PAGE_SHIFT);
    max_pfn
}

pub unsafe fn paging_init() { srmmu_paging_init(); prom_build_devicetree(); of_fill_in_cpu_data(); device_scan(); }

unsafe fn taint_real_pages() {
    let mut i = 0;
    while sp_banks[i].num_bytes != 0 {
        let mut start = sp_banks[i].base_addr;
        let end = start + sp_banks[i].num_bytes;
        while start < end { set_bit(start >> 20, sparc_valid_addr_bitmap); start += PAGE_SIZE; }
        i += 1;
    }
}

pub unsafe fn arch_mm_preinit() {
    if PKMAP_BASE + LAST_PKMAP * PAGE_SIZE >= FIXADDR_START {
        prom_printf(b"BUG: fixmap and pkmap areas overlap\n\0".as_ptr());
        prom_printf(b"pkbase: 0x%lx pkend: 0x%lx fixstart 0x%lx\n\0".as_ptr(), PKMAP_BASE, PKMAP_BASE + LAST_PKMAP * PAGE_SIZE, FIXADDR_START);
        prom_printf(b"Please mail sparclinux@vger.kernel.org.\n\0".as_ptr());
        prom_halt();
    }
    let mut i = (last_valid_pfn >> ((20 - PAGE_SHIFT) + 5)) + 1;
    sparc_valid_addr_bitmap = memblock_alloc(i << 2, SMP_CACHE_BYTES) as *mut usize;
    if sparc_valid_addr_bitmap.is_null() { prom_printf(b"mem_init: Cannot alloc valid_addr_bitmap.\n\0".as_ptr()); prom_halt(); }
    memset(sparc_valid_addr_bitmap as *mut _, 0, i << 2);
    taint_real_pages();
}

pub unsafe fn sparc_flush_page_to_ram(page: *mut page) { __flush_page_to_ram(page_address(page) as usize); }
pub unsafe fn sparc_flush_folio_to_ram(folio: *mut folio) {
    let vaddr = folio_address(folio) as usize;
    let nr = folio_nr_pages(folio);
    for i in 0..nr { __flush_page_to_ram(vaddr + i as usize * PAGE_SIZE); }
}

pub static protection_map: [pgprot_t; 16] = [
    PAGE_NONE, PAGE_READONLY, PAGE_COPY, PAGE_COPY,
    PAGE_READONLY, PAGE_READONLY, PAGE_COPY, PAGE_COPY,
    PAGE_NONE, PAGE_READONLY, PAGE_SHARED, PAGE_SHARED,
    PAGE_READONLY, PAGE_READONLY, PAGE_SHARED, PAGE_SHARED,
];

extern "C" {
    static _end: u8;
}
// EXPORT_SYMBOL and DECLARE_VM_GET_PAGE_PROT are linker/build metadata in C.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
