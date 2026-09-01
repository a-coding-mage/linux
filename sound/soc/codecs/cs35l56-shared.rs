// SPDX-License-Identifier: GPL-2.0-only
//
// Components shared between ASoC and HDA CS35L56 drivers
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.
//
// Rust translation of cs35l56-shared.c. C include dependencies are expected to
// provide the referenced kernel, regmap, DSP, GPIO, regulator and CS35L56 items.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type u8 = core::ffi::c_uchar;
type u32 = c_uint;
type u64 = core::ffi::c_ulonglong;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type irqreturn_t = c_int;
type __be32 = u32;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct spi_controller { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)]
pub union snd_ctl_elem_value_value { pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 4] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
    pub delay_us: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l56_fw_reg {
    pub fw_ver: c_uint,
    pub halo_state: c_uint,
    pub pm_cur_stat: c_uint,
    pub prot_sts: c_uint,
    pub transducer_actual_ps: c_uint,
    pub user_mute: c_uint,
    pub user_volume: c_uint,
    pub posture_number: c_uint,
}

#[repr(C)]
pub struct cs35l56_spi_payload {
    pub addr: __be32,
    pub value: __be32,
}

#[repr(C)]
pub struct spi_device {
    pub controller: *mut spi_controller,
}

#[repr(C)]
pub struct spi_transfer {
    pub tx_buf: *mut c_void,
    pub len: c_uint,
}

#[repr(C)]
pub struct spi_message { _private: [u8; 0] }

#[repr(C)]
pub struct cs_dsp_region {
    pub type_: c_uint,
    pub base: c_uint,
}

#[repr(C)]
pub struct cs_dsp {
    pub num: c_int,
    pub type_: c_int,
    pub rev: c_int,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub base: c_uint,
    pub base_sysinfo: c_uint,
    pub mem: *const cs_dsp_region,
    pub num_mems: c_uint,
    pub no_core_startstop: bool_,
    pub pwr_lock: mutex,
}

#[repr(C)]
pub struct cirrus_amp_cal_controls {
    pub alg_id: c_uint,
    pub mem_region: c_uint,
    pub ambient: *const c_char,
    pub calr: *const c_char,
    pub status: *const c_char,
    pub checksum: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_amp_cal_data {
    pub calTime: [u32; 2],
    pub calTarget: [u32; 2],
    pub calStatus: u32,
    pub calR: u32,
    pub calAmbient: u32,
}

#[repr(C)]
pub struct cs_dsp_coeff_ctl { _private: [u8; 0] }

#[repr(C)]
pub struct file_operations { _private: [u8; 0] }

#[repr(C)]
pub struct cs35l56_cal_debugfs_fops {
    pub calibrate: file_operations,
    pub cal_temperature: file_operations,
    pub cal_data: file_operations,
}

#[repr(C)]
pub struct gpio_descs {
    pub ndescs: c_uint,
    pub desc: [*mut gpio_desc; 0],
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub pad_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_stride: c_uint,
    pub reg_base: c_uint,
    pub reg_format_endian: c_uint,
    pub val_format_endian: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct cs35l56_base {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub type_: c_uint,
    pub rev: c_uint,
    pub fw_reg: *const cs35l56_fw_reg,
    pub spi_payload_buf: *mut cs35l56_spi_payload,
    pub init_done: bool_,
    pub irq_lock: mutex,
    pub irq: c_int,
    pub secured: bool_,
    pub can_hibernate: bool_,
    pub dsp: *mut cs_dsp,
    pub silicon_uid: u64,
    pub cal_index: c_int,
    pub cal_data: cirrus_amp_cal_data,
    pub cal_data_valid: bool_,
    pub calibration_controls: *const cirrus_amp_cal_controls,
    pub num_amps: c_int,
    pub debugfs: *mut dentry,
    pub reset_gpio: *mut c_void,
    pub onchip_spkid_gpios: [u32; 8],
    pub onchip_spkid_pulls: [u32; 8],
    pub num_onchip_spkid_gpios: c_int,
    pub num_onchip_spkid_pulls: c_int,
}

unsafe extern "C" {
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_read_bypassed(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, len: size_t) -> c_int;
    fn regmap_bulk_read(map: *mut regmap, reg: c_uint, val: *mut c_void, count: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool_) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_multi_reg_write_bypassed(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_put(dev: *mut device);
    fn usleep_range(min: c_uint, max: c_uint);
    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn spi_bus_lock(controller: *mut spi_controller) -> c_int;
    fn spi_bus_unlock(controller: *mut spi_controller);
    fn spi_message_init_with_transfers(m: *mut spi_message, t: *mut spi_transfer, n: c_uint);
    fn spi_sync_locked(spi: *mut spi_device, m: *mut spi_message) -> c_int;
    fn cs35l56_is_spi(base: *mut cs35l56_base) -> bool_;
    fn cs_amp_get_efi_calibration_data(dev: *mut device, uid: u64, index: c_int, data: *mut cirrus_amp_cal_data) -> c_int;
    fn cs_amp_set_efi_calibration_data(dev: *mut device, index: c_int, num_amps: c_int, data: *const cirrus_amp_cal_data) -> c_int;
    fn cs_amp_cal_target_u64(data: *const cirrus_amp_cal_data) -> u64;
    fn cs_dsp_get_ctl(dsp: *mut cs_dsp, name: *const c_char, mem: c_uint, alg: c_uint) -> *mut cs_dsp_coeff_ctl;
    fn cs_dsp_coeff_write_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: c_uint, val: *const c_void, len: size_t) -> c_int;
    fn cs_dsp_coeff_read_ctrl(ctl: *mut cs_dsp_coeff_ctl, off: c_uint, val: *mut c_void, len: size_t) -> c_int;
    fn cs_amp_read_cal_coeffs(dsp: *mut cs_dsp, controls: *const cirrus_amp_cal_controls, data: *mut cirrus_amp_cal_data) -> c_int;
    fn cs_amp_write_ambient_temp(dsp: *mut cs_dsp, controls: *const cirrus_amp_cal_controls, val: c_ulong) -> c_int;
    fn simple_write_to_buffer(to: *mut c_void, available: size_t, ppos: *mut loff_t, from: *const c_char, count: size_t) -> ssize_t;
    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_void, available: size_t) -> ssize_t;
    fn sysfs_match_string(strings: *const *const c_char, str_: *const c_char) -> c_int;
    fn kstrtoul_from_user(from: *const c_char, count: size_t, base: c_uint, res: *mut c_ulong) -> c_int;
    fn cs_amp_create_debugfs(dev: *mut device) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(root: *mut dentry);
    fn ERR_PTR(err: c_int) -> *mut dentry;
    fn cs_amp_get_vendor_spkid(dev: *mut device) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut u32) -> c_int;
    fn gpiod_get_array_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_descs;
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_put_array(descs: *mut gpio_descs);
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_crit(dev: *mut device, fmt: *const c_char, ...);
}

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EIO: c_int = 5;
const ENOENT: c_int = 2;
const EOVERFLOW: c_int = 75;
const ENODATA: c_int = 61;
const EINVAL: c_int = 22;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const ERANGE: c_int = 34;
const ETIMEDOUT: c_int = 110;
const EOPNOTSUPP: c_int = 95;
const ENODEV: c_int = 19;
const EMSGSIZE: c_int = 90;
const IRQF_ONESHOT: c_uint = 0x00002000;
const IRQF_SHARED: c_uint = 0x00000080;
const IRQF_TRIGGER_LOW: c_uint = 0x00000008;
const GPIOD_IN: c_uint = 0;
const REGMAP_ENDIAN_BIG: c_uint = 1;
const REGMAP_ENDIAN_LITTLE: c_uint = 2;
const REGCACHE_MAPLE: c_uint = 4;

macro_rules! ARRAY_SIZE { ($a:expr) => { $a.len() as c_int }; }
macro_rules! REG_SEQ0 { ($r:expr, $d:expr) => { reg_sequence { reg: $r, def: $d, delay_us: 0 } }; }
macro_rules! BIT { ($n:expr) => { 1u32 << ($n as u32) }; }
macro_rules! FIELD_PREP { ($mask:expr, $val:expr) => { (($val as u32) << (($mask as u32).trailing_zeros())) & ($mask as u32) }; }
macro_rules! IS_ENABLED { ($cfg:ident) => { false }; }
macro_rules! EXPORT_SYMBOL_NS_GPL { ($sym:ident, $ns:expr) => {}; }

fn cpu_to_be32(v: u32) -> __be32 { v.to_be() }
fn be32_to_cpu(v: __be32) -> u32 { u32::from_be(v) }

unsafe fn regmap_read_poll_timeout_eq_zero(map: *mut regmap, reg: c_uint, val: *mut c_uint, poll_us: c_uint, timeout_us: c_uint) -> c_int {
    let ret = regmap_read(map, reg, val);
    if ret != 0 { return ret; }
    if *val == 0 { 0 } else { -ETIMEDOUT }
}

unsafe fn regmap_read_poll_timeout_ge(map: *mut regmap, reg: c_uint, val: *mut c_uint, target: c_uint, _poll_us: c_uint, _timeout_us: c_uint) -> c_int {
    let ret = regmap_read(map, reg, val);
    if ret != 0 { return ret; }
    if *val >= target { 0 } else { -ETIMEDOUT }
}

unsafe fn read_poll_regmap_read_bypassed_boot(map: *mut regmap, reg: c_uint, val: *mut c_uint, read_ret: *mut c_int) -> c_int {
    *read_ret = regmap_read_bypassed(map, reg, val);
    if *read_ret != 0 { return *read_ret; }
    if (*val < 0xFFFF) && (*val >= CS35L56_HALO_STATE_BOOT_DONE) { 0 } else { -ETIMEDOUT }
}

unsafe fn read_poll_regmap_read_bypassed_mbox2(map: *mut regmap, reg: c_uint, val: *mut c_uint, read_ret: *mut c_int) -> c_int {
    *read_ret = regmap_read_bypassed(map, reg, val);
    if *read_ret != 0 { return *read_ret; }
    if (*val > 0) && (*val < 0xffffffff) { 0 } else { -ETIMEDOUT }
}

unsafe fn read_poll_cs_dsp_coeff_read_ctrl_eq_one(ctl: *mut cs_dsp_coeff_ctl, val: *mut __be32, ret: *mut c_int) -> c_int {
    *ret = cs_dsp_coeff_read_ctrl(ctl, 0, val as *mut c_void, size_of::<__be32>());
    if *ret != 0 { return *ret; }
    if *val == cpu_to_be32(1) { 0 } else { -ETIMEDOUT }
}

static cs35l56_asp_patch: [reg_sequence; 12] = [
    /* Firmware can change these to non-defaults to satisfy SDCA.
     * Ensure that they are at known defaults.
     */
    REG_SEQ0!(CS35L56_ASP1_ENABLES1, 0x00000000),
    REG_SEQ0!(CS35L56_ASP1_CONTROL1, 0x00000028),
    REG_SEQ0!(CS35L56_ASP1_CONTROL2, 0x18180200),
    REG_SEQ0!(CS35L56_ASP1_CONTROL3, 0x00000002),
    REG_SEQ0!(CS35L56_ASP1_FRAME_CONTROL1, 0x03020100),
    REG_SEQ0!(CS35L56_ASP1_FRAME_CONTROL5, 0x00020100),
    REG_SEQ0!(CS35L56_ASP1_DATA_CONTROL1, 0x00000018),
    REG_SEQ0!(CS35L56_ASP1_DATA_CONTROL5, 0x00000018),
    REG_SEQ0!(CS35L56_ASP1TX1_INPUT, 0x00000000),
    REG_SEQ0!(CS35L56_ASP1TX2_INPUT, 0x00000000),
    REG_SEQ0!(CS35L56_ASP1TX3_INPUT, 0x00000000),
    REG_SEQ0!(CS35L56_ASP1TX4_INPUT, 0x00000000),
];

