// SPDX-License-Identifier: GPL-2.0
//
// Driver for Microchip S/PDIF TX Controller
//
// Copyright (C) 2020 Microchip Technology Inc. and its subsidiaries
//
// Author: Codrin Ciubotariu <codrin.ciubotariu@microchip.com>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/*
 * Dependencies originally included from Linux and ALSA headers:
 * linux/bitfield.h, linux/clk.h, linux/io.h, linux/module.h,
 * linux/pm_runtime.h, linux/spinlock.h, sound/asoundef.h,
 * sound/dmaengine_pcm.h, sound/pcm_params.h, sound/soc.h.
 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub iec958: snd_aes_iec958,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aes_iec958 {
    pub status: [u8; SPDIFTX_CS_BITS / 8],
    pub subcode: [u8; SPDIFTX_UD_BITS / 8],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: c_uint,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

pub type u32 = u32;
pub type dma_addr_t = u64;
pub type resource_size_t = u64;
pub type irqreturn_t = c_uint;
pub type spinlock_t = c_uint;

const fn bit(nr: u32) -> u32 {
    1u32 << nr
}

const fn genmask(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

const fn field_prep(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

/*
 * ---- S/PDIF Transmitter Controller Register map ----
 */
const SPDIFTX_CR: c_uint = 0x00; /* Control Register */
const SPDIFTX_MR: c_uint = 0x04; /* Mode Register */
const SPDIFTX_CDR: c_uint = 0x0C; /* Common Data Register */

const SPDIFTX_IER: c_uint = 0x14; /* Interrupt Enable Register */
const SPDIFTX_IDR: c_uint = 0x18; /* Interrupt Disable Register */
const SPDIFTX_IMR: c_uint = 0x1C; /* Interrupt Mask Register */
const SPDIFTX_ISR: c_uint = 0x20; /* Interrupt Status Register */

const fn SPDIFTX_CH1UD(reg: c_uint) -> c_uint {
    0x50 + reg * 4
} /* User Data 1 Register x */

const fn SPDIFTX_CH1S(reg: c_uint) -> c_uint {
    0x80 + reg * 4
} /* Channel Status 1 Register x */

const SPDIFTX_VERSION: c_uint = 0xF0;

/*
 * ---- Control Register (Write-only) ----
 */
const SPDIFTX_CR_SWRST: u32 = bit(0); /* Software Reset */
const SPDIFTX_CR_FCLR: u32 = bit(1); /* FIFO clear */

/*
 * ---- Mode Register (Read/Write) ----
 */
/* Transmit Enable */
const SPDIFTX_MR_TXEN_MASK: u32 = genmask(0, 0);
const SPDIFTX_MR_TXEN_DISABLE: u32 = 0 << 0;
const SPDIFTX_MR_TXEN_ENABLE: u32 = 1 << 0;

/* Multichannel Transfer */
/* Original C uses GENAMSK(1, 1), preserving the apparent build-time dependency/typo intent. */
const SPDIFTX_MR_MULTICH_MASK: u32 = genmask(1, 1);
const SPDIFTX_MR_MULTICH_MONO: u32 = 0 << 1;
const SPDIFTX_MR_MULTICH_DUAL: u32 = 1 << 1;

/* Data Word Endian Mode */
const SPDIFTX_MR_ENDIAN_MASK: u32 = genmask(2, 2);
const SPDIFTX_MR_ENDIAN_LITTLE: u32 = 0 << 2;
const SPDIFTX_MR_ENDIAN_BIG: u32 = 1 << 2;

/* Data Justification */
const SPDIFTX_MR_JUSTIFY_MASK: u32 = genmask(3, 3);
const SPDIFTX_MR_JUSTIFY_LSB: u32 = 0 << 3;
const SPDIFTX_MR_JUSTIFY_MSB: u32 = 1 << 3;

/* Common Audio Register Transfer Mode */
const SPDIFTX_MR_CMODE_MASK: u32 = genmask(5, 4);
const SPDIFTX_MR_CMODE_INDEX_ACCESS: u32 = 0 << 4;
const SPDIFTX_MR_CMODE_TOGGLE_ACCESS: u32 = 1 << 4;
const SPDIFTX_MR_CMODE_INTERLVD_ACCESS: u32 = 2 << 4;

/* Valid Bits per Sample */
const SPDIFTX_MR_VBPS_MASK: u32 = genmask(13, 8);

/* Chunk Size */
const SPDIFTX_MR_CHUNK_MASK: u32 = genmask(19, 16);

/* Validity Bits for Channels 1 and 2 */
const SPDIFTX_MR_VALID1: u32 = bit(24);
const SPDIFTX_MR_VALID2: u32 = bit(25);

/* Disable Null Frame on underrun */
const SPDIFTX_MR_DNFR_MASK: u32 = genmask(27, 27);
const SPDIFTX_MR_DNFR_INVALID: u32 = 0 << 27;
const SPDIFTX_MR_DNFR_VALID: u32 = 1 << 27;

/* Bytes per Sample */
const SPDIFTX_MR_BPS_MASK: u32 = genmask(29, 28);

/*
 * ---- Interrupt Enable/Disable/Mask/Status Register (Write/Read-only) ----
 */
const SPDIFTX_IR_TXRDY: u32 = bit(0);
const SPDIFTX_IR_TXEMPTY: u32 = bit(1);
const SPDIFTX_IR_TXFULL: u32 = bit(2);
const SPDIFTX_IR_TXCHUNK: u32 = bit(3);
const SPDIFTX_IR_TXUDR: u32 = bit(4);
const SPDIFTX_IR_TXOVR: u32 = bit(5);
const SPDIFTX_IR_CSRDY: u32 = bit(6);
const SPDIFTX_IR_UDRDY: u32 = bit(7);
const fn SPDIFTX_IR_TXRDYCH(ch: u32) -> u32 {
    bit(ch + 8)
}
const SPDIFTX_IR_SECE: u32 = bit(10);
const fn SPDIFTX_IR_TXUDRCH(ch: u32) -> u32 {
    bit(ch + 11)
}
const SPDIFTX_IR_BEND: u32 = bit(13);

unsafe extern "C" fn mchp_spdiftx_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        SPDIFTX_MR | SPDIFTX_IMR | SPDIFTX_ISR => true,
        r if r == SPDIFTX_CH1UD(0)
            || r == SPDIFTX_CH1UD(1)
            || r == SPDIFTX_CH1UD(2)
            || r == SPDIFTX_CH1UD(3)
            || r == SPDIFTX_CH1UD(4)
            || r == SPDIFTX_CH1UD(5)
            || r == SPDIFTX_CH1S(0)
            || r == SPDIFTX_CH1S(1)
            || r == SPDIFTX_CH1S(2)
            || r == SPDIFTX_CH1S(3)
            || r == SPDIFTX_CH1S(4)
            || r == SPDIFTX_CH1S(5) =>
        {
            true
        }
        _ => false,
    }
}

unsafe extern "C" fn mchp_spdiftx_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        SPDIFTX_CR | SPDIFTX_MR | SPDIFTX_CDR | SPDIFTX_IER | SPDIFTX_IDR => true,
        r if r == SPDIFTX_CH1UD(0)
            || r == SPDIFTX_CH1UD(1)
            || r == SPDIFTX_CH1UD(2)
            || r == SPDIFTX_CH1UD(3)
            || r == SPDIFTX_CH1UD(4)
            || r == SPDIFTX_CH1UD(5)
            || r == SPDIFTX_CH1S(0)
            || r == SPDIFTX_CH1S(1)
            || r == SPDIFTX_CH1S(2)
            || r == SPDIFTX_CH1S(3)
            || r == SPDIFTX_CH1S(4)
            || r == SPDIFTX_CH1S(5) =>
        {
            true
        }
        _ => false,
    }
}

