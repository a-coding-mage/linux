/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of the PowerPC cacheflush header. */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/mm.h, asm/cputable.h, asm/cpu_has_feature.h,
// asm-generic/cacheflush.h

/// This flag indicates that the page pointed to by a pte is clean and does
/// not require cleaning before returning it to the user.
pub const PG_DCache_CLEAN: usize = PG_arch_1;

// CONFIG_PPC_BOOK3S_64
/// Book3s has no ptesync after setting a pte.
#[inline]
pub unsafe fn flush_cache_vmap(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {
    // asm volatile("ptesync" ::: "memory");
    ::core::arch::asm!("ptesync", options(nostack));
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

/// This is called when a page has been modified by the kernel.  It marks the
/// page as not i-cache clean; the i-cache flush is done later if necessary.
#[inline]
pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    if cpu_has_feature(CPU_FTR_COHERENT_ICACHE) {
        return;
    }
    // Avoid an atomic op if possible.
    if test_bit(PG_DCache_CLEAN, &mut (*folio).flags.f) {
        clear_bit(PG_DCache_CLEAN, &mut (*folio).flags.f);
    }
}

#[inline]
pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

unsafe extern "C" {
    pub fn flush_icache_range(start: ::core::ffi::c_ulong, stop: ::core::ffi::c_ulong);
    pub fn flush_icache_user_page(
        vma: *mut vm_area_struct,
        page: *mut page,
        addr: ::core::ffi::c_ulong,
        len: i32,
    );
    pub fn flush_dcache_icache_folio(folio: *mut folio);
}

/**
 * flush_dcache_range(): Write any modified data cache blocks out to memory and
 * invalidate them. Does not invalidate the corresponding instruction cache
 * blocks.
 *
 * @start: the start address
 * @stop: the stop address (exclusive)
 */
#[inline]
pub unsafe fn flush_dcache_range(start: ::core::ffi::c_ulong, stop: ::core::ffi::c_ulong) {
    let shift = l1_dcache_shift();
    let bytes = l1_dcache_bytes();
    let mut addr = (start & !(bytes - 1)) as *mut ::core::ffi::c_void;
    let size = stop - addr as ::core::ffi::c_ulong + (bytes - 1);
    let mut i: ::core::ffi::c_ulong = 0;

    // IS_ENABLED(CONFIG_PPC64)
    if IS_ENABLED_CONFIG_PPC64 {
        mb(); // sync
    }

    while i < (size >> shift) {
        dcbf(addr);
        addr = (addr as *mut u8).add(bytes as usize) as *mut ::core::ffi::c_void;
        i += 1;
    }
    mb(); // sync
}

/// Write modified data cache blocks out to memory without invalidating them.
#[inline]
pub unsafe fn clean_dcache_range(start: ::core::ffi::c_ulong, stop: ::core::ffi::c_ulong) {
    let shift = l1_dcache_shift();
    let bytes = l1_dcache_bytes();
    let mut addr = (start & !(bytes - 1)) as *mut ::core::ffi::c_void;
    let size = stop - addr as ::core::ffi::c_ulong + (bytes - 1);
    let mut i: ::core::ffi::c_ulong = 0;

    while i < (size >> shift) {
        dcbst(addr);
        addr = (addr as *mut u8).add(bytes as usize) as *mut ::core::ffi::c_void;
        i += 1;
    }
    mb(); // sync
}

/// Invalidate the D-cache, used by the 8xx to avoid stale data from the CPM.
#[inline]
pub unsafe fn invalidate_dcache_range(
    start: ::core::ffi::c_ulong,
    stop: ::core::ffi::c_ulong,
) {
    let shift = l1_dcache_shift();
    let bytes = l1_dcache_bytes();
    let mut addr = (start & !(bytes - 1)) as *mut ::core::ffi::c_void;
    let size = stop - addr as ::core::ffi::c_ulong + (bytes - 1);
    let mut i: ::core::ffi::c_ulong = 0;

    while i < (size >> shift) {
        dcbi(addr);
        addr = (addr as *mut u8).add(bytes as usize) as *mut ::core::ffi::c_void;
        i += 1;
    }
    mb(); // sync
}

// CONFIG_44x
#[inline]
pub unsafe fn flush_instruction_cache() {
    iccci(KERNELBASE as *mut ::core::ffi::c_void);
    isync();
}

// For non-CONFIG_44x builds, the declaration is supplied externally:
// extern "C" { fn flush_instruction_cache(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
