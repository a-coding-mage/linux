/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2022, Google LLC
 */

pub const LOADPIN_IOC_MAGIC: u8 = b'L';

/**
 * LOADPIN_IOC_SET_TRUSTED_VERITY_DIGESTS - Set up the root digests of verity devices
 *                                          that loadpin should trust.
 *
 * Takes a file descriptor from which to read the root digests of trusted verity devices. The file
 * is expected to contain a list of digests in ASCII format, with one line per digest. The ioctl
 * must be issued on the securityfs attribute 'loadpin/dm-verity' (which can be typically found
 * under /sys/kernel/security/loadpin/dm-verity).
 */
// `_IOW` is supplied by the surrounding UAPI ioctl definitions.
pub const LOADPIN_IOC_SET_TRUSTED_VERITY_DIGESTS: u32 =
    _IOW!(LOADPIN_IOC_MAGIC, 0x00, core::ffi::c_uint);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
