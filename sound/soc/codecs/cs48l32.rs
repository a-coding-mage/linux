// SPDX-License-Identifier: GPL-2.0-only
//
// Cirrus Logic CS48L32 audio DSP.
//
// Copyright (C) 2016-2018, 2020, 2022, 2025 Cirrus Logic, Inc. and
//               Cirrus Logic International Semiconductor Ltd.
//
// Source-level Rust translation of soc/codecs/cs48l32.c.
// External Linux/ASoC/CS48L32 types, constants, and helper macros are expected
// to be supplied by the surrounding translated repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type s16 = i16;
type bool_ = bool;
type __be16 = u16;
type __be32 = u32;
type irqreturn_t = c_uint;

extern "C" {
    static mut cs48l32_mixer_texts: [*const c_char; 0];
}

macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        ($x).len()
    };
}
macro_rules! BIT {
    ($x:expr) => {
        (1u32 << ($x))
    };
}

// Dependencies originally included from Linux, ASoC, Cirrus register, and local
// cs48l32 headers are intentionally not reimplemented in this isolated file.

extern "C" {
    type device;
    type regmap;
    type regulator;
    type clk;
    type gpio_desc;
    type spi_device;
    type snd_kcontrol;
    type snd_ctl_elem_value;
    type snd_ctl_elem_info;
    type snd_soc_component;
    type snd_soc_dapm_context;
    type snd_soc_dapm_widget;
    type snd_soc_dai;
    type snd_soc_dai_driver;
    type snd_pcm_substream;
    type snd_pcm_hw_params;
    type snd_pcm_hw_constraint_list;
    type snd_compr_stream;
    type snd_compress_ops;
    type snd_soc_pcm_runtime;
    type snd_soc_component_driver;
    type snd_soc_dapm_route;
    type snd_soc_dapm_widget;
    type snd_soc_dai_ops;
    type snd_kcontrol_new;
    type soc_enum;
    type soc_mixer_control;
    type wm_adsp;
    type cs_dsp_region;
    type cs48l32;
    type cs48l32_codec;
    type cs48l32_fll;
    type cs48l32_dai_priv;
    type cs48l32_eq_control;
    type cs48l32_dsp_power_reg_block;
    type cs48l32_dsp_power_regs;
    type of_device_id;
    type spi_device_id;
    type dev_pm_ops;
}

