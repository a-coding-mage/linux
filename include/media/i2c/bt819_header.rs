/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    bt819.h - bt819 notifications

    Copyright (C) 2009 Hans Verkuil (hverkuil@kernel.org)

*/

/* C header guard: _BT819_H_ */

/* Dependency: <linux/ioctl.h> supplies the _IO macro. */

/* v4l2_device notifications. */

/* Needed to reset the FIFO buffer when changing the input
   or the video standard.

   Note: these ioctls that internal to the kernel and are never called
   from userspace. */
pub const BT819_FIFO_RESET_LOW: ::std::os::raw::c_ulong =
    _IO(b'b' as ::std::os::raw::c_ulong, 0);
pub const BT819_FIFO_RESET_HIGH: ::std::os::raw::c_ulong =
    _IO(b'b' as ::std::os::raw::c_ulong, 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
