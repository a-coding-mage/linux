/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const NR_OPEN: i32 = 1024;

pub const NGROUPS_MAX: i32 = 65536; // supplemental group IDs are available
pub const ARG_MAX: i32 = 131072; // # bytes of args + environ for exec()
pub const LINK_MAX: i32 = 127; // # links a file may have
pub const MAX_CANON: i32 = 255; // size of the canonical input queue
pub const MAX_INPUT: i32 = 255; // size of the type-ahead buffer
pub const NAME_MAX: i32 = 255; // # chars in a file name
pub const PATH_MAX: i32 = 4096; // # chars in a path name including nul
pub const PIPE_BUF: i32 = 4096; // # bytes in atomic write to a pipe
pub const XATTR_NAME_MAX: i32 = 255; // # chars in an extended attribute name
pub const XATTR_SIZE_MAX: i32 = 65536; // size of an extended attribute value (64k)
pub const XATTR_LIST_MAX: i32 = 65536; // size of extended attribute namelist (64k)

pub const RTSIG_MAX: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
