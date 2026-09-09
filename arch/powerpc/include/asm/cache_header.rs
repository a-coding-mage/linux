/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_POWERPC_CACHE_H
// The following declarations are kernel-only (__KERNEL__) in the source.

/* bytes per L1 cache line */
// Build-time configuration selection preserved from the C source:
// CONFIG_PPC_8xx, CONFIG_PPC_E500MC, CONFIG_PPC32, CONFIG_PPC_47x, or PPC64.
#[cfg(CONFIG_PPC_8xx)]
pub const L1_CACHE_SHIFT: u32 = 4;
#[cfg(CONFIG_PPC_E500MC)]
pub const L1_CACHE_SHIFT: u32 = 6;
#[cfg(all(not(CONFIG_PPC_8xx), not(CONFIG_PPC_E500MC), CONFIG_PPC32, CONFIG_PPC_47x))]
pub const L1_CACHE_SHIFT: u32 = 7;
#[cfg(all(not(CONFIG_PPC_8xx), not(CONFIG_PPC_E500MC), CONFIG_PPC32, not(CONFIG_PPC_47x)))]
pub const L1_CACHE_SHIFT: u32 = 5;
#[cfg(all(not(CONFIG_PPC_8xx), not(CONFIG_PPC_E500MC), not(CONFIG_PPC32)))]
pub const L1_CACHE_SHIFT: u32 = 7;

#[cfg(CONFIG_PPC_8xx)]
pub const MAX_COPY_PREFETCH: u32 = 1;
#[cfg(any(CONFIG_PPC_E500MC, CONFIG_PPC32))]
pub const MAX_COPY_PREFETCH: u32 = 4;

#[cfg(any(CONFIG_PPC_8xx, CONFIG_PPC_E500MC))]
pub const IFETCH_ALIGN_SHIFT: u32 = if cfg!(CONFIG_PPC_8xx) { 2 } else { 3 };
#[cfg(CONFIG_PPC32)]
pub const IFETCH_ALIGN_SHIFT: u32 = 3; // 603 fetches 2 insn at a time
#[cfg(all(not(CONFIG_PPC_8xx), not(CONFIG_PPC_E500MC), not(CONFIG_PPC32)))]
pub const IFETCH_ALIGN_SHIFT: u32 = 4; // POWER8,9

pub const L1_CACHE_BYTES: u32 = 1u32 << L1_CACHE_SHIFT;
pub const SMP_CACHE_BYTES: u32 = L1_CACHE_BYTES;
pub const IFETCH_ALIGN_BYTES: u32 = 1u32 << IFETCH_ALIGN_SHIFT;

#[cfg(CONFIG_NOT_COHERENT_CACHE)]
pub const ARCH_DMA_MINALIGN: u32 = L1_CACHE_BYTES;

#[cfg(CONFIG_PPC64)]
#[repr(C)]
pub struct ppc_cache_info {
    pub size: u32,
    pub line_size: u32,
    pub block_size: u32, // L1 only
    pub log_block_size: u32,
    pub blocks_per_page: u32,
    pub sets: u32,
    pub assoc: u32,
}

#[cfg(CONFIG_PPC64)]
#[repr(C)]
pub struct ppc64_caches {
    pub l1d: ppc_cache_info,
    pub l1i: ppc_cache_info,
    pub l2: ppc_cache_info,
    pub l3: ppc_cache_info,
}

#[cfg(CONFIG_PPC64)]
extern "C" {
    pub static mut ppc64_caches: ppc64_caches;
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub unsafe fn l1_dcache_shift() -> u32 { ppc64_caches.l1d.log_block_size }

#[cfg(CONFIG_PPC64)]
#[inline]
pub unsafe fn l1_dcache_bytes() -> u32 { ppc64_caches.l1d.block_size }

#[cfg(CONFIG_PPC64)]
#[inline]
pub unsafe fn l1_icache_shift() -> u32 { ppc64_caches.l1i.log_block_size }

#[cfg(CONFIG_PPC64)]
#[inline]
pub unsafe fn l1_icache_bytes() -> u32 { ppc64_caches.l1i.block_size }

#[cfg(not(CONFIG_PPC64))]
#[inline]
pub const fn l1_dcache_shift() -> u32 { L1_CACHE_SHIFT }

#[cfg(not(CONFIG_PPC64))]
#[inline]
pub const fn l1_dcache_bytes() -> u32 { L1_CACHE_BYTES }

#[cfg(not(CONFIG_PPC64))]
#[inline]
pub const fn l1_icache_shift() -> u32 { L1_CACHE_SHIFT }

#[cfg(not(CONFIG_PPC64))]
#[inline]
pub const fn l1_icache_bytes() -> u32 { L1_CACHE_BYTES }

// __read_mostly expands to __section(".data..read_mostly") in the source.

#[cfg(CONFIG_PPC_BOOK3S_32)]
extern "C" {
    pub fn _get_L2CR() -> ::core::ffi::c_long;
    pub fn _get_L3CR() -> ::core::ffi::c_long;
    pub fn _set_L2CR(val: ::core::ffi::c_ulong);
    pub fn _set_L3CR(val: ::core::ffi::c_ulong);
}

#[cfg(not(CONFIG_PPC_BOOK3S_32))]
#[inline]
pub const fn _get_L2CR() -> ::core::ffi::c_long { 0 }
#[cfg(not(CONFIG_PPC_BOOK3S_32))]
#[inline]
pub const fn _get_L3CR() -> ::core::ffi::c_long { 0 }
#[cfg(not(CONFIG_PPC_BOOK3S_32))]
#[inline]
pub fn _set_L2CR(_val: ::core::ffi::c_ulong) {}
#[cfg(not(CONFIG_PPC_BOOK3S_32))]
#[inline]
pub fn _set_L3CR(_val: ::core::ffi::c_ulong) {}

// PowerPC inline assembly operations from the source header.
#[inline]
pub unsafe fn dcbz(addr: *mut ::core::ffi::c_void) {
    core::arch::asm!("dcbz 0, {0}", in(reg) addr, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn dcbi(addr: *mut ::core::ffi::c_void) {
    core::arch::asm!("dcbi 0, {0}", in(reg) addr, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn dcbf(addr: *mut ::core::ffi::c_void) {
    core::arch::asm!("dcbf 0, {0}", in(reg) addr, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn dcbst(addr: *mut ::core::ffi::c_void) {
    core::arch::asm!("dcbst 0, {0}", in(reg) addr, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn icbi(addr: *mut ::core::ffi::c_void) {
    core::arch::asm!("icbi 0, {0}", in(reg) addr, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn iccci(addr: *mut ::core::ffi::c_void) {
    core::arch::asm!("iccci 0, {0}", in(reg) addr, options(nostack, preserves_flags));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
