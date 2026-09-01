// SPDX-License-Identifier: GPL-2.0
//
// CS35l41 ALSA HDA audio driver
//
// Copyright 2021 Cirrus Logic, Inc.
//
// Author: Lucas Tanure <tanureal@opensource.cirrus.com>
//
// Rust translation of cs35l41_hda.c. C include dependencies are represented by
// external symbols and opaque C-compatible types supplied by the surrounding
// kernel translation.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8_ = u8;
type u32_ = u32;
type u64_ = u64;
type size_t = usize;
type __le32 = u32;
type __be32 = u32;
type acpi_handle = *mut c_void;
type irqreturn_t = c_int;
type control_bus = c_int;

const CS35L41_PART: *const c_char = b"cs35l41\0".as_ptr() as *const c_char;
const HALO_STATE_DSP_CTL_NAME: *const c_char = b"HALO_STATE\0".as_ptr() as *const c_char;
const HALO_STATE_DSP_CTL_TYPE: c_int = 5;
const HALO_STATE_DSP_CTL_ALG: c_uint = 262308;
const CAL_R_DSP_CTL_NAME: *const c_char = b"CAL_R\0".as_ptr() as *const c_char;
const CAL_STATUS_DSP_CTL_NAME: *const c_char = b"CAL_STATUS\0".as_ptr() as *const c_char;
const CAL_CHECKSUM_DSP_CTL_NAME: *const c_char = b"CAL_CHECKSUM\0".as_ptr() as *const c_char;
const CAL_AMBIENT_DSP_CTL_NAME: *const c_char = b"CAL_AMBIENT\0".as_ptr() as *const c_char;
const CAL_DSP_CTL_TYPE: c_int = 5;
const CAL_DSP_CTL_ALG: c_uint = 205;
const CS35L41_UUID: *const c_char =
    b"50d90cdc-3de4-4f18-b528-c7fe3b71f40d\0".as_ptr() as *const c_char;
const CS35L41_DSM_GET_MUTE: c_uint = 5;
const CS35L41_NOTIFY_EVENT: u32_ = 0x91;
const CS35L41_TUNING_SIG: u32_ = 0x109A4A35;

const TUNING_PARAM_GAIN: c_uint = 0;

#[repr(C, packed)]
struct cs35l41_tuning_param_hdr {
    tuning_index: __le32,
    type_: __le32,
    size: __le32,
}

#[repr(C)]
union cs35l41_tuning_param_union {
    gain: __le32,
}

#[repr(C, packed)]
struct cs35l41_tuning_param {
    hdr: cs35l41_tuning_param_hdr,
    u: cs35l41_tuning_param_union,
}

#[repr(C, packed)]
struct cs35l41_tuning_params {
    signature: __le32,
    version: __le32,
    size: __le32,
    num_entries: __le32,
    data: [u8_; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct firmware {
    size: size_t,
    data: *const u8_,
}
#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
struct acpi_device {
    _private: [u8; 0],
}
#[repr(C)]
struct spi_device {
    dev: device,
    max_speed_hz: c_uint,
}
#[repr(C)]
struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap_irq_data {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
struct hda_codec_core {
    subsystem_id: c_uint,
    dev: device,
}
#[repr(C)]
struct hda_codec {
    core: hda_codec_core,
    card: *mut snd_card,
}
#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol_new {
    name: *const c_char,
    iface: c_int,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    access: c_uint,
}
#[repr(C)]
struct snd_ctl_elem_info {
    _private: [u8; 0],
}
#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
    enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [i64; 128],
}
#[repr(C)]
struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 128],
}
#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}
#[repr(C)]
struct cs_dsp {
    name: *const c_char,
    client_ops: *const cs_dsp_client_ops,
    running: bool_,
    booted: bool_,
    pwr_lock: mutex,
}
#[repr(C)]
struct cs_dsp_client_ops {
    _private: [u8; 0],
}
#[repr(C)]
struct cirrus_amp_cal_controls {
    alg_id: c_uint,
    mem_region: c_int,
    ambient: *const c_char,
    calr: *const c_char,
    status: *const c_char,
    checksum: *const c_char,
}
#[repr(C)]
struct cirrus_amp_cal_data {
    calR: c_int,
}
#[repr(C)]
struct cs35l41_gpio_cfg {
    valid: bool_,
    func: c_int,
    out_en: bool_,
}
#[repr(C)]
struct cs35l41_hw_cfg {
    valid: bool_,
    bst_type: c_int,
    bst_ipk: c_int,
    bst_ind: c_int,
    bst_cap: c_int,
    spk_pos: c_uint,
    gpio1: cs35l41_gpio_cfg,
    gpio2: cs35l41_gpio_cfg,
}
#[repr(C)]
struct cs35l41_hda {
    dev: *mut device,
    regmap: *mut regmap,
    irq: c_int,
    irq_data: *mut regmap_irq_data,
    irq_errors: c_ulong,
    cs_dsp: cs_dsp,
    halo_initialized: bool_,
    cal_data_valid: bool_,
    cal_data: cirrus_amp_cal_data,
    tuning_gain: c_uint,
    playback_started: bool_,
    mute_override: bool_,
    request_fw_load: bool_,
    fw_request_ongoing: bool_,
    bypass_fw: bool_,
    fw_load_work: work_struct,
    fw_mutex: mutex,
    hw_cfg: cs35l41_hw_cfg,
    amp_name: *mut c_char,
    speaker_id: c_int,
    acpi_subsystem_id: *const c_char,
    channel_index: c_uint,
    index: c_int,
    codec: *mut hda_codec,
    dacpi: *mut acpi_device,
    reset_gpio: *mut gpio_desc,
    cs_gpio: *mut gpio_desc,
    control_bus: control_bus,
    fw_load_ctl: *mut snd_kcontrol,
    fw_type_ctl: *mut snd_kcontrol,
    mute_override_ctl: *mut snd_kcontrol,
}
#[repr(C)]
struct hda_component {
    dev: *mut device,
    name: [c_char; 64],
    playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
    pre_playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
    post_playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
    acpi_notify: Option<unsafe extern "C" fn(acpi_handle, u32_, *mut device)>,
    adev: *mut acpi_device,
    acpi_notifications_supported: bool_,
}
#[repr(C)]
struct hda_component_parent {
    codec: *mut hda_codec,
}
#[repr(C)]
struct component_ops {
    bind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void) -> c_int>,
    unbind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void)>,
}
#[repr(C)]
struct cs35l41_irq {
    irq: c_int,
    name: *const c_char,
    handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
}
#[repr(C)]
struct regmap_irq {
    reg_offset: c_uint,
    mask: c_uint,
}
#[repr(C)]
struct regmap_irq_chip {
    name: *const c_char,
    status_base: c_uint,
    mask_base: c_uint,
    ack_base: c_uint,
    num_regs: c_uint,
    irqs: *const regmap_irq,
    num_irqs: c_int,
    runtime_pm: bool_,
}
#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct guid_t {
    b: [u8; 16],
}

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EOVERFLOW: c_int = 75;
const EFAULT: c_int = 14;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_ONESHOT: c_int = 0x00002000;
const IRQF_SHARED: c_int = 0x00000080;
const GPIOD_IN: c_int = 0;
const GPIOD_OUT_LOW: c_int = 0;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;
const SNDRV_CTL_ELEM_IFACE_CARD: c_int = 0;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 2;
const DL_FLAG_STATELESS: c_uint = 0;
const ACPI_TYPE_BUFFER: c_int = 3;
const HDA_MAX_COMPONENTS: usize = 4;
const SPI: control_bus = 1;

