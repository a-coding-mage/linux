/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_uint, c_void};

/* C header dependencies: linux/types.h, linux/soc/qcom/apr.h,
 * uapi/sound/snd_ar_tokens.h, sound/soc.h.
 */
pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type phys_addr_t = usize;
pub type bool_ = bool;
pub type gpr_device_t = c_void;
pub type gpr_port_t = c_void;
pub type wait_queue_head_t = c_void;

#[repr(C)]
pub struct q6apm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct q6apm_graph {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpr_ibasic_rsp_result_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpr_pkt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct audioreach_module_priv_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_codec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

/* Module IDs */
pub const MODULE_ID_WR_SHARED_MEM_EP: u32 = 0x07001000;
pub const MODULE_ID_RD_SHARED_MEM_EP: u32 = 0x07001001;
pub const MODULE_ID_GAIN: u32 = 0x07001002;
pub const MODULE_ID_PCM_CNV: u32 = 0x07001003;
pub const MODULE_ID_PCM_ENC: u32 = 0x07001004;
pub const MODULE_ID_PCM_DEC: u32 = 0x07001005;
pub const MODULE_ID_SH_MEM_PULL_MODE: u32 = 0x07001006;
pub const MODULE_ID_SH_MEM_PUSH_MODE: u32 = 0x07001007;
pub const MODULE_ID_PLACEHOLDER_ENCODER: u32 = 0x07001008;
pub const MODULE_ID_PLACEHOLDER_DECODER: u32 = 0x07001009;
pub const MODULE_ID_I2S_SINK: u32 = 0x0700100A;
pub const MODULE_ID_I2S_SOURCE: u32 = 0x0700100B;
pub const MODULE_ID_SAL: u32 = 0x07001010;
pub const MODULE_ID_MFC: u32 = 0x07001015;
pub const MODULE_ID_DATA_LOGGING: u32 = 0x0700101A;
pub const MODULE_ID_AAC_DEC: u32 = 0x0700101F;
pub const MODULE_ID_CODEC_DMA_SINK: u32 = 0x07001023;
pub const MODULE_ID_CODEC_DMA_SOURCE: u32 = 0x07001024;
pub const MODULE_ID_FLAC_DEC: u32 = 0x0700102F;
pub const MODULE_ID_SMECNS_V2: u32 = 0x07001031;
pub const MODULE_ID_MP3_DECODE: u32 = 0x0700103B;
pub const MODULE_ID_GAPLESS: u32 = 0x0700104D;
pub const MODULE_ID_DISPLAY_PORT_SINK: u32 = 0x07001069;
pub const MODULE_ID_SPEAKER_PROTECTION: u32 = 0x070010E2;
pub const MODULE_ID_SPEAKER_PROTECTION_VI: u32 = 0x070010E3;
pub const MODULE_ID_OPUS_DEC: u32 = 0x07001174;
pub const MODULE_ID_AUDIO_IF_SINK: u32 = 0x0700117C;
pub const MODULE_ID_AUDIO_IF_SOURCE: u32 = 0x0700117D;

pub const APM_CMD_GET_SPF_STATE: u32 = 0x01001021;
pub const APM_CMD_RSP_GET_SPF_STATE: u32 = 0x02001007;

pub const APM_MODULE_INSTANCE_ID: u32 = 0x00000001;
pub const PRM_MODULE_INSTANCE_ID: u32 = 0x00000002;
pub const AMDB_MODULE_INSTANCE_ID: u32 = 0x00000003;
pub const VCPM_MODULE_INSTANCE_ID: u32 = 0x00000004;
pub const AR_MODULE_INSTANCE_ID_START: u32 = 0x00006000;
pub const AR_MODULE_INSTANCE_ID_END: u32 = 0x00007000;
pub const AR_MODULE_DYNAMIC_INSTANCE_ID_START: u32 = 0x00007000;
pub const AR_MODULE_DYNAMIC_INSTANCE_ID_END: u32 = 0x00008000;
pub const AR_CONT_INSTANCE_ID_START: u32 = 0x00005000;
pub const AR_CONT_INSTANCE_ID_END: u32 = 0x00006000;
pub const AR_SG_INSTANCE_ID_START: u32 = 0x00004000;

pub const APM_CMD_GRAPH_OPEN: u32 = 0x01001000;
pub const APM_CMD_GRAPH_PREPARE: u32 = 0x01001001;
pub const APM_CMD_GRAPH_START: u32 = 0x01001002;
pub const APM_CMD_GRAPH_STOP: u32 = 0x01001003;
pub const APM_CMD_GRAPH_CLOSE: u32 = 0x01001004;
pub const APM_CMD_GRAPH_FLUSH: u32 = 0x01001005;
pub const APM_CMD_SET_CFG: u32 = 0x01001006;
pub const APM_CMD_GET_CFG: u32 = 0x01001007;
pub const APM_CMD_SHARED_MEM_MAP_REGIONS: u32 = 0x0100100C;
pub const APM_CMD_SHARED_MEM_UNMAP_REGIONS: u32 = 0x0100100D;
pub const APM_CMD_REGISTER_MODULE_EVENTS: u32 = 0x0100100E;
pub const APM_EVENT_MODULE_TO_CLIENT: u32 = 0x03001000;
pub const APM_CMD_RSP_SHARED_MEM_MAP_REGIONS: u32 = 0x02001001;
pub const APM_MMAP_TOKEN_GID_MASK: u32 = genmask(15, 0);
pub const APM_MMAP_TOKEN_MAP_TYPE_POS_BUF: u32 = bit(16);
pub const APM_MMAP_TOKEN_MAP_TYPE_SHIFT: u32 = 16;
pub const APM_CMD_RSP_GET_CFG: u32 = 0x02001000;
pub const APM_CMD_CLOSE_ALL: u32 = 0x01001013;
pub const APM_CMD_REGISTER_SHARED_CFG: u32 = 0x0100100A;
pub const EVENT_ID_SH_MEM_PULL_PUSH_MODE_WATERMARK: u32 = 0x0800101C;

/**
 * struct event_cfg_sh_mem_pull_push_mode_watermark_t - Watermark config
 * @num_water_mark_levels: Number of watermark levels.
 * @level: Watermark levels.
 *
 * If @num_water_mark_levels is zero, no watermark levels are specified
 * and watermark events are not supported.
 */
#[repr(C, packed)]
pub struct event_cfg_sh_mem_pull_push_mode_watermark_t {
    pub num_water_mark_levels: uint32_t,
    pub level: [uint32_t; 0],
}

/**
 * struct apm_module_register_events - Register or unregister module events
 * @module_instance_id: Module instance identifier.
 * @event_id: Module event identifier.
 * @is_register: 1 to register the event, 0 to unregister it.
 * @error_code: Error code for out-of-band command mode.
 * @event_config_payload_size: Event configuration payload size in bytes.
 * @reserved: Reserved for alignment; must be zero.
 */
#[repr(C, packed)]
pub struct apm_module_register_events {
    pub module_instance_id: uint32_t,
    pub event_id: uint32_t,
    pub is_register: uint32_t,
    pub error_code: uint32_t,
    pub event_config_payload_size: uint32_t,
    pub reserved: uint32_t,
}

/**
 * struct apm_module_event - Module event descriptor
 * @event_id: Module event identifier.
 * @event_payload_size: Event payload size in bytes.
 */
#[repr(C, packed)]
pub struct apm_module_event {
    pub event_id: uint32_t,
    pub event_payload_size: uint32_t,
}

pub const APM_MEMORY_MAP_SHMEM8_4K_POOL: u32 = 3;

#[repr(C, packed)]
pub struct apm_cmd_shared_mem_map_regions {
    pub mem_pool_id: uint16_t,
    pub num_regions: uint16_t,
    pub property_flag: uint32_t,
}

#[repr(C, packed)]
pub struct apm_shared_map_region_payload {
    pub shm_addr_lsw: uint32_t,
    pub shm_addr_msw: uint32_t,
    pub mem_size_bytes: uint32_t,
}

#[repr(C, packed)]
pub struct apm_cmd_shared_mem_unmap_regions {
    pub mem_map_handle: uint32_t,
}

#[repr(C, packed)]
pub struct apm_cmd_rsp_shared_mem_map_regions {
    pub mem_map_handle: uint32_t,
}

/* APM module */
pub const APM_PARAM_ID_SUB_GRAPH_LIST: u32 = 0x08001005;
pub const APM_PARAM_ID_MODULE_LIST: u32 = 0x08001002;

#[repr(C, packed)]
pub struct apm_param_id_modules_list {
    pub num_modules_list: uint32_t,
}

pub const APM_PARAM_ID_MODULE_PROP: u32 = 0x08001003;

#[repr(C, packed)]
pub struct apm_param_id_module_prop {
    pub num_modules_prop_cfg: uint32_t,
}

#[repr(C, packed)]
pub struct apm_module_prop_cfg {
    pub instance_id: uint32_t,
    pub num_props: uint32_t,
}

pub const APM_PARAM_ID_MODULE_CONN: u32 = 0x08001004;

#[repr(C, packed)]
pub struct apm_param_id_module_conn {
    pub num_connections: uint32_t,
}

#[repr(C, packed)]
pub struct apm_module_conn_obj {
    pub src_mod_inst_id: uint32_t,
    pub src_mod_op_port_id: uint32_t,
    pub dst_mod_inst_id: uint32_t,
    pub dst_mod_ip_port_id: uint32_t,
}

pub const APM_PARAM_ID_GAIN: u32 = 0x08001006;

#[repr(C, packed)]
pub struct param_id_gain_cfg {
    pub gain: uint16_t,
    pub reserved: uint16_t,
}

pub const PARAM_ID_PCM_OUTPUT_FORMAT_CFG: u32 = 0x08001008;

#[repr(C, packed)]
pub struct param_id_pcm_output_format_cfg {
    pub data_format: uint32_t,
    pub fmt_id: uint32_t,
    pub payload_size: uint32_t,
}

#[repr(C, packed)]
pub struct payload_pcm_output_format_cfg {
    pub bit_width: uint16_t,
    pub alignment: uint16_t,
    pub bits_per_sample: uint16_t,
    pub q_factor: uint16_t,
    pub endianness: uint16_t,
    pub interleaved: uint16_t,
    pub reserved: uint16_t,
    pub num_channels: uint16_t,
    pub channel_mapping: [uint8_t; 0],
}

pub const PARAM_ID_ENC_BITRATE: u32 = 0x08001052;

#[repr(C, packed)]
pub struct param_id_enc_bitrate_param {
    pub bitrate: uint32_t,
}

pub const DATA_FORMAT_FIXED_POINT: u32 = 1;
pub const DATA_FORMAT_GENERIC_COMPRESSED: u32 = 5;
pub const DATA_FORMAT_RAW_COMPRESSED: u32 = 6;
pub const PCM_LSB_ALIGNED: u32 = 1;
pub const PCM_MSB_ALIGNED: u32 = 2;
pub const PCM_LITTLE_ENDIAN: u32 = 1;
pub const PCM_BIT_ENDIAN: u32 = 2;

pub const MEDIA_FMT_ID_PCM: u32 = 0x09001000;
pub const MEDIA_FMT_ID_MP3: u32 = 0x09001009;
pub const SAMPLE_RATE_48K: u32 = 48000;
pub const BIT_WIDTH_16: u32 = 16;

pub const APM_PARAM_ID_PROP_PORT_INFO: u32 = 0x08001015;

#[repr(C, packed)]
pub struct apm_modules_prop_info {
    pub max_ip_port: uint32_t,
    pub max_op_port: uint32_t,
}

/* Shared memory module */
pub const DATA_CMD_WR_SH_MEM_EP_DATA_BUFFER: u32 = 0x04001000;
pub const WR_SH_MEM_EP_TIMESTAMP_VALID_FLAG: u32 = bit(31);
pub const WR_SH_MEM_EP_LAST_BUFFER_FLAG: u32 = bit(30);
pub const WR_SH_MEM_EP_TS_CONTINUE_FLAG: u32 = bit(29);
pub const WR_SH_MEM_EP_EOF_FLAG: u32 = bit(4);

#[repr(C, packed)]
pub struct apm_data_cmd_wr_sh_mem_ep_data_buffer {
    pub buf_addr_lsw: uint32_t,
    pub buf_addr_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub buf_size: uint32_t,
    pub timestamp_lsw: uint32_t,
    pub timestamp_msw: uint32_t,
    pub flags: uint32_t,
}

pub const DATA_CMD_WR_SH_MEM_EP_DATA_BUFFER_V2: u32 = 0x0400100A;

#[repr(C, packed)]
pub struct apm_data_cmd_wr_sh_mem_ep_data_buffer_v2 {
    pub buf_addr_lsw: uint32_t,
    pub buf_addr_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub buf_size: uint32_t,
    pub timestamp_lsw: uint32_t,
    pub timestamp_msw: uint32_t,
    pub flags: uint32_t,
    pub md_addr_lsw: uint32_t,
    pub md_addr_msw: uint32_t,
    pub md_map_handle: uint32_t,
    pub md_buf_size: uint32_t,
}

pub const DATA_CMD_RSP_WR_SH_MEM_EP_DATA_BUFFER_DONE: u32 = 0x05001000;

#[repr(C, packed)]
pub struct data_cmd_rsp_wr_sh_mem_ep_data_buffer_done {
    pub buf_addr_lsw: uint32_t,
    pub buf_addr_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub status: uint32_t,
}

pub const DATA_CMD_RSP_WR_SH_MEM_EP_DATA_BUFFER_DONE_V2: u32 = 0x05001004;

#[repr(C, packed)]
pub struct data_cmd_rsp_wr_sh_mem_ep_data_buffer_done_v2 {
    pub buf_addr_lsw: uint32_t,
    pub buf_addr_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub status: uint32_t,
    pub md_buf_addr_lsw: uint32_t,
    pub md_buf_addr_msw: uint32_t,
    pub md_mem_map_handle: uint32_t,
    pub md_status: uint32_t,
}

pub const PARAM_ID_MEDIA_FORMAT: u32 = 0x0800100C;
pub const DATA_CMD_WR_SH_MEM_EP_MEDIA_FORMAT: u32 = 0x04001001;

#[repr(C, packed)]
pub struct apm_media_format {
    pub data_format: uint32_t,
    pub fmt_id: uint32_t,
    pub payload_size: uint32_t,
}

pub const MEDIA_FMT_ID_FLAC: u32 = 0x09001004;

#[repr(C, packed)]
pub struct payload_media_fmt_flac_t {
    pub num_channels: uint16_t,
    pub sample_size: uint16_t,
    pub min_blk_size: uint16_t,
    pub max_blk_size: uint16_t,
    pub sample_rate: uint32_t,
    pub min_frame_size: uint32_t,
    pub max_frame_size: uint32_t,
}

pub const MEDIA_FMT_ID_AAC: u32 = 0x09001001;

#[repr(C, packed)]
pub struct payload_media_fmt_aac_t {
    pub aac_fmt_flag: uint16_t,
    pub audio_obj_type: uint16_t,
    pub num_channels: uint16_t,
    pub total_size_of_PCE_bits: uint16_t,
    pub sample_rate: uint32_t,
}

pub const MEDIA_FMT_ID_OPUS: u32 = 0x09001039;

#[repr(C, packed)]
pub struct payload_media_fmt_opus_t {
    pub bitstream_format: uint16_t,
    pub payload_type: uint16_t,
    pub version: uint8_t,
    pub num_channels: uint8_t,
    pub pre_skip: uint16_t,
    pub sample_rate: uint32_t,
    pub output_gain: uint16_t,
    pub mapping_family: uint8_t,
    pub stream_count: uint8_t,
    pub coupled_count: uint8_t,
    pub channel_mapping: [uint8_t; 8],
    pub reserved: [uint8_t; 3],
}

pub const DATA_CMD_WR_SH_MEM_EP_EOS: u32 = 0x04001002;
pub const WR_SH_MEM_EP_EOS_POLICY_LAST: u32 = 1;
pub const WR_SH_MEM_EP_EOS_POLICY_EACH: u32 = 2;

#[repr(C, packed)]
pub struct data_cmd_wr_sh_mem_ep_eos {
    pub policy: uint32_t,
}

pub const DATA_CMD_RD_SH_MEM_EP_DATA_BUFFER: u32 = 0x04001003;

#[repr(C, packed)]
pub struct data_cmd_rd_sh_mem_ep_data_buffer {
    pub buf_addr_lsw: uint32_t,
    pub buf_addr_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub buf_size: uint32_t,
}

pub const DATA_CMD_RSP_RD_SH_MEM_EP_DATA_BUFFER: u32 = 0x05001002;

#[repr(C, packed)]
pub struct data_cmd_rsp_rd_sh_mem_ep_data_buffer_done {
    pub status: uint32_t,
    pub buf_addr_lsw: uint32_t,
    pub buf_addr_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub data_size: uint32_t,
    pub offset: uint32_t,
    pub timestamp_lsw: uint32_t,
    pub timestamp_msw: uint32_t,
    pub flags: uint32_t,
    pub num_frames: uint32_t,
}

pub const DATA_CMD_RD_SH_MEM_EP_DATA_BUFFER_V2: u32 = 0x0400100B;

#[repr(C, packed)]
pub struct data_cmd_rd_sh_mem_ep_data_buffer_v2 {
    pub buf_addr_lsw: uint32_t,
    pub buf_addr_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub buf_size: uint32_t,
    pub md_buf_addr_lsw: uint32_t,
    pub md_buf_addr_msw: uint32_t,
    pub md_mem_map_handle: uint32_t,
    pub md_buf_size: uint32_t,
}

pub const DATA_CMD_RSP_RD_SH_MEM_EP_DATA_BUFFER_V2: u32 = 0x05001005;

#[repr(C, packed)]
pub struct data_cmd_rsp_rd_sh_mem_ep_data_buffer_done_v2 {
    pub status: uint32_t,
    pub buf_addr_lsw: uint32_t,
    pub buf_addr_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub data_size: uint32_t,
    pub offset: uint32_t,
    pub timestamp_lsw: uint32_t,
    pub timestamp_msw: uint32_t,
    pub flags: uint32_t,
    pub num_frames: uint32_t,
    pub md_status: uint32_t,
    pub md_buf_addr_lsw: uint32_t,
    pub md_buf_addr_msw: uint32_t,
    pub md_mem_map_handle: uint32_t,
    pub md_size: uint32_t,
}

pub const PARAM_ID_RD_SH_MEM_CFG: u32 = 0x08001007;

#[repr(C, packed)]
pub struct param_id_rd_sh_mem_cfg {
    pub num_frames_per_buffer: uint32_t,
    pub metadata_control_flags: uint32_t,
}

pub const DATA_CMD_WR_SH_MEM_EP_EOS_RENDERED: u32 = 0x05001001;

#[repr(C, packed)]
pub struct data_cmd_wr_sh_mem_ep_eos_rendered {
    pub module_instance_id: uint32_t,
    pub render_status: uint32_t,
}

#[repr(C, packed)]
pub struct apm_cmd_header {
    pub payload_address_lsw: uint32_t,
    pub payload_address_msw: uint32_t,
    pub mem_map_handle: uint32_t,
    pub payload_size: uint32_t,
}

pub const APM_CMD_HDR_SIZE: usize = core::mem::size_of::<apm_cmd_header>();

#[repr(C, packed)]
pub struct apm_module_param_data {
    pub module_instance_id: uint32_t,
    pub param_id: uint32_t,
    pub param_size: uint32_t,
    pub error_code: uint32_t,
}

pub const APM_MODULE_PARAM_DATA_SIZE: usize = core::mem::size_of::<apm_module_param_data>();

#[repr(C, packed)]
pub struct apm_module_param_shared_data {
    pub param_id: uint32_t,
    pub param_size: uint32_t,
}

#[repr(C, packed)]
pub struct apm_prop_data {
    pub prop_id: uint32_t,
    pub prop_size: uint32_t,
}

/* Sub-Graph Properties */
pub const APM_PARAM_ID_SUB_GRAPH_CONFIG: u32 = 0x08001001;

#[repr(C, packed)]
pub struct apm_param_id_sub_graph_cfg {
    pub num_sub_graphs: uint32_t,
}

#[repr(C, packed)]
pub struct apm_sub_graph_cfg {
    pub sub_graph_id: uint32_t,
    pub num_sub_graph_prop: uint32_t,
}

pub const APM_SUB_GRAPH_PROP_ID_PERF_MODE: u32 = 0x0800100E;

#[repr(C, packed)]
pub struct apm_sg_prop_id_perf_mode {
    pub perf_mode: uint32_t,
}

pub const APM_SG_PROP_ID_PERF_MODE_SIZE: u32 = 4;

pub const APM_SUB_GRAPH_PROP_ID_DIRECTION: u32 = 0x0800100F;

#[repr(C, packed)]
pub struct apm_sg_prop_id_direction {
    pub direction: uint32_t,
}

pub const APM_SG_PROP_ID_DIR_SIZE: u32 = 4;

pub const APM_SUB_GRAPH_PROP_ID_SCENARIO_ID: u32 = 0x08001010;
pub const APM_SUB_GRAPH_SID_AUDIO_PLAYBACK: u32 = 0x1;
pub const APM_SUB_GRAPH_SID_AUDIO_RECORD: u32 = 0x2;
pub const APM_SUB_GRAPH_SID_AUDIO_VOICE_CALL: u32 = 0x3;

#[repr(C, packed)]
pub struct apm_sg_prop_id_scenario_id {
    pub scenario_id: uint32_t,
}

pub const APM_SG_PROP_ID_SID_SIZE: u32 = 4;
/* container api */
pub const APM_PARAM_ID_CONTAINER_CONFIG: u32 = 0x08001000;

#[repr(C, packed)]
pub struct apm_param_id_container_cfg {
    pub num_containers: uint32_t,
}

#[repr(C, packed)]
pub struct apm_container_cfg {
    pub container_id: uint32_t,
    pub num_prop: uint32_t,
}

#[repr(C, packed)]
pub struct apm_cont_capability {
    pub capability_id: uint32_t,
}

pub const APM_CONTAINER_PROP_ID_CAPABILITY_LIST: u32 = 0x08001011;
pub const APM_CONTAINER_PROP_ID_CAPABILITY_SIZE: u32 = 8;

pub const APM_PROP_ID_INVALID: u32 = 0x0;
pub const APM_CONTAINER_CAP_ID_PP: u32 = 0x1;

#[repr(C, packed)]
pub struct apm_cont_prop_id_cap_list {
    pub num_capability_id: uint32_t,
}

pub const APM_CONTAINER_PROP_ID_GRAPH_POS: u32 = 0x08001012;

#[repr(C, packed)]
pub struct apm_cont_prop_id_graph_pos {
    pub graph_pos: uint32_t,
}

pub const APM_CONTAINER_PROP_ID_STACK_SIZE: u32 = 0x08001013;

#[repr(C, packed)]
pub struct apm_cont_prop_id_stack_size {
    pub stack_size: uint32_t,
}

pub const APM_CONTAINER_PROP_ID_PROC_DOMAIN: u32 = 0x08001014;

#[repr(C, packed)]
pub struct apm_cont_prop_id_domain {
    pub proc_domain: uint32_t,
}

pub const CONFIG_I2S_WS_SRC_EXTERNAL: u32 = 0x0;
pub const CONFIG_I2S_WS_SRC_INTERNAL: u32 = 0x1;

pub const PARAM_ID_I2S_INTF_CFG: u32 = 0x08001019;

#[repr(C, packed)]
pub struct param_id_i2s_intf_cfg {
    pub lpaif_type: uint32_t,
    pub intf_idx: uint32_t,
    pub sd_line_idx: uint16_t,
    pub ws_src: uint16_t,
}

pub const I2S_INTF_TYPE_PRIMARY: u32 = 0;
pub const I2S_INTF_TYPE_SECONDARY: u32 = 1;
pub const I2S_INTF_TYPE_TERTIARY: u32 = 2;
pub const I2S_INTF_TYPE_QUATERNARY: u32 = 3;
pub const I2S_INTF_TYPE_QUINARY: u32 = 4;
pub const I2S_SD0: u32 = 1;
pub const I2S_SD1: u32 = 2;
pub const I2S_SD2: u32 = 3;
pub const I2S_SD3: u32 = 4;

pub const PORT_ID_I2S_INPUT: u32 = 2;
pub const PORT_ID_I2S_OUPUT: u32 = 1;
pub const I2S_STACK_SIZE: u32 = 2048;

pub const PARAM_ID_AUDIO_IF_INTF_CFG: u32 = 0x08001B11;

/*
 * struct param_id_audio_if_intf_cfg - Audio interface configuration
 * @qaif_type: Audio interface type (e.g. QAIF, QAIF_VA)
 * @intf_idx: Interface instance index
 * @intf_mode: Interface operating mode (TDM/PCM/I2S)
 * @ctrl_data_out_enable: Enable sharing of data-out signal with other masters
 * @active_slot_mask: Bitmask indicating active slots
 * @nslots_per_frame: Number of slots per audio frame
 * @slot_width: Width of each slot in bits
 * @active_lane_mask: Bitmask of active data lanes
 * @frame_sync_rate: Frame sync rate in Hz
 * @frame_sync_src: Frame sync source selection
 * @frame_sync_mode: Frame sync mode configuration
 * @invert_frame_sync_pulse: Invert frame sync polarity when set
 * @frame_sync_data_delay: Data delay from frame sync in bit clocks
 * @bit_clk_type: Bit clock type (internal / external)
 * @inv_int_bit_clk: Invert internal bit clock when set
 * @inv_ext_bit_clk: Invert external bit clock when set
 *
 * This structure defines configuration parameters for the Qualcomm
 * Audio Interface (QAIF) block. It is used to program interface
 * characteristics such as slot configuration, clocking and frame
 * synchronization behaviour.
 */
#[repr(C, packed)]
pub struct param_id_audio_if_intf_cfg {
    pub qaif_type: uint16_t,
    pub intf_idx: uint16_t,
    pub intf_mode: uint16_t,
    pub ctrl_data_out_enable: uint16_t,
    pub active_slot_mask: uint32_t,
    pub nslots_per_frame: uint16_t,
    pub slot_width: uint16_t,
    pub active_lane_mask: uint32_t,
    pub frame_sync_rate: uint32_t,
    pub frame_sync_src: uint16_t,
    pub frame_sync_mode: uint16_t,
    pub invert_frame_sync_pulse: uint16_t,
    pub frame_sync_data_delay: uint16_t,
    pub bit_clk_type: uint16_t,
    pub inv_int_bit_clk: uint8_t,
    pub inv_ext_bit_clk: uint8_t,
}

pub const PARAM_ID_HW_EP_FRAME_DURATION: u32 = 0x08001B2F;
pub const AUDIO_IF_FRAME_DURATION_US: u32 = 1000;
pub const AUDIO_IF_FRAME_DURATION_NORMALIZATION_ENABLE: u32 = 1;
pub const AUDIO_IF_FRAME_DURATION_MIN_US: u32 = 1;
pub const AUDIO_IF_FRAME_DURATION_MAX_US: u32 = 100000;

/**
 * struct param_id_hw_ep_frame_duration - Hardware endpoint frame duration
 * @frame_duration_in_us: Frame duration in microseconds.
 * @allow_frame_duration_normalization: Permit SPF to normalize frame duration.
 * @min_normalized_frame_dur_us: Minimum normalized frame duration in microseconds.
 * @max_normalized_frame_dur_us: Maximum normalized frame duration in microseconds.
 *
 * This structure configures the frame duration for the Audio IF hardware
 * endpoint and, when enabled, the allowed normalization range.
 */
#[repr(C, packed)]
pub struct param_id_hw_ep_frame_duration {
    pub frame_duration_in_us: uint32_t,
    pub allow_frame_duration_normalization: uint32_t,
    pub min_normalized_frame_dur_us: uint32_t,
    pub max_normalized_frame_dur_us: uint32_t,
}

pub const PARAM_ID_DISPLAY_PORT_INTF_CFG: u32 = 0x08001154;

#[repr(C, packed)]
pub struct param_id_display_port_intf_cfg {
    pub channel_allocation: uint32_t,
    /* Multi-Steam Transport index */
    pub mst_idx: uint32_t,
    pub dptx_idx: uint32_t,
}

pub const PARAM_ID_HW_EP_MF_CFG: u32 = 0x08001017;

#[repr(C, packed)]
pub struct param_id_hw_ep_mf {
    pub sample_rate: uint32_t,
    pub bit_width: uint16_t,
    pub num_channels: uint16_t,
    pub data_format: uint32_t,
}

pub const PARAM_ID_HW_EP_FRAME_SIZE_FACTOR: u32 = 0x08001018;

#[repr(C, packed)]
pub struct param_id_fram_size_factor {
    pub frame_size_factor: uint32_t,
}

pub const APM_CONTAINER_PROP_ID_PARENT_CONTAINER_ID: u32 = 0x080010CB;

#[repr(C, packed)]
pub struct apm_cont_prop_id_parent_container {
    pub parent_container_id: uint32_t,
}

pub const APM_CONTAINER_PROP_ID_HEAP_ID: u32 = 0x08001174;
pub const APM_CONT_HEAP_DEFAULT: u32 = 0x1;
pub const APM_CONT_HEAP_LOW_POWER: u32 = 0x2;

#[repr(C, packed)]
pub struct apm_cont_prop_id_headp_id {
    pub heap_id: uint32_t,
}

#[repr(C, packed)]
pub struct apm_modules_list {
    pub sub_graph_id: uint32_t,
    pub container_id: uint32_t,
    pub num_modules: uint32_t,
}

#[repr(C, packed)]
pub struct apm_module_obj {
    pub module_id: uint32_t,
    pub instance_id: uint32_t,
}

pub const APM_MODULE_PROP_ID_PORT_INFO: u32 = 0x08001015;
pub const APM_MODULE_PROP_ID_PORT_INFO_SZ: u32 = 8;

#[repr(C, packed)]
pub struct apm_module_prop_id_port_info {
    pub max_ip_port: uint32_t,
    pub max_op_port: uint32_t,
}

pub const DATA_LOGGING_MAX_INPUT_PORTS: u32 = 0x1;
pub const DATA_LOGGING_MAX_OUTPUT_PORTS: u32 = 0x1;
pub const DATA_LOGGING_STACK_SIZE: u32 = 2048;
pub const PARAM_ID_DATA_LOGGING_CONFIG: u32 = 0x08001031;

#[repr(C, packed)]
pub struct data_logging_config {
    pub log_code: uint32_t,
    pub log_tap_point_id: uint32_t,
    pub mode: uint32_t,
}

/* Speaker Protection */
pub const PARAM_ID_SP_OP_MODE: u32 = 0x080011e9;
pub const PARAM_ID_SP_OP_MODE_NORMAL: u32 = 0;
pub const PARAM_ID_SP_OP_MODE_CALIBRATION: u32 = 1;
pub const PARAM_ID_SP_OP_MODE_FACTORY_TEST: u32 = 2;
pub const PARAM_ID_SP_OP_MODE_VALIDATION: u32 = 3;

#[repr(C, packed)]
pub struct param_id_sp_op_mode {
    pub operation_mode: uint32_t,
}

/* Speaker Protection VI */

pub const PARAM_ID_SP_VI_OP_MODE_CFG: u32 = 0x080011f4;
pub const PARAM_ID_SP_VI_OP_MODE_NORMAL: u32 = 0;
pub const PARAM_ID_SP_VI_OP_MODE_CALIBRATION: u32 = 1;
pub const PARAM_ID_SP_VI_OP_MODE_FACTORY_TEST: u32 = 2;
pub const PARAM_ID_SP_VI_OP_MODE_VALIDATION: u32 = 3;

#[repr(C, packed)]
pub struct param_id_sp_vi_op_mode_cfg {
    pub num_channels: uint32_t,
    pub operation_mode: uint32_t,
    pub quick_calibration: uint32_t,
    pub r0_t0_selection: [uint32_t; 0],
}

pub const PARAM_ID_SP_VI_EX_MODE_CFG: u32 = 0x080011ff;

#[repr(C, packed)]
pub struct param_id_sp_vi_ex_mode_cfg {
    pub factory_mode: uint32_t,
}

pub const PARAM_ID_SP_VI_CHANNEL_MAP_CFG: u32 = 0x08001203;

#[repr(C, packed)]
pub struct param_id_sp_vi_channel_map_cfg {
    pub num_channels: uint32_t,
    /* [ Vsense of ch 1, Isense of ch 1, Vsense of ch 2, Isense of ch 2, ... ] */
    pub channel_mapping: [uint32_t; 0],
}

pub const PARAM_ID_SAL_OUTPUT_CFG: u32 = 0x08001016;

#[repr(C, packed)]
pub struct param_id_sal_output_config {
    pub bits_per_sample: uint32_t,
}

pub const PARAM_ID_SAL_LIMITER_ENABLE: u32 = 0x0800101E;

#[repr(C, packed)]
pub struct param_id_sal_limiter_enable {
    pub enable_lim: uint32_t,
}

pub const PARAM_ID_MFC_OUTPUT_MEDIA_FORMAT: u32 = 0x08001024;
pub const PARAM_ID_EARLY_EOS_DELAY: u32 = 0x0800114C;
pub const EARLY_EOS_DELAY_MS: u32 = 150;

#[repr(C, packed)]
pub struct param_id_mfc_media_format {
    pub sample_rate: uint32_t,
    pub bit_width: uint16_t,
    pub num_channels: uint16_t,
    pub channel_mapping: [uint16_t; 0],
}

#[repr(C, packed)]
pub struct param_id_gapless_early_eos_delay_t {
    pub early_eos_delay_ms: uint32_t,
}

#[repr(C, packed)]
pub struct media_format {
    pub data_format: uint32_t,
    pub fmt_id: uint32_t,
    pub payload_size: uint32_t,
}

#[repr(C, packed)]
pub struct payload_media_fmt_pcm {
    pub sample_rate: uint32_t,
    pub bit_width: uint16_t,
    pub alignment: uint16_t,
    pub bits_per_sample: uint16_t,
    pub q_factor: uint16_t,
    pub endianness: uint16_t,
    pub num_channels: uint16_t,
    pub channel_mapping: [uint8_t; 0],
}

pub const PARAM_ID_MODULE_ENABLE: u32 = 0x08001026;

#[repr(C, packed)]
pub struct param_id_module_enable {
    pub enable: uint32_t,
}

pub const PARAM_ID_CODEC_DMA_INTF_CFG: u32 = 0x08001063;

#[repr(C, packed)]
pub struct param_id_codec_dma_intf_cfg {
    /* 1 - RXTX
     * 2 - WSA
     * 3 - VA
     * 4 - AXI
     */
    pub lpaif_type: uint32_t,
    /*
     *  RX0 | TX0 = 1
     *  RX1 | TX1 = 2
     *  RX2 | TX2 = 3... so on
     */
    pub intf_index: uint32_t,
    pub active_channels_mask: uint32_t,
}

#[repr(C, packed)]
pub struct audio_hw_clk_cfg {
    pub clock_id: uint32_t,
    pub clock_freq: uint32_t,
    pub clock_attri: uint32_t,
    pub clock_root: uint32_t,
}

#[repr(C, packed)]
pub struct audio_hw_clk_rel_cfg {
    pub clock_id: uint32_t,
}

pub const PARAM_ID_HW_EP_POWER_MODE_CFG: u32 = 0x8001176;
pub const AR_HW_EP_POWER_MODE_0: u32 = 0; /* default */
pub const AR_HW_EP_POWER_MODE_1: u32 = 1; /* XO Shutdown allowed */
pub const AR_HW_EP_POWER_MODE_2: u32 = 2; /* XO Shutdown not allowed */

#[repr(C, packed)]
pub struct param_id_hw_ep_power_mode_cfg {
    pub power_mode: uint32_t,
}

pub const PARAM_ID_HW_EP_DMA_DATA_ALIGN: u32 = 0x08001233;
pub const AR_HW_EP_DMA_DATA_ALIGN_MSB: u32 = 0;
pub const AR_HW_EP_DMA_DATA_ALIGN_LSB: u32 = 1;
pub const AR_PCM_MAX_NUM_CHANNEL: usize = 8;

#[repr(C, packed)]
pub struct param_id_hw_ep_dma_data_align {
    pub dma_data_align: uint32_t,
}

pub const PARAM_ID_VOL_CTRL_MASTER_GAIN: u32 = 0x08001035;
pub const VOL_CTRL_DEFAULT_GAIN: u32 = 0x2000;

#[repr(C, packed)]
pub struct param_id_vol_ctrl_master_gain {
    pub master_gain: uint16_t,
    pub reserved: uint16_t,
}

pub const PARAM_ID_REMOVE_INITIAL_SILENCE: u32 = 0x0800114B;
pub const PARAM_ID_REMOVE_TRAILING_SILENCE: u32 = 0x0800115D;

pub const PARAM_ID_REAL_MODULE_ID: u32 = 0x0800100B;

#[repr(C, packed)]
pub struct param_id_placeholder_real_module_id {
    pub real_module_id: uint32_t,
}

pub const PARAM_ID_SH_MEM_PULL_PUSH_MODE_CFG: u32 = 0x0800100A;

/**
 * struct param_id_sh_mem_pull_push_mode_cfg - Shared memory push/pull config
 * @shared_circ_buf_addr_lsw: Lower 32 bits of the circular buffer address.
 * @shared_circ_buf_addr_msw: Upper 32 bits of the circular buffer address.
 * @shared_circ_buf_size: Circular buffer size in bytes.
 * @circ_buf_mem_map_handle: Circular buffer memory map handle.
 * @shared_pos_buf_addr_lsw: Lower 32 bits of the position buffer address.
 * @shared_pos_buf_addr_msw: Upper 32 bits of the position buffer address.
 * @pos_buf_mem_map_handle: Position buffer memory map handle.
 */
#[repr(C, packed)]
pub struct param_id_sh_mem_pull_push_mode_cfg {
    pub shared_circ_buf_addr_lsw: uint32_t,
    pub shared_circ_buf_addr_msw: uint32_t,
    pub shared_circ_buf_size: uint32_t,
    pub circ_buf_mem_map_handle: uint32_t,
    pub shared_pos_buf_addr_lsw: uint32_t,
    pub shared_pos_buf_addr_msw: uint32_t,
    pub pos_buf_mem_map_handle: uint32_t,
}

/**
 * struct sh_mem_pull_push_mode_position_buffer - Shared position buffer
 * @frame_counter: Synchronization counter.
 * @index: Current read/write index in bytes.
 * @timestamp_us_lsw: Lower 32 bits of the timestamp in microseconds.
 * @timestamp_us_msw: Upper 32 bits of the timestamp in microseconds.
 *
 * The frame counter should be read before and after the other fields to
 * ensure the DSP did not update them while they were being read.
 */
#[repr(C, packed)]
pub struct sh_mem_pull_push_mode_position_buffer {
    pub frame_counter: uint32_t,
    pub index: uint32_t,
    pub timestamp_us_lsw: uint32_t,
    pub timestamp_us_msw: uint32_t,
}

/* Graph */
#[repr(C)]
pub struct audioreach_connection {
    /* Connections */
    pub src_mod_inst_id: uint32_t,
    pub src_mod_op_port_id: uint32_t,
    pub dst_mod_inst_id: uint32_t,
    pub dst_mod_ip_port_id: uint32_t,
    pub node: list_head,
}

#[repr(C)]
pub struct audioreach_graph_info {
    pub id: c_int,
    pub mem_map_handle: uint32_t,
    pub pos_buf_mem_map_handle: uint32_t,
    pub num_sub_graphs: uint32_t,
    pub sg_list: list_head,
    pub is_push_pull_mode: bool_,
    /* DPCM connection from FE Graph to BE graph */
    pub src_mod_inst_id: uint32_t,
    pub src_mod_op_port_id: uint32_t,
    pub dst_mod_inst_id: uint32_t,
    pub dst_mod_ip_port_id: uint32_t,
}

#[repr(C)]
pub struct audioreach_sub_graph {
    pub sub_graph_id: uint32_t,
    pub perf_mode: uint32_t,
    pub direction: uint32_t,
    pub scenario_id: uint32_t,
    pub node: list_head,
    pub info: *mut audioreach_graph_info,
    pub num_containers: uint32_t,
    pub container_list: list_head,
}

#[repr(C)]
pub struct audioreach_container {
    pub container_id: uint32_t,
    pub capability_id: uint32_t,
    pub graph_pos: uint32_t,
    pub stack_size: uint32_t,
    pub proc_domain: uint32_t,
    pub node: list_head,
    pub num_modules: uint32_t,
    pub modules_list: list_head,
    pub sub_graph: *mut audioreach_sub_graph,
}

pub const AR_MAX_MOD_LINKS: usize = 8;

#[repr(C)]
pub struct audioreach_module {
    pub module_id: uint32_t,
    pub instance_id: uint32_t,
    pub max_ip_port: uint32_t,
    pub max_op_port: uint32_t,
    pub num_connections: uint32_t,
    /* Connections */
    pub src_mod_inst_id: uint32_t,
    pub src_mod_op_port_id: [uint32_t; AR_MAX_MOD_LINKS],
    pub dst_mod_inst_id: [uint32_t; AR_MAX_MOD_LINKS],
    pub dst_mod_ip_port_id: [uint32_t; AR_MAX_MOD_LINKS],
    /* Format specifics */
    pub ch_fmt: uint32_t,
    pub rate: uint32_t,
    pub bit_depth: uint32_t,
    /* I2S module */
    pub hw_interface_idx: uint32_t,
    pub sd_line_idx: uint32_t,
    pub ws_src: uint32_t,
    pub frame_size_factor: uint32_t,
    pub data_format: uint32_t,
    pub hw_interface_type: uint32_t,
    /* Audio IF module (TDM/PCM/I2S) */
    pub slot_mask: u32,
    pub active_lane_mask: u32,
    pub frame_sync_rate: u32,
    pub qaif_type: u16,
    pub sync_src: u16,
    pub ctrl_data_out_enable: u16,
    pub nslots_per_frame: u16,
    pub slot_width: u16,
    pub intf_mode: u16,
    pub sync_mode: u16,
    pub ctrl_invert_sync_pulse: u16,
    pub ctrl_sync_data_delay: u16,
    pub bit_clk_type: u16,
    pub inv_int_bit_clk: u8,
    pub inv_ext_bit_clk: u8,
    /* PCM module specific */
    pub interleave_type: uint32_t,
    /* GAIN/Vol Control Module */
    pub gain: uint16_t,
    /* Logging */
    pub log_code: uint32_t,
    pub log_tap_point_id: uint32_t,
    pub log_mode: uint32_t,
    /* bookkeeping */
    pub node: list_head,
    pub container: *mut audioreach_container,
    pub widget: *mut snd_soc_dapm_widget,
    pub data: *mut audioreach_module_priv_data,
}

#[repr(C)]
pub struct audioreach_module_config {
    pub direction: c_int,
    pub sample_rate: u32,
    pub bit_width: u16,
    pub bits_per_sample: u16,
    pub data_format: u16,
    pub num_channels: u16,
    pub dp_idx: u16,
    pub channel_allocation: u32,
    pub sd_line_mask: u32,
    pub fmt: c_int,
    pub slot_mask: u32,
    pub nslots_per_frame: u16,
    pub slot_width: u16,
    pub codec: snd_codec,
    pub channel_map: [u8; AR_PCM_MAX_NUM_CHANNEL],
}

unsafe extern "C" {
    /* Packet Allocation routines */
    pub fn audioreach_alloc_apm_cmd_pkt(
        pkt_size: c_int,
        opcode: uint32_t,
        token: uint32_t,
    ) -> *mut c_void;
    pub fn audioreach_set_default_channel_mapping(ch_map: *mut u8, num_channels: c_int);
    pub fn audioreach_alloc_cmd_pkt(
        payload_size: c_int,
        opcode: uint32_t,
        token: uint32_t,
        src_port: uint32_t,
        dest_port: uint32_t,
    ) -> *mut c_void;
    pub fn audioreach_alloc_apm_pkt(
        pkt_size: c_int,
        opcode: uint32_t,
        token: uint32_t,
        src_port: uint32_t,
    ) -> *mut c_void;
    pub fn audioreach_alloc_pkt(
        payload_size: c_int,
        opcode: uint32_t,
        token: uint32_t,
        src_port: uint32_t,
        dest_port: uint32_t,
    ) -> *mut c_void;
    pub fn audioreach_alloc_graph_pkt(
        apm: *mut q6apm,
        info: *const audioreach_graph_info,
    ) -> *mut c_void;
    /* Topology specific */
    pub fn audioreach_tplg_init(component: *mut snd_soc_component) -> c_int;

    /* Module specific */
    pub fn audioreach_graph_free_buf(graph: *mut q6apm_graph);
    pub fn audioreach_send_cmd_sync(
        dev: *mut device,
        gdev: *mut gpr_device_t,
        result: *mut gpr_ibasic_rsp_result_t,
        cmd_lock: *mut mutex,
        port: *mut gpr_port_t,
        cmd_wait: *mut wait_queue_head_t,
        pkt: *const gpr_pkt,
        rsp_opcode: uint32_t,
    ) -> c_int;
    pub fn audioreach_graph_send_cmd_sync(
        graph: *mut q6apm_graph,
        pkt: *const gpr_pkt,
        rsp_opcode: uint32_t,
    ) -> c_int;
    pub fn audioreach_set_media_format(
        graph: *mut q6apm_graph,
        module: *const audioreach_module,
        cfg: *const audioreach_module_config,
    ) -> c_int;
    pub fn audioreach_shared_memory_send_eos(graph: *mut q6apm_graph) -> c_int;
    pub fn audioreach_gain_set_vol_ctrl(
        apm: *mut q6apm,
        module: *const audioreach_module,
        vol: c_int,
    ) -> c_int;
    pub fn audioreach_send_u32_param(
        graph: *mut q6apm_graph,
        module: *const audioreach_module,
        param_id: uint32_t,
        param_val: uint32_t,
    ) -> c_int;
    pub fn audioreach_compr_set_param(
        graph: *mut q6apm_graph,
        mcfg: *const audioreach_module_config,
    ) -> c_int;
    pub fn audioreach_setup_push_pull(
        graph: *mut q6apm_graph,
        bphys: phys_addr_t,
        pphys: phys_addr_t,
        mem_map_handle: uint32_t,
        pos_buf_mem_map_handle: uint32_t,
        size: uint32_t,
    ) -> c_int;
    pub fn audioreach_map_memory_position_buffer(
        graph: *mut q6apm_graph,
        dir: c_uint,
    ) -> c_int;
    pub fn audioreach_shmem_register_event(
        graph: *mut q6apm_graph,
        bytes: c_int,
        num_levels: c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
