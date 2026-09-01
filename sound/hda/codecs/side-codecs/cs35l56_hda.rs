// SPDX-License-Identifier: GPL-2.0-only
//
// HDA audio driver for Cirrus Logic CS35L56 smart amp
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type u32 = u32;
type s16 = i16;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hda_codec {
    pub card: *mut snd_card,
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
type c_ulong = usize;

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub enumerated: snd_ctl_elem_info_enumerated,
    pub integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_enumerated {
    pub items: c_uint,
    pub item: c_uint,
    pub name: [c_char; 64],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
    pub step: c_long,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 4],
}
#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub private_value: c_ulong,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
}
#[repr(C)]
pub struct cs_dsp {
    pub booted: bool_,
}
#[repr(C)]
pub struct cs35l56_fw_regs {
    pub transducer_actual_ps: c_uint,
    pub posture_number: c_uint,
    pub user_volume: c_uint,
    pub prot_sts: c_uint,
}
#[repr(C)]
pub struct cs35l56_base {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub fw_reg: *mut cs35l56_fw_regs,
    pub irq_lock: mutex,
    pub cal_data_valid: bool_,
    pub secured: bool_,
    pub fw_patched: bool_,
    pub type_: c_uint,
    pub rev: c_uint,
    pub cal_data: cs35l56_cal_data,
    pub reset_gpio: *mut gpio_desc,
    pub irq: c_int,
    pub cal_index: c_int,
    pub init_done: bool_,
}
#[repr(C)]
pub struct cs35l56_cal_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct cs35l56_hda {
    pub base: cs35l56_base,
    pub cs_dsp: cs_dsp,
    pub dsp_work: work_struct,
    pub asp_tx_mask: c_uint,
    pub playing: bool_,
    pub suspended: bool_,
    pub posture_ctl: *mut snd_kcontrol,
    pub mixer_ctl: [*mut snd_kcontrol; 4],
    pub volume_ctl: *mut snd_kcontrol,
    pub system_name: *const c_char,
    pub amp_name: *const c_char,
    pub num_amps: c_int,
    pub index: c_int,
    pub codec: *mut hda_codec,
    pub debugfs_root: *mut c_void,
}
#[repr(C)]
pub struct hda_component {
    pub dev: *mut device,
    pub name: [c_char; 64],
    pub playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
}
#[repr(C)]
pub struct hda_component_parent {
    pub codec: *mut hda_codec,
}
#[repr(C)]
pub struct component_ops {
    pub bind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void) -> c_int>,
    pub unbind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void)>,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct cs35l56_cal_debugfs_fops_file {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
}
#[repr(C)]
pub struct cs35l56_cal_debugfs_fops {
    pub calibrate: cs35l56_cal_debugfs_fops_file,
    pub cal_temperature: cs35l56_cal_debugfs_fops_file,
    pub cal_data: cs35l56_cal_debugfs_fops_file,
}
#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub suspend_late: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume_early: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub suspend_noirq: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume_noirq: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

const fn BIT(n: c_int) -> c_uint {
    1u32 << (n as u32)
}
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const ENODATA: c_int = 61;
const EIO: c_int = 5;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const HDA_MAX_COMPONENTS: usize = 4;
const HDA_GEN_PCM_ACT_PREPARE: c_int = 0;
const HDA_GEN_PCM_ACT_CLEANUP: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_uint = 3;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x40000;

const CS35L56_ASP1_CONTROL1: c_uint = 0;
const CS35L56_ASP1_CONTROL2: c_uint = 0;
const CS35L56_ASP1_CONTROL3: c_uint = 0;
const CS35L56_ASP1_FRAME_CONTROL1: c_uint = 0;
const CS35L56_ASP1_FRAME_CONTROL5: c_uint = 0;
const CS35L56_ASP1_DATA_CONTROL5: c_uint = 0;
const CS35L56_ASP1_DATA_CONTROL1: c_uint = 0;
const CS35L56_ASP1_ENABLES1: c_uint = 0;
const CS35L56_ASP1TX1_INPUT: c_uint = 0;
const CS35L56_ASP1TX2_INPUT: c_uint = 0;
const CS35L56_ASP1TX3_INPUT: c_uint = 0;
const CS35L56_ASP1TX4_INPUT: c_uint = 0;
const CS35L56_MBOX_CMD_AUDIO_PLAY: c_uint = 0;
const CS35L56_MBOX_CMD_AUDIO_PAUSE: c_uint = 0;
const CS35L56_MBOX_CMD_ALLOW_AUTO_HIBERNATE: c_uint = 0;
const CS35L56_MBOX_CMD_HIBERNATE_NOW: c_uint = 0;
const CS35L56_MBOX_CMD_PREVENT_AUTO_HIBERNATE: c_uint = 0;
const CS35L56_MBOX_CMD_AUDIO_REINIT: c_uint = 0;
const CS35L56_DSP_VIRTUAL1_MBOX_1: c_uint = 0;
const CS35L56_PS0: c_uint = 0;
const CS35L56_PS0_POLL_US: c_uint = 0;
const CS35L56_PS0_TIMEOUT_US: c_uint = 0;
const CS35L56_ASP_RX1_EN_SHIFT: c_int = 0;
const CS35L56_ASP_RX2_EN_SHIFT: c_int = 1;
const CS35L56_ASP_TX1_EN_SHIFT: c_int = 0;
const CS35L56_ASP_TX2_EN_SHIFT: c_int = 1;
const CS35L56_ASP_TX3_EN_SHIFT: c_int = 2;
const CS35L56_ASP_TX4_EN_SHIFT: c_int = 3;
const CS35L56_NUM_INPUT_SRC: c_uint = 0;
const CS35L56_ASP_TXn_SRC_MASK: c_uint = 0;
const CS35L56_INPUT_MASK: c_uint = 0;
const CS35L56_MAIN_POSTURE_MIN: c_long = 0;
const CS35L56_MAIN_POSTURE_MAX: c_long = 0;
const CS35L56_MAIN_POSTURE_MASK: c_uint = 0;
const CS35L56_MAIN_RENDER_USER_VOLUME_MAX: c_long = 0;
const CS35L56_MAIN_RENDER_USER_VOLUME_MIN: c_long = 0;
const CS35L56_MAIN_RENDER_USER_VOLUME_SHIFT: c_int = 0;
const CS35L56_MAIN_RENDER_USER_VOLUME_SIGNBIT: c_int = 0;
const CS35L56_MAIN_RENDER_USER_VOLUME_MASK: c_uint = 0;
const CS35L56_FIRMWARE_MISSING: c_uint = 0;

