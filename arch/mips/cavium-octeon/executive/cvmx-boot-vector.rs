/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2004-2017 Cavium, Inc.
 */

/* The boot-vector instruction stream and its disassembly are preserved in the
 * original source comments; the executable translation follows. */

use core::ffi::c_void;

/* Supplied by the surrounding Octeon platform bindings. */
#[repr(C)]
pub struct cvmx_boot_vector_element {
    _opaque: [u8; 0],
}

extern "C" {
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn cvmx_ptr_to_phys(ptr: *const c_void) -> u64;
    fn cvmx_write_csr(address: u64, value: u64);
    fn OCTEON_IS_OCTEON1PLUS() -> bool;
    fn cvmx_bootmem_alloc_named_range_once(
        size: u64,
        min_addr: u64,
        max_addr: u64,
        alignment: u64,
        name: *const i8,
        init: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> *mut cvmx_boot_vector_element;
}

/* These values are supplied by the platform header. */
extern "C" {
    static CVMX_MIO_BOOT_LOC_ADR: u64;
    static CVMX_MIO_BOOT_LOC_DAT: u64;
    fn CVMX_MIO_BOOT_LOC_CFGX(core: u64) -> u64;
}

static mut _CVMX_BOOTVECTOR_DATA: [u64; 16] = [
    0x40baf80040bbf803, /* patch low order 8-bits if no KScratch */
    0x401a6000401b7801,
    0x375a0084337b03ff,
    0x409a6000001bd940,
    0x3c1abfc0bc110000,
    0xdf5a0078041f0000,
    0x035bd02d00000000,
    0xdf5a0000403bf803, /* patch low order 8-bits if no KScratch */
    0x1340000500000000,
    0x0340000800000000,
    0x0000000000000000,
    0x4200002000000000,
    0x1000fffd00000000,
    0x0000000000000000,
    OCTEON_BOOT_MOVEABLE_MAGIC1,
    0, /* To be filled in with address of vector block */
];

/* 2^10 CPUs */
const VECTOR_TABLE_SIZE: usize = 1024 * core::mem::size_of::<cvmx_boot_vector_element>();

unsafe extern "C" fn cvmx_boot_vector_init(mem: *mut c_void) {
    let kseg0_mem: u64;
    let mut i: i32;

    memset(mem, 0, VECTOR_TABLE_SIZE);
    kseg0_mem = cvmx_ptr_to_phys(mem) | 0x8000_0000_0000_0000;

    i = 0;
    while i < 15 {
        let mut v = _CVMX_BOOTVECTOR_DATA[i as usize];

        if OCTEON_IS_OCTEON1PLUS() && (i == 0 || i == 7) {
            v &= 0xffff_ffff_0000_0000; /* KScratch not available */
        }
        cvmx_write_csr(CVMX_MIO_BOOT_LOC_ADR, (i as u64) * 8);
        cvmx_write_csr(CVMX_MIO_BOOT_LOC_DAT, v);
        i += 1;
    }
    cvmx_write_csr(CVMX_MIO_BOOT_LOC_ADR, 15 * 8);
    cvmx_write_csr(CVMX_MIO_BOOT_LOC_DAT, kseg0_mem);
    cvmx_write_csr(CVMX_MIO_BOOT_LOC_CFGX(0), 0x81fc0000);
}

/**
 * Get a pointer to the per-core table of reset vector pointers
 *
 */
pub unsafe extern "C" fn cvmx_boot_vector_get() -> *mut cvmx_boot_vector_element {
    cvmx_bootmem_alloc_named_range_once(
        VECTOR_TABLE_SIZE as u64,
        0,
        (1u64 << 32) - 1,
        8,
        b"__boot_vector1__\0".as_ptr() as *const i8,
        Some(cvmx_boot_vector_init),
    )
}

// EXPORT_SYMBOL(cvmx_boot_vector_get)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
