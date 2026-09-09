/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h and asm/pgtable_types.h.

unsafe extern "C" {
    pub fn pat_enabled() -> bool;
    pub fn pat_bp_init();
    pub fn pat_cpu_init();

    pub fn memtype_reserve(
        start: u64,
        end: u64,
        req_pcm: crate::page_cache_mode,
        ret_pcm: *mut crate::page_cache_mode,
    ) -> i32;

    pub fn memtype_free(start: u64, end: u64) -> i32;

    pub fn memtype_kernel_map_sync(
        base: u64,
        size: usize,
        pcm: crate::page_cache_mode,
    ) -> i32;

    pub fn memtype_reserve_io(
        start: crate::resource_size_t,
        end: crate::resource_size_t,
        pcm: *mut crate::page_cache_mode,
    ) -> i32;

    pub fn memtype_free_io(start: crate::resource_size_t, end: crate::resource_size_t);

    pub fn pat_pfn_immune_to_uc_mtrr(pfn: usize) -> bool;

    pub fn x86_has_pat_wp() -> bool;

    pub fn pgprot2cachemode(pgprot: crate::pgprot_t) -> crate::page_cache_mode;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
