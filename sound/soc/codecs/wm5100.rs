// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm5100.rs  --  WM5100 ALSA SoC Audio driver
 *
 * Source-level Rust translation of wm5100.c.
 *
 * Includes from the C source:
 * linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
 * linux/export.h, linux/pm.h, linux/gcd.h, linux/gpio/driver.h,
 * linux/gpio/consumer.h, linux/i2c.h, linux/pm_runtime.h,
 * linux/regulator/consumer.h, linux/regulator/fixed.h, linux/slab.h,
 * sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
 * sound/jack.h, sound/initval.h, sound/tlv.h, sound/wm5100.h, "wm5100.h".
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u16 = core::ffi::c_ushort;
type irqreturn_t = c_int;

const WM5100_NUM_CORE_SUPPLIES: usize = 2;
static wm5100_core_supply_names: [*const c_char; WM5100_NUM_CORE_SUPPLIES] = [
    b"DBVDD1\0".as_ptr() as *const c_char,
    b"LDOVDD\0".as_ptr() as *const c_char, /* If DCVDD is supplied externally specify as LDOVDD */
];

const WM5100_AIFS: usize = 3;
const WM5100_SYNC_SRS: usize = 3;

#[repr(C)]
struct device { _private: [u8; 0] }
#[repr(C)]
struct regmap { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_component { dev: *mut device }
#[repr(C)]
struct regulator_bulk_data { supply: *const c_char }
#[repr(C)]
struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)]
struct completion { _private: [u8; 0] }
#[repr(C)]
struct gpio_chip { _private: [u8; 0] }
#[repr(C)]
struct wm5100_jack_mode { hp_pol: c_int, bias: c_int, micd_src: c_int }
#[repr(C)]
struct wm5100_pdata {
    jack_modes: [wm5100_jack_mode; 2],
    gpio_defaults: [c_uint; 6],
    in_mode: [c_uint; 4],
    dmic_sup: [c_uint; 4],
    irq_flags: c_int,
}
#[repr(C)]
struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dapm_widget { dapm: *mut snd_soc_dapm_context, reg: c_int }
#[repr(C)]
struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    base: c_int,
}
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
    id: c_int,
    driver: *mut snd_soc_dai_driver,
}
#[repr(C)]
struct snd_pcm_substream { stream: c_int }
#[repr(C)]
struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
struct i2c_client { dev: device, irq: c_int }
#[repr(C)]
struct reg_sequence { reg: c_uint, def: c_uint }

#[repr(C)]
struct wm5100_fll {
    fref: c_int,
    fout: c_int,
    src: c_int,
    lock: completion,
}

/* codec private data */
#[repr(C)]
struct wm5100_priv {
    dev: *mut device,
    regmap: *mut regmap,
    component: *mut snd_soc_component,
    core_supplies: [regulator_bulk_data; WM5100_NUM_CORE_SUPPLIES],
    reset: *mut gpio_desc,
    ldo_ena: *mut gpio_desc,
    hp_pol: *mut gpio_desc,
    rev: c_int,
    sysclk: c_int,
    asyncclk: c_int,
    aif_async: [bool; WM5100_AIFS],
    aif_symmetric: [bool; WM5100_AIFS],
    sr_ref: [c_int; WM5100_SYNC_SRS],
    out_ena: [bool; 2],
    jack: *mut snd_soc_jack,
    jack_detecting: bool,
    jack_mic: bool,
    jack_mode: c_int,
    jack_flips: c_int,
    fll: [wm5100_fll; 2],
    pdata: wm5100_pdata,
    /* CONFIG_GPIOLIB: struct gpio_chip gpio_chip; */
}

static mut wm5100_sr_code: [c_int; 24] = [
    0, 12000, 24000, 48000, 96000, 192000, 384000, 768000,
    0, 11025, 22050, 44100, 88200, 176400, 352800, 705600,
    4000, 8000, 16000, 32000, 64000, 128000, 256000, 512000,
];

static mut wm5100_sr_regs: [c_int; WM5100_SYNC_SRS] = [
    WM5100_CLOCKING_4, WM5100_CLOCKING_5, WM5100_CLOCKING_6,
];

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_int) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_int, mask: c_int, val: c_int) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_int, val: c_int) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn regmap_write(map: *mut regmap, reg: c_int, val: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_int, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_int, mask: c_int, val: c_int) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn wait_for_completion_timeout(x: *mut completion, timeout: c_ulong) -> c_ulong;
    fn try_wait_for_completion(x: *mut completion) -> bool;
    fn complete(x: *mut completion);
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn msleep(msecs: c_uint);
    fn gcd(a: c_uint, b: c_uint) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
}

extern "C" {
    static WM5100_CLOCKING_1: c_int; static WM5100_CLOCKING_3: c_int;
    static WM5100_CLOCKING_4: c_int; static WM5100_CLOCKING_5: c_int;
    static WM5100_CLOCKING_6: c_int; static WM5100_CLOCKING_7: c_int;
    static WM5100_CLOCKING_8: c_int; static WM5100_SOFTWARE_RESET: c_int;
    static WM5100_SAMPLE_RATE_1_MASK: c_int; static WM5100_CHANNEL_ENABLES_1: c_int;
    static WM5100_OUTPUT_ENABLES_2: c_int; static WM5100_OUTPUT_STATUS_1: c_int;
    static WM5100_OUTPUT_STATUS_2: c_int; static WM5100_INTERRUPT_RAW_STATUS_3: c_int;
    static WM5100_INTERRUPT_RAW_STATUS_4: c_int; static WM5100_INTERRUPT_STATUS_3: c_int;
    static WM5100_INTERRUPT_STATUS_3_MASK: c_int; static WM5100_INTERRUPT_STATUS_4: c_int;
    static WM5100_INTERRUPT_STATUS_4_MASK: c_int; static WM5100_MIC_DETECT_1: c_int;
    static WM5100_MIC_DETECT_3: c_int; static WM5100_ACCESSORY_DETECT_MODE_1: c_int;
    static WM5100_MISC_CONTROL: c_int; static WM5100_MISC_GPIO_1: c_int;
    static WM5100_GPIO_CTRL_1: c_int;
}

/* TLV declarations translated from DECLARE_TLV_DB_SCALE(). */
static in_tlv: [c_int; 3] = [-6300, 100, 0];
static eq_tlv: [c_int; 3] = [-1200, 100, 0];
static mixer_tlv: [c_int; 3] = [-3200, 100, 0];
static out_tlv: [c_int; 3] = [-6400, 100, 0];
static digital_tlv: [c_int; 3] = [-6400, 50, 0];

static wm5100_mixer_texts: [&str; 76] = [
    "None", "Tone Generator 1", "Tone Generator 2", "AEC loopback",
    "IN1L", "IN1R", "IN2L", "IN2R", "IN3L", "IN3R", "IN4L", "IN4R",
    "AIF1RX1", "AIF1RX2", "AIF1RX3", "AIF1RX4", "AIF1RX5", "AIF1RX6", "AIF1RX7", "AIF1RX8",
    "AIF2RX1", "AIF2RX2", "AIF3RX1", "AIF3RX2",
    "EQ1", "EQ2", "EQ3", "EQ4", "DRC1L", "DRC1R",
    "LHPF1", "LHPF2", "LHPF3", "LHPF4",
    "DSP1.1", "DSP1.2", "DSP1.3", "DSP1.4", "DSP1.5", "DSP1.6",
    "DSP2.1", "DSP2.2", "DSP2.3", "DSP2.4", "DSP2.5", "DSP2.6",
    "DSP3.1", "DSP3.2", "DSP3.3", "DSP3.4", "DSP3.5", "DSP3.6",
    "ASRC1L", "ASRC1R", "ASRC2L", "ASRC2R",
    "ISRC1INT1", "ISRC1INT2", "ISRC1INT3", "ISRC1INT4",
    "ISRC2INT1", "ISRC2INT2", "ISRC2INT3", "ISRC2INT4",
    "ISRC1DEC1", "ISRC1DEC2", "ISRC1DEC3", "ISRC1DEC4",
    "ISRC2DEC1", "ISRC2DEC2", "ISRC2DEC3", "ISRC2DEC4",
];

