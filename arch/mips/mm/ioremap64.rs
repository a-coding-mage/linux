// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the corresponding Linux headers:
//   pgprot_t, phys_addr_t, _CACHE_MASK, _CACHE_UNCACHED, IO_BASE, UNCAC_BASE

extern "C" {
    fn pgprot_val(prot: pgprot_t) -> ::kernel::ffi::c_ulong;
    fn plat_ioremap(
        offset: phys_addr_t,
        size: ::kernel::ffi::c_ulong,
        flags: ::kernel::ffi::c_ulong,
    ) -> *mut ::kernel::ffi::c_void;
    fn plat_iounmap(addr: *const ::kernel::ffi::c_void);
}

pub unsafe extern "C" fn ioremap_prot(
    offset: phys_addr_t,
    size: ::kernel::ffi::c_ulong,
    prot: pgprot_t,
) -> *mut ::kernel::ffi::c_void {
    let flags = pgprot_val(prot) & _CACHE_MASK;
    let base: u64 = if flags == _CACHE_UNCACHED {
        IO_BASE
    } else {
        UNCAC_BASE
    };
    let mut addr: *mut ::kernel::ffi::c_void;

    addr = plat_ioremap(offset, size, flags);
    if addr.is_null() {
        addr = (base + offset as u64) as usize as *mut ::kernel::ffi::c_void;
    }
    addr
}

// EXPORT_SYMBOL(ioremap_prot);

pub unsafe extern "C" fn iounmap(addr: *const ::kernel::ffi::c_void) {
    plat_iounmap(addr);
}

// EXPORT_SYMBOL(iounmap);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