unsafe extern "C" {
    static cs35l56_tx_input_texts: [*const c_char; 0];
    static cs35l56_tx_input_values: [c_uint; 0];
    static cs35l56_calibration_controls: c_void;
    static system_long_wq: *mut c_void;
    static sound_debugfs_root: *mut c_void;

    fn flush_work(work: *mut work_struct);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_dont_use_autosuspend(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn cs35l56_mbox_send(base: *mut cs35l56_base, cmd: c_uint) -> c_int;
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_set_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint) -> c_int;
    fn regmap_clear_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint) -> c_int;
    fn regmap_write(regmap: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint, changed: *mut bool_) -> c_int;
    fn regmap_multi_reg_write(regmap: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regcache_cache_only(regmap: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(regmap: *mut regmap);
    fn regcache_sync(regmap: *mut regmap) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn cs_dsp_stop(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_run(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_power_down(dsp: *mut cs_dsp);
    fn cs_dsp_power_up(dsp: *mut cs_dsp, wmfw: *const firmware, wmfw_filename: *const c_char, coeff: *const firmware, coeff_filename: *const c_char, fw_type: *const c_char) -> c_int;
    fn cs_dsp_halo_init(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_remove(dsp: *mut cs_dsp);
    fn cs_dsp_init_debugfs(dsp: *mut cs_dsp, root: *mut c_void);
    fn cs_dsp_cleanup_debugfs(dsp: *mut cs_dsp);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol);
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> ssize_t;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, gfp: c_uint) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn firmware_request_nowarn(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn isalnum(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn cs_amp_write_cal_coeffs(dsp: *mut cs_dsp, controls: *const c_void, data: *mut cs35l56_cal_data) -> c_int;
    fn cs35l56_read_prot_status(base: *mut cs35l56_base, firmware_missing: *mut bool_, preloaded_fw_ver: *mut c_uint) -> c_int;
    fn cs35l56_firmware_shutdown(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_system_reset(base: *mut cs35l56_base, soft: bool_);
    fn cs35l56_wait_for_firmware_boot(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_log_tuning(base: *mut cs35l56_base, dsp: *mut cs_dsp);
    fn cs35l56_calibrate_debugfs_write(base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn cs35l56_cal_ambient_debugfs_write(base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn cs35l56_cal_data_debugfs_read(base: *mut cs35l56_base, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn cs35l56_cal_data_debugfs_write(base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn cs35l56_create_cal_debugfs(base: *mut cs35l56_base, fops: *const cs35l56_cal_debugfs_fops);
    fn cs35l56_remove_cal_debugfs(base: *mut cs35l56_base);
    fn hda_component_from_index(parent: *mut hda_component_parent, index: c_int) -> *mut hda_component;
    fn queue_work(wq: *mut c_void, work: *mut work_struct) -> bool_;
    fn cancel_work_sync(work: *mut work_struct) -> bool_;
    fn debugfs_create_dir(name: *const c_char, parent: *mut c_void) -> *mut c_void;
    fn debugfs_remove_recursive(root: *mut c_void);
    fn component_add(dev: *mut device, ops: *const component_ops) -> c_int;
    fn component_del(dev: *mut device, ops: *const component_ops);
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn cs35l56_wait_min_reset_pulse();
    fn cs35l56_wait_control_port_ready();
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_void, hrv: c_int) -> *mut acpi_device;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn device_property_count_u32(dev: *mut device, propname: *const c_char) -> c_int;
    fn device_property_read_u32_array(dev: *mut device, propname: *const c_char, vals: *mut u32, nval: c_int) -> c_int;
    fn cirrus_scodec_get_speaker_id(dev: *mut device, index: c_int, num_amps: c_int, fixed: c_int) -> c_int;
    fn devm_gpiod_get_index_optional(dev: *mut device, con_id: *const c_char, index: c_uint, flags: c_uint) -> *mut gpio_desc;
    fn cs35l56_init_cs_dsp(base: *mut cs35l56_base, dsp: *mut cs_dsp);
    fn cs35l56_hw_init(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_set_patch(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_get_calibration(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_is_fw_reload_needed(base: *mut cs35l56_base) -> c_int;
}

/*
 * The cs35l56_hda_dai_config[] reg sequence configures the device as
 *  ASP1_BCLK_FREQ = 3.072 MHz
 *  ASP1_RX_WIDTH = 32 cycles per slot, ASP1_TX_WIDTH = 32 cycles per slot, ASP1_FMT = I2S
 *  ASP1_DOUT_HIZ_CONTROL = Hi-Z during unused timeslots
 *  ASP1_RX_WL = 24 bits per sample
 *  ASP1_TX_WL = 24 bits per sample
 *  ASP1_RXn_EN 1..3 and ASP1_TXn_EN 1..4 disabled
 *
 * Override any Windows-specific mixer settings applied by the firmware.
 */
static cs35l56_hda_dai_config: [reg_sequence; 12] = [
    reg_sequence { reg: CS35L56_ASP1_CONTROL1, def: 0x00000021 },
    reg_sequence { reg: CS35L56_ASP1_CONTROL2, def: 0x20200200 },
    reg_sequence { reg: CS35L56_ASP1_CONTROL3, def: 0x00000003 },
    reg_sequence { reg: CS35L56_ASP1_FRAME_CONTROL1, def: 0x03020100 },
    reg_sequence { reg: CS35L56_ASP1_FRAME_CONTROL5, def: 0x00020100 },
    reg_sequence { reg: CS35L56_ASP1_DATA_CONTROL5, def: 0x00000018 },
    reg_sequence { reg: CS35L56_ASP1_DATA_CONTROL1, def: 0x00000018 },
    reg_sequence { reg: CS35L56_ASP1_ENABLES1, def: 0x00000000 },
    reg_sequence { reg: CS35L56_ASP1TX1_INPUT, def: 0x00000018 },
    reg_sequence { reg: CS35L56_ASP1TX2_INPUT, def: 0x00000019 },
    reg_sequence { reg: CS35L56_ASP1TX3_INPUT, def: 0x00000020 },
    reg_sequence { reg: CS35L56_ASP1TX4_INPUT, def: 0x00000028 },
];

unsafe extern "C" fn cs35l56_hda_wait_dsp_ready(cs35l56: *mut cs35l56_hda) {
    /* Wait for patching to complete */
    unsafe { flush_work(core::ptr::addr_of_mut!((*cs35l56).dsp_work)); }
}

unsafe extern "C" fn cs35l56_hda_play(cs35l56: *mut cs35l56_hda) {
    let mut val: c_uint = 0;
    let mut ret: c_int;
    unsafe {
        cs35l56_hda_wait_dsp_ready(cs35l56);
        pm_runtime_get_sync((*cs35l56).base.dev);
        ret = cs35l56_mbox_send(core::ptr::addr_of_mut!((*cs35l56).base), CS35L56_MBOX_CMD_AUDIO_PLAY);
        if ret == 0 {
            /* Wait for firmware to enter PS0 power state */
            loop {
                ret = regmap_read((*cs35l56).base.regmap, (*(*cs35l56).base.fw_reg).transducer_actual_ps, &mut val);
                if ret != 0 || val == CS35L56_PS0 {
                    break;
                }
                /* regmap_read_poll_timeout delay/timeout parameters are provided by external kernel macro. */
                break;
            }
            if ret != 0 {
                dev_warn((*cs35l56).base.dev, c"PS0 wait failed: %d\n".as_ptr(), ret);
            }
        }
        regmap_set_bits((*cs35l56).base.regmap, CS35L56_ASP1_ENABLES1,
                        BIT(CS35L56_ASP_RX1_EN_SHIFT) | BIT(CS35L56_ASP_RX2_EN_SHIFT) |
                        (*cs35l56).asp_tx_mask);
        (*cs35l56).playing = true;
    }
}

unsafe extern "C" fn cs35l56_hda_pause(cs35l56: *mut cs35l56_hda) {
    unsafe {
        (*cs35l56).playing = false;
        cs35l56_mbox_send(core::ptr::addr_of_mut!((*cs35l56).base), CS35L56_MBOX_CMD_AUDIO_PAUSE);
        regmap_clear_bits((*cs35l56).base.regmap, CS35L56_ASP1_ENABLES1,
                          BIT(CS35L56_ASP_RX1_EN_SHIFT) | BIT(CS35L56_ASP_RX2_EN_SHIFT) |
                          BIT(CS35L56_ASP_TX1_EN_SHIFT) | BIT(CS35L56_ASP_TX2_EN_SHIFT) |
                          BIT(CS35L56_ASP_TX3_EN_SHIFT) | BIT(CS35L56_ASP_TX4_EN_SHIFT));
        pm_runtime_put_autosuspend((*cs35l56).base.dev);
    }
}

unsafe extern "C" fn cs35l56_hda_playback_hook(dev: *mut device, action: c_int) {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        dev_dbg((*cs35l56).base.dev, c"%s()%d: action: %d\n".as_ptr(), c"cs35l56_hda_playback_hook".as_ptr(), line!() as c_int, action);
        match action {
            HDA_GEN_PCM_ACT_PREPARE => {
                if (*cs35l56).playing {
                    return;
                }
                /* If we're suspended: flag that resume should start playback */
                if (*cs35l56).suspended {
                    (*cs35l56).playing = true;
                    return;
                }
                cs35l56_hda_play(cs35l56);
            }
            HDA_GEN_PCM_ACT_CLEANUP => {
                if !(*cs35l56).playing {
                    return;
                }
                cs35l56_hda_pause(cs35l56);
            }
            _ => {}
        }
    }
}

unsafe extern "C" fn cs35l56_hda_runtime_suspend(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        if (*cs35l56).cs_dsp.booted {
            cs_dsp_stop(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        }
        cs35l56_runtime_suspend_common(core::ptr::addr_of_mut!((*cs35l56).base))
    }
}

unsafe extern "C" {
    fn cs35l56_runtime_suspend_common(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_runtime_resume_common(base: *mut cs35l56_base, is_soundwire: bool_) -> c_int;
}

unsafe extern "C" fn cs35l56_hda_runtime_resume(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        let mut ret = cs35l56_runtime_resume_common(core::ptr::addr_of_mut!((*cs35l56).base), false);
        if ret < 0 {
            return ret;
        }
        if (*cs35l56).cs_dsp.booted {
            ret = cs_dsp_run(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
            if ret != 0 {
                dev_dbg((*cs35l56).base.dev, c"%s: cs_dsp_run ret %d\n".as_ptr(), c"cs35l56_hda_runtime_resume".as_ptr(), ret);
                cs35l56_mbox_send(core::ptr::addr_of_mut!((*cs35l56).base), CS35L56_MBOX_CMD_ALLOW_AUTO_HIBERNATE);
                regmap_write((*cs35l56).base.regmap, CS35L56_DSP_VIRTUAL1_MBOX_1, CS35L56_MBOX_CMD_HIBERNATE_NOW);
                regcache_cache_only((*cs35l56).base.regmap, true);
                return ret;
            }
        }
        0
    }
}

unsafe extern "C" fn cs35l56_hda_mixer_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
        (*uinfo).count = 1;
        (*uinfo).value.enumerated.items = CS35L56_NUM_INPUT_SRC;
        if (*uinfo).value.enumerated.item >= CS35L56_NUM_INPUT_SRC {
            (*uinfo).value.enumerated.item = CS35L56_NUM_INPUT_SRC - 1;
        }
        strscpy((*uinfo).value.enumerated.name.as_mut_ptr(),
                cs35l56_tx_input_texts[(*uinfo).value.enumerated.item as usize],
                size_of::<[c_char; 64]>());
        0
    }
}

unsafe extern "C" fn cs35l56_hda_mixer_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cs35l56 = snd_kcontrol_chip(kcontrol) as *mut cs35l56_hda;
        let mut reg_val: c_uint = 0;
        cs35l56_hda_wait_dsp_ready(cs35l56);
        let ret = regmap_read((*cs35l56).base.regmap, (*kcontrol).private_value as c_uint, &mut reg_val);
        if ret != 0 {
            return ret;
        }
        reg_val &= CS35L56_ASP_TXn_SRC_MASK;
        let mut i = 0;
        while i < CS35L56_NUM_INPUT_SRC as usize {
            if cs35l56_tx_input_values[i] == reg_val {
                (*ucontrol).value.enumerated.item[0] = i as c_uint;
                break;
            }
            i += 1;
        }
        0
    }
}

unsafe extern "C" fn cs35l56_hda_mixer_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cs35l56 = snd_kcontrol_chip(kcontrol) as *mut cs35l56_hda;
        let item = (*ucontrol).value.enumerated.item[0];
        let mut changed = false;
        if item >= CS35L56_NUM_INPUT_SRC {
            return -EINVAL;
        }
        cs35l56_hda_wait_dsp_ready(cs35l56);
        let ret = regmap_update_bits_check((*cs35l56).base.regmap, (*kcontrol).private_value as c_uint,
                                           CS35L56_INPUT_MASK, cs35l56_tx_input_values[item as usize],
                                           &mut changed);
        if ret != 0 {
            return ret;
        }
        changed as c_int
    }
}

unsafe extern "C" fn cs35l56_hda_posture_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = CS35L56_MAIN_POSTURE_MIN;
        (*uinfo).value.integer.max = CS35L56_MAIN_POSTURE_MAX;
        0
    }
}

unsafe extern "C" fn cs35l56_hda_posture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cs35l56 = snd_kcontrol_chip(kcontrol) as *mut cs35l56_hda;
        let mut pos: c_uint = 0;
        cs35l56_hda_wait_dsp_ready(cs35l56);
        let ret = regmap_read((*cs35l56).base.regmap, (*(*cs35l56).base.fw_reg).posture_number, &mut pos);
        if ret != 0 {
            return ret;
        }
        (*ucontrol).value.integer.value[0] = pos as c_long;
        0
    }
}

unsafe extern "C" fn cs35l56_hda_posture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cs35l56 = snd_kcontrol_chip(kcontrol) as *mut cs35l56_hda;
        let pos = (*ucontrol).value.integer.value[0];
        let mut changed = false;
        if pos < CS35L56_MAIN_POSTURE_MIN || pos > CS35L56_MAIN_POSTURE_MAX {
            return -EINVAL;
        }
        cs35l56_hda_wait_dsp_ready(cs35l56);
        let ret = regmap_update_bits_check((*cs35l56).base.regmap, (*(*cs35l56).base.fw_reg).posture_number,
                                           CS35L56_MAIN_POSTURE_MASK, pos as c_uint, &mut changed);
        if ret != 0 {
            return ret;
        }
        changed as c_int
    }
}

