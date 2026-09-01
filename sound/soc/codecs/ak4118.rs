// SPDX-License-Identifier: GPL-2.0
/*
 * ak4118.rs  --  Asahi Kasei ALSA Soc Audio driver
 *
 * Copyright 2018 DEVIALET
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub const AK4118_REG_CLK_PWR_CTL: c_uint = 0x00;
pub const AK4118_REG_FORMAT_CTL: c_uint = 0x01;
pub const AK4118_REG_IO_CTL0: c_uint = 0x02;
pub const AK4118_REG_IO_CTL1: c_uint = 0x03;
pub const AK4118_REG_INT0_MASK: c_uint = 0x04;
pub const AK4118_REG_INT1_MASK: c_uint = 0x05;
pub const AK4118_REG_RCV_STATUS0: c_uint = 0x06;
pub const AK4118_REG_RCV_STATUS1: c_uint = 0x07;
pub const AK4118_REG_RXCHAN_STATUS0: c_uint = 0x08;
pub const AK4118_REG_RXCHAN_STATUS1: c_uint = 0x09;
pub const AK4118_REG_RXCHAN_STATUS2: c_uint = 0x0a;
pub const AK4118_REG_RXCHAN_STATUS3: c_uint = 0x0b;
pub const AK4118_REG_RXCHAN_STATUS4: c_uint = 0x0c;
pub const AK4118_REG_TXCHAN_STATUS0: c_uint = 0x0d;
pub const AK4118_REG_TXCHAN_STATUS1: c_uint = 0x0e;
pub const AK4118_REG_TXCHAN_STATUS2: c_uint = 0x0f;
pub const AK4118_REG_TXCHAN_STATUS3: c_uint = 0x10;
pub const AK4118_REG_TXCHAN_STATUS4: c_uint = 0x11;
pub const AK4118_REG_BURST_PREAMB_PC0: c_uint = 0x12;
pub const AK4118_REG_BURST_PREAMB_PC1: c_uint = 0x13;
pub const AK4118_REG_BURST_PREAMB_PD0: c_uint = 0x14;
pub const AK4118_REG_BURST_PREAMB_PD1: c_uint = 0x15;
pub const AK4118_REG_QSUB_CTL: c_uint = 0x16;
pub const AK4118_REG_QSUB_TRACK: c_uint = 0x17;
pub const AK4118_REG_QSUB_INDEX: c_uint = 0x18;
pub const AK4118_REG_QSUB_MIN: c_uint = 0x19;
pub const AK4118_REG_QSUB_SEC: c_uint = 0x1a;
pub const AK4118_REG_QSUB_FRAME: c_uint = 0x1b;
pub const AK4118_REG_QSUB_ZERO: c_uint = 0x1c;
pub const AK4118_REG_QSUB_ABS_MIN: c_uint = 0x1d;
pub const AK4118_REG_QSUB_ABS_SEC: c_uint = 0x1e;
pub const AK4118_REG_QSUB_ABS_FRAME: c_uint = 0x1f;
pub const AK4118_REG_GPE: c_uint = 0x20;
pub const AK4118_REG_GPDR: c_uint = 0x21;
pub const AK4118_REG_GPSCR: c_uint = 0x22;
pub const AK4118_REG_GPLR: c_uint = 0x23;
pub const AK4118_REG_DAT_MASK_DTS: c_uint = 0x24;
pub const AK4118_REG_RX_DETECT: c_uint = 0x25;
pub const AK4118_REG_STC_DAT_DETECT: c_uint = 0x26;
pub const AK4118_REG_RXCHAN_STATUS5: c_uint = 0x27;
pub const AK4118_REG_TXCHAN_STATUS5: c_uint = 0x28;
pub const AK4118_REG_MAX: c_uint = 0x29;

pub const AK4118_REG_FORMAT_CTL_DIF0: c_uint = 1 << 4;
pub const AK4118_REG_FORMAT_CTL_DIF1: c_uint = 1 << 5;
pub const AK4118_REG_FORMAT_CTL_DIF2: c_uint = 1 << 6;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct ak4118_priv {
    pub regmap: *mut regmap,
    pub reset: *mut gpio_desc,
    pub irq: *mut gpio_desc,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

static AK4118_REG_DEFAULTS: [reg_default; 21] = [
    reg_default { reg: AK4118_REG_CLK_PWR_CTL, def: 0x43 },
    reg_default { reg: AK4118_REG_FORMAT_CTL, def: 0x6a },
    reg_default { reg: AK4118_REG_IO_CTL0, def: 0x88 },
    reg_default { reg: AK4118_REG_IO_CTL1, def: 0x48 },
    reg_default { reg: AK4118_REG_INT0_MASK, def: 0xee },
    reg_default { reg: AK4118_REG_INT1_MASK, def: 0xb5 },
    reg_default { reg: AK4118_REG_RCV_STATUS0, def: 0x00 },
    reg_default { reg: AK4118_REG_RCV_STATUS1, def: 0x10 },
    reg_default { reg: AK4118_REG_TXCHAN_STATUS0, def: 0x00 },
    reg_default { reg: AK4118_REG_TXCHAN_STATUS1, def: 0x00 },
    reg_default { reg: AK4118_REG_TXCHAN_STATUS2, def: 0x00 },
    reg_default { reg: AK4118_REG_TXCHAN_STATUS3, def: 0x00 },
    reg_default { reg: AK4118_REG_TXCHAN_STATUS4, def: 0x00 },
    reg_default { reg: AK4118_REG_GPE, def: 0x77 },
    reg_default { reg: AK4118_REG_GPDR, def: 0x00 },
    reg_default { reg: AK4118_REG_GPSCR, def: 0x00 },
    reg_default { reg: AK4118_REG_GPLR, def: 0x00 },
    reg_default { reg: AK4118_REG_DAT_MASK_DTS, def: 0x3f },
    reg_default { reg: AK4118_REG_RX_DETECT, def: 0x00 },
    reg_default { reg: AK4118_REG_STC_DAT_DETECT, def: 0x00 },
    reg_default { reg: AK4118_REG_TXCHAN_STATUS5, def: 0x00 },
];

static AK4118_INPUT_SELECT_TXT: [*const c_char; 8] = [
    b"RX0\0".as_ptr() as *const c_char,
    b"RX1\0".as_ptr() as *const c_char,
    b"RX2\0".as_ptr() as *const c_char,
    b"RX3\0".as_ptr() as *const c_char,
    b"RX4\0".as_ptr() as *const c_char,
    b"RX5\0".as_ptr() as *const c_char,
    b"RX6\0".as_ptr() as *const c_char,
    b"RX7\0".as_ptr() as *const c_char,
];

// SOC_ENUM_SINGLE_DECL(ak4118_insel_enum, AK4118_REG_IO_CTL1, 0x0,
//                      ak4118_input_select_txt);
extern "C" {
    static ak4118_insel_enum: soc_enum;
}

// static const struct snd_kcontrol_new ak4118_input_mux_controls =
//     SOC_DAPM_ENUM("Input Select", ak4118_insel_enum);
extern "C" {
    static ak4118_input_mux_controls: snd_kcontrol_new;
}

static AK4118_IEC958_FS_TXT: [*const c_char; 13] = [
    b"44100\0".as_ptr() as *const c_char,
    b"48000\0".as_ptr() as *const c_char,
    b"32000\0".as_ptr() as *const c_char,
    b"22050\0".as_ptr() as *const c_char,
    b"11025\0".as_ptr() as *const c_char,
    b"24000\0".as_ptr() as *const c_char,
    b"16000\0".as_ptr() as *const c_char,
    b"88200\0".as_ptr() as *const c_char,
    b"8000\0".as_ptr() as *const c_char,
    b"96000\0".as_ptr() as *const c_char,
    b"64000\0".as_ptr() as *const c_char,
    b"176400\0".as_ptr() as *const c_char,
    b"192000\0".as_ptr() as *const c_char,
];

static AK4118_IEC958_FS_VAL: [c_uint; 13] = [
    0x0, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xE,
];

// SOC_VALUE_ENUM_SINGLE_DECL(ak4118_iec958_fs_enum, AK4118_REG_RCV_STATUS1,
//                            0x4, 0x4, ak4118_iec958_fs_txt,
//                            ak4118_iec958_fs_val);
extern "C" {
    static ak4118_iec958_fs_enum: soc_enum;
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
}

// Initializers use ASoC control macros:
// SOC_SINGLE("IEC958 Parity Errors", AK4118_REG_RCV_STATUS0, 0, 1, 0),
// SOC_SINGLE("IEC958 No Audio", AK4118_REG_RCV_STATUS0, 1, 1, 0),
// SOC_SINGLE("IEC958 PLL Lock", AK4118_REG_RCV_STATUS0, 4, 1, 1),
// SOC_SINGLE("IEC958 Non PCM", AK4118_REG_RCV_STATUS0, 6, 1, 0),
// SOC_ENUM("IEC958 Sampling Freq", ak4118_iec958_fs_enum),
static mut AK4118_IEC958_CONTROLS: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new { name: b"IEC958 Parity Errors\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"IEC958 No Audio\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"IEC958 PLL Lock\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"IEC958 Non PCM\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"IEC958 Sampling Freq\0".as_ptr() as *const c_char },
];

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

// SND_SOC_DAPM_INPUT and SND_SOC_DAPM_MUX macro-generated widget initializers.
extern "C" {
    static ak4118_dapm_widgets: [snd_soc_dapm_widget; 9];
}

static AK4118_DAPM_ROUTES: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"RX0\0".as_ptr() as *const c_char, source: b"INRX0\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"RX1\0".as_ptr() as *const c_char, source: b"INRX1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"RX2\0".as_ptr() as *const c_char, source: b"INRX2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"RX3\0".as_ptr() as *const c_char, source: b"INRX3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"RX4\0".as_ptr() as *const c_char, source: b"INRX4\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"RX5\0".as_ptr() as *const c_char, source: b"INRX5\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"RX6\0".as_ptr() as *const c_char, source: b"INRX6\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"RX7\0".as_ptr() as *const c_char, source: b"INRX7\0".as_ptr() as *const c_char },
];

extern "C" {
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static ENOTSUPP: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_HIGH: c_uint;
    static GPIOD_IN: c_uint;
    static IRQF_TRIGGER_RISING: c_uint;
    static IRQF_ONESHOT: c_uint;
    static REGCACHE_NONE: c_uint;
    static SNDRV_PCM_RATE_22050: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
}

unsafe fn ak4118_set_dai_fmt_provider(_ak4118: *mut ak4118_priv, format: c_uint) -> c_int {
    let dif: c_int;

    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            dif = (AK4118_REG_FORMAT_CTL_DIF0 | AK4118_REG_FORMAT_CTL_DIF2) as c_int;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            dif = (AK4118_REG_FORMAT_CTL_DIF0 | AK4118_REG_FORMAT_CTL_DIF1) as c_int;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            dif = AK4118_REG_FORMAT_CTL_DIF2 as c_int;
        }
        _ => {
            return -ENOTSUPP;
        }
    }

    dif
}

unsafe fn ak4118_set_dai_fmt_consumer(_ak4118: *mut ak4118_priv, format: c_uint) -> c_int {
    let dif: c_int;

    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            dif = (AK4118_REG_FORMAT_CTL_DIF0 | AK4118_REG_FORMAT_CTL_DIF1 | AK4118_REG_FORMAT_CTL_DIF2) as c_int;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            dif = (AK4118_REG_FORMAT_CTL_DIF1 | AK4118_REG_FORMAT_CTL_DIF2) as c_int;
        }
        _ => {
            return -ENOTSUPP;
        }
    }

    dif
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
}

unsafe extern "C" fn ak4118_set_dai_fmt(dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ak4118: *mut ak4118_priv = snd_soc_component_get_drvdata(component) as *mut ak4118_priv;
    let dif: c_int;
    let mut ret: c_int = 0;

    match format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            dif = ak4118_set_dai_fmt_provider(ak4118, format);
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            dif = ak4118_set_dai_fmt_consumer(ak4118, format);
        }
        _ => {
            ret = -ENOTSUPP;
            return ret;
        }
    }

    /* format not supported */
    if dif < 0 {
        ret = dif;
        return ret;
    }

    ret = regmap_update_bits(
        (*ak4118).regmap,
        AK4118_REG_FORMAT_CTL,
        AK4118_REG_FORMAT_CTL_DIF0 | AK4118_REG_FORMAT_CTL_DIF1 | AK4118_REG_FORMAT_CTL_DIF2,
        dif as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    ret
}

