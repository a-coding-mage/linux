/* SPDX-License-Identifier: GPL-2.0 */

/* Linux dependencies and the _X86_SGX_H header guard are represented by the
 * surrounding Rust translation unit. */

use core::ffi::c_void;

pub const EREMOVE_ERROR_MESSAGE: &str =
    "EREMOVE returned %d (0x%x) and an EPC page was leaked. SGX may become unusable. "
    "Refer to Documentation/arch/x86/sgx.rst for more information.";

pub const SGX_MAX_EPC_SECTIONS: usize = 8;
pub const SGX_EEXTEND_BLOCK_SIZE: usize = 256;
pub const SGX_NR_TO_SCAN: usize = 16;
pub const SGX_NR_LOW_PAGES: usize = 32;
pub const SGX_NR_HIGH_PAGES: usize = 64;

/* Pages, which are being tracked by the page reclaimer. */
pub const SGX_EPC_PAGE_RECLAIMER_TRACKED: u16 = 1 << 0;

/* Pages on free list */
pub const SGX_EPC_PAGE_IS_FREE: u16 = 1 << 1;

#[repr(C)]
pub struct sgx_epc_page {
    pub section: u32,
    pub flags: u16,
    pub poison: u16,
    pub owner: *mut sgx_encl_page,
    pub list: list_head,
}

/*
 * Contains the tracking data for NUMA nodes having EPC pages. Most importantly,
 * the free page list local to the node is stored here.
 */
#[repr(C)]
pub struct sgx_numa_node {
    pub free_page_list: list_head,
    pub sgx_poison_page_list: list_head,
    pub size: usize,
    pub lock: spinlock_t,
}

/*
 * The firmware can define multiple chunks of EPC to the different areas of the
 * physical memory e.g. for memory areas of the each node. This structure is
 * used to store EPC pages for one EPC section and virtual memory area where the
 * pages have been mapped.
 */
#[repr(C)]
pub struct sgx_epc_section {
    pub phys_addr: usize,
    pub virt_addr: *mut c_void,
    pub pages: *mut sgx_epc_page,
    pub node: *mut sgx_numa_node,
}

unsafe extern "C" {
    pub static mut sgx_epc_sections: [sgx_epc_section; SGX_MAX_EPC_SECTIONS];

    pub fn __sgx_alloc_epc_page() -> *mut sgx_epc_page;
    pub fn sgx_free_epc_page(page: *mut sgx_epc_page);

    pub fn sgx_reclaim_direct();
    pub fn sgx_mark_page_reclaimable(page: *mut sgx_epc_page);
    pub fn sgx_unmark_page_reclaimable(page: *mut sgx_epc_page) -> i32;
    pub fn sgx_alloc_epc_page(owner: *mut c_void, reclaim: bool) -> *mut sgx_epc_page;

    pub fn sgx_ipi_cb(info: *mut c_void);

    pub fn sgx_inc_usage_count() -> i32;
    pub fn sgx_dec_usage_count();

    pub fn sgx_update_lepubkeyhash(lepubkeyhash: *mut u64);
}

#[inline]
pub unsafe fn sgx_get_epc_phys_addr(page: *mut sgx_epc_page) -> usize {
    let section = &sgx_epc_sections[(*page).section as usize];
    let index = (page as usize - section.pages as usize) / core::mem::size_of::<sgx_epc_page>();

    section.phys_addr + index * PAGE_SIZE
}

#[inline]
pub unsafe fn sgx_get_epc_virt_addr(page: *mut sgx_epc_page) -> *mut c_void {
    let section = &sgx_epc_sections[(*page).section as usize];
    let index = (page as usize - section.pages as usize) / core::mem::size_of::<sgx_epc_page>();

    (section.virt_addr as usize + index * PAGE_SIZE) as *mut c_void
}

#[cfg(feature = "CONFIG_X86_SGX_KVM")]
pub unsafe fn sgx_vepc_init() -> i32;

#[cfg(not(feature = "CONFIG_X86_SGX_KVM"))]
#[inline]
pub fn sgx_vepc_init() -> i32 {
    -ENODEV
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
