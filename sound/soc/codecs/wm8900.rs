// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8900.rs  --  WM8900 ALSA Soc Audio driver
 *
 * Copyright 2007, 2008 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 * TODO:
 *  - Tristating.
 *  - TDM.
 *  - Jack detect.
 *  - FLL source configuration, currently only MCLK is supported.
 */

/* Original C dependencies:
 * linux/module.h, linux/moduleparam.h, linux/kernel.h, linux/init.h,
 * linux/delay.h, linux/pm.h, linux/i2c.h, linux/regmap.h, linux/spi/spi.h,
 * linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
 * sound/initval.h, sound/tlv.h, and "wm8900.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const WM8900_REG_RESET: c_uint = 0x0;
const WM8900_REG_ID: c_uint = 0x0;
const WM8900_REG_POWER1: c_uint = 0x1;
const WM8900_REG_POWER2: c_uint = 0x2;
const WM8900_REG_POWER3: c_uint = 0x3;
const WM8900_REG_AUDIO1: c_uint = 0x4;
const WM8900_REG_AUDIO2: c_uint = 0x5;
const WM8900_REG_CLOCKING1: c_uint = 0x6;
const WM8900_REG_CLOCKING2: c_uint = 0x7;
const WM8900_REG_AUDIO3: c_uint = 0x8;
const WM8900_REG_AUDIO4: c_uint = 0x9;
const WM8900_REG_DACCTRL: c_uint = 0xa;
const WM8900_REG_LDAC_DV: c_uint = 0xb;
const WM8900_REG_RDAC_DV: c_uint = 0xc;
const WM8900_REG_SIDETONE: c_uint = 0xd;
const WM8900_REG_ADCCTRL: c_uint = 0xe;
const WM8900_REG_LADC_DV: c_uint = 0xf;
const WM8900_REG_RADC_DV: c_uint = 0x10;
const WM8900_REG_GPIO: c_uint = 0x12;
const WM8900_REG_INCTL: c_uint = 0x15;
const WM8900_REG_LINVOL: c_uint = 0x16;
const WM8900_REG_RINVOL: c_uint = 0x17;
const WM8900_REG_INBOOSTMIX1: c_uint = 0x18;
const WM8900_REG_INBOOSTMIX2: c_uint = 0x19;
const WM8900_REG_ADCPATH: c_uint = 0x1a;
const WM8900_REG_AUXBOOST: c_uint = 0x1b;
const WM8900_REG_ADDCTL: c_uint = 0x1e;
const WM8900_REG_FLLCTL1: c_uint = 0x24;
const WM8900_REG_FLLCTL2: c_uint = 0x25;
const WM8900_REG_FLLCTL3: c_uint = 0x26;
const WM8900_REG_FLLCTL4: c_uint = 0x27;
const WM8900_REG_FLLCTL5: c_uint = 0x28;
const WM8900_REG_FLLCTL6: c_uint = 0x29;
const WM8900_REG_LOUTMIXCTL1: c_uint = 0x2c;
const WM8900_REG_ROUTMIXCTL1: c_uint = 0x2d;
const WM8900_REG_BYPASS1: c_uint = 0x2e;
const WM8900_REG_BYPASS2: c_uint = 0x2f;
const WM8900_REG_AUXOUT_CTL: c_uint = 0x30;
const WM8900_REG_LOUT1CTL: c_uint = 0x33;
const WM8900_REG_ROUT1CTL: c_uint = 0x34;
const WM8900_REG_LOUT2CTL: c_uint = 0x35;
const WM8900_REG_ROUT2CTL: c_uint = 0x36;
const WM8900_REG_HPCTL1: c_uint = 0x3a;
const WM8900_REG_OUTBIASCTL: c_uint = 0x73;

const WM8900_MAXREG: c_uint = 0x80;

const WM8900_REG_ADDCTL_OUT1_DIS: c_uint = 0x80;
const WM8900_REG_ADDCTL_OUT2_DIS: c_uint = 0x40;
const WM8900_REG_ADDCTL_VMID_DIS: c_uint = 0x20;
const WM8900_REG_ADDCTL_BIAS_SRC: c_uint = 0x10;
const WM8900_REG_ADDCTL_VMID_SOFTST: c_uint = 0x04;
const WM8900_REG_ADDCTL_TEMP_SD: c_uint = 0x02;

const WM8900_REG_GPIO_TEMP_ENA: c_uint = 0x2;

const WM8900_REG_POWER1_STARTUP_BIAS_ENA: c_uint = 0x0100;
const WM8900_REG_POWER1_BIAS_ENA: c_uint = 0x0008;
const WM8900_REG_POWER1_VMID_BUF_ENA: c_uint = 0x0004;
const WM8900_REG_POWER1_FLL_ENA: c_uint = 0x0040;

const WM8900_REG_POWER2_SYSCLK_ENA: c_uint = 0x8000;
const WM8900_REG_POWER2_ADCL_ENA: c_uint = 0x0002;
const WM8900_REG_POWER2_ADCR_ENA: c_uint = 0x0001;

const WM8900_REG_POWER3_DACL_ENA: c_uint = 0x0002;
const WM8900_REG_POWER3_DACR_ENA: c_uint = 0x0001;

const WM8900_REG_AUDIO1_AIF_FMT_MASK: c_uint = 0x0018;
const WM8900_REG_AUDIO1_LRCLK_INV: c_uint = 0x0080;
const WM8900_REG_AUDIO1_BCLK_INV: c_uint = 0x0100;

const WM8900_REG_CLOCKING1_BCLK_DIR: c_uint = 0x1;
const WM8900_REG_CLOCKING1_MCLK_SRC: c_uint = 0x100;
const WM8900_REG_CLOCKING1_BCLK_MASK: c_uint = 0x01e;
const WM8900_REG_CLOCKING1_OPCLK_MASK: c_uint = 0x7000;

const WM8900_REG_CLOCKING2_ADC_CLKDIV: c_uint = 0xe0;
const WM8900_REG_CLOCKING2_DAC_CLKDIV: c_uint = 0x1c;

const WM8900_REG_DACCTRL_MUTE: c_uint = 0x004;
const WM8900_REG_DACCTRL_DAC_SB_FILT: c_uint = 0x100;
const WM8900_REG_DACCTRL_AIF_LRCLKRATE: c_uint = 0x400;

const WM8900_REG_AUDIO3_ADCLRC_DIR: c_uint = 0x0800;
const WM8900_REG_AUDIO4_DACLRC_DIR: c_uint = 0x0800;
const WM8900_REG_FLLCTL1_OSC_ENA: c_uint = 0x100;
const WM8900_REG_FLLCTL6_FLL_SLOW_LOCK_REF: c_uint = 0x100;

const WM8900_REG_HPCTL1_HP_IPSTAGE_ENA: c_uint = 0x80;
const WM8900_REG_HPCTL1_HP_OPSTAGE_ENA: c_uint = 0x40;
const WM8900_REG_HPCTL1_HP_CLAMP_IP: c_uint = 0x20;
const WM8900_REG_HPCTL1_HP_CLAMP_OP: c_uint = 0x10;
const WM8900_REG_HPCTL1_HP_SHORT: c_uint = 0x08;
const WM8900_REG_HPCTL1_HP_SHORT2: c_uint = 0x04;

const WM8900_LRC_MASK: c_uint = 0x03ff;
const FIXED_FLL_SIZE: u64 = ((1u64 << 16) * 10);

#[repr(C)]
struct wm8900_priv {
    regmap: *mut regmap,
    fll_in: u32,
    fll_out: u32,
}

#[repr(C)]
struct _fll_div {
    fll_ratio: u16,
    fllclk_div: u16,
    fll_slow_lock_ref: u16,
    n: u16,
    k: u16,
}

unsafe extern "C" {
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn msleep(msecs: c_uint);
    fn schedule_timeout_interruptible(timeout: c_long) -> c_long;
    fn msecs_to_jiffies(msecs: c_uint) -> c_long;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn spi_register_driver(driver: *mut spi_driver) -> c_int;
    fn spi_unregister_driver(driver: *mut spi_driver);
}

unsafe fn wm8900_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM8900_REG_ID => true,
        _ => false,
    }
}

unsafe fn wm8900_reset(component: *mut snd_soc_component) {
    snd_soc_component_write(component, WM8900_REG_RESET, 0);
}

unsafe fn wm8900_hp_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut hpctl1: u16 = snd_soc_component_read(component, WM8900_REG_HPCTL1) as u16;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* Clamp headphone outputs */
            hpctl1 = (WM8900_REG_HPCTL1_HP_CLAMP_IP | WM8900_REG_HPCTL1_HP_CLAMP_OP) as u16;
            snd_soc_component_write(component, WM8900_REG_HPCTL1, hpctl1 as c_uint);
        }
        SND_SOC_DAPM_POST_PMU => {
            /* Enable the input stage */
            hpctl1 &= !(WM8900_REG_HPCTL1_HP_CLAMP_IP as u16);
            hpctl1 |= (WM8900_REG_HPCTL1_HP_SHORT
                | WM8900_REG_HPCTL1_HP_SHORT2
                | WM8900_REG_HPCTL1_HP_IPSTAGE_ENA) as u16;
            snd_soc_component_write(component, WM8900_REG_HPCTL1, hpctl1 as c_uint);

            msleep(400);

            /* Enable the output stage */
            hpctl1 &= !(WM8900_REG_HPCTL1_HP_CLAMP_OP as u16);
            hpctl1 |= WM8900_REG_HPCTL1_HP_OPSTAGE_ENA as u16;
            snd_soc_component_write(component, WM8900_REG_HPCTL1, hpctl1 as c_uint);

            /* Remove the shorts */
            hpctl1 &= !(WM8900_REG_HPCTL1_HP_SHORT2 as u16);
            snd_soc_component_write(component, WM8900_REG_HPCTL1, hpctl1 as c_uint);
            hpctl1 &= !(WM8900_REG_HPCTL1_HP_SHORT as u16);
            snd_soc_component_write(component, WM8900_REG_HPCTL1, hpctl1 as c_uint);
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* Short the output */
            hpctl1 |= WM8900_REG_HPCTL1_HP_SHORT as u16;
            snd_soc_component_write(component, WM8900_REG_HPCTL1, hpctl1 as c_uint);

            /* Disable the output stage */
            hpctl1 &= !(WM8900_REG_HPCTL1_HP_OPSTAGE_ENA as u16);
            snd_soc_component_write(component, WM8900_REG_HPCTL1, hpctl1 as c_uint);

            /* Clamp the outputs and power down input */
            hpctl1 |= (WM8900_REG_HPCTL1_HP_CLAMP_IP | WM8900_REG_HPCTL1_HP_CLAMP_OP) as u16;
            hpctl1 &= !(WM8900_REG_HPCTL1_HP_IPSTAGE_ENA as u16);
            snd_soc_component_write(component, WM8900_REG_HPCTL1, hpctl1 as c_uint);
        }
        SND_SOC_DAPM_POST_PMD => {
            /* Disable everything */
            snd_soc_component_write(component, WM8900_REG_HPCTL1, 0);
        }
        _ => {
            WARN(1, c"Invalid event %d\n".as_ptr(), event);
        }
    }

    0
}

