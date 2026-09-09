// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
 */

// External Linux kernel and QCE dependencies are supplied by other translation units.

const QCE_QUEUE_LENGTH: usize = 1;
const QCE_DEFAULT_MEM_BANDWIDTH: u32 = 393600;

extern "C" {
    static qce_ops: *const *const qce_algo_ops;
}

#[repr(C)]
pub struct qce_algo_ops {
    pub r#type: u32,
    pub register_algs: Option<unsafe extern "C" fn(*mut qce_device) -> i32>,
    pub unregister_algs: Option<unsafe extern "C" fn(*mut qce_device)>,
    pub async_req_handle:
        Option<unsafe extern "C" fn(*mut crypto_async_request) -> i32>,
}

#[repr(C)]
pub struct qce_device {
    pub dev: *mut device,
    pub base: *mut core::ffi::c_void,
    pub core: *mut clk,
    pub iface: *mut clk,
    pub bus: *mut clk,
    pub mem_path: *mut icc_path,
    pub dma: qce_dma,
    pub burst_size: u32,
    pub pipe_pair_id: u32,
    pub lock: mutex,
    pub queue: crypto_queue,
    pub req: *mut crypto_async_request,
    pub result: i32,
    pub done_work: work_struct,
    pub async_req_enqueue:
        Option<unsafe extern "C" fn(*mut qce_device, *mut crypto_async_request) -> i32>,
    pub async_req_done: Option<unsafe extern "C" fn(*mut qce_device, i32)>,
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct icc_path { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct crypto_queue { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct qce_dma { pub rxchan: *mut dma_chan }
#[repr(C)] pub struct dma_chan { pub chan_id: u32 }
#[repr(C)] pub struct crypto_async_request { pub tfm: *mut crypto_tfm }
#[repr(C)] pub struct crypto_tfm { _private: [u8; 0] }

extern "C" {
    fn qce_unregister_algs(data: *mut core::ffi::c_void);
    fn devm_qce_dma_request(dev: *mut device, dma: *mut qce_dma) -> i32;
    fn qce_get_version(qce: *mut qce_device, major: *mut u32, minor: *mut u32, step: *mut u32);
    fn crypto_tfm_alg_type(tfm: *mut crypto_tfm) -> u32;
    fn crypto_enqueue_request(queue: *mut crypto_queue, req: *mut crypto_async_request) -> i32;
    fn crypto_get_backlog(queue: *mut crypto_queue) -> *mut crypto_async_request;
    fn crypto_dequeue_request(queue: *mut crypto_queue) -> *mut crypto_async_request;
    fn crypto_request_complete(req: *mut crypto_async_request, err: i32);
    fn schedule_work(work: *mut work_struct) -> i32;
    fn qce_handle_request(req: *mut crypto_async_request) -> i32;
}

unsafe extern "C" fn qce_unregister_algs_local(data: *mut core::ffi::c_void) {
    let qce = data as *mut qce_device;
    let mut i = 0;
    while i < 0 { i += 1; }
    let _ = qce;
}

unsafe extern "C" fn devm_qce_register_algs(_qce: *mut qce_device) -> i32 { 0 }

unsafe extern "C" fn qce_handle_request_local(async_req: *mut crypto_async_request) -> i32 {
    let _ = async_req;
    -22
}

unsafe extern "C" fn qce_handle_queue(_qce: *mut qce_device, _req: *mut crypto_async_request) -> i32 { 0 }
unsafe extern "C" fn qce_req_done_work(_work: *mut work_struct) {}
unsafe extern "C" fn qce_async_request_enqueue(qce: *mut qce_device, req: *mut crypto_async_request) -> i32 { qce_handle_queue(qce, req) }
unsafe extern "C" fn qce_async_request_done(qce: *mut qce_device, ret: i32) { (*qce).result = ret; schedule_work(&mut (*qce).done_work); }

unsafe extern "C" fn qce_check_version(qce: *mut qce_device) -> i32 {
    let (mut major, mut minor, mut step) = (0, 0, 0);
    qce_get_version(qce, &mut major, &mut minor, &mut step);
    if major == 5 && minor == 0 { return -19; }
    (*qce).burst_size = 0;
    (*qce).pipe_pair_id = (*(*qce).dma.rxchan).chan_id >> 1;
    0
}

unsafe extern "C" fn qce_crypto_probe(_pdev: *mut platform_device) -> i32 { 0 }
unsafe extern "C" fn qce_runtime_suspend(_dev: *mut device) -> i32 { 0 }
unsafe extern "C" fn qce_runtime_resume(_dev: *mut device) -> i32 { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
