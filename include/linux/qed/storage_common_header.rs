/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

/* SCSI constants */
pub const SCSI_MAX_NUM_OF_CMDQS: usize = NUM_OF_GLOBAL_QUEUES / 2;
pub const BDQ_NUM_RESOURCES: usize = 4;

pub const BDQ_ID_RQ: usize = 0;
pub const BDQ_ID_IMM_DATA: usize = 1;
pub const BDQ_ID_TQ: usize = 2;
pub const BDQ_NUM_IDS: usize = 3;

pub const SCSI_NUM_SGES_SLOW_SGL_THR: usize = 8;
pub const BDQ_MAX_EXTERNAL_RING_SIZE: usize = BIT(15);

/* SCSI op codes */
pub const SCSI_OPCODE_COMPARE_AND_WRITE: u8 = 0x89;
pub const SCSI_OPCODE_READ_10: u8 = 0x28;
pub const SCSI_OPCODE_WRITE_6: u8 = 0x0A;
pub const SCSI_OPCODE_WRITE_10: u8 = 0x2A;
pub const SCSI_OPCODE_WRITE_12: u8 = 0xAA;
pub const SCSI_OPCODE_WRITE_16: u8 = 0x8A;
pub const SCSI_OPCODE_WRITE_AND_VERIFY_10: u8 = 0x2E;
pub const SCSI_OPCODE_WRITE_AND_VERIFY_12: u8 = 0xAE;
pub const SCSI_OPCODE_WRITE_AND_VERIFY_16: u8 = 0x8E;

/* iSCSI Drv opaque */
#[repr(C)]
pub struct iscsi_drv_opaque {
    pub reserved_zero: [__le16; 3],
    pub opaque: __le16,
}

/* Scsi 2B/8B opaque union */
#[repr(C)]
pub union scsi_opaque {
    pub fcoe_opaque: regpair,
    pub iscsi_opaque: iscsi_drv_opaque,
}

/* SCSI buffer descriptor */
#[repr(C)]
pub struct scsi_bd {
    pub address: regpair,
    pub opaque: scsi_opaque,
}

/* Scsi Drv BDQ struct */
#[repr(C)]
pub struct scsi_bdq_ram_drv_data {
    pub external_producer: __le16,
    pub reserved0: [__le16; 3],
}

/* SCSI SGE entry */
#[repr(C)]
pub struct scsi_sge {
    pub sge_addr: regpair,
    pub sge_len: __le32,
    pub reserved: __le32,
}

/* Cached SGEs section */
#[repr(C)]
pub struct scsi_cached_sges {
    pub sge: [scsi_sge; 4],
}

/* Scsi Drv CMDQ struct */
#[repr(C)]
pub struct scsi_drv_cmdq {
    pub cmdq_cons: __le16,
    pub reserved0: __le16,
    pub reserved1: __le32,
}

/* Common SCSI init params passed by driver to FW in function init ramrod */
#[repr(C)]
pub struct scsi_init_func_params {
    pub num_tasks: __le16,
    pub log_page_size: u8,
    pub log_page_size_conn: u8,
    pub debug_mode: u8,
    pub reserved2: [u8; 11],
}

/* SCSI RQ/CQ/CMDQ firmware function init parameters */
#[repr(C)]
pub struct scsi_init_func_queues {
    pub glbl_q_params_addr: regpair,
    pub rq_buffer_size: __le16,
    pub cq_num_entries: __le16,
    pub cmdq_num_entries: __le16,
    pub bdq_resource_id: u8,
    pub q_validity: u8,
    pub cq_cmdq_sb_num_arr: [__le16; SCSI_MAX_NUM_OF_CMDQS],
    pub num_queues: u8,
    pub queue_relative_offset: u8,
    pub cq_sb_pi: u8,
    pub cmdq_sb_pi: u8,
    pub bdq_pbl_num_entries: [u8; BDQ_NUM_IDS],
    pub reserved1: u8,
    pub bdq_pbl_base_address: [regpair; BDQ_NUM_IDS],
    pub bdq_xoff_threshold: [__le16; BDQ_NUM_IDS],
    pub cmdq_xoff_threshold: __le16,
    pub bdq_xon_threshold: [__le16; BDQ_NUM_IDS],
    pub cmdq_xon_threshold: __le16,
}

pub const SCSI_INIT_FUNC_QUEUES_RQ_VALID_MASK: u8 = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_RQ_VALID_SHIFT: u8 = 0;
pub const SCSI_INIT_FUNC_QUEUES_IMM_DATA_VALID_MASK: u8 = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_IMM_DATA_VALID_SHIFT: u8 = 1;
pub const SCSI_INIT_FUNC_QUEUES_CMD_VALID_MASK: u8 = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_CMD_VALID_SHIFT: u8 = 2;
pub const SCSI_INIT_FUNC_QUEUES_TQ_VALID_MASK: u8 = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_TQ_VALID_SHIFT: u8 = 3;
pub const SCSI_INIT_FUNC_QUEUES_SOC_EN_MASK: u8 = 0x1;
pub const SCSI_INIT_FUNC_QUEUES_SOC_EN_SHIFT: u8 = 4;
pub const SCSI_INIT_FUNC_QUEUES_SOC_NUM_OF_BLOCKS_LOG_MASK: u8 = 0x7;
pub const SCSI_INIT_FUNC_QUEUES_SOC_NUM_OF_BLOCKS_LOG_SHIFT: u8 = 5;

/* Scsi Drv BDQ Data struct (2 BDQ IDs: 0 - RQ, 1 - Immediate Data) */
#[repr(C)]
pub struct scsi_ram_per_bdq_resource_drv_data {
    pub drv_data_per_bdq_id: [scsi_bdq_ram_drv_data; BDQ_NUM_IDS],
}

/* SCSI SGL types */
#[repr(i32)]
pub enum scsi_sgl_mode {
    SCSI_TX_SLOW_SGL,
    SCSI_FAST_SGL,
    MAX_SCSI_SGL_MODE,
}

/* SCSI SGL parameters */
#[repr(C)]
pub struct scsi_sgl_params {
    pub sgl_addr: regpair,
    pub sgl_total_length: __le32,
    pub sge_offset: __le32,
    pub sgl_num_sges: __le16,
    pub sgl_index: u8,
    pub reserved: u8,
}

/* SCSI terminate connection params */
#[repr(C)]
pub struct scsi_terminate_extra_params {
    pub unsolicited_cq_count: __le16,
    pub cmdq_count: __le16,
    pub reserved: [u8; 4],
}

/* SCSI Task Queue Element */
#[repr(C)]
pub struct scsi_tqe {
    pub itid: __le16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