static wm8900_reg_defaults: [reg_default; 55] = [
    reg_default { reg: 1, def: 0x0000 }, reg_default { reg: 2, def: 0xc000 },
    reg_default { reg: 3, def: 0x0000 }, reg_default { reg: 4, def: 0x4050 },
    reg_default { reg: 5, def: 0x4000 }, reg_default { reg: 6, def: 0x0008 },
    reg_default { reg: 7, def: 0x0000 }, reg_default { reg: 8, def: 0x0040 },
    reg_default { reg: 9, def: 0x0040 }, reg_default { reg: 10, def: 0x1004 },
    reg_default { reg: 11, def: 0x00c0 }, reg_default { reg: 12, def: 0x00c0 },
    reg_default { reg: 13, def: 0x0000 }, reg_default { reg: 14, def: 0x0100 },
    reg_default { reg: 15, def: 0x00c0 }, reg_default { reg: 16, def: 0x00c0 },
    reg_default { reg: 17, def: 0x0000 }, reg_default { reg: 18, def: 0xb001 },
    reg_default { reg: 19, def: 0x0000 }, reg_default { reg: 20, def: 0x0000 },
    reg_default { reg: 21, def: 0x0044 }, reg_default { reg: 22, def: 0x004c },
    reg_default { reg: 23, def: 0x004c }, reg_default { reg: 24, def: 0x0044 },
    reg_default { reg: 25, def: 0x0044 }, reg_default { reg: 26, def: 0x0000 },
    reg_default { reg: 27, def: 0x0044 }, reg_default { reg: 28, def: 0x0000 },
    reg_default { reg: 29, def: 0x0000 }, reg_default { reg: 30, def: 0x0002 },
    reg_default { reg: 31, def: 0x0000 }, reg_default { reg: 32, def: 0x0000 },
    reg_default { reg: 33, def: 0x0000 }, reg_default { reg: 34, def: 0x0000 },
    reg_default { reg: 35, def: 0x0000 }, reg_default { reg: 36, def: 0x0008 },
    reg_default { reg: 37, def: 0x0000 }, reg_default { reg: 38, def: 0x0000 },
    reg_default { reg: 39, def: 0x0008 }, reg_default { reg: 40, def: 0x0097 },
    reg_default { reg: 41, def: 0x0100 }, reg_default { reg: 42, def: 0x0000 },
    reg_default { reg: 43, def: 0x0000 }, reg_default { reg: 44, def: 0x0050 },
    reg_default { reg: 45, def: 0x0050 }, reg_default { reg: 46, def: 0x0055 },
    reg_default { reg: 47, def: 0x0055 }, reg_default { reg: 48, def: 0x0055 },
    reg_default { reg: 49, def: 0x0000 }, reg_default { reg: 50, def: 0x0000 },
    reg_default { reg: 51, def: 0x0079 }, reg_default { reg: 52, def: 0x0079 },
    reg_default { reg: 53, def: 0x0079 }, reg_default { reg: 54, def: 0x0079 },
    reg_default { reg: 55, def: 0x0000 },
];

static out_pga_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-5700, 100, 0);
static out_mix_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-1500, 300, 0);
static in_boost_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-1200, 600, 0);
static in_pga_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-1200, 100, 0);
static dac_boost_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(0, 600, 0);
static dac_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-7200, 75, 1);
static adc_svol_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-3600, 300, 0);
static adc_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-7200, 75, 1);

