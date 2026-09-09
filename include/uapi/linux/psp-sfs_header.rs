/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Userspace interface for AMD Seamless Firmware Servicing (SFS)
 *
 * Copyright (C) 2025 Advanced Micro Devices, Inc.
 *
 * Author: Ashish Kalra <ashish.kalra@amd.com>
 */

// SFS: AMD Seamless Firmware Support (SFS) interface

pub const PAYLOAD_NAME_SIZE: usize = 64;
pub const TEE_EXT_CMD_BUFFER_SIZE: usize = 4096;

/**
 * struct sfs_user_get_fw_versions - get current level of base firmware (output).
 * @blob:                  current level of base firmware for ASP and patch levels (input/output).
 * @sfs_status:            32-bit SFS status value (output).
 * @sfs_extended_status:   32-bit SFS extended status value (output).
 */
#[repr(C, packed)]
pub struct sfs_user_get_fw_versions {
    pub blob: [u8; TEE_EXT_CMD_BUFFER_SIZE],
    pub sfs_status: u32,
    pub sfs_extended_status: u32,
}

/**
 * struct sfs_user_update_package - update SFS package (input).
 * @payload_name:          name of SFS package to load, verify and execute (input).
 * @sfs_status:            32-bit SFS status value (output).
 * @sfs_extended_status:   32-bit SFS extended status value (output).
 */
#[repr(C, packed)]
pub struct sfs_user_update_package {
    pub payload_name: [core::ffi::c_char; PAYLOAD_NAME_SIZE],
    pub sfs_status: u32,
    pub sfs_extended_status: u32,
}

/*
 * Seamless Firmware Support (SFS) IOC
 *
 * possible return codes for all SFS IOCTLs:
 *  0:          success
 *  -EINVAL:    invalid input
 *  -E2BIG:     excess data passed
 *  -EFAULT:    failed to copy to/from userspace
 *  -EBUSY:     mailbox in recovery or in use
 *  -ENODEV:    driver not bound with PSP device
 *  -EACCES:    request isn't authorized
 *  -EINVAL:    invalid parameter
 *  -ETIMEDOUT: request timed out
 *  -EAGAIN:    invalid request for state machine
 *  -ENOENT:    not implemented
 *  -ENFILE:    overflow
 *  -EPERM:     invalid signature
 *  -EIO:       PSP I/O error
 */
pub const SFS_IOC_TYPE: u8 = b'S';

/**
 * SFSIOCFWVERS - returns blob containing FW versions
 *                ASP provides the current level of Base Firmware for the ASP
 *                and the other microprocessors as well as current patch
 *                level(s).
 */
// _IOWR is supplied by the Linux ioctl interface dependency.
pub const SFSIOCFWVERS: usize = _IOWR!(SFS_IOC_TYPE, 0x1, sfs_user_get_fw_versions);

/**
 * SFSIOCUPDATEPKG - updates package/payload
 *                   ASP loads, verifies and executes the SFS package.
 *                   By default, the SFS package/payload is loaded from
 *                   /lib/firmware/amd, but alternative firmware loading path
 *                   can be specified using kernel parameter
 *                   firmware_class.path or the firmware loading path
 *                   can be customized using sysfs file:
 *                   /sys/module/firmware_class/parameters/path.
 */
pub const SFSIOCUPDATEPKG: usize = _IOWR!(SFS_IOC_TYPE, 0x2, sfs_user_update_package);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
