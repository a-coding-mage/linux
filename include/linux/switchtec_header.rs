/* SPDX-License-Identifier: GPL-2.0-only */
/* Microsemi Switchtec PCIe Driver; Copyright (c) 2017, Microsemi Corporation */

use core::ffi::c_void;

pub const SWITCHTEC_MRPC_PAYLOAD_SIZE: usize = 1024;
pub const SWITCHTEC_MAX_PFF_CSR: usize = 255;
pub const SWITCHTEC_EVENT_OCCURRED: u32 = 1 << 0;
pub const SWITCHTEC_EVENT_CLEAR: u32 = 1 << 0;
pub const SWITCHTEC_EVENT_EN_LOG: u32 = 1 << 1;
pub const SWITCHTEC_EVENT_EN_CLI: u32 = 1 << 2;
pub const SWITCHTEC_EVENT_EN_IRQ: u32 = 1 << 3;
pub const SWITCHTEC_EVENT_FATAL: u32 = 1 << 4;
pub const SWITCHTEC_EVENT_NOT_SUPP: u32 = 1 << 31;
pub const SWITCHTEC_DMA_MRPC_EN: u32 = 1 << 0;
pub const MRPC_GAS_READ: u32 = 0x29;
pub const MRPC_GAS_WRITE: u32 = 0x87;
#[inline] pub const fn MRPC_CMD_ID(x: u32) -> u32 { x & 0xffff }

pub const SWITCHTEC_GAS_MRPC_OFFSET: usize = 0x0000;
pub const SWITCHTEC_GAS_TOP_CFG_OFFSET: usize = 0x1000;
pub const SWITCHTEC_GAS_SW_EVENT_OFFSET: usize = 0x1800;
pub const SWITCHTEC_GAS_SYS_INFO_OFFSET: usize = 0x2000;
pub const SWITCHTEC_GAS_FLASH_INFO_OFFSET: usize = 0x2200;
pub const SWITCHTEC_GAS_PART_CFG_OFFSET: usize = 0x4000;
pub const SWITCHTEC_GAS_NTB_OFFSET: usize = 0x10000;
pub const SWITCHTEC_GAS_PFF_CSR_OFFSET: usize = 0x134000;

#[repr(C)] #[derive(Copy, Clone)] pub enum switchtec_gen { SWITCHTEC_GEN3, SWITCHTEC_GEN4, SWITCHTEC_GEN5, SWITCHTEC_GEN6 }

#[repr(C, packed)] pub struct mrpc_regs { pub input_data: [u8; 1024], pub output_data: [u8; 1024], pub cmd: u32, pub status: u32, pub ret_value: u32, pub dma_en: u32, pub dma_addr: u64, pub dma_vector: u32, pub dma_ver: u32 }
#[repr(C)] pub enum mrpc_status { SWITCHTEC_MRPC_STATUS_INPROGRESS = 1, SWITCHTEC_MRPC_STATUS_DONE = 2, SWITCHTEC_MRPC_STATUS_ERROR = 0xff, SWITCHTEC_MRPC_STATUS_INTERRUPTED = 0x100 }

#[repr(C, packed)] pub struct sw_event_regs {
 pub event_report_ctrl:u64,pub reserved1:u64,pub part_event_bitmap:u64,pub reserved2:u64,pub global_summary:u32,pub reserved3:[u32;3],
 pub stack_error_event_hdr:u32,pub stack_error_event_data:u32,pub reserved4:[u32;4],pub ppu_error_event_hdr:u32,pub ppu_error_event_data:u32,pub reserved5:[u32;4],pub isp_error_event_hdr:u32,pub isp_error_event_data:u32,pub reserved6:[u32;4],pub sys_reset_event_hdr:u32,pub reserved7:[u32;5],pub fw_exception_hdr:u32,pub reserved8:[u32;5],pub fw_nmi_hdr:u32,pub reserved9:[u32;5],pub fw_non_fatal_hdr:u32,pub reserved10:[u32;5],pub fw_fatal_hdr:u32,pub reserved11:[u32;5],pub twi_mrpc_comp_hdr:u32,pub twi_mrpc_comp_data:u32,pub reserved12:[u32;4],pub twi_mrpc_comp_async_hdr:u32,pub twi_mrpc_comp_async_data:u32,pub reserved13:[u32;4],pub cli_mrpc_comp_hdr:u32,pub cli_mrpc_comp_data:u32,pub reserved14:[u32;4],pub cli_mrpc_comp_async_hdr:u32,pub cli_mrpc_comp_async_data:u32,pub reserved15:[u32;4],pub gpio_interrupt_hdr:u32,pub gpio_interrupt_data:u32,pub reserved16:[u32;4],pub gfms_event_hdr:u32,pub gfms_event_data:u32,pub reserved17:[u32;4]
}

