/* SPDX-License-Identifier: GPL-2.0-only */
/* Header file for dma buffer sharing framework. */

// C dependencies supplied by other translated headers are intentionally not implemented here.

#[repr(C)]
pub struct dma_buf_ops {
    pub attach: Option<unsafe extern "C" fn(*mut dma_buf, *mut dma_buf_attachment) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut dma_buf, *mut dma_buf_attachment)>,
    pub pin: Option<unsafe extern "C" fn(*mut dma_buf_attachment) -> i32>,
    pub unpin: Option<unsafe extern "C" fn(*mut dma_buf_attachment)>,
    pub map_dma_buf: Option<unsafe extern "C" fn(*mut dma_buf_attachment, enum_dma_data_direction) -> *mut sg_table>,
    pub unmap_dma_buf: Option<unsafe extern "C" fn(*mut dma_buf_attachment, *mut sg_table, enum_dma_data_direction)>,
    pub release: Option<unsafe extern "C" fn(*mut dma_buf)>,
    pub begin_cpu_access: Option<unsafe extern "C" fn(*mut dma_buf, enum_dma_data_direction) -> i32>,
    pub end_cpu_access: Option<unsafe extern "C" fn(*mut dma_buf, enum_dma_data_direction) -> i32>,
    pub mmap: Option<unsafe extern "C" fn(*mut dma_buf, *mut vm_area_struct) -> i32>,
    pub vmap: Option<unsafe extern "C" fn(*mut dma_buf, *mut iosys_map) -> i32>,
    pub vunmap: Option<unsafe extern "C" fn(*mut dma_buf, *mut iosys_map)>,
}

#[repr(C)]
pub struct dma_buf {
    pub size: usize,
    pub file: *mut file,
    pub attachments: list_head,
    pub ops: *const dma_buf_ops,
    pub vmapping_counter: u32,
    pub vmap_ptr: iosys_map,
    pub exp_name: *const core::ffi::c_char,
    pub name: *const core::ffi::c_char,
    pub name_lock: spinlock_t,
    pub owner: *mut module,
    pub list_node: list_head,
    pub r#priv: *mut core::ffi::c_void,
    pub resv: *mut dma_resv,
    pub poll: wait_queue_head_t,
    pub cb_in: dma_buf_poll_cb_t,
    pub cb_out: dma_buf_poll_cb_t,
}

#[repr(C)]
pub struct dma_buf_poll_cb_t {
    pub cb: dma_fence_cb,
    pub poll: *mut wait_queue_head_t,
    pub active: __poll_t,
}

#[repr(C)]
pub struct dma_buf_attach_ops {
    pub allow_peer2peer: bool,
    pub invalidate_mappings: Option<unsafe extern "C" fn(*mut dma_buf_attachment)>,
}

#[repr(C)]
pub struct dma_buf_attachment {
    pub dmabuf: *mut dma_buf,
    pub dev: *mut device,
    pub node: list_head,
    pub peer2peer: bool,
    pub importer_ops: *const dma_buf_attach_ops,
    pub importer_priv: *mut core::ffi::c_void,
    pub r#priv: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct dma_buf_export_info {
    pub exp_name: *const core::ffi::c_char,
    pub owner: *mut module,
    pub ops: *const dma_buf_ops,
    pub size: usize,
    pub flags: i32,
    pub resv: *mut dma_resv,
    pub r#priv: *mut core::ffi::c_void,
}

#[macro_export]
macro_rules! DEFINE_DMA_BUF_EXPORT_INFO {
    ($name:ident) => {
        let mut $name: dma_buf_export_info = dma_buf_export_info {
            exp_name: KBUILD_MODNAME,
            owner: THIS_MODULE,
            ops: core::ptr::null(),
            size: 0,
            flags: 0,
            resv: core::ptr::null_mut(),
            r#priv: core::ptr::null_mut(),
        };
    };
}

#[inline]
pub unsafe fn get_dma_buf(dmabuf: *mut dma_buf) {
    get_file((*dmabuf).file);
}

#[inline]
pub unsafe fn dma_buf_is_dynamic(dmabuf: *mut dma_buf) -> bool {
    !(*(*dmabuf).ops).pin.is_none()
}

extern "C" {
    pub fn dma_buf_attach(dmabuf: *mut dma_buf, dev: *mut device) -> *mut dma_buf_attachment;
    pub fn dma_buf_dynamic_attach(dmabuf: *mut dma_buf, dev: *mut device, importer_ops: *const dma_buf_attach_ops, importer_priv: *mut core::ffi::c_void) -> *mut dma_buf_attachment;
    pub fn dma_buf_detach(dmabuf: *mut dma_buf, attach: *mut dma_buf_attachment);
    pub fn dma_buf_pin(attach: *mut dma_buf_attachment) -> i32;
    pub fn dma_buf_unpin(attach: *mut dma_buf_attachment);
    pub fn dma_buf_export(exp_info: *const dma_buf_export_info) -> *mut dma_buf;
    pub fn dma_buf_fd(dmabuf: *mut dma_buf, flags: i32) -> i32;
    pub fn dma_buf_get(fd: i32) -> *mut dma_buf;
    pub fn dma_buf_put(dmabuf: *mut dma_buf);
    pub fn dma_buf_map_attachment(attach: *mut dma_buf_attachment, direction: enum_dma_data_direction) -> *mut sg_table;
    pub fn dma_buf_unmap_attachment(attach: *mut dma_buf_attachment, sg_table: *mut sg_table, direction: enum_dma_data_direction);
    pub fn dma_buf_invalidate_mappings(dma_buf: *mut dma_buf);
    pub fn dma_buf_attach_revocable(attach: *mut dma_buf_attachment) -> bool;
    pub fn dma_buf_begin_cpu_access(dma_buf: *mut dma_buf, dir: enum_dma_data_direction) -> i32;
    pub fn dma_buf_end_cpu_access(dma_buf: *mut dma_buf, dir: enum_dma_data_direction) -> i32;
    pub fn dma_buf_map_attachment_unlocked(attach: *mut dma_buf_attachment, direction: enum_dma_data_direction) -> *mut sg_table;
    pub fn dma_buf_unmap_attachment_unlocked(attach: *mut dma_buf_attachment, sg_table: *mut sg_table, direction: enum_dma_data_direction);
    pub fn dma_buf_mmap(dmabuf: *mut dma_buf, vma: *mut vm_area_struct, pgoff: c_ulong) -> i32;
    pub fn dma_buf_vmap(dmabuf: *mut dma_buf, map: *mut iosys_map) -> i32;
    pub fn dma_buf_vunmap(dmabuf: *mut dma_buf, map: *mut iosys_map);
    pub fn dma_buf_vmap_unlocked(dmabuf: *mut dma_buf, map: *mut iosys_map) -> i32;
    pub fn dma_buf_vunmap_unlocked(dmabuf: *mut dma_buf, map: *mut iosys_map);
    pub fn dma_buf_iter_begin() -> *mut dma_buf;
    pub fn dma_buf_iter_next(dmbuf: *mut dma_buf) -> *mut dma_buf;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