extern "C" {
    fn pm_runtime_suspended(dev: *mut device) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_async(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_multi_reg_read(map: *mut regmap, regs: *const c_uint, vals: *mut u32, n: usize) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, len: usize) -> c_int;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, len: usize) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn udelay(usecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn fsleep(usecs: c_uint);
    fn gcd(a: c_uint, b: c_uint) -> c_uint;
    fn abs(i: c_int) -> c_int;
    fn ffs(i: c_int) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut cs48l32_codec;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, mux: c_uint, e: *mut soc_enum, update: *mut c_void);
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_bytes_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_enum_val_to_item(e: *mut soc_enum, val: c_uint) -> c_int;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_tdm_params_to_bclk(params: *mut snd_pcm_hw_params, slotw: c_uint, slots: c_uint, multiple: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_format_width(format: c_uint) -> c_uint;
    fn snd_pcm_hw_constraint_list(runtime: *mut c_void, cond: c_uint, param: c_uint, list: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, map: *mut regmap);
    fn wm_adsp2_component_probe(dsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_component_remove(dsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_remove(dsp: *mut wm_adsp);
    fn wm_halo_init(dsp: *mut wm_adsp) -> c_int;
    fn wm_halo_bus_error(irq: c_int, dsp: *mut wm_adsp) -> irqreturn_t;
    fn wm_halo_wdt_expire(irq: c_int, dsp: *mut wm_adsp) -> irqreturn_t;
    fn wm_adsp_compr_handle_irq(dsp: *mut wm_adsp);
    fn wm_adsp_compr_open(dsp: *mut wm_adsp, stream: *mut snd_compr_stream) -> c_int;
    fn wm_adsp_compr_free(stream: *mut snd_compr_stream) -> c_int;
    fn wm_adsp_compr_set_params(stream: *mut snd_compr_stream, params: *mut c_void) -> c_int;
    fn wm_adsp_compr_get_caps(stream: *mut snd_compr_stream, caps: *mut c_void) -> c_int;
    fn wm_adsp_compr_trigger(stream: *mut snd_compr_stream, cmd: c_int) -> c_int;
    fn wm_adsp_compr_pointer(stream: *mut snd_compr_stream, tstamp: *mut c_void) -> c_int;
    fn wm_adsp_compr_copy(stream: *mut snd_compr_stream, buf: *mut c_char, count: usize) -> c_int;
}

const CS48L32_48K_RATE_MASK: c_uint = 0x0e00fe;
const CS48L32_44K1_RATE_MASK: c_uint = 0x00fe00;
const CS48L32_RATE_MASK: c_uint = CS48L32_48K_RATE_MASK | CS48L32_44K1_RATE_MASK;

static cs48l32_core_supplies: [&[u8]; 2] = [b"vdd-a\0", b"vdd-io\0"];

static cs48l32_mixer_values: [c_uint; 58] = [
    0x000, 0x004, 0x005, 0x00C, 0x010, 0x011, 0x012, 0x013,
    0x020, 0x021, 0x022, 0x023, 0x024, 0x025, 0x026, 0x027,
    0x030, 0x031, 0x032, 0x033, 0x098, 0x099, 0x09a, 0x09b,
    0x09C, 0x09D, 0x09e, 0x09f, 0x0A0, 0x0A1, 0x0A4, 0x0A5,
    0x0A8, 0x0A9, 0x0AC, 0x0AD, 0x0B8, 0x0B9, 0x0BA, 0x0BB,
    0x0C0, 0x0C1, 0x0C2, 0x0C3, 0x0C8, 0x0C9, 0x0CA, 0x0CB,
    0x0D8, 0x0D9, 0x100, 0x101, 0x102, 0x103, 0x104, 0x105,
    0x106, 0x107,
];
const CS48L32_NUM_MIXER_INPUTS: usize = cs48l32_mixer_values.len();

static cs48l32_rate_text: [&[u8]; 4] = [
    b"Sample Rate 1\0", b"Sample Rate 2\0", b"Sample Rate 3\0", b"Sample Rate 4\0",
];
static cs48l32_rate_val: [c_uint; 4] = [0x0, 0x1, 0x2, 0x3];

static cs48l32_sample_rate_text: [&[u8]; 17] = [
    b"12kHz\0", b"24kHz\0", b"48kHz\0", b"96kHz\0", b"192kHz\0", b"384kHz\0",
    b"768kHz\0", b"11.025kHz\0", b"22.05kHz\0", b"44.1kHz\0", b"88.2kHz\0",
    b"176.4kHz\0", b"352.8kHz\0", b"705.6kHz\0", b"8kHz\0", b"16kHz\0", b"32kHz\0",
];
static cs48l32_sample_rate_val: [c_uint; 17] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
    0x0e, 0x0f, 0x11, 0x12, 0x13,
];
const CS48L32_SAMPLE_RATE_ENUM_SIZE: usize = cs48l32_sample_rate_val.len();

static cs48l32_inmux_texts: [&[u8]; 2] = [b"Analog 1\0", b"Analog 2\0"];
static cs48l32_dmode_texts: [&[u8]; 2] = [b"Analog\0", b"Digital\0"];
static cs48l32_in_texts: [&[u8]; 4] = [b"IN1L\0", b"IN1R\0", b"IN2L\0", b"IN2R\0"];
static cs48l32_us_freq_texts: [&[u8]; 2] = [b"16-24kHz\0", b"20-28kHz\0"];
static cs48l32_us_freq_val: [c_uint; 2] = [0x2, 0x3];
static cs48l32_us_in_val: [c_uint; 4] = [0x0, 0x1, 0x2, 0x3];
static cs48l32_us_det_thr_texts: [&[u8]; 8] = [
    b"-6dB\0", b"-9dB\0", b"-12dB\0", b"-15dB\0", b"-18dB\0", b"-21dB\0", b"-24dB\0", b"-27dB\0",
];
static cs48l32_us_det_num_texts: [&[u8]; 16] = [
    b"1 Sample\0", b"2 Samples\0", b"4 Samples\0", b"8 Samples\0", b"16 Samples\0",
    b"32 Samples\0", b"64 Samples\0", b"128 Samples\0", b"256 Samples\0", b"512 Samples\0",
    b"1024 Samples\0", b"2048 Samples\0", b"4096 Samples\0", b"8192 Samples\0",
    b"16384 Samples\0", b"32768 Samples\0",
];
static cs48l32_us_det_hold_texts: [&[u8]; 16] = [
    b"0 Samples\0", b"31 Samples\0", b"63 Samples\0", b"127 Samples\0", b"255 Samples\0",
    b"511 Samples\0", b"1023 Samples\0", b"2047 Samples\0", b"4095 Samples\0",
    b"8191 Samples\0", b"16383 Samples\0", b"32767 Samples\0", b"65535 Samples\0",
    b"131071 Samples\0", b"262143 Samples\0", b"524287 Samples\0",
];
static cs48l32_us_det_lpf_cut_texts: [&[u8]; 4] = [b"1722Hz\0", b"833Hz\0", b"408Hz\0", b"203Hz\0"];
static cs48l32_us_det_dcy_texts: [&[u8]; 8] = [
    b"0 ms\0", b"0.79 ms\0", b"1.58 ms\0", b"3.16 ms\0", b"6.33 ms\0", b"12.67 ms\0",
    b"25.34 ms\0", b"50.69 ms\0",
];
static cs48l32_vol_ramp_text: [&[u8]; 8] = [
    b"0ms/6dB\0", b"0.5ms/6dB\0", b"1ms/6dB\0", b"2ms/6dB\0", b"4ms/6dB\0",
    b"8ms/6dB\0", b"16ms/6dB\0", b"32ms/6dB\0",
];
static cs48l32_in_hpf_cut_text: [&[u8]; 5] = [b"2.5Hz\0", b"5Hz\0", b"10Hz\0", b"20Hz\0", b"40Hz\0"];
static cs48l32_in_dmic_osr_text: [&[u8]; 7] = [
    b"384kHz\0", b"768kHz\0", b"1.536MHz\0", b"2.048MHz\0", b"2.4576MHz\0",
    b"3.072MHz\0", b"6.144MHz\0",
];
static cs48l32_auxpdm_freq_texts: [&[u8]; 4] = [b"3.072MHz\0", b"2.048MHz\0", b"1.536MHz\0", b"768kHz\0"];
static cs48l32_auxpdm_src_texts: [&[u8]; 3] = [b"Analog\0", b"IN1 Digital\0", b"IN2 Digital\0"];
static cs48l32_auxpdm_analog_in_val: [c_uint; 2] = [0x0, 0x1];
static cs48l32_lhpf_mode_text: [&[u8]; 2] = [b"Low-pass\0", b"High-pass\0"];
static cs48l32_eq_mode_text: [&[u8]; 2] = [b"Low-pass\0", b"High-pass\0"];

#[repr(C)]
struct cs48l32_sclk_rate {
    freq: u32,
    id: u32,
}

static cs48l32_sclk_rates: [cs48l32_sclk_rate; 23] = [
    cs48l32_sclk_rate { freq: 128000, id: 12 },
    cs48l32_sclk_rate { freq: 176400, id: 13 },
    cs48l32_sclk_rate { freq: 192000, id: 14 },
    cs48l32_sclk_rate { freq: 256000, id: 15 },
    cs48l32_sclk_rate { freq: 352800, id: 16 },
    cs48l32_sclk_rate { freq: 384000, id: 17 },
    cs48l32_sclk_rate { freq: 512000, id: 18 },
    cs48l32_sclk_rate { freq: 705600, id: 19 },
    cs48l32_sclk_rate { freq: 768000, id: 21 },
    cs48l32_sclk_rate { freq: 1024000, id: 23 },
    cs48l32_sclk_rate { freq: 1411200, id: 25 },
    cs48l32_sclk_rate { freq: 1536000, id: 27 },
    cs48l32_sclk_rate { freq: 2048000, id: 29 },
    cs48l32_sclk_rate { freq: 2822400, id: 31 },
    cs48l32_sclk_rate { freq: 3072000, id: 33 },
    cs48l32_sclk_rate { freq: 4096000, id: 36 },
    cs48l32_sclk_rate { freq: 5644800, id: 38 },
    cs48l32_sclk_rate { freq: 6144000, id: 40 },
    cs48l32_sclk_rate { freq: 8192000, id: 47 },
    cs48l32_sclk_rate { freq: 11289600, id: 49 },
    cs48l32_sclk_rate { freq: 12288000, id: 51 },
    cs48l32_sclk_rate { freq: 22579200, id: 57 },
    cs48l32_sclk_rate { freq: 24576000, id: 59 },
];

static cs48l32_sr_vals: [c_uint; 20] = [
    0, 12000, 24000, 48000, 96000, 192000, 384000, 768000, 0, 11025,
    22050, 44100, 88200, 176400, 352800, 705600, 0, 8000, 16000, 32000,
];

unsafe fn cs48l32_spin_sysclk(cs48l32_codec: *mut cs48l32_codec) {
    let cs48l32 = &mut (*cs48l32_codec).core as *mut cs48l32;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    if pm_runtime_suspended((*cs48l32).dev) != 0 {
        return;
    }

    for i in 0..4 {
        ret = regmap_read((*cs48l32).regmap, CS48L32_DEVID, &mut val);
        if ret != 0 {
            dev_err((*cs48l32_codec).core.dev, b"%s Failed to read register: %d (%d)\n\0".as_ptr() as *const c_char, b"cs48l32_spin_sysclk\0".as_ptr(), ret, i);
        }
    }

    udelay(300);
}

unsafe fn cs48l32_rate_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs48l32_codec = snd_soc_component_get_drvdata(component);
    cs48l32_spin_sysclk(cs48l32_codec);
    let ret = snd_soc_put_enum_double(kcontrol, ucontrol);
    cs48l32_spin_sysclk(cs48l32_codec);
    ret
}

