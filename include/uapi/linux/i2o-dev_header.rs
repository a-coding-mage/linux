/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* I2O user space accessible structures/APIs. Translated from i2o-dev.h. */

// linux/types.h and linux/ioctl.h dependencies are supplied by the surrounding bindings.
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;

pub const MAX_I2O_CONTROLLERS: usize = 32;
pub const I2O_MAGIC_NUMBER: u8 = b'i';

// ioctl encodings depend on the target platform's linux/ioctl.h definitions.
pub const I2O_EVT_Q_LEN: usize = 32;
pub const I2O_EVT_DATA_SIZE: usize = 88;

pub const I2O_EVT_IND_STATE_CHANGE:u32=0x80000000; pub const I2O_EVT_IND_GENERAL_WARNING:u32=0x40000000; pub const I2O_EVT_IND_CONFIGURATION_FLAG:u32=0x20000000; pub const I2O_EVT_IND_LOCK_RELEASE:u32=0x10000000; pub const I2O_EVT_IND_CAPABILITY_CHANGE:u32=0x08000000; pub const I2O_EVT_IND_DEVICE_RESET:u32=0x04000000; pub const I2O_EVT_IND_EVT_MASK_MODIFIED:u32=0x02000000; pub const I2O_EVT_IND_FIELD_MODIFIED:u32=0x01000000; pub const I2O_EVT_IND_VENDOR_EVT:u32=0x00800000; pub const I2O_EVT_IND_DEVICE_STATE:u32=0x00400000;
pub const I2O_EVT_IND_EXEC_RESOURCE_LIMITS:u32=1; pub const I2O_EVT_IND_EXEC_CONNECTION_FAIL:u32=2; pub const I2O_EVT_IND_EXEC_ADAPTER_FAULT:u32=4; pub const I2O_EVT_IND_EXEC_POWER_FAIL:u32=8; pub const I2O_EVT_IND_EXEC_RESET_PENDING:u32=0x10; pub const I2O_EVT_IND_EXEC_RESET_IMMINENT:u32=0x20; pub const I2O_EVT_IND_EXEC_HW_FAIL:u32=0x40; pub const I2O_EVT_IND_EXEC_XCT_CHANGE:u32=0x80; pub const I2O_EVT_IND_EXEC_NEW_LCT_ENTRY:u32=0x100; pub const I2O_EVT_IND_EXEC_MODIFIED_LCT:u32=0x200; pub const I2O_EVT_IND_EXEC_DDM_AVAILABILITY:u32=0x400;
pub const I2O_EVT_IND_BSA_VOLUME_LOAD:u32=1; pub const I2O_EVT_IND_BSA_VOLUME_UNLOAD:u32=2; pub const I2O_EVT_IND_BSA_VOLUME_UNLOAD_REQ:u32=4; pub const I2O_EVT_IND_BSA_CAPACITY_CHANGE:u32=8; pub const I2O_EVT_IND_BSA_SCSI_SMART:u32=0x10;
pub const I2O_EVT_STATE_CHANGE_NORMAL:u32=0; pub const I2O_EVT_STATE_CHANGE_SUSPENDED:u32=1; pub const I2O_EVT_STATE_CHANGE_RESTART:u32=2; pub const I2O_EVT_STATE_CHANGE_NA_RECOVER:u32=3; pub const I2O_EVT_STATE_CHANGE_NA_NO_RECOVER:u32=4; pub const I2O_EVT_STATE_CHANGE_QUIESCE_REQUEST:u32=5; pub const I2O_EVT_STATE_CHANGE_FAILED:u32=0x10; pub const I2O_EVT_STATE_CHANGE_FAULTED:u32=0x11;
pub const I2O_EVT_GEN_WARNING_NORMAL:u32=0; pub const I2O_EVT_GEN_WARNING_ERROR_THRESHOLD:u32=1; pub const I2O_EVT_GEN_WARNING_MEDIA_FAULT:u32=2; pub const I2O_EVT_CAPABILITY_OTHER:u32=1; pub const I2O_EVT_CAPABILITY_CHANGED:u32=2; pub const I2O_EVT_SENSOR_STATE_CHANGED:u32=1;

