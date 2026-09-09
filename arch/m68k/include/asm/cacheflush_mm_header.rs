/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/mm.h and, when CONFIG_COLDFIRE is enabled, asm/mcfsim.h.

pub const FLUSH_I_AND_D: u32 = 0x0000_0808;
pub const FLUSH_I: u32 = 0x0000_0008;

#[cfg(not(any()))]
const _UNUSED: () = ();

// These build-time definitions are supplied externally when present.
// Defaults from the C header are zero.
pub const ICACHE_MAX_ADDR: usize = 0;
pub const ICACHE_SET_MASK: usize = 0;
pub const DCACHE_MAX_ADDR: usize = 0;
pub const DCACHE_SETMASK: usize = 0;
pub const CACHE_MODE: usize = 0;
pub const CACR_ICINVA: usize = 0;
pub const CACR_DCINVA: usize = 0;
pub const CACR_BCINVA: usize = 0;

/*
 * ColdFire architecture has no way to clear individual cache lines, so we
 * are stuck invalidating all the cache entries when we want a clear operation.
 */
#[inline]
pub unsafe fn clear_cf_icache(_start: usize, _end: usize) {
    core::arch::asm!("movec {0},cacr", "nop", in(reg) (CACHE_MODE | CACR_ICINVA | CACR_BCINVA));
}

#[inline]
pub unsafe fn clear_cf_dcache(_start: usize, _end: usize) {
    core::arch::asm!("movec {0},cacr", "nop", in(reg) (CACHE_MODE | CACR_DCINVA));
}

#[inline]
pub unsafe fn clear_cf_bcache(_start: usize, _end: usize) {
    core::arch::asm!("movec {0},cacr", "nop", in(reg) (CACHE_MODE | CACR_ICINVA | CACR_BCINVA | CACR_DCINVA));
}

/* Use the ColdFire cpushl instruction to push (and invalidate) cache lines. */
#[inline]
pub unsafe fn flush_cf_icache(start: usize, end: usize) {
    let mut set = start;
    while set <= end {
        core::arch::asm!(
            "cpushl ic,({0})", "addq.l #1,{0}",
            "cpushl ic,({0})", "addq.l #1,{0}",
            "cpushl ic,({0})", "addq.l #1,{0}", "cpushl ic,({0})",
            inout(reg) set,
        );
        set = set.wrapping_add(0x10 - 3);
    }
}

#[inline]
pub unsafe fn flush_cf_dcache(start: usize, end: usize) {
    let mut set = start;
    while set <= end {
        core::arch::asm!(
            "cpushl dc,({0})", "addq.l #1,{0}",
            "cpushl dc,({0})", "addq.l #1,{0}",
            "cpushl dc,({0})", "addq.l #1,{0}", "cpushl dc,({0})",
            inout(reg) set,
        );
        set = set.wrapping_add(0x10 - 3);
    }
}

#[inline]
pub unsafe fn flush_cf_bcache(start: usize, end: usize) {
    let mut set = start;
    while set <= end {
        core::arch::asm!(
            "cpushl bc,({0})", "addq.l #1,{0}",
            "cpushl bc,({0})", "addq.l #1,{0}",
            "cpushl bc,({0})", "addq.l #1,{0}", "cpushl bc,({0})",
            inout(reg) set,
        );
        set = set.wrapping_add(0x10 - 3);
    }
}

/* Cache handling functions. CPU_IS_* and kernel types are external symbols. */
#[inline]
pub unsafe fn flush_icache() {
    if CPU_IS_COLDFIRE {
        flush_cf_icache(0, ICACHE_MAX_ADDR);
    } else if CPU_IS_040_OR_060 {
        core::arch::asm!("nop", ".chip 68040", "cpusha %bc", ".chip 68k");
    } else {
        let mut tmp: usize;
        core::arch::asm!("movec cacr,{0}", "or.w {1},{0}", "movec {0},cacr", out(reg) tmp, in(reg) FLUSH_I);
        let _ = tmp;
    }
}

extern "C" {
    pub fn cache_clear(paddr: usize, len: i32);
    pub fn cache_push(paddr: usize, len: i32);
    pub fn cache_push_v(vaddr: usize, len: i32);
    pub fn flush_icache_user_page(vma: *mut vm_area_struct, page: *mut page, addr: usize, len: i32);
    pub fn flush_icache_range(address: usize, endaddr: usize);
    pub fn flush_icache_user_range(address: usize, endaddr: usize);
}

