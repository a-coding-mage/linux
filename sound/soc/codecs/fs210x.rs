// SPDX-License-Identifier: GPL-2.0
//
// fs210x.c -- Driver for the FS2104/5S Audio Amplifier
//
// Copyright (C) 2016-2025 Shanghai FourSemi Semiconductor Co.,Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type ssize_t = isize;
type size_t = usize;
type bool_ = bool;

const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const EOPNOTSUPP: c_int = 95;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ERANGE: c_int = 34;
const GFP_KERNEL: c_uint = 0;

const FS210X_DEFAULT_FWM_NAME: *const c_char = b"fs210x_fwm.bin\0".as_ptr() as *const c_char;
const FS210X_DEFAULT_DAI_NAME: *const c_char = b"fs210x-aif\0".as_ptr() as *const c_char;
const FS2105S_DEVICE_ID: u16 = 0x20; /* FS2105S */
const FS210X_DEVICE_ID: u16 = 0x45; /* FS2104 */
const FS210X_REG_MAX: c_uint = 0xF8;
const FS210X_INIT_SCENE: c_int = 0;
const FS210X_DEFAULT_SCENE: c_int = 1;
const FS210X_START_DELAY_MS: c_uint = 5;
const FS210X_FAULT_CHECK_INTERVAL_MS: c_uint = 2000;

extern "C" {
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
}

unsafe fn FS2105S_RATES() -> c_uint {
    SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 |
        SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000
}
unsafe fn FS210X_RATES() -> c_uint {
    SNDRV_PCM_RATE_16000 | FS2105S_RATES()
}
unsafe fn FS210X_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE |
        SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE
}

#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct regulator_bulk_data { pub supply: *const c_char }
#[repr(C)] pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }

#[repr(C)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint }
#[repr(C)] pub struct fs_pll_div { pub bclk: c_uint, pub pll1: u16, pub pll2: u16, pub pll3: u16 }
#[repr(C)] pub struct fs_i2s_srate { pub srate: c_uint, pub i2ssr: u16 }
#[repr(C)] pub struct fs_reg_val { pub reg: u8, pub val: u16 }
#[repr(C)] pub struct fs_reg_bits { pub reg: u8, pub mask: u16, pub val: u16 }
#[repr(C)] pub struct fs_cmd_pkg { pub cmd: c_uint, pub regv: fs_reg_val, pub regb: fs_reg_bits }
#[repr(C)] pub struct fs_reg_table { pub size: c_uint, pub buf: *const u8 }
#[repr(C)] pub struct fs_file_table { pub size: c_uint, pub buf: *const c_void }
#[repr(C)] pub struct fs_fwm_table { pub buf: *const c_void }
#[repr(C)] pub struct fs_amp_scene {
    pub name: *const c_char,
    pub reg: *const fs_reg_table,
    pub effect: *const fs_file_table,
}
#[repr(C)] pub struct fs_amp_lib {
    pub dev: *mut device,
    pub devid: u16,
    pub table: [*const fs_fwm_table; 1],
    pub scene_count: c_int,
    pub scene: *const fs_amp_scene,
}

#[repr(C)]
struct fs210x_platform_data {
    fwm_name: *const c_char,
}

#[repr(C)]
struct fs210x_priv {
    i2c: *mut i2c_client,
    dev: *mut device,
    regmap: *mut regmap,
    pdata: fs210x_platform_data,
    supplies: [regulator_bulk_data; FS210X_NUM_SUPPLIES],
    gpio_sdz: *mut gpio_desc,
    start_work: delayed_work,
    fault_check_work: delayed_work,
    amp_lib: fs_amp_lib,
    cur_scene: *const fs_amp_scene,
    clk_bclk: *mut clk,
    /*
     * @lock: Mutex ensuring exclusive access for critical device operations
     *
     * This lock serializes access between the following actions:
     *  - Device initialization procedures(probe)
     *  - Enable/disable device(DAPM event)
     *  - Suspend/resume device(PM)
     *  - Runtime scene switching(control)
     *  - Scheduling/execution of delayed works items(delayed works)
     */
    lock: mutex,
    check_interval_ms: c_uint,
    bclk: c_uint,
    srate: c_uint,
    scene_id: c_int,
    devid: u16,
    is_inited: bool,
    is_suspended: bool,
    is_bclk_on: bool,
    is_playing: bool,
}

static fs210x_supply_names: [*const c_char; 2] = [
    b"pvdd\0".as_ptr() as *const c_char,
    b"dvdd\0".as_ptr() as *const c_char,
];
const FS210X_NUM_SUPPLIES: usize = 2;

static fs2105s_rates: [c_uint; 5] = [32000, 44100, 48000, 88200, 96000];
static fs2105s_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 5,
    list: fs2105s_rates.as_ptr(),
};

static fs210x_rates: [c_uint; 6] = [16000, 32000, 44100, 48000, 88200, 96000];
static fs210x_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: 6,
    list: fs210x_rates.as_ptr(),
};

static fs210x_pll_div: [fs_pll_div; 26] = [
    /*    bclk,   pll1,   pll2,   pll3 */
    fs_pll_div { bclk:   512000, pll1: 0x006C, pll2: 0x0120, pll3: 0x0001 },
    fs_pll_div { bclk:   768000, pll1: 0x016C, pll2: 0x00C0, pll3: 0x0001 },
    fs_pll_div { bclk:  1024000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0001 },
    fs_pll_div { bclk:  1536000, pll1: 0x016C, pll2: 0x0060, pll3: 0x0001 },
    fs_pll_div { bclk:  2048000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0002 },
    fs_pll_div { bclk:  2304000, pll1: 0x016C, pll2: 0x0080, pll3: 0x0002 },
    fs_pll_div { bclk:  3072000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0003 },
    fs_pll_div { bclk:  4096000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0004 },
    fs_pll_div { bclk:  4608000, pll1: 0x016C, pll2: 0x0080, pll3: 0x0004 },
    fs_pll_div { bclk:  6144000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0006 },
    fs_pll_div { bclk:  8192000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0008 },
    fs_pll_div { bclk:  9216000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0009 },
    fs_pll_div { bclk: 12288000, pll1: 0x016C, pll2: 0x0090, pll3: 0x000C },
    fs_pll_div { bclk: 16384000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0010 },
    fs_pll_div { bclk: 18432000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0012 },
    fs_pll_div { bclk: 24576000, pll1: 0x016C, pll2: 0x0090, pll3: 0x0018 },
    fs_pll_div { bclk:  1411200, pll1: 0x016C, pll2: 0x0060, pll3: 0x0001 },
    fs_pll_div { bclk:  2116800, pll1: 0x016C, pll2: 0x0080, pll3: 0x0002 },
    fs_pll_div { bclk:  2822400, pll1: 0x016C, pll2: 0x0090, pll3: 0x0003 },
    fs_pll_div { bclk:  4233600, pll1: 0x016C, pll2: 0x0080, pll3: 0x0004 },
    fs_pll_div { bclk:  5644800, pll1: 0x016C, pll2: 0x0090, pll3: 0x0006 },
    fs_pll_div { bclk:  8467200, pll1: 0x016C, pll2: 0x0090, pll3: 0x0009 },
    fs_pll_div { bclk: 11289600, pll1: 0x016C, pll2: 0x0090, pll3: 0x000C },
    fs_pll_div { bclk: 16934400, pll1: 0x016C, pll2: 0x0090, pll3: 0x0012 },
    fs_pll_div { bclk: 22579200, pll1: 0x016C, pll2: 0x0090, pll3: 0x0018 },
    fs_pll_div { bclk:  2000000, pll1: 0x017C, pll2: 0x0093, pll3: 0x0002 },
];

