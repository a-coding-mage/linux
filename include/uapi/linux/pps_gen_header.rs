/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * PPS generator API header
 *
 * Copyright (C) 2024 Rodolfo Giometti <giometti@enneenne.com>
 */

/**
 * struct pps_gen_event - the PPS generator events
 * @event: the event type
 * @sequence: the event sequence number
 *
 * Userspace can get the last PPS generator event by using the
 * ioctl(pps_gen, PPS_GEN_FETCHEVENT, ...) syscall.
 * The sequence field can be used to save the last event ID, while in the
 * event field is stored the last event type. Currently known event is:
 *
 *     PPS_GEN_EVENT_MISSEDPULSE : last pulse was not generated
 */
#[repr(C)]
pub struct pps_gen_event {
    pub event: core::ffi::c_uint,
    pub sequence: core::ffi::c_uint,
}

pub const PPS_GEN_EVENT_MISSEDPULSE: core::ffi::c_uint = 1;

/* These ioctl values depend on the platform's Linux ioctl encoding and the
 * externally supplied _IOW/_IOR definitions. */
pub const PPS_GEN_SETENABLE: u32 = _IOW(b'p', 0xb1, core::mem::size_of::<*mut core::ffi::c_uint>());
pub const PPS_GEN_USESYSTEMCLOCK: u32 = _IOR(b'p', 0xb2, core::mem::size_of::<*mut core::ffi::c_uint>());
pub const PPS_GEN_FETCHEVENT: u32 = _IOR(b'p', 0xb3, core::mem::size_of::<*mut pps_gen_event>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
