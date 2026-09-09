/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <uapi/asm/socket.h>

/* O_NONBLOCK clashed with the bits used for socket types.  Therefore we
 * had to define SOCK_NONBLOCK to a different value here.
 */
pub const SOCK_NONBLOCK: u32 = 0x4000_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
