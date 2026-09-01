// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CS4271 ASoC codec driver
 *
 * Copyright (c) 2010 Alexander Sverdlin <subaparts@yandex.ru>
 *
 * This driver support CS4271 codec being master or slave, working
 * in control port mode, connected either via SPI or I2C.
 * The data format accepted is I2S or left-justified.
 * DAPM support not implemented.
 */

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};
use core::ptr;

const CS4271_PCM_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
const CS4271_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;

/*
 * CS4271 registers
 */
const CS4271_MODE1: c_uint = 0x01; /* Mode Control 1 */
const CS4271_DACCTL: c_uint = 0x02; /* DAC Control */
const CS4271_DACVOL: c_uint = 0x03; /* DAC Volume & Mixing Control */
const CS4271_VOLA: c_uint = 0x04; /* DAC Channel A Volume Control */
const CS4271_VOLB: c_uint = 0x05; /* DAC Channel B Volume Control */
const CS4271_ADCCTL: c_uint = 0x06; /* ADC Control */
const CS4271_MODE2: c_uint = 0x07; /* Mode Control 2 */
const CS4271_CHIPID: c_uint = 0x08; /* Chip ID */

const CS4271_FIRSTREG: c_uint = CS4271_MODE1;
const CS4271_LASTREG: c_uint = CS4271_MODE2;
const CS4271_NR_REGS: c_uint = (CS4271_LASTREG & 0xFF) + 1;

/* Bit masks for the CS4271 registers */
const CS4271_MODE1_MODE_MASK: c_uint = 0xC0;
const CS4271_MODE1_MODE_1X: c_uint = 0x00;
const CS4271_MODE1_MODE_2X: c_uint = 0x80;
const CS4271_MODE1_MODE_4X: c_uint = 0xC0;

const CS4271_MODE1_DIV_MASK: c_uint = 0x30;
const CS4271_MODE1_DIV_1: c_uint = 0x00;
const CS4271_MODE1_DIV_15: c_uint = 0x10;
const CS4271_MODE1_DIV_2: c_uint = 0x20;
const CS4271_MODE1_DIV_3: c_uint = 0x30;

const CS4271_MODE1_MASTER: c_uint = 0x08;

const CS4271_MODE1_DAC_DIF_MASK: c_uint = 0x07;
const CS4271_MODE1_DAC_DIF_LJ: c_uint = 0x00;
const CS4271_MODE1_DAC_DIF_I2S: c_uint = 0x01;
const CS4271_MODE1_DAC_DIF_RJ16: c_uint = 0x02;
const CS4271_MODE1_DAC_DIF_RJ24: c_uint = 0x03;
const CS4271_MODE1_DAC_DIF_RJ20: c_uint = 0x04;
const CS4271_MODE1_DAC_DIF_RJ18: c_uint = 0x05;

const CS4271_DACCTL_AMUTE: c_uint = 0x80;
const CS4271_DACCTL_IF_SLOW: c_uint = 0x40;

const CS4271_DACCTL_DEM_MASK: c_uint = 0x30;
const CS4271_DACCTL_DEM_DIS: c_uint = 0x00;
const CS4271_DACCTL_DEM_441: c_uint = 0x10;
const CS4271_DACCTL_DEM_48: c_uint = 0x20;
const CS4271_DACCTL_DEM_32: c_uint = 0x30;

const CS4271_DACCTL_SVRU: c_uint = 0x08;
const CS4271_DACCTL_SRD: c_uint = 0x04;
const CS4271_DACCTL_INVA: c_uint = 0x02;
const CS4271_DACCTL_INVB: c_uint = 0x01;

const CS4271_DACVOL_BEQUA: c_uint = 0x40;
const CS4271_DACVOL_SOFT: c_uint = 0x20;
const CS4271_DACVOL_ZEROC: c_uint = 0x10;