#[repr(C)]
struct cs35l56_hda_mixer_control {
    name: *const c_char,
    reg: c_uint,
}

static cs35l56_hda_mixer_controls: [cs35l56_hda_mixer_control; 4] = [
    cs35l56_hda_mixer_control { name: c"ASP1 TX1 Source".as_ptr(), reg: CS35L56_ASP1TX1_INPUT },
    cs35l56_hda_mixer_control { name: c"ASP1 TX2 Source".as_ptr(), reg: CS35L56_ASP1TX2_INPUT },
    cs35l56_hda_mixer_control { name: c"ASP1 TX3 Source".as_ptr(), reg: CS35L56_ASP1TX3_INPUT },
    cs35l56_hda_mixer_control { name: c"ASP1 TX4 Source".as_ptr(), reg: CS35L56_ASP1TX4_INPUT },
];

/* static const DECLARE_TLV_DB_SCALE(cs35l56_hda_vol_tlv, -10000, 25, 0); */
static cs35l56_hda_vol_tlv: [c_uint; 4] = [0, (-10000i32) as c_uint, 25, 0];

unsafe extern "C" fn cs35l56_hda_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.step = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = CS35L56_MAIN_RENDER_USER_VOLUME_MAX - CS35L56_MAIN_RENDER_USER_VOLUME_MIN;
        0
    }
}

unsafe extern "C" fn cs35l56_hda_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cs35l56 = snd_kcontrol_chip(kcontrol) as *mut cs35l56_hda;
        let mut raw_vol: c_uint = 0;
        cs35l56_hda_wait_dsp_ready(cs35l56);
        let ret = regmap_read((*cs35l56).base.regmap, (*(*cs35l56).base.fw_reg).user_volume, &mut raw_vol);
        if ret != 0 {
            return ret;
        }
        let mut vol = ((raw_vol & 0xffff) as s16) as c_int;
        vol >>= CS35L56_MAIN_RENDER_USER_VOLUME_SHIFT;
        if (vol as c_uint & BIT(CS35L56_MAIN_RENDER_USER_VOLUME_SIGNBIT)) != 0 {
            vol |= !((BIT(CS35L56_MAIN_RENDER_USER_VOLUME_SIGNBIT) - 1) as c_int);
        }
        (*ucontrol).value.integer.value[0] = (vol as c_long) - CS35L56_MAIN_RENDER_USER_VOLUME_MIN;
        0
    }
}