pub const SWITCHTEC_GEN3_CFG0_RUNNING:u32=0x04; pub const SWITCHTEC_GEN3_CFG1_RUNNING:u32=0x05; pub const SWITCHTEC_GEN3_IMG0_RUNNING:u32=0x03; pub const SWITCHTEC_GEN3_IMG1_RUNNING:u32=0x07;
pub const SWITCHTEC_GEN4_MAP0_RUNNING:u32=0; pub const SWITCHTEC_GEN4_MAP1_RUNNING:u32=1; pub const SWITCHTEC_GEN4_KEY0_RUNNING:u32=2; pub const SWITCHTEC_GEN4_KEY1_RUNNING:u32=3; pub const SWITCHTEC_GEN4_BL2_0_RUNNING:u32=4; pub const SWITCHTEC_GEN4_BL2_1_RUNNING:u32=5; pub const SWITCHTEC_GEN4_CFG0_RUNNING:u32=6; pub const SWITCHTEC_GEN4_CFG1_RUNNING:u32=7; pub const SWITCHTEC_GEN4_IMG0_RUNNING:u32=8; pub const SWITCHTEC_GEN4_IMG1_RUNNING:u32=9;
pub const SWITCHTEC_GEN4_KEY0_ACTIVE:u32=0; pub const SWITCHTEC_GEN4_KEY1_ACTIVE:u32=1; pub const SWITCHTEC_GEN4_BL2_0_ACTIVE:u32=0; pub const SWITCHTEC_GEN4_BL2_1_ACTIVE:u32=1; pub const SWITCHTEC_GEN4_CFG0_ACTIVE:u32=0; pub const SWITCHTEC_GEN4_CFG1_ACTIVE:u32=1; pub const SWITCHTEC_GEN4_IMG0_ACTIVE:u32=0; pub const SWITCHTEC_GEN4_IMG1_ACTIVE:u32=1;

#[repr(C, packed)] pub struct sys_info_regs_gen3 { pub reserved1:u32,pub vendor_table_revision:u32,pub table_format_version:u32,pub partition_id:u32,pub cfg_file_fmt_version:u32,pub cfg_running:u16,pub img_running:u16,pub reserved2:[u32;57],pub vendor_id:[u8;8],pub product_id:[u8;16],pub product_revision:[u8;4],pub component_vendor:[u8;8],pub component_id:u16,pub component_revision:u8 }
#[repr(C, packed)] pub struct sys_info_regs_gen4 { pub gas_layout_ver:u16,pub evlist_ver:u8,pub reserved1:u8,pub mgmt_cmd_set_ver:u16,pub fabric_cmd_set_ver:u16,pub reserved2:[u32;2],pub mrpc_uart_ver:u8,pub mrpc_twi_ver:u8,pub mrpc_eth_ver:u8,pub mrpc_inband_ver:u8,pub reserved3:[u32;7],pub fw_update_tmo:u32,pub xml_version_cfg:u32,pub xml_version_img:u32,pub partition_id:u32,pub bl2_running:u16,pub cfg_running:u16,pub img_running:u16,pub key_running:u16,pub reserved4:[u32;43],pub vendor_seeprom_twi:u32,pub vendor_table_revision:u32,pub vendor_specific_info:[u32;2],pub p2p_vendor_id:u16,pub p2p_device_id:u16,pub p2p_revision_id:u8,pub reserved5:[u8;3],pub p2p_class_id:u32,pub subsystem_vendor_id:u16,pub subsystem_id:u16,pub p2p_serial_number:[u32;2],pub mac_addr:[u8;6],pub reserved6:[u8;2],pub reserved7:[u32;3],pub vendor_id:[u8;8],pub product_id:[u8;24],pub product_revision:[u8;2],pub reserved8:u16 }
#[repr(C, packed)] pub union sys_info_regs_union { pub gen3: sys_info_regs_gen3, pub gen4: sys_info_regs_gen4 }
#[repr(C, packed)] pub struct sys_info_regs { pub device_id:u32,pub device_version:u32,pub firmware_version:u32,pub gen:sys_info_regs_union }

