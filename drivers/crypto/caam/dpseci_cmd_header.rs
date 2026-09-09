/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 * Copyright 2017-2018 NXP
 */

/* DPSECI Version */
pub const DPSECI_VER_MAJOR: u32 = 5;
pub const DPSECI_VER_MINOR: u32 = 3;

#[inline]
pub const fn DPSECI_VER(maj: u32, min: u32) -> u32 {
    (maj << 16) | min
}
pub const DPSECI_VERSION: u32 = DPSECI_VER(DPSECI_VER_MAJOR, DPSECI_VER_MINOR);

/* Command versioning */
pub const DPSECI_CMD_BASE_VERSION: u32 = 1;
pub const DPSECI_CMD_BASE_VERSION_V2: u32 = 2;
pub const DPSECI_CMD_ID_OFFSET: u32 = 4;

#[inline]
pub const fn DPSECI_CMD_V1(id: u32) -> u32 {
    (id << DPSECI_CMD_ID_OFFSET) | DPSECI_CMD_BASE_VERSION
}

#[inline]
pub const fn DPSECI_CMD_V2(id: u32) -> u32 {
    (id << DPSECI_CMD_ID_OFFSET) | DPSECI_CMD_BASE_VERSION_V2
}

/* Command IDs */
pub const DPSECI_CMDID_CLOSE: u32 = DPSECI_CMD_V1(0x800);
pub const DPSECI_CMDID_OPEN: u32 = DPSECI_CMD_V1(0x809);
pub const DPSECI_CMDID_GET_API_VERSION: u32 = DPSECI_CMD_V1(0xa09);
pub const DPSECI_CMDID_ENABLE: u32 = DPSECI_CMD_V1(0x002);
pub const DPSECI_CMDID_DISABLE: u32 = DPSECI_CMD_V1(0x003);
pub const DPSECI_CMDID_GET_ATTR: u32 = DPSECI_CMD_V1(0x004);
pub const DPSECI_CMDID_RESET: u32 = DPSECI_CMD_V1(0x005);
pub const DPSECI_CMDID_IS_ENABLED: u32 = DPSECI_CMD_V1(0x006);
pub const DPSECI_CMDID_SET_RX_QUEUE: u32 = DPSECI_CMD_V1(0x194);
pub const DPSECI_CMDID_GET_RX_QUEUE: u32 = DPSECI_CMD_V1(0x196);
pub const DPSECI_CMDID_GET_TX_QUEUE: u32 = DPSECI_CMD_V1(0x197);
pub const DPSECI_CMDID_GET_SEC_ATTR: u32 = DPSECI_CMD_V2(0x198);
pub const DPSECI_CMDID_SET_CONGESTION_NOTIFICATION: u32 = DPSECI_CMD_V1(0x170);
pub const DPSECI_CMDID_GET_CONGESTION_NOTIFICATION: u32 = DPSECI_CMD_V1(0x171);

/* Macros for accessing command fields smaller than 1 byte */
#[macro_export]
macro_rules! dpseci_field_shift {
    (ENABLE) => { DPSECI_ENABLE_SHIFT };
    (DEST_TYPE) => { DPSECI_DEST_TYPE_SHIFT };
    (ORDER_PRESERVATION) => { DPSECI_ORDER_PRESERVATION_SHIFT };
    (CGN_DEST_TYPE) => { DPSECI_CGN_DEST_TYPE_SHIFT };
    (CGN_UNITS) => { DPSECI_CGN_UNITS_SHIFT };
}

#[macro_export]
macro_rules! dpseci_field_size {
    (ENABLE) => { DPSECI_ENABLE_SIZE };
    (DEST_TYPE) => { DPSECI_DEST_TYPE_SIZE };
    (ORDER_PRESERVATION) => { DPSECI_ORDER_PRESERVATION_SIZE };
    (CGN_DEST_TYPE) => { DPSECI_CGN_DEST_TYPE_SIZE };
    (CGN_UNITS) => { DPSECI_CGN_UNITS_SIZE };
}

