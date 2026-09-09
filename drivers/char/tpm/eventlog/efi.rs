// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Google
 *
 * Authors:
 *      Thiebaud Weksteen <tweek@google.com>
 */

// Dependency declarations and kernel-provided types are supplied by the
// surrounding translation unit.

use core::ffi::c_void;

extern "C" {
    static mut efi_tpm_final_log_size: i32;
    static mut efi: Efi;

    fn memremap(addr: usize, size: usize, flags: u64) -> *mut c_void;
    fn memunmap(addr: *mut c_void);
    fn pr_err(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn devm_kmemdup(dev: *mut c_void, src: *const c_void, size: usize, flags: u64) -> *mut u8;
    fn devm_krealloc(dev: *mut c_void, ptr: *mut u8, size: usize, flags: u64) -> *mut c_void;
    fn devm_kfree(dev: *mut c_void, ptr: *mut u8);
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
}

#[repr(C)]
pub struct Efi {
    pub tpm_log: usize,
    pub tpm_final_log: usize,
}

#[repr(C)]
pub struct EfiTcg2FinalEventsTable {
    pub events: [u8; 0],
}

#[repr(C)]
pub struct LinuxEfiTpmEventlog {
    pub size: u32,
    pub version: u8,
    pub final_events_preboot_size: i32,
    pub log: [u8; 0],
}

#[repr(C)]
pub struct TpmBiosLog {
    pub bios_event_log: *mut u8,
    pub bios_event_log_end: *mut u8,
}

#[repr(C)]
pub struct Device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct TpmChip {
    pub flags: u32,
    pub dev: Device,
    pub log: TpmBiosLog,
}

const TPM_CHIP_FLAG_TPM2: u32 = 1 << 1;
const EFI_INVALID_TABLE_ADDR: usize = usize::MAX;
const MEMREMAP_WB: u64 = 1;
const GFP_KERNEL: u64 = 0;
const EFI_TCG2_EVENT_LOG_FORMAT_TCG_2: u8 = 2;

/* read binary bios log from EFI configuration table */
pub unsafe fn tpm_read_log_efi(chip: *mut TpmChip) -> i32 {
    let mut final_tbl: *mut EfiTcg2FinalEventsTable = core::ptr::null_mut();
    let mut final_events_log_size: i32 = efi_tpm_final_log_size;
    let mut log_tbl: *mut LinuxEfiTpmEventlog;
    let log: *mut TpmBiosLog;
    let mut log_size: u32;
    let mut tpm_log_version: u8;
    let mut tmp: *mut c_void;
    let mut ret: i32;

    if ((*chip).flags & TPM_CHIP_FLAG_TPM2) == 0 {
        return -19; // -ENODEV
    }

    if efi.tpm_log == EFI_INVALID_TABLE_ADDR {
        return -19; // -ENODEV
    }

    log = &mut (*chip).log;

    log_tbl = memremap(
        efi.tpm_log,
        core::mem::size_of::<LinuxEfiTpmEventlog>(),
        MEMREMAP_WB,
    ) as *mut LinuxEfiTpmEventlog;
    if log_tbl.is_null() {
        pr_err(b"Could not map UEFI TPM log table !\0".as_ptr());
        return -12; // -ENOMEM
    }

    log_size = (*log_tbl).size;
    memunmap(log_tbl as *mut c_void);

    if log_size == 0 {
        pr_warn(b"UEFI TPM log area empty\0".as_ptr());
        return -5; // -EIO
    }

    log_tbl = memremap(
        efi.tpm_log,
        core::mem::size_of::<LinuxEfiTpmEventlog>() + log_size as usize,
        MEMREMAP_WB,
    ) as *mut LinuxEfiTpmEventlog;
    if log_tbl.is_null() {
        pr_err(b"Could not map UEFI TPM log table payload!\0".as_ptr());
        return -12; // -ENOMEM
    }

    (*log).bios_event_log = devm_kmemdup(
        &mut (*chip).dev as *mut Device as *mut c_void,
        (*log_tbl).log.as_ptr() as *const c_void,
        log_size as usize,
        GFP_KERNEL,
    );
    if (*log).bios_event_log.is_null() {
        ret = -12;
        goto_out(final_tbl, log_tbl);
        return ret;
    }

    (*log).bios_event_log_end = (*log).bios_event_log.add(log_size as usize);
    tpm_log_version = (*log_tbl).version;

    ret = tpm_log_version as i32;

    if efi.tpm_final_log == EFI_INVALID_TABLE_ADDR
        || final_events_log_size == 0
        || tpm_log_version != EFI_TCG2_EVENT_LOG_FORMAT_TCG_2
    {
        goto_out(final_tbl, log_tbl);
        return ret;
    }

    final_tbl = memremap(
        efi.tpm_final_log,
        core::mem::size_of::<EfiTcg2FinalEventsTable>() + final_events_log_size as usize,
        MEMREMAP_WB,
    ) as *mut EfiTcg2FinalEventsTable;
    if final_tbl.is_null() {
        pr_err(b"Could not map UEFI TPM final log\0".as_ptr());
        devm_kfree(&mut (*chip).dev as *mut Device as *mut c_void, (*log).bios_event_log);
        ret = -12;
        goto_out(final_tbl, log_tbl);
        return ret;
    }

    final_events_log_size -= (*log_tbl).final_events_preboot_size;
    tmp = devm_krealloc(
        &mut (*chip).dev as *mut Device as *mut c_void,
        (*log).bios_event_log,
        log_size as usize + final_events_log_size as usize,
        GFP_KERNEL,
    );
    if tmp.is_null() {
        devm_kfree(&mut (*chip).dev as *mut Device as *mut c_void, (*log).bios_event_log);
        ret = -12;
        goto_out(final_tbl, log_tbl);
        return ret;
    }

    (*log).bios_event_log = tmp as *mut u8;
    memcpy(
        (*log).bios_event_log.add(log_size as usize) as *mut c_void,
        (*final_tbl).events.as_ptr().add((*log_tbl).final_events_preboot_size as usize)
            as *const c_void,
        final_events_log_size as usize,
    );
    (*log).bios_event_log_end = (*log).bios_event_log.add(
        log_size as usize + final_events_log_size as usize,
    );

    goto_out(final_tbl, log_tbl);
    ret
}

#[inline]
unsafe fn goto_out(
    final_tbl: *mut EfiTcg2FinalEventsTable,
    log_tbl: *mut LinuxEfiTpmEventlog,
) {
    memunmap(final_tbl as *mut c_void);
    memunmap(log_tbl as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
