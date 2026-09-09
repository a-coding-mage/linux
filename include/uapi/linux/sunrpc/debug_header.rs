/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * linux/include/linux/sunrpc/debug.h
 *
 * Debugging support for sunrpc module
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

/*
 * RPC debug facilities
 */
pub const RPCDBG_XPRT: u32 = 0x0001;
pub const RPCDBG_CALL: u32 = 0x0002;
pub const RPCDBG_DEBUG: u32 = 0x0004;
pub const RPCDBG_NFS: u32 = 0x0008;
pub const RPCDBG_AUTH: u32 = 0x0010;
pub const RPCDBG_BIND: u32 = 0x0020;
pub const RPCDBG_SCHED: u32 = 0x0040;
pub const RPCDBG_TRANS: u32 = 0x0080;
pub const RPCDBG_SVCXPRT: u32 = 0x0100;
pub const RPCDBG_SVCDSP: u32 = 0x0200;
pub const RPCDBG_MISC: u32 = 0x0400;
pub const RPCDBG_CACHE: u32 = 0x0800;
pub const RPCDBG_ALL: u32 = 0x7fff;

/*
 * Declarations for the sysctl debug interface, which allows to read or
 * change the debug flags for rpc, nfs, nfsd, and lockd. Since the sunrpc
 * module currently registers its sysctl table dynamically, the sysctl path
 * for module FOO is <CTL_SUNRPC, CTL_FOODEBUG>.
 */
pub const CTL_RPCDEBUG: i32 = 1;
pub const CTL_NFSDEBUG: i32 = 2;
pub const CTL_NFSDDEBUG: i32 = 3;
pub const CTL_NLMDEBUG: i32 = 4;
pub const CTL_SLOTTABLE_UDP: i32 = 5;
pub const CTL_SLOTTABLE_TCP: i32 = 6;
pub const CTL_MIN_RESVPORT: i32 = 7;
pub const CTL_MAX_RESVPORT: i32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