unsafe extern "C" fn mchp_spdiftx_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        SPDIFTX_CDR | SPDIFTX_ISR => true,
        _ => false,
    }
}

static mchp_spdiftx_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: SPDIFTX_VERSION,
    readable_reg: Some(mchp_spdiftx_readable_reg),
    writeable_reg: Some(mchp_spdiftx_writeable_reg),
    precious_reg: Some(mchp_spdiftx_precious_reg),
    cache_type: REGCACHE_FLAT,
};

const SPDIFTX_GCLK_RATIO: u32 = 128;

const SPDIFTX_CS_BITS: usize = 192;
const SPDIFTX_UD_BITS: usize = 192;

#[repr(C)]
pub struct mchp_spdiftx_mixer_control {
    pub ch_stat: [u8; SPDIFTX_CS_BITS / 8],
    pub user_data: [u8; SPDIFTX_UD_BITS / 8],
    pub lock: spinlock_t, /* exclusive access to control data */
}

#[repr(C)]
pub struct mchp_spdiftx_dev {
    pub control: mchp_spdiftx_mixer_control,
    pub playback: snd_dmaengine_dai_dma_data,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pclk: *mut clk,
    pub gclk: *mut clk,
    pub fmt: c_uint,
    pub suspend_irq: c_uint,
}