const CS4271_DACVOL_ATAPI_MASK: c_uint = 0x0F;
const CS4271_DACVOL_ATAPI_M_M: c_uint = 0x00;
const CS4271_DACVOL_ATAPI_M_BR: c_uint = 0x01;
const CS4271_DACVOL_ATAPI_M_BL: c_uint = 0x02;
const CS4271_DACVOL_ATAPI_M_BLR2: c_uint = 0x03;
const CS4271_DACVOL_ATAPI_AR_M: c_uint = 0x04;
const CS4271_DACVOL_ATAPI_AR_BR: c_uint = 0x05;
const CS4271_DACVOL_ATAPI_AR_BL: c_uint = 0x06;
const CS4271_DACVOL_ATAPI_AR_BLR2: c_uint = 0x07;
const CS4271_DACVOL_ATAPI_AL_M: c_uint = 0x08;
const CS4271_DACVOL_ATAPI_AL_BR: c_uint = 0x09;
const CS4271_DACVOL_ATAPI_AL_BL: c_uint = 0x0A;
const CS4271_DACVOL_ATAPI_AL_BLR2: c_uint = 0x0B;
const CS4271_DACVOL_ATAPI_ALR2_M: c_uint = 0x0C;
const CS4271_DACVOL_ATAPI_ALR2_BR: c_uint = 0x0D;
const CS4271_DACVOL_ATAPI_ALR2_BL: c_uint = 0x0E;
const CS4271_DACVOL_ATAPI_ALR2_BLR2: c_uint = 0x0F;

const CS4271_VOLA_MUTE: c_uint = 0x80;
const CS4271_VOLA_VOL_MASK: c_uint = 0x7F;
const CS4271_VOLB_MUTE: c_uint = 0x80;
const CS4271_VOLB_VOL_MASK: c_uint = 0x7F;

const CS4271_ADCCTL_DITHER16: c_uint = 0x20;

const CS4271_ADCCTL_ADC_DIF_MASK: c_uint = 0x10;
const CS4271_ADCCTL_ADC_DIF_LJ: c_uint = 0x00;
const CS4271_ADCCTL_ADC_DIF_I2S: c_uint = 0x10;

const CS4271_ADCCTL_MUTEA: c_uint = 0x08;
const CS4271_ADCCTL_MUTEB: c_uint = 0x04;
const CS4271_ADCCTL_HPFDA: c_uint = 0x02;
const CS4271_ADCCTL_HPFDB: c_uint = 0x01;

const CS4271_MODE2_LOOP: c_uint = 0x10;
const CS4271_MODE2_MUTECAEQUB: c_uint = 0x08;
const CS4271_MODE2_FREEZE: c_uint = 0x04;
const CS4271_MODE2_CPEN: c_uint = 0x02;
const CS4271_MODE2_PDN: c_uint = 0x01;

const CS4271_CHIPID_PART_MASK: c_uint = 0xF0;
const CS4271_CHIPID_REV_MASK: c_uint = 0x0F;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device_with_data,
}
#[repr(C)]
pub struct device_with_data {
    pub platform_data: *mut c_void,
    pub of_node: *mut c_void,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
type c_long = isize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

/*
 * Default CS4271 power-up configuration
 * Array contains non-existing in hw register at address 0
 * Array do not include Chip ID, as codec driver does not use
 * registers read operations at all
 */
static cs4271_reg_defaults: [reg_default; 7] = [
    reg_default { reg: CS4271_MODE1, def: 0 },
    reg_default { reg: CS4271_DACCTL, def: CS4271_DACCTL_AMUTE },
    reg_default {
        reg: CS4271_DACVOL,
        def: CS4271_DACVOL_SOFT | CS4271_DACVOL_ATAPI_AL_BR,
    },
    reg_default { reg: CS4271_VOLA, def: 0 },
    reg_default { reg: CS4271_VOLB, def: 0 },
    reg_default { reg: CS4271_ADCCTL, def: 0 },
    reg_default { reg: CS4271_MODE2, def: 0 },
];

unsafe extern "C" fn cs4271_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == CS4271_CHIPID
}

static supply_names: [*const c_char; 3] = [c"vd".as_ptr(), c"vl".as_ptr(), c"va".as_ptr()];

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct cs4271_private {
    pub mclk: c_uint,
    pub master: bool,
    pub deemph: bool,
    pub regmap: *mut regmap,
    /* Current sample rate for de-emphasis control */
    pub rate: c_int,
    /* GPIO driving Reset pin, if any */
    pub reset: *mut gpio_desc,
    /* enable soft reset workaround */
    pub enable_soft_reset: bool,
    pub supplies: [regulator_bulk_data; 3],
    pub clk: *mut clk,
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
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
}
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
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

