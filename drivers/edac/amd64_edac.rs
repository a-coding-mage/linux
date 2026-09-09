// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level Rust translation of amd64_edac.c.  Kernel-provided declarations
// referenced by the implementation are intentionally left external.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct edac_pci_ctl_info { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct pci_dev { pub devfn: c_uint }
#[repr(C)]
pub struct msr { _private: [u8; 0] }
#[repr(C)]
pub struct amd64_pvt { _private: [u8; 0] }
#[repr(C)]
pub struct mem_ctl_info { pub pvt_info: *mut c_void }
#[repr(C)]
pub struct ecc_settings { _private: [u8; 0] }

static mut pci_ctl: *mut edac_pci_ctl_info = core::ptr::null_mut();
static mut ecc_enable_override: c_int = 0;
static mut msrs: *mut msr = core::ptr::null_mut();
static mut ecc_stngs: *mut *mut ecc_settings = core::ptr::null_mut();
static mut pci_ctl_dev: *mut device = core::ptr::null_mut();

#[repr(C)]
struct scrubrate { scrubval: u32, bandwidth: u32 }

static scrubrates: [scrubrate; 24] = [
    scrubrate { scrubval: 0x01, bandwidth: 1600000000 },
    scrubrate { scrubval: 0x02, bandwidth: 800000000 },
    scrubrate { scrubval: 0x03, bandwidth: 400000000 },
    scrubrate { scrubval: 0x04, bandwidth: 200000000 },
    scrubrate { scrubval: 0x05, bandwidth: 100000000 },
    scrubrate { scrubval: 0x06, bandwidth: 50000000 },
    scrubrate { scrubval: 0x07, bandwidth: 25000000 },
    scrubrate { scrubval: 0x08, bandwidth: 12284069 },
    scrubrate { scrubval: 0x09, bandwidth: 6274509 },
    scrubrate { scrubval: 0x0a, bandwidth: 3121951 },
    scrubrate { scrubval: 0x0b, bandwidth: 1560975 },
    scrubrate { scrubval: 0x0c, bandwidth: 781440 },
    scrubrate { scrubval: 0x0d, bandwidth: 390720 },
    scrubrate { scrubval: 0x0e, bandwidth: 195300 },
    scrubrate { scrubval: 0x0f, bandwidth: 97650 },
    scrubrate { scrubval: 0x10, bandwidth: 48854 },
    scrubrate { scrubval: 0x11, bandwidth: 24427 },
    scrubrate { scrubval: 0x12, bandwidth: 12213 },
    scrubrate { scrubval: 0x13, bandwidth: 6101 },
    scrubrate { scrubval: 0x14, bandwidth: 3051 },
    scrubrate { scrubval: 0x15, bandwidth: 1523 },
    scrubrate { scrubval: 0x16, bandwidth: 761 },
    scrubrate { scrubval: 0x00, bandwidth: 0 },
    scrubrate { scrubval: 0, bandwidth: 0 },
];

extern "C" {
    pub fn __amd64_read_pci_cfg_dword(pdev: *mut pci_dev, offset: c_int,
                                      val: *mut u32, func: *const c_char) -> c_int;
    pub fn __amd64_write_pci_cfg_dword(pdev: *mut pci_dev, offset: c_int,
                                       val: u32, func: *const c_char) -> c_int;
}

// The remaining definitions retain the source-level kernel implementation and
// are supplied by the surrounding kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