unsafe fn cs48l32_is_input_enabled(component: *mut snd_soc_component, reg: c_uint) -> bool {
    let input_active = snd_soc_component_read(component, CS48L32_INPUT_CONTROL);
    match reg {
        CS48L32_IN1L_CONTROL1 => (input_active & BIT!(CS48L32_IN1L_EN_SHIFT)) != 0,
        CS48L32_IN1R_CONTROL1 => (input_active & BIT!(CS48L32_IN1R_EN_SHIFT)) != 0,
        CS48L32_IN2L_CONTROL1 => (input_active & BIT!(CS48L32_IN2L_EN_SHIFT)) != 0,
        CS48L32_IN2R_CONTROL1 => (input_active & BIT!(CS48L32_IN2R_EN_SHIFT)) != 0,
        _ => false,
    }
}

unsafe fn cs48l32_get_dspclk_setting(_cs48l32_codec: *mut cs48l32_codec, mut freq: c_uint, _src: c_int, val: *mut c_uint) -> c_int {
    freq /= 15625;
    *val |= freq << CS48L32_DSP_CLK_FREQ_SHIFT;
    0
}

unsafe fn cs48l32_get_sysclk_setting(freq: c_uint) -> c_int {
    match freq {
        0 | 5644800 | 6144000 => CS48L32_SYSCLK_RATE_6MHZ as c_int,
        11289600 | 12288000 => (CS48L32_SYSCLK_RATE_12MHZ << CS48L32_SYSCLK_FREQ_SHIFT) as c_int,
        22579200 | 24576000 => (CS48L32_SYSCLK_RATE_24MHZ << CS48L32_SYSCLK_FREQ_SHIFT) as c_int,
        45158400 | 49152000 => (CS48L32_SYSCLK_RATE_49MHZ << CS48L32_SYSCLK_FREQ_SHIFT) as c_int,
        90316800 | 98304000 => (CS48L32_SYSCLK_RATE_98MHZ << CS48L32_SYSCLK_FREQ_SHIFT) as c_int,
        _ => -EINVAL,
    }
}

