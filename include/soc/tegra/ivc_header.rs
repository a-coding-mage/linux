/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016, NVIDIA CORPORATION.  All rights reserved.
 */

// C dependencies: linux/device.h, linux/dma-mapping.h, linux/iosys-map.h,
// and linux/types.h provide `device`, `iosys_map`, and `dma_addr_t`.

use core::ffi::c_void;

#[repr(C)]
pub struct tegra_ivc_header;

#[repr(C)]
pub struct tegra_ivc_map_state {
    pub map: iosys_map,
    pub position: core::ffi::c_uint,
    pub phys: dma_addr_t,
}

#[repr(C)]
pub struct tegra_ivc {
    pub peer: *mut device,
    pub rx: tegra_ivc_map_state,
    pub tx: tegra_ivc_map_state,
    pub notify: Option<unsafe extern "C" fn(ivc: *mut tegra_ivc, data: *mut c_void)>,
    pub notify_data: *mut c_void,
    pub num_frames: core::ffi::c_uint,
    pub frame_size: usize,
}

/**
 * tegra_ivc_read_get_next_frame - Peek at the next frame to receive
 * @ivc  pointer of the IVC channel
 *
 * Peek at the next frame to be received, without removing it from
 * the queue.
 *
 * Returns a pointer to the frame, or an error encoded pointer.
 */
extern "C" {
    pub fn tegra_ivc_read_get_next_frame(
        ivc: *mut tegra_ivc,
        map: *mut iosys_map,
    ) -> core::ffi::c_int;
}

/**
 * tegra_ivc_read_advance - Advance the read queue
 * @ivc  pointer of the IVC channel
 *
 * Advance the read queue
 *
 * Returns 0, or a negative error value if failed.
 */
extern "C" {
    pub fn tegra_ivc_read_advance(ivc: *mut tegra_ivc) -> core::ffi::c_int;
}

/**
 * tegra_ivc_write_get_next_frame - Poke at the next frame to transmit
 * @ivc  pointer of the IVC channel
 *
 * Get access to the next frame.
 *
 * Returns a pointer to the frame, or an error encoded pointer.
 */
extern "C" {
    pub fn tegra_ivc_write_get_next_frame(
        ivc: *mut tegra_ivc,
        map: *mut iosys_map,
    ) -> core::ffi::c_int;
}

/**
 * tegra_ivc_write_advance - Advance the write queue
 * @ivc  pointer of the IVC channel
 *
 * Advance the write queue
 *
 * Returns 0, or a negative error value if failed.
 */
extern "C" {
    pub fn tegra_ivc_write_advance(ivc: *mut tegra_ivc) -> core::ffi::c_int;
}

/**
 * tegra_ivc_notified - handle internal messages
 * @ivc  pointer of the IVC channel
 *
 * This function must be called following every notification.
 *
 * Returns 0 if the channel is ready for communication, or -EAGAIN if a channel
 * reset is in progress.
 */
extern "C" {
    pub fn tegra_ivc_notified(ivc: *mut tegra_ivc) -> core::ffi::c_int;
}

/**
 * tegra_ivc_reset - initiates a reset of the shared memory state
 * @ivc  pointer of the IVC channel
 *
 * This function must be called after a channel is reserved before it is used
 * for communication. The channel will be ready for use when a subsequent call
 * to notify the remote of the channel reset.
 */
extern "C" {
    pub fn tegra_ivc_reset(ivc: *mut tegra_ivc);

    pub fn tegra_ivc_align(size: usize) -> usize;
    pub fn tegra_ivc_total_queue_size(queue_size: core::ffi::c_uint) -> core::ffi::c_uint;
    pub fn tegra_ivc_init(
        ivc: *mut tegra_ivc,
        peer: *mut device,
        rx: *const iosys_map,
        rx_phys: dma_addr_t,
        tx: *const iosys_map,
        tx_phys: dma_addr_t,
        num_frames: core::ffi::c_uint,
        frame_size: usize,
        notify: Option<unsafe extern "C" fn(ivc: *mut tegra_ivc, data: *mut c_void)>,
        data: *mut c_void,
    ) -> core::ffi::c_int;
    pub fn tegra_ivc_cleanup(ivc: *mut tegra_ivc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
