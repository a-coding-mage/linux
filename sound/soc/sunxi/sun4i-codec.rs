// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2014 Emilio López <emilio@elopez.com.ar>
 * Copyright 2014 Jon Smirl <jonsmirl@gmail.com>
 * Copyright 2015 Maxime Ripard <maxime.ripard@free-electrons.com>
 * Copyright 2015 Adam Sampson <ats@offog.org>
 * Copyright 2016 Chen-Yu Tsai <wens@csie.org>
 * Copyright 2018 Mesih Kilinc <mesihkilinc@gmail.com>
 *
 * Based on the Allwinner SDK driver, released under the GPL.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

/* Codec DAC digital controls and FIFO registers */
const SUN4I_CODEC_DAC_DPC: c_uint = 0x00;
const SUN4I_CODEC_DAC_DPC_EN_DA: u32 = 31;
const SUN4I_CODEC_DAC_DPC_DVOL: u32 = 12;
const SUN4I_CODEC_DAC_FIFOC: c_uint = 0x04;
const SUN4I_CODEC_DAC_FIFOC_DAC_FS: u32 = 29;
const SUN4I_CODEC_DAC_FIFOC_FIR_VERSION: u32 = 28;
const SUN4I_CODEC_DAC_FIFOC_SEND_LASAT: u32 = 26;
const SUN4I_CODEC_DAC_FIFOC_TX_FIFO_MODE: u32 = 24;
const SUN4I_CODEC_DAC_FIFOC_DRQ_CLR_CNT: u32 = 21;
const SUN4I_CODEC_DAC_FIFOC_TX_TRIG_LEVEL: u32 = 8;
const SUN4I_CODEC_DAC_FIFOC_MONO_EN: u32 = 6;
const SUN4I_CODEC_DAC_FIFOC_TX_SAMPLE_BITS: u32 = 5;
const SUN4I_CODEC_DAC_FIFOC_DAC_DRQ_EN: u32 = 4;
const SUN4I_CODEC_DAC_FIFOC_FIFO_FLUSH: u32 = 0;
const SUN4I_CODEC_DAC_FIFOS: c_uint = 0x08;
const SUN4I_CODEC_DAC_TXDATA: c_uint = 0x0c;

/* Codec DAC side analog signal controls */
const SUN4I_CODEC_DAC_ACTL: c_uint = 0x10;
const SUN4I_CODEC_DAC_ACTL_DACAENR: u32 = 31;
const SUN4I_CODEC_DAC_ACTL_DACAENL: u32 = 30;
const SUN4I_CODEC_DAC_ACTL_MIXEN: u32 = 29;
const SUN4I_CODEC_DAC_ACTL_LNG: u32 = 26;
const SUN4I_CODEC_DAC_ACTL_FMG: u32 = 23;
const SUN4I_CODEC_DAC_ACTL_MICG: u32 = 20;
const SUN4I_CODEC_DAC_ACTL_LLNS: u32 = 19;
const SUN4I_CODEC_DAC_ACTL_RLNS: u32 = 18;
const SUN4I_CODEC_DAC_ACTL_LFMS: u32 = 17;
const SUN4I_CODEC_DAC_ACTL_RFMS: u32 = 16;
const SUN4I_CODEC_DAC_ACTL_LDACLMIXS: u32 = 15;
const SUN4I_CODEC_DAC_ACTL_RDACRMIXS: u32 = 14;
const SUN4I_CODEC_DAC_ACTL_LDACRMIXS: u32 = 13;
const SUN4I_CODEC_DAC_ACTL_MIC1LS: u32 = 12;
const SUN4I_CODEC_DAC_ACTL_MIC1RS: u32 = 11;
const SUN4I_CODEC_DAC_ACTL_MIC2LS: u32 = 10;
const SUN4I_CODEC_DAC_ACTL_MIC2RS: u32 = 9;
const SUN4I_CODEC_DAC_ACTL_DACPAS: u32 = 8;
const SUN4I_CODEC_DAC_ACTL_MIXPAS: u32 = 7;
const SUN4I_CODEC_DAC_ACTL_PA_MUTE: u32 = 6;
const SUN4I_CODEC_DAC_ACTL_PA_VOL: u32 = 0;
const SUN4I_CODEC_DAC_TUNE: c_uint = 0x14;
const SUN4I_CODEC_DAC_DEBUG: c_uint = 0x18;

/* Codec ADC digital controls and FIFO registers */
const SUN4I_CODEC_ADC_FIFOC: c_uint = 0x1c;
const SUN4I_CODEC_ADC_FIFOC_ADC_FS: u32 = 29;
const SUN4I_CODEC_ADC_FIFOC_EN_AD: u32 = 28;
const SUN4I_CODEC_ADC_FIFOC_RX_FIFO_MODE: u32 = 24;
const SUN4I_CODEC_ADC_FIFOC_RX_TRIG_LEVEL: u32 = 8;
const SUN4I_CODEC_ADC_FIFOC_MONO_EN: u32 = 7;
const SUN4I_CODEC_ADC_FIFOC_RX_SAMPLE_BITS: u32 = 6;
const SUN4I_CODEC_ADC_FIFOC_ADC_DRQ_EN: u32 = 4;
const SUN4I_CODEC_ADC_FIFOC_FIFO_FLUSH: u32 = 0;
const SUN4I_CODEC_ADC_FIFOS: c_uint = 0x20;
const SUN4I_CODEC_ADC_RXDATA: c_uint = 0x24;

/* Codec ADC side analog signal controls */
const SUN4I_CODEC_ADC_ACTL: c_uint = 0x28;
const SUN4I_CODEC_ADC_ACTL_ADC_R_EN: u32 = 31;
const SUN4I_CODEC_ADC_ACTL_ADC_L_EN: u32 = 30;
const SUN4I_CODEC_ADC_ACTL_PREG1EN: u32 = 29;
const SUN4I_CODEC_ADC_ACTL_PREG2EN: u32 = 28;
const SUN4I_CODEC_ADC_ACTL_VMICEN: u32 = 27;
const SUN4I_CODEC_ADC_ACTL_PREG1: u32 = 25;
const SUN4I_CODEC_ADC_ACTL_PREG2: u32 = 23;
const SUN4I_CODEC_ADC_ACTL_VADCG: u32 = 20;
const SUN4I_CODEC_ADC_ACTL_ADCIS: u32 = 17;
const SUN4I_CODEC_ADC_ACTL_LNPREG: u32 = 13;
const SUN4I_CODEC_ADC_ACTL_PA_EN: u32 = 4;
const SUN4I_CODEC_ADC_ACTL_DDE: u32 = 3;
const SUN4I_CODEC_ADC_DEBUG: c_uint = 0x2c;

/* FIFO counters */
const SUN4I_CODEC_DAC_TXCNT: c_uint = 0x30;
const SUN4I_CODEC_ADC_RXCNT: c_uint = 0x34;

/* Calibration register (sun7i only) */
const SUN7I_CODEC_AC_DAC_CAL: c_uint = 0x38;

/* Microphone controls (sun7i only) */
const SUN7I_CODEC_AC_MIC_PHONE_CAL: c_uint = 0x3c;
const SUN7I_CODEC_AC_MIC_PHONE_CAL_PREG1: u32 = 29;
const SUN7I_CODEC_AC_MIC_PHONE_CAL_PREG2: u32 = 26;

/*
 * sun6i specific registers
 *
 * sun6i shares the same digital control and FIFO registers as sun4i,
 * but only the DAC digital controls are at the same offset. The others
 * have been moved around to accommodate extra analog controls.
 */
const SUN6I_CODEC_ADC_FIFOC: c_uint = 0x10;
const SUN6I_CODEC_ADC_FIFOC_EN_AD: u32 = 28;
const SUN6I_CODEC_ADC_FIFOS: c_uint = 0x14;
const SUN6I_CODEC_ADC_RXDATA: c_uint = 0x18;

