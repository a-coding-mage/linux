/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2021-2022 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

use core::ffi::{c_char, c_void};
use core::mem::ManuallyDrop;

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type size_t = usize;
pub type bool_ = bool;
pub type irqreturn_t = i32;
pub type atomic_t = i32;
pub type spinlock_t = c_void;
pub type wait_queue_head_t = c_void;
pub type guid_t = c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kfifo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_ext_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_tplg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_tplg_library {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_fw_version {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_fw_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_hw_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_mods_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_module_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_log_enable {
    _private: [u8; 0],
}

#[repr(C)]
pub union avs_notify_msg {
    _private: [u8; 0],
}

#[repr(C)]
pub union avs_global_msg {
    _private: [u8; 0],
}

#[repr(C)]
pub union avs_reply_msg {
    _private: [u8; 0],
}

/* CONFIG_ACPI: AVS_S0IX_SUPPORTED is (acpi_gbl_FADT.flags & ACPI_FADT_LOW_POWER_S0);
 * otherwise it is false.
 */

/*
 * struct avs_dsp_ops - Platform-specific DSP operations
 *
 * @power: Power on or off DSP cores
 * @reset: Enter or exit reset state on DSP cores
 * @stall: Stall or run DSP cores
 * @irq_handler: Top half of IPC servicing
 * @irq_thread: Bottom half of IPC servicing
 * @int_control: Enable or disable IPC interrupts
 */
#[repr(C)]
pub struct avs_dsp_ops {
    pub power: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool) -> i32>,
    pub reset: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool) -> i32>,
    pub stall: Option<unsafe extern "C" fn(*mut avs_dev, u32, bool) -> i32>,
    pub dsp_interrupt: Option<unsafe extern "C" fn(*mut avs_dev) -> irqreturn_t>,
    pub int_control: Option<unsafe extern "C" fn(*mut avs_dev, bool)>,
    pub load_basefw: Option<unsafe extern "C" fn(*mut avs_dev, *mut firmware) -> i32>,
    pub load_lib: Option<unsafe extern "C" fn(*mut avs_dev, *mut firmware, u32) -> i32>,
    pub transfer_mods:
        Option<unsafe extern "C" fn(*mut avs_dev, bool, *mut avs_module_entry, u32) -> i32>,
    pub config_basefw: Option<unsafe extern "C" fn(*mut avs_dev) -> i32>,
    pub enable_logs: Option<
        unsafe extern "C" fn(
            *mut avs_dev,
            avs_log_enable,
            u32,
            u32,
            usize,
            *mut u32,
        ) -> i32,
    >,
    pub log_buffer_offset: Option<unsafe extern "C" fn(*mut avs_dev, u32) -> i32>,
    pub log_buffer_status: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_notify_msg) -> i32>,
    pub coredump: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_notify_msg) -> i32>,
    pub d0ix_toggle: Option<unsafe extern "C" fn(*mut avs_dev, *mut avs_ipc_msg, bool) -> bool>,
    pub set_d0ix: Option<unsafe extern "C" fn(*mut avs_dev, bool) -> i32>,
}

#[macro_export]
macro_rules! avs_dsp_op {
    ($adev:expr, $op:ident $(, $arg:expr)* $(,)?) => {{
        ((*(*(*$adev).spec).dsp_ops).$op.unwrap())($adev $(, $arg)*)
    }};
}

unsafe extern "C" {
    pub static avs_skl_dsp_ops: avs_dsp_ops;
    pub static avs_apl_dsp_ops: avs_dsp_ops;
    pub static avs_cnl_dsp_ops: avs_dsp_ops;
    pub static avs_icl_dsp_ops: avs_dsp_ops;
    pub static avs_tgl_dsp_ops: avs_dsp_ops;
    pub static avs_ptl_dsp_ops: avs_dsp_ops;
}

