/* SPDX-License-Identifier: (GPL-2.0 OR Linux-OpenIB) OR BSD-2-Clause */
/* Copyright(c) 2023 Advanced Micro Devices, Inc. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;

pub const PCI_VENDOR_ID_PENSANDO: u32 = 0x1dd8;
pub const PCI_DEVICE_ID_PENSANDO_CORE_PF: u32 = 0x100c;
pub const PCI_DEVICE_ID_VIRTIO_NET_TRANS: u32 = 0x1000;
pub const PCI_DEVICE_ID_PENSANDO_IONIC_ETH_VF: u32 = 0x1003;
pub const PCI_DEVICE_ID_PENSANDO_VDPA_VF: u32 = 0x100b;
pub const PDS_CORE_BARS_MAX: u32 = 4;
pub const PDS_CORE_PCI_BAR_DBELL: u32 = 1;
pub const PDS_CORE_DEV_INFO_SIGNATURE: u32 = 0x44455649;
pub const PDS_CORE_BAR0_SIZE: u32 = 0x8000;
pub const PDS_CORE_BAR0_DEV_INFO_REGS_OFFSET: u32 = 0;
pub const PDS_CORE_BAR0_DEV_CMD_REGS_OFFSET: u32 = 0x0800;
pub const PDS_CORE_BAR0_DEV_CMD_DATA_REGS_OFFSET: u32 = 0x0c00;
pub const PDS_CORE_BAR0_INTR_STATUS_OFFSET: u32 = 0x1000;
pub const PDS_CORE_BAR0_INTR_CTRL_OFFSET: u32 = 0x2000;
pub const PDS_CORE_DEV_CMD_DONE: u32 = 1;
pub const PDS_CORE_DEVCMD_TIMEOUT: u32 = 5;
pub const PDS_CORE_CLIENT_ID: u32 = 0;
pub const PDS_CORE_ASIC_TYPE_CAPRI: u32 = 0;

#[repr(u32)] pub enum pds_core_cmd_opcode { PDS_CORE_CMD_NOP=0, PDS_CORE_CMD_IDENTIFY=1, PDS_CORE_CMD_RESET=2, PDS_CORE_CMD_INIT=3, PDS_CORE_CMD_FW_DOWNLOAD=4, PDS_CORE_CMD_FW_CONTROL=5, PDS_CORE_CMD_GET_COMPONENT_INFO=6, PDS_CORE_CMD_SEND_PKG_DATA=7, PDS_CORE_CMD_SEND_COMPONENT_TBL=8, PDS_CORE_CMD_SEND_COMPONENT=9, PDS_CORE_CMD_FINALIZE_UPDATE=10, PDS_CORE_CMD_MATCH_RECORD_DESC=11, PDS_CORE_CMD_HOST_MEM=12, PDS_CORE_CMD_VF_GETATTR=60, PDS_CORE_CMD_VF_SETATTR=61, PDS_CORE_CMD_VF_CTRL=62, PDS_CORE_CMD_MAX, PDS_CORE_CMD_COUNT }
#[repr(u8)] pub enum pds_core_status_code { PDS_RC_SUCCESS=0, PDS_RC_EVERSION=1, PDS_RC_EOPCODE=2, PDS_RC_EIO=3, PDS_RC_EPERM=4, PDS_RC_EQID=5, PDS_RC_EQTYPE=6, PDS_RC_ENOENT=7, PDS_RC_EINTR=8, PDS_RC_EAGAIN=9, PDS_RC_ENOMEM=10, PDS_RC_EFAULT=11, PDS_RC_EBUSY=12, PDS_RC_EEXIST=13, PDS_RC_EINVAL=14, PDS_RC_ENOSPC=15, PDS_RC_ERANGE=16, PDS_RC_BAD_ADDR=17, PDS_RC_DEV_CMD=18, PDS_RC_ENOSUPP=19, PDS_RC_ERROR=29, PDS_RC_ERDMA=30, PDS_RC_EVFID=31, PDS_RC_BAD_FW=32, PDS_RC_ECLIENT=33, PDS_RC_BAD_PCI=255 }

#[repr(C)] pub struct pds_core_drv_identity { pub drv_type:__le32, pub os_dist:__le32, pub os_dist_str:[i8;128], pub kernel_ver:__le32, pub kernel_ver_str:[i8;32], pub driver_ver_str:[i8;32] }
#[repr(u64)] pub enum pds_core_dev_capability { PDS_CORE_DEV_CAP_PLDM_FW_UPDATE=1, PDS_CORE_DEV_CAP_HOST_MEM=2 }
pub const PDS_DEV_TYPE_MAX: usize = 16;
#[repr(C)] pub struct pds_core_dev_identity { pub version:u8,pub type_:u8,pub state:u8,pub rsvd:u8,pub nlifs:__le32,pub nintrs:__le32,pub ndbpgs_per_lif:__le32,pub intr_coal_mult:__le32,pub intr_coal_div:__le32,pub vif_types:[__le16;16],pub max_fw_slots:__le16,pub rsvd2:[u8;6],pub capabilities:__le64 }
pub const PDS_CORE_IDENTITY_VERSION_1:u32=1; pub const PDS_CORE_IDENTITY_VERSION_2:u32=2;

#[repr(C)] pub struct pds_core_dev_identify_cmd {pub opcode:u8,pub ver:u8}
#[repr(C)] pub struct pds_core_dev_identify_comp {pub status:u8,pub ver:u8}
#[repr(C)] pub struct pds_core_dev_reset_cmd {pub opcode:u8}
#[repr(C)] pub struct pds_core_dev_reset_comp {pub status:u8}
#[repr(C)] pub struct pds_core_dev_init_data_in {pub adminq_q_base:__le64,pub adminq_cq_base:__le64,pub notifyq_cq_base:__le64,pub flags:__le32,pub intr_index:__le16,pub adminq_ring_size:u8,pub notifyq_ring_size:u8}
#[repr(C)] pub struct pds_core_dev_init_data_out {pub core_hw_index:__le32,pub adminq_hw_index:__le32,pub notifyq_hw_index:__le32,pub adminq_hw_type:u8,pub notifyq_hw_type:u8}
#[repr(C)] pub struct pds_core_dev_init_cmd {pub opcode:u8}
#[repr(C)] pub struct pds_core_dev_init_comp {pub status:u8}
#[repr(C)] pub struct pds_core_fw_download_cmd {pub opcode:u8,pub rsvd:[u8;3],pub offset:__le32,pub addr:__le64,pub length:__le32}
#[repr(C)] pub struct pds_core_fw_download_comp {pub status:u8}
#[repr(u8)] pub enum pds_core_fw_control_oper {PDS_CORE_FW_INSTALL_ASYNC=0,PDS_CORE_FW_INSTALL_STATUS=1,PDS_CORE_FW_ACTIVATE_ASYNC=2,PDS_CORE_FW_ACTIVATE_STATUS=3,PDS_CORE_FW_UPDATE_CLEANUP=4,PDS_CORE_FW_GET_BOOT=5,PDS_CORE_FW_SET_BOOT=6,PDS_CORE_FW_GET_LIST=7}
#[repr(u8)] pub enum pds_core_fw_slot {PDS_CORE_FW_SLOT_INVALID=0,PDS_CORE_FW_SLOT_A=1,PDS_CORE_FW_SLOT_B=2,PDS_CORE_FW_SLOT_GOLD=3,PDS_CORE_FW_SLOT_MAX=255}
#[repr(C)] pub struct pds_core_fw_control_cmd {pub opcode:u8,pub rsvd:[u8;3],pub oper:u8,pub slot:u8}
#[repr(C)] pub struct pds_core_fw_control_comp {pub status:u8,pub rsvd:[u8;3],pub slot:u8,pub rsvd1:[u8;10],pub color:u8}
pub const PDS_CORE_FWSLOT_BUFLEN:usize=8; pub const PDS_CORE_FWVERS_BUFLEN:usize=32;
#[repr(C)] pub struct pds_core_fw_name_info {pub slotname:[i8;8],pub fw_version:[i8;32]}
pub const PDS_CORE_FWVERS_LIST_LEN:usize=16;
#[repr(C,packed)] pub struct pds_core_fw_list_info {pub num_fw_slots:u8,pub fw_names:[pds_core_fw_name_info;16]}
#[repr(u8)] pub enum pds_core_vf_attr {PDS_CORE_VF_ATTR_SPOOFCHK=1,PDS_CORE_VF_ATTR_TRUST=2,PDS_CORE_VF_ATTR_MAC=3,PDS_CORE_VF_ATTR_LINKSTATE=4,PDS_CORE_VF_ATTR_VLAN=5,PDS_CORE_VF_ATTR_RATE=6,PDS_CORE_VF_ATTR_STATSADDR=7}
#[repr(u8)] pub enum pds_core_vf_link_status {PDS_CORE_VF_LINK_STATUS_AUTO=0,PDS_CORE_VF_LINK_STATUS_UP=1,PDS_CORE_VF_LINK_STATUS_DOWN=2}
#[repr(C,packed)] pub struct pds_core_vf_stats {pub pa:__le64,pub len:__le32}
#[repr(C)] pub union pds_core_vf_setattr_data {pub macaddr:[u8;6],pub vlanid:__le16,pub maxrate:__le32,pub spoofchk:u8,pub trust:u8,pub linkstate:u8,pub stats:pds_core_vf_stats,pub pad:[u8;60]}
#[repr(C)] pub struct pds_core_vf_setattr_cmd {pub opcode:u8,pub attr:u8,pub vf_index:__le16,pub data:pds_core_vf_setattr_data}
#[repr(C)] pub struct pds_core_vf_setattr_comp {pub status:u8,pub attr:u8,pub vf_index:__le16,pub comp_index:__le16,pub rsvd:[u8;9],pub color:u8}
#[repr(C)] pub struct pds_core_vf_getattr_cmd {pub opcode:u8,pub attr:u8,pub vf_index:__le16}
#[repr(C)] pub union pds_core_vf_getattr_data {pub macaddr:[u8;6],pub vlanid:__le16,pub maxrate:__le32,pub spoofchk:u8,pub trust:u8,pub linkstate:u8,pub stats_pa:__le64,pub pad:[u8;11]}
#[repr(C)] pub struct pds_core_vf_getattr_comp {pub status:u8,pub attr:u8,pub vf_index:__le16,pub data:pds_core_vf_getattr_data,pub color:u8}
#[repr(u8)] pub enum pds_core_vf_ctrl_opcode {PDS_CORE_VF_CTRL_START_ALL=0,PDS_CORE_VF_CTRL_START=1}
#[repr(C)] pub struct pds_core_vf_ctrl_cmd {pub opcode:u8,pub ctrl_opcode:u8,pub vf_index:__le16}
#[repr(C)] pub struct pds_core_vf_ctrl_comp {pub status:u8}

#[repr(C)] pub struct pds_core_send_pkg_data_cmd {pub opcode:u8,pub ver:u8,pub total_len:__le16,pub offset:__le16,pub data_len:__le16,pub data_pa:__le64}
#[repr(C)] pub struct pds_core_send_pkg_data_comp {pub status:u8,pub ver:u8,pub rsvd:[u8;2]}
#[repr(C)] pub struct pds_core_component_tbl {pub comparison_stamp:__le32,pub classification:__le16,pub identifier:__le16,pub transfer_flag:u8,pub version_str_type:u8,pub version_str_len:u8,pub version_str:[u8;0]}
#[repr(C)] pub struct pds_core_send_component_tbl_cmd {pub opcode:u8,pub ver:u8,pub slot_id:u8,pub rsvd:u8}
#[repr(u8)] pub enum pds_core_component_resp_code {PDS_CORE_COMPONENT_VALID=0,PDS_CORE_COMPONENT_STAMP_IDENTICAL=1,PDS_CORE_COMPONENT_STAMP_LOWER=2,PDS_CORE_COMPONENT_STAMP_OR_VERSION_INVALID=3,PDS_CORE_COMPONENT_CONFLICT=4,PDS_CORE_COMPONENT_PREREQS_NOT_MET=5,PDS_CORE_COMPONENT_NOT_SUPPORTED=6,PDS_CORE_COMPONENT_FW_TYPE_INVALID=0xd0}
#[repr(C)] pub struct pds_core_send_component_tbl_comp {pub status:u8,pub ver:u8,pub completion_code:u8,pub response:u8,pub response_code:u8,pub slot_id:u8,pub rsvd:[u8;2]}
#[repr(u8)] pub enum pds_core_send_component_op {PDS_CORE_SEND_COMPONENT_START=0,PDS_CORE_SEND_COMPONENT_STATUS=1}
pub const PDS_CORE_FW_COMPONENT_ID_INVALID:u16=0xffff;
#[repr(C)] pub struct pds_core_flash_component {pub comparison_stamp:__le32,pub image_size:__le32,pub classification:__le16,pub identifier:__le16,pub options:__le16,pub rsvd:[u8;3],pub version_str_type:u8,pub version_str_len:u8,pub version_str:[u8;0]}
#[repr(C)] pub struct pds_core_send_component_cmd {pub opcode:u8,pub ver:u8,pub slot_id:u8,pub operation:u8,pub offset:__le32,pub data_len:__le32,pub rsvd:[u8;4],pub data_pa:__le64}
#[repr(C)] pub struct pds_core_send_component_comp {pub status:u8,pub ver:u8,pub completion_code:u8,pub compat_response:u8,pub compat_response_code:u8,pub rsvd:[u8;3]}
#[repr(u8)] pub enum pds_core_fw_component_type {PDS_CORE_FW_TYPE_UNKNOWN=0,PDS_CORE_FW_TYPE_MAIN=1,PDS_CORE_FW_TYPE_BOOT=2,PDS_CORE_FW_TYPE_CPLD=3,PDS_CORE_FW_TYPE_SECURE=4,PDS_CORE_FW_TYPE_FPGA=5,PDS_CORE_FW_TYPE_SUC_MAIN=6,PDS_CORE_FW_TYPE_SUC_BOOT=7,PDS_CORE_FW_TYPE_UBOOT=8}
#[repr(u16)] pub enum pds_core_component_info_flags {PDS_CORE_FW_COMPONENT_INFO_F_RUNNING=1,PDS_CORE_FW_COMPONENT_INFO_F_STARTUP=2,PDS_CORE_FW_COMPONENT_INFO_F_FIXED=4,PDS_CORE_FW_COMPONENT_INFO_F_UPDATE_BY_NAME=8}
#[repr(C)] pub struct pds_core_fw_component_info {pub name:[i8;24],pub component_type:u8,pub rsvd:[u8;3],pub flags:__le16,pub identifier:u8,pub slot_id:u8,pub version:[i8;32]}
pub const PDS_CORE_FW_COMPONENT_NAME_BUFLEN:usize=24; pub const PDS_CORE_FW_COMPONENT_VER_BUFLEN:usize=32;
#[repr(C)] pub struct pds_core_component_list_info {pub num_components:u8,pub rsvd:[u8;7],pub info:[pds_core_fw_component_info;0]}
#[repr(C)] pub struct pds_core_get_component_info_cmd {pub opcode:u8,pub ver:u8,pub data_len:__le16,pub rsvd:[u8;4],pub data_pa:__le64}
#[repr(C)] pub struct pds_core_get_component_info_comp {pub status:u8,pub ver:u8,pub rsvd:[u8;2]}
#[repr(C)] pub struct pds_core_finalize_update_cmd {pub opcode:u8,pub ver:u8,pub rsvd:[u8;2]}
#[repr(C)] pub struct pds_core_finalize_update_comp {pub status:u8,pub ver:u8,pub rsvd:[u8;2]}
#[repr(C)] pub struct pds_core_match_record_desc_cmd {pub opcode:u8,pub ver:u8,pub type_:__le16,pub size:__le16,pub rsvd:[u8;2]}
#[repr(C)] pub struct pds_core_match_record_desc_comp {pub status:u8,pub ver:u8,pub match_:u8,pub rsvd:u8}
#[repr(u8)] pub enum pds_core_host_mem_oper {PDS_CORE_HOST_MEM_GET_COUNT=0,PDS_CORE_HOST_MEM_QUERY=1,PDS_CORE_HOST_MEM_ADD=2,PDS_CORE_HOST_MEM_DEL=3}
#[repr(C)] pub struct pds_core_host_mem_cmd {pub opcode:u8,pub oper:u8,pub index:__le16,pub tag:__le16,pub reason:u8,pub rsvd:u8,pub max_contig:__le32,pub size:__le32,pub buf_pa:__le64}
#[repr(C)] pub struct pds_core_host_mem_comp {pub status:u8,pub oper:u8,pub count:__le16,pub size:__le32,pub tag:__le16,pub rsvd:[u8;6]}

#[repr(C)] pub union pds_core_dev_cmd {pub opcode:u8,pub words:[u32;16],pub identify:pds_core_dev_identify_cmd,pub init:pds_core_dev_init_cmd,pub reset:pds_core_dev_reset_cmd,pub fw_download:pds_core_fw_download_cmd,pub fw_control:pds_core_fw_control_cmd,pub vf_setattr:pds_core_vf_setattr_cmd,pub vf_getattr:pds_core_vf_getattr_cmd,pub vf_ctrl:pds_core_vf_ctrl_cmd,pub get_component_info:pds_core_get_component_info_cmd,pub send_pkg_data:pds_core_send_pkg_data_cmd,pub send_component_tbl:pds_core_send_component_tbl_cmd,pub send_component:pds_core_send_component_cmd,pub finalize_update:pds_core_finalize_update_cmd,pub match_record_desc:pds_core_match_record_desc_cmd,pub host_mem:pds_core_host_mem_cmd}
#[repr(C)] pub union pds_core_dev_comp {pub status:u8,pub bytes:[u8;16],pub identify:pds_core_dev_identify_comp,pub reset:pds_core_dev_reset_comp,pub init:pds_core_dev_init_comp,pub fw_download:pds_core_fw_download_comp,pub fw_control:pds_core_fw_control_comp,pub vf_setattr:pds_core_vf_setattr_comp,pub vf_getattr:pds_core_vf_getattr_comp,pub vf_ctrl:pds_core_vf_ctrl_comp,pub get_component_info:pds_core_get_component_info_comp,pub send_pkg_data:pds_core_send_pkg_data_comp,pub send_component_tbl:pds_core_send_component_tbl_comp,pub send_component:pds_core_send_component_comp,pub finalize_update:pds_core_finalize_update_comp,pub match_record_desc:pds_core_match_record_desc_comp,pub host_mem:pds_core_host_mem_comp}
#[repr(C)] pub struct pds_core_dev_hwstamp_regs {pub tick_low:u32,pub tick_high:u32}
#[repr(C,packed)] pub struct pds_core_dev_info_regs {pub signature:u32,pub version:u8,pub asic_type:u8,pub asic_rev:u8,pub fw_status:u8,pub fw_heartbeat:__le32,pub fw_version:[i8;32],pub serial_num:[i8;32],pub oprom_regs:[u8;32],pub rsvd_pad1024:[u8;916],pub hwstamp:pds_core_dev_hwstamp_regs,pub rsvd_pad2048:[u8;1016]}
#[repr(C,packed)] pub struct pds_core_dev_cmd_regs {pub doorbell:u32,pub done:u32,pub cmd:pds_core_dev_cmd,pub comp:pds_core_dev_comp,pub rsvd:[u8;48],pub data:[u32;478]}
#[repr(C,packed)] pub struct pds_core_dev_regs {pub info:pds_core_dev_info_regs,pub devcmd:pds_core_dev_cmd_regs}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
