/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust source-level translation of v4l2-subdev.h. */

/* Types supplied by the included kernel/media headers remain external. */
pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type ssize_t = isize;
pub type size_t = usize;
pub type v4l2_std_id = u64;

pub enum v4l2_device {}
pub enum v4l2_ctrl_handler {}
pub enum v4l2_event {}
pub enum v4l2_event_subscription {}
pub enum v4l2_fh {}
pub enum v4l2_subdev_fh {}
pub enum v4l2_subdev_stream_config {}
pub enum tuner_setup {}
pub enum v4l2_subdev_route {}
pub enum v4l2_dbg_register {}
pub enum v4l2_frequency {}
pub enum v4l2_frequency_band {}
pub enum v4l2_tuner {}
pub enum v4l2_modulator {}
pub enum v4l2_priv_tun_config {}
pub enum v4l2_sliced_vbi_data {}
pub enum v4l2_sliced_vbi_cap {}
pub enum v4l2_vbi_format {}
pub enum v4l2_sliced_vbi_format {}
pub enum v4l2_mbus_framefmt {}
pub enum v4l2_rect {}
pub enum v4l2_fract {}
pub enum v4l2_subdev_mbus_code_enum {}
pub enum v4l2_subdev_frame_size_enum {}
pub enum v4l2_subdev_frame_interval_enum {}
pub enum v4l2_subdev_format {}
pub enum v4l2_subdev_selection {}
pub enum v4l2_subdev_frame_interval {}
pub enum v4l2_edid {}
pub enum v4l2_dv_timings {}
pub enum v4l2_dv_timings_cap {}
pub enum v4l2_enum_dv_timings {}
pub enum media_entity {}
pub enum media_link {}
pub enum module {}
pub enum video_device {}
pub enum device {}
pub enum fwnode_handle {}
pub enum fwnode_endpoint {}
pub enum v4l2_async_notifier {}
pub enum regulator_bulk_data {}
pub enum led_classdev {}
pub enum mutex {}
pub enum list_head {}
pub enum lock_class_key {}
pub enum v4l2_file_operations {}
pub enum v4l2_mbus_config {}

pub const V4L2_SUBDEV_IR_RX_NOTIFY: u32 = 0; /* _IOW('v', 0, u32), external ioctl encoding */
pub const V4L2_SUBDEV_IR_RX_FIFO_SERVICE_REQ: u32 = 0x00000001;
pub const V4L2_SUBDEV_IR_RX_END_OF_RX_DETECTED: u32 = 0x00000002;
pub const V4L2_SUBDEV_IR_RX_HW_FIFO_OVERRUN: u32 = 0x00000004;
pub const V4L2_SUBDEV_IR_RX_SW_FIFO_OVERRUN: u32 = 0x00000008;
pub const V4L2_SUBDEV_IR_TX_NOTIFY: u32 = 0; /* _IOW('v', 1, u32), external ioctl encoding */
pub const V4L2_SUBDEV_IR_TX_FIFO_SERVICE_REQ: u32 = 0x00000001;
pub const V4L2_DEVICE_NOTIFY_EVENT: u32 = 0; /* _IOW('v', 2, struct v4l2_event) */

#[repr(C)]
pub struct v4l2_decode_vbi_line { pub is_second_field: u32, pub p: *mut u8, pub line: u32, pub type_: u32 }

#[repr(C)]
pub enum v4l2_subdev_io_pin_bits { V4L2_SUBDEV_IO_PIN_DISABLE=0, V4L2_SUBDEV_IO_PIN_OUTPUT=1, V4L2_SUBDEV_IO_PIN_INPUT=2, V4L2_SUBDEV_IO_PIN_SET_VALUE=3, V4L2_SUBDEV_IO_PIN_ACTIVE_LOW=4 }
#[repr(C)]
pub struct v4l2_subdev_io_pin_config { pub flags:u32, pub pin:u8, pub function:u8, pub value:u8, pub strength:u8 }