pub const AVS_PLATATTR_CLDMA: u64 = 1u64 << 0;
pub const AVS_PLATATTR_IMR: u64 = 1u64 << 1;
pub const AVS_PLATATTR_ACE: u64 = 1u64 << 2;
pub const AVS_PLATATTR_ALTHDA: u64 = 1u64 << 3;

#[macro_export]
macro_rules! avs_platattr_test {
    ($adev:expr, CLDMA) => {
        ((*(*$adev).spec).attributes & $crate::AVS_PLATATTR_CLDMA)
    };
    ($adev:expr, IMR) => {
        ((*(*$adev).spec).attributes & $crate::AVS_PLATATTR_IMR)
    };
    ($adev:expr, ACE) => {
        ((*(*$adev).spec).attributes & $crate::AVS_PLATATTR_ACE)
    };
    ($adev:expr, ALTHDA) => {
        ((*(*$adev).spec).attributes & $crate::AVS_PLATATTR_ALTHDA)
    };
}

#[repr(C)]
pub struct avs_sram_spec {
    pub base_offset: u32,
    pub window_size: u32,
}

#[repr(C)]
pub struct avs_hipc_spec {
    pub req_offset: u32,
    pub req_ext_offset: u32,
    pub req_busy_mask: u32,
    pub ack_offset: u32,
    pub ack_done_mask: u32,
    pub rsp_offset: u32,
    pub rsp_busy_mask: u32,
    pub ctl_offset: u32,
    pub sts_offset: u32,
}

/* Platform specific descriptor */
#[repr(C)]
pub struct avs_spec {
    pub name: *const c_char,

    pub dsp_ops: *const avs_dsp_ops,
    pub min_fw_version: avs_fw_version, /* anything below is rejected */

    pub core_init_mask: u32,      /* used during DSP boot */
    pub attributes: u64,          /* bitmask of AVS_PLATATTR_* */
    pub sram: *const avs_sram_spec,
    pub hipc: *const avs_hipc_spec,
}

#[repr(C)]
pub struct avs_fw_entry {
    pub name: *const c_char,
    pub fw: *const firmware,

    pub node: list_head,
}

/*
 * struct avs_dev - Intel HD-Audio driver data
 *
 * @dev: PCI device
 * @dsp_ba: DSP bar address
 * @spec: platform-specific descriptor
 * @fw_cfg: Firmware configuration, obtained through FW_CONFIG message
 * @hw_cfg: Hardware configuration, obtained through HW_CONFIG message
 * @mods_info: Available module-types, obtained through MODULES_INFO message
 * @mod_idas: Module instance ID pool, one per module-type
 * @modres_mutex: For synchronizing any @mods_info updates
 * @ppl_ida: Pipeline instance ID pool
 * @fw_list: List of libraries loaded, including base firmware
 */
#[repr(C)]
pub struct avs_dev {
    pub base: hda_bus,
    pub dev: *mut device,

    pub dsp_ba: *mut c_void,
    pub spec: *const avs_spec,
    pub ipc: *mut avs_ipc,

    pub fw_cfg: avs_fw_cfg,
    pub hw_cfg: avs_hw_cfg,
    pub mods_info: *mut avs_mods_info,
    pub mod_idas: *mut *mut ida,
    pub modres_mutex: mutex,
    pub modcfg_buf: *mut c_void, /* module configuration buffer */
    pub ppl_ida: ida,
    pub fw_list: list_head,
    pub core_refs: *mut i32, /* reference count per core */
    pub lib_names: *mut *mut c_char,
    pub num_lp_paths: i32,
    pub l1sen_counter: atomic_t, /* controls whether L1SEN should be disabled */

    pub fw_ready: completion,
    pub probe_work: work_struct,

    pub comp_list: list_head,
    pub comp_list_mutex: mutex,
    pub path_list: list_head,
    pub path_list_lock: spinlock_t,
    pub path_mutex: mutex,

    pub trace_lock: spinlock_t, /* serialize debug window I/O between each LOG_BUFFER_STATUS */
    /* CONFIG_DEBUG_FS fields:
     * trace_fifo, trace_waitq, aging_timer_period, fifo_full_timer_period,
     * logged_resources, debugfs_root, extractor, num_probe_streams.
     */
}