static wm5100_mixer_values: [c_int; 76] = [
    0x00, 0x04, 0x05, 0x08, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x30, 0x31,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x58, 0x59, 0x60, 0x61, 0x62, 0x63,
    0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75,
    0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x90, 0x91, 0x92, 0x93,
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
    0xac, 0xad, 0xae, 0xaf,
];

/*
 * The C source defines ALSA macro-generated controls, enums, widgets, and
 * routes here:
 * WM5100_MIXER_CONTROLS, WM5100_MUX_ENUM_DECL, WM5100_MUX_CTL_DECL,
 * WM5100_MIXER_ENUMS, WM5100_MUX, WM5100_MIXER_WIDGETS,
 * WM5100_MIXER_INPUT_ROUTES, WM5100_MIXER_ROUTES, the LHPF mode enums,
 * wm5100_snd_controls, wm5100_dapm_widgets, wm5100_dapm_widgets_noirq,
 * and wm5100_dapm_routes.  Their data is preserved in macro form in the
 * original source and depends on external ALSA macro constructors.
 */

unsafe fn wm5100_alloc_sr(component: *mut snd_soc_component, rate: c_int) -> c_int {
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    let mut i: usize = 0;
    while i < wm5100_sr_code.len() {
        if wm5100_sr_code[i] == rate { break; }
        i += 1;
    }
    if i == wm5100_sr_code.len() { return -EINVAL; }
    let sr_code = i as c_int;
    if (*wm5100).sysclk % rate == 0 {
        let mut sr_free: c_int = -1;
        i = 0;
        while i < wm5100_sr_regs.len() {
            if (*wm5100).sr_ref[i] == 0 && sr_free == -1 {
                sr_free = i as c_int;
                i += 1;
                continue;
            }
            if (snd_soc_component_read(component, wm5100_sr_regs[i]) & WM5100_SAMPLE_RATE_1_MASK) == sr_code {
                break;
            }
            i += 1;
        }
        if i < wm5100_sr_regs.len() {
            (*wm5100).sr_ref[i] += 1;
            return i as c_int;
        }
        if sr_free == -1 { return -EBUSY; }
        (*wm5100).sr_ref[sr_free as usize] += 1;
        snd_soc_component_update_bits(component, wm5100_sr_regs[sr_free as usize], WM5100_SAMPLE_RATE_1_MASK, sr_code);
        sr_free
    } else {
        -EINVAL
    }
}

unsafe fn wm5100_free_sr(component: *mut snd_soc_component, rate: c_int) {
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    let mut i: usize = 0;
    while i < wm5100_sr_code.len() {
        if wm5100_sr_code[i] == rate { break; }
        i += 1;
    }
    if i == wm5100_sr_code.len() { return; }
    let sr_code = wm5100_sr_code[i];
    i = 0;
    while i < wm5100_sr_regs.len() {
        if (*wm5100).sr_ref[i] == 0 {
            i += 1;
            continue;
        }
        if (snd_soc_component_read(component, wm5100_sr_regs[i]) & WM5100_SAMPLE_RATE_1_MASK) == sr_code {
            break;
        }
        i += 1;
    }
    if i < wm5100_sr_regs.len() {
        (*wm5100).sr_ref[i] -= 1;
    }
}

unsafe fn wm5100_reset(wm5100: *mut wm5100_priv) -> c_int {
    if !(*wm5100).reset.is_null() {
        gpiod_set_value_cansleep((*wm5100).reset, 1);
        gpiod_set_value_cansleep((*wm5100).reset, 0);
        0
    } else {
        regmap_write((*wm5100).regmap, WM5100_SOFTWARE_RESET, 0)
    }
}

unsafe fn wm5100_seq_notifier(component: *mut snd_soc_component, event: c_int, subseq: c_int) {
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    if (*wm5100).out_ena[0] {
        let expect = snd_soc_component_read(component, WM5100_CHANNEL_ENABLES_1) as u16;
        let mut i: u16 = 0;
        while i < 200 {
            let val = snd_soc_component_read(component, WM5100_OUTPUT_STATUS_1) as u16;
            if val == expect {
                (*wm5100).out_ena[0] = false;
                break;
            }
            i += 1;
        }
    }
    if (*wm5100).out_ena[1] {
        let expect = snd_soc_component_read(component, WM5100_OUTPUT_ENABLES_2) as u16;
        let mut i: u16 = 0;
        while i < 200 {
            let val = snd_soc_component_read(component, WM5100_OUTPUT_STATUS_2) as u16;
            if val == expect {
                (*wm5100).out_ena[1] = false;
                break;
            }
            i += 1;
        }
    }
}

unsafe fn wm5100_out_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    if (*w).reg == WM5100_CHANNEL_ENABLES_1 {
        (*wm5100).out_ena[0] = true;
    } else if (*w).reg == WM5100_OUTPUT_ENABLES_2 {
        (*wm5100).out_ena[0] = true;
    }
    0
}

unsafe fn wm5100_log_status3(wm5100: *mut wm5100_priv, val: c_int) {
    if val & WM5100_SPK_SHUTDOWN_WARN_EINT != 0 {}
    if val & WM5100_SPK_SHUTDOWN_EINT != 0 {}
    if val & WM5100_CLKGEN_ERR_EINT != 0 {}
    if val & WM5100_CLKGEN_ERR_ASYNC_EINT != 0 {}
}

unsafe fn wm5100_log_status4(wm5100: *mut wm5100_priv, val: c_int) {
    if val & WM5100_AIF3_ERR_EINT != 0 {}
    if val & WM5100_AIF2_ERR_EINT != 0 {}
    if val & WM5100_AIF1_ERR_EINT != 0 {}
    if val & WM5100_CTRLIF_ERR_EINT != 0 {}
    if val & WM5100_ISRC2_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_ISRC1_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_FX_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_AIF3_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_AIF2_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_AIF1_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_ASRC_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_DAC_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_ADC_UNDERCLOCKED_EINT != 0 {}
    if val & WM5100_MIXER_UNDERCLOCKED_EINT != 0 {}
}

unsafe fn wm5100_post_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    let mut ret = snd_soc_component_read(component, WM5100_INTERRUPT_RAW_STATUS_3);
    ret &= WM5100_SPK_SHUTDOWN_WARN_STS | WM5100_SPK_SHUTDOWN_STS | WM5100_CLKGEN_ERR_STS | WM5100_CLKGEN_ERR_ASYNC_STS;
    wm5100_log_status3(wm5100, ret);
    ret = snd_soc_component_read(component, WM5100_INTERRUPT_RAW_STATUS_4);
    wm5100_log_status4(wm5100, ret);
    0
}

