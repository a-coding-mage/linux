// SPDX-License-Identifier: GPL-2.0
//
// Driver for the TAS5805M Audio Amplifier
//
// Author: Andy Liu <andy-liu@ti.com>
// Author: Daniel Beer <daniel.beer@igorinstitute.com>
//
// This is based on a driver originally written by Andy Liu at TI and
// posted here:
//
//    https://e2e.ti.com/support/audio-group/audio/f/audio-forum/722027/linux-tas5825m-linux-drivers
//
// It has been simplified a little and reworked for the 5.x ALSA SoC API.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Datasheet-defined registers on page 0, book 0 */
const REG_PAGE: c_uint = 0x00;
const REG_DEVICE_CTRL_1: c_uint = 0x02;
const REG_DEVICE_CTRL_2: c_uint = 0x03;
const REG_SIG_CH_CTRL: c_uint = 0x28;
const REG_SAP_CTRL_1: c_uint = 0x33;
const REG_FS_MON: c_uint = 0x37;
const REG_BCK_MON: c_uint = 0x38;
const REG_CLKDET_STATUS: c_uint = 0x39;
const REG_VOL_CTL: c_uint = 0x4c;
const REG_AGAIN: c_uint = 0x54;
const REG_ADR_PIN_CTRL: c_uint = 0x60;
const REG_ADR_PIN_CONFIG: c_uint = 0x61;
const REG_CHAN_FAULT: c_uint = 0x70;
const REG_GLOBAL_FAULT1: c_uint = 0x71;
const REG_GLOBAL_FAULT2: c_uint = 0x72;
const REG_FAULT: c_uint = 0x78;
const REG_BOOK: c_uint = 0x7f;

/* DEVICE_CTRL_2 register values */
const DCTRL2_MODE_DEEP_SLEEP: c_uint = 0x00;
const DCTRL2_MODE_SLEEP: c_uint = 0x01;
const DCTRL2_MODE_HIZ: c_uint = 0x02;
const DCTRL2_MODE_PLAY: c_uint = 0x03;

const DCTRL2_MUTE: c_uint = 0x08;
const DCTRL2_DIS_DSP: c_uint = 0x10;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const REGCACHE_NONE: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 0;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 0;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 0;
const SND_SOC_DAPM_PRE_PMD: c_int = 0;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 0;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct i2c_client {
    dev: device,
}

#[repr(C)]
struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
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
struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}