static mic_bias_level_txt: [*const c_char; 2] = [c"0.9*AVDD".as_ptr(), c"0.65*AVDD".as_ptr()];
static mic_bias_level: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_INCTL, 8, mic_bias_level_txt);
static dac_mute_rate_txt: [*const c_char; 2] = [c"Fast".as_ptr(), c"Slow".as_ptr()];
static dac_mute_rate: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_DACCTRL, 7, dac_mute_rate_txt);
static dac_deemphasis_txt: [*const c_char; 4] = [c"Disabled".as_ptr(), c"32kHz".as_ptr(), c"44.1kHz".as_ptr(), c"48kHz".as_ptr()];
static dac_deemphasis: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_DACCTRL, 4, dac_deemphasis_txt);
static adc_hpf_cut_txt: [*const c_char; 4] = [c"Hi-fi mode".as_ptr(), c"Voice mode 1".as_ptr(), c"Voice mode 2".as_ptr(), c"Voice mode 3".as_ptr()];
static adc_hpf_cut: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_ADCCTRL, 5, adc_hpf_cut_txt);
static lr_txt: [*const c_char; 2] = [c"Left".as_ptr(), c"Right".as_ptr()];
static aifl_src: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_AUDIO1, 15, lr_txt);
static aifr_src: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_AUDIO1, 14, lr_txt);
static dacl_src: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_AUDIO2, 15, lr_txt);
static dacr_src: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_AUDIO2, 14, lr_txt);
static sidetone_txt: [*const c_char; 3] = [c"Disabled".as_ptr(), c"Left ADC".as_ptr(), c"Right ADC".as_ptr()];
static dacl_sidetone: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_SIDETONE, 2, sidetone_txt);
static dacr_sidetone: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_SIDETONE, 0, sidetone_txt);

static wm8900_snd_controls: [snd_kcontrol_new; 47] = [
    SOC_ENUM!(c"Mic Bias Level".as_ptr(), mic_bias_level),
    SOC_SINGLE_TLV!(c"Left Input PGA Volume".as_ptr(), WM8900_REG_LINVOL, 0, 31, 0, in_pga_tlv),
    SOC_SINGLE!(c"Left Input PGA Switch".as_ptr(), WM8900_REG_LINVOL, 6, 1, 1),
    SOC_SINGLE!(c"Left Input PGA ZC Switch".as_ptr(), WM8900_REG_LINVOL, 7, 1, 0),
    SOC_SINGLE_TLV!(c"Right Input PGA Volume".as_ptr(), WM8900_REG_RINVOL, 0, 31, 0, in_pga_tlv),
    SOC_SINGLE!(c"Right Input PGA Switch".as_ptr(), WM8900_REG_RINVOL, 6, 1, 1),
    SOC_SINGLE!(c"Right Input PGA ZC Switch".as_ptr(), WM8900_REG_RINVOL, 7, 1, 0),
    SOC_SINGLE!(c"DAC Soft Mute Switch".as_ptr(), WM8900_REG_DACCTRL, 6, 1, 1),
    SOC_ENUM!(c"DAC Mute Rate".as_ptr(), dac_mute_rate),
    SOC_SINGLE!(c"DAC Mono Switch".as_ptr(), WM8900_REG_DACCTRL, 9, 1, 0),
    SOC_ENUM!(c"DAC Deemphasis".as_ptr(), dac_deemphasis),
    SOC_SINGLE!(c"DAC Sigma-Delta Modulator Clock Switch".as_ptr(), WM8900_REG_DACCTRL, 12, 1, 0),
    SOC_SINGLE!(c"ADC HPF Switch".as_ptr(), WM8900_REG_ADCCTRL, 8, 1, 0),
    SOC_ENUM!(c"ADC HPF Cut-Off".as_ptr(), adc_hpf_cut),
    SOC_DOUBLE!(c"ADC Invert Switch".as_ptr(), WM8900_REG_ADCCTRL, 1, 0, 1, 0),
    SOC_SINGLE_TLV!(c"Left ADC Sidetone Volume".as_ptr(), WM8900_REG_SIDETONE, 9, 12, 0, adc_svol_tlv),
    SOC_SINGLE_TLV!(c"Right ADC Sidetone Volume".as_ptr(), WM8900_REG_SIDETONE, 5, 12, 0, adc_svol_tlv),
    SOC_ENUM!(c"Left Digital Audio Source".as_ptr(), aifl_src),
    SOC_ENUM!(c"Right Digital Audio Source".as_ptr(), aifr_src),
    SOC_SINGLE_TLV!(c"DAC Input Boost Volume".as_ptr(), WM8900_REG_AUDIO2, 10, 4, 0, dac_boost_tlv),
    SOC_ENUM!(c"Left DAC Source".as_ptr(), dacl_src),
    SOC_ENUM!(c"Right DAC Source".as_ptr(), dacr_src),
    SOC_ENUM!(c"Left DAC Sidetone".as_ptr(), dacl_sidetone),
    SOC_ENUM!(c"Right DAC Sidetone".as_ptr(), dacr_sidetone),
    SOC_DOUBLE!(c"DAC Invert Switch".as_ptr(), WM8900_REG_DACCTRL, 1, 0, 1, 0),
    SOC_DOUBLE_R_TLV!(c"Digital Playback Volume".as_ptr(), WM8900_REG_LDAC_DV, WM8900_REG_RDAC_DV, 1, 96, 0, dac_tlv),
    SOC_DOUBLE_R_TLV!(c"Digital Capture Volume".as_ptr(), WM8900_REG_LADC_DV, WM8900_REG_RADC_DV, 1, 119, 0, adc_tlv),
    SOC_SINGLE_TLV!(c"LINPUT3 Bypass Volume".as_ptr(), WM8900_REG_LOUTMIXCTL1, 4, 7, 0, out_mix_tlv),
    SOC_SINGLE_TLV!(c"RINPUT3 Bypass Volume".as_ptr(), WM8900_REG_ROUTMIXCTL1, 4, 7, 0, out_mix_tlv),
    SOC_SINGLE_TLV!(c"Left AUX Bypass Volume".as_ptr(), WM8900_REG_AUXOUT_CTL, 4, 7, 0, out_mix_tlv),
    SOC_SINGLE_TLV!(c"Right AUX Bypass Volume".as_ptr(), WM8900_REG_AUXOUT_CTL, 0, 7, 0, out_mix_tlv),
    SOC_SINGLE_TLV!(c"LeftIn to RightOut Mixer Volume".as_ptr(), WM8900_REG_BYPASS1, 0, 7, 0, out_mix_tlv),
    SOC_SINGLE_TLV!(c"LeftIn to LeftOut Mixer Volume".as_ptr(), WM8900_REG_BYPASS1, 4, 7, 0, out_mix_tlv),
    SOC_SINGLE_TLV!(c"RightIn to LeftOut Mixer Volume".as_ptr(), WM8900_REG_BYPASS2, 0, 7, 0, out_mix_tlv),
    SOC_SINGLE_TLV!(c"RightIn to RightOut Mixer Volume".as_ptr(), WM8900_REG_BYPASS2, 4, 7, 0, out_mix_tlv),
    SOC_SINGLE_TLV!(c"IN2L Boost Volume".as_ptr(), WM8900_REG_INBOOSTMIX1, 0, 3, 0, in_boost_tlv),
    SOC_SINGLE_TLV!(c"IN3L Boost Volume".as_ptr(), WM8900_REG_INBOOSTMIX1, 4, 3, 0, in_boost_tlv),
    SOC_SINGLE_TLV!(c"IN2R Boost Volume".as_ptr(), WM8900_REG_INBOOSTMIX2, 0, 3, 0, in_boost_tlv),
    SOC_SINGLE_TLV!(c"IN3R Boost Volume".as_ptr(), WM8900_REG_INBOOSTMIX2, 4, 3, 0, in_boost_tlv),
    SOC_SINGLE_TLV!(c"Left AUX Boost Volume".as_ptr(), WM8900_REG_AUXBOOST, 4, 3, 0, in_boost_tlv),
    SOC_SINGLE_TLV!(c"Right AUX Boost Volume".as_ptr(), WM8900_REG_AUXBOOST, 0, 3, 0, in_boost_tlv),
    SOC_DOUBLE_R_TLV!(c"LINEOUT1 Volume".as_ptr(), WM8900_REG_LOUT1CTL, WM8900_REG_ROUT1CTL, 0, 63, 0, out_pga_tlv),
    SOC_DOUBLE_R!(c"LINEOUT1 Switch".as_ptr(), WM8900_REG_LOUT1CTL, WM8900_REG_ROUT1CTL, 6, 1, 1),
    SOC_DOUBLE_R!(c"LINEOUT1 ZC Switch".as_ptr(), WM8900_REG_LOUT1CTL, WM8900_REG_ROUT1CTL, 7, 1, 0),
    SOC_DOUBLE_R_TLV!(c"LINEOUT2 Volume".as_ptr(), WM8900_REG_LOUT2CTL, WM8900_REG_ROUT2CTL, 0, 63, 0, out_pga_tlv),
    SOC_DOUBLE_R!(c"LINEOUT2 Switch".as_ptr(), WM8900_REG_LOUT2CTL, WM8900_REG_ROUT2CTL, 6, 1, 1),
    SOC_DOUBLE_R!(c"LINEOUT2 ZC Switch".as_ptr(), WM8900_REG_LOUT2CTL, WM8900_REG_ROUT2CTL, 7, 1, 0),
    SOC_SINGLE!(c"LINEOUT2 LP -12dB".as_ptr(), WM8900_REG_LOUTMIXCTL1, 0, 1, 1),
];