unsafe extern "C" {
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static GPIOD_ASIS: c_int;
    static REGCACHE_FLAT: c_uint;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dai_stream_active(dai: *mut snd_soc_dai, stream: c_int) -> bool;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn gpiod_direction_output(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn mdelay(msecs: c_uint);
    fn udelay(usecs: c_uint);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn of_property_read_bool(np: *mut c_void, propname: *const c_char) -> bool;
    fn dev_err(dev: *mut device_with_data, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
}

/* DAPM widget macro expansions are supplied by the surrounding ASoC bindings. */
static cs4271_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_INPUT(c"AINA".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AINB".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"AOUTA+".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"AOUTA-".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"AOUTB+".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"AOUTB-".as_ptr()),
];

static cs4271_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: ptr::null(), source: c"AINA".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: ptr::null(), source: c"AINB".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUTA+".as_ptr(), control: ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUTA-".as_ptr(), control: ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUTB+".as_ptr(), control: ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUTB-".as_ptr(), control: ptr::null(), source: c"Playback".as_ptr() },
];

/*
 * @freq is the desired MCLK rate
 * MCLK rate should (c) be the sample rate, multiplied by one of the
 * ratios listed in cs4271_mclk_fs_ratios table
 */
unsafe extern "C" fn cs4271_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;

    (*cs4271).mclk = freq;
    0
}

unsafe extern "C" fn cs4271_set_dai_fmt(codec_dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    match format & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            (*cs4271).master = false;
        }
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            (*cs4271).master = true;
            val |= CS4271_MODE1_MASTER;
        }
        _ => {
            dev_err((*component).dev, c"Invalid DAI format\n".as_ptr());
            return -EINVAL;
        }
    }

    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            val |= CS4271_MODE1_DAC_DIF_LJ;
            ret = regmap_update_bits(
                (*cs4271).regmap,
                CS4271_ADCCTL,
                CS4271_ADCCTL_ADC_DIF_MASK,
                CS4271_ADCCTL_ADC_DIF_LJ,
            );
            if ret < 0 {
                return ret;
            }
        }
        x if x == SND_SOC_DAIFMT_I2S => {
            val |= CS4271_MODE1_DAC_DIF_I2S;
            ret = regmap_update_bits(
                (*cs4271).regmap,
                CS4271_ADCCTL,
                CS4271_ADCCTL_ADC_DIF_MASK,
                CS4271_ADCCTL_ADC_DIF_I2S,
            );
            if ret < 0 {
                return ret;
            }
        }
        _ => {
            dev_err((*component).dev, c"Invalid DAI format\n".as_ptr());
            return -EINVAL;
        }
    }

    ret = regmap_update_bits(
        (*cs4271).regmap,
        CS4271_MODE1,
        CS4271_MODE1_DAC_DIF_MASK | CS4271_MODE1_MASTER,
        val,
    );
    if ret < 0 {
        return ret;
    }
    0
}

static mut cs4271_deemph: [c_int; 4] = [0, 44100, 48000, 32000];

unsafe extern "C" fn cs4271_set_deemph(component: *mut snd_soc_component) -> c_int {
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;
    let mut i: usize;
    let ret: c_int;
    let mut val: c_int = CS4271_DACCTL_DEM_DIS as c_int;

    if (*cs4271).deemph {
        /* Find closest de-emphasis freq */
        val = 1;
        i = 2;
        while i < cs4271_deemph.len() {
            if (cs4271_deemph[i] - (*cs4271).rate).abs()
                < (cs4271_deemph[val as usize] - (*cs4271).rate).abs()
            {
                val = i as c_int;
            }
            i += 1;
        }
        val <<= 4;
    }

    ret = regmap_update_bits(
        (*cs4271).regmap,
        CS4271_DACCTL,
        CS4271_DACCTL_DEM_MASK,
        val as c_uint,
    );
    if ret < 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn cs4271_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;

    (*ucontrol).value.integer.value[0] = (*cs4271).deemph as c_long;
    0
}

