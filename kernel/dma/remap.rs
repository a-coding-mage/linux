// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 The Linux Foundation
 */

// Linux dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_struct {
    pub flags: usize,
    pub pages: *mut *mut page,
}

pub type pgprot_t = usize;

unsafe extern "C" {
    fn find_vm_area(addr: *mut core::ffi::c_void) -> *mut vm_struct;
    fn vmap(
        pages: *mut *mut page,
        count: usize,
        flags: usize,
        prot: pgprot_t,
    ) -> *mut core::ffi::c_void;
    fn vunmap(addr: *mut core::ffi::c_void);
    fn kvfree(ptr: *mut core::ffi::c_void);
    fn kvmalloc_objs(size: usize) -> *mut *mut page;
    fn warn(condition: i32, fmt: *const core::ffi::c_char, ...);
}

// These values and helpers are provided by the Linux DMA/vmalloc environment.
extern "C" {
    static VM_DMA_COHERENT: usize;
    static PAGE_SHIFT: usize;
}

#[inline]
unsafe fn page_align(size: usize) -> usize {
    let page_size = 1usize << PAGE_SHIFT;
    (size + page_size - 1) & !(page_size - 1)
}

pub unsafe fn dma_common_find_pages(cpu_addr: *mut core::ffi::c_void) -> *mut *mut page {
    let area = find_vm_area(cpu_addr);

    if area.is_null() || ((*area).flags & VM_DMA_COHERENT) == 0 {
        return core::ptr::null_mut();
    }
    warn(
        ((*area).flags != VM_DMA_COHERENT) as i32,
        c"unexpected flags in area: %p\n".as_ptr(),
        cpu_addr,
    );
    (*area).pages
}

/*
 * Remaps an array of PAGE_SIZE pages into another vm_area.
 * Cannot be used in non-sleeping contexts
 */
pub unsafe fn dma_common_pages_remap(
    pages: *mut *mut page,
    size: usize,
    prot: pgprot_t,
    _caller: *const core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let vaddr = vmap(pages, page_align(size) >> PAGE_SHIFT, VM_DMA_COHERENT, prot);
    if !vaddr.is_null() {
        (*find_vm_area(vaddr)).pages = pages;
    }
    vaddr
}

/*
 * Remaps an allocated contiguous region into another vm_area.
 * Cannot be used in non-sleeping contexts
 */
pub unsafe fn dma_common_contiguous_remap(
    mut page: *mut page,
    size: usize,
    prot: pgprot_t,
    _caller: *const core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let count = page_align(size) >> PAGE_SHIFT;
    let pages = kvmalloc_objs(count);
    if pages.is_null() {
        return core::ptr::null_mut();
    }
    for i in 0..count {
        *pages.add(i) = page;
        page = page.add(1);
    }
    let vaddr = vmap(pages, count, VM_DMA_COHERENT, prot);
    kvfree(pages.cast());

    vaddr
}

/*
 * Unmaps a range previously mapped by dma_common_*_remap
 */
pub unsafe fn dma_common_free_remap(cpu_addr: *mut core::ffi::c_void, _size: usize) {
    let area = find_vm_area(cpu_addr);

    if area.is_null() || ((*area).flags & VM_DMA_COHERENT) == 0 {
        warn(
            1,
            c"trying to free invalid coherent area: %p\n".as_ptr(),
            cpu_addr,
        );
        return;
    }

    vunmap(cpu_addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
