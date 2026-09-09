/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/* Copyright(c) 2015-17 Intel Corporation. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Linux headers and CONFIG_SOUNDWIRE are external dependencies. */
use core::ffi::c_void;

pub type __u8 = u8;
pub type __u16 = u16;
pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type size_t = usize;

#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct irq_chip { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct sdca_device_data { _private: [u8; 0] }
#[repr(C)] pub struct sdw_msg { _private: [u8; 0] }
#[repr(C)] pub struct sdw_bpt_msg { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct sdw_device_id { pub mfg_id: u16, pub part_id: u16, pub sdw_version: u8, pub class_id: u8, pub driver_data: usize }
#[repr(C)] pub struct sdw_bus { _private: [u8; 0] }
#[repr(C)] pub struct sdw_slave { _private: [u8; 0] }

pub const SDW_BROADCAST_DEV_NUM: u32 = 15;
pub const SDW_ENUM_DEV_NUM: u32 = 0;
pub const SDW_GROUP12_DEV_NUM: u32 = 12;
pub const SDW_GROUP13_DEV_NUM: u32 = 13;
pub const SDW_MASTER_DEV_NUM: u32 = 14;
pub const SDW_NUM_DEV_ID_REGISTERS: u32 = 6;
pub const SDW_FRAME_ROWS: u32 = 24;
pub const SDW_FRAME_COLS: u32 = 8;
pub const SDW_FRAME_ROW_COLS: u32 = SDW_FRAME_ROWS * SDW_FRAME_COLS;
pub const SDW_FRAME_CTRL_BITS: u32 = 48;
pub const SDW_MAX_DEVICES: u32 = 11;
pub const SDW_FW_MAX_DEVICES: u32 = 16;
pub const SDW_MAX_PORTS: u32 = 15;
pub const SDW_MAX_LANES: u32 = 8;
pub const SDW_PORT_FLOW_MODE_ISOCH: u32 = 0;
pub const SDW_PORT_FLOW_MODE_TX_CNTRL: u32 = 1;
pub const SDW_PORT_FLOW_MODE_RX_CNTRL: u32 = 2;
pub const SDW_PORT_FLOW_MODE_ASYNC: u32 = 3;
pub const SDW_BLOCK_PACKG_PER_PORT: u32 = 1;
pub const SDW_BLOCK_PACKG_PER_CH: u32 = 2;
pub const SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: u32 = 1;
pub const SDW_MASTER_QUIRKS_CLEAR_INITIAL_CLASH: u64 = 1;
pub const SDW_MASTER_QUIRKS_CLEAR_INITIAL_PARITY: u64 = 2;
pub const SDW_IGNORED_UNIQUE_ID: u32 = 0xff;
pub const SDW_BPT_MSG_MAX_BYTES: u32 = 1024 * 1024;
pub const SDW_BRA_MAX_BYTES_PER_FRAME: u32 = 470;

#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_port_dirn { SDW_PORT_DIRN_SINK=0, SDW_PORT_DIRN_SOURCE=1, SDW_PORT_DIRN_MAX=2 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_slave_status { SDW_SLAVE_UNATTACHED=0, SDW_SLAVE_ATTACHED=1, SDW_SLAVE_ALERT=2, SDW_SLAVE_RESERVED=3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_clk_stop_type { SDW_CLK_PRE_PREPARE=0, SDW_CLK_POST_PREPARE, SDW_CLK_PRE_DEPREPARE, SDW_CLK_POST_DEPREPARE }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_command_response { SDW_CMD_OK=0, SDW_CMD_IGNORED=1, SDW_CMD_FAIL=2, SDW_CMD_TIMEOUT=3, SDW_CMD_FAIL_OTHER=4 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_dpn_grouping { SDW_BLK_GRP_CNT_1=0, SDW_BLK_GRP_CNT_2=1, SDW_BLK_GRP_CNT_3=2, SDW_BLK_GRP_CNT_4=3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_dpn_pkg_mode { SDW_BLK_PKG_PER_PORT=0, SDW_BLK_PKG_PER_CHANNEL=1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_stream_type { SDW_STREAM_PCM=0, SDW_STREAM_PDM=1, SDW_STREAM_BPT=2 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_data_direction { SDW_DATA_DIR_RX=0, SDW_DATA_DIR_TX=1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_port_data_mode { SDW_PORT_DATA_MODE_NORMAL=0, SDW_PORT_DATA_MODE_PRBS=1, SDW_PORT_DATA_MODE_STATIC_0=2, SDW_PORT_DATA_MODE_STATIC_1=3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_clk_stop_reset_behave { SDW_CLK_STOP_KEEP_STATUS=1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_p15_behave { SDW_P15_READ_IGNORED=0, SDW_P15_CMD_OK=1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_dpn_type { SDW_DPN_FULL=0, SDW_DPN_SIMPLE=1, SDW_DPN_REDUCED=2 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_clk_stop_mode { SDW_CLK_STOP_MODE0=0, SDW_CLK_STOP_MODE1=1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_reg_bank { SDW_BANK0=0, SDW_BANK1=1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_port_prep_ops { SDW_OPS_PORT_PRE_PREP=0, SDW_OPS_PORT_PRE_DEPREP, SDW_OPS_PORT_POST_PREP, SDW_OPS_PORT_POST_DEPREP }
#[repr(C)] #[derive(Copy, Clone)] pub enum sdw_stream_state { SDW_STREAM_ALLOCATED=0, SDW_STREAM_CONFIGURED=1, SDW_STREAM_PREPARED=2, SDW_STREAM_ENABLED=3, SDW_STREAM_DISABLED=4, SDW_STREAM_DEPREPARED=5, SDW_STREAM_RELEASED=6 }

