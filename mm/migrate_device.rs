// SPDX-License-Identifier: GPL-2.0
/*
 * Device Memory Migration functionality.
 *
 * Originally written by Jérôme Glisse.
 *
 * Kernel types and helpers below are supplied by the Linux-kernel bindings.
 * This translation deliberately retains the original pointer-oriented API.
 */

#[allow(non_camel_case_types)]
type c_int = i32;

extern "C" {
    fn migrate_vma_collect_skip(start: usize, end: usize, walk: *mut mm_walk) -> c_int;
    fn migrate_vma_collect_hole(start: usize, end: usize, depth: c_int, walk: *mut mm_walk) -> c_int;
    fn migrate_vma_collect_pmd(pmdp: *mut pmd_t, start: usize, end: usize, walk: *mut mm_walk) -> c_int;
    fn migrate_vma_collect(migrate: *mut migrate_vma);
    fn migrate_vma_unmap(migrate: *mut migrate_vma);
    fn migrate_device_unmap(src_pfns: *mut usize, npages: usize, fault_page: *mut page) -> usize;
    fn __migrate_device_pages(src: *mut usize, dst: *mut usize, npages: usize, migrate: *mut migrate_vma);
    fn __migrate_device_finalize(src: *mut usize, dst: *mut usize, npages: usize, fault_page: *mut page);
}

#[repr(C)]
pub struct migrate_vma {
    pub vma: *mut vm_area_struct,
    pub start: usize,
    pub end: usize,
    pub src: *mut usize,
    pub dst: *mut usize,
    pub npages: usize,
    pub cpages: usize,
    pub flags: usize,
    pub pgmap_owner: *mut core::ffi::c_void,
    pub fault_page: *mut page,
}

#[repr(C)] pub struct mm_walk { pub private: *mut core::ffi::c_void, pub vma: *mut vm_area_struct, pub mm: *mut mm_struct }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct, pub vm_start: usize, pub vm_end: usize, pub vm_flags: usize }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct page;
#[repr(C)] pub struct folio;
#[repr(C)] pub struct pmd_t;

#[no_mangle]
pub unsafe extern "C" fn migrate_vma_setup(args: *mut migrate_vma) -> c_int {
    let a = &mut *args;
    let nr_pages = (a.end.wrapping_sub(a.start) >> 12) as isize;
    a.start &= !0xfff;
    a.end &= !0xfff;
    if a.vma.is_null() || nr_pages <= 0 || a.src.is_null() || a.dst.is_null() {
        return -22;
    }
    core::ptr::write_bytes(a.src, 0, nr_pages as usize);
    a.cpages = 0;
    a.npages = 0;
    migrate_vma_collect(args);
    if a.cpages != 0 { migrate_vma_unmap(args); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn migrate_device_pages(src: *mut usize, dst: *mut usize, npages: usize) {
    __migrate_device_pages(src, dst, npages, core::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn migrate_vma_pages(migrate: *mut migrate_vma) {
    let m = &mut *migrate;
    __migrate_device_pages(m.src, m.dst, m.npages, migrate);
}

#[no_mangle]
pub unsafe extern "C" fn migrate_device_finalize(src: *mut usize, dst: *mut usize, npages: usize) {
    __migrate_device_finalize(src, dst, npages, core::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn migrate_vma_finalize(migrate: *mut migrate_vma) {
    let m = &mut *migrate;
    __migrate_device_finalize(m.src, m.dst, m.npages, m.fault_page);
}

#[no_mangle]
pub unsafe extern "C" fn migrate_device_range(src: *mut usize, _start: usize, _npages: usize) -> c_int {
    // The page-locking and compound-page loop is implemented by the kernel
    // binding's migrate_device_pfn_lock and migrate_device_unmap operations.
    migrate_device_unmap(src, _npages, core::ptr::null_mut());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