/* from hda_bus to avs_dev */
/* hda_to_avs(hda) maps container_of(hda, struct avs_dev, base). */
/* from hdac_bus to avs_dev */
/* hdac_to_avs(hdac) maps hda_to_avs(to_hda_bus(hdac)). */
/* from device to avs_dev */
/* to_avs_dev(dev) gets drvdata as hdac_bus and maps it to avs_dev. */

unsafe extern "C" {
    pub fn avs_dsp_core_power(adev: *mut avs_dev, core_mask: u32, power: bool) -> i32;
    pub fn avs_dsp_core_reset(adev: *mut avs_dev, core_mask: u32, reset: bool) -> i32;
    pub fn avs_dsp_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> i32;
    pub fn avs_dsp_core_enable(adev: *mut avs_dev, core_mask: u32) -> i32;
    pub fn avs_dsp_core_disable(adev: *mut avs_dev, core_mask: u32) -> i32;
}

/* Inter Process Communication */

#[repr(C)]
pub union avs_ipc_msg_header {
    pub header: u64,
    pub glb: ManuallyDrop<avs_global_msg>,
    pub rsp: ManuallyDrop<avs_reply_msg>,
}

#[repr(C)]
pub struct avs_ipc_msg {
    pub u: avs_ipc_msg_header,
    pub data: *mut c_void,
    pub size: size_t,
}

/*
 * struct avs_ipc - DSP IPC context
 *
 * @dev: PCI device
 * @rx: Reply message cache
 * @default_timeout_ms: default message timeout in MS
 * @ready: whether firmware is ready and communication is open
 * @rx_completed: whether RX for previously sent TX has been received
 * @rx_lock: for serializing manipulation of rx_* fields
 * @msg_lock: for synchronizing request handling
 * @done_completion: DONE-part of IPC i.e. ROM and ACKs from FW
 * @busy_completion: BUSY-part of IPC i.e. receiving responses from FW
 */
#[repr(C)]
pub struct avs_ipc {
    pub dev: *mut device,

    pub rx: avs_ipc_msg,
    pub default_timeout_ms: u32,
    pub ready: bool,
    pub recovering: atomic_t,

    pub rx_completed: bool,
    pub rx_lock: spinlock_t,
    pub msg_mutex: mutex,
    pub done_completion: completion,
    pub busy_completion: completion,

    pub recovery_work: work_struct,
    pub d0ix_work: delayed_work,
    pub d0ix_disable_depth: atomic_t,
    pub in_d0ix: bool,
}

pub const AVS_EIPC: i32 = EREMOTEIO;
/*
 * IPC handlers may return positive value (firmware error code) what denotes
 * successful HOST <-> DSP communication yet failure to process specific request.
 *
 * Below macro converts returned value to linux kernel error code.
 * All IPC callers MUST use it as soon as firmware error code is consumed.
 */
#[inline]
pub const fn AVS_IPC_RET(ret: i32) -> i32 {
    if ret <= 0 {
        ret
    } else {
        -AVS_EIPC
    }
}

