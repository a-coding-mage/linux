/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2013-2015 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

/* C dependencies: linux/atomic.h, linux/list.h, linux/kref.h,
 * linux/spinlock.h, linux/mutex.h, linux/iio/buffer_impl.h. */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct iio_dma_buffer_queue;
#[repr(C)]
pub struct iio_dma_buffer_ops;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct dma_buf_attachment;
#[repr(C)]
pub struct dma_fence;
#[repr(C)]
pub struct sg_table;
#[repr(C)]
pub struct iio_buffer;
#[repr(C)]
pub struct iio_dev;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct kref;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct spinlock_t;
#[repr(C)]
pub struct atomic_t;

pub type size_t = usize;
pub type dma_addr_t = usize;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_block_state {
    IIO_BLOCK_STATE_QUEUED,
    IIO_BLOCK_STATE_ACTIVE,
    IIO_BLOCK_STATE_DONE,
    IIO_BLOCK_STATE_DEAD,
}

#[repr(C)]
pub struct iio_dma_buffer_block {
    pub head: list_head,
    pub bytes_used: size_t,
    pub vaddr: *mut c_void,
    pub phys_addr: dma_addr_t,
    pub size: size_t,
    pub queue: *mut iio_dma_buffer_queue,
    pub kref: kref,
    pub state: iio_block_state,
    pub cyclic: bool,
    pub fileio: bool,
    pub sg_table: *mut sg_table,
    pub fence: *mut dma_fence,
}

#[repr(C)]
pub struct iio_dma_buffer_queue_fileio {
    pub blocks: [*mut iio_dma_buffer_block; 2],
    pub active_block: *mut iio_dma_buffer_block,
    pub pos: size_t,
    pub block_size: size_t,
    pub next_dequeue: c_uint,
    pub enabled: bool,
}

#[repr(C)]
pub struct iio_dma_buffer_queue {
    pub buffer: iio_buffer,
    pub dev: *mut device,
    pub ops: *const iio_dma_buffer_ops,
    pub lock: mutex,
    pub list_lock: spinlock_t,
    pub incoming: list_head,
    pub active: bool,
    pub num_dmabufs: atomic_t,
    pub fileio: iio_dma_buffer_queue_fileio,
}

#[repr(C)]
pub struct iio_dma_buffer_ops {
    pub submit: Option<unsafe extern "C" fn(
        queue: *mut iio_dma_buffer_queue,
        block: *mut iio_dma_buffer_block,
    ) -> c_int>,
    pub abort: Option<unsafe extern "C" fn(queue: *mut iio_dma_buffer_queue)>,
}

extern "C" {
    pub fn iio_dma_buffer_block_done(block: *mut iio_dma_buffer_block);
    pub fn iio_dma_buffer_block_list_abort(
        queue: *mut iio_dma_buffer_queue,
        list: *mut list_head,
    );

    pub fn iio_dma_buffer_enable(buffer: *mut iio_buffer, indio_dev: *mut iio_dev) -> c_int;
    pub fn iio_dma_buffer_disable(buffer: *mut iio_buffer, indio_dev: *mut iio_dev) -> c_int;
    pub fn iio_dma_buffer_read(buffer: *mut iio_buffer, n: size_t, user_buffer: *mut c_char) -> c_int;
    pub fn iio_dma_buffer_write(buffer: *mut iio_buffer, n: size_t, user_buffer: *const c_char) -> c_int;
    pub fn iio_dma_buffer_usage(buffer: *mut iio_buffer) -> size_t;
    pub fn iio_dma_buffer_set_bytes_per_datum(buffer: *mut iio_buffer, bpd: size_t) -> c_int;
    pub fn iio_dma_buffer_set_length(buffer: *mut iio_buffer, length: c_uint) -> c_int;
    pub fn iio_dma_buffer_request_update(buffer: *mut iio_buffer) -> c_int;

    pub fn iio_dma_buffer_init(
        queue: *mut iio_dma_buffer_queue,
        dev: *mut device,
        ops: *const iio_dma_buffer_ops,
    );
    pub fn iio_dma_buffer_exit(queue: *mut iio_dma_buffer_queue);
    pub fn iio_dma_buffer_release(queue: *mut iio_dma_buffer_queue);

    pub fn iio_dma_buffer_attach_dmabuf(
        buffer: *mut iio_buffer,
        attach: *mut dma_buf_attachment,
    ) -> *mut iio_dma_buffer_block;
    pub fn iio_dma_buffer_detach_dmabuf(
        buffer: *mut iio_buffer,
        block: *mut iio_dma_buffer_block,
    );
    pub fn iio_dma_buffer_enqueue_dmabuf(
        buffer: *mut iio_buffer,
        block: *mut iio_dma_buffer_block,
        fence: *mut dma_fence,
        sgt: *mut sg_table,
        size: size_t,
        cyclic: bool,
    ) -> c_int;
    pub fn iio_dma_buffer_lock_queue(buffer: *mut iio_buffer);
    pub fn iio_dma_buffer_unlock_queue(buffer: *mut iio_buffer);
    pub fn iio_dma_buffer_get_dma_dev(buffer: *mut iio_buffer) -> *mut device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