extern "C" {
    static FS_CMD_UPDATE: c_uint;
    static FS_CMD_DELAY: c_uint;
    static FS_CMD_DELAY_MS_MAX: u16;
    static FS_INDEX_WOOFER: usize;
    static SNDRV_PCM_HW_PARAM_FORMAT: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_NOPM: c_int;
    static REGMAP_ENDIAN_BIG: c_int;
    static REGCACHE_MAPLE: c_int;

    static FS210X_00H_STATUS: c_uint;
    static FS210X_03H_DEVID: u8;
    static FS210X_05H_ANASTAT: u8;
    static FS210X_05H_PVDD_MASK: u16;
    static FS210X_05H_OCDL_MASK: u16;
    static FS210X_05H_UVDL_MASK: u16;
    static FS210X_05H_OVDL_MASK: u16;
    static FS210X_05H_OTPDL_MASK: u16;
    static FS210X_05H_OCRDL_MASK: u16;
    static FS210X_05H_OCLDL_MASK: u16;
    static FS210X_05H_DCRDL_MASK: u16;
    static FS210X_05H_DCLDL_MASK: u16;
    static FS210X_05H_SRDL_MASK: u16;
    static FS210X_05H_OTWDL_MASK: u16;
    static FS210X_05H_AMPS_MASK: u16;
    static FS210X_05H_PLLS_MASK: u16;
    static FS210X_05H_ANAS_MASK: u16;
    static FS210X_0BH_ACCKEY: u8;
    static FS210X_0BH_ACCKEY_ON: u16;
    static FS210X_0BH_ACCKEY_OFF: u16;
    static FS210X_0FH_I2CADDR: c_uint;
    static FS210X_10H_PWRCTRL: u8;
    static FS210X_10H_I2C_RESET: u16;
    static FS210X_11H_SYSCTRL: u8;
    static FS210X_11H_DPS_PLAY: u16;
    static FS210X_11H_DPS_PWDN: u16;
    static FS210X_11H_DPS_HIZ: u16;
    static FS210X_17H_I2SCTRL: u8;
    static FS210X_17H_I2SSR_SHIFT: c_uint;
    static FS210X_17H_I2SSR_MASK: u16;
    static FS210X_30H_DACCTRL: c_uint;
    static FS210X_39H_LVOLCTRL: c_uint;
    static FS210X_3AH_RVOLCTRL: c_uint;
    static FS210X_42H_DACEQWL: u8;
    static FS210X_46H_DACEQA: u8;
    static FS210X_46H_CAM_BURST_L: u16;
    static FS210X_46H_CAM_BURST_R: u16;
    static FS210X_46H_CAM_CLEAR: u16;
    static FS2105S_46H_CAM_BURST_W: u16;
    static FS210X_A1H_PLLCTRL1: u8;
    static FS210X_A2H_PLLCTRL2: u8;
    static FS210X_A3H_PLLCTRL3: u8;
    static FS210X_ABH_INTSTAT: c_uint;
    static FS210X_ACH_INTSTATR: c_uint;

    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn fsleep(usecs: c_uint);
    fn regmap_write(map: *mut regmap, reg: u8, val: u16) -> c_int;
    fn regmap_read(map: *mut regmap, reg: u8, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: u8, mask: u16, val: u16) -> c_int;
    fn regmap_bulk_write(map: *mut regmap, reg: u8, val: *const c_void, count: c_uint) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut fs210x_priv;
    fn snd_pcm_hw_constraint_mask64(runtime: *mut snd_pcm_runtime, var: c_int, mask: u64) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn snd_kcontrol_chip(kctrl: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn fs_amp_load_firmware(lib: *mut fs_amp_lib, name: *const c_char) -> c_int;
    fn snd_soc_add_component_controls(c: *mut snd_soc_component, k: *const snd_kcontrol_new, n: c_int) -> c_int;
    fn regulator_bulk_disable(n: c_int, s: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(n: c_int, s: *mut regulator_bulk_data) -> c_int;
    fn of_property_read_string(n: *mut device_node, p: *const c_char, out: *mut *const c_char) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, name: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn devm_regulator_bulk_get(dev: *mut device, n: c_int, s: *mut regulator_bulk_data) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: size_t, gfp: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn snd_soc_register_component(dev: *mut device, cd: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver, n: c_int) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn dev_get_drvdata(dev: *mut device) -> *mut fs210x_priv;
    fn kstrtouint(s: *const c_char, base: c_uint, res: *mut c_uint) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut fs210x_priv);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut fs210x_priv;
    fn devm_kzalloc(dev: *mut device, size: size_t, gfp: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, cfg: *const regmap_config) -> *mut regmap;
    fn devm_device_add_group(dev: *mut device, group: *const attribute_group) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn IS_ERR(p: *const c_void) -> bool;
    fn PTR_ERR(p: *const c_void) -> c_int;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: extern "C" fn(*mut work_struct));
}

#[inline] unsafe fn HI_U16(v: u16) -> u16 { v >> 8 }

unsafe fn fs210x_bclk_set(fs210x: *mut fs210x_priv, on: bool) -> c_int {
    let mut ret = 0;
    if fs210x.is_null() || (*fs210x).dev.is_null() { return -EINVAL; }
    if (((*fs210x).is_bclk_on as c_int) ^ (on as c_int)) == 0 { return 0; }
    if on {
        clk_set_rate((*fs210x).clk_bclk, (*fs210x).bclk);
        ret = clk_prepare_enable((*fs210x).clk_bclk);
        (*fs210x).is_bclk_on = true;
        fsleep(2000); /* >= 2ms */
    } else {
        clk_disable_unprepare((*fs210x).clk_bclk);
        (*fs210x).is_bclk_on = false;
    }
    ret
}

unsafe fn fs210x_reg_write(fs210x: *mut fs210x_priv, reg: u8, val: u16) -> c_int {
    let ret = regmap_write((*fs210x).regmap, reg, val);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to write %02Xh: %d\n\0".as_ptr() as *const c_char, reg as c_int, ret);
        return ret;
    }
    0
}

unsafe fn fs210x_reg_read(fs210x: *mut fs210x_priv, reg: u8, pval: *mut u16) -> c_int {
    let mut val: c_uint = 0;
    let ret = regmap_read((*fs210x).regmap, reg, &mut val);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to read %02Xh: %d\n\0".as_ptr() as *const c_char, reg as c_int, ret);
        return ret;
    }
    *pval = val as u16;
    0
}

unsafe fn fs210x_reg_update_bits(fs210x: *mut fs210x_priv, reg: u8, mask: u16, val: u16) -> c_int {
    let ret = regmap_update_bits((*fs210x).regmap, reg, mask, val);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to update %02Xh: %d\n\0".as_ptr() as *const c_char, reg as c_int, ret);
        return ret;
    }
    0
}

unsafe fn fs210x_reg_bulk_write(fs210x: *mut fs210x_priv, reg: u8, val: *const c_void, size: u32) -> c_int {
    let ret = regmap_bulk_write((*fs210x).regmap, reg, val, size / 2);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to bulk write %02Xh: %d\n\0".as_ptr() as *const c_char, reg as c_int, ret);
        return ret;
    }
    0
}

#[inline] unsafe fn fs210x_write_reg_val(fs210x: *mut fs210x_priv, regv: *const fs_reg_val) -> c_int {
    fs210x_reg_write(fs210x, (*regv).reg, (*regv).val)
}

#[inline] unsafe fn fs210x_write_reg_bits(fs210x: *mut fs210x_priv, regu: *const fs_reg_bits) -> c_int {
    fs210x_reg_update_bits(fs210x, (*regu).reg, (*regu).mask, (*regu).val)
}

