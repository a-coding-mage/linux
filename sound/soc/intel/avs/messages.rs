// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const AVS_CL_TIMEOUT_MS: u32 = 5000;

extern "C" {
    fn AVS_GLOBAL_REQUEST(msg_type: u32) -> avs_global_msg;
    fn AVS_MODULE_REQUEST(msg_type: u32) -> avs_module_msg;

    static ROM_CONTROL: u32;
    static LOAD_MULTIPLE_MODULES: u32;
    static UNLOAD_MULTIPLE_MODULES: u32;
    static LOAD_LIBRARY: u32;
    static CREATE_PIPELINE: u32;
    static DELETE_PIPELINE: u32;
    static SET_PIPELINE_STATE: u32;
    static GET_PIPELINE_STATE: u32;
    static INIT_INSTANCE: u32;
    static DELETE_INSTANCE: u32;
    static BIND: u32;
    static UNBIND: u32;
    static LARGE_CONFIG_SET: u32;
    static LARGE_CONFIG_GET: u32;
    static SET_DX: u32;
    static SET_D0IX: u32;

    static AVS_ROM_SET_BOOT_CONFIG: u32;
    static AVS_MAILBOX_SIZE: usize;
    static GFP_KERNEL: u32;
    static ENOMEM: c_int;
    static EREMOTEIO: c_int;
    static ERANGE: c_int;
    static EINVAL: c_int;
    static AVS_BASEFW_MOD_ID: u16;
    static AVS_BASEFW_INST_ID: u8;
    static AVS_BASEFW_FIRMWARE_CONFIG: u8;
    static AVS_BASEFW_HARDWARE_CONFIG: u8;
    static AVS_BASEFW_MODULES_INFO: u8;
    static AVS_BASEFW_ENABLE_LOGS: u8;
    static AVS_BASEFW_SYSTEM_TIME: u8;
    static AVS_COPIER_SET_SINK_FORMAT: u8;
    static AVS_PEAKVOL_VOLUME: u8;
    static AVS_PEAKVOL_MUTE: u8;
    static AVS_VENDOR_CONFIG: u8;
    static AVS_PROBE_INST_ID: u8;
    static AVS_PROBE_INJECTION_DMA: u8;
    static AVS_PROBE_INJECTION_DMA_DETACH: u8;
    static AVS_PROBE_POINTS: u8;
    static AVS_PROBE_POINTS_DISCONNECT: u8;
    static AVS_PROBE_MOD_UUID: c_void;

    static AVS_FW_CFG_FW_VERSION: u32;
    static AVS_FW_CFG_MEMORY_RECLAIMED: u32;
    static AVS_FW_CFG_SLOW_CLOCK_FREQ_HZ: u32;
    static AVS_FW_CFG_FAST_CLOCK_FREQ_HZ: u32;
    static AVS_FW_CFG_ALH_SUPPORT_LEVEL: u32;
    static AVS_FW_CFG_IPC_DL_MAILBOX_BYTES: u32;
    static AVS_FW_CFG_IPC_UL_MAILBOX_BYTES: u32;
    static AVS_FW_CFG_TRACE_LOG_BYTES: u32;
    static AVS_FW_CFG_MAX_PPL_COUNT: u32;
    static AVS_FW_CFG_MAX_ASTATE_COUNT: u32;
    static AVS_FW_CFG_MAX_MODULE_PIN_COUNT: u32;
    static AVS_FW_CFG_MODULES_COUNT: u32;
    static AVS_FW_CFG_MAX_MOD_INST_COUNT: u32;
    static AVS_FW_CFG_MAX_LL_TASKS_PER_PRI_COUNT: u32;
    static AVS_FW_CFG_LL_PRI_COUNT: u32;
    static AVS_FW_CFG_MAX_DP_TASKS_COUNT: u32;
    static AVS_FW_CFG_MAX_LIBS_COUNT: u32;
    static AVS_FW_CFG_XTAL_FREQ_HZ: u32;
    static AVS_FW_CFG_POWER_GATING_POLICY: u32;
    static AVS_FW_CFG_DMA_BUFFER_CONFIG: u32;
    static AVS_FW_CFG_SCHEDULER_CONFIG: u32;
    static AVS_FW_CFG_CLOCKS_CONFIG: u32;
    static AVS_FW_CFG_RESERVED: u32;

    static AVS_HW_CFG_AVS_VER: u32;
    static AVS_HW_CFG_DSP_CORES: u32;
    static AVS_HW_CFG_MEM_PAGE_BYTES: u32;
    static AVS_HW_CFG_TOTAL_PHYS_MEM_PAGES: u32;
    static AVS_HW_CFG_I2S_CAPS: u32;
    static AVS_HW_CFG_GATEWAY_COUNT: u32;
    static AVS_HW_CFG_HP_EBB_COUNT: u32;
    static AVS_HW_CFG_LP_EBB_COUNT: u32;
    static AVS_HW_CFG_EBB_SIZE_BYTES: u32;
    static AVS_HW_CFG_GPDMA_CAPS: u32;

    fn avs_dsp_send_rom_msg(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        name: *const c_char,
    ) -> c_int;
    fn avs_dsp_send_msg(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        reply: *mut avs_ipc_msg,
        name: *const c_char,
    ) -> c_int;
    fn avs_dsp_send_msg_timeout(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        reply: *mut avs_ipc_msg,
        timeout_ms: u32,
        name: *const c_char,
    ) -> c_int;
    fn avs_dsp_send_pm_msg(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        reply: *mut avs_ipc_msg,
        wake_d0i0: bool,
        name: *const c_char,
    ) -> c_int;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn krealloc(ptr: *mut c_void, size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn devm_kmemdup(dev: *mut c_void, src: *const c_void, len: usize, gfp: u32) -> *mut c_void;
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn ktime_get() -> i64;
    fn ktime_to_us(kt: i64) -> u64;
    fn avs_get_module_id(adev: *mut avs_dev, uuid: *const c_void) -> u32;
}

#[repr(C)]
pub struct avs_dev {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct avs_ipc_msg {
    pub header: u64,
    pub data: *mut c_void,
    pub size: usize,
    pub rsp: avs_ipc_rsp,
}

#[repr(C)]
pub struct avs_ipc_rsp {
    pub ext: avs_ipc_rsp_ext,
}

#[repr(C)]
pub struct avs_ipc_rsp_ext {
    pub get_ppl_state: avs_ipc_rsp_get_ppl_state,
}

#[repr(C)]
pub struct avs_ipc_rsp_get_ppl_state {
    pub state: avs_pipeline_state,
}

#[repr(C)]
pub struct avs_global_msg {
    pub val: u64,
    pub boot_cfg: avs_boot_cfg_msg,
    pub load_multi_mods: avs_load_multi_mods_msg,
    pub load_lib: avs_load_lib_msg,
    pub create_ppl: avs_create_ppl_msg,
    pub ppl: avs_ppl_msg,
    pub set_ppl_state: avs_set_ppl_state_msg,
    pub get_ppl_state: avs_get_ppl_state_msg,
    pub ext: avs_global_msg_ext,
}

#[repr(C)]
pub struct avs_global_msg_ext {
    pub create_ppl: avs_create_ppl_ext_msg,
}

#[repr(C)]
pub struct avs_boot_cfg_msg {
    pub rom_ctrl_msg_type: u32,
    pub dma_id: u32,
    pub purge_request: u32,
}

#[repr(C)]
pub struct avs_load_multi_mods_msg {
    pub mod_cnt: u32,
}

#[repr(C)]
pub struct avs_load_lib_msg {
    pub dma_id: u32,
    pub lib_id: u32,
}

#[repr(C)]
pub struct avs_create_ppl_msg {
    pub ppl_mem_size: u16,
    pub ppl_priority: u8,
    pub instance_id: u8,
}

#[repr(C)]
pub struct avs_create_ppl_ext_msg {
    pub lp: bool,
    pub attributes: u16,
}

#[repr(C)]
pub struct avs_ppl_msg {
    pub instance_id: u8,
}

#[repr(C)]
pub struct avs_set_ppl_state_msg {
    pub ppl_id: u8,
    pub state: avs_pipeline_state,
}

#[repr(C)]
pub struct avs_get_ppl_state_msg {
    pub ppl_id: u8,
}

#[repr(C)]
pub struct avs_module_msg {
    pub val: u64,
    pub module_id: u16,
    pub instance_id: u8,
    pub ext: avs_module_msg_ext,
}

#[repr(C)]
pub struct avs_module_msg_ext {
    pub init_instance: avs_init_instance_msg,
    pub bind_unbind: avs_bind_unbind_msg,
    pub large_config: avs_large_config_msg,
    pub set_d0ix: avs_set_d0ix_msg,
}

#[repr(C)]
pub struct avs_init_instance_msg {
    pub param_block_size: u32,
    pub ppl_instance_id: u8,
    pub core_id: u8,
    pub proc_domain: u8,
}

#[repr(C)]
pub struct avs_bind_unbind_msg {
    pub dst_module_id: u16,
    pub dst_instance_id: u8,
    pub dst_queue: u8,
    pub src_queue: u8,
}

#[repr(C)]
pub struct avs_large_config_msg {
    pub data_off_size: usize,
    pub large_param_id: u8,
    pub final_block: bool,
    pub init_block: bool,
}

#[repr(C)]
pub struct avs_set_d0ix_msg {
    pub wake: bool,
    pub streaming: bool,
    pub prevent_pg: bool,
}

#[repr(C)]
pub struct avs_dxstate_info {
    pub core_mask: u32,
    pub dx_mask: u32,
}

#[repr(C)]
pub struct avs_tlv {
    pub type_: u32,
    pub length: u32,
    pub value: [u8; 0],
}

#[repr(C)]
pub struct avs_fw_cfg {
    pub fw_version: u32,
    pub memory_reclaimed: u8,
    pub slow_clock_freq_hz: u8,
    pub fast_clock_freq_hz: u8,
    pub alh_support: u8,
    pub ipc_dl_mailbox_bytes: u8,
    pub ipc_ul_mailbox_bytes: u8,
    pub trace_log_bytes: u8,
    pub max_ppl_count: u8,
    pub max_astate_count: u8,
    pub max_module_pin_count: u8,
    pub modules_count: u8,
    pub max_mod_inst_count: u8,
    pub max_ll_tasks_per_pri_count: u8,
    pub ll_pri_count: u8,
    pub max_dp_tasks_count: u8,
    pub max_libs_count: u8,
    pub xtal_freq_hz: u8,
    pub power_gating_policy: u8,
}

#[repr(C)]
pub struct avs_hw_cfg {
    pub avs_version: u8,
    pub dsp_cores: u8,
    pub mem_page_bytes: u8,
    pub total_phys_mem_pages: u8,
    pub i2s_caps: avs_i2s_caps,
    pub gateway_count: u8,
    pub hp_ebb_count: u8,
    pub lp_ebb_count: u8,
    pub ebb_size_bytes: u8,
}

#[repr(C)]
pub struct avs_i2s_caps {
    pub i2s_version: u8,
    pub ctrl_count: usize,
    pub ctrl_base_addr: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_audio_format {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_copier_sink_format {
    pub sink_id: u32,
    pub src_fmt: avs_audio_format,
    pub sink_fmt: avs_audio_format,
}

#[repr(C)]
pub struct avs_volume_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_mute_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_mods_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_sys_time {
    pub val_l: u32,
    pub val_u: u32,
}

#[repr(C)]
pub struct avs_probe_dma {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_probe_point_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub union avs_connector_node_id {
    _private: [u8; 0],
}

#[repr(C)]
pub union avs_probe_point_id {
    _private: [u8; 0],
}

pub type avs_pipeline_state = u32;

#[inline]
fn div_round_up(n: u32, d: usize) -> u32 {
    (n + d as u32 - 1) / d as u32
}

#[inline]
fn array_size(elem_size: usize, count: usize) -> usize {
    elem_size.wrapping_mul(count)
}

unsafe fn zeroed_ipc_msg() -> avs_ipc_msg {
    core::mem::zeroed()
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_set_boot_config(
    adev: *mut avs_dev,
    dma_id: u32,
    purge: u32,
) -> c_int {
    let mut msg = AVS_GLOBAL_REQUEST(ROM_CONTROL);
    let mut request = zeroed_ipc_msg();

    msg.boot_cfg.rom_ctrl_msg_type = AVS_ROM_SET_BOOT_CONFIG;
    msg.boot_cfg.dma_id = dma_id;
    msg.boot_cfg.purge_request = purge;
    request.header = msg.val;

    avs_dsp_send_rom_msg(adev, &mut request, c"set boot config".as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_load_modules(
    adev: *mut avs_dev,
    mod_ids: *mut u16,
    num_mod_ids: u32,
) -> c_int {
    let mut msg = AVS_GLOBAL_REQUEST(LOAD_MULTIPLE_MODULES);
    let mut request: avs_ipc_msg = core::mem::zeroed();

    msg.load_multi_mods.mod_cnt = num_mod_ids;
    request.header = msg.val;
    request.data = mod_ids as *mut c_void;
    request.size = size_of::<u16>() * num_mod_ids as usize;

    avs_dsp_send_msg_timeout(
        adev,
        &mut request,
        ptr::null_mut(),
        AVS_CL_TIMEOUT_MS,
        c"load multiple modules".as_ptr(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_unload_modules(
    adev: *mut avs_dev,
    mod_ids: *mut u16,
    num_mod_ids: u32,
) -> c_int {
    let mut msg = AVS_GLOBAL_REQUEST(UNLOAD_MULTIPLE_MODULES);
    let mut request: avs_ipc_msg = core::mem::zeroed();

    msg.load_multi_mods.mod_cnt = num_mod_ids;
    request.header = msg.val;
    request.data = mod_ids as *mut c_void;
    request.size = size_of::<u16>() * num_mod_ids as usize;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"unload multiple modules".as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_load_library(
    adev: *mut avs_dev,
    dma_id: u32,
    lib_id: u32,
) -> c_int {
    let mut msg = AVS_GLOBAL_REQUEST(LOAD_LIBRARY);
    let mut request = zeroed_ipc_msg();

    msg.load_lib.dma_id = dma_id;
    msg.load_lib.lib_id = lib_id;
    request.header = msg.val;

    avs_dsp_send_msg_timeout(
        adev,
        &mut request,
        ptr::null_mut(),
        AVS_CL_TIMEOUT_MS,
        c"load library".as_ptr(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_create_pipeline(
    adev: *mut avs_dev,
    req_size: u16,
    priority: u8,
    instance_id: u8,
    lp: bool,
    attributes: u16,
) -> c_int {
    let mut msg = AVS_GLOBAL_REQUEST(CREATE_PIPELINE);
    let mut request = zeroed_ipc_msg();

    msg.create_ppl.ppl_mem_size = req_size;
    msg.create_ppl.ppl_priority = priority;
    msg.create_ppl.instance_id = instance_id;
    msg.ext.create_ppl.lp = lp;
    msg.ext.create_ppl.attributes = attributes;
    request.header = msg.val;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"create pipeline".as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_delete_pipeline(adev: *mut avs_dev, instance_id: u8) -> c_int {
    let mut msg = AVS_GLOBAL_REQUEST(DELETE_PIPELINE);
    let mut request = zeroed_ipc_msg();

    msg.ppl.instance_id = instance_id;
    request.header = msg.val;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"delete pipeline".as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_set_pipeline_state(
    adev: *mut avs_dev,
    instance_id: u8,
    state: avs_pipeline_state,
) -> c_int {
    let mut msg = AVS_GLOBAL_REQUEST(SET_PIPELINE_STATE);
    let mut request = zeroed_ipc_msg();

    msg.set_ppl_state.ppl_id = instance_id;
    msg.set_ppl_state.state = state;
    request.header = msg.val;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"set pipeline state".as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_get_pipeline_state(
    adev: *mut avs_dev,
    instance_id: u8,
    state: *mut avs_pipeline_state,
) -> c_int {
    let mut msg = AVS_GLOBAL_REQUEST(GET_PIPELINE_STATE);
    let mut request = zeroed_ipc_msg();
    let mut reply = zeroed_ipc_msg();

    msg.get_ppl_state.ppl_id = instance_id;
    request.header = msg.val;

    let ret = avs_dsp_send_msg(adev, &mut request, &mut reply, c"get pipeline state".as_ptr());
    if ret == 0 {
        *state = reply.rsp.ext.get_ppl_state.state;
    }
    ret
}

/*
 * avs_ipc_init_instance - Initialize module instance
 *
 * @adev: Driver context
 * @module_id: Module-type id
 * @instance_id: Unique module instance id
 * @ppl_id: Parent pipeline id
 * @core_id: DSP core to allocate module on
 * @domain: Processing domain (low latency or data processing)
 * @param: Module-type specific configuration
 * @param_size: Size of @param in bytes
 *
 * Argument verification, as well as pipeline state checks are done by the
 * firmware.
 *
 * Note: @ppl_id and @core_id are independent of each other as single pipeline
 * can be composed of module instances located on different DSP cores.
 */
#[no_mangle]
pub unsafe extern "C" fn avs_ipc_init_instance(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    ppl_id: u8,
    core_id: u8,
    domain: u8,
    param: *mut c_void,
    param_size: u32,
) -> c_int {
    let mut msg = AVS_MODULE_REQUEST(INIT_INSTANCE);
    let mut request: avs_ipc_msg = core::mem::zeroed();

    msg.module_id = module_id;
    msg.instance_id = instance_id;
    /* firmware expects size provided in dwords */
    msg.ext.init_instance.param_block_size = div_round_up(param_size, size_of::<u32>());
    msg.ext.init_instance.ppl_instance_id = ppl_id;
    msg.ext.init_instance.core_id = core_id;
    msg.ext.init_instance.proc_domain = domain;

    request.header = msg.val;
    request.data = param;
    request.size = param_size as usize;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"init instance".as_ptr())
}

/*
 * avs_ipc_delete_instance - Delete module instance
 *
 * @adev: Driver context
 * @module_id: Module-type id
 * @instance_id: Unique module instance id
 *
 * Argument verification, as well as pipeline state checks are done by the
 * firmware.
 *
 * Note: only standalone modules i.e. without a parent pipeline shall be
 * deleted using this IPC message. In all other cases, pipeline owning the
 * modules performs cleanup automatically when it is deleted.
 */
#[no_mangle]
pub unsafe extern "C" fn avs_ipc_delete_instance(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
) -> c_int {
    let mut msg = AVS_MODULE_REQUEST(DELETE_INSTANCE);
    let mut request = zeroed_ipc_msg();

    msg.module_id = module_id;
    msg.instance_id = instance_id;
    request.header = msg.val;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"delete instance".as_ptr())
}

/*
 * avs_ipc_bind - Bind two module instances
 *
 * @adev: Driver context
 * @module_id: Source module-type id
 * @instance_id: Source module instance id
 * @dst_module_id: Sink module-type id
 * @dst_instance_id: Sink module instance id
 * @dst_queue: Sink module pin to bind @src_queue with
 * @src_queue: Source module pin to bind @dst_queue with
 */
#[no_mangle]
pub unsafe extern "C" fn avs_ipc_bind(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    dst_module_id: u16,
    dst_instance_id: u8,
    dst_queue: u8,
    src_queue: u8,
) -> c_int {
    let mut msg = AVS_MODULE_REQUEST(BIND);
    let mut request = zeroed_ipc_msg();

    msg.module_id = module_id;
    msg.instance_id = instance_id;
    msg.ext.bind_unbind.dst_module_id = dst_module_id;
    msg.ext.bind_unbind.dst_instance_id = dst_instance_id;
    msg.ext.bind_unbind.dst_queue = dst_queue;
    msg.ext.bind_unbind.src_queue = src_queue;
    request.header = msg.val;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"bind modules".as_ptr())
}

/*
 * avs_ipc_unbind - Unbind two module instances
 *
 * @adev: Driver context
 * @module_id: Source module-type id
 * @instance_id: Source module instance id
 * @dst_module_id: Sink module-type id
 * @dst_instance_id: Sink module instance id
 * @dst_queue: Sink module pin to unbind @src_queue from
 * @src_queue: Source module pin to unbind @dst_queue from
 */
#[no_mangle]
pub unsafe extern "C" fn avs_ipc_unbind(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    dst_module_id: u16,
    dst_instance_id: u8,
    dst_queue: u8,
    src_queue: u8,
) -> c_int {
    let mut msg = AVS_MODULE_REQUEST(UNBIND);
    let mut request = zeroed_ipc_msg();

    msg.module_id = module_id;
    msg.instance_id = instance_id;
    msg.ext.bind_unbind.dst_module_id = dst_module_id;
    msg.ext.bind_unbind.dst_instance_id = dst_instance_id;
    msg.ext.bind_unbind.dst_queue = dst_queue;
    msg.ext.bind_unbind.src_queue = src_queue;
    request.header = msg.val;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"unbind modules".as_ptr())
}

unsafe fn __avs_ipc_set_large_config(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    param_id: u8,
    init_block: bool,
    final_block: bool,
    request_data: *mut u8,
    request_size: usize,
    off_size: usize,
) -> c_int {
    let mut msg = AVS_MODULE_REQUEST(LARGE_CONFIG_SET);
    let mut request: avs_ipc_msg = core::mem::zeroed();

    msg.module_id = module_id;
    msg.instance_id = instance_id;
    msg.ext.large_config.data_off_size = off_size;
    msg.ext.large_config.large_param_id = param_id;
    msg.ext.large_config.final_block = final_block;
    msg.ext.large_config.init_block = init_block;

    request.header = msg.val;
    request.data = request_data as *mut c_void;
    request.size = request_size;

    avs_dsp_send_msg(adev, &mut request, ptr::null_mut(), c"large config set".as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_set_large_config(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    param_id: u8,
    request: *mut u8,
    request_size: usize,
) -> c_int {
    let mut remaining = request_size;
    let mut tx_size = core::cmp::min(AVS_MAILBOX_SIZE, remaining);
    let mut final_block = tx_size == remaining;

    /* Initial request states total payload size. */
    let mut ret = __avs_ipc_set_large_config(
        adev,
        module_id,
        instance_id,
        param_id,
        true,
        final_block,
        request,
        tx_size,
        request_size,
    );
    if ret != 0 {
        return ret;
    }

    remaining -= tx_size;

    /* Loop the rest only when payload exceeds mailbox's size. */
    while remaining != 0 {
        let offset = request_size - remaining;
        tx_size = core::cmp::min(AVS_MAILBOX_SIZE, remaining);
        final_block = tx_size == remaining;

        ret = __avs_ipc_set_large_config(
            adev,
            module_id,
            instance_id,
            param_id,
            false,
            final_block,
            request.add(offset),
            tx_size,
            offset,
        );
        if ret != 0 {
            return ret;
        }

        remaining -= tx_size;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_get_large_config(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    param_id: u8,
    request_data: *mut u8,
    request_size: usize,
    reply_data: *mut *mut u8,
    reply_size: *mut usize,
) -> c_int {
    let mut msg = AVS_MODULE_REQUEST(LARGE_CONFIG_GET);
    let mut request: avs_ipc_msg = core::mem::zeroed();
    let mut reply = zeroed_ipc_msg();

    reply.data = kzalloc(AVS_MAILBOX_SIZE, GFP_KERNEL);
    if reply.data.is_null() {
        return -ENOMEM;
    }

    msg.module_id = module_id;
    msg.instance_id = instance_id;
    msg.ext.large_config.data_off_size = request_size;
    msg.ext.large_config.large_param_id = param_id;
    /* final_block is always 0 on request. Updated by fw on reply. */
    msg.ext.large_config.final_block = false;
    msg.ext.large_config.init_block = true;

    request.header = msg.val;
    request.data = request_data as *mut c_void;
    request.size = request_size;
    reply.size = AVS_MAILBOX_SIZE;

    let ret = avs_dsp_send_msg(adev, &mut request, &mut reply, c"large config get".as_ptr());
    if ret != 0 {
        kfree(reply.data);
        return ret;
    }

    let buf = krealloc(reply.data, reply.size, GFP_KERNEL);
    if buf.is_null() {
        kfree(reply.data);
        return -ENOMEM;
    }

    *reply_data = buf as *mut u8;
    *reply_size = reply.size;

    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_set_dx(
    adev: *mut avs_dev,
    core_mask: u32,
    powerup: bool,
) -> c_int {
    let msg = AVS_MODULE_REQUEST(SET_DX);
    let mut request: avs_ipc_msg = core::mem::zeroed();
    let mut dx = avs_dxstate_info {
        core_mask,
        dx_mask: if powerup { core_mask } else { 0 },
    };

    request.header = msg.val;
    request.data = &mut dx as *mut avs_dxstate_info as *mut c_void;
    request.size = size_of::<avs_dxstate_info>();

    avs_dsp_send_pm_msg(adev, &mut request, ptr::null_mut(), true, c"set dx".as_ptr())
}

/*
 * avs_ipc_set_d0ix - Set power gating policy (entering D0IX substates)
 *
 * @enable_pg: Whether to enable or disable power gating
 * @streaming: Whether a stream is running when transitioning
 */
#[no_mangle]
pub unsafe extern "C" fn avs_ipc_set_d0ix(
    adev: *mut avs_dev,
    enable_pg: bool,
    streaming: bool,
) -> c_int {
    let mut msg = AVS_MODULE_REQUEST(SET_D0IX);
    let mut request = zeroed_ipc_msg();

    msg.ext.set_d0ix.wake = enable_pg;
    msg.ext.set_d0ix.streaming = streaming;
    msg.ext.set_d0ix.prevent_pg = !enable_pg;

    request.header = msg.val;

    avs_dsp_send_pm_msg(adev, &mut request, ptr::null_mut(), false, c"set d0ix".as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_get_fw_config(
    adev: *mut avs_dev,
    cfg: *mut avs_fw_cfg,
) -> c_int {
    let mut payload_size: usize = 0;
    let mut offset: usize = 0;
    let mut payload: *mut u8 = ptr::null_mut();

    let mut ret = avs_ipc_get_large_config(
        adev,
        AVS_BASEFW_MOD_ID,
        AVS_BASEFW_INST_ID,
        AVS_BASEFW_FIRMWARE_CONFIG,
        ptr::null_mut(),
        0,
        &mut payload,
        &mut payload_size,
    );
    if ret != 0 {
        goto_fw_err(adev, ret);
        return ret;
    }
    /* Non-zero payload expected for FIRMWARE_CONFIG. */
    if payload_size == 0 {
        ret = -EREMOTEIO;
        goto_fw_err(adev, ret);
        return ret;
    }

    while offset < payload_size {
        let tlv = payload.add(offset) as *mut avs_tlv;

        if (*tlv).type_ == AVS_FW_CFG_FW_VERSION {
            memcpy(
                &mut (*cfg).fw_version as *mut u32 as *mut c_void,
                (*tlv).value.as_ptr() as *const c_void,
                size_of::<u32>(),
            );
        } else if (*tlv).type_ == AVS_FW_CFG_MEMORY_RECLAIMED {
            (*cfg).memory_reclaimed = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_SLOW_CLOCK_FREQ_HZ {
            (*cfg).slow_clock_freq_hz = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_FAST_CLOCK_FREQ_HZ {
            (*cfg).fast_clock_freq_hz = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_ALH_SUPPORT_LEVEL {
            (*cfg).alh_support = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_IPC_DL_MAILBOX_BYTES {
            (*cfg).ipc_dl_mailbox_bytes = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_IPC_UL_MAILBOX_BYTES {
            (*cfg).ipc_ul_mailbox_bytes = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_TRACE_LOG_BYTES {
            (*cfg).trace_log_bytes = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_MAX_PPL_COUNT {
            (*cfg).max_ppl_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_MAX_ASTATE_COUNT {
            (*cfg).max_astate_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_MAX_MODULE_PIN_COUNT {
            (*cfg).max_module_pin_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_MODULES_COUNT {
            (*cfg).modules_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_MAX_MOD_INST_COUNT {
            (*cfg).max_mod_inst_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_MAX_LL_TASKS_PER_PRI_COUNT {
            (*cfg).max_ll_tasks_per_pri_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_LL_PRI_COUNT {
            (*cfg).ll_pri_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_MAX_DP_TASKS_COUNT {
            (*cfg).max_dp_tasks_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_MAX_LIBS_COUNT {
            (*cfg).max_libs_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_XTAL_FREQ_HZ {
            (*cfg).xtal_freq_hz = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_FW_CFG_POWER_GATING_POLICY {
            (*cfg).power_gating_policy = *(*tlv).value.as_ptr();
        /* Known but not useful to us. */
        } else if (*tlv).type_ == AVS_FW_CFG_DMA_BUFFER_CONFIG
            || (*tlv).type_ == AVS_FW_CFG_SCHEDULER_CONFIG
            || (*tlv).type_ == AVS_FW_CFG_CLOCKS_CONFIG
            || (*tlv).type_ == AVS_FW_CFG_RESERVED
        {
        } else {
            dev_info((*adev).dev, c"Unrecognized fw param: %d\n".as_ptr(), (*tlv).type_);
        }

        offset += size_of::<avs_tlv>() + (*tlv).length as usize;
    }

    /* No longer needed, free it as it's owned by the get_large_config() caller. */
    kfree(payload as *mut c_void);
    ret
}

unsafe fn goto_fw_err(adev: *mut avs_dev, ret: c_int) {
    if ret != 0 {
        dev_err((*adev).dev, c"get fw cfg failed: %d\n".as_ptr(), ret);
    }
}

/*
 * Original C signature is variadic:
 * int avs_ipc_set_fw_config(struct avs_dev *adev, size_t num_tlvs, ...)
 */
#[no_mangle]
pub unsafe extern "C" fn avs_ipc_set_fw_config(
    adev: *mut avs_dev,
    num_tlvs: usize,
    mut args: ...
) -> c_int {
    let mut offset: usize;
    let mut i: usize;
    let mut ret: c_int;

    let payload = kzalloc(AVS_MAILBOX_SIZE, GFP_KERNEL) as *mut u8;
    if payload.is_null() {
        return -ENOMEM;
    }

    offset = 0;
    i = 0;
    while i < num_tlvs && offset < AVS_MAILBOX_SIZE - size_of::<avs_tlv>() {
        let tlv = payload.add(offset) as *mut avs_tlv;
        (*tlv).type_ = args.arg::<u32>();
        (*tlv).length = args.arg::<u32>();

        offset += size_of::<avs_tlv>() + (*tlv).length as usize;
        if offset > AVS_MAILBOX_SIZE {
            break;
        }

        let src = args.arg::<*mut u8>();
        memcpy((*tlv).value.as_ptr() as *mut c_void, src as *const c_void, (*tlv).length as usize);
        i += 1;
    }

    if i == num_tlvs {
        ret = avs_ipc_set_large_config(
            adev,
            AVS_BASEFW_MOD_ID,
            AVS_BASEFW_INST_ID,
            AVS_BASEFW_FIRMWARE_CONFIG,
            payload,
            offset,
        );
    } else {
        ret = -ERANGE;
    }

    kfree(payload as *mut c_void);
    if ret != 0 {
        dev_err((*adev).dev, c"set fw cfg failed: %d\n".as_ptr(), ret);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_get_hw_config(
    adev: *mut avs_dev,
    cfg: *mut avs_hw_cfg,
) -> c_int {
    let mut payload_size: usize = 0;
    let mut offset: usize = 0;
    let mut payload: *mut u8 = ptr::null_mut();
    let mut ret = avs_ipc_get_large_config(
        adev,
        AVS_BASEFW_MOD_ID,
        AVS_BASEFW_INST_ID,
        AVS_BASEFW_HARDWARE_CONFIG,
        ptr::null_mut(),
        0,
        &mut payload,
        &mut payload_size,
    );
    if ret != 0 {
        goto_hw_err(adev, ret);
        return ret;
    }
    /* Non-zero payload expected for HARDWARE_CONFIG. */
    if payload_size == 0 {
        ret = -EREMOTEIO;
        goto_hw_err(adev, ret);
        return ret;
    }

    while offset < payload_size {
        let tlv = payload.add(offset) as *mut avs_tlv;

        if (*tlv).type_ == AVS_HW_CFG_AVS_VER {
            (*cfg).avs_version = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_HW_CFG_DSP_CORES {
            (*cfg).dsp_cores = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_HW_CFG_MEM_PAGE_BYTES {
            (*cfg).mem_page_bytes = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_HW_CFG_TOTAL_PHYS_MEM_PAGES {
            (*cfg).total_phys_mem_pages = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_HW_CFG_I2S_CAPS {
            (*cfg).i2s_caps.i2s_version = *(*tlv).value.as_ptr().add(0);
            let mut size = *(*tlv).value.as_ptr().add(1) as usize;
            (*cfg).i2s_caps.ctrl_count = size;
            if size != 0 {
                /* Multiply to get entire array size. */
                size *= size_of::<u32>();
                (*cfg).i2s_caps.ctrl_base_addr = devm_kmemdup(
                    (*adev).dev,
                    (*tlv).value.as_ptr().add(2) as *const c_void,
                    size,
                    GFP_KERNEL,
                ) as *mut u32;
                if (*cfg).i2s_caps.ctrl_base_addr.is_null() {
                    ret = -ENOMEM;
                    break;
                }
            }
        } else if (*tlv).type_ == AVS_HW_CFG_GATEWAY_COUNT {
            (*cfg).gateway_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_HW_CFG_HP_EBB_COUNT {
            (*cfg).hp_ebb_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_HW_CFG_LP_EBB_COUNT {
            (*cfg).lp_ebb_count = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_HW_CFG_EBB_SIZE_BYTES {
            (*cfg).ebb_size_bytes = *(*tlv).value.as_ptr();
        } else if (*tlv).type_ == AVS_HW_CFG_GPDMA_CAPS {
        } else {
            dev_info((*adev).dev, c"Unrecognized hw config: %d\n".as_ptr(), (*tlv).type_);
        }

        offset += size_of::<avs_tlv>() + (*tlv).length as usize;
    }

    /* No longer needed, free it as it's owned by the get_large_config() caller. */
    kfree(payload as *mut c_void);
    if ret != 0 {
        dev_err((*adev).dev, c"get hw cfg failed: %d\n".as_ptr(), ret);
    }
    ret
}

unsafe fn goto_hw_err(adev: *mut avs_dev, ret: c_int) {
    if ret != 0 {
        dev_err((*adev).dev, c"get hw cfg failed: %d\n".as_ptr(), ret);
    }
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_get_modules_info(
    adev: *mut avs_dev,
    info: *mut *mut avs_mods_info,
) -> c_int {
    let mut payload_size: usize = 0;
    let mut payload: *mut u8 = ptr::null_mut();

    let ret = avs_ipc_get_large_config(
        adev,
        AVS_BASEFW_MOD_ID,
        AVS_BASEFW_INST_ID,
        AVS_BASEFW_MODULES_INFO,
        ptr::null_mut(),
        0,
        &mut payload,
        &mut payload_size,
    );
    if ret != 0 {
        return ret;
    }
    /* Non-zero payload expected for MODULES_INFO. */
    if payload_size == 0 {
        return -EREMOTEIO;
    }

    *info = payload as *mut avs_mods_info;
    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_copier_set_sink_format(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    sink_id: u32,
    src_fmt: *const avs_audio_format,
    sink_fmt: *const avs_audio_format,
) -> c_int {
    let mut cpr_fmt: avs_copier_sink_format = core::mem::zeroed();

    cpr_fmt.sink_id = sink_id;
    /* Firmware expects driver to resend copier's input format. */
    cpr_fmt.src_fmt = *src_fmt;
    cpr_fmt.sink_fmt = *sink_fmt;

    avs_ipc_set_large_config(
        adev,
        module_id,
        instance_id,
        AVS_COPIER_SET_SINK_FORMAT,
        &mut cpr_fmt as *mut avs_copier_sink_format as *mut u8,
        size_of::<avs_copier_sink_format>(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_peakvol_get_volume(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    vols: *mut *mut avs_volume_cfg,
    num_vols: *mut usize,
) -> c_int {
    let mut payload_size: usize = 0;
    let mut payload: *mut u8 = ptr::null_mut();
    let ret = avs_ipc_get_large_config(
        adev,
        module_id,
        instance_id,
        AVS_PEAKVOL_VOLUME,
        ptr::null_mut(),
        0,
        &mut payload,
        &mut payload_size,
    );
    if ret != 0 {
        return ret;
    }

    /* Non-zero payload expected for PEAKVOL_VOLUME. */
    if payload_size == 0 {
        return -EREMOTEIO;
    }

    *vols = payload as *mut avs_volume_cfg;
    *num_vols = payload_size / size_of::<avs_volume_cfg>();

    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_peakvol_set_volume(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    vol: *mut avs_volume_cfg,
) -> c_int {
    avs_ipc_set_large_config(
        adev,
        module_id,
        instance_id,
        AVS_PEAKVOL_VOLUME,
        vol as *mut u8,
        size_of::<avs_volume_cfg>(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_peakvol_set_volumes(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    vols: *mut avs_volume_cfg,
    num_vols: usize,
) -> c_int {
    let mut offset: usize;
    let size = num_vols * size_of::<avs_volume_cfg>() + num_vols * size_of::<avs_tlv>();
    if size > AVS_MAILBOX_SIZE {
        return -EINVAL;
    }

    let payload = kzalloc(AVS_MAILBOX_SIZE, GFP_KERNEL) as *mut u8;
    if payload.is_null() {
        return -ENOMEM;
    }

    offset = 0;
    for i in 0..num_vols {
        let tlv = payload.add(offset) as *mut avs_tlv;

        (*tlv).type_ = AVS_PEAKVOL_VOLUME as u32;
        (*tlv).length = size_of::<avs_volume_cfg>() as u32;
        memcpy(
            (*tlv).value.as_ptr() as *mut c_void,
            vols.add(i) as *const c_void,
            (*tlv).length as usize,
        );

        offset += size_of::<avs_tlv>() + (*tlv).length as usize;
    }

    let ret = avs_ipc_set_large_config(
        adev,
        module_id,
        instance_id,
        AVS_VENDOR_CONFIG,
        payload,
        size,
    );
    kfree(payload as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_peakvol_get_mute(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    mutes: *mut *mut avs_mute_cfg,
    num_mutes: *mut usize,
) -> c_int {
    let mut payload_size: usize = 0;
    let mut payload: *mut u8 = ptr::null_mut();
    let ret = avs_ipc_get_large_config(
        adev,
        module_id,
        instance_id,
        AVS_PEAKVOL_MUTE,
        ptr::null_mut(),
        0,
        &mut payload,
        &mut payload_size,
    );
    if ret != 0 {
        return ret;
    }

    /* Non-zero payload expected for PEAKVOL_MUTE. */
    if payload_size == 0 {
        return -EREMOTEIO;
    }

    *mutes = payload as *mut avs_mute_cfg;
    *num_mutes = payload_size / size_of::<avs_mute_cfg>();

    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_peakvol_set_mute(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    mute: *mut avs_mute_cfg,
) -> c_int {
    avs_ipc_set_large_config(
        adev,
        module_id,
        instance_id,
        AVS_PEAKVOL_MUTE,
        mute as *mut u8,
        size_of::<avs_mute_cfg>(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_peakvol_set_mutes(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
    mutes: *mut avs_mute_cfg,
    num_mutes: usize,
) -> c_int {
    let mut offset: usize;
    let size = num_mutes * size_of::<avs_mute_cfg>() + num_mutes * size_of::<avs_tlv>();
    if size > AVS_MAILBOX_SIZE {
        return -EINVAL;
    }

    let payload = kzalloc(AVS_MAILBOX_SIZE, GFP_KERNEL) as *mut u8;
    if payload.is_null() {
        return -ENOMEM;
    }

    offset = 0;
    for i in 0..num_mutes {
        let tlv = payload.add(offset) as *mut avs_tlv;

        (*tlv).type_ = AVS_PEAKVOL_MUTE as u32;
        (*tlv).length = size_of::<avs_mute_cfg>() as u32;
        memcpy(
            (*tlv).value.as_ptr() as *mut c_void,
            mutes.add(i) as *const c_void,
            (*tlv).length as usize,
        );

        offset += size_of::<avs_tlv>() + (*tlv).length as usize;
    }

    let ret = avs_ipc_set_large_config(
        adev,
        module_id,
        instance_id,
        AVS_VENDOR_CONFIG,
        payload,
        size,
    );
    kfree(payload as *mut c_void);
    ret
}

// CONFIG_DEBUG_FS
#[no_mangle]
pub unsafe extern "C" fn avs_ipc_set_enable_logs(
    adev: *mut avs_dev,
    log_info: *mut u8,
    size: usize,
) -> c_int {
    avs_ipc_set_large_config(
        adev,
        AVS_BASEFW_MOD_ID,
        AVS_BASEFW_INST_ID,
        AVS_BASEFW_ENABLE_LOGS,
        log_info,
        size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_set_system_time(adev: *mut avs_dev) -> c_int {
    let mut sys_time: avs_sys_time = core::mem::zeroed();

    /* firmware expects UTC time in micro seconds */
    let us = ktime_to_us(ktime_get());
    sys_time.val_l = (us & u32::MAX as u64) as u32;
    sys_time.val_u = (us >> 32) as u32;

    avs_ipc_set_large_config(
        adev,
        AVS_BASEFW_MOD_ID,
        AVS_BASEFW_INST_ID,
        AVS_BASEFW_SYSTEM_TIME,
        &mut sys_time as *mut avs_sys_time as *mut u8,
        size_of::<avs_sys_time>(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_probe_get_dma(
    adev: *mut avs_dev,
    dmas: *mut *mut avs_probe_dma,
    num_dmas: *mut usize,
) -> c_int {
    let mut payload_size: usize = 0;
    let mut payload: *mut u8 = ptr::null_mut();
    let module_id = avs_get_module_id(adev, &AVS_PROBE_MOD_UUID as *const c_void);

    let ret = avs_ipc_get_large_config(
        adev,
        module_id as u16,
        AVS_PROBE_INST_ID,
        AVS_PROBE_INJECTION_DMA,
        ptr::null_mut(),
        0,
        &mut payload,
        &mut payload_size,
    );
    if ret != 0 {
        return ret;
    }

    *dmas = payload as *mut avs_probe_dma;
    *num_dmas = payload_size / size_of::<avs_probe_dma>();

    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_probe_attach_dma(
    adev: *mut avs_dev,
    dmas: *mut avs_probe_dma,
    num_dmas: usize,
) -> c_int {
    let module_id = avs_get_module_id(adev, &AVS_PROBE_MOD_UUID as *const c_void);

    avs_ipc_set_large_config(
        adev,
        module_id as u16,
        AVS_PROBE_INST_ID,
        AVS_PROBE_INJECTION_DMA,
        dmas as *mut u8,
        array_size(size_of::<avs_probe_dma>(), num_dmas),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_probe_detach_dma(
    adev: *mut avs_dev,
    node_ids: *mut avs_connector_node_id,
    num_node_ids: usize,
) -> c_int {
    let module_id = avs_get_module_id(adev, &AVS_PROBE_MOD_UUID as *const c_void);

    avs_ipc_set_large_config(
        adev,
        module_id as u16,
        AVS_PROBE_INST_ID,
        AVS_PROBE_INJECTION_DMA_DETACH,
        node_ids as *mut u8,
        array_size(size_of::<avs_connector_node_id>(), num_node_ids),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_probe_get_points(
    adev: *mut avs_dev,
    descs: *mut *mut avs_probe_point_desc,
    num_descs: *mut usize,
) -> c_int {
    let mut payload_size: usize = 0;
    let mut payload: *mut u8 = ptr::null_mut();
    let module_id = avs_get_module_id(adev, &AVS_PROBE_MOD_UUID as *const c_void);

    let ret = avs_ipc_get_large_config(
        adev,
        module_id as u16,
        AVS_PROBE_INST_ID,
        AVS_PROBE_POINTS,
        ptr::null_mut(),
        0,
        &mut payload,
        &mut payload_size,
    );
    if ret != 0 {
        return ret;
    }

    *descs = payload as *mut avs_probe_point_desc;
    *num_descs = payload_size / size_of::<avs_probe_point_desc>();

    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_probe_connect_points(
    adev: *mut avs_dev,
    descs: *mut avs_probe_point_desc,
    num_descs: usize,
) -> c_int {
    let module_id = avs_get_module_id(adev, &AVS_PROBE_MOD_UUID as *const c_void);

    avs_ipc_set_large_config(
        adev,
        module_id as u16,
        AVS_PROBE_INST_ID,
        AVS_PROBE_POINTS,
        descs as *mut u8,
        array_size(size_of::<avs_probe_point_desc>(), num_descs),
    )
}

#[no_mangle]
pub unsafe extern "C" fn avs_ipc_probe_disconnect_points(
    adev: *mut avs_dev,
    ids: *mut avs_probe_point_id,
    num_ids: usize,
) -> c_int {
    let module_id = avs_get_module_id(adev, &AVS_PROBE_MOD_UUID as *const c_void);

    avs_ipc_set_large_config(
        adev,
        module_id as u16,
        AVS_PROBE_INST_ID,
        AVS_PROBE_POINTS_DISCONNECT,
        ids as *mut u8,
        array_size(size_of::<avs_probe_point_id>(), num_ids),
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
