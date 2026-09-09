/* SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note */
/* Driver for Broadcom MPI3 Storage Controllers */

// Linux types supplied by the including translation unit.
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __le16 = u16;
pub type __le32 = u32;

pub const MPI3MR_IOCTL_VERSION: u32 = 0x06;
pub const MPI3MR_APP_DEFAULT_TIMEOUT: u32 = 60;
pub const MPI3MR_BSG_ADPTYPE_UNKNOWN: u32 = 0;
pub const MPI3MR_BSG_ADPTYPE_AVGFAMILY: u32 = 1;
pub const MPI3MR_BSG_ADPSTATE_UNKNOWN: u32 = 0;
pub const MPI3MR_BSG_ADPSTATE_OPERATIONAL: u32 = 1;
pub const MPI3MR_BSG_ADPSTATE_FAULT: u32 = 2;
pub const MPI3MR_BSG_ADPSTATE_IN_RESET: u32 = 3;
pub const MPI3MR_BSG_ADPSTATE_UNRECOVERABLE: u32 = 4;
pub const MPI3MR_BSG_ADPRESET_UNKNOWN: u32 = 0;
pub const MPI3MR_BSG_ADPRESET_SOFT: u32 = 1;
pub const MPI3MR_BSG_ADPRESET_DIAG_FAULT: u32 = 2;
pub const MPI3MR_BSG_LOGDATA_MAX_ENTRIES: u32 = 400;
pub const MPI3MR_BSG_LOGDATA_ENTRY_HEADER_SZ: u32 = 4;
pub const MPI3MR_DRVBSG_OPCODE_UNKNOWN: u32 = 0;
pub const MPI3MR_DRVBSG_OPCODE_ADPINFO: u32 = 1;
pub const MPI3MR_DRVBSG_OPCODE_ADPRESET: u32 = 2;
pub const MPI3MR_DRVBSG_OPCODE_ALLTGTDEVINFO: u32 = 4;
pub const MPI3MR_DRVBSG_OPCODE_GETCHGCNT: u32 = 5;
pub const MPI3MR_DRVBSG_OPCODE_LOGDATAENABLE: u32 = 6;
pub const MPI3MR_DRVBSG_OPCODE_PELENABLE: u32 = 7;
pub const MPI3MR_DRVBSG_OPCODE_GETLOGDATA: u32 = 8;
pub const MPI3MR_DRVBSG_OPCODE_QUERY_HDB: u32 = 9;
pub const MPI3MR_DRVBSG_OPCODE_REPOST_HDB: u32 = 10;
pub const MPI3MR_DRVBSG_OPCODE_UPLOAD_HDB: u32 = 11;
pub const MPI3MR_DRVBSG_OPCODE_REFRESH_HDB_TRIGGERS: u32 = 12;
pub const MPI3MR_BSG_BUFTYPE_UNKNOWN: u32 = 0;
pub const MPI3MR_BSG_BUFTYPE_RAIDMGMT_CMD: u32 = 1;
pub const MPI3MR_BSG_BUFTYPE_RAIDMGMT_RESP: u32 = 2;
pub const MPI3MR_BSG_BUFTYPE_DATA_IN: u32 = 3;
pub const MPI3MR_BSG_BUFTYPE_DATA_OUT: u32 = 4;
pub const MPI3MR_BSG_BUFTYPE_MPI_REPLY: u32 = 5;
pub const MPI3MR_BSG_BUFTYPE_ERR_RESPONSE: u32 = 6;
pub const MPI3MR_BSG_BUFTYPE_MPI_REQUEST: u32 = 0xfe;
pub const MPI3MR_BSG_MPI_REPLY_BUFTYPE_UNKNOWN: u32 = 0;
pub const MPI3MR_BSG_MPI_REPLY_BUFTYPE_STATUS: u32 = 1;
pub const MPI3MR_BSG_MPI_REPLY_BUFTYPE_ADDRESS: u32 = 2;
pub const MPI3MR_HDB_BUFTYPE_UNKNOWN: u32 = 0;
pub const MPI3MR_HDB_BUFTYPE_TRACE: u32 = 1;
pub const MPI3MR_HDB_BUFTYPE_FIRMWARE: u32 = 2;
pub const MPI3MR_HDB_BUFTYPE_RESERVED: u32 = 3;
pub const MPI3MR_HDB_BUFSTATUS_UNKNOWN: u32 = 0;
pub const MPI3MR_HDB_BUFSTATUS_NOT_ALLOCATED: u32 = 1;
pub const MPI3MR_HDB_BUFSTATUS_POSTED_UNPAUSED: u32 = 2;
pub const MPI3MR_HDB_BUFSTATUS_POSTED_PAUSED: u32 = 3;
pub const MPI3MR_HDB_BUFSTATUS_RELEASED: u32 = 4;
pub const MPI3MR_HDB_TRIGGER_TYPE_UNKNOWN: u32 = 0;
pub const MPI3MR_HDB_TRIGGER_TYPE_DIAGFAULT: u32 = 1;
pub const MPI3MR_HDB_TRIGGER_TYPE_ELEMENT: u32 = 2;
pub const MPI3MR_HDB_TRIGGER_TYPE_MASTER: u32 = 3;