#[repr(C)] pub struct v4l2_subdev_core_ops {
 pub log_status: Option<unsafe extern "C" fn(*mut v4l2_subdev)->core::ffi::c_int>,
 pub s_io_pin_config: Option<unsafe extern "C" fn(*mut v4l2_subdev,size_t,*mut v4l2_subdev_io_pin_config)->core::ffi::c_int>,
 pub init: Option<unsafe extern "C" fn(*mut v4l2_subdev,u32)->core::ffi::c_int>, pub load_fw: Option<unsafe extern "C" fn(*mut v4l2_subdev)->core::ffi::c_int>, pub reset: Option<unsafe extern "C" fn(*mut v4l2_subdev,u32)->core::ffi::c_int>, pub s_gpio: Option<unsafe extern "C" fn(*mut v4l2_subdev,u32)->core::ffi::c_int>,
 pub command: Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut core::ffi::c_void)->isize>, pub ioctl: Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut core::ffi::c_void)->isize>,
 /* CONFIG_COMPAT: compat_ioctl32 */ pub compat_ioctl32: Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,usize)->isize>,
 /* CONFIG_VIDEO_ADV_DEBUG */ pub g_register: Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_dbg_register)->core::ffi::c_int>, pub s_register: Option<unsafe extern "C" fn(*mut v4l2_subdev,*const v4l2_dbg_register)->core::ffi::c_int>,
 pub s_power: Option<unsafe extern "C" fn(*mut v4l2_subdev,core::ffi::c_int)->core::ffi::c_int>, pub interrupt_service_routine: Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut bool)->core::ffi::c_int>,
 pub subscribe_event: Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_fh,*mut v4l2_event_subscription)->core::ffi::c_int>, pub unsubscribe_event: Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_fh,*mut v4l2_event_subscription)->core::ffi::c_int>,
}

#[repr(C)] pub struct v4l2_subdev_tuner_ops { pub standby:Option<unsafe extern "C" fn(*mut v4l2_subdev)->i32>, pub s_radio:Option<unsafe extern "C" fn(*mut v4l2_subdev)->i32>, pub s_frequency:Option<unsafe extern "C" fn(*mut v4l2_subdev,*const v4l2_frequency)->i32>, pub g_frequency:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_frequency)->i32>, pub enum_freq_bands:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_frequency_band)->i32>, pub g_tuner:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_tuner)->i32>, pub s_tuner:Option<unsafe extern "C" fn(*mut v4l2_subdev,*const v4l2_tuner)->i32>, pub g_modulator:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_modulator)->i32>, pub s_modulator:Option<unsafe extern "C" fn(*mut v4l2_subdev,*const v4l2_modulator)->i32>, pub s_type_addr:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut tuner_setup)->i32>, pub s_config:Option<unsafe extern "C" fn(*mut v4l2_subdev,*const v4l2_priv_tun_config)->i32> }
#[repr(C)] pub struct v4l2_subdev_audio_ops { pub s_clock_freq:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32)->i32>, pub s_i2s_clock_freq:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32)->i32>, pub s_routing:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,u32,u32)->i32>, pub s_stream:Option<unsafe extern "C" fn(*mut v4l2_subdev,i32)->i32> }
#[repr(C)] pub struct v4l2_mbus_frame_desc_entry_csi2 { pub vc:u8, pub dt:u8 }
#[repr(C)] pub enum v4l2_mbus_frame_desc_flags { V4L2_MBUS_FRAME_DESC_FL_LEN_MAX=1, V4L2_MBUS_FRAME_DESC_FL_BLOB=2 }
#[repr(C)] pub union v4l2_mbus_frame_desc_entry_bus { pub csi2:v4l2_mbus_frame_desc_entry_csi2 }
#[repr(C)] pub struct v4l2_mbus_frame_desc_entry { pub flags:v4l2_mbus_frame_desc_flags, pub stream:u32, pub pixelcode:u32, pub length:u32, pub bus:v4l2_mbus_frame_desc_entry_bus }
pub const V4L2_FRAME_DESC_ENTRY_MAX:usize=8;
#[repr(C)] pub enum v4l2_mbus_frame_desc_type { V4L2_MBUS_FRAME_DESC_TYPE_UNDEFINED=0, V4L2_MBUS_FRAME_DESC_TYPE_PARALLEL, V4L2_MBUS_FRAME_DESC_TYPE_CSI2 }
#[repr(C)] pub struct v4l2_mbus_frame_desc { pub type_:v4l2_mbus_frame_desc_type, pub entry:[v4l2_mbus_frame_desc_entry;V4L2_FRAME_DESC_ENTRY_MAX], pub num_entries:u16 }
#[repr(C)] pub enum v4l2_subdev_pre_streamon_flags { V4L2_SUBDEV_PRE_STREAMON_FL_MANUAL_LP=1 }