unsafe extern "C" {
    pub static EREMOTEIO: i32;

    pub fn avs_dsp_process_response(adev: *mut avs_dev, header: u64);
    pub fn avs_dsp_send_msg_timeout(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        reply: *mut avs_ipc_msg,
        timeout: i32,
        name: *const c_char,
    ) -> i32;
    pub fn avs_dsp_send_msg(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        reply: *mut avs_ipc_msg,
        name: *const c_char,
    ) -> i32;
    /* Two variants below are for messages that control DSP power states. */
    pub fn avs_dsp_send_pm_msg_timeout(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        reply: *mut avs_ipc_msg,
        timeout: i32,
        wake_d0i0: bool,
        name: *const c_char,
    ) -> i32;
    pub fn avs_dsp_send_pm_msg(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        reply: *mut avs_ipc_msg,
        wake_d0i0: bool,
        name: *const c_char,
    ) -> i32;
    pub fn avs_dsp_send_rom_msg_timeout(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        timeout: i32,
        name: *const c_char,
    ) -> i32;
    pub fn avs_dsp_send_rom_msg(
        adev: *mut avs_dev,
        request: *mut avs_ipc_msg,
        name: *const c_char,
    ) -> i32;
    pub fn avs_dsp_interrupt_control(adev: *mut avs_dev, enable: bool);
    pub fn avs_ipc_init(ipc: *mut avs_ipc, dev: *mut device) -> i32;
    pub fn avs_ipc_block(ipc: *mut avs_ipc);

    pub fn avs_dsp_disable_d0ix(adev: *mut avs_dev) -> i32;
    pub fn avs_dsp_enable_d0ix(adev: *mut avs_dev) -> i32;

    pub fn avs_mtl_core_power(adev: *mut avs_dev, core_mask: u32, power: bool) -> i32;
    pub fn avs_mtl_core_reset(adev: *mut avs_dev, core_mask: u32, power: bool) -> i32;
    pub fn avs_mtl_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> i32;
    pub fn avs_lnl_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> i32;
    pub fn avs_mtl_interrupt_control(adev: *mut avs_dev, enable: bool);
    pub fn avs_skl_ipc_interrupt(adev: *mut avs_dev);
    pub fn avs_cnl_dsp_interrupt(adev: *mut avs_dev) -> irqreturn_t;
    pub fn avs_mtl_dsp_interrupt(adev: *mut avs_dev) -> irqreturn_t;
    pub fn avs_apl_enable_logs(
        adev: *mut avs_dev,
        enable: avs_log_enable,
        aging_period: u32,
        fifo_full_period: u32,
        resource_mask: usize,
        priorities: *mut u32,
    ) -> i32;
    pub fn avs_icl_enable_logs(
        adev: *mut avs_dev,
        enable: avs_log_enable,
        aging_period: u32,
        fifo_full_period: u32,
        resource_mask: usize,
        priorities: *mut u32,
    ) -> i32;
    pub fn avs_skl_log_buffer_offset(adev: *mut avs_dev, core: u32) -> i32;
    pub fn avs_icl_log_buffer_offset(adev: *mut avs_dev, core: u32) -> i32;
    pub fn avs_apl_log_buffer_status(adev: *mut avs_dev, msg: *mut avs_notify_msg) -> i32;
    pub fn avs_apl_coredump(adev: *mut avs_dev, msg: *mut avs_notify_msg) -> i32;
    pub fn avs_apl_d0ix_toggle(adev: *mut avs_dev, tx: *mut avs_ipc_msg, wake: bool) -> bool;
    pub fn avs_icl_d0ix_toggle(adev: *mut avs_dev, tx: *mut avs_ipc_msg, wake: bool) -> bool;
    pub fn avs_apl_set_d0ix(adev: *mut avs_dev, enable: bool) -> i32;
    pub fn avs_icl_set_d0ix(adev: *mut avs_dev, enable: bool) -> i32;

    /* Firmware resources management */

    pub fn avs_get_module_entry(
        adev: *mut avs_dev,
        uuid: *const guid_t,
        entry: *mut avs_module_entry,
    ) -> i32;
    pub fn avs_get_module_id_entry(
        adev: *mut avs_dev,
        module_id: u32,
        entry: *mut avs_module_entry,
    ) -> i32;
    pub fn avs_get_module_id(adev: *mut avs_dev, uuid: *const guid_t) -> i32;
    pub fn avs_is_module_ida_empty(adev: *mut avs_dev, module_id: u32) -> bool;

    pub fn avs_module_info_init(adev: *mut avs_dev, purge: bool) -> i32;
    pub fn avs_module_info_free(adev: *mut avs_dev);
    pub fn avs_module_id_alloc(adev: *mut avs_dev, module_id: u16) -> i32;
    pub fn avs_module_id_free(adev: *mut avs_dev, module_id: u16, instance_id: u8);
    pub fn avs_request_firmware(
        adev: *mut avs_dev,
        fw_p: *mut *const firmware,
        name: *const c_char,
    ) -> i32;
    pub fn avs_release_last_firmware(adev: *mut avs_dev);
    pub fn avs_release_firmwares(adev: *mut avs_dev);

    pub fn avs_dsp_init_module(
        adev: *mut avs_dev,
        module_id: u16,
        ppl_instance_id: u8,
        core_id: u8,
        domain: u8,
        param: *mut c_void,
        param_size: u32,
        instance_id: *mut u8,
    ) -> i32;
    pub fn avs_dsp_delete_module(
        adev: *mut avs_dev,
        module_id: u16,
        instance_id: u8,
        ppl_instance_id: u8,
        core_id: u8,
    );
    pub fn avs_dsp_create_pipeline(
        adev: *mut avs_dev,
        req_size: u16,
        priority: u8,
        lp: bool,
        attributes: u16,
        instance_id: *mut u8,
    ) -> i32;
    pub fn avs_dsp_delete_pipeline(adev: *mut avs_dev, instance_id: u8) -> i32;

    /* Firmware loading */

    pub fn avs_hda_clock_gating_enable(adev: *mut avs_dev, enable: bool);
    pub fn avs_hda_power_gating_enable(adev: *mut avs_dev, enable: bool);
    pub fn avs_hda_l1sen_enable(adev: *mut avs_dev, enable: bool);

    pub fn avs_dsp_load_libraries(
        adev: *mut avs_dev,
        libs: *mut avs_tplg_library,
        num_libs: u32,
    ) -> i32;
    pub fn avs_dsp_boot_firmware(adev: *mut avs_dev, purge: bool) -> i32;
    pub fn avs_dsp_first_boot_firmware(adev: *mut avs_dev) -> i32;

    pub fn avs_cldma_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> i32;
    pub fn avs_cldma_load_library(adev: *mut avs_dev, lib: *mut firmware, id: u32) -> i32;
    pub fn avs_cldma_transfer_modules(
        adev: *mut avs_dev,
        load: bool,
        mods: *mut avs_module_entry,
        num_mods: u32,
    ) -> i32;
    pub fn avs_hda_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> i32;
    pub fn avs_hda_load_library(adev: *mut avs_dev, lib: *mut firmware, id: u32) -> i32;
    pub fn avs_hda_transfer_modules(
        adev: *mut avs_dev,
        load: bool,
        mods: *mut avs_module_entry,
        num_mods: u32,
    ) -> i32;

    pub fn avs_icl_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> i32;
}

