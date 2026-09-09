/* SPDX-License-Identifier: GPL-2.0 */
/**
 * Copyright(c) 2016-20 Intel Corporation.
 *
 * Contains the software defined data structures for enclaves.
 */

// C header dependencies are supplied by the surrounding translation unit.

/* 'desc' bits holding the offset in the VA (version array) page. */
pub const SGX_ENCL_PAGE_VA_OFFSET_MASK: u64 = 0x0000_0fff & !0x7;

/* 'desc' bit marking that the page is being reclaimed. */
pub const SGX_ENCL_PAGE_BEING_RECLAIMED: u64 = 1u64 << 3;

#[repr(C)]
pub struct sgx_encl_page {
    pub desc: ::core::ffi::c_ulong,
    pub vm_max_prot_bits: u8,
    pub type_: sgx_page_type,
    pub epc_page: *mut sgx_epc_page,
    pub encl: *mut sgx_encl,
    pub va_page: *mut sgx_va_page,
}

#[repr(u32)]
pub enum sgx_encl_flags {
    SGX_ENCL_IOCTL = 1u32 << 0,
    SGX_ENCL_DEBUG = 1u32 << 1,
    SGX_ENCL_CREATED = 1u32 << 2,
    SGX_ENCL_INITIALIZED = 1u32 << 3,
}

#[repr(C)]
pub struct sgx_encl_mm {
    pub encl: *mut sgx_encl,
    pub mm: *mut mm_struct,
    pub list: list_head,
    pub mmu_notifier: mmu_notifier,
}

#[repr(C)]
pub struct sgx_encl {
    pub base: ::core::ffi::c_ulong,
    pub size: ::core::ffi::c_ulong,
    pub flags: ::core::ffi::c_ulong,
    pub page_cnt: u32,
    pub secs_child_cnt: u32,
    pub lock: mutex,
    pub page_array: xarray,
    pub secs: sgx_encl_page,
    pub attributes: ::core::ffi::c_ulong,
    pub attributes_mask: ::core::ffi::c_ulong,
    pub cpumask: cpumask_t,
    pub backing: *mut file,
    pub refcount: kref,
    pub va_pages: list_head,
    pub mm_list_version: ::core::ffi::c_ulong,
    pub mm_list: list_head,
    pub mm_lock: spinlock_t,
    pub srcu: srcu_struct,
}

pub const SGX_VA_SLOT_COUNT: usize = 512;

#[repr(C)]
pub struct sgx_va_page {
    pub epc_page: *mut sgx_epc_page,
    pub slots: [usize; SGX_VA_SLOT_COUNT / (usize::BITS as usize)],
    pub list: list_head,
}

#[repr(C)]
pub struct sgx_backing {
    pub contents: *mut page,
    pub pcmd: *mut page,
    pub pcmd_offset: ::core::ffi::c_ulong,
}

extern "C" {
    pub static sgx_vm_ops: vm_operations_struct;

    pub fn vma_lookup(mm: *mut mm_struct, addr: ::core::ffi::c_ulong)
        -> *mut vm_area_struct;

    pub fn sgx_encl_may_map(
        encl: *mut sgx_encl,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        vm_flags: vm_flags_t,
    ) -> i32;
    pub fn current_is_ksgxd() -> bool;
    pub fn sgx_encl_release(ref_: *mut kref);
    pub fn sgx_encl_mm_add(encl: *mut sgx_encl, mm: *mut mm_struct) -> i32;
    pub fn sgx_encl_cpumask(encl: *mut sgx_encl) -> *const cpumask_t;
    pub fn sgx_encl_alloc_backing(
        encl: *mut sgx_encl,
        page_index: ::core::ffi::c_ulong,
        backing: *mut sgx_backing,
    ) -> i32;
    pub fn sgx_encl_put_backing(backing: *mut sgx_backing);
    pub fn sgx_encl_test_and_clear_young(mm: *mut mm_struct, page: *mut sgx_encl_page) -> i32;
    pub fn sgx_encl_page_alloc(
        encl: *mut sgx_encl,
        offset: ::core::ffi::c_ulong,
        secinfo_flags: u64,
    ) -> *mut sgx_encl_page;
    pub fn sgx_zap_enclave_ptes(encl: *mut sgx_encl, addr: ::core::ffi::c_ulong);
    pub fn sgx_alloc_va_page(reclaim: bool) -> *mut sgx_epc_page;
    pub fn sgx_alloc_va_slot(va_page: *mut sgx_va_page) -> u32;
    pub fn sgx_free_va_slot(va_page: *mut sgx_va_page, offset: u32);
    pub fn sgx_va_page_full(va_page: *mut sgx_va_page) -> bool;
    pub fn sgx_encl_free_epc_page(page: *mut sgx_epc_page);
    pub fn sgx_encl_load_page(encl: *mut sgx_encl, addr: ::core::ffi::c_ulong)
        -> *mut sgx_encl_page;
    pub fn sgx_encl_grow(encl: *mut sgx_encl, reclaim: bool) -> *mut sgx_va_page;
    pub fn sgx_encl_shrink(encl: *mut sgx_encl, va_page: *mut sgx_va_page);
}

#[inline]
pub unsafe fn sgx_encl_find(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
    vma: *mut *mut vm_area_struct,
) -> i32 {
    let result = vma_lookup(mm, addr);
    if result.is_null() || (*result).vm_ops != &sgx_vm_ops {
        return -22;
    }
    *vma = result;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
