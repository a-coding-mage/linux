/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2012-2016 Mentor Graphics Inc.
 *
 * i.MX Queued image conversion support, with tiling and rotation.
 */

// Dependency supplied by video/imx-ipu-v3.h.

#[repr(C)]
pub struct ipu_image_convert_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ipu_image_convert_run {
    pub ctx: *mut ipu_image_convert_ctx,

    pub in_phys: dma_addr_t,
    pub out_phys: dma_addr_t,

    pub status: ::core::ffi::c_int,

    /* private: */
    /* internal to image converter, callers don't touch */
    pub list: list_head,
}

/**
 * typedef ipu_image_convert_cb_t - conversion callback function prototype
 *
 * @run:\tthe completed conversion run pointer
 * @ctx:\ta private context pointer for the callback
 */
pub type ipu_image_convert_cb_t = Option<
    unsafe extern "C" fn(run: *mut ipu_image_convert_run, ctx: *mut ::core::ffi::c_void),
>;

/**
 * ipu_image_convert_adjust() - adjust input/output images to IPU restrictions.
 *
 * @in:\t\tinput image format, adjusted on return
 * @out:\toutput image format, adjusted on return
 * @rot_mode:\trotation mode
 *
 * In V4L2, drivers can call ipu_image_convert_adjust() in .try_fmt.
 */
pub unsafe extern "C" fn ipu_image_convert_adjust(
    input: *mut ipu_image,
    output: *mut ipu_image,
    rot_mode: ipu_rotate_mode,
);

/**
 * ipu_image_convert_verify() - verify that input/output image formats
 *         and rotation mode meet IPU restrictions.
 */
pub unsafe extern "C" fn ipu_image_convert_verify(
    input: *mut ipu_image,
    output: *mut ipu_image,
    rot_mode: ipu_rotate_mode,
) -> ::core::ffi::c_int;

/**
 * ipu_image_convert_prepare() - prepare a conversion context.
 */
pub unsafe extern "C" fn ipu_image_convert_prepare(
    ipu: *mut ipu_soc,
    ic_task: ipu_ic_task,
    input: *mut ipu_image,
    output: *mut ipu_image,
    rot_mode: ipu_rotate_mode,
    complete: ipu_image_convert_cb_t,
    complete_context: *mut ::core::ffi::c_void,
) -> *mut ipu_image_convert_ctx;

/**
 * ipu_image_convert_unprepare() - unprepare a conversion context.
 */
pub unsafe extern "C" fn ipu_image_convert_unprepare(ctx: *mut ipu_image_convert_ctx);

/**
 * ipu_image_convert_queue() - queue a conversion run
 */
pub unsafe extern "C" fn ipu_image_convert_queue(
    run: *mut ipu_image_convert_run,
) -> ::core::ffi::c_int;

/**
 * ipu_image_convert_abort() - abort conversions
 */
pub unsafe extern "C" fn ipu_image_convert_abort(ctx: *mut ipu_image_convert_ctx);

/**
 * ipu_image_convert() - asynchronous image conversion request
 */
pub unsafe extern "C" fn ipu_image_convert(
    ipu: *mut ipu_soc,
    ic_task: ipu_ic_task,
    input: *mut ipu_image,
    output: *mut ipu_image,
    rot_mode: ipu_rotate_mode,
    complete: ipu_image_convert_cb_t,
    complete_context: *mut ::core::ffi::c_void,
) -> *mut ipu_image_convert_run;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