#[repr(C)]
union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 2],
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    access: c_uint,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_dapm_widget_c {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component_driver {
    controls: *const snd_kcontrol_new,
    num_controls: usize,
    dapm_widgets: *const snd_soc_dapm_widget_c,
    num_dapm_widgets: usize,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: usize,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    no_capture_mute: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    cache_type: c_uint,
}

#[repr(C)]
struct firmware {
    size: usize,
    data: *const u8,
}

#[repr(C)]
struct i2c_device_id {
    name: [c_char; 32],
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct i2c_driver_inner {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct i2c_driver {
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    id_table: *const i2c_device_id,
    driver: i2c_driver_inner,
}

/* This sequence of register writes must always be sent, prior to the
 * 5ms delay while we wait for the DSP to boot.
 */
static dsp_cfg_preboot: [u8; 22] = [
    0x00, 0x00, 0x7f, 0x00, 0x03, 0x02, 0x01, 0x11,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7f, 0x00, 0x03, 0x02,
];

static tas5805m_volume: [u32; 159] = [
    0x0000001B, /*   0, -110dB */ 0x0000001E, /*   1, -109dB */
    0x00000021, /*   2, -108dB */ 0x00000025, /*   3, -107dB */
    0x0000002A, /*   4, -106dB */ 0x0000002F, /*   5, -105dB */
    0x00000035, /*   6, -104dB */ 0x0000003B, /*   7, -103dB */
    0x00000043, /*   8, -102dB */ 0x0000004B, /*   9, -101dB */
    0x00000054, /*  10, -100dB */ 0x0000005E, /*  11,  -99dB */
    0x0000006A, /*  12,  -98dB */ 0x00000076, /*  13,  -97dB */
    0x00000085, /*  14,  -96dB */ 0x00000095, /*  15,  -95dB */
    0x000000A7, /*  16,  -94dB */ 0x000000BC, /*  17,  -93dB */
    0x000000D3, /*  18,  -92dB */ 0x000000EC, /*  19,  -91dB */
    0x00000109, /*  20,  -90dB */ 0x0000012A, /*  21,  -89dB */
    0x0000014E, /*  22,  -88dB */ 0x00000177, /*  23,  -87dB */
    0x000001A4, /*  24,  -86dB */ 0x000001D8, /*  25,  -85dB */
    0x00000211, /*  26,  -84dB */ 0x00000252, /*  27,  -83dB */
    0x0000029A, /*  28,  -82dB */ 0x000002EC, /*  29,  -81dB */
    0x00000347, /*  30,  -80dB */ 0x000003AD, /*  31,  -79dB */
    0x00000420, /*  32,  -78dB */ 0x000004A1, /*  33,  -77dB */
    0x00000532, /*  34,  -76dB */ 0x000005D4, /*  35,  -75dB */
    0x0000068A, /*  36,  -74dB */ 0x00000756, /*  37,  -73dB */
    0x0000083B, /*  38,  -72dB */ 0x0000093C, /*  39,  -71dB */
    0x00000A5D, /*  40,  -70dB */ 0x00000BA0, /*  41,  -69dB */
    0x00000D0C, /*  42,  -68dB */ 0x00000EA3, /*  43,  -67dB */
    0x0000106C, /*  44,  -66dB */ 0x0000126D, /*  45,  -65dB */
    0x000014AD, /*  46,  -64dB */ 0x00001733, /*  47,  -63dB */
    0x00001A07, /*  48,  -62dB */ 0x00001D34, /*  49,  -61dB */
    0x000020C5, /*  50,  -60dB */ 0x000024C4, /*  51,  -59dB */
    0x00002941, /*  52,  -58dB */ 0x00002E49, /*  53,  -57dB */
    0x000033EF, /*  54,  -56dB */ 0x00003A45, /*  55,  -55dB */
    0x00004161, /*  56,  -54dB */ 0x0000495C, /*  57,  -53dB */
    0x0000524F, /*  58,  -52dB */ 0x00005C5A, /*  59,  -51dB */
    0x0000679F, /*  60,  -50dB */ 0x00007444, /*  61,  -49dB */
    0x00008274, /*  62,  -48dB */ 0x0000925F, /*  63,  -47dB */
    0x0000A43B, /*  64,  -46dB */ 0x0000B845, /*  65,  -45dB */
    0x0000CEC1, /*  66,  -44dB */ 0x0000E7FB, /*  67,  -43dB */
    0x00010449, /*  68,  -42dB */ 0x0001240C, /*  69,  -41dB */
    0x000147AE, /*  70,  -40dB */ 0x00016FAA, /*  71,  -39dB */
    0x00019C86, /*  72,  -38dB */ 0x0001CEDC, /*  73,  -37dB */
    0x00020756, /*  74,  -36dB */ 0x000246B5, /*  75,  -35dB */
    0x00028DCF, /*  76,  -34dB */ 0x0002DD96, /*  77,  -33dB */
    0x00033718, /*  78,  -32dB */ 0x00039B87, /*  79,  -31dB */
    0x00040C37, /*  80,  -30dB */ 0x00048AA7, /*  81,  -29dB */
    0x00051884, /*  82,  -28dB */ 0x0005B7B1, /*  83,  -27dB */
    0x00066A4A, /*  84,  -26dB */ 0x000732AE, /*  85,  -25dB */
    0x00081385, /*  86,  -24dB */ 0x00090FCC, /*  87,  -23dB */
    0x000A2ADB, /*  88,  -22dB */ 0x000B6873, /*  89,  -21dB */
    0x000CCCCD, /*  90,  -20dB */ 0x000E5CA1, /*  91,  -19dB */
    0x00101D3F, /*  92,  -18dB */ 0x0012149A, /*  93,  -17dB */
    0x00144961, /*  94,  -16dB */ 0x0016C311, /*  95,  -15dB */
    0x00198A13, /*  96,  -14dB */ 0x001CA7D7, /*  97,  -13dB */
    0x002026F3, /*  98,  -12dB */ 0x00241347, /*  99,  -11dB */
    0x00287A27, /* 100,  -10dB */ 0x002D6A86, /* 101,  -9dB */
    0x0032F52D, /* 102,   -8dB */ 0x00392CEE, /* 103,   -7dB */
    0x004026E7, /* 104,   -6dB */ 0x0047FACD, /* 105,   -5dB */
    0x0050C336, /* 106,   -4dB */ 0x005A9DF8, /* 107,   -3dB */
    0x0065AC8C, /* 108,   -2dB */ 0x00721483, /* 109,   -1dB */
    0x00800000, /* 110,    0dB */ 0x008F9E4D, /* 111,    1dB */
    0x00A12478, /* 112,    2dB */ 0x00B4CE08, /* 113,    3dB */
    0x00CADDC8, /* 114,    4dB */ 0x00E39EA9, /* 115,    5dB */
    0x00FF64C1, /* 116,    6dB */ 0x011E8E6A, /* 117,    7dB */
    0x0141857F, /* 118,    8dB */ 0x0168C0C6, /* 119,    9dB */
    0x0194C584, /* 120,   10dB */ 0x01C62940, /* 121,   11dB */
    0x01FD93C2, /* 122,   12dB */ 0x023BC148, /* 123,   13dB */
    0x02818508, /* 124,   14dB */ 0x02CFCC01, /* 125,   15dB */
    0x0327A01A, /* 126,   16dB */ 0x038A2BAD, /* 127,   17dB */
    0x03F8BD7A, /* 128,   18dB */ 0x0474CD1B, /* 129,   19dB */
    0x05000000, /* 130,   20dB */ 0x059C2F02, /* 131,   21dB */
    0x064B6CAE, /* 132,   22dB */ 0x07100C4D, /* 133,   23dB */
    0x07ECA9CD, /* 134,   24dB */ 0x08E43299, /* 135,   25dB */
    0x09F9EF8E, /* 136,   26dB */ 0x0B319025, /* 137,   27dB */
    0x0C8F36F2, /* 138,   28dB */ 0x0E1787B8, /* 139,   29dB */
    0x0FCFB725, /* 140,   30dB */ 0x11BD9C84, /* 141,   31dB */
    0x13E7C594, /* 142,   32dB */ 0x16558CCB, /* 143,   33dB */
    0x190F3254, /* 144,   34dB */ 0x1C1DF80E, /* 145,   35dB */
    0x1F8C4107, /* 146,   36dB */ 0x2365B4BF, /* 147,   37dB */
    0x27B766C2, /* 148,   38dB */ 0x2C900313, /* 149,   39dB */
    0x32000000, /* 150,   40dB */ 0x3819D612, /* 151,   41dB */
    0x3EF23ECA, /* 152,   42dB */ 0x46A07B07, /* 153,   43dB */
    0x4F3EA203, /* 154,   44dB */ 0x58E9F9F9, /* 155,   45dB */
    0x63C35B8E, /* 156,   46dB */ 0x6FEFA16D, /* 157,   47dB */
    0x7D982575, /* 158,   48dB */
];

const TAS5805M_VOLUME_MAX: c_int = tas5805m_volume.len() as c_int - 1;
const TAS5805M_VOLUME_MIN: c_int = 0;

#[repr(C)]
struct tas5805m_priv {
    i2c: *mut i2c_client,
    pvdd: *mut regulator,
    gpio_pdn_n: *mut gpio_desc,

    dsp_cfg_data: *mut u8,
    dsp_cfg_len: c_int,

    regmap: *mut regmap,

    vol: [c_int; 2],
    is_powered: bool,
    is_muted: bool,

    work: work_struct,
    lock: mutex,
}

extern "C" {
    fn regmap_bulk_write(map: *mut regmap, reg: c_uint, val: *const c_void, val_count: usize) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn usleep_range(min: c_uint, max: c_uint);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn device_property_read_string(dev: *mut device, propname: *const c_char, val: *mut *const c_char) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn module_i2c_driver(driver: *mut i2c_driver);
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn dev_dbg(_dev: *mut device, _fmt: *const c_char, _args: ...) {}
unsafe fn dev_err(_dev: *mut device, _fmt: *const c_char, _args: ...) {}

unsafe fn set_dsp_scale(rm: *mut regmap, offset: c_int, vol: c_int) {
    let mut v: [u8; 4] = [0; 4];
    let mut x: u32 = tas5805m_volume[vol as usize];
    let mut i: c_int;

    i = 0;
    while i < 4 {
        v[(3 - i) as usize] = x as u8;
        x >>= 8;
        i += 1;
    }

    regmap_bulk_write(rm, offset as c_uint, v.as_ptr() as *const c_void, v.len());
}

unsafe fn tas5805m_refresh(tas5805m: *mut tas5805m_priv) {
    let rm: *mut regmap = (*tas5805m).regmap;

    dev_dbg(
        &mut (*(*tas5805m).i2c).dev,
        c_str!("refresh: is_muted=%d, vol=%d/%d\n"),
        (*tas5805m).is_muted as c_int,
        (*tas5805m).vol[0],
        (*tas5805m).vol[1],
    );

    regmap_write(rm, REG_PAGE, 0x00);
    regmap_write(rm, REG_BOOK, 0x8c);
    regmap_write(rm, REG_PAGE, 0x2a);

    /* Refresh volume. The actual volume control documented in the
     * datasheet doesn't seem to work correctly. This is a pair of
     * DSP registers which are *not* documented in the datasheet.
     */
    set_dsp_scale(rm, 0x24, (*tas5805m).vol[0]);
    set_dsp_scale(rm, 0x28, (*tas5805m).vol[1]);

    regmap_write(rm, REG_PAGE, 0x00);
    regmap_write(rm, REG_BOOK, 0x00);

    /* Set/clear digital soft-mute */
    regmap_write(
        rm,
        REG_DEVICE_CTRL_2,
        (if (*tas5805m).is_muted { DCTRL2_MUTE } else { 0 }) | DCTRL2_MODE_PLAY,
    );
}

unsafe extern "C" fn tas5805m_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;

    (*uinfo).value.integer.min = TAS5805M_VOLUME_MIN as c_long;
    (*uinfo).value.integer.max = TAS5805M_VOLUME_MAX as c_long;
    0
}

unsafe extern "C" fn tas5805m_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let tas5805m: *mut tas5805m_priv =
        snd_soc_component_get_drvdata(component) as *mut tas5805m_priv;

    mutex_lock(&mut (*tas5805m).lock);
    (*ucontrol).value.integer.value[0] = (*tas5805m).vol[0] as c_long;
    (*ucontrol).value.integer.value[1] = (*tas5805m).vol[1] as c_long;
    mutex_unlock(&mut (*tas5805m).lock);

    0
}

#[inline]
fn volume_is_valid(v: c_int) -> bool {
    (v >= TAS5805M_VOLUME_MIN) && (v <= TAS5805M_VOLUME_MAX)
}

unsafe extern "C" fn tas5805m_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let tas5805m: *mut tas5805m_priv =
        snd_soc_component_get_drvdata(component) as *mut tas5805m_priv;

    if !(volume_is_valid((*ucontrol).value.integer.value[0] as c_int)
        && volume_is_valid((*ucontrol).value.integer.value[1] as c_int))
    {
        return -EINVAL;
    }

    mutex_lock(&mut (*tas5805m).lock);
    if (*tas5805m).vol[0] != (*ucontrol).value.integer.value[0] as c_int
        || (*tas5805m).vol[1] != (*ucontrol).value.integer.value[1] as c_int
    {
        (*tas5805m).vol[0] = (*ucontrol).value.integer.value[0] as c_int;
        (*tas5805m).vol[1] = (*ucontrol).value.integer.value[1] as c_int;
        dev_dbg(
            (*component).dev,
            c_str!("set vol=%d/%d (is_powered=%d)\n"),
            (*tas5805m).vol[0],
            (*tas5805m).vol[1],
            (*tas5805m).is_powered as c_int,
        );
        if (*tas5805m).is_powered {
            tas5805m_refresh(tas5805m);
        }
        mutex_unlock(&mut (*tas5805m).lock);
        return 1;
    }

    mutex_unlock(&mut (*tas5805m).lock);
    0
}

static tas5805m_snd_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Master Playback Volume"),
    access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas5805m_vol_info),
    get: Some(tas5805m_vol_get),
    put: Some(tas5805m_vol_put),
}];

