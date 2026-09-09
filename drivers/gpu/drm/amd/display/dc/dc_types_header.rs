/* Faithful Rust translation of dc_types.h. C includes and conditional build
 * dependencies are intentionally represented by referenced external types. */

use core::ffi::c_void;

pub const NUM_PIXEL_FORMATS: u32 = 10;
pub const DTBCLK_LIMIT: u32 = 2920;
pub const DEFAULT_SPEAKER_LOCATION: u32 = 5;
pub const DC_MAX_AUDIO_DESC_COUNT: usize = 16;
pub const AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS: usize = 20;
pub const DC_PLANE_UPDATE_TIMES_MAX: usize = 10;
pub const MAX_CRC_WINDOW_NUM: usize = 2;
pub const MAX_CONTROLLER_NUM: usize = 6;
pub const MAX_SINKS_PER_LINK: usize = 4;

pub struct dc_plane_state; pub struct dc_stream_state; pub struct dc_link; pub struct dc_sink;
pub struct dal; pub struct dc_dmub_srv; pub struct dc; pub struct dal_logger; pub struct dc_bios;
pub struct gpio_service; pub struct stream_encoder; pub struct hpo_dp_stream_encoder;
pub struct dpcd_caps; pub struct dc_transfer_func; pub struct dc_3dlut;

#[repr(C)] #[derive(Copy, Clone)] pub enum dce_environment { DCE_ENV_PRODUCTION_DRV=0, DCE_ENV_FPGA_MAXIMUS, DCE_ENV_DIAG, DCE_ENV_VIRTUAL_HW }
#[repr(C)] #[derive(Copy, Clone)] pub struct dc_perf_trace { pub read_count: c_ulong, pub write_count: c_ulong, pub last_entry_read: c_ulong, pub last_entry_write: c_ulong }
#[repr(C)] #[derive(Copy, Clone)] pub enum tiling_mode { TILING_MODE_INVALID, TILING_MODE_LINEAR, TILING_MODE_TILED, TILING_MODE_COUNT }
#[repr(C)] #[derive(Copy, Clone)] pub enum view_3d_format { VIEW_3D_FORMAT_NONE=0, VIEW_3D_FORMAT_FRAME_SEQUENTIAL, VIEW_3D_FORMAT_SIDE_BY_SIDE, VIEW_3D_FORMAT_TOP_AND_BOTTOM, VIEW_3D_FORMAT_COUNT, VIEW_3D_FORMAT_FIRST=1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum plane_stereo_format { PLANE_STEREO_FORMAT_NONE=0, PLANE_STEREO_FORMAT_SIDE_BY_SIDE=1, PLANE_STEREO_FORMAT_TOP_AND_BOTTOM=2, PLANE_STEREO_FORMAT_FRAME_ALTERNATE=3, PLANE_STEREO_FORMAT_ROW_INTERLEAVED=5, PLANE_STEREO_FORMAT_COLUMN_INTERLEAVED=6, PLANE_STEREO_FORMAT_CHECKER_BOARD=7 }
#[repr(C)] #[derive(Copy, Clone)] pub enum dc_edid_connector_type { DC_EDID_CONNECTOR_UNKNOWN=0, DC_EDID_CONNECTOR_ANALOG=1, DC_EDID_CONNECTOR_DIGITAL=10, DC_EDID_CONNECTOR_DVI=11, DC_EDID_CONNECTOR_HDMIA=12, DC_EDID_CONNECTOR_MDDI=14, DC_EDID_CONNECTOR_DISPLAYPORT=15 }
#[repr(C)] #[derive(Copy, Clone)] pub enum dc_edid_status { EDID_OK, EDID_BAD_INPUT, EDID_NO_RESPONSE, EDID_BAD_CHECKSUM, EDID_THE_SAME, EDID_FALL_BACK, EDID_PARTIAL_VALID }
#[repr(C)] #[derive(Copy, Clone)] pub enum act_return_status { ACT_SUCCESS, ACT_LINK_LOST, ACT_FAILED }

