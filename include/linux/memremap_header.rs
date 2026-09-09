/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* Types and helpers below are supplied by the corresponding Linux headers. */

#[repr(C)]
pub struct vmem_altmap {
    pub base_pfn: ::core::ffi::c_ulong,
    pub end_pfn: ::core::ffi::c_ulong,
    pub reserve: ::core::ffi::c_ulong,
    pub free: ::core::ffi::c_ulong,
    pub align: ::core::ffi::c_ulong,
    pub alloc: ::core::ffi::c_ulong,
}

#[repr(i32)]
pub enum memory_type {
    MEMORY_DEVICE_PRIVATE = 1,
    MEMORY_DEVICE_COHERENT,
    MEMORY_DEVICE_FS_DAX,
    MEMORY_DEVICE_GENERIC,
    MEMORY_DEVICE_PCI_P2PDMA,
}

#[repr(C)]
pub struct dev_pagemap_ops {
    pub folio_free: Option<unsafe extern "C" fn(folio: *mut folio)>,
    pub migrate_to_ram: Option<unsafe extern "C" fn(vmf: *mut vm_fault) -> vm_fault_t>,
    pub memory_failure: Option<unsafe extern "C" fn(
        pgmap: *mut dev_pagemap,
        pfn: ::core::ffi::c_ulong,
        nr_pages: ::core::ffi::c_ulong,
        mf_flags: i32,
    ) -> i32>,
    pub folio_split: Option<unsafe extern "C" fn(head: *mut folio, tail: *mut folio)>,
}

pub const PGMAP_ALTMAP_VALID: u32 = 1 << 0;

#[repr(C)]
pub union dev_pagemap_range {
    pub range: range,
    pub ranges: [range; 0],
}

#[repr(C)]
pub struct dev_pagemap {
    pub altmap: vmem_altmap,
    pub ref_: percpu_ref,
    pub done: completion,
    pub type_: memory_type,
    pub flags: u32,
    pub vmemmap_shift: ::core::ffi::c_ulong,
    pub ops: *const dev_pagemap_ops,
    pub owner: *mut c_void,
    pub nr_range: i32,
    pub range: dev_pagemap_range,
}

#[inline]
pub unsafe fn pgmap_has_memory_failure(pgmap: *mut dev_pagemap) -> bool {
    !(*pgmap).ops.is_null() && (*(*pgmap).ops).memory_failure.is_some()
}

