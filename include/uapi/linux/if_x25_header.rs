/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  Linux X.25 packet to device interface
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU General Public License for more details.
 */

// Dependency intent from C: #include <linux/types.h>

/* Documentation/networking/x25-iface.rst */
pub const X25_IFACE_DATA: u32 = 0x00;
pub const X25_IFACE_CONNECT: u32 = 0x01;
pub const X25_IFACE_DISCONNECT: u32 = 0x02;
pub const X25_IFACE_PARAMS: u32 = 0x03;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
