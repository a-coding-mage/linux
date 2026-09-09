/* SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note */
/*
 * Definitions for virtio I2C Adpter
 *
 * Copyright (c) 2021 Intel Corporation. All rights reserved.
 */

/* Virtio I2C Feature bits */
pub const VIRTIO_I2C_F_ZERO_LENGTH_REQUEST: u32 = 0;

/* The bit 0 of the @virtio_i2c_out_hdr.@flags, used to group the requests */
pub const VIRTIO_I2C_FLAGS_FAIL_NEXT: u32 = 1 << 0;

/* The bit 1 of the @virtio_i2c_out_hdr.@flags, used to mark a buffer as read */
pub const VIRTIO_I2C_FLAGS_M_RD: u32 = 1 << 1;

/**
 * struct virtio_i2c_out_hdr - the virtio I2C message OUT header
 * @addr: the controlled device address
 * @padding: used to pad to full dword
 * @flags: used for feature extensibility
 */
#[repr(C)]
pub struct virtio_i2c_out_hdr {
	pub addr: u16,
	pub padding: u16,
	pub flags: u32,
}

/**
 * struct virtio_i2c_in_hdr - the virtio I2C message IN header
 * @status: the processing result from the backend
 */
#[repr(C)]
pub struct virtio_i2c_in_hdr {
	pub status: u8,
}

/* The final status written by the device */
pub const VIRTIO_I2C_MSG_OK: u32 = 0;
pub const VIRTIO_I2C_MSG_ERR: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