#[no_mangle]
pub unsafe extern "C" fn cs35l56_set_asp_patch(cs35l56_base: *mut cs35l56_base) -> c_int {
    regmap_register_patch((*cs35l56_base).regmap, cs35l56_asp_patch.as_ptr(), ARRAY_SIZE!(cs35l56_asp_patch))
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_set_asp_patch, "SND_SOC_CS35L56_SHARED");

static cs35l56_patch: [reg_sequence; 5] = [
    /* Firmware can change these to non-defaults to satisfy SDCA.
     * Ensure that they are at known defaults.
     */
    REG_SEQ0!(CS35L56_SWIRE_DP3_CH1_INPUT, 0x00000018),
    REG_SEQ0!(CS35L56_SWIRE_DP3_CH2_INPUT, 0x00000019),
    REG_SEQ0!(CS35L56_SWIRE_DP3_CH3_INPUT, 0x00000029),
    REG_SEQ0!(CS35L56_SWIRE_DP3_CH4_INPUT, 0x00000028),
    REG_SEQ0!(CS35L56_IRQ1_MASK_18, 0x1f7df0ff),
];

static cs35l56_patch_fw: [reg_sequence; 3] = [
    /* These are not reset by a soft-reset, so patch to defaults. */
    REG_SEQ0!(CS35L56_MAIN_RENDER_USER_MUTE, 0x00000000),
    REG_SEQ0!(CS35L56_MAIN_RENDER_USER_VOLUME, 0x00000000),
    REG_SEQ0!(CS35L56_MAIN_POSTURE_NUMBER, 0x00000000),
];

static cs35l63_patch_fw: [reg_sequence; 3] = [
    /* These are not reset by a soft-reset, so patch to defaults. */
    REG_SEQ0!(CS35L63_MAIN_RENDER_USER_MUTE, 0x00000000),
    REG_SEQ0!(CS35L63_MAIN_RENDER_USER_VOLUME, 0x00000000),
    REG_SEQ0!(CS35L63_MAIN_POSTURE_NUMBER, 0x00000000),
];

#[no_mangle]
pub unsafe extern "C" fn cs35l56_set_patch(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut ret: c_int;
    ret = regmap_register_patch((*cs35l56_base).regmap, cs35l56_patch.as_ptr(), ARRAY_SIZE!(cs35l56_patch));
    if ret != 0 { return ret; }
    match (*cs35l56_base).type_ {
        0x54 | 0x56 | 0x57 => {
            ret = regmap_register_patch((*cs35l56_base).regmap, cs35l56_patch_fw.as_ptr(), ARRAY_SIZE!(cs35l56_patch_fw));
        }
        0x63 | 0x62 => {
            ret = regmap_register_patch((*cs35l56_base).regmap, cs35l63_patch_fw.as_ptr(), ARRAY_SIZE!(cs35l63_patch_fw));
        }
        _ => {}
    }
    ret
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_set_patch, "SND_SOC_CS35L56_SHARED");

static cs35l56_reg_defaults: [reg_default; 25] = [
    reg_default { reg: CS35L56_ASP1_ENABLES1, def: 0x00000000 },
    reg_default { reg: CS35L56_ASP1_CONTROL1, def: 0x00000028 },
    reg_default { reg: CS35L56_ASP1_CONTROL2, def: 0x18180200 },
    reg_default { reg: CS35L56_ASP1_CONTROL3, def: 0x00000002 },
    reg_default { reg: CS35L56_ASP1_FRAME_CONTROL1, def: 0x03020100 },
    reg_default { reg: CS35L56_ASP1_FRAME_CONTROL5, def: 0x00020100 },
    reg_default { reg: CS35L56_ASP1_DATA_CONTROL1, def: 0x00000018 },
    reg_default { reg: CS35L56_ASP1_DATA_CONTROL5, def: 0x00000018 },
    reg_default { reg: CS35L56_ASP1TX1_INPUT, def: 0x00000000 },
    reg_default { reg: CS35L56_ASP1TX2_INPUT, def: 0x00000000 },
    reg_default { reg: CS35L56_ASP1TX3_INPUT, def: 0x00000000 },
    reg_default { reg: CS35L56_ASP1TX4_INPUT, def: 0x00000000 },
    reg_default { reg: CS35L56_SWIRE_DP3_CH1_INPUT, def: 0x00000018 },
    reg_default { reg: CS35L56_SWIRE_DP3_CH2_INPUT, def: 0x00000019 },
    reg_default { reg: CS35L56_SWIRE_DP3_CH3_INPUT, def: 0x00000029 },
    reg_default { reg: CS35L56_SWIRE_DP3_CH4_INPUT, def: 0x00000028 },
    reg_default { reg: CS35L56_IRQ1_MASK_1, def: 0x83ffffff },
    reg_default { reg: CS35L56_IRQ1_MASK_2, def: 0xffff7fff },
    reg_default { reg: CS35L56_IRQ1_MASK_4, def: 0xe0ffffff },
    reg_default { reg: CS35L56_IRQ1_MASK_8, def: 0xfc000fff },
    reg_default { reg: CS35L56_IRQ1_MASK_18, def: 0x1f7df0ff },
    reg_default { reg: CS35L56_IRQ1_MASK_20, def: 0x15c00000 },
    reg_default { reg: CS35L56_MAIN_RENDER_USER_MUTE, def: 0x00000000 },
    reg_default { reg: CS35L56_MAIN_RENDER_USER_VOLUME, def: 0x00000000 },
    reg_default { reg: CS35L56_MAIN_POSTURE_NUMBER, def: 0x00000000 },
];

static cs35l63_reg_defaults: [reg_default; 25] = [
    reg_default { reg: CS35L56_ASP1_ENABLES1, def: 0x00000000 },
    reg_default { reg: CS35L56_ASP1_CONTROL1, def: 0x00000028 },
    reg_default { reg: CS35L56_ASP1_CONTROL2, def: 0x18180200 },
    reg_default { reg: CS35L56_ASP1_CONTROL3, def: 0x00000002 },
    reg_default { reg: CS35L56_ASP1_FRAME_CONTROL1, def: 0x03020100 },
    reg_default { reg: CS35L56_ASP1_FRAME_CONTROL5, def: 0x00020100 },
    reg_default { reg: CS35L56_ASP1_DATA_CONTROL1, def: 0x00000018 },
    reg_default { reg: CS35L56_ASP1_DATA_CONTROL5, def: 0x00000018 },
    reg_default { reg: CS35L56_ASP1TX1_INPUT, def: 0x00000000 },
    reg_default { reg: CS35L56_ASP1TX2_INPUT, def: 0x00000000 },
    reg_default { reg: CS35L56_ASP1TX3_INPUT, def: 0x00000000 },
    reg_default { reg: CS35L56_ASP1TX4_INPUT, def: 0x00000000 },
    reg_default { reg: CS35L56_SWIRE_DP3_CH1_INPUT, def: 0x00000018 },
    reg_default { reg: CS35L56_SWIRE_DP3_CH2_INPUT, def: 0x00000019 },
    reg_default { reg: CS35L56_SWIRE_DP3_CH3_INPUT, def: 0x00000029 },
    reg_default { reg: CS35L56_SWIRE_DP3_CH4_INPUT, def: 0x00000028 },
    reg_default { reg: CS35L56_IRQ1_MASK_1, def: 0x8003ffff },
    reg_default { reg: CS35L56_IRQ1_MASK_2, def: 0xffff7fff },
    reg_default { reg: CS35L56_IRQ1_MASK_4, def: 0xe0ffffff },
    reg_default { reg: CS35L56_IRQ1_MASK_8, def: 0x8c000fff },
    reg_default { reg: CS35L56_IRQ1_MASK_18, def: 0x0760f000 },
    reg_default { reg: CS35L56_IRQ1_MASK_20, def: 0x15c00000 },
    reg_default { reg: CS35L63_MAIN_RENDER_USER_MUTE, def: 0x00000000 },
    reg_default { reg: CS35L63_MAIN_RENDER_USER_VOLUME, def: 0x00000000 },
    reg_default { reg: CS35L63_MAIN_POSTURE_NUMBER, def: 0x00000000 },
];

unsafe extern "C" fn cs35l56_is_dsp_memory(reg: c_uint) -> bool_ {
    match reg {
        CS35L56_DSP1_XMEM_PACKED_0..=CS35L56_DSP1_XMEM_PACKED_6143 |
        CS35L56_DSP1_XMEM_UNPACKED32_0..=CS35L56_DSP1_XMEM_UNPACKED32_4095 |
        CS35L56_DSP1_XMEM_UNPACKED24_0..=CS35L56_DSP1_XMEM_UNPACKED24_8191 |
        CS35L56_DSP1_YMEM_PACKED_0..=CS35L56_DSP1_YMEM_PACKED_4604 |
        CS35L56_DSP1_YMEM_UNPACKED32_0..=CS35L56_DSP1_YMEM_UNPACKED32_3070 |
        CS35L56_DSP1_YMEM_UNPACKED24_0..=CS35L56_DSP1_YMEM_UNPACKED24_6141 |
        CS35L56_DSP1_PMEM_0..=CS35L56_DSP1_PMEM_5114 => true,
        _ => false,
    }
}

unsafe extern "C" fn cs35l56_readable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        CS35L56_DEVID | CS35L56_REVID | CS35L56_RELID | CS35L56_OTPID |
        CS35L56_SFT_RESET | CS35L56_GLOBAL_ENABLES | CS35L56_BLOCK_ENABLES |
        CS35L56_BLOCK_ENABLES2 | CS35L56_REFCLK_INPUT | CS35L56_GLOBAL_SAMPLE_RATE |
        CS35L56_OTP_MEM_53 | CS35L56_OTP_MEM_54 | CS35L56_OTP_MEM_55 |
        CS35L56_UPDATE_REGS | CS35L56_ASP1_ENABLES1 | CS35L56_ASP1_CONTROL1 |
        CS35L56_ASP1_CONTROL2 | CS35L56_ASP1_CONTROL3 | CS35L56_ASP1_FRAME_CONTROL1 |
        CS35L56_ASP1_FRAME_CONTROL5 | CS35L56_ASP1_DATA_CONTROL1 | CS35L56_ASP1_DATA_CONTROL5 |
        CS35L56_DACPCM1_INPUT | CS35L56_DACPCM2_INPUT | CS35L56_ASP1TX1_INPUT |
        CS35L56_ASP1TX2_INPUT | CS35L56_ASP1TX3_INPUT | CS35L56_ASP1TX4_INPUT |
        CS35L56_DSP1RX1_INPUT | CS35L56_DSP1RX2_INPUT | CS35L56_SWIRE_DP3_CH1_INPUT |
        CS35L56_SWIRE_DP3_CH2_INPUT | CS35L56_SWIRE_DP3_CH3_INPUT | CS35L56_SWIRE_DP3_CH4_INPUT |
        CS35L56_IRQ1_CFG | CS35L56_IRQ1_STATUS | CS35L56_IRQ1_EINT_18 |
        CS35L56_IRQ1_EINT_20 | CS35L56_IRQ1_MASK_1 | CS35L56_IRQ1_MASK_2 |
        CS35L56_IRQ1_MASK_4 | CS35L56_IRQ1_MASK_8 | CS35L56_IRQ1_MASK_18 |
        CS35L56_IRQ1_MASK_20 | CS35L56_MIXER_NGATE_CH1_CFG | CS35L56_MIXER_NGATE_CH2_CFG |
        CS35L56_DSP_VIRTUAL1_MBOX_1 | CS35L56_DSP_VIRTUAL1_MBOX_2 | CS35L56_DSP_VIRTUAL1_MBOX_3 |
        CS35L56_DSP_VIRTUAL1_MBOX_4 | CS35L56_DSP_VIRTUAL1_MBOX_5 | CS35L56_DSP_VIRTUAL1_MBOX_6 |
        CS35L56_DSP_VIRTUAL1_MBOX_7 | CS35L56_DSP_VIRTUAL1_MBOX_8 | CS35L56_DIE_STS1 |
        CS35L56_DIE_STS2 | CS35L56_DSP_RESTRICT_STS1 | CS35L56_DSP1_AHBM_WINDOW_DEBUG_0 |
        CS35L56_DSP1_AHBM_WINDOW_DEBUG_1 | CS35L56_DSP1_SCRATCH1 | CS35L56_DSP1_SCRATCH2 |
        CS35L56_DSP1_SCRATCH3 | CS35L56_DSP1_SCRATCH4 => true,
        CS35L56_SYNC_GPIO1_CFG..=CS35L56_ASP2_DIO_GPIO13_CFG |
        CS35L56_IRQ1_EINT_1..=CS35L56_IRQ1_EINT_8 |
        CS35L56_GPIO_STATUS1..=CS35L56_GPIO13_CTRL1 |
        CS35L56_DSP1_SYS_INFO_ID..=CS35L56_DSP1_SYS_INFO_END => true,
        _ => cs35l56_is_dsp_memory(reg),
    }
}