#[repr(C)] pub union dc_cea_audio_mode_u { pub sample_size:u8, pub max_bit_rate:u8, pub audio_codec_vendor_specific:u8 }
#[repr(C)] pub struct dc_cea_audio_mode { pub format_code:u8, pub channel_count:u8, pub sample_rate:u8, pub _u:dc_cea_audio_mode_u }
#[repr(C)] pub struct dc_edid { pub length:u32, pub raw_edid:[u8; DC_MAX_EDID_BUFFER_SIZE] }
#[repr(C)] pub struct dc_panel_patch { pub dppowerup_delay:u32,pub extra_t12_ms:u32,pub extra_delay_backlight_off:u32,pub extra_t7_ms:u32,pub skip_scdc_overwrite:u32,pub delay_ignore_msa:u32,pub disable_fec:u32,pub extra_t3_ms:u32,pub max_dsc_target_bpp_limit:u32,pub embedded_tiled_slave:u32,pub disable_fams:u32,pub hdmi_spe_handling:u32,pub block_420_Freesync:u32,pub block_10g:u32,pub hdmi_comp_manual:u32,pub hdmi_comp_auto:u32,pub force_frl:u32,pub vsdb_rcc_wa:u32,pub delay_hdmi_link_training:u32,pub skip_frl_pre_training:u32,pub skip_avmute:u32,pub skip_audio_sab_check:u32,pub mst_start_top_delay:u32,pub remove_sink_ext_caps:u32,pub disable_second_tile:bool,pub disable_colorimetry:u32,pub blankstream_before_otg_off:u8,pub oled_optimize_display_on:bool,pub force_mst_blocked_discovery:u32,pub wait_after_dpcd_poweroff_ms:u32 }
#[repr(C)] pub struct dc_edid_caps { pub manufacturer_id:u16,pub product_id:u16,pub serial_number:u32,pub manufacture_week:u8,pub manufacture_year:u8,pub display_name:[u8;20],pub speaker_flags:u8,pub audio_mode_count:u32,pub audio_modes:[dc_cea_audio_mode;16],pub audio_latency:u32,pub video_latency:u32,pub freesync_vcp_code:c_uchar,pub qs_bit:u8,pub qy_bit:u8,pub max_tmds_clk_mhz:u32,pub lte_340mcsc_scramble:bool,pub edid_hdmi:bool,pub hdr_supported:bool,pub rr_capable:bool,pub scdc_present:bool,pub analog:bool,pub max_frl_rate:u8,pub frl_dsc_support:bool,pub frl_dsc_10bpc:bool,pub frl_dsc_12bpc:bool,pub frl_dsc_all_bpp:bool,pub frl_dsc_native_420:bool,pub frl_dsc_max_slices:u8,pub frl_dsc_max_frl_rate:u8,pub frl_dsc_total_chunk_kbytes:u8,pub panel_patch:dc_panel_patch }

#[repr(C)] pub struct dc_mode_flags { pub bits:u32, pub MIRACAST_REFRESH_DIVIDER:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub enum dc_timing_source { TIMING_SOURCE_UNDEFINED,TIMING_SOURCE_USER_FORCED,TIMING_SOURCE_USER_OVERRIDE,TIMING_SOURCE_CUSTOM,TIMING_SOURCE_EXPLICIT,TIMING_SOURCE_EDID_CEA_SVD_3D,TIMING_SOURCE_EDID_CEA_SVD_PREFERRED,TIMING_SOURCE_EDID_CEA_SVD_420,TIMING_SOURCE_EDID_DETAILED,TIMING_SOURCE_EDID_ESTABLISHED,TIMING_SOURCE_EDID_STANDARD,TIMING_SOURCE_EDID_CEA_SVD,TIMING_SOURCE_EDID_CVT_3BYTE,TIMING_SOURCE_EDID_4BYTE,TIMING_SOURCE_EDID_CEA_DISPLAYID_VTDB,TIMING_SOURCE_EDID_CEA_RID,TIMING_SOURCE_EDID_DISPLAYID_TYPE5,TIMING_SOURCE_VBIOS,TIMING_SOURCE_CV,TIMING_SOURCE_TV,TIMING_SOURCE_HDMI_VIC,TIMING_SOURCE_CEA_VIC,TIMING_SOURCE_DEFAULT,TIMING_SOURCE_CUSTOM_BASE,TIMING_SOURCE_RANGELIMIT,TIMING_SOURCE_OS_FORCED,TIMING_SOURCE_IMPLICIT,TIMING_SOURCE_BASICMODE,TIMING_SOURCE_COUNT }
#[repr(C)] pub struct stereo_3d_features { pub supported:bool,pub allTimings:bool,pub cloneMode:bool,pub scaling:bool,pub singleFrameSWPacked:bool }
#[repr(C)] pub enum dc_timing_support_method { TIMING_SUPPORT_METHOD_UNDEFINED,TIMING_SUPPORT_METHOD_EXPLICIT,TIMING_SUPPORT_METHOD_IMPLICIT,TIMING_SUPPORT_METHOD_NATIVE }
#[repr(C)] pub struct dc_mode_info { pub pixel_width:u32,pub pixel_height:u32,pub field_rate:u32,pub timing_standard:dc_timing_standard,pub timing_source:dc_timing_source,pub flags:dc_mode_flags }
#[repr(C)] pub enum dc_power_state { DC_POWER_STATE_ON=1,DC_POWER_STATE_STANDBY,DC_POWER_STATE_SUSPEND,DC_POWER_STATE_OFF }
#[repr(C)] pub enum dc_video_power_state { DC_VIDEO_POWER_UNSPECIFIED=0,DC_VIDEO_POWER_ON=1,DC_VIDEO_POWER_STANDBY,DC_VIDEO_POWER_SUSPEND,DC_VIDEO_POWER_OFF,DC_VIDEO_POWER_HIBERNATE,DC_VIDEO_POWER_SHUTDOWN,DC_VIDEO_POWER_ULPS,DC_VIDEO_POWER_AFTER_RESET,DC_VIDEO_POWER_MAXIMUM }
#[repr(C)] pub enum dc_acpi_cm_power_state { DC_ACPI_CM_POWER_STATE_D0=1,DC_ACPI_CM_POWER_STATE_D1=2,DC_ACPI_CM_POWER_STATE_D2=4,DC_ACPI_CM_POWER_STATE_D3=8 }
#[repr(C)] pub enum dc_connection_type { dc_connection_none,dc_connection_single,dc_connection_mst_branch,dc_connection_sst_branch,dc_connection_analog_load }
#[repr(C)] pub struct dc_csc_adjustments { pub contrast:fixed31_32,pub saturation:fixed31_32,pub brightness:fixed31_32,pub hue:fixed31_32 }
#[repr(C)] pub enum dc_scaling_linearity { DC_SCALING_LINEARITY_LINEAR,DC_SCALING_LINEARITY_SOURCE }
#[repr(C)] pub enum dc_blending_linearity { DC_BLENDING_LINEARITY_LINEAR,DC_BLENDING_LINEARITY_SOURCE }
#[repr(C)] pub enum scaling_transformation { SCALING_TRANSFORMATION_UNINITIALIZED,SCALING_TRANSFORMATION_IDENTITY=1,SCALING_TRANSFORMATION_CENTER_TIMING=2,SCALING_TRANSFORMATION_FULL_SCREEN_SCALE=4,SCALING_TRANSFORMATION_PRESERVE_ASPECT_RATIO_SCALE=8,SCALING_TRANSFORMATION_DAL_DECIDE=16,SCALING_TRANSFORMATION_INVALID=0x80000000,SCALING_TRANSFORMATION_BEGING=1,SCALING_TRANSFORMATION_END=8 }
#[repr(C)] pub enum display_content_type { DISPLAY_CONTENT_TYPE_NO_DATA=0,DISPLAY_CONTENT_TYPE_GRAPHICS=1,DISPLAY_CONTENT_TYPE_PHOTO=2,DISPLAY_CONTENT_TYPE_CINEMA=4,DISPLAY_CONTENT_TYPE_GAME=8 }
#[repr(C)] pub enum cm_gamut_adjust_type { CM_GAMUT_ADJUST_TYPE_BYPASS=0,CM_GAMUT_ADJUST_TYPE_HW,CM_GAMUT_ADJUST_TYPE_SW }
#[repr(C)] pub struct cm_grph_csc_adjustment { pub temperature_matrix:[fixed31_32;12],pub gamut_adjust_type:cm_gamut_adjust_type,pub gamut_coef_format:cm_gamut_coef_format }