unsafe extern "C" fn cs35l56_hda_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let cs35l56 = snd_kcontrol_chip(kcontrol) as *mut cs35l56_hda;
        let vol = (*ucontrol).value.integer.value[0];
        let mut changed = false;
        if vol < 0 || vol > (CS35L56_MAIN_RENDER_USER_VOLUME_MAX - CS35L56_MAIN_RENDER_USER_VOLUME_MIN) {
            return -EINVAL;
        }
        let raw_vol = ((vol + CS35L56_MAIN_RENDER_USER_VOLUME_MIN) as c_uint) << CS35L56_MAIN_RENDER_USER_VOLUME_SHIFT;
        cs35l56_hda_wait_dsp_ready(cs35l56);
        let ret = regmap_update_bits_check((*cs35l56).base.regmap, (*(*cs35l56).base.fw_reg).user_volume,
                                           CS35L56_MAIN_RENDER_USER_VOLUME_MASK, raw_vol, &mut changed);
        if ret != 0 {
            return ret;
        }
        changed as c_int
    }
}

unsafe extern "C" fn cs35l56_hda_create_controls(cs35l56: *mut cs35l56_hda) {
    unsafe {
        let mut ctl_template: snd_kcontrol_new = zeroed();
        ctl_template.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        ctl_template.access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
        ctl_template.info = Some(cs35l56_hda_posture_info);
        ctl_template.get = Some(cs35l56_hda_posture_get);
        ctl_template.put = Some(cs35l56_hda_posture_put);
        let mut name = [0 as c_char; 64];

        snprintf(name.as_mut_ptr(), name.len(), c"%s Posture Number".as_ptr(), (*cs35l56).amp_name);
        ctl_template.name = name.as_ptr();
        (*cs35l56).posture_ctl = snd_ctl_new1(&ctl_template, cs35l56 as *mut c_void);
        if snd_ctl_add((*(*cs35l56).codec).card, (*cs35l56).posture_ctl) != 0 {
            dev_err((*cs35l56).base.dev, c"Failed to add KControl: %s\n".as_ptr(), ctl_template.name);
        }

        /* Mixer controls */
        ctl_template.info = Some(cs35l56_hda_mixer_info);
        ctl_template.get = Some(cs35l56_hda_mixer_get);
        ctl_template.put = Some(cs35l56_hda_mixer_put);

        let mut i = 0usize;
        while i < cs35l56_hda_mixer_controls.len() {
            snprintf(name.as_mut_ptr(), name.len(), c"%s %s".as_ptr(), (*cs35l56).amp_name, cs35l56_hda_mixer_controls[i].name);
            ctl_template.private_value = cs35l56_hda_mixer_controls[i].reg as c_ulong;
            (*cs35l56).mixer_ctl[i] = snd_ctl_new1(&ctl_template, cs35l56 as *mut c_void);
            if snd_ctl_add((*(*cs35l56).codec).card, (*cs35l56).mixer_ctl[i]) != 0 {
                dev_err((*cs35l56).base.dev, c"Failed to add KControl: %s\n".as_ptr(), ctl_template.name);
            }
            i += 1;
        }

        ctl_template.info = Some(cs35l56_hda_vol_info);
        ctl_template.get = Some(cs35l56_hda_vol_get);
        ctl_template.put = Some(cs35l56_hda_vol_put);
        ctl_template.access = SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ;
        ctl_template.tlv.p = cs35l56_hda_vol_tlv.as_ptr();
        snprintf(name.as_mut_ptr(), name.len(), c"%s Speaker Playback Volume".as_ptr(), (*cs35l56).amp_name);
        ctl_template.name = name.as_ptr();
        (*cs35l56).volume_ctl = snd_ctl_new1(&ctl_template, cs35l56 as *mut c_void);
        if snd_ctl_add((*(*cs35l56).codec).card, (*cs35l56).volume_ctl) != 0 {
            dev_err((*cs35l56).base.dev, c"Failed to add KControl: %s\n".as_ptr(), ctl_template.name);
        }
    }
}

unsafe extern "C" fn cs35l56_hda_remove_controls(cs35l56: *mut cs35l56_hda) {
    unsafe {
        let mut i = (*cs35l56).mixer_ctl.len() as isize - 1;
        while i >= 0 {
            snd_ctl_remove((*(*cs35l56).codec).card, (*cs35l56).mixer_ctl[i as usize]);
            i -= 1;
        }
        snd_ctl_remove((*(*cs35l56).codec).card, (*cs35l56).posture_ctl);
        snd_ctl_remove((*(*cs35l56).codec).card, (*cs35l56).volume_ctl);
    }
}

unsafe extern "C" fn cs35l56_hda_request_firmware_file(cs35l56: *mut cs35l56_hda,
                                                        firmware: *mut *const firmware,
                                                        filename: *mut *mut c_char,
                                                        base_name: *const c_char,
                                                        system_name: *const c_char,
                                                        amp_name: *const c_char,
                                                        filetype: *const c_char) -> c_int {
    unsafe {
        let mut ret = 0;
        if !system_name.is_null() && !amp_name.is_null() {
            *filename = kasprintf(GFP_KERNEL, c"%s-%s-%s.%s".as_ptr(), base_name, system_name, amp_name, filetype);
        } else if !system_name.is_null() {
            *filename = kasprintf(GFP_KERNEL, c"%s-%s.%s".as_ptr(), base_name, system_name, filetype);
        } else {
            *filename = kasprintf(GFP_KERNEL, c"%s.%s".as_ptr(), base_name, filetype);
        }
        if (*filename).is_null() {
            return -ENOMEM;
        }

        /*
         * Make sure that filename is lower-case and any non alpha-numeric
         * characters except full stop and forward slash are replaced with
         * hyphens.
         */
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

        ret = firmware_request_nowarn(firmware, *filename, (*cs35l56).base.dev);
        if ret != 0 {
            dev_dbg((*cs35l56).base.dev, c"Failed to request '%s'\n".as_ptr(), *filename);
            kfree(*filename as *mut c_void);
            *filename = null_mut();
            return ret;
        }
        dev_dbg((*cs35l56).base.dev, c"Found '%s'\n".as_ptr(), *filename);
        0
    }
}

