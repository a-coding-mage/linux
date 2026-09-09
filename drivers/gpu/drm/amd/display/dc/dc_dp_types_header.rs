/* Translated from dc_dp_types.h. Includes are external dependencies. */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

pub type uint8_t = u8; pub type uint16_t = u16; pub type uint32_t = u32; pub type uint64_t = u64;
pub type int8_t = i8; pub type c_ulonglong = u64;

#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dc_lane_count { LANE_COUNT_UNKNOWN=0, LANE_COUNT_ONE=1, LANE_COUNT_TWO=2, LANE_COUNT_FOUR=4, LANE_COUNT_EIGHT=8, LANE_COUNT_DP_MAX=4 }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dc_link_rate { LINK_RATE_UNKNOWN=0, LINK_RATE_LOW=0x06, LINK_RATE_RATE_2=0x08, LINK_RATE_RATE_3=0x09, LINK_RATE_HIGH=0x0a, LINK_RATE_RBR2=0x0c, LINK_RATE_RATE_6=0x10, LINK_RATE_HIGH2=0x14, LINK_RATE_RATE_8=0x19, LINK_RATE_HIGH3=0x1e, LINK_RATE_UHBR10=1000, LINK_RATE_UHBR13_5=1350, LINK_RATE_UHBR20=2000 }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dc_link_spread { LINK_SPREAD_DISABLED=0, LINK_SPREAD_05_DOWNSPREAD_30KHZ=0x10, LINK_SPREAD_05_DOWNSPREAD_33KHZ=0x11 }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dc_voltage_swing { VOLTAGE_SWING_LEVEL0=0, VOLTAGE_SWING_LEVEL1, VOLTAGE_SWING_LEVEL2, VOLTAGE_SWING_LEVEL3, VOLTAGE_SWING_MAX_LEVEL=3 }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dc_pre_emphasis { PRE_EMPHASIS_DISABLED=0, PRE_EMPHASIS_LEVEL1, PRE_EMPHASIS_LEVEL2, PRE_EMPHASIS_LEVEL3, PRE_EMPHASIS_MAX_LEVEL=3 }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dc_post_cursor2 { POST_CURSOR2_DISABLED=0, POST_CURSOR2_LEVEL1, POST_CURSOR2_LEVEL2, POST_CURSOR2_LEVEL3, POST_CURSOR2_MAX_LEVEL=3 }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dc_dp_ffe_preset_level { DP_FFE_PRESET_LEVEL0=0, DP_FFE_PRESET_LEVEL1, DP_FFE_PRESET_LEVEL2, DP_FFE_PRESET_LEVEL3, DP_FFE_PRESET_LEVEL4, DP_FFE_PRESET_LEVEL5, DP_FFE_PRESET_LEVEL6, DP_FFE_PRESET_LEVEL7, DP_FFE_PRESET_LEVEL8, DP_FFE_PRESET_LEVEL9, DP_FFE_PRESET_LEVEL10, DP_FFE_PRESET_LEVEL11, DP_FFE_PRESET_LEVEL12, DP_FFE_PRESET_LEVEL13, DP_FFE_PRESET_LEVEL14, DP_FFE_PRESET_LEVEL15, DP_FFE_PRESET_MAX_LEVEL=15 }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dc_dp_training_pattern { DP_TRAINING_PATTERN_SEQUENCE_1=0, DP_TRAINING_PATTERN_SEQUENCE_2, DP_TRAINING_PATTERN_SEQUENCE_3, DP_TRAINING_PATTERN_SEQUENCE_4, DP_TRAINING_PATTERN_VIDEOIDLE, DP_128b_132b_TPS1, DP_128b_132b_TPS2, DP_128b_132b_TPS2_CDS }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dp_link_encoding { DP_UNKNOWN_ENCODING=0, DP_8b_10b_ENCODING=1, DP_128b_132b_ENCODING=2 }
#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum dp_test_link_rate { DP_TEST_LINK_RATE_RBR=6, DP_TEST_LINK_RATE_RATE_2=8, DP_TEST_LINK_RATE_RATE_3=9, DP_TEST_LINK_RATE_HBR=0x0a, DP_TEST_LINK_RATE_RBR2=0x0c, DP_TEST_LINK_RATE_RATE_6=0x10, DP_TEST_LINK_RATE_HBR2=0x14, DP_TEST_LINK_RATE_RATE_8=0x19, DP_TEST_LINK_RATE_HBR3=0x1e, DP_TEST_LINK_RATE_UHBR10=1, DP_TEST_LINK_RATE_UHBR20=2, DP_TEST_LINK_RATE_UHBR13_5_LEGACY=3, DP_TEST_LINK_RATE_UHBR13_5=4 }