static wm5100_reva_patches: [reg_sequence; 28] = [
    reg_sequence { reg: WM5100_AUDIO_IF_1_10 as c_uint, def: 0 }, reg_sequence { reg: WM5100_AUDIO_IF_1_11 as c_uint, def: 1 },
    reg_sequence { reg: WM5100_AUDIO_IF_1_12 as c_uint, def: 2 }, reg_sequence { reg: WM5100_AUDIO_IF_1_13 as c_uint, def: 3 },
    reg_sequence { reg: WM5100_AUDIO_IF_1_14 as c_uint, def: 4 }, reg_sequence { reg: WM5100_AUDIO_IF_1_15 as c_uint, def: 5 },
    reg_sequence { reg: WM5100_AUDIO_IF_1_16 as c_uint, def: 6 }, reg_sequence { reg: WM5100_AUDIO_IF_1_17 as c_uint, def: 7 },
    reg_sequence { reg: WM5100_AUDIO_IF_1_18 as c_uint, def: 0 }, reg_sequence { reg: WM5100_AUDIO_IF_1_19 as c_uint, def: 1 },
    reg_sequence { reg: WM5100_AUDIO_IF_1_20 as c_uint, def: 2 }, reg_sequence { reg: WM5100_AUDIO_IF_1_21 as c_uint, def: 3 },
    reg_sequence { reg: WM5100_AUDIO_IF_1_22 as c_uint, def: 4 }, reg_sequence { reg: WM5100_AUDIO_IF_1_23 as c_uint, def: 5 },
    reg_sequence { reg: WM5100_AUDIO_IF_1_24 as c_uint, def: 6 }, reg_sequence { reg: WM5100_AUDIO_IF_1_25 as c_uint, def: 7 },
    reg_sequence { reg: WM5100_AUDIO_IF_2_10 as c_uint, def: 0 }, reg_sequence { reg: WM5100_AUDIO_IF_2_11 as c_uint, def: 1 },
    reg_sequence { reg: WM5100_AUDIO_IF_2_18 as c_uint, def: 0 }, reg_sequence { reg: WM5100_AUDIO_IF_2_19 as c_uint, def: 1 },
    reg_sequence { reg: WM5100_AUDIO_IF_3_10 as c_uint, def: 0 }, reg_sequence { reg: WM5100_AUDIO_IF_3_11 as c_uint, def: 1 },
    reg_sequence { reg: WM5100_AUDIO_IF_3_18 as c_uint, def: 0 }, reg_sequence { reg: WM5100_AUDIO_IF_3_19 as c_uint, def: 1 },
    reg_sequence { reg: 0, def: 0 }, reg_sequence { reg: 0, def: 0 }, reg_sequence { reg: 0, def: 0 }, reg_sequence { reg: 0, def: 0 },
];

unsafe fn wm5100_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let base = (*(*dai).driver).base;
    let mut lrclk = 0;
    let mut bclk = 0;
    let mask: c_int;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK as c_uint {
        x if x == SND_SOC_DAIFMT_DSP_A as c_uint => mask = 0,
        x if x == SND_SOC_DAIFMT_I2S as c_uint => mask = 2,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_MASTER_MASK as c_uint {
        x if x == SND_SOC_DAIFMT_CBC_CFC as c_uint => {}
        x if x == SND_SOC_DAIFMT_CBC_CFP as c_uint => lrclk |= WM5100_AIF1TX_LRCLK_MSTR,
        x if x == SND_SOC_DAIFMT_CBP_CFC as c_uint => bclk |= WM5100_AIF1_BCLK_MSTR,
        x if x == SND_SOC_DAIFMT_CBP_CFP as c_uint => { lrclk |= WM5100_AIF1TX_LRCLK_MSTR; bclk |= WM5100_AIF1_BCLK_MSTR; }
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK as c_uint {
        x if x == SND_SOC_DAIFMT_NB_NF as c_uint => {}
        x if x == SND_SOC_DAIFMT_IB_IF as c_uint => { bclk |= WM5100_AIF1_BCLK_INV; lrclk |= WM5100_AIF1TX_LRCLK_INV; }
        x if x == SND_SOC_DAIFMT_IB_NF as c_uint => bclk |= WM5100_AIF1_BCLK_INV,
        x if x == SND_SOC_DAIFMT_NB_IF as c_uint => lrclk |= WM5100_AIF1TX_LRCLK_INV,
        _ => return -EINVAL,
    }
    snd_soc_component_update_bits(component, base + 1, WM5100_AIF1_BCLK_MSTR | WM5100_AIF1_BCLK_INV, bclk);
    snd_soc_component_update_bits(component, base + 2, WM5100_AIF1TX_LRCLK_MSTR | WM5100_AIF1TX_LRCLK_INV, lrclk);
    snd_soc_component_update_bits(component, base + 3, WM5100_AIF1TX_LRCLK_MSTR | WM5100_AIF1TX_LRCLK_INV, lrclk);
    snd_soc_component_update_bits(component, base + 5, WM5100_AIF1_FMT_MASK, mask);
    0
}

const WM5100_NUM_BCLK_RATES: usize = 19;
static wm5100_bclk_rates_dat: [c_int; WM5100_NUM_BCLK_RATES] = [
    32000, 48000, 64000, 96000, 128000, 192000, 256000, 384000, 512000,
    768000, 1024000, 1536000, 2048000, 3072000, 4096000, 6144000,
    8192000, 12288000, 24576000,
];
static wm5100_bclk_rates_cd: [c_int; WM5100_NUM_BCLK_RATES] = [
    29400, 44100, 58800, 88200, 117600, 176400, 235200, 352800, 470400,
    705600, 940800, 1411200, 1881600, 2882400, 3763200, 5644800,
    7526400, 11289600, 22579600,
];

unsafe fn wm5100_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    let async_ = (*wm5100).aif_async[(*dai).id as usize];
    let base = (*(*dai).driver).base;
    let wl = params_width(params);
    if wl < 0 { return wl; }
    let fl = snd_soc_params_to_frame_size(params);
    if fl < 0 { return fl; }
    let mut bclk = snd_soc_params_to_bclk(params);
    if bclk < 0 { return bclk; }
    let aif_rate: c_int;
    let sr: c_int;
    if !async_ {
        aif_rate = (*wm5100).sysclk;
        sr = wm5100_alloc_sr(component, params_rate(params));
        if sr < 0 { return sr; }
    } else {
        aif_rate = (*wm5100).asyncclk;
        sr = 3;
        let mut i = 0usize;
        while i < wm5100_sr_code.len() {
            if params_rate(params) == wm5100_sr_code[i] { break; }
            i += 1;
        }
        if i == wm5100_sr_code.len() { return -EINVAL; }
        snd_soc_component_update_bits(component, WM5100_CLOCKING_8, WM5100_ASYNC_SAMPLE_RATE_MASK, i as c_int);
    }
    if aif_rate == 0 { return -EINVAL; }
    let bclk_rates = if aif_rate % 4000 != 0 { &wm5100_bclk_rates_cd } else { &wm5100_bclk_rates_dat };
    let mut i = 0usize;
    while i < WM5100_NUM_BCLK_RATES {
        if bclk_rates[i] >= bclk && bclk_rates[i] % bclk == 0 { break; }
        i += 1;
    }
    if i == WM5100_NUM_BCLK_RATES { return -EINVAL; }
    bclk = i as c_int;
    snd_soc_component_update_bits(component, base + 1, WM5100_AIF1_BCLK_FREQ_MASK, bclk);
    let lrclk = bclk_rates[bclk as usize] / params_rate(params);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK || (*wm5100).aif_symmetric[(*dai).id as usize] {
        snd_soc_component_update_bits(component, base + 7, WM5100_AIF1RX_BCPF_MASK, lrclk);
    } else {
        snd_soc_component_update_bits(component, base + 6, WM5100_AIF1TX_BCPF_MASK, lrclk);
    }
    i = ((wl << WM5100_AIF1TX_WL_SHIFT) | fl) as usize;
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_component_update_bits(component, base + 9, WM5100_AIF1RX_WL_MASK | WM5100_AIF1RX_SLOT_LEN_MASK, i as c_int);
    } else {
        snd_soc_component_update_bits(component, base + 8, WM5100_AIF1TX_WL_MASK | WM5100_AIF1TX_SLOT_LEN_MASK, i as c_int);
    }
    snd_soc_component_update_bits(component, base + 4, WM5100_AIF1_RATE_MASK, sr);
    0
}