#[repr(C)] pub struct sdw_dp0_prop { pub words:*mut u32,pub max_word:u32,pub min_word:u32,pub num_words:u32,pub ch_prep_timeout:u32,pub BRA_flow_controlled:bool,pub simple_ch_prep_sm:bool,pub imp_def_interrupts:bool,pub num_lanes:i32,pub lane_list:*mut u32 }
#[repr(C)] pub struct sdw_dpn_prop { pub num:u32,pub max_word:u32,pub min_word:u32,pub num_words:u32,pub words:*mut u32,pub r#type:sdw_dpn_type,pub max_grouping:u32,pub ch_prep_timeout:u32,pub imp_def_interrupts:u32,pub max_ch:u32,pub min_ch:u32,pub num_channels:u32,pub num_ch_combinations:u32,pub channels:*mut u32,pub ch_combinations:*mut u32,pub lane_list:*mut u32,pub num_lanes:i32,pub modes:u32,pub max_async_buffer:u32,pub port_encoding:u32,pub block_pack_mode:bool,pub read_only_wordlength:bool,pub simple_ch_prep_sm:bool }
#[repr(C)] pub struct sdw_slave_prop { pub dp0_prop:*mut sdw_dp0_prop,pub src_dpn_prop:*mut sdw_dpn_prop,pub sink_dpn_prop:*mut sdw_dpn_prop,pub mipi_revision:u32,pub wake_capable:bool,pub test_mode_capable:bool,pub clk_stop_mode1:bool,pub simple_clk_stop_capable:bool,pub clk_stop_timeout:u32,pub ch_prep_timeout:u32,pub reset_behave:sdw_clk_stop_reset_behave,pub high_PHY_capable:bool,pub paging_support:bool,pub bank_delay_support:bool,pub lane_control_support:bool,pub p15_behave:sdw_p15_behave,pub master_count:u32,pub source_ports:u32,pub sink_ports:u32,pub quirks:u32,pub sdca_interrupt_register_list:u32,pub commit_register_supported:u8,pub scp_int1_mask:u8,pub lane_maps:[u8;8],pub bra_block_alignment:u32,pub bra_max_data_per_frame:u32,pub clock_reg_supported:bool,pub use_domain_irq:bool }
#[repr(C)] pub struct sdw_master_prop { pub clk_gears:*mut u32,pub clk_freq:*mut u32,pub quirks:u64,pub revision:u32,pub clk_stop_modes:u32,pub max_clk_freq:u32,pub num_clk_gears:u32,pub num_clk_freq:u32,pub default_frame_rate:u32,pub default_row:u32,pub default_col:u32,pub err_threshold:u32,pub mclk_freq:u32,pub dynamic_frame:bool,pub hw_disabled:bool }
#[repr(C)] pub struct sdw_slave_id { pub mfg_id:u16,pub part_id:u16,pub class_id:u8,pub unique_id:u8,pub sdw_version:u8 }
#[repr(C)] pub struct sdw_peripherals { pub num_peripherals:i32,pub array:[*mut sdw_slave;0] }
#[repr(C)] pub struct sdw_slave_intr_status { pub sdca_cascade:bool,pub control_port:u8,pub port:[u8;15] }
#[repr(C)] pub struct sdw_prepare_ch { pub num:u32,pub ch_mask:u32,pub prepare:bool,pub bank:u32 }
#[repr(C)] pub struct sdw_bus_params { pub curr_bank:sdw_reg_bank,pub next_bank:sdw_reg_bank,pub max_dr_freq:u32,pub curr_dr_freq:u32,pub bandwidth:u32,pub col:u32,pub row:u32,pub s_data_mode:i32,pub m_data_mode:i32 }
#[repr(C)] pub struct sdw_port_params { pub num:u32,pub bps:u32,pub flow_mode:u32,pub data_mode:u32 }
#[repr(C)] pub struct sdw_transport_params { pub blk_grp_ctrl_valid:bool,pub port_num:u32,pub blk_grp_ctrl:u32,pub sample_interval:u32,pub offset1:u32,pub offset2:u32,pub hstart:u32,pub hstop:u32,pub blk_pkg_mode:u32,pub lane_ctrl:u32 }
#[repr(C)] pub struct sdw_enable_ch { pub port_num:u32,pub ch_mask:u32,pub enable:bool }
#[repr(C)] pub struct sdw_port_config { pub num:u32,pub ch_mask:u32 }
#[repr(C)] pub struct sdw_stream_config { pub frame_rate:u32,pub ch_count:u32,pub bps:u32,pub direction:sdw_data_direction,pub r#type:sdw_stream_type }
#[repr(C)] pub struct sdw_stream_params { pub rate:u32,pub ch_count:u32,pub bps:u32 }
#[repr(C)] pub struct sdw_stream_runtime { pub name:*const i8,pub params:sdw_stream_params,pub state:sdw_stream_state,pub r#type:sdw_stream_type,pub m_rt_count:i32,pub master_list:list_head }
#[repr(C)] pub struct sdw_defer { pub msg:*mut sdw_msg,pub length:i32,pub complete:completion }