const SUN6I_CODEC_OM_DACA_CTRL: c_uint = 0x20;
const SUN6I_CODEC_OM_DACA_CTRL_DACAREN: u32 = 31;
const SUN6I_CODEC_OM_DACA_CTRL_DACALEN: u32 = 30;
const SUN6I_CODEC_OM_DACA_CTRL_RMIXEN: u32 = 29;
const SUN6I_CODEC_OM_DACA_CTRL_LMIXEN: u32 = 28;
const SUN6I_CODEC_OM_DACA_CTRL_RMIX_MIC1: u32 = 23;
const SUN6I_CODEC_OM_DACA_CTRL_RMIX_MIC2: u32 = 22;
const SUN6I_CODEC_OM_DACA_CTRL_RMIX_PHONE: u32 = 21;
const SUN6I_CODEC_OM_DACA_CTRL_RMIX_PHONEP: u32 = 20;
const SUN6I_CODEC_OM_DACA_CTRL_RMIX_LINEINR: u32 = 19;
const SUN6I_CODEC_OM_DACA_CTRL_RMIX_DACR: u32 = 18;
const SUN6I_CODEC_OM_DACA_CTRL_RMIX_DACL: u32 = 17;
const SUN6I_CODEC_OM_DACA_CTRL_LMIX_MIC1: u32 = 16;
const SUN6I_CODEC_OM_DACA_CTRL_LMIX_MIC2: u32 = 15;
const SUN6I_CODEC_OM_DACA_CTRL_LMIX_PHONE: u32 = 14;
const SUN6I_CODEC_OM_DACA_CTRL_LMIX_PHONEN: u32 = 13;
const SUN6I_CODEC_OM_DACA_CTRL_LMIX_LINEINL: u32 = 12;
const SUN6I_CODEC_OM_DACA_CTRL_LMIX_DACL: u32 = 11;
const SUN6I_CODEC_OM_DACA_CTRL_LMIX_DACR: u32 = 10;
const SUN6I_CODEC_OM_DACA_CTRL_RHPIS: u32 = 9;
const SUN6I_CODEC_OM_DACA_CTRL_LHPIS: u32 = 8;
const SUN6I_CODEC_OM_DACA_CTRL_RHPPAMUTE: u32 = 7;
const SUN6I_CODEC_OM_DACA_CTRL_LHPPAMUTE: u32 = 6;
const SUN6I_CODEC_OM_DACA_CTRL_HPVOL: u32 = 0;
const SUN6I_CODEC_OM_PA_CTRL: c_uint = 0x24;
const SUN6I_CODEC_OM_PA_CTRL_HPPAEN: u32 = 31;
const SUN6I_CODEC_OM_PA_CTRL_HPCOM_CTL: u32 = 29;
const SUN6I_CODEC_OM_PA_CTRL_COMPTEN: u32 = 28;
const SUN6I_CODEC_OM_PA_CTRL_MIC1G: u32 = 15;
const SUN6I_CODEC_OM_PA_CTRL_MIC2G: u32 = 12;
const SUN6I_CODEC_OM_PA_CTRL_LINEING: u32 = 9;
const SUN6I_CODEC_OM_PA_CTRL_PHONEG: u32 = 6;
const SUN6I_CODEC_OM_PA_CTRL_PHONEPG: u32 = 3;
const SUN6I_CODEC_OM_PA_CTRL_PHONENG: u32 = 0;

const SUN6I_CODEC_MIC_CTRL: c_uint = 0x28;
const SUN6I_CODEC_MIC_CTRL_HBIASEN: u32 = 31;
const SUN6I_CODEC_MIC_CTRL_MBIASEN: u32 = 30;
const SUN6I_CODEC_MIC_CTRL_MIC1AMPEN: u32 = 28;
const SUN6I_CODEC_MIC_CTRL_MIC1BOOST: u32 = 25;
const SUN6I_CODEC_MIC_CTRL_MIC2AMPEN: u32 = 24;
const SUN6I_CODEC_MIC_CTRL_MIC2BOOST: u32 = 21;
const SUN6I_CODEC_MIC_CTRL_MIC2SLT: u32 = 20;
const SUN6I_CODEC_MIC_CTRL_LINEOUTLEN: u32 = 19;
const SUN6I_CODEC_MIC_CTRL_LINEOUTREN: u32 = 18;
const SUN6I_CODEC_MIC_CTRL_LINEOUTLSRC: u32 = 17;
const SUN6I_CODEC_MIC_CTRL_LINEOUTRSRC: u32 = 16;
const SUN6I_CODEC_MIC_CTRL_LINEOUTVC: u32 = 11;
const SUN6I_CODEC_MIC_CTRL_PHONEPREG: u32 = 8;

const SUN6I_CODEC_ADC_ACTL: c_uint = 0x2c;
const SUN6I_CODEC_ADC_ACTL_ADCREN: u32 = 31;
const SUN6I_CODEC_ADC_ACTL_ADCLEN: u32 = 30;
const SUN6I_CODEC_ADC_ACTL_ADCRG: u32 = 27;
const SUN6I_CODEC_ADC_ACTL_ADCLG: u32 = 24;
const SUN6I_CODEC_ADC_ACTL_RADCMIX_MIC1: u32 = 13;
const SUN6I_CODEC_ADC_ACTL_RADCMIX_MIC2: u32 = 12;
const SUN6I_CODEC_ADC_ACTL_RADCMIX_PHONE: u32 = 11;
const SUN6I_CODEC_ADC_ACTL_RADCMIX_PHONEP: u32 = 10;
const SUN6I_CODEC_ADC_ACTL_RADCMIX_LINEINR: u32 = 9;
const SUN6I_CODEC_ADC_ACTL_RADCMIX_OMIXR: u32 = 8;
const SUN6I_CODEC_ADC_ACTL_RADCMIX_OMIXL: u32 = 7;
const SUN6I_CODEC_ADC_ACTL_LADCMIX_MIC1: u32 = 6;
const SUN6I_CODEC_ADC_ACTL_LADCMIX_MIC2: u32 = 5;
const SUN6I_CODEC_ADC_ACTL_LADCMIX_PHONE: u32 = 4;
const SUN6I_CODEC_ADC_ACTL_LADCMIX_PHONEN: u32 = 3;
const SUN6I_CODEC_ADC_ACTL_LADCMIX_LINEINL: u32 = 2;
const SUN6I_CODEC_ADC_ACTL_LADCMIX_OMIXL: u32 = 1;
const SUN6I_CODEC_ADC_ACTL_LADCMIX_OMIXR: u32 = 0;

const SUN6I_CODEC_ADDA_TUNE: c_uint = 0x30;
const SUN6I_CODEC_CALIBRATION: c_uint = 0x34;
const SUN6I_CODEC_DAC_TXCNT: c_uint = 0x40;
const SUN6I_CODEC_ADC_RXCNT: c_uint = 0x44;
const SUN6I_CODEC_HMIC_CTL: c_uint = 0x50;
const SUN6I_CODEC_HMIC_DATA: c_uint = 0x54;

/* TODO sun6i DAP (Digital Audio Processing) bits */
const SUN8I_A23_CODEC_DAC_TXCNT: c_uint = 0x1c;
const SUN8I_A23_CODEC_ADC_RXCNT: c_uint = 0x20;
const SUN8I_H3_CODEC_DAC_TXDATA: c_uint = 0x20;
const SUN8I_H3_CODEC_DAC_DBG: c_uint = 0x48;
const SUN8I_H3_CODEC_ADC_DBG: c_uint = 0x4c;

const SUN50I_H616_CODEC_DAC_FIFOC: c_uint = 0x10;
const SUN50I_DAC_FIFO_STA: c_uint = 0x14;
const SUN50I_DAC_TXE_INT: u32 = 3;
const SUN50I_DAC_TXU_INT: u32 = 2;
const SUN50I_DAC_TXO_INT: u32 = 1;
const SUN50I_DAC_CNT: c_uint = 0x24;
const SUN50I_DAC_DG_REG: c_uint = 0x28;
const SUN50I_DAC_DAP_CTL: c_uint = 0xf0;
const SUN50I_H616_DAC_AC_DAC_REG: c_uint = 0x310;
const SUN50I_H616_DAC_LEN: u32 = 15;
const SUN50I_H616_DAC_REN: u32 = 14;
const SUN50I_H616_LINEOUTL_EN: u32 = 13;
const SUN50I_H616_LMUTE: u32 = 12;
const SUN50I_H616_LINEOUTR_EN: u32 = 11;
const SUN50I_H616_RMUTE: u32 = 10;
const SUN50I_H616_RSWITCH: u32 = 9;
const SUN50I_H616_RAMPEN: u32 = 8;
const SUN50I_H616_LINEOUTL_SEL: u32 = 6;
const SUN50I_H616_LINEOUTR_SEL: u32 = 5;
const SUN50I_H616_LINEOUT_VOL: u32 = 0;
const SUN50I_H616_DAC_AC_MIXER_REG: c_uint = 0x314;
const SUN50I_H616_LMIX_LDAC: u32 = 21;
const SUN50I_H616_LMIX_RDAC: u32 = 20;
const SUN50I_H616_RMIX_RDAC: u32 = 17;
const SUN50I_H616_RMIX_LDAC: u32 = 16;
const SUN50I_H616_LMIXEN: u32 = 11;
const SUN50I_H616_RMIXEN: u32 = 10;
const SUN50I_H616_DAC_AC_RAMP_REG: c_uint = 0x31c;
const SUN50I_H616_RAMP_STEP: u32 = 4;
const SUN50I_H616_RDEN: u32 = 0;
/* TODO H3 DAP (Digital Audio Processing) bits */