extern "C" {
    static CS35L41_PLL_CLK_CTRL: c_uint;
    static CS35L41_DSP_CLK_CTRL: c_uint;
    static CS35L41_GLOBAL_CLK_CTRL: c_uint;
    static CS35L41_SP_RATE_CTRL: c_uint;
    static CS35L41_SP_FORMAT: c_uint;
    static CS35L41_SP_TX_WL: c_uint;
    static CS35L41_SP_RX_WL: c_uint;
    static CS35L41_ASP_TX1_SRC: c_uint;
    static CS35L41_ASP_TX2_SRC: c_uint;
    static CS35L41_DSP1_RX3_SRC: c_uint;
    static CS35L41_DSP1_RX4_SRC: c_uint;
    static CS35L41_SP_HIZ_CTRL: c_uint;
    static CS35L41_DAC_PCM1_SRC: c_uint;
    static CS35L41_ASP_TX3_SRC: c_uint;
    static CS35L41_ASP_TX4_SRC: c_uint;
    static CS35L41_DSP1_RX5_SRC: c_uint;
    static CS35L41_DSP1_RX6_SRC: c_uint;
    static CS35L41_AMP_DIG_VOL_CTRL: c_uint;
    static CS35L41_AMP_GAIN_CTRL: c_uint;
    static CS35L41_DIE_STS2: c_uint;
    static CS35L41_DIE_STS1: c_uint;
    static CS35L41_PROTECT_REL_ERR_IGN: c_uint;
    static CS35L41_ASP_TX1_EN_MASK: c_uint;
    static CS35L41_ASP_RX1_EN_MASK: c_uint;
    static CS35L41_ASP_RX2_EN_MASK: c_uint;
    static CS35L41_INT_BOOST: c_int;
    static CS35L41_EXT_BOOST: c_int;
    static CS35L41_EXT_BOOST_NO_VSPK_SWITCH: c_int;
    static CS35L41_CENTER: c_uint;
    static CS35L41_INPUT_SRC_VPMON: c_uint;
    static CS35L41_INPUT_SRC_VBSTMON: c_uint;
    static CS35L41_SP_ENABLES: c_uint;
    static CS35L41_DSP1_RX1_SRC: c_uint;
    static CS35L41_DSP1_RX2_SRC: c_uint;
    static CS35L41_PWR_CTRL2: c_uint;
    static CS35L41_VMON_EN_MASK: c_uint;
    static CS35L41_IMON_EN_MASK: c_uint;
    static CS35L41_VMON_EN_SHIFT: c_uint;
    static CS35L41_IMON_EN_SHIFT: c_uint;
    static CS35L41_AMP_EN_MASK: c_uint;
    static CS35L41_AMP_EN_SHIFT: c_uint;
    static CS35L41_GPIO1_CTRL1: c_uint;
    static CS35L41_DEVID: c_uint;
    static CS35L41_REVID: c_uint;
    static CS35L41_MTLREVID_MASK: c_uint;
    static CS35L41R_CHIP_ID: c_uint;
    static CS35L41_CHIP_ID: c_uint;
    static CS35L41_IRQ1_STATUS4: c_uint;
    static CS35L41_OTP_BOOT_DONE: c_uint;
    static CS35L41_IRQ1_STATUS3: c_uint;
    static CS35L41_OTP_BOOT_ERR: c_uint;
    static CS35L41_SFT_RESET: c_uint;
    static CS35L41_SOFTWARE_RESET: c_uint;
    static CSPL_MBOX_CMD_RESUME: c_uint;
    static CSPL_MBOX_CMD_PAUSE: c_uint;
    static CSPL_MBOX_STS_RUNNING: c_uint;
    static CSPL_MBOX_STS_PAUSED: c_uint;
    static CS35L41_DSP_MBOX_2: c_uint;
    static HALO_STATE_CODE_RUN: c_uint;
    static DEFAULT_AMP_GAIN_PCM: c_uint;
    static DEFAULT_AMP_GAIN_PDM: c_uint;
    static CS35L41_AMP_GAIN_PCM_SHIFT: c_uint;
    static CS35L41_AMP_GAIN_PDM_SHIFT: c_uint;
    static HDA_GEN_PCM_ACT_OPEN: c_int;
    static HDA_GEN_PCM_ACT_PREPARE: c_int;
    static HDA_GEN_PCM_ACT_CLEANUP: c_int;
    static HDA_GEN_PCM_ACT_CLOSE: c_int;
    static CS35L41_NOT_USED: c_int;
    static CS35l41_VSPK_SWITCH: c_int;
    static CS35l41_SYNC: c_int;
    static CS35L41_GPIO1_GPIO: c_int;
    static CS35L41_GPIO1_MDSYNC: c_int;
    static CS35L41_INTERRUPT: c_int;
    static CS35L41_GPIO2_INT_OPEN_DRAIN: c_int;
    static CS35L41_MAX_ACCEPTABLE_SPI_SPEED_HZ: c_uint;
    static CS35L41_NUM_IRQ: usize;
    static CS35L41_IRQ1_STATUS1: c_uint;
    static CS35L41_IRQ1_MASK1: c_uint;
    static CS35L41_BST_SHORT_ERR_RLS_SHIFT: c_int;
    static CS35L41_BST_UVP_ERR_RLS_SHIFT: c_int;
    static CS35L41_BST_OVP_ERR_RLS_SHIFT: c_int;
    static CS35L41_TEMP_ERR_RLS_SHIFT: c_int;
    static CS35L41_TEMP_WARN_ERR_RLS_SHIFT: c_int;
    static CS35L41_AMP_SHORT_ERR_RLS_SHIFT: c_int;

    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn firmware_request_nowarn(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_once(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn isalnum(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn cs_amp_write_cal_coeffs(dsp: *mut cs_dsp, ctrls: *const cirrus_amp_cal_controls, cal: *const cirrus_amp_cal_data) -> c_int;
    fn cs_amp_get_efi_calibration_data(dev: *mut device, uid: u64_, index: c_int, cal: *mut cirrus_amp_cal_data) -> c_int;
    fn cs35l41_configure_cs_dsp(dev: *mut device, map: *mut regmap, dsp: *mut cs_dsp);
    fn cs_dsp_halo_init(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_power_up(dsp: *mut cs_dsp, wmfw: *const firmware, wmfw_name: *const c_char, coeff: *const firmware, coeff_name: *const c_char, fw_type: *const c_char) -> c_int;
    fn cs_dsp_stop(dsp: *mut cs_dsp);
    fn cs_dsp_power_down(dsp: *mut cs_dsp);
    fn cs_dsp_remove(dsp: *mut cs_dsp);
    fn cs_dsp_run(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_get_ctl(dsp: *mut cs_dsp, name: *const c_char, type_: c_int, alg: c_uint) -> *mut c_void;
    fn cs_dsp_coeff_read_ctrl(ctl: *mut c_void, off: c_uint, buf: *mut c_void, len: size_t) -> c_int;
    fn cs35l41_write_fs_errata(dev: *mut device, map: *mut regmap) -> c_int;
    fn cs35l41_set_cspl_mbox_cmd(dev: *mut device, map: *mut regmap, cmd: c_uint) -> c_int;
    fn cs35l41_global_enable(dev: *mut device, map: *mut regmap, bst_type: c_int, enable: c_int, dsp: *mut cs_dsp);
    fn cs35l41_set_channels(dev: *mut device, map: *mut regmap, tx_num: c_uint, tx_slot: *mut c_uint, rx_num: c_uint, rx_slot: *mut c_uint) -> c_int;
    fn cs35l41_enter_hibernate(dev: *mut device, map: *mut regmap, bst_type: c_int) -> c_int;
    fn cs35l41_exit_hibernate(dev: *mut device, map: *mut regmap) -> c_int;
    fn cs35l41_safe_reset(map: *mut regmap, bst_type: c_int) -> c_int;
    fn cs35l41_test_key_unlock(dev: *mut device, map: *mut regmap) -> c_int;
    fn cs35l41_test_key_lock(dev: *mut device, map: *mut regmap) -> c_int;
    fn cs35l41_init_boost(dev: *mut device, map: *mut regmap, cfg: *mut cs35l41_hw_cfg) -> c_int;
    fn cs35l41_gpio_config(map: *mut regmap, cfg: *mut cs35l41_hw_cfg) -> c_int;
    fn cs35l41_register_errata_patch(dev: *mut device, map: *mut regmap, revid: c_uint) -> c_int;
    fn cs35l41_otp_unpack(dev: *mut device, map: *mut regmap) -> c_int;
    fn cancel_work_sync(work: *mut work_struct) -> bool_;
    fn schedule_work(work: *mut work_struct) -> bool_;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_dont_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiod_get_index(dev: *mut device, con_id: *const c_char, idx: c_uint, flags: c_int) -> *mut gpio_desc;
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_put(desc: *mut gpio_desc);
    fn gpiod_count(dev: *mut device, con_id: *const c_char) -> c_int;
    fn fwnode_gpiod_get_index(fwnode: *mut c_void, con_id: *const c_char, index: c_uint, flags: c_int, label: *const c_char) -> *mut gpio_desc;
    fn acpi_fwnode_handle(adev: *mut acpi_device) -> *mut c_void;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_char, hrv: c_long) -> *mut acpi_device;
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn acpi_get_subsystem_id(handle: acpi_handle) -> *const c_char;
    fn acpi_device_handle(adev: *mut acpi_device) -> acpi_handle;
    fn acpi_check_dsm(handle: acpi_handle, guid: *const guid_t, rev: c_uint, funcs: u64_) -> bool_;
    fn acpi_evaluate_dsm_typed(handle: acpi_handle, guid: *const guid_t, rev: c_uint, func: c_uint, argv4: *mut c_void, type_: c_int) -> *mut acpi_object;
    fn ACPI_FREE(ptr: *mut c_void);
    fn guid_parse(str_: *const c_char, guid: *mut guid_t) -> c_int;
    fn device_property_count_u32(dev: *mut device, propname: *const c_char) -> c_int;
    fn device_property_read_u32_array(dev: *mut device, propname: *const c_char, val: *mut u32_, nval: size_t) -> c_int;
    fn cs35l41_add_dsd_properties(cs35l41: *mut cs35l41_hda, physdev: *mut device, id: c_int, hid: *const c_char) -> c_int;
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, names: *const *const c_char) -> c_int;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_new1(ctl: *mut snd_kcontrol_new, data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_remove(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn hda_component_from_index(parent: *mut hda_component_parent, index: c_int) -> *mut hda_component;
    fn lock_system_sleep() -> c_uint;
    fn unlock_system_sleep(flags: c_uint);
    fn device_link_add(consumer: *mut device, supplier: *mut device, flags: c_uint) -> *mut c_void;
    fn device_link_remove(consumer: *mut device, supplier: *mut device);
    fn component_add(dev: *mut device, ops: *const component_ops) -> c_int;
    fn component_del(dev: *mut device, ops: *const component_ops);
    fn devm_regmap_add_irq_chip(dev: *mut device, map: *mut regmap, irq: c_int, flags: c_int, irq_base: c_int, chip: *const regmap_irq_chip, data: *mut *mut regmap_irq_data) -> c_int;
    fn regmap_irq_get_virq(data: *mut regmap_irq_data, irq: c_int) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *mut c_void, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_int, name: *const c_char, data: *mut c_void) -> c_int;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn usleep_range(min: c_ulong, max: c_ulong);
}

#[repr(C)]
struct acpi_object_buffer {
    length: c_uint,
    pointer: *mut u8_,
}
#[repr(C)]
union acpi_object_union {
    buffer: acpi_object_buffer,
}
#[repr(C)]
struct acpi_object {
    type_: c_uint,
    u: acpi_object_union,
}
#[repr(C)]
struct reg_sequence {
    reg: c_uint,
    def: c_uint,
}

unsafe fn le32_to_cpu(v: __le32) -> u32_ {
    u32::from_le(v)
}
unsafe fn be32_to_cpu(v: __be32) -> u32_ {
    u32::from_be(v)
}
unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}
unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}
const fn BIT(n: c_uint) -> u64_ {
    1u64 << n
}

static cs35l41_calibration_controls: cirrus_amp_cal_controls = cirrus_amp_cal_controls {
    alg_id: CAL_DSP_CTL_ALG,
    mem_region: CAL_DSP_CTL_TYPE,
    ambient: CAL_AMBIENT_DSP_CTL_NAME,
    calr: CAL_R_DSP_CTL_NAME,
    status: CAL_STATUS_DSP_CTL_NAME,
    checksum: CAL_CHECKSUM_DSP_CTL_NAME,
};

const CS35L41_HDA_FW_SPK_PROT: usize = 0;
const CS35L41_HDA_FW_SPK_CALI: usize = 1;
const CS35L41_HDA_FW_SPK_DIAG: usize = 2;
const CS35L41_HDA_FW_MISC: usize = 3;
const CS35L41_HDA_NUM_FW: usize = 4;

static cs35l41_hda_fw_ids: [*const c_char; CS35L41_HDA_NUM_FW] = [
    b"spk-prot\0".as_ptr() as *const c_char,
    b"spk-cali\0".as_ptr() as *const c_char,
    b"spk-diag\0".as_ptr() as *const c_char,
    b"misc\0".as_ptr() as *const c_char,
];

static mut firmware_autostart: bool_ = true;
// module_param(firmware_autostart, bool, 0444);
// MODULE_PARM_DESC(...);

static channel_name: [c_char; 3] = [b'L' as c_char, b'R' as c_char, b'C' as c_char];

static cs35l41_hda_config: [reg_sequence; 11] = unsafe {
    [
        reg_sequence { reg: CS35L41_PLL_CLK_CTRL, def: 0x00000430 },
        reg_sequence { reg: CS35L41_DSP_CLK_CTRL, def: 0x00000003 },
        reg_sequence { reg: CS35L41_GLOBAL_CLK_CTRL, def: 0x00000003 },
        reg_sequence { reg: CS35L41_SP_RATE_CTRL, def: 0x00000021 },
        reg_sequence { reg: CS35L41_SP_FORMAT, def: 0x20200200 },
        reg_sequence { reg: CS35L41_SP_TX_WL, def: 0x00000018 },
        reg_sequence { reg: CS35L41_SP_RX_WL, def: 0x00000018 },
        reg_sequence { reg: CS35L41_ASP_TX1_SRC, def: 0x00000018 },
        reg_sequence { reg: CS35L41_ASP_TX2_SRC, def: 0x00000019 },
        reg_sequence { reg: CS35L41_DSP1_RX3_SRC, def: 0x00000018 },
        reg_sequence { reg: CS35L41_DSP1_RX4_SRC, def: 0x00000019 },
    ]
};
static cs35l41_hda_config_no_dsp: [reg_sequence; 6] = unsafe {
    [
        reg_sequence { reg: CS35L41_SP_HIZ_CTRL, def: 0x00000002 },
        reg_sequence { reg: CS35L41_DAC_PCM1_SRC, def: 0x00000008 },
        reg_sequence { reg: CS35L41_ASP_TX3_SRC, def: 0x00000000 },
        reg_sequence { reg: CS35L41_ASP_TX4_SRC, def: 0x00000000 },
        reg_sequence { reg: CS35L41_DSP1_RX5_SRC, def: 0x00000020 },
        reg_sequence { reg: CS35L41_DSP1_RX6_SRC, def: 0x00000021 },
    ]
};
static cs35l41_hda_config_dsp: [reg_sequence; 5] = unsafe {
    [
        reg_sequence { reg: CS35L41_SP_HIZ_CTRL, def: 0x00000003 },
        reg_sequence { reg: CS35L41_DAC_PCM1_SRC, def: 0x00000032 },
        reg_sequence { reg: CS35L41_ASP_TX3_SRC, def: 0x00000028 },
        reg_sequence { reg: CS35L41_ASP_TX4_SRC, def: 0x00000029 },
        reg_sequence { reg: CS35L41_DSP1_RX6_SRC, def: 0x00000029 },
    ]
};
static cs35l41_hda_unmute: [reg_sequence; 2] = unsafe {
    [
        reg_sequence { reg: CS35L41_AMP_DIG_VOL_CTRL, def: 0x00008000 },
        reg_sequence { reg: CS35L41_AMP_GAIN_CTRL, def: 0x00000084 },
    ]
};
static cs35l41_hda_mute: [reg_sequence; 2] = unsafe {
    [
        reg_sequence { reg: CS35L41_AMP_GAIN_CTRL, def: 0x00000000 },
        reg_sequence { reg: CS35L41_AMP_DIG_VOL_CTRL, def: 0x0000A678 },
    ]
};

static client_ops: cs_dsp_client_ops = cs_dsp_client_ops { _private: [] };

unsafe fn cs35l41_request_tuning_param_file(
    cs35l41: *mut cs35l41_hda,
    tuning_filename: *mut c_char,
    firmware: *mut *const firmware,
    filename: *mut *mut c_char,
    _ssid: *const c_char,
) -> c_int {
    let mut ret: c_int;
    *filename = kasprintf(GFP_KERNEL, b"%scfg\0".as_ptr() as *const c_char, tuning_filename);
    if (*filename).is_null() {
        return -ENOMEM;
    }
    ret = firmware_request_nowarn(firmware, *filename, (*cs35l41).dev);
    if ret != 0 {
        dev_dbg((*cs35l41).dev, b"Failed to request '%s'\n\0".as_ptr() as *const c_char, *filename);
        kfree(*filename as *const c_void);
        *filename = ptr::null_mut();
    }
    ret
}

unsafe fn cs35l41_request_firmware_file(
    cs35l41: *mut cs35l41_hda,
    firmware: *mut *const firmware,
    filename: *mut *mut c_char,
    ssid: *const c_char,
    amp_name: *const c_char,
    spkid: c_int,
    filetype: *const c_char,
) -> c_int {
    let dsp_name = (*cs35l41).cs_dsp.name;
    *firmware = ptr::null();
    if spkid > -1 && !ssid.is_null() && !amp_name.is_null() {
        *filename = kasprintf(GFP_KERNEL, b"cirrus/%s-%s-%s-%s-spkid%d-%s.%s\0".as_ptr() as *const c_char, CS35L41_PART, dsp_name, cs35l41_hda_fw_ids[(*cs35l41).firmware_type as usize], ssid, spkid, amp_name, filetype);
    } else if spkid > -1 && !ssid.is_null() {
        *filename = kasprintf(GFP_KERNEL, b"cirrus/%s-%s-%s-%s-spkid%d.%s\0".as_ptr() as *const c_char, CS35L41_PART, dsp_name, cs35l41_hda_fw_ids[(*cs35l41).firmware_type as usize], ssid, spkid, filetype);
    } else if !ssid.is_null() && !amp_name.is_null() {
        *filename = kasprintf(GFP_KERNEL, b"cirrus/%s-%s-%s-%s-%s.%s\0".as_ptr() as *const c_char, CS35L41_PART, dsp_name, cs35l41_hda_fw_ids[(*cs35l41).firmware_type as usize], ssid, amp_name, filetype);
    } else if !ssid.is_null() {
        *filename = kasprintf(GFP_KERNEL, b"cirrus/%s-%s-%s-%s.%s\0".as_ptr() as *const c_char, CS35L41_PART, dsp_name, cs35l41_hda_fw_ids[(*cs35l41).firmware_type as usize], ssid, filetype);
    } else {
        *filename = kasprintf(GFP_KERNEL, b"cirrus/%s-%s-%s.%s\0".as_ptr() as *const c_char, CS35L41_PART, dsp_name, cs35l41_hda_fw_ids[(*cs35l41).firmware_type as usize], filetype);
    }
    if (*filename).is_null() {
        return -ENOMEM;
    }
    let mut s = *filename;
    while *s != 0 {
        let c = *s;
        if isalnum(c as c_int) != 0 {
            *s = tolower(c as c_int) as c_char;
        } else if c != b'.' as c_char && c != b'/' as c_char {
            *s = b'-' as c_char;
        }
        s = s.add(1);
    }
    let ret = firmware_request_nowarn(firmware, *filename, (*cs35l41).dev);
    if ret != 0 {
        dev_dbg((*cs35l41).dev, b"Failed to request '%s'\n\0".as_ptr() as *const c_char, *filename);
        kfree(*filename as *const c_void);
        *filename = ptr::null_mut();
    }
    ret
}

unsafe fn cs35l41_request_firmware_files_spkid(cs35l41: *mut cs35l41_hda, wmfw_firmware: *mut *const firmware, wmfw_filename: *mut *mut c_char, coeff_firmware: *mut *const firmware, coeff_filename: *mut *mut c_char) -> c_int {
    let mut ret = cs35l41_request_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, (*cs35l41).speaker_id, b"wmfw\0".as_ptr() as *const c_char);
    if ret == 0 {
        ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, (*cs35l41).speaker_id, b"bin\0".as_ptr() as *const c_char);
        if ret != 0 { release_firmware(*wmfw_firmware); kfree(*wmfw_filename as *const c_void); }
        return ret;
    }
    ret = cs35l41_request_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, -1, b"wmfw\0".as_ptr() as *const c_char);
    if ret == 0 {
        ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, (*cs35l41).speaker_id, b"bin\0".as_ptr() as *const c_char);
        if ret != 0 { release_firmware(*wmfw_firmware); kfree(*wmfw_filename as *const c_void); }
        return ret;
    }
    ret = cs35l41_request_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, (*cs35l41).acpi_subsystem_id, ptr::null(), (*cs35l41).speaker_id, b"wmfw\0".as_ptr() as *const c_char);
    if ret == 0 {
        ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, (*cs35l41).speaker_id, b"bin\0".as_ptr() as *const c_char);
        if ret != 0 {
            ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, ptr::null(), (*cs35l41).speaker_id, b"bin\0".as_ptr() as *const c_char);
        }
        if ret != 0 { release_firmware(*wmfw_firmware); kfree(*wmfw_filename as *const c_void); }
        return ret;
    }
    ret = cs35l41_request_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, (*cs35l41).acpi_subsystem_id, ptr::null(), -1, b"wmfw\0".as_ptr() as *const c_char);
    if ret == 0 {
        ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, (*cs35l41).speaker_id, b"bin\0".as_ptr() as *const c_char);
        if ret != 0 {
            ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, ptr::null(), (*cs35l41).speaker_id, b"bin\0".as_ptr() as *const c_char);
        }
        if ret != 0 { release_firmware(*wmfw_firmware); kfree(*wmfw_filename as *const c_void); }
    }
    ret
}