#[inline] unsafe fn fs210x_set_cmd_pkg(fs210x: *mut fs210x_priv, pkg: *const fs_cmd_pkg, offset: *mut c_uint) -> c_int {
    let delay_us: c_int;
    if (*pkg).cmd <= FS210X_REG_MAX {
        *offset = size_of::<fs_reg_val>() as c_uint;
        return fs210x_write_reg_val(fs210x, &(*pkg).regv);
    } else if (*pkg).cmd == FS_CMD_UPDATE {
        *offset = size_of::<fs_reg_bits>() as c_uint;
        return fs210x_write_reg_bits(fs210x, &(*pkg).regb);
    } else if (*pkg).cmd == FS_CMD_DELAY {
        if (*pkg).regv.val > FS_CMD_DELAY_MS_MAX { return -EOPNOTSUPP; }
        delay_us = ((*pkg).regv.val as c_int) * 1000; /* ms -> us */
        fsleep(delay_us as c_uint);
        *offset = size_of::<fs_reg_val>() as c_uint;
        return 0;
    }
    dev_err((*fs210x).dev, b"Invalid pkg cmd: %d\n\0".as_ptr() as *const c_char, (*pkg).cmd as c_int);
    -EOPNOTSUPP
}

unsafe fn fs210x_reg_write_table(fs210x: *mut fs210x_priv, reg: *const fs_reg_table) -> c_int {
    let mut pkg: *const fs_cmd_pkg;
    let mut index: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut ret: c_int;
    if fs210x.is_null() || (*fs210x).dev.is_null() { return -EINVAL; }
    if reg.is_null() || (*reg).size == 0 { return -EFAULT; }
    while index < (*reg).size {
        pkg = (*reg).buf.add(index as usize) as *const fs_cmd_pkg;
        ret = fs210x_set_cmd_pkg(fs210x, pkg, &mut offset);
        if ret != 0 {
            dev_err((*fs210x).dev, b"Failed to set cmd pkg: %02X-%d\n\0".as_ptr() as *const c_char, (*pkg).cmd as c_int, ret);
            return ret;
        }
        index = index.wrapping_add(offset);
    }
    if index != (*reg).size {
        dev_err((*fs210x).dev, b"Invalid reg table size: %d-%d\n\0".as_ptr() as *const c_char, index as c_int, (*reg).size as c_int);
        return -EFAULT;
    }
    0
}

unsafe fn fs210x_dev_play(fs210x: *mut fs210x_priv) -> c_int {
    if !(*fs210x).is_inited { return -EFAULT; }
    if (*fs210x).is_playing { return 0; }
    let ret = fs210x_reg_write(fs210x, FS210X_11H_SYSCTRL, FS210X_11H_DPS_PLAY);
    if ret == 0 { (*fs210x).is_playing = true; }
    fsleep(10000); /* >= 10ms */
    ret
}

unsafe fn fs210x_dev_stop(fs210x: *mut fs210x_priv) -> c_int {
    if !(*fs210x).is_inited { return -EFAULT; }
    if !(*fs210x).is_playing { return 0; }
    let ret = fs210x_reg_write(fs210x, FS210X_11H_SYSCTRL, FS210X_11H_DPS_PWDN);
    (*fs210x).is_playing = false;
    fsleep(30000); /* >= 30ms */
    ret
}

unsafe fn fs210x_set_reg_table(fs210x: *mut fs210x_priv, scene: *const fs_amp_scene) -> c_int {
    if fs210x.is_null() || (*fs210x).dev.is_null() || scene.is_null() { return -EINVAL; }
    let cur_scene = (*fs210x).cur_scene;
    if (*scene).reg.is_null() || cur_scene == scene {
        dev_dbg((*fs210x).dev, b"Skip writing reg table\n\0".as_ptr() as *const c_char);
        return 0;
    }
    let reg = (*scene).reg;
    dev_dbg((*fs210x).dev, b"reg table size: %d\n\0".as_ptr() as *const c_char, (*reg).size as c_int);
    fs210x_reg_write_table(fs210x, reg)
}

unsafe fn fs210x_set_woofer_table(fs210x: *mut fs210x_priv) -> c_int {
    if fs210x.is_null() || (*fs210x).dev.is_null() { return -EINVAL; }
    /* NOTE: fs2105s has woofer ram only */
    if (*fs210x).devid != FS2105S_DEVICE_ID { return 0; }
    let table = (*fs210x).amp_lib.table[FS_INDEX_WOOFER];
    if table.is_null() {
        dev_dbg((*fs210x).dev, b"Skip writing woofer table\n\0".as_ptr() as *const c_char);
        return 0;
    }
    let woofer = (*table).buf as *const fs_file_table;
    dev_dbg((*fs210x).dev, b"woofer table size: %d\n\0".as_ptr() as *const c_char, (*woofer).size as c_int);
    /* Unit of woofer data is u32(4 bytes) */
    if (*woofer).size == 0 || ((*woofer).size & 0x3) != 0 {
        dev_err((*fs210x).dev, b"Invalid woofer size: %d\n\0".as_ptr() as *const c_char, (*woofer).size as c_int);
        return -EINVAL;
    }
    let mut ret = fs210x_reg_write(fs210x, FS210X_46H_DACEQA, FS2105S_46H_CAM_BURST_W);
    ret |= fs210x_reg_bulk_write(fs210x, FS210X_42H_DACEQWL, (*woofer).buf, (*woofer).size);
    ret
}

unsafe fn fs210x_set_effect_table(fs210x: *mut fs210x_priv, scene: *const fs_amp_scene) -> c_int {
    if fs210x.is_null() || (*fs210x).dev.is_null() || scene.is_null() { return -EINVAL; }
    let cur_scene = (*fs210x).cur_scene;
    if (*scene).effect.is_null() || cur_scene == scene {
        dev_dbg((*fs210x).dev, b"Skip writing effect table\n\0".as_ptr() as *const c_char);
        return 0;
    }
    let effect = (*scene).effect;
    dev_dbg((*fs210x).dev, b"effect table size: %d\n\0".as_ptr() as *const c_char, (*effect).size as c_int);
    /* Unit of effect data is u32(4 bytes), 2 channels */
    if (*effect).size == 0 || ((*effect).size & 0x7) != 0 {
        dev_err((*fs210x).dev, b"Invalid effect size: %d\n\0".as_ptr() as *const c_char, (*effect).size as c_int);
        return -EINVAL;
    }
    let half_size = (*effect).size / 2;
    /* Left channel */
    let mut ret = fs210x_reg_write(fs210x, FS210X_46H_DACEQA, FS210X_46H_CAM_BURST_L);
    ret |= fs210x_reg_bulk_write(fs210x, FS210X_42H_DACEQWL, (*effect).buf, half_size);
    if ret != 0 { return ret; }
    /* Right channel */
    ret = fs210x_reg_write(fs210x, FS210X_46H_DACEQA, FS210X_46H_CAM_BURST_R);
    ret |= fs210x_reg_bulk_write(fs210x, FS210X_42H_DACEQWL, ((*effect).buf as *const u8).add(half_size as usize) as *const c_void, half_size);
    ret
}