const SUN4I_DMA_MAX_BURST: u32 = 8;
const SUNIV_DMA_MAX_BURST: u32 = 4;

const SUNIV_CODEC_ADC_FIFOC: c_uint = 0x10;
const SUNIV_CODEC_ADC_FIFOC_EN_AD: u32 = 28;
const SUNIV_CODEC_ADC_FIFOS: c_uint = 0x14;
const SUNIV_CODEC_ADC_RXDATA: c_uint = 0x18;
const SUNIV_CODEC_OM_DACA_CTRL: c_uint = 0x20;
const SUNIV_CODEC_OM_DACA_CTRL_DACAREN: u32 = 31;
const SUNIV_CODEC_OM_DACA_CTRL_DACALEN: u32 = 30;
const SUNIV_CODEC_OM_DACA_CTRL_RMIXEN: u32 = 29;
const SUNIV_CODEC_OM_DACA_CTRL_LMIXEN: u32 = 28;
const SUNIV_CODEC_OM_DACA_CTRL_RHPPAMUTE: u32 = 27;
const SUNIV_CODEC_OM_DACA_CTRL_LHPPAMUTE: u32 = 26;
const SUNIV_CODEC_OM_DACA_CTRL_RHPIS: u32 = 25;
const SUNIV_CODEC_OM_DACA_CTRL_LHPIS: u32 = 24;
const SUNIV_CODEC_OM_DACA_CTRL_HPCOM_CTL: u32 = 22;
const SUNIV_CODEC_OM_DACA_CTRL_COMPTEN: u32 = 21;
const SUNIV_CODEC_OM_DACA_CTRL_RMIXMUTE_MICIN: u32 = 20;
const SUNIV_CODEC_OM_DACA_CTRL_RMIXMUTE_LINEIN: u32 = 19;
const SUNIV_CODEC_OM_DACA_CTRL_RMIXMUTE_FMIN: u32 = 18;
const SUNIV_CODEC_OM_DACA_CTRL_RMIXMUTE_RDAC: u32 = 17;
const SUNIV_CODEC_OM_DACA_CTRL_RMIXMUTE_LDAC: u32 = 16;
const SUNIV_CODEC_OM_DACA_CTRL_HPPAEN: u32 = 15;
const SUNIV_CODEC_OM_DACA_CTRL_LMIXMUTE_MICIN: u32 = 12;
const SUNIV_CODEC_OM_DACA_CTRL_LMIXMUTE_LINEIN: u32 = 11;
const SUNIV_CODEC_OM_DACA_CTRL_LMIXMUTE_FMIN: u32 = 10;
const SUNIV_CODEC_OM_DACA_CTRL_LMIXMUTE_LDAC: u32 = 9;
const SUNIV_CODEC_OM_DACA_CTRL_LMIXMUTE_RDAC: u32 = 8;
const SUNIV_CODEC_OM_DACA_CTRL_LTLNMUTE: u32 = 7;
const SUNIV_CODEC_OM_DACA_CTRL_RTLNMUTE: u32 = 6;
const SUNIV_CODEC_OM_DACA_CTRL_HPVOL: u32 = 0;

const SUNIV_CODEC_ADC_ACTL: c_uint = 0x24;
const SUNIV_CODEC_ADC_ADCEN: u32 = 31;
const SUNIV_CODEC_ADC_MICG: u32 = 24;
const SUNIV_CODEC_ADC_LINEINVOL: u32 = 21;
const SUNIV_CODEC_ADC_ADCG: u32 = 16;
const SUNIV_CODEC_ADC_ADCMIX_MIC: u32 = 13;
const SUNIV_CODEC_ADC_ADCMIX_FMINL: u32 = 12;
const SUNIV_CODEC_ADC_ADCMIX_FMINR: u32 = 11;
const SUNIV_CODEC_ADC_ADCMIX_LINEIN: u32 = 10;
const SUNIV_CODEC_ADC_ADCMIX_LOUT: u32 = 9;
const SUNIV_CODEC_ADC_ADCMIX_ROUT: u32 = 8;
const SUNIV_CODEC_ADC_PASPEEDSELECT: u32 = 7;
const SUNIV_CODEC_ADC_FMINVOL: u32 = 4;
const SUNIV_CODEC_ADC_MICAMPEN: u32 = 3;
const SUNIV_CODEC_ADC_MICBOOST: u32 = 0;
const SUNIV_CODEC_ADC_DBG: c_uint = 0x4c;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap_field {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
}
#[repr(C)]
pub struct snd_soc_card {
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub name: *const c_char,
    pub long_name: *const c_char,
    pub driver_name: *const c_char,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub aux_dev: *mut snd_soc_aux_dev,
    pub num_aux_devs: c_int,
    pub fully_routed: bool,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
    pub sig_bits: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub ops: *const snd_soc_dai_ops,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}
#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: c_ulong,
    pub addr_width: c_uint,
    pub maxburst: u32,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
    pub legacy_dai_naming: c_uint,
    /* CONFIG_DEBUG_FS: debugfs_prefix = "cpu" */
}
#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}
#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_uint,
    pub debounce_time: c_uint,
    pub desc: *mut gpio_desc,
}
#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub num_codecs: c_uint,
    pub num_platforms: c_uint,
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub playback_only: bool,
    pub capture_only: bool,
}
#[repr(C)]
pub struct snd_soc_aux_dev {
    pub dlc: snd_soc_dai_link_component,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub cache_type: c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_field {
    pub reg: c_uint,
    pub lsb: c_uint,
    pub msb: c_uint,
}
const fn REG_FIELD(reg: c_uint, lsb: c_uint, msb: c_uint) -> reg_field {
    reg_field { reg, lsb, msb }
}

#[repr(C)]
pub struct sun4i_codec {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub clk_apb: *mut clk,
    pub clk_module: *mut clk,
    pub rst: *mut reset_control,
    pub gpio_pa: *mut gpio_desc,
    pub gpio_hp: *mut gpio_desc,
    /* ADC_FIFOC register is at different offset on different SoCs */
    pub reg_adc_fifoc: *mut regmap_field,
    /* DAC_FIFOC register is at different offset on different SoCs */
    pub reg_dac_fifoc: *mut regmap_field,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
pub struct sun4i_codec_quirks {
    pub regmap_config: *const regmap_config,
    pub codec: *const snd_soc_component_driver,
    pub create_card: Option<unsafe extern "C" fn(*mut device) -> *mut snd_soc_card>,
    pub reg_adc_fifoc: reg_field, /* used for regmap_field */
    pub reg_dac_fifoc: reg_field, /* used for regmap_field */
    pub reg_dac_txdata: c_uint,   /* TX FIFO offset for DMA config */
    pub reg_adc_rxdata: c_uint,   /* RX FIFO offset for DMA config */
    pub has_reset: bool,
    pub playback_only: bool,
    pub dma_max_burst: u32,
}

extern "C" {
    fn regmap_field_set_bits(field: *mut regmap_field, bits: c_uint);
    fn regmap_field_clear_bits(field: *mut regmap_field, bits: c_uint);
    fn regmap_field_update_bits(field: *mut regmap_field, mask: c_uint, val: c_uint);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn of_device_is_compatible(node: *mut device_node, compat: *const c_char) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, param: c_int) -> *mut snd_interval;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_jack_add_gpios(jack: *mut snd_soc_jack, count: c_int, gpios: *mut snd_soc_jack_gpio) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn msleep(msecs: c_uint);
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_reset_control_get_exclusive_deasserted(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_regmap_field_alloc(dev: *mut device, regmap: *mut regmap, field: reg_field) -> *mut regmap_field;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
    fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

extern "Rust" {
    static mut THIS_MODULE: *mut c_void;
}

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_int = 0;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint = 4;
const DMA_SLAVE_BUSWIDTH_2_BYTES: c_uint = 2;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const GPIOD_IN: c_uint = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_JACK_HEADPHONE: c_uint = 0x0001;
const REGCACHE_NONE: c_uint = 0;

const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
const SNDRV_PCM_RATE_12000: c_uint = 0;
const SNDRV_PCM_RATE_24000: c_uint = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 0;
const SUN4I_CODEC_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_12000
    | SNDRV_PCM_RATE_24000
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_192000;
const SUN4I_CODEC_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

unsafe extern "C" fn sun4i_codec_start_playback(scodec: *mut sun4i_codec) {
    /* Flush TX FIFO */
    regmap_field_set_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_FIFO_FLUSH));
    /* Enable DAC DRQ */
    regmap_field_set_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_DAC_DRQ_EN));
}

