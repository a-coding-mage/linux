/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of ufshci.h. External UFS types are supplied by dependencies. */

pub const TASK_REQ_UPIU_SIZE_DWORDS: u32 = 8;
pub const TASK_RSP_UPIU_SIZE_DWORDS: u32 = 8;
pub const ALIGNED_UPIU_SIZE: usize = 512;
pub const ALIGNED_DEVMAN_RSP_SIZE: usize = 4096;

pub const REG_CONTROLLER_CAPABILITIES: u32 = 0x00;
pub const REG_MCQCAP: u32 = 0x04;
pub const REG_UFS_VERSION: u32 = 0x08;
pub const REG_EXT_CONTROLLER_CAPABILITIES: u32 = 0x0c;
pub const REG_CONTROLLER_PID: u32 = 0x10;
pub const REG_CONTROLLER_MID: u32 = 0x14;
pub const REG_AUTO_HIBERNATE_IDLE_TIMER: u32 = 0x18;
pub const REG_INTERRUPT_STATUS: u32 = 0x20;
pub const REG_INTERRUPT_ENABLE: u32 = 0x24;
pub const REG_CONTROLLER_STATUS: u32 = 0x30;
pub const REG_CONTROLLER_ENABLE: u32 = 0x34;
pub const REG_UIC_ERROR_CODE_PHY_ADAPTER_LAYER: u32 = 0x38;
pub const REG_UIC_ERROR_CODE_DATA_LINK_LAYER: u32 = 0x3c;
pub const REG_UIC_ERROR_CODE_NETWORK_LAYER: u32 = 0x40;
pub const REG_UIC_ERROR_CODE_TRANSPORT_LAYER: u32 = 0x44;
pub const REG_UIC_ERROR_CODE_DME: u32 = 0x48;
pub const REG_UTP_TRANSFER_REQ_INT_AGG_CONTROL: u32 = 0x4c;
pub const REG_UTP_TRANSFER_REQ_LIST_BASE_L: u32 = 0x50;
pub const REG_UTP_TRANSFER_REQ_LIST_BASE_H: u32 = 0x54;
pub const REG_UTP_TRANSFER_REQ_DOOR_BELL: u32 = 0x58;
pub const REG_UTP_TRANSFER_REQ_LIST_CLEAR: u32 = 0x5c;
pub const REG_UTP_TRANSFER_REQ_LIST_RUN_STOP: u32 = 0x60;
pub const REG_UTP_TASK_REQ_LIST_BASE_L: u32 = 0x70;
pub const REG_UTP_TASK_REQ_LIST_BASE_H: u32 = 0x74;
pub const REG_UTP_TASK_REQ_DOOR_BELL: u32 = 0x78;
pub const REG_UTP_TASK_REQ_LIST_CLEAR: u32 = 0x7c;
pub const REG_UTP_TASK_REQ_LIST_RUN_STOP: u32 = 0x80;
pub const REG_UIC_COMMAND: u32 = 0x90;
pub const REG_UIC_COMMAND_ARG_1: u32 = 0x94;
pub const REG_UIC_COMMAND_ARG_2: u32 = 0x98;
pub const REG_UIC_COMMAND_ARG_3: u32 = 0x9c;
pub const UFSHCI_REG_SPACE_SIZE: u32 = 0xa0;
pub const REG_UFS_CCAP: u32 = 0x100;
pub const REG_UFS_CRYPTOCAP: u32 = 0x104;
pub const REG_UFS_MEM_CFG: u32 = 0x300;
pub const REG_UFS_MCQ_CFG: u32 = 0x380;
pub const REG_UFS_ESILBA: u32 = 0x384;
pub const REG_UFS_ESIUBA: u32 = 0x388;
pub const UFSHCI_CRYPTO_REG_SPACE_SIZE: u32 = 0x400;

