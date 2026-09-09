/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* Faithful low-level Rust translation of the IPC4 header declarations. */

use core::ffi::c_void;

pub const SOF_IPC4_MSG_MAX_SIZE: usize = 4096;
pub const fn bit(n: u32) -> u32 { 1u32 << n }
pub const fn genmask(hi: u32, lo: u32) -> u32 { ((1u32 << (hi + 1)) - 1) & !((1u32 << lo) - 1) }

#[repr(C)]
pub union sof_ipc4_msg_header { pub header_u64: u64, pub fields: sof_ipc4_msg_header_fields }
#[repr(C)] pub struct sof_ipc4_msg_header_fields { pub primary: u32, pub extension: u32 }
#[repr(C)] pub struct sof_ipc4_msg { pub header: sof_ipc4_msg_header, pub data_size: usize, pub data_ptr: *mut c_void }
#[repr(C, packed)] pub struct sof_ipc4_tuple { pub type_: u32, pub size: u32, pub value: [u32; 0] }

#[repr(i32)] pub enum sof_ipc4_msg_target { SOF_IPC4_FW_GEN_MSG, SOF_IPC4_MODULE_MSG }
#[repr(i32)] pub enum sof_ipc4_global_msg {
 SOF_IPC4_GLB_BOOT_CONFIG, SOF_IPC4_GLB_ROM_CONTROL, SOF_IPC4_GLB_IPCGATEWAY_CMD,
 SOF_IPC4_GLB_PERF_MEASUREMENTS_CMD = 13, SOF_IPC4_GLB_CHAIN_DMA,
 SOF_IPC4_GLB_LOAD_MULTIPLE_MODULES, SOF_IPC4_GLB_UNLOAD_MULTIPLE_MODULES,
 SOF_IPC4_GLB_CREATE_PIPELINE, SOF_IPC4_GLB_DELETE_PIPELINE, SOF_IPC4_GLB_SET_PIPELINE_STATE,
 SOF_IPC4_GLB_GET_PIPELINE_STATE, SOF_IPC4_GLB_GET_PIPELINE_CONTEXT_SIZE, SOF_IPC4_GLB_SAVE_PIPELINE,
 SOF_IPC4_GLB_RESTORE_PIPELINE, SOF_IPC4_GLB_LOAD_LIBRARY, SOF_IPC4_GLB_LOAD_LIBRARY_PREPARE,
 SOF_IPC4_GLB_INTERNAL_MESSAGE, SOF_IPC4_GLB_NOTIFICATION, SOF_IPC4_GLB_TYPE_LAST,
}
#[repr(i32)] pub enum sof_ipc4_msg_dir { SOF_IPC4_MSG_REQUEST, SOF_IPC4_MSG_REPLY }
#[repr(i32)] pub enum sof_ipc4_pipeline_state { SOF_IPC4_PIPE_INVALID_STATE, SOF_IPC4_PIPE_UNINITIALIZED, SOF_IPC4_PIPE_RESET, SOF_IPC4_PIPE_PAUSED, SOF_IPC4_PIPE_RUNNING, SOF_IPC4_PIPE_EOS }

macro_rules! m { ($n:ident, $e:expr) => { pub const $n: u32 = $e; }; }
m!(SOF_IPC4_MSG_TARGET_SHIFT,30); m!(SOF_IPC4_MSG_TARGET_MASK,bit(30));
macro_rules! SOF_IPC4_MSG_TARGET { ($x:expr) => { ($x) << 30 }; }
macro_rules! SOF_IPC4_MSG_IS_MODULE_MSG { ($x:expr) => { if (($x) & bit(30)) != 0 { 1 } else { 0 } }; }
m!(SOF_IPC4_MSG_DIR_SHIFT,29); m!(SOF_IPC4_MSG_DIR_MASK,bit(29)); macro_rules! SOF_IPC4_MSG_DIR { ($x:expr) => { ($x) << 29 }; }
m!(SOF_IPC4_MSG_TYPE_SHIFT,24); m!(SOF_IPC4_MSG_TYPE_MASK,genmask(28,24));
macro_rules! SOF_IPC4_MSG_TYPE_SET { ($x:expr) => { (($x) << 24) & genmask(28,24) }; } macro_rules! SOF_IPC4_MSG_TYPE_GET { ($x:expr) => { (($x)&genmask(28,24)) >> 24 }; }