static wm8900_loutmix_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE!(c"LINPUT3 Bypass Switch".as_ptr(), WM8900_REG_LOUTMIXCTL1, 7, 1, 0),
    SOC_DAPM_SINGLE!(c"AUX Bypass Switch".as_ptr(), WM8900_REG_AUXOUT_CTL, 7, 1, 0),
    SOC_DAPM_SINGLE!(c"Left Input Mixer Switch".as_ptr(), WM8900_REG_BYPASS1, 7, 1, 0),
    SOC_DAPM_SINGLE!(c"Right Input Mixer Switch".as_ptr(), WM8900_REG_BYPASS2, 3, 1, 0),
    SOC_DAPM_SINGLE!(c"DACL Switch".as_ptr(), WM8900_REG_LOUTMIXCTL1, 8, 1, 0),
];

static wm8900_routmix_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE!(c"RINPUT3 Bypass Switch".as_ptr(), WM8900_REG_ROUTMIXCTL1, 7, 1, 0),
    SOC_DAPM_SINGLE!(c"AUX Bypass Switch".as_ptr(), WM8900_REG_AUXOUT_CTL, 3, 1, 0),
    SOC_DAPM_SINGLE!(c"Left Input Mixer Switch".as_ptr(), WM8900_REG_BYPASS1, 3, 1, 0),
    SOC_DAPM_SINGLE!(c"Right Input Mixer Switch".as_ptr(), WM8900_REG_BYPASS2, 7, 1, 0),
    SOC_DAPM_SINGLE!(c"DACR Switch".as_ptr(), WM8900_REG_ROUTMIXCTL1, 8, 1, 0),
];

static wm8900_linmix_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!(c"LINPUT2 Switch".as_ptr(), WM8900_REG_INBOOSTMIX1, 2, 1, 1),
    SOC_DAPM_SINGLE!(c"LINPUT3 Switch".as_ptr(), WM8900_REG_INBOOSTMIX1, 6, 1, 1),
    SOC_DAPM_SINGLE!(c"AUX Switch".as_ptr(), WM8900_REG_AUXBOOST, 6, 1, 1),
    SOC_DAPM_SINGLE!(c"Input PGA Switch".as_ptr(), WM8900_REG_ADCPATH, 6, 1, 0),
];

static wm8900_rinmix_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!(c"RINPUT2 Switch".as_ptr(), WM8900_REG_INBOOSTMIX2, 2, 1, 1),
    SOC_DAPM_SINGLE!(c"RINPUT3 Switch".as_ptr(), WM8900_REG_INBOOSTMIX2, 6, 1, 1),
    SOC_DAPM_SINGLE!(c"AUX Switch".as_ptr(), WM8900_REG_AUXBOOST, 2, 1, 1),
    SOC_DAPM_SINGLE!(c"Input PGA Switch".as_ptr(), WM8900_REG_ADCPATH, 2, 1, 0),
];

static wm8900_linpga_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!(c"LINPUT1 Switch".as_ptr(), WM8900_REG_INCTL, 6, 1, 0),
    SOC_DAPM_SINGLE!(c"LINPUT2 Switch".as_ptr(), WM8900_REG_INCTL, 5, 1, 0),
    SOC_DAPM_SINGLE!(c"LINPUT3 Switch".as_ptr(), WM8900_REG_INCTL, 4, 1, 0),
];

static wm8900_rinpga_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!(c"RINPUT1 Switch".as_ptr(), WM8900_REG_INCTL, 2, 1, 0),
    SOC_DAPM_SINGLE!(c"RINPUT2 Switch".as_ptr(), WM8900_REG_INCTL, 1, 1, 0),
    SOC_DAPM_SINGLE!(c"RINPUT3 Switch".as_ptr(), WM8900_REG_INCTL, 0, 1, 0),
];

static wm8900_lp_mux: [*const c_char; 2] = [c"Disabled".as_ptr(), c"Enabled".as_ptr()];
static wm8900_lineout2_lp_mux: soc_enum = SOC_ENUM_SINGLE_DECL!(WM8900_REG_LOUTMIXCTL1, 1, wm8900_lp_mux);
static wm8900_lineout2_lp: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Route".as_ptr(), wm8900_lineout2_lp_mux);

