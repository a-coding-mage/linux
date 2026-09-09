/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* The industrial I/O - event passing to userspace
 *
 * Copyright (c) 2008-2011 Jonathan Cameron
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

/* Dependency intent: linux/ioctl.h and linux/types.h are supplied externally. */

/**
 * struct iio_event_data - The actual event being pushed to userspace
 * @id:          event identifier
 * @timestamp:   best estimate of time of event occurrence (often from
 *              the interrupt handler)
 */
#[repr(C)]
pub struct iio_event_data {
    pub id: u64,
    pub timestamp: i64,
}

/* Equivalent of _IOR('i', 0x90, int); ioctl encoding is supplied externally. */
pub const IIO_GET_EVENT_FD_IOCTL: u32 = _IOR(b'i' as u32, 0x90, core::mem::size_of::<i32>() as u32);

#[inline]
pub const fn IIO_EVENT_CODE_EXTRACT_TYPE(mask: u64) -> u64 {
    (mask >> 56) & 0xFF
}

#[inline]
pub const fn IIO_EVENT_CODE_EXTRACT_DIR(mask: u64) -> u64 {
    (mask >> 48) & 0x7F
}

#[inline]
pub const fn IIO_EVENT_CODE_EXTRACT_CHAN_TYPE(mask: u64) -> u64 {
    (mask >> 32) & 0xFF
}

/* Event code number extraction depends on which type of event we have.
 * Perhaps review this function in the future */
#[inline]
pub const fn IIO_EVENT_CODE_EXTRACT_CHAN(mask: u64) -> i16 {
    (mask & 0xFFFF) as i16
}

#[inline]
pub const fn IIO_EVENT_CODE_EXTRACT_CHAN2(mask: u64) -> i16 {
    (((mask >> 16) & 0xFFFF) as i16)
}

#[inline]
pub const fn IIO_EVENT_CODE_EXTRACT_MODIFIER(mask: u64) -> u64 {
    (mask >> 40) & 0xFF
}

#[inline]
pub const fn IIO_EVENT_CODE_EXTRACT_DIFF(mask: u64) -> u64 {
    (mask >> 55) & 0x1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