unsafe fn cs35l41_fallback_firmware_file(cs35l41: *mut cs35l41_hda, wmfw_firmware: *mut *const firmware, wmfw_filename: *mut *mut c_char, coeff_firmware: *mut *const firmware, coeff_filename: *mut *mut c_char) -> c_int {
    dev_warn((*cs35l41).dev, b"Falling back to default firmware.\n\0".as_ptr() as *const c_char);
    let mut ret = cs35l41_request_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, ptr::null(), ptr::null(), -1, b"wmfw\0".as_ptr() as *const c_char);
    if ret == 0 {
        ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, ptr::null(), ptr::null(), -1, b"bin\0".as_ptr() as *const c_char);
        if ret != 0 { release_firmware(*wmfw_firmware); kfree(*wmfw_filename as *const c_void); }
    }
    if ret != 0 {
        dev_warn((*cs35l41).dev, b"Unable to find firmware and tuning\n\0".as_ptr() as *const c_char);
    }
    ret
}

unsafe fn cs35l41_request_firmware_files(cs35l41: *mut cs35l41_hda, wmfw_firmware: *mut *const firmware, wmfw_filename: *mut *mut c_char, coeff_firmware: *mut *const firmware, coeff_filename: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    if (*cs35l41).speaker_id > -1 {
        ret = cs35l41_request_firmware_files_spkid(cs35l41, wmfw_firmware, wmfw_filename, coeff_firmware, coeff_filename);
        if ret != 0 {
            return cs35l41_fallback_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, coeff_firmware, coeff_filename);
        }
        return 0;
    }
    ret = cs35l41_request_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, -1, b"wmfw\0".as_ptr() as *const c_char);
    if ret == 0 {
        ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, -1, b"bin\0".as_ptr() as *const c_char);
        if ret != 0 { release_firmware(*wmfw_firmware); kfree(*wmfw_filename as *const c_void); }
        return if ret != 0 { cs35l41_fallback_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, coeff_firmware, coeff_filename) } else { 0 };
    }
    ret = cs35l41_request_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, (*cs35l41).acpi_subsystem_id, ptr::null(), -1, b"wmfw\0".as_ptr() as *const c_char);
    if ret == 0 {
        ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, (*cs35l41).amp_name, -1, b"bin\0".as_ptr() as *const c_char);
        if ret != 0 {
            ret = cs35l41_request_firmware_file(cs35l41, coeff_firmware, coeff_filename, (*cs35l41).acpi_subsystem_id, ptr::null(), -1, b"bin\0".as_ptr() as *const c_char);
        }
        if ret != 0 { release_firmware(*wmfw_firmware); kfree(*wmfw_filename as *const c_void); }
    }
    if ret != 0 { cs35l41_fallback_firmware_file(cs35l41, wmfw_firmware, wmfw_filename, coeff_firmware, coeff_filename) } else { 0 }
}

unsafe fn cs35l41_hda_apply_calibration(cs35l41: *mut cs35l41_hda) {
    if !(*cs35l41).cal_data_valid { return; }
    let ret = cs_amp_write_cal_coeffs(&mut (*cs35l41).cs_dsp, &cs35l41_calibration_controls, &(*cs35l41).cal_data);
    if ret < 0 {
        dev_warn((*cs35l41).dev, b"Failed to apply calibration: %d\n\0".as_ptr() as *const c_char, ret);
    } else {
        dev_info((*cs35l41).dev, b"Calibration applied: R0=%d\n\0".as_ptr() as *const c_char, (*cs35l41).cal_data.calR);
    }
}