unsafe fn mchp_spdiftx_is_running(dev: *mut mchp_spdiftx_dev) -> c_int {
    let mut mr: u32 = 0;

    regmap_read((*dev).regmap, SPDIFTX_MR, &mut mr);
    ((mr & SPDIFTX_MR_TXEN_ENABLE) != 0) as c_int
}

unsafe fn mchp_spdiftx_channel_status_write(dev: *mut mchp_spdiftx_dev) {
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;
    let mut val: u32;

    for i in 0..((*ctrl).ch_stat.len() / 4) {
        val = (((*ctrl).ch_stat[(i * 4) + 0] as u32) << 0)
            | (((*ctrl).ch_stat[(i * 4) + 1] as u32) << 8)
            | (((*ctrl).ch_stat[(i * 4) + 2] as u32) << 16)
            | (((*ctrl).ch_stat[(i * 4) + 3] as u32) << 24);

        regmap_write((*dev).regmap, SPDIFTX_CH1S(i as c_uint), val);
    }
}

unsafe fn mchp_spdiftx_user_data_write(dev: *mut mchp_spdiftx_dev) {
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;
    let mut val: u32;

    for i in 0..((*ctrl).user_data.len() / 4) {
        val = (((*ctrl).user_data[(i * 4) + 0] as u32) << 0)
            | (((*ctrl).user_data[(i * 4) + 1] as u32) << 8)
            | (((*ctrl).user_data[(i * 4) + 2] as u32) << 16)
            | (((*ctrl).user_data[(i * 4) + 3] as u32) << 24);

        regmap_write((*dev).regmap, SPDIFTX_CH1UD(i as c_uint), val);
    }
}

unsafe extern "C" fn mchp_spdiftx_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let dev = dev_id as *mut mchp_spdiftx_dev;
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;
    let mut sr: u32 = 0;
    let mut imr: u32 = 0;
    let pending: u32;
    let mut idr: u32 = 0;

    regmap_read((*dev).regmap, SPDIFTX_ISR, &mut sr);
    regmap_read((*dev).regmap, SPDIFTX_IMR, &mut imr);
    pending = sr & imr;

    if pending == 0 {
        return IRQ_NONE;
    }

    if (pending & SPDIFTX_IR_TXUDR) != 0 {
        dev_warn((*dev).dev, c"underflow detected\n".as_ptr());
        idr |= SPDIFTX_IR_TXUDR;
    }

    if (pending & SPDIFTX_IR_TXOVR) != 0 {
        dev_warn((*dev).dev, c"overflow detected\n".as_ptr());
        idr |= SPDIFTX_IR_TXOVR;
    }

    if (pending & SPDIFTX_IR_UDRDY) != 0 {
        spin_lock(&mut (*ctrl).lock);
        mchp_spdiftx_user_data_write(dev);
        spin_unlock(&mut (*ctrl).lock);
        idr |= SPDIFTX_IR_UDRDY;
    }

    if (pending & SPDIFTX_IR_CSRDY) != 0 {
        spin_lock(&mut (*ctrl).lock);
        mchp_spdiftx_channel_status_write(dev);
        spin_unlock(&mut (*ctrl).lock);
        idr |= SPDIFTX_IR_CSRDY;
    }

    regmap_write((*dev).regmap, SPDIFTX_IDR, idr);

    IRQ_HANDLED
}

unsafe extern "C" fn mchp_spdiftx_dai_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;

    /* Software reset the IP */
    regmap_write((*dev).regmap, SPDIFTX_CR, SPDIFTX_CR_SWRST | SPDIFTX_CR_FCLR);

    0
}

unsafe extern "C" fn mchp_spdiftx_dai_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;

    /* Disable interrupts */
    regmap_write((*dev).regmap, SPDIFTX_IDR, 0xffffffff);
}