#[repr(C)] pub struct partition_info { pub address:u32,pub length:u32 }
#[repr(C)] pub struct active_partition_info_gen3 { pub address:u32,pub build_version:u32,pub build_string:u32 }
#[repr(C)] pub struct flash_info_regs_gen3 { pub flash_part_map_upd_idx:u32,pub active_img:active_partition_info_gen3,pub active_cfg:active_partition_info_gen3,pub inactive_img:active_partition_info_gen3,pub inactive_cfg:active_partition_info_gen3,pub flash_length:u32,pub cfg0:partition_info,pub cfg1:partition_info,pub img0:partition_info,pub img1:partition_info,pub nvlog:partition_info,pub vendor:[partition_info;8] }
#[repr(C)] pub struct active_partition_info_gen4 { pub bl2:u8,pub cfg:u8,pub img:u8,pub key:u8 }
#[repr(C)] pub struct flash_info_regs_gen4 { pub flash_address:u32,pub flash_length:u32,pub active_flag:active_partition_info_gen4,pub reserved:[u32;3],pub map0:partition_info,pub map1:partition_info,pub key0:partition_info,pub key1:partition_info,pub bl2_0:partition_info,pub bl2_1:partition_info,pub cfg0:partition_info,pub cfg1:partition_info,pub img0:partition_info,pub img1:partition_info,pub nvlog:partition_info,pub vendor:[partition_info;8] }
#[repr(C)] pub union flash_info_regs_union { pub gen3:flash_info_regs_gen3,pub gen4:flash_info_regs_gen4 }
#[repr(C)] pub struct flash_info_regs { pub gen:flash_info_regs_union }

pub const SWITCHTEC_NTB_REG_INFO_OFFSET:usize=0; pub const SWITCHTEC_NTB_REG_CTRL_OFFSET:usize=0x4000; pub const SWITCHTEC_NTB_REG_DBMSG_OFFSET:usize=0x64000;
#[repr(C, packed)] pub struct nt_partition_info { pub xlink_enabled:u32,pub target_part_low:u32,pub target_part_high:u32,pub reserved:u32 }
#[repr(C, packed)] pub struct ntb_info_regs { pub partition_count:u8,pub partition_id:u8,pub reserved1:u16,pub ep_map:u64,pub requester_id:u16,pub reserved2:u16,pub reserved3:[u32;4],pub ntp_info:[nt_partition_info;48] }

#[repr(C, packed)] pub struct part_cfg_regs { pub status:u32,pub state:u32,pub port_cnt:u32,pub usp_port_mode:u32,pub usp_pff_inst_id:u32,pub vep_pff_inst_id:u32,pub dsp_pff_inst_id:[u32;47],pub reserved1:[u32;11],pub vep_vector_number:u16,pub usp_vector_number:u16,pub port_event_bitmap:u32,pub reserved2:[u32;3],pub part_event_summary:u32,pub reserved3:[u32;3],pub part_reset_hdr:u32,pub part_reset_data:[u32;5],pub mrpc_comp_hdr:u32,pub mrpc_comp_data:[u32;5],pub mrpc_comp_async_hdr:u32,pub mrpc_comp_async_data:[u32;5],pub dyn_binding_hdr:u32,pub dyn_binding_data:[u32;5],pub intercomm_notify_hdr:u32,pub intercomm_notify_data:[u32;5],pub reserved4:[u32;153] }
pub const NTB_CTRL_PART_OP_LOCK:u32=1; pub const NTB_CTRL_PART_OP_CFG:u32=2; pub const NTB_CTRL_PART_OP_RESET:u32=3; pub const NTB_CTRL_PART_STATUS_NORMAL:u32=1; pub const NTB_CTRL_PART_STATUS_LOCKED:u32=2; pub const NTB_CTRL_PART_STATUS_LOCKING:u32=3; pub const NTB_CTRL_PART_STATUS_CONFIGURING:u32=4; pub const NTB_CTRL_PART_STATUS_RESETTING:u32=5; pub const NTB_CTRL_BAR_VALID:u32=1; pub const NTB_CTRL_BAR_DIR_WIN_EN:u32=1<<4; pub const NTB_CTRL_BAR_LUT_WIN_EN:u32=1<<5; pub const NTB_CTRL_REQ_ID_EN:u32=1; pub const NTB_CTRL_LUT_EN:u32=1;