pub const MASK_TRANSFER_REQUESTS_SLOTS_SDB: u32 = 0x1f;
pub const MASK_TRANSFER_REQUESTS_SLOTS_MCQ: u32 = 0xff;
pub const MASK_NUMBER_OUTSTANDING_RTT: u32 = 0xff00;
pub const MASK_TASK_MANAGEMENT_REQUEST_SLOTS: u32 = 0x70000;
pub const MASK_EHSLUTRD_SUPPORTED: u32 = 0x400000;
pub const MASK_AUTO_HIBERN8_SUPPORT: u32 = 0x800000;
pub const MASK_64_ADDRESSING_SUPPORT: u32 = 0x1000000;
pub const MASK_OUT_OF_ORDER_DATA_DELIVERY_SUPPORT: u32 = 0x2000000;
pub const MASK_UIC_DME_TEST_MODE_SUPPORT: u32 = 0x4000000;
pub const MASK_CRYPTO_SUPPORT: u32 = 0x10000000;
pub const MASK_LSDB_SUPPORT: u32 = 0x20000000;
pub const MASK_MCQ_SUPPORT: u32 = 0x40000000;
pub const REG_SQATTR:u32=0; pub const REG_SQLBA:u32=4; pub const REG_SQUBA:u32=8; pub const REG_SQDAO:u32=0xc; pub const REG_SQISAO:u32=0x10;
pub const REG_CQATTR:u32=0x20; pub const REG_CQLBA:u32=0x24; pub const REG_CQUBA:u32=0x28; pub const REG_CQDAO:u32=0x2c; pub const REG_CQISAO:u32=0x30;
pub const REG_SQHP:u32=0; pub const REG_SQTP:u32=4; pub const REG_SQRTC:u32=8; pub const REG_SQCTI:u32=0xc; pub const REG_SQRTS:u32=0x10; pub const REG_CQHP:u32=0; pub const REG_CQTP:u32=4; pub const REG_CQIS:u32=0; pub const REG_CQIE:u32=4; pub const REG_MCQIACR:u32=8;
pub const SQ_START:u32=0; pub const SQ_STOP:u32=1; pub const SQ_ICU:u32=2; pub const SQ_STS:u32=1; pub const SQ_CUS:u32=2;

pub const SQ_ICU_ERR_CODE_MASK: u32 = 0xf0;
pub const fn ufs_mask(mask: u32, offset: u32) -> u32 { mask << offset }
pub const MINOR_VERSION_NUM_MASK: u32 = ufs_mask(0xffff, 0);
pub const MAJOR_VERSION_NUM_MASK: u32 = ufs_mask(0xffff, 16);
pub const UFSHCD_NUM_RESERVED: u32 = 1;
pub const fn ufshci_version(major: u32, minor: u32) -> u32 { (major << 8).wrapping_add(minor << 4) }

pub const DEVICE_CLASS: u32 = ufs_mask(0xffff, 0);
pub const DEVICE_ID: u32 = ufs_mask(0xff, 24);
pub const MANUFACTURE_ID_MASK: u32 = ufs_mask(0xffff, 0);
pub const PRODUCT_ID_MASK: u32 = ufs_mask(0xffff, 16);
pub const UFSHCI_AHIBERN8_TIMER_MASK: u32 = 0x3ff;
pub const UFSHCI_AHIBERN8_SCALE_MASK: u32 = 0x1c00;
pub const UFSHCI_AHIBERN8_SCALE_FACTOR: u32 = 10;
pub const UFSHCI_AHIBERN8_MAX: u32 = 1023 * 100000;

