/*
 * SPDX-License-Identifier: GPL-2.0
 *
 * dvb-vb2.h - DVB driver helper framework for streaming I/O
 *
 * Copyright (C) 2015 Samsung Electronics
 *
 * Author: jh1009.sung@samsung.com
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

// C dependencies supplied by other translation units/headers:
// linux/mutex.h, linux/poll.h, linux/dvb/dmx.h,
// media/videobuf2-core.h, media/videobuf2-dma-contig.h,
// media/videobuf2-vmalloc.h

/**
 * enum dvb_buf_type - types of Digital TV memory-mapped buffers
 *
 * @DVB_BUF_TYPE_CAPTURE: buffer is filled by the Kernel,
 *                        with a received Digital TV stream
 */
#[repr(C)]
pub enum dvb_buf_type {
    DVB_BUF_TYPE_CAPTURE = 1,
}

/**
 * enum dvb_vb2_states - states to control VB2 state machine
 * @DVB_VB2_STATE_NONE:
 *     VB2 engine not initialized yet, init failed or VB2 was released.
 * @DVB_VB2_STATE_INIT:
 *     VB2 engine initialized.
 * @DVB_VB2_STATE_REQBUFS:
 *     Buffers were requested
 * @DVB_VB2_STATE_STREAMON:
 *     VB2 is streaming. Callers should not check it directly. Instead,
 *     they should use dvb_vb2_is_streaming().
 *
 * Note:
 *
 * Callers should not touch at the state machine directly. This
 * is handled inside dvb_vb2.c.
 */
#[repr(C)]
pub enum dvb_vb2_states {
    DVB_VB2_STATE_NONE = 0x0,
    DVB_VB2_STATE_INIT = 0x1,
    DVB_VB2_STATE_REQBUFS = 0x2,
    DVB_VB2_STATE_STREAMON = 0x4,
}

pub const DVB_VB2_NAME_MAX: usize = 20;

/**
 * struct dvb_buffer - video buffer information for v4l2.
 *
 * @vb:     embedded struct &vb2_buffer.
 * @list:   list of &struct dvb_buffer.
 */
#[repr(C)]
pub struct dvb_buffer {
    pub vb: vb2_buffer,
    pub list: list_head,
}

/**
 * struct dvb_vb2_ctx - control struct for VB2 handler
 * @vb_q: pointer to &struct vb2_queue with videobuf2 queue.
 * @slock: spin lock used to protect buffer filling at dvb_vb2.c.
 * @dvb_q: List of buffers that are not filled yet.
 * @buf: Pointer to the buffer that are currently being filled.
 * @offset: index to the next position at the @buf to be filled.
 * @remain: How many bytes are left to be filled at @buf.
 * @state: bitmask of buffer states as defined by &enum dvb_vb2_states.
 * @buf_siz: size of each VB2 buffer.
 * @buf_cnt: number of VB2 buffers.
 * @nonblocking: If different than zero, device is operating on non-blocking mode.
 * @flags: buffer flags as defined by &enum dmx_buffer_flags.
 * @count: monotonic counter for filled buffers.
 * @name: name of the device type.
 */
#[repr(C)]
pub struct dvb_vb2_ctx {
    pub vb_q: vb2_queue,
    pub slock: spinlock_t,
    pub dvb_q: list_head,
    pub buf: *mut dvb_buffer,
    pub offset: i32,
    pub remain: i32,
    pub state: i32,
    pub buf_siz: i32,
    pub buf_cnt: i32,
    pub nonblocking: i32,
    pub flags: dmx_buffer_flags,
    pub count: u32,
    pub name: [::std::os::raw::c_char; DVB_VB2_NAME_MAX + 1],
}

// When CONFIG_DVB_MMAP is disabled, these inline stubs and macros return zero.
#[cfg(not(CONFIG_DVB_MMAP))]
pub unsafe fn dvb_vb2_init(
    _ctx: *mut dvb_vb2_ctx,
    _name: *const ::std::os::raw::c_char,
    _mutex: *mut mutex,
    _non_blocking: i32,
) -> i32 { 0 }

#[cfg(not(CONFIG_DVB_MMAP))]
pub unsafe fn dvb_vb2_release(_ctx: *mut dvb_vb2_ctx) -> i32 { 0 }

#[cfg(not(CONFIG_DVB_MMAP))]
pub unsafe fn dvb_vb2_is_streaming(_ctx: *mut dvb_vb2_ctx) -> i32 { 0 }

#[cfg(not(CONFIG_DVB_MMAP))]
pub unsafe fn dvb_vb2_fill_buffer(
    _ctx: *mut dvb_vb2_ctx,
    _file: *mut file,
    _wait: *mut poll_table,
    _flags: u32,
    _flush: bool,
) -> i32 { 0 }

#[cfg(not(CONFIG_DVB_MMAP))]
pub unsafe fn dvb_vb2_poll(
    _ctx: *mut dvb_vb2_ctx,
    _file: *mut file,
    _wait: *mut poll_table,
) -> __poll_t { 0 }

// When CONFIG_DVB_MMAP is enabled, the following declarations are external.
#[cfg(CONFIG_DVB_MMAP)]
extern "C" {
    pub fn dvb_vb2_init(ctx: *mut dvb_vb2_ctx, name: *const ::std::os::raw::c_char,
                        mutex: *mut mutex, non_blocking: i32) -> i32;
    pub fn dvb_vb2_release(ctx: *mut dvb_vb2_ctx) -> i32;
    pub fn dvb_vb2_is_streaming(ctx: *mut dvb_vb2_ctx) -> i32;
    pub fn dvb_vb2_fill_buffer(ctx: *mut dvb_vb2_ctx, src: *const u8, len: i32,
                               buffer_flags: *mut dmx_buffer_flags, flush: bool) -> i32;
    pub fn dvb_vb2_poll(ctx: *mut dvb_vb2_ctx, file: *mut file,
                        wait: *mut poll_table) -> __poll_t;
}

extern "C" {
    pub fn dvb_vb2_stream_on(ctx: *mut dvb_vb2_ctx) -> i32;
    pub fn dvb_vb2_stream_off(ctx: *mut dvb_vb2_ctx) -> i32;
    pub fn dvb_vb2_reqbufs(ctx: *mut dvb_vb2_ctx, req: *mut dmx_requestbuffers) -> i32;
    pub fn dvb_vb2_querybuf(ctx: *mut dvb_vb2_ctx, b: *mut dmx_buffer) -> i32;
    pub fn dvb_vb2_expbuf(ctx: *mut dvb_vb2_ctx, exp: *mut dmx_exportbuffer) -> i32;
    pub fn dvb_vb2_qbuf(ctx: *mut dvb_vb2_ctx, b: *mut dmx_buffer) -> i32;
    pub fn dvb_vb2_dqbuf(ctx: *mut dvb_vb2_ctx, b: *mut dmx_buffer) -> i32;
    pub fn dvb_vb2_mmap(ctx: *mut dvb_vb2_ctx, vma: *mut vm_area_struct) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