#[repr(i32)]
pub enum command { MPI3MR_DRV_CMD = 1, MPI3MR_MPT_CMD = 2 }

#[repr(C)]
pub struct mpi3_driver_info_layout { pub information_length: __le32, pub driver_signature: [__u8;12], pub os_name: [__u8;16], pub os_version: [__u8;12], pub driver_name: [__u8;20], pub driver_version: [__u8;32], pub driver_release_date: [__u8;20], pub driver_capabilities: __le32 }
#[repr(C)]
pub struct mpi3mr_bsg_in_adpinfo { pub adp_type: __u32, pub rsvd1: __u32, pub pci_dev_id: __u32, pub pci_dev_hw_rev: __u32, pub pci_subsys_dev_id: __u32, pub pci_subsys_ven_id: __u32, pub pci_dev_func_bus: __u32, pub rsvd2: __u16, pub pci_seg_id: __u32, pub app_intfc_ver: __u32, pub adp_state: __u8, pub rsvd3: __u8, pub rsvd4: __u16, pub rsvd5: [__u32;2], pub driver_info: mpi3_driver_info_layout }
#[repr(C)] pub struct mpi3mr_bsg_adp_reset { pub reset_type: __u8, pub rsvd1: __u8, pub rsvd2: __u16 }
#[repr(C)] pub struct mpi3mr_change_count { pub change_count: __u16, pub rsvd: __u16 }
#[repr(C)] pub struct mpi3mr_device_map_info { pub handle: __u16, pub perst_id: __u16, pub target_id: __u32, pub bus_id: __u8, pub rsvd1: __u8, pub rsvd2: __u16 }
#[repr(C)] pub struct mpi3mr_all_tgt_info { pub num_devices: __u16, pub rsvd1: __u16, pub rsvd2: __u32, pub dmi: [mpi3mr_device_map_info;1] }
#[repr(C)] pub struct mpi3mr_logdata_enable { pub max_entries: __u16, pub rsvd: __u16 }
#[repr(C)] pub struct mpi3mr_bsg_out_pel_enable { pub pel_locale: __u16, pub pel_class: __u8, pub rsvd: __u8 }
#[repr(C)] pub struct mpi3mr_logdata_entry { pub valid_entry: __u8, pub rsvd1: __u8, pub rsvd2: __u16, pub data: [__u8;1] }
#[repr(C)] pub struct mpi3mr_bsg_in_log_data { pub entry: [mpi3mr_logdata_entry;1] }
#[repr(C)] pub struct mpi3mr_hdb_entry { pub buf_type: __u8, pub status: __u8, pub trigger_type: __u8, pub rsvd1: __u8, pub size: __u16, pub rsvd2: __u16, pub trigger_data: __u64, pub rsvd3: __u32, pub rsvd4: __u32 }
#[repr(C)] pub struct mpi3mr_bsg_in_hdb_status { pub num_hdb_types: __u8, pub element_trigger_format: __u8, pub rsvd2: __u16, pub rsvd3: __u32, pub entry: [mpi3mr_hdb_entry;1] }
#[repr(C)] pub struct mpi3mr_bsg_out_repost_hdb { pub buf_type: __u8, pub rsvd1: __u8, pub rsvd2: __u16 }
#[repr(C)] pub struct mpi3mr_bsg_out_upload_hdb { pub buf_type: __u8, pub rsvd1: __u8, pub rsvd2: __u16, pub start_offset: __u32, pub length: __u32 }
#[repr(C)] pub struct mpi3mr_bsg_out_refresh_hdb_triggers { pub page_type: __u8, pub rsvd1: __u8, pub rsvd2: __u16 }
#[repr(C)] pub struct mpi3mr_bsg_drv_cmd { pub mrioc_id: __u8, pub opcode: __u8, pub rsvd1: __u16, pub rsvd2: [__u32;4] }
#[repr(C)] pub struct mpi3mr_bsg_in_reply_buf { pub mpi_reply_type: __u8, pub rsvd1: __u8, pub rsvd2: __u16, pub reply_buf: [__u8;0] }
#[repr(C)] pub struct mpi3mr_buf_entry { pub buf_type: __u8, pub rsvd1: __u8, pub rsvd2: __u16, pub buf_len: __u32 }
#[repr(C)] pub struct mpi3mr_buf_entry_list { pub num_of_entries: __u8, pub rsvd1: __u8, pub rsvd2: __u16, pub rsvd3: __u32, pub buf_entry: [mpi3mr_buf_entry;1] }
#[repr(C)] pub struct mpi3mr_bsg_mptcmd { pub mrioc_id: __u8, pub rsvd1: __u8, pub timeout: __u16, pub rsvd2: __u32, pub buf_entry_list: mpi3mr_buf_entry_list }
#[repr(C)] pub union mpi3mr_bsg_packet_cmd { pub drvrcmd: mpi3mr_bsg_drv_cmd, pub mptcmd: mpi3mr_bsg_mptcmd }
#[repr(C)] pub struct mpi3mr_bsg_packet { pub cmd_type: __u8, pub rsvd1: __u8, pub rsvd2: __u16, pub rsvd3: __u32, pub cmd: mpi3mr_bsg_packet_cmd }

