/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004 IBM Corporation
 * Copyright (C) 2015 Intel Corporation
 *
 * Authors:
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Dave Safford <safford@watson.ibm.com>
 * Reiner Sailer <sailer@watson.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 */

// C dependencies supplied by other translation units are intentionally not implemented here.

pub const TPM_MINOR: u32 = 224; // officially assigned
pub const TPM_NUM_DEVICES: u32 = 65536;
pub const TPM_RETRY: u32 = 50;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tpm_timeout {
    TPM_TIMEOUT = 5, // msecs
    TPM_TIMEOUT_RETRY = 100, // msecs
    TPM_TIMEOUT_RANGE_US = 300, // usecs
    TPM_TIMEOUT_POLL = 1, // msecs
    TPM_TIMEOUT_USECS_MIN = 100, // usecs
    TPM_TIMEOUT_USECS_MAX = 500, // usecs
}

/* TPM addresses */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tpm_addr {
    TPM_SUPERIO_ADDR = 0x2E,
    TPM_ADDR = 0x4E,
}

extern "C" {
    pub static tpm_class: class;
    pub static tpmrm_class: class;
    pub static mut tpm_devt: dev_t;
    pub static tpm_fops: file_operations;
    pub static tpmrm_fops: file_operations;
    pub static mut dev_nums_idr: idr;

    pub fn tpm_transmit(chip: *mut tpm_chip, buf: *mut u8, bufsiz: usize) -> isize;
    pub fn tpm_get_timeouts(chip: *mut tpm_chip) -> i32;
    pub fn tpm_auto_startup(chip: *mut tpm_chip) -> i32;

    pub fn tpm1_pm_suspend(chip: *mut tpm_chip, tpm_suspend_pcr: u32) -> i32;
    pub fn tpm1_auto_startup(chip: *mut tpm_chip) -> i32;
    pub fn tpm1_do_selftest(chip: *mut tpm_chip) -> i32;
    pub fn tpm1_get_timeouts(chip: *mut tpm_chip) -> i32;
    pub fn tpm1_calc_ordinal_duration(chip: *mut tpm_chip, ordinal: u32) -> usize;
    pub fn tpm1_pcr_extend(chip: *mut tpm_chip, pcr_idx: u32, hash: *const u8, log_msg: *const i8) -> i32;
    pub fn tpm1_pcr_read(chip: *mut tpm_chip, pcr_idx: u32, res_buf: *mut u8) -> i32;
    pub fn tpm1_getcap(chip: *mut tpm_chip, subcap_id: u32, cap: *mut cap_t, desc: *const i8, min_cap_length: usize) -> isize;
    pub fn tpm1_get_random(chip: *mut tpm_chip, out: *mut u8, max: usize) -> i32;
    pub fn tpm1_get_pcr_allocation(chip: *mut tpm_chip) -> i32;
    pub fn tpm_calc_ordinal_duration(chip: *mut tpm_chip, ordinal: u32) -> usize;
    pub fn tpm_pm_suspend(dev: *mut device) -> i32;
    pub fn tpm_pm_resume(dev: *mut device) -> i32;
    pub fn tpm_class_shutdown(dev: *mut device) -> i32;

    pub fn tpm_chip_bootstrap(chip: *mut tpm_chip) -> i32;
    pub fn tpm_chip_start(chip: *mut tpm_chip) -> i32;
    pub fn tpm_chip_stop(chip: *mut tpm_chip);
    pub fn tpm_chip_alloc(dev: *mut device, ops: *const tpm_class_ops) -> *mut tpm_chip;
    pub fn tpmm_chip_alloc(pdev: *mut device, ops: *const tpm_class_ops) -> *mut tpm_chip;
    pub fn tpm_chip_register(chip: *mut tpm_chip) -> i32;
    pub fn tpm_chip_unregister(chip: *mut tpm_chip);
    pub fn tpm_sysfs_add_device(chip: *mut tpm_chip);

    pub fn tpm2_get_timeouts(chip: *mut tpm_chip) -> i32;
    pub fn tpm2_pcr_read(chip: *mut tpm_chip, pcr_idx: u32, digest: *mut tpm_digest, digest_size_ptr: *mut u16) -> i32;
    pub fn tpm2_pcr_extend(chip: *mut tpm_chip, pcr_idx: u32, digests: *mut tpm_digest) -> i32;
    pub fn tpm2_get_random(chip: *mut tpm_chip, dest: *mut u8, max: usize) -> i32;
    pub fn tpm2_get_tpm_pt(chip: *mut tpm_chip, property_id: u32, value: *mut u32, desc: *const i8) -> isize;
    pub fn tpm2_get_pcr_allocation(chip: *mut tpm_chip) -> isize;
    pub fn tpm2_auto_startup(chip: *mut tpm_chip) -> i32;
    pub fn tpm2_shutdown(chip: *mut tpm_chip, shutdown_type: u16);
    pub fn tpm2_calc_ordinal_duration(ordinal: u32) -> usize;
    pub fn tpm2_probe(chip: *mut tpm_chip) -> i32;
    pub fn tpm2_get_cc_attrs_tbl(chip: *mut tpm_chip) -> i32;
    pub fn tpm2_find_cc(chip: *mut tpm_chip, cc: u32) -> i32;
    pub fn tpm2_init_space(space: *mut tpm_space, buf_size: u32) -> i32;
    pub fn tpm2_del_space(chip: *mut tpm_chip, space: *mut tpm_space);
    pub fn tpm2_flush_space(chip: *mut tpm_chip);
    pub fn tpm2_prepare_space(chip: *mut tpm_chip, space: *mut tpm_space, cmd: *mut u8, cmdsiz: usize) -> i32;
    pub fn tpm2_commit_space(chip: *mut tpm_chip, space: *mut tpm_space, buf: *mut core::ffi::c_void, bufsiz: *mut usize) -> i32;
    pub fn tpm_devs_add(chip: *mut tpm_chip) -> i32;
    pub fn tpm_devs_remove(chip: *mut tpm_chip);
    pub fn tpm2_save_context(chip: *mut tpm_chip, handle: u32, buf: *mut u8, buf_size: u32, offset: *mut u32) -> i32;
    pub fn tpm2_load_context(chip: *mut tpm_chip, buf: *mut u8, offset: *mut u32, handle: *mut u32) -> i32;
    pub fn tpm_bios_log_setup(chip: *mut tpm_chip);
    pub fn tpm_bios_log_teardown(chip: *mut tpm_chip);
    pub fn tpm_dev_common_init() -> i32;
    pub fn tpm_dev_common_exit();
}

#[inline]
pub unsafe fn tpm_msleep(delay_msec: u32) {
    usleep_range(
        delay_msec.wrapping_mul(1000).wrapping_sub(TPM_TIMEOUT_RANGE_US as u32),
        delay_msec.wrapping_mul(1000),
    );
}

#[cfg(not(feature = "CONFIG_ACPI"))]
#[inline]
pub unsafe fn tpm_add_ppi(_chip: *mut tpm_chip) {}

#[cfg(feature = "CONFIG_ACPI")]
extern "C" {
    pub fn tpm_add_ppi(chip: *mut tpm_chip);
}

#[cfg(not(feature = "CONFIG_TCG_TPM2_HMAC"))]
#[inline]
pub unsafe fn tpm2_sessions_init(_chip: *mut tpm_chip) -> i32 { 0 }

#[cfg(feature = "CONFIG_TCG_TPM2_HMAC")]
extern "C" {
    pub fn tpm2_sessions_init(chip: *mut tpm_chip) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