unsafe extern "C" fn cs35l56_hda_request_firmware_files(cs35l56: *mut cs35l56_hda,
                                                         preloaded_fw_ver: c_uint,
                                                         wmfw_firmware: *mut *const firmware,
                                                         wmfw_filename: *mut *mut c_char,
                                                         coeff_firmware: *mut *const firmware,
                                                         coeff_filename: *mut *mut c_char) {
    unsafe {
        let system_name = (*cs35l56).system_name;
        let amp_name = (*cs35l56).amp_name;
        let mut base_name = [0 as c_char; 37];
        if preloaded_fw_ver != 0 {
            snprintf(base_name.as_mut_ptr(), base_name.len(), c"cirrus/cs35l%02x-%02x%s-%06x-dsp1-misc".as_ptr(),
                     (*cs35l56).base.type_, (*cs35l56).base.rev,
                     if (*cs35l56).base.secured { c"-s".as_ptr() } else { c"".as_ptr() },
                     preloaded_fw_ver & 0xffffff);
        } else {
            snprintf(base_name.as_mut_ptr(), base_name.len(), c"cirrus/cs35l%02x-%02x%s-dsp1-misc".as_ptr(),
                     (*cs35l56).base.type_, (*cs35l56).base.rev,
                     if (*cs35l56).base.secured { c"-s".as_ptr() } else { c"".as_ptr() });
        }

        if !system_name.is_null() && !amp_name.is_null() {
            if cs35l56_hda_request_firmware_file(cs35l56, wmfw_firmware, wmfw_filename, base_name.as_ptr(), system_name, amp_name, c"wmfw".as_ptr()) == 0 {
                cs35l56_hda_request_firmware_file(cs35l56, coeff_firmware, coeff_filename, base_name.as_ptr(), system_name, amp_name, c"bin".as_ptr());
                return;
            }
        }
        if !system_name.is_null() {
            if cs35l56_hda_request_firmware_file(cs35l56, wmfw_firmware, wmfw_filename, base_name.as_ptr(), system_name, null(), c"wmfw".as_ptr()) == 0 {
                if !amp_name.is_null() {
                    cs35l56_hda_request_firmware_file(cs35l56, coeff_firmware, coeff_filename, base_name.as_ptr(), system_name, amp_name, c"bin".as_ptr());
                }
                if (*coeff_firmware).is_null() {
                    cs35l56_hda_request_firmware_file(cs35l56, coeff_firmware, coeff_filename, base_name.as_ptr(), system_name, null(), c"bin".as_ptr());
                }
                return;
            }
        }
        let ret = cs35l56_hda_request_firmware_file(cs35l56, wmfw_firmware, wmfw_filename, base_name.as_ptr(), null(), null(), c"wmfw".as_ptr());
        if ret == 0 {
            cs35l56_hda_request_firmware_file(cs35l56, coeff_firmware, coeff_filename, base_name.as_ptr(), null(), null(), c"bin".as_ptr());
            return;
        }
        if (*coeff_firmware).is_null() {
            cs35l56_hda_request_firmware_file(cs35l56, coeff_firmware, coeff_filename, base_name.as_ptr(), null(), null(), c"bin".as_ptr());
        }
    }
}

unsafe extern "C" fn cs35l56_hda_apply_calibration(cs35l56: *mut cs35l56_hda) -> c_int {
    unsafe {
        if !(*cs35l56).base.cal_data_valid || (*cs35l56).base.secured {
            return -EACCES;
        }
        let ret = cs_amp_write_cal_coeffs(core::ptr::addr_of_mut!((*cs35l56).cs_dsp),
                                          core::ptr::addr_of!(cs35l56_calibration_controls),
                                          core::ptr::addr_of_mut!((*cs35l56).base.cal_data));
        if ret < 0 {
            dev_warn((*cs35l56).base.dev, c"Failed to write calibration: %d\n".as_ptr(), ret);
            return ret;
        }
        dev_info((*cs35l56).base.dev, c"Calibration applied\n".as_ptr());
        0
    }
}