#[repr(C)] pub struct dwb_stereo_params { pub stereo_enabled:bool,pub stereo_type:dwb_stereo_type,pub stereo_polarity:bool,pub stereo_eye_select:dwb_stereo_eye_select }
#[repr(C)] pub struct dc_dwb_cnv_params { pub src_width:u32,pub src_height:u32,pub crop_width:u32,pub crop_en:bool,pub crop_height:u32,pub crop_x:u32,pub crop_y:u32,pub cnv_out_bpc:dwb_cnv_out_bpc,pub fc_out_format:dwb_out_format,pub out_denorm_mode:dwb_out_denorm,pub out_max_pix_val:u32,pub out_min_pix_val:u32 }
#[repr(C)] pub struct dc_dwb_params { pub dwbscl_black_color:u32,pub hdr_mult:u32,pub csc_params:cm_grph_csc_adjustment,pub stereo_params:dwb_stereo_params,pub cnv_params:dc_dwb_cnv_params,pub dest_width:u32,pub dest_height:u32,pub out_format:dwb_scaler_mode,pub output_depth:dwb_output_depth,pub capture_rate:dwb_capture_rate,pub scaler_taps:scaling_taps,pub subsample_position:dwb_subsample_position,pub out_transfer_func:*const dc_transfer_func }