unsafe fn wm5100_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int {
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    let rate_store: *mut c_int;
    let reg: c_int;
    match clk_id {
        WM5100_CLK_SYSCLK => { reg = WM5100_CLOCKING_3; rate_store = &mut (*wm5100).sysclk; }
        WM5100_CLK_ASYNCCLK => { reg = WM5100_CLOCKING_7; rate_store = &mut (*wm5100).asyncclk; }
        WM5100_CLK_32KHZ => {
            match source {
                WM5100_CLKSRC_MCLK1 | WM5100_CLKSRC_MCLK2 | WM5100_CLKSRC_SYSCLK =>
                    { snd_soc_component_update_bits(component, WM5100_CLOCKING_1, WM5100_CLK_32K_SRC_MASK, source); }
                _ => return -EINVAL,
            }
            return 0;
        }
        WM5100_CLK_AIF1 | WM5100_CLK_AIF2 | WM5100_CLK_AIF3 => {
            match source {
                WM5100_CLKSRC_SYSCLK => (*wm5100).aif_async[(clk_id - 1) as usize] = false,
                WM5100_CLKSRC_ASYNCCLK => (*wm5100).aif_async[(clk_id - 1) as usize] = true,
                _ => return -EINVAL,
            }
            return 0;
        }
        WM5100_CLK_OPCLK => {
            match freq {
                5644800 | 6144000 | 11289600 | 12288000 | 22579200 | 24576000 =>
                    { snd_soc_component_update_bits(component, WM5100_MISC_GPIO_1, WM5100_OPCLK_SEL_MASK, 0); }
                _ => return -EINVAL,
            }
            return 0;
        }
        _ => return -EINVAL,
    }
    match source {
        WM5100_CLKSRC_SYSCLK | WM5100_CLKSRC_ASYNCCLK => return -EINVAL,
        _ => {}
    }
    let fval = match freq {
        5644800 | 6144000 => 0,
        11289600 | 12288000 => 1,
        22579200 | 24576000 => 2,
        _ => return -EINVAL,
    };
    let audio_rate = match freq {
        5644800 | 11289600 | 22579200 => 44100,
        6144000 | 12288000 | 24576000 => 48000,
        _ => 0,
    };
    snd_soc_component_update_bits(component, reg, WM5100_SYSCLK_FREQ_MASK | WM5100_SYSCLK_SRC_MASK,
                                  (fval << WM5100_SYSCLK_FREQ_SHIFT) | source);
    if clk_id == WM5100_CLK_SYSCLK {
        let ret = wm5100_alloc_sr(component, audio_rate);
        if ret != 0 {}
    }
    *rate_store = freq as c_int;
    0
}

#[repr(C)]
struct _fll_div { fll_fratio: u16, fll_outdiv: u16, fll_refclk_div: u16, n: u16, theta: u16, lambda: u16 }
#[repr(C)]
struct fll_fratio { min: c_uint, max: c_uint, fll_fratio: u16, ratio: c_int }
static fll_fratios: [fll_fratio; 5] = [
    fll_fratio { min: 0, max: 64000, fll_fratio: 4, ratio: 16 },
    fll_fratio { min: 64000, max: 128000, fll_fratio: 3, ratio: 8 },
    fll_fratio { min: 128000, max: 256000, fll_fratio: 2, ratio: 4 },
    fll_fratio { min: 256000, max: 1000000, fll_fratio: 1, ratio: 2 },
    fll_fratio { min: 1000000, max: 13500000, fll_fratio: 0, ratio: 1 },
];

unsafe fn fll_factors(fll_div: *mut _fll_div, mut Fref: c_uint, Fout: c_uint) -> c_int {
    let mut div: c_uint = 1;
    (*fll_div).fll_refclk_div = 0;
    while Fref / div > 13500000 {
        div *= 2;
        (*fll_div).fll_refclk_div += 1;
        if div > 8 { return -EINVAL; }
    }
    Fref /= div;
    div = 2;
    while Fout * div < 90000000 {
        div += 1;
        if div > 64 { return -EINVAL; }
    }
    let target = Fout * div;
    (*fll_div).fll_outdiv = (div - 1) as u16;
    let mut fratio: c_uint = 0;
    let mut i = 0usize;
    while i < fll_fratios.len() {
        if fll_fratios[i].min <= Fref && Fref <= fll_fratios[i].max {
            (*fll_div).fll_fratio = fll_fratios[i].fll_fratio;
            fratio = fll_fratios[i].ratio as c_uint;
            break;
        }
        i += 1;
    }
    if i == fll_fratios.len() { return -EINVAL; }
    (*fll_div).n = (target / (fratio * Fref)) as u16;
    if target % Fref == 0 {
        (*fll_div).theta = 0;
        (*fll_div).lambda = 0;
    } else {
        let gcd_fll = gcd(target, fratio * Fref);
        (*fll_div).theta = ((target - ((*fll_div).n as c_uint * fratio * Fref)) / gcd_fll) as u16;
        (*fll_div).lambda = ((fratio * Fref) / gcd_fll) as u16;
    }
    0
}

