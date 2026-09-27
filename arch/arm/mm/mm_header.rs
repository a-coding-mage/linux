/* SPDX-License-Identifier: GPL-2.0 */

/* C conditional: CONFIG_MMU. */
#[cfg(CONFIG_MMU)]
extern "C" {
    /* the upper-most page table pointer */
    pub static mut top_pmd: *mut pmd_t;

    pub static mut icache_size: ::kernel::ffi::c_int;
}

/*
 * 0xffff8000 to 0xffffffff is reserved for any ARM architecture
 * specific hacks for copying pages efficiently, while 0xffff4000
 * is reserved for VIPT aliasing flushing by generic code.
 *
 * Note that we don't allow VIPT aliasing caches with SMP.
 */
pub const COPYPAGE_MINICACHE: ::kernel::ffi::c_ulong = 0xffff8000;
pub const COPYPAGE_V6_FROM: ::kernel::ffi::c_ulong = 0xffff8000;
pub const COPYPAGE_V6_TO: ::kernel::ffi::c_ulong = 0xffffc000;
/* PFN alias flushing, for VIPT caches */
pub const FLUSH_ALIAS_START: ::kernel::ffi::c_ulong = 0xffff4000;

#[cfg(CONFIG_MMU)]
#[inline]
pub unsafe fn set_top_pte(va: ::kernel::ffi::c_ulong, pte: pte_t) {
    let ptep: *mut pte_t = pte_offset_kernel(top_pmd, va);
    set_pte_ext(ptep, pte, 0);
    local_flush_tlb_kernel_page(va);
}

#[cfg(CONFIG_MMU)]
#[inline]
pub unsafe fn get_top_pte(va: ::kernel::ffi::c_ulong) -> pte_t {
    let ptep: *mut pte_t = pte_offset_kernel(top_pmd, va);
    *ptep
}

#[cfg(CONFIG_MMU)]
#[repr(C)]
pub struct mem_type {
    pub prot_pte: pteval_t,
    pub prot_pte_s2: pteval_t,
    pub prot_l1: pmdval_t,
    pub prot_sect: pmdval_t,
    pub domain: ::kernel::ffi::c_uint,
}

#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn get_mem_type(type_: ::kernel::ffi::c_uint) -> *const mem_type;
    pub fn __flush_dcache_folio(mapping: *mut address_space, folio: *mut folio);
}

/* ARM specific vm_struct->flags bits. */

/* (super)section-mapped I/O regions used by ioremap()/iounmap() */
pub const VM_ARM_SECTION_MAPPING: ::kernel::ffi::c_ulong = 0x80000000;

/* permanent static mappings from iotable_init() */
pub const VM_ARM_STATIC_MAPPING: ::kernel::ffi::c_ulong = 0x40000000;

/* empty mapping */
pub const VM_ARM_EMPTY_MAPPING: ::kernel::ffi::c_ulong = 0x20000000;

/* mapping type (attributes) for permanent static mappings */
#[inline]
pub const fn VM_ARM_MTYPE(mt: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong {
    mt << 20
}
pub const VM_ARM_MTYPE_MASK: ::kernel::ffi::c_ulong = 0x1f << 20;

#[cfg(CONFIG_MMU)]
#[repr(C)]
pub struct static_vm {
    pub vm: vm_struct,
    pub list: list_head,
}

#[cfg(CONFIG_MMU)]
extern "C" {
    pub static mut static_vmlist: list_head;
    pub fn find_static_vm_vaddr(vaddr: *mut ::kernel::ffi::c_void) -> *mut static_vm;
    pub fn add_static_vm_early(svm: *mut static_vm);
}

#[cfg(CONFIG_ZONE_DMA)]
extern "C" {
    pub static mut arm_dma_limit: phys_addr_t;
    pub static mut arm_dma_pfn_limit: ::kernel::ffi::c_ulong;
}

#[cfg(not(CONFIG_ZONE_DMA))]
pub const arm_dma_limit: phys_addr_t = !0 as phys_addr_t;

#[cfg(not(CONFIG_ZONE_DMA))]
pub const arm_dma_pfn_limit: ::kernel::ffi::c_ulong = (!0 as ::kernel::ffi::c_ulong) >> PAGE_SHIFT;

extern "C" {
    pub static mut arm_lowmem_limit: phys_addr_t;
    pub fn bootmem_init();
    pub fn arm_mm_memblock_reserve();
}

#[cfg(CONFIG_CMA_AREAS)]
extern "C" {
    pub fn dma_contiguous_remap();
}

#[cfg(not(CONFIG_CMA_AREAS))]
#[inline]
pub fn dma_contiguous_remap() {}

extern "C" {
    pub fn __clear_cr(mask: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