static wm8900_dapm_widgets: [snd_soc_dapm_widget; 28] = [
    /* Externally visible pins */
    SND_SOC_DAPM_OUTPUT!(c"LINEOUT1L".as_ptr()), SND_SOC_DAPM_OUTPUT!(c"LINEOUT1R".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"LINEOUT2L".as_ptr()), SND_SOC_DAPM_OUTPUT!(c"LINEOUT2R".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"HP_L".as_ptr()), SND_SOC_DAPM_OUTPUT!(c"HP_R".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"RINPUT1".as_ptr()), SND_SOC_DAPM_INPUT!(c"LINPUT1".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"RINPUT2".as_ptr()), SND_SOC_DAPM_INPUT!(c"LINPUT2".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"RINPUT3".as_ptr()), SND_SOC_DAPM_INPUT!(c"LINPUT3".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"AUX".as_ptr()),
    SND_SOC_DAPM_VMID!(c"VMID".as_ptr()),
    /* Input */
    SND_SOC_DAPM_MIXER!(c"Left Input PGA".as_ptr(), WM8900_REG_POWER2, 3, 0, wm8900_linpga_controls.as_ptr(), wm8900_linpga_controls.len()),
    SND_SOC_DAPM_MIXER!(c"Right Input PGA".as_ptr(), WM8900_REG_POWER2, 2, 0, wm8900_rinpga_controls.as_ptr(), wm8900_rinpga_controls.len()),
    SND_SOC_DAPM_MIXER!(c"Left Input Mixer".as_ptr(), WM8900_REG_POWER2, 5, 0, wm8900_linmix_controls.as_ptr(), wm8900_linmix_controls.len()),
    SND_SOC_DAPM_MIXER!(c"Right Input Mixer".as_ptr(), WM8900_REG_POWER2, 4, 0, wm8900_rinmix_controls.as_ptr(), wm8900_rinmix_controls.len()),
    SND_SOC_DAPM_SUPPLY!(c"Mic Bias".as_ptr(), WM8900_REG_POWER1, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_ADC!(c"ADCL".as_ptr(), c"Left HiFi Capture".as_ptr(), WM8900_REG_POWER2, 1, 0),
    SND_SOC_DAPM_ADC!(c"ADCR".as_ptr(), c"Right HiFi Capture".as_ptr(), WM8900_REG_POWER2, 0, 0),
    /* Output */
    SND_SOC_DAPM_DAC!(c"DACL".as_ptr(), c"Left HiFi Playback".as_ptr(), WM8900_REG_POWER3, 1, 0),
    SND_SOC_DAPM_DAC!(c"DACR".as_ptr(), c"Right HiFi Playback".as_ptr(), WM8900_REG_POWER3, 0, 0),
    SND_SOC_DAPM_PGA_E!(c"Headphone Amplifier".as_ptr(), WM8900_REG_POWER3, 7, 0, core::ptr::null(), 0, wm8900_hp_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA!(c"LINEOUT1L PGA".as_ptr(), WM8900_REG_POWER2, 8, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"LINEOUT1R PGA".as_ptr(), WM8900_REG_POWER2, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MUX!(c"LINEOUT2 LP".as_ptr(), SND_SOC_NOPM, 0, 0, &wm8900_lineout2_lp),
    SND_SOC_DAPM_PGA!(c"LINEOUT2L PGA".as_ptr(), WM8900_REG_POWER3, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!(c"LINEOUT2R PGA".as_ptr(), WM8900_REG_POWER3, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!(c"Left Output Mixer".as_ptr(), WM8900_REG_POWER3, 3, 0, wm8900_loutmix_controls.as_ptr(), wm8900_loutmix_controls.len()),
    SND_SOC_DAPM_MIXER!(c"Right Output Mixer".as_ptr(), WM8900_REG_POWER3, 2, 0, wm8900_routmix_controls.as_ptr(), wm8900_routmix_controls.len()),
];

/* Target, Path, Source */
static wm8900_dapm_routes: [snd_soc_dapm_route; 42] = [
    DAPM_ROUTE!(c"Left Input PGA".as_ptr(), c"LINPUT1 Switch".as_ptr(), c"LINPUT1".as_ptr()),
    DAPM_ROUTE!(c"Left Input PGA".as_ptr(), c"LINPUT2 Switch".as_ptr(), c"LINPUT2".as_ptr()),
    DAPM_ROUTE!(c"Left Input PGA".as_ptr(), c"LINPUT3 Switch".as_ptr(), c"LINPUT3".as_ptr()),
    DAPM_ROUTE!(c"Right Input PGA".as_ptr(), c"RINPUT1 Switch".as_ptr(), c"RINPUT1".as_ptr()),
    DAPM_ROUTE!(c"Right Input PGA".as_ptr(), c"RINPUT2 Switch".as_ptr(), c"RINPUT2".as_ptr()),
    DAPM_ROUTE!(c"Right Input PGA".as_ptr(), c"RINPUT3 Switch".as_ptr(), c"RINPUT3".as_ptr()),
    DAPM_ROUTE!(c"Left Input Mixer".as_ptr(), c"LINPUT2 Switch".as_ptr(), c"LINPUT2".as_ptr()),
    DAPM_ROUTE!(c"Left Input Mixer".as_ptr(), c"LINPUT3 Switch".as_ptr(), c"LINPUT3".as_ptr()),
    DAPM_ROUTE!(c"Left Input Mixer".as_ptr(), c"AUX Switch".as_ptr(), c"AUX".as_ptr()),
    DAPM_ROUTE!(c"Left Input Mixer".as_ptr(), c"Input PGA Switch".as_ptr(), c"Left Input PGA".as_ptr()),
    DAPM_ROUTE!(c"Right Input Mixer".as_ptr(), c"RINPUT2 Switch".as_ptr(), c"RINPUT2".as_ptr()),
    DAPM_ROUTE!(c"Right Input Mixer".as_ptr(), c"RINPUT3 Switch".as_ptr(), c"RINPUT3".as_ptr()),
    DAPM_ROUTE!(c"Right Input Mixer".as_ptr(), c"AUX Switch".as_ptr(), c"AUX".as_ptr()),
    DAPM_ROUTE!(c"Right Input Mixer".as_ptr(), c"Input PGA Switch".as_ptr(), c"Right Input PGA".as_ptr()),
    DAPM_ROUTE!(c"ADCL".as_ptr(), core::ptr::null(), c"Left Input Mixer".as_ptr()),
    DAPM_ROUTE!(c"ADCR".as_ptr(), core::ptr::null(), c"Right Input Mixer".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT1L".as_ptr(), core::ptr::null(), c"LINEOUT1L PGA".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT1L PGA".as_ptr(), core::ptr::null(), c"Left Output Mixer".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT1R".as_ptr(), core::ptr::null(), c"LINEOUT1R PGA".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT1R PGA".as_ptr(), core::ptr::null(), c"Right Output Mixer".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT2L PGA".as_ptr(), core::ptr::null(), c"Left Output Mixer".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT2 LP".as_ptr(), c"Disabled".as_ptr(), c"LINEOUT2L PGA".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT2 LP".as_ptr(), c"Enabled".as_ptr(), c"Left Output Mixer".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT2L".as_ptr(), core::ptr::null(), c"LINEOUT2 LP".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT2R PGA".as_ptr(), core::ptr::null(), c"Right Output Mixer".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT2 LP".as_ptr(), c"Disabled".as_ptr(), c"LINEOUT2R PGA".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT2 LP".as_ptr(), c"Enabled".as_ptr(), c"Right Output Mixer".as_ptr()),
    DAPM_ROUTE!(c"LINEOUT2R".as_ptr(), core::ptr::null(), c"LINEOUT2 LP".as_ptr()),
    DAPM_ROUTE!(c"Left Output Mixer".as_ptr(), c"LINPUT3 Bypass Switch".as_ptr(), c"LINPUT3".as_ptr()),
    DAPM_ROUTE!(c"Left Output Mixer".as_ptr(), c"AUX Bypass Switch".as_ptr(), c"AUX".as_ptr()),
    DAPM_ROUTE!(c"Left Output Mixer".as_ptr(), c"Left Input Mixer Switch".as_ptr(), c"Left Input Mixer".as_ptr()),
    DAPM_ROUTE!(c"Left Output Mixer".as_ptr(), c"Right Input Mixer Switch".as_ptr(), c"Right Input Mixer".as_ptr()),
    DAPM_ROUTE!(c"Left Output Mixer".as_ptr(), c"DACL Switch".as_ptr(), c"DACL".as_ptr()),
    DAPM_ROUTE!(c"Right Output Mixer".as_ptr(), c"RINPUT3 Bypass Switch".as_ptr(), c"RINPUT3".as_ptr()),
    DAPM_ROUTE!(c"Right Output Mixer".as_ptr(), c"AUX Bypass Switch".as_ptr(), c"AUX".as_ptr()),
    DAPM_ROUTE!(c"Right Output Mixer".as_ptr(), c"Left Input Mixer Switch".as_ptr(), c"Left Input Mixer".as_ptr()),
    DAPM_ROUTE!(c"Right Output Mixer".as_ptr(), c"Right Input Mixer Switch".as_ptr(), c"Right Input Mixer".as_ptr()),
    DAPM_ROUTE!(c"Right Output Mixer".as_ptr(), c"DACR Switch".as_ptr(), c"DACR".as_ptr()),
    /* The headphone output stage needs external LINEOUT2 DC blocking capacitors. */
    DAPM_ROUTE!(c"Headphone Amplifier".as_ptr(), core::ptr::null(), c"LINEOUT2 LP".as_ptr()),
    DAPM_ROUTE!(c"Headphone Amplifier".as_ptr(), core::ptr::null(), c"LINEOUT2 LP".as_ptr()),
    DAPM_ROUTE!(c"HP_L".as_ptr(), core::ptr::null(), c"Headphone Amplifier".as_ptr()),
    DAPM_ROUTE!(c"HP_R".as_ptr(), core::ptr::null(), c"Headphone Amplifier".as_ptr()),
];

unsafe fn wm8900_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let mut reg: u16 = (snd_soc_component_read(component, WM8900_REG_AUDIO1) & !0x60) as u16;

    match params_width(params) {
        16 => {}
        20 => reg |= 0x20,
        24 => reg |= 0x40,
        32 => reg |= 0x60,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8900_REG_AUDIO1, reg as c_uint);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        reg = snd_soc_component_read(component, WM8900_REG_DACCTRL) as u16;
        if params_rate(params) <= 24000 {
            reg |= WM8900_REG_DACCTRL_DAC_SB_FILT as u16;
        } else {
            reg &= !(WM8900_REG_DACCTRL_DAC_SB_FILT as u16);
        }
        snd_soc_component_write(component, WM8900_REG_DACCTRL, reg as c_uint);
    }

    0
}

unsafe fn fll_factors(fll_div: *mut _fll_div, Fref: c_uint, Fout: c_uint) -> c_int {
    let mut Kpart: u64;
    let mut K: c_uint;
    let Ndiv: c_uint;
    let Nmod: c_uint;
    let mut target: c_uint;
    let mut div: c_uint;

    if WARN_ON((Fout == 0) as c_int) != 0 {
        return -EINVAL;
    }

    /* The FLL must run at 90-100MHz which is then scaled down to the output value by FLLCLK_DIV. */
    target = Fout;
    div = 1;
    while target < 90000000 {
        div = div.wrapping_mul(2);
        target = target.wrapping_mul(2);
    }

    if target > 100000000 {
        printk(c"wm8900: FLL rate %u out of range, Fref=%u Fout=%u\n".as_ptr(), target, Fref, Fout);
    }
    if div > 32 {
        printk(c"wm8900: Invalid FLL division rate %u, Fref=%u, Fout=%u, target=%u\n".as_ptr(), div, Fref, Fout, target);
        return -EINVAL;
    }

    (*fll_div).fllclk_div = (div >> 2) as u16;
    (*fll_div).fll_slow_lock_ref = if Fref < 48000 { 1 } else { 0 };

    Ndiv = target / Fref;
    (*fll_div).fll_ratio = if Fref < 1000000 { 8 } else { 1 };
    (*fll_div).n = (Ndiv / (*fll_div).fll_ratio as c_uint) as u16;
    Nmod = (target / (*fll_div).fll_ratio as c_uint) % Fref;

    /* Calculate fractional part - scale up so we can round. */
    Kpart = FIXED_FLL_SIZE.wrapping_mul(Nmod as u64);
    Kpart /= Fref as u64;
    K = (Kpart & 0xFFFFFFFF) as c_uint;

    if (K % 10) >= 5 {
        K = K.wrapping_add(5);
    }

    /* Move down to proper range now rounding is done */
    (*fll_div).k = (K / 10) as u16;

    if WARN_ON((target != Fout.wrapping_mul(((*fll_div).fllclk_div as c_uint) << 2)) as c_int) != 0
        || WARN_ON((K == 0 && target != Fref.wrapping_mul((*fll_div).fll_ratio as c_uint).wrapping_mul((*fll_div).n as c_uint)) as c_int) != 0
    {
        return -EINVAL;
    }

    0
}

unsafe fn wm8900_set_fll(
    component: *mut snd_soc_component,
    _fll_id: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let wm8900 = snd_soc_component_get_drvdata(component) as *mut wm8900_priv;
    let mut fll_div = _fll_div { fll_ratio: 0, fllclk_div: 0, fll_slow_lock_ref: 0, n: 0, k: 0 };

    if (*wm8900).fll_in == freq_in && (*wm8900).fll_out == freq_out {
        return 0;
    }

    /* The digital side should be disabled during any change. */
    snd_soc_component_update_bits(component, WM8900_REG_POWER1, WM8900_REG_POWER1_FLL_ENA, 0);

    /* Disable the FLL? */
    if freq_in == 0 || freq_out == 0 {
        snd_soc_component_update_bits(component, WM8900_REG_CLOCKING1, WM8900_REG_CLOCKING1_MCLK_SRC, 0);
        snd_soc_component_update_bits(component, WM8900_REG_FLLCTL1, WM8900_REG_FLLCTL1_OSC_ENA, 0);
        (*wm8900).fll_in = freq_in;
        (*wm8900).fll_out = freq_out;
        return 0;
    }

    if fll_factors(&mut fll_div, freq_in, freq_out) == 0 {
        (*wm8900).fll_in = freq_in;
        (*wm8900).fll_out = freq_out;

        /* The osclilator *MUST* be enabled before we enable the digital circuit. */
        snd_soc_component_write(component, WM8900_REG_FLLCTL1, fll_div.fll_ratio as c_uint | WM8900_REG_FLLCTL1_OSC_ENA);
        snd_soc_component_write(component, WM8900_REG_FLLCTL4, (fll_div.n >> 5) as c_uint);
        snd_soc_component_write(component, WM8900_REG_FLLCTL5, ((fll_div.fllclk_div as c_uint) << 6) | ((fll_div.n as c_uint) & 0x1f));

        if fll_div.k != 0 {
            snd_soc_component_write(component, WM8900_REG_FLLCTL2, ((fll_div.k as c_uint) >> 8) | 0x100);
            snd_soc_component_write(component, WM8900_REG_FLLCTL3, (fll_div.k as c_uint) & 0xff);
        } else {
            snd_soc_component_write(component, WM8900_REG_FLLCTL2, 0);
        }

        if fll_div.fll_slow_lock_ref != 0 {
            snd_soc_component_write(component, WM8900_REG_FLLCTL6, WM8900_REG_FLLCTL6_FLL_SLOW_LOCK_REF);
        } else {
            snd_soc_component_write(component, WM8900_REG_FLLCTL6, 0);
        }

        snd_soc_component_update_bits(component, WM8900_REG_POWER1, WM8900_REG_POWER1_FLL_ENA, WM8900_REG_POWER1_FLL_ENA);
    }

    snd_soc_component_update_bits(component, WM8900_REG_CLOCKING1, WM8900_REG_CLOCKING1_MCLK_SRC, WM8900_REG_CLOCKING1_MCLK_SRC);
    0
}

unsafe fn wm8900_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    wm8900_set_fll((*codec_dai).component, pll_id, freq_in, freq_out)
}

unsafe fn wm8900_set_dai_clkdiv(codec_dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int {
    let component = (*codec_dai).component;
    match div_id {
        WM8900_BCLK_DIV => snd_soc_component_update_bits(component, WM8900_REG_CLOCKING1, WM8900_REG_CLOCKING1_BCLK_MASK, div as c_uint),
        WM8900_OPCLK_DIV => snd_soc_component_update_bits(component, WM8900_REG_CLOCKING1, WM8900_REG_CLOCKING1_OPCLK_MASK, div as c_uint),
        WM8900_DAC_LRCLK => snd_soc_component_update_bits(component, WM8900_REG_AUDIO4, WM8900_LRC_MASK, div as c_uint),
        WM8900_ADC_LRCLK => snd_soc_component_update_bits(component, WM8900_REG_AUDIO3, WM8900_LRC_MASK, div as c_uint),
        WM8900_DAC_CLKDIV => snd_soc_component_update_bits(component, WM8900_REG_CLOCKING2, WM8900_REG_CLOCKING2_DAC_CLKDIV, div as c_uint),
        WM8900_ADC_CLKDIV => snd_soc_component_update_bits(component, WM8900_REG_CLOCKING2, WM8900_REG_CLOCKING2_ADC_CLKDIV, div as c_uint),
        WM8900_LRCLK_MODE => snd_soc_component_update_bits(component, WM8900_REG_DACCTRL, WM8900_REG_DACCTRL_AIF_LRCLKRATE, div as c_uint),
        _ => return -EINVAL,
    };
    0
}

unsafe fn wm8900_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut clocking1 = snd_soc_component_read(component, WM8900_REG_CLOCKING1);
    let mut aif1 = snd_soc_component_read(component, WM8900_REG_AUDIO1);
    let mut aif3 = snd_soc_component_read(component, WM8900_REG_AUDIO3);
    let mut aif4 = snd_soc_component_read(component, WM8900_REG_AUDIO4);

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {
            clocking1 &= !WM8900_REG_CLOCKING1_BCLK_DIR;
            aif3 &= !WM8900_REG_AUDIO3_ADCLRC_DIR;
            aif4 &= !WM8900_REG_AUDIO4_DACLRC_DIR;
        }
        SND_SOC_DAIFMT_CBC_CFP => {
            clocking1 &= !WM8900_REG_CLOCKING1_BCLK_DIR;
            aif3 |= WM8900_REG_AUDIO3_ADCLRC_DIR;
            aif4 |= WM8900_REG_AUDIO4_DACLRC_DIR;
        }
        SND_SOC_DAIFMT_CBP_CFP => {
            clocking1 |= WM8900_REG_CLOCKING1_BCLK_DIR;
            aif3 |= WM8900_REG_AUDIO3_ADCLRC_DIR;
            aif4 |= WM8900_REG_AUDIO4_DACLRC_DIR;
        }
        SND_SOC_DAIFMT_CBP_CFC => {
            clocking1 |= WM8900_REG_CLOCKING1_BCLK_DIR;
            aif3 &= !WM8900_REG_AUDIO3_ADCLRC_DIR;
            aif4 &= !WM8900_REG_AUDIO4_DACLRC_DIR;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {
            aif1 |= WM8900_REG_AUDIO1_AIF_FMT_MASK;
            aif1 &= !WM8900_REG_AUDIO1_LRCLK_INV;
        }
        SND_SOC_DAIFMT_DSP_B => {
            aif1 |= WM8900_REG_AUDIO1_AIF_FMT_MASK;
            aif1 |= WM8900_REG_AUDIO1_LRCLK_INV;
        }
        SND_SOC_DAIFMT_I2S => {
            aif1 &= !WM8900_REG_AUDIO1_AIF_FMT_MASK;
            aif1 |= 0x10;
        }
        SND_SOC_DAIFMT_RIGHT_J => aif1 &= !WM8900_REG_AUDIO1_AIF_FMT_MASK,
        SND_SOC_DAIFMT_LEFT_J => {
            aif1 &= !WM8900_REG_AUDIO1_AIF_FMT_MASK;
            aif1 |= 0x8;
        }
        _ => return -EINVAL,
    }

    /* Clock inversion */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {
            /* frame inversion not valid for DSP modes */
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_NB_NF => aif1 &= !WM8900_REG_AUDIO1_BCLK_INV,
                SND_SOC_DAIFMT_IB_NF => aif1 |= WM8900_REG_AUDIO1_BCLK_INV,
                _ => return -EINVAL,
            }
        }
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_LEFT_J => {
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_NB_NF => {
                    aif1 &= !WM8900_REG_AUDIO1_BCLK_INV;
                    aif1 &= !WM8900_REG_AUDIO1_LRCLK_INV;
                }
                SND_SOC_DAIFMT_IB_IF => {
                    aif1 |= WM8900_REG_AUDIO1_BCLK_INV;
                    aif1 |= WM8900_REG_AUDIO1_LRCLK_INV;
                }
                SND_SOC_DAIFMT_IB_NF => {
                    aif1 |= WM8900_REG_AUDIO1_BCLK_INV;
                    aif1 &= !WM8900_REG_AUDIO1_LRCLK_INV;
                }
                SND_SOC_DAIFMT_NB_IF => {
                    aif1 &= !WM8900_REG_AUDIO1_BCLK_INV;
                    aif1 |= WM8900_REG_AUDIO1_LRCLK_INV;
                }
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8900_REG_CLOCKING1, clocking1);
    snd_soc_component_write(component, WM8900_REG_AUDIO1, aif1);
    snd_soc_component_write(component, WM8900_REG_AUDIO3, aif3);
    snd_soc_component_write(component, WM8900_REG_AUDIO4, aif4);
    0
}

unsafe fn wm8900_mute(codec_dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: u16 = snd_soc_component_read(component, WM8900_REG_DACCTRL) as u16;
    if mute != 0 {
        reg |= WM8900_REG_DACCTRL_MUTE as u16;
    } else {
        reg &= !(WM8900_REG_DACCTRL_MUTE as u16);
    }
    snd_soc_component_write(component, WM8900_REG_DACCTRL, reg as c_uint);
    0
}

const WM8900_RATES: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_11025 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
const WM8900_PCM_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static wm8900_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8900_hw_params),
    set_clkdiv: Some(wm8900_set_dai_clkdiv),
    set_pll: Some(wm8900_set_dai_pll),
    set_fmt: Some(wm8900_set_dai_fmt),
    mute_stream: Some(wm8900_mute),
    no_capture_mute: 1,
};