unsafe fn send_cfg(rm: *mut regmap, s: *const u8, len: c_uint) {
    let mut i: c_uint;

    i = 0;
    while i + 1 < len {
        regmap_write(rm, *s.add(i as usize) as c_uint, *s.add((i + 1) as usize) as c_uint);
        i += 2;
    }
}

/* The TAS5805M DSP can't be configured until the I2S clock has been
 * present and stable for 5ms, or else it won't boot and we get no
 * sound.
 */
unsafe extern "C" fn tas5805m_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let tas5805m: *mut tas5805m_priv =
        snd_soc_component_get_drvdata(component) as *mut tas5805m_priv;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            dev_dbg((*component).dev, c_str!("clock start\n"));
            schedule_work(&mut (*tas5805m).work);
        }

        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {}

        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn do_work(work: *mut work_struct) {
    let tas5805m: *mut tas5805m_priv =
        (work as *mut u8).sub(offset_of_tas5805m_priv_work()) as *mut tas5805m_priv;
    let rm: *mut regmap = (*tas5805m).regmap;

    dev_dbg(&mut (*(*tas5805m).i2c).dev, c_str!("DSP startup\n"));

    mutex_lock(&mut (*tas5805m).lock);
    /* We mustn't issue any I2C transactions until the I2S
     * clock is stable. Furthermore, we must allow a 5ms
     * delay after the first set of register writes to
     * allow the DSP to boot before configuring it.
     */
    usleep_range(5000, 10000);
    send_cfg(rm, dsp_cfg_preboot.as_ptr(), dsp_cfg_preboot.len() as c_uint);
    usleep_range(5000, 15000);
    send_cfg(rm, (*tas5805m).dsp_cfg_data, (*tas5805m).dsp_cfg_len as c_uint);

    (*tas5805m).is_powered = true;
    tas5805m_refresh(tas5805m);
    mutex_unlock(&mut (*tas5805m).lock);
}

