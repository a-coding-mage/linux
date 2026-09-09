/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 96, 97, 98, 99, 2003, 05 Ralf Baechle
 */

// Dependency supplied by the original header: asm/sgidefs.h.

pub const O_APPEND: i32 = 1 << 3;
pub const O_DSYNC: i32 = 1 << 4; // used to be O_SYNC, see below
pub const O_NONBLOCK: i32 = 1 << 7;
pub const O_CREAT: i32 = 1 << 8; // not fcntl
pub const O_TRUNC: i32 = 1 << 9; // not fcntl
pub const O_EXCL: i32 = 1 << 10; // not fcntl
pub const O_NOCTTY: i32 = 1 << 11; // not fcntl
pub const FASYNC: i32 = 1 << 12; // fcntl, for BSD compatibility
pub const O_LARGEFILE: i32 = 1 << 13; // allow large file opens
/*
 * Before Linux 2.6.33 only O_DSYNC semantics were implemented, but using
 * the O_SYNC flag.  We continue to use the existing numerical value
 * for O_DSYNC semantics now, but using the correct symbolic name for it.
 * This new value is used to request true Posix O_SYNC semantics.  It is
 * defined in this strange way to make sure applications compiled against
 * new headers get at least O_DSYNC semantics on older kernels.
 *
 * This has the nice side-effect that we can simply test for O_DSYNC
 * wherever we do not care if O_DSYNC or O_SYNC is used.
 *
 * Note: __O_SYNC must never be used directly.
 */
pub const __O_SYNC: i32 = 1 << 14;
pub const O_SYNC: i32 = __O_SYNC | O_DSYNC;
pub const O_DIRECT: i32 = 1 << 15; // direct disk access hint

pub const F_GETLK: i32 = 14;
pub const F_SETLK: i32 = 6;
pub const F_SETLKW: i32 = 7;

pub const F_SETOWN: i32 = 24; // for sockets.
pub const F_GETOWN: i32 = 23; // for sockets.

// Original condition: __BITS_PER_LONG == 32 || defined(__KERNEL__).
// These constants are emitted here; consumers should apply the original
// configuration condition when exposing them in an ABI-specific interface.
pub const F_GETLK64: i32 = 33; // using 'struct flock64'
pub const F_SETLK64: i32 = 34;
pub const F_SETLKW64: i32 = 35;

// Original condition: _MIPS_SIM != _MIPS_SIM_ABI64.
// These macros contribute fields to a containing flock structure in C.
#[macro_export]
macro_rules! __ARCH_FLOCK_EXTRA_SYSID {
    () => { l_sysid: core::ffi::c_long, };
}

#[macro_export]
macro_rules! __ARCH_FLOCK_PAD {
    () => { pad: [core::ffi::c_long; 4], };
}

// Dependency supplied by the original header: asm-generic/fcntl.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