#[repr(C)] pub struct v4l2_subdev_video_ops { pub s_routing:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,u32,u32)->i32>, pub s_crystal_freq:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,u32)->i32>, pub g_std:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_std_id)->i32>, pub s_std:Option<unsafe extern "C" fn(*mut v4l2_subdev,v4l2_std_id)->i32>, pub s_std_output:Option<unsafe extern "C" fn(*mut v4l2_subdev,v4l2_std_id)->i32>, pub g_std_output:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_std_id)->i32>, pub querystd:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_std_id)->i32>, pub g_tvnorms:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_std_id)->i32>, pub g_tvnorms_output:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_std_id)->i32>, pub g_input_status:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut u32)->i32>, pub s_stream:Option<unsafe extern "C" fn(*mut v4l2_subdev,i32)->i32>, pub s_rx_buffer:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut core::ffi::c_void,*mut u32)->i32>, pub pre_streamon:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32)->i32>, pub post_streamoff:Option<unsafe extern "C" fn(*mut v4l2_subdev)->i32> }
#[repr(C)] pub struct v4l2_subdev_vbi_ops { pub decode_vbi_line:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_decode_vbi_line)->i32>, pub s_vbi_data:Option<unsafe extern "C" fn(*mut v4l2_subdev,*const v4l2_sliced_vbi_data)->i32>, pub g_vbi_data:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_sliced_vbi_data)->i32>, pub g_sliced_vbi_cap:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_sliced_vbi_cap)->i32>, pub s_raw_fmt:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_vbi_format)->i32>, pub g_sliced_fmt:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_sliced_vbi_format)->i32>, pub s_sliced_fmt:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_sliced_vbi_format)->i32> }
#[repr(C)] pub struct v4l2_subdev_sensor_ops { pub g_skip_top_lines:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut u32)->i32>, pub g_skip_frames:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut u32)->i32> }
#[repr(C)] pub enum v4l2_subdev_ir_mode { V4L2_SUBDEV_IR_MODE_PULSE_WIDTH }
#[repr(C)] pub struct v4l2_subdev_ir_parameters { pub bytes_per_data_element:u32, pub mode:v4l2_subdev_ir_mode, pub enable:bool, pub interrupt_enable:bool, pub shutdown:bool, pub modulation:bool, pub max_pulse_width:u32, pub carrier_freq:u32, pub duty_cycle:u32, pub invert_level:bool, pub invert_carrier_sense:bool, pub noise_filter_min_width:u32, pub carrier_range_lower:u32, pub carrier_range_upper:u32, pub resolution:u32 }
#[repr(C)] pub struct v4l2_subdev_ir_ops { pub rx_read:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut u8,size_t,*mut ssize_t)->i32>, pub rx_g_parameters:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_ir_parameters)->i32>, pub rx_s_parameters:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_ir_parameters)->i32>, pub tx_write:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut u8,size_t,*mut ssize_t)->i32>, pub tx_g_parameters:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_ir_parameters)->i32>, pub tx_s_parameters:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_ir_parameters)->i32> }

#[repr(C)] pub struct v4l2_subdev_pad_config { pub format:v4l2_mbus_framefmt, pub crop:v4l2_rect, pub compose:v4l2_rect, pub interval:v4l2_fract }
#[repr(C)] pub struct v4l2_subdev_stream_configs { pub num_configs:u32, pub configs:*mut v4l2_subdev_stream_config }
#[repr(C)] pub struct v4l2_subdev_krouting { pub len_routes:u32, pub num_routes:u32, pub routes:*mut v4l2_subdev_route }
#[repr(C)] pub struct v4l2_subdev_state { pub _lock:mutex, pub lock:*mut mutex, pub sd:*mut v4l2_subdev, pub pads:*mut v4l2_subdev_pad_config, pub routing:v4l2_subdev_krouting, pub stream_configs:v4l2_subdev_stream_configs }

