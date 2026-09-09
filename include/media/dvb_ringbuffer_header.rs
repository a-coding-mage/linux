/*
 * dvb_ringbuffer.h: ring buffer implementation for the dvb driver
 *
 * Copyright (C) 2003 Oliver Endriss
 * Copyright (C) 2004 Andrew de Quincey
 *
 * based on code originally found in av7110.c & dvb_ci.c:
 * Copyright (C) 1999-2003 Ralph Metzler & Marcus Metzler
 *                         for convergence integrated media GmbH
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1
 * of the License, or (at your option) any later version.
 */

// <linux/spinlock.h> and <linux/wait.h> supply these types in the C build.
#[allow(non_camel_case_types)]
pub enum wait_queue_head_t {}
#[allow(non_camel_case_types)]
pub enum spinlock_t {}

#[repr(C)]
pub struct dvb_ringbuffer {
    pub data: *mut u8,
    pub size: isize,
    pub pread: isize,
    pub pwrite: isize,
    pub error: i32,
    pub queue: wait_queue_head_t,
    pub lock: spinlock_t,
}

pub const DVB_RINGBUFFER_PKTHDRSIZE: usize = 3;

unsafe extern "C" {
    pub fn dvb_ringbuffer_init(rbuf: *mut dvb_ringbuffer, data: *mut core::ffi::c_void, len: usize);
    pub fn dvb_ringbuffer_empty(rbuf: *mut dvb_ringbuffer) -> i32;
    pub fn dvb_ringbuffer_free(rbuf: *mut dvb_ringbuffer) -> isize;
    pub fn dvb_ringbuffer_avail(rbuf: *mut dvb_ringbuffer) -> isize;
    pub fn dvb_ringbuffer_reset(rbuf: *mut dvb_ringbuffer);
    pub fn dvb_ringbuffer_flush(rbuf: *mut dvb_ringbuffer);
    pub fn dvb_ringbuffer_flush_spinlock_wakeup(rbuf: *mut dvb_ringbuffer);
    pub fn dvb_ringbuffer_read_user(rbuf: *mut dvb_ringbuffer, buf: *mut u8, len: usize) -> isize;
    pub fn dvb_ringbuffer_read(rbuf: *mut dvb_ringbuffer, buf: *mut u8, len: usize);
    pub fn dvb_ringbuffer_write(rbuf: *mut dvb_ringbuffer, buf: *const u8, len: usize) -> isize;
    pub fn dvb_ringbuffer_write_user(rbuf: *mut dvb_ringbuffer, buf: *const u8, len: usize) -> isize;
    pub fn dvb_ringbuffer_pkt_write(rbuf: *mut dvb_ringbuffer, buf: *mut u8, len: usize) -> isize;
    pub fn dvb_ringbuffer_pkt_read_user(rbuf: *mut dvb_ringbuffer, idx: usize, offset: i32, buf: *mut u8, len: usize) -> isize;
    pub fn dvb_ringbuffer_pkt_read(rbuf: *mut dvb_ringbuffer, idx: usize, offset: i32, buf: *mut u8, len: usize) -> isize;
    pub fn dvb_ringbuffer_pkt_dispose(rbuf: *mut dvb_ringbuffer, idx: usize);
    pub fn dvb_ringbuffer_pkt_next(rbuf: *mut dvb_ringbuffer, idx: usize, pktlen: *mut usize) -> isize;
}

#[inline]
pub unsafe fn DVB_RINGBUFFER_PEEK(rbuf: *mut dvb_ringbuffer, offs: isize) -> u8 {
    (*rbuf).data[((*rbuf).pread + offs) as usize % (*rbuf).size as usize]
}

#[inline]
pub unsafe fn DVB_RINGBUFFER_SKIP(rbuf: *mut dvb_ringbuffer, num: isize) {
    (*rbuf).pread = ((*rbuf).pread + num) % (*rbuf).size;
}

#[inline]
pub unsafe fn DVB_RINGBUFFER_WRITE_BYTE(rbuf: *mut dvb_ringbuffer, byte: u8) {
    (*rbuf).data[(*rbuf).pwrite as usize] = byte;
    (*rbuf).pwrite = ((*rbuf).pwrite + 1) % (*rbuf).size;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