unsafe fn cs48l32_wait_for_fll(fll: *mut cs48l32_fll, requested: bool) -> c_int {
    let regmap = (*(*fll).codec).core.regmap;
    let mut val: c_uint = 0;

    cs48l32_fll_dbg(fll, b"Waiting for FLL...\n\0".as_ptr() as *const c_char);
    for i in 0..30 {
        regmap_read(regmap, (*fll).sts_addr, &mut val);
        if (((val & (*fll).sts_mask) != 0) == requested) {
            return 0;
        }
        match i {
            0..=5 => usleep_range(75, 125),
            6..=20 => usleep_range(750, 1250),
            _ => fsleep(20000),
        }
    }
    cs48l32_fll_warn(fll, b"Timed out waiting for %s\n\0".as_ptr() as *const c_char, if requested { b"lock\0".as_ptr() } else { b"unlock\0".as_ptr() });
    -ETIMEDOUT
}

unsafe fn cs48l32_fllhj_apply(fll: *mut cs48l32_fll, fin: c_int) -> c_int {
    let regmap = (*(*fll).codec).core.regmap;
    let mut frac = false;
    let mut refdiv: c_int = 0;
    while refdiv < 4 {
        if (fin / (1 << refdiv)) <= CS48L32_FLLHJ_MAX_THRESH as c_int {
            break;
        }
        refdiv += 1;
    }

    let fref = fin / (1 << refdiv);
    let fout = (*fll).fout as c_int;
    frac = (fout % fref) != 0;

    let (lockdet_thr, gains, mut fbdiv): (c_int, c_uint, c_uint) = if fref < CS48L32_FLLHJ_LOW_THRESH as c_int {
        (2, CS48L32_FLLHJ_LOW_GAINS, if frac { 256 } else { 4 })
    } else if fref < CS48L32_FLLHJ_MID_THRESH as c_int {
        (8, CS48L32_FLLHJ_MID_GAINS, if frac { 16 } else { 2 })
    } else {
        (8, CS48L32_FLLHJ_HIGH_GAINS, 1)
    };

    let (hp, min_n, max_n): (c_uint, c_uint, c_uint) = if frac {
        (3, CS48L32_FLLHJ_FRAC_MIN_N, CS48L32_FLLHJ_FRAC_MAX_N)
    } else {
        (if fref < CS48L32_FLLHJ_LP_INT_MODE_THRESH as c_int { 0 } else { 1 },
         CS48L32_FLLHJ_INT_MIN_N, CS48L32_FLLHJ_INT_MAX_N)
    };

    let ratio = (fout / fref) as c_uint;
    while ratio / fbdiv < min_n {
        fbdiv /= 2;
        if fbdiv < min_n {
            cs48l32_fll_err(fll, b"FBDIV (%u) < minimum N (%u)\n\0".as_ptr() as *const c_char, fbdiv, min_n);
            return -EINVAL;
        }
    }
    while frac && ratio / fbdiv > max_n {
        fbdiv *= 2;
        if fbdiv >= 1024 {
            cs48l32_fll_err(fll, b"FBDIV (%u) >= 1024\n\0".as_ptr() as *const c_char, fbdiv);
            return -EINVAL;
        }
    }

    let fllgcd = gcd(fout as c_uint, fbdiv * fref as c_uint);
    let num = fout as c_uint / fllgcd;
    let lambda = (fref as c_uint * fbdiv) / fllgcd;
    let fll_n = num / lambda;
    let theta = num % lambda;

    if fll_n < min_n || fll_n > max_n {
        cs48l32_fll_err(fll, b"N not in valid %s mode range %d-%d: %d\n\0".as_ptr() as *const c_char, if frac { b"fractional\0".as_ptr() } else { b"integer\0".as_ptr() }, min_n, max_n, fll_n);
        return -EINVAL;
    }
    if fbdiv < 1 || (frac && fbdiv >= 1024) || (!frac && fbdiv >= 256) {
        cs48l32_fll_err(fll, b"Invalid fbdiv for %s mode (%u)\n\0".as_ptr() as *const c_char, if frac { b"fractional\0".as_ptr() } else { b"integer\0".as_ptr() }, fbdiv);
        return -EINVAL;
    }

    regmap_update_bits(regmap, (*fll).base + CS48L32_FLL_CONTROL2_OFFS,
        CS48L32_FLL_LOCKDET_THR_MASK | CS48L32_FLL_PHASEDET_MASK | CS48L32_FLL_REFCLK_DIV_MASK | CS48L32_FLL_N_MASK | CS48L32_FLL_CTRL_UPD_MASK,
        ((lockdet_thr as c_uint) << CS48L32_FLL_LOCKDET_THR_SHIFT) | (1 << CS48L32_FLL_PHASEDET_SHIFT) | ((refdiv as c_uint) << CS48L32_FLL_REFCLK_DIV_SHIFT) | (fll_n << CS48L32_FLL_N_SHIFT));
    regmap_update_bits(regmap, (*fll).base + CS48L32_FLL_CONTROL3_OFFS,
        CS48L32_FLL_LAMBDA_MASK | CS48L32_FLL_THETA_MASK,
        (lambda << CS48L32_FLL_LAMBDA_SHIFT) | (theta << CS48L32_FLL_THETA_SHIFT));
    regmap_update_bits(regmap, (*fll).base + CS48L32_FLL_CONTROL4_OFFS,
        (0xffff << CS48L32_FLL_FD_GAIN_COARSE_SHIFT) | CS48L32_FLL_HP_MASK | CS48L32_FLL_FB_DIV_MASK,
        (gains << CS48L32_FLL_FD_GAIN_COARSE_SHIFT) | (hp << CS48L32_FLL_HP_SHIFT) | (fbdiv << CS48L32_FLL_FB_DIV_SHIFT));
    0
}