unsafe extern "C" fn mchp_spdiftx_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;
    let ret: c_int;

    /* do not start/stop while channel status or user data is updated */
    spin_lock(&mut (*ctrl).lock);
    match cmd {
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_START => {
            regmap_write(
                (*dev).regmap,
                SPDIFTX_IER,
                (*dev).suspend_irq | SPDIFTX_IR_TXUDR | SPDIFTX_IR_TXOVR,
            );
            (*dev).suspend_irq = 0;
            ret = regmap_update_bits(
                (*dev).regmap,
                SPDIFTX_MR,
                SPDIFTX_MR_TXEN_MASK,
                SPDIFTX_MR_TXEN_ENABLE,
            );
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ret = regmap_update_bits(
                (*dev).regmap,
                SPDIFTX_MR,
                SPDIFTX_MR_TXEN_MASK,
                SPDIFTX_MR_TXEN_ENABLE,
            );
        }
        SNDRV_PCM_TRIGGER_SUSPEND => {
            regmap_read((*dev).regmap, SPDIFTX_IMR, &mut (*dev).suspend_irq);
            regmap_write(
                (*dev).regmap,
                SPDIFTX_IDR,
                (*dev).suspend_irq | SPDIFTX_IR_TXUDR | SPDIFTX_IR_TXOVR,
            );
            ret = regmap_update_bits(
                (*dev).regmap,
                SPDIFTX_MR,
                SPDIFTX_MR_TXEN_MASK,
                SPDIFTX_MR_TXEN_DISABLE,
            );
        }
        SNDRV_PCM_TRIGGER_STOP => {
            regmap_write(
                (*dev).regmap,
                SPDIFTX_IDR,
                (*dev).suspend_irq | SPDIFTX_IR_TXUDR | SPDIFTX_IR_TXOVR,
            );
            ret = regmap_update_bits(
                (*dev).regmap,
                SPDIFTX_MR,
                SPDIFTX_MR_TXEN_MASK,
                SPDIFTX_MR_TXEN_DISABLE,
            );
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            ret = regmap_update_bits(
                (*dev).regmap,
                SPDIFTX_MR,
                SPDIFTX_MR_TXEN_MASK,
                SPDIFTX_MR_TXEN_DISABLE,
            );
        }
        _ => {
            ret = -EINVAL;
        }
    }
    spin_unlock(&mut (*ctrl).lock);
    if ret != 0 {
        dev_err((*dev).dev, c"unable to start/stop TX: %d\n".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn mchp_spdiftx_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut flags: c_ulong = 0;
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;
    let mut mr: u32 = 0;
    let bps: c_uint = params_physical_width(params) / 8;
    let aes3: u8;
    let mut ret: c_int;

    dev_dbg(
        (*dev).dev,
        c"%s() rate=%u format=%#x width=%u channels=%u\n".as_ptr(),
        c"mchp_spdiftx_hw_params".as_ptr(),
        params_rate(params),
        params_format(params),
        params_width(params),
        params_channels(params),
    );

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        dev_err((*dev).dev, c"Capture is not supported\n".as_ptr());
        return -EINVAL;
    }

    regmap_read((*dev).regmap, SPDIFTX_MR, &mut mr);

    if (mr & SPDIFTX_MR_TXEN_ENABLE) != 0 {
        dev_err((*dev).dev, c"PCM already running\n".as_ptr());
        return -EBUSY;
    }

    /* Defaults: Toggle mode, justify to LSB, chunksize 1 */
    mr = SPDIFTX_MR_CMODE_TOGGLE_ACCESS | SPDIFTX_MR_JUSTIFY_LSB;
    (*dev).playback.maxburst = 1;
    match params_channels(params) {
        1 => {
            mr |= SPDIFTX_MR_MULTICH_MONO;
        }
        2 => {
            mr |= SPDIFTX_MR_MULTICH_DUAL;
            if bps > 2 {
                (*dev).playback.maxburst = 2;
            }
        }
        _ => {
            dev_err(
                (*dev).dev,
                c"unsupported number of channels: %d\n".as_ptr(),
                params_channels(params),
            );
            return -EINVAL;
        }
    }
    mr |= field_prep(SPDIFTX_MR_CHUNK_MASK, (*dev).playback.maxburst);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => {
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 8);
        }
        SNDRV_PCM_FORMAT_S16_BE => {
            mr |= SPDIFTX_MR_ENDIAN_BIG;
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 16);
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 16);
        }
        SNDRV_PCM_FORMAT_S18_3BE => {
            mr |= SPDIFTX_MR_ENDIAN_BIG;
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 18);
        }
        SNDRV_PCM_FORMAT_S18_3LE => {
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 18);
        }
        SNDRV_PCM_FORMAT_S20_3BE => {
            mr |= SPDIFTX_MR_ENDIAN_BIG;
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 20);
        }
        SNDRV_PCM_FORMAT_S20_3LE => {
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 20);
        }
        SNDRV_PCM_FORMAT_S24_3BE => {
            mr |= SPDIFTX_MR_ENDIAN_BIG;
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 24);
        }
        SNDRV_PCM_FORMAT_S24_3LE => {
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 24);
        }
        SNDRV_PCM_FORMAT_S24_BE => {
            mr |= SPDIFTX_MR_ENDIAN_BIG;
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 24);
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 24);
        }
        SNDRV_PCM_FORMAT_S32_BE => {
            mr |= SPDIFTX_MR_ENDIAN_BIG;
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 32);
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            mr |= field_prep(SPDIFTX_MR_VBPS_MASK, 32);
        }
        _ => {
            dev_err(
                (*dev).dev,
                c"unsupported PCM format: %d\n".as_ptr(),
                params_format(params),
            );
            return -EINVAL;
        }
    }

    mr |= field_prep(SPDIFTX_MR_BPS_MASK, bps - 1);

    match params_rate(params) {
        22050 => aes3 = IEC958_AES3_CON_FS_22050,
        24000 => aes3 = IEC958_AES3_CON_FS_24000,
        32000 => aes3 = IEC958_AES3_CON_FS_32000,
        44100 => aes3 = IEC958_AES3_CON_FS_44100,
        48000 => aes3 = IEC958_AES3_CON_FS_48000,
        88200 => aes3 = IEC958_AES3_CON_FS_88200,
        96000 => aes3 = IEC958_AES3_CON_FS_96000,
        176400 => aes3 = IEC958_AES3_CON_FS_176400,
        192000 => aes3 = IEC958_AES3_CON_FS_192000,
        8000 | 11025 | 16000 | 64000 => aes3 = IEC958_AES3_CON_FS_NOTID,
        _ => {
            dev_err(
                (*dev).dev,
                c"unsupported sample frequency: %u\n".as_ptr(),
                params_rate(params),
            );
            return -EINVAL;
        }
    }
    spin_lock_irqsave(&mut (*ctrl).lock, &mut flags);
    (*ctrl).ch_stat[3] &= !IEC958_AES3_CON_FS;
    (*ctrl).ch_stat[3] |= aes3;
    mchp_spdiftx_channel_status_write(dev);
    spin_unlock_irqrestore(&mut (*ctrl).lock, flags);

    /* GCLK is enabled by runtime PM. */
    clk_disable_unprepare((*dev).gclk);

    ret = clk_set_rate((*dev).gclk, params_rate(params) * SPDIFTX_GCLK_RATIO);
    if ret != 0 {
        dev_err(
            (*dev).dev,
            c"unable to change gclk rate to: rate %u * ratio %u\n".as_ptr(),
            params_rate(params),
            SPDIFTX_GCLK_RATIO,
        );
        return ret;
    }
    ret = clk_prepare_enable((*dev).gclk);
    if ret != 0 {
        dev_err((*dev).dev, c"unable to enable gclk: %d\n".as_ptr(), ret);
        return ret;
    }

    dev_dbg(
        (*dev).dev,
        c"%s(): GCLK set to %d\n".as_ptr(),
        c"mchp_spdiftx_hw_params".as_ptr(),
        params_rate(params) * SPDIFTX_GCLK_RATIO,
    );

    regmap_write((*dev).regmap, SPDIFTX_MR, mr);

    0
}

