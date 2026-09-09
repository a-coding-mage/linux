/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2023 Advanced Micro Devices, Inc */

// Translated from pds_adminq.h. Kernel types and bitfield helpers are external dependencies.

pub const PDSC_ADMINQ_MAX_POLL_INTERVAL: u32 = 256000;
pub const PDS_AQ_FLAG_FASTPOLL: u32 = BIT(1);

#[repr(u32)] pub enum pds_core_adminq_opcode { PDS_AQ_CMD_NOP=0, PDS_AQ_CMD_CLIENT_REG=6, PDS_AQ_CMD_CLIENT_UNREG=7, PDS_AQ_CMD_CLIENT_CMD=8, PDS_AQ_CMD_LIF_IDENTIFY=20, PDS_AQ_CMD_LIF_INIT=21, PDS_AQ_CMD_LIF_RESET=22, PDS_AQ_CMD_LIF_GETATTR=23, PDS_AQ_CMD_LIF_SETATTR=24, PDS_AQ_CMD_LIF_SETPHC=25, PDS_AQ_CMD_RX_MODE_SET=30, PDS_AQ_CMD_RX_FILTER_ADD=31, PDS_AQ_CMD_RX_FILTER_DEL=32, PDS_AQ_CMD_Q_IDENTIFY=39, PDS_AQ_CMD_Q_INIT=40, PDS_AQ_CMD_Q_CONTROL=41, PDS_AQ_CMD_VF_GETATTR=60, PDS_AQ_CMD_VF_SETATTR=61 }
#[repr(u32)] pub enum pds_core_notifyq_opcode { PDS_EVENT_LINK_CHANGE=1, PDS_EVENT_RESET=2, PDS_EVENT_XCVR=5, PDS_EVENT_CLIENT=6 }
pub const PDS_COMP_COLOR_MASK: u8 = 0x80;

#[repr(C)] pub struct pds_core_notifyq_event { pub eid: __le64, pub ecode: __le16 }
#[repr(C)] pub struct pds_core_link_change_event { pub eid: __le64, pub ecode: __le16, pub link_status: __le16, pub link_speed: __le32 }
#[repr(C)] pub struct pds_core_reset_event { pub eid: __le64, pub ecode: __le16, pub reset_code: u8, pub state: u8 }
#[repr(C)] pub struct pds_core_client_event { pub eid: __le64, pub ecode: __le16, pub client_id: __le16, pub client_event: [u8;54] }
#[repr(C)] pub struct pds_core_notifyq_cmd { pub data: __le32 }
#[repr(C)] pub union pds_core_notifyq_comp { pub header: pds_core_notifyq_header, pub event: pds_core_notifyq_event, pub link_change: pds_core_link_change_event, pub reset: pds_core_reset_event, pub data: [u8;64] }
#[repr(C)] pub struct pds_core_notifyq_header { pub eid: __le64, pub ecode: __le16 }

pub const PDS_DEVNAME_LEN: usize = 32;
#[repr(C)] pub struct pds_core_client_reg_cmd { pub opcode:u8, pub rsvd:[u8;3], pub devname:[std::ffi::c_char;PDS_DEVNAME_LEN], pub vif_type:u8 }
#[repr(C)] pub struct pds_core_client_reg_comp { pub status:u8, pub rsvd:u8, pub comp_index:__le16, pub client_id:__le16, pub rsvd1:[u8;9], pub color:u8 }
#[repr(C)] pub struct pds_core_client_unreg_cmd { pub opcode:u8, pub rsvd:u8, pub client_id:__le16 }
#[repr(C)] pub struct pds_core_client_request_cmd { pub opcode:u8, pub rsvd:u8, pub client_id:__le16, pub client_cmd:[u8;60] }