unsafe fn fs210x_access_dsp_ram(fs210x: *mut fs210x_priv, enable: bool) -> c_int {
    if fs210x.is_null() || (*fs210x).dev.is_null() { return -EINVAL; }
    let mut ret;
    if enable {
        ret = fs210x_reg_write(fs210x, FS210X_11H_SYSCTRL, FS210X_11H_DPS_HIZ);
        ret |= fs210x_reg_write(fs210x, FS210X_0BH_ACCKEY, FS210X_0BH_ACCKEY_ON);
    } else {
        ret = fs210x_reg_write(fs210x, FS210X_0BH_ACCKEY, FS210X_0BH_ACCKEY_OFF);
        ret |= fs210x_reg_write(fs210x, FS210X_11H_SYSCTRL, FS210X_11H_DPS_PWDN);
    }
    fsleep(10000); /* >= 10ms */
    ret
}

unsafe fn fs210x_write_dsp_effect(fs210x: *mut fs210x_priv, scene: *const fs_amp_scene, scene_id: c_int) -> c_int {
    if fs210x.is_null() || scene.is_null() { return -EINVAL; }
    let mut ret = fs210x_access_dsp_ram(fs210x, true);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to access dsp: %d\n\0".as_ptr() as *const c_char, ret);
        fs210x_reg_write(fs210x, FS210X_46H_DACEQA, FS210X_46H_CAM_CLEAR);
        fs210x_access_dsp_ram(fs210x, false);
        return ret;
    }
    ret = fs210x_set_effect_table(fs210x, scene);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to set effect: %d\n\0".as_ptr() as *const c_char, ret);
        fs210x_reg_write(fs210x, FS210X_46H_DACEQA, FS210X_46H_CAM_CLEAR);
        fs210x_access_dsp_ram(fs210x, false);
        return ret;
    }
    if scene_id == FS210X_INIT_SCENE { ret = fs210x_set_woofer_table(fs210x); }
    fs210x_reg_write(fs210x, FS210X_46H_DACEQA, FS210X_46H_CAM_CLEAR);
    fs210x_access_dsp_ram(fs210x, false);
    ret
}

