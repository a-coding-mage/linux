/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* These are specified by iBCS2 */
pub const POLLIN: i32 = 0x0001;
pub const POLLPRI: i32 = 0x0002;
pub const POLLOUT: i32 = 0x0004;
pub const POLLERR: i32 = 0x0008;
pub const POLLHUP: i32 = 0x0010;
pub const POLLNVAL: i32 = 0x0020;

/* The rest seem to be more-or-less nonstandard. Check them! */
pub const POLLRDNORM: i32 = 0x0040;
pub const POLLRDBAND: i32 = 0x0080;
/* C header condition: define only when POLLWRNORM is not already defined. */
pub const POLLWRNORM: i32 = 0x0100;
/* C header condition: define only when POLLWRBAND is not already defined. */
pub const POLLWRBAND: i32 = 0x0200;
/* C header condition: define only when POLLMSG is not already defined. */
pub const POLLMSG: i32 = 0x0400;
/* C header condition: define only when POLLREMOVE is not already defined. */
pub const POLLREMOVE: i32 = 0x1000;
/* C header condition: define only when POLLRDHUP is not already defined. */
pub const POLLRDHUP: i32 = 0x2000;

pub const POLLFREE: __poll_t = 0x4000 as __poll_t;

pub const POLL_BUSY_LOOP: __poll_t = 0x8000 as __poll_t;

#[repr(C)]
pub struct pollfd {
    pub fd: core::ffi::c_int,
    pub events: i16,
    pub revents: i16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