unsafe extern "C" fn cs4271_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;

    (*cs4271).deemph = (*ucontrol).value.integer.value[0] != 0;
    cs4271_set_deemph(component)
}

#[repr(C)]
pub struct cs4271_clk_cfg {
    pub master: bool,       /* codec mode */
    pub speed_mode: u8,     /* codec speed mode: 1x, 2x, 4x */
    pub ratio: c_ushort,    /* MCLK / sample rate */
    pub ratio_mask: u8,     /* ratio bit mask for Master mode */
}

static mut cs4271_clk_tab: [cs4271_clk_cfg; 27] = [
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 256, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 384, ratio_mask: CS4271_MODE1_DIV_15 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 512, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 768, ratio_mask: CS4271_MODE1_DIV_3 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 128, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 192, ratio_mask: CS4271_MODE1_DIV_15 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 256, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 384, ratio_mask: CS4271_MODE1_DIV_3 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 64, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 96, ratio_mask: CS4271_MODE1_DIV_15 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 128, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
    cs4271_clk_cfg { master: true, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 192, ratio_mask: CS4271_MODE1_DIV_3 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 256, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 384, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 512, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 768, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_1X as u8, ratio: 1024, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 128, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 192, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 256, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 384, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_2X as u8, ratio: 512, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 64, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 96, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 128, ratio_mask: CS4271_MODE1_DIV_1 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 192, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
    cs4271_clk_cfg { master: false, speed_mode: CS4271_MODE1_MODE_4X as u8, ratio: 256, ratio_mask: CS4271_MODE1_DIV_2 as u8 },
];

const CS4271_NR_RATIOS: usize = 27;

unsafe extern "C" fn cs4271_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;
    let mut i: usize;
    let mut ret: c_int;
    let ratio: c_uint;
    let mut val: c_uint;

    if (*cs4271).enable_soft_reset {
        /*
         * Put the codec in soft reset and back again in case it's not
         * currently streaming data. This way of bringing the codec in
         * sync to the current clocks is not explicitly documented in
         * the data sheet, but it seems to work fine, and in contrast
         * to a read hardware reset, we don't have to sync back all
         * registers every time.
         */

        if ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK
            && !snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_CAPTURE))
            || ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE
                && !snd_soc_dai_stream_active(dai, SNDRV_PCM_STREAM_PLAYBACK))
        {
            ret = regmap_update_bits((*cs4271).regmap, CS4271_MODE2, CS4271_MODE2_PDN, CS4271_MODE2_PDN);
            if ret < 0 {
                return ret;
            }

            ret = regmap_update_bits((*cs4271).regmap, CS4271_MODE2, CS4271_MODE2_PDN, 0);
            if ret < 0 {
                return ret;
            }
        }
    }

    (*cs4271).rate = params_rate(params) as c_int;

    /* Configure DAC */
    if (*cs4271).rate < 50000 {
        val = CS4271_MODE1_MODE_1X;
    } else if (*cs4271).rate < 100000 {
        val = CS4271_MODE1_MODE_2X;
    } else {
        val = CS4271_MODE1_MODE_4X;
    }

    ratio = (*cs4271).mclk / (*cs4271).rate as c_uint;
    i = 0;
    while i < CS4271_NR_RATIOS {
        if cs4271_clk_tab[i].master == (*cs4271).master
            && cs4271_clk_tab[i].speed_mode as c_uint == val
            && cs4271_clk_tab[i].ratio as c_uint == ratio
        {
            break;
        }
        i += 1;
    }

    if i == CS4271_NR_RATIOS {
        dev_err((*component).dev, c"Invalid sample rate\n".as_ptr());
        return -EINVAL;
    }

    val |= cs4271_clk_tab[i].ratio_mask as c_uint;

    ret = regmap_update_bits(
        (*cs4271).regmap,
        CS4271_MODE1,
        CS4271_MODE1_MODE_MASK | CS4271_MODE1_DIV_MASK,
        val,
    );
    if ret < 0 {
        return ret;
    }

    cs4271_set_deemph(component)
}