unsafe extern "C" fn mchp_spdiftx_hw_free(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;

    regmap_write((*dev).regmap, SPDIFTX_CR, SPDIFTX_CR_SWRST | SPDIFTX_CR_FCLR)
}

const MCHP_SPDIFTX_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;

const MCHP_SPDIFTX_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_U16_BE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S18_3BE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S20_3BE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_3BE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S24_BE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_S32_BE;

unsafe extern "C" fn mchp_spdiftx_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;

    0
}

unsafe extern "C" fn mchp_spdiftx_cs_get(
    kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> c_int {
    let mut flags: c_ulong = 0;
    let dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;

    spin_lock_irqsave(&mut (*ctrl).lock, &mut flags);
    ptr::copy_nonoverlapping(
        (*ctrl).ch_stat.as_ptr(),
        (*uvalue).value.iec958.status.as_mut_ptr(),
        size_of_val(&(*ctrl).ch_stat),
    );
    spin_unlock_irqrestore(&mut (*ctrl).lock, flags);

    0
}

unsafe extern "C" fn mchp_spdiftx_cs_put(
    kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> c_int {
    let mut flags: c_ulong = 0;
    let dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;
    let mut changed: c_int = 0;

    spin_lock_irqsave(&mut (*ctrl).lock, &mut flags);
    for i in 0..(*ctrl).ch_stat.len() {
        if (*ctrl).ch_stat[i] != (*uvalue).value.iec958.status[i] {
            changed = 1;
        }
        (*ctrl).ch_stat[i] = (*uvalue).value.iec958.status[i];
    }

    if changed != 0 {
        /* don't enable IP while we copy the channel status */
        if mchp_spdiftx_is_running(dev) != 0 {
            /*
             * if SPDIF is running, wait for interrupt to write
             * channel status
             */
            regmap_write((*dev).regmap, SPDIFTX_IER, SPDIFTX_IR_CSRDY);
        } else {
            mchp_spdiftx_channel_status_write(dev);
        }
    }
    spin_unlock_irqrestore(&mut (*ctrl).lock, flags);

    changed
}

unsafe extern "C" fn mchp_spdiftx_cs_mask(
    _kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> c_int {
    ptr::write_bytes(
        (*uvalue).value.iec958.status.as_mut_ptr(),
        0xff,
        size_of_val(&(*uvalue).value.iec958.status),
    );

    0
}

unsafe extern "C" fn mchp_spdiftx_subcode_get(
    kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*ctrl).lock, &mut flags);
    ptr::copy_nonoverlapping(
        (*ctrl).user_data.as_ptr(),
        (*uvalue).value.iec958.subcode.as_mut_ptr(),
        size_of_val(&(*ctrl).user_data),
    );
    spin_unlock_irqrestore(&mut (*ctrl).lock, flags);

    0
}

unsafe extern "C" fn mchp_spdiftx_subcode_put(
    kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> c_int {
    let mut flags: c_ulong = 0;
    let dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;
    let ctrl: *mut mchp_spdiftx_mixer_control = &mut (*dev).control;
    let mut changed: c_int = 0;

    spin_lock_irqsave(&mut (*ctrl).lock, &mut flags);
    for i in 0..(*ctrl).user_data.len() {
        if (*ctrl).user_data[i] != (*uvalue).value.iec958.subcode[i] {
            changed = 1;
        }

        (*ctrl).user_data[i] = (*uvalue).value.iec958.subcode[i];
    }
    if changed != 0 {
        if mchp_spdiftx_is_running(dev) != 0 {
            /*
             * if SPDIF is running, wait for interrupt to write
             * user data
             */
            regmap_write((*dev).regmap, SPDIFTX_IER, SPDIFTX_IR_UDRDY);
        } else {
            mchp_spdiftx_user_data_write(dev);
        }
    }
    spin_unlock_irqrestore(&mut (*ctrl).lock, flags);

    changed
}

static mut mchp_spdiftx_ctrls: [snd_kcontrol_new; 3] = [
    /* Channel status controller */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Playback Default".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(mchp_spdiftx_info),
        get: Some(mchp_spdiftx_cs_get),
        put: Some(mchp_spdiftx_cs_put),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Playback Mask".as_ptr(),
        /*
         * Original C initializer has a comma instead of '|':
         * .access = SNDRV_CTL_ELEM_ACCESS_READ,
         *     SNDRV_CTL_ELEM_ACCESS_VOLATILE,
         */
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(mchp_spdiftx_info),
        get: Some(mchp_spdiftx_cs_mask),
        put: None,
    },
    /* User bits controller */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Subcode Playback Default".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        info: Some(mchp_spdiftx_info),
        get: Some(mchp_spdiftx_subcode_get),
        put: Some(mchp_spdiftx_subcode_put),
    },
];