pub const PDS_CORE_MAX_FRAGS: usize=16;
pub const PDS_CORE_QCQ_F_INITED:u32=BIT(0); pub const PDS_CORE_QCQ_F_SG:u32=BIT(1); pub const PDS_CORE_QCQ_F_INTR:u32=BIT(2); pub const PDS_CORE_QCQ_F_TX_STATS:u32=BIT(3); pub const PDS_CORE_QCQ_F_RX_STATS:u32=BIT(4); pub const PDS_CORE_QCQ_F_NOTIFYQ:u32=BIT(5); pub const PDS_CORE_QCQ_F_CMB_RINGS:u32=BIT(6); pub const PDS_CORE_QCQ_F_CORE:u32=BIT(7);
#[repr(u32)] pub enum pds_core_lif_type { PDS_CORE_LIF_TYPE_DEFAULT=0 }
pub const PDS_CORE_IFNAMSIZ: usize=16;
#[repr(u32)] pub enum pds_core_logical_qtype { PDS_CORE_QTYPE_ADMINQ=0, PDS_CORE_QTYPE_NOTIFYQ=1, PDS_CORE_QTYPE_RXQ=2, PDS_CORE_QTYPE_TXQ=3, PDS_CORE_QTYPE_EQ=4, PDS_CORE_QTYPE_MAX=16 }
#[repr(C, packed)] pub struct pds_core_lif_config_fields { pub state:u8,pub rsvd:[u8;3],pub name:[std::ffi::c_char;16],pub rsvd2:[u8;12],pub features:__le64,pub queue_count:[__le32;16] }
#[repr(C)] pub union pds_core_lif_config { pub fields:pds_core_lif_config_fields, pub words:[__le32;64] }
#[repr(C)] pub struct pds_core_lif_status { pub eid:__le64,pub rsvd:[u8;56] }
#[repr(C)] pub struct pds_core_lif_info { pub config:pds_core_lif_config,pub status:pds_core_lif_status }
#[repr(C)] pub struct pds_core_lif_identity { pub features:__le64,pub version:u8,pub hw_index:u8,pub rsvd:[u8;2],pub max_nb_sessions:__le32,pub rsvd2:[u8;120],pub config:pds_core_lif_config }
#[repr(C)] pub struct pds_core_lif_identify_cmd { pub opcode:u8,pub r#type:u8,pub client_id:__le16,pub ver:u8,pub rsvd:[u8;3],pub ident_pa:__le64 }
#[repr(C)] pub struct pds_core_lif_identify_comp { pub status:u8,pub ver:u8,pub bytes:__le16,pub rsvd:[u8;11],pub color:u8 }
#[repr(C)] pub struct pds_core_lif_init_cmd { pub opcode:u8,pub r#type:u8,pub client_id:__le16,pub rsvd:__le32,pub info_pa:__le64 }
#[repr(C)] pub struct pds_core_lif_init_comp { pub status:u8,pub rsvd:u8,pub hw_index:__le16,pub rsvd1:[u8;11],pub color:u8 }
#[repr(C)] pub struct pds_core_lif_reset_cmd { pub opcode:u8,pub rsvd:u8,pub client_id:__le16 }
#[repr(u32)] pub enum pds_core_lif_attr { PDS_CORE_LIF_ATTR_STATE=0,PDS_CORE_LIF_ATTR_NAME=1,PDS_CORE_LIF_ATTR_FEATURES=4,PDS_CORE_LIF_ATTR_STATS_CTRL=6 }
#[repr(C)] pub union pds_core_lif_setattr_data { pub state:u8,pub name:[std::ffi::c_char;16],pub features:__le64,pub stats_ctl:u8,pub rsvd:[u8;60] }
#[repr(C)] pub struct pds_core_lif_setattr_cmd { pub opcode:u8,pub attr:u8,pub client_id:__le16,pub data:pds_core_lif_setattr_data }
#[repr(C)] pub union pds_core_lif_setattr_comp_data { pub features:__le64,pub rsvd2:[u8;11] }
#[repr(C)] pub struct pds_core_lif_setattr_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub data:pds_core_lif_setattr_comp_data,pub color:u8 }
#[repr(C)] pub struct pds_core_lif_getattr_cmd { pub opcode:u8,pub attr:u8,pub client_id:__le16 }
#[repr(C)] pub union pds_core_lif_getattr_comp_data { pub state:u8,pub features:__le64,pub rsvd2:[u8;11] }
#[repr(C)] pub struct pds_core_lif_getattr_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub data:pds_core_lif_getattr_comp_data,pub color:u8 }

