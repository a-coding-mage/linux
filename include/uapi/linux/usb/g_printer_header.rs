/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * g_printer.h -- Header file for USB Printer gadget driver
 *
 * Copyright (C) 2007 Craig W. Nadler
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

pub const PRINTER_NOT_ERROR: u8 = 0x08;
pub const PRINTER_SELECTED: u8 = 0x10;
pub const PRINTER_PAPER_EMPTY: u8 = 0x20;

/* The 'g' code is also used by gadgetfs ioctl requests.
 * Don't add any colliding codes to either driver, and keep
 * them in unique ranges (size 0x20 for now).
 *
 * These are the Linux _IOR/_IOWR encodings for unsigned char, with
 * the usual Linux ioctl bit layout.
 */
pub const GADGET_GET_PRINTER_STATUS: u32 = 0x8001_6721;
pub const GADGET_SET_PRINTER_STATUS: u32 = 0xc001_6722;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
