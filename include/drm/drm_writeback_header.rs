/* SPDX-License-Identifier: GPL-2.0 */
/*
 * (C) COPYRIGHT 2016 ARM Limited. All rights reserved.
 * Author: Brian Starkey <brian.starkey@arm.com>
 *
 * This program is free software and is provided to you under the terms of the
 * GNU General Public License version 2 as published by the Free Software
 * Foundation, and any use of this program is subject to the terms of such
 * GNU licence.
 */

// Dependencies supplied by the corresponding DRM and Linux headers.

/**
 * struct drm_writeback_connector - DRM writeback connector
 */
#[repr(C)]
pub struct drm_writeback_connector {
    /** @base: base drm_connector object */
    pub base: drm_connector,

    /**
     * @encoder: Internal encoder used by the connector to fulfill
     * the DRM framework requirements. The users of the
     * @drm_writeback_connector control the behaviour of the @encoder
     * by passing the @enc_funcs parameter to drm_writeback_connector_init()
     * function.
     * For users of drm_writeback_connector_init_with_encoder(), this field
     * is not valid as the encoder is managed within their drivers.
     */
    pub encoder: drm_encoder,

    /**
     * @pixel_formats_blob_ptr:
     *
     * DRM blob property data for the pixel formats list on writeback
     * connectors
     * See also drm_writeback_connector_init()
     */
    pub pixel_formats_blob_ptr: *mut drm_property_blob,

    /** @job_lock: Protects job_queue */
    pub job_lock: spinlock_t,

    /**
     * @job_queue:
     *
     * Holds a list of a connector's writeback jobs; the last item is the
     * most recent. The first item may be either waiting for the hardware
     * to begin writing, or currently being written.
     *
     * See also: drm_writeback_queue_job() and
     * drm_writeback_signal_completion()
     */
    pub job_queue: list_head,

    /** @fence_context: timeline context used for fence operations. */
    pub fence_context: u32,
    /** @fence_lock: spinlock to protect the fences in the fence_context. */
    pub fence_lock: spinlock_t,
    /** @fence_seqno: Seqno variable used as monotonic counter for the fences
     * created on the connector's timeline.
     */
    pub fence_seqno: c_ulong,
    /** @timeline_name: The name of the connector's fence timeline. */
    pub timeline_name: [c_char; 32],
}

/**
 * struct drm_writeback_job - DRM writeback job
 */
#[repr(C)]
pub struct drm_writeback_job {
    /** @connector: Back-pointer to the writeback connector associated with the job */
    pub connector: *mut drm_writeback_connector,
    /** @prepared: Set when the job has been prepared with drm_writeback_prepare_job() */
    pub prepared: bool,
    /** @cleanup_work: Used to allow drm_writeback_signal_completion to defer dropping the
     * framebuffer reference to a workqueue
     */
    pub cleanup_work: work_struct,
    /** @list_entry: List item for the writeback connector's @job_queue */
    pub list_entry: list_head,
    /** @fb: Framebuffer to be written to by the writeback connector. Do not set
     * directly, use drm_writeback_set_fb()
     */
    pub fb: *mut drm_framebuffer,
    /** @out_fence: Fence which will signal once the writeback has completed */
    pub out_fence: *mut dma_fence,
    /** @priv: Driver-private data */
    pub priv_: *mut c_void,
}

pub unsafe fn drm_connector_to_writeback(
    connector: *mut drm_connector,
) -> *mut drm_writeback_connector {
    container_of!(connector, drm_writeback_connector, base)
}

extern "C" {
    pub fn drm_writeback_connector_init(
        dev: *mut drm_device,
        wb_connector: *mut drm_writeback_connector,
        con_funcs: *const drm_connector_funcs,
        enc_helper_funcs: *const drm_encoder_helper_funcs,
        formats: *const u32,
        n_formats: c_int,
        possible_crtcs: u32,
    ) -> c_int;

    pub fn drm_writeback_connector_init_with_encoder(
        dev: *mut drm_device,
        wb_connector: *mut drm_writeback_connector,
        enc: *mut drm_encoder,
        con_funcs: *const drm_connector_funcs,
        formats: *const u32,
        n_formats: c_int,
    ) -> c_int;

    pub fn drmm_writeback_connector_init(
        dev: *mut drm_device,
        wb_connector: *mut drm_writeback_connector,
        con_funcs: *const drm_connector_funcs,
        enc: *mut drm_encoder,
        formats: *const u32,
        n_formats: c_int,
    ) -> c_int;

    pub fn drm_writeback_set_fb(
        conn_state: *mut drm_connector_state,
        fb: *mut drm_framebuffer,
    ) -> c_int;

    pub fn drm_writeback_prepare_job(job: *mut drm_writeback_job) -> c_int;

    pub fn drm_writeback_queue_job(
        wb_connector: *mut drm_writeback_connector,
        conn_state: *mut drm_connector_state,
    );

    pub fn drm_writeback_cleanup_job(job: *mut drm_writeback_job);

    pub fn drm_writeback_signal_completion(
        wb_connector: *mut drm_writeback_connector,
        status: c_int,
    );

    pub fn drm_writeback_get_out_fence(
        wb_connector: *mut drm_writeback_connector,
    ) -> *mut dma_fence;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
