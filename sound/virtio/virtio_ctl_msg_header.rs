// SPDX-License-Identifier: GPL-2.0+
// virtio-snd: Virtio sound device
// Copyright (C) 2021 OpenSynergy GmbH

// Requires: linux/atomic.h, linux/virtio.h (external dependencies)

use std::ffi::c_void;

// Forward declarations for external types
pub struct virtio_snd;
pub struct virtio_snd_msg;
pub struct virtqueue;
pub struct scatterlist;

// gfp_t represents GFP allocation flags from the Linux kernel
pub type gfp_t = u32;

extern "C" {
    pub fn virtsnd_ctl_msg_ref(msg: *mut virtio_snd_msg);

    pub fn virtsnd_ctl_msg_unref(msg: *mut virtio_snd_msg);

    pub fn virtsnd_ctl_msg_request(msg: *mut virtio_snd_msg) -> *mut c_void;

    pub fn virtsnd_ctl_msg_response(msg: *mut virtio_snd_msg) -> *mut c_void;

    pub fn virtsnd_ctl_msg_alloc(
        request_size: usize,
        response_size: usize,
        gfp: gfp_t,
    ) -> *mut virtio_snd_msg;

    pub fn virtsnd_ctl_msg_send(
        snd: *mut virtio_snd,
        msg: *mut virtio_snd_msg,
        out_sgs: *mut scatterlist,
        in_sgs: *mut scatterlist,
        nowait: bool,
    ) -> i32;

    pub fn virtsnd_ctl_msg_cancel_all(snd: *mut virtio_snd);

    pub fn virtsnd_ctl_msg_complete(msg: *mut virtio_snd_msg);

    pub fn virtsnd_ctl_query_info(
        snd: *mut virtio_snd,
        command: i32,
        start_id: i32,
        count: i32,
        size: usize,
        info: *mut c_void,
    ) -> i32;

    pub fn virtsnd_ctl_notify_cb(vqueue: *mut virtqueue);
}

/// Simplified sending of synchronous message.
///
/// # Arguments
///
/// * `snd` - VirtIO sound device
/// * `msg` - Control message
///
/// After returning from this function, the message will be deleted. If message
/// content is still needed, the caller must additionally call
/// virtsnd_ctl_msg_ref()/virtsnd_ctl_msg_unref() on it.
///
/// The msg_timeout_ms module parameter defines the message completion timeout.
/// If the message is not completed within this time, the function will return an
/// error.
///
/// # Context
///
/// Any context that permits to sleep.
///
/// # Returns
///
/// 0 on success, -errno on failure.
/// The return value is a message status code (VIRTIO_SND_S_XXX) converted to an
/// appropriate -errno value.
#[inline]
pub fn virtsnd_ctl_msg_send_sync(snd: *mut virtio_snd, msg: *mut virtio_snd_msg) -> i32 {
    unsafe {
        virtsnd_ctl_msg_send(
            snd,
            msg,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            false,
        )
    }
}

/// Simplified sending of asynchronous message.
///
/// # Arguments
///
/// * `snd` - VirtIO sound device
/// * `msg` - Control message
///
/// # Context
///
/// Any context.
///
/// # Returns
///
/// 0 on success, -errno on failure.
#[inline]
pub fn virtsnd_ctl_msg_send_async(snd: *mut virtio_snd, msg: *mut virtio_snd_msg) -> i32 {
    unsafe {
        virtsnd_ctl_msg_send(
            snd,
            msg,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            true,
        )
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
