/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2017-2018, Mellanox Technologies inc.  All rights reserved.
 *
 * This software is available under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// C dependencies: <linux/types.h>, <rdma/ib_user_verbs.h>

pub const IB_UVERBS_ACCESS_OPTIONAL_FIRST: u32 = 1 << 20;
pub const IB_UVERBS_ACCESS_OPTIONAL_LAST: u32 = 1 << 29;

#[repr(u32)]
pub enum ib_uverbs_core_support {
    IB_UVERBS_CORE_SUPPORT_OPTIONAL_MR_ACCESS = 1 << 0,
    IB_UVERBS_CORE_SUPPORT_ROBUST_UDATA = 1 << 1,
}

#[repr(u32)]
pub enum ib_uverbs_access_flags {
    IB_UVERBS_ACCESS_LOCAL_WRITE = 1 << 0,
    IB_UVERBS_ACCESS_REMOTE_WRITE = 1 << 1,
    IB_UVERBS_ACCESS_REMOTE_READ = 1 << 2,
    IB_UVERBS_ACCESS_REMOTE_ATOMIC = 1 << 3,
    IB_UVERBS_ACCESS_MW_BIND = 1 << 4,
    IB_UVERBS_ACCESS_ZERO_BASED = 1 << 5,
    IB_UVERBS_ACCESS_ON_DEMAND = 1 << 6,
    IB_UVERBS_ACCESS_HUGETLB = 1 << 7,
    IB_UVERBS_ACCESS_FLUSH_GLOBAL = 1 << 8,
    IB_UVERBS_ACCESS_FLUSH_PERSISTENT = 1 << 9,
    IB_UVERBS_ACCESS_RELAXED_ORDERING = IB_UVERBS_ACCESS_OPTIONAL_FIRST,
    IB_UVERBS_ACCESS_OPTIONAL_RANGE = ((IB_UVERBS_ACCESS_OPTIONAL_LAST << 1) - 1) & !(IB_UVERBS_ACCESS_OPTIONAL_FIRST - 1),
}

#[repr(u32)]
pub enum ib_uverbs_srq_type { IB_UVERBS_SRQT_BASIC, IB_UVERBS_SRQT_XRC, IB_UVERBS_SRQT_TM }
#[repr(u32)]
pub enum ib_uverbs_wq_type { IB_UVERBS_WQT_RQ }
#[repr(u32)]
pub enum ib_uverbs_wq_flags {
    IB_UVERBS_WQ_FLAGS_CVLAN_STRIPPING = 1 << 0,
    IB_UVERBS_WQ_FLAGS_SCATTER_FCS = 1 << 1,
    IB_UVERBS_WQ_FLAGS_DELAY_DROP = 1 << 2,
    IB_UVERBS_WQ_FLAGS_PCI_WRITE_END_PADDING = 1 << 3,
}
#[repr(u32)]
pub enum ib_uverbs_qp_type {
    IB_UVERBS_QPT_RC = 2, IB_UVERBS_QPT_UC, IB_UVERBS_QPT_UD,
    IB_UVERBS_QPT_RAW_PACKET = 8, IB_UVERBS_QPT_XRC_INI, IB_UVERBS_QPT_XRC_TGT,
    IB_UVERBS_QPT_DRIVER = 0xFF,
}
#[repr(u32)]
pub enum ib_uverbs_qp_create_flags {
    IB_UVERBS_QP_CREATE_BLOCK_MULTICAST_LOOPBACK = 1 << 1,
    IB_UVERBS_QP_CREATE_SCATTER_FCS = 1 << 8,
    IB_UVERBS_QP_CREATE_CVLAN_STRIPPING = 1 << 9,
    IB_UVERBS_QP_CREATE_PCI_WRITE_END_PADDING = 1 << 11,
    IB_UVERBS_QP_CREATE_SQ_SIG_ALL = 1 << 12,
}