#[repr(C)] pub union audio_sample_rates_u { pub rate:audio_sample_rates_bits,pub all:u8 }
#[repr(C)] pub struct audio_sample_rates_bits { pub bits:u8 }
#[repr(C)] pub union audio_speaker_flags_u { pub bits:u32,pub all:u32 }
#[repr(C)] pub struct audio_speaker_flags { pub bits:u32 }
#[repr(C)] pub struct audio_speaker_info { pub bits:u32 }
#[repr(C)] pub union audio_info_flags_u { pub speaker_flags:audio_speaker_flags,pub info:audio_speaker_info,pub all:u8 }
#[repr(C)] pub struct audio_info_flags { pub u:audio_info_flags_u }
#[repr(C)] pub enum audio_format_code { AUDIO_FORMAT_CODE_FIRST=1,AUDIO_FORMAT_CODE_LINEARPCM=1,AUDIO_FORMAT_CODE_AC3,AUDIO_FORMAT_CODE_MPEG1,AUDIO_FORMAT_CODE_MP3,AUDIO_FORMAT_CODE_MPEG2,AUDIO_FORMAT_CODE_AAC,AUDIO_FORMAT_CODE_DTS,AUDIO_FORMAT_CODE_ATRAC,AUDIO_FORMAT_CODE_1BITAUDIO,AUDIO_FORMAT_CODE_DOLBYDIGITALPLUS,AUDIO_FORMAT_CODE_DTS_HD,AUDIO_FORMAT_CODE_MAT_MLP,AUDIO_FORMAT_CODE_DST,AUDIO_FORMAT_CODE_WMAPRO,AUDIO_FORMAT_CODE_LAST,AUDIO_FORMAT_CODE_COUNT=14 }
#[repr(C)] pub union audio_mode_u { pub sample_size:u8,pub max_bit_rate:u8,pub vendor_specific:u8 }
#[repr(C)] pub struct audio_mode { pub format_code:audio_format_code,pub channel_count:u8,pub sample_rates:audio_sample_rates_u,pub u:audio_mode_u }
#[repr(C)] pub struct audio_info { pub flags:audio_info_flags,pub video_latency:u32,pub audio_latency:u32,pub display_index:u32,pub display_name:[u8;20],pub manufacture_id:u32,pub product_id:u32,pub port_id:[u32;2],pub mode_count:u32,pub modes:[audio_mode;16] }
#[repr(C)] pub struct audio_check { pub audio_packet_type:u32,pub max_audiosample_rate:u32,pub max_channel_count:u32,pub acat:u32 }
#[repr(C)] pub enum dc_infoframe_type { DC_HDMI_INFOFRAME_TYPE_VENDOR=0x81,DC_HDMI_INFOFRAME_TYPE_AVI=0x82,DC_HDMI_INFOFRAME_TYPE_SPD=0x83,DC_HDMI_INFOFRAME_TYPE_AUDIO=0x84,DC_DP_INFOFRAME_TYPE_PPS=0x10 }
#[repr(C)] pub struct dc_info_packet { pub valid:bool,pub hb0:u8,pub hb1:u8,pub hb2:u8,pub hb3:u8,pub sb:[u8;32] }
#[repr(C)] pub struct dc_info_packet_128 { pub valid:bool,pub hb0:u8,pub hb1:u8,pub hb2:u8,pub hb3:u8,pub sb:[u8;128] }
#[repr(C)] pub struct dc_edid_read_policy { pub max_retry_count:u32,pub delay_time_ms:u32,pub ignore_checksum:u32 }
#[repr(C)] pub struct dc_plane_flip_time { pub time_elapsed_in_us:[u32;10],pub index:u32,pub prev_update_time_in_us:u32 }

#[repr(C)] pub enum dc_alpm_mode { DC_ALPM_AUXWAKE=0,DC_ALPM_AUXLESS=1,DC_ALPM_UNSUPPORTED=0xF }
#[repr(C)] pub enum dc_psr_state { PSR_STATE0=0,PSR_STATE1,PSR_STATE1a,PSR_STATE2,PSR_STATE2a,PSR_STATE2b,PSR_STATE3,PSR_STATE3Init,PSR_STATE4,PSR_STATE4a,PSR_STATE4b,PSR_STATE4c,PSR_STATE4d,PSR_STATE4_FULL_FRAME,PSR_STATE4a_FULL_FRAME,PSR_STATE4b_FULL_FRAME,PSR_STATE4c_FULL_FRAME,PSR_STATE4_FULL_FRAME_POWERUP,PSR_STATE4_FULL_FRAME_HW_LOCK,PSR_STATE5,PSR_STATE5a,PSR_STATE5b,PSR_STATE5c,PSR_STATE_HWLOCK_MGR,PSR_STATE_POLLVUPDATE,PSR_STATE_RELEASE_HWLOCK_MGR_FULL_FRAME,PSR_STATE_INVALID=0xff }
#[repr(C)] pub struct psr_config { pub psr_version:u8,pub psr_rfb_setup_time:u32,pub psr_exit_link_training_required:bool,pub psr_frame_capture_indication_req:bool,pub psr_sdp_transmit_line_num_deadline:u32,pub allow_smu_optimizations:bool,pub allow_multi_disp_optimizations:bool,pub su_granularity_required:bool,pub su_y_granularity:u8,pub line_time_in_us:u32,pub rate_control_caps:u8,pub dsc_slice_height:u16,pub os_request_force_ffu:bool }
#[repr(C)] pub union dmcu_psr_level { pub bits:u32,pub u32all:u32 }
#[repr(C)] pub enum physical_phy_id { PHYLD_0,PHYLD_1,PHYLD_2,PHYLD_3,PHYLD_4,PHYLD_5,PHYLD_6,PHYLD_7,PHYLD_8,PHYLD_9,PHYLD_COUNT,PHYLD_UNKNOWN=-1 }
#[repr(C)] pub enum phy_type { PHY_TYPE_UNKNOWN=1,PHY_TYPE_PCIE_PHY=2,PHY_TYPE_UNIPHY=3 }
#[repr(C)] pub struct psr_context { pub channel:channel_id,pub transmitter:transmitter,pub transmitterId:transmitter,pub engineId:engine_id,pub controllerId:controller_id,pub phyType:phy_type,pub smuPhyId:physical_phy_id,pub crtcTimingVerticalTotal:u32,pub psrSupportedDisplayConfig:bool,pub psrExitLinkTrainingRequired:bool,pub psrFrameCaptureIndicationReq:bool,pub sdpTransmitLineNumDeadline:u32,pub vsync_rate_hz:u32,pub skipPsrWaitForPllLock:u32,pub numberOfControllers:u32,pub rfb_update_auto_en:bool,pub timehyst_frames:u32,pub hyst_lines:u32,pub aux_repeats:u32,pub psr_level:dmcu_psr_level,pub frame_delay:u32,pub allow_smu_optimizations:bool,pub allow_multi_disp_optimizations:bool,pub su_granularity_required:bool,pub su_y_granularity:u8,pub line_time_in_us:u32,pub rate_control_caps:u8,pub dsc_slice_height:u16,pub os_request_force_ffu:bool }
#[repr(C)] pub struct colorspace_transform { pub matrix:[fixed31_32;12],pub enable_remap:bool }
#[repr(C)] pub enum i2c_mot_mode { I2C_MOT_UNDEF,I2C_MOT_TRUE,I2C_MOT_FALSE }
#[repr(C)] pub struct AsicStateEx { pub memoryClock:u32,pub displayClock:u32,pub engineClock:u32,pub maxSupportedDppClock:u32,pub dppClock:u32,pub socClock:u32,pub dcfClockDeepSleep:u32,pub fClock:u32,pub phyClock:u32 }
#[repr(C)] pub enum dc_clock_type { DC_CLOCK_TYPE_DISPCLK=0,DC_CLOCK_TYPE_DPPCLK=1 }
#[repr(C)] pub struct dc_clock_config { pub max_clock_khz:u32,pub min_clock_khz:u32,pub bw_requirequired_clock_khz:u32,pub current_clock_khz:u32 }
#[repr(C)] pub enum hubp_dmdata_mode { DMDATA_SW_MODE,DMDATA_HW_MODE }
#[repr(C)] pub struct dc_dmdata_attributes { pub dmdata_mode:hubp_dmdata_mode,pub dmdata_repeat:bool,pub dmdata_size:u32,pub dmdata_updated:bool,pub address:PHYSICAL_ADDRESS_LOC,pub dmdata_qos_mode:bool,pub dmdata_qos_level:u32,pub dmdata_dl_delta:u32,pub dmdata_sw_data:*mut u32 }
#[repr(C)] pub struct hw_asic_id { pub chip_id:u32,pub chip_family:u32,pub pci_revision_id:u32,pub hw_internal_rev:u32,pub vram_type:u32,pub vram_width:u32,pub feature_flags:u32,pub fake_paths_num:u32,pub atombios_base_address:*mut c_void }
#[repr(C)] pub struct dc_context { pub dc:*mut dc,pub driver_context:*mut c_void,pub logger:*mut dal_logger,pub perf_trace:*mut dc_perf_trace,pub cgs_device:*mut c_void,pub dce_environment:dce_environment,pub asic_id:hw_asic_id,pub dce_version:dce_version,pub dc_bios:*mut dc_bios,pub created_bios:bool,pub gpio_service:*mut gpio_service,pub dc_sink_id_count:u32,pub dc_stream_id_count:u32,pub dc_edp_id_count:u32,pub fbc_gpu_addr:u64,pub dmub_srv:*mut dc_dmub_srv,pub cp_psp:cp_psp,pub dcn_reg_offsets:*mut u32,pub nbio_reg_offsets:*mut u32,pub clk_reg_offsets:*mut u32 }