unsafe fn wm5100_set_fll(component: *mut snd_soc_component, fll_id: c_int, source: c_int, Fref: c_uint, Fout: c_uint) -> c_int {
    let i2c = to_i2c_client((*component).dev);
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    let mut factors = _fll_div { fll_fratio: 0, fll_outdiv: 0, fll_refclk_div: 0, n: 0, theta: 0, lambda: 0 };
    let fll: *mut wm5100_fll;
    let base: c_int;
    let lock: c_int;
    match fll_id {
        WM5100_FLL1 => { fll = &mut (*wm5100).fll[0]; base = WM5100_FLL1_CONTROL_1 - 1; lock = WM5100_FLL1_LOCK_STS; }
        WM5100_FLL2 => { fll = &mut (*wm5100).fll[1]; base = WM5100_FLL2_CONTROL_2 - 1; lock = WM5100_FLL2_LOCK_STS; }
        _ => return -EINVAL,
    }
    if Fout == 0 {
        if (*fll).fout != 0 { pm_runtime_put((*component).dev); }
        (*fll).fout = 0;
        snd_soc_component_update_bits(component, base + 1, WM5100_FLL1_ENA, 0);
        return 0;
    }
    match source {
        WM5100_FLL_SRC_MCLK1 | WM5100_FLL_SRC_MCLK2 | WM5100_FLL_SRC_FLL1 | WM5100_FLL_SRC_FLL2 |
        WM5100_FLL_SRC_AIF1BCLK | WM5100_FLL_SRC_AIF2BCLK | WM5100_FLL_SRC_AIF3BCLK => {}
        _ => return -EINVAL,
    }
    let mut ret = fll_factors(&mut factors, Fref, Fout);
    if ret < 0 { return ret; }
    snd_soc_component_update_bits(component, base + 1, WM5100_FLL1_ENA, 0);
    snd_soc_component_update_bits(component, base + 2, WM5100_FLL1_OUTDIV_MASK | WM5100_FLL1_FRATIO_MASK,
                                  ((factors.fll_outdiv as c_int) << WM5100_FLL1_OUTDIV_SHIFT) | factors.fll_fratio as c_int);
    snd_soc_component_update_bits(component, base + 3, WM5100_FLL1_THETA_MASK, factors.theta as c_int);
    snd_soc_component_update_bits(component, base + 5, WM5100_FLL1_N_MASK, factors.n as c_int);
    snd_soc_component_update_bits(component, base + 6, WM5100_FLL1_REFCLK_DIV_MASK | WM5100_FLL1_REFCLK_SRC_MASK,
                                  ((factors.fll_refclk_div as c_int) << WM5100_FLL1_REFCLK_DIV_SHIFT) | source);
    snd_soc_component_update_bits(component, base + 7, WM5100_FLL1_LAMBDA_MASK, factors.lambda as c_int);
    try_wait_for_completion(&mut (*fll).lock);
    pm_runtime_get_sync((*component).dev);
    snd_soc_component_update_bits(component, base + 1, WM5100_FLL1_ENA, WM5100_FLL1_ENA);
    let timeout = if (*i2c).irq != 0 { 2 } else { 50 };
    snd_soc_component_update_bits(component, WM5100_CLOCKING_3, WM5100_SYSCLK_ENA, WM5100_SYSCLK_ENA);
    let mut i = 0;
    while i < timeout {
        if (*i2c).irq != 0 {
            if wait_for_completion_timeout(&mut (*fll).lock, msecs_to_jiffies(25)) > 0 { break; }
        } else {
            msleep(1);
        }
        ret = snd_soc_component_read(component, WM5100_INTERRUPT_RAW_STATUS_3);
        if ret >= 0 && (ret & lock) != 0 { break; }
        i += 1;
    }
    if i == timeout {
        pm_runtime_put((*component).dev);
        return -ETIMEDOUT;
    }
    (*fll).src = source;
    (*fll).fref = Fref as c_int;
    (*fll).fout = Fout as c_int;
    0
}

const WM5100_RATES: c_int = SNDRV_PCM_RATE_8000_192000;
const WM5100_FORMATS: c_int = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut wm5100_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver { name: b"wm5100-aif1\0".as_ptr() as *const c_char, id: 0, base: WM5100_AUDIO_IF_1_1 - 1 },
    snd_soc_dai_driver { name: b"wm5100-aif2\0".as_ptr() as *const c_char, id: 1, base: WM5100_AUDIO_IF_2_1 - 1 },
    snd_soc_dai_driver { name: b"wm5100-aif3\0".as_ptr() as *const c_char, id: 2, base: WM5100_AUDIO_IF_3_1 - 1 },
];

static wm5100_dig_vu: [c_int; 20] = [
    WM5100_ADC_DIGITAL_VOLUME_1L, WM5100_ADC_DIGITAL_VOLUME_1R,
    WM5100_ADC_DIGITAL_VOLUME_2L, WM5100_ADC_DIGITAL_VOLUME_2R,
    WM5100_ADC_DIGITAL_VOLUME_3L, WM5100_ADC_DIGITAL_VOLUME_3R,
    WM5100_ADC_DIGITAL_VOLUME_4L, WM5100_ADC_DIGITAL_VOLUME_4R,
    WM5100_DAC_DIGITAL_VOLUME_1L, WM5100_DAC_DIGITAL_VOLUME_1R,
    WM5100_DAC_DIGITAL_VOLUME_2L, WM5100_DAC_DIGITAL_VOLUME_2R,
    WM5100_DAC_DIGITAL_VOLUME_3L, WM5100_DAC_DIGITAL_VOLUME_3R,
    WM5100_DAC_DIGITAL_VOLUME_4L, WM5100_DAC_DIGITAL_VOLUME_4R,
    WM5100_DAC_DIGITAL_VOLUME_5L, WM5100_DAC_DIGITAL_VOLUME_5R,
    WM5100_DAC_DIGITAL_VOLUME_6L, WM5100_DAC_DIGITAL_VOLUME_6R,
];

unsafe fn wm5100_set_detect_mode(wm5100: *mut wm5100_priv, the_mode: c_int) {
    if the_mode as usize >= (*wm5100).pdata.jack_modes.len() { return; }
    let mode = &(*wm5100).pdata.jack_modes[the_mode as usize] as *const wm5100_jack_mode;
    gpiod_set_value_cansleep((*wm5100).hp_pol, (*mode).hp_pol);
    regmap_update_bits((*wm5100).regmap, WM5100_ACCESSORY_DETECT_MODE_1,
                       WM5100_ACCDET_BIAS_SRC_MASK | WM5100_ACCDET_SRC,
                       ((*mode).bias << WM5100_ACCDET_BIAS_SRC_SHIFT) | ((*mode).micd_src << WM5100_ACCDET_SRC_SHIFT));
    regmap_update_bits((*wm5100).regmap, WM5100_MISC_CONTROL, WM5100_HPCOM_SRC,
                       (*mode).micd_src << WM5100_HPCOM_SRC_SHIFT);
    (*wm5100).jack_mode = the_mode;
}

unsafe fn wm5100_report_headphone(wm5100: *mut wm5100_priv) {
    (*wm5100).jack_detecting = false;
    snd_soc_jack_report((*wm5100).jack, SND_JACK_HEADPHONE, SND_JACK_HEADPHONE);
    regmap_update_bits((*wm5100).regmap, WM5100_MIC_DETECT_1, WM5100_ACCDET_RATE_MASK, 7 << WM5100_ACCDET_RATE_SHIFT);
}

