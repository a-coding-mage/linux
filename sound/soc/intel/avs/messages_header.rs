/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2021-2022 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

use core::ffi::{c_int, c_void};

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type s32 = ::core::primitive::i32;
pub type size_t = usize;

#[repr(C)]
pub struct avs_dev {
    _private: [u8; 0],
}

/* Supplied by Linux guid_t/GUID_INIT in the original include environment. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guid_t {
    pub b: [u8; 16],
}

pub const fn GUID_INIT(a: u32, b: u16, c: u16, d0: u8, d1: u8, d2: u8, d3: u8, d4: u8, d5: u8, d6: u8, d7: u8) -> guid_t {
    guid_t {
        b: [
            (a & 0xff) as u8,
            ((a >> 8) & 0xff) as u8,
            ((a >> 16) & 0xff) as u8,
            ((a >> 24) & 0xff) as u8,
            (b & 0xff) as u8,
            ((b >> 8) & 0xff) as u8,
            (c & 0xff) as u8,
            ((c >> 8) & 0xff) as u8,
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
        ],
    }
}

pub const AVS_MAILBOX_SIZE: usize = 4 * 1024;

pub const AVS_FW_GEN_MSG: u32 = 0;
pub const AVS_MOD_MSG: u32 = 1;

pub const AVS_MSG_REQUEST: u32 = 0;
pub const AVS_MSG_REPLY: u32 = 1;

pub const AVS_GLB_ROM_CONTROL: u32 = 1;
pub const AVS_GLB_LOAD_MULTIPLE_MODULES: u32 = 15;
pub const AVS_GLB_UNLOAD_MULTIPLE_MODULES: u32 = 16;
pub const AVS_GLB_CREATE_PIPELINE: u32 = 17;
pub const AVS_GLB_DELETE_PIPELINE: u32 = 18;
pub const AVS_GLB_SET_PIPELINE_STATE: u32 = 19;
pub const AVS_GLB_GET_PIPELINE_STATE: u32 = 20;
pub const AVS_GLB_LOAD_LIBRARY: u32 = 24;
pub const AVS_GLB_NOTIFICATION: u32 = 27;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_global_msg {
    pub val: u64,
    pub raw: avs_msg_u64_raw,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct avs_msg_u64_raw {
    pub primary: u32,
    pub ext: u32,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_global_msg>()];

#[repr(C, packed)]
pub struct avs_tlv {
    pub type_: u32,
    pub length: u32,
    pub value: [u32; 0],
}

const _: [(); 8] = [(); core::mem::size_of::<avs_tlv>()];

pub unsafe fn avs_tlv_size(tlv: *const avs_tlv) -> usize {
    core::mem::size_of::<avs_tlv>() + ((*tlv).length as usize / 4) * core::mem::size_of::<u32>()
}

pub const AVS_MOD_INIT_INSTANCE: u32 = 0;
pub const AVS_MOD_LARGE_CONFIG_GET: u32 = 3;
pub const AVS_MOD_LARGE_CONFIG_SET: u32 = 4;
pub const AVS_MOD_BIND: u32 = 5;
pub const AVS_MOD_UNBIND: u32 = 6;
pub const AVS_MOD_SET_DX: u32 = 7;
pub const AVS_MOD_SET_D0IX: u32 = 8;
pub const AVS_MOD_DELETE_INSTANCE: u32 = 11;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_module_msg {
    pub val: u64,
    pub raw: avs_msg_u64_raw,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_module_msg>()];

pub const AVS_IPC_NOT_SUPPORTED: u32 = 15;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_reply_msg {
    pub val: u64,
    pub raw: avs_msg_u64_raw,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_reply_msg>()];

pub const AVS_NOTIFY_PHRASE_DETECTED: u32 = 4;
pub const AVS_NOTIFY_RESOURCE_EVENT: u32 = 5;
pub const AVS_NOTIFY_LOG_BUFFER_STATUS: u32 = 6;
pub const AVS_NOTIFY_FW_READY: u32 = 8;
pub const AVS_NOTIFY_EXCEPTION_CAUGHT: u32 = 10;
pub const AVS_NOTIFY_MODULE_EVENT: u32 = 12;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_notify_msg {
    pub val: u64,
    pub raw: avs_msg_u64_raw,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_notify_msg>()];

pub const fn AVS_MSG(hdr: u64) -> avs_reply_msg {
    avs_reply_msg { val: hdr }
}

pub const fn AVS_GLOBAL_REQUEST(msg_type: u32) -> avs_global_msg {
    avs_global_msg {
        val: ((msg_type as u64) << 24) | ((AVS_MSG_REQUEST as u64) << 29) | ((AVS_FW_GEN_MSG as u64) << 30),
    }
}

pub const fn AVS_MODULE_REQUEST(msg_type: u32) -> avs_module_msg {
    avs_module_msg {
        val: ((msg_type as u64) << 24) | ((AVS_MSG_REQUEST as u64) << 29) | ((AVS_MOD_MSG as u64) << 30),
    }
}

pub const fn AVS_NOTIFICATION(msg_type: u32) -> avs_notify_msg {
    avs_notify_msg {
        val: ((msg_type as u64) << 16)
            | ((AVS_GLB_NOTIFICATION as u64) << 24)
            | ((AVS_MSG_REPLY as u64) << 29)
            | ((AVS_FW_GEN_MSG as u64) << 30),
    }
}

pub const fn avs_msg_is_reply(hdr: u64) -> bool {
    let primary = hdr as u32;
    ((primary >> 29) & 0x1) == AVS_MSG_REPLY && ((primary >> 24) & 0x1f) != AVS_GLB_NOTIFICATION
}

/* Notification types */

