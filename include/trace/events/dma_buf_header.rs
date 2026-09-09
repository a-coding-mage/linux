/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the Linux dma_buf tracepoint header.
// The C tracepoint infrastructure and the types supplied by the included
// kernel headers remain external dependencies.

use core::ffi::c_void;

/// Entry data shared by the `dma_buf` event class.
#[repr(C)]
pub struct DmaBufEntry {
    pub exp_name: *const core::ffi::c_char,
    pub size: usize,
    pub ino: usize,
}

/// Entry data shared by the `dma_buf_attach_dev` event class.
#[repr(C)]
pub struct DmaBufAttachDevEntry {
    pub dev_name: *const core::ffi::c_char,
    pub exp_name: *const core::ffi::c_char,
    pub size: usize,
    pub ino: usize,
    pub attach: *mut c_void,
    pub is_dynamic: bool,
}

/// Entry data shared by the `dma_buf_fd` event class.
#[repr(C)]
pub struct DmaBufFdEntry {
    pub exp_name: *const core::ffi::c_char,
    pub size: usize,
    pub ino: usize,
    pub fd: i32,
}

// DECLARE_EVENT_CLASS(dma_buf)
// TP_PROTO(struct dma_buf *dmabuf)
// TP_fast_assign:
//   __assign_str(exp_name); __entry->size = dmabuf->size;
//   __entry->ino = dmabuf->file->f_inode->i_ino;
// TP_printk("exp_name=%s size=%zu ino=%lu", __get_str(exp_name),
//           __entry->size, __entry->ino)

// DECLARE_EVENT_CLASS(dma_buf_attach_dev)
// TP_PROTO(struct dma_buf *dmabuf, struct dma_buf_attachment *attach,
//          bool is_dynamic, struct device *dev)
// TP_fast_assign:
//   __assign_str(dev_name); __assign_str(exp_name);
//   __entry->size = dmabuf->size;
//   __entry->ino = dmabuf->file->f_inode->i_ino;
//   __entry->is_dynamic = is_dynamic; __entry->attach = attach;
// TP_printk("exp_name=%s size=%zu ino=%lu attachment:%p is_dynamic=%d dev_name=%s", ...)

// DECLARE_EVENT_CLASS(dma_buf_fd)
// TP_PROTO(struct dma_buf *dmabuf, int fd)
// TP_fast_assign:
//   __assign_str(exp_name); __entry->size = dmabuf->size;
//   __entry->ino = dmabuf->file->f_inode->i_ino; __entry->fd = fd;
// TP_printk("exp_name=%s size=%zu ino=%lu fd=%d", __get_str(exp_name),
//           __entry->size, __entry->ino, __entry->fd)

// DEFINE_EVENT(dma_buf, dma_buf_export)
// DEFINE_EVENT(dma_buf, dma_buf_mmap_internal)
// DEFINE_EVENT(dma_buf, dma_buf_mmap)
// DEFINE_EVENT(dma_buf, dma_buf_put)
// DEFINE_EVENT(dma_buf_attach_dev, dma_buf_dynamic_attach)
// DEFINE_EVENT(dma_buf_attach_dev, dma_buf_detach)
// DEFINE_EVENT_CONDITION(dma_buf_fd, dma_buf_fd, fd >= 0)
// DEFINE_EVENT(dma_buf_fd, dma_buf_get)


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
