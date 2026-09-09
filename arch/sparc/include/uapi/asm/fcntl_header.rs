/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translation of the SPARC fcntl header.

pub const O_APPEND: i32 = 1 << 3;
pub const FASYNC: i32 = 1 << 6; // fcntl, for BSD compatibility
pub const O_CREAT: i32 = 1 << 9; // not fcntl
pub const O_TRUNC: i32 = 1 << 10; // not fcntl
pub const O_EXCL: i32 = 1 << 11; // not fcntl
pub const O_DSYNC: i32 = 1 << 13; // used to be O_SYNC, see below
pub const O_NONBLOCK: i32 = 1 << 14;

#[cfg(target_arch = "sparc64")]
pub const O_NDELAY: i32 = 1 << 2;
#[cfg(not(target_arch = "sparc64"))]
pub const O_NDELAY: i32 = (1 << 2) | O_NONBLOCK;

pub const O_NOCTTY: i32 = 1 << 15; // not fcntl
pub const O_LARGEFILE: i32 = 1 << 18;
pub const O_DIRECT: i32 = 1 << 20; // direct disk access hint
pub const O_NOATIME: i32 = 1 << 21;
pub const O_CLOEXEC: i32 = 1 << 22;

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
pub const __O_SYNC: i32 = 1 << 23;
pub const O_SYNC: i32 = __O_SYNC | O_DSYNC;

pub const O_PATH: i32 = 1 << 24;
pub const __O_TMPFILE: i32 = 1 << 25;

pub const F_GETOWN: i32 = 5; // for sockets.
pub const F_SETOWN: i32 = 6; // for sockets.
pub const F_GETLK: i32 = 7;
pub const F_SETLK: i32 = 8;
pub const F_SETLKW: i32 = 9;

// For POSIX fcntl() and lockf().
pub const F_RDLCK: i32 = 1;
pub const F_WRLCK: i32 = 2;
pub const F_UNLCK: i32 = 3;

// C macros defining fields in architecture-specific flock structures.
// __ARCH_FLOCK_PAD: short __unused;
// __ARCH_FLOCK64_PAD: short __unused;

// Dependency supplied by the asm-generic/fcntl.h translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