#[repr(C, packed)]
pub struct avs_notify_voice_data {
    pub kpd_score: u16,
    pub reserved: u16,
}

const _: [(); 4] = [(); core::mem::size_of::<avs_notify_voice_data>()];

#[repr(C, packed)]
pub struct avs_notify_res_data {
    pub resource_type: u32,
    pub resource_id: u32,
    pub event_type: u32,
    pub reserved: u32,
    pub data: [u32; 6],
}

const _: [(); 40] = [(); core::mem::size_of::<avs_notify_res_data>()];

#[repr(C, packed)]
pub struct avs_notify_mod_data {
    pub module_instance_id: u32,
    pub event_id: u32,
    pub data_size: u32,
    pub data: [u32; 0],
}

const _: [(); 12] = [(); core::mem::size_of::<avs_notify_mod_data>()];

/* ROM messages */
pub const AVS_ROM_SET_BOOT_CONFIG: u32 = 0;

unsafe extern "C" {
    pub fn avs_ipc_set_boot_config(adev: *mut avs_dev, dma_id: u32, purge: u32) -> c_int;
    pub fn avs_ipc_load_modules(adev: *mut avs_dev, mod_ids: *mut u16, num_mod_ids: u32) -> c_int;
    pub fn avs_ipc_unload_modules(adev: *mut avs_dev, mod_ids: *mut u16, num_mod_ids: u32) -> c_int;
    pub fn avs_ipc_load_library(adev: *mut avs_dev, dma_id: u32, lib_id: u32) -> c_int;
}

/* Pipeline management messages */
pub const AVS_PPL_STATE_INVALID: u32 = 0;
pub const AVS_PPL_STATE_UNINITIALIZED: u32 = 1;
pub const AVS_PPL_STATE_RESET: u32 = 2;
pub const AVS_PPL_STATE_PAUSED: u32 = 3;
pub const AVS_PPL_STATE_RUNNING: u32 = 4;
pub type avs_pipeline_state = u32;

unsafe extern "C" {
    pub fn avs_ipc_create_pipeline(adev: *mut avs_dev, req_size: u16, priority: u8, instance_id: u8, lp: bool, attributes: u16) -> c_int;
    pub fn avs_ipc_delete_pipeline(adev: *mut avs_dev, instance_id: u8) -> c_int;
    pub fn avs_ipc_set_pipeline_state(adev: *mut avs_dev, instance_id: u8, state: avs_pipeline_state) -> c_int;
    pub fn avs_ipc_get_pipeline_state(adev: *mut avs_dev, instance_id: u8, state: *mut avs_pipeline_state) -> c_int;
    pub fn avs_ipc_init_instance(adev: *mut avs_dev, module_id: u16, instance_id: u8, ppl_id: u8, core_id: u8, domain: u8, param: *mut c_void, param_size: u32) -> c_int;
    pub fn avs_ipc_delete_instance(adev: *mut avs_dev, module_id: u16, instance_id: u8) -> c_int;
    pub fn avs_ipc_bind(adev: *mut avs_dev, module_id: u16, instance_id: u8, dst_module_id: u16, dst_instance_id: u8, dst_queue: u8, src_queue: u8) -> c_int;
    pub fn avs_ipc_unbind(adev: *mut avs_dev, module_id: u16, instance_id: u8, dst_module_id: u16, dst_instance_id: u8, dst_queue: u8, src_queue: u8) -> c_int;
    pub fn avs_ipc_set_large_config(adev: *mut avs_dev, module_id: u16, instance_id: u8, param_id: u8, request: *mut u8, request_size: size_t) -> c_int;
    pub fn avs_ipc_get_large_config(adev: *mut avs_dev, module_id: u16, instance_id: u8, param_id: u8, request_data: *mut u8, request_size: size_t, reply_data: *mut *mut u8, reply_size: *mut size_t) -> c_int;
}

#[repr(C, packed)]
pub struct avs_dxstate_info {
    pub core_mask: u32,
    pub dx_mask: u32,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_dxstate_info>()];

unsafe extern "C" {
    pub fn avs_ipc_set_dx(adev: *mut avs_dev, core_mask: u32, powerup: bool) -> c_int;
    pub fn avs_ipc_set_d0ix(adev: *mut avs_dev, enable_pg: bool, streaming: bool) -> c_int;
}

pub const AVS_BASEFW_MOD_ID: u32 = 0;
pub const AVS_BASEFW_INST_ID: u32 = 0;

