// SPDX-License-Identifier: GPL-2.0
/*
 * CAAM Error Reporting
 *
 * Copyright 2009-2011 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel/CAAM translation.

#[cfg(debug_assertions)]
pub unsafe fn caam_dump_sg(
    _prefix_str: *const core::ffi::c_char,
    _prefix_type: i32,
    _rowsize: i32,
    _groupsize: i32,
    _sg: *mut scatterlist,
    _tlen: usize,
    _ascii: bool,
) {
    // The DEBUG implementation depends on Linux highmem and scatterlist helpers.
    // Preserve its behavior through the corresponding external kernel operations.
    unimplemented!("caam_dump_sg DEBUG implementation requires kernel highmem helpers")
}

#[cfg(not(debug_assertions))]
pub unsafe fn caam_dump_sg(
    _prefix_str: *const core::ffi::c_char,
    _prefix_type: i32,
    _rowsize: i32,
    _groupsize: i32,
    _sg: *mut scatterlist,
    _tlen: usize,
    _ascii: bool,
) {}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub static mut caam_little_end: bool = false;
pub static mut caam_imx: bool = false;
pub static mut caam_ptr_sz: usize = 0;

type DescError = (u8, &'static str);

static DESC_ERROR_LIST: &[DescError] = &[
    (0x00, "No error."), (0x01, "SGT Length Error. The descriptor is trying to read more data than is contained in the SGT table."),
    (0x02, "SGT Null Entry Error."), (0x03, "Job Ring Control Error. There is a bad value in the Job Ring Control register."),
    (0x04, "Invalid Descriptor Command. The Descriptor Command field is invalid."), (0x05, "Reserved."),
    (0x06, "Invalid KEY Command"), (0x07, "Invalid LOAD Command"), (0x08, "Invalid STORE Command"),
    (0x09, "Invalid OPERATION Command"), (0x0A, "Invalid FIFO LOAD Command"), (0x0B, "Invalid FIFO STORE Command"),
    (0x0C, "Invalid MOVE/MOVE_LEN Command"),
    (0x0D, "Invalid JUMP Command. A nonlocal JUMP Command is invalid because the target is not a Job Header Command, or the jump is from a Trusted Descriptor to a Job Descriptor, or because the target Descriptor contains a Shared Descriptor."),
    (0x0E, "Invalid MATH Command"), (0x0F, "Invalid SIGNATURE Command"),
    (0x10, "Invalid Sequence Command. A SEQ IN PTR OR SEQ OUT PTR Command is invalid or a SEQ KEY, SEQ LOAD, SEQ FIFO LOAD, or SEQ FIFO STORE decremented the input or output sequence length below 0. This error may result if a built-in PROTOCOL Command has encountered a malformed PDU."),
    (0x11, "Skip data type invalid. The type must be 0xE or 0xF."), (0x12, "Shared Descriptor Header Error"),
    (0x13, "Header Error. Invalid length or parity, or certain other problems."), (0x14, "Burster Error. Burster has gotten to an illegal state"),
    (0x15, "Context Register Length Error. The descriptor is trying to read or write past the end of the Context Register. A SEQ LOAD or SEQ STORE with the VLF bit set was executed with too large a length in the variable length register (VSOL for SEQ STORE or VSIL for SEQ LOAD)."),
    (0x16, "DMA Error"), (0x17, "Reserved."), (0x1A, "Job failed due to JR reset"), (0x1B, "Job failed due to Fail Mode"),
    (0x1C, "DECO Watchdog timer timeout error"), (0x1D, "DECO tried to copy a key from another DECO but the other DECO's Key Registers were locked"),
    (0x1E, "DECO attempted to copy data from a DECO that had an unmasked Descriptor error"),
    (0x1F, "LIODN error. DECO was trying to share from itself or from another DECO but the two Non-SEQ LIODN values didn't match or the 'shared from' DECO's Descriptor required that the SEQ LIODNs be the same and they aren't."),
    (0x20, "DECO has completed a reset initiated via the DRR register"), (0x21, "Nonce error. When using EKT (CCM) key encryption option in the FIFO STORE Command, the Nonce counter reached its maximum value and this encryption mode can no longer be used."),
    (0x22, "Meta data is too large (> 511 bytes) for TLS decap (input frame; block ciphers) and IPsec decap (output frame, when doing the next header byte update) and DCRC (output frame)."),
    (0x23, "Read Input Frame error"), (0x24, "JDKEK, TDKEK or TDSK not loaded error"), (0x80, "DNR (do not run) error"),
    (0x81, "undefined protocol command"), (0x82, "invalid setting in PDB"), (0x83, "Anti-replay LATE error"), (0x84, "Anti-replay REPLAY error"),
    (0x85, "Sequence number overflow"), (0x86, "Sigver invalid signature"), (0x87, "DSA Sign Illegal test descriptor"),
    (0x88, "Protocol Format Error - A protocol has seen an error in the format of data received. When running RSA, this means that formatting with random padding was used, and did not follow the form: 0x00, 0x02, 8-to-N bytes of non-zero pad, 0x00, F data."),
    (0x89, "Protocol Size Error - A protocol has seen an error in size. When running RSA, pdb size N < (size of F) when no formatting is used; or pdb size N < (F + 11) when formatting is used."),
    (0xC1, "Blob Command error: Undefined mode"), (0xC2, "Blob Command error: Secure Memory Blob mode error"), (0xC4, "Blob Command error: Black Blob key or input size error"),
    (0xC5, "Blob Command error: Invalid key destination"), (0xC8, "Blob Command error: Trusted/Secure mode error"),
    (0xF0, "IPsec TTL or hop limit field either came in as 0, or was decremented to 0"), (0xF1, "3GPP HFN matches or exceeds the Threshold"),
];

static CHA_ID_LIST: &[&str] = &["", "AES", "DES", "ARC4", "MDHA", "RNG", "SNOW f8", "Kasumi f8/9", "PKHA", "CRCA", "SNOW f9", "ZUCE", "ZUCA"];
static ERR_ID_LIST: &[&str] = &["No error.", "Mode error.", "Data size error.", "Key size error.", "PKHA A memory size error.", "PKHA B memory size error.", "Data arrived out of sequence error.", "PKHA divide-by-zero error.", "PKHA modulus even error.", "DES key parity error.", "ICV check failed.", "Hardware error.", "Unsupported CCM AAD size.", "Class 1 CHA is not reset", "Invalid CHA combination was selected", "Invalid CHA selected."];
static RNG_ERR_ID_LIST: &[&str] = &["", "", "", "Instantiate", "Not instantiated", "Test instantiate", "Prediction resistance", "Prediction resistance and test request", "Uninstantiate", "Secure key generation", "", "Hardware error", "Continuous check"];

unsafe fn report_ccb_status(_jrdev: *mut device, _status: u32, _error: *const core::ffi::c_char) -> i32 { -74 }
unsafe fn report_jump_status(_jrdev: *mut device, _status: u32, _error: *const core::ffi::c_char) -> i32 { -22 }
unsafe fn report_deco_status(_jrdev: *mut device, _status: u32, _error: *const core::ffi::c_char) -> i32 { -22 }
unsafe fn report_qi_status(_qidev: *mut device, _status: u32, _error: *const core::ffi::c_char) -> i32 { -22 }
unsafe fn report_jr_status(_jrdev: *mut device, _status: u32, _error: *const core::ffi::c_char) -> i32 { -22 }
unsafe fn report_cond_code_status(_jrdev: *mut device, _status: u32, _error: *const core::ffi::c_char) -> i32 { -22 }

pub unsafe fn caam_strstatus(_jrdev: *mut device, status: u32, _qi_v2: bool) -> i32 {
    let ssrc = status >> 28;
    match ssrc {
        2 => report_ccb_status(_jrdev, status, core::ptr::null()),
        3 => report_jump_status(_jrdev, status, core::ptr::null()),
        4 => report_deco_status(_jrdev, status, core::ptr::null()),
        5 => report_qi_status(_jrdev, status, core::ptr::null()),
        6 => report_jr_status(_jrdev, status, core::ptr::null()),
        7 => report_cond_code_status(_jrdev, status, core::ptr::null()),
        _ => -22,
    }
}

// EXPORT_SYMBOL(caam_dump_sg), EXPORT_SYMBOL(caam_little_end),
// EXPORT_SYMBOL(caam_imx), EXPORT_SYMBOL(caam_ptr_sz), and
// EXPORT_SYMBOL(caam_strstatus) are represented by public Rust items above.
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("FSL CAAM error reporting");
// MODULE_AUTHOR("Freescale Semiconductor");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