unsafe fn fs210x_check_scene(fs210x: *mut fs210x_priv, scene_id: c_int, skip_set: *mut bool) -> c_int {
    if fs210x.is_null() || skip_set.is_null() { return -EINVAL; }
    let amp_lib = &mut (*fs210x).amp_lib as *mut fs_amp_lib;
    if (*amp_lib).scene_count == 0 || (*amp_lib).scene.is_null() {
        dev_err((*fs210x).dev, b"There's no scene data\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if scene_id < 0 || scene_id >= (*amp_lib).scene_count {
        dev_err((*fs210x).dev, b"Invalid scene_id: %d\n\0".as_ptr() as *const c_char, scene_id);
        return -EINVAL;
    }
    if (*fs210x).scene_id == scene_id {
        dev_dbg((*fs210x).dev, b"Skip to set same scene\n\0".as_ptr() as *const c_char);
        return 0;
    }
    *skip_set = false;
    0
}

unsafe fn fs210x_set_scene(fs210x: *mut fs210x_priv, scene_id: c_int) -> c_int {
    let mut skip_set = true;
    if fs210x.is_null() || (*fs210x).dev.is_null() { return -EINVAL; }
    let ret = fs210x_check_scene(fs210x, scene_id, &mut skip_set);
    if ret != 0 || skip_set { return ret; }
    let scene = (*fs210x).amp_lib.scene.add(scene_id as usize);
    dev_info((*fs210x).dev, b"Switch scene.%d: %s\n\0".as_ptr() as *const c_char, scene_id, (*scene).name);
    let is_playing = (*fs210x).is_playing;
    if is_playing { fs210x_dev_stop(fs210x); }
    let mut ret2 = fs210x_set_reg_table(fs210x, scene);
    if ret2 != 0 {
        dev_err((*fs210x).dev, b"Failed to set reg: %d\n\0".as_ptr() as *const c_char, ret2);
        return ret2;
    }
    ret2 = fs210x_write_dsp_effect(fs210x, scene, scene_id);
    if ret2 != 0 {
        dev_err((*fs210x).dev, b"Failed to write ram: %d\n\0".as_ptr() as *const c_char, ret2);
        return ret2;
    }
    (*fs210x).cur_scene = scene;
    (*fs210x).scene_id = scene_id;
    if is_playing { fs210x_dev_play(fs210x); }
    0
}

unsafe fn fs210x_init_chip(fs210x: *mut fs210x_priv) -> c_int {
    regcache_cache_bypass((*fs210x).regmap, true);
    let mut ret: c_int;
    if (*fs210x).gpio_sdz.is_null() {
        /* Gpio is not found, i2c reset */
        ret = fs210x_reg_write(fs210x, FS210X_10H_PWRCTRL, FS210X_10H_I2C_RESET);
        if ret != 0 { goto_power_down(fs210x, ret) } else { ret = 0; }
    } else {
        /* gpio reset, deactivate */
        gpiod_set_value_cansleep((*fs210x).gpio_sdz, 0);
        ret = 0;
    }
    fsleep(10000); /* >= 10ms */
    /* Backup scene id */
    let mut scene_id = (*fs210x).scene_id;
    (*fs210x).scene_id = -1;
    /* Init registers/RAM by init scene */
    ret = fs210x_set_scene(fs210x, FS210X_INIT_SCENE);
    if ret != 0 { return goto_power_down(fs210x, ret); }
    /*
     * If the firmware has effect scene(s),
     * we load effect scene by default scene or scene_id
     */
    if (*fs210x).amp_lib.scene_count > 1 {
        if scene_id < FS210X_DEFAULT_SCENE { scene_id = FS210X_DEFAULT_SCENE; }
        ret = fs210x_set_scene(fs210x, scene_id);
        if ret != 0 { return goto_power_down(fs210x, ret); }
    }
    goto_power_down(fs210x, ret)
}

unsafe fn goto_power_down(fs210x: *mut fs210x_priv, mut ret: c_int) -> c_int {
    /* Power down the device */
    ret |= fs210x_reg_write(fs210x, FS210X_11H_SYSCTRL, FS210X_11H_DPS_PWDN);
    fsleep(10000); /* >= 10ms */
    regcache_cache_bypass((*fs210x).regmap, false);
    if ret == 0 {
        regcache_cache_only((*fs210x).regmap, false);
        regcache_mark_dirty((*fs210x).regmap);
        regcache_sync((*fs210x).regmap);
        (*fs210x).is_inited = true;
    }
    ret
}

unsafe fn fs210x_set_i2s_params(fs210x: *mut fs210x_priv) -> c_int {
    let params = [
        fs_i2s_srate { srate: 16000, i2ssr: 0x3 },
        fs_i2s_srate { srate: 32000, i2ssr: 0x7 },
        fs_i2s_srate { srate: 44100, i2ssr: 0x8 },
        fs_i2s_srate { srate: 48000, i2ssr: 0x9 },
        fs_i2s_srate { srate: 88200, i2ssr: 0xA },
        fs_i2s_srate { srate: 96000, i2ssr: 0xB },
    ];
    for p in params.iter() {
        if p.srate != (*fs210x).srate { continue; }
        let val = p.i2ssr << FS210X_17H_I2SSR_SHIFT;
        return fs210x_reg_update_bits(fs210x, FS210X_17H_I2SCTRL, FS210X_17H_I2SSR_MASK, val);
    }
    dev_err((*fs210x).dev, b"Invalid sample rate: %d\n\0".as_ptr() as *const c_char, (*fs210x).srate as c_int);
    -EINVAL
}

unsafe fn fs210x_get_pll_div(fs210x: *mut fs210x_priv, pll_div: *mut *const fs_pll_div) -> c_int {
    if fs210x.is_null() || pll_div.is_null() { return -EINVAL; }
    for i in 0..fs210x_pll_div.len() {
        if fs210x_pll_div[i].bclk != (*fs210x).bclk { continue; }
        *pll_div = fs210x_pll_div.as_ptr().add(i);
        return 0;
    }
    dev_err((*fs210x).dev, b"No PLL table for bclk: %d\n\0".as_ptr() as *const c_char, (*fs210x).bclk as c_int);
    -EFAULT
}

unsafe fn fs210x_set_hw_params(fs210x: *mut fs210x_priv) -> c_int {
    let mut pll_div: *const fs_pll_div = ptr::null();
    let mut ret = fs210x_set_i2s_params(fs210x);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to set i2s params: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    /* Set pll params */
    ret = fs210x_get_pll_div(fs210x, &mut pll_div);
    if ret != 0 { return ret; }
    ret = fs210x_reg_write(fs210x, FS210X_A1H_PLLCTRL1, (*pll_div).pll1);
    ret |= fs210x_reg_write(fs210x, FS210X_A2H_PLLCTRL2, (*pll_div).pll2);
    ret |= fs210x_reg_write(fs210x, FS210X_A3H_PLLCTRL3, (*pll_div).pll3);
    ret
}

unsafe extern "C" fn fs210x_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let list: *const snd_pcm_hw_constraint_list;
    let fs210x = snd_soc_component_get_drvdata((*dai).component);
    if fs210x.is_null() {
        pr_err(b"dai_startup: fs210x is null\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if (*substream).runtime.is_null() { return 0; }
    let mut ret = snd_pcm_hw_constraint_mask64((*substream).runtime, SNDRV_PCM_HW_PARAM_FORMAT, FS210X_FORMATS());
    if ret < 0 {
        dev_err((*fs210x).dev, b"Failed to set hw param format: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if (*fs210x).devid == FS2105S_DEVICE_ID {
        list = &fs2105s_constraints;
    } else {
        list = &fs210x_constraints;
    }
    ret = snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, list);
    if ret < 0 {
        dev_err((*fs210x).dev, b"Failed to set hw param rate: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}

unsafe extern "C" fn fs210x_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let fs210x = snd_soc_component_get_drvdata((*dai).component);
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            /* Only supports consumer mode */
        }
        _ => {
            dev_err((*fs210x).dev, b"Only supports consumer mode\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn fs210x_dai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK { return 0; }
    let fs210x = snd_soc_component_get_drvdata((*dai).component);
    (*fs210x).srate = params_rate(params);
    (*fs210x).bclk = snd_soc_params_to_bclk(params);
    let chn_num = params_channels(params);
    if chn_num == 1 { /* mono */
        (*fs210x).bclk = (*fs210x).bclk.wrapping_mul(2); /* I2S bus has 2 channels */
    }
    /* The FS2105S can't support 16kHz sample rate. */
    if (*fs210x).devid == FS2105S_DEVICE_ID && (*fs210x).srate == 16000 { return -EOPNOTSUPP; }
    mutex_lock(&mut (*fs210x).lock);
    let ret = fs210x_set_hw_params(fs210x);
    mutex_unlock(&mut (*fs210x).lock);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to set hw params: %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret
}

unsafe extern "C" fn fs210x_dai_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    if stream != SNDRV_PCM_STREAM_PLAYBACK { return 0; }
    let fs210x = snd_soc_component_get_drvdata((*dai).component);
    mutex_lock(&mut (*fs210x).lock);
    if !(*fs210x).is_inited || (*fs210x).is_suspended {
        mutex_unlock(&mut (*fs210x).lock);
        return 0;
    }
    mutex_unlock(&mut (*fs210x).lock);
    if mute != 0 {
        cancel_delayed_work_sync(&mut (*fs210x).fault_check_work);
        cancel_delayed_work_sync(&mut (*fs210x).start_work);
    } else {
        let delay = msecs_to_jiffies((*fs210x).check_interval_ms);
        schedule_delayed_work(&mut (*fs210x).fault_check_work, delay);
    }
    0
}

unsafe extern "C" fn fs210x_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let fs210x = snd_soc_component_get_drvdata((*dai).component);
    mutex_lock(&mut (*fs210x).lock);
    if !(*fs210x).is_inited || (*fs210x).is_suspended || (*fs210x).is_playing {
        mutex_unlock(&mut (*fs210x).lock);
        return 0;
    }
    mutex_unlock(&mut (*fs210x).lock);
    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START || x == SNDRV_PCM_TRIGGER_RESUME || x == SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            /*
             * According to the power up/down sequence of FS210x,
             * it requests the I2S clock has been present
             * and stable(>= 2ms) before playing.
             */
            schedule_delayed_work(&mut (*fs210x).start_work, msecs_to_jiffies(FS210X_START_DELAY_MS));
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn fs210x_start_work(work: *mut work_struct) {
    let fs210x = container_of_start_work(work);
    mutex_lock(&mut (*fs210x).lock);
    let ret = fs210x_dev_play(fs210x);
    mutex_unlock(&mut (*fs210x).lock);
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to start playing: %d\n\0".as_ptr() as *const c_char, ret);
    }
}

unsafe extern "C" fn fs210x_fault_check_work(work: *mut work_struct) {
    let fs210x = container_of_fault_check_work(work);
    let mut status: u16 = 0;
    mutex_lock(&mut (*fs210x).lock);
    if !(*fs210x).is_inited || (*fs210x).is_suspended || !(*fs210x).is_playing {
        mutex_unlock(&mut (*fs210x).lock);
        return;
    }
    let ret = fs210x_reg_read(fs210x, FS210X_05H_ANASTAT, &mut status);
    mutex_unlock(&mut (*fs210x).lock);
    if ret != 0 { return; }
    if (status & FS210X_05H_PVDD_MASK) == 0 { dev_err((*fs210x).dev, b"PVDD fault\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_OCDL_MASK) != 0 { dev_err((*fs210x).dev, b"OC detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_UVDL_MASK) != 0 { dev_err((*fs210x).dev, b"UV detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_OVDL_MASK) != 0 { dev_err((*fs210x).dev, b"OV detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_OTPDL_MASK) != 0 { dev_err((*fs210x).dev, b"OT detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_OCRDL_MASK) != 0 { dev_err((*fs210x).dev, b"OCR detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_OCLDL_MASK) != 0 { dev_err((*fs210x).dev, b"OCL detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_DCRDL_MASK) != 0 { dev_err((*fs210x).dev, b"DCR detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_DCLDL_MASK) != 0 { dev_err((*fs210x).dev, b"DCL detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_SRDL_MASK) != 0 { dev_err((*fs210x).dev, b"SR detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_OTWDL_MASK) != 0 { dev_err((*fs210x).dev, b"OTW detected\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_AMPS_MASK) == 0 { dev_dbg((*fs210x).dev, b"Amplifier unready\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_PLLS_MASK) == 0 { dev_err((*fs210x).dev, b"PLL unlock\n\0".as_ptr() as *const c_char); }
    if (status & FS210X_05H_ANAS_MASK) == 0 { dev_err((*fs210x).dev, b"Analog power fault\n\0".as_ptr() as *const c_char); }
    schedule_delayed_work(&mut (*fs210x).fault_check_work, msecs_to_jiffies((*fs210x).check_interval_ms));
}

unsafe fn container_of_start_work(work: *mut work_struct) -> *mut fs210x_priv {
    (work as *mut u8).sub(offset_of_start_work_work()) as *mut fs210x_priv
}

unsafe fn container_of_fault_check_work(work: *mut work_struct) -> *mut fs210x_priv {
    (work as *mut u8).sub(offset_of_fault_check_work_work()) as *mut fs210x_priv
}

fn offset_of_start_work_work() -> usize {
    unsafe { &(*(ptr::null::<fs210x_priv>())).start_work.work as *const _ as usize }
}

fn offset_of_fault_check_work_work() -> usize {
    unsafe { &(*(ptr::null::<fs210x_priv>())).fault_check_work.work as *const _ as usize }
}

#[repr(C)] pub struct snd_ctl_elem_info_value_enumerated { pub items: c_uint, pub item: c_uint, pub name: [c_char; 64] }
#[repr(C)] pub union snd_ctl_elem_info_value { pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_info_value_enumerated> }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
type c_long = isize;
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer> }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
extern "C" { static SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_uint; }

unsafe fn fs210x_get_drvdata_from_kctrl(kctrl: *mut snd_kcontrol, fs210x: *mut *mut fs210x_priv) -> c_int {
    if kctrl.is_null() {
        pr_err(b"fs210x: kcontrol is null\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let cmpnt = snd_kcontrol_chip(kctrl);
    if cmpnt.is_null() {
        pr_err(b"fs210x: component is null\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    *fs210x = snd_soc_component_get_drvdata(cmpnt);
    0
}

unsafe extern "C" fn fs210x_effect_scene_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mut fs210x: *mut fs210x_priv = ptr::null_mut();
    let ret = fs210x_get_drvdata_from_kctrl(kcontrol, &mut fs210x);
    if ret != 0 || (*fs210x).dev.is_null() {
        pr_err(b"scene_effect_info: fs210x is null\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
    (*uinfo).count = 1;
    let count = (*fs210x).amp_lib.scene_count - 1; /* Skip init scene */
    if count < 1 {
        (*uinfo).value.enumerated.items = 0;
        return 0;
    }
    (*uinfo).value.enumerated.items = count as c_uint;
    if (*uinfo).value.enumerated.item >= count as c_uint {
        (*uinfo).value.enumerated.item = (count - 1) as c_uint;
    }
    let idx = (*uinfo).value.enumerated.item as c_int;
    let scene = (*fs210x).amp_lib.scene.add((idx + 1) as usize);
    let mut name = b"N/A\0".as_ptr() as *const c_char;
    if !(*scene).name.is_null() { name = (*scene).name; }
    strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), name);
    0
}

unsafe extern "C" fn fs210x_effect_scene_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mut fs210x: *mut fs210x_priv = ptr::null_mut();
    let ret = fs210x_get_drvdata_from_kctrl(kcontrol, &mut fs210x);
    if ret != 0 || (*fs210x).dev.is_null() {
        pr_err(b"scene_effect_get: fs210x is null\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    /* The id of effect scene is from 1 to N. */
    if (*fs210x).scene_id < 1 { return -EINVAL; }
    mutex_lock(&mut (*fs210x).lock);
    /*
     * FS210x has scene(s) as below:
     * init scene: id = 0
     * effect scene(s): id = 1~N (optional)
     * effect_index = scene_id - 1
     */
    let index = (*fs210x).scene_id - 1;
    (*ucontrol).value.integer.value[0] = index as c_long;
    mutex_unlock(&mut (*fs210x).lock);
    0
}

unsafe extern "C" fn fs210x_effect_scene_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mut fs210x: *mut fs210x_priv = ptr::null_mut();
    let ret = fs210x_get_drvdata_from_kctrl(kcontrol, &mut fs210x);
    if ret != 0 || (*fs210x).dev.is_null() {
        pr_err(b"scene_effect_put: fs210x is null\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    mutex_lock(&mut (*fs210x).lock);
    /*
     * FS210x has scene(s) as below:
     * init scene: id = 0 (It's set in fs210x_init_chip() only)
     * effect scene(s): id = 1~N (optional)
     * scene_id = effect_index + 1.
     */
    let scene_id = (*ucontrol).value.integer.value[0] as c_int + 1;
    let scene_count = (*fs210x).amp_lib.scene_count - 1; /* Skip init scene */
    if scene_id < 1 || scene_id > scene_count {
        mutex_unlock(&mut (*fs210x).lock);
        return -ERANGE;
    }
    let is_changed = scene_id != (*fs210x).scene_id;
    if (*fs210x).is_suspended {
        (*fs210x).scene_id = scene_id;
        mutex_unlock(&mut (*fs210x).lock);
        return is_changed as c_int;
    }
    let ret2 = fs210x_set_scene(fs210x, scene_id);
    mutex_unlock(&mut (*fs210x).lock);
    if ret2 != 0 {
        dev_err((*fs210x).dev, b"Failed to set scene: %d\n\0".as_ptr() as *const c_char, ret2);
    }
    if ret2 == 0 && is_changed { return 1; }
    ret2
}

unsafe extern "C" fn fs210x_playback_event(w: *mut snd_soc_dapm_widget, _kc: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let fs210x = snd_soc_component_get_drvdata(cmpnt);
    let mut ret = 0;
    mutex_lock(&mut (*fs210x).lock);
    if (*fs210x).is_suspended {
        mutex_unlock(&mut (*fs210x).lock);
        return 0;
    }
    match event {
        x if x == SND_SOC_DAPM_PRE_PMU => {
            /*
             * If there is no bclk for us to set the clock output,
             * we will enable the device(start_work) in dai trigger.
             */
            if !(*fs210x).clk_bclk.is_null() {
                fs210x_bclk_set(fs210x, true);
                ret = fs210x_dev_play(fs210x);
            }
        }
        x if x == SND_SOC_DAPM_POST_PMD => {
            ret = fs210x_dev_stop(fs210x);
            fs210x_bclk_set(fs210x, false);
        }
        _ => {}
    }
    mutex_unlock(&mut (*fs210x).lock);
    ret
}

#[repr(C)] struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)] struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)] struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    symmetric_sample_bits: c_uint,
}

static fs210x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(fs210x_dai_startup),
    set_fmt: Some(fs210x_dai_set_fmt),
    hw_params: Some(fs210x_dai_hw_params),
    mute_stream: Some(fs210x_dai_mute),
    trigger: Some(fs210x_dai_trigger),
};

static fs210x_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: FS210X_DEFAULT_DAI_NAME,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: 0, /* FS210X_RATES, computed from external rate flags */
        formats: 0, /* FS210X_FORMATS, computed from external format flags */
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: 0, /* FS210X_RATES, computed from external rate flags */
        formats: 0, /* FS210X_FORMATS, computed from external format flags */
    },
    ops: &fs210x_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
};

/* static const DECLARE_TLV_DB_SCALE(fs2105s_vol_tlv, -9709, 19, 1); */
static fs2105s_vol_tlv: [c_uint; 4] = [0, (-9709i32) as c_uint, 19, 1];
/* static const DECLARE_TLV_DB_SCALE(fs210x_vol_tlv, -13357, 19, 1); */
static fs210x_vol_tlv: [c_uint; 4] = [0, (-13357i32) as c_uint, 19, 1];

#[repr(C)] struct snd_kcontrol_new { _private: [u8; 0] }
/* Mixer-control macro initializers from the C source are preserved as comments because their concrete layout is supplied by ASoC headers. */
/* SOC_DOUBLE_R_TLV("PCM Playback Volume", FS210X_39H_LVOLCTRL, FS210X_3AH_RVOLCTRL, 7, 0x1FF, 0, fs2105s_vol_tlv) */
static fs2105s_vol_control: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _private: [] }];
/* SOC_DOUBLE_R_TLV("PCM Playback Volume", FS210X_39H_LVOLCTRL, FS210X_3AH_RVOLCTRL, 6, 0x2BF, 0, fs210x_vol_tlv) */
static fs210x_vol_control: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _private: [] }];
/* SOC_DOUBLE("DAC Mute Switch", FS210X_30H_DACCTRL, 4, 8, 1, 0), SOC_DOUBLE("DAC Fade Switch", FS210X_30H_DACCTRL, 5, 9, 1, 0) */
static fs210x_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
/* FS_SOC_ENUM_EXT("Effect Scene", fs210x_effect_scene_info, fs210x_effect_scene_get, fs210x_effect_scene_put) */
static fs210x_scene_control: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _private: [] }];

#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
/* SND_SOC_DAPM_* macro initializers from the C source are preserved as comments. */
static fs210x_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];
static fs210x_dapm_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: b"OUTL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIF IN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"OUTR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIF IN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIF OUT\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SDO\0".as_ptr() as *const c_char },
];

unsafe fn fs210x_add_mixer_controls(fs210x: *mut fs210x_priv, cmpnt: *mut snd_soc_component) -> c_int {
    if fs210x.is_null() || cmpnt.is_null() { return -EINVAL; }
    let (mut kctrl, mut count) = if (*fs210x).devid == FS2105S_DEVICE_ID {
        (fs2105s_vol_control.as_ptr(), fs2105s_vol_control.len() as c_int)
    } else {
        (fs210x_vol_control.as_ptr(), fs210x_vol_control.len() as c_int)
    };
    let ret = snd_soc_add_component_controls(cmpnt, kctrl, count);
    if ret != 0 { return ret; }
    /*
     * If the firmware has no scene or only init scene,
     * we skip adding this mixer control.
     */
    if (*fs210x).amp_lib.scene_count < 2 { return 0; }
    kctrl = fs210x_scene_control.as_ptr();
    count = fs210x_scene_control.len() as c_int;
    snd_soc_add_component_controls(cmpnt, kctrl, count)
}

unsafe extern "C" fn fs210x_probe(cmpnt: *mut snd_soc_component) -> c_int {
    let fs210x = snd_soc_component_get_drvdata(cmpnt);
    if fs210x.is_null() || (*fs210x).dev.is_null() { return -EINVAL; }
    (*fs210x).amp_lib.dev = (*fs210x).dev;
    (*fs210x).amp_lib.devid = (*fs210x).devid;
    let mut ret = fs_amp_load_firmware(&mut (*fs210x).amp_lib, (*fs210x).pdata.fwm_name);
    if ret != 0 { return ret; }
    ret = fs210x_add_mixer_controls(fs210x, cmpnt);
    if ret != 0 { return ret; }
    mutex_lock(&mut (*fs210x).lock);
    ret = fs210x_init_chip(fs210x);
    mutex_unlock(&mut (*fs210x).lock);
    ret
}

unsafe extern "C" fn fs210x_remove(cmpnt: *mut snd_soc_component) {
    let fs210x = snd_soc_component_get_drvdata(cmpnt);
    if fs210x.is_null() || (*fs210x).dev.is_null() { return; }
    cancel_delayed_work_sync(&mut (*fs210x).start_work);
    cancel_delayed_work_sync(&mut (*fs210x).fault_check_work);
}

/* CONFIG_PM conditional in the C source: suspend/resume are NULL when CONFIG_PM is unset. */
unsafe extern "C" fn fs210x_suspend(cmpnt: *mut snd_soc_component) -> c_int {
    let fs210x = snd_soc_component_get_drvdata(cmpnt);
    if fs210x.is_null() || (*fs210x).dev.is_null() { return -EINVAL; }
    regcache_cache_only((*fs210x).regmap, true);
    mutex_lock(&mut (*fs210x).lock);
    (*fs210x).cur_scene = ptr::null();
    (*fs210x).is_inited = false;
    (*fs210x).is_playing = false;
    (*fs210x).is_suspended = true;
    gpiod_set_value_cansleep((*fs210x).gpio_sdz, 1); /* Active */
    fsleep(30000); /* >= 30ms */
    mutex_unlock(&mut (*fs210x).lock);
    cancel_delayed_work_sync(&mut (*fs210x).start_work);
    cancel_delayed_work_sync(&mut (*fs210x).fault_check_work);
    let ret = regulator_bulk_disable(FS210X_NUM_SUPPLIES as c_int, (*fs210x).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to suspend: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}

unsafe extern "C" fn fs210x_resume(cmpnt: *mut snd_soc_component) -> c_int {
    let fs210x = snd_soc_component_get_drvdata(cmpnt);
    if fs210x.is_null() || (*fs210x).dev.is_null() { return -EINVAL; }
    let ret = regulator_bulk_enable(FS210X_NUM_SUPPLIES as c_int, (*fs210x).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*fs210x).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    mutex_lock(&mut (*fs210x).lock);
    (*fs210x).is_suspended = false;
    let ret2 = fs210x_init_chip(fs210x);
    mutex_unlock(&mut (*fs210x).lock);
    ret2
}

unsafe extern "C" fn fs210x_volatile_registers(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x >= FS210X_00H_STATUS && x <= FS210X_0FH_I2CADDR => true,
        x if x == FS210X_ABH_INTSTAT => true,
        x if x == FS210X_ACH_INTSTATR => true,
        _ => false,
    }
}

#[repr(C)] struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
}