fn offset_of_tas5805m_priv_work() -> usize {
    let uninit = core::mem::MaybeUninit::<tas5805m_priv>::uninit();
    let base = uninit.as_ptr();
    unsafe { (&(*base).work as *const work_struct as usize) - (base as usize) }
}

unsafe extern "C" fn tas5805m_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let tas5805m: *mut tas5805m_priv =
        snd_soc_component_get_drvdata(component) as *mut tas5805m_priv;
    let rm: *mut regmap = (*tas5805m).regmap;

    if event & SND_SOC_DAPM_PRE_PMD != 0 {
        let mut chan: c_uint = 0;
        let mut global1: c_uint = 0;
        let mut global2: c_uint = 0;

        dev_dbg((*component).dev, c_str!("DSP shutdown\n"));
        cancel_work_sync(&mut (*tas5805m).work);

        mutex_lock(&mut (*tas5805m).lock);
        if (*tas5805m).is_powered {
            (*tas5805m).is_powered = false;

            regmap_write(rm, REG_PAGE, 0x00);
            regmap_write(rm, REG_BOOK, 0x00);

            regmap_read(rm, REG_CHAN_FAULT, &mut chan);
            regmap_read(rm, REG_GLOBAL_FAULT1, &mut global1);
            regmap_read(rm, REG_GLOBAL_FAULT2, &mut global2);

            dev_dbg(
                (*component).dev,
                c_str!("fault regs: CHAN=%02x, GLOBAL1=%02x, GLOBAL2=%02x\n"),
                chan,
                global1,
                global2,
            );

            regmap_write(rm, REG_DEVICE_CTRL_2, DCTRL2_MODE_HIZ);
        }
        mutex_unlock(&mut (*tas5805m).lock);
    }

    0
}