/* C preprocessor field operations, retained as Rust macros for expression-level compatibility. */
m!(SOF_IPC4_GLB_PIPE_INSTANCE_SHIFT,16); m!(SOF_IPC4_GLB_PIPE_INSTANCE_MASK,genmask(23,16)); macro_rules! SOF_IPC4_GLB_PIPE_INSTANCE_ID {($x:expr)=>{($x)<<16};}
m!(SOF_IPC4_GLB_PIPE_PRIORITY_SHIFT,11); m!(SOF_IPC4_GLB_PIPE_PRIORITY_MASK,genmask(15,11)); macro_rules! SOF_IPC4_GLB_PIPE_PRIORITY {($x:expr)=>{($x)<<11};}
m!(SOF_IPC4_GLB_PIPE_MEM_SIZE_SHIFT,0); m!(SOF_IPC4_GLB_PIPE_MEM_SIZE_MASK,genmask(10,0)); macro_rules! SOF_IPC4_GLB_PIPE_MEM_SIZE {($x:expr)=>{$x};}
m!(SOF_IPC4_GLB_PIPE_EXT_LP_SHIFT,0); m!(SOF_IPC4_GLB_PIPE_EXT_LP_MASK,bit(0)); macro_rules! SOF_IPC4_GLB_PIPE_EXT_LP {($x:expr)=>{$x};}
m!(SOF_IPC4_GLB_PIPE_EXT_CORE_ID_SHIFT,20); m!(SOF_IPC4_GLB_PIPE_EXT_CORE_ID_MASK,genmask(23,20)); macro_rules! SOF_IPC4_GLB_PIPE_EXT_CORE_ID {($x:expr)=>{($x)<<20};}
m!(SOF_IPC4_GLB_PIPE_PAYLOAD_SHIFT,29); m!(SOF_IPC4_GLB_PIPE_PAYLOAD_MASK,bit(29)); macro_rules! SOF_IPC4_GLB_PIPE_PAYLOAD {($x:expr)=>{($x)<<29};}
m!(SOF_IPC4_GLB_PIPE_STATE_ID_SHIFT,16); m!(SOF_IPC4_GLB_PIPE_STATE_ID_MASK,genmask(23,16)); macro_rules! SOF_IPC4_GLB_PIPE_STATE_ID {($x:expr)=>{($x)<<16};}
m!(SOF_IPC4_GLB_PIPE_STATE_SHIFT,0); m!(SOF_IPC4_GLB_PIPE_STATE_MASK,genmask(15,0)); macro_rules! SOF_IPC4_GLB_PIPE_STATE {($x:expr)=>{$x};} m!(SOF_IPC4_GLB_PIPE_STATE_EXT_MULTI,bit(0));
m!(SOF_IPC4_GLB_LOAD_LIBRARY_LIB_ID_SHIFT,16); macro_rules! SOF_IPC4_GLB_LOAD_LIBRARY_LIB_ID {($x:expr)=>{($x)<<16};}
m!(SOF_IPC4_GLB_CHAIN_DMA_HOST_ID_SHIFT,0); m!(SOF_IPC4_GLB_CHAIN_DMA_HOST_ID_MASK,genmask(4,0)); macro_rules! SOF_IPC4_GLB_CHAIN_DMA_HOST_ID {($x:expr)=>{($x)&genmask(4,0)};}
m!(SOF_IPC4_GLB_CHAIN_DMA_LINK_ID_SHIFT,8); m!(SOF_IPC4_GLB_CHAIN_DMA_LINK_ID_MASK,genmask(12,8)); macro_rules! SOF_IPC4_GLB_CHAIN_DMA_LINK_ID {($x:expr)=>{(($x)<<8)&genmask(12,8)};}
m!(SOF_IPC4_GLB_CHAIN_DMA_ALLOCATE_SHIFT,16); m!(SOF_IPC4_GLB_CHAIN_DMA_ALLOCATE_MASK,bit(16)); macro_rules! SOF_IPC4_GLB_CHAIN_DMA_ALLOCATE {($x:expr)=>{(($x)&1)<<16};}
m!(SOF_IPC4_GLB_CHAIN_DMA_ENABLE_SHIFT,17); m!(SOF_IPC4_GLB_CHAIN_DMA_ENABLE_MASK,bit(17)); macro_rules! SOF_IPC4_GLB_CHAIN_DMA_ENABLE {($x:expr)=>{(($x)&1)<<17};}
m!(SOF_IPC4_GLB_CHAIN_DMA_SCS_SHIFT,18); m!(SOF_IPC4_GLB_CHAIN_DMA_SCS_MASK,bit(18)); macro_rules! SOF_IPC4_GLB_CHAIN_DMA_SCS {($x:expr)=>{(($x)&1)<<18};}
m!(SOF_IPC4_GLB_EXT_CHAIN_DMA_FIFO_SIZE_SHIFT,0); m!(SOF_IPC4_GLB_EXT_CHAIN_DMA_FIFO_SIZE_MASK,genmask(24,0)); macro_rules! SOF_IPC4_GLB_EXT_CHAIN_DMA_FIFO_SIZE {($x:expr)=>{($x)&genmask(24,0)};}