unsafe extern "C" fn cs35l56_precious_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        CS35L56_DSP1_XMEM_PACKED_0..=CS35L56_DSP1_XMEM_PACKED_6143 |
        CS35L56_DSP1_YMEM_PACKED_0..=CS35L56_DSP1_YMEM_PACKED_4604 |
        CS35L56_DSP1_PMEM_0..=CS35L56_DSP1_PMEM_5114 => true,
        _ => false,
    }
}

unsafe fn cs35l56_common_volatile_reg(reg: c_uint) -> bool_ {
    match reg {
        CS35L56_DEVID | CS35L56_REVID | CS35L56_RELID | CS35L56_OTPID |
        CS35L56_SFT_RESET |
        CS35L56_GLOBAL_ENABLES | /* owned by firmware */
        CS35L56_BLOCK_ENABLES |  /* owned by firmware */
        CS35L56_BLOCK_ENABLES2 | /* owned by firmware */
        CS35L56_OTP_MEM_53 | CS35L56_OTP_MEM_54 | CS35L56_OTP_MEM_55 |
        CS35L56_UPDATE_REGS |
        CS35L56_REFCLK_INPUT |        /* owned by firmware */
        CS35L56_GLOBAL_SAMPLE_RATE |  /* owned by firmware */
        CS35L56_DACPCM1_INPUT |       /* owned by firmware */
        CS35L56_DACPCM2_INPUT |       /* owned by firmware */
        CS35L56_DSP1RX1_INPUT |       /* owned by firmware */
        CS35L56_DSP1RX2_INPUT |       /* owned by firmware */
        CS35L56_IRQ1_STATUS | CS35L56_IRQ1_EINT_18 | CS35L56_IRQ1_EINT_20 |
        CS35L56_MIXER_NGATE_CH1_CFG | CS35L56_MIXER_NGATE_CH2_CFG |
        CS35L56_DSP_VIRTUAL1_MBOX_1 | CS35L56_DSP_VIRTUAL1_MBOX_2 |
        CS35L56_DSP_VIRTUAL1_MBOX_3 | CS35L56_DSP_VIRTUAL1_MBOX_4 |
        CS35L56_DSP_VIRTUAL1_MBOX_5 | CS35L56_DSP_VIRTUAL1_MBOX_6 |
        CS35L56_DSP_VIRTUAL1_MBOX_7 | CS35L56_DSP_VIRTUAL1_MBOX_8 |
        CS35L56_DSP_RESTRICT_STS1 | CS35L56_DSP1_AHBM_WINDOW_DEBUG_0 |
        CS35L56_DSP1_AHBM_WINDOW_DEBUG_1 | CS35L56_DSP1_SCRATCH1 |
        CS35L56_DSP1_SCRATCH2 | CS35L56_DSP1_SCRATCH3 | CS35L56_DSP1_SCRATCH4 => true,
        CS35L56_SYNC_GPIO1_CFG..=CS35L56_ASP2_DIO_GPIO13_CFG |
        CS35L56_IRQ1_EINT_1..=CS35L56_IRQ1_EINT_8 |
        CS35L56_GPIO_STATUS1..=CS35L56_GPIO13_CTRL1 |
        CS35L56_DSP1_SYS_INFO_ID..=CS35L56_DSP1_SYS_INFO_END => true,
        _ => cs35l56_is_dsp_memory(reg),
    }
}

unsafe extern "C" fn cs35l56_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        CS35L56_MAIN_RENDER_USER_MUTE | CS35L56_MAIN_RENDER_USER_VOLUME | CS35L56_MAIN_POSTURE_NUMBER => false,
        _ => cs35l56_common_volatile_reg(reg),
    }
}

unsafe extern "C" fn cs35l63_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        CS35L63_MAIN_RENDER_USER_MUTE | CS35L63_MAIN_RENDER_USER_VOLUME | CS35L63_MAIN_POSTURE_NUMBER => false,
        _ => cs35l56_common_volatile_reg(reg),
    }
}

static cs35l56_fw_reg: cs35l56_fw_reg = cs35l56_fw_reg {
    fw_ver: CS35L56_DSP1_FW_VER,
    halo_state: CS35L56_DSP1_HALO_STATE,
    pm_cur_stat: CS35L56_DSP1_PM_CUR_STATE,
    prot_sts: CS35L56_PROTECTION_STATUS,
    transducer_actual_ps: CS35L56_TRANSDUCER_ACTUAL_PS,
    user_mute: CS35L56_MAIN_RENDER_USER_MUTE,
    user_volume: CS35L56_MAIN_RENDER_USER_VOLUME,
    posture_number: CS35L56_MAIN_POSTURE_NUMBER,
};

static cs35l56_b2_fw_reg: cs35l56_fw_reg = cs35l56_fw_reg {
    fw_ver: CS35L56_DSP1_FW_VER,
    halo_state: CS35L56_B2_DSP1_HALO_STATE,
    pm_cur_stat: CS35L56_B2_DSP1_PM_CUR_STATE,
    prot_sts: CS35L56_PROTECTION_STATUS,
    transducer_actual_ps: CS35L56_TRANSDUCER_ACTUAL_PS,
    user_mute: CS35L56_MAIN_RENDER_USER_MUTE,
    user_volume: CS35L56_MAIN_RENDER_USER_VOLUME,
    posture_number: CS35L56_MAIN_POSTURE_NUMBER,
};

static cs35l63_fw_reg: cs35l56_fw_reg = cs35l56_fw_reg {
    fw_ver: CS35L63_DSP1_FW_VER,
    halo_state: CS35L63_DSP1_HALO_STATE,
    pm_cur_stat: CS35L63_DSP1_PM_CUR_STATE,
    prot_sts: CS35L63_PROTECTION_STATUS,
    transducer_actual_ps: CS35L63_TRANSDUCER_ACTUAL_PS,
    user_mute: CS35L63_MAIN_RENDER_USER_MUTE,
    user_volume: CS35L63_MAIN_RENDER_USER_VOLUME,
    posture_number: CS35L63_MAIN_POSTURE_NUMBER,
};

