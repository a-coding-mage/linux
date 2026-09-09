/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Management Complex (MC) userspace public interface
 *
 * Copyright 2021 NXP
 *
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

pub const MC_CMD_NUM_OF_PARAMS: usize = 7;

/**
 * struct fsl_mc_command - Management Complex (MC) command structure
 * @header: MC command header
 * @params: MC command parameters
 *
 * Used by FSL_MC_SEND_MC_COMMAND
 */
#[repr(C)]
pub struct fsl_mc_command {
    pub header: __le64,
    pub params: [__le64; MC_CMD_NUM_OF_PARAMS],
}

pub const FSL_MC_SEND_CMD_IOCTL_TYPE: u8 = b'R';
pub const FSL_MC_SEND_CMD_IOCTL_SEQ: u8 = 0xE0;

// _IOWR is supplied by the target ioctl definitions.
pub const FSL_MC_SEND_MC_COMMAND: _ = _IOWR!(
    FSL_MC_SEND_CMD_IOCTL_TYPE,
    FSL_MC_SEND_CMD_IOCTL_SEQ,
    fsl_mc_command
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