pub const UTP_TRANSFER_REQ_COMPL:u32=0x1; pub const UIC_DME_END_PT_RESET:u32=0x2; pub const UIC_ERROR:u32=0x4; pub const UIC_TEST_MODE:u32=0x8; pub const UIC_POWER_MODE:u32=0x10; pub const UIC_HIBERNATE_EXIT:u32=0x20; pub const UIC_HIBERNATE_ENTER:u32=0x40; pub const UIC_LINK_LOST:u32=0x80; pub const UIC_LINK_STARTUP:u32=0x100; pub const UTP_TASK_REQ_COMPL:u32=0x200; pub const UIC_COMMAND_COMPL:u32=0x400; pub const DEVICE_FATAL_ERROR:u32=0x800; pub const UTP_ERROR:u32=0x1000; pub const CONTROLLER_FATAL_ERROR:u32=0x10000; pub const SYSTEM_BUS_FATAL_ERROR:u32=0x20000; pub const CRYPTO_ENGINE_FATAL_ERROR:u32=0x40000; pub const MCQ_CQ_EVENT_STATUS:u32=0x100000; pub const MCQ_IAG_EVENT_STATUS:u32=0x200000;
pub const INT_FATAL_ERRORS:u32=DEVICE_FATAL_ERROR|CONTROLLER_FATAL_ERROR|SYSTEM_BUS_FATAL_ERROR|CRYPTO_ENGINE_FATAL_ERROR|UIC_LINK_LOST|UTP_ERROR;
pub const UFSHCD_UIC_HIBERN8_MASK:u32=UIC_HIBERNATE_ENTER|UIC_HIBERNATE_EXIT; pub const UFSHCD_UIC_PWR_MASK:u32=UFSHCD_UIC_HIBERN8_MASK|UIC_POWER_MODE; pub const UFSHCD_UIC_MASK:u32=UIC_COMMAND_COMPL|UFSHCD_UIC_PWR_MASK; pub const UFSHCD_ERROR_MASK:u32=UIC_ERROR|INT_FATAL_ERRORS;
pub const DEVICE_PRESENT:u32=1; pub const UTP_TRANSFER_REQ_LIST_READY:u32=2; pub const UTP_TASK_REQ_LIST_READY:u32=4; pub const UIC_COMMAND_READY:u32=8; pub const HOST_ERROR_INDICATOR:u32=0x10; pub const DEVICE_ERROR_INDICATOR:u32=0x20; pub const UIC_POWER_MODE_CHANGE_REQ_STATUS_MASK:u32=ufs_mask(7,8); pub const UFSHCD_STATUS_READY:u32=UTP_TRANSFER_REQ_LIST_READY|UTP_TASK_REQ_LIST_READY|UIC_COMMAND_READY;
pub const PWR_OK:u32=0; pub const PWR_LOCAL:u32=1; pub const PWR_REMOTE:u32=2; pub const PWR_BUSY:u32=3; pub const PWR_ERROR_CAP:u32=4; pub const PWR_FATAL_ERROR:u32=5;
pub const CONTROLLER_ENABLE:u32=1; pub const CONTROLLER_DISABLE:u32=0; pub const CRYPTO_GENERAL_ENABLE:u32=2;

