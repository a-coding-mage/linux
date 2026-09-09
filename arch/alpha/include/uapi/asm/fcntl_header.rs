/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const O_CREAT: i32 = 1 << 9; /* not fcntl */
pub const O_TRUNC: i32 = 1 << 10; /* not fcntl */
pub const O_EXCL: i32 = 1 << 11; /* not fcntl */
pub const O_NOCTTY: i32 = 1 << 12; /* not fcntl */

pub const O_NONBLOCK: i32 = 1 << 2;
pub const O_APPEND: i32 = 1 << 3;
pub const O_DSYNC: i32 = 1 << 14; /* used to be O_SYNC, see below */
pub const O_DIRECTORY: i32 = 1 << 15; /* must be a directory */
pub const O_NOFOLLOW: i32 = 1 << 16; /* don't follow links */
pub const O_LARGEFILE: i32 = 1 << 17; /* will be set by the kernel on every open */
pub const O_DIRECT: i32 = 1 << 19; /* direct disk access - should check with OSF/1 */
pub const O_NOATIME: i32 = 1 << 20;
pub const O_CLOEXEC: i32 = 1 << 21; /* set close_on_exec */
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
pub const __O_SYNC: i32 = 1 << 22;
pub const O_SYNC: i32 = __O_SYNC | O_DSYNC;

pub const O_PATH: i32 = 1 << 23;
pub const __O_TMPFILE: i32 = 1 << 24;

pub const F_GETLK: i32 = 7;
pub const F_SETLK: i32 = 8;
pub const F_SETLKW: i32 = 9;

pub const F_SETOWN: i32 = 5; /*  for sockets. */
pub const F_GETOWN: i32 = 6; /*  for sockets. */
pub const F_SETSIG: i32 = 10; /*  for sockets. */
pub const F_GETSIG: i32 = 11; /*  for sockets. */

/* for posix fcntl() and lockf() */
pub const F_RDLCK: i32 = 1;
pub const F_WRLCK: i32 = 2;
pub const F_UNLCK: i32 = 8;

/* for old implementation of bsd flock () */
pub const F_EXLCK: i32 = 16; /* or 3 */
pub const F_SHLCK: i32 = 32; /* or 4 */

// Dependency: declarations from <asm-generic/fcntl.h> are supplied by another translated header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