static tas5805m_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: c_str!("DAC"), control: ptr::null(), source: c_str!("DAC IN") },
    snd_soc_dapm_route { sink: c_str!("OUT"), control: ptr::null(), source: c_str!("DAC") },
];

// SND_SOC_DAPM_AIF_IN("DAC IN", "Playback", 0, SND_SOC_NOPM, 0, 0),
// SND_SOC_DAPM_DAC_E("DAC", NULL, SND_SOC_NOPM, 0, 0,
//     tas5805m_dac_event, SND_SOC_DAPM_PRE_PMD),
// SND_SOC_DAPM_OUTPUT("OUT")
static tas5805m_dapm_widgets: [snd_soc_dapm_widget_c; 3] = [
    snd_soc_dapm_widget_c { _private: [] },
    snd_soc_dapm_widget_c { _private: [] },
    snd_soc_dapm_widget_c { _private: [] },
];

static soc_codec_dev_tas5805m: snd_soc_component_driver = snd_soc_component_driver {
    controls: tas5805m_snd_controls.as_ptr(),
    num_controls: tas5805m_snd_controls.len(),
    dapm_widgets: tas5805m_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas5805m_dapm_widgets.len(),
    dapm_routes: tas5805m_audio_map.as_ptr(),
    num_dapm_routes: tas5805m_audio_map.len(),
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn tas5805m_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let tas5805m: *mut tas5805m_priv =
        snd_soc_component_get_drvdata(component) as *mut tas5805m_priv;

    mutex_lock(&mut (*tas5805m).lock);
    dev_dbg(
        (*component).dev,
        c_str!("set mute=%d (is_powered=%d)\n"),
        mute,
        (*tas5805m).is_powered as c_int,
    );

    (*tas5805m).is_muted = mute != 0;
    if (*tas5805m).is_powered {
        tas5805m_refresh(tas5805m);
    }

    mutex_unlock(&mut (*tas5805m).lock);
    0
}