#[repr(u64)]
pub enum ib_uverbs_query_port_cap_flags {
    IB_UVERBS_PCF_SM = 1 << 1, IB_UVERBS_PCF_NOTICE_SUP = 1 << 2,
    IB_UVERBS_PCF_TRAP_SUP = 1 << 3, IB_UVERBS_PCF_OPT_IPD_SUP = 1 << 4,
    IB_UVERBS_PCF_AUTO_MIGR_SUP = 1 << 5, IB_UVERBS_PCF_SL_MAP_SUP = 1 << 6,
    IB_UVERBS_PCF_MKEY_NVRAM = 1 << 7, IB_UVERBS_PCF_PKEY_NVRAM = 1 << 8,
    IB_UVERBS_PCF_LED_INFO_SUP = 1 << 9, IB_UVERBS_PCF_SM_DISABLED = 1 << 10,
    IB_UVERBS_PCF_SYS_IMAGE_GUID_SUP = 1 << 11, IB_UVERBS_PCF_PKEY_SW_EXT_PORT_TRAP_SUP = 1 << 12,
    IB_UVERBS_PCF_EXTENDED_SPEEDS_SUP = 1 << 14, IB_UVERBS_PCF_CM_SUP = 1 << 16,
    IB_UVERBS_PCF_SNMP_TUNNEL_SUP = 1 << 17, IB_UVERBS_PCF_REINIT_SUP = 1 << 18,
    IB_UVERBS_PCF_DEVICE_MGMT_SUP = 1 << 19, IB_UVERBS_PCF_VENDOR_CLASS_SUP = 1 << 20,
    IB_UVERBS_PCF_DR_NOTICE_SUP = 1 << 21, IB_UVERBS_PCF_CAP_MASK_NOTICE_SUP = 1 << 22,
    IB_UVERBS_PCF_BOOT_MGMT_SUP = 1 << 23, IB_UVERBS_PCF_LINK_LATENCY_SUP = 1 << 24,
    IB_UVERBS_PCF_CLIENT_REG_SUP = 1 << 25,
    /* IsOtherLocalChangesNoticeSupported is aliased by IP_BASED_GIDS and is inaccessible */
    IB_UVERBS_PCF_LINK_SPEED_WIDTH_TABLE_SUP = 1 << 27,
    IB_UVERBS_PCF_VENDOR_SPECIFIC_MADS_TABLE_SUP = 1 << 28,
    IB_UVERBS_PCF_MCAST_PKEY_TRAP_SUPPRESSION_SUP = 1 << 29,
    IB_UVERBS_PCF_MCAST_FDB_TOP_SUP = 1 << 30,
    IB_UVERBS_PCF_HIERARCHY_INFO_SUP = 1u64 << 31,
    /* NOTE this is an internal flag, not an IBA flag */
    IB_UVERBS_PCF_IP_BASED_GIDS = 1 << 26,
}
#[repr(u32)] pub enum ib_uverbs_query_port_flags { IB_UVERBS_QPF_GRH_REQUIRED = 1 << 0 }
#[repr(u32)] pub enum ib_uverbs_flow_action_esp_keymat { IB_UVERBS_FLOW_ACTION_ESP_KEYMAT_AES_GCM }
#[repr(u32)] pub enum ib_uverbs_flow_action_esp_keymat_aes_gcm_iv_algo { IB_UVERBS_FLOW_ACTION_IV_ALGO_SEQ }

#[repr(C)]
pub struct ib_uverbs_flow_action_esp_keymat_aes_gcm {
    pub iv: u64,
    pub iv_algo: u32, // Use enum ib_uverbs_flow_action_esp_keymat_aes_gcm_iv_algo
    pub salt: u32,
    pub icv_len: u32,
    pub key_len: u32,
    pub aes_key: [u32; 256 / 32],
}
#[repr(u32)] pub enum ib_uverbs_flow_action_esp_replay { IB_UVERBS_FLOW_ACTION_ESP_REPLAY_NONE, IB_UVERBS_FLOW_ACTION_ESP_REPLAY_BMP }
#[repr(C)] pub struct ib_uverbs_flow_action_esp_replay_bmp { pub size: u32 }
#[repr(u32)]
pub enum ib_uverbs_flow_action_esp_flags {
    IB_UVERBS_FLOW_ACTION_ESP_FLAGS_INLINE_CRYPTO = 0u32 << 0,
    IB_UVERBS_FLOW_ACTION_ESP_FLAGS_FULL_OFFLOAD = 1u32 << 0,
    IB_UVERBS_FLOW_ACTION_ESP_FLAGS_TUNNEL = 0u32 << 1,
    IB_UVERBS_FLOW_ACTION_ESP_FLAGS_TRANSPORT = 1u32 << 1,
    IB_UVERBS_FLOW_ACTION_ESP_FLAGS_DECRYPT = 0u32 << 2,
    IB_UVERBS_FLOW_ACTION_ESP_FLAGS_ENCRYPT = 1u32 << 2,
    IB_UVERBS_FLOW_ACTION_ESP_FLAGS_ESN_NEW_WINDOW = 1u32 << 3,
}
#[repr(C)]
pub struct ib_uverbs_flow_action_esp_encap {
    // This struct represents a list of pointers to flow_xxxx_filter that encapsulates the payload in ESP tunnel mode.
    pub val_ptr: u64, // pointer to a flow_xxxx_filter
    pub next_ptr: u64,
    pub len: u16, // Len of the filter struct val_ptr points to
    pub type_: u16, // Use flow_spec_type enum
}
#[repr(C)] pub struct ib_uverbs_flow_action_esp { pub spi: u32, pub seq: u32, pub tfc_pad: u32, pub flags: u32, pub hard_limit_pkts: u64 }
#[repr(u32)] pub enum ib_uverbs_read_counters_flags { IB_UVERBS_READ_COUNTERS_PREFER_CACHED = 1 << 0 }
#[repr(u32)] pub enum ib_uverbs_advise_mr_advice { IB_UVERBS_ADVISE_MR_ADVICE_PREFETCH, IB_UVERBS_ADVISE_MR_ADVICE_PREFETCH_WRITE, IB_UVERBS_ADVISE_MR_ADVICE_PREFETCH_NO_FAULT }
#[repr(u32)] pub enum ib_uverbs_advise_mr_flag { IB_UVERBS_ADVISE_MR_FLAG_FLUSH = 1 << 0 }