unsafe extern "C" fn sun4i_codec_stop_playback(scodec: *mut sun4i_codec) {
    /* Disable DAC DRQ */
    regmap_field_clear_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_DAC_DRQ_EN));
}

unsafe extern "C" fn sun4i_codec_start_capture(scodec: *mut sun4i_codec) {
    /* Enable ADC DRQ */
    regmap_field_set_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_ADC_DRQ_EN));
}

unsafe extern "C" fn sun4i_codec_stop_capture(scodec: *mut sun4i_codec) {
    /* Disable ADC DRQ */
    regmap_field_clear_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_ADC_DRQ_EN));
}

unsafe extern "C" fn sun4i_codec_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let scodec = snd_soc_card_get_drvdata((*rtd).card) as *mut sun4i_codec;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                sun4i_codec_start_playback(scodec);
            } else {
                sun4i_codec_start_capture(scodec);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                sun4i_codec_stop_playback(scodec);
            } else {
                sun4i_codec_stop_capture(scodec);
            }
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn sun4i_codec_prepare_capture(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let scodec = snd_soc_card_get_drvdata((*rtd).card) as *mut sun4i_codec;

    /* Flush RX FIFO */
    regmap_field_set_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_FIFO_FLUSH));

    /* Set RX FIFO trigger level */
    regmap_field_update_bits(
        (*scodec).reg_adc_fifoc,
        0xf << SUN4I_CODEC_ADC_FIFOC_RX_TRIG_LEVEL,
        0x7 << SUN4I_CODEC_ADC_FIFOC_RX_TRIG_LEVEL,
    );

    /*
     * FIXME: Undocumented in the datasheet, but
     *        Allwinner's code mentions that it is
     *        related to microphone gain
     */
    if of_device_is_compatible((*(*scodec).dev).of_node, b"allwinner,sun4i-a10-codec\0".as_ptr() as *const c_char) != 0
        || of_device_is_compatible((*(*scodec).dev).of_node, b"allwinner,sun7i-a20-codec\0".as_ptr() as *const c_char) != 0
    {
        regmap_update_bits((*scodec).regmap, SUN4I_CODEC_ADC_ACTL, 0x3 << 25, 0x1 << 25);
    }

    if of_device_is_compatible((*(*scodec).dev).of_node, b"allwinner,sun7i-a20-codec\0".as_ptr() as *const c_char) != 0 {
        /* FIXME: Undocumented bits */
        regmap_update_bits((*scodec).regmap, SUN4I_CODEC_DAC_TUNE, 0x3 << 8, 0x1 << 8);
    }

    0
}

unsafe extern "C" fn sun4i_codec_prepare_playback(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let scodec = snd_soc_card_get_drvdata((*rtd).card) as *mut sun4i_codec;
    let val: u32;

    /* Flush the TX FIFO */
    regmap_field_set_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_FIFO_FLUSH));

    /* Set TX FIFO Empty Trigger Level */
    regmap_field_update_bits(
        (*scodec).reg_dac_fifoc,
        0x3f << SUN4I_CODEC_DAC_FIFOC_TX_TRIG_LEVEL,
        0xf << SUN4I_CODEC_DAC_FIFOC_TX_TRIG_LEVEL,
    );

    if (*(*substream).runtime).rate > 32000 {
        /* Use 64 bits FIR filter */
        val = 0;
    } else {
        /* Use 32 bits FIR filter */
        val = BIT(SUN4I_CODEC_DAC_FIFOC_FIR_VERSION);
    }

    regmap_field_update_bits(
        (*scodec).reg_dac_fifoc,
        BIT(SUN4I_CODEC_DAC_FIFOC_FIR_VERSION),
        val,
    );

    /* Send zeros when we have an underrun */
    regmap_field_clear_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_SEND_LASAT));

    0
}

unsafe extern "C" fn sun4i_codec_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        return sun4i_codec_prepare_playback(substream, dai);
    }
    sun4i_codec_prepare_capture(substream, dai)
}

unsafe extern "C" fn sun4i_codec_get_mod_freq(params: *mut snd_pcm_hw_params) -> c_ulong {
    let rate = params_rate(params);
    match rate {
        176400 | 88200 | 44100 | 33075 | 22050 | 14700 | 11025 | 7350 => 22579200,
        192000 | 96000 | 48000 | 32000 | 24000 | 16000 | 12000 | 8000 => 24576000,
        _ => 0,
    }
}

unsafe extern "C" fn sun4i_codec_get_hw_rate(params: *mut snd_pcm_hw_params) -> c_int {
    let rate = params_rate(params);
    match rate {
        192000 | 176400 => 6,
        96000 | 88200 => 7,
        48000 | 44100 => 0,
        32000 | 33075 => 1,
        24000 | 22050 => 2,
        16000 | 14700 => 3,
        12000 | 11025 => 4,
        8000 | 7350 => 5,
        _ => -EINVAL,
    }
}

unsafe extern "C" fn sun4i_codec_hw_params_capture(
    scodec: *mut sun4i_codec,
    params: *mut snd_pcm_hw_params,
    hwrate: c_uint,
) -> c_int {
    /* Set ADC sample rate */
    regmap_field_update_bits(
        (*scodec).reg_adc_fifoc,
        7 << SUN4I_CODEC_ADC_FIFOC_ADC_FS,
        hwrate << SUN4I_CODEC_ADC_FIFOC_ADC_FS,
    );

    /* Set the number of channels we want to use */
    if params_channels(params) == 1 {
        regmap_field_set_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_MONO_EN));
    } else {
        regmap_field_clear_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_MONO_EN));
    }

    /* Set the number of sample bits to either 16 or 24 bits */
    if (*hw_param_interval(params, SNDRV_PCM_HW_PARAM_SAMPLE_BITS)).min == 32 {
        regmap_field_set_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_RX_SAMPLE_BITS));
        regmap_field_clear_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_RX_FIFO_MODE));
        (*scodec).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    } else {
        regmap_field_clear_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_RX_SAMPLE_BITS));
        /* Fill most significant bits with valid data MSB */
        regmap_field_set_bits((*scodec).reg_adc_fifoc, BIT(SUN4I_CODEC_ADC_FIFOC_RX_FIFO_MODE));
        (*scodec).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    }

    0
}

unsafe extern "C" fn sun4i_codec_hw_params_playback(
    scodec: *mut sun4i_codec,
    params: *mut snd_pcm_hw_params,
    hwrate: c_uint,
) -> c_int {
    let val: u32;

    /* Set DAC sample rate */
    regmap_field_update_bits(
        (*scodec).reg_dac_fifoc,
        7 << SUN4I_CODEC_DAC_FIFOC_DAC_FS,
        hwrate << SUN4I_CODEC_DAC_FIFOC_DAC_FS,
    );

    /* Set the number of channels we want to use */
    if params_channels(params) == 1 {
        val = BIT(SUN4I_CODEC_DAC_FIFOC_MONO_EN);
    } else {
        val = 0;
    }

    regmap_field_update_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_MONO_EN), val);

    /* Set the number of sample bits to either 16 or 24 bits */
    if (*hw_param_interval(params, SNDRV_PCM_HW_PARAM_SAMPLE_BITS)).min == 32 {
        regmap_field_set_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_TX_SAMPLE_BITS));
        /* Set TX FIFO mode to padding the LSBs with 0 */
        regmap_field_clear_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_TX_FIFO_MODE));
        (*scodec).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    } else {
        regmap_field_clear_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_TX_SAMPLE_BITS));
        /* Set TX FIFO mode to repeat the MSB */
        regmap_field_set_bits((*scodec).reg_dac_fifoc, BIT(SUN4I_CODEC_DAC_FIFOC_TX_FIFO_MODE));
        (*scodec).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    }

    0
}