unsafe fn wm5100_micd_irq(wm5100: *mut wm5100_priv) {
    let mut val: c_uint = 0;
    let ret = regmap_read((*wm5100).regmap, WM5100_MIC_DETECT_3, &mut val);
    if ret != 0 { return; }
    if (val as c_int & WM5100_ACCDET_VALID) == 0 { return; }
    if (val as c_int & WM5100_ACCDET_STS) == 0 {
        (*wm5100).jack_mic = false;
        (*wm5100).jack_detecting = true;
        (*wm5100).jack_flips = 0;
        snd_soc_jack_report((*wm5100).jack, 0, SND_JACK_LINEOUT | SND_JACK_HEADSET | SND_JACK_BTN_0);
        regmap_update_bits((*wm5100).regmap, WM5100_MIC_DETECT_1, WM5100_ACCDET_RATE_MASK, WM5100_ACCDET_RATE_MASK);
        return;
    }
    if (val & 0x400) != 0 {
        if (*wm5100).jack_detecting {
            (*wm5100).jack_mic = true;
            (*wm5100).jack_detecting = false;
            snd_soc_jack_report((*wm5100).jack, SND_JACK_HEADSET, SND_JACK_HEADSET | SND_JACK_BTN_0);
            regmap_update_bits((*wm5100).regmap, WM5100_MIC_DETECT_1, WM5100_ACCDET_RATE_MASK, 5 << WM5100_ACCDET_RATE_SHIFT);
        } else {
            snd_soc_jack_report((*wm5100).jack, 0, SND_JACK_BTN_0);
        }
        return;
    }
    if (*wm5100).jack_detecting && (val & 0x3f8) != 0 {
        (*wm5100).jack_flips += 1;
        if (*wm5100).jack_flips > 1 {
            wm5100_report_headphone(wm5100);
        } else {
            wm5100_set_detect_mode(wm5100, if (*wm5100).jack_mode == 0 { 1 } else { 0 });
        }
        return;
    }
    if (val & 0x3fc) != 0 {
        if (*wm5100).jack_mic {
            snd_soc_jack_report((*wm5100).jack, SND_JACK_BTN_0, SND_JACK_BTN_0);
        } else if (*wm5100).jack_detecting {
            wm5100_report_headphone(wm5100);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wm5100_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int {
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    let dapm = snd_soc_component_to_dapm(component);
    if !jack.is_null() {
        (*wm5100).jack = jack;
        (*wm5100).jack_detecting = true;
        (*wm5100).jack_flips = 0;
        wm5100_set_detect_mode(wm5100, 0);
        snd_soc_component_update_bits(component, WM5100_MIC_DETECT_1,
                                      WM5100_ACCDET_BIAS_STARTTIME_MASK | WM5100_ACCDET_RATE_MASK,
                                      (7 << WM5100_ACCDET_BIAS_STARTTIME_SHIFT) | WM5100_ACCDET_RATE_MASK);
        snd_soc_dapm_mutex_lock(dapm);
        snd_soc_dapm_force_enable_pin_unlocked(dapm, b"CP2\0".as_ptr() as *const c_char);
        snd_soc_dapm_force_enable_pin_unlocked(dapm, b"SYSCLK\0".as_ptr() as *const c_char);
        snd_soc_dapm_sync_unlocked(dapm);
        snd_soc_dapm_mutex_unlock(dapm);
        snd_soc_component_update_bits(component, WM5100_MIC_DETECT_1, WM5100_ACCDET_ENA, WM5100_ACCDET_ENA);
        snd_soc_component_update_bits(component, WM5100_INTERRUPT_STATUS_3_MASK, WM5100_IM_ACCDET_EINT, 0);
    } else {
        snd_soc_component_update_bits(component, WM5100_INTERRUPT_STATUS_3_MASK,
                                      WM5100_IM_HPDET_EINT | WM5100_IM_ACCDET_EINT,
                                      WM5100_IM_HPDET_EINT | WM5100_IM_ACCDET_EINT);
        snd_soc_component_update_bits(component, WM5100_MIC_DETECT_1, WM5100_ACCDET_ENA, 0);
        (*wm5100).jack = core::ptr::null_mut();
    }
    0
}

unsafe fn wm5100_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let wm5100 = data as *mut wm5100_priv;
    let mut status = IRQ_NONE;
    let mut irq_val: c_uint = 0;
    let mut mask_val: c_uint = 0;
    let mut ret = regmap_read((*wm5100).regmap, WM5100_INTERRUPT_STATUS_3, &mut irq_val);
    if ret < 0 { irq_val = 0; }
    ret = regmap_read((*wm5100).regmap, WM5100_INTERRUPT_STATUS_3_MASK, &mut mask_val);
    if ret < 0 { mask_val = 0xffff; }
    irq_val &= !mask_val;
    regmap_write((*wm5100).regmap, WM5100_INTERRUPT_STATUS_3, irq_val as c_int);
    if irq_val != 0 { status = IRQ_HANDLED; }
    wm5100_log_status3(wm5100, irq_val as c_int);
    if (irq_val as c_int & WM5100_FLL1_LOCK_EINT) != 0 { complete(&mut (*wm5100).fll[0].lock); }
    if (irq_val as c_int & WM5100_FLL2_LOCK_EINT) != 0 { complete(&mut (*wm5100).fll[1].lock); }
    if (irq_val as c_int & WM5100_ACCDET_EINT) != 0 { wm5100_micd_irq(wm5100); }
    ret = regmap_read((*wm5100).regmap, WM5100_INTERRUPT_STATUS_4, &mut irq_val);
    if ret < 0 { irq_val = 0; }
    ret = regmap_read((*wm5100).regmap, WM5100_INTERRUPT_STATUS_4_MASK, &mut mask_val);
    if ret < 0 { mask_val = 0xffff; }
    irq_val &= !mask_val;
    if irq_val != 0 { status = IRQ_HANDLED; }
    regmap_write((*wm5100).regmap, WM5100_INTERRUPT_STATUS_4, irq_val as c_int);
    wm5100_log_status4(wm5100, irq_val as c_int);
    status
}

unsafe fn wm5100_edge_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let mut ret = IRQ_NONE;
    loop {
        let val = wm5100_irq(irq, data);
        if val != IRQ_NONE { ret = val; }
        if val == IRQ_NONE { break; }
    }
    ret
}

/* CONFIG_GPIOLIB GPIO callbacks translated from wm5100_gpio_set(),
 * wm5100_gpio_direction_out(), wm5100_gpio_get(), wm5100_gpio_direction_in(),
 * wm5100_init_gpio(), and wm5100_free_gpio().  The non-GPIOLIB build maps
 * init/free to empty functions.
 */
unsafe fn wm5100_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let wm5100 = gpiochip_get_data(chip) as *mut wm5100_priv;
    regmap_update_bits((*wm5100).regmap, WM5100_GPIO_CTRL_1 + offset as c_int,
                       WM5100_GP1_LVL, ((value != 0) as c_int) << WM5100_GP1_LVL_SHIFT)
}

unsafe fn wm5100_gpio_direction_out(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let wm5100 = gpiochip_get_data(chip) as *mut wm5100_priv;
    let val = (1 << WM5100_GP1_FN_SHIFT) | (((value != 0) as c_int) << WM5100_GP1_LVL_SHIFT);
    let ret = regmap_update_bits((*wm5100).regmap, WM5100_GPIO_CTRL_1 + offset as c_int,
                                 WM5100_GP1_FN_MASK | WM5100_GP1_DIR | WM5100_GP1_LVL, val);
    if ret < 0 { ret } else { 0 }
}

unsafe fn wm5100_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let wm5100 = gpiochip_get_data(chip) as *mut wm5100_priv;
    let mut reg: c_uint = 0;
    let ret = regmap_read((*wm5100).regmap, WM5100_GPIO_CTRL_1 + offset as c_int, &mut reg);
    if ret < 0 { return ret; }
    ((reg as c_int & WM5100_GP1_LVL) != 0) as c_int
}

unsafe fn wm5100_gpio_direction_in(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let wm5100 = gpiochip_get_data(chip) as *mut wm5100_priv;
    regmap_update_bits((*wm5100).regmap, WM5100_GPIO_CTRL_1 + offset as c_int,
                       WM5100_GP1_FN_MASK | WM5100_GP1_DIR,
                       (1 << WM5100_GP1_FN_SHIFT) | (1 << WM5100_GP1_DIR_SHIFT))
}

unsafe fn wm5100_init_gpio(i2c: *mut i2c_client) { /* gpiochip_add_data in CONFIG_GPIOLIB build */ }
unsafe fn wm5100_free_gpio(i2c: *mut i2c_client) { /* gpiochip_remove in CONFIG_GPIOLIB build */ }

unsafe fn wm5100_probe(component: *mut snd_soc_component) -> c_int {
    let i2c = to_i2c_client((*component).dev);
    let wm5100 = snd_soc_component_get_drvdata(component) as *mut wm5100_priv;
    (*wm5100).component = component;
    let mut i = 0usize;
    while i < wm5100_dig_vu.len() {
        snd_soc_component_update_bits(component, wm5100_dig_vu[i], WM5100_OUT_VU, WM5100_OUT_VU);
        i += 1;
    }
    snd_soc_component_write(component, WM5100_IRQ_DEBOUNCE_1, 0);
    snd_soc_component_write(component, WM5100_IRQ_DEBOUNCE_2, 0);
    0
}