pub const UIC_PHY_ADAPTER_LAYER_ERROR:u32=0x80000000; pub const UIC_PHY_ADAPTER_LAYER_ERROR_CODE_MASK:u32=0x1f; pub const UIC_PHY_ADAPTER_LAYER_LANE_ERR_MASK:u32=0xf; pub const UIC_PHY_ADAPTER_LAYER_GENERIC_ERROR:u32=0x10;
pub const UIC_DATA_LINK_LAYER_ERROR:u32=0x80000000; pub const UIC_DATA_LINK_LAYER_ERROR_CODE_MASK:u32=0xffff; pub const UIC_DATA_LINK_LAYER_ERROR_TCX_REP_TIMER_EXP:u32=2; pub const UIC_DATA_LINK_LAYER_ERROR_AFCX_REQ_TIMER_EXP:u32=4; pub const UIC_DATA_LINK_LAYER_ERROR_FCX_PRO_TIMER_EXP:u32=8; pub const UIC_DATA_LINK_LAYER_ERROR_RX_BUF_OF:u32=0x20; pub const UIC_DATA_LINK_LAYER_ERROR_PA_INIT:u32=0x2000; pub const UIC_DATA_LINK_LAYER_ERROR_NAC_RECEIVED:u32=1; pub const UIC_DATA_LINK_LAYER_ERROR_TCx_REPLAY_TIMEOUT:u32=2;
pub const UIC_NETWORK_LAYER_ERROR:u32=0x80000000; pub const UIC_NETWORK_LAYER_ERROR_CODE_MASK:u32=7; pub const UIC_NETWORK_UNSUPPORTED_HEADER_TYPE:u32=1; pub const UIC_NETWORK_BAD_DEVICEID_ENC:u32=2; pub const UIC_NETWORK_LHDR_TRAP_PACKET_DROPPING:u32=4;
pub const UIC_TRANSPORT_LAYER_ERROR:u32=0x80000000; pub const UIC_TRANSPORT_LAYER_ERROR_CODE_MASK:u32=0x7f; pub const UIC_TRANSPORT_UNSUPPORTED_HEADER_TYPE:u32=1; pub const UIC_TRANSPORT_UNKNOWN_CPORTID:u32=2; pub const UIC_TRANSPORT_NO_CONNECTION_RX:u32=4; pub const UIC_TRANSPORT_CONTROLLED_SEGMENT_DROPPING:u32=8; pub const UIC_TRANSPORT_BAD_TC:u32=0x10; pub const UIC_TRANSPORT_E2E_CREDIT_OVERFOW:u32=0x20; pub const UIC_TRANSPORT_SAFETY_VALUE_DROPPING:u32=0x40;
pub const UIC_DME_ERROR:u32=0x80000000; pub const UIC_DME_ERROR_CODE_MASK:u32=1; pub const UIC_DME_QOS_MASK:u32=0xe;
pub const INT_AGGR_TIMEOUT_VAL_MASK:u32=0xff; pub const INT_AGGR_COUNTER_THRESHOLD_MASK:u32=ufs_mask(0x1f,8); pub const INT_AGGR_COUNTER_AND_TIMER_RESET:u32=0x10000; pub const INT_AGGR_STATUS_BIT:u32=0x100000; pub const INT_AGGR_PARAM_WRITE:u32=0x1000000; pub const INT_AGGR_ENABLE:u32=0x80000000;
pub const UTP_TRANSFER_REQ_LIST_RUN_STOP_BIT:u32=1; pub const UTP_TASK_REQ_LIST_RUN_STOP_BIT:u32=1; pub const MCQ_MODE_SELECT:u32=1; pub const ESI_ENABLE:u32=2; pub const UFSHCD_MCQ_CQIS_TAIL_ENT_PUSH_STS:u32=1;
pub const COMMAND_OPCODE_MASK:u32=0xff; pub const GEN_SELECTOR_INDEX_MASK:u32=0xffff; pub const MIB_ATTRIBUTE_MASK:u32=ufs_mask(0xffff,16); pub const RESET_LEVEL:u32=0xff; pub const ATTR_SET_TYPE_MASK:u32=ufs_mask(0xff,16); pub const CONFIG_RESULT_CODE_MASK:u32=0xff; pub const GENERIC_ERROR_CODE_MASK:u32=0xff;
pub const fn uic_arg_mphy_tx_gen_sel_index(lane:u32)->u32 { lane }
pub const fn uic_arg_mphy_rx_gen_sel_index(lane:u32)->u32 { PA_MAXDATALANES + lane }
pub const fn uic_arg_mib_sel(attr:u32,sel:u32)->u32 { ((attr&0xffff)<<16)|(sel&0xffff) } pub const fn uic_arg_mib(attr:u32)->u32 { uic_arg_mib_sel(attr,0) } pub const fn uic_arg_attr_type(t:u32)->u32 { (t&0xff)<<16 } pub const fn uic_get_attr_id(v:u32)->u32 { (v>>16)&0xffff }

