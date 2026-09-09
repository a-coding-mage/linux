/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers are referenced by
// name here; their declarations are intentionally not reproduced.

/* List of all valid flags for the open/openat flags argument: */
pub const VALID_OPEN_FLAGS: _ =
    O_RDONLY | O_WRONLY | O_RDWR | O_CREAT | O_EXCL | O_NOCTTY | O_TRUNC |
    O_APPEND | O_NDELAY | O_NONBLOCK | __O_SYNC | O_DSYNC |
    FASYNC | O_DIRECT | O_LARGEFILE | O_DIRECTORY | O_NOFOLLOW |
    O_NOATIME | O_CLOEXEC | O_PATH | __O_TMPFILE | O_EMPTYPATH;

/* List of all valid flags for openat2(2)'s how->flags argument. */
pub const VALID_OPENAT2_FLAGS: _ = VALID_OPEN_FLAGS | OPENAT2_REGULAR;

/*
 * Kernel-internal carrier for OPENAT2_REGULAR. The UAPI bit lives in the
 * upper 32 bits of open_how::flags so open()/openat() cannot encode it.
 * build_open_flags() translates it to this internal flag, which then
 * propagates through op->open_flag and f->f_flags exactly like __FMODE_EXEC.
 * do_dentry_open() strips it so userspace cannot observe it via
 * fcntl(F_GETFL).
 *
 * Bit 30 is not claimed by any O_* flag on any architecture and stays clear
 * of the sign bit of the int op->open_flag. fcntl_init() enforces that it
 * never aliases an open-flag bit.
 */
pub const __O_REGULAR: i32 = 1 << 30;

/* List of all valid flags for the how->resolve argument: */
pub const VALID_RESOLVE_FLAGS: _ =
    RESOLVE_NO_XDEV | RESOLVE_NO_MAGICLINKS | RESOLVE_NO_SYMLINKS |
    RESOLVE_BENEATH | RESOLVE_IN_ROOT | RESOLVE_CACHED;

/* List of all open_how "versions". */
pub const OPEN_HOW_SIZE_VER0: usize = 24; /* sizeof first published struct */
pub const OPEN_HOW_SIZE_LATEST: usize = OPEN_HOW_SIZE_VER0;

#[macro_export]
macro_rules! force_o_largefile {
    () => { !IS_ENABLED(CONFIG_ARCH_32BIT_OFF_T) };
}

// The following conditional preserves the source condition
// `BITS_PER_LONG == 32` using Rust's target pointer width.
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! IS_GETLK32 { ($cmd:expr) => { ($cmd) == F_GETLK }; }
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! IS_SETLK32 { ($cmd:expr) => { ($cmd) == F_SETLK }; }
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! IS_SETLKW32 { ($cmd:expr) => { ($cmd) == F_SETLKW }; }
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! IS_GETLK64 { ($cmd:expr) => { ($cmd) == F_GETLK64 }; }
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! IS_SETLK64 { ($cmd:expr) => { ($cmd) == F_SETLK64 }; }
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! IS_SETLKW64 { ($cmd:expr) => { ($cmd) == F_SETLKW64 }; }

#[cfg(not(target_pointer_width = "32"))]
#[macro_export]
macro_rules! IS_GETLK32 { ($cmd:expr) => { 0 }; }
#[cfg(not(target_pointer_width = "32"))]
#[macro_export]
macro_rules! IS_SETLK32 { ($cmd:expr) => { 0 }; }
#[cfg(not(target_pointer_width = "32"))]
#[macro_export]
macro_rules! IS_SETLKW32 { ($cmd:expr) => { 0 }; }
#[cfg(not(target_pointer_width = "32"))]
#[macro_export]
macro_rules! IS_GETLK64 { ($cmd:expr) => { ($cmd) == F_GETLK }; }
#[cfg(not(target_pointer_width = "32"))]
#[macro_export]
macro_rules! IS_SETLK64 { ($cmd:expr) => { ($cmd) == F_SETLK }; }
#[cfg(not(target_pointer_width = "32"))]
#[macro_export]
macro_rules! IS_SETLKW64 { ($cmd:expr) => { ($cmd) == F_SETLKW }; }

#[macro_export]
macro_rules! IS_GETLK { ($cmd:expr) => { IS_GETLK32!($cmd) || IS_GETLK64!($cmd) }; }
#[macro_export]
macro_rules! IS_SETLK { ($cmd:expr) => { IS_SETLK32!($cmd) || IS_SETLK64!($cmd) }; }
#[macro_export]
macro_rules! IS_SETLKW { ($cmd:expr) => { IS_SETLKW32!($cmd) || IS_SETLKW64!($cmd) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