#[repr(C)]
pub struct ib_uverbs_query_port_resp_ex {
    pub legacy_resp: ib_uverbs_query_port_resp,
    pub port_cap_flags2: u16,
    pub reserved: [u8; 2],
    pub active_speed_ex: u32,
}
#[repr(C)] pub struct ib_uverbs_qp_cap { pub max_send_wr: u32, pub max_recv_wr: u32, pub max_send_sge: u32, pub max_recv_sge: u32, pub max_inline_data: u32 }

#[repr(u32)]
pub enum rdma_driver_id {
    RDMA_DRIVER_UNKNOWN, RDMA_DRIVER_MLX5, RDMA_DRIVER_MLX4, RDMA_DRIVER_CXGB3,
    RDMA_DRIVER_CXGB4, RDMA_DRIVER_MTHCA, RDMA_DRIVER_BNXT_RE, RDMA_DRIVER_OCRDMA,
    RDMA_DRIVER_NES, RDMA_DRIVER_I40IW, RDMA_DRIVER_IRDMA = RDMA_DRIVER_I40IW as isize,
    RDMA_DRIVER_VMW_PVRDMA, RDMA_DRIVER_QEDR, RDMA_DRIVER_HNS, RDMA_DRIVER_USNIC,
    RDMA_DRIVER_RXE, RDMA_DRIVER_HFI1, RDMA_DRIVER_QIB, RDMA_DRIVER_EFA,
    RDMA_DRIVER_SIW, RDMA_DRIVER_ERDMA, RDMA_DRIVER_MANA, RDMA_DRIVER_IONIC,
}
#[repr(u32)] pub enum ib_uverbs_gid_type { IB_UVERBS_GID_TYPE_IB, IB_UVERBS_GID_TYPE_ROCE_V1, IB_UVERBS_GID_TYPE_ROCE_V2 }
#[repr(C)] pub struct ib_uverbs_gid_entry { pub gid: [u64; 2], pub gid_index: u32, pub port_num: u32, pub gid_type: u32, pub netdev_ifindex: u32 /* It is 0 if there is no netdev associated with it */ }
#[repr(u32)] pub enum ib_uverbs_buffer_type { IB_UVERBS_BUFFER_TYPE_DMABUF, IB_UVERBS_BUFFER_TYPE_VA }
/* Describes a single buffer backed by dma-buf or user virtual address. */
#[repr(C)] pub struct ib_uverbs_buffer_desc { pub type_: u32, pub fd: i32, pub flags: u32, pub optional_flags: u32, pub addr: u64, pub length: u64 }
#[repr(u32)] pub enum ib_uverbs_comp_cntr_entry { IB_UVERBS_COMP_CNTR_ENTRY_COMP, IB_UVERBS_COMP_CNTR_ENTRY_ERR }
#[repr(u32)] pub enum ib_uverbs_comp_cntr_modify_op { IB_UVERBS_COMP_CNTR_MODIFY_OP_SET, IB_UVERBS_COMP_CNTR_MODIFY_OP_INC }
#[repr(u32)]
pub enum ib_uverbs_qp_attach_comp_cntr_op {
    IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_SEND = 1 << 0,
    IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_RECV = 1 << 1,
    IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_RDMA_READ = 1 << 2,
    IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_REMOTE_RDMA_READ = 1 << 3,
    IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_RDMA_WRITE = 1 << 4,
    IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_REMOTE_RDMA_WRITE = 1 << 5,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