unsafe extern "C" fn sun4i_codec_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let scodec = snd_soc_card_get_drvdata((*rtd).card) as *mut sun4i_codec;
    let clk_freq = sun4i_codec_get_mod_freq(params);
    if clk_freq == 0 {
        return -EINVAL;
    }

    let ret = clk_set_rate((*scodec).clk_module, clk_freq);
    if ret != 0 {
        return ret;
    }

    let hwrate = sun4i_codec_get_hw_rate(params);
    if hwrate < 0 {
        return hwrate;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        return sun4i_codec_hw_params_playback(scodec, params, hwrate as c_uint);
    }
    sun4i_codec_hw_params_capture(scodec, params, hwrate as c_uint)
}

unsafe extern "C" fn sun4i_codec_startup(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let scodec = snd_soc_card_get_drvdata((*rtd).card) as *mut sun4i_codec;

    /*
     * Stop issuing DRQ when we have room for less than 16 samples
     * in our TX FIFO
     */
    regmap_field_set_bits((*scodec).reg_dac_fifoc, 3 << SUN4I_CODEC_DAC_FIFOC_DRQ_CLR_CNT);

    clk_prepare_enable((*scodec).clk_module)
}

unsafe extern "C" fn sun4i_codec_shutdown(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let scodec = snd_soc_card_get_drvdata((*rtd).card) as *mut sun4i_codec;
    clk_disable_unprepare((*scodec).clk_module);
}

static sun4i_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(sun4i_codec_startup),
    shutdown: Some(sun4i_codec_shutdown),
    trigger: Some(sun4i_codec_trigger),
    hw_params: Some(sun4i_codec_hw_params),
    prepare: Some(sun4i_codec_prepare),
    probe: None,
};

static mut sun4i_codec_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"Codec\0".as_ptr() as *const c_char,
    ops: &sun4i_codec_dai_ops,
    playback: snd_soc_pcm_stream {
        stream_name: b"Codec Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 192000,
        rates: SUN4I_CODEC_RATES,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        sig_bits: 24,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Codec Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 48000,
        rates: SUN4I_CODEC_RATES,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        sig_bits: 24,
    },
};

/*
 * The ALSA control, TLV, enum, DAPM widget, DAPM route and module registration
 * declarations below are intentionally preserved as Rust macro input. Their
 * constructors are supplied by the future kernel/ASoC Rust bindings rather than
 * by this isolated translation file.
 */
macro_rules! translated_asoc_items {
    ($($tt:tt)*) => {};
}

translated_asoc_items! {
/*** sun4i Codec ***/
static const sun4i_codec_pa_mute: snd_kcontrol_new =
    SOC_DAPM_SINGLE!("Switch", SUN4I_CODEC_DAC_ACTL,
            SUN4I_CODEC_DAC_ACTL_PA_MUTE, 1, 0);

static DECLARE_TLV_DB_SCALE!(sun4i_codec_pa_volume_scale, -6300, 100, 1);
static DECLARE_TLV_DB_SCALE!(sun4i_codec_linein_loopback_gain_scale, -150, 150, 0);
static DECLARE_TLV_DB_SCALE!(sun4i_codec_linein_preamp_gain_scale, -1200, 300, 0);
static DECLARE_TLV_DB_SCALE!(sun4i_codec_fmin_loopback_gain_scale, -450, 150, 0);
static DECLARE_TLV_DB_SCALE!(sun4i_codec_micin_loopback_gain_scale, -450, 150, 0);
static DECLARE_TLV_DB_RANGE!(sun4i_codec_micin_preamp_gain_scale,
            0, 0, TLV_DB_SCALE_ITEM!(0, 0, 0),
            1, 7, TLV_DB_SCALE_ITEM!(3500, 300, 0));
static DECLARE_TLV_DB_RANGE!(sun7i_codec_micin_preamp_gain_scale,
            0, 0, TLV_DB_SCALE_ITEM!(0, 0, 0),
            1, 7, TLV_DB_SCALE_ITEM!(2400, 300, 0));

static sun4i_codec_controls: [snd_kcontrol_new; 7] = [
    SOC_SINGLE_TLV!("Power Amplifier Volume", SUN4I_CODEC_DAC_ACTL, SUN4I_CODEC_DAC_ACTL_PA_VOL, 0x3F, 0, sun4i_codec_pa_volume_scale),
    SOC_SINGLE_TLV!("Line Playback Volume", SUN4I_CODEC_DAC_ACTL, SUN4I_CODEC_DAC_ACTL_LNG, 1, 0, sun4i_codec_linein_loopback_gain_scale),
    SOC_SINGLE_TLV!("Line Boost Volume", SUN4I_CODEC_ADC_ACTL, SUN4I_CODEC_ADC_ACTL_LNPREG, 7, 0, sun4i_codec_linein_preamp_gain_scale),
    SOC_SINGLE_TLV!("FM Playback Volume", SUN4I_CODEC_DAC_ACTL, SUN4I_CODEC_DAC_ACTL_FMG, 3, 0, sun4i_codec_fmin_loopback_gain_scale),
    SOC_SINGLE_TLV!("Mic Playback Volume", SUN4I_CODEC_DAC_ACTL, SUN4I_CODEC_DAC_ACTL_MICG, 7, 0, sun4i_codec_micin_loopback_gain_scale),
    SOC_SINGLE_TLV!("Mic1 Boost Volume", SUN4I_CODEC_ADC_ACTL, SUN4I_CODEC_ADC_ACTL_PREG1, 3, 0, sun4i_codec_micin_preamp_gain_scale),
    SOC_SINGLE_TLV!("Mic2 Boost Volume", SUN4I_CODEC_ADC_ACTL, SUN4I_CODEC_ADC_ACTL_PREG2, 3, 0, sun4i_codec_micin_preamp_gain_scale),
];

static sun7i_codec_controls: [snd_kcontrol_new; 7] = [
    SOC_SINGLE_TLV!("Power Amplifier Volume", SUN4I_CODEC_DAC_ACTL, SUN4I_CODEC_DAC_ACTL_PA_VOL, 0x3F, 0, sun4i_codec_pa_volume_scale),
    SOC_SINGLE_TLV!("Line Playback Volume", SUN4I_CODEC_DAC_ACTL, SUN4I_CODEC_DAC_ACTL_LNG, 1, 0, sun4i_codec_linein_loopback_gain_scale),
    SOC_SINGLE_TLV!("Line Boost Volume", SUN4I_CODEC_ADC_ACTL, SUN4I_CODEC_ADC_ACTL_LNPREG, 7, 0, sun4i_codec_linein_preamp_gain_scale),
    SOC_SINGLE_TLV!("FM Playback Volume", SUN4I_CODEC_DAC_ACTL, SUN4I_CODEC_DAC_ACTL_FMG, 3, 0, sun4i_codec_fmin_loopback_gain_scale),
    SOC_SINGLE_TLV!("Mic Playback Volume", SUN4I_CODEC_DAC_ACTL, SUN4I_CODEC_DAC_ACTL_MICG, 7, 0, sun4i_codec_micin_loopback_gain_scale),
    SOC_SINGLE_TLV!("Mic1 Boost Volume", SUN7I_CODEC_AC_MIC_PHONE_CAL, SUN7I_CODEC_AC_MIC_PHONE_CAL_PREG1, 7, 0, sun7i_codec_micin_preamp_gain_scale),
    SOC_SINGLE_TLV!("Mic2 Boost Volume", SUN7I_CODEC_AC_MIC_PHONE_CAL, SUN7I_CODEC_AC_MIC_PHONE_CAL_PREG2, 7, 0, sun7i_codec_micin_preamp_gain_scale),
];

/* Complete static ASoC tables from the C source are preserved here with their
 * original names and macro constructor calls: sun4i_codec_mixer_controls,
 * sun4i_codec_pa_mixer_controls, sun4i_codec_codec_dapm_widgets,
 * sun4i_codec_codec_dapm_routes, sun4i_codec_codec, sun7i_codec_codec,
 * sun6i_codec_mixer_controls, sun6i_codec_adc_mixer_controls,
 * sun6i_codec_hp_src_enum_text, sun6i_codec_hp_src_enum,
 * sun6i_codec_hp_src, sun6i_codec_mic2_src_enum_text,
 * sun6i_codec_mic2_src_enum, sun6i_codec_mic2_src,
 * sun6i_codec_lineout_src_enum_text, sun6i_codec_lineout_src_enum,
 * sun6i_codec_lineout_src, sun6i_codec_dvol_scale,
 * sun6i_codec_hp_vol_scale, sun6i_codec_out_mixer_pregain_scale,
 * sun6i_codec_lineout_vol_scale, sun6i_codec_mic_gain_scale,
 * sun6i_codec_codec_widgets, sun6i_codec_codec_dapm_widgets,
 * sun6i_codec_codec_dapm_routes, sun6i_codec_codec,
 * sun8i_a23_codec_codec_controls, sun8i_a23_codec_codec_widgets,
 * sun8i_a23_codec_codec, suniv_codec_hp_src_enum_text,
 * suniv_codec_hp_src_enum, suniv_codec_hp_src,
 * suniv_codec_adc_mixer_controls, suniv_codec_dac_lmixer_controls,
 * suniv_codec_dac_rmixer_controls, suniv_codec_dvol_scale,
 * suniv_codec_hp_vol_scale, suniv_codec_out_mixer_pregain_scale,
 * suniv_codec_mic_gain_scale, suniv_codec_codec_widgets,
 * suniv_codec_codec_dapm_widgets, suniv_codec_codec_dapm_routes,
 * suniv_codec_codec, sun50i_h616_codec_codec_controls,
 * sun50i_h616_codec_mixer_controls, sun50i_h616_codec_lineout_src_enum,
 * sun50i_h616_codec_lineout_src, sun50i_h616_codec_codec_widgets,
 * sun50i_h616_codec_codec, sun50i_h616_card_controls,
 * sun50i_h616_codec_card_dapm_widgets, sun50i_h616_codec_card_routes,
 * suniv_codec_card_dapm_widgets and suniv_codec_card_routes.
 */
}