unsafe extern "C" fn mchp_spdiftx_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut mchp_spdiftx_dev;

    snd_soc_dai_init_dma_data(dai, &mut (*dev).playback, ptr::null_mut());

    /* Add controls */
    snd_soc_add_dai_controls(dai, mchp_spdiftx_ctrls.as_mut_ptr(), mchp_spdiftx_ctrls.len());

    0
}

static mchp_spdiftx_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(mchp_spdiftx_dai_probe),
    startup: Some(mchp_spdiftx_dai_startup),
    shutdown: Some(mchp_spdiftx_dai_shutdown),
    trigger: Some(mchp_spdiftx_trigger),
    hw_params: Some(mchp_spdiftx_hw_params),
    hw_free: Some(mchp_spdiftx_hw_free),
};

static mut mchp_spdiftx_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"mchp-spdiftx".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MCHP_SPDIFTX_RATES,
        formats: MCHP_SPDIFTX_FORMATS,
    },
    ops: &mchp_spdiftx_dai_ops,
};

static mchp_spdiftx_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"mchp-spdiftx".as_ptr(),
    legacy_dai_naming: 1,
};

static mchp_spdiftx_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"microchip,sama7g5-spdiftx".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    }, /* sentinel */
];
/* MODULE_DEVICE_TABLE(of, mchp_spdiftx_dt_ids); */