#[inline]
pub unsafe fn pgmap_altmap(pgmap: *mut dev_pagemap) -> *mut vmem_altmap {
    if (*pgmap).flags & PGMAP_ALTMAP_VALID != 0 {
        &mut (*pgmap).altmap
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn pgmap_vmemmap_nr(pgmap: *mut dev_pagemap) -> ::core::ffi::c_ulong {
    1 << (*pgmap).vmemmap_shift
}

#[inline]
pub unsafe fn folio_is_device_private(folio: *const folio) -> bool {
    IS_ENABLED(CONFIG_DEVICE_PRIVATE) && folio_is_zone_device(folio)
        && (*(*folio).pgmap).type_ == memory_type::MEMORY_DEVICE_PRIVATE
}

#[inline]
pub unsafe fn is_device_private_page(page: *const page) -> bool {
    IS_ENABLED(CONFIG_DEVICE_PRIVATE) && folio_is_device_private(page_folio(page))
}

#[inline]
pub unsafe fn folio_is_pci_p2pdma(folio: *const folio) -> bool {
    IS_ENABLED(CONFIG_PCI_P2PDMA) && folio_is_zone_device(folio)
        && (*(*folio).pgmap).type_ == memory_type::MEMORY_DEVICE_PCI_P2PDMA
}

#[inline]
pub unsafe fn folio_zone_device_data(folio: *const folio) -> *mut c_void {
    VM_WARN_ON_FOLIO(!folio_is_device_private(folio), folio);
    (*folio).page.zone_device_data
}

#[inline]
pub unsafe fn folio_set_zone_device_data(folio: *mut folio, data: *mut c_void) {
    VM_WARN_ON_FOLIO(!folio_is_device_private(folio), folio);
    (*folio).page.zone_device_data = data;
}

#[inline]
pub unsafe fn is_pci_p2pdma_page(page: *const page) -> bool {
    IS_ENABLED(CONFIG_PCI_P2PDMA) && folio_is_pci_p2pdma(page_folio(page))
}

#[inline]
pub unsafe fn folio_is_device_coherent(folio: *const folio) -> bool {
    folio_is_zone_device(folio)
        && (*(*folio).pgmap).type_ == memory_type::MEMORY_DEVICE_COHERENT
}

#[inline]
pub unsafe fn is_device_coherent_page(page: *const page) -> bool {
    folio_is_device_coherent(page_folio(page))
}

#[inline]
pub unsafe fn folio_is_fsdax(folio: *const folio) -> bool {
    folio_is_zone_device(folio)
        && (*(*folio).pgmap).type_ == memory_type::MEMORY_DEVICE_FS_DAX
}

#[inline]
pub unsafe fn is_fsdax_page(page: *const page) -> bool {
    folio_is_fsdax(page_folio(page))
}

/* CONFIG_ZONE_DEVICE-enabled declarations. */
extern "C" {
    pub fn zone_device_page_init(page: *mut page, pgmap: *mut dev_pagemap, order: u32);
    pub fn memremap_pages(pgmap: *mut dev_pagemap, nid: i32) -> *mut c_void;
    pub fn memunmap_pages(pgmap: *mut dev_pagemap);
    pub fn devm_memremap_pages(dev: *mut device, pgmap: *mut dev_pagemap) -> *mut c_void;
    pub fn devm_memunmap_pages(dev: *mut device, pgmap: *mut dev_pagemap);
    pub fn get_dev_pagemap(pfn: ::core::ffi::c_ulong) -> *mut dev_pagemap;
    pub fn pgmap_pfn_valid(pgmap: *mut dev_pagemap, pfn: ::core::ffi::c_ulong) -> bool;
    pub fn memremap_compat_align() -> ::core::ffi::c_ulong;
}

#[inline]
pub unsafe fn zone_device_folio_init(folio: *mut folio, pgmap: *mut dev_pagemap, order: u32) {
    zone_device_page_init(&mut (*folio).page, pgmap, order);
    if order != 0 {
        folio_set_large_rmappable(folio);
    }
}

#[inline]
pub unsafe fn zone_device_private_split_cb(original_folio: *mut folio, new_folio: *mut folio) {
    if folio_is_device_private(original_folio) {
        if (*(*original_folio).pgmap).ops.is_null()
            || (*(*(*original_folio).pgmap).ops).folio_split.is_none()
        {
            if !new_folio.is_null() {
                (*new_folio).pgmap = (*original_folio).pgmap;
                (*new_folio).page.mapping = (*original_folio).page.mapping;
            }
        } else {
            ((*(*(*original_folio).pgmap).ops).folio_split.unwrap())(original_folio, new_folio);
        }
    }
}

/* CONFIG_ZONE_DEVICE-disabled fallbacks. */
#[inline]
pub unsafe fn devm_memremap_pages_disabled(_dev: *mut device, _pgmap: *mut dev_pagemap) -> *mut c_void {
    WARN_ON_ONCE(1);
    ERR_PTR(-ENXIO)
}

#[inline]
pub unsafe fn devm_memunmap_pages_disabled(_dev: *mut device, _pgmap: *mut dev_pagemap) {}

#[inline]
pub unsafe fn get_dev_pagemap_disabled(_pfn: ::core::ffi::c_ulong) -> *mut dev_pagemap { core::ptr::null_mut() }

#[inline]
pub unsafe fn pgmap_pfn_valid_disabled(_pgmap: *mut dev_pagemap, _pfn: ::core::ffi::c_ulong) -> bool { false }

#[inline]
pub unsafe fn memremap_compat_align_disabled() -> ::core::ffi::c_ulong { PAGE_SIZE }

#[inline]
pub unsafe fn zone_device_private_split_cb_disabled(_original_folio: *mut folio, _new_folio: *mut folio) {}

#[inline]
pub unsafe fn put_dev_pagemap(pgmap: *mut dev_pagemap) {
    if !pgmap.is_null() {
        percpu_ref_put(&mut (*pgmap).ref_);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