#[repr(C)] #[derive(Copy, Clone)] pub struct dc_link_settings { pub lane_count: dc_lane_count, pub link_rate: dc_link_rate, pub link_spread: dc_link_spread, pub use_link_rate_set: bool, pub link_rate_set: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct dc_tunnel_settings { pub should_enable_dp_tunneling: bool, pub should_use_dp_bw_allocation: bool, pub cm_id:u8, pub group_id:u8, pub bw_granularity:u32, pub estimated_bw:u32, pub allocated_bw:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct dc_dp_ffe_preset_settings { pub level:u8, pub reserved:u8, pub no_preshoot:u8, pub no_deemphasis:u8, pub method2:u8 }
#[repr(C)] pub union dc_dp_ffe_preset { pub settings: dc_dp_ffe_preset_settings, pub raw:u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct dc_lane_settings { pub VOLTAGE_SWING:dc_voltage_swing, pub PRE_EMPHASIS:dc_pre_emphasis, pub POST_CURSOR2:dc_post_cursor2, pub FFE_PRESET:dc_dp_ffe_preset }
#[repr(C)] pub struct dc_link_training_overrides { pub voltage_swing:*mut dc_voltage_swing, pub pre_emphasis:*mut dc_pre_emphasis, pub post_cursor2:*mut dc_post_cursor2, pub ffe_preset:*mut dc_dp_ffe_preset, pub cr_pattern_time:*mut u16, pub eq_pattern_time:*mut u16, pub pattern_for_cr:*mut dc_dp_training_pattern, pub pattern_for_eq:*mut dc_dp_training_pattern, pub downspread:*mut dc_link_spread, pub alternate_scrambler_reset:*mut bool, pub enhanced_framing:*mut bool, pub mst_enable:*mut bool, pub fec_enable:*mut bool }

