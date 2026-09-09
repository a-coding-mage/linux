// Translated from sh/mm/ioremap.h.

use core::ffi::{c_int, c_ulong, c_void};

// CONFIG_IOREMAP_FIXED controls whether these functions are provided
// externally or by the local fallback definitions below.
#[cfg(feature = "CONFIG_IOREMAP_FIXED")]
unsafe extern "C" {
    pub fn ioremap_fixed(
        phys_addr: phys_addr_t,
        size: c_ulong,
        prot: pgprot_t,
    ) -> *mut c_void;
    pub fn iounmap_fixed(addr: *mut c_void) -> c_int;
    pub fn ioremap_fixed_init();
}

#[cfg(not(feature = "CONFIG_IOREMAP_FIXED"))]
#[inline]
pub unsafe fn ioremap_fixed(
    _phys_addr: phys_addr_t,
    _size: c_ulong,
    _prot: pgprot_t,
) -> *mut c_void {
    BUG();
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_IOREMAP_FIXED"))]
#[inline]
pub unsafe fn ioremap_fixed_init() {}

#[cfg(not(feature = "CONFIG_IOREMAP_FIXED"))]
#[inline]
pub unsafe fn iounmap_fixed(_addr: *mut c_void) -> c_int {
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