#[repr(i32)] pub enum sof_ipc4_channel_config { SOF_IPC4_CHANNEL_CONFIG_MONO, SOF_IPC4_CHANNEL_CONFIG_STEREO, SOF_IPC4_CHANNEL_CONFIG_2_POINT_1, SOF_IPC4_CHANNEL_CONFIG_3_POINT_0, SOF_IPC4_CHANNEL_CONFIG_3_POINT_1, SOF_IPC4_CHANNEL_CONFIG_QUATRO, SOF_IPC4_CHANNEL_CONFIG_4_POINT_0, SOF_IPC4_CHANNEL_CONFIG_5_POINT_0, SOF_IPC4_CHANNEL_CONFIG_5_POINT_1, SOF_IPC4_CHANNEL_CONFIG_DUAL_MONO, SOF_IPC4_CHANNEL_CONFIG_I2S_DUAL_STEREO_0, SOF_IPC4_CHANNEL_CONFIG_I2S_DUAL_STEREO_1, SOF_IPC4_CHANNEL_CONFIG_7_POINT_1 }
#[repr(i32)] pub enum sof_ipc4_interleaved_style { SOF_IPC4_CHANNELS_INTERLEAVED, SOF_IPC4_CHANNELS_NONINTERLEAVED }
#[repr(i32)] pub enum sof_ipc4_sample_type { SOF_IPC4_MSB_INTEGER, SOF_IPC4_LSB_INTEGER }
#[repr(C, packed, align(4))] pub struct sof_ipc4_audio_format { pub sampling_frequency:u32,pub bit_depth:u32,pub ch_map:u32,pub ch_cfg:u32,pub interleaving_style:u32,pub fmt_cfg:u32 }
m!(SOF_IPC4_AUDIO_FORMAT_CFG_CHANNELS_COUNT_SHIFT,0);m!(SOF_IPC4_AUDIO_FORMAT_CFG_CHANNELS_COUNT_MASK,genmask(7,0));macro_rules! SOF_IPC4_AUDIO_FORMAT_CFG_CHANNELS_COUNT {($x:expr)=>{($x)&genmask(7,0)};}
m!(SOF_IPC4_AUDIO_FORMAT_CFG_V_BIT_DEPTH_SHIFT,8);m!(SOF_IPC4_AUDIO_FORMAT_CFG_V_BIT_DEPTH_MASK,genmask(15,8));macro_rules! SOF_IPC4_AUDIO_FORMAT_CFG_V_BIT_DEPTH {($x:expr)=>{(($x)&genmask(15,8))>>8};}
m!(SOF_IPC4_AUDIO_FORMAT_CFG_SAMPLE_TYPE_SHIFT,16);m!(SOF_IPC4_AUDIO_FORMAT_CFG_SAMPLE_TYPE_MASK,genmask(23,16));macro_rules! SOF_IPC4_AUDIO_FORMAT_CFG_SAMPLE_TYPE {($x:expr)=>{(($x)&genmask(23,16))>>16};}