unsafe extern "C" fn ak4118_hw_params(
    _substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    _dai: *mut snd_soc_dai,
) -> c_int {
    0
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

static AK4118_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ak4118_hw_params),
    set_fmt: Some(ak4118_set_dai_fmt),
};

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

static mut AK4118_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ak4118-hifi\0".as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: unsafe {
            SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_32000 |
            SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 |
            SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 |
            SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000
        },
        formats: unsafe {
            SNDRV_PCM_FMTBIT_S16_LE |
            SNDRV_PCM_FMTBIT_S24_3LE |
            SNDRV_PCM_FMTBIT_S24_LE
        },
    },
    ops: unsafe { &AK4118_DAI_OPS as *const snd_soc_dai_ops },
};

pub type irqreturn_t = c_int;

extern "C" {
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    fn snd_soc_component_notify_control(component: *mut snd_soc_component, name: *const c_char);
}

unsafe extern "C" fn ak4118_irq_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let ak4118: *mut ak4118_priv = data as *mut ak4118_priv;
    let component: *mut snd_soc_component = (*ak4118).component;
    let mut kctl_new: *mut snd_kcontrol_new;
    let mut i: c_uint;

    if component.is_null() {
        return IRQ_NONE;
    }

    i = 0;
    while (i as usize) < AK4118_IEC958_CONTROLS.len() {
        kctl_new = &mut AK4118_IEC958_CONTROLS[i as usize];

        snd_soc_component_notify_control(component, (*kctl_new).name);
        i += 1;
    }

    IRQ_HANDLED
}

