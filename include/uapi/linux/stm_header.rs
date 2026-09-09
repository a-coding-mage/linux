/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * System Trace Module (STM) userspace interfaces
 * Copyright (c) 2014, Intel Corporation.
 *
 * STM class implements generic infrastructure for  System Trace Module devices
 * as defined in MIPI STPv2 specification.
 */

/* Dependency equivalent of <linux/types.h>. */

/* Maximum allowed master and channel values */
pub const STP_MASTER_MAX: u32 = 0xffff;
pub const STP_CHANNEL_MAX: u32 = 0xffff;

/**
 * struct stp_policy_id - identification for the STP policy
 * @size:\tsize of the structure including real id[] length
 * @master:\tassigned master
 * @channel:\tfirst assigned channel
 * @width:\tnumber of requested channels
 * @id:\t\tidentification string
 *
 * User must calculate the total size of the structure and put it into
 * @size field, fill out the @id and desired @width. In return, kernel
 * fills out @master, @channel and @width.
 */
#[repr(C)]
pub struct stp_policy_id {
    pub size: u32,
    pub master: u16,
    pub channel: u16,
    pub width: u16,
    /* padding */
    pub __reserved_0: u16,
    pub __reserved_1: u32,
    pub id: [core::ffi::c_char; 0],
}

/* The _IOWR/_IOR/_IOW ioctl encodings are supplied by the target Linux ABI. */
// #define STP_POLICY_ID_SET _IOWR('%', 0, struct stp_policy_id)
// #define STP_POLICY_ID_GET _IOR('%', 1, struct stp_policy_id)
// #define STP_SET_OPTIONS   _IOW('%', 2, __u64)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