unsafe fn cs35l41_read_silicon_uid(cs35l41: *mut cs35l41_hda, uid: *mut u64_) -> c_int {
    let mut tmp: c_uint = 0;
    let mut ret = regmap_read((*cs35l41).regmap, CS35L41_DIE_STS2, &mut tmp);
    if ret != 0 {
        dev_err((*cs35l41).dev, b"Cannot obtain CS35L41_DIE_STS2: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    *uid = (tmp as u64_) << 32;
    ret = regmap_read((*cs35l41).regmap, CS35L41_DIE_STS1, &mut tmp);
    if ret != 0 {
        dev_err((*cs35l41).dev, b"Cannot obtain CS35L41_DIE_STS1: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    *uid |= tmp as u64_;
    dev_dbg((*cs35l41).dev, b"UniqueID = %#llx\n\0".as_ptr() as *const c_char, *uid);
    0
}

unsafe fn cs35l41_get_calibration(cs35l41: *mut cs35l41_hda) -> c_int {
    let mut silicon_uid: u64_ = 0;
    let mut ret = cs35l41_read_silicon_uid(cs35l41, &mut silicon_uid);
    if ret < 0 { return ret; }
    ret = cs_amp_get_efi_calibration_data((*cs35l41).dev, silicon_uid, (*cs35l41).index, &mut (*cs35l41).cal_data);
    if ret == -ENODEV || ret == -EOVERFLOW { return 0; }
    if ret < 0 { return ret; }
    (*cs35l41).cal_data_valid = true;
    0
}

unsafe fn cs35l41_set_default_tuning_params(cs35l41: *mut cs35l41_hda) {
    (*cs35l41).tuning_gain = DEFAULT_AMP_GAIN_PCM;
}

unsafe fn cs35l41_read_tuning_params(cs35l41: *mut cs35l41_hda, fw: *const firmware) -> c_int {
    let params = (*fw).data as *const cs35l41_tuning_params;
    let size = le32_to_cpu((*params).size) as size_t;
    if size != (*fw).size {
        dev_err((*cs35l41).dev, b"Wrong Size for Tuning Param file. Expected %d got %zu\n\0".as_ptr() as *const c_char, le32_to_cpu((*params).size), (*fw).size);
        return -EINVAL;
    }
    if le32_to_cpu((*params).version) != 1 {
        dev_err((*cs35l41).dev, b"Unsupported Tuning Param Version: %d\n\0".as_ptr() as *const c_char, le32_to_cpu((*params).version));
        return -EINVAL;
    }
    if le32_to_cpu((*params).signature) != CS35L41_TUNING_SIG {
        dev_err((*cs35l41).dev, b"Mismatched Signature for Tuning Param file. Expected %#x got %#x\n\0".as_ptr() as *const c_char, CS35L41_TUNING_SIG, le32_to_cpu((*params).signature));
        return -EINVAL;
    }
    let mut offset: c_uint = 0;
    let end = ((*fw).size - size_of::<cs35l41_tuning_params>()) as c_uint;
    let mut i = 0;
    while i < le32_to_cpu((*params).num_entries) {
        if offset >= end || offset + size_of::<cs35l41_tuning_param_hdr>() as c_uint >= end { return -EFAULT; }
        let param = (*params).data.as_ptr().add(offset as usize) as *const cs35l41_tuning_param;
        offset = offset.wrapping_add(le32_to_cpu((*param).hdr.size));
        if offset > end { return -EFAULT; }
        match le32_to_cpu((*param).hdr.type_) {
            TUNING_PARAM_GAIN => {
                (*cs35l41).tuning_gain = le32_to_cpu((*param).u.gain);
                dev_dbg((*cs35l41).dev, b"Applying Gain: %d\n\0".as_ptr() as *const c_char, (*cs35l41).tuning_gain);
            }
            _ => {}
        }
        i += 1;
    }
    0
}

unsafe fn cs35l41_load_tuning_params(cs35l41: *mut cs35l41_hda, tuning_filename: *mut c_char) -> c_int {
    let mut tuning_param_file: *const firmware = ptr::null();
    let mut tuning_param_filename: *mut c_char = ptr::null_mut();
    let ret_req = cs35l41_request_tuning_param_file(cs35l41, tuning_filename, &mut tuning_param_file, &mut tuning_param_filename, (*cs35l41).acpi_subsystem_id);
    if ret_req != 0 {
        dev_dbg((*cs35l41).dev, b"Missing Tuning Param for file: %s: %d\n\0".as_ptr() as *const c_char, tuning_filename, ret_req);
        return 0;
    }
    let ret = cs35l41_read_tuning_params(cs35l41, tuning_param_file);
    if ret != 0 {
        dev_err((*cs35l41).dev, b"Error reading Tuning Params from file: %s: %d\n\0".as_ptr() as *const c_char, tuning_param_filename, ret);
        cs35l41_set_default_tuning_params(cs35l41);
    }
    release_firmware(tuning_param_file);
    kfree(tuning_param_filename as *const c_void);
    ret
}

unsafe fn cs35l41_init_dsp(cs35l41: *mut cs35l41_hda) -> c_int {
    let mut coeff_firmware: *const firmware = ptr::null();
    let mut wmfw_firmware: *const firmware = ptr::null();
    let dsp = &mut (*cs35l41).cs_dsp as *mut cs_dsp;
    let mut coeff_filename: *mut c_char = ptr::null_mut();
    let mut wmfw_filename: *mut c_char = ptr::null_mut();
    if !(*cs35l41).halo_initialized {
        cs35l41_configure_cs_dsp((*cs35l41).dev, (*cs35l41).regmap, dsp);
        (*dsp).client_ops = &client_ops;
        let ret = cs_dsp_halo_init(&mut (*cs35l41).cs_dsp);
        if ret != 0 { return ret; }
        (*cs35l41).halo_initialized = true;
    }
    cs35l41_set_default_tuning_params(cs35l41);
    let mut ret = cs35l41_request_firmware_files(cs35l41, &mut wmfw_firmware, &mut wmfw_filename, &mut coeff_firmware, &mut coeff_filename);
    if ret < 0 { return ret; }
    dev_dbg((*cs35l41).dev, b"Loading WMFW Firmware: %s\n\0".as_ptr() as *const c_char, wmfw_filename);
    if !coeff_filename.is_null() {
        dev_dbg((*cs35l41).dev, b"Loading Coefficient File: %s\n\0".as_ptr() as *const c_char, coeff_filename);
        ret = cs35l41_load_tuning_params(cs35l41, coeff_filename);
        if ret != 0 { dev_warn((*cs35l41).dev, b"Unable to load Tuning Parameters: %d\n\0".as_ptr() as *const c_char, ret); }
    } else {
        dev_warn((*cs35l41).dev, b"No Coefficient File available.\n\0".as_ptr() as *const c_char);
    }
    ret = cs_dsp_power_up(dsp, wmfw_firmware, wmfw_filename, coeff_firmware, coeff_filename, cs35l41_hda_fw_ids[(*cs35l41).firmware_type as usize]);
    if ret != 0 {
        cs35l41_set_default_tuning_params(cs35l41);
        return ret;
    }
    cs35l41_hda_apply_calibration(cs35l41);
    0
}

unsafe fn cs35l41_shutdown_dsp(cs35l41: *mut cs35l41_hda) {
    let dsp = &mut (*cs35l41).cs_dsp;
    cs35l41_set_default_tuning_params(cs35l41);
    cs_dsp_stop(dsp);
    cs_dsp_power_down(dsp);
    dev_dbg((*cs35l41).dev, b"Unloaded Firmware\n\0".as_ptr() as *const c_char);
}

unsafe fn cs35l41_remove_dsp(cs35l41: *mut cs35l41_hda) {
    cancel_work_sync(&mut (*cs35l41).fw_load_work);
    mutex_lock(&mut (*cs35l41).fw_mutex);
    cs35l41_shutdown_dsp(cs35l41);
    cs_dsp_remove(&mut (*cs35l41).cs_dsp);
    (*cs35l41).halo_initialized = false;
    mutex_unlock(&mut (*cs35l41).fw_mutex);
}

unsafe fn cs35l41_error_release(_dev: *mut device, map: *mut regmap, mask: c_uint) {
    regmap_write(map, CS35L41_PROTECT_REL_ERR_IGN, 0);
    regmap_set_bits(map, CS35L41_PROTECT_REL_ERR_IGN, mask);
    regmap_clear_bits(map, CS35L41_PROTECT_REL_ERR_IGN, mask);
}

unsafe fn cs35l41_irq_release(cs35l41: *mut cs35l41_hda) {
    cs35l41_error_release((*cs35l41).dev, (*cs35l41).regmap, (*cs35l41).irq_errors as c_uint);
    (*cs35l41).irq_errors = 0;
}

unsafe fn cs35l41_update_mixer(cs35l41: *mut cs35l41_hda) {
    let reg = (*cs35l41).regmap;
    let mut asp_en: c_uint = 0;
    let dsp1rx2_src: c_uint;
    regmap_multi_reg_write(reg, cs35l41_hda_config.as_ptr(), cs35l41_hda_config.len() as c_int);
    if (*cs35l41).cs_dsp.running {
        asp_en |= CS35L41_ASP_TX1_EN_MASK;
        regmap_multi_reg_write(reg, cs35l41_hda_config_dsp.as_ptr(), cs35l41_hda_config_dsp.len() as c_int);
        if (*cs35l41).hw_cfg.bst_type == CS35L41_INT_BOOST {
            regmap_write(reg, CS35L41_DSP1_RX5_SRC, CS35L41_INPUT_SRC_VPMON);
        } else {
            regmap_write(reg, CS35L41_DSP1_RX5_SRC, CS35L41_INPUT_SRC_VBSTMON);
        }
    } else {
        regmap_multi_reg_write(reg, cs35l41_hda_config_no_dsp.as_ptr(), cs35l41_hda_config_no_dsp.len() as c_int);
    }
    if (*cs35l41).hw_cfg.spk_pos == CS35L41_CENTER {
        asp_en |= CS35L41_ASP_RX2_EN_MASK;
        dsp1rx2_src = 0x00000009;
    } else {
        dsp1rx2_src = 0x00000008;
    }
    asp_en |= CS35L41_ASP_RX1_EN_MASK;
    regmap_write(reg, CS35L41_SP_ENABLES, asp_en);
    regmap_write(reg, CS35L41_DSP1_RX1_SRC, 0x00000008);
    regmap_write(reg, CS35L41_DSP1_RX2_SRC, dsp1rx2_src);
}

unsafe extern "C" fn cs35l41_hda_play_start(dev: *mut device) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    let reg = (*cs35l41).regmap;
    dev_dbg(dev, b"Play (Start)\n\0".as_ptr() as *const c_char);
    if (*cs35l41).playback_started {
        dev_dbg(dev, b"Playback already started.\0".as_ptr() as *const c_char);
        return;
    }
    (*cs35l41).playback_started = true;
    cs35l41_update_mixer(cs35l41);
    if (*cs35l41).cs_dsp.running {
        regmap_update_bits(reg, CS35L41_PWR_CTRL2, CS35L41_VMON_EN_MASK | CS35L41_IMON_EN_MASK, (1 << CS35L41_VMON_EN_SHIFT) | (1 << CS35L41_IMON_EN_SHIFT));
        cs35l41_set_cspl_mbox_cmd((*cs35l41).dev, reg, CSPL_MBOX_CMD_RESUME);
    }
    regmap_update_bits(reg, CS35L41_PWR_CTRL2, CS35L41_AMP_EN_MASK, 1 << CS35L41_AMP_EN_SHIFT);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST {
        regmap_write(reg, CS35L41_GPIO1_CTRL1, 0x00008001);
    }
}

unsafe extern "C" fn cs35l41_mute(dev: *mut device, mute: bool_) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    let reg = (*cs35l41).regmap;
    let mut amp_gain: c_uint;
    dev_dbg(dev, b"Mute(%d:%d) Playback Started: %d\n\0".as_ptr() as *const c_char, mute as c_int, (*cs35l41).mute_override as c_int, (*cs35l41).playback_started as c_int);
    if (*cs35l41).playback_started {
        if mute || (*cs35l41).mute_override {
            dev_dbg(dev, b"Muting\n\0".as_ptr() as *const c_char);
            regmap_multi_reg_write(reg, cs35l41_hda_mute.as_ptr(), cs35l41_hda_mute.len() as c_int);
        } else {
            dev_dbg(dev, b"Unmuting\n\0".as_ptr() as *const c_char);
            if (*cs35l41).cs_dsp.running {
                dev_dbg(dev, b"Using Tuned Gain: %d\n\0".as_ptr() as *const c_char, (*cs35l41).tuning_gain);
                amp_gain = ((*cs35l41).tuning_gain << CS35L41_AMP_GAIN_PCM_SHIFT) | (DEFAULT_AMP_GAIN_PDM << CS35L41_AMP_GAIN_PDM_SHIFT);
                regmap_write(reg, CS35L41_AMP_DIG_VOL_CTRL, 0x00008000);
                regmap_write(reg, CS35L41_AMP_GAIN_CTRL, amp_gain);
            } else {
                regmap_multi_reg_write(reg, cs35l41_hda_unmute.as_ptr(), cs35l41_hda_unmute.len() as c_int);
            }
        }
    }
}

unsafe extern "C" fn cs35l41_hda_play_done(dev: *mut device) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    dev_dbg(dev, b"Play (Complete)\n\0".as_ptr() as *const c_char);
    cs35l41_global_enable(dev, (*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type, 1, &mut (*cs35l41).cs_dsp);
    cs35l41_mute(dev, false);
}

unsafe extern "C" fn cs35l41_hda_pause_start(dev: *mut device) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    dev_dbg(dev, b"Pause (Start)\n\0".as_ptr() as *const c_char);
    cs35l41_mute(dev, true);
    cs35l41_global_enable(dev, (*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type, 0, &mut (*cs35l41).cs_dsp);
}

unsafe extern "C" fn cs35l41_hda_pause_done(dev: *mut device) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    let reg = (*cs35l41).regmap;
    dev_dbg(dev, b"Pause (Complete)\n\0".as_ptr() as *const c_char);
    regmap_update_bits(reg, CS35L41_PWR_CTRL2, CS35L41_AMP_EN_MASK, 0 << CS35L41_AMP_EN_SHIFT);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST { regmap_write(reg, CS35L41_GPIO1_CTRL1, 0x00000001); }
    if (*cs35l41).cs_dsp.running {
        cs35l41_set_cspl_mbox_cmd(dev, reg, CSPL_MBOX_CMD_PAUSE);
        regmap_update_bits(reg, CS35L41_PWR_CTRL2, CS35L41_VMON_EN_MASK | CS35L41_IMON_EN_MASK, (0 << CS35L41_VMON_EN_SHIFT) | (0 << CS35L41_IMON_EN_SHIFT));
    }
    cs35l41_irq_release(cs35l41);
    (*cs35l41).playback_started = false;
}

unsafe extern "C" fn cs35l41_hda_pre_playback_hook(dev: *mut device, action: c_int) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    if action == HDA_GEN_PCM_ACT_CLEANUP {
        mutex_lock(&mut (*cs35l41).fw_mutex);
        cs35l41_hda_pause_start(dev);
        mutex_unlock(&mut (*cs35l41).fw_mutex);
    }
}

unsafe extern "C" fn cs35l41_hda_playback_hook(dev: *mut device, action: c_int) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    if action == HDA_GEN_PCM_ACT_OPEN {
        pm_runtime_get_sync(dev);
    } else if action == HDA_GEN_PCM_ACT_PREPARE {
        mutex_lock(&mut (*cs35l41).fw_mutex);
        cs35l41_hda_play_start(dev);
        mutex_unlock(&mut (*cs35l41).fw_mutex);
    } else if action == HDA_GEN_PCM_ACT_CLEANUP {
        mutex_lock(&mut (*cs35l41).fw_mutex);
        cs35l41_hda_pause_done(dev);
        mutex_unlock(&mut (*cs35l41).fw_mutex);
    } else if action == HDA_GEN_PCM_ACT_CLOSE {
        mutex_lock(&mut (*cs35l41).fw_mutex);
        if !(*cs35l41).cs_dsp.running && (*cs35l41).request_fw_load && !(*cs35l41).fw_request_ongoing {
            dev_info(dev, b"Requesting Firmware Load after HDA_GEN_PCM_ACT_CLOSE\n\0".as_ptr() as *const c_char);
            (*cs35l41).fw_request_ongoing = true;
            schedule_work(&mut (*cs35l41).fw_load_work);
        }
        mutex_unlock(&mut (*cs35l41).fw_mutex);
        pm_runtime_put_autosuspend(dev);
    }
}

unsafe extern "C" fn cs35l41_hda_post_playback_hook(dev: *mut device, action: c_int) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    if action == HDA_GEN_PCM_ACT_PREPARE {
        mutex_lock(&mut (*cs35l41).fw_mutex);
        cs35l41_hda_play_done(dev);
        mutex_unlock(&mut (*cs35l41).fw_mutex);
    }
}

unsafe fn cs35l41_hda_channel_map(cs35l41: *mut cs35l41_hda) -> c_int {
    let tx_num: c_uint = 0;
    let tx_slot: *mut c_uint = ptr::null_mut();
    let rx_num: c_uint = 1;
    let mut mono: c_uint = 0;
    if (*cs35l41).amp_name.is_null() {
        if (*cs35l41).hw_cfg.spk_pos as usize >= channel_name.len() { return -EINVAL; }
        (*cs35l41).amp_name = devm_kasprintf((*cs35l41).dev, GFP_KERNEL, b"%c%d\0".as_ptr() as *const c_char, channel_name[(*cs35l41).hw_cfg.spk_pos as usize] as c_int, (*cs35l41).channel_index);
        if (*cs35l41).amp_name.is_null() { return -ENOMEM; }
    }
    let rx_slot = if (*cs35l41).hw_cfg.spk_pos == CS35L41_CENTER { &mut mono } else { &mut (*cs35l41).hw_cfg.spk_pos };
    cs35l41_set_channels((*cs35l41).dev, (*cs35l41).regmap, tx_num, tx_slot, rx_num, rx_slot)
}