pub const AVS_BASEFW_ENABLE_LOGS: u32 = 6;
pub const AVS_BASEFW_FIRMWARE_CONFIG: u32 = 7;
pub const AVS_BASEFW_HARDWARE_CONFIG: u32 = 8;
pub const AVS_BASEFW_MODULES_INFO: u32 = 9;
pub const AVS_BASEFW_LIBRARIES_INFO: u32 = 16;
pub const AVS_BASEFW_SYSTEM_TIME: u32 = 20;

pub const AVS_LOG_DISABLE: u32 = 0;
pub const AVS_LOG_ENABLE: u32 = 1;

pub const AVS_SKL_LOG_CRITICAL: u32 = 1;
pub const AVS_SKL_LOG_HIGH: u32 = 2;
pub const AVS_SKL_LOG_MEDIUM: u32 = 3;
pub const AVS_SKL_LOG_LOW: u32 = 4;
pub const AVS_SKL_LOG_VERBOSE: u32 = 5;

#[repr(C, packed)]
pub struct avs_skl_log_state {
    pub enable: u32,
    pub min_priority: u32,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_skl_log_state>()];

#[repr(C, packed)]
pub struct avs_skl_log_state_info {
    pub core_mask: u32,
    pub logs_core: [avs_skl_log_state; 0],
}

const _: [(); 4] = [(); core::mem::size_of::<avs_skl_log_state_info>()];

#[repr(C, packed)]
pub struct avs_apl_log_state_info {
    pub aging_timer_period: u32,
    pub fifo_full_timer_period: u32,
    pub core_mask: u32,
    pub logs_core: [avs_skl_log_state; 0],
}

const _: [(); 12] = [(); core::mem::size_of::<avs_apl_log_state_info>()];

pub const AVS_ICL_LOG_CRITICAL: u32 = 0;
pub const AVS_ICL_LOG_HIGH: u32 = 1;
pub const AVS_ICL_LOG_MEDIUM: u32 = 2;
pub const AVS_ICL_LOG_LOW: u32 = 3;
pub const AVS_ICL_LOG_VERBOSE: u32 = 4;

pub const AVS_ICL_LOG_INFRA: u32 = 0;
pub const AVS_ICL_LOG_HAL: u32 = 1;
pub const AVS_ICL_LOG_MODULE: u32 = 2;
pub const AVS_ICL_LOG_AUDIO: u32 = 3;
pub const AVS_ICL_LOG_SENSING: u32 = 4;
pub const AVS_ICL_LOG_ULP_INFRA: u32 = 5;

#[repr(C, packed)]
pub struct avs_icl_log_state_info {
    pub aging_timer_period: u32,
    pub fifo_full_timer_period: u32,
    pub enable: u32,
    pub logs_priorities_mask: [u32; 0],
}

const _: [(); 12] = [(); core::mem::size_of::<avs_icl_log_state_info>()];

unsafe extern "C" {
    pub fn avs_ipc_set_enable_logs(adev: *mut avs_dev, log_info: *mut u8, size: size_t) -> c_int;
}

#[repr(C)]
pub struct avs_fw_version {
    pub major: u16,
    pub minor: u16,
    pub hotfix: u16,
    pub build: u16,
}

pub const AVS_FW_CFG_FW_VERSION: u32 = 0;
pub const AVS_FW_CFG_MEMORY_RECLAIMED: u32 = 1;
pub const AVS_FW_CFG_SLOW_CLOCK_FREQ_HZ: u32 = 2;
pub const AVS_FW_CFG_FAST_CLOCK_FREQ_HZ: u32 = 3;
pub const AVS_FW_CFG_DMA_BUFFER_CONFIG: u32 = 4;
pub const AVS_FW_CFG_ALH_SUPPORT_LEVEL: u32 = 5;
pub const AVS_FW_CFG_IPC_DL_MAILBOX_BYTES: u32 = 6;
pub const AVS_FW_CFG_IPC_UL_MAILBOX_BYTES: u32 = 7;
pub const AVS_FW_CFG_TRACE_LOG_BYTES: u32 = 8;
pub const AVS_FW_CFG_MAX_PPL_COUNT: u32 = 9;
pub const AVS_FW_CFG_MAX_ASTATE_COUNT: u32 = 10;
pub const AVS_FW_CFG_MAX_MODULE_PIN_COUNT: u32 = 11;
pub const AVS_FW_CFG_MODULES_COUNT: u32 = 12;
pub const AVS_FW_CFG_MAX_MOD_INST_COUNT: u32 = 13;
pub const AVS_FW_CFG_MAX_LL_TASKS_PER_PRI_COUNT: u32 = 14;
pub const AVS_FW_CFG_LL_PRI_COUNT: u32 = 15;
pub const AVS_FW_CFG_MAX_DP_TASKS_COUNT: u32 = 16;
pub const AVS_FW_CFG_MAX_LIBS_COUNT: u32 = 17;
pub const AVS_FW_CFG_SCHEDULER_CONFIG: u32 = 18;
pub const AVS_FW_CFG_XTAL_FREQ_HZ: u32 = 19;
pub const AVS_FW_CFG_CLOCKS_CONFIG: u32 = 20;
pub const AVS_FW_CFG_RESERVED: u32 = 21;
pub const AVS_FW_CFG_POWER_GATING_POLICY: u32 = 22;
pub const AVS_FW_CFG_ASSERT_MODE: u32 = 23;
pub const AVS_FW_CFG_RESERVED2: u32 = 24;
pub const AVS_FW_CFG_BUS_HARDWARE_ID: u32 = 25;

