/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * The ioctl encoding helpers `_IO` and `_IOR` are supplied by the
 * surrounding Linux UAPI bindings.
 */

pub const FBIO_WAITEVENT: core::ffi::c_ulong = _IO(b'F', 0x88);
pub const FBIO_GETCONTROL2: core::ffi::c_ulong = _IOR(b'F', 0x89, usize);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
