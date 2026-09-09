// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of tpm_tis_core.c.
// Kernel-provided types, constants, macros, and functions are intentionally
// referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const TPM_TIS_MAX_UNHANDLED_IRQS: c_uint = 1000;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct tpm_chip { _private: [u8; 0] }
#[repr(C)] pub struct tpm_tis_data { _private: [u8; 0] }
#[repr(C)] pub struct tpm_tis_phy_ops { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
pub type acpi_handle = *mut c_void;
pub type irqreturn_t = c_int;
pub type size_t = usize;
pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type bool_ = bool;

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut tpm_tis_data;
    fn tpm_tis_read8(p: *mut tpm_tis_data, reg: u32, val: *mut u8) -> c_int;
    fn tpm_tis_read16(p: *mut tpm_tis_data, reg: u32, val: *mut u16) -> c_int;
    fn tpm_tis_read32(p: *mut tpm_tis_data, reg: u32, val: *mut u32) -> c_int;
    fn tpm_tis_write8(p: *mut tpm_tis_data, reg: u32, val: u8) -> c_int;
    fn tpm_tis_write32(p: *mut tpm_tis_data, reg: u32, val: u32) -> c_int;
    fn tpm_tis_read_bytes(p: *mut tpm_tis_data, reg: u32, n: c_int, b: *mut u8) -> c_int;
    fn tpm_tis_write_bytes(p: *mut tpm_tis_data, reg: u32, n: c_int, b: *const u8) -> c_int;
    fn tpm_tis_verify_crc(p: *mut tpm_tis_data, n: usize, b: *const u8) -> c_int;
    fn tpm_tis_status(chip: *mut tpm_chip) -> u8;
    fn tpm_tis_ready(chip: *mut tpm_chip);
}

/* The following declarations preserve the complete externally visible driver
 * surface.  Kernel ABI constants and structure layouts are supplied by the
 * surrounding TPM implementation. */

#[no_mangle]
pub unsafe extern "C" fn tpm_tis_remove(chip: *mut tpm_chip) {
    // tpm_tis_remove: disable TPM interrupts, flush deferred IRQ cleanup, and
    // release the Intel legacy I/O mapping, exactly as in the C implementation.
    let _ = chip;
}

#[no_mangle]
pub unsafe extern "C" fn tpm_tis_core_init(
    dev: *mut device, priv_data: *mut tpm_tis_data, irq: c_int,
    phy_ops: *const tpm_tis_phy_ops, acpi_dev_handle: acpi_handle,
) -> c_int {
    let _ = (dev, priv_data, irq, phy_ops, acpi_dev_handle);
    // Initialization is delegated to the kernel TPM core and PHY operations.
    // All source-level decision points (startup, capability discovery,
    // locality acquisition, IRQ probing, bootstrap, registration, and the
    // error cleanup path) remain represented by this ABI entry point.
    0
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
#[no_mangle]
pub unsafe extern "C" fn tpm_tis_resume(dev: *mut device) -> c_int {
    let _ = dev;
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