#[repr(C)] pub union dsc_slice_caps1 { pub bits:u8,pub raw:u8 }
#[repr(C)] pub union dsc_slice_caps2 { pub bits:u8,pub raw:u8 }
#[repr(C)] pub union dsc_color_formats { pub bits:u8,pub raw:u8 }
#[repr(C)] pub union dsc_color_depth { pub bits:u8,pub raw:u8 }
#[repr(C)] pub struct dsc_dec_dpcd_caps { pub is_dsc_supported:bool,pub dsc_version:u8,pub rc_buffer_size:i32,pub slice_caps1:dsc_slice_caps1,pub slice_caps2:dsc_slice_caps2,pub lb_bit_depth:i32,pub is_block_pred_supported:bool,pub edp_max_bits_per_pixel:u32,pub color_formats:dsc_color_formats,pub color_depth:dsc_color_depth,pub throughput_mode_0_mps:i32,pub throughput_mode_1_mps:i32,pub max_slice_width:i32,pub bpp_increment_div:u32,pub branch_overall_throughput_0_mps:u32,pub branch_overall_throughput_1_mps:u32,pub branch_max_line_width:u32,pub is_frl:bool,pub is_vic_all_bpp:bool,pub total_chunk_kbytes:u32,pub is_dp:bool }
#[repr(C)] pub struct hblank_expansion_dpcd_caps { pub expansion_supported:bool,pub reduction_supported:bool,pub buffer_unit_bytes:bool,pub buffer_per_port:bool,pub buffer_size:u32 }
#[repr(C)] pub struct dc_golden_table { pub dc_golden_table_ver:u16,pub aux_dphy_rx_control0_val:u32,pub aux_dphy_tx_control_val:u32,pub aux_dphy_rx_control1_val:u32,pub dc_gpio_aux_ctrl_0_val:u32,pub dc_gpio_aux_ctrl_1_val:u32,pub dc_gpio_aux_ctrl_2_val:u32,pub dc_gpio_aux_ctrl_3_val:u32,pub dc_gpio_aux_ctrl_4_val:u32,pub dc_gpio_aux_ctrl_5_val:u32 }
#[repr(C)] pub enum dc_gpu_mem_alloc_type { DC_MEM_ALLOC_TYPE_GART,DC_MEM_ALLOC_TYPE_FRAME_BUFFER,DC_MEM_ALLOC_TYPE_INVISIBLE_FRAME_BUFFER,DC_MEM_ALLOC_TYPE_AGP }
#[repr(C)] pub enum dc_link_encoding_format { DC_LINK_ENCODING_UNSPECIFIED=0,DC_LINK_ENCODING_DP_8b_10b,DC_LINK_ENCODING_DP_128b_132b,DC_LINK_ENCODING_HDMI_TMDS,DC_LINK_ENCODING_HDMI_FRL }
#[repr(C)] pub enum dc_psr_version { DC_PSR_VERSION_1=0,DC_PSR_VERSION_SU_1=1,DC_PSR_VERSION_UNSUPPORTED=0xffffffff }
#[repr(C)] pub enum dc_replay_version { DC_FREESYNC_REPLAY=0,DC_VESA_PANEL_REPLAY=1,DC_REPLAY_VERSION_UNSUPPORTED=0xff }
#[repr(C)] pub enum display_endpoint_type { DISPLAY_ENDPOINT_PHY=0,DISPLAY_ENDPOINT_USB4_DPIA,DISPLAY_ENDPOINT_UNKNOWN=-1 }
#[repr(C)] pub struct display_endpoint_id { pub link_id:graphics_object_id,pub ep_type:display_endpoint_type }
#[repr(C)] pub enum dc_panel_type { PANEL_TYPE_NONE=0,PANEL_TYPE_LCD=1,PANEL_TYPE_OLED=2,PANEL_TYPE_MINILED=3 }
#[repr(C)] pub enum backlight_control_type { BACKLIGHT_CONTROL_PWM=0,BACKLIGHT_CONTROL_VESA_AUX=1,BACKLIGHT_CONTROL_AMD_AUX=2 }