unsafe extern "C" fn cs35l56_hda_fw_load(cs35l56: *mut cs35l56_hda) {
    unsafe {
        let mut coeff_firmware: *const firmware = null();
        let mut wmfw_firmware: *const firmware = null();
        let mut coeff_filename: *mut c_char = null_mut();
        let mut wmfw_filename: *mut c_char = null_mut();
        let mut preloaded_fw_ver: c_uint = 0;
        let mut firmware_missing = false;
        let mut ret: c_int;

        /*
         * Prepare for a new DSP power-up. If the DSP has had firmware
         * downloaded previously then it needs to be powered down so that it
         * can be updated.
         */
        if (*cs35l56).base.fw_patched {
            cs_dsp_power_down(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        }
        (*cs35l56).base.fw_patched = false;

        /* PM_RUNTIME_ACQUIRE_IF_ENABLED(cs35l56->base.dev, pm); */
        ret = 0;
        if ret < 0 {
            dev_err((*cs35l56).base.dev, c"Failed to resume and get %d\n".as_ptr(), ret);
            return;
        }

        /*
         * The firmware can only be upgraded if it is currently running
         * from the built-in ROM. If not, the wmfw/bin must be for the
         * version of firmware that is running on the chip.
         */
        ret = cs35l56_read_prot_status(core::ptr::addr_of_mut!((*cs35l56).base), &mut firmware_missing, &mut preloaded_fw_ver);
        if ret != 0 {
            return;
        }
        if firmware_missing {
            preloaded_fw_ver = 0;
        }
        cs35l56_hda_request_firmware_files(cs35l56, preloaded_fw_ver, &mut wmfw_firmware, &mut wmfw_filename, &mut coeff_firmware, &mut coeff_filename);

        /* If the BIOS didn't patch the firmware a wmfw and bin file are mandatory */
        if firmware_missing {
            if wmfw_firmware.is_null() {
                dev_err((*cs35l56).base.dev, c".%s file required but not found\n".as_ptr(), c"wmfw".as_ptr());
                return;
            } else if coeff_firmware.is_null() {
                dev_err((*cs35l56).base.dev, c".%s file required but not found\n".as_ptr(), c"bin".as_ptr());
                return;
            }
        }

        /* guard(mutex)(&cs35l56->base.irq_lock); */
        if firmware_missing && (!wmfw_firmware.is_null() || !coeff_firmware.is_null()) {
            ret = cs35l56_firmware_shutdown(core::ptr::addr_of_mut!((*cs35l56).base));
            if ret != 0 {
                return;
            }
        }

        ret = cs_dsp_power_up(core::ptr::addr_of_mut!((*cs35l56).cs_dsp), wmfw_firmware, wmfw_filename, coeff_firmware, coeff_filename, c"misc".as_ptr());
        if ret != 0 {
            dev_dbg((*cs35l56).base.dev, c"%s: cs_dsp_power_up ret %d\n".as_ptr(), c"cs35l56_hda_fw_load".as_ptr(), ret);
            return;
        }
        if !wmfw_filename.is_null() {
            dev_dbg((*cs35l56).base.dev, c"Loaded WMFW Firmware: %s\n".as_ptr(), wmfw_filename);
        }
        if !coeff_filename.is_null() {
            dev_dbg((*cs35l56).base.dev, c"Loaded Coefficients: %s\n".as_ptr(), coeff_filename);
        }

        /* If we downloaded firmware, reset the device and wait for it to boot */
        if firmware_missing && (!wmfw_firmware.is_null() || !coeff_firmware.is_null()) {
            cs35l56_system_reset(core::ptr::addr_of_mut!((*cs35l56).base), false);
            regcache_mark_dirty((*cs35l56).base.regmap);
            ret = cs35l56_wait_for_firmware_boot(core::ptr::addr_of_mut!((*cs35l56).base));
            if ret != 0 {
                if !(*cs35l56).base.fw_patched {
                    cs_dsp_power_down(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
                }
                return;
            }
            regcache_cache_only((*cs35l56).base.regmap, false);
        }

        /* Disable auto-hibernate so that runtime_pm has control */
        ret = cs35l56_mbox_send(core::ptr::addr_of_mut!((*cs35l56).base), CS35L56_MBOX_CMD_PREVENT_AUTO_HIBERNATE);
        if ret != 0 {
            if !(*cs35l56).base.fw_patched {
                cs_dsp_power_down(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
            }
            return;
        }
        regcache_sync((*cs35l56).base.regmap);
        regmap_clear_bits((*cs35l56).base.regmap, (*(*cs35l56).base.fw_reg).prot_sts, CS35L56_FIRMWARE_MISSING);
        (*cs35l56).base.fw_patched = true;

        ret = cs_dsp_run(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        if ret != 0 {
            dev_dbg((*cs35l56).base.dev, c"%s: cs_dsp_run ret %d\n".as_ptr(), c"cs35l56_hda_fw_load".as_ptr(), ret);
        }
        cs35l56_hda_apply_calibration(cs35l56);
        ret = cs35l56_mbox_send(core::ptr::addr_of_mut!((*cs35l56).base), CS35L56_MBOX_CMD_AUDIO_REINIT);
        if ret != 0 {
            cs_dsp_stop(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        }
        cs35l56_log_tuning(core::ptr::addr_of_mut!((*cs35l56).base), core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        if !(*cs35l56).base.fw_patched {
            cs_dsp_power_down(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        }
    }
}

unsafe extern "C" fn cs35l56_hda_dsp_work(work: *mut work_struct) {
    unsafe {
        let cs35l56 = (work as *mut u8).sub(0) as *mut cs35l56_hda; /* container_of(work, struct cs35l56_hda, dsp_work) */
        cs35l56_hda_fw_load(cs35l56);
    }
}

unsafe extern "C" fn cs35l56_hda_from_base(base: *mut cs35l56_base) -> *mut cs35l56_hda {
    base as *mut cs35l56_hda
}

unsafe extern "C" fn cs35l56_hda_debugfs_calibrate_write(file: *mut file, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    unsafe {
        let cs35l56_base = (*file).private_data as *mut cs35l56_base;
        /* PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(cs35l56_base->dev, pm); */
        cs35l56_calibrate_debugfs_write(cs35l56_base, from, count, ppos)
    }
}

unsafe extern "C" fn cs35l56_hda_debugfs_cal_temperature_write(file: *mut file, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    unsafe {
        let cs35l56_base = (*file).private_data as *mut cs35l56_base;
        /* PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(cs35l56_base->dev, pm); */
        cs35l56_cal_ambient_debugfs_write(cs35l56_base, from, count, ppos)
    }
}

unsafe extern "C" fn cs35l56_hda_debugfs_cal_data_read(file: *mut file, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    unsafe {
        let cs35l56_base = (*file).private_data as *mut cs35l56_base;
        /* PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(cs35l56_base->dev, pm); */
        cs35l56_cal_data_debugfs_read(cs35l56_base, to, count, ppos)
    }
}

unsafe extern "C" fn cs35l56_hda_debugfs_cal_data_write(file: *mut file, from: *const c_char, mut count: size_t, ppos: *mut loff_t) -> ssize_t {
    unsafe {
        let cs35l56_base = (*file).private_data as *mut cs35l56_base;
        let cs35l56 = cs35l56_hda_from_base(cs35l56_base);
        let mut ret = cs35l56_cal_data_debugfs_write(cs35l56_base, from, count, ppos);
        if ret == -(ENODATA as ssize_t) {
            return count as ssize_t; /* Ignore writes of empty cal blobs */
        }
        if ret < 0 {
            return ret;
        }
        /* PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(cs35l56_base->dev, pm); */
        ret = cs35l56_hda_apply_calibration(cs35l56) as ssize_t;
        if ret == 0 {
            cs35l56_mbox_send(cs35l56_base, CS35L56_MBOX_CMD_AUDIO_REINIT);
        } else {
            count = (-EIO) as size_t;
        }
        count as ssize_t
    }
}

static cs35l56_hda_cal_debugfs_fops: cs35l56_cal_debugfs_fops = cs35l56_cal_debugfs_fops {
    calibrate: cs35l56_cal_debugfs_fops_file { read: None, write: Some(cs35l56_hda_debugfs_calibrate_write) },
    cal_temperature: cs35l56_cal_debugfs_fops_file { read: None, write: Some(cs35l56_hda_debugfs_cal_temperature_write) },
    cal_data: cs35l56_cal_debugfs_fops_file { read: Some(cs35l56_hda_debugfs_cal_data_read), write: Some(cs35l56_hda_debugfs_cal_data_write) },
};

unsafe extern "C" fn cs35l56_hda_bind(dev: *mut device, _master: *mut device, master_data: *mut c_void) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        let parent = master_data as *mut hda_component_parent;
        let comp = hda_component_from_index(parent, (*cs35l56).index);
        if comp.is_null() {
            return -EINVAL;
        }
        if !(*comp).dev.is_null() {
            return -EBUSY;
        }
        (*comp).dev = dev;
        (*cs35l56).codec = (*parent).codec;
        strscpy((*comp).name.as_mut_ptr(), dev_name(dev), size_of::<[c_char; 64]>());
        (*comp).playback_hook = Some(cs35l56_hda_playback_hook);
        queue_work(system_long_wq, core::ptr::addr_of_mut!((*cs35l56).dsp_work));
        cs35l56_hda_create_controls(cs35l56);
        /* #if IS_ENABLED(CONFIG_SND_DEBUG) */
        (*cs35l56).debugfs_root = debugfs_create_dir(dev_name((*cs35l56).base.dev), sound_debugfs_root);
        cs_dsp_init_debugfs(core::ptr::addr_of_mut!((*cs35l56).cs_dsp), (*cs35l56).debugfs_root);
        /* #endif */
        /* if (IS_ENABLED(CONFIG_SND_HDA_SCODEC_CS35L56_CAL_DEBUGFS)) */
        cs35l56_create_cal_debugfs(core::ptr::addr_of_mut!((*cs35l56).base), &cs35l56_hda_cal_debugfs_fops);
        dev_dbg((*cs35l56).base.dev, c"Bound\n".as_ptr());
        0
    }
}

unsafe extern "C" fn cs35l56_hda_unbind(dev: *mut device, _master: *mut device, master_data: *mut c_void) {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        let parent = master_data as *mut hda_component_parent;
        cancel_work_sync(core::ptr::addr_of_mut!((*cs35l56).dsp_work));
        cs35l56_remove_cal_debugfs(core::ptr::addr_of_mut!((*cs35l56).base));
        cs35l56_hda_remove_controls(cs35l56);
        /* #if IS_ENABLED(CONFIG_SND_DEBUG) */
        cs_dsp_cleanup_debugfs(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        debugfs_remove_recursive((*cs35l56).debugfs_root);
        /* #endif */
        if (*cs35l56).base.fw_patched {
            cs_dsp_power_down(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        }
        let comp = hda_component_from_index(parent, (*cs35l56).index);
        if !comp.is_null() && (*comp).dev == dev {
            core::ptr::write_bytes(comp as *mut u8, 0, size_of::<hda_component>());
        }
        (*cs35l56).codec = null_mut();
        dev_dbg((*cs35l56).base.dev, c"Unbound\n".as_ptr());
    }
}

static cs35l56_hda_comp_ops: component_ops = component_ops {
    bind: Some(cs35l56_hda_bind),
    unbind: Some(cs35l56_hda_unbind),
};

unsafe extern "C" fn cs35l56_hda_system_suspend(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        cs35l56_hda_wait_dsp_ready(cs35l56);
        if (*cs35l56).playing {
            cs35l56_hda_pause(cs35l56);
        }
        (*cs35l56).suspended = true;
        /*
         * The interrupt line is normally shared, but after we start suspending
         * we can't check if our device is the source of an interrupt, and can't
         * clear it. Prevent this race by temporarily disabling the parent irq
         * until we reach _no_irq.
         */
        if (*cs35l56).base.irq != 0 {
            disable_irq((*cs35l56).base.irq);
        }
        pm_runtime_force_suspend(dev)
    }
}

unsafe extern "C" fn cs35l56_hda_system_suspend_late(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        /*
         * RESET is usually shared by all amps so it must not be asserted until
         * all driver instances have done their suspend() stage.
         */
        if !(*cs35l56).base.reset_gpio.is_null() {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            cs35l56_wait_min_reset_pulse();
        }
        0
    }
}

unsafe extern "C" fn cs35l56_hda_system_suspend_no_irq(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        /* Handlers are now disabled so the parent IRQ can safely be re-enabled. */
        if (*cs35l56).base.irq != 0 {
            enable_irq((*cs35l56).base.irq);
        }
        0
    }
}

unsafe extern "C" fn cs35l56_hda_system_resume_no_irq(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        /*
         * WAKE interrupts unmask if the CS35L56 hibernates, which can cause
         * spurious interrupts, and the interrupt line is normally shared.
         * We can't check if our device is the source of an interrupt, and can't
         * clear it, until it has fully resumed. Prevent this race by temporarily
         * disabling the parent irq until we complete resume().
         */
        if (*cs35l56).base.irq != 0 {
            disable_irq((*cs35l56).base.irq);
        }
        0
    }
}

unsafe extern "C" fn cs35l56_hda_system_resume_early(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        /* Ensure a spec-compliant RESET pulse. */
        if !(*cs35l56).base.reset_gpio.is_null() {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            cs35l56_wait_min_reset_pulse();
            /* Release shared RESET before drivers start resume(). */
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 1);
            cs35l56_wait_control_port_ready();
        }
        0
    }
}