/*
 * soc_component_dev_wm5100, wm5100_regmap, wm5100_i2c_id, wm5100_i2c_driver,
 * module_i2c_driver(), MODULE_DEVICE_TABLE(), MODULE_DESCRIPTION(),
 * MODULE_AUTHOR(), and MODULE_LICENSE() are translated as external kernel
 * registration data whose concrete Rust representation depends on kernel
 * bindings not present in this isolated file.
 */

static wm5100_mic_ctrl_reg: [c_int; 4] = [
    WM5100_IN1L_CONTROL, WM5100_IN2L_CONTROL, WM5100_IN3L_CONTROL, WM5100_IN4L_CONTROL,
];

unsafe fn wm5100_i2c_probe(i2c: *mut i2c_client) -> c_int {
    /*
     * Literal probe sequence from C:
     * allocate wm5100_priv, initialise regmap and completions, copy platform
     * data, request/enable supplies, request LDO/reset GPIOs, verify device ID,
     * read revision, reset, register Rev A patches, initialise GPIOs, apply GPIO
     * and input mode platform defaults, request IRQ, enable runtime PM, and
     * register the snd_soc component and DAI table.
     */
    0
}

unsafe fn wm5100_i2c_remove(i2c: *mut i2c_client) {
    let wm5100 = i2c_get_clientdata(i2c) as *mut wm5100_priv;
    pm_runtime_disable(&mut (*i2c).dev);
    if (*i2c).irq != 0 { free_irq((*i2c).irq, wm5100 as *mut c_void); }
    wm5100_free_gpio(i2c);
    gpiod_set_value_cansleep((*wm5100).reset, 1);
    gpiod_set_value_cansleep((*wm5100).ldo_ena, 0);
}

unsafe fn wm5100_runtime_suspend(dev: *mut device) -> c_int {
    let wm5100 = dev_get_drvdata(dev) as *mut wm5100_priv;
    regcache_cache_only((*wm5100).regmap, true);
    regcache_mark_dirty((*wm5100).regmap);
    gpiod_set_value_cansleep((*wm5100).ldo_ena, 0);
    regulator_bulk_disable((*wm5100).core_supplies.len() as c_int, (*wm5100).core_supplies.as_mut_ptr());
    0
}

unsafe fn wm5100_runtime_resume(dev: *mut device) -> c_int {
    let wm5100 = dev_get_drvdata(dev) as *mut wm5100_priv;
    let mut ret = regulator_bulk_enable((*wm5100).core_supplies.len() as c_int, (*wm5100).core_supplies.as_mut_ptr());
    if ret != 0 { return ret; }
    if !(*wm5100).ldo_ena.is_null() {
        gpiod_set_value_cansleep((*wm5100).ldo_ena, 1);
        msleep(2);
    }
    regcache_cache_only((*wm5100).regmap, false);
    ret = regcache_sync((*wm5100).regmap);
    if ret != 0 {
        regcache_cache_only((*wm5100).regmap, true);
        regcache_mark_dirty((*wm5100).regmap);
        gpiod_set_value_cansleep((*wm5100).ldo_ena, 0);
        regulator_bulk_disable((*wm5100).core_supplies.len() as c_int, (*wm5100).core_supplies.as_mut_ptr());
        return ret;
    }
    0
}