pub const PDS_CORE_QIDENT_F_CQ:u8=0x01;
#[repr(C)] pub struct pds_core_q_identity { pub version:u8,pub supported:u8,pub rsvd:[u8;6],pub features:__le64,pub desc_sz:__le16,pub comp_sz:__le16,pub rsvd2:[u8;6] }
#[repr(C)] pub struct pds_core_q_identify_cmd { pub opcode:u8,pub r#type:u8,pub client_id:__le16,pub ver:u8,pub rsvd:[u8;3],pub ident_pa:__le64 }
#[repr(C)] pub struct pds_core_q_identify_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub ver:u8,pub rsvd1:[u8;10],pub color:u8 }
pub const PDS_CORE_QINIT_F_IRQ:u16=0x01; pub const PDS_CORE_QINIT_F_ENA:u16=0x02; pub const PDS_CORE_QSIZE_MIN_LG2:u8=2; pub const PDS_CORE_QSIZE_MAX_LG2:u8=12;
#[repr(C, packed)] pub struct pds_core_q_init_cmd { pub opcode:u8,pub r#type:u8,pub client_id:__le16,pub ver:u8,pub rsvd:[u8;3],pub index:__le32,pub pid:__le16,pub intr_index:__le16,pub flags:__le16,pub cos:u8,pub ring_size:u8,pub ring_base:__le64,pub cq_ring_base:__le64 }
#[repr(C)] pub struct pds_core_q_init_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub hw_index:__le32,pub hw_type:u8,pub rsvd2:[u8;6],pub color:u8 }

#[repr(u32)] pub enum pds_vdpa_cmd_opcode { PDS_VDPA_CMD_INIT=48,PDS_VDPA_CMD_IDENT=49,PDS_VDPA_CMD_RESET=51,PDS_VDPA_CMD_VQ_RESET=52,PDS_VDPA_CMD_VQ_INIT=53,PDS_VDPA_CMD_STATUS_UPDATE=54,PDS_VDPA_CMD_SET_FEATURES=55,PDS_VDPA_CMD_SET_ATTR=56 }
#[repr(C)] pub struct pds_vdpa_cmd { pub opcode:u8,pub vdpa_index:u8,pub vf_id:__le16 }
pub type pds_vdpa_init_cmd=pds_vdpa_cmd;
#[repr(C)] pub struct pds_vdpa_ident { pub hw_features:__le64,pub max_vqs:__le16,pub max_qlen:__le16,pub min_qlen:__le16 }
#[repr(C)] pub struct pds_vdpa_ident_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16,pub len:__le32,pub ident_pa:__le64 }
#[repr(C)] pub struct pds_vdpa_status_cmd { pub opcode:u8,pub vdpa_index:u8,pub vf_id:__le16,pub status:u8 }
#[repr(u32)] pub enum pds_vdpa_attr { PDS_VDPA_ATTR_MAC=1,PDS_VDPA_ATTR_MAX_VQ_PAIRS=2 }
#[repr(C)] pub union pds_vdpa_setattr_data { pub mac:[u8;6],pub max_vq_pairs:__le16 }
#[repr(C)] pub struct pds_vdpa_setattr_cmd { pub opcode:u8,pub vdpa_index:u8,pub vf_id:__le16,pub attr:u8,pub pad:[u8;3],pub data:pds_vdpa_setattr_data }
#[repr(C)] pub struct pds_vdpa_vq_init_cmd { pub opcode:u8,pub vdpa_index:u8,pub vf_id:__le16,pub qid:__le16,pub len:__le16,pub desc_addr:__le64,pub avail_addr:__le64,pub used_addr:__le64,pub intr_index:__le16,pub avail_index:__le16,pub used_index:__le16 }
#[repr(C)] pub struct pds_vdpa_vq_init_comp { pub status:u8,pub hw_qtype:u8,pub hw_qindex:__le16,pub rsvd:[u8;11],pub color:u8 }
#[repr(C)] pub struct pds_vdpa_vq_reset_cmd { pub opcode:u8,pub vdpa_index:u8,pub vf_id:__le16,pub qid:__le16 }
#[repr(C)] pub struct pds_vdpa_vq_reset_comp { pub status:u8,pub rsvd0:u8,pub avail_index:__le16,pub used_index:__le16,pub rsvd:[u8;9],pub color:u8 }
#[repr(C)] pub struct pds_vdpa_set_features_cmd { pub opcode:u8,pub vdpa_index:u8,pub vf_id:__le16,pub rsvd:__le32,pub features:__le64 }

