/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Memory-to-memory device framework for Video for Linux 2. */
/* C header dependency: <media/videobuf2-v4l2.h> */

#[repr(C)]
pub struct v4l2_m2m_ops {
    pub device_run: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void)>,
    pub job_ready: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void) -> i32>,
    pub job_abort: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct v4l2_m2m_queue_ctx {
    pub q: vb2_queue,
    pub rdy_queue: list_head,
    pub rdy_spinlock: spinlock_t,
    pub num_rdy: u8,
    pub buffered: bool,
}

#[repr(C)]
pub struct v4l2_m2m_ctx {
    pub q_lock: *mut mutex,
    pub new_frame: bool,
    pub is_draining: bool,
    pub last_src_buf: *mut vb2_v4l2_buffer,
    pub next_buf_last: bool,
    pub has_stopped: bool,
    pub ignore_cap_streaming: bool,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub cap_q_ctx: v4l2_m2m_queue_ctx,
    pub out_q_ctx: v4l2_m2m_queue_ctx,
    pub queue: list_head,
    pub job_flags: core::ffi::c_ulong,
    pub finished: wait_queue_head_t,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct v4l2_m2m_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

extern "C" {
    pub fn v4l2_m2m_get_curr_priv(m2m_dev: *mut v4l2_m2m_dev) -> *mut core::ffi::c_void;
    pub fn v4l2_m2m_get_vq(m2m_ctx: *mut v4l2_m2m_ctx, type_: v4l2_buf_type) -> *mut vb2_queue;
    pub fn v4l2_m2m_try_schedule(m2m_ctx: *mut v4l2_m2m_ctx);
    pub fn v4l2_m2m_job_finish(m2m_dev: *mut v4l2_m2m_dev, m2m_ctx: *mut v4l2_m2m_ctx);
    pub fn v4l2_m2m_buf_done_and_job_finish(m2m_dev: *mut v4l2_m2m_dev, m2m_ctx: *mut v4l2_m2m_ctx, state: vb2_buffer_state);
    pub fn v4l2_m2m_last_buffer_done(m2m_ctx: *mut v4l2_m2m_ctx, vbuf: *mut vb2_v4l2_buffer);
    pub fn v4l2_m2m_suspend(m2m_dev: *mut v4l2_m2m_dev);
    pub fn v4l2_m2m_resume(m2m_dev: *mut v4l2_m2m_dev);
    pub fn v4l2_m2m_reqbufs(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, reqbufs: *mut v4l2_requestbuffers) -> i32;
    pub fn v4l2_m2m_querybuf(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, buf: *mut v4l2_buffer) -> i32;
    pub fn v4l2_m2m_qbuf(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, buf: *mut v4l2_buffer) -> i32;
    pub fn v4l2_m2m_dqbuf(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, buf: *mut v4l2_buffer) -> i32;
    pub fn v4l2_m2m_prepare_buf(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, buf: *mut v4l2_buffer) -> i32;
    pub fn v4l2_m2m_create_bufs(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, create: *mut v4l2_create_buffers) -> i32;
    pub fn v4l2_m2m_expbuf(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, eb: *mut v4l2_exportbuffer) -> i32;
    pub fn v4l2_m2m_streamon(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, type_: v4l2_buf_type) -> i32;
    pub fn v4l2_m2m_streamoff(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, type_: v4l2_buf_type) -> i32;
    pub fn v4l2_m2m_update_start_streaming_state(m2m_ctx: *mut v4l2_m2m_ctx, q: *mut vb2_queue);
    pub fn v4l2_m2m_update_stop_streaming_state(m2m_ctx: *mut v4l2_m2m_ctx, q: *mut vb2_queue);
    pub fn v4l2_m2m_encoder_cmd(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, ec: *mut v4l2_encoder_cmd) -> i32;
    pub fn v4l2_m2m_decoder_cmd(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, dc: *mut v4l2_decoder_cmd) -> i32;
    pub fn v4l2_m2m_poll(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, wait: *mut poll_table_struct) -> __poll_t;
    pub fn v4l2_m2m_mmap(file: *mut file, m2m_ctx: *mut v4l2_m2m_ctx, vma: *mut vm_area_struct) -> i32;
    /* CONFIG_MMU=n */
    pub fn v4l2_m2m_get_unmapped_area(file: *mut file, addr: core::ffi::c_ulong, len: core::ffi::c_ulong, pgoff: core::ffi::c_ulong, flags: core::ffi::c_ulong) -> core::ffi::c_ulong;
    pub fn v4l2_m2m_init(m2m_ops: *const v4l2_m2m_ops) -> *mut v4l2_m2m_dev;
    /* CONFIG_MEDIA_CONTROLLER */
    pub fn v4l2_m2m_unregister_media_controller(m2m_dev: *mut v4l2_m2m_dev);
    pub fn v4l2_m2m_register_media_controller(m2m_dev: *mut v4l2_m2m_dev, vdev: *mut video_device, function: i32) -> i32;
    pub fn v4l2_m2m_release(m2m_dev: *mut v4l2_m2m_dev);
    pub fn v4l2_m2m_get(m2m_dev: *mut v4l2_m2m_dev);
    pub fn v4l2_m2m_put(m2m_dev: *mut v4l2_m2m_dev);
    pub fn v4l2_m2m_ctx_init(m2m_dev: *mut v4l2_m2m_dev, drv_priv: *mut core::ffi::c_void, queue_init: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut vb2_queue, *mut vb2_queue) -> i32>) -> *mut v4l2_m2m_ctx;
    pub fn v4l2_m2m_ctx_release(m2m_ctx: *mut v4l2_m2m_ctx);
    pub fn v4l2_m2m_buf_queue(m2m_ctx: *mut v4l2_m2m_ctx, vbuf: *mut vb2_v4l2_buffer);
    pub fn v4l2_m2m_next_buf(q_ctx: *mut v4l2_m2m_queue_ctx) -> *mut vb2_v4l2_buffer;
    pub fn v4l2_m2m_last_buf(q_ctx: *mut v4l2_m2m_queue_ctx) -> *mut vb2_v4l2_buffer;
    pub fn v4l2_m2m_buf_remove(q_ctx: *mut v4l2_m2m_queue_ctx) -> *mut vb2_v4l2_buffer;
    pub fn v4l2_m2m_buf_remove_by_buf(q_ctx: *mut v4l2_m2m_queue_ctx, vbuf: *mut vb2_v4l2_buffer);
    pub fn v4l2_m2m_buf_remove_by_idx(q_ctx: *mut v4l2_m2m_queue_ctx, idx: core::ffi::c_uint) -> *mut vb2_v4l2_buffer;
    pub fn v4l2_m2m_buf_copy_metadata(out_vb: *const vb2_v4l2_buffer, cap_vb: *mut vb2_v4l2_buffer);
    pub fn v4l2_m2m_request_queue(req: *mut media_request);
    pub fn v4l2_m2m_ioctl_reqbufs(file: *mut file, priv_: *mut core::ffi::c_void, rb: *mut v4l2_requestbuffers) -> i32;
    pub fn v4l2_m2m_ioctl_create_bufs(file: *mut file, priv_: *mut core::ffi::c_void, create: *mut v4l2_create_buffers) -> i32;
    pub fn v4l2_m2m_ioctl_remove_bufs(file: *mut file, priv_: *mut core::ffi::c_void, d: *mut v4l2_remove_buffers) -> i32;
    pub fn v4l2_m2m_ioctl_querybuf(file: *mut file, priv_: *mut core::ffi::c_void, buf: *mut v4l2_buffer) -> i32;
    pub fn v4l2_m2m_ioctl_expbuf(file: *mut file, priv_: *mut core::ffi::c_void, eb: *mut v4l2_exportbuffer) -> i32;
    pub fn v4l2_m2m_ioctl_qbuf(file: *mut file, priv_: *mut core::ffi::c_void, buf: *mut v4l2_buffer) -> i32;
    pub fn v4l2_m2m_ioctl_dqbuf(file: *mut file, priv_: *mut core::ffi::c_void, buf: *mut v4l2_buffer) -> i32;
    pub fn v4l2_m2m_ioctl_prepare_buf(file: *mut file, priv_: *mut core::ffi::c_void, buf: *mut v4l2_buffer) -> i32;
    pub fn v4l2_m2m_ioctl_streamon(file: *mut file, priv_: *mut core::ffi::c_void, type_: v4l2_buf_type) -> i32;
    pub fn v4l2_m2m_ioctl_streamoff(file: *mut file, priv_: *mut core::ffi::c_void, type_: v4l2_buf_type) -> i32;
    pub fn v4l2_m2m_ioctl_encoder_cmd(file: *mut file, priv_: *mut core::ffi::c_void, ec: *mut v4l2_encoder_cmd) -> i32;
    pub fn v4l2_m2m_ioctl_decoder_cmd(file: *mut file, priv_: *mut core::ffi::c_void, dc: *mut v4l2_decoder_cmd) -> i32;
    pub fn v4l2_m2m_ioctl_try_encoder_cmd(file: *mut file, priv_: *mut core::ffi::c_void, ec: *mut v4l2_encoder_cmd) -> i32;
    pub fn v4l2_m2m_ioctl_try_decoder_cmd(file: *mut file, priv_: *mut core::ffi::c_void, dc: *mut v4l2_decoder_cmd) -> i32;
    pub fn v4l2_m2m_ioctl_stateless_try_decoder_cmd(file: *mut file, priv_: *mut core::ffi::c_void, dc: *mut v4l2_decoder_cmd) -> i32;
    pub fn v4l2_m2m_ioctl_stateless_decoder_cmd(file: *mut file, priv_: *mut core::ffi::c_void, dc: *mut v4l2_decoder_cmd) -> i32;
    pub fn v4l2_m2m_fop_mmap(file: *mut file, vma: *mut vm_area_struct) -> i32;
    pub fn v4l2_m2m_fop_poll(file: *mut file, wait: *mut poll_table) -> __poll_t;
}