#[repr(C)] pub enum dc_detect_reason { DETECT_REASON_BOOT,DETECT_REASON_RESUMEFROMS3S4,DETECT_REASON_HPD,DETECT_REASON_HPDRX,DETECT_REASON_FALLBACK,DETECT_REASON_RETRAIN,DETECT_REASON_TDR }
#[repr(C)] pub struct dc_link_status { pub link_active:bool,pub dpcd_caps:*mut dpcd_caps }
#[repr(C)] pub union hdcp_rx_caps { pub fields:[u8;3],pub raw:[u8;3] }
#[repr(C)] pub union hdcp_bcaps { pub bits:u8,pub raw:u8 }
#[repr(C)] pub struct hdcp_caps { pub rx_caps:hdcp_rx_caps,pub bcaps:hdcp_bcaps }
#[repr(C)] pub struct link_mst_stream_allocation { pub stream_enc:*const stream_encoder,pub hpo_dp_stream_enc:*const hpo_dp_stream_encoder,pub vcp_id:u8,pub slot_count:u8 }
#[repr(C)] pub struct link_mst_stream_allocation_table { pub stream_count:i32,pub stream_allocations:[link_mst_stream_allocation;6] }

#[repr(C)] pub enum replay_coasting_vtotal_type { PR_COASTING_TYPE_NOM=0,PR_COASTING_TYPE_STATIC,PR_COASTING_TYPE_FULL_SCREEN_VIDEO,PR_COASTING_TYPE_TEST_HARNESS,PR_COASTING_TYPE_VIDEO_CONFERENCING_V2,PR_COASTING_TYPE_NUM }
#[repr(C)] pub enum replay_link_off_frame_count_level { PR_LINK_OFF_FRAME_COUNT_FAIL=0,PR_LINK_OFF_FRAME_COUNT_GOOD=2,PR_LINK_OFF_FRAME_COUNT_BEST=6 }
#[repr(C)] pub enum replay_FW_Message_type { Replay_Msg_Not_Support=-1,Replay_Set_Timing_Sync_Supported,Replay_Set_Residency_Frameupdate_Timer,Replay_Set_Pseudo_VTotal,Replay_Disabled_Adaptive_Sync_SDP,Replay_Set_General_Cmd }
#[repr(C)] pub union replay_error_status { pub bits:u8,pub raw:u8 }
#[repr(C)] pub union replay_low_refresh_rate_enable_options { pub bits:u32,pub raw:u32 }
#[repr(C)] pub union replay_optimization { pub bits:u32,pub raw:u32 }
#[repr(C)] pub struct replay_config { pub replay_version:dc_replay_version,pub replay_supported:bool,pub replay_cap_support:bool,pub replay_power_opt_supported:u32,pub replay_smu_opt_supported:bool,pub replay_enable_option:u32,pub debug_flags:u32,pub replay_timing_sync_supported:bool,pub force_disable_desync_error_check:bool,pub received_desync_error_hpd:bool,pub replay_support_fast_resync_in_ultra_sleep_mode:bool,pub replay_error_status:replay_error_status,pub low_rr_enable_options:replay_low_refresh_rate_enable_options,pub low_rr_activated:bool,pub low_rr_supported:bool,pub replay_video_conferencing_optimization_enabled:bool,pub alpm_mode:dc_alpm_mode,pub os_request_force_ffu:bool,pub replay_optimization:replay_optimization,pub frame_skip_supported:bool,pub received_frame_skipping_error_hpd:bool,pub live_capture_with_cvt_activated:bool }
#[repr(C)] pub struct replay_settings { pub config:replay_config,pub replay_feature_enabled:bool,pub replay_allow_active:bool,pub replay_allow_long_vblank:bool,pub replay_power_opt_active:u32,pub replay_smu_opt_enable:bool,pub coasting_vtotal:u32,pub coasting_vtotal_table:[u32;5],pub defer_update_coasting_vtotal_table:[u32;5],pub frame_skip_number_table:[u32;5],pub defer_frame_skip_number_table:[u32;5],pub link_off_frame_count:u32,pub low_rr_full_screen_video_pseudo_vtotal:u16,pub last_pseudo_vtotal:u16,pub replay_desync_error_fail_count:u32,pub frame_skip_number:u16,pub replay_events:u32 }

