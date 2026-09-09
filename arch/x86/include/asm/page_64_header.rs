/* SPDX-License-Identifier: GPL-2.0 */
// Translated from asm/page_64.h.  The included architecture definitions are
// supplied by other translation units.

#[cfg(not(feature = "assembler"))]
extern "C" {
    pub static mut max_pfn: usize;
    pub static mut phys_base: usize;
    pub static mut page_offset_base: usize;
    pub static mut vmalloc_base: usize;
    pub static mut vmemmap_base: usize;
    pub static mut direct_map_physmem_end: usize;
}

#[cfg(not(feature = "debug_virtual"))]
#[inline(always)]
pub unsafe fn __phys_addr_nodebug(mut x: usize) -> usize {
    let y = x.wrapping_sub(__START_KERNEL_map);
    // Use the carry flag equivalent to determine whether x was below the map.
    x = y.wrapping_add(if x > y {
        phys_base
    } else {
        __START_KERNEL_map.wrapping_sub(PAGE_OFFSET)
    });
    x
}

#[cfg(feature = "debug_virtual")]
extern "C" {
    pub fn __phys_addr(x: usize) -> usize;
}

#[cfg(not(feature = "debug_virtual"))]
#[inline(always)]
pub unsafe fn __phys_addr(x: usize) -> usize {
    __phys_addr_nodebug(x)
}

#[inline]
pub unsafe fn __phys_addr_symbol(x: usize) -> usize {
    let y = x.wrapping_sub(__START_KERNEL_map);
    // VIRTUAL_BUG_ON(y >= KERNEL_IMAGE_SIZE);
    debug_assert!(y < KERNEL_IMAGE_SIZE);
    y.wrapping_add(phys_base)
}

#[inline(always)]
pub const fn __phys_reloc_hide(x: usize) -> usize { x }

extern "C" {
    pub fn __clear_pages_unrolled(page: *mut core::ffi::c_void);
    pub fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
    pub fn kmsan_unpoison_memory(addr: *mut core::ffi::c_void, len: u64);
}

// KCFI_REFERENCE(__clear_pages_unrolled);
// KCFI_REFERENCE(copy_page);

/**
 * clear_pages() - clear a page range using a kernel virtual address.
 * The C implementation selects __clear_pages_unrolled, REP STOSQ, or REP
 * STOSB through ALTERNATIVE_2 according to CPU capabilities.
 */
#[inline]
pub unsafe fn clear_pages(mut addr: *mut core::ffi::c_void, npages: u32) {
    let len = (npages as u64).wrapping_mul(PAGE_SIZE as u64);
    // Clean up KMSAN metadata before the assembly call clobbers addr.
    kmsan_unpoison_memory(addr, len);
    // The original uses volatile inline assembly with ALTERNATIVE_2:
    // call __clear_pages_unrolled / rep stosq / rep stosb.
    __clear_pages_unrolled(addr);
    let _ = &mut addr;
}

#[inline]
pub unsafe fn clear_page(addr: *mut core::ffi::c_void) {
    clear_pages(addr, 1);
}

// User space process size; ALTERNATIVE selects the LA57 or non-LA57 limit.
#[inline(always)]
pub unsafe fn task_size_max() -> usize {
    // alternative_io("movq %[small],%0", "movq %[large],%0", X86_FEATURE_LA57, ...)
    if cfg!(feature = "x86_la57") {
        (1usize << 56).wrapping_sub(PAGE_SIZE)
    } else {
        (1usize << 47).wrapping_sub(PAGE_SIZE)
    }
}

// #ifdef CONFIG_X86_VSYSCALL_EMULATION
// #define __HAVE_ARCH_GATE_AREA 1
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
