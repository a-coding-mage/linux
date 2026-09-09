/* SPDX-License-Identifier: MIT */
/*
 * Copyright (c) 2025 Intel Corporation
 */

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * define INTEL_LB_FLAG_IS_PERSISTENT - Mark the payload as persistent
 *
 * This flag indicates that the late binding payload should be stored
 * persistently in flash across warm resets.
 */
pub const INTEL_LB_FLAG_IS_PERSISTENT: u32 = 1u32 << 0;

/**
 * enum intel_lb_type - enum to determine late binding payload type
 * @INTEL_LB_TYPE_FAN_CONTROL: Fan controller configuration
 * @INTEL_LB_TYPE_OCODE: Ocode firmware
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_lb_type {
    INTEL_LB_TYPE_FAN_CONTROL = 1,
    INTEL_LB_TYPE_OCODE = 3,
}

/**
 * enum intel_lb_status - Status codes returned on late binding transmissions
 * @INTEL_LB_STATUS_SUCCESS: Operation completed successfully
 * @INTEL_LB_STATUS_4ID_MISMATCH: Mismatch in the expected 4ID (firmware identity/token)
 * @INTEL_LB_STATUS_ARB_FAILURE: Arbitration failure (e.g. conflicting access or state)
 * @INTEL_LB_STATUS_GENERAL_ERROR: General firmware error not covered by other codes
 * @INTEL_LB_STATUS_INVALID_PARAMS: One or more input parameters are invalid
 * @INTEL_LB_STATUS_INVALID_SIGNATURE: Payload has an invalid or untrusted signature
 * @INTEL_LB_STATUS_INVALID_PAYLOAD: Payload contents are not accepted by firmware
 * @INTEL_LB_STATUS_TIMEOUT: Operation timed out before completion
 * @INTEL_LB_STATUS_BUFFER_TOO_SMALL: Buffer provided is smaller when expected
 * @INTEL_LB_STATUS_INTERNAL_ERROR: Internal firmware error
 * @INTEL_LB_STATUS_INVALID_FPT_TABLE: Invalid firmware format table
 * @INTEL_LB_STATUS_SIGNED_PAYLOAD_VERIFICATION_ERROR: Error in signature verification
 * @INTEL_LB_STATUS_SIGNED_PAYLOAD_INVALID_CPD: Invalid CPD
 * @INTEL_LB_STATUS_SIGNED_PAYLOAD_FW_VERSION_MISMATCH: Firmware version mismatch
 * @INTEL_LB_STATUS_SIGNED_PAYLOAD_INVALID_MANIFEST: Invalid firmware manifest
 * @INTEL_LB_STATUS_SIGNED_PAYLOAD_INVALID_HASH: Wrong hash in signature
 * @INTEL_LB_STATUS_SIGNED_PAYLOAD_BINDING_TYPE_MISMATCH: Wrong firmware type provided
 * @INTEL_LB_STATUS_SIGNED_PAYLOAD_HANDLE_SVN_FAILED: SVN check failed
 * @INTEL_LB_STATUS_DESTINATION_MBOX_FAILURE: Failed to send datat to destination
 * @INTEL_LB_STATUS_MISSING_LOADING_PATCH: No loading patch found
 * @INTEL_LB_STATUS_INVALID_COMMAND: Invalid command number
 * @INTEL_LB_STATUS_INVALID_HECI_HEADER: Invalid transport header
 * @INTEL_LB_STATUS_IP_ERROR_START: Base for internal errors
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_lb_status {
    INTEL_LB_STATUS_SUCCESS = 0,
    INTEL_LB_STATUS_4ID_MISMATCH = 1,
    INTEL_LB_STATUS_ARB_FAILURE = 2,
    INTEL_LB_STATUS_GENERAL_ERROR = 3,
    INTEL_LB_STATUS_INVALID_PARAMS = 4,
    INTEL_LB_STATUS_INVALID_SIGNATURE = 5,
    INTEL_LB_STATUS_INVALID_PAYLOAD = 6,
    INTEL_LB_STATUS_TIMEOUT = 7,
    INTEL_LB_STATUS_BUFFER_TOO_SMALL = 8,
    INTEL_LB_STATUS_INTERNAL_ERROR = 9,
    INTEL_LB_STATUS_INVALID_FPT_TABLE = 10,
    INTEL_LB_STATUS_SIGNED_PAYLOAD_VERIFICATION_ERROR = 11,
    INTEL_LB_STATUS_SIGNED_PAYLOAD_INVALID_CPD = 12,
    INTEL_LB_STATUS_SIGNED_PAYLOAD_FW_VERSION_MISMATCH = 13,
    INTEL_LB_STATUS_SIGNED_PAYLOAD_INVALID_MANIFEST = 14,
    INTEL_LB_STATUS_SIGNED_PAYLOAD_INVALID_HASH = 15,
    INTEL_LB_STATUS_SIGNED_PAYLOAD_BINDING_TYPE_MISMATCH = 16,
    INTEL_LB_STATUS_SIGNED_PAYLOAD_HANDLE_SVN_FAILED = 17,
    INTEL_LB_STATUS_DESTINATION_MBOX_FAILURE = 18,
    INTEL_LB_STATUS_MISSING_LOADING_PATCH = 19,
    INTEL_LB_STATUS_INVALID_COMMAND = 20,
    INTEL_LB_STATUS_INVALID_HECI_HEADER = 21,
    INTEL_LB_STATUS_IP_ERROR_START = 1u32 << 31,
}

/**
 * struct intel_lb_component_ops - Ops for late binding services
 */
#[repr(C)]
pub struct intel_lb_component_ops {
    /**
     * @push_payload: Sends a payload to the authentication firmware
     *
     * @dev: Device struct corresponding to the mei device
     * @type: Payload type (see &enum intel_lb_type)
     * @flags: Payload flags bitmap (e.g. %INTEL_LB_FLAGS_IS_PERSISTENT)
     * @payload: Pointer to payload buffer
     * @payload_size: Payload buffer size in bytes
     *
     * Return: 0 success, negative errno value on transport failure,
     *         positive error status returned by firmware
     */
    pub push_payload: Option<unsafe extern "C" fn(
        dev: *mut device,
        type_: u32,
        flags: u32,
        payload: *const c_void,
        payload_size: usize,
    ) -> c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
