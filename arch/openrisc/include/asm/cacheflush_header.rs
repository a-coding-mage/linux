/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) Jan Henrik Weinstock <jan.weinstock@rwth-aachen.de>
 * et al.
 */

// C dependency: #include <linux/mm.h>

/*
 * Helper function for flushing or invalidating entire pages from data
 * and instruction caches. SMP needs a little extra work, since we need
 * to flush the pages on all cpus.
 */
unsafe extern "C" {
    pub fn local_dcache_page_flush(page: *mut page);
    pub fn local_icache_page_inv(page: *mut page);
    pub fn local_dcache_range_flush(start: c_ulong, end: c_ulong);
    pub fn local_dcache_range_inv(start: c_ulong, end: c_ulong);
    pub fn local_icache_range_inv(start: c_ulong, end: c_ulong);
    pub fn local_icache_all_inv();
}

/*
 * Data cache flushing always happen on the local cpu. Instruction cache
 * invalidations need to be broadcasted to all other cpu in the system in
 * case of SMP configurations.
 *
 * The CONFIG_SMP conditional is preserved from the C header: select the
 * local instruction-cache operations when disabled, and the SMP operations
 * when enabled.
 */
#[cfg(not(CONFIG_SMP))]
pub unsafe fn dcache_page_flush(page: *mut page) {
    local_dcache_page_flush(page);
}

#[cfg(not(CONFIG_SMP))]
pub unsafe fn icache_page_inv(page: *mut page) {
    local_icache_page_inv(page);
}

#[cfg(not(CONFIG_SMP))]
pub unsafe fn icache_all_inv() {
    local_icache_all_inv();
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" {
    pub fn smp_icache_page_inv(page: *mut page);
    pub fn smp_icache_all_inv();
}

#[cfg(CONFIG_SMP)]
pub unsafe fn dcache_page_flush(page: *mut page) {
    local_dcache_page_flush(page);
}

#[cfg(CONFIG_SMP)]
pub unsafe fn icache_page_inv(page: *mut page) {
    smp_icache_page_inv(page);
}

#[cfg(CONFIG_SMP)]
pub unsafe fn icache_all_inv() {
    smp_icache_all_inv();
}

/*
 * Even if the actual block size is larger than L1_CACHE_BYTES, paddr
 * can be incremented by L1_CACHE_BYTES. When paddr is written to the
 * invalidate register, the entire cache line encompassing this address
 * is invalidated. Each subsequent reference to the same cache line will
 * not affect the invalidation process.
 */
#[inline]
pub unsafe fn local_dcache_block_flush(addr: c_ulong) {
    local_dcache_range_flush(addr, addr.wrapping_add(L1_CACHE_BYTES));
}

#[inline]
pub unsafe fn local_dcache_block_inv(addr: c_ulong) {
    local_dcache_range_inv(addr, addr.wrapping_add(L1_CACHE_BYTES));
}

#[inline]
pub unsafe fn local_icache_block_inv(addr: c_ulong) {
    local_icache_range_inv(addr, addr.wrapping_add(L1_CACHE_BYTES));
}

/*
 * Synchronizes caches. Whenever a cpu writes executable code to memory, this
 * should be called to make sure the processor sees the newly written code.
 */
#[inline]
pub unsafe fn sync_icache_dcache(page: *mut page) {
    if !IS_ENABLED(CONFIG_DCACHE_WRITETHROUGH) {
        dcache_page_flush(page);
    }
    icache_page_inv(page);
}

/*
 * Pages with this bit set need not be flushed/invalidated, since
 * they have not changed since last flush. New pages start with
 * PG_arch_1 not set and are therefore dirty by default.
 */
pub const PG_DC_CLEAN: _ = PG_arch_1;

#[inline]
pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    clear_bit(PG_DC_CLEAN, &mut (*folio).flags.f);
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[inline]
pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

#[inline]
pub unsafe fn flush_icache_user_page(
    vma: *mut vm_area_struct,
    page: *mut page,
    _addr: c_ulong,
    _len: c_ulong,
) {
    if (*vma).vm_flags & VM_EXEC != 0 {
        sync_icache_dcache(page);
    }
}

// C dependency: #include <asm-generic/cacheflush.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
