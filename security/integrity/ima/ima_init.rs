// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005,2006,2007,2008 IBM Corporation
 *
 * Authors:
 * Reiner Sailer      <sailer@watson.ibm.com>
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Mimi Zohar         <zohar@us.ibm.com>
 *
 * File: ima_init.rs (translated from ima_init.c)
 *             initialization and cleanup functions
 */

// Equivalent to: #include <linux/init.h>, <linux/scatterlist.h>, <linux/slab.h>,
// <linux/err.h>, <linux/ima.h>, <generated/utsrelease.h>, and "ima.h"
// These headers provide types and function declarations used below.

use core::mem;
use core::ptr;

// name for boot aggregate entry
pub const BOOT_AGGREGATE_NAME: &[u8] = b"boot_aggregate";
pub const BOOT_AGGREGATE_LATE_NAME: &[u8] = b"boot_aggregate_late";

// External global variable (declared, not defined here)
pub static mut IMA_TPM_CHIP: *mut TpmChip = ptr::null_mut();

// External types (placeholders for dependency types)
#[repr(C)]
pub struct TpmChip {
    // Opaque type from external module
}

#[repr(C)]
pub struct ImaTemplateEntry {
    // Opaque type from external module
}

#[repr(C)]
pub struct ImaIintCache {
    pub ima_hash: *mut ImaDigestData,
}

#[repr(C)]
pub struct ImaEventData {
    pub iint: *mut ImaIintCache,
    pub filename: *const u8,
}

#[repr(C)]
pub struct ImaDigestHdr {
    // Opaque type from external module
}

#[repr(C)]
pub struct ImaDigestData {
    pub hdr: ImaDigestHdr,
    pub algo: u32,
    pub length: u32,
}

#[repr(C)]
pub struct ImaMaxDigestData {
    pub hdr: ImaDigestHdr,
    // Additional digest data follows
}

// External constants and variables
extern "C" {
    pub static mut IMA_HASH_ALGO: u32;
    pub static HASH_DIGEST_SIZE: [u32; 256];

    // External function declarations
    pub fn ima_calc_boot_aggregate(hash_hdr: *mut ImaDigestData) -> i32;
    pub fn ima_alloc_init_template(
        event_data: *const ImaEventData,
        entry: *mut *mut ImaTemplateEntry,
        template: *const u8,
    ) -> i32;
    pub fn ima_store_template(
        entry: *mut ImaTemplateEntry,
        violation: i32,
        pathname: *const u8,
        filename: *const u8,
        pcr_idx: u32,
    ) -> i32;
    pub fn ima_free_template_entry(entry: *mut ImaTemplateEntry);
    pub fn integrity_audit_msg(
        audit_type: u32,
        inode: *const core::ffi::c_void,
        filename: *const u8,
        op: *const u8,
        cause: *const u8,
        result: i32,
        info: i32,
    );
    pub fn integrity_init_keyring(keyring_id: u32) -> i32;
    pub fn ima_init_crypto() -> i32;
    pub fn ima_init_template() -> i32;
    pub fn ima_load_kexec_buffer();
    pub fn ima_init_digests() -> i32;
    pub fn ima_init_htable() -> i32;
    pub fn ima_init_policy();
    pub fn ima_fs_init() -> i32;
    pub fn ima_init_key_queue();
    pub fn ima_init_reboot_notifier();
    pub fn ima_measure_critical_data(
        event_name: *const u8,
        event_data_name: *const u8,
        event_data: *const u8,
        event_data_len: usize,
        hash: bool,
        func_data: *const u8,
        func_data_len: u32,
    );
    pub fn integrity_load_x509(keyring_id: u32, path: *const u8);
    pub fn evm_load_x509();
    pub fn tpm_default_chip() -> *mut TpmChip;
    pub fn pr_info(fmt: *const u8, ...);
}

// Compile-time configuration check for CONFIG_IMA_INIT_LATE_SYNC
// This value is determined at build time; in Rust this would typically be a cfg attribute.
const IMA_INIT_LATE_SYNC_ENABLED: bool = cfg!(feature = "ima_init_late_sync");

const INTEGRITY_KEYRING_IMA: u32 = 0;
const CONFIG_IMA_MEASURE_PCR_IDX: u32 = 10;
const IMA_APPRAISE: u32 = 0x20;

// Configuration value from build (UTS_RELEASE macro equivalent)
const UTS_RELEASE: &[u8] = b"";