pub const PDS_LM_DEVICE_STATE_LENGTH:usize=65536;
#[repr(u32)] pub enum pds_lm_cmd_opcode { PDS_LM_CMD_HOST_VF_STATUS=1,PDS_LM_CMD_STATE_SIZE=16,PDS_LM_CMD_SUSPEND=18,PDS_LM_CMD_SUSPEND_STATUS=19,PDS_LM_CMD_RESUME=20,PDS_LM_CMD_SAVE=21,PDS_LM_CMD_RESTORE=22,PDS_LM_CMD_DIRTY_STATUS=32,PDS_LM_CMD_DIRTY_ENABLE=33,PDS_LM_CMD_DIRTY_DISABLE=34,PDS_LM_CMD_DIRTY_READ_SEQ=35,PDS_LM_CMD_DIRTY_WRITE_ACK=36 }
#[repr(C)] pub struct pds_lm_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16,pub rsvd2:[u8;56] }
pub type pds_lm_state_size_cmd=pds_lm_cmd;
#[repr(C)] pub union pds_lm_state_size_comp_data { pub size:__le64,pub rsvd2:[u8;11] }
#[repr(C)] pub struct pds_lm_state_size_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub data:pds_lm_state_size_comp_data,pub color:u8 }
#[repr(u32)] pub enum pds_lm_suspend_resume_type { PDS_LM_SUSPEND_RESUME_TYPE_FULL=0,PDS_LM_SUSPEND_RESUME_TYPE_P2P=1 }
#[repr(C)] pub struct pds_lm_suspend_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16,pub r#type:u8 }
pub type pds_lm_suspend_status_cmd=pds_lm_suspend_cmd; pub type pds_lm_resume_cmd=pds_lm_suspend_cmd;
#[repr(C)] pub struct pds_lm_sg_elem { pub addr:__le64,pub len:__le32,pub rsvd:[__le16;2] }
#[repr(C)] pub struct pds_lm_save_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16,pub rsvd2:[u8;4],pub sgl_addr:__le64,pub num_sge:__le32 }
pub type pds_lm_restore_cmd=pds_lm_save_cmd;
#[repr(C)] pub union pds_lm_dev_state { pub words:[__le32;PDS_LM_DEVICE_STATE_LENGTH/4] }
#[repr(u32)] pub enum pds_lm_host_vf_status { PDS_LM_STA_NONE=0,PDS_LM_STA_IN_PROGRESS=1,PDS_LM_STA_MAX=2 }
#[repr(C)] pub struct pds_lm_dirty_region_info { pub dma_base:__le64,pub page_count:__le32,pub page_size_log2:u8,pub rsvd:[u8;3] }
#[repr(C)] pub struct pds_lm_dirty_status_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16,pub max_regions:u8,pub rsvd2:[u8;3],pub regions_dma:__le64 }
#[repr(u32)] pub enum pds_lm_dirty_bmp_type { PDS_LM_DIRTY_BMP_TYPE_NONE=0,PDS_LM_DIRTY_BMP_TYPE_SEQ_ACK=1 }
#[repr(C)] pub struct pds_lm_dirty_status_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub max_regions:u8,pub num_regions:u8,pub bmp_type:u8,pub rsvd2:u8,pub bmp_type_mask:__le32,pub rsvd3:[u8;3],pub color:u8 }
#[repr(C)] pub struct pds_lm_dirty_enable_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16,pub bmp_type:u8,pub num_regions:u8,pub rsvd2:[u8;2],pub regions_dma:__le64 }
#[repr(C)] pub struct pds_lm_dirty_disable_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16 }
#[repr(C)] pub struct pds_lm_dirty_seq_ack_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16,pub off_bytes:__le32,pub len_bytes:__le32,pub num_sge:__le16,pub rsvd2:[u8;2],pub sgl_addr:__le64 }
#[repr(C)] pub struct pds_lm_host_vf_status_cmd { pub opcode:u8,pub rsvd:u8,pub vf_id:__le16,pub status:u8 }