#[macro_export]
macro_rules! DPSECI_MASK {
    ($field:ident) => {
        (((1u32 << dpseci_field_size!($field)) - 1) << dpseci_field_shift!($field))
    };
}

#[macro_export]
macro_rules! dpseci_set_field {
    ($var:expr, $field:ident, $val:expr) => {
        $var |= (($val << dpseci_field_shift!($field)) & DPSECI_MASK!($field))
    };
}

#[macro_export]
macro_rules! dpseci_get_field {
    ($var:expr, $field:ident) => {
        (($var & DPSECI_MASK!($field)) >> dpseci_field_shift!($field))
    };
}

#[repr(C)]
pub struct dpseci_cmd_open {
    pub dpseci_id: u32,
}

pub const DPSECI_ENABLE_SHIFT: u32 = 0;
pub const DPSECI_ENABLE_SIZE: u32 = 1;

#[repr(C)]
pub struct dpseci_rsp_is_enabled {
    pub is_enabled: u8,
}

#[repr(C)]
pub struct dpseci_rsp_get_attributes {
    pub id: u32,
    pub pad0: u32,
    pub num_tx_queues: u8,
    pub num_rx_queues: u8,
    pub pad1: [u8; 6],
    pub options: u32,
}

pub const DPSECI_DEST_TYPE_SHIFT: u32 = 0;
pub const DPSECI_DEST_TYPE_SIZE: u32 = 4;
pub const DPSECI_ORDER_PRESERVATION_SHIFT: u32 = 0;
pub const DPSECI_ORDER_PRESERVATION_SIZE: u32 = 1;

#[repr(C)]
pub union dpseci_cmd_queue__bindgen_ty_1 {
    pub options: u32,
    pub fqid: u32,
}

#[repr(C)]
pub struct dpseci_cmd_queue {
    pub dest_id: u32,
    pub priority: u8,
    pub queue: u8,
    pub dest_type: u8,
    pub pad: u8,
    pub user_ctx: u64,
    pub __bindgen_anon_1: dpseci_cmd_queue__bindgen_ty_1,
    pub order_preservation_en: u8,
}

#[repr(C)]
pub struct dpseci_rsp_get_tx_queue {
    pub pad: u32,
    pub fqid: u32,
    pub priority: u8,
}

#[repr(C)]
pub struct dpseci_rsp_get_sec_attr {
    pub ip_id: u16,
    pub major_rev: u8,
    pub minor_rev: u8,
    pub era: u8,
    pub pad0: [u8; 3],
    pub deco_num: u8,
    pub zuc_auth_acc_num: u8,
    pub zuc_enc_acc_num: u8,
    pub pad1: u8,
    pub snow_f8_acc_num: u8,
    pub snow_f9_acc_num: u8,
    pub crc_acc_num: u8,
    pub pad2: u8,
    pub pk_acc_num: u8,
    pub kasumi_acc_num: u8,
    pub rng_acc_num: u8,
    pub pad3: u8,
    pub md_acc_num: u8,
    pub arc4_acc_num: u8,
    pub des_acc_num: u8,
    pub aes_acc_num: u8,
    pub ccha_acc_num: u8,
    pub ptha_acc_num: u8,
}

#[repr(C)]
pub struct dpseci_rsp_get_api_version {
    pub major: u16,
    pub minor: u16,
}

pub const DPSECI_CGN_DEST_TYPE_SHIFT: u32 = 0;
pub const DPSECI_CGN_DEST_TYPE_SIZE: u32 = 4;
pub const DPSECI_CGN_UNITS_SHIFT: u32 = 4;
pub const DPSECI_CGN_UNITS_SIZE: u32 = 2;

#[repr(C)]
pub struct dpseci_cmd_congestion_notification {
    pub dest_id: u32,
    pub notification_mode: u16,
    pub priority: u8,
    pub options: u8,
    pub message_iova: u64,
    pub message_ctx: u64,
    pub threshold_entry: u32,
    pub threshold_exit: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