unsafe fn cs35l41_verify_id(cs35l41: *mut cs35l41_hda, regid: *mut c_uint, reg_revid: *mut c_uint) -> c_int {
    let mut ret = regmap_read((*cs35l41).regmap, CS35L41_DEVID, regid);
    if ret != 0 { dev_err_probe((*cs35l41).dev, ret, b"Get Device ID failed\n\0".as_ptr() as *const c_char); return ret; }
    ret = regmap_read((*cs35l41).regmap, CS35L41_REVID, reg_revid);
    if ret != 0 { dev_err_probe((*cs35l41).dev, ret, b"Get Revision ID failed\n\0".as_ptr() as *const c_char); return ret; }
    let mtl_revid = *reg_revid & CS35L41_MTLREVID_MASK;
    let chipid = if (mtl_revid % 2) != 0 { CS35L41R_CHIP_ID } else { CS35L41_CHIP_ID };
    if *regid != chipid {
        dev_err((*cs35l41).dev, b"CS35L41 Device ID (%X). Expected ID %X\n\0".as_ptr() as *const c_char, *regid, chipid);
        return -ENODEV;
    }
    0
}

unsafe fn cs35l41_ready_for_reset(cs35l41: *mut cs35l41_hda) -> c_int {
    mutex_lock(&mut (*cs35l41).fw_mutex);
    if (*cs35l41).cs_dsp.running {
        (*cs35l41).cs_dsp.running = false;
        (*cs35l41).cs_dsp.booted = false;
    }
    regcache_mark_dirty((*cs35l41).regmap);
    mutex_unlock(&mut (*cs35l41).fw_mutex);
    0
}

unsafe extern "C" fn cs35l41_system_suspend_prep(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    dev_dbg((*cs35l41).dev, b"System Suspend Prepare\n\0".as_ptr() as *const c_char);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH {
        dev_err_once((*cs35l41).dev, b"System Suspend not supported\n\0".as_ptr() as *const c_char);
        return 0;
    }
    mutex_lock(&mut (*cs35l41).fw_mutex);
    if (*cs35l41).playback_started { cs35l41_hda_pause_start(dev); }
    mutex_unlock(&mut (*cs35l41).fw_mutex);
    0
}

unsafe extern "C" fn cs35l41_system_suspend(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    dev_dbg((*cs35l41).dev, b"System Suspend\n\0".as_ptr() as *const c_char);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH {
        dev_err_once((*cs35l41).dev, b"System Suspend not supported\n\0".as_ptr() as *const c_char);
        return 0;
    }
    mutex_lock(&mut (*cs35l41).fw_mutex);
    if (*cs35l41).playback_started { cs35l41_hda_pause_done(dev); }
    mutex_unlock(&mut (*cs35l41).fw_mutex);
    let mut ret = pm_runtime_force_suspend(dev);
    if ret != 0 { dev_err(dev, b"System Suspend Failed, unable to runtime suspend: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    ret = cs35l41_ready_for_reset(cs35l41);
    if ret != 0 { dev_err(dev, b"System Suspend Failed, not ready for Reset: %d\n\0".as_ptr() as *const c_char, ret); }
    if !(*cs35l41).reset_gpio.is_null() {
        dev_info((*cs35l41).dev, b"Asserting Reset\n\0".as_ptr() as *const c_char);
        gpiod_set_value_cansleep((*cs35l41).reset_gpio, 0);
        usleep_range(2000, 2100);
    }
    dev_dbg((*cs35l41).dev, b"System Suspended\n\0".as_ptr() as *const c_char);
    ret
}

unsafe fn cs35l41_wait_boot_done(cs35l41: *mut cs35l41_hda) -> c_int {
    let mut int_status: c_uint = 0;
    let mut ret: c_int;
    let mut waited = 0;
    loop {
        ret = regmap_read((*cs35l41).regmap, CS35L41_IRQ1_STATUS4, &mut int_status);
        if ret != 0 || (int_status & CS35L41_OTP_BOOT_DONE) != 0 || waited >= 100000 { break; }
        usleep_range(1000, 1000);
        waited += 1000;
    }
    if ret != 0 || (int_status & CS35L41_OTP_BOOT_DONE) == 0 {
        dev_err((*cs35l41).dev, b"Failed waiting for OTP_BOOT_DONE\n\0".as_ptr() as *const c_char);
        return if ret != 0 { ret } else { -EIO };
    }
    ret = regmap_read((*cs35l41).regmap, CS35L41_IRQ1_STATUS3, &mut int_status);
    if ret != 0 || (int_status & CS35L41_OTP_BOOT_ERR) != 0 {
        dev_err((*cs35l41).dev, b"OTP Boot status %x error\n\0".as_ptr() as *const c_char, int_status & CS35L41_OTP_BOOT_ERR);
        return if ret != 0 { ret } else { -EIO };
    }
    0
}

unsafe extern "C" fn cs35l41_system_resume(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    dev_dbg((*cs35l41).dev, b"System Resume\n\0".as_ptr() as *const c_char);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH {
        dev_err_once((*cs35l41).dev, b"System Resume not supported\n\0".as_ptr() as *const c_char);
        return 0;
    }
    if !(*cs35l41).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*cs35l41).reset_gpio, 0);
        usleep_range(2000, 2100);
        gpiod_set_value_cansleep((*cs35l41).reset_gpio, 1);
    }
    usleep_range(2000, 2100);
    regcache_cache_only((*cs35l41).regmap, false);
    regmap_write((*cs35l41).regmap, CS35L41_SFT_RESET, CS35L41_SOFTWARE_RESET);
    usleep_range(2000, 2100);
    let mut ret = cs35l41_wait_boot_done(cs35l41);
    if ret != 0 { return ret; }
    regcache_cache_only((*cs35l41).regmap, true);
    ret = pm_runtime_force_resume(dev);
    if ret != 0 { dev_err(dev, b"System Resume Failed: Unable to runtime resume: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    mutex_lock(&mut (*cs35l41).fw_mutex);
    if (*cs35l41).request_fw_load && !(*cs35l41).fw_request_ongoing {
        (*cs35l41).fw_request_ongoing = true;
        schedule_work(&mut (*cs35l41).fw_load_work);
    }
    mutex_unlock(&mut (*cs35l41).fw_mutex);
    ret
}

unsafe extern "C" fn cs35l41_runtime_idle(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH { return -EBUSY; }
    0
}

unsafe extern "C" fn cs35l41_runtime_suspend(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    dev_dbg((*cs35l41).dev, b"Runtime Suspend\n\0".as_ptr() as *const c_char);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH {
        dev_dbg((*cs35l41).dev, b"Runtime Suspend not supported\n\0".as_ptr() as *const c_char);
        return 0;
    }
    mutex_lock(&mut (*cs35l41).fw_mutex);
    let ret = if (*cs35l41).cs_dsp.running {
        cs35l41_enter_hibernate((*cs35l41).dev, (*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type)
    } else {
        cs35l41_safe_reset((*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type); 0
    };
    if ret == 0 {
        regcache_cache_only((*cs35l41).regmap, true);
        regcache_mark_dirty((*cs35l41).regmap);
    }
    mutex_unlock(&mut (*cs35l41).fw_mutex);
    ret
}

unsafe extern "C" fn cs35l41_runtime_resume(dev: *mut device) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    let mut regid: c_uint = 0;
    let mut reg_revid: c_uint = 0;
    dev_dbg((*cs35l41).dev, b"Runtime Resume\n\0".as_ptr() as *const c_char);
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH {
        dev_dbg((*cs35l41).dev, b"Runtime Resume not supported\n\0".as_ptr() as *const c_char);
        return 0;
    }
    mutex_lock(&mut (*cs35l41).fw_mutex);
    regcache_cache_only((*cs35l41).regmap, false);
    if (*cs35l41).cs_dsp.running {
        let ret = cs35l41_exit_hibernate((*cs35l41).dev, (*cs35l41).regmap);
        if ret != 0 { dev_warn((*cs35l41).dev, b"Unable to exit Hibernate.\0".as_ptr() as *const c_char); mutex_unlock(&mut (*cs35l41).fw_mutex); return ret; }
    }
    let mut ret = cs35l41_verify_id(cs35l41, &mut regid, &mut reg_revid);
    if ret != 0 { mutex_unlock(&mut (*cs35l41).fw_mutex); return ret; }
    cs35l41_test_key_unlock((*cs35l41).dev, (*cs35l41).regmap);
    ret = regcache_sync((*cs35l41).regmap);
    cs35l41_test_key_lock((*cs35l41).dev, (*cs35l41).regmap);
    if ret != 0 {
        dev_err((*cs35l41).dev, b"Failed to restore register cache: %d\n\0".as_ptr() as *const c_char, ret);
        mutex_unlock(&mut (*cs35l41).fw_mutex);
        return ret;
    }
    if (*cs35l41).hw_cfg.bst_type == CS35L41_EXT_BOOST {
        cs35l41_init_boost((*cs35l41).dev, (*cs35l41).regmap, &mut (*cs35l41).hw_cfg);
    }
    dev_dbg((*cs35l41).dev, b"CS35L41 Resumed (%x), Revision: %02X\n\0".as_ptr() as *const c_char, regid, reg_revid);
    mutex_unlock(&mut (*cs35l41).fw_mutex);
    0
}

unsafe fn cs35l41_hda_read_ctl(dsp: *mut cs_dsp, name: *const c_char, type_: c_int, alg: c_uint, buf: *mut c_void, len: size_t) -> c_int {
    mutex_lock(&mut (*dsp).pwr_lock);
    let ret = cs_dsp_coeff_read_ctrl(cs_dsp_get_ctl(dsp, name, type_, alg), 0, buf, len);
    mutex_unlock(&mut (*dsp).pwr_lock);
    ret
}

unsafe fn cs35l41_smart_amp(cs35l41: *mut cs35l41_hda) -> c_int {
    let mut fw_status: c_uint = 0;
    let mut halo_sts: __be32 = 0;
    if (*cs35l41).bypass_fw {
        dev_warn((*cs35l41).dev, b"Bypassing Firmware.\n\0".as_ptr() as *const c_char);
        return 0;
    }
    let mut ret = cs35l41_init_dsp(cs35l41);
    if ret != 0 { dev_warn((*cs35l41).dev, b"Cannot Initialize Firmware. Error: %d\n\0".as_ptr() as *const c_char, ret); cs35l41_shutdown_dsp(cs35l41); return ret; }
    ret = cs35l41_write_fs_errata((*cs35l41).dev, (*cs35l41).regmap);
    if ret != 0 { dev_err((*cs35l41).dev, b"Cannot Write FS Errata: %d\n\0".as_ptr() as *const c_char, ret); cs35l41_shutdown_dsp(cs35l41); return ret; }
    ret = cs_dsp_run(&mut (*cs35l41).cs_dsp);
    if ret != 0 { dev_err((*cs35l41).dev, b"Fail to start dsp: %d\n\0".as_ptr() as *const c_char, ret); cs35l41_shutdown_dsp(cs35l41); return ret; }
    let mut waited = 0;
    while waited < 15000 {
        ret = cs35l41_hda_read_ctl(&mut (*cs35l41).cs_dsp, HALO_STATE_DSP_CTL_NAME, HALO_STATE_DSP_CTL_TYPE, HALO_STATE_DSP_CTL_ALG, &mut halo_sts as *mut _ as *mut c_void, size_of::<__be32>());
        if ret == 0 && be32_to_cpu(halo_sts) == HALO_STATE_CODE_RUN { break; }
        usleep_range(1000, 1000);
        waited += 1000;
    }
    if ret != 0 || be32_to_cpu(halo_sts) != HALO_STATE_CODE_RUN {
        dev_err((*cs35l41).dev, b"Timeout waiting for HALO Core to start. State: %u\n\0".as_ptr() as *const c_char, halo_sts);
        cs35l41_shutdown_dsp(cs35l41);
        return if ret != 0 { ret } else { -EIO };
    }
    ret = regmap_read((*cs35l41).regmap, CS35L41_DSP_MBOX_2, &mut fw_status);
    if ret < 0 { dev_err((*cs35l41).dev, b"Failed to read firmware status: %d\n\0".as_ptr() as *const c_char, ret); cs35l41_shutdown_dsp(cs35l41); return ret; }
    if fw_status != CSPL_MBOX_STS_RUNNING && fw_status != CSPL_MBOX_STS_PAUSED {
        dev_err((*cs35l41).dev, b"Firmware status is invalid: %u\n\0".as_ptr() as *const c_char, fw_status);
        cs35l41_shutdown_dsp(cs35l41);
        return -EINVAL;
    }
    ret = cs35l41_set_cspl_mbox_cmd((*cs35l41).dev, (*cs35l41).regmap, CSPL_MBOX_CMD_PAUSE);
    if ret != 0 { dev_err((*cs35l41).dev, b"Error waiting for DSP to pause: %u\n\0".as_ptr() as *const c_char, ret); cs35l41_shutdown_dsp(cs35l41); return ret; }
    dev_info((*cs35l41).dev, b"Firmware Loaded - Type: %s, Gain: %d\n\0".as_ptr() as *const c_char, cs35l41_hda_fw_ids[(*cs35l41).firmware_type as usize], (*cs35l41).tuning_gain);
    0
}

unsafe fn cs35l41_load_firmware(cs35l41: *mut cs35l41_hda, load: bool_) {
    if (*cs35l41).cs_dsp.running && !load {
        dev_dbg((*cs35l41).dev, b"Unloading Firmware\n\0".as_ptr() as *const c_char);
        cs35l41_shutdown_dsp(cs35l41);
    } else if !(*cs35l41).cs_dsp.running && load {
        dev_dbg((*cs35l41).dev, b"Loading Firmware\n\0".as_ptr() as *const c_char);
        cs35l41_smart_amp(cs35l41);
    } else {
        dev_dbg((*cs35l41).dev, b"Unable to Load firmware.\n\0".as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn cs35l41_fw_load_ctl_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cs35l41 = snd_kcontrol_chip(kcontrol) as *mut cs35l41_hda;
    (*ucontrol).value.integer.value[0] = (*cs35l41).request_fw_load as i64;
    0
}
unsafe extern "C" fn cs35l41_mute_override_ctl_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cs35l41 = snd_kcontrol_chip(kcontrol) as *mut cs35l41_hda;
    (*ucontrol).value.integer.value[0] = (*cs35l41).mute_override as i64;
    0
}
unsafe extern "C" fn cs35l41_fw_load_work(work: *mut work_struct) {
    let cs35l41 = (work as *mut u8).sub(0) as *mut cs35l41_hda; // container_of(..., fw_load_work); offset supplied by translated layout.
    mutex_lock(&mut (*cs35l41).fw_mutex);
    if (*cs35l41).playback_started {
        dev_err((*cs35l41).dev, b"Cannot Load/Unload firmware during Playback. Retrying...\n\0".as_ptr() as *const c_char);
    } else {
        cs35l41_load_firmware(cs35l41, (*cs35l41).request_fw_load);
    }
    (*cs35l41).fw_request_ongoing = false;
    mutex_unlock(&mut (*cs35l41).fw_mutex);
}

unsafe extern "C" fn cs35l41_fw_load_ctl_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cs35l41 = snd_kcontrol_chip(kcontrol) as *mut cs35l41_hda;
    if (*cs35l41).request_fw_load as i64 == (*ucontrol).value.integer.value[0] { return 0; }
    if (*cs35l41).fw_request_ongoing { dev_dbg((*cs35l41).dev, b"Existing request not complete\n\0".as_ptr() as *const c_char); return -EBUSY; }
    if (*cs35l41).playback_started { dev_err((*cs35l41).dev, b"Cannot Load/Unload firmware during Playback\n\0".as_ptr() as *const c_char); return -EBUSY; }
    (*cs35l41).fw_request_ongoing = true;
    (*cs35l41).request_fw_load = (*ucontrol).value.integer.value[0] != 0;
    schedule_work(&mut (*cs35l41).fw_load_work);
    1
}
unsafe extern "C" fn cs35l41_fw_type_ctl_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cs35l41 = snd_kcontrol_chip(kcontrol) as *mut cs35l41_hda;
    (*ucontrol).value.enumerated.item[0] = (*cs35l41).firmware_type as c_uint;
    0
}
unsafe extern "C" fn cs35l41_fw_type_ctl_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cs35l41 = snd_kcontrol_chip(kcontrol) as *mut cs35l41_hda;
    let item = (*ucontrol).value.enumerated.item[0] as usize;
    if item < CS35L41_HDA_NUM_FW {
        if (*cs35l41).firmware_type as usize != item {
            (*cs35l41).firmware_type = item as c_int;
            return 1;
        }
        return 0;
    }
    -EINVAL
}
unsafe extern "C" fn cs35l41_fw_type_ctl_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_enum_info(uinfo, 1, cs35l41_hda_fw_ids.len() as c_uint, cs35l41_hda_fw_ids.as_ptr())
}

