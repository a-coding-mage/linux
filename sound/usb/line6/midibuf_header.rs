// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

pub const LINE6_MIDIBUF_READ_TX: i32 = 0;
pub const LINE6_MIDIBUF_READ_RX: i32 = 1;

#[repr(C)]
pub struct midi_buffer {
    pub buf: *mut u8,
    pub size: i32,
    pub split: i32,
    pub pos_read: i32,
    pub pos_write: i32,
    pub full: i32,
    pub command_prev: i32,
}

extern "C" {
    pub fn line6_midibuf_bytes_used(mb: *mut midi_buffer) -> i32;
    pub fn line6_midibuf_bytes_free(mb: *mut midi_buffer) -> i32;
    pub fn line6_midibuf_destroy(mb: *mut midi_buffer);
    pub fn line6_midibuf_ignore(mb: *mut midi_buffer, length: i32) -> i32;
    pub fn line6_midibuf_init(mb: *mut midi_buffer, size: i32, split: i32) -> i32;
    pub fn line6_midibuf_read(
        mb: *mut midi_buffer,
        data: *mut u8,
        length: i32,
        read_type: i32,
    ) -> i32;
    pub fn line6_midibuf_reset(mb: *mut midi_buffer);
    pub fn line6_midibuf_write(mb: *mut midi_buffer, data: *mut u8, length: i32) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
