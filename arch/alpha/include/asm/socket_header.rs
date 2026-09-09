/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the declarations from <uapi/asm/socket.h> are supplied
// by the corresponding Rust translation and are not reproduced here.

/* O_NONBLOCK clashes with the bits used for socket types.  Therefore we
 * have to define SOCK_NONBLOCK to a different value here.
 */
pub const SOCK_NONBLOCK: i32 = 0x4000_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
