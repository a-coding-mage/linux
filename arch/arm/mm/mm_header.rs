/* SPDX-License-Identifier: GPL-2.0 */

/* C conditional: CONFIG_MMU. */
#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    /* the upper-most page table pointer */
    pub static mut top_pmd: *mut pmd_t;

    pub static mut icache_size: ::core::ffi::c_int;
}

/*
 * 0xffff8000 to 0xffffffff is reserved for any ARM architecture
 * specific hacks for copying pages efficiently, while 0xffff4000
 * is reserved for VIPT aliasing flushing by generic code.
 *
 * Note that we don't allow VIPT aliasing caches with SMP.
 */
pub const COPYPAGE_MINICACHE: ::core::ffi::c_ulong = 0xffff8000;
pub const COPYPAGE_V6_FROM: ::core::ffi::c_ulong = 0xffff8000;
pub const COPYPAGE_V6_TO: ::core::ffi::c_ulong = 0xffffc000;
/* PFN alias flushing, for VIPT caches */
pub const FLUSH_ALIAS_START: ::core::ffi::c_ulong = 0xffff4000;

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn set_top_pte(va: ::core::ffi::c_ulong, pte: pte_t) {
    let ptep: *mut pte_t = pte_offset_kernel(top_pmd, va);
    set_pte_ext(ptep, pte, 0);
    local_flush_tlb_kernel_page(va);
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn get_top_pte(va: ::core::ffi::c_ulong) -> pte_t {
    let ptep: *mut pte_t = pte_offset_kernel(top_pmd, va);
    *ptep
}

#[cfg(feature = "CONFIG_MMU")]
#[repr(C)]
pub struct mem_type {
    pub prot_pte: pteval_t,
    pub prot_pte_s2: pteval_t,
    pub prot_l1: pmdval_t,
    pub prot_sect: pmdval_t,
    pub domain: ::core::ffi::c_uint,
}

#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    pub fn get_mem_type(type_: ::core::ffi::c_uint) -> *const mem_type;
    pub fn __flush_dcache_folio(mapping: *mut address_space, folio: *mut folio);
}

/* ARM specific vm_struct->flags bits. */

/* (super)section-mapped I/O regions used by ioremap()/iounmap() */
pub const VM_ARM_SECTION_MAPPING: ::core::ffi::c_ulong = 0x80000000;

/* permanent static mappings from iotable_init() */
pub const VM_ARM_STATIC_MAPPING: ::core::ffi::c_ulong = 0x40000000;

/* empty mapping */
pub const VM_ARM_EMPTY_MAPPING: ::core::ffi::c_ulong = 0x20000000;

/* mapping type (attributes) for permanent static mappings */
#[inline]
pub const fn VM_ARM_MTYPE(mt: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    mt << 20
}
pub const VM_ARM_MTYPE_MASK: ::core::ffi::c_ulong = 0x1f << 20;

#[cfg(feature = "CONFIG_MMU")]
#[repr(C)]
pub struct static_vm {
    pub vm: vm_struct,
    pub list: list_head,
}

#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    pub static mut static_vmlist: list_head;
    pub fn find_static_vm_vaddr(vaddr: *mut ::core::ffi::c_void) -> *mut static_vm;
    pub fn add_static_vm_early(svm: *mut static_vm);
}

#[cfg(feature = "CONFIG_ZONE_DMA")]
extern "C" {
    pub static mut arm_dma_limit: phys_addr_t;
    pub static mut arm_dma_pfn_limit: ::core::ffi::c_ulong;
}

#[cfg(not(feature = "CONFIG_ZONE_DMA"))]
pub const arm_dma_limit: phys_addr_t = !0 as phys_addr_t;

#[cfg(not(feature = "CONFIG_ZONE_DMA"))]
pub const arm_dma_pfn_limit: ::core::ffi::c_ulong = (!0 as ::core::ffi::c_ulong) >> PAGE_SHIFT;

extern "C" {
    pub static mut arm_lowmem_limit: phys_addr_t;
    pub fn bootmem_init();
    pub fn arm_mm_memblock_reserve();
}

#[cfg(feature = "CONFIG_CMA_AREAS")]
extern "C" {
    pub fn dma_contiguous_remap();
}

#[cfg(not(feature = "CONFIG_CMA_AREAS"))]
#[inline]
pub fn dma_contiguous_remap() {}

extern "C" {
    pub fn __clear_cr(mask: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