static tas5805m_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(tas5805m_trigger),
    mute_stream: Some(tas5805m_mute),
    no_capture_mute: 1,
};

static mut tas5805m_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("tas5805m-amplifier"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &tas5805m_dai_ops,
};

static tas5805m_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    /* We have quite a lot of multi-level bank switching and a
     * relatively small number of register writes between bank
     * switches.
     */
    cache_type: REGCACHE_NONE,
};

unsafe extern "C" fn tas5805m_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev: *mut device = &mut (*i2c).dev;
    let mut regmap: *mut regmap;
    let mut tas5805m: *mut tas5805m_priv;
    let mut filename: [c_char; 128] = [0; 128];
    let mut config_name: *const c_char;
    let mut ret: c_int;

    regmap = devm_regmap_init_i2c(i2c, &tas5805m_regmap);
    if IS_ERR(regmap as *const c_void) {
        ret = PTR_ERR(regmap as *const c_void) as c_int;
        dev_err(dev, c_str!("unable to allocate register map: %d\n"), ret);
        return ret;
    }

    tas5805m = devm_kzalloc(dev, size_of::<tas5805m_priv>(), GFP_KERNEL) as *mut tas5805m_priv;
    if tas5805m.is_null() {
        return -ENOMEM;
    }

    (*tas5805m).i2c = i2c;
    (*tas5805m).pvdd = devm_regulator_get(dev, c_str!("pvdd"));
    if IS_ERR((*tas5805m).pvdd as *const c_void) {
        dev_err(
            dev,
            c_str!("failed to get pvdd supply: %ld\n"),
            PTR_ERR((*tas5805m).pvdd as *const c_void),
        );
        return PTR_ERR((*tas5805m).pvdd as *const c_void) as c_int;
    }

    dev_set_drvdata(dev, tas5805m as *mut c_void);
    (*tas5805m).regmap = regmap;
    (*tas5805m).gpio_pdn_n = devm_gpiod_get(dev, c_str!("pdn"), GPIOD_OUT_LOW);
    if IS_ERR((*tas5805m).gpio_pdn_n as *const c_void) {
        dev_err(
            dev,
            c_str!("error requesting PDN gpio: %ld\n"),
            PTR_ERR((*tas5805m).gpio_pdn_n as *const c_void),
        );
        return PTR_ERR((*tas5805m).gpio_pdn_n as *const c_void) as c_int;
    }

    /* This configuration must be generated by PPC3. The file loaded
     * consists of a sequence of register writes, where bytes at
     * even indices are register addresses and those at odd indices
     * are register values.
     *
     * The fixed portion of PPC3's output prior to the 5ms delay
     * should be omitted.
     */
    if device_property_read_string(dev, c_str!("ti,dsp-config-name"), &mut config_name) != 0 {
        config_name = c_str!("default");
    }

    snprintf(
        filename.as_mut_ptr(),
        filename.len(),
        c_str!("tas5805m_dsp_%s.bin"),
        config_name,
    );
    let mut fw: *const firmware = ptr::null();
    ret = request_firmware(&mut fw, filename.as_ptr(), dev);
    if ret != 0 {
        return ret;
    }

    if ((*fw).size < 2) || (((*fw).size & 1) != 0) {
        dev_err(dev, c_str!("firmware is invalid\n"));
        return -EINVAL;
    }

    (*tas5805m).dsp_cfg_len = (*fw).size as c_int;
    (*tas5805m).dsp_cfg_data =
        devm_kmemdup(dev, (*fw).data as *const c_void, (*fw).size, GFP_KERNEL) as *mut u8;
    if (*tas5805m).dsp_cfg_data.is_null() {
        return -ENOMEM;
    }

    /* Do the first part of the power-on here, while we can expect
     * the I2S interface to be quiet. We must raise PDN# and then
     * wait 5ms before any I2S clock is sent, or else the internal
     * regulator apparently won't come on.
     *
     * Also, we must keep the device in power down for 100ms or so
     * after PVDD is applied, or else the ADR pin is sampled
     * incorrectly and the device comes up with an unpredictable I2C
     * address.
     */
    (*tas5805m).vol[0] = TAS5805M_VOLUME_MIN;
    (*tas5805m).vol[1] = TAS5805M_VOLUME_MIN;

    ret = regulator_enable((*tas5805m).pvdd);
    if ret < 0 {
        dev_err(dev, c_str!("failed to enable pvdd: %d\n"), ret);
        return ret;
    }

    usleep_range(100000, 150000);
    gpiod_set_value((*tas5805m).gpio_pdn_n, 1);
    usleep_range(10000, 15000);

    INIT_WORK(&mut (*tas5805m).work, do_work);
    mutex_init(&mut (*tas5805m).lock);

    /* Don't register through devm. We need to be able to unregister
     * the component prior to deasserting PDN#
     */
    ret = snd_soc_register_component(
        dev,
        &soc_codec_dev_tas5805m,
        &raw mut tas5805m_dai,
        1,
    );
    if ret < 0 {
        dev_err(dev, c_str!("unable to register codec: %d\n"), ret);
        gpiod_set_value((*tas5805m).gpio_pdn_n, 0);
        regulator_disable((*tas5805m).pvdd);
        return ret;
    }

    0
}