static fs210x_soc_component_dev: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(fs210x_probe),
    remove: Some(fs210x_remove),
    suspend: Some(fs210x_suspend),
    resume: Some(fs210x_resume),
    controls: fs210x_controls.as_ptr(),
    num_controls: 2,
    dapm_widgets: fs210x_dapm_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: fs210x_dapm_routes.as_ptr(),
    num_dapm_routes: 3,
};

#[repr(C)] struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    val_format_endian: c_int,
    cache_type: c_int,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}

static fs210x_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: FS210X_REG_MAX,
    val_format_endian: 0, /* REGMAP_ENDIAN_BIG */
    cache_type: 0, /* REGCACHE_MAPLE */
    volatile_reg: Some(fs210x_volatile_registers),
};

unsafe fn fs210x_detect_device(fs210x: *mut fs210x_priv) -> c_int {
    let mut devid: u16 = 0;
    let ret = fs210x_reg_read(fs210x, FS210X_03H_DEVID, &mut devid);
    if ret != 0 { return ret; }
    (*fs210x).devid = HI_U16(devid);
    match (*fs210x).devid {
        FS210X_DEVICE_ID => dev_info((*fs210x).dev, b"FS2104 detected\n\0".as_ptr() as *const c_char),
        FS2105S_DEVICE_ID => dev_info((*fs210x).dev, b"FS2105S detected\n\0".as_ptr() as *const c_char),
        _ => {
            dev_err((*fs210x).dev, b"DEVID: 0x%04X dismatch\n\0".as_ptr() as *const c_char, devid as c_int);
            return -ENODEV;
        }
    }
    0
}