#[repr(C)] pub struct v4l2_subdev_pad_ops { pub enum_mbus_code:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_mbus_code_enum)->i32>, pub enum_frame_size:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_frame_size_enum)->i32>, pub enum_frame_interval:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_frame_interval_enum)->i32>, pub get_fmt:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_format)->i32>, pub set_fmt:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_format)->i32>, pub get_selection:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_selection)->i32>, pub set_selection:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_selection)->i32>, pub get_frame_interval:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_frame_interval)->i32>, pub set_frame_interval:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,*mut v4l2_subdev_frame_interval)->i32>, pub get_edid:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_edid)->i32>, pub set_edid:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_edid)->i32>, pub s_dv_timings:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut v4l2_dv_timings)->i32>, pub g_dv_timings:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut v4l2_dv_timings)->i32>, pub query_dv_timings:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut v4l2_dv_timings)->i32>, pub dv_timings_cap:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_dv_timings_cap)->i32>, pub enum_dv_timings:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_enum_dv_timings)->i32>, pub link_validate:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut media_link,*mut v4l2_subdev_format,*mut v4l2_subdev_format)->i32>, pub get_frame_desc:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut v4l2_mbus_frame_desc)->i32>, pub set_frame_desc:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut v4l2_mbus_frame_desc)->i32>, pub get_mbus_config:Option<unsafe extern "C" fn(*mut v4l2_subdev,u32,*mut v4l2_mbus_config)->i32>, pub set_routing:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,u32,*mut v4l2_subdev_krouting)->i32>, pub enable_streams:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,u32,u64)->i32>, pub disable_streams:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state,u32,u64)->i32> }
#[repr(C)] pub struct v4l2_subdev_ops { pub core:*const v4l2_subdev_core_ops, pub tuner:*const v4l2_subdev_tuner_ops, pub audio:*const v4l2_subdev_audio_ops, pub video:*const v4l2_subdev_video_ops, pub vbi:*const v4l2_subdev_vbi_ops, pub ir:*const v4l2_subdev_ir_ops, pub sensor:*const v4l2_subdev_sensor_ops, pub pad:*const v4l2_subdev_pad_ops }
#[repr(C)] pub struct v4l2_subdev_internal_ops { pub init_state:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_state)->i32>, pub registered:Option<unsafe extern "C" fn(*mut v4l2_subdev)->i32>, pub unregistered:Option<unsafe extern "C" fn(*mut v4l2_subdev)>, pub open:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_fh)->i32>, pub close:Option<unsafe extern "C" fn(*mut v4l2_subdev,*mut v4l2_subdev_fh)->i32>, pub release:Option<unsafe extern "C" fn(*mut v4l2_subdev)> }

pub const V4L2_SUBDEV_FL_IS_I2C:u32=1<<0; pub const V4L2_SUBDEV_FL_IS_SPI:u32=1<<1; pub const V4L2_SUBDEV_FL_HAS_DEVNODE:u32=1<<2; pub const V4L2_SUBDEV_FL_HAS_EVENTS:u32=1<<3; pub const V4L2_SUBDEV_FL_STREAMS:u32=1<<4;
#[repr(C)] pub struct v4l2_subdev_platform_data { pub regulators:*mut regulator_bulk_data, pub num_regulators:i32, pub host_priv:*mut core::ffi::c_void }
#[repr(C)] pub struct v4l2_subdev { pub entity:media_entity, pub list:list_head, pub owner:*mut module, pub owner_v4l2_dev:bool, pub flags:u32, pub v4l2_dev:*mut v4l2_device, pub ops:*const v4l2_subdev_ops, pub internal_ops:*const v4l2_subdev_internal_ops, pub ctrl_handler:*mut v4l2_ctrl_handler, pub name:[u8;52], pub grp_id:u32, pub dev_priv:*mut core::ffi::c_void, pub host_priv:*mut core::ffi::c_void, pub devnode:*mut video_device, pub dev:*mut device, pub fwnode:*mut fwnode_handle, pub async_list:list_head, pub async_subdev_endpoint_list:list_head, pub subdev_notifier:*mut v4l2_async_notifier, pub asc_list:list_head, pub pdata:*mut v4l2_subdev_platform_data, pub state_lock:*mut mutex, pub privacy_led:*mut led_classdev, pub active_state:*mut v4l2_subdev_state, pub enabled_pads:u64, pub s_stream_enabled:bool }
#[repr(C)] pub struct v4l2_subdev_fh { pub vfh:v4l2_fh, pub owner:*mut module, pub state:*mut v4l2_subdev_state, pub client_caps:u64 }