extern "C" {
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *mut snd_kcontrol_new, num_controls: c_uint) -> c_int;
}

unsafe extern "C" fn ak4118_probe(component: *mut snd_soc_component) -> c_int {
    let ak4118: *mut ak4118_priv = snd_soc_component_get_drvdata(component) as *mut ak4118_priv;
    let mut ret: c_int = 0;

    (*ak4118).component = component;

    /* release reset */
    gpiod_set_value((*ak4118).reset, 0);

    /* unmask all int1 sources */
    ret = regmap_write((*ak4118).regmap, AK4118_REG_INT1_MASK, 0x00);
    if ret < 0 {
        dev_err(
            (*component).dev,
            b"failed to write regmap 0x%x 0x%x: %d\n\0".as_ptr() as *const c_char,
            AK4118_REG_INT1_MASK,
            0x00,
            ret,
        );
        return ret;
    }

    /* rx detect enable on all channels */
    ret = regmap_write((*ak4118).regmap, AK4118_REG_RX_DETECT, 0xff);
    if ret < 0 {
        dev_err(
            (*component).dev,
            b"failed to write regmap 0x%x 0x%x: %d\n\0".as_ptr() as *const c_char,
            AK4118_REG_RX_DETECT,
            0xff,
            ret,
        );
        return ret;
    }

    ret = snd_soc_add_component_controls(
        component,
        AK4118_IEC958_CONTROLS.as_mut_ptr(),
        AK4118_IEC958_CONTROLS.len() as c_uint,
    );
    if ret != 0 {
        dev_err(
            (*component).dev,
            b"failed to add component kcontrols: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    0
}

unsafe extern "C" fn ak4118_remove(component: *mut snd_soc_component) {
    let ak4118: *mut ak4118_priv = snd_soc_component_get_drvdata(component) as *mut ak4118_priv;

    /* hold reset */
    gpiod_set_value((*ak4118).reset, 1);
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

static SOC_COMPONENT_DRV_AK4118: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ak4118_probe),
    remove: Some(ak4118_remove),
    dapm_widgets: unsafe { ak4118_dapm_widgets.as_ptr() },
    num_dapm_widgets: 9,
    dapm_routes: AK4118_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: AK4118_DAPM_ROUTES.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub max_register: c_uint,
}

static AK4118_REGMAP: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    reg_defaults: AK4118_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: AK4118_REG_DEFAULTS.len() as c_uint,

    cache_type: unsafe { REGCACHE_NONE },
    max_register: AK4118_REG_MAX - 1,
};

extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_int,
        handler: *mut c_void,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_uint,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn gpiod_to_irq(desc: *mut gpio_desc) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn ak4118_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ak4118: *mut ak4118_priv;
    let mut ret: c_int;

    ak4118 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<ak4118_priv>(),
        GFP_KERNEL,
    ) as *mut ak4118_priv;
    if ak4118.is_null() {
        return -ENOMEM;
    }

    (*ak4118).regmap = devm_regmap_init_i2c(i2c, &AK4118_REGMAP);
    if IS_ERR((*ak4118).regmap as *const c_void) {
        return PTR_ERR((*ak4118).regmap as *const c_void);
    }

    i2c_set_clientdata(i2c, ak4118 as *mut c_void);

    (*ak4118).reset = devm_gpiod_get(&mut (*i2c).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*ak4118).reset as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*ak4118).reset as *const c_void),
            b"Failed to get reset\n\0".as_ptr() as *const c_char,
        );
    }

    (*ak4118).irq = devm_gpiod_get(&mut (*i2c).dev, b"irq\0".as_ptr() as *const c_char, GPIOD_IN);
    if IS_ERR((*ak4118).irq as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*ak4118).irq as *const c_void),
            b"Failed to get IRQ\n\0".as_ptr() as *const c_char,
        );
    }

    ret = devm_request_threaded_irq(
        &mut (*i2c).dev,
        gpiod_to_irq((*ak4118).irq),
        ptr::null_mut(),
        Some(ak4118_irq_handler),
        IRQF_TRIGGER_RISING | IRQF_ONESHOT,
        b"ak4118-irq\0".as_ptr() as *const c_char,
        ak4118 as *mut c_void,
    );
    if ret < 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Fail to request_irq: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &SOC_COMPONENT_DRV_AK4118,
        &mut AK4118_DAI,
        1,
    )
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

/* CONFIG_OF: Open Firmware device match table. */
static AK4118_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"asahi-kasei,ak4118\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, ak4118_of_match);

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

static AK4118_ID_TABLE: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'a' as c_char, b'k' as c_char, b'4' as c_char, b'1' as c_char,
            b'1' as c_char, b'8' as c_char, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(i2c, ak4118_id_table);

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

// of_match_ptr(ak4118_of_match)
static mut AK4118_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"ak4118\0".as_ptr() as *const c_char,
        of_match_table: AK4118_OF_MATCH.as_ptr(),
    },
    id_table: AK4118_ID_TABLE.as_ptr(),
    probe: Some(ak4118_i2c_probe),
};

// module_i2c_driver(ak4118_i2c_driver);

// MODULE_DESCRIPTION("Asahi Kasei AK4118 ALSA SoC driver");
// MODULE_AUTHOR("Adrien Charruel <adrien.charruel@devialet.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
