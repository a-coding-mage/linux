/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2005, 2006 IBM Corporation
 * Copyright (C) 2014, 2015 Intel Corporation
 *
 * Authors:
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 *
 * This device driver implements the TPM interface as defined in
 * the TCG TPM Interface Spec version 1.2, revision 1.0.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not implemented here: linux/tpm_ptp.h and tpm.h.

#[repr(u32)]
pub enum tpm_tis_flags {
    TPM_TIS_ITPM_WORKAROUND = 0,
    TPM_TIS_INVALID_STATUS = 1,
    TPM_TIS_DEFAULT_CANCELLATION = 2,
    TPM_TIS_IRQ_TESTED = 3,
    TPM_TIS_STATUS_VALID_RETRY = 4,
    TPM_TIS_SETTLE_AFTER_RELINQUISH = 5,
}

#[repr(C)]
pub struct tpm_tis_data {
    pub chip: *mut tpm_chip,
    pub did_vid: u32,
    pub locality_count_mutex: mutex,
    pub locality_count: c_uint,
    pub locality: c_int,
    pub irq: c_int,
    pub free_irq_work: work_struct,
    pub last_unhandled_irq: c_ulong,
    pub unhandled_irqs: c_uint,
    pub int_mask: c_uint,
    pub flags: c_ulong,
    pub ilb_base_addr: *mut c_void,
    pub clkrun_enabled: u16,
    pub int_queue: wait_queue_head_t,
    pub read_queue: wait_queue_head_t,
    pub phy_ops: *const tpm_tis_phy_ops,
    pub rng_quality: c_ushort,
    pub timeout_min: c_uint, /* usecs */
    pub timeout_max: c_uint, /* usecs */
}

/* IO modes indicate how many bytes are read/written at once. */
#[repr(u32)]
pub enum tpm_tis_io_mode {
    TPM_TIS_PHYS_8,
    TPM_TIS_PHYS_16,
    TPM_TIS_PHYS_32,
}

#[repr(C)]
pub struct tpm_tis_phy_ops {
    /* data is passed in little endian */
    pub read_bytes: Option<unsafe extern "C" fn(
        data: *mut tpm_tis_data,
        addr: u32,
        len: u16,
        result: *mut u8,
        mode: tpm_tis_io_mode,
    ) -> c_int>,
    pub write_bytes: Option<unsafe extern "C" fn(
        data: *mut tpm_tis_data,
        addr: u32,
        len: u16,
        value: *const u8,
        mode: tpm_tis_io_mode,
    ) -> c_int>,
    pub verify_crc: Option<unsafe extern "C" fn(
        data: *mut tpm_tis_data,
        len: size_t,
        value: *const u8,
    ) -> c_int>,
}

#[inline]
pub unsafe fn tpm_tis_read_bytes(data: *mut tpm_tis_data, addr: u32, len: u16, result: *mut u8) -> c_int {
    ((*data).phy_ops.as_ref().unwrap().read_bytes.unwrap())(data, addr, len, result, tpm_tis_io_mode::TPM_TIS_PHYS_8)
}

#[inline]
pub unsafe fn tpm_tis_read8(data: *mut tpm_tis_data, addr: u32, result: *mut u8) -> c_int {
    ((*data).phy_ops.as_ref().unwrap().read_bytes.unwrap())(data, addr, 1, result, tpm_tis_io_mode::TPM_TIS_PHYS_8)
}

#[inline]
pub unsafe fn tpm_tis_read16(data: *mut tpm_tis_data, addr: u32, result: *mut u16) -> c_int {
    let mut result_le: u16 = 0;
    let rc = ((*data).phy_ops.as_ref().unwrap().read_bytes.unwrap())(data, addr, core::mem::size_of::<u16>() as u16, &mut result_le as *mut u16 as *mut u8, tpm_tis_io_mode::TPM_TIS_PHYS_16);
    if rc == 0 { *result = u16::from_le(result_le); }
    rc
}

#[inline]
pub unsafe fn tpm_tis_read32(data: *mut tpm_tis_data, addr: u32, result: *mut u32) -> c_int {
    let mut result_le: u32 = 0;
    let rc = ((*data).phy_ops.as_ref().unwrap().read_bytes.unwrap())(data, addr, core::mem::size_of::<u32>() as u16, &mut result_le as *mut u32 as *mut u8, tpm_tis_io_mode::TPM_TIS_PHYS_32);
    if rc == 0 { *result = u32::from_le(result_le); }
    rc
}

#[inline]
pub unsafe fn tpm_tis_write_bytes(data: *mut tpm_tis_data, addr: u32, len: u16, value: *const u8) -> c_int {
    ((*data).phy_ops.as_ref().unwrap().write_bytes.unwrap())(data, addr, len, value, tpm_tis_io_mode::TPM_TIS_PHYS_8)
}

#[inline]
pub unsafe fn tpm_tis_write8(data: *mut tpm_tis_data, addr: u32, value: u8) -> c_int {
    ((*data).phy_ops.as_ref().unwrap().write_bytes.unwrap())(data, addr, 1, &value, tpm_tis_io_mode::TPM_TIS_PHYS_8)
}

#[inline]
pub unsafe fn tpm_tis_write32(data: *mut tpm_tis_data, addr: u32, value: u32) -> c_int {
    let value_le = value.to_le();
    ((*data).phy_ops.as_ref().unwrap().write_bytes.unwrap())(data, addr, core::mem::size_of::<u32>() as u16, &value_le as *const u32 as *const u8, tpm_tis_io_mode::TPM_TIS_PHYS_32)
}

#[inline]
pub unsafe fn tpm_tis_verify_crc(data: *mut tpm_tis_data, len: size_t, value: *const u8) -> c_int {
    match (*data).phy_ops.as_ref().unwrap().verify_crc {
        None => 0,
        Some(verify_crc) => verify_crc(data, len, value),
    }
}

#[inline]
pub unsafe fn is_bsw() -> bool {
    #[cfg(target_arch = "x86")]
    { (boot_cpu_data.x86_vfm == INTEL_ATOM_AIRMONT) }
    #[cfg(not(target_arch = "x86"))]
    { false }
}

extern "C" {
    pub fn tpm_tis_remove(chip: *mut tpm_chip);
    pub fn tpm_tis_core_init(dev: *mut device, priv_: *mut tpm_tis_data, irq: c_int,
                             phy_ops: *const tpm_tis_phy_ops, acpi_dev_handle: acpi_handle) -> c_int;
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub fn tpm_tis_resume(dev: *mut device) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
