/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 * Copyright 2017-2018 NXP
 */

/*
 * Data Path SEC Interface API
 * Contains initialization APIs and runtime control APIs for DPSECI
 */

/* Maximum number of Tx/Rx queues per DPSECI object */
pub const DPSECI_MAX_QUEUE_NUM: usize = 16;

/* All queues considered; see dpseci_set_rx_queue() */
pub const DPSECI_ALL_QUEUES: u8 = u8::MAX;

extern "C" {
    pub fn dpseci_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpseci_id: i32,
                       token: *mut u16) -> i32;
    pub fn dpseci_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
}

/* Enable the Congestion Group support */
pub const DPSECI_OPT_HAS_CG: u32 = 0x000020;

#[repr(C)]
pub struct dpseci_cfg {
    pub options: u32,
    pub num_tx_queues: u8,
    pub num_rx_queues: u8,
    pub priorities: [u8; DPSECI_MAX_QUEUE_NUM],
}

extern "C" {
    pub fn dpseci_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpseci_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpseci_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> i32;
    pub fn dpseci_is_enabled(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                             en: *mut i32) -> i32;
}

#[repr(C)]
pub struct dpseci_attr {
    pub id: i32,
    pub num_tx_queues: u8,
    pub num_rx_queues: u8,
    pub options: u32,
}

extern "C" {
    pub fn dpseci_get_attributes(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                                 attr: *mut dpseci_attr) -> i32;
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dpseci_dest {
    DPSECI_DEST_NONE = 0,
    DPSECI_DEST_DPIO,
    DPSECI_DEST_DPCON,
}

#[repr(C)]
pub struct dpseci_dest_cfg {
    pub dest_type: dpseci_dest,
    pub dest_id: i32,
    pub priority: u8,
}

pub const DPSECI_QUEUE_OPT_USER_CTX: u32 = 0x00000001;
pub const DPSECI_QUEUE_OPT_DEST: u32 = 0x00000002;
pub const DPSECI_QUEUE_OPT_ORDER_PRESERVATION: u32 = 0x00000004;

#[repr(C)]
pub struct dpseci_rx_queue_cfg {
    pub options: u32,
    pub order_preservation_en: i32,
    pub user_ctx: u64,
    pub dest_cfg: dpseci_dest_cfg,
}

extern "C" {
    pub fn dpseci_set_rx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                               queue: u8, cfg: *const dpseci_rx_queue_cfg) -> i32;
}

#[repr(C)]
pub struct dpseci_rx_queue_attr {
    pub user_ctx: u64,
    pub order_preservation_en: i32,
    pub dest_cfg: dpseci_dest_cfg,
    pub fqid: u32,
}

extern "C" {
    pub fn dpseci_get_rx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                               queue: u8, attr: *mut dpseci_rx_queue_attr) -> i32;
}

#[repr(C)]
pub struct dpseci_tx_queue_attr {
    pub fqid: u32,
    pub priority: u8,
}

extern "C" {
    pub fn dpseci_get_tx_queue(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                               queue: u8, attr: *mut dpseci_tx_queue_attr) -> i32;
}

#[repr(C)]
pub struct dpseci_sec_attr {
    pub ip_id: u16,
    pub major_rev: u8,
    pub minor_rev: u8,
    pub era: u8,
    pub deco_num: u8,
    pub zuc_auth_acc_num: u8,
    pub zuc_enc_acc_num: u8,
    pub snow_f8_acc_num: u8,
    pub snow_f9_acc_num: u8,
    pub crc_acc_num: u8,
    pub pk_acc_num: u8,
    pub kasumi_acc_num: u8,
    pub rng_acc_num: u8,
    pub md_acc_num: u8,
    pub arc4_acc_num: u8,
    pub des_acc_num: u8,
    pub aes_acc_num: u8,
    pub ccha_acc_num: u8,
    pub ptha_acc_num: u8,
}

extern "C" {
    pub fn dpseci_get_sec_attr(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
                               attr: *mut dpseci_sec_attr) -> i32;
    pub fn dpseci_get_api_version(mc_io: *mut fsl_mc_io, cmd_flags: u32,
                                  major_ver: *mut u16, minor_ver: *mut u16) -> i32;
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dpseci_congestion_unit {
    DPSECI_CONGESTION_UNIT_BYTES = 0,
    DPSECI_CONGESTION_UNIT_FRAMES,
}

pub const DPSECI_CGN_MODE_WRITE_MEM_ON_ENTER: u32 = 0x00000001;
pub const DPSECI_CGN_MODE_WRITE_MEM_ON_EXIT: u32 = 0x00000002;
pub const DPSECI_CGN_MODE_COHERENT_WRITE: u32 = 0x00000004;
pub const DPSECI_CGN_MODE_NOTIFY_DEST_ON_ENTER: u32 = 0x00000008;
pub const DPSECI_CGN_MODE_NOTIFY_DEST_ON_EXIT: u32 = 0x00000010;
pub const DPSECI_CGN_MODE_INTR_COALESCING_DISABLED: u32 = 0x00000020;

#[repr(C)]
pub struct dpseci_congestion_notification_cfg {
    pub units: dpseci_congestion_unit,
    pub threshold_entry: u32,
    pub threshold_exit: u32,
    pub message_ctx: u64,
    pub message_iova: u64,
    pub dest_cfg: dpseci_dest_cfg,
    pub notification_mode: u16,
}

extern "C" {
    pub fn dpseci_set_congestion_notification(
        mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
        cfg: *const dpseci_congestion_notification_cfg) -> i32;
    pub fn dpseci_get_congestion_notification(
        mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16,
        cfg: *mut dpseci_congestion_notification_cfg) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