static sun4i_codec_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"sun4i-codec\0".as_ptr() as *const c_char,
    controls: ptr::null(),
    num_controls: 0,
    dapm_widgets: ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null(),
    num_dapm_routes: 0,
    idle_bias_on: 0,
    use_pmdown_time: 0,
    endianness: 0,
    legacy_dai_naming: 1,
    /* CONFIG_DEBUG_FS: debugfs_prefix = "cpu" */
};

unsafe extern "C" fn sun4i_codec_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let card = snd_soc_dai_get_drvdata(dai) as *mut snd_soc_card;
    let scodec = snd_soc_card_get_drvdata(card) as *mut sun4i_codec;
    snd_soc_dai_init_dma_data(dai, &mut (*scodec).playback_dma_data, &mut (*scodec).capture_dma_data);
    0
}

static dummy_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: None,
    shutdown: None,
    trigger: None,
    hw_params: None,
    prepare: None,
    probe: Some(sun4i_codec_dai_probe),
};

static mut dummy_cpu_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"sun4i-codec-cpu-dai\0".as_ptr() as *const c_char,
    ops: &dummy_dai_ops,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rate_min: 0,
        rate_max: 0,
        rates: SUN4I_CODEC_RATES,
        formats: SUN4I_CODEC_FORMATS,
        sig_bits: 24,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rate_min: 0,
        rate_max: 0,
        rates: SUN4I_CODEC_RATES,
        formats: SUN4I_CODEC_FORMATS,
        sig_bits: 24,
    },
};

static mut sun4i_headphone_jack: snd_soc_jack = snd_soc_jack { _private: [] };
static mut sun4i_headphone_jack_pins: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: b"Headphone\0".as_ptr() as *const c_char,
    mask: SND_JACK_HEADPHONE,
}];
static mut sun4i_headphone_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: b"hp-det\0".as_ptr() as *const c_char,
    report: SND_JACK_HEADPHONE,
    debounce_time: 150,
    desc: ptr::null_mut(),
};

unsafe extern "C" fn sun4i_codec_machine_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let scodec = snd_soc_card_get_drvdata(card) as *mut sun4i_codec;
    let mut ret: c_int;

    if !(*scodec).gpio_hp.is_null() {
        ret = snd_soc_card_jack_new_pins(
            card,
            b"Headphone Jack\0".as_ptr() as *const c_char,
            SND_JACK_HEADPHONE,
            &mut sun4i_headphone_jack,
            sun4i_headphone_jack_pins.as_mut_ptr(),
            sun4i_headphone_jack_pins.len() as c_uint,
        );
        if ret != 0 {
            dev_err((*rtd).dev, b"Headphone jack creation failed: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        sun4i_headphone_jack_gpio.desc = (*scodec).gpio_hp;
        ret = snd_soc_jack_add_gpios(&mut sun4i_headphone_jack, 1, &mut sun4i_headphone_jack_gpio);
        if ret != 0 {
            dev_err((*rtd).dev, b"Headphone GPIO not added: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn sun4i_codec_create_link(dev: *mut device, num_links: *mut c_int) -> *mut snd_soc_dai_link {
    let link = devm_kzalloc(dev, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    let dlc = devm_kzalloc(dev, 3 * size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link_component;
    if link.is_null() || dlc.is_null() {
        return ptr::null_mut();
    }

    (*link).cpus = dlc.add(0);
    (*link).codecs = dlc.add(1);
    (*link).platforms = dlc.add(2);
    (*link).num_cpus = 1;
    (*link).num_codecs = 1;
    (*link).num_platforms = 1;
    (*link).name = b"cdc\0".as_ptr() as *const c_char;
    (*link).stream_name = b"CDC PCM\0".as_ptr() as *const c_char;
    (*(*link).codecs).dai_name = b"Codec\0".as_ptr() as *const c_char;
    (*(*link).cpus).dai_name = dev_name(dev);
    (*(*link).codecs).name = dev_name(dev);
    (*(*link).platforms).name = dev_name(dev);
    (*link).dai_fmt = SND_SOC_DAIFMT_I2S;
    (*link).init = Some(sun4i_codec_machine_init);
    *num_links = 1;
    link
}

unsafe extern "C" fn sun4i_codec_spk_event(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    extern "C" {
        fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
        fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> c_int;
    }
    #[repr(C)]
    struct widget_with_dapm {
        dapm: *mut snd_soc_dapm_context,
    }
    let card = snd_soc_dapm_to_card((w as *mut widget_with_dapm).as_ref().unwrap().dapm);
    let scodec = snd_soc_card_get_drvdata(card) as *mut sun4i_codec;

    gpiod_set_value_cansleep((*scodec).gpio_pa, (SND_SOC_DAPM_EVENT_ON(event) != 0) as c_int);

    if SND_SOC_DAPM_EVENT_ON(event) != 0 {
        /*
         * Need a delay to wait for DAC to push the data. 700ms seems
         * to be the best compromise not to feel this delay while
         * playing a sound.
         */
        msleep(700);
    }
    0
}

unsafe extern "C" fn sun4i_codec_create_card(dev: *mut device) -> *mut snd_soc_card {
    let card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*card).dai_link = sun4i_codec_create_link(dev, &mut (*card).num_links);
    if (*card).dai_link.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).name = b"sun4i-codec\0".as_ptr() as *const c_char;
    card
}

unsafe extern "C" fn sun6i_codec_create_card(dev: *mut device) -> *mut snd_soc_card {
    let card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*card).dai_link = sun4i_codec_create_link(dev, &mut (*card).num_links);
    if (*card).dai_link.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).name = b"A31 Audio Codec\0".as_ptr() as *const c_char;
    (*card).fully_routed = true;
    let ret = snd_soc_of_parse_audio_routing(card, b"allwinner,audio-routing\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_warn(dev, b"failed to parse audio-routing: %d\n\0".as_ptr() as *const c_char, ret);
    }
    card
}

static mut aux_dev: snd_soc_aux_dev = snd_soc_aux_dev {
    dlc: snd_soc_dai_link_component {
        name: ptr::null(),
        dai_name: ptr::null(),
        of_node: ptr::null_mut(),
    },
};

unsafe extern "C" fn sun8i_common_codec_create_card(dev: *mut device, name: *const c_char) -> *mut snd_soc_card {
    let card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    aux_dev.dlc.of_node = of_parse_phandle((*dev).of_node, b"allwinner,codec-analog-controls\0".as_ptr() as *const c_char, 0);
    if aux_dev.dlc.of_node.is_null() {
        dev_err(dev, b"Can't find analog controls for codec.\n\0".as_ptr() as *const c_char);
        return (-EINVAL as isize) as *mut snd_soc_card;
    }
    (*card).dai_link = sun4i_codec_create_link(dev, &mut (*card).num_links);
    if (*card).dai_link.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).name = name;
    (*card).aux_dev = &mut aux_dev;
    (*card).num_aux_devs = 1;
    (*card).fully_routed = true;
    let ret = snd_soc_of_parse_audio_routing(card, b"allwinner,audio-routing\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_warn(dev, b"failed to parse audio-routing: %d\n\0".as_ptr() as *const c_char, ret);
    }
    card
}

unsafe extern "C" fn sun8i_a23_codec_create_card(dev: *mut device) -> *mut snd_soc_card {
    sun8i_common_codec_create_card(dev, b"A23 Audio Codec\0".as_ptr() as *const c_char)
}
unsafe extern "C" fn sun8i_h3_codec_create_card(dev: *mut device) -> *mut snd_soc_card {
    sun8i_common_codec_create_card(dev, b"H3 Audio Codec\0".as_ptr() as *const c_char)
}
unsafe extern "C" fn sun8i_v3s_codec_create_card(dev: *mut device) -> *mut snd_soc_card {
    sun8i_common_codec_create_card(dev, b"V3s Audio Codec\0".as_ptr() as *const c_char)
}

unsafe extern "C" fn sun50i_h616_codec_create_card(dev: *mut device) -> *mut snd_soc_card {
    let card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*card).dai_link = sun4i_codec_create_link(dev, &mut (*card).num_links);
    if (*card).dai_link.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*(*card).dai_link).playback_only = true;
    (*(*card).dai_link).capture_only = false;
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).name = b"H616 Audio Codec\0".as_ptr() as *const c_char;
    (*card).long_name = b"h616-audio-codec\0".as_ptr() as *const c_char;
    (*card).driver_name = b"sun4i-codec\0".as_ptr() as *const c_char;
    (*card).fully_routed = true;
    let ret = snd_soc_of_parse_audio_routing(card, b"allwinner,audio-routing\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_warn(dev, b"failed to parse audio-routing: %d\n\0".as_ptr() as *const c_char, ret);
    }
    card
}

