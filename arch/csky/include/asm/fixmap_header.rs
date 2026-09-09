/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture and generic headers:
// asm/page.h, asm/memory.h, linux/threads.h, and asm/kmap_size.h.

#[repr(isize)]
pub enum FixedAddresses {
    // CONFIG_HAVE_TCM
    #[cfg(CONFIG_HAVE_TCM)]
    FIX_TCM = TCM_NR_PAGES as isize,

    // CONFIG_HIGHMEM
    #[cfg(CONFIG_HIGHMEM)]
    FIX_KMAP_BEGIN,
    #[cfg(CONFIG_HIGHMEM)]
    FIX_KMAP_END = FIX_KMAP_BEGIN as isize + (KM_MAX_IDX as isize * NR_CPUS as isize) - 1,

    __end_of_fixed_addresses,
}

pub const FIXADDR_SIZE: usize = (__end_of_fixed_addresses as usize) << PAGE_SHIFT;
pub const FIXADDR_START: usize = (FIXADDR_TOP as usize) - FIXADDR_SIZE;

// Declarations from asm-generic/fixmap.h are supplied by that dependency.

unsafe extern "C" {
    pub fn fixrange_init(start: ::core::ffi::c_ulong,
                         end: ::core::ffi::c_ulong,
                         pgd_base: *mut pgd_t);
    // C __init annotation has no direct Rust equivalent.
    pub fn fixaddr_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