#[repr(C)]
pub struct avs_fw_cfg {
    pub fw_version: avs_fw_version,
    pub memory_reclaimed: u32,
    pub slow_clock_freq_hz: u32,
    pub fast_clock_freq_hz: u32,
    pub alh_support: u32,
    pub ipc_dl_mailbox_bytes: u32,
    pub ipc_ul_mailbox_bytes: u32,
    pub trace_log_bytes: u32,
    pub max_ppl_count: u32,
    pub max_astate_count: u32,
    pub max_module_pin_count: u32,
    pub modules_count: u32,
    pub max_mod_inst_count: u32,
    pub max_ll_tasks_per_pri_count: u32,
    pub ll_pri_count: u32,
    pub max_dp_tasks_count: u32,
    pub max_libs_count: u32,
    pub xtal_freq_hz: u32,
    pub power_gating_policy: u32,
}

#[repr(C)]
pub struct avs_bus_hwid {
    pub device: u32,
    pub subsystem: u32,
    pub revision: u8,
}

unsafe extern "C" {
    pub fn avs_ipc_get_fw_config(adev: *mut avs_dev, cfg: *mut avs_fw_cfg) -> c_int;
    pub fn avs_ipc_set_fw_config(adev: *mut avs_dev, num_tlvs: size_t, ...) -> c_int;
}

pub const AVS_HW_CFG_AVS_VER: u32 = 0;
pub const AVS_HW_CFG_DSP_CORES: u32 = 1;
pub const AVS_HW_CFG_MEM_PAGE_BYTES: u32 = 2;
pub const AVS_HW_CFG_TOTAL_PHYS_MEM_PAGES: u32 = 3;
pub const AVS_HW_CFG_I2S_CAPS: u32 = 4;
pub const AVS_HW_CFG_GPDMA_CAPS: u32 = 5;
pub const AVS_HW_CFG_GATEWAY_COUNT: u32 = 6;
pub const AVS_HW_CFG_HP_EBB_COUNT: u32 = 7;
pub const AVS_HW_CFG_LP_EBB_COUNT: u32 = 8;
pub const AVS_HW_CFG_EBB_SIZE_BYTES: u32 = 9;

pub const AVS_AVS_VER_1_5: u32 = 0x10005;
pub const AVS_AVS_VER_1_8: u32 = 0x10008;

pub const AVS_I2S_VER_15_SKYLAKE: u32 = 0x00000;
pub const AVS_I2S_VER_15_BROXTON: u32 = 0x10000;
pub const AVS_I2S_VER_15_BROXTON_P: u32 = 0x20000;
pub const AVS_I2S_VER_18_KBL_CNL: u32 = 0x30000;

#[repr(C)]
pub struct avs_i2s_caps {
    pub i2s_version: u32,
    pub ctrl_count: u32,
    pub ctrl_base_addr: *mut u32,
}

#[repr(C)]
pub struct avs_hw_cfg {
    pub avs_version: u32,
    pub dsp_cores: u32,
    pub mem_page_bytes: u32,
    pub total_phys_mem_pages: u32,
    pub i2s_caps: avs_i2s_caps,
    pub gateway_count: u32,
    pub hp_ebb_count: u32,
    pub lp_ebb_count: u32,
    pub ebb_size_bytes: u32,
}

unsafe extern "C" {
    pub fn avs_ipc_get_hw_config(adev: *mut avs_dev, cfg: *mut avs_hw_cfg) -> c_int;
}

pub const AVS_MODULE_LOAD_TYPE_BUILTIN: u32 = 0;
pub const AVS_MODULE_LOAD_TYPE_LOADABLE: u32 = 1;
pub const AVS_MODULE_STATE_LOADED: u16 = 1u16 << 0;

#[repr(C, packed)]
pub struct avs_module_type {
    pub bits: u32,
}

impl avs_module_type {
    pub const fn load_type(&self) -> u32 {
        self.bits & 0x0f
    }
}

const _: [(); 4] = [(); core::mem::size_of::<avs_module_type>()];

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_segment_flags {
    pub ul: u32,
}

const _: [(); 4] = [(); core::mem::size_of::<avs_segment_flags>()];

#[repr(C, packed)]
pub struct avs_segment_desc {
    pub flags: avs_segment_flags,
    pub v_base_addr: u32,
    pub file_offset: u32,
}

const _: [(); 12] = [(); core::mem::size_of::<avs_segment_desc>()];

#[repr(C, packed)]
pub struct avs_module_entry {
    pub module_id: u16,
    pub state_flags: u16,
    pub name: [u8; 8],
    pub uuid: guid_t,
    pub type_: avs_module_type,
    pub hash: [u8; 32],
    pub entry_point: u32,
    pub cfg_offset: u16,
    pub cfg_count: u16,
    pub affinity_mask: u32,
    pub instance_max_count: u16,
    pub instance_bss_size: u16,
    pub segments: [avs_segment_desc; 3],
}

const _: [(); 116] = [(); core::mem::size_of::<avs_module_entry>()];