extern "C" { pub static v4l2_subdev_fops:v4l2_file_operations; pub fn v4l2_subdev_init(sd:*mut v4l2_subdev,ops:*const v4l2_subdev_ops); pub static v4l2_subdev_call_wrappers:v4l2_subdev_ops; pub fn v4l2_subdev_notify_event(sd:*mut v4l2_subdev,ev:*const v4l2_event); pub fn v4l2_subdev_is_streaming(sd:*mut v4l2_subdev)->bool; }
pub unsafe fn v4l2_set_subdevdata(sd:*mut v4l2_subdev,p:*mut core::ffi::c_void){(*sd).dev_priv=p}
pub unsafe fn v4l2_get_subdevdata(sd:*const v4l2_subdev)->*mut core::ffi::c_void{(*sd).dev_priv}
pub unsafe fn v4l2_set_subdev_hostdata(sd:*mut v4l2_subdev,p:*mut core::ffi::c_void){(*sd).host_priv=p}
pub unsafe fn v4l2_get_subdev_hostdata(sd:*const v4l2_subdev)->*mut core::ffi::c_void{(*sd).host_priv}
pub unsafe fn v4l2_subdev_lock_state(state:*mut v4l2_subdev_state){ extern "C"{fn mutex_lock(m:*mut mutex);} mutex_lock((*state).lock) }
pub unsafe fn v4l2_subdev_unlock_state(state:*mut v4l2_subdev_state){ extern "C"{fn mutex_unlock(m:*mut mutex);} mutex_unlock((*state).lock) }
pub unsafe fn v4l2_subdev_lock_and_get_active_state(sd:*mut v4l2_subdev)->*mut v4l2_subdev_state{let s=(*sd).active_state;if !s.is_null(){v4l2_subdev_lock_state(s)};s}
pub unsafe fn v4l2_subdev_get_unlocked_active_state(sd:*mut v4l2_subdev)->*mut v4l2_subdev_state{(*sd).active_state}
pub unsafe fn v4l2_subdev_get_locked_active_state(sd:*mut v4l2_subdev)->*mut v4l2_subdev_state{(*sd).active_state}