unsafe extern "C" fn cs4271_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    stream: c_int,
) -> c_int {
    let component = (*dai).component;
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;
    let mut ret: c_int;
    let mut val_a: c_int = 0;
    let mut val_b: c_int = 0;

    if stream != SNDRV_PCM_STREAM_PLAYBACK {
        return 0;
    }

    if mute != 0 {
        val_a = CS4271_VOLA_MUTE as c_int;
        val_b = CS4271_VOLB_MUTE as c_int;
    }

    ret = regmap_update_bits((*cs4271).regmap, CS4271_VOLA, CS4271_VOLA_MUTE, val_a as c_uint);
    if ret < 0 {
        return ret;
    }

    ret = regmap_update_bits((*cs4271).regmap, CS4271_VOLB, CS4271_VOLB_MUTE, val_b as c_uint);
    if ret < 0 {
        return ret;
    }

    0
}

/* CS4271 controls */
static cs4271_dac_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE(-12700, 100, 0);

static cs4271_snd_controls: [snd_kcontrol_new; 15] = [
    SOC_DOUBLE_R_TLV(c"Master Playback Volume".as_ptr(), CS4271_VOLA, CS4271_VOLB, 0, 0x7F, 1, cs4271_dac_tlv.as_ptr()),
    SOC_SINGLE(c"Digital Loopback Switch".as_ptr(), CS4271_MODE2, 4, 1, 0),
    SOC_SINGLE(c"Soft Ramp Switch".as_ptr(), CS4271_DACVOL, 5, 1, 0),
    SOC_SINGLE(c"Zero Cross Switch".as_ptr(), CS4271_DACVOL, 4, 1, 0),
    SOC_SINGLE_BOOL_EXT(c"De-emphasis Switch".as_ptr(), 0, Some(cs4271_get_deemph), Some(cs4271_put_deemph)),
    SOC_SINGLE(c"Auto-Mute Switch".as_ptr(), CS4271_DACCTL, 7, 1, 0),
    SOC_SINGLE(c"Slow Roll Off Filter Switch".as_ptr(), CS4271_DACCTL, 6, 1, 0),
    SOC_SINGLE(c"Soft Volume Ramp-Up Switch".as_ptr(), CS4271_DACCTL, 3, 1, 0),
    SOC_SINGLE(c"Soft Ramp-Down Switch".as_ptr(), CS4271_DACCTL, 2, 1, 0),
    SOC_SINGLE(c"Left Channel Inversion Switch".as_ptr(), CS4271_DACCTL, 1, 1, 0),
    SOC_SINGLE(c"Right Channel Inversion Switch".as_ptr(), CS4271_DACCTL, 0, 1, 0),
    SOC_DOUBLE(c"Master Capture Switch".as_ptr(), CS4271_ADCCTL, 3, 2, 1, 1),
    SOC_SINGLE(c"Dither 16-Bit Data Switch".as_ptr(), CS4271_ADCCTL, 5, 1, 0),
    SOC_DOUBLE(c"High Pass Filter Switch".as_ptr(), CS4271_ADCCTL, 1, 0, 1, 1),
    SOC_DOUBLE_R(c"Master Playback Switch".as_ptr(), CS4271_VOLA, CS4271_VOLB, 7, 1, 1),
];

static cs4271_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cs4271_hw_params),
    set_sysclk: Some(cs4271_set_dai_sysclk),
    set_fmt: Some(cs4271_set_dai_fmt),
    mute_stream: Some(cs4271_mute_stream),
};

static mut cs4271_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"cs4271-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: CS4271_PCM_RATES,
        formats: CS4271_PCM_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: CS4271_PCM_RATES,
        formats: CS4271_PCM_FORMATS,
    },
    ops: &cs4271_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn cs4271_reset(component: *mut snd_soc_component) -> c_int {
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;

    gpiod_direction_output((*cs4271).reset, 1);
    mdelay(1);
    gpiod_set_value((*cs4271).reset, 0);
    mdelay(1);

    0
}