unsafe fn cs35l56_set_fw_reg_table(cs35l56_base: *mut cs35l56_base) {
    match (*cs35l56_base).type_ {
        0x63 | 0x62 => (*cs35l56_base).fw_reg = &cs35l63_fw_reg,
        _ => match (*cs35l56_base).rev {
            0xb0 => (*cs35l56_base).fw_reg = &cs35l56_fw_reg,
            _ => (*cs35l56_base).fw_reg = &cs35l56_b2_fw_reg,
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_mbox_send(cs35l56_base: *mut cs35l56_base, command: c_uint) -> c_int {
    let mut val: c_uint = 0;
    regmap_write((*cs35l56_base).regmap, CS35L56_DSP_VIRTUAL1_MBOX_1, command);
    let ret = regmap_read_poll_timeout_eq_zero((*cs35l56_base).regmap, CS35L56_DSP_VIRTUAL1_MBOX_1, &mut val, CS35L56_MBOX_POLL_US, CS35L56_MBOX_TIMEOUT_US);
    if ret != 0 {
        dev_warn((*cs35l56_base).dev, b"MBOX command %#x failed: %d\n\0".as_ptr() as *const c_char, command, ret);
        return ret;
    }
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_mbox_send, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_firmware_shutdown(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut val: c_uint = 0;
    let mut ret = cs35l56_mbox_send(cs35l56_base, CS35L56_MBOX_CMD_SHUTDOWN);
    if ret != 0 { return ret; }
    ret = regmap_read_poll_timeout_ge((*cs35l56_base).regmap, (*(*cs35l56_base).fw_reg).pm_cur_stat, &mut val, CS35L56_HALO_STATE_SHUTDOWN, CS35L56_HALO_STATE_POLL_US, CS35L56_HALO_STATE_TIMEOUT_US);
    if ret < 0 {
        dev_err((*cs35l56_base).dev, b"Failed to poll PM_CUR_STATE to 1 is %d (ret %d)\n\0".as_ptr() as *const c_char, val, ret);
    }
    ret
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_firmware_shutdown, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_wait_for_firmware_boot(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut val: c_uint = 0;
    let mut read_ret: c_int = 0;
    /* The regmap must remain in cache-only until the chip has booted, so use a bypassed read of the status register. */
    let poll_ret = read_poll_regmap_read_bypassed_boot((*cs35l56_base).regmap, (*(*cs35l56_base).fw_reg).halo_state, &mut val, &mut read_ret);
    if poll_ret != 0 {
        dev_err((*cs35l56_base).dev, b"Firmware boot timed out(%d): HALO_STATE=%#x\n\0".as_ptr() as *const c_char, read_ret, val);
        return -EIO;
    }
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_wait_for_firmware_boot, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_wait_control_port_ready() {
    /* Wait for control port to be ready (datasheet tIRS). */
    usleep_range(CS35L56_CONTROL_PORT_READY_US, 2 * CS35L56_CONTROL_PORT_READY_US);
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_wait_control_port_ready, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_wait_min_reset_pulse() {
    /* Satisfy minimum reset pulse width spec */
    usleep_range(CS35L56_RESET_PULSE_MIN_US, 2 * CS35L56_RESET_PULSE_MIN_US);
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_wait_min_reset_pulse, "SND_SOC_CS35L56_SHARED");

#[repr(C)]
#[derive(Copy, Clone)]
struct cs35l56_spi_system_reset_stage { addr: u32, value: u32 }

static cs35l56_spi_system_reset_stages: [cs35l56_spi_system_reset_stage; 2] = [
    cs35l56_spi_system_reset_stage { addr: CS35L56_DSP_VIRTUAL1_MBOX_1, value: CS35L56_MBOX_CMD_SYSTEM_RESET },
    /* The next write is necessary to delimit the soft reset */
    cs35l56_spi_system_reset_stage { addr: CS35L56_DSP_MBOX_1_RAW, value: CS35L56_MBOX_CMD_PING },
];

unsafe fn cs35l56_spi_issue_bus_locked_reset(cs35l56_base: *mut cs35l56_base, spi: *mut spi_device) {
    let buf = (*cs35l56_base).spi_payload_buf;
    let mut t = spi_transfer { tx_buf: buf as *mut c_void, len: size_of::<cs35l56_spi_payload>() as c_uint };
    let mut m: spi_message = zeroed();
    for i in 0..cs35l56_spi_system_reset_stages.len() {
        (*buf).addr = cpu_to_be32(cs35l56_spi_system_reset_stages[i].addr);
        (*buf).value = cpu_to_be32(cs35l56_spi_system_reset_stages[i].value);
        spi_message_init_with_transfers(&mut m, &mut t, 1);
        let ret = spi_sync_locked(spi, &mut m);
        if ret != 0 {
            dev_warn((*cs35l56_base).dev, b"spi_sync failed: %d\n\0".as_ptr() as *const c_char, ret);
        }
        usleep_range(CS35L56_SPI_RESET_TO_PORT_READY_US, 2 * CS35L56_SPI_RESET_TO_PORT_READY_US);
    }
}

unsafe fn cs35l56_spi_system_reset(cs35l56_base: *mut cs35l56_base) {
    let spi = to_spi_device((*cs35l56_base).dev);
    let mut val: c_uint = 0;
    let mut read_ret: c_int = 0;
    /* There must not be any other SPI bus activity while the amp is soft-resetting. */
    let ret = spi_bus_lock((*spi).controller);
    if ret != 0 {
        dev_warn((*cs35l56_base).dev, b"spi_bus_lock failed: %d\n\0".as_ptr() as *const c_char, ret);
        return;
    }
    cs35l56_spi_issue_bus_locked_reset(cs35l56_base, spi);
    spi_bus_unlock((*spi).controller);
    /*
     * Check firmware boot by testing for a response in MBOX_2.
     * HALO_STATE cannot be trusted yet because the reset sequence can leave it with stale state.
     * But MBOX is reset. The regmap must remain in cache-only until the chip has booted, so use a bypassed read.
     */
    let ret2 = read_poll_regmap_read_bypassed_mbox2((*cs35l56_base).regmap, CS35L56_DSP_VIRTUAL1_MBOX_2, &mut val, &mut read_ret);
    if ret2 != 0 {
        dev_err((*cs35l56_base).dev, b"SPI reboot timed out(%d): MBOX2=%#x\n\0".as_ptr() as *const c_char, read_ret, val);
    }
}

static cs35l56_system_reset_seq: [reg_sequence; 2] = [
    REG_SEQ0!(CS35L56_DSP1_HALO_STATE, 0),
    REG_SEQ0!(CS35L56_DSP_VIRTUAL1_MBOX_1, CS35L56_MBOX_CMD_SYSTEM_RESET),
];
static cs35l56_b2_system_reset_seq: [reg_sequence; 2] = [
    REG_SEQ0!(CS35L56_B2_DSP1_HALO_STATE, 0),
    REG_SEQ0!(CS35L56_DSP_VIRTUAL1_MBOX_1, CS35L56_MBOX_CMD_SYSTEM_RESET),
];
static cs35l63_system_reset_seq: [reg_sequence; 2] = [
    REG_SEQ0!(CS35L63_DSP1_HALO_STATE, 0),
    REG_SEQ0!(CS35L56_DSP_VIRTUAL1_MBOX_1, CS35L56_MBOX_CMD_SYSTEM_RESET),
];

#[no_mangle]
pub unsafe extern "C" fn cs35l56_system_reset(cs35l56_base: *mut cs35l56_base, is_soundwire: bool_) {
    /* Must enter cache-only first so there can't be any more register accesses other than the controlled system reset sequence below. */
    regcache_cache_only((*cs35l56_base).regmap, true);
    if cs35l56_is_spi(cs35l56_base) {
        cs35l56_spi_system_reset(cs35l56_base);
        return;
    }
    match (*cs35l56_base).type_ {
        0x54 | 0x56 | 0x57 => match (*cs35l56_base).rev {
            0xb0 => { regmap_multi_reg_write_bypassed((*cs35l56_base).regmap, cs35l56_system_reset_seq.as_ptr(), ARRAY_SIZE!(cs35l56_system_reset_seq)); }
            _ => { regmap_multi_reg_write_bypassed((*cs35l56_base).regmap, cs35l56_b2_system_reset_seq.as_ptr(), ARRAY_SIZE!(cs35l56_b2_system_reset_seq)); }
        },
        0x63 | 0x62 => { regmap_multi_reg_write_bypassed((*cs35l56_base).regmap, cs35l63_system_reset_seq.as_ptr(), ARRAY_SIZE!(cs35l63_system_reset_seq)); }
        _ => {}
    }
    /* On SoundWire the registers won't be accessible until it re-enumerates. */
    if is_soundwire { return; }
    cs35l56_wait_control_port_ready();
    /* Leave in cache-only. This will be revoked when the chip has rebooted. */
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_system_reset, "SND_SOC_CS35L56_SHARED");

unsafe extern "C" fn cs35l56_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l56_base = data as *mut cs35l56_base;
    let mut status1: c_uint = 0;
    let mut status8: c_uint = 0;
    let mut status20: c_uint = 0;
    let mut mask1: c_uint = 0;
    let mut mask8: c_uint = 0;
    let mut mask20: c_uint = 0;
    let mut val: c_uint = 0;
    if !(*cs35l56_base).init_done { return IRQ_NONE; }
    /* guard(mutex)(&cs35l56_base->irq_lock); */
    /* PM_RUNTIME_ACQUIRE_IF_ENABLED / PM_RUNTIME_ACQUIRE_ERR are C scoped-cleanup helpers. */
    regmap_read((*cs35l56_base).regmap, CS35L56_IRQ1_STATUS, &mut val);
    if (val & CS35L56_IRQ1_STS_MASK) == 0 {
        dev_dbg((*cs35l56_base).dev, b"Spurious IRQ: no pending interrupt\n\0".as_ptr() as *const c_char);
        return IRQ_NONE;
    }
    /* Ack interrupts */
    regmap_read((*cs35l56_base).regmap, CS35L56_IRQ1_EINT_1, &mut status1);
    regmap_read((*cs35l56_base).regmap, CS35L56_IRQ1_MASK_1, &mut mask1);
    status1 &= !mask1;
    regmap_write((*cs35l56_base).regmap, CS35L56_IRQ1_EINT_1, status1);
    regmap_read((*cs35l56_base).regmap, CS35L56_IRQ1_EINT_8, &mut status8);
    regmap_read((*cs35l56_base).regmap, CS35L56_IRQ1_MASK_8, &mut mask8);
    status8 &= !mask8;
    regmap_write((*cs35l56_base).regmap, CS35L56_IRQ1_EINT_8, status8);
    regmap_read((*cs35l56_base).regmap, CS35L56_IRQ1_EINT_20, &mut status20);
    regmap_read((*cs35l56_base).regmap, CS35L56_IRQ1_MASK_20, &mut mask20);
    status20 &= !mask20;
    /* We don't want EINT20 but they default to unmasked: force mask */
    regmap_write((*cs35l56_base).regmap, CS35L56_IRQ1_MASK_20, 0xffffffff);
    dev_dbg((*cs35l56_base).dev, b"%s: %#x %#x\n\0".as_ptr() as *const c_char, b"cs35l56_irq\0".as_ptr() as *const c_char, status1, status8);
    /* Check to see if unmasked bits are active */
    if status1 == 0 && status8 == 0 && status20 == 0 { return IRQ_NONE; }
    if (status1 & CS35L56_AMP_SHORT_ERR_EINT1_MASK) != 0 {
        dev_crit((*cs35l56_base).dev, b"Amp short error\n\0".as_ptr() as *const c_char);
    }
    if (status8 & CS35L56_TEMP_ERR_EINT1_MASK) != 0 {
        dev_crit((*cs35l56_base).dev, b"Overtemp error\n\0".as_ptr() as *const c_char);
    }
    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_irq_request(cs35l56_base: *mut cs35l56_base, irq: c_int) -> c_int {
    if irq < 1 { return 0; }
    let ret = devm_request_threaded_irq((*cs35l56_base).dev, irq, ptr::null(), cs35l56_irq, IRQF_ONESHOT | IRQF_SHARED | IRQF_TRIGGER_LOW, b"cs35l56\0".as_ptr() as *const c_char, cs35l56_base as *mut c_void);
    if ret == 0 {
        (*cs35l56_base).irq = irq;
    } else {
        dev_err((*cs35l56_base).dev, b"Failed to get IRQ: %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_irq_request, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_is_fw_reload_needed(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut val: c_uint = 0;
    if (*cs35l56_base).secured { return true as c_int; }
    let mut ret = pm_runtime_resume_and_get((*cs35l56_base).dev);
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"Failed to runtime_get: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = regmap_read((*cs35l56_base).regmap, (*(*cs35l56_base).fw_reg).prot_sts, &mut val);
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"Failed to read PROTECTION_STATUS: %d\n\0".as_ptr() as *const c_char, ret);
    } else {
        ret = ((val & CS35L56_FIRMWARE_MISSING) != 0) as c_int;
    }
    pm_runtime_put_autosuspend((*cs35l56_base).dev);
    ret
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_is_fw_reload_needed, "SND_SOC_CS35L56_SHARED");

static cs35l56_hibernate_seq: [reg_sequence; 1] = [
    /* This must be the last register access */
    REG_SEQ0!(CS35L56_DSP_VIRTUAL1_MBOX_1, CS35L56_MBOX_CMD_ALLOW_AUTO_HIBERNATE),
];

unsafe fn cs35l56_issue_wake_event(cs35l56_base: *mut cs35l56_base) {
    let mut val: c_uint = 0;
    /*
     * Dummy transactions to trigger I2C/SPI auto-wake. Issue two transactions to meet the minimum
     * required time from the rising edge to the last falling edge of wake.
     *
     * It uses bypassed read because we must wake the chip before disabling regmap cache-only.
     */
    regmap_read_bypassed((*cs35l56_base).regmap, CS35L56_IRQ1_STATUS, &mut val);
    usleep_range(CS35L56_WAKE_HOLD_TIME_US, 2 * CS35L56_WAKE_HOLD_TIME_US);
    regmap_read_bypassed((*cs35l56_base).regmap, CS35L56_IRQ1_STATUS, &mut val);
    cs35l56_wait_control_port_ready();
}

unsafe fn cs35l56_wait_for_ps3(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut val: c_uint = 0;
    let ret = regmap_read_poll_timeout_ge((*cs35l56_base).regmap, (*(*cs35l56_base).fw_reg).transducer_actual_ps, &mut val, CS35L56_PS3, CS35L56_PS3_POLL_US, CS35L56_PS3_TIMEOUT_US);
    if ret != 0 { dev_warn((*cs35l56_base).dev, b"PS3 wait failed: %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_runtime_suspend_common(cs35l56_base: *mut cs35l56_base) -> c_int {
    if !(*cs35l56_base).init_done { return 0; }
    /* Firmware must have entered a power-save state */
    cs35l56_wait_for_ps3(cs35l56_base);
    /* Clear BOOT_DONE so it can be used to detect a reboot */
    regmap_write((*cs35l56_base).regmap, CS35L56_IRQ1_EINT_4, CS35L56_OTP_BOOT_DONE_MASK);
    if !(*cs35l56_base).can_hibernate {
        regcache_cache_only((*cs35l56_base).regmap, true);
        dev_dbg((*cs35l56_base).dev, b"Suspended: no hibernate\0".as_ptr() as *const c_char);
        return 0;
    }
    /* Must enter cache-only first so there can't be any more register accesses other than the controlled hibernate sequence below. */
    regcache_cache_only((*cs35l56_base).regmap, true);
    regmap_multi_reg_write_bypassed((*cs35l56_base).regmap, cs35l56_hibernate_seq.as_ptr(), ARRAY_SIZE!(cs35l56_hibernate_seq));
    dev_dbg((*cs35l56_base).dev, b"Suspended: hibernate\0".as_ptr() as *const c_char);
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_runtime_suspend_common, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_runtime_resume_common(cs35l56_base: *mut cs35l56_base, is_soundwire: bool_) -> c_int {
    let mut val: c_uint = 0;
    if !(*cs35l56_base).init_done { return 0; }
    /* Hibernate wake must be done before releasing cache-only */
    if (*cs35l56_base).can_hibernate && !is_soundwire { cs35l56_issue_wake_event(cs35l56_base); }
    let mut ret = cs35l56_wait_for_firmware_boot(cs35l56_base);
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"Hibernate wake failed: %d\n\0".as_ptr() as *const c_char, ret);
        regcache_cache_only((*cs35l56_base).regmap, true);
        if (*cs35l56_base).can_hibernate {
            regmap_multi_reg_write_bypassed((*cs35l56_base).regmap, cs35l56_hibernate_seq.as_ptr(), ARRAY_SIZE!(cs35l56_hibernate_seq));
        }
        return ret;
    }
    regcache_cache_only((*cs35l56_base).regmap, false);
    ret = cs35l56_mbox_send(cs35l56_base, CS35L56_MBOX_CMD_PREVENT_AUTO_HIBERNATE);
    if ret != 0 {
        regcache_cache_only((*cs35l56_base).regmap, true);
        if (*cs35l56_base).can_hibernate {
            regmap_multi_reg_write_bypassed((*cs35l56_base).regmap, cs35l56_hibernate_seq.as_ptr(), ARRAY_SIZE!(cs35l56_hibernate_seq));
        }
        return ret;
    }
    /* BOOT_DONE will be 1 if the amp reset */
    regmap_read((*cs35l56_base).regmap, CS35L56_IRQ1_EINT_4, &mut val);
    if (val & CS35L56_OTP_BOOT_DONE_MASK) != 0 {
        dev_dbg((*cs35l56_base).dev, b"Registers reset in suspend\n\0".as_ptr() as *const c_char);
        regcache_mark_dirty((*cs35l56_base).regmap);
    }
    regcache_sync((*cs35l56_base).regmap);
    dev_dbg((*cs35l56_base).dev, b"Resumed\0".as_ptr() as *const c_char);
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_runtime_resume_common, "SND_SOC_CS35L56_SHARED");

static cs35l56_dsp1_regions: [cs_dsp_region; 5] = [
    cs_dsp_region { type_: WMFW_HALO_PM_PACKED, base: CS35L56_DSP1_PMEM_0 },
    cs_dsp_region { type_: WMFW_HALO_XM_PACKED, base: CS35L56_DSP1_XMEM_PACKED_0 },
    cs_dsp_region { type_: WMFW_HALO_YM_PACKED, base: CS35L56_DSP1_YMEM_PACKED_0 },
    cs_dsp_region { type_: WMFW_ADSP2_XM, base: CS35L56_DSP1_XMEM_UNPACKED24_0 },
    cs_dsp_region { type_: WMFW_ADSP2_YM, base: CS35L56_DSP1_YMEM_UNPACKED24_0 },
];

#[no_mangle]
pub unsafe extern "C" fn cs35l56_init_cs_dsp(cs35l56_base: *mut cs35l56_base, cs_dsp: *mut cs_dsp) {
    (*cs_dsp).num = 1;
    (*cs_dsp).type_ = WMFW_HALO as c_int;
    (*cs_dsp).rev = 0;
    (*cs_dsp).dev = (*cs35l56_base).dev;
    (*cs_dsp).regmap = (*cs35l56_base).regmap;
    (*cs_dsp).base = CS35L56_DSP1_CORE_BASE;
    (*cs_dsp).base_sysinfo = CS35L56_DSP1_SYS_INFO_ID;
    (*cs_dsp).mem = cs35l56_dsp1_regions.as_ptr();
    (*cs_dsp).num_mems = ARRAY_SIZE!(cs35l56_dsp1_regions) as c_uint;
    (*cs_dsp).no_core_startstop = true;
    (*cs35l56_base).dsp = cs_dsp;
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_init_cs_dsp, "SND_SOC_CS35L56_SHARED");

#[repr(C, packed)]
struct cs35l56_pte {
    x: u8,
    wafer_id: u8,
    pte: [u8; 2],
    lot: [u8; 3],
    y: u8,
    unused: [u8; 3],
    dvs: u8,
}

const _: [(); 0] = [(); size_of::<cs35l56_pte>() % size_of::<u32>()];

unsafe fn cs35l56_read_silicon_uid(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut pte: cs35l56_pte = zeroed();
    let ret = regmap_raw_read((*cs35l56_base).regmap, CS35L56_OTP_MEM_53, &mut pte as *mut _ as *mut c_void, size_of::<cs35l56_pte>());
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"Failed to read OTP: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let mut unique_id: u64 = (pte.lot[2] as u32 | ((pte.lot[1] as u32) << 8) | ((pte.lot[0] as u32) << 16)) as u64;
    unique_id <<= 32;
    unique_id |= (pte.x as u32 | ((pte.y as u32) << 8) | ((pte.wafer_id as u32) << 16) | ((pte.dvs as u32) << 24)) as u64;
    (*cs35l56_base).silicon_uid = unique_id;
    0
}

unsafe fn cs35l63_read_silicon_uid(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut tmp: [u32; 2] = [0; 2];
    let ret = regmap_bulk_read((*cs35l56_base).regmap, CS35L56_DIE_STS1, tmp.as_mut_ptr() as *mut c_void, ARRAY_SIZE!(tmp));
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"Cannot obtain CS35L56_DIE_STS: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let mut unique_id = tmp[1] as u64;
    unique_id <<= 32;
    unique_id |= tmp[0] as u64;
    (*cs35l56_base).silicon_uid = unique_id;
    0
}

/* Firmware calibration controls */
#[no_mangle]
pub static cs35l56_calibration_controls: cirrus_amp_cal_controls = cirrus_amp_cal_controls {
    alg_id: 0x9f210,
    mem_region: WMFW_ADSP2_YM,
    ambient: b"CAL_AMBIENT\0".as_ptr() as *const c_char,
    calr: b"CAL_R\0".as_ptr() as *const c_char,
    status: b"CAL_STATUS\0".as_ptr() as *const c_char,
    checksum: b"CAL_CHECKSUM\0".as_ptr() as *const c_char,
};
EXPORT_SYMBOL_NS_GPL!(cs35l56_calibration_controls, "SND_SOC_CS35L56_SHARED");

static cs35l63_calibration_controls: cirrus_amp_cal_controls = cirrus_amp_cal_controls {
    alg_id: 0xbf210,
    mem_region: WMFW_ADSP2_YM,
    ambient: b"CAL_AMBIENT\0".as_ptr() as *const c_char,
    calr: b"CAL_R\0".as_ptr() as *const c_char,
    status: b"CAL_STATUS\0".as_ptr() as *const c_char,
    checksum: b"CAL_CHECKSUM\0".as_ptr() as *const c_char,
};

#[no_mangle]
pub unsafe extern "C" fn cs35l56_get_calibration(cs35l56_base: *mut cs35l56_base) -> c_int {
    /* Driver can't apply calibration to a secured part, so skip */
    if (*cs35l56_base).secured { return 0; }
    let ret = cs_amp_get_efi_calibration_data((*cs35l56_base).dev, (*cs35l56_base).silicon_uid, (*cs35l56_base).cal_index, &mut (*cs35l56_base).cal_data);
    /* Only return an error status if probe should be aborted */
    if ret == -ENOENT || ret == -EOVERFLOW { return 0; }
    if ret < 0 { return ret; }
    (*cs35l56_base).cal_data_valid = true;
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_get_calibration, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_stash_calibration(cs35l56_base: *mut cs35l56_base, data: *const cirrus_amp_cal_data) -> c_int {
    /* Ignore if it is empty */
    if (*data).calTime[0] == 0 && (*data).calTime[1] == 0 { return -ENODATA; }
    if cs_amp_cal_target_u64(data) != (*cs35l56_base).silicon_uid {
        dev_err((*cs35l56_base).dev, b"cal_data not for this silicon ID\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    (*cs35l56_base).cal_data = *data;
    (*cs35l56_base).cal_data_valid = true;
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_stash_calibration, "SND_SOC_CS35L56_SHARED");

unsafe fn cs35l56_perform_calibration(cs35l56_base: *mut cs35l56_base) -> c_int {
    let calibration_controls = (*cs35l56_base).calibration_controls;
    let dsp = (*cs35l56_base).dsp;
    let mut cal_data: cirrus_amp_cal_data = zeroed();
    let mut ngate_ch1_was_enabled = false;
    let mut ngate_ch2_was_enabled = false;
    let cali_norm_en_alg_id: c_int;
    let cali_norm_en_mem: c_int;
    let mut ret: c_int;
    let mut val: __be32;
    if (*cs35l56_base).silicon_uid == 0 {
        dev_err((*cs35l56_base).dev, b"Cannot calibrate: no silicon UID\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    match (*cs35l56_base).type_ {
        0x54 | 0x56 | 0x57 => {
            if (*cs35l56_base).rev < 0xb2 {
                cali_norm_en_alg_id = 0x9f22f;
                cali_norm_en_mem = WMFW_ADSP2_YM as c_int;
            } else {
                cali_norm_en_alg_id = 0x9f210;
                cali_norm_en_mem = WMFW_ADSP2_XM as c_int;
            }
        }
        _ => {
            cali_norm_en_alg_id = 0xbf210;
            cali_norm_en_mem = WMFW_ADSP2_XM as c_int;
        }
    }
    ret = pm_runtime_resume_and_get((*cs35l56_base).dev);
    if ret != 0 { return ret; }
    ret = cs35l56_wait_for_ps3(cs35l56_base);
    if ret != 0 {
        ret = -EBUSY;
        pm_runtime_put((*cs35l56_base).dev);
        return ret;
    }
    regmap_update_bits_check((*cs35l56_base).regmap, CS35L56_MIXER_NGATE_CH1_CFG, CS35L56_AUX_NGATE_CHn_EN, 0, &mut ngate_ch1_was_enabled);
    regmap_update_bits_check((*cs35l56_base).regmap, CS35L56_MIXER_NGATE_CH2_CFG, CS35L56_AUX_NGATE_CHn_EN, 0, &mut ngate_ch2_was_enabled);
    /* scoped_guard(mutex, &dsp->pwr_lock) */
    let ctl = cs_dsp_get_ctl(dsp, (*calibration_controls).status, (*calibration_controls).mem_region, (*calibration_controls).alg_id);
    if ctl.is_null() {
        dev_err((*cs35l56_base).dev, b"Could not get %s control\n\0".as_ptr() as *const c_char, (*calibration_controls).status);
        ret = -EIO;
    } else {
        val = cpu_to_be32(0);
        ret = cs_dsp_coeff_write_ctrl(cs_dsp_get_ctl(dsp, b"CALI_NORM_EN\0".as_ptr() as *const c_char, cali_norm_en_mem as c_uint, cali_norm_en_alg_id as c_uint), 0, &val as *const _ as *const c_void, size_of::<__be32>());
        if ret < 0 {
            dev_err((*cs35l56_base).dev, b"Could not write %s: %d\n\0".as_ptr() as *const c_char, b"CALI_NORM_EN\0".as_ptr() as *const c_char, ret);
            ret = -EIO;
        } else {
            ret = cs35l56_mbox_send(cs35l56_base, CS35L56_MBOX_CMD_AUDIO_CALIBRATION);
            if ret != 0 {
                ret = -EIO;
            } else {
                let mut read_ret = 0;
                if read_poll_cs_dsp_coeff_read_ctrl_eq_one(ctl, &mut val, &mut read_ret) != 0 {
                    dev_err((*cs35l56_base).dev, b"Calibration timed out (CAL_STATUS: %u)\n\0".as_ptr() as *const c_char, be32_to_cpu(val));
                    match be32_to_cpu(val) {
                        CS35L56_CAL_STATUS_OUT_OF_RANGE => ret = -ERANGE,
                        _ => ret = -ETIMEDOUT,
                    }
                } else {
                    ret = 0;
                }
            }
        }
    }
    if ret == 0 {
        (*cs35l56_base).cal_data_valid = false;
        cal_data = zeroed();
        ret = cs_amp_read_cal_coeffs(dsp, calibration_controls, &mut cal_data);
        if ret != 0 {
            ret = -EIO;
        } else {
            dev_info((*cs35l56_base).dev, b"Cal status:%d calR:%d ambient:%d\n\0".as_ptr() as *const c_char, cal_data.calStatus, cal_data.calR, cal_data.calAmbient);
            cal_data.calTarget[0] = (*cs35l56_base).silicon_uid as u32;
            cal_data.calTarget[1] = ((*cs35l56_base).silicon_uid >> 32) as u32;
            (*cs35l56_base).cal_data = cal_data;
            (*cs35l56_base).cal_data_valid = true;
            ret = 0;
        }
    }
    if ngate_ch1_was_enabled {
        regmap_set_bits((*cs35l56_base).regmap, CS35L56_MIXER_NGATE_CH1_CFG, CS35L56_AUX_NGATE_CHn_EN);
    }
    if ngate_ch2_was_enabled {
        regmap_set_bits((*cs35l56_base).regmap, CS35L56_MIXER_NGATE_CH2_CFG, CS35L56_AUX_NGATE_CHn_EN);
    }
    pm_runtime_put((*cs35l56_base).dev);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_calibrate_debugfs_write(cs35l56_base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    static options: [*const c_char; 2] = [b"factory\0".as_ptr() as *const c_char, b"store_uefi\0".as_ptr() as *const c_char];
    let mut buf: [c_char; 11] = [0; 11];
    let mut ret: c_int;
    if !IS_ENABLED!(CONFIG_SND_SOC_CS35L56_CAL_DEBUGFS_COMMON) { return -ENXIO as ssize_t; }
    if *ppos != 0 { return -EINVAL as ssize_t; }
    ret = simple_write_to_buffer(buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 11]>() - 1, ppos, from, count) as c_int;
    if ret < 0 { return ret as ssize_t; }
    match sysfs_match_string(options.as_ptr(), buf.as_ptr()) {
        0 => { ret = cs35l56_perform_calibration(cs35l56_base); if ret < 0 { return ret as ssize_t; } }
        1 => {
            if !(*cs35l56_base).cal_data_valid { return -ENODATA as ssize_t; }
            let mut num_amps = (*cs35l56_base).num_amps;
            if num_amps == 0 { num_amps = -1; }
            ret = cs_amp_set_efi_calibration_data((*cs35l56_base).dev, (*cs35l56_base).cal_index, num_amps, &(*cs35l56_base).cal_data);
            if ret < 0 { return ret as ssize_t; }
        }
        _ => return -EOPNOTSUPP as ssize_t,
    }
    count as ssize_t
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_calibrate_debugfs_write, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_factory_calibrate(cs35l56_base: *mut cs35l56_base) -> c_int {
    if !IS_ENABLED!(CONFIG_SND_SOC_CS35L56_CAL_PERFORM_CTRL) { return -ENXIO; }
    cs35l56_perform_calibration(cs35l56_base)
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_factory_calibrate, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_cal_ambient_debugfs_write(cs35l56_base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut val: c_ulong = 0;
    if !IS_ENABLED!(CONFIG_SND_SOC_CS35L56_CAL_DEBUGFS_COMMON) { return -ENXIO as ssize_t; }
    if *ppos != 0 { return -EINVAL as ssize_t; }
    let mut ret = pm_runtime_resume_and_get((*cs35l56_base).dev);
    if ret != 0 { return ret as ssize_t; }
    ret = kstrtoul_from_user(from, count, 10, &mut val);
    if ret >= 0 {
        ret = cs_amp_write_ambient_temp((*cs35l56_base).dsp, (*cs35l56_base).calibration_controls, val);
        if ret != 0 { ret = -EIO; }
    }
    pm_runtime_put((*cs35l56_base).dev);
    if ret < 0 { return ret as ssize_t; }
    count as ssize_t
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_cal_ambient_debugfs_write, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_cal_data_debugfs_read(cs35l56_base: *mut cs35l56_base, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    if !IS_ENABLED!(CONFIG_SND_SOC_CS35L56_CAL_DEBUGFS_COMMON) { return -ENXIO as ssize_t; }
    if !(*cs35l56_base).cal_data_valid { return 0; }
    simple_read_from_buffer(to, count, ppos, &(*cs35l56_base).cal_data as *const _ as *const c_void, size_of::<cirrus_amp_cal_data>())
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_cal_data_debugfs_read, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_cal_data_debugfs_write(cs35l56_base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut cal_data: cirrus_amp_cal_data = zeroed();
    if !IS_ENABLED!(CONFIG_SND_SOC_CS35L56_CAL_DEBUGFS_COMMON) { return -ENXIO as ssize_t; }
    /* Only allow a full blob to be written */
    if *ppos != 0 || count != size_of::<cirrus_amp_cal_data>() { return -EMSGSIZE as ssize_t; }
    let mut ret = simple_write_to_buffer(&mut cal_data as *mut _ as *mut c_void, size_of::<cirrus_amp_cal_data>(), ppos, from, count) as c_int;
    if ret < 0 { return ret as ssize_t; }
    ret = cs35l56_stash_calibration(cs35l56_base, &cal_data);
    if ret != 0 { return ret as ssize_t; }
    count as ssize_t
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_cal_data_debugfs_write, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_create_cal_debugfs(cs35l56_base: *mut cs35l56_base, fops: *const cs35l56_cal_debugfs_fops) {
    if !IS_ENABLED!(CONFIG_SND_SOC_CS35L56_CAL_DEBUGFS_COMMON) { return; }
    (*cs35l56_base).debugfs = cs_amp_create_debugfs((*cs35l56_base).dev);
    debugfs_create_file(b"calibrate\0".as_ptr() as *const c_char, 0o200, (*cs35l56_base).debugfs, cs35l56_base as *mut c_void, &(*fops).calibrate);
    debugfs_create_file(b"cal_temperature\0".as_ptr() as *const c_char, 0o200, (*cs35l56_base).debugfs, cs35l56_base as *mut c_void, &(*fops).cal_temperature);
    debugfs_create_file(b"cal_data\0".as_ptr() as *const c_char, 0o644, (*cs35l56_base).debugfs, cs35l56_base as *mut c_void, &(*fops).cal_data);
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_create_cal_debugfs, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_remove_cal_debugfs(cs35l56_base: *mut cs35l56_base) {
    debugfs_remove_recursive((*cs35l56_base).debugfs);
    (*cs35l56_base).debugfs = ERR_PTR(-ENOENT);
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_remove_cal_debugfs, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub static cs35l56_cal_set_status_text: [*const c_char; 3] = [
    b"Unknown\0".as_ptr() as *const c_char,
    b"Default\0".as_ptr() as *const c_char,
    b"Set\0".as_ptr() as *const c_char,
];
EXPORT_SYMBOL_NS_GPL!(cs35l56_cal_set_status_text, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_cal_set_status_get(cs35l56_base: *mut cs35l56_base, uvalue: *mut snd_ctl_elem_value) -> c_int {
    let dsp = (*cs35l56_base).dsp;
    let mut cal_set_status_be: __be32 = 0;
    let alg_id = match (*cs35l56_base).type_ { 0x54 | 0x56 | 0x57 => 0x9f210, _ => 0xbf210 };
    let ret = cs_dsp_coeff_read_ctrl(cs_dsp_get_ctl(dsp, b"CAL_SET_STATUS\0".as_ptr() as *const c_char, WMFW_ADSP2_YM, alg_id), 0, &mut cal_set_status_be as *mut _ as *mut c_void, size_of::<__be32>());
    if ret != 0 {
        (*uvalue).value.enumerated.item[0] = CS35L56_CAL_SET_STATUS_UNKNOWN;
        return 0;
    }
    match be32_to_cpu(cal_set_status_be) {
        CS35L56_CAL_SET_STATUS_DEFAULT | CS35L56_CAL_SET_STATUS_SET => {
            (*uvalue).value.enumerated.item[0] = be32_to_cpu(cal_set_status_be);
            0
        }
        _ => {
            (*uvalue).value.enumerated.item[0] = CS35L56_CAL_SET_STATUS_UNKNOWN;
            0
        }
    }
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_cal_set_status_get, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_read_prot_status(cs35l56_base: *mut cs35l56_base, fw_missing: *mut bool_, fw_version: *mut c_uint) -> c_int {
    let mut prot_status: c_uint = 0;
    let mut ret = regmap_read((*cs35l56_base).regmap, (*(*cs35l56_base).fw_reg).prot_sts, &mut prot_status);
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"Get PROTECTION_STATUS failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    *fw_missing = (prot_status & CS35L56_FIRMWARE_MISSING) != 0;
    ret = regmap_read((*cs35l56_base).regmap, (*(*cs35l56_base).fw_reg).fw_ver, fw_version);
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"Get FW VER failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_read_prot_status, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_warn_if_firmware_missing(cs35l56_base: *mut cs35l56_base) {
    let mut firmware_version: c_uint = 0;
    let mut firmware_missing = false;
    let ret = cs35l56_read_prot_status(cs35l56_base, &mut firmware_missing, &mut firmware_version);
    if ret != 0 || !firmware_missing { return; }
    dev_warn((*cs35l56_base).dev, b"FIRMWARE_MISSING\n\0".as_ptr() as *const c_char);
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_warn_if_firmware_missing, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_log_tuning(cs35l56_base: *mut cs35l56_base, cs_dsp: *mut cs_dsp) {
    let mut pid: __be32 = 0;
    let mut sid: __be32 = 0;
    let mut tid: __be32 = 0;
    let alg_id = match (*cs35l56_base).type_ { 0x54 | 0x56 | 0x57 => 0x9f212, _ => 0xbf212 };
    let mut ret = cs_dsp_coeff_read_ctrl(cs_dsp_get_ctl(cs_dsp, b"AS_PRJCT_ID\0".as_ptr() as *const c_char, WMFW_ADSP2_XM, alg_id), 0, &mut pid as *mut _ as *mut c_void, size_of::<__be32>());
    if ret == 0 { ret = cs_dsp_coeff_read_ctrl(cs_dsp_get_ctl(cs_dsp, b"AS_CHNNL_ID\0".as_ptr() as *const c_char, WMFW_ADSP2_XM, alg_id), 0, &mut sid as *mut _ as *mut c_void, size_of::<__be32>()); }
    if ret == 0 { ret = cs_dsp_coeff_read_ctrl(cs_dsp_get_ctl(cs_dsp, b"AS_SNPSHT_ID\0".as_ptr() as *const c_char, WMFW_ADSP2_XM, alg_id), 0, &mut tid as *mut _ as *mut c_void, size_of::<__be32>()); }
    if ret != 0 {
        dev_warn((*cs35l56_base).dev, b"Can't read tuning IDs\0".as_ptr() as *const c_char);
    } else {
        dev_info((*cs35l56_base).dev, b"Tuning PID: %#x, SID: %#x, TID: %#x\n\0".as_ptr() as *const c_char, be32_to_cpu(pid), be32_to_cpu(sid), be32_to_cpu(tid));
    }
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_log_tuning, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_hw_init(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut devid: c_uint = 0;
    let mut revid: c_uint = 0;
    let mut otpid: c_uint = 0;
    let mut secured: c_uint = 0;
    let mut fw_ver: c_uint = 0;
    let mut fw_missing = false;
    /* When the system is not using a reset_gpio ensure the device is awake, otherwise wait for the control port. */
    if (*cs35l56_base).reset_gpio.is_null() { cs35l56_issue_wake_event(cs35l56_base); } else { cs35l56_wait_control_port_ready(); }
    let mut ret = regmap_read_bypassed((*cs35l56_base).regmap, CS35L56_REVID, &mut revid);
    if ret < 0 {
        dev_err((*cs35l56_base).dev, b"Get Revision ID failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    (*cs35l56_base).rev = revid & (CS35L56_AREVID_MASK | CS35L56_MTLREVID_MASK);
    cs35l56_set_fw_reg_table(cs35l56_base);
    ret = cs35l56_wait_for_firmware_boot(cs35l56_base);
    if ret != 0 { return ret; }
    ret = regmap_read_bypassed((*cs35l56_base).regmap, CS35L56_DEVID, &mut devid);
    if ret < 0 {
        dev_err((*cs35l56_base).dev, b"Get Device ID failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    devid &= CS35L56_DEVID_MASK;
    match devid {
        0x35A54 | 0x35A56 | 0x35A57 => (*cs35l56_base).calibration_controls = &cs35l56_calibration_controls,
        0x35A630 | 0x35A620 => { (*cs35l56_base).calibration_controls = &cs35l63_calibration_controls; devid >>= 4; }
        _ => {
            dev_err((*cs35l56_base).dev, b"Unknown device %x\n\0".as_ptr() as *const c_char, devid);
            return -ENODEV;
        }
    }
    (*cs35l56_base).type_ = devid & 0xFF;
    /* Silicon is now identified and booted so exit cache-only */
    regcache_cache_only((*cs35l56_base).regmap, false);
    ret = regmap_read((*cs35l56_base).regmap, CS35L56_DSP_RESTRICT_STS1, &mut secured);
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"Get Secure status failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    /* When any bus is restricted treat the device as secured */
    if (secured & CS35L56_RESTRICTED_MASK) != 0 { (*cs35l56_base).secured = true; }
    ret = regmap_read((*cs35l56_base).regmap, CS35L56_OTPID, &mut otpid);
    if ret < 0 {
        dev_err((*cs35l56_base).dev, b"Get OTP ID failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    ret = cs35l56_read_prot_status(cs35l56_base, &mut fw_missing, &mut fw_ver);
    if ret != 0 { return ret; }
    dev_info((*cs35l56_base).dev, b"Cirrus Logic CS35L%02X%s Rev %02X OTP%d fw:%d.%d.%d (patched=%u)\n\0".as_ptr() as *const c_char, (*cs35l56_base).type_, if (*cs35l56_base).secured { b"s\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char, (*cs35l56_base).rev, otpid, fw_ver >> 16, (fw_ver >> 8) & 0xff, fw_ver & 0xff, (!fw_missing) as c_uint);
    /* Wake source and *_BLOCKED interrupts default to unmasked, so mask them */
    regmap_write((*cs35l56_base).regmap, CS35L56_IRQ1_MASK_20, 0xffffffff);
    regmap_update_bits((*cs35l56_base).regmap, CS35L56_IRQ1_MASK_1, CS35L56_AMP_SHORT_ERR_EINT1_MASK, 0);
    regmap_update_bits((*cs35l56_base).regmap, CS35L56_IRQ1_MASK_8, CS35L56_TEMP_ERR_EINT1_MASK, 0);
    ret = match (*cs35l56_base).type_ { 0x54 | 0x56 | 0x57 => cs35l56_read_silicon_uid(cs35l56_base), _ => cs35l63_read_silicon_uid(cs35l56_base) };
    if ret != 0 { return ret; }
    dev_dbg((*cs35l56_base).dev, b"SiliconID = %#llx\n\0".as_ptr() as *const c_char, (*cs35l56_base).silicon_uid);
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_hw_init, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_get_speaker_id(cs35l56_base: *mut cs35l56_base) -> c_int {
    let mut speaker_id: u32 = 0;
    /* Check for vendor-specific speaker ID method */
    let mut ret = cs_amp_get_vendor_spkid((*cs35l56_base).dev);
    if ret >= 0 {
        dev_dbg((*cs35l56_base).dev, b"Vendor Speaker ID = %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    } else if ret != -ENOENT {
        dev_err((*cs35l56_base).dev, b"Error getting vendor Speaker ID: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    /* Attempt to read the speaker type from a device property */
    ret = device_property_read_u32((*cs35l56_base).dev, b"cirrus,speaker-id\0".as_ptr() as *const c_char, &mut speaker_id);
    if ret == 0 {
        dev_dbg((*cs35l56_base).dev, b"Speaker ID = %d\n\0".as_ptr() as *const c_char, speaker_id);
        return speaker_id as c_int;
    }
    /* Read the speaker type qualifier from the motherboard GPIOs */
    let descs = gpiod_get_array_optional((*cs35l56_base).dev, b"spk-id\0".as_ptr() as *const c_char, GPIOD_IN);
    if descs.is_null() {
        return -ENOENT;
    } else if IS_ERR(descs as *const c_void) {
        ret = PTR_ERR(descs as *const c_void);
        return dev_err_probe((*cs35l56_base).dev, ret, b"Failed to get spk-id-gpios\n\0".as_ptr() as *const c_char);
    }
    speaker_id = 0;
    let mut i = 0;
    while i < (*descs).ndescs as usize {
        ret = gpiod_get_value_cansleep(*(*descs).desc.as_ptr().add(i));
        if ret < 0 {
            dev_err_probe((*cs35l56_base).dev, ret, b"Failed to read spk-id[%d]\n\0".as_ptr() as *const c_char, i as c_int);
            gpiod_put_array(descs);
            return ret;
        }
        speaker_id |= (ret as u32) << i;
        i += 1;
    }
    dev_dbg((*cs35l56_base).dev, b"Speaker ID = %d\n\0".as_ptr() as *const c_char, speaker_id);
    ret = speaker_id as c_int;
    gpiod_put_array(descs);
    ret
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_get_speaker_id, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub unsafe extern "C" fn cs35l56_check_and_save_onchip_spkid_gpios(cs35l56_base: *mut cs35l56_base, gpios: *const u32, num_gpios: c_int, pulls: *const u32, num_pulls: c_int) -> c_int {
    let mut ret = 0;
    if num_gpios as usize > (*cs35l56_base).onchip_spkid_gpios.len() || num_pulls as usize > (*cs35l56_base).onchip_spkid_pulls.len() { return -EOVERFLOW; }
    let max_gpio = match (*cs35l56_base).type_ { 0x54 | 0x56 | 0x57 => CS35L56_MAX_GPIO, _ => CS35L63_MAX_GPIO };
    for i in 0..num_gpios as usize {
        if *gpios.add(i) < 1 || *gpios.add(i) > max_gpio {
            dev_err((*cs35l56_base).dev, b"Invalid spkid GPIO %d\n\0".as_ptr() as *const c_char, *gpios.add(i));
            /* Keep going so we log all bad values */
            ret = -EINVAL;
        }
        /* Change to zero-based */
        (*cs35l56_base).onchip_spkid_gpios[i] = *gpios.add(i) - 1;
    }
    for i in 0..num_pulls as usize {
        match *pulls.add(i) {
            0 => (*cs35l56_base).onchip_spkid_pulls[i] = CS35L56_PAD_PULL_NONE,
            1 => (*cs35l56_base).onchip_spkid_pulls[i] = CS35L56_PAD_PULL_UP,
            2 => (*cs35l56_base).onchip_spkid_pulls[i] = CS35L56_PAD_PULL_DOWN,
            _ => {
                dev_err((*cs35l56_base).dev, b"Invalid spkid pull %d\n\0".as_ptr() as *const c_char, *pulls.add(i));
                /* Keep going so we log all bad values */
                ret = -EINVAL;
            }
        }
    }
    if ret != 0 { return ret; }
    (*cs35l56_base).num_onchip_spkid_gpios = num_gpios;
    (*cs35l56_base).num_onchip_spkid_pulls = num_pulls;
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_check_and_save_onchip_spkid_gpios, "SND_SOC_CS35L56_SHARED");

/* Caller must pm_runtime resume before calling this function */
#[no_mangle]
pub unsafe extern "C" fn cs35l56_configure_onchip_spkid_pads(cs35l56_base: *mut cs35l56_base) -> c_int {
    let regmap = (*cs35l56_base).regmap;
    /* KUNIT_STATIC_STUB_REDIRECT(cs35l56_configure_onchip_spkid_pads, cs35l56_base); */
    if (*cs35l56_base).num_onchip_spkid_gpios == 0 { return 0; }
    let num_gpios = core::cmp::min((*cs35l56_base).num_onchip_spkid_gpios as usize, (*cs35l56_base).onchip_spkid_gpios.len());
    let num_pulls = core::cmp::min((*cs35l56_base).num_onchip_spkid_pulls as usize, (*cs35l56_base).onchip_spkid_pulls.len());
    for i in 0..num_gpios {
        let addr_offset = (*cs35l56_base).onchip_spkid_gpios[i] * size_of::<u32>() as u32;
        /* Set unspecified pulls to NONE */
        let val = if i < num_pulls {
            FIELD_PREP!(CS35L56_PAD_GPIO_PULL_MASK, (*cs35l56_base).onchip_spkid_pulls[i])
        } else {
            FIELD_PREP!(CS35L56_PAD_GPIO_PULL_MASK, CS35L56_PAD_PULL_NONE)
        };
        let ret = regmap_update_bits(regmap, CS35L56_SYNC_GPIO1_CFG + addr_offset, CS35L56_PAD_GPIO_PULL_MASK | CS35L56_PAD_GPIO_IE, val | CS35L56_PAD_GPIO_IE);
        if ret != 0 {
            dev_err((*cs35l56_base).dev, b"GPIO%d set pad fail: %d\n\0".as_ptr() as *const c_char, (*cs35l56_base).onchip_spkid_gpios[i] + 1, ret);
            return ret;
        }
    }
    let ret = regmap_write(regmap, CS35L56_UPDATE_REGS, CS35L56_UPDT_GPIO_PRES);
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"UPDT_GPIO_PRES failed:%d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    usleep_range(CS35L56_PAD_PULL_SETTLE_US, CS35L56_PAD_PULL_SETTLE_US * 2);
    0
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_configure_onchip_spkid_pads, "SND_SOC_CS35L56_SHARED");

/* Caller must pm_runtime resume before calling this function */
#[no_mangle]
pub unsafe extern "C" fn cs35l56_read_onchip_spkid(cs35l56_base: *mut cs35l56_base) -> c_int {
    let regmap = (*cs35l56_base).regmap;
    let mut val: c_uint = 0;
    let mut speaker_id: c_int = 0;
    /* KUNIT_STATIC_STUB_REDIRECT(cs35l56_read_onchip_spkid, cs35l56_base); */
    if (*cs35l56_base).num_onchip_spkid_gpios == 0 { return -ENOENT; }
    let num_gpios = core::cmp::min((*cs35l56_base).num_onchip_spkid_gpios as usize, (*cs35l56_base).onchip_spkid_gpios.len());
    for i in 0..num_gpios {
        let addr_offset = (*cs35l56_base).onchip_spkid_gpios[i] * size_of::<u32>() as u32;
        let ret = regmap_update_bits(regmap, CS35L56_GPIO1_CTRL1 + addr_offset, CS35L56_GPIO_DIR_MASK | CS35L56_GPIO_FN_MASK, CS35L56_GPIO_DIR_MASK | CS35L56_GPIO_FN_GPIO);
        if ret != 0 {
            dev_err((*cs35l56_base).dev, b"GPIO%u set func fail: %d\n\0".as_ptr() as *const c_char, (*cs35l56_base).onchip_spkid_gpios[i] + 1, ret);
            return ret;
        }
    }
    let ret = regmap_read(regmap, CS35L56_GPIO_STATUS1, &mut val);
    if ret != 0 {
        dev_err((*cs35l56_base).dev, b"GPIO status read failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    for i in 0..num_gpios {
        speaker_id <<= 1;
        if (val & BIT!((*cs35l56_base).onchip_spkid_gpios[i])) != 0 { speaker_id |= 1; }
    }
    dev_dbg((*cs35l56_base).dev, b"Onchip GPIO Speaker ID = %d\n\0".as_ptr() as *const c_char, speaker_id);
    speaker_id
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_read_onchip_spkid, "SND_SOC_CS35L56_SHARED");

static cs35l56_bclk_valid_for_pll_freq_table: [u32; 0x3c] = {
    let mut a = [0u32; 0x3c];
    a[0x0C] = 128000; a[0x0F] = 256000; a[0x11] = 384000; a[0x12] = 512000;
    a[0x15] = 768000; a[0x17] = 1024000; a[0x1A] = 1500000; a[0x1B] = 1536000;
    a[0x1C] = 2000000; a[0x1D] = 2048000; a[0x1E] = 2400000; a[0x20] = 3000000;
    a[0x21] = 3072000; a[0x23] = 4000000; a[0x24] = 4096000; a[0x25] = 4800000;
    a[0x27] = 6000000; a[0x28] = 6144000; a[0x29] = 6250000; a[0x2A] = 6400000;
    a[0x2E] = 8000000; a[0x2F] = 8192000; a[0x30] = 9600000; a[0x32] = 12000000;
    a[0x33] = 12288000; a[0x37] = 13500000; a[0x38] = 19200000; a[0x39] = 22579200;
    a[0x3B] = 24576000;
    a
};

#[no_mangle]
pub unsafe extern "C" fn cs35l56_get_bclk_freq_id(freq: c_uint) -> c_int {
    if freq == 0 { return -EINVAL; }
    /* The BCLK frequency must be a valid PLL REFCLK */
    for i in 0..cs35l56_bclk_valid_for_pll_freq_table.len() {
        if cs35l56_bclk_valid_for_pll_freq_table[i] == freq { return i as c_int; }
    }
    -EINVAL
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_get_bclk_freq_id, "SND_SOC_CS35L56_SHARED");

static cs35l56_supplies: [*const c_char; 3] = [
    b"VDD_P\0".as_ptr() as *const c_char,
    b"VDD_IO\0".as_ptr() as *const c_char,
    b"VDD_A\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn cs35l56_fill_supply_names(data: *mut regulator_bulk_data) {
    /* BUILD_BUG_ON(ARRAY_SIZE(cs35l56_supplies) != CS35L56_NUM_BULK_SUPPLIES); */
    for i in 0..cs35l56_supplies.len() {
        (*data.add(i)).supply = cs35l56_supplies[i];
    }
}
EXPORT_SYMBOL_NS_GPL!(cs35l56_fill_supply_names, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub static cs35l56_tx_input_texts: [*const c_char; 21] = [
    b"None\0".as_ptr() as *const c_char, b"ASP1RX1\0".as_ptr() as *const c_char,
    b"ASP1RX2\0".as_ptr() as *const c_char, b"VMON\0".as_ptr() as *const c_char,
    b"IMON\0".as_ptr() as *const c_char, b"ERRVOL\0".as_ptr() as *const c_char,
    b"CLASSH\0".as_ptr() as *const c_char, b"VDDBMON\0".as_ptr() as *const c_char,
    b"VBSTMON\0".as_ptr() as *const c_char, b"DSP1TX1\0".as_ptr() as *const c_char,
    b"DSP1TX2\0".as_ptr() as *const c_char, b"DSP1TX3\0".as_ptr() as *const c_char,
    b"DSP1TX4\0".as_ptr() as *const c_char, b"DSP1TX5\0".as_ptr() as *const c_char,
    b"DSP1TX6\0".as_ptr() as *const c_char, b"DSP1TX7\0".as_ptr() as *const c_char,
    b"DSP1TX8\0".as_ptr() as *const c_char, b"TEMPMON\0".as_ptr() as *const c_char,
    b"INTERPOLATOR\0".as_ptr() as *const c_char, b"SDW1RX1\0".as_ptr() as *const c_char,
    b"SDW1RX2\0".as_ptr() as *const c_char,
];
EXPORT_SYMBOL_NS_GPL!(cs35l56_tx_input_texts, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub static cs35l56_tx_input_values: [c_uint; 21] = [
    CS35L56_INPUT_SRC_NONE,
    CS35L56_INPUT_SRC_ASP1RX1,
    CS35L56_INPUT_SRC_ASP1RX2,
    CS35L56_INPUT_SRC_VMON,
    CS35L56_INPUT_SRC_IMON,
    CS35L56_INPUT_SRC_ERR_VOL,
    CS35L56_INPUT_SRC_CLASSH,
    CS35L56_INPUT_SRC_VDDBMON,
    CS35L56_INPUT_SRC_VBSTMON,
    CS35L56_INPUT_SRC_DSP1TX1,
    CS35L56_INPUT_SRC_DSP1TX2,
    CS35L56_INPUT_SRC_DSP1TX3,
    CS35L56_INPUT_SRC_DSP1TX4,
    CS35L56_INPUT_SRC_DSP1TX5,
    CS35L56_INPUT_SRC_DSP1TX6,
    CS35L56_INPUT_SRC_DSP1TX7,
    CS35L56_INPUT_SRC_DSP1TX8,
    CS35L56_INPUT_SRC_TEMPMON,
    CS35L56_INPUT_SRC_INTERPOLATOR,
    CS35L56_INPUT_SRC_SWIRE_DP1_CHANNEL1,
    CS35L56_INPUT_SRC_SWIRE_DP1_CHANNEL2,
];
EXPORT_SYMBOL_NS_GPL!(cs35l56_tx_input_values, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub static cs35l56_regmap_i2c: regmap_config = regmap_config {
    reg_bits: 32, val_bits: 32, pad_bits: 0, reg_stride: 4, reg_base: 0,
    reg_format_endian: REGMAP_ENDIAN_BIG, val_format_endian: REGMAP_ENDIAN_BIG,
    max_register: CS35L56_DSP1_PMEM_5114,
    reg_defaults: cs35l56_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE!(cs35l56_reg_defaults) as c_uint,
    volatile_reg: Some(cs35l56_volatile_reg), readable_reg: Some(cs35l56_readable_reg), precious_reg: Some(cs35l56_precious_reg),
    cache_type: REGCACHE_MAPLE,
};
EXPORT_SYMBOL_NS_GPL!(cs35l56_regmap_i2c, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub static cs35l56_regmap_spi: regmap_config = regmap_config {
    reg_bits: 32, val_bits: 32, pad_bits: 16, reg_stride: 4, reg_base: 0,
    reg_format_endian: REGMAP_ENDIAN_BIG, val_format_endian: REGMAP_ENDIAN_BIG,
    max_register: CS35L56_DSP1_PMEM_5114,
    reg_defaults: cs35l56_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE!(cs35l56_reg_defaults) as c_uint,
    volatile_reg: Some(cs35l56_volatile_reg), readable_reg: Some(cs35l56_readable_reg), precious_reg: Some(cs35l56_precious_reg),
    cache_type: REGCACHE_MAPLE,
};
EXPORT_SYMBOL_NS_GPL!(cs35l56_regmap_spi, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub static cs35l56_regmap_sdw: regmap_config = regmap_config {
    reg_bits: 32, reg_base: 0x8000, val_bits: 32, pad_bits: 0, reg_stride: 4,
    reg_format_endian: REGMAP_ENDIAN_LITTLE, val_format_endian: REGMAP_ENDIAN_BIG,
    max_register: CS35L56_DSP1_PMEM_5114,
    reg_defaults: cs35l56_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE!(cs35l56_reg_defaults) as c_uint,
    volatile_reg: Some(cs35l56_volatile_reg), readable_reg: Some(cs35l56_readable_reg), precious_reg: Some(cs35l56_precious_reg),
    cache_type: REGCACHE_MAPLE,
};
EXPORT_SYMBOL_NS_GPL!(cs35l56_regmap_sdw, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub static cs35l63_regmap_i2c: regmap_config = regmap_config {
    reg_bits: 32, val_bits: 32, pad_bits: 0, reg_stride: 4, reg_base: 0x8000,
    reg_format_endian: REGMAP_ENDIAN_BIG, val_format_endian: REGMAP_ENDIAN_BIG,
    max_register: CS35L56_DSP1_PMEM_5114,
    reg_defaults: cs35l63_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE!(cs35l63_reg_defaults) as c_uint,
    volatile_reg: Some(cs35l63_volatile_reg), readable_reg: Some(cs35l56_readable_reg), precious_reg: Some(cs35l56_precious_reg),
    cache_type: REGCACHE_MAPLE,
};
EXPORT_SYMBOL_NS_GPL!(cs35l63_regmap_i2c, "SND_SOC_CS35L56_SHARED");

#[no_mangle]
pub static cs35l63_regmap_sdw: regmap_config = regmap_config {
    reg_bits: 32, val_bits: 32, pad_bits: 0, reg_stride: 4, reg_base: 0x8000,
    reg_format_endian: REGMAP_ENDIAN_LITTLE, val_format_endian: REGMAP_ENDIAN_BIG,
    max_register: CS35L56_DSP1_PMEM_5114,
    reg_defaults: cs35l63_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE!(cs35l63_reg_defaults) as c_uint,
    volatile_reg: Some(cs35l63_volatile_reg), readable_reg: Some(cs35l56_readable_reg), precious_reg: Some(cs35l56_precious_reg),
    cache_type: REGCACHE_MAPLE,
};
EXPORT_SYMBOL_NS_GPL!(cs35l63_regmap_sdw, "SND_SOC_CS35L56_SHARED");

/* MODULE_DESCRIPTION("ASoC CS35L56 Shared"); */
/* MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>"); */
/* MODULE_AUTHOR("Simon Trimmer <simont@opensource.cirrus.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_IMPORT_NS("SND_SOC_CS_AMP_LIB"); */
/* MODULE_IMPORT_NS("FW_CS_DSP"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
