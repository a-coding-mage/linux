/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Following copyright information was take from the original file
 * <include/linux/tpm.h> where the definitions were moved from:
 *
 * Copyright (C) 2004,2007,2008 IBM Corporation
 *
 * Authors:
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Dave Safford <safford@watson.ibm.com>
 * Reiner Sailer <sailer@watson.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 * Debora Velarde <dvelarde@us.ibm.com>
 *
 * Maintained by: <tpmdd_devel@lists.sourceforge.net>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not implemented here: linux/types.h and linux/bits.h.

#[repr(u32)]
pub enum TpmBufFlags {
    /* TPM2B format: */
    TPM_BUF_TPM2B = 1u32 << 0,
    /* The buffer is in invalid and unusable state: */
    TPM_BUF_INVALID = 1u32 << 1,
}

/*
 * A buffer for constructing and parsing TPM commands, responses and sized
 * (TPM2B) buffers.
 */
#[repr(C)]
pub struct tpm_buf {
    pub flags: u8,
    pub handles: u8,
    pub length: u16,
    pub capacity: u16,
    pub data: [u8; 0],
}

unsafe extern "C" {
    pub fn tpm_buf_init(buf: *mut tpm_buf, buf_size: u16);
    pub fn tpm_buf_init_sized(buf: *mut tpm_buf, buf_size: u16);
    pub fn tpm_buf_reset(buf: *mut tpm_buf, tag: u16, ordinal: u32);
    pub fn tpm_buf_reset_sized(buf: *mut tpm_buf);
    pub fn tpm_buf_length(buf: *mut tpm_buf) -> u16;
    pub fn tpm_buf_append(buf: *mut tpm_buf, new_data: *const u8, new_length: u16);
    pub fn tpm_buf_append_u8(buf: *mut tpm_buf, value: u8);
    pub fn tpm_buf_append_u16(buf: *mut tpm_buf, value: u16);
    pub fn tpm_buf_append_u32(buf: *mut tpm_buf, value: u32);
    pub fn tpm_buf_read_u8(buf: *mut tpm_buf, offset: *mut off_t) -> u8;
    pub fn tpm_buf_read_u16(buf: *mut tpm_buf, offset: *mut off_t) -> u16;
    pub fn tpm_buf_read_u32(buf: *mut tpm_buf, offset: *mut off_t) -> u32;
    pub fn tpm_buf_append_handle(buf: *mut tpm_buf, handle: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