#[inline]
pub unsafe fn v4l2_m2m_buf_done(buf: *mut vb2_v4l2_buffer, state: vb2_buffer_state) { vb2_buffer_done(&mut (*buf).vb2_buf, state); }
#[inline]
pub unsafe fn v4l2_m2m_clear_state(ctx: *mut v4l2_m2m_ctx) { (*ctx).next_buf_last = false; (*ctx).is_draining = false; (*ctx).has_stopped = false; }
#[inline]
pub unsafe fn v4l2_m2m_mark_stopped(ctx: *mut v4l2_m2m_ctx) { (*ctx).next_buf_last = false; (*ctx).is_draining = false; (*ctx).has_stopped = true; }
#[inline]
pub unsafe fn v4l2_m2m_dst_buf_is_last(ctx: *mut v4l2_m2m_ctx) -> bool { (*ctx).is_draining && (*ctx).next_buf_last }
#[inline]
pub unsafe fn v4l2_m2m_has_stopped(ctx: *mut v4l2_m2m_ctx) -> bool { (*ctx).has_stopped }
#[inline]
pub unsafe fn v4l2_m2m_is_last_draining_src_buf(ctx: *mut v4l2_m2m_ctx, vbuf: *mut vb2_v4l2_buffer) -> bool { (*ctx).is_draining && vbuf == (*ctx).last_src_buf }
#[inline]
pub unsafe fn v4l2_m2m_set_src_buffered(ctx: *mut v4l2_m2m_ctx, buffered: bool) { (*ctx).out_q_ctx.buffered = buffered; }
#[inline]
pub unsafe fn v4l2_m2m_set_dst_buffered(ctx: *mut v4l2_m2m_ctx, buffered: bool) { (*ctx).cap_q_ctx.buffered = buffered; }
#[inline]
pub unsafe fn v4l2_m2m_next_src_buf(ctx: *mut v4l2_m2m_ctx) -> *mut vb2_v4l2_buffer { v4l2_m2m_next_buf(&mut (*ctx).out_q_ctx) }
#[inline]
pub unsafe fn v4l2_m2m_next_dst_buf(ctx: *mut v4l2_m2m_ctx) -> *mut vb2_v4l2_buffer { v4l2_m2m_next_buf(&mut (*ctx).cap_q_ctx) }
#[inline]
pub unsafe fn v4l2_m2m_last_src_buf(ctx: *mut v4l2_m2m_ctx) -> *mut vb2_v4l2_buffer { v4l2_m2m_last_buf(&mut (*ctx).out_q_ctx) }
#[inline]
pub unsafe fn v4l2_m2m_last_dst_buf(ctx: *mut v4l2_m2m_ctx) -> *mut vb2_v4l2_buffer { v4l2_m2m_last_buf(&mut (*ctx).cap_q_ctx) }
#[inline]
pub unsafe fn v4l2_m2m_get_src_vq(ctx: *mut v4l2_m2m_ctx) -> *mut vb2_queue { &mut (*ctx).out_q_ctx.q }
#[inline]
pub unsafe fn v4l2_m2m_get_dst_vq(ctx: *mut v4l2_m2m_ctx) -> *mut vb2_queue { &mut (*ctx).cap_q_ctx.q }
#[inline]
pub unsafe fn v4l2_m2m_src_buf_remove(ctx: *mut v4l2_m2m_ctx) -> *mut vb2_v4l2_buffer { v4l2_m2m_buf_remove(&mut (*ctx).out_q_ctx) }
#[inline]
pub unsafe fn v4l2_m2m_dst_buf_remove(ctx: *mut v4l2_m2m_ctx) -> *mut vb2_v4l2_buffer { v4l2_m2m_buf_remove(&mut (*ctx).cap_q_ctx) }
#[inline]
pub unsafe fn v4l2_m2m_src_buf_remove_by_buf(ctx: *mut v4l2_m2m_ctx, vbuf: *mut vb2_v4l2_buffer) { v4l2_m2m_buf_remove_by_buf(&mut (*ctx).out_q_ctx, vbuf); }
#[inline]
pub unsafe fn v4l2_m2m_dst_buf_remove_by_buf(ctx: *mut v4l2_m2m_ctx, vbuf: *mut vb2_v4l2_buffer) { v4l2_m2m_buf_remove_by_buf(&mut (*ctx).cap_q_ctx, vbuf); }
#[inline]
pub unsafe fn v4l2_m2m_src_buf_remove_by_idx(ctx: *mut v4l2_m2m_ctx, idx: core::ffi::c_uint) -> *mut vb2_v4l2_buffer { v4l2_m2m_buf_remove_by_idx(&mut (*ctx).out_q_ctx, idx) }
#[inline]
pub unsafe fn v4l2_m2m_dst_buf_remove_by_idx(ctx: *mut v4l2_m2m_ctx, idx: core::ffi::c_uint) -> *mut vb2_v4l2_buffer { v4l2_m2m_buf_remove_by_idx(&mut (*ctx).cap_q_ctx, idx) }

/* C preprocessor list-iteration macros are preserved as Rust documentation. */
// v4l2_m2m_for_each_dst_buf(m2m_ctx, b): list_for_each_entry(b, &m2m_ctx->cap_q_ctx.rdy_queue, list)
// v4l2_m2m_for_each_src_buf(m2m_ctx, b): list_for_each_entry(b, &m2m_ctx->out_q_ctx.rdy_queue, list)
// v4l2_m2m_for_each_dst_buf_safe(m2m_ctx, b, n): list_for_each_entry_safe(b, n, &m2m_ctx->cap_q_ctx.rdy_queue, list)
// v4l2_m2m_for_each_src_buf_safe(m2m_ctx, b, n): list_for_each_entry_safe(b, n, &m2m_ctx->out_q_ctx.rdy_queue, list)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