unsafe extern "C" fn mchp_spdiftx_runtime_suspend(dev: *mut device) -> c_int {
    let spdiftx = dev_get_drvdata(dev) as *mut mchp_spdiftx_dev;

    regcache_cache_only((*spdiftx).regmap, true);

    clk_disable_unprepare((*spdiftx).gclk);
    clk_disable_unprepare((*spdiftx).pclk);

    0
}

unsafe extern "C" fn mchp_spdiftx_runtime_resume(dev: *mut device) -> c_int {
    let spdiftx = dev_get_drvdata(dev) as *mut mchp_spdiftx_dev;
    let mut ret: c_int;

    ret = clk_prepare_enable((*spdiftx).pclk);
    if ret != 0 {
        dev_err(
            (*spdiftx).dev,
            c"failed to enable the peripheral clock: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }
    ret = clk_prepare_enable((*spdiftx).gclk);
    if ret != 0 {
        dev_err(
            (*spdiftx).dev,
            c"failed to enable generic clock: %d\n".as_ptr(),
            ret,
        );
        clk_disable_unprepare((*spdiftx).pclk);
        return ret;
    }

    regcache_cache_only((*spdiftx).regmap, false);
    regcache_mark_dirty((*spdiftx).regmap);
    ret = regcache_sync((*spdiftx).regmap);
    if ret != 0 {
        regcache_cache_only((*spdiftx).regmap, true);
        clk_disable_unprepare((*spdiftx).gclk);
        clk_disable_unprepare((*spdiftx).pclk);
    }

    ret
}

/*
 * static const struct dev_pm_ops mchp_spdiftx_pm_ops = {
 *     SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
 *     RUNTIME_PM_OPS(mchp_spdiftx_runtime_suspend, mchp_spdiftx_runtime_resume, NULL)
 * };
 */
extern "C" {
    static mchp_spdiftx_pm_ops: dev_pm_ops;
}

unsafe extern "C" fn mchp_spdiftx_probe(pdev: *mut platform_device) -> c_int {
    let mut dev: *mut mchp_spdiftx_dev;
    let mut mem: *mut resource = ptr::null_mut();
    let mut regmap: *mut regmap;
    let base: *mut c_void;
    let ctrl: *mut mchp_spdiftx_mixer_control;
    let irq: c_int;
    let mut err: c_int;

    /* Get memory for driver data. */
    dev = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<mchp_spdiftx_dev>(),
        GFP_KERNEL,
    ) as *mut mchp_spdiftx_dev;
    if dev.is_null() {
        return -ENOMEM;
    }

    /* Map I/O registers. */
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, &mchp_spdiftx_regmap_config);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    /* Request IRQ */
    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    err = devm_request_irq(
        &mut (*pdev).dev,
        irq,
        Some(mchp_spdiftx_interrupt),
        0,
        dev_name(&mut (*pdev).dev),
        dev as *mut c_void,
    );
    if err != 0 {
        return err;
    }

    /* Get the peripheral clock */
    (*dev).pclk = devm_clk_get(&mut (*pdev).dev, c"pclk".as_ptr());
    if IS_ERR((*dev).pclk as *const c_void) {
        err = PTR_ERR((*dev).pclk as *const c_void);
        dev_err(
            &mut (*pdev).dev,
            c"failed to get the peripheral clock: %d\n".as_ptr(),
            err,
        );
        return err;
    }

    /* Get the generic clock */
    (*dev).gclk = devm_clk_get(&mut (*pdev).dev, c"gclk".as_ptr());
    if IS_ERR((*dev).gclk as *const c_void) {
        err = PTR_ERR((*dev).gclk as *const c_void);
        dev_err(
            &mut (*pdev).dev,
            c"failed to get the PMC generic clock: %d\n".as_ptr(),
            err,
        );
        return err;
    }

    ctrl = &mut (*dev).control;
    spin_lock_init(&mut (*ctrl).lock);

    /* Init channel status */
    (*ctrl).ch_stat[0] = IEC958_AES0_CON_NOT_COPYRIGHT | IEC958_AES0_CON_EMPHASIS_NONE;

    (*dev).dev = &mut (*pdev).dev;
    (*dev).regmap = regmap;
    platform_set_drvdata(pdev, dev as *mut c_void);

    pm_runtime_enable((*dev).dev);
    if !pm_runtime_enabled((*dev).dev) {
        err = mchp_spdiftx_runtime_resume((*dev).dev);
        if err != 0 {
            return err;
        }
    }

    (*dev).playback.addr = (*mem).start as dma_addr_t + SPDIFTX_CDR as dma_addr_t;
    (*dev).playback.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;

    err = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if err != 0 {
        dev_err(&mut (*pdev).dev, c"failed to register PMC: %d\n".as_ptr(), err);
        if !pm_runtime_status_suspended((*dev).dev) {
            mchp_spdiftx_runtime_suspend((*dev).dev);
        }
        pm_runtime_disable((*dev).dev);
        return err;
    }

    err = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &mchp_spdiftx_component,
        &mut mchp_spdiftx_dai,
        1,
    );
    if err != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"failed to register component: %d\n".as_ptr(),
            err,
        );
        if !pm_runtime_status_suspended((*dev).dev) {
            mchp_spdiftx_runtime_suspend((*dev).dev);
        }
        pm_runtime_disable((*dev).dev);
        return err;
    }

    0
}