static mut wm8900_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"wm8900-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: WM8900_RATES,
        formats: WM8900_PCM_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"HiFi Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: WM8900_RATES,
        formats: WM8900_PCM_FORMATS,
    },
    ops: &wm8900_dai_ops,
};

unsafe fn wm8900_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let mut reg: u16;

    match level {
        SND_SOC_BIAS_ON => {
            /* Enable thermal shutdown */
            snd_soc_component_update_bits(component, WM8900_REG_GPIO, WM8900_REG_GPIO_TEMP_ENA, WM8900_REG_GPIO_TEMP_ENA);
            snd_soc_component_update_bits(component, WM8900_REG_ADDCTL, WM8900_REG_ADDCTL_TEMP_SD, WM8900_REG_ADDCTL_TEMP_SD);
        }
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            /* Charge capacitors if initial power up */
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                /* STARTUP_BIAS_ENA on */
                snd_soc_component_write(component, WM8900_REG_POWER1, WM8900_REG_POWER1_STARTUP_BIAS_ENA);
                /* Startup bias mode */
                snd_soc_component_write(component, WM8900_REG_ADDCTL, WM8900_REG_ADDCTL_BIAS_SRC | WM8900_REG_ADDCTL_VMID_SOFTST);
                /* VMID 2x50k */
                snd_soc_component_write(component, WM8900_REG_POWER1, WM8900_REG_POWER1_STARTUP_BIAS_ENA | 0x1);
                /* Allow capacitors to charge */
                schedule_timeout_interruptible(msecs_to_jiffies(400));
                /* Enable bias */
                snd_soc_component_write(component, WM8900_REG_POWER1, WM8900_REG_POWER1_STARTUP_BIAS_ENA | WM8900_REG_POWER1_BIAS_ENA | 0x1);
                snd_soc_component_write(component, WM8900_REG_ADDCTL, 0);
                snd_soc_component_write(component, WM8900_REG_POWER1, WM8900_REG_POWER1_BIAS_ENA | 0x1);
            }

            reg = snd_soc_component_read(component, WM8900_REG_POWER1) as u16;
            snd_soc_component_write(component, WM8900_REG_POWER1, ((reg as c_uint) & WM8900_REG_POWER1_FLL_ENA) | WM8900_REG_POWER1_BIAS_ENA | 0x1);
            snd_soc_component_write(component, WM8900_REG_POWER2, WM8900_REG_POWER2_SYSCLK_ENA);
            snd_soc_component_write(component, WM8900_REG_POWER3, 0);
        }
        SND_SOC_BIAS_OFF => {
            /* Startup bias enable */
            reg = snd_soc_component_read(component, WM8900_REG_POWER1) as u16;
            snd_soc_component_write(component, WM8900_REG_POWER1, (reg as c_uint) & WM8900_REG_POWER1_STARTUP_BIAS_ENA);
            snd_soc_component_write(component, WM8900_REG_ADDCTL, WM8900_REG_ADDCTL_BIAS_SRC | WM8900_REG_ADDCTL_VMID_SOFTST);
            /* Discharge caps */
            snd_soc_component_write(component, WM8900_REG_POWER1, WM8900_REG_POWER1_STARTUP_BIAS_ENA);
            schedule_timeout_interruptible(msecs_to_jiffies(500));
            /* Remove clamp */
            snd_soc_component_write(component, WM8900_REG_HPCTL1, 0);
            /* Power down */
            snd_soc_component_write(component, WM8900_REG_ADDCTL, 0);
            snd_soc_component_write(component, WM8900_REG_POWER1, 0);
            snd_soc_component_write(component, WM8900_REG_POWER2, 0);
            snd_soc_component_write(component, WM8900_REG_POWER3, 0);
            /* Need to let things settle before stopping the clock to ensure that restart works. */
            schedule_timeout_interruptible(msecs_to_jiffies(1));
            snd_soc_component_write(component, WM8900_REG_POWER2, WM8900_REG_POWER2_SYSCLK_ENA);
        }
    }
    0
}

