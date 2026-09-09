/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* \file cc_request_mgr.h
 * Request Manager
 */

/* Dependency: cc_hw_queue_defs.h */

use core::ffi::c_int;

/* Opaque types supplied by the dependent headers. */
#[repr(C)]
pub struct cc_drvdata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cc_crypto_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cc_hw_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_async_request {
    _private: [u8; 0],
}

extern "C" {
    pub fn cc_req_mgr_init(drvdata: *mut cc_drvdata) -> c_int;

    /**
     * cc_send_request() - Enqueue caller request to crypto hardware.
     *
     * @drvdata: Associated device driver context
     * @cc_req: The request to enqueue
     * @desc: The crypto sequence
     * @len: The crypto sequence length
     * @req: Asynchronous crypto request
     *
     * Return:
     * Returns -EINPROGRESS or error
     */
    pub fn cc_send_request(
        drvdata: *mut cc_drvdata,
        cc_req: *mut cc_crypto_req,
        desc: *mut cc_hw_desc,
        len: u32,
        req: *mut crypto_async_request,
    ) -> c_int;

    pub fn cc_send_sync_request(
        drvdata: *mut cc_drvdata,
        cc_req: *mut cc_crypto_req,
        desc: *mut cc_hw_desc,
        len: u32,
    ) -> c_int;

    pub fn send_request_init(
        drvdata: *mut cc_drvdata,
        desc: *mut cc_hw_desc,
        len: u32,
    ) -> c_int;

    pub fn complete_request(drvdata: *mut cc_drvdata);

    pub fn cc_req_mgr_fini(drvdata: *mut cc_drvdata);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
