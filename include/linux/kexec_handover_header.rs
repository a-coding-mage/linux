/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/kexec_handover.h.
// The C includes and CONFIG_KEXEC_HANDOVER build condition are represented by
// references to the corresponding external Rust dependencies and cfg feature.

pub struct kho_vmalloc;

pub struct folio;
pub struct page;

#[cfg(feature = "CONFIG_KEXEC_HANDOVER")]
extern "C" {
    pub fn kho_is_enabled() -> bool;
    pub fn is_kho_boot() -> bool;

    pub fn kho_preserve_folio(folio: *mut folio) -> ::core::ffi::c_int;
    pub fn kho_unpreserve_folio(folio: *mut folio);
    pub fn kho_preserve_pages(page: *mut page, nr_pages: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn kho_unpreserve_pages(page: *mut page, nr_pages: ::core::ffi::c_ulong);
    pub fn kho_preserve_vmalloc(ptr: *mut ::core::ffi::c_void, preservation: *mut kho_vmalloc) -> ::core::ffi::c_int;
    pub fn kho_unpreserve_vmalloc(preservation: *mut kho_vmalloc);
    pub fn kho_alloc_preserve(size: usize) -> *mut ::core::ffi::c_void;
    pub fn kho_unpreserve_free(mem: *mut ::core::ffi::c_void);
    pub fn kho_restore_free(mem: *mut ::core::ffi::c_void);
    pub fn kho_restore_folio(phys: phys_addr_t) -> *mut folio;
    pub fn kho_restore_pages(phys: phys_addr_t, nr_pages: ::core::ffi::c_ulong) -> *mut page;
    pub fn kho_restore_vmalloc(preservation: *const kho_vmalloc) -> *mut ::core::ffi::c_void;
    pub fn kho_add_subtree(name: *const ::core::ffi::c_char, blob: *mut ::core::ffi::c_void, size: usize) -> ::core::ffi::c_int;
    pub fn kho_remove_subtree(blob: *mut ::core::ffi::c_void);
    pub fn kho_retrieve_subtree(name: *const ::core::ffi::c_char, phys: *mut phys_addr_t, size: *mut usize) -> ::core::ffi::c_int;

    pub fn kho_memory_init();
    pub fn kho_memory_init_early();
    pub fn kho_populate(fdt_phys: phys_addr_t, fdt_len: u64, scratch_phys: phys_addr_t, scratch_len: u64);
    pub fn kho_scratch_overlap(phys: phys_addr_t, size: usize) -> bool;
}

#[cfg(feature = "CONFIG_KEXEC_HANDOVER")]
#[inline]
pub unsafe fn kho_scratch_migratetype(pfn: ::core::ffi::c_ulong, mt: migratetype) -> migratetype {
    if kho_scratch_overlap(PFN_PHYS(pfn), pageblock_nr_pages << PAGE_SHIFT) {
        MIGRATE_CMA
    } else {
        mt
    }
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_is_enabled() -> bool { false }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn is_kho_boot() -> bool { false }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_preserve_folio(_folio: *mut folio) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_unpreserve_folio(_folio: *mut folio) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_preserve_pages(_page: *mut page, _nr_pages: ::core::ffi::c_uint) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_unpreserve_pages(_page: *mut page, _nr_pages: ::core::ffi::c_uint) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_preserve_vmalloc(_ptr: *mut ::core::ffi::c_void, _preservation: *mut kho_vmalloc) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_unpreserve_vmalloc(_preservation: *mut kho_vmalloc) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_alloc_preserve(_size: usize) -> *mut ::core::ffi::c_void { ERR_PTR(-EOPNOTSUPP) }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_unpreserve_free(_mem: *mut ::core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_restore_free(_mem: *mut ::core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_restore_folio(_phys: phys_addr_t) -> *mut folio { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_restore_pages(_phys: phys_addr_t, _nr_pages: ::core::ffi::c_uint) -> *mut page { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_restore_vmalloc(_preservation: *const kho_vmalloc) -> *mut ::core::ffi::c_void { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_add_subtree(_name: *const ::core::ffi::c_char, _blob: *mut ::core::ffi::c_void, _size: usize) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_remove_subtree(_blob: *mut ::core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_retrieve_subtree(_name: *const ::core::ffi::c_char, _phys: *mut phys_addr_t, _size: *mut usize) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_memory_init() {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_memory_init_early() {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_populate(_fdt_phys: phys_addr_t, _fdt_len: u64, _scratch_phys: phys_addr_t, _scratch_len: u64) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_scratch_overlap(_phys: phys_addr_t, _size: usize) -> bool { false }

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
#[inline]
pub fn kho_scratch_migratetype(_pfn: ::core::ffi::c_ulong, mt: migratetype) -> migratetype { mt }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