unsafe fn cs35l41_remove_controls(cs35l41: *mut cs35l41_hda) {
    if (*cs35l41).codec.is_null() { return; }
    snd_ctl_remove((*(*cs35l41).codec).card, (*cs35l41).mute_override_ctl); (*cs35l41).mute_override_ctl = ptr::null_mut();
    snd_ctl_remove((*(*cs35l41).codec).card, (*cs35l41).fw_load_ctl); (*cs35l41).fw_load_ctl = ptr::null_mut();
    snd_ctl_remove((*(*cs35l41).codec).card, (*cs35l41).fw_type_ctl); (*cs35l41).fw_type_ctl = ptr::null_mut();
}

unsafe fn cs35l41_add_control(cs35l41: *mut cs35l41_hda, ctl: *mut snd_kcontrol_new, kctl: *mut *mut snd_kcontrol) -> c_int {
    *kctl = snd_ctl_new1(ctl, cs35l41 as *mut c_void);
    if (*kctl).is_null() { return -ENOMEM; }
    let ret = snd_ctl_add((*(*cs35l41).codec).card, *kctl);
    if ret != 0 {
        dev_err((*cs35l41).dev, b"Failed to add KControl %s = %d\n\0".as_ptr() as *const c_char, (*ctl).name, ret);
        *kctl = ptr::null_mut();
        return ret;
    }
    dev_dbg((*cs35l41).dev, b"Added Control %s\n\0".as_ptr() as *const c_char, (*ctl).name);
    0
}