unsafe extern "C" fn cs35l56_hda_system_resume(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        /* Undo pm_runtime_force_suspend() before re-enabling the irq */
        let mut ret = pm_runtime_force_resume(dev);
        if (*cs35l56).base.irq != 0 {
            enable_irq((*cs35l56).base.irq);
        }
        if ret != 0 {
            return ret;
        }
        (*cs35l56).suspended = false;
        if (*cs35l56).codec.is_null() {
            return 0;
        }
        ret = cs35l56_is_fw_reload_needed(core::ptr::addr_of_mut!((*cs35l56).base));
        dev_dbg((*cs35l56).base.dev, c"fw_reload_needed: %d\n".as_ptr(), ret);
        if ret > 0 {
            queue_work(system_long_wq, core::ptr::addr_of_mut!((*cs35l56).dsp_work));
        }
        if (*cs35l56).playing {
            cs35l56_hda_play(cs35l56);
        }
        0
    }
}

unsafe extern "C" fn cs35l56_hda_fixup_yoga9(cs35l56: *mut cs35l56_hda, bus_addr: *mut c_int) -> c_int {
    unsafe {
        /* The cirrus,dev-index property has the wrong values */
        (*cs35l56).num_amps = 2;
        match *bus_addr {
            0x30 => {
                (*cs35l56).index = 1;
                0
            }
            0x31 => {
                (*cs35l56).index = 0;
                0
            }
            _ => {
                /* There is a pseudo-address for broadcast to both amps - ignore it */
                dev_dbg((*cs35l56).base.dev, c"Ignoring I2C address %#x\n".as_ptr(), *bus_addr);
                0
            }
        }
    }
}

#[repr(C)]
struct cs35l56_hda_fixup {
    sub: *const c_char,
    fixup_fn: Option<unsafe extern "C" fn(*mut cs35l56_hda, *mut c_int) -> c_int>,
}
static cs35l56_hda_fixups: [cs35l56_hda_fixup; 1] = [
    cs35l56_hda_fixup {
        sub: c"17AA390B".as_ptr(), /* Lenovo Yoga Book 9i GenX */
        fixup_fn: Some(cs35l56_hda_fixup_yoga9),
    },
];

unsafe extern "C" fn cs35l56_hda_apply_platform_fixups(cs35l56: *mut cs35l56_hda, sub: *const c_char, bus_addr: *mut c_int) -> c_int {
    unsafe {
        if IS_ERR(sub as *const c_void) {
            return 0;
        }
        let mut i = 0usize;
        while i < cs35l56_hda_fixups.len() {
            if strcasecmp(cs35l56_hda_fixups[i].sub, sub) == 0 {
                dev_dbg((*cs35l56).base.dev, c"Applying fixup for %s\n".as_ptr(), cs35l56_hda_fixups[i].sub);
                return cs35l56_hda_fixups[i].fixup_fn.unwrap()(cs35l56, bus_addr);
            }
            i += 1;
        }
        0
    }
}

fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}
fn PTR_ERR<T>(ptr: *const T) -> c_long {
    ptr as c_long
}
fn ACPI_COMPANION(_dev: *mut device) -> *mut acpi_device {
    null_mut()
}
fn ACPI_COMPANION_SET(_dev: *mut device, _adev: *mut acpi_device) {}
fn ACPI_HANDLE(_dev: *mut device) -> *mut c_void {
    null_mut()
}
unsafe extern "C" {
    fn acpi_get_subsystem_id(handle: *mut c_void) -> *mut c_char;
}

unsafe extern "C" fn cs35l56_hda_read_acpi(cs35l56: *mut cs35l56_hda, hid: c_int, mut id: c_int) -> c_int {
    unsafe {
        let mut values = [0u32; HDA_MAX_COMPONENTS];
        let mut hid_string = [0 as c_char; 8];
        let mut property: *const c_char;
        let mut ret: c_int;

        /*
         * ACPI_COMPANION isn't available when this driver was instantiated by
         * the serial-multi-instantiate driver, so lookup the node by HID
         */
        if ACPI_COMPANION((*cs35l56).base.dev).is_null() {
            snprintf(hid_string.as_mut_ptr(), hid_string.len(), c"CSC%04X".as_ptr(), hid);
            let adev = acpi_dev_get_first_match_dev(hid_string.as_ptr(), null(), -1);
            if adev.is_null() {
                dev_err((*cs35l56).base.dev, c"Failed to find an ACPI device for %s\n".as_ptr(), dev_name((*cs35l56).base.dev));
                return -ENODEV;
            }
            ACPI_COMPANION_SET((*cs35l56).base.dev, adev);
            acpi_dev_put(adev);
        }

        /* Initialize things that could be overwritten by a fixup */
        (*cs35l56).index = -1;
        let sub = acpi_get_subsystem_id(ACPI_HANDLE((*cs35l56).base.dev));
        ret = cs35l56_hda_apply_platform_fixups(cs35l56, sub, &mut id);
        if ret != 0 {
            return ret;
        }

        if (*cs35l56).index == -1 {
            property = c"cirrus,dev-index".as_ptr();
            ret = device_property_count_u32((*cs35l56).base.dev, property);
            if ret <= 0 {
                if ret != -ENODEV {
                    dev_err((*cs35l56).base.dev, c"Failed property %s: %d\n".as_ptr(), property, ret);
                }
                return ret;
            }
            if ret as usize > values.len() {
                ret = -EINVAL;
                dev_err((*cs35l56).base.dev, c"Failed property %s: %d\n".as_ptr(), property, ret);
                return ret;
            }
            (*cs35l56).num_amps = ret;
            ret = device_property_read_u32_array((*cs35l56).base.dev, property, values.as_mut_ptr(), (*cs35l56).num_amps);
            if ret != 0 {
                dev_err((*cs35l56).base.dev, c"Failed property %s: %d\n".as_ptr(), property, ret);
                return ret;
            }
            let mut i = 0;
            while i < (*cs35l56).num_amps {
                if values[i as usize] == id as u32 {
                    (*cs35l56).index = i;
                    break;
                }
                i += 1;
            }
            /*
             * It's not an error for the ID to be missing: for I2C there can be
             * an alias address that is not a real device. So reject silently.
             */
            if (*cs35l56).index == -1 {
                dev_dbg((*cs35l56).base.dev, c"No index found in %s\n".as_ptr(), property);
                return -ENODEV;
            }
        }

        if IS_ERR(sub as *const c_void) {
            dev_info((*cs35l56).base.dev, c"Read ACPI _SUB failed(%ld): fallback to generic firmware\n".as_ptr(), PTR_ERR(sub as *const c_void));
        } else {
            ret = cirrus_scodec_get_speaker_id((*cs35l56).base.dev, (*cs35l56).index, (*cs35l56).num_amps, -1);
            if ret == -ENOENT {
                (*cs35l56).system_name = devm_kstrdup((*cs35l56).base.dev, sub, GFP_KERNEL);
            } else if ret >= 0 {
                (*cs35l56).system_name = devm_kasprintf((*cs35l56).base.dev, GFP_KERNEL, c"%s-spkid%d".as_ptr(), sub, ret);
            } else {
                return ret;
            }
            if (*cs35l56).system_name.is_null() {
                return -ENOMEM;
            }
        }

        (*cs35l56).base.reset_gpio = devm_gpiod_get_index_optional((*cs35l56).base.dev, c"reset".as_ptr(), (*cs35l56).index as c_uint, GPIOD_OUT_LOW);
        if IS_ERR((*cs35l56).base.reset_gpio as *const c_void) {
            ret = PTR_ERR((*cs35l56).base.reset_gpio as *const c_void) as c_int;
            /*
             * If RESET is shared the first amp to probe will grab the reset
             * line and reset all the amps
             */
            if ret != -EBUSY {
                return dev_err_probe((*cs35l56).base.dev, ret, c"Failed to get reset GPIO\n".as_ptr());
            }
            dev_info((*cs35l56).base.dev, c"Reset GPIO busy, assume shared reset\n".as_ptr());
            (*cs35l56).base.reset_gpio = null_mut();
        }
        0
    }
}