unsafe fn wm8900_suspend(component: *mut snd_soc_component) -> c_int {
    let wm8900 = snd_soc_component_get_drvdata(component) as *mut wm8900_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let fll_out: c_int = (*wm8900).fll_out as c_int;
    let fll_in: c_int = (*wm8900).fll_in as c_int;
    let ret = wm8900_set_fll(component, 0, 0, 0);
    if ret != 0 {
        dev_err((*component).dev, c"Failed to stop FLL\n".as_ptr());
        return ret;
    }
    (*wm8900).fll_out = fll_out as u32;
    (*wm8900).fll_in = fll_in as u32;
    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_OFF);
    0
}

unsafe fn wm8900_resume(component: *mut snd_soc_component) -> c_int {
    let wm8900 = snd_soc_component_get_drvdata(component) as *mut wm8900_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    wm8900_reset(component);
    ret = regcache_sync((*wm8900).regmap);
    if ret != 0 {
        dev_err((*component).dev, c"Failed to restore cache: %d\n".as_ptr(), ret);
        return ret;
    }
    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);

    /* Restart the FLL? */
    if (*wm8900).fll_out != 0 {
        let fll_out = (*wm8900).fll_out;
        let fll_in = (*wm8900).fll_in;
        (*wm8900).fll_in = 0;
        (*wm8900).fll_out = 0;
        ret = wm8900_set_fll(component, 0, fll_in, fll_out);
        if ret != 0 {
            dev_err((*component).dev, c"Failed to restart FLL\n".as_ptr());
            return ret;
        }
    }
    0
}