#[repr(C, packed)] pub struct ntb_bar_entry { pub ctl:u32,pub win_size:u32,pub xlate_addr:u64 }
#[repr(C, packed)] pub struct ntb_bar_ext_entry { pub win_size:u32,pub reserved:[u32;3] }
#[repr(C, packed)] pub struct ntb_ctrl_regs { pub partition_status:u32,pub partition_op:u32,pub partition_ctrl:u32,pub bar_setup:u32,pub bar_error:u32,pub lut_table_entries:u16,pub lut_table_offset:u16,pub lut_error:u32,pub req_id_table_size:u16,pub req_id_table_offset:u16,pub req_id_error:u32,pub reserved1:[u32;7],pub bar_entry:[ntb_bar_entry;6],pub bar_ext_entry:[ntb_bar_ext_entry;6],pub reserved2:[u32;192],pub req_id_table:[u32;512],pub reserved3:[u32;256],pub lut_entry:[u64;512] }
pub const NTB_DBMSG_IMSG_STATUS:u64=1u64<<32; pub const NTB_DBMSG_IMSG_MASK:u64=1u64<<40;
#[repr(C, packed)] pub struct ntb_dbmsg_omsg { pub msg:u32,pub status:u32 } #[repr(C, packed)] pub struct ntb_dbmsg_imsg { pub msg:u32,pub status:u8,pub mask:u8,pub src:u8,pub reserved:u8 }
#[repr(C, packed)] pub struct ntb_dbmsg_regs { pub reserved1:[u32;1024],pub odb:u64,pub odb_mask:u64,pub idb:u64,pub idb_mask:u64,pub idb_vec_map:[u8;64],pub msg_map:u32,pub reserved2:u32,pub omsg:[ntb_dbmsg_omsg;4],pub imsg:[ntb_dbmsg_imsg;4],pub reserved3:[u8;3928],pub msix_table:[u8;1024],pub reserved4:[u8;3072],pub pba:[u8;24],pub reserved5:[u8;4072] }
pub const SWITCHTEC_PART_CFG_EVENT_RESET:u32=1; pub const SWITCHTEC_PART_CFG_EVENT_MRPC_CMP:u32=1<<1; pub const SWITCHTEC_PART_CFG_EVENT_MRPC_ASYNC_CMP:u32=1<<2; pub const SWITCHTEC_PART_CFG_EVENT_DYN_PART_CMP:u32=1<<3;

#[repr(C, packed)] pub struct pff_csr_regs { pub vendor_id:u16,pub device_id:u16,pub pcicmd:u16,pub pcists:u16,pub pci_class:u32,pub pci_opts:u32,pub pci_bar:[u32;6],pub pci_cardbus:u32,pub pci_subsystem_id:u32,pub pci_expansion_rom:u32,pub pci_cap_ptr:u32,pub reserved1:u32,pub pci_irq:u32,pub pci_cap_region:[u32;48],pub pcie_cap_region:[u32;448],pub indirect_gas_window:[u32;128],pub indirect_gas_window_off:u32,pub reserved:[u32;127],pub pff_event_summary:u32,pub reserved2:[u32;3],pub aer_in_p2p_hdr:u32,pub aer_in_p2p_data:[u32;5],pub aer_in_vep_hdr:u32,pub aer_in_vep_data:[u32;5],pub dpc_hdr:u32,pub dpc_data:[u32;5],pub cts_hdr:u32,pub cts_data:[u32;5],pub uec_hdr:u32,pub uec_data:[u32;5],pub hotplug_hdr:u32,pub hotplug_data:[u32;5],pub ier_hdr:u32,pub ier_data:[u32;5],pub threshold_hdr:u32,pub threshold_data:[u32;5],pub power_mgmt_hdr:u32,pub power_mgmt_data:[u32;5],pub tlp_throttling_hdr:u32,pub tlp_throttling_data:[u32;5],pub force_speed_hdr:u32,pub force_speed_data:[u32;5],pub credit_timeout_hdr:u32,pub credit_timeout_data:[u32;5],pub link_state_hdr:u32,pub link_state_data:[u32;5],pub reserved4:[u32;174] }

pub struct switchtec_ntb;
#[repr(C)] pub struct dma_mrpc_output { pub status:u32,pub cmd_id:u32,pub rtn_code:u32,pub output_size:u32,pub data:[u8;1024] }
#[repr(C)] pub struct switchtec_dev { pub pdev:*mut c_void,pub dev:c_void,pub cdev:c_void,pub gen:switchtec_gen,pub partition:i32,pub partition_count:i32,pub pff_csr_count:i32,pub pff_local:[i8;255],pub mmio:*mut c_void,pub mmio_mrpc:*mut mrpc_regs,pub mmio_sw_event:*mut sw_event_regs,pub mmio_sys_info:*mut sys_info_regs,pub mmio_flash_info:*mut flash_info_regs,pub mmio_ntb:*mut ntb_info_regs,pub mmio_part_cfg:*mut part_cfg_regs,pub mmio_part_cfg_all:*mut part_cfg_regs,pub mmio_pff_csr:*mut pff_csr_regs,pub mrpc_mutex:c_void,pub mrpc_queue:c_void,pub mrpc_busy:i32,pub mrpc_work:c_void,pub mrpc_timeout:c_void,pub alive:bool,pub event_wq:c_void,pub event_cnt:c_void,pub link_event_work:c_void,pub link_notifier:Option<unsafe extern "C" fn(*mut switchtec_dev)>,pub link_event_count:[u8;255],pub sndev:*mut switchtec_ntb,pub dma_mrpc:*mut dma_mrpc_output,pub dma_mrpc_dma_addr:usize }
#[inline] pub unsafe fn to_stdev(dev:*mut c_void)->*mut switchtec_dev { dev as *mut switchtec_dev }
extern "C" { pub static switchtec_class: c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