/* Callback-bearing structures retain C ABI function-pointer signatures. */
#[repr(C)] pub struct sdw_slave_ops { pub read_prop:Option<unsafe extern "C" fn(*mut sdw_slave)->i32>, pub interrupt_callback:Option<unsafe extern "C" fn(*mut sdw_slave,*mut sdw_slave_intr_status)->i32>, pub update_status:Option<unsafe extern "C" fn(*mut sdw_slave,sdw_slave_status)->i32>, pub bus_config:Option<unsafe extern "C" fn(*mut sdw_slave,*mut sdw_bus_params)->i32>, pub port_prep:Option<unsafe extern "C" fn(*mut sdw_slave,*mut sdw_prepare_ch,sdw_port_prep_ops)->i32>, pub clk_stop:Option<unsafe extern "C" fn(*mut sdw_slave,sdw_clk_stop_mode,sdw_clk_stop_type)->i32> }
#[repr(C)] pub struct sdw_master_port_ops { pub dpn_set_port_params:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_port_params,u32)->i32>, pub dpn_set_port_transport_params:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_transport_params,sdw_reg_bank)->i32>, pub dpn_port_prep:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_prepare_ch)->i32>, pub dpn_port_enable_ch:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_enable_ch,u32)->i32> }
#[repr(C)] pub struct sdw_master_ops { pub read_prop:Option<unsafe extern "C" fn(*mut sdw_bus)->i32>, pub override_adr:Option<unsafe extern "C" fn(*mut sdw_bus,u64)->u64>, pub xfer_msg:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_msg)->sdw_command_response>, pub xfer_msg_defer:Option<unsafe extern "C" fn(*mut sdw_bus)->sdw_command_response>, pub set_bus_conf:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_bus_params)->i32>, pub pre_bank_switch:Option<unsafe extern "C" fn(*mut sdw_bus)->i32>, pub post_bank_switch:Option<unsafe extern "C" fn(*mut sdw_bus)->i32>, pub read_ping_status:Option<unsafe extern "C" fn(*mut sdw_bus)->u32>, pub get_device_num:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_slave)->i32>, pub put_device_num:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_slave)>, pub new_peripheral_assigned:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_slave,i32)>, pub bpt_send_async:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_slave,*mut sdw_bpt_msg)->i32>, pub bpt_wait:Option<unsafe extern "C" fn(*mut sdw_bus,*mut sdw_slave,*mut sdw_bpt_msg)->i32> }
#[repr(C)] pub struct sdw_driver { pub probe:Option<unsafe extern "C" fn(*mut sdw_slave,*const sdw_device_id)->i32>, pub remove:Option<unsafe extern "C" fn(*mut sdw_slave)>, pub shutdown:Option<unsafe extern "C" fn(*mut sdw_slave)>, pub id_table:*const sdw_device_id, pub ops:*const sdw_slave_ops, pub driver:device_driver }

#[repr(C)] pub struct sdw_master_device { pub dev:device,pub bus:*mut sdw_bus }