#[repr(C, packed)]
pub struct avs_mods_info {
    pub count: u32,
    pub entries: [avs_module_entry; 0],
}

const _: [(); 4] = [(); core::mem::size_of::<avs_mods_info>()];

pub unsafe fn avs_module_entry_is_loaded(mentry: *mut avs_module_entry) -> bool {
    (*mentry).type_.load_type() == AVS_MODULE_LOAD_TYPE_BUILTIN
        || ((*mentry).state_flags & AVS_MODULE_STATE_LOADED) != 0
}

unsafe extern "C" {
    pub fn avs_ipc_get_modules_info(adev: *mut avs_dev, info: *mut *mut avs_mods_info) -> c_int;
}

#[repr(C, packed)]
pub struct avs_sys_time {
    pub val_l: u32,
    pub val_u: u32,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_sys_time>()];

unsafe extern "C" {
    pub fn avs_ipc_set_system_time(adev: *mut avs_dev) -> c_int;
}

pub const AVS_MIXIN_MOD_UUID: guid_t = GUID_INIT(0x39656EB2, 0x3B71, 0x4049, 0x8D, 0x3F, 0xF9, 0x2C, 0xD5, 0xC4, 0x3C, 0x09);
pub const AVS_MIXOUT_MOD_UUID: guid_t = GUID_INIT(0x3C56505A, 0x24D7, 0x418F, 0xBD, 0xDC, 0xC1, 0xF5, 0xA3, 0xAC, 0x2A, 0xE0);
pub const AVS_COPIER_MOD_UUID: guid_t = GUID_INIT(0x9BA00C83, 0xCA12, 0x4A83, 0x94, 0x3C, 0x1F, 0xA2, 0xE8, 0x2F, 0x9D, 0xDA);
pub const AVS_PEAKVOL_MOD_UUID: guid_t = GUID_INIT(0x8A171323, 0x94A3, 0x4E1D, 0xAF, 0xE9, 0xFE, 0x5D, 0xBA, 0xa4, 0xC3, 0x93);
pub const AVS_GAIN_MOD_UUID: guid_t = GUID_INIT(0x61BCA9A8, 0x18D0, 0x4A18, 0x8E, 0x7B, 0x26, 0x39, 0x21, 0x98, 0x04, 0xB7);
pub const AVS_KPBUFF_MOD_UUID: guid_t = GUID_INIT(0xA8A0CB32, 0x4A77, 0x4DB1, 0x85, 0xC7, 0x53, 0xD7, 0xEE, 0x07, 0xBC, 0xE6);
pub const AVS_MICSEL_MOD_UUID: guid_t = GUID_INIT(0x32FE92C1, 0x1E17, 0x4FC2, 0x97, 0x58, 0xC7, 0xF3, 0x54, 0x2E, 0x98, 0x0A);
pub const AVS_MUX_MOD_UUID: guid_t = GUID_INIT(0x64CE6E35, 0x857A, 0x4878, 0xAC, 0xE8, 0xE2, 0xA2, 0xF4, 0x2e, 0x30, 0x69);
pub const AVS_UPDWMIX_MOD_UUID: guid_t = GUID_INIT(0x42F8060C, 0x832F, 0x4DBF, 0xB2, 0x47, 0x51, 0xE9, 0x61, 0x99, 0x7b, 0x35);
pub const AVS_SRCINTC_MOD_UUID: guid_t = GUID_INIT(0xE61BB28D, 0x149A, 0x4C1F, 0xB7, 0x09, 0x46, 0x82, 0x3E, 0xF5, 0xF5, 0xAE);
pub const AVS_PROBE_MOD_UUID: guid_t = GUID_INIT(0x7CAD0808, 0xAB10, 0xCD23, 0xEF, 0x45, 0x12, 0xAB, 0x34, 0xCD, 0x56, 0xEF);
pub const AVS_AEC_MOD_UUID: guid_t = GUID_INIT(0x46CB87FB, 0xD2C9, 0x4970, 0x96, 0xD2, 0x6D, 0x7E, 0x61, 0x4B, 0xB6, 0x05);
pub const AVS_ASRC_MOD_UUID: guid_t = GUID_INIT(0x66B4402D, 0xB468, 0x42F2, 0x81, 0xA7, 0xB3, 0x71, 0x21, 0x86, 0x3D, 0xD4);
pub const AVS_INTELWOV_MOD_UUID: guid_t = GUID_INIT(0xEC774FA9, 0x28D3, 0x424A, 0x90, 0xE4, 0x69, 0xF9, 0x84, 0xF1, 0xEE, 0xB7);
pub const AVS_WOVHOSTM_MOD_UUID: guid_t = GUID_INIT(0xF9ED62B7, 0x092E, 0x4A90, 0x8F, 0x4D, 0x82, 0xDA, 0xA8, 0xB3, 0x8F, 0x3B);

