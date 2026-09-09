/* SPDX-License-Identifier: GPL-2.0 */
/*
 * dma-bufs for virtio exported objects
 *
 * Copyright (C) 2020 Google, Inc.
 */

// Dependencies supplied by the corresponding Linux kernel interfaces.

#[repr(C)]
pub struct dma_buf_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_buf_attachment {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_buf_export_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct uuid_t {
    _private: [u8; 0],
}

/**
 * struct virtio_dma_buf_ops - operations possible on exported object dma-buf
 * @ops: the base dma_buf_ops. ops.attach MUST be virtio_dma_buf_attach.
 * @device_attach: [optional] callback invoked by virtio_dma_buf_attach during
 *                 all attach operations.
 * @get_uuid: [required] callback to get the uuid of the exported object.
 */
#[repr(C)]
pub struct virtio_dma_buf_ops {
    pub ops: dma_buf_ops,
    pub device_attach: Option<
        unsafe extern "C" fn(
            dma_buf: *mut dma_buf,
            attach: *mut dma_buf_attachment,
        ) -> i32,
    >,
    pub get_uuid: Option<
        unsafe extern "C" fn(dma_buf: *mut dma_buf, uuid: *mut uuid_t) -> i32,
    >,
}

pub unsafe extern "C" fn virtio_dma_buf_attach(
    dma_buf: *mut dma_buf,
    attach: *mut dma_buf_attachment,
) -> i32;

pub unsafe extern "C" fn virtio_dma_buf_export(
    exp_info: *const dma_buf_export_info,
) -> *mut dma_buf;

pub unsafe extern "C" fn is_virtio_dma_buf(dma_buf: *mut dma_buf) -> bool;

pub unsafe extern "C" fn virtio_dma_buf_get_uuid(
    dma_buf: *mut dma_buf,
    uuid: *mut uuid_t,
) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
