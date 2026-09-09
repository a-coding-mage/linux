/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Copyright (C) 2021-2023 OpenSynergy GmbH
 * Copyright Red Hat, Inc. 2025
 */

// Dependencies supplied by the corresponding Linux/VirtIO headers:
// linux/types.h, linux/virtio_types.h, linux/virtio_ids.h,
// linux/virtio_config.h

/* Feature bit numbers */
pub const VIRTIO_CAN_F_CAN_CLASSIC: u32 = 0;
pub const VIRTIO_CAN_F_CAN_FD: u32 = 1;
pub const VIRTIO_CAN_F_RTR_FRAMES: u32 = 2;
pub const VIRTIO_CAN_F_LATE_TX_ACK: u32 = 3;

/* CAN Result Types */
pub const VIRTIO_CAN_RESULT_OK: u32 = 0;
pub const VIRTIO_CAN_RESULT_NOT_OK: u32 = 1;

/* CAN flags to determine type of CAN Id */
pub const VIRTIO_CAN_FLAGS_EXTENDED: u32 = 0x8000;
pub const VIRTIO_CAN_FLAGS_FD: u32 = 0x4000;
pub const VIRTIO_CAN_FLAGS_RTR: u32 = 0x2000;

pub const VIRTIO_CAN_MAX_DLEN: usize = 64; /* this is like CANFD_MAX_DLEN */

#[repr(C)]
pub struct virtio_can_config {
    pub status: u16,
}

pub const VIRTIO_CAN_S_CTRL_BUSOFF: u32 = 1u32 << 0; /* Controller BusOff */

/* TX queue message types */
#[repr(C)]
pub struct virtio_can_tx_out {
    pub msg_type: u16,
    pub length: u16, /* 0..8 CC, 0..64 CAN-FD, 0..2048 CAN-XL, 12 bits */
    pub reserved_classic_dlc: u8, /* If CAN classic length = 8 then DLC can be 8..15 */
    pub padding: u8,
    pub reserved_xl_priority: u16, /* May be needed for CAN XL priority */
    pub flags: u32,
    pub can_id: u32,
    // Flexible array member, counted by `length` in little-endian representation.
    pub sdu: [u8; 0],
}

pub const VIRTIO_CAN_TX: u32 = 0x0001;

#[repr(C)]
pub struct virtio_can_tx_in {
    pub result: u8,
}

/* RX queue message types */
#[repr(C)]
pub struct virtio_can_rx {
    pub msg_type: u16,
    pub length: u16, /* 0..8 CC, 0..64 CAN-FD, 0..2048 CAN-XL, 12 bits */
    pub reserved_classic_dlc: u8, /* If CAN classic length = 8 then DLC can be 8..15 */
    pub padding: u8,
    pub reserved_xl_priority: u16, /* May be needed for CAN XL priority */
    pub flags: u32,
    pub can_id: u32,
    // Flexible array member, counted by `length` in little-endian representation.
    pub sdu: [u8; 0],
}

pub const VIRTIO_CAN_RX: u32 = 0x0101;

/* Control queue message types */
#[repr(C)]
pub struct virtio_can_control_out {
    pub msg_type: u16,
}

pub const VIRTIO_CAN_SET_CTRL_MODE_START: u32 = 0x0201;
pub const VIRTIO_CAN_SET_CTRL_MODE_STOP: u32 = 0x0202;

#[repr(C)]
pub struct virtio_can_control_in {
    pub result: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
