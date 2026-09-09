/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
 */

use core::ffi::c_void;

/* Dependencies supplied by the surrounding translation unit. */

/**
 * struct qce_device - crypto engine device structure
 * @queue: crypto request queue
 * @lock: the lock protects queue and req
 * @done_work: workqueue context
 * @req: current active request
 * @result: result of current transform
 * @base: virtual IO base
 * @dev: pointer to device structure
 * @core: core device clock
 * @iface: interface clock
 * @bus: bus clock
 * @dma: pointer to dma data
 * @burst_size: the crypto burst size
 * @pipe_pair_id: which pipe pair id the device using
 * @async_req_enqueue: invoked by every algorithm to enqueue a request
 * @async_req_done: invoked by every algorithm to finish its request
 */
#[repr(C)]
pub struct qce_device {
    pub queue: crypto_queue,
    pub lock: mutex,
    pub done_work: work_struct,
    pub req: *mut crypto_async_request,
    pub result: i32,
    pub base: *mut c_void,
    pub dev: *mut device,
    pub core: *mut clk,
    pub iface: *mut clk,
    pub bus: *mut clk,
    pub mem_path: *mut icc_path,
    pub dma: qce_dma_data,
    pub burst_size: i32,
    pub pipe_pair_id: u32,
    pub async_req_enqueue: Option<unsafe extern "C" fn(
        qce: *mut qce_device,
        req: *mut crypto_async_request,
    ) -> i32>,
    pub async_req_done: Option<unsafe extern "C" fn(qce: *mut qce_device, ret: i32)>,
}

/**
 * struct qce_algo_ops - algorithm operations per crypto type
 * @type: should be CRYPTO_ALG_TYPE_XXX
 * @register_algs: invoked by core to register the algorithms
 * @unregister_algs: invoked by core to unregister the algorithms
 * @async_req_handle: invoked by core to handle enqueued request
 */
#[repr(C)]
pub struct qce_algo_ops {
    pub type_: u32,
    pub register_algs: Option<unsafe extern "C" fn(qce: *mut qce_device) -> i32>,
    pub unregister_algs: Option<unsafe extern "C" fn(qce: *mut qce_device)>,
    pub async_req_handle:
        Option<unsafe extern "C" fn(async_req: *mut crypto_async_request) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