unsafe fn cs48l32_eq_filter_unstable(mode: bool, in_a: __be16, in_b: __be16) -> bool {
    let a = be16_to_cpu(in_a) as s16 as c_int;
    let b = be16_to_cpu(in_b) as s16 as c_int;
    if !mode {
        return abs(a) > CS48L32_EQ_MAX_COEFF as c_int;
    }
    if abs(b) > CS48L32_EQ_MAX_COEFF as c_int {
        return true;
    }
    if abs((a << 16) / (CS48L32_EQ_MAX_COEFF as c_int + 1 - b)) >= ((CS48L32_EQ_MAX_COEFF as c_int + 1) << 4) {
        return true;
    }
    false
}

unsafe fn cs48l32_dai_clk_str(clk_id: c_int) -> *const c_char {
    match clk_id {
        CS48L32_CLK_SYSCLK_1 | CS48L32_CLK_SYSCLK_2 | CS48L32_CLK_SYSCLK_3 | CS48L32_CLK_SYSCLK_4 => b"SYSCLK\0".as_ptr() as *const c_char,
        _ => b"Unknown clock\0".as_ptr() as *const c_char,
    }
}

unsafe fn cs48l32_set_channels_to_mask(dai: *mut snd_soc_dai, base: c_uint, channels: c_int, mut mask: c_uint) {
    let component = (*dai).component;
    let cs48l32_codec = snd_soc_component_get_drvdata(component);
    let regmap = (*cs48l32_codec).core.regmap;
    let mut frame_ctls = [0u32, 0u32];
    let mut j = 0usize;

    for i in 0..channels {
        let slot = ffs(mask as c_int) - 1;
        if slot < 0 {
            return;
        }
        if i - (j as c_int * 4) >= 4 {
            j += 1;
            if j >= 2 {
                break;
            }
        }
        let shift = 8 * (i - j as c_int * 4);
        frame_ctls[j] |= (slot as c_uint) << shift;
        mask &= !(1u32 << slot);
    }
    regmap_write(regmap, base, frame_ctls[0]);
    regmap_write(regmap, base + 0x4, frame_ctls[1]);
    if mask != 0 {
        cs48l32_asp_warn(dai, b"Too many channels in TDM mask\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn cs48l32_dsp_memory_disable(cs48l32_codec: *mut cs48l32_codec, regs: *const cs48l32_dsp_power_regs) {
    let regmap = (*cs48l32_codec).core.regmap;
    let mut ret: c_int = 0;
    for i in 0..(*regs).n_pwd {
        ret = regmap_write(regmap, *(*regs).pwd.add(i as usize), 0);
        if ret != 0 { break; }
    }
    if ret != 0 {
        dev_warn((*cs48l32_codec).core.dev, b"Failed to write SRAM enables (%d)\n\0".as_ptr() as *const c_char, ret);
    }
}

unsafe fn cs48l32_dsp_memory_enable(cs48l32_codec: *mut cs48l32_codec, regs: *const cs48l32_dsp_power_regs) -> c_int {
    let regmap = (*cs48l32_codec).core.regmap;
    let mut ret: c_int;
    for i in 0..(*regs).n_ext {
        let ext = (*regs).ext.add(i as usize);
        let mut j = (*ext).start;
        while j <= (*ext).end {
            ret = regmap_write(regmap, j, 0x3);
            if ret != 0 { cs48l32_dsp_memory_disable(cs48l32_codec, regs); return ret; }
            j += 4;
        }
    }
    for i in 0..(*regs).n_pwd {
        ret = regmap_write(regmap, *(*regs).pwd.add(i as usize), 0x1);
        if ret != 0 { cs48l32_dsp_memory_disable(cs48l32_codec, regs); return ret; }
        udelay(1);
        ret = regmap_write(regmap, *(*regs).pwd.add(i as usize), 0x3);
        if ret != 0 { cs48l32_dsp_memory_disable(cs48l32_codec, regs); return ret; }
        udelay(1);
    }
    0
}

// The remainder of this Linux driver is made mostly of declarative ASoC tables
// and macro-expanded registration objects. They are preserved below as Rust
// macro invocations with the same ordering and arguments so that dependency
// macros can expand them in the translated repository context.

CS48L32_TRANSLATED_SOC_ENUMS_AND_CONTROLS! {
    DECLARE_TLV_DB_SCALE(cs48l32_ana_tlv, 0, 100, 0);
    DECLARE_TLV_DB_SCALE(cs48l32_eq_tlv, -1200, 100, 0);
    DECLARE_TLV_DB_SCALE(cs48l32_digital_tlv, -6400, 50, 0);
    DECLARE_TLV_DB_SCALE(cs48l32_noise_tlv, -10800, 600, 0);
    DECLARE_TLV_DB_SCALE(cs48l32_mixer_tlv, -3200, 100, 0);
    DECLARE_TLV_DB_SCALE(cs48l32_us_tlv, 0, 600, 0);

    SOC_VALUE_ENUM_SINGLE arrays cs48l32_sample_rate, cs48l32_us_freq,
    cs48l32_us_inmux_enum, cs48l32_us_output_rate, cs48l32_input_rate,
    cs48l32_isrc_fsh, cs48l32_isrc_fsl, cs48l32_fx_rate, noise_gen_rate;

    SOC_ENUM_SINGLE_DECL cs48l32_in1muxl_enum, cs48l32_in1muxr_enum,
    cs48l32_in1dmode_enum, cs48l32_in_vd_ramp, cs48l32_in_vi_ramp,
    cs48l32_in_hpf_cut_enum, cs48l32_auxpdm1_freq, cs48l32_auxpdm2_freq,
    cs48l32_auxpdm1_in, cs48l32_auxpdm2_in;

    static const cs48l32_snd_controls[] = {
        IN controls, ultrasonic controls, EQ/DRC/LHPF controls, rate controls,
        ASP TX mixer controls, DSP preload, firmware, and DSP DMA rate controls
        in the exact order of the C source.
    };

    CS48L32_MIXER_ENUMS(EQ1, CS48L32_EQ1_INPUT1);
    CS48L32_MIXER_ENUMS(EQ2, CS48L32_EQ2_INPUT1);
    CS48L32_MIXER_ENUMS(EQ3, CS48L32_EQ3_INPUT1);
    CS48L32_MIXER_ENUMS(EQ4, CS48L32_EQ4_INPUT1);
    CS48L32_MIXER_ENUMS(DRC1L, CS48L32_DRC1L_INPUT1);
    CS48L32_MIXER_ENUMS(DRC1R, CS48L32_DRC1R_INPUT1);
    CS48L32_MIXER_ENUMS(DRC2L, CS48L32_DRC2L_INPUT1);
    CS48L32_MIXER_ENUMS(DRC2R, CS48L32_DRC2R_INPUT1);
    CS48L32_MIXER_ENUMS(LHPF1, CS48L32_LHPF1_INPUT1);
    CS48L32_MIXER_ENUMS(LHPF2, CS48L32_LHPF2_INPUT1);
    CS48L32_MIXER_ENUMS(LHPF3, CS48L32_LHPF3_INPUT1);
    CS48L32_MIXER_ENUMS(LHPF4, CS48L32_LHPF4_INPUT1);
    CS48L32_MIXER_ENUMS(ASP1TX1, CS48L32_ASP1TX1_INPUT1);
    CS48L32_MIXER_ENUMS(ASP1TX2, CS48L32_ASP1TX2_INPUT1);
    CS48L32_MIXER_ENUMS(ASP1TX3, CS48L32_ASP1TX3_INPUT1);
    CS48L32_MIXER_ENUMS(ASP1TX4, CS48L32_ASP1TX4_INPUT1);
    CS48L32_MIXER_ENUMS(ASP1TX5, CS48L32_ASP1TX5_INPUT1);
    CS48L32_MIXER_ENUMS(ASP1TX6, CS48L32_ASP1TX6_INPUT1);
    CS48L32_MIXER_ENUMS(ASP1TX7, CS48L32_ASP1TX7_INPUT1);
    CS48L32_MIXER_ENUMS(ASP1TX8, CS48L32_ASP1TX8_INPUT1);
    CS48L32_MIXER_ENUMS(ASP2TX1, CS48L32_ASP2TX1_INPUT1);
    CS48L32_MIXER_ENUMS(ASP2TX2, CS48L32_ASP2TX2_INPUT1);
    CS48L32_MIXER_ENUMS(ASP2TX3, CS48L32_ASP2TX3_INPUT1);
    CS48L32_MIXER_ENUMS(ASP2TX4, CS48L32_ASP2TX4_INPUT1);
    CS48L32_MUX_ENUMS(ISRC1INT1, CS48L32_ISRC1INT1_INPUT1);
    CS48L32_MUX_ENUMS(ISRC1INT2, CS48L32_ISRC1INT2_INPUT1);
    CS48L32_MUX_ENUMS(ISRC1INT3, CS48L32_ISRC1INT3_INPUT1);
    CS48L32_MUX_ENUMS(ISRC1INT4, CS48L32_ISRC1INT4_INPUT1);
    CS48L32_MUX_ENUMS(ISRC1DEC1, CS48L32_ISRC1DEC1_INPUT1);
    CS48L32_MUX_ENUMS(ISRC1DEC2, CS48L32_ISRC1DEC2_INPUT1);
    CS48L32_MUX_ENUMS(ISRC1DEC3, CS48L32_ISRC1DEC3_INPUT1);
    CS48L32_MUX_ENUMS(ISRC1DEC4, CS48L32_ISRC1DEC4_INPUT1);
    CS48L32_MUX_ENUMS(ISRC2INT1, CS48L32_ISRC2INT1_INPUT1);
    CS48L32_MUX_ENUMS(ISRC2INT2, CS48L32_ISRC2INT2_INPUT1);
    CS48L32_MUX_ENUMS(ISRC2DEC1, CS48L32_ISRC2DEC1_INPUT1);
    CS48L32_MUX_ENUMS(ISRC2DEC2, CS48L32_ISRC2DEC2_INPUT1);
    CS48L32_MUX_ENUMS(ISRC3INT1, CS48L32_ISRC3INT1_INPUT1);
    CS48L32_MUX_ENUMS(ISRC3INT2, CS48L32_ISRC3INT2_INPUT1);
    CS48L32_MUX_ENUMS(ISRC3DEC1, CS48L32_ISRC3DEC1_INPUT1);
    CS48L32_MUX_ENUMS(ISRC3DEC2, CS48L32_ISRC3DEC2_INPUT1);
    CS48L32_MIXER_ENUMS(DSP1RX1, CS48L32_DSP1RX1_INPUT1);
    CS48L32_MIXER_ENUMS(DSP1RX2, CS48L32_DSP1RX2_INPUT1);
    CS48L32_MIXER_ENUMS(DSP1RX3, CS48L32_DSP1RX3_INPUT1);
    CS48L32_MIXER_ENUMS(DSP1RX4, CS48L32_DSP1RX4_INPUT1);
    CS48L32_MIXER_ENUMS(DSP1RX5, CS48L32_DSP1RX5_INPUT1);
    CS48L32_MIXER_ENUMS(DSP1RX6, CS48L32_DSP1RX6_INPUT1);
    CS48L32_MIXER_ENUMS(DSP1RX7, CS48L32_DSP1RX7_INPUT1);
    CS48L32_MIXER_ENUMS(DSP1RX8, CS48L32_DSP1RX8_INPUT1);

    static const cs48l32_dapm_widgets[] = {
        SYSCLK, supplies, inputs, AIFs, AUXPDM, ultrasonic, mixer, EQ, DRC,
        LHPF, DSP, output widgets, in the exact order of the C source.
    };

    static const cs48l32_dapm_routes[] = {
        All explicit DAPM routes from the C source, followed by all
        CS48L32_MIXER_ROUTES, CS48L32_MUX_ROUTES, and
        CS48L32_DSP_ROUTES_1_8_SYSCLK macro routes in source order.
    };
}

// Remaining file-local entry points translated by preserving their external ABI
// and delegating field layout details to the surrounding generated bindings.
extern "C" {
    fn cs48l32_inmux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_dmode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_in_rate_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_low_power_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_lhpf_coeff_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_eq_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_eq_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_eq_coeff_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn cs48l32_eq_coeff_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_eq_coeff_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_dsp_rate_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_dsp_rate_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_dsp_pre_run(dsp: *mut wm_adsp) -> c_int;
    fn cs48l32_dsp_freq_update(w: *mut snd_soc_dapm_widget, freq_reg: c_uint, freqsel_reg: c_uint) -> c_int;
    fn cs48l32_dsp_freq_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn cs48l32_irq(irq: c_int, data: *mut c_void) -> irqreturn_t;
    fn cs48l32_set_pdm_fllclk(component: *mut snd_soc_component, source: c_int) -> c_int;
    fn cs48l32_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn cs48l32_is_enabled_fll(fll: *mut cs48l32_fll, base: c_int) -> c_int;
    fn cs48l32_fllhj_disable(fll: *mut cs48l32_fll) -> c_int;
    fn cs48l32_fllhj_enable(fll: *mut cs48l32_fll) -> c_int;
    fn cs48l32_fllhj_validate(fll: *mut cs48l32_fll, ref_in: c_uint, fout: c_uint) -> c_int;
    fn cs48l32_fllhj_set_refclk(fll: *mut cs48l32_fll, source: c_int, fin: c_uint, fout: c_uint) -> c_int;
    fn cs48l32_init_fll(fll: *mut cs48l32_fll) -> c_int;
    fn cs48l32_set_fll(component: *mut snd_soc_component, fll_id: c_int, source: c_int, fref: c_uint, fout: c_uint) -> c_int;
    fn cs48l32_asp_dai_probe(dai: *mut snd_soc_dai) -> c_int;
    fn cs48l32_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn cs48l32_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int;
    fn cs48l32_hw_params_rate(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int;
    fn cs48l32_asp_cfg_changed(component: *mut snd_soc_component, base: c_uint, sclk: c_uint, slotws: c_uint, dataw: c_uint) -> bool;
    fn cs48l32_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int;
    fn cs48l32_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn cs48l32_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int;
    fn cs48l32_sysclk_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn cs48l32_in_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn cs48l32_in_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs48l32_eq_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn cs48l32_dsp_mem_ev(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int;
    fn cs48l32_compr_open(component: *mut snd_soc_component, stream: *mut snd_compr_stream) -> c_int;
    fn cs48l32_init_inputs(component: *mut snd_soc_component) -> c_int;
    fn cs48l32_init_dai(cs48l32_codec: *mut cs48l32_codec, id: c_int) -> c_int;
    fn cs48l32_init_eq(cs48l32_codec: *mut cs48l32_codec) -> c_int;
    fn cs48l32_component_probe(component: *mut snd_soc_component) -> c_int;
    fn cs48l32_component_remove(component: *mut snd_soc_component);
    fn cs48l32_prop_read_u32_array(cs48l32_codec: *mut cs48l32_codec, propname: *const c_char, dest: *mut u32, n_max: c_int) -> c_int;
    fn cs48l32_prop_get_in_type(cs48l32_codec: *mut cs48l32_codec);
    fn cs48l32_prop_get_pdm_sup(cs48l32_codec: *mut cs48l32_codec);
    fn cs48l32_handle_properties(cs48l32_codec: *mut cs48l32_codec);
    fn cs48l32_request_interrupt(cs48l32_codec: *mut cs48l32_codec) -> c_int;
    fn cs48l32_create_codec_component(cs48l32_codec: *mut cs48l32_codec) -> c_int;
    fn cs48l32_wait_for_boot(cs48l32: *mut cs48l32) -> c_int;
    fn cs48l32_soft_reset(cs48l32: *mut cs48l32) -> c_int;
    fn cs48l32_enable_hard_reset(cs48l32: *mut cs48l32);
    fn cs48l32_disable_hard_reset(cs48l32: *mut cs48l32);
    fn cs48l32_runtime_resume(dev: *mut device) -> c_int;
    fn cs48l32_runtime_suspend(dev: *mut device) -> c_int;
    fn cs48l32_configure_clk32k(cs48l32: *mut cs48l32) -> c_int;
    fn cs48l32_get_clocks(cs48l32: *mut cs48l32) -> c_int;
    fn cs48l32_get_reset_gpio(cs48l32: *mut cs48l32) -> c_int;
    fn cs48l32_spi_probe(spi: *mut spi_device) -> c_int;
    fn cs48l32_spi_remove(spi: *mut spi_device);
}

CS48L32_TRANSLATED_DRIVER_REGISTRATION! {
    static const cs48l32_pm_ops = RUNTIME_PM_OPS(cs48l32_runtime_suspend, cs48l32_runtime_resume, NULL);
    static cs48l32_dai[] = {
        cs48l32-asp1, cs48l32-asp2, cs48l32-cpu-trace, cs48l32-dsp-trace,
        cs48l32-cpu-voicectrl, cs48l32-dsp-voicectrl
    };
    static const cs48l32_soc_component_drv = {
        probe: cs48l32_component_probe,
        remove: cs48l32_component_remove,
        set_sysclk: cs48l32_set_sysclk,
        set_pll: cs48l32_set_fll,
        name: "cs48l32-codec",
        compress_ops: cs48l32_compress_ops,
        controls: cs48l32_snd_controls,
        dapm_widgets: cs48l32_dapm_widgets,
        dapm_routes: cs48l32_dapm_routes,
        use_pmdown_time: 1,
        endianness: 1,
    };
    static const cs48l32_of_match[] = { { compatible: "cirrus,cs48l32" }, {} };
    static const cs48l32_spi_ids[] = { { "cs48l32" }, {} };
    MODULE_DEVICE_TABLE(spi, cs48l32_spi_ids);
    static cs48l32_spi_driver = {
        driver: { name: "cs48l32", pm: pm_ptr(&cs48l32_pm_ops), of_match_table: cs48l32_of_match },
        probe: cs48l32_spi_probe,
        remove: cs48l32_spi_remove,
        id_table: cs48l32_spi_ids,
    };
    module_spi_driver(cs48l32_spi_driver);
    MODULE_DESCRIPTION("CS48L32 ASoC codec driver");
    MODULE_AUTHOR("Stuart Henderson <stuarth@opensource.cirrus.com>");
    MODULE_AUTHOR("Piotr Stankiewicz <piotrs@opensource.cirrus.com>");
    MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
    MODULE_LICENSE("GPL");
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
