/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from psp-platform-access.h.
// Dependency intent: u32 and related platform definitions are supplied by the
// surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum psp_platform_access_msg {
    PSP_CMD_NONE = 0x0,
    PSP_SFS_GET_FW_VERSIONS = 0x1,
    PSP_SFS_UPDATE = 0x2,
    PSP_CMD_HSTI_QUERY = 0x14,
    PSP_I2C_REQ_BUS_CMD = 0x64,
    PSP_DYNAMIC_BOOST_GET_NONCE = 0x65,
    PSP_DYNAMIC_BOOST_SET_UID = 0x66,
    PSP_DYNAMIC_BOOST_GET_PARAMETER = 0x67,
    PSP_DYNAMIC_BOOST_SET_PARAMETER = 0x68,
}

#[repr(C, packed)]
pub struct psp_req_buffer_hdr {
    pub payload_size: u32,
    pub status: u32,
}

#[repr(C, packed)]
pub struct psp_request {
    pub header: psp_req_buffer_hdr,
    pub buf: *mut core::ffi::c_void,
}

/**
 * psp_send_platform_access_msg() - Send a message to control platform features
 *
 * This function is intended to be used by drivers outside of ccp to communicate
 * with the platform.
 *
 * Returns:
 *  0:           success
 *  -%EBUSY:     mailbox in recovery or in use
 *  -%ENODEV:    driver not bound with PSP device
 *  -%ETIMEDOUT: request timed out
 *  -%EIO:       unknown error (see kernel log)
 */
unsafe extern "C" {
    pub fn psp_send_platform_access_msg(
        msg: psp_platform_access_msg,
        req: *mut psp_request,
    ) -> i32;
}

/**
 * psp_ring_platform_doorbell() - Ring platform doorbell
 *
 * This function is intended to be used by drivers outside of ccp to ring the
 * platform doorbell with a message.
 *
 * Returns:
 *  0:           success
 *  -%EBUSY:     mailbox in recovery or in use
 *  -%ENODEV:    driver not bound with PSP device
 *  -%ETIMEDOUT: request timed out
 *  -%EIO:       error will be stored in result argument
 */
unsafe extern "C" {
    pub fn psp_ring_platform_doorbell(msg: i32, result: *mut u32) -> i32;
}

/**
 * psp_check_platform_access_status() - Checks whether platform features is ready
 *
 * This function is intended to be used by drivers outside of ccp to determine
 * if platform features has initialized.
 *
 * Returns:
 * 0          platform features is ready
 * -%ENODEV   platform features is not ready or present
 */
unsafe extern "C" {
    pub fn psp_check_platform_access_status() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
