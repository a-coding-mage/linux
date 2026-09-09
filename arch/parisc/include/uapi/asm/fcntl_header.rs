/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: declarations from <asm-generic/fcntl.h> are supplied by
// the corresponding translated dependency.

pub const O_APPEND: i32 = 1 << 3;
pub const O_CREAT: i32 = 1 << 8; // not fcntl
pub const O_EXCL: i32 = 1 << 10; // not fcntl
pub const O_LARGEFILE: i32 = 1 << 11;
pub const __O_SYNC: i32 = 1 << 15;
pub const O_SYNC: i32 = __O_SYNC | O_DSYNC;
pub const O_NONBLOCK: i32 = 1 << 16;
pub const O_NOCTTY: i32 = 1 << 17; // not fcntl
pub const O_DSYNC: i32 = 1 << 18;
pub const O_NOATIME: i32 = 1 << 20;
pub const O_CLOEXEC: i32 = 1 << 21; // set close_on_exec

pub const O_DIRECTORY: i32 = 1 << 12; // must be a directory
pub const O_NOFOLLOW: i32 = 1 << 7; // don't follow links

pub const O_PATH: i32 = 1 << 22;
pub const __O_TMPFILE: i32 = 1 << 23;

pub const F_GETLK64: i32 = 8;
pub const F_SETLK64: i32 = 9;
pub const F_SETLKW64: i32 = 10;

pub const F_GETOWN: i32 = 11; // for sockets.
pub const F_SETOWN: i32 = 12; // for sockets.
pub const F_SETSIG: i32 = 13; // for sockets.
pub const F_GETSIG: i32 = 14; // for sockets.

// for posix fcntl() and lockf()
pub const F_RDLCK: i32 = 0o1;
pub const F_WRLCK: i32 = 0o2;
pub const F_UNLCK: i32 = 0o3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