pub const AVS_CHANNEL_LEFT: u32 = 0;
pub const AVS_CHANNEL_RIGHT: u32 = 1;
pub const AVS_CHANNEL_CENTER: u32 = 2;
pub const AVS_CHANNEL_LEFT_SURROUND: u32 = 3;
pub const AVS_CHANNEL_CENTER_SURROUND: u32 = 3;
pub const AVS_CHANNEL_RIGHT_SURROUND: u32 = 4;
pub const AVS_CHANNEL_LFE: u32 = 7;
pub const AVS_CHANNEL_INVALID: u32 = 0xF;

pub const AVS_CHANNEL_CONFIG_MONO: u32 = 0;
pub const AVS_CHANNEL_CONFIG_STEREO: u32 = 1;
pub const AVS_CHANNEL_CONFIG_2_1: u32 = 2;
pub const AVS_CHANNEL_CONFIG_3_0: u32 = 3;
pub const AVS_CHANNEL_CONFIG_3_1: u32 = 4;
pub const AVS_CHANNEL_CONFIG_QUATRO: u32 = 5;
pub const AVS_CHANNEL_CONFIG_4_0: u32 = 6;
pub const AVS_CHANNEL_CONFIG_5_0: u32 = 7;
pub const AVS_CHANNEL_CONFIG_5_1: u32 = 8;
pub const AVS_CHANNEL_CONFIG_DUAL_MONO: u32 = 9;
pub const AVS_CHANNEL_CONFIG_I2S_DUAL_STEREO_0: u32 = 10;
pub const AVS_CHANNEL_CONFIG_I2S_DUAL_STEREO_1: u32 = 11;
pub const AVS_CHANNEL_CONFIG_7_1: u32 = 12;
pub const AVS_CHANNEL_CONFIG_INVALID: u32 = 13;

pub const AVS_INTERLEAVING_PER_CHANNEL: u32 = 0;
pub const AVS_INTERLEAVING_PER_SAMPLE: u32 = 1;

pub const AVS_SAMPLE_TYPE_INT_MSB: u32 = 0;
pub const AVS_SAMPLE_TYPE_INT_LSB: u32 = 1;
pub const AVS_SAMPLE_TYPE_INT_SIGNED: u32 = 2;
pub const AVS_SAMPLE_TYPE_INT_UNSIGNED: u32 = 3;
pub const AVS_SAMPLE_TYPE_FLOAT: u32 = 4;

pub const AVS_COEFF_CHANNELS_MAX: usize = 8;
pub const AVS_ALL_CHANNELS_MASK: u32 = u32::MAX;
pub const AVS_CHANNELS_MAX: usize = 16;

#[repr(C, packed)]
pub struct avs_audio_format {
    pub sampling_freq: u32,
    pub bit_depth: u32,
    pub channel_map: u32,
    pub channel_config: u32,
    pub interleaving: u32,
    pub bitfields: u32,
}

const _: [(); 24] = [(); core::mem::size_of::<avs_audio_format>()];

#[repr(C, packed)]
pub struct avs_modcfg_base {
    pub cpc: u32,
    pub ibs: u32,
    pub obs: u32,
    pub is_pages: u32,
    pub audio_fmt: avs_audio_format,
}

const _: [(); 40] = [(); core::mem::size_of::<avs_modcfg_base>()];

#[repr(C, packed)]
pub struct avs_pin_format {
    pub pin_index: u32,
    pub iobs: u32,
    pub audio_fmt: avs_audio_format,
}

const _: [(); 32] = [(); core::mem::size_of::<avs_pin_format>()];

#[repr(C, packed)]
pub struct avs_modcfg_ext {
    pub base: avs_modcfg_base,
    pub num_input_pins: u16,
    pub num_output_pins: u16,
    pub reserved: [u8; 12],
    pub pin_fmts: [avs_pin_format; 0],
}

const _: [(); 56] = [(); core::mem::size_of::<avs_modcfg_ext>()];

pub const AVS_DMA_HDA_HOST_OUTPUT: u32 = 0;
pub const AVS_DMA_HDA_HOST_INPUT: u32 = 1;
pub const AVS_DMA_HDA_LINK_OUTPUT: u32 = 8;
pub const AVS_DMA_HDA_LINK_INPUT: u32 = 9;
pub const AVS_DMA_DMIC_LINK_INPUT: u32 = 11;
pub const AVS_DMA_I2S_LINK_OUTPUT: u32 = 12;
pub const AVS_DMA_I2S_LINK_INPUT: u32 = 13;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_virtual_index {
    pub val: u8,
}

const _: [(); 1] = [(); core::mem::size_of::<avs_virtual_index>()];

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_connector_node_id {
    pub val: u32,
}

const _: [(); 4] = [(); core::mem::size_of::<avs_connector_node_id>()];

pub const INVALID_PIPELINE_ID: u32 = 0xFF;
pub const INVALID_NODE_ID: avs_connector_node_id = avs_connector_node_id { val: u32::MAX };

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_gtw_attributes {
    pub val: u32,
}

const _: [(); 4] = [(); core::mem::size_of::<avs_gtw_attributes>()];

pub const AVS_GTW_DMA_CONFIG_ID: u32 = 0x1000;
pub const AVS_DMA_METHOD_HDA: u32 = 1;

#[repr(C, packed)]
pub struct avs_dma_device_stream_channel_map {
    pub device_address: u32,
    pub channel_map: u32,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_dma_device_stream_channel_map>()];