#[inline]
pub unsafe fn __flush_cache_all() {
    if CPU_IS_COLDFIRE { flush_cf_dcache(0, DCACHE_MAX_ADDR); }
    else if CPU_IS_040_OR_060 { core::arch::asm!("nop", ".chip 68040", "cpusha %dc", ".chip 68k"); }
    else { let mut tmp: usize; core::arch::asm!("movec cacr,{0}", "or.w {1},{0}", "movec {0},cacr", out(reg) tmp, in(reg) FLUSH_I_AND_D); let _ = tmp; }
}

#[inline]
pub unsafe fn __flush_cache_030() {
    if CPU_IS_020_OR_030 { let mut tmp: usize; core::arch::asm!("movec cacr,{0}", "or.w {1},{0}", "movec {0},cacr", out(reg) tmp, in(reg) FLUSH_I_AND_D); let _ = tmp; }
}

#[inline] pub unsafe fn flush_cache_all() { __flush_cache_all(); }
#[inline] pub unsafe fn flush_cache_vmap(_start: usize, _end: usize) { flush_cache_all(); }
#[inline] pub unsafe fn flush_cache_vmap_early(_start: usize, _end: usize) {}
#[inline] pub unsafe fn flush_cache_vunmap(_start: usize, _end: usize) { flush_cache_all(); }

#[inline]
pub unsafe fn flush_cache_mm(mm: *mut mm_struct) { if (*mm).mm == current.mm { __flush_cache_030(); } }
#[inline] pub unsafe fn flush_cache_dup_mm(mm: *mut mm_struct) { flush_cache_mm(mm); }

#[inline]
pub unsafe fn flush_cache_range(vma: *mut vm_area_struct, _start: usize, _end: usize) { if (*vma).vm_mm == current.mm { __flush_cache_030(); } }
#[inline]
pub unsafe fn flush_cache_page(vma: *mut vm_area_struct, _vmaddr: usize, _pfn: usize) { if (*vma).vm_mm == current.mm { __flush_cache_030(); } }

#[inline]
pub unsafe fn __flush_pages_to_ram(vaddr: *mut core::ffi::c_void, mut nr: u32) {
    if CPU_IS_COLDFIRE {
        let addr = vaddr as usize & !(PAGE_SIZE - 1);
        let mut start = addr & ICACHE_SET_MASK;
        let mut end = (addr + nr as usize * PAGE_SIZE - 1) & ICACHE_SET_MASK;
        if start > end { flush_cf_bcache(0, end); end = ICACHE_MAX_ADDR; }
        flush_cf_bcache(start, end);
    } else if CPU_IS_040_OR_060 {
        let mut paddr = __pa(vaddr);
        loop { core::arch::asm!("nop", ".chip 68040", "cpushp %bc,({0})", ".chip 68k", in(reg) paddr); paddr += PAGE_SIZE; nr -= 1; if nr == 0 { break; } }
    } else { let mut tmp: usize; core::arch::asm!("movec cacr,{0}", "or.w {1},{0}", "movec {0},cacr", out(reg) tmp, in(reg) FLUSH_I); let _ = tmp; }
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[inline] pub unsafe fn flush_dcache_page(page: *mut page) { __flush_pages_to_ram(page_address(page), 1); }
#[inline] pub unsafe fn flush_dcache_folio(folio: *mut folio) { __flush_pages_to_ram(folio_address(folio), folio_nr_pages(folio)); }
#[inline] pub unsafe fn flush_dcache_mmap_lock(_mapping: *mut core::ffi::c_void) {}
#[inline] pub unsafe fn flush_dcache_mmap_unlock(_mapping: *mut core::ffi::c_void) {}
#[inline] pub unsafe fn flush_icache_pages(vma: *mut vm_area_struct, page: *mut page, nr: u32) { let _ = vma; __flush_pages_to_ram(page_address(page), nr); }

#[inline]
pub unsafe fn copy_to_user_page(vma: *mut vm_area_struct, page: *mut page, vaddr: usize, dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) {
    flush_cache_page(vma, vaddr, page_to_pfn(page));
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    flush_icache_user_page(vma, page, vaddr, len as i32);
}

#[inline]
pub unsafe fn copy_from_user_page(vma: *mut vm_area_struct, page: *mut page, vaddr: usize, dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) {
    flush_cache_page(vma, vaddr, page_to_pfn(page));
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