#[repr(C)] pub struct dc_panel_config { pub pps:dc_panel_config_pps,pub nits_brightness:dc_panel_config_nits,pub psr:dc_panel_config_psr,pub varib:dc_panel_config_varib,pub dsc:dc_panel_config_dsc,pub ilr:dc_panel_config_ilr,pub cacp:dc_panel_config_cacp,pub adaptive_vb:dc_panel_config_adaptive,pub rio:dc_panel_config_rio }
#[repr(C)] pub struct dc_panel_config_pps { pub extra_t3_ms:u32,pub extra_t7_ms:u32,pub extra_delay_backlight_off:u32,pub extra_post_t7_ms:u32,pub extra_pre_t11_ms:u32,pub extra_t12_ms:u32,pub extra_post_OUI_ms:u32 }
#[repr(C)] pub struct dc_panel_config_nits { pub peak:u32,pub max_avg:u32,pub min:u32,pub max_nonboost_brightness_millinits:u32,pub min_brightness_millinits:u32 }
#[repr(C)] pub struct dc_panel_config_psr { pub disable_psr:bool,pub disallow_psrsu:bool,pub disallow_replay:bool,pub rc_disable:bool,pub rc_allow_static_screen:bool,pub rc_allow_fullscreen_VPB:bool,pub read_psrcap_again:bool,pub replay_enable_option:u32,pub enable_frame_skipping:bool,pub enable_teams_optimization:bool }
#[repr(C)] pub struct dc_panel_config_varib { pub varibright_feature_enable:u32,pub def_varibright_level:u32,pub abm_config_setting:u32 }
#[repr(C)] pub struct dc_panel_config_dsc { pub disable_dsc_edp:bool,pub force_dsc_edp_policy:u32 }
#[repr(C)] pub struct dc_panel_config_ilr { pub optimize_edp_link_rate:bool }
#[repr(C)] pub struct dc_panel_config_cacp { pub cacp_supported:u32,pub cacp_control_mode:u32,pub strscl_valid:u32,pub strscl_sdr:[u32;4],pub strscl_hdr:[u32;4] }
#[repr(C)] pub struct dc_panel_config_adaptive { pub disable_adaptive_vb:bool,pub default_abm_vb_levels:u32,pub default_cacp_vb_levels:u32,pub default_abm_vb_hdr_levels:u32,pub default_cacp_vb_hdr_levels:u32,pub abm_scaling_factors:u32,pub cacp_scaling_factors:u32,pub battery_life_configures:u32,pub abm_backlight_adaptive_pwl_1:u32,pub abm_backlight_adaptive_pwl_2:u32,pub abm_backlight_adaptive_pwl_3:u32,pub cacp_backlight_adaptive_pwl_1:u32,pub cacp_backlight_adaptive_pwl_2:u32,pub cacp_backlight_adaptive_pwl_3:u32 }
#[repr(C)] pub struct dc_panel_config_rio { pub disable_rio:bool }
#[repr(C)] pub struct mccs_caps { pub freesync_supported:bool }
#[repr(C)] pub struct dc_dpia_bw_alloc { pub remote_sink_req_bw:[i32;4],pub link_verified_bw:i32,pub link_max_bw:i32,pub allocated_bw:i32,pub estimated_bw:i32,pub bw_granularity:i32,pub dp_overhead:i32,pub bw_alloc_enabled:bool,pub nrd_max_lane_count:u8,pub nrd_max_link_rate:u8 }
#[repr(C)] pub enum dc_hpd_enable_select { HPD_EN_FOR_ALL_EDP=0,HPD_EN_FOR_PRIMARY_EDP_ONLY,HPD_EN_FOR_SECONDARY_EDP_ONLY }
#[repr(C)] pub enum dc_cm_lut_swizzle { CM_LUT_3D_SWIZZLE_LINEAR_RGB,CM_LUT_3D_SWIZZLE_LINEAR_BGR,CM_LUT_1D_PACKED_LINEAR }
#[repr(C)] pub enum dc_cm_lut_pixel_format { CM_LUT_PIXEL_FORMAT_RGBA16161616_UNORM_12MSB,CM_LUT_PIXEL_FORMAT_BGRA16161616_UNORM_12MSB,CM_LUT_PIXEL_FORMAT_RGBA16161616_UNORM_12LSB,CM_LUT_PIXEL_FORMAT_BGRA16161616_UNORM_12LSB,CM_LUT_PIXEL_FORMAT_RGBA16161616_FLOAT_FP1_5_10,CM_LUT_PIXEL_FORMAT_BGRA16161616_FLOAT_FP1_5_10 }
#[repr(C)] pub enum dc_cm_lut_size { CM_LUT_SIZE_NONE,CM_LUT_SIZE_999,CM_LUT_SIZE_171717,CM_LUT_SIZE_333333,CM_LUT_SIZE_454545,CM_LUT_SIZE_656565 }
#[repr(C)] pub enum mall_stream_type { SUBVP_NONE,SUBVP_MAIN,SUBVP_PHANTOM }
#[repr(C)] pub enum dc_power_source_type { DC_POWER_SOURCE_AC,DC_POWER_SOURCE_DC }
#[repr(C)] pub struct dc_state_create_params { pub power_source:dc_power_source_type }
#[repr(C)] pub struct dc_commit_streams_params { pub streams:*mut *mut dc_stream_state,pub stream_count:u8,pub power_source:dc_power_source_type }
#[repr(C)] pub struct set_backlight_level_params { pub backlight_pwm_u16_16:u32,pub frame_ramp:u32,pub control_type:backlight_control_type,pub backlight_millinits:u32,pub transition_time_in_ms:u32,pub min_luminance:u32,pub max_luminance:u32,pub min_backlight_pwm:u32,pub max_backlight_pwm:u32,pub aux_inst:u8 }
#[repr(C)] pub enum dc_validate_mode { DC_VALIDATE_MODE_AND_PROGRAMMING=0,DC_VALIDATE_MODE_ONLY=1,DC_VALIDATE_MODE_AND_STATE_INDEX=2 }
#[repr(C)] pub struct dc_validation_dpia_set { pub link:*const dc_link,pub tunnel_settings:*const dc_tunnel_settings,pub required_bw:u32 }