unsafe fn cs35l41_create_controls(cs35l41: *mut cs35l41_hda) -> c_int {
    let mut fw_type_ctl_name = [0 as c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
    let mut fw_load_ctl_name = [0 as c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
    let mut mute_override_ctl_name = [0 as c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
    scnprintf(fw_type_ctl_name.as_mut_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN, b"%s DSP1 Firmware Type\0".as_ptr() as *const c_char, (*cs35l41).amp_name);
    scnprintf(fw_load_ctl_name.as_mut_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN, b"%s DSP1 Firmware Load\0".as_ptr() as *const c_char, (*cs35l41).amp_name);
    scnprintf(mute_override_ctl_name.as_mut_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN, b"%s Forced Mute Status\0".as_ptr() as *const c_char, (*cs35l41).amp_name);
    let mut fw_type_ctl = snd_kcontrol_new { name: fw_type_ctl_name.as_ptr(), iface: SNDRV_CTL_ELEM_IFACE_CARD, info: Some(cs35l41_fw_type_ctl_info), get: Some(cs35l41_fw_type_ctl_get), put: Some(cs35l41_fw_type_ctl_put), access: 0 };
    let mut fw_load_ctl = snd_kcontrol_new { name: fw_load_ctl_name.as_ptr(), iface: SNDRV_CTL_ELEM_IFACE_CARD, info: None, get: Some(cs35l41_fw_load_ctl_get), put: Some(cs35l41_fw_load_ctl_put), access: 0 };
    let mut mute_override_ctl = snd_kcontrol_new { name: mute_override_ctl_name.as_ptr(), iface: SNDRV_CTL_ELEM_IFACE_CARD, info: None, get: Some(cs35l41_mute_override_ctl_get), put: None, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE };
    let mut ret = cs35l41_add_control(cs35l41, &mut fw_type_ctl, &mut (*cs35l41).fw_type_ctl);
    if ret == 0 { ret = cs35l41_add_control(cs35l41, &mut fw_load_ctl, &mut (*cs35l41).fw_load_ctl); }
    if ret == 0 { ret = cs35l41_add_control(cs35l41, &mut mute_override_ctl, &mut (*cs35l41).mute_override_ctl); }
    if ret != 0 { cs35l41_remove_controls(cs35l41); }
    ret
}

unsafe fn cs35l41_dsm_supported(handle: acpi_handle, commands: c_uint) -> bool_ {
    let mut guid = guid_t { b: [0; 16] };
    guid_parse(CS35L41_UUID, &mut guid);
    acpi_check_dsm(handle, &guid, 0, BIT(commands))
}

unsafe fn cs35l41_get_acpi_mute_state(cs35l41: *mut cs35l41_hda, handle: acpi_handle) -> c_int {
    let mut guid = guid_t { b: [0; 16] };
    let mut mute = -ENODEV;
    guid_parse(CS35L41_UUID, &mut guid);
    if cs35l41_dsm_supported(handle, CS35L41_DSM_GET_MUTE) {
        let ret = acpi_evaluate_dsm_typed(handle, &guid, 0, CS35L41_DSM_GET_MUTE, ptr::null_mut(), ACPI_TYPE_BUFFER);
        if ret.is_null() { return -EINVAL; }
        if (*ret).u.buffer.length == 0 || (*ret).u.buffer.pointer.is_null() {
            ACPI_FREE(ret as *mut c_void);
            return -EINVAL;
        }
        mute = *(*ret).u.buffer.pointer as c_int;
        ACPI_FREE(ret as *mut c_void);
        dev_dbg((*cs35l41).dev, b"CS35L41_DSM_GET_MUTE: %d\n\0".as_ptr() as *const c_char, mute);
    }
    dev_dbg((*cs35l41).dev, b"%s: %d\n\0".as_ptr() as *const c_char, b"cs35l41_get_acpi_mute_state\0".as_ptr() as *const c_char, mute);
    mute
}

unsafe extern "C" fn cs35l41_acpi_device_notify(handle: acpi_handle, event: u32_, dev: *mut device) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    if event != CS35L41_NOTIFY_EVENT { return; }
    let mute = cs35l41_get_acpi_mute_state(cs35l41, handle);
    if mute < 0 {
        dev_warn((*cs35l41).dev, b"Unable to retrieve mute state: %d\n\0".as_ptr() as *const c_char, mute);
        return;
    }
    dev_dbg((*cs35l41).dev, b"Requesting mute value: %d\n\0".as_ptr() as *const c_char, mute);
    (*cs35l41).mute_override = mute > 0;
    cs35l41_mute((*cs35l41).dev, (*cs35l41).mute_override);
}

unsafe extern "C" fn cs35l41_hda_bind(dev: *mut device, _master: *mut device, master_data: *mut c_void) -> c_int {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    let parent = master_data as *mut hda_component_parent;
    let comp = hda_component_from_index(parent, (*cs35l41).index);
    if comp.is_null() { return -EINVAL; }
    if !(*comp).dev.is_null() { return -EBUSY; }
    mutex_lock(&mut (*cs35l41).fw_mutex);
    (*comp).dev = dev;
    (*cs35l41).codec = (*parent).codec;
    if (*cs35l41).acpi_subsystem_id.is_null() {
        (*cs35l41).acpi_subsystem_id = kasprintf(GFP_KERNEL, b"%.8x\0".as_ptr() as *const c_char, (*(*cs35l41).codec).core.subsystem_id);
    }
    strscpy((*comp).name.as_mut_ptr(), dev_name(dev), (*comp).name.len());
    (*cs35l41).firmware_type = CS35L41_HDA_FW_SPK_PROT as c_int;
    if firmware_autostart {
        dev_dbg((*cs35l41).dev, b"Firmware Autostart.\n\0".as_ptr() as *const c_char);
        (*cs35l41).request_fw_load = true;
        if cs35l41_smart_amp(cs35l41) < 0 { dev_warn((*cs35l41).dev, b"Cannot Run Firmware, reverting to dsp bypass...\n\0".as_ptr() as *const c_char); }
    } else {
        dev_dbg((*cs35l41).dev, b"Firmware Autostart is disabled.\n\0".as_ptr() as *const c_char);
    }
    let ret = cs35l41_create_controls(cs35l41);
    (*comp).playback_hook = Some(cs35l41_hda_playback_hook);
    (*comp).pre_playback_hook = Some(cs35l41_hda_pre_playback_hook);
    (*comp).post_playback_hook = Some(cs35l41_hda_post_playback_hook);
    (*comp).acpi_notify = Some(cs35l41_acpi_device_notify);
    (*comp).adev = (*cs35l41).dacpi;
    (*comp).acpi_notifications_supported = cs35l41_dsm_supported(acpi_device_handle((*comp).adev), CS35L41_DSM_GET_MUTE);
    (*cs35l41).mute_override = cs35l41_get_acpi_mute_state(cs35l41, acpi_device_handle((*cs35l41).dacpi)) > 0;
    mutex_unlock(&mut (*cs35l41).fw_mutex);
    let sleep_flags = lock_system_sleep();
    if device_link_add(&mut (*(*cs35l41).codec).core.dev, (*cs35l41).dev, DL_FLAG_STATELESS).is_null() {
        dev_warn(dev, b"Unable to create device link\n\0".as_ptr() as *const c_char);
    }
    unlock_system_sleep(sleep_flags);
    dev_info((*cs35l41).dev, b"CS35L41 Bound - SSID: %s, BST: %d, VSPK: %d, CH: %c, FW EN: %d, SPKID: %d\n\0".as_ptr() as *const c_char, (*cs35l41).acpi_subsystem_id, (*cs35l41).hw_cfg.bst_type, ((*cs35l41).hw_cfg.gpio1.func == CS35l41_VSPK_SWITCH) as c_int, channel_name[(*cs35l41).hw_cfg.spk_pos as usize] as c_int, (*cs35l41).cs_dsp.running as c_int, (*cs35l41).speaker_id);
    ret
}

unsafe extern "C" fn cs35l41_hda_unbind(dev: *mut device, _master: *mut device, master_data: *mut c_void) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    let parent = master_data as *mut hda_component_parent;
    let comp = hda_component_from_index(parent, (*cs35l41).index);
    if comp.is_null() { return; }
    if (*comp).dev == dev {
        let sleep_flags = lock_system_sleep();
        device_link_remove(&mut (*(*cs35l41).codec).core.dev, (*cs35l41).dev);
        unlock_system_sleep(sleep_flags);
        ptr::write_bytes(comp as *mut u8, 0, size_of::<hda_component>());
        cs35l41_remove_controls(cs35l41);
        cancel_work_sync(&mut (*cs35l41).fw_load_work);
        (*cs35l41).codec = ptr::null_mut();
    }
}

static cs35l41_hda_comp_ops: component_ops = component_ops {
    bind: Some(cs35l41_hda_bind),
    unbind: Some(cs35l41_hda_unbind),
};

unsafe extern "C" fn cs35l41_bst_short_err(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l41 = data as *mut cs35l41_hda;
    dev_crit_ratelimited((*cs35l41).dev, b"LBST Error\n\0".as_ptr() as *const c_char);
    set_bit(CS35L41_BST_SHORT_ERR_RLS_SHIFT, &mut (*cs35l41).irq_errors);
    IRQ_HANDLED
}
unsafe extern "C" fn cs35l41_bst_dcm_uvp_err(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l41 = data as *mut cs35l41_hda;
    dev_crit_ratelimited((*cs35l41).dev, b"DCM VBST Under Voltage Error\n\0".as_ptr() as *const c_char);
    set_bit(CS35L41_BST_UVP_ERR_RLS_SHIFT, &mut (*cs35l41).irq_errors);
    IRQ_HANDLED
}
unsafe extern "C" fn cs35l41_bst_ovp_err(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l41 = data as *mut cs35l41_hda;
    dev_crit_ratelimited((*cs35l41).dev, b"VBST Over Voltage error\n\0".as_ptr() as *const c_char);
    set_bit(CS35L41_BST_OVP_ERR_RLS_SHIFT, &mut (*cs35l41).irq_errors);
    IRQ_HANDLED
}
unsafe extern "C" fn cs35l41_temp_err(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l41 = data as *mut cs35l41_hda;
    dev_crit_ratelimited((*cs35l41).dev, b"Over temperature error\n\0".as_ptr() as *const c_char);
    set_bit(CS35L41_TEMP_ERR_RLS_SHIFT, &mut (*cs35l41).irq_errors);
    IRQ_HANDLED
}
unsafe extern "C" fn cs35l41_temp_warn(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l41 = data as *mut cs35l41_hda;
    dev_crit_ratelimited((*cs35l41).dev, b"Over temperature warning\n\0".as_ptr() as *const c_char);
    set_bit(CS35L41_TEMP_WARN_ERR_RLS_SHIFT, &mut (*cs35l41).irq_errors);
    IRQ_HANDLED
}
unsafe extern "C" fn cs35l41_amp_short(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l41 = data as *mut cs35l41_hda;
    dev_crit_ratelimited((*cs35l41).dev, b"Amp short error\n\0".as_ptr() as *const c_char);
    set_bit(CS35L41_AMP_SHORT_ERR_RLS_SHIFT, &mut (*cs35l41).irq_errors);
    IRQ_HANDLED
}

static cs35l41_irqs: [cs35l41_irq; 6] = unsafe {
    [
        cs35l41_irq { irq: 0, name: b"Boost Overvoltage Error\0".as_ptr() as *const c_char, handler: Some(cs35l41_bst_ovp_err) },
        cs35l41_irq { irq: 1, name: b"Boost Undervoltage Error\0".as_ptr() as *const c_char, handler: Some(cs35l41_bst_dcm_uvp_err) },
        cs35l41_irq { irq: 2, name: b"Boost Inductor Short Error\0".as_ptr() as *const c_char, handler: Some(cs35l41_bst_short_err) },
        cs35l41_irq { irq: 3, name: b"Temperature Warning\0".as_ptr() as *const c_char, handler: Some(cs35l41_temp_warn) },
        cs35l41_irq { irq: 4, name: b"Temperature Error\0".as_ptr() as *const c_char, handler: Some(cs35l41_temp_err) },
        cs35l41_irq { irq: 5, name: b"Amp Short\0".as_ptr() as *const c_char, handler: Some(cs35l41_amp_short) },
    ]
};

static cs35l41_reg_irqs: [regmap_irq; 6] = [
    regmap_irq { reg_offset: 0, mask: 0 },
    regmap_irq { reg_offset: 0, mask: 0 },
    regmap_irq { reg_offset: 0, mask: 0 },
    regmap_irq { reg_offset: 0, mask: 0 },
    regmap_irq { reg_offset: 0, mask: 0 },
    regmap_irq { reg_offset: 0, mask: 0 },
];

static cs35l41_regmap_irq_chip: regmap_irq_chip = unsafe {
    regmap_irq_chip {
        name: b"cs35l41 IRQ1 Controller\0".as_ptr() as *const c_char,
        status_base: CS35L41_IRQ1_STATUS1,
        mask_base: CS35L41_IRQ1_MASK1,
        ack_base: CS35L41_IRQ1_STATUS1,
        num_regs: 4,
        irqs: cs35l41_reg_irqs.as_ptr(),
        num_irqs: cs35l41_reg_irqs.len() as c_int,
        runtime_pm: true,
    }
};

unsafe fn cs35l41_configure_interrupt(cs35l41: *mut cs35l41_hda, irq_pol: c_int) {
    if (*cs35l41).irq == 0 {
        dev_warn((*cs35l41).dev, b"No Interrupt Found\0".as_ptr() as *const c_char);
        dev_warn((*cs35l41).dev, b"IRQ Config Failed. Amp errors may not be recoverable without reboot.\0".as_ptr() as *const c_char);
        return;
    }
    let mut ret = devm_regmap_add_irq_chip((*cs35l41).dev, (*cs35l41).regmap, (*cs35l41).irq, IRQF_ONESHOT | IRQF_SHARED | irq_pol, 0, &cs35l41_regmap_irq_chip, &mut (*cs35l41).irq_data);
    if ret != 0 {
        dev_dbg((*cs35l41).dev, b"Unable to add IRQ Chip: %d.\0".as_ptr() as *const c_char, ret);
        dev_warn((*cs35l41).dev, b"IRQ Config Failed. Amp errors may not be recoverable without reboot.\0".as_ptr() as *const c_char);
        return;
    }
    for irqdesc in cs35l41_irqs.iter() {
        let irq = regmap_irq_get_virq((*cs35l41).irq_data, irqdesc.irq);
        if irq < 0 {
            ret = irq;
            dev_dbg((*cs35l41).dev, b"Unable to map IRQ %s: %d.\0".as_ptr() as *const c_char, irqdesc.name, ret);
            dev_warn((*cs35l41).dev, b"IRQ Config Failed. Amp errors may not be recoverable without reboot.\0".as_ptr() as *const c_char);
            return;
        }
        ret = devm_request_threaded_irq((*cs35l41).dev, irq, ptr::null_mut(), irqdesc.handler, IRQF_ONESHOT | IRQF_SHARED | irq_pol, irqdesc.name, cs35l41 as *mut c_void);
        if ret != 0 {
            dev_dbg((*cs35l41).dev, b"Unable to allocate IRQ %s:: %d.\0".as_ptr() as *const c_char, irqdesc.name, ret);
            dev_warn((*cs35l41).dev, b"IRQ Config Failed. Amp errors may not be recoverable without reboot.\0".as_ptr() as *const c_char);
            return;
        }
    }
}

unsafe fn cs35l41_hda_apply_properties(cs35l41: *mut cs35l41_hda) -> c_int {
    let hw_cfg = &mut (*cs35l41).hw_cfg as *mut cs35l41_hw_cfg;
    let mut using_irq = false;
    if !(*cs35l41).hw_cfg.valid { return -EINVAL; }
    let ret = cs35l41_init_boost((*cs35l41).dev, (*cs35l41).regmap, hw_cfg);
    if ret != 0 { return ret; }
    if (*hw_cfg).gpio1.valid {
        if (*hw_cfg).gpio1.func == CS35L41_NOT_USED {
        } else if (*hw_cfg).gpio1.func == CS35l41_VSPK_SWITCH {
            (*hw_cfg).gpio1.func = CS35L41_GPIO1_GPIO;
            (*hw_cfg).gpio1.out_en = true;
        } else if (*hw_cfg).gpio1.func == CS35l41_SYNC {
            (*hw_cfg).gpio1.func = CS35L41_GPIO1_MDSYNC;
        } else {
            dev_err((*cs35l41).dev, b"Invalid function %d for GPIO1\n\0".as_ptr() as *const c_char, (*hw_cfg).gpio1.func);
            return -EINVAL;
        }
    }
    if (*hw_cfg).gpio2.valid {
        if (*hw_cfg).gpio2.func == CS35L41_NOT_USED {
        } else if (*hw_cfg).gpio2.func == CS35L41_INTERRUPT {
            using_irq = true;
            (*hw_cfg).gpio2.func = CS35L41_GPIO2_INT_OPEN_DRAIN;
        } else {
            dev_err((*cs35l41).dev, b"Invalid GPIO2 function %d\n\0".as_ptr() as *const c_char, (*hw_cfg).gpio2.func);
            return -EINVAL;
        }
    }
    let irq_pol = cs35l41_gpio_config((*cs35l41).regmap, hw_cfg);
    if using_irq { cs35l41_configure_interrupt(cs35l41, irq_pol); }
    cs35l41_hda_channel_map(cs35l41)
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_get_speaker_id(dev: *mut device, amp_index: c_int, num_amps: c_int, fixed_gpio_id: c_int) -> c_int {
    let mut speaker_id = -ENODEV;
    if fixed_gpio_id >= 0 {
        dev_dbg(dev, b"Found Fixed Speaker ID GPIO (index = %d)\n\0".as_ptr() as *const c_char, fixed_gpio_id);
        let desc = gpiod_get_index(dev, ptr::null(), fixed_gpio_id as c_uint, GPIOD_IN);
        if IS_ERR(desc) { return PTR_ERR(desc); }
        speaker_id = gpiod_get_value_cansleep(desc);
        gpiod_put(desc);
        dev_dbg(dev, b"Speaker ID = %d\n\0".as_ptr() as *const c_char, speaker_id);
    } else {
        let count = gpiod_count(dev, b"spk-id\0".as_ptr() as *const c_char);
        if count > 0 {
            speaker_id = 0;
            let gpios_per_amp = count / num_amps;
            let base_index = gpios_per_amp * amp_index;
            if count % num_amps != 0 { return -EINVAL; }
            dev_dbg(dev, b"Found %d Speaker ID GPIOs per Amp\n\0".as_ptr() as *const c_char, gpios_per_amp);
            let mut i = 0;
            while i < gpios_per_amp {
                let desc = gpiod_get_index(dev, b"spk-id\0".as_ptr() as *const c_char, (i + base_index) as c_uint, GPIOD_IN);
                if IS_ERR(desc) { speaker_id = PTR_ERR(desc); break; }
                let tmp = gpiod_get_value_cansleep(desc);
                gpiod_put(desc);
                if tmp < 0 { speaker_id = tmp; break; }
                speaker_id |= tmp << i;
                i += 1;
            }
            dev_dbg(dev, b"Speaker ID = %d\n\0".as_ptr() as *const c_char, speaker_id);
        }
    }
    speaker_id
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_hda_parse_acpi(cs35l41: *mut cs35l41_hda, physdev: *mut device, id: c_int) -> c_int {
    let hw_cfg = &mut (*cs35l41).hw_cfg as *mut cs35l41_hw_cfg;
    let mut values = [0u32; HDA_MAX_COMPONENTS];
    let mut property = b"cirrus,dev-index\0".as_ptr() as *const c_char;
    let mut ret = device_property_count_u32(physdev, property);
    if ret <= 0 { return cs35l41_hda_parse_acpi_err(cs35l41, hw_cfg, property, ret); }
    if ret as usize > values.len() { return cs35l41_hda_parse_acpi_err(cs35l41, hw_cfg, property, -EINVAL); }
    let nval = ret as size_t;
    ret = device_property_read_u32_array(physdev, property, values.as_mut_ptr(), nval);
    if ret != 0 { return cs35l41_hda_parse_acpi_err(cs35l41, hw_cfg, property, ret); }
    (*cs35l41).index = -1;
    for i in 0..nval {
        if values[i] == id as u32 { (*cs35l41).index = i as c_int; break; }
    }
    if (*cs35l41).index == -1 {
        dev_err((*cs35l41).dev, b"No index found in %s\n\0".as_ptr() as *const c_char, property);
        return cs35l41_hda_parse_acpi_err(cs35l41, hw_cfg, property, -ENODEV);
    }
    (*cs35l41).reset_gpio = fwnode_gpiod_get_index(acpi_fwnode_handle((*cs35l41).dacpi), b"reset\0".as_ptr() as *const c_char, (*cs35l41).index as c_uint, GPIOD_OUT_LOW, b"cs35l41-reset\0".as_ptr() as *const c_char);
    property = b"cirrus,speaker-position\0".as_ptr() as *const c_char;
    ret = device_property_read_u32_array(physdev, property, values.as_mut_ptr(), nval);
    if ret != 0 { return cs35l41_hda_parse_acpi_err(cs35l41, hw_cfg, property, ret); }
    (*hw_cfg).spk_pos = values[(*cs35l41).index as usize];
    (*cs35l41).channel_index = 0;
    for i in 0..(*cs35l41).index as usize {
        if values[i] == (*hw_cfg).spk_pos { (*cs35l41).channel_index += 1; }
    }
    property = b"cirrus,gpio1-func\0".as_ptr() as *const c_char;
    ret = device_property_read_u32_array(physdev, property, values.as_mut_ptr(), nval);
    if ret != 0 { return cs35l41_hda_parse_acpi_err(cs35l41, hw_cfg, property, ret); }
    (*hw_cfg).gpio1.func = values[(*cs35l41).index as usize] as c_int;
    (*hw_cfg).gpio1.valid = true;
    property = b"cirrus,gpio2-func\0".as_ptr() as *const c_char;
    ret = device_property_read_u32_array(physdev, property, values.as_mut_ptr(), nval);
    if ret != 0 { return cs35l41_hda_parse_acpi_err(cs35l41, hw_cfg, property, ret); }
    (*hw_cfg).gpio2.func = values[(*cs35l41).index as usize] as c_int;
    (*hw_cfg).gpio2.valid = true;
    property = b"cirrus,boost-peak-milliamp\0".as_ptr() as *const c_char;
    ret = device_property_read_u32_array(physdev, property, values.as_mut_ptr(), nval);
    (*hw_cfg).bst_ipk = if ret == 0 { values[(*cs35l41).index as usize] as c_int } else { -1 };
    property = b"cirrus,boost-ind-nanohenry\0".as_ptr() as *const c_char;
    ret = device_property_read_u32_array(physdev, property, values.as_mut_ptr(), nval);
    (*hw_cfg).bst_ind = if ret == 0 { values[(*cs35l41).index as usize] as c_int } else { -1 };
    property = b"cirrus,boost-cap-microfarad\0".as_ptr() as *const c_char;
    ret = device_property_read_u32_array(physdev, property, values.as_mut_ptr(), nval);
    (*hw_cfg).bst_cap = if ret == 0 { values[(*cs35l41).index as usize] as c_int } else { -1 };
    (*cs35l41).speaker_id = cs35l41_get_speaker_id(physdev, (*cs35l41).index, nval as c_int, -1);
    (*hw_cfg).bst_type = if (*hw_cfg).bst_ind > 0 || (*hw_cfg).bst_cap > 0 || (*hw_cfg).bst_ipk > 0 { CS35L41_INT_BOOST } else { CS35L41_EXT_BOOST };
    (*hw_cfg).valid = true;
    0
}

unsafe fn cs35l41_hda_parse_acpi_err(cs35l41: *mut cs35l41_hda, hw_cfg: *mut cs35l41_hw_cfg, property: *const c_char, ret: c_int) -> c_int {
    dev_err((*cs35l41).dev, b"Failed property %s: %d\n\0".as_ptr() as *const c_char, property, ret);
    (*hw_cfg).valid = false;
    (*hw_cfg).gpio1.valid = false;
    (*hw_cfg).gpio2.valid = false;
    acpi_dev_put((*cs35l41).dacpi);
    ret
}

unsafe fn cs35l41_hda_read_acpi(cs35l41: *mut cs35l41_hda, hid: *const c_char, id: c_int) -> c_int {
    let adev = acpi_dev_get_first_match_dev(hid, ptr::null(), -1);
    if adev.is_null() {
        dev_err((*cs35l41).dev, b"Failed to find an ACPI device for %s\n\0".as_ptr() as *const c_char, hid);
        return -ENODEV;
    }
    (*cs35l41).dacpi = adev;
    let physdev = get_device(acpi_get_first_physical_node(adev));
    if physdev.is_null() { acpi_dev_put(adev); return -ENODEV; }
    let mut sub = acpi_get_subsystem_id(physdev as acpi_handle);
    if IS_ERR(sub) { sub = ptr::null(); }
    (*cs35l41).acpi_subsystem_id = sub;
    let mut ret = cs35l41_add_dsd_properties(cs35l41, physdev, id, hid);
    if ret == 0 {
        dev_info((*cs35l41).dev, b"Using extra _DSD properties, bypassing _DSD in ACPI\n\0".as_ptr() as *const c_char);
    } else {
        ret = cs35l41_hda_parse_acpi(cs35l41, physdev, id);
        if ret != 0 { put_device(physdev); return ret; }
    }
    (*cs35l41).bypass_fw = false;
    if (*cs35l41).control_bus == SPI {
        let spi = to_spi_device((*cs35l41).dev);
        if (*spi).max_speed_hz < CS35L41_MAX_ACCEPTABLE_SPI_SPEED_HZ {
            dev_warn((*cs35l41).dev, b"SPI speed is too slow to support firmware download: %d Hz.\n\0".as_ptr() as *const c_char, (*spi).max_speed_hz);
            (*cs35l41).bypass_fw = true;
        }
    }
    put_device(physdev);
    0
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_hda_probe(dev: *mut device, device_name: *const c_char, id: c_int, irq: c_int, regmap_: *mut regmap, control_bus_: control_bus) -> c_int {
    let mut regid: c_uint = 0;
    let mut reg_revid: c_uint = 0;
    if IS_ERR(regmap_) { return PTR_ERR(regmap_); }
    let cs35l41 = devm_kzalloc(dev, size_of::<cs35l41_hda>(), GFP_KERNEL) as *mut cs35l41_hda;
    if cs35l41.is_null() { return -ENOMEM; }
    (*cs35l41).dev = dev;
    (*cs35l41).irq = irq;
    (*cs35l41).regmap = regmap_;
    (*cs35l41).control_bus = control_bus_;
    dev_set_drvdata(dev, cs35l41 as *mut c_void);
    let mut ret = cs35l41_hda_read_acpi(cs35l41, device_name, id);
    if ret != 0 { return dev_err_probe((*cs35l41).dev, ret, b"Platform not supported\n\0".as_ptr() as *const c_char); }
    if IS_ERR((*cs35l41).reset_gpio) {
        ret = PTR_ERR((*cs35l41).reset_gpio);
        (*cs35l41).reset_gpio = ptr::null_mut();
        if ret == -EBUSY {
            dev_info((*cs35l41).dev, b"Reset line busy, assuming shared reset\n\0".as_ptr() as *const c_char);
        } else {
            dev_err_probe((*cs35l41).dev, ret, b"Failed to get reset GPIO\n\0".as_ptr() as *const c_char);
            return cs35l41_hda_probe_err(cs35l41, ret);
        }
    }
    if !(*cs35l41).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*cs35l41).reset_gpio, 0);
        usleep_range(2000, 2100);
        gpiod_set_value_cansleep((*cs35l41).reset_gpio, 1);
    }
    usleep_range(2000, 2100);
    regmap_write((*cs35l41).regmap, CS35L41_SFT_RESET, CS35L41_SOFTWARE_RESET);
    usleep_range(2000, 2100);
    ret = cs35l41_wait_boot_done(cs35l41);
    if ret != 0 { return cs35l41_hda_probe_err(cs35l41, ret); }
    ret = cs35l41_verify_id(cs35l41, &mut regid, &mut reg_revid);
    if ret != 0 { return cs35l41_hda_probe_err(cs35l41, ret); }
    ret = cs35l41_test_key_unlock((*cs35l41).dev, (*cs35l41).regmap);
    if ret != 0 { return cs35l41_hda_probe_err(cs35l41, ret); }
    ret = cs35l41_register_errata_patch((*cs35l41).dev, (*cs35l41).regmap, reg_revid);
    if ret != 0 { return cs35l41_hda_probe_err(cs35l41, ret); }
    ret = cs35l41_otp_unpack((*cs35l41).dev, (*cs35l41).regmap);
    if ret != 0 {
        dev_err_probe((*cs35l41).dev, ret, b"OTP Unpack failed\n\0".as_ptr() as *const c_char);
        return cs35l41_hda_probe_err(cs35l41, ret);
    }
    ret = cs35l41_test_key_lock((*cs35l41).dev, (*cs35l41).regmap);
    if ret != 0 { return cs35l41_hda_probe_err(cs35l41, ret); }
    ret = cs35l41_get_calibration(cs35l41);
    if ret != 0 && ret != -ENODEV { return cs35l41_hda_probe_err(cs35l41, ret); }
    cs35l41_mute((*cs35l41).dev, true);
    // INIT_WORK(&cs35l41->fw_load_work, cs35l41_fw_load_work)
    mutex_init(&mut (*cs35l41).fw_mutex);
    pm_runtime_set_autosuspend_delay((*cs35l41).dev, 3000);
    pm_runtime_use_autosuspend((*cs35l41).dev);
    pm_runtime_set_active((*cs35l41).dev);
    pm_runtime_get_noresume((*cs35l41).dev);
    pm_runtime_enable((*cs35l41).dev);
    ret = cs35l41_hda_apply_properties(cs35l41);
    if ret != 0 { return cs35l41_hda_probe_err_pm(cs35l41, ret); }
    pm_runtime_put_autosuspend((*cs35l41).dev);
    ret = component_add((*cs35l41).dev, &cs35l41_hda_comp_ops);
    if ret != 0 {
        dev_err_probe((*cs35l41).dev, ret, b"Register component failed\n\0".as_ptr() as *const c_char);
        return cs35l41_hda_probe_err_pm(cs35l41, ret);
    }
    dev_info((*cs35l41).dev, b"Cirrus Logic CS35L41 (%x), Revision: %02X\n\0".as_ptr() as *const c_char, regid, reg_revid);
    0
}

unsafe fn cs35l41_hda_probe_err_pm(cs35l41: *mut cs35l41_hda, ret: c_int) -> c_int {
    pm_runtime_dont_use_autosuspend((*cs35l41).dev);
    pm_runtime_disable((*cs35l41).dev);
    pm_runtime_put_noidle((*cs35l41).dev);
    cs35l41_hda_probe_err(cs35l41, ret)
}

unsafe fn cs35l41_hda_probe_err(cs35l41: *mut cs35l41_hda, ret: c_int) -> c_int {
    if cs35l41_safe_reset((*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type) != 0 {
        gpiod_set_value_cansleep((*cs35l41).reset_gpio, 0);
    }
    gpiod_put((*cs35l41).reset_gpio);
    gpiod_put((*cs35l41).cs_gpio);
    acpi_dev_put((*cs35l41).dacpi);
    kfree((*cs35l41).acpi_subsystem_id as *const c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_hda_remove(dev: *mut device) {
    let cs35l41 = dev_get_drvdata(dev) as *mut cs35l41_hda;
    component_del((*cs35l41).dev, &cs35l41_hda_comp_ops);
    cancel_work_sync(&mut (*cs35l41).fw_load_work);
    pm_runtime_get_sync((*cs35l41).dev);
    pm_runtime_dont_use_autosuspend((*cs35l41).dev);
    pm_runtime_disable((*cs35l41).dev);
    if (*cs35l41).halo_initialized { cs35l41_remove_dsp(cs35l41); }
    acpi_dev_put((*cs35l41).dacpi);
    pm_runtime_put_noidle((*cs35l41).dev);
    if cs35l41_safe_reset((*cs35l41).regmap, (*cs35l41).hw_cfg.bst_type) != 0 {
        gpiod_set_value_cansleep((*cs35l41).reset_gpio, 0);
    }
    gpiod_put((*cs35l41).reset_gpio);
    gpiod_put((*cs35l41).cs_gpio);
    kfree((*cs35l41).acpi_subsystem_id as *const c_void);
}

#[no_mangle]
pub static cs35l41_hda_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(cs35l41_runtime_suspend),
    runtime_resume: Some(cs35l41_runtime_resume),
    runtime_idle: Some(cs35l41_runtime_idle),
    prepare: Some(cs35l41_system_suspend_prep),
    suspend: Some(cs35l41_system_suspend),
    resume: Some(cs35l41_system_resume),
};

// EXPORT_SYMBOL_NS_GPL(cs35l41_hda_probe, "SND_HDA_SCODEC_CS35L41");
// EXPORT_SYMBOL_NS_GPL(cs35l41_hda_remove, "SND_HDA_SCODEC_CS35L41");
// EXPORT_SYMBOL_NS_GPL(cs35l41_hda_pm_ops, "SND_HDA_SCODEC_CS35L41");
// MODULE_DESCRIPTION("CS35L41 HDA Driver");
// MODULE_IMPORT_NS("SND_SOC_CS_AMP_LIB");
// MODULE_AUTHOR("Lucas Tanure, Cirrus Logic Inc, <tanureal@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("FW_CS_DSP");
// MODULE_FIRMWARE("cirrus/cs35l41-*.wmfw");
// MODULE_FIRMWARE("cirrus/cs35l41-*.bin");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