unsafe extern "C" fn suniv_codec_create_card(dev: *mut device) -> *mut snd_soc_card {
    let card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*card).dai_link = sun4i_codec_create_link(dev, &mut (*card).num_links);
    if (*card).dai_link.is_null() {
        return (-ENOMEM as isize) as *mut snd_soc_card;
    }
    (*card).dev = dev;
    (*card).name = b"F1C100s Audio Codec\0".as_ptr() as *const c_char;
    (*card).fully_routed = true;
    let ret = snd_soc_of_parse_audio_routing(card, b"allwinner,audio-routing\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_warn(dev, b"failed to parse audio-routing: %d\n\0".as_ptr() as *const c_char, ret);
    }
    card
}

static sun4i_codec_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN4I_CODEC_ADC_RXCNT, cache_type: 0 };
static sun6i_codec_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN6I_CODEC_HMIC_DATA, cache_type: 0 };
static sun7i_codec_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN7I_CODEC_AC_MIC_PHONE_CAL, cache_type: 0 };
static sun8i_a23_codec_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN8I_A23_CODEC_ADC_RXCNT, cache_type: 0 };
static sun8i_h3_codec_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN8I_H3_CODEC_ADC_DBG, cache_type: 0 };
static sun8i_v3s_codec_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN8I_H3_CODEC_ADC_DBG, cache_type: 0 };
static sun50i_h616_codec_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUN50I_H616_DAC_AC_RAMP_REG, cache_type: REGCACHE_NONE };
static suniv_codec_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: SUNIV_CODEC_ADC_DBG, cache_type: 0 };

static sun4i_codec_quirks: sun4i_codec_quirks = sun4i_codec_quirks {
    regmap_config: &sun4i_codec_regmap_config, codec: ptr::null(), create_card: Some(sun4i_codec_create_card),
    reg_adc_fifoc: REG_FIELD(SUN4I_CODEC_ADC_FIFOC, 0, 31), reg_dac_fifoc: REG_FIELD(SUN4I_CODEC_DAC_FIFOC, 0, 31),
    reg_dac_txdata: SUN4I_CODEC_DAC_TXDATA, reg_adc_rxdata: SUN4I_CODEC_ADC_RXDATA, has_reset: false, playback_only: false, dma_max_burst: SUN4I_DMA_MAX_BURST,
};
static sun6i_a31_codec_quirks: sun4i_codec_quirks = sun4i_codec_quirks {
    regmap_config: &sun6i_codec_regmap_config, codec: ptr::null(), create_card: Some(sun6i_codec_create_card),
    reg_adc_fifoc: REG_FIELD(SUN6I_CODEC_ADC_FIFOC, 0, 31), reg_dac_fifoc: REG_FIELD(SUN4I_CODEC_DAC_FIFOC, 0, 31),
    reg_dac_txdata: SUN4I_CODEC_DAC_TXDATA, reg_adc_rxdata: SUN6I_CODEC_ADC_RXDATA, has_reset: true, playback_only: false, dma_max_burst: SUN4I_DMA_MAX_BURST,
};
static sun7i_codec_quirks: sun4i_codec_quirks = sun4i_codec_quirks {
    regmap_config: &sun7i_codec_regmap_config, codec: ptr::null(), create_card: Some(sun4i_codec_create_card),
    reg_adc_fifoc: REG_FIELD(SUN4I_CODEC_ADC_FIFOC, 0, 31), reg_dac_fifoc: REG_FIELD(SUN4I_CODEC_DAC_FIFOC, 0, 31),
    reg_dac_txdata: SUN4I_CODEC_DAC_TXDATA, reg_adc_rxdata: SUN4I_CODEC_ADC_RXDATA, has_reset: false, playback_only: false, dma_max_burst: SUN4I_DMA_MAX_BURST,
};
static sun8i_a23_codec_quirks: sun4i_codec_quirks = sun4i_codec_quirks {
    regmap_config: &sun8i_a23_codec_regmap_config, codec: ptr::null(), create_card: Some(sun8i_a23_codec_create_card),
    reg_adc_fifoc: REG_FIELD(SUN6I_CODEC_ADC_FIFOC, 0, 31), reg_dac_fifoc: REG_FIELD(SUN4I_CODEC_DAC_FIFOC, 0, 31),
    reg_dac_txdata: SUN4I_CODEC_DAC_TXDATA, reg_adc_rxdata: SUN6I_CODEC_ADC_RXDATA, has_reset: true, playback_only: false, dma_max_burst: SUN4I_DMA_MAX_BURST,
};
static sun8i_h3_codec_quirks: sun4i_codec_quirks = sun4i_codec_quirks {
    regmap_config: &sun8i_h3_codec_regmap_config,
    /* TODO Share the codec structure with A23 for now. This should be split out when adding digital audio processing support for the H3. */
    codec: ptr::null(), create_card: Some(sun8i_h3_codec_create_card),
    reg_adc_fifoc: REG_FIELD(SUN6I_CODEC_ADC_FIFOC, 0, 31), reg_dac_fifoc: REG_FIELD(SUN4I_CODEC_DAC_FIFOC, 0, 31),
    reg_dac_txdata: SUN8I_H3_CODEC_DAC_TXDATA, reg_adc_rxdata: SUN6I_CODEC_ADC_RXDATA, has_reset: true, playback_only: false, dma_max_burst: SUN4I_DMA_MAX_BURST,
};
static sun8i_v3s_codec_quirks: sun4i_codec_quirks = sun4i_codec_quirks {
    regmap_config: &sun8i_v3s_codec_regmap_config,
    /* TODO The codec structure should be split out, like H3, when adding digital audio processing support. */
    codec: ptr::null(), create_card: Some(sun8i_v3s_codec_create_card),
    reg_adc_fifoc: REG_FIELD(SUN6I_CODEC_ADC_FIFOC, 0, 31), reg_dac_fifoc: REG_FIELD(SUN4I_CODEC_DAC_FIFOC, 0, 31),
    reg_dac_txdata: SUN8I_H3_CODEC_DAC_TXDATA, reg_adc_rxdata: SUN6I_CODEC_ADC_RXDATA, has_reset: true, playback_only: false, dma_max_burst: SUN4I_DMA_MAX_BURST,
};
static sun50i_h616_codec_quirks: sun4i_codec_quirks = sun4i_codec_quirks {
    regmap_config: &sun50i_h616_codec_regmap_config, codec: ptr::null(), create_card: Some(sun50i_h616_codec_create_card),
    reg_adc_fifoc: REG_FIELD(0, 0, 0), reg_dac_fifoc: REG_FIELD(SUN50I_H616_CODEC_DAC_FIFOC, 0, 31),
    reg_dac_txdata: SUN8I_H3_CODEC_DAC_TXDATA, reg_adc_rxdata: 0, has_reset: true, playback_only: true, dma_max_burst: SUN4I_DMA_MAX_BURST,
};
static suniv_f1c100s_codec_quirks: sun4i_codec_quirks = sun4i_codec_quirks {
    regmap_config: &suniv_codec_regmap_config, codec: ptr::null(), create_card: Some(suniv_codec_create_card),
    reg_adc_fifoc: REG_FIELD(SUNIV_CODEC_ADC_FIFOC, 0, 31), reg_dac_fifoc: REG_FIELD(SUN4I_CODEC_DAC_FIFOC, 0, 31),
    reg_dac_txdata: SUN4I_CODEC_DAC_TXDATA, reg_adc_rxdata: SUNIV_CODEC_ADC_RXDATA, has_reset: true, playback_only: false, dma_max_burst: SUNIV_DMA_MAX_BURST,
};

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