pub const UFSHCD_LINK_IS_DOWN:u32=1; pub const UFSHCD_LINK_IS_UP:u32=2;
pub const UIC_CMD_DME_GET:u32=1; pub const UIC_CMD_DME_SET:u32=2; pub const UIC_CMD_DME_PEER_GET:u32=3; pub const UIC_CMD_DME_PEER_SET:u32=4; pub const UIC_CMD_DME_POWERON:u32=0x10; pub const UIC_CMD_DME_POWEROFF:u32=0x11; pub const UIC_CMD_DME_ENABLE:u32=0x12; pub const UIC_CMD_DME_RESET:u32=0x14; pub const UIC_CMD_DME_END_PT_RST:u32=0x15; pub const UIC_CMD_DME_LINK_STARTUP:u32=0x16; pub const UIC_CMD_DME_HIBER_ENTER:u32=0x17; pub const UIC_CMD_DME_HIBER_EXIT:u32=0x18; pub const UIC_CMD_DME_TEST_MODE:u32=0x1a;
pub const UIC_CMD_RESULT_SUCCESS:u32=0; pub const UIC_CMD_RESULT_INVALID_ATTR:u32=1; pub const UIC_CMD_RESULT_FAILURE:u32=1; pub const UIC_CMD_RESULT_INVALID_ATTR_VALUE:u32=2; pub const UIC_CMD_RESULT_READ_ONLY_ATTR:u32=3; pub const UIC_CMD_RESULT_WRITE_ONLY_ATTR:u32=4; pub const UIC_CMD_RESULT_BAD_INDEX:u32=5; pub const UIC_CMD_RESULT_LOCKED_ATTR:u32=6; pub const UIC_CMD_RESULT_BAD_TEST_FEATURE_INDEX:u32=7; pub const UIC_CMD_RESULT_PEER_COMM_FAILURE:u32=8; pub const UIC_CMD_RESULT_BUSY:u32=9; pub const UIC_CMD_RESULT_DME_FAILURE:u32=0xa; pub const MASK_UIC_COMMAND_RESULT:u32=0xff;
pub const fn int_aggr_counter_thld_val(c:u32)->u32 { (c&0x1f)<<8 } pub const fn int_aggr_timeout_val(t:u32)->u32 { t&0xff }
pub const INTERRUPT_MASK_ALL_VER_11:u32=0x31fff; pub const INTERRUPT_MASK_ALL_VER_21:u32=0x71fff;

#[repr(C)] pub union ufs_crypto_capabilities { pub reg_val: u32, pub fields: CryptoCapabilitiesFields }
#[repr(C)] #[derive(Copy,Clone)] pub struct CryptoCapabilitiesFields { pub num_crypto_cap:u8, pub config_count:u8, pub reserved:u8, pub config_array_ptr:u8 }
pub const UFS_CRYPTO_KEY_SIZE_INVALID:u32=0; pub const UFS_CRYPTO_KEY_SIZE_128:u32=1; pub const UFS_CRYPTO_KEY_SIZE_192:u32=2; pub const UFS_CRYPTO_KEY_SIZE_256:u32=3; pub const UFS_CRYPTO_KEY_SIZE_512:u32=4;
pub const UFS_CRYPTO_ALG_AES_XTS:u32=0; pub const UFS_CRYPTO_ALG_BITLOCKER_AES_CBC:u32=1; pub const UFS_CRYPTO_ALG_AES_ECB:u32=2; pub const UFS_CRYPTO_ALG_ESSIV_AES_CBC:u32=3;
#[repr(C)] pub union ufs_crypto_cap_entry { pub reg_val:u32, pub fields: CryptoCapEntryFields }
#[repr(C)] #[derive(Copy,Clone)] pub struct CryptoCapEntryFields { pub algorithm_id:u8, pub sdus_mask:u8, pub key_size:u8, pub reserved:u8 }
pub const UFS_CRYPTO_CONFIGURATION_ENABLE:u32=1<<7; pub const UFS_CRYPTO_KEY_MAX_SIZE:usize=64;
#[repr(C)] pub union ufs_crypto_cfg_entry { pub reg_val:[u32;32], pub fields: CryptoCfgEntryFields }
#[repr(C)] #[derive(Copy,Clone)] pub struct CryptoCfgEntryFields { pub crypto_key:[u8;64], pub data_unit_size:u8, pub crypto_cap_idx:u8, pub reserved_1:u8, pub config_enable:u8, pub reserved_multi_host:u8, pub reserved_2:u8, pub vsb:[u8;2], pub reserved_3:[u8;56] }

