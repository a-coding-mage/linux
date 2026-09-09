/* SPDX-License-Identifier: GPL-2.0 */

/* bytes per L1 cache line */
pub const L1_CACHE_SHIFT: u32 = CONFIG_L1_CACHE_SHIFT;

pub const L1_CACHE_BYTES: u32 = 1u32 << L1_CACHE_SHIFT;

pub const ARCH_DMA_MINALIGN: u32 = L1_CACHE_BYTES;

/* C header declarations are omitted when __ASSEMBLER__ is defined. */

unsafe extern "C" {
    pub fn dcache_wb_line(start: ::core::ffi::c_ulong);

    pub fn icache_inv_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn icache_inv_all();
    pub fn local_icache_inv_all(priv_: *mut ::core::ffi::c_void);

    pub fn dcache_wb_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dcache_wbinv_all();

    pub fn cache_wbinv_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn cache_wbinv_all();

    pub fn dma_wbinv_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dma_inv_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dma_wb_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