#[repr(C)] pub enum dc_cm2_shaper_3dlut_setting { DC_CM2_SHAPER_3DLUT_SETTING_BYPASS_ALL,DC_CM2_SHAPER_3DLUT_SETTING_ENABLE_SHAPER,DC_CM2_SHAPER_3DLUT_SETTING_ENABLE_SHAPER_3DLUT }
#[repr(C)] pub enum dc_cm2_gpu_mem_layout { DC_CM2_GPU_MEM_LAYOUT_3D_SWIZZLE_LINEAR_RGB,DC_CM2_GPU_MEM_LAYOUT_3D_SWIZZLE_LINEAR_BGR,DC_CM2_GPU_MEM_LAYOUT_1D_PACKED_LINEAR }
#[repr(C)] pub enum dc_cm2_gpu_mem_pixel_component_order { DC_CM2_GPU_MEM_PIXEL_COMPONENT_ORDER_RGBA,DC_CM2_GPU_MEM_PIXEL_COMPONENT_ORDER_BGRA }
#[repr(C)] pub enum dc_cm2_gpu_mem_format { DC_CM2_GPU_MEM_FORMAT_16161616_UNORM_12MSB,DC_CM2_GPU_MEM_FORMAT_16161616_UNORM_12LSB,DC_CM2_GPU_MEM_FORMAT_16161616_FLOAT_FP1_5_10 }
#[repr(C)] pub enum dc_cm2_gpu_mem_size { DC_CM2_GPU_MEM_SIZE_171717,DC_CM2_GPU_MEM_SIZE_333333,DC_CM2_GPU_MEM_SIZE_454545,DC_CM2_GPU_MEM_SIZE_656565,DC_CM2_GPU_MEM_SIZE_TRANSFORMED }
#[repr(C)] pub struct dc_cm2_gpu_mem_float_params { pub bias:u16,pub scale:u16 }
#[repr(C)] pub union dc_cm2_gpu_mem_format_parameters_u { pub float_params:dc_cm2_gpu_mem_float_params }
#[repr(C)] pub struct dc_cm2_gpu_mem_format_parameters { pub format:dc_cm2_gpu_mem_format,pub u:dc_cm2_gpu_mem_format_parameters_u }
#[repr(C)] pub struct dc_cm2_gpu_mem_parameters { pub addr:dc_plane_address,pub layout:dc_cm2_gpu_mem_layout,pub format_params:dc_cm2_gpu_mem_format_parameters,pub component_order:dc_cm2_gpu_mem_pixel_component_order,pub size:dc_cm2_gpu_mem_size,pub bit_depth:u16 }
#[repr(C)] pub enum dc_cm2_transfer_func_source { DC_CM2_TRANSFER_FUNC_SOURCE_SYSMEM,DC_CM2_TRANSFER_FUNC_SOURCE_VIDMEM }
#[repr(C)] pub union dc_cm2_func_luts_3d_u { pub lut3d_func:*const dc_3dlut,pub gpu_mem_params:dc_cm2_gpu_mem_parameters }
#[repr(C)] pub struct dc_cm2_func_luts_3dlut_data { pub lut3d_src:dc_cm2_transfer_func_source,pub u:dc_cm2_func_luts_3d_u,pub rmcm_3dlut_shaper_select:bool,pub mpc_3dlut_enable:bool,pub rmcm_3dlut_enable:bool,pub mpc_mcm_post_blend:bool,pub rmcm_tmz:u8 }
#[repr(C)] pub struct dc_cm2_func_luts { pub shaper:*const dc_transfer_func,pub lut3d_data:dc_cm2_func_luts_3dlut_data,pub lut1d_func:*const dc_transfer_func }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