pub const I2O_CLASS_VERSION_10:u32=0; pub const I2O_CLASS_VERSION_11:u32=1; pub const I2O_CLASS_EXECUTIVE:u32=0; pub const I2O_CLASS_DDM:u32=1; pub const I2O_CLASS_RANDOM_BLOCK_STORAGE:u32=0x10; pub const I2O_CLASS_SEQUENTIAL_STORAGE:u32=0x11; pub const I2O_CLASS_LAN:u32=0x20; pub const I2O_CLASS_WAN:u32=0x30; pub const I2O_CLASS_FIBRE_CHANNEL_PORT:u32=0x40; pub const I2O_CLASS_FIBRE_CHANNEL_PERIPHERAL:u32=0x41; pub const I2O_CLASS_SCSI_PERIPHERAL:u32=0x51; pub const I2O_CLASS_ATE_PORT:u32=0x60; pub const I2O_CLASS_ATE_PERIPHERAL:u32=0x61; pub const I2O_CLASS_FLOPPY_CONTROLLER:u32=0x70; pub const I2O_CLASS_FLOPPY_DEVICE:u32=0x71; pub const I2O_CLASS_BUS_ADAPTER:u32=0x80; pub const I2O_CLASS_PEER_TRANSPORT_AGENT:u32=0x90; pub const I2O_CLASS_PEER_TRANSPORT:u32=0x91; pub const I2O_CLASS_END:u32=0xfff; pub const I2O_CLASS_MATCH_ANYCLASS:u32=0xffffffff;
pub const I2O_SUBCLASS_i960:u32=1; pub const I2O_SUBCLASS_HDM:u32=0x20; pub const I2O_SUBCLASS_ISM:u32=0x21;
pub const I2O_PARAMS_FIELD_GET:u32=1; pub const I2O_PARAMS_LIST_GET:u32=2; pub const I2O_PARAMS_MORE_GET:u32=3; pub const I2O_PARAMS_SIZE_GET:u32=4; pub const I2O_PARAMS_TABLE_GET:u32=5; pub const I2O_PARAMS_FIELD_SET:u32=6; pub const I2O_PARAMS_LIST_SET:u32=7; pub const I2O_PARAMS_ROW_ADD:u32=8; pub const I2O_PARAMS_ROW_DELETE:u32=9; pub const I2O_PARAMS_TABLE_CLEAR:u32=0xA;
pub const I2O_SNFORMAT_UNKNOWN:u32=0; pub const I2O_SNFORMAT_BINARY:u32=1; pub const I2O_SNFORMAT_ASCII:u32=2; pub const I2O_SNFORMAT_UNICODE:u32=3; pub const I2O_SNFORMAT_LAN48_MAC:u32=4; pub const I2O_SNFORMAT_WAN:u32=5; pub const I2O_SNFORMAT_LAN64_MAC:u32=6; pub const I2O_SNFORMAT_DDM:u32=7; pub const I2O_SNFORMAT_IEEE_REG64:u32=8; pub const I2O_SNFORMAT_IEEE_REG128:u32=9; pub const I2O_SNFORMAT_UNKNOWN2:u32=0xff;
pub const ADAPTER_STATE_INITIALIZING:u32=1; pub const ADAPTER_STATE_RESET:u32=2; pub const ADAPTER_STATE_HOLD:u32=4; pub const ADAPTER_STATE_READY:u32=5; pub const ADAPTER_STATE_OPERATIONAL:u32=8; pub const ADAPTER_STATE_FAILED:u32=0x10; pub const ADAPTER_STATE_FAULTED:u32=0x11;
pub const I2O_SOFTWARE_MODULE_IRTOS:u32=0x11; pub const I2O_SOFTWARE_MODULE_IOP_PRIVATE:u32=0x22; pub const I2O_SOFTWARE_MODULE_IOP_CONFIG:u32=0x23; pub const I2O_VENDOR_DPT:u32=0x1b; pub const I2O_DPT_SG_FLAG_INTERPRET:u32=0x10000; pub const I2O_DPT_SG_FLAG_PHYSICAL:u32=0x20000; pub const I2O_DPT_FLASH_FRAG_SIZE:u32=0x10000; pub const I2O_DPT_FLASH_READ:u32=0x101; pub const I2O_DPT_FLASH_WRITE:u32=0x102;