unsafe fn fs210x_parse_dts(fs210x: *mut fs210x_priv, pdata: *mut fs210x_platform_data) -> c_int {
    let node = (*(*fs210x).dev).of_node;
    if node.is_null() { return 0; }
    let ret = of_property_read_string(node, b"firmware-name\0".as_ptr() as *const c_char, &mut (*pdata).fwm_name);
    if ret != 0 { (*pdata).fwm_name = FS210X_DEFAULT_FWM_NAME; }
    (*fs210x).gpio_sdz = devm_gpiod_get_optional((*fs210x).dev, b"reset\0".as_ptr() as *const c_char, 0);
    if IS_ERR((*fs210x).gpio_sdz as *const c_void) {
        return dev_err_probe((*fs210x).dev, PTR_ERR((*fs210x).gpio_sdz as *const c_void), b"Failed to get reset-gpios\n\0".as_ptr() as *const c_char);
    }
    for i in 0..FS210X_NUM_SUPPLIES {
        (*fs210x).supplies[i].supply = fs210x_supply_names[i];
    }
    let ret2 = devm_regulator_bulk_get((*fs210x).dev, FS210X_NUM_SUPPLIES as c_int, (*fs210x).supplies.as_mut_ptr());
    if ret2 != 0 {
        return dev_err_probe((*fs210x).dev, ret2, b"Failed to get supplies\n\0".as_ptr() as *const c_char);
    }
    0
}