/* CONFIG_MEDIA_CONTROLLER / CONFIG_VIDEO_V4L2_SUBDEV_API declarations. */
#[repr(C)] pub enum v4l2_subdev_routing_restriction { V4L2_SUBDEV_ROUTING_NO_1_TO_N=1, V4L2_SUBDEV_ROUTING_NO_N_TO_1=2, V4L2_SUBDEV_ROUTING_NO_SINK_STREAM_MIX=4, V4L2_SUBDEV_ROUTING_NO_SOURCE_STREAM_MIX=8, V4L2_SUBDEV_ROUTING_NO_SINK_MULTIPLEXING=16, V4L2_SUBDEV_ROUTING_NO_SOURCE_MULTIPLEXING=32, V4L2_SUBDEV_ROUTING_ONLY_1_TO_1=3, V4L2_SUBDEV_ROUTING_NO_STREAM_MIX=12, V4L2_SUBDEV_ROUTING_NO_MULTIPLEXING=48 }
extern "C" {
 pub fn v4l2_subdev_enable_streams(sd:*mut v4l2_subdev,pad:u32,streams_mask:u64)->i32;
 pub fn v4l2_subdev_disable_streams(sd:*mut v4l2_subdev,pad:u32,streams_mask:u64)->i32;
 pub fn v4l2_subdev_s_stream_helper(sd:*mut v4l2_subdev,enable:i32)->i32;
 pub fn v4l2_subdev_get_fwnode_pad_1_to_1(entity:*mut media_entity,endpoint:*mut fwnode_endpoint)->i32;
 pub fn v4l2_subdev_link_validate_default(sd:*mut v4l2_subdev,link:*mut media_link,source_fmt:*mut v4l2_subdev_format,sink_fmt:*mut v4l2_subdev_format)->i32;
 pub fn v4l2_subdev_link_validate(link:*mut media_link)->i32;
 pub fn v4l2_subdev_has_pad_interdep(entity:*mut media_entity,pad0:u32,pad1:u32)->bool;
 pub fn __v4l2_subdev_state_alloc(sd:*mut v4l2_subdev,lock_name:*const core::ffi::c_char,key:*mut lock_class_key)->*mut v4l2_subdev_state;
 pub fn __v4l2_subdev_state_free(state:*mut v4l2_subdev);
 pub fn __v4l2_subdev_init_finalize(sd:*mut v4l2_subdev,name:*const core::ffi::c_char,key:*mut lock_class_key)->i32;
 pub fn v4l2_subdev_cleanup(sd:*mut v4l2_subdev);
 pub fn __v4l2_subdev_state_get_format(state:*mut v4l2_subdev_state,pad:u32,stream:u32)->*mut v4l2_mbus_framefmt;
 pub fn __v4l2_subdev_state_get_crop(state:*mut v4l2_subdev_state,pad:u32,stream:u32)->*mut v4l2_rect;
 pub fn __v4l2_subdev_state_get_compose(state:*mut v4l2_subdev_state,pad:u32,stream:u32)->*mut v4l2_rect;
 pub fn __v4l2_subdev_state_get_interval(state:*mut v4l2_subdev_state,pad:u32,stream:u32)->*mut v4l2_fract;
 pub fn v4l2_subdev_get_fmt(sd:*mut v4l2_subdev,state:*mut v4l2_subdev_state,format:*mut v4l2_subdev_format)->i32;
 pub fn v4l2_subdev_get_frame_interval(sd:*mut v4l2_subdev,state:*mut v4l2_subdev_state,fi:*mut v4l2_subdev_frame_interval)->i32;
 pub fn v4l2_subdev_set_routing(sd:*mut v4l2_subdev,state:*mut v4l2_subdev_state,routing:*const v4l2_subdev_krouting)->i32;
 pub fn __v4l2_subdev_next_active_route(routing:*const v4l2_subdev_krouting,route:*mut v4l2_subdev_route)->*mut v4l2_subdev_route;
 pub fn v4l2_subdev_set_routing_with_fmt(sd:*mut v4l2_subdev,state:*mut v4l2_subdev_state,routing:*const v4l2_subdev_krouting,fmt:*const v4l2_mbus_framefmt)->i32;
 pub fn v4l2_subdev_routing_find_opposite_end(routing:*const v4l2_subdev_krouting,pad:u32,stream:u32,other_pad:*mut u32,other_stream:*mut u32)->i32;
 pub fn v4l2_subdev_state_get_opposite_stream_format(state:*mut v4l2_subdev_state,pad:u32,stream:u32)->*mut v4l2_mbus_framefmt;
 pub fn v4l2_subdev_state_xlate_streams(state:*const v4l2_subdev_state,pad0:u32,pad1:u32,streams:*mut u64)->u64;
 pub fn v4l2_subdev_routing_validate(sd:*mut v4l2_subdev,routing:*const v4l2_subdev_krouting,disallow:v4l2_subdev_routing_restriction)->i32;
 pub fn __v4l2_subdev_get_frame_desc_passthrough(sd:*mut v4l2_subdev,state:*mut v4l2_subdev_state,pad:u32,fd:*mut v4l2_mbus_frame_desc)->i32;
 pub fn v4l2_subdev_get_frame_desc_passthrough(sd:*mut v4l2_subdev,pad:u32,fd:*mut v4l2_mbus_frame_desc)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
