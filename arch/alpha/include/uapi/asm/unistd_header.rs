/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// _UAPI_ALPHA_UNISTD_H

/* These are traditionally the names linux-alpha uses for
 * the two otherwise generic system calls */
pub const __NR_umount: _ = __NR_umount2;
pub const __NR_osf_shmat: _ = __NR_shmat;

/* These return an extra value but can be used as aliases */
pub const __NR_getpid: _ = __NR_getxpid;
pub const __NR_getuid: _ = __NR_getxuid;
pub const __NR_getgid: _ = __NR_getxgid;

// Dependency intent: declarations from <asm/unistd_32.h> are supplied by
// another translated header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
