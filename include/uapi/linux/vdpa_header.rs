/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * vdpa device management interface
 * Copyright (c) 2020 Mellanox Technologies Ltd. All rights reserved.
 */

pub const VDPA_GENL_NAME: &str = "vdpa";
pub const VDPA_GENL_VERSION: u32 = 0x1;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vdpa_command {
    VDPA_CMD_UNSPEC = 0,
    VDPA_CMD_MGMTDEV_NEW = 1,
    VDPA_CMD_MGMTDEV_GET = 2, // can dump
    VDPA_CMD_DEV_NEW = 3,
    VDPA_CMD_DEV_DEL = 4,
    VDPA_CMD_DEV_GET = 5, // can dump
    VDPA_CMD_DEV_CONFIG_GET = 6, // can dump
    VDPA_CMD_DEV_VSTATS_GET = 7,
    VDPA_CMD_DEV_ATTR_SET = 8,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vdpa_attr {
    VDPA_ATTR_UNSPEC = 0,

    // Pad attribute for 64b alignment

    // bus name (optional) + dev name together make the parent device handle
    VDPA_ATTR_MGMTDEV_BUS_NAME = 1, // string
    VDPA_ATTR_MGMTDEV_DEV_NAME = 2, // string
    VDPA_ATTR_MGMTDEV_SUPPORTED_CLASSES = 3, // u64

    VDPA_ATTR_DEV_NAME = 4, // string
    VDPA_ATTR_DEV_ID = 5, // u32
    VDPA_ATTR_DEV_VENDOR_ID = 6, // u32
    VDPA_ATTR_DEV_MAX_VQS = 7, // u32
    VDPA_ATTR_DEV_MAX_VQ_SIZE = 8, // u16
    VDPA_ATTR_DEV_MIN_VQ_SIZE = 9, // u16

    VDPA_ATTR_DEV_NET_CFG_MACADDR = 10, // binary
    VDPA_ATTR_DEV_NET_STATUS = 11, // u8
    VDPA_ATTR_DEV_NET_CFG_MAX_VQP = 12, // u16
    VDPA_ATTR_DEV_NET_CFG_MTU = 13, // u16

    VDPA_ATTR_DEV_NEGOTIATED_FEATURES = 14, // u64
    VDPA_ATTR_DEV_MGMTDEV_MAX_VQS = 15, // u32
    // virtio features that are supported by the vDPA management device
    VDPA_ATTR_DEV_SUPPORTED_FEATURES = 16, // u64

    VDPA_ATTR_DEV_QUEUE_INDEX = 17, // u32
    VDPA_ATTR_DEV_VENDOR_ATTR_NAME = 18, // string
    VDPA_ATTR_DEV_VENDOR_ATTR_VALUE = 19, // u64

    // virtio features that are provisioned to the vDPA device
    VDPA_ATTR_DEV_FEATURES = 20, // u64

    VDPA_ATTR_DEV_BLK_CFG_CAPACITY = 21, // u64
    VDPA_ATTR_DEV_BLK_CFG_SIZE_MAX = 22, // u32
    VDPA_ATTR_DEV_BLK_CFG_BLK_SIZE = 23, // u32
    VDPA_ATTR_DEV_BLK_CFG_SEG_MAX = 24, // u32
    VDPA_ATTR_DEV_BLK_CFG_NUM_QUEUES = 25, // u16
    VDPA_ATTR_DEV_BLK_CFG_PHY_BLK_EXP = 26, // u8
    VDPA_ATTR_DEV_BLK_CFG_ALIGN_OFFSET = 27, // u8
    VDPA_ATTR_DEV_BLK_CFG_MIN_IO_SIZE = 28, // u16
    VDPA_ATTR_DEV_BLK_CFG_OPT_IO_SIZE = 29, // u32
    VDPA_ATTR_DEV_BLK_CFG_MAX_DISCARD_SEC = 30, // u32
    VDPA_ATTR_DEV_BLK_CFG_MAX_DISCARD_SEG = 31, // u32
    VDPA_ATTR_DEV_BLK_CFG_DISCARD_SEC_ALIGN = 32, // u32
    VDPA_ATTR_DEV_BLK_CFG_MAX_WRITE_ZEROES_SEC = 33, // u32
    VDPA_ATTR_DEV_BLK_CFG_MAX_WRITE_ZEROES_SEG = 34, // u32
    VDPA_ATTR_DEV_BLK_READ_ONLY = 35, // u8
    VDPA_ATTR_DEV_BLK_FLUSH = 36, // u8

    // new attributes must be added above here
    VDPA_ATTR_MAX = 37,
}

// VDPA_ATTR_PAD = VDPA_ATTR_UNSPEC; Pad attribute for 64b alignment.
pub const VDPA_ATTR_PAD: vdpa_attr = vdpa_attr::VDPA_ATTR_UNSPEC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