/* CONFIG_PM: suspend/resume callbacks are conditionally built in C. */
unsafe extern "C" fn cs4271_soc_suspend(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;

    /* Set power-down bit */
    ret = regmap_update_bits((*cs4271).regmap, CS4271_MODE2, CS4271_MODE2_PDN, CS4271_MODE2_PDN);
    if ret < 0 {
        return ret;
    }

    regcache_mark_dirty((*cs4271).regmap);
    clk_disable_unprepare((*cs4271).clk);
    regulator_bulk_disable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());

    0
}

unsafe extern "C" fn cs4271_soc_resume(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;

    ret = regulator_bulk_enable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*component).dev, c"Failed to enable regulators: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = clk_prepare_enable((*cs4271).clk);
    if ret != 0 {
        dev_err((*component).dev, c"Failed to enable clk: %d\n".as_ptr(), ret);
        goto_err_disable_regulators(component, cs4271, ret)
    } else {
        /* Do a proper reset after power up */
        cs4271_reset(component);

        /* Restore codec state */
        ret = regcache_sync((*cs4271).regmap);
        if ret < 0 {
            goto_err_disable_clk(component, cs4271, ret)
        } else {
            /* then disable the power-down bit */
            ret = regmap_update_bits((*cs4271).regmap, CS4271_MODE2, CS4271_MODE2_PDN, 0);
            if ret < 0 {
                goto_err_disable_clk(component, cs4271, ret)
            } else {
                0
            }
        }
    }
}

unsafe fn goto_err_disable_clk(
    component: *mut snd_soc_component,
    cs4271: *mut cs4271_private,
    ret: c_int,
) -> c_int {
    clk_disable_unprepare((*cs4271).clk);
    goto_err_disable_regulators(component, cs4271, ret)
}

unsafe fn goto_err_disable_regulators(
    _component: *mut snd_soc_component,
    cs4271: *mut cs4271_private,
    ret: c_int,
) -> c_int {
    regulator_bulk_disable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());
    ret
}

/* CONFIG_OF */
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

pub static cs4271_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"cirrus,cs4271".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, cs4271_dt_ids); */
/* EXPORT_SYMBOL_GPL(cs4271_dt_ids); */

#[repr(C)]
pub struct cs4271_platform_data {
    pub amutec_eq_bmutec: bool,
    pub enable_soft_reset: bool,
}

unsafe extern "C" fn cs4271_component_probe(component: *mut snd_soc_component) -> c_int {
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;
    let cs4271plat = (*(*component).dev).platform_data as *mut cs4271_platform_data;
    let mut ret: c_int;
    let mut amutec_eq_bmutec: bool;

    amutec_eq_bmutec =
        of_property_read_bool((*(*component).dev).of_node, c"cirrus,amutec-eq-bmutec".as_ptr());
    (*cs4271).enable_soft_reset =
        of_property_read_bool((*(*component).dev).of_node, c"cirrus,enable-soft-reset".as_ptr());

    ret = regulator_bulk_enable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*component).dev, c"Failed to enable regulators: %d\n".as_ptr(), ret);
        return ret;
    }

    if !cs4271plat.is_null() {
        amutec_eq_bmutec = (*cs4271plat).amutec_eq_bmutec;
        (*cs4271).enable_soft_reset = (*cs4271plat).enable_soft_reset;
    }

    ret = clk_prepare_enable((*cs4271).clk);
    if ret != 0 {
        dev_err((*component).dev, c"Failed to enable clk: %d\n".as_ptr(), ret);
        regulator_bulk_disable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());
        return ret;
    }

    /* Reset codec */
    cs4271_reset(component);

    ret = regcache_sync((*cs4271).regmap);
    if ret < 0 {
        clk_disable_unprepare((*cs4271).clk);
        regulator_bulk_disable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());
        return ret;
    }

    ret = regmap_update_bits(
        (*cs4271).regmap,
        CS4271_MODE2,
        CS4271_MODE2_PDN | CS4271_MODE2_CPEN,
        CS4271_MODE2_PDN | CS4271_MODE2_CPEN,
    );
    if ret < 0 {
        clk_disable_unprepare((*cs4271).clk);
        regulator_bulk_disable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());
        return ret;
    }

    ret = regmap_update_bits((*cs4271).regmap, CS4271_MODE2, CS4271_MODE2_PDN, 0);
    if ret < 0 {
        clk_disable_unprepare((*cs4271).clk);
        regulator_bulk_disable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());
        return ret;
    }

    /* Power-up sequence requires 85 uS */
    udelay(85);

    if amutec_eq_bmutec {
        regmap_update_bits(
            (*cs4271).regmap,
            CS4271_MODE2,
            CS4271_MODE2_MUTECAEQUB,
            CS4271_MODE2_MUTECAEQUB,
        );
    }

    0
}