#[repr(C)]
pub struct i2o_cmd_passthru32 { pub iop: u32, pub msg: __u32 }
#[repr(C)]
pub struct i2o_cmd_passthru { pub iop: u32, pub msg: *mut core::ffi::c_void }
#[repr(C)]
pub struct i2o_cmd_hrtlct { pub iop: u32, pub resbuf: *mut core::ffi::c_void, pub reslen: *mut u32 }
#[repr(C)]
pub struct i2o_cmd_psetget { pub iop: u32, pub tid: u32, pub opbuf: *mut core::ffi::c_void, pub oplen: u32, pub resbuf: *mut core::ffi::c_void, pub reslen: *mut u32 }
#[repr(C)]
pub struct i2o_sw_xfer { pub iop: u32, pub flags: u8, pub sw_type: u8, pub sw_id: u32, pub buf: *mut core::ffi::c_void, pub swlen: *mut u32, pub maxfrag: *mut u32, pub curfrag: *mut u32 }
#[repr(C)]
pub struct i2o_html { pub iop: u32, pub tid: u32, pub page: u32, pub resbuf: *mut core::ffi::c_void, pub reslen: *mut u32, pub qbuf: *mut core::ffi::c_void, pub qlen: u32 }
#[repr(C)]
pub struct i2o_evt_id { pub iop: u32, pub tid: u32, pub evt_mask: u32 }
#[repr(C)]
pub struct i2o_evt_info { pub id: i2o_evt_id, pub evt_data: [u8; I2O_EVT_DATA_SIZE], pub data_size: u32 }
#[repr(C)]
pub struct i2o_evt_get { pub info: i2o_evt_info, pub pending: i32, pub lost: i32 }
#[repr(C)]
pub struct i2o_sg_io_hdr_t { pub flags: u32 }

pub const I2O_BUS_LOCAL: u32 = 0; pub const I2O_BUS_ISA: u32 = 1; pub const I2O_BUS_EISA: u32 = 2;
pub const I2O_BUS_PCI: u32 = 4; pub const I2O_BUS_PCMCIA: u32 = 5; pub const I2O_BUS_NUBUS: u32 = 6;
pub const I2O_BUS_CARDBUS: u32 = 7; pub const I2O_BUS_UNKNOWN: u32 = 0x80;

#[repr(C)] pub struct i2o_pci_bus { pub PciFunctionNumber:u8,pub PciDeviceNumber:u8,pub PciBusNumber:u8,pub reserved:u8,pub PciVendorID:u16,pub PciDeviceID:u16 }
#[repr(C)] pub struct i2o_local_bus { pub LbBaseIOPort:u16,pub reserved:u16,pub LbBaseMemoryAddress:u32 }
#[repr(C)] pub struct i2o_isa_bus { pub IsaBaseIOPort:u16,pub CSN:u8,pub reserved:u8,pub IsaBaseMemoryAddress:u32 }
#[repr(C)] pub struct i2o_eisa_bus { pub EisaBaseIOPort:u16,pub reserved:u8,pub EisaSlotNumber:u8,pub EisaBaseMemoryAddress:u32 }
#[repr(C)] pub struct i2o_mca_bus { pub McaBaseIOPort:u16,pub reserved:u8,pub McaSlotNumber:u8,pub McaBaseMemoryAddress:u32 }
#[repr(C)] pub struct i2o_other_bus { pub BaseIOPort:u16,pub reserved:u16,pub BaseMemoryAddress:u32 }

#[repr(C)] pub union i2o_hrt_entry_bus { pub pci_bus:i2o_pci_bus,pub local_bus:i2o_local_bus,pub isa_bus:i2o_isa_bus,pub eisa_bus:i2o_eisa_bus,pub mca_bus:i2o_mca_bus,pub other_bus:i2o_other_bus }
#[repr(C)] pub struct i2o_hrt_entry { pub adapter_id:u32, pub parent_tid_state_bus:u32, pub bus:i2o_hrt_entry_bus }
#[repr(C)] pub struct i2o_hrt { pub num_entries:u16,pub entry_len:u8,pub hrt_version:u8,pub change_ind:u32,pub hrt_entry:[i2o_hrt_entry;1] }
#[repr(C)] pub struct i2o_lct_entry { pub entry_size_tid_reserved:u32,pub change_ind:u32,pub device_flags:u32,pub class_version_vendor:u32,pub sub_class:u32,pub user_tid_parent_tid_bios_info:u32,pub identity_tag:[u8;8],pub event_capabilities:u32 }
#[repr(C)] pub struct i2o_lct { pub table_size_boot_tid_ver:u32,pub iop_flags:u32,pub change_ind:u32,pub lct_entry:[i2o_lct_entry;1] }
#[repr(C)] pub struct i2o_status_block { pub org_id:u16,pub reserved:u16,pub iop_id_reserved1:u16,pub host_unit_id:u16,pub segment_number_version:u16,pub iop_state:u8,pub msg_type:u8,pub inbound_frame_size:u16,pub init_code:u8,pub reserved2:u8,pub max_inbound_frames:u32,pub cur_inbound_frames:u32,pub max_outbound_frames:u32,pub product_id:[core::ffi::c_char;24],pub expected_lct_size:u32,pub iop_capabilities:u32,pub desired_mem_size:u32,pub current_mem_size:u32,pub current_mem_base:u32,pub desired_io_size:u32,pub current_io_size:u32,pub current_io_base:u32,pub reserved3_cmd_status:u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