#[repr(u32)] pub enum pds_fwctl_cmd_opcode { PDS_FWCTL_CMD_IDENT=70,PDS_FWCTL_CMD_RPC=71,PDS_FWCTL_CMD_QUERY=72 }
#[repr(C)] pub struct pds_fwctl_cmd { pub opcode:u8,pub rsvd:[u8;3],pub ep:__le32,pub op:__le32 }
#[repr(C)] pub struct pds_fwctl_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub rsvd2:[u8;11],pub color:u8 }
#[repr(C)] pub struct pds_fwctl_ident_cmd { pub opcode:u8,pub rsvd:u8,pub version:u8,pub rsvd2:u8,pub len:__le32,pub ident_pa:__le64 }
#[repr(C)] pub struct pds_fwctl_ident { pub features:__le64,pub version:u8,pub rsvd:[u8;3],pub max_req_sz:__le32,pub max_resp_sz:__le32,pub max_req_sg_elems:u8,pub max_resp_sg_elems:u8 }
#[repr(u32)] pub enum pds_fwctl_query_entity { PDS_FWCTL_RPC_ROOT=0,PDS_FWCTL_RPC_ENDPOINT=1,PDS_FWCTL_RPC_OPERATION=2 }
pub const PDS_FWCTL_RPC_OPCODE_CMD_SHIFT:u32=0; pub const PDS_FWCTL_RPC_OPCODE_CMD_MASK:u32=GENMASK(15,PDS_FWCTL_RPC_OPCODE_CMD_SHIFT); pub const PDS_FWCTL_RPC_OPCODE_VER_SHIFT:u32=16; pub const PDS_FWCTL_RPC_OPCODE_VER_MASK:u32=GENMASK(23,PDS_FWCTL_RPC_OPCODE_VER_SHIFT);
#[inline] pub fn pds_fwctl_rpc_opcode_get_cmd(op:u32)->u32 { FIELD_GET(PDS_FWCTL_RPC_OPCODE_CMD_MASK,op) } #[inline] pub fn pds_fwctl_rpc_opcode_get_ver(op:u32)->u32 { FIELD_GET(PDS_FWCTL_RPC_OPCODE_VER_MASK,op) }
#[inline] pub fn pds_fwctl_rpc_opcode_cmp(op1:u32,op2:u32)->bool { pds_fwctl_rpc_opcode_get_cmd(op1)==pds_fwctl_rpc_opcode_get_cmd(op2) && pds_fwctl_rpc_opcode_get_ver(op1)<=pds_fwctl_rpc_opcode_get_ver(op2) }
pub const PDSFC_FW_CMD_ATTR_READ:u32=0x00; pub const PDSFC_FW_CMD_ATTR_DEBUG_READ:u32=0x02; pub const PDSFC_FW_CMD_ATTR_WRITE:u32=0x04; pub const PDSFC_FW_CMD_ATTR_DEBUG_WRITE:u32=0x08; pub const PDSFC_FW_CMD_ATTR_SYNC:u32=0x10;
#[repr(C)] pub union pds_fwctl_query_data_target { pub ep:__le32,pub op:__le32 }
#[repr(C)] pub struct pds_fwctl_query_cmd { pub opcode:u8,pub entity:u8,pub version:u8,pub rsvd:u8,pub query_data_buf_len:__le32,pub query_data_buf_pa:__le64,pub target:pds_fwctl_query_data_target }
#[repr(C)] pub struct pds_fwctl_query_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub version:u8,pub rsvd2:[u8;2],pub color:u8 }
#[repr(C)] pub struct pds_fwctl_query_data_endpoint { pub id:__le32 }
#[repr(C)] pub struct pds_fwctl_query_data_operation { pub id:__le32,pub scope:u8,pub rsvd:[u8;3] }
#[repr(C)] pub struct pds_fwctl_query_data { pub version:u8,pub rsvd:[u8;3],pub num_entries:__le32,pub entries:[u8;0] }
pub const PDS_FWCTL_RPC_IND_REQ:__le16=1; pub const PDS_FWCTL_RPC_IND_RESP:__le16=2;
#[repr(C)] pub union pds_fwctl_rpc_req { pub inline_req1:[u8;16],pub indirect:pds_fwctl_rpc_indirect }
#[repr(C)] pub struct pds_fwctl_rpc_indirect { pub req_pa:__le64,pub req_sz:__le32,pub req_sg_elems:u8,pub req_rsvd:[u8;3] }
#[repr(C)] pub union pds_fwctl_rpc_resp { pub inline_req2:[u8;16],pub indirect:pds_fwctl_rpc_indirect_resp }
#[repr(C)] pub struct pds_fwctl_rpc_indirect_resp { pub resp_pa:__le64,pub resp_sz:__le32,pub resp_sg_elems:u8,pub resp_rsvd:[u8;3] }
#[repr(C)] pub struct pds_fwctl_rpc_cmd { pub opcode:u8,pub rsvd:u8,pub flags:__le16,pub ep:__le32,pub op:__le32,pub inline_req0:[u8;16],pub req:pds_fwctl_rpc_req,pub resp:pds_fwctl_rpc_resp }
#[repr(C)] pub struct pds_sg_elem { pub addr:__le64,pub len:__le32,pub rsvd:[u8;4] }
#[repr(C)] pub struct pds_fwctl_rpc_comp { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub err:__le32,pub resp_sz:__le32,pub rsvd2:[u8;3],pub color:u8 }

