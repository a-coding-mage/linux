/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 James.Bottomley@HansenPartnership.com
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 *
 * Helpers for the SVSM_VTPM_CMD calls used by the vTPM protocol defined by the
 * AMD SVSM spec [1].
 *
 * The vTPM protocol follows the Official TPM 2.0 Reference Implementation
 * (originally by Microsoft, now part of the TCG) simulator protocol.
 *
 * [1] "Secure VM Service Module for SEV-SNP Guests"
 *     Publication # 58019 Revision: 1.00
 */

pub const SVSM_VTPM_MAX_BUFFER: usize = 4096; /* max req/resp buffer size */

/**
 * struct svsm_vtpm_request - Generic request for single word command
 * @cmd: The command to send
 *
 * Defined by AMD SVSM spec [1] in section "8.2 SVSM_VTPM_CMD Call" -
 * Table 15: vTPM Common Request/Response Structure.
 */
#[repr(C)]
pub struct svsm_vtpm_request {
    pub cmd: u32,
}

/**
 * struct svsm_vtpm_response - Generic response
 * @size: The response size (zero if nothing follows)
 *
 * Defined by AMD SVSM spec [1] in section "8.2 SVSM_VTPM_CMD Call" -
 * Table 15: vTPM Common Request/Response Structure.
 */
#[repr(C)]
pub struct svsm_vtpm_response {
    pub size: u32,
}

/**
 * struct svsm_vtpm_cmd_request - Structure for a TPM_SEND_COMMAND request
 * @cmd: The command to send (must be TPM_SEND_COMMAND)
 * @locality: The locality
 * @buf_size: The size of the input buffer following
 * @buf: A buffer of size buf_size
 *
 * Defined by AMD SVSM spec [1] in section "8.2 SVSM_VTPM_CMD Call" -
 * Table 16: TPM_SEND_COMMAND Request Structure.
 */
#[repr(C, packed)]
pub struct svsm_vtpm_cmd_request {
    pub cmd: u32,
    pub locality: u8,
    pub buf_size: u32,
    pub buf: [u8; 0],
}

/**
 * struct svsm_vtpm_cmd_response - Structure for a TPM_SEND_COMMAND response
 * @buf_size: The size of the output buffer following
 * @buf: A buffer of size buf_size
 *
 * Defined by AMD SVSM spec [1] in section "8.2 SVSM_VTPM_CMD Call" -
 * Table 17: TPM_SEND_COMMAND Response Structure.
 */
#[repr(C)]
pub struct svsm_vtpm_cmd_response {
    pub buf_size: u32,
    pub buf: [u8; 0],
}

/** Fill a TPM_SEND_COMMAND request to be sent to SVSM. */
#[inline]
pub unsafe fn svsm_vtpm_cmd_request_fill(
    req: *mut svsm_vtpm_cmd_request,
    locality: u8,
    buf: *const u8,
    len: usize,
) -> i32 {
    if len > SVSM_VTPM_MAX_BUFFER - core::mem::size_of::<svsm_vtpm_cmd_request>() {
        return -EINVAL;
    }

    core::ptr::addr_of_mut!((*req).cmd).write_unaligned(8); /* TPM_SEND_COMMAND */
    core::ptr::addr_of_mut!((*req).locality).write(locality);
    core::ptr::addr_of_mut!((*req).buf_size).write_unaligned(len as u32);

    core::ptr::copy_nonoverlapping(
        buf,
        (req as *mut u8).add(core::mem::size_of::<svsm_vtpm_cmd_request>()),
        len,
    );

    0
}

/** Parse a TPM_SEND_COMMAND response received from SVSM. */
#[inline]
pub unsafe fn svsm_vtpm_cmd_response_parse(
    resp: *const svsm_vtpm_cmd_response,
    buf: *mut u8,
    len: usize,
) -> i32 {
    let buf_size = core::ptr::addr_of!((*resp).buf_size).read_unaligned();

    if len < buf_size as usize {
        return -E2BIG;
    }

    if buf_size as usize > SVSM_VTPM_MAX_BUFFER - core::mem::size_of::<svsm_vtpm_cmd_response>() {
        return -EINVAL; // Invalid response from the platform TPM
    }

    core::ptr::copy_nonoverlapping(
        (resp as *const u8).add(core::mem::size_of::<svsm_vtpm_cmd_response>()),
        buf,
        buf_size as usize,
    );

    buf_size as i32
}

/* EINVAL and E2BIG are supplied by the Linux errno definitions. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