#[repr(i32)] pub enum sof_ipc4_module_type { SOF_IPC4_MOD_INIT_INSTANCE,SOF_IPC4_MOD_CONFIG_GET,SOF_IPC4_MOD_CONFIG_SET,SOF_IPC4_MOD_LARGE_CONFIG_GET,SOF_IPC4_MOD_LARGE_CONFIG_SET,SOF_IPC4_MOD_BIND,SOF_IPC4_MOD_UNBIND,SOF_IPC4_MOD_SET_DX,SOF_IPC4_MOD_SET_D0IX,SOF_IPC4_MOD_ENTER_MODULE_RESTORE,SOF_IPC4_MOD_EXIT_MODULE_RESTORE,SOF_IPC4_MOD_DELETE_INSTANCE,SOF_IPC4_MOD_TYPE_LAST }
#[repr(C,packed,align(4))] pub struct sof_ipc4_base_module_cfg {pub cpc:u32,pub ibs:u32,pub obs:u32,pub is_pages:u32,pub audio_fmt:sof_ipc4_audio_format}
m!(SOF_IPC4_MOD_INSTANCE_SHIFT,16);m!(SOF_IPC4_MOD_INSTANCE_MASK,genmask(23,16));macro_rules! SOF_IPC4_MOD_INSTANCE {($x:expr)=>{($x)<<16};}macro_rules! SOF_IPC4_MOD_INSTANCE_GET {($x:expr)=>{(($x)&genmask(23,16))>>16};}
m!(SOF_IPC4_MOD_ID_SHIFT,0);m!(SOF_IPC4_MOD_ID_MASK,genmask(15,0));macro_rules! SOF_IPC4_MOD_ID {($x:expr)=>{$x};}macro_rules! SOF_IPC4_MOD_ID_GET {($x:expr)=>{($x)&genmask(15,0)};}
m!(SOF_IPC4_MOD_EXT_PARAM_SIZE_SHIFT,0);m!(SOF_IPC4_MOD_EXT_PARAM_SIZE_MASK,genmask(15,0));macro_rules! SOF_IPC4_MOD_EXT_PARAM_SIZE {($x:expr)=>{$x};}m!(SOF_IPC4_MOD_EXT_PPL_ID_SHIFT,16);m!(SOF_IPC4_MOD_EXT_PPL_ID_MASK,genmask(23,16));macro_rules! SOF_IPC4_MOD_EXT_PPL_ID {($x:expr)=>{($x)<<16};}m!(SOF_IPC4_MOD_EXT_CORE_ID_SHIFT,24);m!(SOF_IPC4_MOD_EXT_CORE_ID_MASK,genmask(27,24));macro_rules! SOF_IPC4_MOD_EXT_CORE_ID {($x:expr)=>{($x)<<24};}m!(SOF_IPC4_MOD_EXT_DOMAIN_SHIFT,28);m!(SOF_IPC4_MOD_EXT_DOMAIN_MASK,bit(28));macro_rules! SOF_IPC4_MOD_EXT_DOMAIN {($x:expr)=>{($x)<<28};}m!(SOF_IPC4_MOD_EXT_EXTENDED_INIT_SHIFT,29);m!(SOF_IPC4_MOD_EXT_EXTENDED_INIT_MASK,bit(29));macro_rules! SOF_IPC4_MOD_EXT_EXTENDED_INIT {($x:expr)=>{($x)<<29};}
m!(SOF_IPC4_MOD_EXT_DST_MOD_ID_SHIFT,0);m!(SOF_IPC4_MOD_EXT_DST_MOD_ID_MASK,genmask(15,0));macro_rules! SOF_IPC4_MOD_EXT_DST_MOD_ID {($x:expr)=>{$x};}m!(SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE_SHIFT,16);m!(SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE_MASK,genmask(23,16));macro_rules! SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE {($x:expr)=>{($x)<<16};}m!(SOF_IPC4_MOD_EXT_DST_MOD_QUEUE_ID_SHIFT,24);m!(SOF_IPC4_MOD_EXT_DST_MOD_QUEUE_ID_MASK,genmask(26,24));macro_rules! SOF_IPC4_MOD_EXT_DST_MOD_QUEUE_ID {($x:expr)=>{($x)<<24};}m!(SOF_IPC4_MOD_EXT_SRC_MOD_QUEUE_ID_SHIFT,27);m!(SOF_IPC4_MOD_EXT_SRC_MOD_QUEUE_ID_MASK,genmask(29,27));macro_rules! SOF_IPC4_MOD_EXT_SRC_MOD_QUEUE_ID {($x:expr)=>{($x)<<27};}
m!(MOD_ENABLE_LOG,6);m!(MOD_SYSTEM_TIME,20);m!(SOF_IPC4_MOD_EXT_MSG_SIZE_SHIFT,0);m!(SOF_IPC4_MOD_EXT_MSG_SIZE_MASK,genmask(19,0));macro_rules! SOF_IPC4_MOD_EXT_MSG_SIZE {($x:expr)=>{$x};}m!(SOF_IPC4_MOD_EXT_MSG_PARAM_ID_SHIFT,20);m!(SOF_IPC4_MOD_EXT_MSG_PARAM_ID_MASK,genmask(27,20));macro_rules! SOF_IPC4_MOD_EXT_MSG_PARAM_ID {($x:expr)=>{($x)<<20};}m!(SOF_IPC4_MOD_EXT_MSG_LAST_BLOCK_SHIFT,28);m!(SOF_IPC4_MOD_EXT_MSG_LAST_BLOCK_MASK,bit(28));macro_rules! SOF_IPC4_MOD_EXT_MSG_LAST_BLOCK {($x:expr)=>{($x)<<28};}m!(SOF_IPC4_MOD_EXT_MSG_FIRST_BLOCK_SHIFT,29);m!(SOF_IPC4_MOD_EXT_MSG_FIRST_BLOCK_MASK,bit(29));macro_rules! SOF_IPC4_MOD_EXT_MSG_FIRST_BLOCK {($x:expr)=>{($x)<<29};}