#[repr(C)] pub union pds_core_adminq_cmd { pub opcode:u8,pub bytes:[u8;64],pub client_reg:pds_core_client_reg_cmd,pub client_unreg:pds_core_client_unreg_cmd,pub client_request:pds_core_client_request_cmd,pub lif_ident:pds_core_lif_identify_cmd,pub lif_init:pds_core_lif_init_cmd,pub lif_reset:pds_core_lif_reset_cmd,pub lif_setattr:pds_core_lif_setattr_cmd,pub lif_getattr:pds_core_lif_getattr_cmd,pub q_ident:pds_core_q_identify_cmd,pub q_init:pds_core_q_init_cmd,pub vdpa:pds_vdpa_cmd,pub vdpa_init:pds_vdpa_init_cmd,pub vdpa_ident:pds_vdpa_ident_cmd,pub vdpa_status:pds_vdpa_status_cmd,pub vdpa_setattr:pds_vdpa_setattr_cmd,pub vdpa_set_features:pds_vdpa_set_features_cmd,pub vdpa_vq_init:pds_vdpa_vq_init_cmd,pub vdpa_vq_reset:pds_vdpa_vq_reset_cmd,pub lm_suspend:pds_lm_suspend_cmd,pub lm_suspend_status:pds_lm_suspend_status_cmd,pub lm_resume:pds_lm_resume_cmd,pub lm_state_size:pds_lm_state_size_cmd,pub lm_save:pds_lm_save_cmd,pub lm_restore:pds_lm_restore_cmd,pub lm_host_vf_status:pds_lm_host_vf_status_cmd,pub lm_dirty_status:pds_lm_dirty_status_cmd,pub lm_dirty_enable:pds_lm_dirty_enable_cmd,pub lm_dirty_disable:pds_lm_dirty_disable_cmd,pub lm_dirty_seq_ack:pds_lm_dirty_seq_ack_cmd,pub fwctl:pds_fwctl_cmd,pub fwctl_ident:pds_fwctl_ident_cmd,pub fwctl_rpc:pds_fwctl_rpc_cmd,pub fwctl_query:pds_fwctl_query_cmd }
#[repr(C)] pub struct pds_core_adminq_comp_header { pub status:u8,pub rsvd:u8,pub comp_index:__le16,pub rsvd2:[u8;11],pub color:u8 }
#[repr(C)] pub union pds_core_adminq_comp { pub header:pds_core_adminq_comp_header,pub words:[u32;4],pub client_reg:pds_core_client_reg_comp,pub lif_ident:pds_core_lif_identify_comp,pub lif_init:pds_core_lif_init_comp,pub lif_setattr:pds_core_lif_setattr_comp,pub lif_getattr:pds_core_lif_getattr_comp,pub q_ident:pds_core_q_identify_comp,pub q_init:pds_core_q_init_comp,pub vdpa_vq_init:pds_vdpa_vq_init_comp,pub vdpa_vq_reset:pds_vdpa_vq_reset_comp,pub lm_state_size:pds_lm_state_size_comp,pub lm_dirty_status:pds_lm_dirty_status_comp,pub fwctl:pds_fwctl_comp,pub fwctl_rpc:pds_fwctl_rpc_comp,pub fwctl_query:pds_fwctl_query_comp }

pub fn pdsc_color_match(color:u8, done_color:bool)->bool { ((color & PDS_COMP_COLOR_MASK)!=0)==done_color }
#[repr(C)] pub struct pdsc { _private: [u8;0] }
extern "C" { pub fn pdsc_adminq_post(pdsc:*mut pdsc, cmd:*mut pds_core_adminq_cmd, comp:*mut pds_core_adminq_comp, fast_poll:bool) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
