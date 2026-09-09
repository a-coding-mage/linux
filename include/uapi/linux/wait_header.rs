/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const WNOHANG: u32 = 0x00000001;
pub const WUNTRACED: u32 = 0x00000002;
pub const WSTOPPED: u32 = WUNTRACED;
pub const WEXITED: u32 = 0x00000004;
pub const WCONTINUED: u32 = 0x00000008;
pub const WNOWAIT: u32 = 0x01000000; /* Don't reap, just poll status.  */

pub const __WNOTHREAD: u32 = 0x20000000; /* Don't wait on children of other threads in this group */
pub const __WALL: u32 = 0x40000000; /* Wait on all children, regardless of type */
pub const __WCLONE: u32 = 0x80000000; /* Wait only on non-SIGCHLD children */

/* First argument to waitid: */
pub const P_ALL: u32 = 0;
pub const P_PID: u32 = 1;
pub const P_PGID: u32 = 2;
pub const P_PIDFD: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