m!(SOF_IPC4_MOD_INIT_BASEFW_MOD_ID,0);m!(SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID,0);
#[repr(i32)] pub enum sof_ipc4_base_fw_params { SOF_IPC4_FW_PARAM_ENABLE_LOGS=6,SOF_IPC4_FW_PARAM_FW_CONFIG,SOF_IPC4_FW_PARAM_HW_CONFIG_GET,SOF_IPC4_FW_PARAM_MODULES_INFO_GET,SOF_IPC4_FW_PARAM_LIBRARIES_INFO_GET=16,SOF_IPC4_FW_PARAM_SYSTEM_TIME=20,SOF_IPC4_FW_PARAM_MIC_PRIVACY_STATE_CHANGE=35 }
#[repr(i32)] pub enum sof_ipc4_fw_config_params { SOF_IPC4_FW_CFG_FW_VERSION,SOF_IPC4_FW_CFG_MEMORY_RECLAIMED,SOF_IPC4_FW_CFG_SLOW_CLOCK_FREQ_HZ,SOF_IPC4_FW_CFG_FAST_CLOCK_FREQ_HZ,SOF_IPC4_FW_CFG_DMA_BUFFER_CONFIG,SOF_IPC4_FW_CFG_ALH_SUPPORT_LEVEL,SOF_IPC4_FW_CFG_DL_MAILBOX_BYTES,SOF_IPC4_FW_CFG_UL_MAILBOX_BYTES,SOF_IPC4_FW_CFG_TRACE_LOG_BYTES,SOF_IPC4_FW_CFG_MAX_PPL_COUNT,SOF_IPC4_FW_CFG_MAX_ASTATE_COUNT,SOF_IPC4_FW_CFG_MAX_MODULE_PIN_COUNT,SOF_IPC4_FW_CFG_MODULES_COUNT,SOF_IPC4_FW_CFG_MAX_MOD_INST_COUNT,SOF_IPC4_FW_CFG_MAX_LL_TASKS_PER_PRI_COUNT,SOF_IPC4_FW_CFG_LL_PRI_COUNT,SOF_IPC4_FW_CFG_MAX_DP_TASKS_COUNT,SOF_IPC4_FW_CFG_MAX_LIBS_COUNT,SOF_IPC4_FW_CFG_SCHEDULER_CONFIG,SOF_IPC4_FW_CFG_XTAL_FREQ_HZ,SOF_IPC4_FW_CFG_CLOCKS_CONFIG,SOF_IPC4_FW_CFG_RESERVED,SOF_IPC4_FW_CFG_POWER_GATING_POLICY,SOF_IPC4_FW_CFG_ASSERT_MODE,SOF_IPC4_FW_RESERVED1,SOF_IPC4_FW_RESERVED2,SOF_IPC4_FW_RESERVED3,SOF_IPC4_FW_RESERVED4,SOF_IPC4_FW_RESERVED5,SOF_IPC4_FW_CONTEXT_SAVE }
#[repr(C,packed)] pub struct sof_ipc4_fw_version {pub major:u16,pub minor:u16,pub hotfix:u16,pub build:u16}
#[repr(C,packed,align(4))] pub struct sof_ipc4_dx_state_info {pub core_mask:u32,pub dx_mask:u32}
#[repr(i32)] pub enum sof_ipc4_hw_config_params { SOF_IPC4_HW_CFG_INTEL_MIC_PRIVACY_CAPS=11 }
m!(SOF_IPC_INTEL_MIC_PRIVACY_VERSION_PTL,1);#[repr(C,packed)] pub struct sof_ipc4_intel_mic_privacy_cap {pub version:u32,pub capabilities_length:u32,pub capabilities:[u32;0]}
m!(SOF_IPC4_REPLY_STATUS,genmask(23,0));macro_rules! SOF_IPC4_MSG_IS_NOTIFICATION {($x:expr)=>{(($x)&genmask(28,24))>>24 == sof_ipc4_global_msg::SOF_IPC4_GLB_NOTIFICATION as u32};}
m!(SOF_IPC4_NOTIFICATION_TYPE_SHIFT,16);m!(SOF_IPC4_NOTIFICATION_TYPE_MASK,genmask(23,16));macro_rules! SOF_IPC4_NOTIFICATION_TYPE_GET {($x:expr)=>{(($x)&genmask(23,16))>>16};}m!(SOF_IPC4_LOG_CORE_SHIFT,12);m!(SOF_IPC4_LOG_CORE_MASK,genmask(15,12));macro_rules! SOF_IPC4_LOG_CORE_GET {($x:expr)=>{(($x)&genmask(15,12))>>12};}m!(SOF_IPC4_FW_READY_LIB_RESTORED,bit(15));
#[repr(i32)] pub enum sof_ipc4_notification_type { SOF_IPC4_NOTIFY_PHRASE_DETECTED=4,SOF_IPC4_NOTIFY_RESOURCE_EVENT,SOF_IPC4_NOTIFY_LOG_BUFFER_STATUS,SOF_IPC4_NOTIFY_TIMESTAMP_CAPTURED,SOF_IPC4_NOTIFY_FW_READY,SOF_IPC4_NOTIFY_FW_AUD_CLASS_RESULT,SOF_IPC4_NOTIFY_EXCEPTION_CAUGHT,SOF_IPC4_NOTIFY_MODULE_NOTIFICATION=12,SOF_IPC4_NOTIFY_PROBE_DATA_AVAILABLE=14,SOF_IPC4_NOTIFY_ASYNC_MSG_SRVC_MESSAGE,SOF_IPC4_NOTIFY_TYPE_LAST }
#[repr(i32)] pub enum sof_ipc4_resource_type { SOF_IPC4_MODULE_INSTANCE,SOF_IPC4_PIPELINE,SOF_IPC4_GATEWAY,SOF_IPC4_EDF_TASK,SOF_IPC4_INVALID_RESOURCE_TYPE }
#[repr(i32)] pub enum sof_ipc4_event_type { SOF_IPC4_MIXER_UNDERRUN_DETECTED=1,SOF_IPC4_PROCESS_DATA_ERROR=3,SOF_IPC4_GATEWAY_UNDERRUN_DETECTED=6,SOF_IPC4_GATEWAY_OVERRUN_DETECTED }
#[repr(C)] pub struct sof_ipc4_process_data_error_event_data {pub error_code:u32} #[repr(C)] pub struct sof_ipc4_mixer_underrun_event_data {pub eos_flag:u32,pub data_mixed:u32,pub expected_data_mixed:u32}
#[repr(C)] pub union sof_ipc4_resource_event_data {pub dws:[u32;6],pub process_data_error:sof_ipc4_process_data_error_event_data,pub mixer_underrun:sof_ipc4_mixer_underrun_event_data}
#[repr(C,packed,align(4))] pub struct sof_ipc4_notify_resource_data {pub resource_type:u32,pub resource_id:u32,pub event_type:u32,pub reserved:u32,pub data:sof_ipc4_resource_event_data}
m!(SOF_IPC4_DEBUG_DESCRIPTOR_SIZE,12);m!(SOF_IPC4_MAX_DEBUG_SLOTS,15);m!(SOF_IPC4_DEBUG_SLOT_SIZE,0x1000);m!(SOF_IPC4_DEBUG_SLOT_UNUSED,0);m!(SOF_IPC4_DEBUG_SLOT_CRITICAL_LOG,0x54524300);m!(SOF_IPC4_DEBUG_SLOT_DEBUG_LOG,0x474f4c00);m!(SOF_IPC4_DEBUG_SLOT_GDB_STUB,0x42444700);m!(SOF_IPC4_DEBUG_SLOT_TELEMETRY,0x4c455400);m!(SOF_IPC4_DEBUG_SLOT_BROKEN,0x44414544);
#[repr(C,packed,align(4))] pub struct sof_ipc4_notify_module_data {pub instance_id:u16,pub module_id:u16,pub event_id:u32,pub event_data_size:u32,pub event_data:[u8;0]}
m!(SOF_IPC4_NOTIFY_MODULE_EVENTID_ALSA_MAGIC_MASK,genmask(31,16));m!(SOF_IPC4_NOTIFY_MODULE_EVENTID_ALSA_MAGIC_VAL,0xA15A0000);m!(SOF_IPC4_NOTIFY_MODULE_EVENTID_ALSA_PARAMID_MASK,genmask(15,0));
m!(SOF_IPC4_MOD_INIT_EXT_RTOS_DOMAIN_SHIFT,0);m!(SOF_IPC4_MOD_INIT_EXT_RTOS_DOMAIN_MASK,bit(0));macro_rules! SOF_IPC4_MOD_INIT_EXT_RTOS_DOMAIN {($x:expr)=>{$x};}m!(SOF_IPC4_MOD_INIT_EXT_GNA_USED_SHIFT,1);m!(SOF_IPC4_MOD_INIT_EXT_GNA_USED_MASK,bit(1));macro_rules! SOF_IPC4_MOD_INIT_EXT_GNA_USED {($x:expr)=>{($x)<<1};}m!(SOF_IPC4_MOD_INIT_EXT_OBJ_ARRAY_SHIFT,2);m!(SOF_IPC4_MOD_INIT_EXT_OBJ_ARRAY_MASK,bit(2));macro_rules! SOF_IPC4_MOD_INIT_EXT_DATA_ARRAY {($x:expr)=>{($x)<<2};}
#[repr(C,packed,align(4))] pub struct sof_ipc4_module_init_ext_init {pub word0:u32,pub rsvd1:u32,pub rsvd2:u32} #[repr(C,packed,align(4))] pub struct sof_ipc4_module_init_ext_object {pub header:u32,pub data:[u32;0]}
#[repr(i32)] pub enum sof_ipc4_mod_init_ext_obj_id { SOF_IPC4_MOD_INIT_DATA_ID_INVALID=0,SOF_IPC4_MOD_INIT_DATA_ID_DP_DATA,SOF_IPC4_MOD_INIT_DATA_ID_MAX=SOF_IPC4_MOD_INIT_DATA_ID_DP_DATA }
#[repr(C,packed,align(4))] pub struct sof_ipc4_mod_init_ext_dp_memory_data {pub domain_id:u32,pub stack_bytes:u32,pub heap_bytes:u32}
m!(SOF_IPC4_GLB_PIPE_PAYLOAD_WORDS_SHIFT,0);m!(SOF_IPC4_GLB_PIPE_PAYLOAD_WORDS_MASK,genmask(23,0));macro_rules! SOF_IPC4_GLB_PIPE_PAYLOAD_WORDS {($x:expr)=>{$x};}m!(SOF_IPC4_GLB_PIPE_EXT_OBJ_ARRAY_SHIFT,24);m!(SOF_IPC4_GLB_PIPE_EXT_OBJ_ARRAY_MASK,bit(24));macro_rules! SOF_IPC4_GLB_PIPE_EXT_OBJ_ARRAY {($x:expr)=>{($x)<<24};}
#[repr(C,packed,align(4))] pub struct sof_ipc4_glb_pipe_payload {pub word0:u32,pub rsvd1:u32,pub rsvd2:u32} #[repr(C,packed,align(4))] pub struct sof_ipc4_glb_pipe_ext_object {pub header:u32,pub data:[u32;0]}
#[repr(i32)] pub enum sof_ipc4_glb_pipe_ext_obj_id { SOF_IPC4_GLB_PIPE_DATA_ID_INVALID=0,SOF_IPC4_GLB_PIPE_DATA_ID_MEM_DATA,SOF_IPC4_GLB_PIPE_DATA_ID_MAX=SOF_IPC4_GLB_PIPE_DATA_ID_MEM_DATA }
#[repr(C,packed,align(4))] pub struct sof_ipc4_glb_pipe_ext_obj_memory_data {pub domain_id:u32,pub stack_bytes:u32,pub heap_bytes:u32}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