unsafe extern "C" fn mchp_spdiftx_remove(pdev: *mut platform_device) {
    let dev = platform_get_drvdata(pdev) as *mut mchp_spdiftx_dev;

    if !pm_runtime_status_suspended((*dev).dev) {
        mchp_spdiftx_runtime_suspend((*dev).dev);
    }

    pm_runtime_disable((*dev).dev);
}

static mut mchp_spdiftx_driver: platform_driver = platform_driver {
    probe: Some(mchp_spdiftx_probe),
    remove: Some(mchp_spdiftx_remove),
    driver: device_driver {
        name: c"mchp_spdiftx".as_ptr(),
        of_match_table: mchp_spdiftx_dt_ids.as_ptr(),
        pm: &mchp_spdiftx_pm_ops,
    },
};

/* module_platform_driver(mchp_spdiftx_driver); */

/* MODULE_AUTHOR("Codrin Ciubotariu <codrin.ciubotariu@microchip.com>"); */
/* MODULE_DESCRIPTION("Microchip S/PDIF TX Controller Driver"); */
/* MODULE_LICENSE("GPL v2"); */

extern "C" {
    static REGCACHE_FLAT: c_uint;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static EINVAL: c_int;
    static EBUSY: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_FORMAT_S8: c_uint;
    static SNDRV_PCM_FORMAT_S16_BE: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_uint;
    static SNDRV_PCM_FORMAT_S18_3BE: c_uint;
    static SNDRV_PCM_FORMAT_S18_3LE: c_uint;
    static SNDRV_PCM_FORMAT_S20_3BE: c_uint;
    static SNDRV_PCM_FORMAT_S20_3LE: c_uint;
    static SNDRV_PCM_FORMAT_S24_3BE: c_uint;
    static SNDRV_PCM_FORMAT_S24_3LE: c_uint;
    static SNDRV_PCM_FORMAT_S24_BE: c_uint;
    static SNDRV_PCM_FORMAT_S24_LE: c_uint;
    static SNDRV_PCM_FORMAT_S32_BE: c_uint;
    static SNDRV_PCM_FORMAT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_U16_BE: u64;
    static SNDRV_PCM_FMTBIT_S18_3LE: u64;
    static SNDRV_PCM_FMTBIT_S18_3BE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3BE: u64;
    static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_3BE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_BE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_BE: u64;
    static IEC958_AES3_CON_FS_22050: u8;
    static IEC958_AES3_CON_FS_24000: u8;
    static IEC958_AES3_CON_FS_32000: u8;
    static IEC958_AES3_CON_FS_44100: u8;
    static IEC958_AES3_CON_FS_48000: u8;
    static IEC958_AES3_CON_FS_88200: u8;
    static IEC958_AES3_CON_FS_96000: u8;
    static IEC958_AES3_CON_FS_176400: u8;
    static IEC958_AES3_CON_FS_192000: u8;
    static IEC958_AES3_CON_FS_NOTID: u8;
    static IEC958_AES3_CON_FS: u8;
    static IEC958_AES0_CON_NOT_COPYRIGHT: u8;
    static IEC958_AES0_CON_EMPHASIS_NONE: u8;
    static SNDRV_CTL_ELEM_TYPE_IEC958: c_uint;
    static SNDRV_CTL_ELEM_IFACE_PCM: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READ: c_uint;
    static DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint;

    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: u32, val: u32) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_add_dai_controls(
        dai: *mut snd_soc_dai,
        controls: *mut snd_kcontrol_new,
        num_controls: usize,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const c_void,
        flags: c_uint,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