/* C bitfields are retained as raw byte storage; masks and shifts are defined by the source register specifications. */
macro_rules! byte_union { ($($n:ident),* $(,)?) => { $(#[repr(C)] pub union $n { pub bits:[u8;1], pub raw:u8 })* }; }
byte_union!(payload_table_update_status,dpcd_rev,max_lane_count,max_down_spread,mstm_cap,lane_count_set,lane_status,device_service_irq,sink_count,lane_align_status_updated,link_service_irq_vector_esi0,lane_adjust,dpcd_training_pattern,dpcd_training_lane,dwnstream_port_caps_byte0,dwnstream_port_caps_byte2,dwnstream_port_caps_byte3_dvi,dwnstream_port_caps_byte3_hdmi,hdmi_encoded_link_bw,hdmi_tx_link_status,autonomous_mode_and_frl_link_status,sink_status,down_stream_port_count,down_spread_ctrl,dpcd_edp_config,edp_configuration_cap,dprx_feature,training_aux_rd_interval,test_request,test_response,phy_test_pattern,compliance_test_state,link_test_pattern,test_misc,audio_test_mode,audio_test_pattern_period,dpcd_fec_capability,dp_tun_cap_support,dpia_info,usb4_driver_bw_cap,dpia_tunnel_info,dp_main_line_channel_coding_cap,dp_main_link_channel_coding_lttpr_cap,dp_128b_132b_supported_link_rates,dp_128b_132b_supported_lttpr_link_rates,dp_alpm_lttpr_cap,dp_sink_video_fallback_formats,dp_fec_capability1,dp_cable_id,dp_128b_132b_training_aux_rd_interval,edp_alpm_caps,edp_psr_dpcd_caps,dpcd_panel_replay_capability_supported,dpcd_panel_replay_capability,dpcd_sink_ext_caps,dpcd_psr_configuration,replay_enable_and_configuration,dpcd_replay_configuration,panel_replay_enable_and_configuration_1,panel_replay_enable_and_configuration_2,dpcd_alpm_configuration,dpcd_sink_active_vtotal_control_mode,pr_error_status,psr_error_status,psr_sink_psr_status);

#[repr(C)] pub union dwnstream_portxcaps { pub raw:[u8;4], pub bytes:[u8;4] }
#[repr(C)] pub union downstream_port { pub raw:u8, pub bits:u8 }
#[repr(C)] pub union dp_downstream_port_present { pub byte:u8, pub fields:u8 }
#[repr(C)] pub union hpd_irq_data { pub raw:[u8;7], pub bytes:[u8;7] }
#[repr(C)] pub union dp_receive_port0_cap { pub raw:[u8;2], pub bits:[u8;2] }
#[repr(C)] pub union dpcd_max_uncompressed_pixel_rate_cap { pub raw:[u8;2], pub bits:u16 }
#[repr(C)] pub union dp_dfp_cap_ext { pub raw:[u8;12], pub fields:[u8;12] }

#[repr(C)] pub struct dp_device_vendor_id { pub ieee_oui:[u8;3], pub ieee_device_id:[u8;6] }
#[repr(C)] pub struct dp_sink_hw_fw_revision { pub ieee_hw_rev:u8, pub ieee_fw_rev:[u8;2] }
#[repr(C)] pub union dpcd_ieee_vendor_signature { pub raw:[u8;12], pub fields:[u8;12] }
#[repr(C)] pub struct dpcd_vendor_signature { pub is_valid:bool, pub data:dpcd_ieee_vendor_signature }
#[repr(C)] pub struct dpcd_amd_signature { pub AMD_IEEE_TxSignature_byte1:u8,pub AMD_IEEE_TxSignature_byte2:u8,pub AMD_IEEE_TxSignature_byte3:u8 }
#[repr(C)] pub struct dpcd_amd_device_id { pub device_id_byte1:u8,pub device_id_byte2:u8,pub zero:[u8;4],pub dce_version:u8,pub dal_version_byte1:u8,pub dal_version_byte2:u8 }
#[repr(C)] pub struct target_luminance_value { pub byte0:u8,pub byte1:u8,pub byte2:u8 }
#[repr(C)] pub struct dpcd_source_backlight_set { pub backlight_level_millinits:[u8;4], pub backlight_transition_time_ms:[u8;2] }
#[repr(C)] pub union dpcd_source_backlight_get { pub raw:[u8;8], pub bytes:[u32;2] }

#[repr(C)] pub struct dpcd_dsc_support { pub raw:u8 } #[repr(C)] pub struct dpcd_dsc_algorithm_revision{pub raw:u8} #[repr(C)] pub struct dpcd_dsc_rc_buffer_block_size{pub raw:u8} #[repr(C)] pub struct dpcd_dsc_slice_capability1{pub raw:u8} #[repr(C)] pub struct dpcd_dsc_line_buffer_bit_depth{pub raw:u8} #[repr(C)] pub struct dpcd_dsc_block_prediction_support{pub raw:u8} #[repr(C)] pub struct dpcd_maximum_bits_per_pixel_supported_by_the_decompressor{pub raw:[u8;2]} #[repr(C)] pub struct dpcd_dsc_decoder_color_format_capabilities{pub raw:u8} #[repr(C)] pub struct dpcd_dsc_decoder_color_depth_capabilities{pub raw:u8} #[repr(C)] pub struct dpcd_peak_dsc_throughput_dsc_sink{pub raw:u8} #[repr(C)] pub struct dpcd_dsc_slice_capabilities_2{pub raw:u8} #[repr(C)] pub struct dpcd_bits_per_pixel_increment{pub raw:u8}
#[repr(C)] pub union dpcd_dsc_basic_capabilities { pub raw:[u8;16], pub fields:[u8;16] } #[repr(C)] pub union dpcd_dsc_branch_decoder_capabilities { pub raw:[u8;3], pub fields:[u8;3] }
#[repr(C)] pub struct dpcd_dsc_capabilities { pub dsc_basic_caps:dpcd_dsc_basic_capabilities, pub dsc_branch_decoder_caps:dpcd_dsc_branch_decoder_capabilities }
#[repr(C)] pub struct psr_caps { pub psr_version:u8,pub psr_rfb_setup_time:u32,pub psr_exit_link_training_required:bool,pub edp_revision:u8,pub support_ver:u8,pub su_granularity_required:bool,pub y_coordinate_required:bool,pub su_y_granularity:u8,pub alpm_cap:bool,pub standby_support:bool,pub rate_control_caps:u8,pub psr_power_opt_flag:u32 }
#[repr(C)] pub struct adaptive_sync_caps { pub dp_adap_sync_caps:dpcd_dprx_feature_enumeration_list_cont_1 }
#[repr(C)] pub union dpcd_dprx_feature_enumeration_list_cont_1 { pub bits:u8,pub raw:u8 }
pub const DPCD_USB4_TOPOLOGY_ID_LEN:usize=5; pub const MAX_REPEATER_CNT:usize=8;
#[repr(C)] pub struct dpcd_usb4_dp_tunneling_info { pub dp_tun_cap:dp_tun_cap_support,pub dpia_info:dpia_info,pub driver_bw_cap:usb4_driver_bw_cap,pub dpia_tunnel_info:dpia_tunnel_info,pub usb4_driver_id:u8,pub usb4_topology_id:[u8;DPCD_USB4_TOPOLOGY_ID_LEN] }
#[repr(C)] pub struct replay_info{pub pixel_deviation_per_line:u8,pub max_deviation_line:u8} #[repr(C)] pub struct dprx_states{pub cable_id_written:bool} #[repr(C)] pub struct dpcd_panel_replay_selective_update_info{pub pr_su_x_granularity:u16,pub pr_su_y_granularity:u8,pub pr_su_y_granularity_extended_caps:u16}
#[repr(C)] pub struct edp_psr_info{pub psr_version:u8,pub psr_dpcd_caps:edp_psr_dpcd_caps,pub psr2_su_y_granularity_cap:u8,pub force_psrsu_cap:u8,pub psr_active_vtotal_control_cap:u8}
#[repr(C)] pub struct edp_trace_power_timestamps{pub poweroff:u64,pub poweron:u64} #[repr(C)] pub struct dp_trace_lt_counts{pub total:u32,pub fail:u32}
#[repr(C)] pub enum link_training_result{LINK_TRAINING_SUCCESS,LINK_TRAINING_CR_FAIL_LANE0,LINK_TRAINING_CR_FAIL_LANE1,LINK_TRAINING_CR_FAIL_LANE23,LINK_TRAINING_EQ_FAIL_CR,LINK_TRAINING_EQ_FAIL_CR_PARTIAL,LINK_TRAINING_EQ_FAIL_EQ,LINK_TRAINING_LQA_FAIL,LINK_TRAINING_LINK_LOSS,LINK_TRAINING_ABORT,DP_128b_132b_LT_FAILED,DP_128b_132b_MAX_LOOP_COUNT_REACHED,DP_128b_132b_CHANNEL_EQ_DONE_TIMEOUT,DP_128b_132b_CDS_DONE_TIMEOUT}
#[repr(C)] pub struct dp_trace_lt{pub counts:dp_trace_lt_counts,pub timestamps:[u64;2],pub result:link_training_result,pub is_logged:bool} #[repr(C)] pub struct dp_trace{pub detect_lt_trace:dp_trace_lt,pub commit_lt_trace:dp_trace_lt,pub link_loss_count:u32,pub is_initialized:bool,pub edp_trace_power_timestamps:edp_trace_power_timestamps}
pub const DP_LINK_SQUARE_PATTERN:u32=0x10f; pub const DP_CABLE_ATTRIBUTES_UPDATED_BY_DPRX:u32=0x2217; pub const DP_CABLE_ATTRIBUTES_UPDATED_BY_DPTX:u32=0x110; pub const DPCD_MAX_UNCOMPRESSED_PIXEL_RATE_CAP:u32=0x221c; pub const DP_LTTPR_ALPM_CAPABILITIES:u32=0xf0009; pub const DP_TUNNELING_IRQ:u32=1<<5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