#[repr(C)] pub struct mpi3_nvme_encapsulated_request { pub host_tag: __le16, pub ioc_use_only02: __u8, pub function: __u8, pub ioc_use_only04: __le16, pub ioc_use_only06: __u8, pub msg_flags: __u8, pub change_count: __le16, pub dev_handle: __le16, pub encapsulated_command_length: __le16, pub flags: __le16, pub data_length: __le32, pub reserved14: [__le32;3], pub command: [__le32;0] }
#[repr(C)] pub struct mpi3_nvme_encapsulated_error_reply { pub host_tag: __le16, pub ioc_use_only02: __u8, pub function: __u8, pub ioc_use_only04: __le16, pub ioc_use_only06: __u8, pub msg_flags: __u8, pub ioc_use_only08: __le16, pub ioc_status: __le16, pub ioc_log_info: __le32, pub nvme_completion_entry: [__le32;4] }
pub const MPI3MR_NVME_PRP_SIZE: u32=8; pub const MPI3MR_NVME_CMD_PRP1_OFFSET: u32=24; pub const MPI3MR_NVME_CMD_PRP2_OFFSET: u32=32; pub const MPI3MR_NVME_CMD_SGL_OFFSET: u32=24; pub const MPI3MR_NVME_DATA_FORMAT_PRP: u32=0; pub const MPI3MR_NVME_DATA_FORMAT_SGL1: u32=1; pub const MPI3MR_NVME_DATA_FORMAT_SGL2: u32=2; pub const MPI3MR_NVMESGL_DATA_SEGMENT: u32=0; pub const MPI3MR_NVMESGL_LAST_SEGMENT: u32=3;
#[repr(C)] pub struct mpi3_scsi_task_mgmt_request { pub host_tag: __le16, pub ioc_use_only02: __u8, pub function: __u8, pub ioc_use_only04: __le16, pub ioc_use_only06: __u8, pub msg_flags: __u8, pub change_count: __le16, pub dev_handle: __le16, pub task_host_tag: __le16, pub task_type: __u8, pub reserved0f: __u8, pub task_request_queue_id: __le16, pub reserved12: __le16, pub reserved14: __le32, pub lun: [__u8;8] }
#[repr(C)] pub struct mpi3_scsi_task_mgmt_reply { pub host_tag: __le16, pub ioc_use_only02: __u8, pub function: __u8, pub ioc_use_only04: __le16, pub ioc_use_only06: __u8, pub msg_flags: __u8, pub ioc_use_only08: __le16, pub ioc_status: __le16, pub ioc_log_info: __le32, pub termination_count: __le32, pub response_data: __le32, pub reserved18: __le32 }
pub const MPI3_SCSITASKMGMT_MSGFLAGS_DO_NOT_SEND_TASK_IU: u32=0x08; pub const MPI3_SCSITASKMGMT_TASKTYPE_ABORT_TASK: u32=1; pub const MPI3_SCSITASKMGMT_TASKTYPE_ABORT_TASK_SET: u32=2; pub const MPI3_SCSITASKMGMT_TASKTYPE_TARGET_RESET: u32=3; pub const MPI3_SCSITASKMGMT_TASKTYPE_LOGICAL_UNIT_RESET: u32=5; pub const MPI3_SCSITASKMGMT_TASKTYPE_CLEAR_TASK_SET: u32=6; pub const MPI3_SCSITASKMGMT_TASKTYPE_QUERY_TASK: u32=7; pub const MPI3_SCSITASKMGMT_TASKTYPE_CLEAR_ACA: u32=8; pub const MPI3_SCSITASKMGMT_TASKTYPE_QUERY_TASK_SET: u32=9; pub const MPI3_SCSITASKMGMT_TASKTYPE_QUERY_ASYNC_EVENT: u32=10; pub const MPI3_SCSITASKMGMT_TASKTYPE_I_T_NEXUS_RESET: u32=11;
pub const MPI3_SCSITASKMGMT_RSPCODE_TM_COMPLETE: u32=0; pub const MPI3_SCSITASKMGMT_RSPCODE_INVALID_FRAME: u32=2; pub const MPI3_SCSITASKMGMT_RSPCODE_TM_FUNCTION_NOT_SUPPORTED: u32=4; pub const MPI3_SCSITASKMGMT_RSPCODE_TM_FAILED: u32=5; pub const MPI3_SCSITASKMGMT_RSPCODE_TM_SUCCEEDED: u32=8; pub const MPI3_SCSITASKMGMT_RSPCODE_TM_INVALID_LUN: u32=9; pub const MPI3_SCSITASKMGMT_RSPCODE_TM_OVERLAPPED_TAG: u32=10; pub const MPI3_SCSITASKMGMT_RSPCODE_IO_QUEUED_ON_IOC: u32=0x80; pub const MPI3_SCSITASKMGMT_RSPCODE_TM_NVME_DENIED: u32=0x81;
pub const MPI3_PEL_LOCALE_FLAGS_NON_BLOCKING_BOOT_EVENT: u32=0x0200; pub const MPI3_PEL_LOCALE_FLAGS_BLOCKING_BOOT_EVENT: u32=0x0100; pub const MPI3_PEL_LOCALE_FLAGS_PCIE: u32=0x0080; pub const MPI3_PEL_LOCALE_FLAGS_CONFIGURATION: u32=0x0040; pub const MPI3_PEL_LOCALE_FLAGS_CONTROLER: u32=0x0020; pub const MPI3_PEL_LOCALE_FLAGS_SAS: u32=0x0010; pub const MPI3_PEL_LOCALE_FLAGS_EPACK: u32=8; pub const MPI3_PEL_LOCALE_FLAGS_ENCLOSURE: u32=4; pub const MPI3_PEL_LOCALE_FLAGS_PD: u32=2; pub const MPI3_PEL_LOCALE_FLAGS_VD: u32=1;
pub const MPI3_PEL_CLASS_DEBUG: u32=0; pub const MPI3_PEL_CLASS_PROGRESS: u32=1; pub const MPI3_PEL_CLASS_INFORMATIONAL: u32=2; pub const MPI3_PEL_CLASS_WARNING: u32=3; pub const MPI3_PEL_CLASS_CRITICAL: u32=4; pub const MPI3_PEL_CLASS_FATAL: u32=5; pub const MPI3_PEL_CLASS_FAULT: u32=6;
pub const MPI3_BSG_FUNCTION_MGMT_PASSTHROUGH: u32=0x0a; pub const MPI3_BSG_FUNCTION_SCSI_IO: u32=0x20; pub const MPI3_BSG_FUNCTION_SCSI_TASK_MGMT: u32=0x21; pub const MPI3_BSG_FUNCTION_SMP_PASSTHROUGH: u32=0x22; pub const MPI3_BSG_FUNCTION_NVME_ENCAPSULATED: u32=0x24;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