pub const UTP_CMD_TYPE_UFS_STORAGE:u32=1; pub const UTP_SCSI_COMMAND:u32=0; pub const UTP_NATIVE_UFS_COMMAND:u32=0x10000000; pub const UTP_DEVICE_MANAGEMENT_FUNCTION:u32=0x20000000;
pub const UTP_NO_DATA_TRANSFER:u32=0; pub const UTP_HOST_TO_DEVICE:u32=1; pub const UTP_DEVICE_TO_HOST:u32=2;
pub const OCS_SUCCESS:u32=0; pub const OCS_INVALID_CMD_TABLE_ATTR:u32=1; pub const OCS_INVALID_PRDT_ATTR:u32=2; pub const OCS_MISMATCH_DATA_BUF_SIZE:u32=3; pub const OCS_MISMATCH_RESP_UPIU_SIZE:u32=4; pub const OCS_PEER_COMM_FAILURE:u32=5; pub const OCS_ABORTED:u32=6; pub const OCS_FATAL_ERROR:u32=7; pub const OCS_DEVICE_FATAL_ERROR:u32=8; pub const OCS_INVALID_CRYPTO_CONFIG:u32=9; pub const OCS_GENERAL_CRYPTO_ERROR:u32=0xa; pub const OCS_INVALID_COMMAND_STATUS:u32=0xf; pub const MASK_OCS:u32=0xf;
pub const PRDT_DATA_BYTE_COUNT_MAX:usize=256*1024; pub const PRDT_DATA_BYTE_COUNT_PAD:usize=4;

#[repr(C)] pub struct ufshcd_sg_entry { pub addr:u64, pub reserved:u32, pub size:u32 }
#[repr(C)] pub struct utp_transfer_cmd_desc { pub command_upiu:[u8;ALIGNED_UPIU_SIZE], pub response_upiu:[u8;ALIGNED_UPIU_SIZE], pub prd_table:[u8;0] }
#[repr(C)] pub struct utp_devman_cmd_desc { pub command_upiu:[u8;ALIGNED_UPIU_SIZE], pub response_upiu:[u8;ALIGNED_DEVMAN_RSP_SIZE], pub prd_table:[u8;0] }

#[repr(C)] pub struct request_desc_header { pub cci:u8, pub ehs_length:u8, pub flags:u8, pub command_type_and_flags:u8, pub dunl:u32, pub ocs:u8, pub cds:u8, pub ldbc:u16, pub dunu:u32 }
#[repr(C)] pub struct utp_transfer_req_desc { pub header:request_desc_header, pub command_desc_base_addr:u64, pub response_upiu_length:u16, pub response_upiu_offset:u16, pub prd_table_length:u16, pub prd_table_offset:u16 }
#[repr(C)] pub struct cq_entry { pub command_desc_base_addr:u64, pub response_upiu_length:u16, pub response_upiu_offset:u16, pub prd_table_length:u16, pub prd_table_offset:u16, pub overall_status:u8, pub extended_error_code:u8, pub reserved_1:u16, pub task_tag:u8, pub lun:u8, pub iid_ext_iid:u8, pub reserved_2:u8, pub reserved_3:[u32;2] }
#[repr(C)] pub struct utp_task_req_desc { pub header:request_desc_header, pub upiu_req:UtpTaskReq, pub upiu_rsp:UtpTaskRsp }
#[repr(C)] pub struct UtpTaskReq { pub req_header:utp_upiu_header, pub input_param1:u32, pub input_param2:u32, pub input_param3:u32, pub reserved1:[u32;2] }
#[repr(C)] pub struct UtpTaskRsp { pub rsp_header:utp_upiu_header, pub output_param1:u32, pub output_param2:u32, pub reserved2:[u32;3] }

/* External symbols supplied by ufs/ufs.h. */
extern "Rust" { static PA_MAXDATALANES: u32; }
extern "Rust" { type utp_upiu_header; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