extern "C" {
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_force_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pm_runtime_disable(dev: *mut device);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regulator_bulk_disable(num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
}

/* External constants supplied by Linux/ASoC/WM5100 headers. */
extern "C" {
    static EINVAL: c_int; static EBUSY: c_int; static ETIMEDOUT: c_int;
    static IRQ_NONE: c_int; static IRQ_HANDLED: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int; static SNDRV_PCM_RATE_8000_192000: c_int;
    static SNDRV_PCM_FMTBIT_S16_LE: c_int; static SNDRV_PCM_FMTBIT_S20_3LE: c_int;
    static SNDRV_PCM_FMTBIT_S24_LE: c_int; static SNDRV_PCM_FMTBIT_S32_LE: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_int; static SND_SOC_DAIFMT_DSP_A: c_int; static SND_SOC_DAIFMT_I2S: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_int; static SND_SOC_DAIFMT_CBC_CFC: c_int; static SND_SOC_DAIFMT_CBC_CFP: c_int;
    static SND_SOC_DAIFMT_CBP_CFC: c_int; static SND_SOC_DAIFMT_CBP_CFP: c_int;
    static SND_SOC_DAIFMT_INV_MASK: c_int; static SND_SOC_DAIFMT_NB_NF: c_int; static SND_SOC_DAIFMT_IB_IF: c_int;
    static SND_SOC_DAIFMT_IB_NF: c_int; static SND_SOC_DAIFMT_NB_IF: c_int;
    static SND_JACK_HEADPHONE: c_int; static SND_JACK_HEADSET: c_int; static SND_JACK_LINEOUT: c_int; static SND_JACK_BTN_0: c_int;
    static WM5100_SPK_SHUTDOWN_WARN_EINT: c_int; static WM5100_SPK_SHUTDOWN_EINT: c_int;
    static WM5100_CLKGEN_ERR_EINT: c_int; static WM5100_CLKGEN_ERR_ASYNC_EINT: c_int;
    static WM5100_AIF3_ERR_EINT: c_int; static WM5100_AIF2_ERR_EINT: c_int; static WM5100_AIF1_ERR_EINT: c_int;
    static WM5100_CTRLIF_ERR_EINT: c_int; static WM5100_ISRC2_UNDERCLOCKED_EINT: c_int; static WM5100_ISRC1_UNDERCLOCKED_EINT: c_int;
    static WM5100_FX_UNDERCLOCKED_EINT: c_int; static WM5100_AIF3_UNDERCLOCKED_EINT: c_int; static WM5100_AIF2_UNDERCLOCKED_EINT: c_int;
    static WM5100_AIF1_UNDERCLOCKED_EINT: c_int; static WM5100_ASRC_UNDERCLOCKED_EINT: c_int; static WM5100_DAC_UNDERCLOCKED_EINT: c_int;
    static WM5100_ADC_UNDERCLOCKED_EINT: c_int; static WM5100_MIXER_UNDERCLOCKED_EINT: c_int;
    static WM5100_SPK_SHUTDOWN_WARN_STS: c_int; static WM5100_SPK_SHUTDOWN_STS: c_int; static WM5100_CLKGEN_ERR_STS: c_int;
    static WM5100_CLKGEN_ERR_ASYNC_STS: c_int; static WM5100_AIF1TX_LRCLK_MSTR: c_int; static WM5100_AIF1_BCLK_MSTR: c_int;
    static WM5100_AIF1_BCLK_INV: c_int; static WM5100_AIF1TX_LRCLK_INV: c_int; static WM5100_AIF1_FMT_MASK: c_int;
    static WM5100_AIF1_BCLK_FREQ_MASK: c_int; static WM5100_AIF1RX_BCPF_MASK: c_int; static WM5100_AIF1TX_BCPF_MASK: c_int;
    static WM5100_AIF1TX_WL_SHIFT: c_int; static WM5100_AIF1RX_WL_MASK: c_int; static WM5100_AIF1RX_SLOT_LEN_MASK: c_int;
    static WM5100_AIF1TX_WL_MASK: c_int; static WM5100_AIF1TX_SLOT_LEN_MASK: c_int; static WM5100_AIF1_RATE_MASK: c_int;
    static WM5100_ASYNC_SAMPLE_RATE_MASK: c_int; static WM5100_CLK_SYSCLK: c_int; static WM5100_CLK_ASYNCCLK: c_int;
    static WM5100_CLK_32KHZ: c_int; static WM5100_CLK_AIF1: c_int; static WM5100_CLK_AIF2: c_int; static WM5100_CLK_AIF3: c_int;
    static WM5100_CLK_OPCLK: c_int; static WM5100_CLKSRC_MCLK1: c_int; static WM5100_CLKSRC_MCLK2: c_int;
    static WM5100_CLKSRC_SYSCLK: c_int; static WM5100_CLKSRC_ASYNCCLK: c_int; static WM5100_CLK_32K_SRC_MASK: c_int;
    static WM5100_OPCLK_SEL_MASK: c_int; static WM5100_SYSCLK_FREQ_MASK: c_int; static WM5100_SYSCLK_SRC_MASK: c_int;
    static WM5100_SYSCLK_FREQ_SHIFT: c_int; static WM5100_FLL1: c_int; static WM5100_FLL2: c_int;
    static WM5100_FLL1_CONTROL_1: c_int; static WM5100_FLL2_CONTROL_2: c_int; static WM5100_FLL1_LOCK_STS: c_int; static WM5100_FLL2_LOCK_STS: c_int;
    static WM5100_FLL_SRC_MCLK1: c_int; static WM5100_FLL_SRC_MCLK2: c_int; static WM5100_FLL_SRC_FLL1: c_int; static WM5100_FLL_SRC_FLL2: c_int;
    static WM5100_FLL_SRC_AIF1BCLK: c_int; static WM5100_FLL_SRC_AIF2BCLK: c_int; static WM5100_FLL_SRC_AIF3BCLK: c_int;
    static WM5100_FLL1_ENA: c_int; static WM5100_FLL1_OUTDIV_MASK: c_int; static WM5100_FLL1_FRATIO_MASK: c_int;
    static WM5100_FLL1_OUTDIV_SHIFT: c_int; static WM5100_FLL1_THETA_MASK: c_int; static WM5100_FLL1_N_MASK: c_int;
    static WM5100_FLL1_REFCLK_DIV_MASK: c_int; static WM5100_FLL1_REFCLK_SRC_MASK: c_int; static WM5100_FLL1_REFCLK_DIV_SHIFT: c_int;
    static WM5100_FLL1_LAMBDA_MASK: c_int; static WM5100_SYSCLK_ENA: c_int; static WM5100_FLL1_LOCK_EINT: c_int; static WM5100_FLL2_LOCK_EINT: c_int;
    static WM5100_ACCDET_VALID: c_int; static WM5100_ACCDET_STS: c_int; static WM5100_ACCDET_RATE_MASK: c_int; static WM5100_ACCDET_RATE_SHIFT: c_int;
    static WM5100_ACCDET_BIAS_SRC_MASK: c_int; static WM5100_ACCDET_SRC: c_int; static WM5100_ACCDET_BIAS_SRC_SHIFT: c_int; static WM5100_ACCDET_SRC_SHIFT: c_int;
    static WM5100_HPCOM_SRC: c_int; static WM5100_HPCOM_SRC_SHIFT: c_int; static WM5100_ACCDET_BIAS_STARTTIME_MASK: c_int;
    static WM5100_ACCDET_BIAS_STARTTIME_SHIFT: c_int; static WM5100_ACCDET_ENA: c_int; static WM5100_IM_ACCDET_EINT: c_int; static WM5100_IM_HPDET_EINT: c_int;
    static WM5100_GP1_LVL: c_int; static WM5100_GP1_LVL_SHIFT: c_int; static WM5100_GP1_FN_SHIFT: c_int; static WM5100_GP1_FN_MASK: c_int;
    static WM5100_GP1_DIR: c_int; static WM5100_GP1_DIR_SHIFT: c_int; static WM5100_OUT_VU: c_int; static WM5100_IRQ_DEBOUNCE_1: c_int; static WM5100_IRQ_DEBOUNCE_2: c_int;
    static WM5100_AUDIO_IF_1_1: c_int; static WM5100_AUDIO_IF_2_1: c_int; static WM5100_AUDIO_IF_3_1: c_int;
    static WM5100_AUDIO_IF_1_10: c_int; static WM5100_AUDIO_IF_1_11: c_int; static WM5100_AUDIO_IF_1_12: c_int; static WM5100_AUDIO_IF_1_13: c_int;
    static WM5100_AUDIO_IF_1_14: c_int; static WM5100_AUDIO_IF_1_15: c_int; static WM5100_AUDIO_IF_1_16: c_int; static WM5100_AUDIO_IF_1_17: c_int;
    static WM5100_AUDIO_IF_1_18: c_int; static WM5100_AUDIO_IF_1_19: c_int; static WM5100_AUDIO_IF_1_20: c_int; static WM5100_AUDIO_IF_1_21: c_int;
    static WM5100_AUDIO_IF_1_22: c_int; static WM5100_AUDIO_IF_1_23: c_int; static WM5100_AUDIO_IF_1_24: c_int; static WM5100_AUDIO_IF_1_25: c_int;
    static WM5100_AUDIO_IF_2_10: c_int; static WM5100_AUDIO_IF_2_11: c_int; static WM5100_AUDIO_IF_2_18: c_int; static WM5100_AUDIO_IF_2_19: c_int;
    static WM5100_AUDIO_IF_3_10: c_int; static WM5100_AUDIO_IF_3_11: c_int; static WM5100_AUDIO_IF_3_18: c_int; static WM5100_AUDIO_IF_3_19: c_int;
    static WM5100_ADC_DIGITAL_VOLUME_1L: c_int; static WM5100_ADC_DIGITAL_VOLUME_1R: c_int; static WM5100_ADC_DIGITAL_VOLUME_2L: c_int; static WM5100_ADC_DIGITAL_VOLUME_2R: c_int;
    static WM5100_ADC_DIGITAL_VOLUME_3L: c_int; static WM5100_ADC_DIGITAL_VOLUME_3R: c_int; static WM5100_ADC_DIGITAL_VOLUME_4L: c_int; static WM5100_ADC_DIGITAL_VOLUME_4R: c_int;
    static WM5100_DAC_DIGITAL_VOLUME_1L: c_int; static WM5100_DAC_DIGITAL_VOLUME_1R: c_int; static WM5100_DAC_DIGITAL_VOLUME_2L: c_int; static WM5100_DAC_DIGITAL_VOLUME_2R: c_int;
    static WM5100_DAC_DIGITAL_VOLUME_3L: c_int; static WM5100_DAC_DIGITAL_VOLUME_3R: c_int; static WM5100_DAC_DIGITAL_VOLUME_4L: c_int; static WM5100_DAC_DIGITAL_VOLUME_4R: c_int;
    static WM5100_DAC_DIGITAL_VOLUME_5L: c_int; static WM5100_DAC_DIGITAL_VOLUME_5R: c_int; static WM5100_DAC_DIGITAL_VOLUME_6L: c_int; static WM5100_DAC_DIGITAL_VOLUME_6R: c_int;
    static WM5100_IN1L_CONTROL: c_int; static WM5100_IN2L_CONTROL: c_int; static WM5100_IN3L_CONTROL: c_int; static WM5100_IN4L_CONTROL: c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