unsafe fn wm8900_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let reg = snd_soc_component_read(component, WM8900_REG_ID) as c_int;
    if reg != 0x8900 {
        dev_err((*component).dev, c"Device is not a WM8900 - ID %x\n".as_ptr(), reg);
        return -ENODEV;
    }

    wm8900_reset(component);
    /* Turn the chip on */
    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);

    /* Latch the volume update bits */
    snd_soc_component_update_bits(component, WM8900_REG_LINVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_RINVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_LOUT1CTL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_ROUT1CTL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_LOUT2CTL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_ROUT2CTL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_LDAC_DV, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_RDAC_DV, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_LADC_DV, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8900_REG_RADC_DV, 0x100, 0x100);

    /* Set the DAC and mixer output bias */
    snd_soc_component_write(component, WM8900_REG_OUTBIASCTL, 0x81);
    0
}

static soc_component_dev_wm8900: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8900_probe),
    suspend: Some(wm8900_suspend),
    resume: Some(wm8900_resume),
    set_bias_level: Some(wm8900_set_bias_level),
    controls: wm8900_snd_controls.as_ptr(),
    num_controls: wm8900_snd_controls.len() as c_uint,
    dapm_widgets: wm8900_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8900_dapm_widgets.len() as c_uint,
    dapm_routes: wm8900_dapm_routes.as_ptr(),
    num_dapm_routes: wm8900_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8900_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: WM8900_MAXREG,
    reg_defaults: wm8900_reg_defaults.as_ptr(),
    num_reg_defaults: wm8900_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(wm8900_volatile_register),
};

/* CONFIG_SPI_MASTER */
unsafe fn wm8900_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8900 = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<wm8900_priv>(), GFP_KERNEL) as *mut wm8900_priv;
    if wm8900.is_null() {
        return -ENOMEM;
    }
    (*wm8900).regmap = devm_regmap_init_spi(spi, &wm8900_regmap);
    if IS_ERR((*wm8900).regmap as *const c_void) {
        return PTR_ERR((*wm8900).regmap as *const c_void);
    }
    spi_set_drvdata(spi, wm8900 as *mut c_void);
    devm_snd_soc_register_component(&mut (*spi).dev, &soc_component_dev_wm8900, &mut wm8900_dai, 1)
}

static mut wm8900_spi_driver: spi_driver = spi_driver {
    driver: device_driver { name: c"wm8900".as_ptr() },
    probe: Some(wm8900_spi_probe),
};

/* IS_ENABLED(CONFIG_I2C) */
unsafe fn wm8900_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8900 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8900_priv>(), GFP_KERNEL) as *mut wm8900_priv;
    if wm8900.is_null() {
        return -ENOMEM;
    }
    (*wm8900).regmap = devm_regmap_init_i2c(i2c, &wm8900_regmap);
    if IS_ERR((*wm8900).regmap as *const c_void) {
        return PTR_ERR((*wm8900).regmap as *const c_void);
    }
    i2c_set_clientdata(i2c, wm8900 as *mut c_void);
    devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_wm8900, &mut wm8900_dai, 1)
}

static wm8900_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"wm8900".as_ptr() },
    i2c_device_id { name: core::ptr::null() },
];
MODULE_DEVICE_TABLE!(i2c, wm8900_i2c_id);

static mut wm8900_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver { name: c"wm8900".as_ptr() },
    probe: Some(wm8900_i2c_probe),
    id_table: wm8900_i2c_id.as_ptr(),
};

unsafe fn wm8900_modinit() -> c_int {
    let mut ret: c_int = 0;
    /* IS_ENABLED(CONFIG_I2C) */
    ret = i2c_add_driver(&mut wm8900_i2c_driver);
    if ret != 0 {
        printk(c"Failed to register wm8900 I2C driver: %d\n".as_ptr(), ret);
    }
    /* CONFIG_SPI_MASTER */
    ret = spi_register_driver(&mut wm8900_spi_driver);
    if ret != 0 {
        printk(c"Failed to register wm8900 SPI driver: %d\n".as_ptr(), ret);
    }
    ret
}
module_init!(wm8900_modinit);

unsafe fn wm8900_exit() {
    /* IS_ENABLED(CONFIG_I2C) */
    i2c_del_driver(&mut wm8900_i2c_driver);
    /* CONFIG_SPI_MASTER */
    spi_unregister_driver(&mut wm8900_spi_driver);
}
module_exit!(wm8900_exit);

MODULE_DESCRIPTION!(c"ASoC WM8900 driver".as_ptr());
MODULE_AUTHOR!(c"Mark Brown <broonie@opensource.wolfonmicro.com>".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