extern "C" {
 pub fn sdw_master_read_prop(bus:*mut sdw_bus)->i32; pub fn sdw_slave_read_prop(slave:*mut sdw_slave)->i32; pub fn sdw_slave_read_lane_mapping(slave:*mut sdw_slave)->i32;
 pub fn sdw_handle_slave_status(bus:*mut sdw_bus,status:*mut sdw_slave_status)->i32; pub fn sdw_bus_master_add(bus:*mut sdw_bus,parent:*mut device,fwnode:*mut fwnode_handle)->i32; pub fn sdw_bus_master_delete(bus:*mut sdw_bus); pub fn sdw_show_ping_status(bus:*mut sdw_bus,sync_delay:bool);
 pub fn sdw_alloc_stream(name:*const i8,r#type:sdw_stream_type)->*mut sdw_stream_runtime; pub fn sdw_release_stream(stream:*mut sdw_stream_runtime); pub fn sdw_compute_params(bus:*mut sdw_bus,stream:*mut sdw_stream_runtime)->i32;
 pub fn sdw_stream_add_master(bus:*mut sdw_bus,config:*mut sdw_stream_config,ports:*const sdw_port_config,num_ports:u32,stream:*mut sdw_stream_runtime)->i32; pub fn sdw_stream_remove_master(bus:*mut sdw_bus,stream:*mut sdw_stream_runtime)->i32; pub fn sdw_startup_stream(substream:*mut c_void)->i32; pub fn sdw_prepare_stream(stream:*mut sdw_stream_runtime)->i32; pub fn sdw_enable_stream(stream:*mut sdw_stream_runtime)->i32; pub fn sdw_disable_stream(stream:*mut sdw_stream_runtime)->i32; pub fn sdw_deprepare_stream(stream:*mut sdw_stream_runtime)->i32; pub fn sdw_shutdown_stream(substream:*mut c_void);
 pub fn sdw_bus_prep_clk_stop(bus:*mut sdw_bus)->i32; pub fn sdw_bus_clk_stop(bus:*mut sdw_bus)->i32; pub fn sdw_bus_exit_clk_stop(bus:*mut sdw_bus)->i32; pub fn sdw_compare_devid(slave:*mut sdw_slave,id:sdw_slave_id)->i32; pub fn sdw_extract_slave_id(bus:*mut sdw_bus,addr:u64,id:*mut sdw_slave_id); pub fn is_clock_scaling_supported_by_slave(slave:*mut sdw_slave)->bool;
 pub fn sdw_bpt_send_async(bus:*mut sdw_bus,slave:*mut sdw_slave,msg:*mut sdw_bpt_msg)->i32; pub fn sdw_bpt_wait(bus:*mut sdw_bus,slave:*mut sdw_slave,msg:*mut sdw_bpt_msg)->i32; pub fn sdw_bpt_send_sync(bus:*mut sdw_bus,slave:*mut sdw_slave,msg:*mut sdw_bpt_msg)->i32;
 pub fn sdw_stream_add_slave(slave:*mut sdw_slave,config:*mut sdw_stream_config,ports:*const sdw_port_config,num_ports:u32,stream:*mut sdw_stream_runtime)->i32; pub fn sdw_stream_remove_slave(slave:*mut sdw_slave,stream:*mut sdw_stream_runtime)->i32; pub fn of_sdw_find_device_by_node(np:*mut device_node)->*mut device; pub fn sdw_slave_get_current_bank(sdev:*mut sdw_slave)->i32; pub fn sdw_slave_get_scale_index(slave:*mut sdw_slave,base:*mut u8)->i32;
 pub fn sdw_read(slave:*mut sdw_slave,addr:u32)->i32; pub fn sdw_write(slave:*mut sdw_slave,addr:u32,value:u8)->i32; pub fn sdw_write_no_pm(slave:*mut sdw_slave,addr:u32,value:u8)->i32; pub fn sdw_read_no_pm(slave:*mut sdw_slave,addr:u32)->i32; pub fn sdw_nread(slave:*mut sdw_slave,addr:u32,count:size_t,val:*mut u8)->i32; pub fn sdw_nread_no_pm(slave:*mut sdw_slave,addr:u32,count:size_t,val:*mut u8)->i32; pub fn sdw_nwrite(slave:*mut sdw_slave,addr:u32,count:size_t,val:*const u8)->i32; pub fn sdw_nwrite_no_pm(slave:*mut sdw_slave,addr:u32,count:size_t,val:*const u8)->i32; pub fn sdw_update(slave:*mut sdw_slave,addr:u32,mask:u8,val:u8)->i32; pub fn sdw_update_no_pm(slave:*mut sdw_slave,addr:u32,mask:u8,val:u8)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
