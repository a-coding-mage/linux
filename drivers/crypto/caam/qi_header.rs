/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Public definitions for the CAAM/QI (Queue Interface) backend.
 *
 * Copyright 2013-2016 Freescale Semiconductor, Inc.
 * Copyright 2016-2017, 2020 NXP
 */

/* C dependencies: crypto/algapi.h, linux/compiler_attributes.h,
 * soc/fsl/qman.h, compat.h, desc.h, and desc_constr.h. */

/* Length of a single buffer in the QI driver memory cache */
pub const CAAM_QI_MEMCACHE_SIZE: usize = 768;

/* C: extern bool caam_congested __read_mostly; */
extern "C" {
    pub static mut caam_congested: bool;
}

/* This is the request structure the driver application should fill while
 * submitting a job to driver. */
#[repr(C)]
pub struct caam_drv_req;

/* caam_qi_cbk - application's callback function invoked by the driver when the
 * request has been successfully processed.
 * @drv_req: original request that was submitted
 * @status: completion status of request (0 - success, non-zero - error code)
 */
pub type caam_qi_cbk = Option<unsafe extern "C" fn(drv_req: *mut caam_drv_req, status: u32)>;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum optype {
    ENCRYPT,
    DECRYPT,
    NUM_OP,
}

/**
 * caam_drv_ctx - CAAM/QI backend driver context
 *
 * The jobs are processed by the driver against a driver context.
 * With every cryptographic context, a driver context is attached.
 * The driver context contains data for private use by driver.
 * For the applications, this is an opaque structure.
 *
 * @prehdr: preheader placed before shrd desc
 * @sh_desc: shared descriptor
 * @context_a: shared descriptor dma address
 * @req_fq: to-CAAM request frame queue
 * @rsp_fq: from-CAAM response frame queue
 * @refcnt: reference counter incremented for each frame enqueued in to-CAAM FQ
 * @cpu: cpu on which to receive CAAM response
 * @op_type: operation type
 * @qidev: device pointer for CAAM/QI backend
 */
#[repr(C)]
pub struct caam_drv_ctx {
    pub prehdr: [u32; 2],
    pub sh_desc: [u32; MAX_SDLEN],
    pub context_a: dma_addr_t,
    pub req_fq: *mut qman_fq,
    pub rsp_fq: *mut qman_fq,
    pub refcnt: refcount_t,
    pub cpu: i32,
    pub op_type: optype,
    pub qidev: *mut device,
}

/**
 * caam_drv_req - The request structure the driver application should fill while
 *                submitting a job to driver.
 * @fd_sgt: QMan S/G pointing to output (fd_sgt[0]) and input (fd_sgt[1])
 *          buffers.
 * @cbk: callback function to invoke when job is completed
 * @app_ctx: arbitrary context attached with request by the application
 *
 * The fields mentioned below should not be used by application.
 * These are for private use by driver.
 *
 * @hdr__: linked list header to maintain list of outstanding requests to CAAM
 * @hwaddr: DMA address for the S/G table.
 */
#[repr(C)]
pub struct caam_drv_req {
    pub fd_sgt: [qm_sg_entry; 2],
    pub drv_ctx: *mut caam_drv_ctx,
    pub cbk: caam_qi_cbk,
    pub app_ctx: *mut core::ffi::c_void,
}

extern "C" {
    pub fn caam_drv_ctx_init(qidev: *mut device, cpu: *mut i32, sh_desc: *mut u32) -> *mut caam_drv_ctx;
    pub fn caam_qi_enqueue(qidev: *mut device, req: *mut caam_drv_req) -> i32;
    pub fn caam_drv_ctx_busy(drv_ctx: *mut caam_drv_ctx) -> bool;
    pub fn caam_drv_ctx_update(drv_ctx: *mut caam_drv_ctx, sh_desc: *mut u32) -> i32;
    pub fn caam_drv_ctx_rel(drv_ctx: *mut caam_drv_ctx);
    pub fn caam_qi_init(pdev: *mut platform_device) -> i32;
    pub fn qi_cache_alloc(flags: gfp_t) -> *mut core::ffi::c_void;
    pub fn qi_cache_free(obj: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