#[repr(C, packed)]
pub struct avs_dma_stream_channel_map {
    pub device_count: u32,
    pub map: [avs_dma_device_stream_channel_map; 16],
}

const _: [(); 132] = [(); core::mem::size_of::<avs_dma_stream_channel_map>()];

#[repr(C, packed)]
pub struct avs_dma_cfg {
    pub dma_method: u8,
    pub pre_allocated: u8,
    pub rsvd: u16,
    pub dma_channel_id: u32,
    pub stream_id: u32,
    pub map: avs_dma_stream_channel_map,
    pub config_size: u32,
    pub config: [u8; 0],
}

const _: [(); 148] = [(); core::mem::size_of::<avs_dma_cfg>()];

#[repr(C, packed)]
pub union avs_copier_gtw_cfg_config {
    pub attrs: avs_gtw_attributes,
    pub blob: [u32; 0],
}

#[repr(C, packed)]
pub struct avs_copier_gtw_cfg {
    pub node_id: avs_connector_node_id,
    pub dma_buffer_size: u32,
    pub config_length: u32,
    pub config: avs_copier_gtw_cfg_config,
}

const _: [(); 16] = [(); core::mem::size_of::<avs_copier_gtw_cfg>()];

#[repr(C, packed)]
pub struct avs_copier_cfg {
    pub base: avs_modcfg_base,
    pub out_fmt: avs_audio_format,
    pub feature_mask: u32,
    pub gtw_cfg: avs_copier_gtw_cfg,
}

const _: [(); 84] = [(); core::mem::size_of::<avs_copier_cfg>()];

#[repr(C, packed)]
pub struct avs_volume_cfg {
    pub channel_id: u32,
    pub target_volume: u32,
    pub curve_type: u32,
    pub reserved: u32,
    pub curve_duration: u64,
}

const _: [(); 24] = [(); core::mem::size_of::<avs_volume_cfg>()];

#[repr(C, packed)]
pub struct avs_mute_cfg {
    pub channel_id: u32,
    pub mute: u32,
    pub curve_type: u32,
    pub reserved: u32,
    pub curve_duration: u64,
}

const _: [(); 24] = [(); core::mem::size_of::<avs_mute_cfg>()];

#[repr(C, packed)]
pub struct avs_peakvol_cfg {
    pub base: avs_modcfg_base,
    pub vols: [avs_volume_cfg; 0],
}

const _: [(); 40] = [(); core::mem::size_of::<avs_peakvol_cfg>()];

#[repr(C, packed)]
pub struct avs_micsel_cfg {
    pub base: avs_modcfg_base,
    pub out_fmt: avs_audio_format,
}

const _: [(); 64] = [(); core::mem::size_of::<avs_micsel_cfg>()];

#[repr(C, packed)]
pub struct avs_mux_cfg {
    pub base: avs_modcfg_base,
    pub ref_fmt: avs_audio_format,
    pub out_fmt: avs_audio_format,
}

const _: [(); 88] = [(); core::mem::size_of::<avs_mux_cfg>()];

#[repr(C, packed)]
pub struct avs_updown_mixer_cfg {
    pub base: avs_modcfg_base,
    pub out_channel_config: u32,
    pub coefficients_select: u32,
    pub coefficients: [s32; AVS_COEFF_CHANNELS_MAX],
    pub channel_map: u32,
}

const _: [(); 84] = [(); core::mem::size_of::<avs_updown_mixer_cfg>()];

#[repr(C, packed)]
pub struct avs_src_cfg {
    pub base: avs_modcfg_base,
    pub out_freq: u32,
}

const _: [(); 44] = [(); core::mem::size_of::<avs_src_cfg>()];

#[repr(C, packed)]
pub struct avs_probe_gtw_cfg {
    pub node_id: avs_connector_node_id,
    pub dma_buffer_size: u32,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_probe_gtw_cfg>()];

#[repr(C, packed)]
pub struct avs_probe_cfg {
    pub base: avs_modcfg_base,
    pub gtw_cfg: avs_probe_gtw_cfg,
}

const _: [(); 48] = [(); core::mem::size_of::<avs_probe_cfg>()];

#[repr(C, packed)]
pub struct avs_aec_cfg {
    pub base: avs_modcfg_base,
    pub ref_fmt: avs_audio_format,
    pub out_fmt: avs_audio_format,
    pub cpc_lp_mode: u32,
}

const _: [(); 92] = [(); core::mem::size_of::<avs_aec_cfg>()];

#[repr(C, packed)]
pub struct avs_asrc_cfg {
    pub base: avs_modcfg_base,
    pub out_freq: u32,
    pub bitfields: u32,
}

const _: [(); 48] = [(); core::mem::size_of::<avs_asrc_cfg>()];

#[repr(C, packed)]
pub struct avs_wov_cfg {
    pub base: avs_modcfg_base,
    pub cpc_lp_mode: u32,
}

const _: [(); 44] = [(); core::mem::size_of::<avs_wov_cfg>()];

#[repr(C, packed)]
pub struct avs_whm_cfg {
    pub base: avs_modcfg_base,
    /* Audio format for output pin 0 */
    pub ref_fmt: avs_audio_format,
    pub out_fmt: avs_audio_format,
    pub wake_tick_period: u32,
    pub gtw_cfg: avs_copier_gtw_cfg,
}

