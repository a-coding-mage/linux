/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of buffer_impl.h.  CONFIG_IIO_BUFFER controls availability of
 * the declarations below in the original build. */

use core::ffi::{c_char, c_int, c_void};

pub const INDIO_BUFFER_FLAG_FIXED_WATERMARK: u32 = 1u32 << 0;

pub type size_t = usize;
pub type bool_t = bool;
pub type iio_buffer_direction = c_int;

#[repr(C)]
pub struct dma_buf_attachment {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_fence {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iio_dma_buffer_block {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sg_table {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iio_dev_attr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kref {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_buffer_access_funcs {
    pub store_to: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, data: *const c_void) -> c_int>,
    pub read: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, n: size_t, buf: *mut c_char) -> c_int>,
    pub data_available: Option<unsafe extern "C" fn(buffer: *mut iio_buffer) -> size_t>,
    pub remove_from: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, data: *mut c_void) -> c_int>,
    pub write: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, n: size_t, buf: *const c_char) -> c_int>,
    pub space_available: Option<unsafe extern "C" fn(buffer: *mut iio_buffer) -> size_t>,
    pub request_update: Option<unsafe extern "C" fn(buffer: *mut iio_buffer) -> c_int>,
    pub set_bytes_per_datum: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, bpd: size_t) -> c_int>,
    pub set_length: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, length: u32) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, indio_dev: *mut iio_dev) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, indio_dev: *mut iio_dev) -> c_int>,
    pub release: Option<unsafe extern "C" fn(buffer: *mut iio_buffer)>,
    pub attach_dmabuf: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, attach: *mut dma_buf_attachment) -> *mut iio_dma_buffer_block>,
    pub detach_dmabuf: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, block: *mut iio_dma_buffer_block)>,
    pub enqueue_dmabuf: Option<unsafe extern "C" fn(buffer: *mut iio_buffer, block: *mut iio_dma_buffer_block, fence: *mut dma_fence, sgt: *mut sg_table, size: size_t, cyclic: bool) -> c_int>,
    pub get_dma_dev: Option<unsafe extern "C" fn(buffer: *mut iio_buffer) -> *mut device>,
    pub lock_queue: Option<unsafe extern "C" fn(buffer: *mut iio_buffer)>,
    pub unlock_queue: Option<unsafe extern "C" fn(buffer: *mut iio_buffer)>,
    pub modes: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct iio_buffer {
    pub length: u32,
    pub flags: usize,
    pub bytes_per_datum: size_t,
    pub direction: iio_buffer_direction,
    pub access: *const iio_buffer_access_funcs,
    pub scan_mask: *mut isize,
    pub demux_list: list_head,
    pub pollq: wait_queue_head_t,
    pub watermark: u32,
    pub scan_timestamp: bool,
    pub buffer_attr_list: list_head,
    pub buffer_group: attribute_group,
    pub attrs: *const *const iio_dev_attr,
    pub demux_bounce: *mut c_void,
    pub attached_entry: list_head,
    pub buffer_list: list_head,
    pub ref_: kref,
    pub dmabufs: list_head,
    pub dmabufs_mutex: mutex,
}

unsafe extern "C" {
    pub fn iio_update_buffers(indio_dev: *mut iio_dev, insert_buffer: *mut iio_buffer, remove_buffer: *mut iio_buffer) -> c_int;
    pub fn iio_buffer_init(buffer: *mut iio_buffer);
    pub fn iio_buffer_get(buffer: *mut iio_buffer) -> *mut iio_buffer;
    pub fn iio_buffer_put(buffer: *mut iio_buffer);
    pub fn iio_buffer_signal_dmabuf_done(fence: *mut dma_fence, ret: c_int);
}

/* When CONFIG_IIO_BUFFER is disabled, the original header provides no-op
 * inline reference helpers. */
#[inline]
pub unsafe fn iio_buffer_get_disabled(_buffer: *mut iio_buffer) {}
#[inline]
pub unsafe fn iio_buffer_put_disabled(_buffer: *mut iio_buffer) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