unsafe extern "C" {
    fn mutex_init(lock: *mut mutex);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs35l56_hda_common_probe(cs35l56: *mut cs35l56_hda, hid: c_int, id: c_int) -> c_int {
    unsafe {
        let mut ret: c_int;
        mutex_init(core::ptr::addr_of_mut!((*cs35l56).base.irq_lock));
        dev_set_drvdata((*cs35l56).base.dev, cs35l56 as *mut c_void);
        INIT_WORK(core::ptr::addr_of_mut!((*cs35l56).dsp_work), cs35l56_hda_dsp_work);

        ret = cs35l56_hda_read_acpi(cs35l56, hid, id);
        if ret != 0 {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }
        (*cs35l56).amp_name = devm_kasprintf((*cs35l56).base.dev, GFP_KERNEL, c"AMP%d".as_ptr(), (*cs35l56).index + 1);
        if (*cs35l56).amp_name.is_null() {
            ret = -ENOMEM;
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }

        (*cs35l56).base.type_ = (hid & 0xff) as c_uint;
        (*cs35l56).base.cal_index = (*cs35l56).index;
        cs35l56_init_cs_dsp(core::ptr::addr_of_mut!((*cs35l56).base), core::ptr::addr_of_mut!((*cs35l56).cs_dsp));

        if !(*cs35l56).base.reset_gpio.is_null() {
            dev_dbg((*cs35l56).base.dev, c"Hard reset\n".as_ptr());
            /*
             * The GPIOD_OUT_LOW to *_gpiod_get_*() will be ignored if the
             * ACPI defines a different default state. So explicitly set low.
             */
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            cs35l56_wait_min_reset_pulse();
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 1);
        }

        ret = cs35l56_hw_init(core::ptr::addr_of_mut!((*cs35l56).base));
        if ret < 0 {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }
        /* Reset the device and wait for it to boot */
        cs35l56_system_reset(core::ptr::addr_of_mut!((*cs35l56).base), false);
        ret = cs35l56_wait_for_firmware_boot(core::ptr::addr_of_mut!((*cs35l56).base));
        if ret != 0 {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }
        regcache_cache_only((*cs35l56).base.regmap, false);
        ret = cs35l56_set_patch(core::ptr::addr_of_mut!((*cs35l56).base));
        if ret != 0 {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }
        regcache_mark_dirty((*cs35l56).base.regmap);
        regcache_sync((*cs35l56).base.regmap);
        /* Disable auto-hibernate so that runtime_pm has control */
        ret = cs35l56_mbox_send(core::ptr::addr_of_mut!((*cs35l56).base), CS35L56_MBOX_CMD_PREVENT_AUTO_HIBERNATE);
        if ret != 0 {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }
        ret = cs35l56_get_calibration(core::ptr::addr_of_mut!((*cs35l56).base));
        if ret != 0 {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }
        ret = cs_dsp_halo_init(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        if ret != 0 {
            dev_err_probe((*cs35l56).base.dev, ret, c"cs_dsp_halo_init failed\n".as_ptr());
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }
        dev_info((*cs35l56).base.dev, c"DSP system name: '%s', amp name: '%s'\n".as_ptr(), (*cs35l56).system_name, (*cs35l56).amp_name);
        regmap_multi_reg_write((*cs35l56).base.regmap, cs35l56_hda_dai_config.as_ptr(), cs35l56_hda_dai_config.len() as c_int);

        /*
         * By default only enable one ASP1TXn, where n=amplifier index,
         * This prevents multiple amps trying to drive the same slot.
         */
        (*cs35l56).asp_tx_mask = BIT((*cs35l56).index);

        pm_runtime_set_autosuspend_delay((*cs35l56).base.dev, 3000);
        pm_runtime_use_autosuspend((*cs35l56).base.dev);
        pm_runtime_set_active((*cs35l56).base.dev);
        pm_runtime_mark_last_busy((*cs35l56).base.dev);
        pm_runtime_enable((*cs35l56).base.dev);
        (*cs35l56).base.init_done = true;

        ret = component_add((*cs35l56).base.dev, &cs35l56_hda_comp_ops);
        if ret != 0 {
            dev_err((*cs35l56).base.dev, c"Register component failed: %d\n".as_ptr(), ret);
            pm_runtime_disable((*cs35l56).base.dev);
            cs_dsp_remove(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            return ret;
        }
        0
    }
}
/* EXPORT_SYMBOL_NS_GPL(cs35l56_hda_common_probe, "SND_HDA_SCODEC_CS35L56"); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs35l56_hda_remove(dev: *mut device) {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev) as *mut cs35l56_hda;
        component_del((*cs35l56).base.dev, &cs35l56_hda_comp_ops);
        pm_runtime_dont_use_autosuspend((*cs35l56).base.dev);
        pm_runtime_get_sync((*cs35l56).base.dev);
        pm_runtime_disable((*cs35l56).base.dev);
        cs_dsp_remove(core::ptr::addr_of_mut!((*cs35l56).cs_dsp));
        pm_runtime_put_noidle((*cs35l56).base.dev);
        gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
    }
}
/* EXPORT_SYMBOL_NS_GPL(cs35l56_hda_remove, "SND_HDA_SCODEC_CS35L56"); */

#[unsafe(no_mangle)]
pub static cs35l56_hda_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(cs35l56_hda_runtime_suspend),
    runtime_resume: Some(cs35l56_hda_runtime_resume),
    suspend: Some(cs35l56_hda_system_suspend),
    resume: Some(cs35l56_hda_system_resume),
    suspend_late: Some(cs35l56_hda_system_suspend_late),
    resume_early: Some(cs35l56_hda_system_resume_early),
    suspend_noirq: Some(cs35l56_hda_system_suspend_no_irq),
    resume_noirq: Some(cs35l56_hda_system_resume_no_irq),
};
/* EXPORT_SYMBOL_NS_GPL(cs35l56_hda_pm_ops, "SND_HDA_SCODEC_CS35L56"); */

/* MODULE_DESCRIPTION("CS35L56 HDA Driver"); */
/* MODULE_IMPORT_NS("FW_CS_DSP"); */
/* MODULE_IMPORT_NS("SND_HDA_CIRRUS_SCODEC"); */
/* MODULE_IMPORT_NS("SND_SOC_CS35L56_SHARED"); */
/* MODULE_IMPORT_NS("SND_SOC_CS_AMP_LIB"); */
/* MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>"); */
/* MODULE_AUTHOR("Simon Trimmer <simont@opensource.cirrus.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_FIRMWARE("cirrus/cs35l54-*.wmfw"); */
/* MODULE_FIRMWARE("cirrus/cs35l54-*.bin"); */
/* MODULE_FIRMWARE("cirrus/cs35l56-*.wmfw"); */
/* MODULE_FIRMWARE("cirrus/cs35l56-*.bin"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