unsafe fn fs210x_deinit(fs210x: *mut fs210x_priv) {
    gpiod_set_value_cansleep((*fs210x).gpio_sdz, 1); /* Active */
    fsleep(10000); /* >= 10ms */
    regulator_bulk_disable(FS210X_NUM_SUPPLIES as c_int, (*fs210x).supplies.as_mut_ptr());
}

unsafe fn fs210x_init(fs210x: *mut fs210x_priv) -> c_int {
    let mut ret = fs210x_parse_dts(fs210x, &mut (*fs210x).pdata);
    if ret != 0 { return ret; }
    (*fs210x).clk_bclk = devm_clk_get_optional((*fs210x).dev, b"bclk\0".as_ptr() as *const c_char);
    if IS_ERR((*fs210x).clk_bclk as *const c_void) {
        return dev_err_probe((*fs210x).dev, PTR_ERR((*fs210x).clk_bclk as *const c_void), b"Failed to get bclk\n\0".as_ptr() as *const c_char);
    }
    ret = regulator_bulk_enable(FS210X_NUM_SUPPLIES as c_int, (*fs210x).supplies.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe((*fs210x).dev, ret, b"Failed to enable supplies\n\0".as_ptr() as *const c_char);
    }
    /* Make sure the SDZ pin is pulled down enough time. */
    fsleep(10000); /* >= 10ms */
    gpiod_set_value_cansleep((*fs210x).gpio_sdz, 0); /* Deactivate */
    fsleep(10000); /* >= 10ms */
    ret = fs210x_detect_device(fs210x);
    if ret != 0 {
        fs210x_deinit(fs210x);
        return ret;
    }
    (*fs210x).scene_id = -1; /* Invalid scene */
    (*fs210x).cur_scene = ptr::null();
    (*fs210x).is_playing = false;
    (*fs210x).is_inited = false;
    (*fs210x).is_suspended = false;
    (*fs210x).check_interval_ms = FS210X_FAULT_CHECK_INTERVAL_MS;
    INIT_DELAYED_WORK(&mut (*fs210x).fault_check_work, fs210x_fault_check_work);
    INIT_DELAYED_WORK(&mut (*fs210x).start_work, fs210x_start_work);
    mutex_init(&mut (*fs210x).lock);
    0
}

unsafe fn fs210x_register_snd_component(fs210x: *mut fs210x_priv) -> c_int {
    static mut instance_id: c_int = 0;
    let dai_drv = devm_kmemdup((*fs210x).dev, &fs210x_dai as *const _ as *const c_void, size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if dai_drv.is_null() { return -ENOMEM; }
    (*dai_drv).name = devm_kasprintf((*fs210x).dev, GFP_KERNEL, b"%s-%d\0".as_ptr() as *const c_char, (*dai_drv).name, instance_id);
    if (*dai_drv).name.is_null() { return -ENOMEM; }
    instance_id += 1;
    if (*fs210x).devid == FS2105S_DEVICE_ID {
        (*dai_drv).playback.rates = FS2105S_RATES();
        (*dai_drv).capture.rates = FS2105S_RATES();
    }
    snd_soc_register_component((*fs210x).dev, &fs210x_soc_component_dev, dai_drv, 1)
}

unsafe extern "C" fn check_interval_ms_show(dev: *mut device, _devattr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let fs210x = dev_get_drvdata(dev);
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, (*fs210x).check_interval_ms as c_int)
}

unsafe extern "C" fn check_interval_ms_store(dev: *mut device, _devattr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let fs210x = dev_get_drvdata(dev);
    let ret = kstrtouint(buf, 10, &mut (*fs210x).check_interval_ms);
    if ret != 0 { return -EINVAL as ssize_t; }
    count as ssize_t
}

/* static DEVICE_ATTR_RW(check_interval_ms); */
static mut fs210x_attrs: [*mut attribute; 2] = [ptr::null_mut(), ptr::null_mut()];

#[repr(C)] struct attribute_group { attrs: *mut *mut attribute }
static mut fs210x_attr_group: attribute_group = attribute_group { attrs: ptr::null_mut() };

unsafe extern "C" fn fs210x_i2c_probe(client: *mut i2c_client) -> c_int {
    let fs210x = devm_kzalloc(&mut (*client).dev, size_of::<fs210x_priv>(), GFP_KERNEL) as *mut fs210x_priv;
    if fs210x.is_null() { return -ENOMEM; }
    (*fs210x).i2c = client;
    (*fs210x).dev = &mut (*client).dev;
    i2c_set_clientdata(client, fs210x);
    (*fs210x).regmap = devm_regmap_init_i2c(client, &fs210x_regmap);
    if IS_ERR((*fs210x).regmap as *const c_void) {
        return dev_err_probe((*fs210x).dev, PTR_ERR((*fs210x).regmap as *const c_void), b"Failed to get regmap\n\0".as_ptr() as *const c_char);
    }
    let mut ret = fs210x_init(fs210x);
    if ret != 0 { return ret; }
    ret = devm_device_add_group((*fs210x).dev, &raw const fs210x_attr_group);
    if ret != 0 {
        fs210x_deinit(fs210x);
        return dev_err_probe((*fs210x).dev, ret, b"Failed to create sysfs group\n\0".as_ptr() as *const c_char);
    }
    ret = fs210x_register_snd_component(fs210x);
    if ret != 0 {
        fs210x_deinit(fs210x);
        return dev_err_probe((*fs210x).dev, ret, b"Failed to register component\n\0".as_ptr() as *const c_char);
    }
    0
}

unsafe extern "C" fn fs210x_i2c_remove(client: *mut i2c_client) {
    let fs210x = i2c_get_clientdata(client);
    snd_soc_unregister_component((*fs210x).dev);
    fs210x_deinit(fs210x);
}

#[repr(C)] struct i2c_device_id { name: [c_char; 20] }
static fs210x_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: [b'f' as c_char, b's' as c_char, b'2' as c_char, b'1' as c_char, b'0' as c_char, b'4' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [b'f' as c_char, b's' as c_char, b'2' as c_char, b'1' as c_char, b'0' as c_char, b'5' as c_char, b's' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, fs210x_i2c_id); */

#[repr(C)] struct of_device_id { compatible: *const c_char }
static fs210x_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"foursemi,fs2105s\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, fs210x_of_match); */

#[repr(C)] struct driver_inner { name: *const c_char, of_match_table: *const of_device_id }
#[repr(C)] struct i2c_driver {
    driver: driver_inner,
    id_table: *const i2c_device_id,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

static fs210x_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_inner {
        name: b"fs210x\0".as_ptr() as *const c_char,
        of_match_table: fs210x_of_match.as_ptr(),
    },
    id_table: fs210x_i2c_id.as_ptr(),
    probe: Some(fs210x_i2c_probe),
    remove: Some(fs210x_i2c_remove),
};

/* module_i2c_driver(fs210x_i2c_driver); */
/* MODULE_AUTHOR("Nick Li <nick.li@foursemi.com>"); */
/* MODULE_DESCRIPTION("FS2104/5S Audio Amplifier Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