unsafe extern "C" fn tas5805m_i2c_remove(i2c: *mut i2c_client) {
    let dev: *mut device = &mut (*i2c).dev;
    let tas5805m: *mut tas5805m_priv = dev_get_drvdata(dev) as *mut tas5805m_priv;

    cancel_work_sync(&mut (*tas5805m).work);
    snd_soc_unregister_component(dev);
    gpiod_set_value((*tas5805m).gpio_pdn_n, 0);
    usleep_range(10000, 15000);
    regulator_disable((*tas5805m).pvdd);
}

static tas5805m_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [
        b't' as c_char, b'a' as c_char, b's' as c_char, b'5' as c_char,
        b'8' as c_char, b'0' as c_char, b'5' as c_char, b'm' as c_char,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ] },
    i2c_device_id { name: [0; 32] },
];
// MODULE_DEVICE_TABLE(i2c, tas5805m_i2c_id);

// #if IS_ENABLED(CONFIG_OF)
static tas5805m_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c_str!("ti,tas5805m") },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, tas5805m_of_match);
// #endif

static mut tas5805m_i2c_driver: i2c_driver = i2c_driver {
    probe: Some(tas5805m_i2c_probe),
    remove: Some(tas5805m_i2c_remove),
    id_table: tas5805m_i2c_id.as_ptr(),
    driver: i2c_driver_inner {
        name: c_str!("tas5805m"),
        of_match_table: tas5805m_of_match.as_ptr(),
    },
};

unsafe fn init_module_translation() {
    module_i2c_driver(&raw mut tas5805m_i2c_driver);
}

// MODULE_AUTHOR("Andy Liu <andy-liu@ti.com>");
// MODULE_AUTHOR("Daniel Beer <daniel.beer@igorinstitute.com>");
// MODULE_DESCRIPTION("TAS5805M Audio Amplifier Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
