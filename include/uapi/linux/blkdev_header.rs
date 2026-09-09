/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * io_uring block file commands, see IORING_OP_URING_CMD.
 * It's a different number space from ioctl(), reuse the block's code 0x12.
 *
 * The _IO encoding is supplied by the translated linux/ioctl.h dependency.
 */
pub const BLOCK_URING_CMD_DISCARD: u32 = _IO!(0x12, 0);
pub const BLOCK_URING_CMD_ZONE_RESET_ALL: u32 = _IO!(0x12, 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
