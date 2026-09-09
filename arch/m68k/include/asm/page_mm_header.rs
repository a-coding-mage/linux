/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from m68k/include/asm/page_mm.h. */

/* The original header includes compiler and module definitions. */

#[cfg(CPU_M68040_OR_M68060_ONLY)]
#[inline]
pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void) {
    /* The original implementation is m68k 68040 `move16` inline assembly. */
    core::arch::asm!(
        "1:",
        ".chip 68040",
        "move16 {from}@+,{to}@+",
        "move16 {from}@+,{to}@+",
        ".chip 68k",
        "dbra {tmp},1b",
        to = inout(reg) to => _,
        from = inout(reg) from => _,
        tmp = inout(reg) (PAGE_SIZE / 32 - 1) => _,
        options(nostack)
    );
}

#[cfg(CPU_M68040_OR_M68060_ONLY)]
#[inline]
pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    let mut sp = page as *mut usize;
    *sp = 0; sp = sp.add(1);
    *sp = 0; sp = sp.add(1);
    *sp = 0; sp = sp.add(1);
    *sp = 0; sp = sp.add(1);
    let mut tmp: usize;
    core::arch::asm!(
        "1:",
        ".chip 68040",
        "move16 {page}@+,{sp}@+",
        ".chip 68k",
        "subqw #8,{page}",
        "subqw #8,{page}",
        "dbra {tmp},1b",
        sp = inout(reg) sp => _,
        tmp = inout(reg) ((PAGE_SIZE - 16) / 16 - 1) => _,
        page = inout(reg) page => _,
        options(nostack)
    );
}

#[cfg(not(CPU_M68040_OR_M68060_ONLY))]
#[inline]
pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    core::ptr::write_bytes(page, 0, PAGE_SIZE);
}

#[cfg(not(CPU_M68040_OR_M68060_ONLY))]
#[inline]
pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, PAGE_SIZE);
}

#[inline]
pub unsafe fn clear_user_page(addr: *mut core::ffi::c_void, vaddr: usize, page: *mut Page) {
    clear_page(addr);
    flush_dcache_page(page);
    let _ = vaddr;
}

#[inline]
pub unsafe fn copy_user_page(to: *mut core::ffi::c_void, from: *const core::ffi::c_void,
                             vaddr: usize, page: *mut Page) {
    copy_page(to, from);
    flush_dcache_page(page);
    let _ = vaddr;
}

extern "C" {
    pub static mut m68k_memoffset: c_ulong;
    pub static mut m68k_virt_to_node_shift: c_int;
    pub static mut high_memory: *mut core::ffi::c_void;
    pub fn flush_dcache_page(page: *mut Page);
    pub fn pfn_to_page(pfn: c_ulong) -> *mut Page;
    pub fn page_to_pfn(page: *mut Page) -> c_ulong;
}

#[repr(C)]
pub struct Page {
    _private: [u8; 0],
}

pub type c_ulong = usize;
pub type c_int = i32;

#[cfg(not(CONFIG_SUN3))]
#[allow(non_upper_case_globals)]
pub const WANT_PAGE_VIRTUAL: bool = true;

#[cfg(not(CONFIG_SUN3))]
#[inline]
pub unsafe fn ___pa(vaddr: *mut core::ffi::c_void) -> c_ulong {
    /* m68k_fixup inline assembly performs the architecture-specific conversion. */
    vaddr as c_ulong
}

#[cfg(not(CONFIG_SUN3))]
#[inline]
pub unsafe fn __pa(vaddr: usize) -> c_ulong { ___pa(vaddr as *mut core::ffi::c_void) }

#[cfg(not(CONFIG_SUN3))]
#[inline]
pub unsafe fn __va(paddr: c_ulong) -> *mut core::ffi::c_void {
    paddr as *mut core::ffi::c_void
}

#[cfg(CONFIG_SUN3)]
#[inline]
pub unsafe fn __pa(x: usize) -> c_ulong { ___pa(x) }

#[cfg(CONFIG_SUN3)]
#[inline]
pub unsafe fn ___pa(x: c_ulong) -> c_ulong {
    if x == 0 { 0 } else if x >= PAGE_OFFSET { x - PAGE_OFFSET } else { x + 0x2000000 }
}

#[cfg(CONFIG_SUN3)]
#[inline]
pub unsafe fn __va(x: c_ulong) -> *mut core::ffi::c_void {
    if x == 0 { core::ptr::null_mut() }
    else if x < 0x2000000 { (x + PAGE_OFFSET) as *mut core::ffi::c_void }
    else { (x - 0x2000000) as *mut core::ffi::c_void }
}

#[inline]
pub unsafe fn virt_to_pfn(kaddr: *const core::ffi::c_void) -> c_ulong {
    __pa(kaddr as usize) >> PAGE_SHIFT
}

#[inline]
pub unsafe fn pfn_to_virt(pfn: c_ulong) -> *mut core::ffi::c_void {
    __va(pfn << PAGE_SHIFT)
}

#[inline]
pub unsafe fn virt_to_page(addr: *const core::ffi::c_void) -> *mut Page {
    pfn_to_page(virt_to_pfn(addr))
}

#[inline]
pub unsafe fn page_to_virt(page: *mut Page) -> *mut core::ffi::c_void {
    pfn_to_virt(page_to_pfn(page))
}

/* ARCH_PFN_OFFSET is m68k_memory[0].addr >> PAGE_SHIFT. */
pub const ARCH_PFN_OFFSET: c_ulong = 0; // m68k_memory[0].addr >> PAGE_SHIFT

#[inline]
pub unsafe fn virt_addr_valid(kaddr: usize) -> bool {
    kaddr >= PAGE_OFFSET && kaddr < high_memory as usize
}

#[inline]
pub unsafe fn pfn_valid(pfn: c_ulong) -> bool {
    virt_addr_valid(pfn_to_virt(pfn) as usize)
}

/* PAGE_SIZE, PAGE_SHIFT, and PAGE_OFFSET are supplied by dependent headers. */
extern "C" {
    static PAGE_SIZE: usize;
    static PAGE_SHIFT: u32;
    static PAGE_OFFSET: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