// Add the boot aggregate to the IMA measurement list and extend
// the PCR register.
//
// Calculate the boot aggregate, a hash over tpm registers 0-7,
// assuming a TPM chip exists, and zeroes if the TPM chip does not
// exist.  Add the boot aggregate measurement to the measurement
// list and extend the PCR register.
//
// If a tpm chip does not exist, indicate the core root of trust is
// not hardware based by invalidating the aggregate PCR value.
// (The aggregate PCR value is invalidated by adding one value to
// the measurement list and extending the aggregate PCR value with
// a different value.) Violations add a zero entry to the measurement
// list and extend the aggregate PCR value with ff...ff's.
#[inline(never)]
fn ima_add_boot_aggregate() -> i32 {
    const OP: &[u8] = b"add_boot_aggregate";

    let mut audit_cause: &[u8] = b"ENOMEM";
    let mut entry: *mut ImaTemplateEntry = ptr::null_mut();
    let mut tmp_iint = ImaIintCache {
        ima_hash: ptr::null_mut(),
    };
    let iint = &mut tmp_iint;
    let mut event_data = ImaEventData {
        iint: iint as *mut ImaIintCache,
        filename: ptr::null(),
    };
    let mut hash = ImaMaxDigestData {
        hdr: unsafe { mem::zeroed() },
    };

    // Equivalent to: struct ima_digest_data *hash_hdr = container_of(&hash.hdr,
    //                    struct ima_digest_data, hdr);
    let hash_hdr: *mut ImaDigestData = unsafe {
        let hdr_ptr = &mut hash.hdr as *mut ImaDigestHdr;
        // container_of macro: cast from hdr field to ImaDigestData struct
        (hdr_ptr as *mut u8).offset(
            -(mem::offset_of!(ImaDigestData, hdr) as isize)
        ) as *mut ImaDigestData
    };

    let filename: *const u8;
    let mut result: i32 = -12; // ENOMEM
    let mut violation: i32 = 0;

    unsafe {
        // memset(iint, 0, sizeof(*iint))
        ptr::write_bytes(iint, 0, 1);
        // memset(&hash, 0, sizeof(hash))
        ptr::write_bytes(&mut hash as *mut ImaMaxDigestData, 0, 1);
    }

    iint.ima_hash = hash_hdr;

    unsafe {
        (*hash_hdr).algo = IMA_HASH_ALGO;
        (*hash_hdr).length = HASH_DIGEST_SIZE[IMA_HASH_ALGO as usize];
    }

    if IMA_INIT_LATE_SYNC_ENABLED {
        filename = BOOT_AGGREGATE_LATE_NAME.as_ptr();
    } else {
        filename = BOOT_AGGREGATE_NAME.as_ptr();
    }
    event_data.filename = filename;

    // With TPM 2.0 hash agility, TPM chips could support multiple TPM
    // PCR banks, allowing firmware to configure and enable different
    // banks.  The SHA1 bank is not necessarily enabled.
    //
    // Use the same hash algorithm for reading the TPM PCRs as for
    // calculating the boot aggregate digest.  Preference is given to
    // the configured IMA default hash algorithm.  Otherwise, use the
    // TCG required banks - SHA256 for TPM 2.0, SHA1 for TPM 1.2.
    // Ultimately select SHA1 also for TPM 2.0 if the SHA256 PCR bank
    // is not found.

    unsafe {
        if !IMA_TPM_CHIP.is_null() {
            result = ima_calc_boot_aggregate(hash_hdr);
            if result < 0 {
                audit_cause = b"hashing_error";
                return ima_add_boot_aggregate_err_out(audit_cause, filename, OP, result);
            }
        }
    }

    result = unsafe {
        ima_alloc_init_template(&event_data, &mut entry, ptr::null())
    };
    if result < 0 {
        audit_cause = b"alloc_entry";
        return ima_add_boot_aggregate_err_out(audit_cause, filename, OP, result);
    }

    result = unsafe {
        ima_store_template(
            entry,
            violation,
            ptr::null(),
            filename,
            CONFIG_IMA_MEASURE_PCR_IDX,
        )
    };
    if result < 0 {
        unsafe {
            ima_free_template_entry(entry);
        }
        audit_cause = b"store_entry";
        return ima_add_boot_aggregate_err_out(audit_cause, filename, OP, result);
    }

    0
}

#[inline(never)]
fn ima_add_boot_aggregate_err_out(
    audit_cause: &[u8],
    filename: *const u8,
    op: &[u8],
    result: i32,
) -> i32 {
    unsafe {
        integrity_audit_msg(
            4202, // AUDIT_INTEGRITY_PCR
            ptr::null(),
            filename,
            op.as_ptr(),
            audit_cause.as_ptr(),
            result,
            0,
        );
    }
    result
}

#[cfg(feature = "ima_load_x509")]
pub fn ima_load_x509() {
    unsafe {
        let unset_flags = IMA_HASH_ALGO & IMA_APPRAISE;

        IMA_HASH_ALGO &= !unset_flags;
        integrity_load_x509(INTEGRITY_KEYRING_IMA, b"CONFIG_IMA_X509_PATH\0".as_ptr());

        // load also EVM key to avoid appraisal
        evm_load_x509();

        IMA_HASH_ALGO |= unset_flags;
    }
}

pub fn ima_init() -> i32 {
    let mut rc: i32;

    unsafe {
        IMA_TPM_CHIP = tpm_default_chip();
        if IMA_TPM_CHIP.is_null() {
            pr_info(b"No TPM chip found, activating TPM-bypass!\n\0".as_ptr());
        }
    }

    rc = unsafe { integrity_init_keyring(INTEGRITY_KEYRING_IMA) };
    if rc != 0 {
        return rc;
    }

    rc = unsafe { ima_init_crypto() };
    if rc != 0 {
        return rc;
    }

    rc = unsafe { ima_init_template() };
    if rc != 0 {
        return rc;
    }

    // It can be called before ima_init_digests(), it does not use TPM.
    unsafe {
        ima_load_kexec_buffer();
    }

    rc = unsafe { ima_init_digests() };
    if rc != 0 {
        return rc;
    }

    rc = unsafe { ima_init_htable() };
    if rc != 0 {
        return rc;
    }

    rc = ima_add_boot_aggregate(); // boot aggregate must be first entry
    if rc != 0 {
        return rc;
    }

    unsafe {
        ima_init_policy();
    }

    rc = unsafe { ima_fs_init() };
    if rc != 0 {
        return rc;
    }

    unsafe {
        ima_init_key_queue();
    }

    unsafe {
        ima_init_reboot_notifier();
    }

    unsafe {
        ima_measure_critical_data(
            b"kernel_info\0".as_ptr(),
            b"kernel_version\0".as_ptr(),
            UTS_RELEASE.as_ptr(),
            UTS_RELEASE.len(),
            false,
            ptr::null(),
            0,
        );
    }

    rc
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