/* Soc component members */

#[repr(C)]
pub struct avs_soc_component {
    pub base: *mut snd_soc_component,
    pub tplg: *mut avs_tplg,

    pub node: list_head,
}

/* to_avs_soc_component(comp) maps to snd_soc_component_to_priv(comp). */

unsafe extern "C" {
    pub static avs_dai_fe_ops: snd_soc_dai_ops;

    pub fn avs_register_dmic_component(adev: *mut avs_dev, name: *const c_char) -> i32;
    pub fn avs_register_i2s_component(
        adev: *mut avs_dev,
        name: *const c_char,
        port_mask: usize,
        tdms: *mut usize,
    ) -> i32;
    pub fn avs_register_hda_component(adev: *mut avs_dev, name: *const c_char) -> i32;
    pub fn avs_register_component(
        dev: *mut device,
        name: *const c_char,
        drv: *mut snd_soc_component_driver,
        cpu_dais: *mut snd_soc_dai_driver,
        num_cpu_dais: i32,
    ) -> i32;

    pub fn avs_register_all_boards(adev: *mut avs_dev) -> i32;
    pub fn avs_unregister_all_boards(adev: *mut avs_dev);

    pub fn avs_parse_sched_cfg(adev: *mut avs_dev, buf: *const c_char, len: size_t) -> i32;

    /* Filesystems integration */

    pub static avs_attr_groups: [*const attribute_group; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
