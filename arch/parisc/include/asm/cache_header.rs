/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-parisc/cache.h
 */

/* C header guard: __ARCH_PARISC_CACHE_H */
/* Dependency: asm/alternative.h */

/*
 * PA 2.0 processors have 64 and 128-byte L2 cachelines; PA 1.1 processors
 * have 32-byte cachelines.  The L1 length appears to be 16 bytes but this
 * is not clearly documented.
 */
pub const L1_CACHE_BYTES: usize = 16;
pub const L1_CACHE_SHIFT: usize = 4;

pub const SMP_CACHE_BYTES: usize = L1_CACHE_BYTES;

/* CONFIG_PA20 selects 128; otherwise this is 32. */
#[cfg(feature = "CONFIG_PA20")]
pub const ARCH_DMA_MINALIGN: usize = 128;
#[cfg(not(feature = "CONFIG_PA20"))]
pub const ARCH_DMA_MINALIGN: usize = 32;

pub const ARCH_KMALLOC_MINALIGN: usize = 16; /* ldcw requires 16-byte alignment */

extern "C" {
    pub static mut dcache_stride: ::core::ffi::c_int;
    pub static mut icache_stride: ::core::ffi::c_int;
    pub static mut split_tlb: ::core::ffi::c_int;
}

#[inline]
pub unsafe fn arch_slab_minalign() -> ::core::ffi::c_uint {
    dcache_stride as ::core::ffi::c_uint
}

#[inline]
pub unsafe fn cache_line_size() -> ::core::ffi::c_int {
    dcache_stride
}

#[inline]
pub unsafe fn dma_get_cache_alignment() -> ::core::ffi::c_int {
    cache_line_size()
}

/* __read_mostly expands to __section(".data..read_mostly") in C. */

extern "C" {
    pub fn parisc_cache_init(); /* initializes cache-flushing */
    pub fn disable_sr_hashing_asm(arg: ::core::ffi::c_int); /* low level support for above */
    pub fn disable_sr_hashing(); /* turns off space register hashing */
    pub fn free_sid(arg: ::core::ffi::c_ulong);
    pub fn alloc_sid() -> ::core::ffi::c_ulong;
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

extern "C" {
    pub fn show_cache_info(m: *mut seq_file);
}

#[repr(C)]
pub struct pdc_cache_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pdc_btlb_info {
    _private: [u8; 0],
}

extern "C" {
    pub static mut cache_info: pdc_cache_info;
    pub static mut btlb_info: pdc_btlb_info;
    pub fn parisc_setup_cache_timing();
}

/* The ALTERNATIVE() instruction patching conditions are supplied by asm/alternative.h. */
#[macro_export]
macro_rules! pdtlb {
    ($sr:expr, $addr:expr) => {{
        unsafe {
            ::core::arch::asm!("pdtlb 0(%%sr{0},{1})", in(reg) $sr, in(reg) $addr, options(nostack));
        }
    }};
}

#[macro_export]
macro_rules! pitlb {
    ($sr:expr, $addr:expr) => {{
        unsafe {
            ::core::arch::asm!("pitlb 0(%%sr{0},{1})", in(reg) $sr, in(reg) $addr, options(nostack));
        }
    }};
}

#[macro_export]
macro_rules! asm_io_fdc {
    ($addr:expr) => {{
        unsafe {
            ::core::arch::asm!("fdc %%r0({0})", in(reg) $addr, options(nostack));
        }
    }};
}

#[macro_export]
macro_rules! asm_io_sync {
    () => {{
        unsafe {
            ::core::arch::asm!("sync", options(nostack));
        }
    }};
}

#[macro_export]
macro_rules! asm_syncdma {
    () => {{
        unsafe {
            ::core::arch::asm!("syncdma", options(nostack));
        }
    }};
}

/* Classes of processor wrt: disabling space register hashing */
pub const SRHASH_PCXST: u32 = 0; /* pcxs, pcxt, pcxt_ */
pub const SRHASH_PCXL: u32 = 1; /* pcxl */
pub const SRHASH_PA20: u32 = 2; /* pcxu, pcxu_, pcxw, pcxw_ */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
