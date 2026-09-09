/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the generic Linux fcntl header.
// The original include and header guard are intentionally omitted.

/*
 * FMODE_EXEC is 0x20
 * These cannot be used by userspace O_* until internal and external open
 * flags are split.
 * -Eric Paris
 */

/* When introducing new O_* bits, please check its uniqueness in fcntl_init(). */

pub const O_ACCMODE: u32 = 3;
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1 << 0;
pub const O_RDWR: u32 = 1 << 1;
/* (1 << 2) must not be used -- it collides with flags on alpha, sparc */
/* (1 << 3) must not be used -- it collides with flags on alpha, mips, parisc, sparc */
/* (1 << 4) must not be used -- it collides with flags on mips */
/* (1 << 5) is free */

// Original #ifndef guards preserve compatibility with pre-existing definitions.
pub const O_CREAT: u32 = 1 << 6; // not fcntl
pub const O_EXCL: u32 = 1 << 7; // not fcntl
pub const O_NOCTTY: u32 = 1 << 8; // not fcntl
pub const O_TRUNC: u32 = 1 << 9; // not fcntl
pub const O_APPEND: u32 = 1 << 10;
pub const O_NONBLOCK: u32 = 1 << 11;
pub const O_DSYNC: u32 = 1 << 12; // used to be O_SYNC, see below
pub const FASYNC: u32 = 1 << 13; // fcntl, for BSD compatibility
pub const O_DIRECT: u32 = 1 << 14; // direct disk access hint
pub const O_LARGEFILE: u32 = 1 << 15;
pub const O_DIRECTORY: u32 = 1 << 16; // must be a directory
pub const O_NOFOLLOW: u32 = 1 << 17; // don't follow links
pub const O_NOATIME: u32 = 1 << 18;
pub const O_CLOEXEC: u32 = 1 << 19; // set close_on_exec

/*
 * Before Linux 2.6.33 only O_DSYNC semantics were implemented, but using
 * the O_SYNC flag. We continue to use the existing numerical value for
 * O_DSYNC semantics now, but using the correct symbolic name for it. This
 * new value is used to request true Posix O_SYNC semantics. It is defined
 * in this strange way to make sure applications compiled against new
 * headers get at least O_DSYNC semantics on older kernels.
 *
 * This has the nice side-effect that we can simply test for O_DSYNC
 * wherever we do not care if O_DSYNC or O_SYNC is used.
 *
 * Note: __O_SYNC must never be used directly.
 */
pub const __O_SYNC: u32 = 1 << 20;
pub const O_SYNC: u32 = __O_SYNC | O_DSYNC;
pub const O_PATH: u32 = 1 << 21;
pub const __O_TMPFILE: u32 = 1 << 22;
pub const O_EMPTYPATH: u32 = 1 << 26; // allow empty path

/* A horrid kludge trying to make sure that this will fail on old kernels. */
pub const O_TMPFILE: u32 = __O_TMPFILE | O_DIRECTORY;
pub const O_NDELAY: u32 = O_NONBLOCK;

/* (1 << 23) must not be used -- it collides with flags on alpha, parisc, sparc */
/* (1 << 24) must not be used -- it collides with flags on alpha, sparc */
/* (1 << 25) must not be used -- it collides with flags on sparc */

pub const F_DUPFD: u32 = 0; // dup
pub const F_GETFD: u32 = 1; // get close_on_exec
pub const F_SETFD: u32 = 2; // set/clear close_on_exec
pub const F_GETFL: u32 = 3; // get file->f_flags
pub const F_SETFL: u32 = 4; // set file->f_flags
pub const F_GETLK: u32 = 5;
pub const F_SETLK: u32 = 6;
pub const F_SETLKW: u32 = 7;
pub const F_SETOWN: u32 = 8; // for sockets.
pub const F_GETOWN: u32 = 9; // for sockets.
pub const F_SETSIG: u32 = 10; // for sockets.
pub const F_GETSIG: u32 = 11; // for sockets.

// Original: #if __BITS_PER_LONG == 32 || defined(__KERNEL__)
pub const F_GETLK64: u32 = 12; // using 'struct flock64'
pub const F_SETLK64: u32 = 13;
pub const F_SETLKW64: u32 = 14;
// #endif

pub const F_SETOWN_EX: u32 = 15;
pub const F_GETOWN_EX: u32 = 16;
pub const F_GETOWNER_UIDS: u32 = 17;

/* Open File Description Locks */
pub const F_OFD_GETLK: u32 = 36;
pub const F_OFD_SETLK: u32 = 37;
pub const F_OFD_SETLKW: u32 = 38;

pub const F_OWNER_TID: u32 = 0;
pub const F_OWNER_PID: u32 = 1;
pub const F_OWNER_PGRP: u32 = 2;

#[repr(C)]
pub struct f_owner_ex {
    pub type_: core::ffi::c_int,
    pub pid: __kernel_pid_t,
}

pub const FD_CLOEXEC: u32 = 1; // actually anything with low bit set goes

pub const F_RDLCK: u32 = 0;
pub const F_WRLCK: u32 = 1;
pub const F_UNLCK: u32 = 2;
pub const F_EXLCK: u32 = 4; // or 3
pub const F_SHLCK: u32 = 8; // or 4

pub const LOCK_SH: u32 = 1; // shared lock
pub const LOCK_EX: u32 = 2; // exclusive lock
pub const LOCK_NB: u32 = 4; // or'd with one of the above to prevent blocking
pub const LOCK_UN: u32 = 8; // remove lock

/*
 * LOCK_MAND support has been removed from the kernel. We leave the symbols
 * here to not break legacy builds, but these should not be used in new code.
 */
pub const LOCK_MAND: u32 = 32; // This is a mandatory flock ...
pub const LOCK_READ: u32 = 64; // which allows concurrent read operations
pub const LOCK_WRITE: u32 = 128; // which allows concurrent write operations
pub const LOCK_RW: u32 = 192; // which allows concurrent read & write ops

pub const F_LINUX_SPECIFIC_BASE: u32 = 1024;

// Original: #ifndef HAVE_ARCH_STRUCT_FLOCK
#[repr(C)]
pub struct flock {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: __kernel_off_t,
    pub l_len: __kernel_off_t,
    pub l_pid: __kernel_pid_t,
    // Original conditional __ARCH_FLOCK_EXTRA_SYSID / __ARCH_FLOCK_PAD fields
    // are architecture-provided and have no file-local Rust mapping.
}

#[repr(C)]
pub struct flock64 {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: __kernel_loff_t,
    pub l_len: __kernel_loff_t,
    pub l_pid: __kernel_pid_t,
    // Original conditional __ARCH_FLOCK64_PAD fields are architecture-provided.
}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
