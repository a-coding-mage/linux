/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * linux/include/linux/nfsd/debug.h
 *
 * Debugging-related stuff for nfsd
 *
 * Copyright (C) 1995 Olaf Kirch <okir@monad.swb.de>
 */

// Dependency supplied by the Linux SunRPC debug header.

/*
 * knfsd debug flags
 */
pub const NFSDDBG_SOCK: u32 = 0x0001;
pub const NFSDDBG_FH: u32 = 0x0002;
pub const NFSDDBG_EXPORT: u32 = 0x0004;
pub const NFSDDBG_SVC: u32 = 0x0008;
pub const NFSDDBG_PROC: u32 = 0x0010;
pub const NFSDDBG_FILEOP: u32 = 0x0020;
pub const NFSDDBG_AUTH: u32 = 0x0040;
pub const NFSDDBG_REPCACHE: u32 = 0x0080;
pub const NFSDDBG_XDR: u32 = 0x0100;
pub const NFSDDBG_LOCKD: u32 = 0x0200;
pub const NFSDDBG_PNFS: u32 = 0x0400;
pub const NFSDDBG_ALL: u32 = 0x7FFF;
pub const NFSDDBG_NOCHANGE: u32 = 0xFFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
