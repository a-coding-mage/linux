/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Sony Programmable I/O Control Device driver for VAIO
 *
 * Copyright (C) 2001-2005 Stelian Pop <stelian@popies.net>
 *
 * Copyright (C) 2005 Narayanan R S <nars@kadamba.org>

 * Copyright (C) 2001-2002 Alcôve <www.alcove.com>
 *
 * Copyright (C) 2001 Michael Ashley <m.ashley@unsw.edu.au>
 *
 * Copyright (C) 2001 Junichi Morita <jun1m@mars.dti.ne.jp>
 *
 * Copyright (C) 2000 Takaya Kinjo <t-kinjo@tc4.so-net.ne.jp>
 *
 * Copyright (C) 2000 Andrew Tridgell <tridge@valinux.com>
 *
 * Earlier work by Werner Almesberger, Paul `Rusty' Russell and Paul Mackerras.
 */

// Dependency supplied by the corresponding uapi/linux/sonypi.h header.

/* used only for communication between v4l and sonypi */

pub const SONYPI_COMMAND_GETCAMERA: i32 = 1; // obsolete
pub const SONYPI_COMMAND_SETCAMERA: i32 = 2;
pub const SONYPI_COMMAND_GETCAMERABRIGHTNESS: i32 = 3; // obsolete
pub const SONYPI_COMMAND_SETCAMERABRIGHTNESS: i32 = 4;
pub const SONYPI_COMMAND_GETCAMERACONTRAST: i32 = 5; // obsolete
pub const SONYPI_COMMAND_SETCAMERACONTRAST: i32 = 6;
pub const SONYPI_COMMAND_GETCAMERAHUE: i32 = 7; // obsolete
pub const SONYPI_COMMAND_SETCAMERAHUE: i32 = 8;
pub const SONYPI_COMMAND_GETCAMERACOLOR: i32 = 9; // obsolete
pub const SONYPI_COMMAND_SETCAMERACOLOR: i32 = 10;
pub const SONYPI_COMMAND_GETCAMERASHARPNESS: i32 = 11; // obsolete
pub const SONYPI_COMMAND_SETCAMERASHARPNESS: i32 = 12;
pub const SONYPI_COMMAND_GETCAMERAPICTURE: i32 = 13; // obsolete
pub const SONYPI_COMMAND_SETCAMERAPICTURE: i32 = 14;
pub const SONYPI_COMMAND_GETCAMERAAGC: i32 = 15; // obsolete
pub const SONYPI_COMMAND_SETCAMERAAGC: i32 = 16;
pub const SONYPI_COMMAND_GETCAMERADIRECTION: i32 = 17; // obsolete
pub const SONYPI_COMMAND_GETCAMERAROMVERSION: i32 = 18; // obsolete
pub const SONYPI_COMMAND_GETCAMERAREVISION: i32 = 19; // obsolete

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