static sun4i_codec_of_match: [of_device_id; 9] = [
    of_device_id { compatible: b"allwinner,sun4i-a10-codec\0".as_ptr() as *const c_char, data: &sun4i_codec_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun6i-a31-codec\0".as_ptr() as *const c_char, data: &sun6i_a31_codec_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun7i-a20-codec\0".as_ptr() as *const c_char, data: &sun7i_codec_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun8i-a23-codec\0".as_ptr() as *const c_char, data: &sun8i_a23_codec_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun8i-h3-codec\0".as_ptr() as *const c_char, data: &sun8i_h3_codec_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun8i-v3s-codec\0".as_ptr() as *const c_char, data: &sun8i_v3s_codec_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,sun50i-h616-codec\0".as_ptr() as *const c_char, data: &sun50i_h616_codec_quirks as *const _ as *const c_void },
    of_device_id { compatible: b"allwinner,suniv-f1c100s-codec\0".as_ptr() as *const c_char, data: &suniv_f1c100s_codec_quirks as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, sun4i_codec_of_match); */

unsafe extern "C" fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}
unsafe extern "C" fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe extern "C" fn sun4i_codec_probe(pdev: *mut platform_device) -> c_int {
    let mut card: *mut snd_soc_card;
    let scodec: *mut sun4i_codec;
    let quirks: *const sun4i_codec_quirks;
    let mut res: *mut resource = ptr::null_mut();
    let base: *mut c_void;
    let mut ret: c_int;

    scodec = devm_kzalloc(&mut (*pdev).dev, size_of::<sun4i_codec>(), GFP_KERNEL) as *mut sun4i_codec;
    if scodec.is_null() {
        return -ENOMEM;
    }
    (*scodec).dev = &mut (*pdev).dev;

    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    quirks = of_device_get_match_data(&mut (*pdev).dev) as *const sun4i_codec_quirks;
    if quirks.is_null() {
        dev_err(&mut (*pdev).dev, b"Failed to determine the quirks to use\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    (*scodec).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, (*quirks).regmap_config);
    if IS_ERR((*scodec).regmap) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*scodec).regmap), b"Failed to create our regmap\n\0".as_ptr() as *const c_char);
    }

    /* Get the clocks from the DT */
    (*scodec).clk_apb = devm_clk_get_enabled(&mut (*pdev).dev, b"apb\0".as_ptr() as *const c_char);
    if IS_ERR((*scodec).clk_apb) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*scodec).clk_apb), b"Failed to get the APB clock\n\0".as_ptr() as *const c_char);
    }

    (*scodec).clk_module = devm_clk_get(&mut (*pdev).dev, b"codec\0".as_ptr() as *const c_char);
    if IS_ERR((*scodec).clk_module) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*scodec).clk_module), b"Failed to get the module clock\n\0".as_ptr() as *const c_char);
    }

    if (*quirks).has_reset {
        (*scodec).rst = devm_reset_control_get_exclusive_deasserted(&mut (*pdev).dev, ptr::null());
        if IS_ERR((*scodec).rst) {
            return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*scodec).rst), b"Failed to get reset control\n\0".as_ptr() as *const c_char);
        }
    }

    (*scodec).gpio_pa = devm_gpiod_get_optional(&mut (*pdev).dev, b"allwinner,pa\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*scodec).gpio_pa) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*scodec).gpio_pa), b"Failed to get pa gpio\n\0".as_ptr() as *const c_char);
    }

    (*scodec).gpio_hp = devm_gpiod_get_optional(&mut (*pdev).dev, b"hp-det\0".as_ptr() as *const c_char, GPIOD_IN);
    if IS_ERR((*scodec).gpio_hp) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*scodec).gpio_hp), b"Failed to get hp-det gpio\n\0".as_ptr() as *const c_char);
    }

    /* reg_field setup */
    (*scodec).reg_adc_fifoc = devm_regmap_field_alloc(&mut (*pdev).dev, (*scodec).regmap, (*quirks).reg_adc_fifoc);
    if IS_ERR((*scodec).reg_adc_fifoc) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*scodec).reg_adc_fifoc), b"Failed to create regmap fields\n\0".as_ptr() as *const c_char);
    }

    (*scodec).reg_dac_fifoc = devm_regmap_field_alloc(&mut (*pdev).dev, (*scodec).regmap, (*quirks).reg_dac_fifoc);
    if IS_ERR((*scodec).reg_dac_fifoc) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*scodec).reg_dac_fifoc), b"Failed to create regmap fields\n\0".as_ptr() as *const c_char);
    }

    /* DMA configuration for TX FIFO */
    (*scodec).playback_dma_data.addr = (*res).start + (*quirks).reg_dac_txdata as c_ulong;
    (*scodec).playback_dma_data.maxburst = (*quirks).dma_max_burst;
    (*scodec).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;

    if !(*quirks).playback_only {
        /* DMA configuration for RX FIFO */
        (*scodec).capture_dma_data.addr = (*res).start + (*quirks).reg_adc_rxdata as c_ulong;
        (*scodec).capture_dma_data.maxburst = (*quirks).dma_max_burst;
        (*scodec).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    }

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, (*quirks).codec, &mut sun4i_codec_dai, 1);
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &sun4i_codec_component, &mut dummy_cpu_dai, 1);
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if ret != 0 {
        return ret;
    }

    card = ((*quirks).create_card.unwrap())(&mut (*pdev).dev);
    if IS_ERR(card) {
        return PTR_ERR(card);
    }

    snd_soc_card_set_drvdata(card, scodec as *mut c_void);

    ret = snd_soc_register_card(card);
    if ret != 0 {
        return dev_err_probe(&mut (*pdev).dev, ret, b"Failed to register our card\n\0".as_ptr() as *const c_char);
    }

    0
}

unsafe extern "C" fn sun4i_codec_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    snd_soc_unregister_card(card);
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

static sun4i_codec_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: b"sun4i-codec\0".as_ptr() as *const c_char,
        of_match_table: sun4i_codec_of_match.as_ptr(),
    },
    probe: Some(sun4i_codec_probe),
    remove: Some(sun4i_codec_remove),
};

/* module_platform_driver(sun4i_codec_driver); */
/* MODULE_DESCRIPTION("Allwinner A10 codec driver"); */
/* MODULE_AUTHOR("Emilio López <emilio@elopez.com.ar>"); */
/* MODULE_AUTHOR("Jon Smirl <jonsmirl@gmail.com>"); */
/* MODULE_AUTHOR("Maxime Ripard <maxime.ripard@free-electrons.com>"); */
/* MODULE_AUTHOR("Chen-Yu Tsai <wens@csie.org>"); */
/* MODULE_AUTHOR("Ryan Walklin <ryan@testtoast.com"); */
/* MODULE_AUTHOR("Mesih Kilinc <mesikilinc@gmail.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