unsafe extern "C" fn cs4271_component_remove(component: *mut snd_soc_component) {
    let cs4271 = snd_soc_component_get_drvdata(component) as *mut cs4271_private;

    /* Set codec to the reset state */
    gpiod_set_value((*cs4271).reset, 1);

    regcache_mark_dirty((*cs4271).regmap);
    regulator_bulk_disable((*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());
    clk_disable_unprepare((*cs4271).clk);
}

static soc_component_dev_cs4271: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs4271_component_probe),
    remove: Some(cs4271_component_remove),
    suspend: Some(cs4271_soc_suspend),
    resume: Some(cs4271_soc_resume),
    controls: cs4271_snd_controls.as_ptr(),
    num_controls: 15,
    dapm_widgets: cs4271_dapm_widgets.as_ptr(),
    num_dapm_widgets: 6,
    dapm_routes: cs4271_dapm_routes.as_ptr(),
    num_dapm_routes: 6,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn cs4271_common_probe(dev: *mut device, c: *mut *mut cs4271_private) -> c_int {
    let cs4271: *mut cs4271_private;
    let mut i: usize;
    let mut ret: c_int;

    cs4271 = devm_kzalloc(dev, core::mem::size_of::<cs4271_private>(), GFP_KERNEL) as *mut cs4271_private;
    if cs4271.is_null() {
        return -ENOMEM;
    }

    (*cs4271).reset = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_ASIS);
    if IS_ERR((*cs4271).reset as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*cs4271).reset as *const c_void),
            c"error retrieving RESET GPIO\n".as_ptr(),
        );
    }
    gpiod_set_consumer_name((*cs4271).reset, c"CS4271 Reset".as_ptr());

    (*cs4271).clk = devm_clk_get_optional(dev, c"mclk".as_ptr());
    if IS_ERR((*cs4271).clk as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*cs4271).clk as *const c_void), c"Failed to get mclk\n".as_ptr());
    }

    i = 0;
    while i < supply_names.len() {
        (*cs4271).supplies[i].supply = supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*cs4271).supplies.len() as c_int, (*cs4271).supplies.as_mut_ptr());

    if ret < 0 {
        dev_err(dev as *mut device_with_data, c"Failed to get regulators: %d\n".as_ptr(), ret);
        return ret;
    }

    *c = cs4271;
    0
}

#[repr(C)]
pub struct regmap_config {
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub val_bits: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}

pub static cs4271_regmap_config: regmap_config = regmap_config {
    max_register: CS4271_LASTREG,

    reg_defaults: cs4271_reg_defaults.as_ptr(),
    num_reg_defaults: 7,
    cache_type: REGCACHE_FLAT,
    val_bits: 8,
    volatile_reg: Some(cs4271_volatile_reg),
};
/* EXPORT_SYMBOL_GPL(cs4271_regmap_config); */

#[no_mangle]
pub unsafe extern "C" fn cs4271_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    let mut cs4271: *mut cs4271_private = ptr::null_mut();
    let mut ret: c_int;

    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void) as c_int;
    }

    ret = cs4271_common_probe(dev, &mut cs4271);
    if ret < 0 {
        return ret;
    }

    dev_set_drvdata(dev, cs4271 as *mut c_void);
    (*cs4271).regmap = regmap;

    devm_snd_soc_register_component(dev, &soc_component_dev_cs4271, &mut cs4271_dai, 1)
}
/* EXPORT_SYMBOL_GPL(cs4271_probe); */

/* MODULE_AUTHOR("Alexander Sverdlin <subaparts@yandex.ru>"); */
/* MODULE_DESCRIPTION("Cirrus Logic CS4271 ALSA SoC Codec Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
