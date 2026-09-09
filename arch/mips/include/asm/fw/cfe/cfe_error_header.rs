/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2000, 2001, 2002 Broadcom Corporation
 */

/*
 * Broadcom Common Firmware Environment (CFE)
 *
 * CFE's global error code list is here.
 *
 * Author:  Mitch Lichtenberg
 */

pub const CFE_OK: i32 = 0;
pub const CFE_ERR: i32 = -1; /* generic error */
pub const CFE_ERR_INV_COMMAND: i32 = -2;
pub const CFE_ERR_EOF: i32 = -3;
pub const CFE_ERR_IOERR: i32 = -4;
pub const CFE_ERR_NOMEM: i32 = -5;
pub const CFE_ERR_DEVNOTFOUND: i32 = -6;
pub const CFE_ERR_DEVOPEN: i32 = -7;
pub const CFE_ERR_INV_PARAM: i32 = -8;
pub const CFE_ERR_ENVNOTFOUND: i32 = -9;
pub const CFE_ERR_ENVREADONLY: i32 = -10;

pub const CFE_ERR_NOTELF: i32 = -11;
pub const CFE_ERR_NOT32BIT: i32 = -12;
pub const CFE_ERR_WRONGENDIAN: i32 = -13;
pub const CFE_ERR_BADELFVERS: i32 = -14;
pub const CFE_ERR_NOTMIPS: i32 = -15;
pub const CFE_ERR_BADELFFMT: i32 = -16;
pub const CFE_ERR_BADADDR: i32 = -17;

pub const CFE_ERR_FILENOTFOUND: i32 = -18;
pub const CFE_ERR_UNSUPPORTED: i32 = -19;

pub const CFE_ERR_HOSTUNKNOWN: i32 = -20;

pub const CFE_ERR_TIMEOUT: i32 = -21;

pub const CFE_ERR_PROTOCOLERR: i32 = -22;

pub const CFE_ERR_NETDOWN: i32 = -23;
pub const CFE_ERR_NONAMESERVER: i32 = -24;

pub const CFE_ERR_NOHANDLES: i32 = -25;
pub const CFE_ERR_ALREADYBOUND: i32 = -26;

pub const CFE_ERR_CANNOTSET: i32 = -27;
pub const CFE_ERR_NOMORE: i32 = -28;
pub const CFE_ERR_BADFILESYS: i32 = -29;
pub const CFE_ERR_FSNOTAVAIL: i32 = -30;

pub const CFE_ERR_INVBOOTBLOCK: i32 = -31;
pub const CFE_ERR_WRONGDEVTYPE: i32 = -32;
pub const CFE_ERR_BBCHECKSUM: i32 = -33;
pub const CFE_ERR_BOOTPROGCHKSUM: i32 = -34;

pub const CFE_ERR_LDRNOTAVAIL: i32 = -35;

pub const CFE_ERR_NOTREADY: i32 = -36;

pub const CFE_ERR_GETMEM: i32 = -37;
pub const CFE_ERR_SETMEM: i32 = -38;

pub const CFE_ERR_NOTCONN: i32 = -39;
pub const CFE_ERR_ADDRINUSE: i32 = -40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