const _: [(); 108] = [(); core::mem::size_of::<avs_whm_cfg>()];

/* Module runtime parameters */

pub const AVS_VENDOR_CONFIG: u32 = 0xFF;

pub const AVS_COPIER_SET_SINK_FORMAT: u32 = 2;

#[repr(C, packed)]
pub struct avs_copier_sink_format {
    pub sink_id: u32,
    pub src_fmt: avs_audio_format,
    pub sink_fmt: avs_audio_format,
}

const _: [(); 52] = [(); core::mem::size_of::<avs_copier_sink_format>()];

unsafe extern "C" {
    pub fn avs_ipc_copier_set_sink_format(adev: *mut avs_dev, module_id: u16, instance_id: u8, sink_id: u32, src_fmt: *const avs_audio_format, sink_fmt: *const avs_audio_format) -> c_int;
}

pub const AVS_PEAKVOL_VOLUME: u32 = 0;
pub const AVS_PEAKVOL_MUTE: u32 = 3;

pub const AVS_AUDIO_CURVE_NONE: u32 = 0;
pub const AVS_AUDIO_CURVE_WINDOWS_FADE: u32 = 1;

unsafe extern "C" {
    pub fn avs_ipc_peakvol_get_volume(adev: *mut avs_dev, module_id: u16, instance_id: u8, vols: *mut *mut avs_volume_cfg, num_vols: *mut size_t) -> c_int;
    pub fn avs_ipc_peakvol_set_volume(adev: *mut avs_dev, module_id: u16, instance_id: u8, vol: *mut avs_volume_cfg) -> c_int;
    pub fn avs_ipc_peakvol_set_volumes(adev: *mut avs_dev, module_id: u16, instance_id: u8, vols: *mut avs_volume_cfg, num_vols: size_t) -> c_int;
    pub fn avs_ipc_peakvol_get_mute(adev: *mut avs_dev, module_id: u16, instance_id: u8, mutes: *mut *mut avs_mute_cfg, num_mutes: *mut size_t) -> c_int;
    pub fn avs_ipc_peakvol_set_mute(adev: *mut avs_dev, module_id: u16, instance_id: u8, mute: *mut avs_mute_cfg) -> c_int;
    pub fn avs_ipc_peakvol_set_mutes(adev: *mut avs_dev, module_id: u16, instance_id: u8, mutes: *mut avs_mute_cfg, num_mutes: size_t) -> c_int;
}

pub const AVS_PROBE_INST_ID: u32 = 0;

pub const AVS_PROBE_INJECTION_DMA: u32 = 1;
pub const AVS_PROBE_INJECTION_DMA_DETACH: u32 = 2;
pub const AVS_PROBE_POINTS: u32 = 3;
pub const AVS_PROBE_POINTS_DISCONNECT: u32 = 4;

#[repr(C, packed)]
pub struct avs_probe_dma {
    pub node_id: avs_connector_node_id,
    pub dma_buffer_size: u32,
}

const _: [(); 8] = [(); core::mem::size_of::<avs_probe_dma>()];

pub const AVS_PROBE_TYPE_INPUT: u32 = 0;
pub const AVS_PROBE_TYPE_OUTPUT: u32 = 1;
pub const AVS_PROBE_TYPE_INTERNAL: u32 = 2;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union avs_probe_point_id {
    pub value: u32,
}

const _: [(); 4] = [(); core::mem::size_of::<avs_probe_point_id>()];

pub const AVS_CONNECTION_PURPOSE_EXTRACT: u32 = 0;
pub const AVS_CONNECTION_PURPOSE_INJECT: u32 = 1;
pub const AVS_CONNECTION_PURPOSE_INJECT_REEXTRACT: u32 = 2;

#[repr(C, packed)]
pub struct avs_probe_point_desc {
    pub id: avs_probe_point_id,
    pub purpose: u32,
    pub node_id: avs_connector_node_id,
}

const _: [(); 12] = [(); core::mem::size_of::<avs_probe_point_desc>()];

unsafe extern "C" {
    pub fn avs_ipc_probe_get_dma(adev: *mut avs_dev, dmas: *mut *mut avs_probe_dma, num_dmas: *mut size_t) -> c_int;
    pub fn avs_ipc_probe_attach_dma(adev: *mut avs_dev, dmas: *mut avs_probe_dma, num_dmas: size_t) -> c_int;
    pub fn avs_ipc_probe_detach_dma(adev: *mut avs_dev, node_ids: *mut avs_connector_node_id, num_node_ids: size_t) -> c_int;
    pub fn avs_ipc_probe_get_points(adev: *mut avs_dev, descs: *mut *mut avs_probe_point_desc, num_descs: *mut size_t) -> c_int;
    pub fn avs_ipc_probe_connect_points(adev: *mut avs_dev, descs: *mut avs_probe_point_desc, num_descs: size_t) -> c_int;
    pub fn avs_ipc_probe_disconnect_points(adev: *mut avs_dev, ids: *mut avs_probe_point_id, num_ids: size_t) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
