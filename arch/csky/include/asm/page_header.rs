/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding C headers:
// asm/setup.h, asm/cache.h, linux/const.h, vdso/page.h, linux/pfn.h,
// abi/page.h, asm-generic/memory_model.h, and asm-generic/getorder.h.

pub const THREAD_SIZE: usize = PAGE_SIZE * 2;
pub const THREAD_MASK: usize = !(THREAD_SIZE - 1);
pub const THREAD_SHIFT: usize = PAGE_SHIFT + 1;

/*
 * For C-SKY "User-space:Kernel-space" is "2GB:2GB" fixed by hardware and there
 * are two segment registers (MSA0 + MSA1) to mapping 512MB + 512MB physical
 * address region. We use them mapping kernel 1GB direct-map address area and
 * for more than 1GB of memory we use highmem.
 */
pub const PAGE_OFFSET: usize = CONFIG_PAGE_OFFSET;
pub const SSEG_SIZE: usize = 0x20000000;
pub const LOWMEM_LIMIT: usize = SSEG_SIZE * 2;
pub const PHYS_OFFSET_OFFSET: usize = CONFIG_DRAM_BASE & (SSEG_SIZE - 1);

// The following declarations are omitted by the C header for assembler users.

pub unsafe extern "C" {
    pub static mut high_memory: *mut core::ffi::c_void;
    pub static mut mem_map: *mut page;
    pub static mut va_pa_offset: usize;

    pub fn memset(
        dest: *mut core::ffi::c_void,
        c: core::ffi::c_int,
        l: usize,
    ) -> *mut core::ffi::c_void;
    pub fn memcpy(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        l: usize,
    ) -> *mut core::ffi::c_void;
}

pub const fn virt_addr_valid(kaddr: usize) -> bool {
    kaddr >= PAGE_OFFSET && kaddr < high_memory as usize
}

pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    let _ = memset(page, 0, PAGE_SIZE);
}

pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void) {
    let _ = memcpy(to, from, PAGE_SIZE);
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    pub pte_low: usize,
}

pub const fn pte_val(x: pte_t) -> usize { x.pte_low }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t {
    pub pgd: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t {
    pub pgprot: usize,
}

pub type pgtable_t = *mut page;

pub const fn pgd_val(x: pgd_t) -> usize { x.pgd }
pub const fn pgprot_val(x: pgprot_t) -> usize { x.pgprot }

pub unsafe fn ptep_buddy(x: *mut pte_t) -> *mut pte_t {
    ((x as usize) ^ core::mem::size_of::<pte_t>()) as *mut pte_t
}

pub const fn __pte(x: usize) -> pte_t { pte_t { pte_low: x } }
pub const fn __pgd(x: usize) -> pgd_t { pgd_t { pgd: x } }
pub const fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x } }

pub const ARCH_PFN_OFFSET: usize = PFN_DOWN(va_pa_offset + PHYS_OFFSET_OFFSET);

pub unsafe fn __pa(x: usize) -> usize { x - PAGE_OFFSET + va_pa_offset }
pub unsafe fn __va(x: usize) -> *mut core::ffi::c_void {
    (x + PAGE_OFFSET - va_pa_offset) as *mut core::ffi::c_void
}

pub unsafe fn __pa_symbol(x: usize) -> usize { __pa(x) }

pub unsafe fn virt_to_pfn(kaddr: *const core::ffi::c_void) -> usize {
    __pa(kaddr as usize) >> PAGE_SHIFT
}

pub const fn MAP_NR(x: usize) -> usize {
    PFN_DOWN(x - PAGE_OFFSET - PHYS_OFFSET_OFFSET)
}

pub unsafe fn virt_to_page(x: usize) -> *mut page {
    mem_map.add(MAP_NR(x))
}

pub unsafe fn pfn_to_kaddr(x: usize) -> *mut core::ffi::c_void {
    __va(PFN_PHYS(x))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
