// SPDX-License-Identifier: GPL-2.0-only
/*
 * Au12x0/Au1550 PSC ALSA ASoC audio support.
 *
 * (c) 2007-2008 MSC Vertriebsges.m.b.H.,
 *	Manuel Lauss <manuel.lauss@gmail.com>
 *
 * Au1xxx-PSC I2S glue.
 *
 * NOTE: so far only PSC slave mode (bit- and frameclock) is supported.
 */

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// Dependencies originally supplied by Linux, ALSA ASoC, Au1xxx, and "psc.h".
#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub msbits: c_int,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_stream {
    pub rates: c_uint,
    pub formats: c_ulong,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_int,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct au1xpsc_audio_data {
    pub cfg: c_ulong,
    pub rate: c_uint,
    pub dmaids: [c_ulong; 2],
    pub mmio: *mut c_void,
    pub pm: [c_ulong; 1],
    pub dai_drv: snd_soc_dai_driver,
}

unsafe extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_void,
    );
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: c_uint,
        num: c_uint,
    ) -> *mut resource;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn __raw_readl(addr: *mut c_void) -> c_ulong;
    fn __raw_writel(value: c_ulong, addr: *mut c_void);
    fn wmb();

    fn PSC_CTRL(data: *mut au1xpsc_audio_data) -> *mut c_void;
    fn PSC_SEL(data: *mut au1xpsc_audio_data) -> *mut c_void;
    fn I2S_STAT(data: *mut au1xpsc_audio_data) -> *mut c_void;
    fn I2S_CFG(data: *mut au1xpsc_audio_data) -> *mut c_void;
    fn I2S_PCR(data: *mut au1xpsc_audio_data) -> *mut c_void;
    fn PSC_I2SCFG_GET_LEN(value: c_int) -> c_int;
    fn PSC_I2SCFG_SET_LEN(value: c_int) -> c_ulong;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ETIMEDOUT: c_int = 110;
const GFP_KERNEL: c_uint = 0;
const IORESOURCE_DMA: c_uint = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;

const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_MSB: c_uint = SND_SOC_DAIFMT_LEFT_J;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 3;
const SND_SOC_DAIFMT_LSB: c_uint = SND_SOC_DAIFMT_RIGHT_J;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0010;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0020;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0030;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0040;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x0100;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x0200;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIDIR_PLAYBACK: c_uint = 1;
const SND_SOC_DAIDIR_CAPTURE: c_uint = 2;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64 = 1 << 1;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64 = 1 << 2;
const SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64 = 1 << 3;
const SND_SOC_POSSIBLE_DAIFMT_NB_IF: u64 = 1 << 4;
const SND_SOC_POSSIBLE_DAIFMT_IB_NF: u64 = 1 << 5;
const SND_SOC_POSSIBLE_DAIFMT_IB_IF: u64 = 1 << 6;

const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 1;

const PSC_I2SCFG_XM: c_ulong = 1 << 0;
const PSC_I2SCFG_MLJ: c_ulong = 1 << 1;
const PSC_I2SCFG_BI: c_ulong = 1 << 2;
const PSC_I2SCFG_WI: c_ulong = 1 << 3;
const PSC_I2SCFG_MS: c_ulong = 1 << 9;
const PSC_I2SCFG_DE_ENABLE: c_ulong = 1 << 10;
const PSC_I2SCFG_RT_FIFO8: c_ulong = 1 << 11;
const PSC_I2SCFG_TT_FIFO8: c_ulong = 1 << 12;
const PSC_I2SSTAT_TB: c_ulong = 1 << 0;
const PSC_I2SSTAT_RB: c_ulong = 1 << 1;
const PSC_I2SSTAT_SR: c_ulong = 1 << 2;
const PSC_I2SSTAT_DR: c_ulong = 1 << 3;
const PSC_I2SPCR_TS: c_ulong = 1 << 0;
const PSC_I2SPCR_RS: c_ulong = 1 << 1;
const PSC_I2SPCR_TP: c_ulong = 1 << 2;
const PSC_I2SPCR_RP: c_ulong = 1 << 3;
const PSC_I2SPCR_TC: c_ulong = 1 << 4;
const PSC_I2SPCR_RC: c_ulong = 1 << 5;
const PSC_CTRL_ENABLE: c_ulong = 1;
const PSC_CTRL_DISABLE: c_ulong = 0;
const PSC_CTRL_SUSPEND: c_ulong = 2;
const PSC_SEL_CLK_MASK: c_ulong = 0xff;
const PSC_SEL_PS_I2SMODE: c_ulong = 1 << 8;

/* supported I2S DAI hardware formats */
const AU1XPSC_I2S_DAIFMT: c_uint =
    SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF;

/* supported I2S direction */
const AU1XPSC_I2S_DIR: c_uint = SND_SOC_DAIDIR_PLAYBACK | SND_SOC_DAIDIR_CAPTURE;

const AU1XPSC_I2S_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;

const AU1XPSC_I2S_FMTS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

fn I2SSTAT_BUSY(stype: c_int) -> c_ulong {
    if stype == SNDRV_PCM_STREAM_PLAYBACK {
        PSC_I2SSTAT_TB
    } else {
        PSC_I2SSTAT_RB
    }
}

fn I2SPCR_START(stype: c_int) -> c_ulong {
    if stype == SNDRV_PCM_STREAM_PLAYBACK {
        PSC_I2SPCR_TS
    } else {
        PSC_I2SPCR_RS
    }
}

fn I2SPCR_STOP(stype: c_int) -> c_ulong {
    if stype == SNDRV_PCM_STREAM_PLAYBACK {
        PSC_I2SPCR_TP
    } else {
        PSC_I2SPCR_RP
    }
}

fn I2SPCR_CLRFIFO(stype: c_int) -> c_ulong {
    if stype == SNDRV_PCM_STREAM_PLAYBACK {
        PSC_I2SPCR_TC
    } else {
        PSC_I2SPCR_RC
    }
}

unsafe extern "C" fn au1xpsc_i2s_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let pscdata = snd_soc_dai_get_drvdata(cpu_dai) as *mut au1xpsc_audio_data;
    let mut ret: c_int;
    let mut ct: c_ulong;

    ret = -EINVAL;

    ct = (*pscdata).cfg;

    ct &= !(PSC_I2SCFG_XM | PSC_I2SCFG_MLJ); /* left-justified */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            ct |= PSC_I2SCFG_XM; /* enable I2S mode */
        }
        SND_SOC_DAIFMT_MSB => {}
        SND_SOC_DAIFMT_LSB => {
            ct |= PSC_I2SCFG_MLJ; /* LSB (right-) justified */
        }
        _ => {
            return ret;
        }
    }

    ct &= !(PSC_I2SCFG_BI | PSC_I2SCFG_WI); /* IB-IF */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            ct |= PSC_I2SCFG_BI | PSC_I2SCFG_WI;
        }
        SND_SOC_DAIFMT_NB_IF => {
            ct |= PSC_I2SCFG_BI;
        }
        SND_SOC_DAIFMT_IB_NF => {
            ct |= PSC_I2SCFG_WI;
        }
        SND_SOC_DAIFMT_IB_IF => {}
        _ => {
            return ret;
        }
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {
            /* CODEC provider */
            ct |= PSC_I2SCFG_MS; /* PSC I2S consumer mode */
        }
        SND_SOC_DAIFMT_BP_FP => {
            /* CODEC consumer */
            ct &= !PSC_I2SCFG_MS; /* PSC I2S provider mode */
        }
        _ => {
            return ret;
        }
    }

    (*pscdata).cfg = ct;
    ret = 0;
    ret
}

unsafe extern "C" fn au1xpsc_i2s_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pscdata = snd_soc_dai_get_drvdata(dai) as *mut au1xpsc_audio_data;
    let cfgbits: c_int;
    let stat: c_ulong;

    /* check if the PSC is already streaming data */
    stat = __raw_readl(I2S_STAT(pscdata));
    if (stat & (PSC_I2SSTAT_TB | PSC_I2SSTAT_RB)) != 0 {
        /* reject parameters not currently set up in hardware */
        cfgbits = __raw_readl(I2S_CFG(pscdata)) as c_int;
        if PSC_I2SCFG_GET_LEN(cfgbits) != (*params).msbits || params_rate(params) != (*pscdata).rate
        {
            return -EINVAL;
        }
    } else {
        /* set sample bitdepth */
        (*pscdata).cfg &= !(0x1f << 4);
        (*pscdata).cfg |= PSC_I2SCFG_SET_LEN((*params).msbits);
        /* remember current rate for other stream */
        (*pscdata).rate = params_rate(params);
    }
    0
}

/* Configure PSC late:  on my devel systems the codec  is I2S master and
 * supplies the i2sbitclock __AND__ i2sMclk (!) to the PSC unit.  ASoC
 * uses aggressive PM and  switches the codec off  when it is not in use
 * which also means the PSC unit doesn't get any clocks and is therefore
 * dead. That's why this chunk here gets called from the trigger callback
 * because I can be reasonably certain the codec is driving the clocks.
 */
unsafe fn au1xpsc_i2s_configure(pscdata: *mut au1xpsc_audio_data) -> c_int {
    let mut tmo: c_ulong;

    /* bring PSC out of sleep, and configure I2S unit */
    __raw_writel(PSC_CTRL_ENABLE, PSC_CTRL(pscdata));
    wmb(); /* drain writebuffer */

    tmo = 1000000;
    while (__raw_readl(I2S_STAT(pscdata)) & PSC_I2SSTAT_SR) == 0 && tmo != 0 {
        tmo = tmo.wrapping_sub(1);
    }

    if tmo == 0 {
        __raw_writel(0, I2S_CFG(pscdata));
        __raw_writel(PSC_CTRL_SUSPEND, PSC_CTRL(pscdata));
        wmb(); /* drain writebuffer */
        return -ETIMEDOUT;
    }

    __raw_writel(0, I2S_CFG(pscdata));
    wmb(); /* drain writebuffer */
    __raw_writel((*pscdata).cfg | PSC_I2SCFG_DE_ENABLE, I2S_CFG(pscdata));
    wmb(); /* drain writebuffer */

    /* wait for I2S controller to become ready */
    tmo = 1000000;
    while (__raw_readl(I2S_STAT(pscdata)) & PSC_I2SSTAT_DR) == 0 && tmo != 0 {
        tmo = tmo.wrapping_sub(1);
    }

    if tmo != 0 {
        return 0;
    }

    __raw_writel(0, I2S_CFG(pscdata));
    __raw_writel(PSC_CTRL_SUSPEND, PSC_CTRL(pscdata));
    wmb(); /* drain writebuffer */
    -ETIMEDOUT
}

unsafe fn au1xpsc_i2s_start(pscdata: *mut au1xpsc_audio_data, stype: c_int) -> c_int {
    let mut tmo: c_ulong;
    let stat: c_ulong;
    let mut ret: c_int;

    ret = 0;

    /* if both TX and RX are idle, configure the PSC  */
    stat = __raw_readl(I2S_STAT(pscdata));
    if (stat & (PSC_I2SSTAT_TB | PSC_I2SSTAT_RB)) == 0 {
        ret = au1xpsc_i2s_configure(pscdata);
        if ret != 0 {
            return ret;
        }
    }

    __raw_writel(I2SPCR_CLRFIFO(stype), I2S_PCR(pscdata));
    wmb(); /* drain writebuffer */
    __raw_writel(I2SPCR_START(stype), I2S_PCR(pscdata));
    wmb(); /* drain writebuffer */

    /* wait for start confirmation */
    tmo = 1000000;
    while (__raw_readl(I2S_STAT(pscdata)) & I2SSTAT_BUSY(stype)) == 0 && tmo != 0 {
        tmo = tmo.wrapping_sub(1);
    }

    if tmo == 0 {
        __raw_writel(I2SPCR_STOP(stype), I2S_PCR(pscdata));
        wmb(); /* drain writebuffer */
        ret = -ETIMEDOUT;
    }
    ret
}

unsafe fn au1xpsc_i2s_stop(pscdata: *mut au1xpsc_audio_data, stype: c_int) -> c_int {
    let mut tmo: c_ulong;
    let stat: c_ulong;

    __raw_writel(I2SPCR_STOP(stype), I2S_PCR(pscdata));
    wmb(); /* drain writebuffer */

    /* wait for stop confirmation */
    tmo = 1000000;
    while (__raw_readl(I2S_STAT(pscdata)) & I2SSTAT_BUSY(stype)) != 0 && tmo != 0 {
        tmo = tmo.wrapping_sub(1);
    }

    /* if both TX and RX are idle, disable PSC */
    stat = __raw_readl(I2S_STAT(pscdata));
    if (stat & (PSC_I2SSTAT_TB | PSC_I2SSTAT_RB)) == 0 {
        __raw_writel(0, I2S_CFG(pscdata));
        wmb(); /* drain writebuffer */
        __raw_writel(PSC_CTRL_SUSPEND, PSC_CTRL(pscdata));
        wmb(); /* drain writebuffer */
    }
    0
}

unsafe extern "C" fn au1xpsc_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pscdata = snd_soc_dai_get_drvdata(dai) as *mut au1xpsc_audio_data;
    let ret: c_int;
    let stype: c_int = (*substream).stream;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            ret = au1xpsc_i2s_start(pscdata, stype);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            ret = au1xpsc_i2s_stop(pscdata, stype);
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

unsafe extern "C" fn au1xpsc_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pscdata = snd_soc_dai_get_drvdata(dai) as *mut au1xpsc_audio_data;
    snd_soc_dai_set_dma_data(
        dai,
        substream,
        (*pscdata).dmaids.as_mut_ptr() as *mut c_void,
    );
    0
}

static au1xpsc_selectable_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
    | SND_SOC_POSSIBLE_DAIFMT_NB_NF
    | SND_SOC_POSSIBLE_DAIFMT_NB_IF
    | SND_SOC_POSSIBLE_DAIFMT_IB_NF
    | SND_SOC_POSSIBLE_DAIFMT_IB_IF;

static au1xpsc_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(au1xpsc_i2s_startup),
    trigger: Some(au1xpsc_i2s_trigger),
    hw_params: Some(au1xpsc_i2s_hw_params),
    set_fmt: Some(au1xpsc_i2s_set_fmt),
    auto_selectable_formats: &au1xpsc_selectable_formats,
    num_auto_selectable_formats: 1,
};

static au1xpsc_i2s_dai_template: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        rates: AU1XPSC_I2S_RATES,
        formats: AU1XPSC_I2S_FMTS,
        channels_min: 2,
        channels_max: 8, /* 2 without external help */
    },
    capture: snd_soc_pcm_stream {
        rates: AU1XPSC_I2S_RATES,
        formats: AU1XPSC_I2S_FMTS,
        channels_min: 2,
        channels_max: 8, /* 2 without external help */
    },
    ops: &au1xpsc_i2s_dai_ops,
    name: ptr::null(),
};

static au1xpsc_i2s_component_name: &[u8] = b"au1xpsc-i2s\0";

static au1xpsc_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: au1xpsc_i2s_component_name.as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn au1xpsc_i2s_drvprobe(pdev: *mut platform_device) -> c_int {
    let mut dmares: *mut resource;
    let sel: c_ulong;
    let wd: *mut au1xpsc_audio_data;

    wd = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<au1xpsc_audio_data>(),
        GFP_KERNEL,
    ) as *mut au1xpsc_audio_data;
    if wd.is_null() {
        return -ENOMEM;
    }

    (*wd).mmio = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*wd).mmio) {
        return PTR_ERR((*wd).mmio);
    }

    dmares = platform_get_resource(pdev, IORESOURCE_DMA, 0);
    if dmares.is_null() {
        return -EBUSY;
    }
    (*wd).dmaids[SNDRV_PCM_STREAM_PLAYBACK as usize] = (*dmares).start;

    dmares = platform_get_resource(pdev, IORESOURCE_DMA, 1);
    if dmares.is_null() {
        return -EBUSY;
    }
    (*wd).dmaids[SNDRV_PCM_STREAM_CAPTURE as usize] = (*dmares).start;

    /* preserve PSC clock source set up by platform (dev.platform_data
     * is already occupied by soc layer)
     */
    sel = __raw_readl(PSC_SEL(wd)) & PSC_SEL_CLK_MASK;
    __raw_writel(PSC_CTRL_DISABLE, PSC_CTRL(wd));
    wmb(); /* drain writebuffer */
    __raw_writel(PSC_SEL_PS_I2SMODE | sel, PSC_SEL(wd));
    __raw_writel(0, I2S_CFG(wd));
    wmb(); /* drain writebuffer */

    /* preconfigure: set max rx/tx fifo depths */
    (*wd).cfg |= PSC_I2SCFG_RT_FIFO8 | PSC_I2SCFG_TT_FIFO8;

    /* don't wait for I2S core to become ready now; clocks may not
     * be running yet; depending on clock input for PSC a wait might
     * time out.
     */

    /* name the DAI like this device instance ("au1xpsc-i2s.PSCINDEX") */
    ptr::copy_nonoverlapping(
        &au1xpsc_i2s_dai_template,
        &mut (*wd).dai_drv,
        1,
    );
    (*wd).dai_drv.name = dev_name(&mut (*pdev).dev);

    platform_set_drvdata(pdev, wd as *mut c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &au1xpsc_i2s_component,
        &mut (*wd).dai_drv,
        1,
    )
}

unsafe extern "C" fn au1xpsc_i2s_drvremove(pdev: *mut platform_device) {
    let wd = platform_get_drvdata(pdev) as *mut au1xpsc_audio_data;

    __raw_writel(0, I2S_CFG(wd));
    wmb(); /* drain writebuffer */
    __raw_writel(PSC_CTRL_DISABLE, PSC_CTRL(wd));
    wmb(); /* drain writebuffer */
}

unsafe extern "C" fn au1xpsc_i2s_drvsuspend(dev: *mut device) -> c_int {
    let wd = dev_get_drvdata(dev) as *mut au1xpsc_audio_data;

    /* save interesting register and disable PSC */
    (*wd).pm[0] = __raw_readl(PSC_SEL(wd));

    __raw_writel(0, I2S_CFG(wd));
    wmb(); /* drain writebuffer */
    __raw_writel(PSC_CTRL_DISABLE, PSC_CTRL(wd));
    wmb(); /* drain writebuffer */

    0
}

unsafe extern "C" fn au1xpsc_i2s_drvresume(dev: *mut device) -> c_int {
    let wd = dev_get_drvdata(dev) as *mut au1xpsc_audio_data;

    /* select I2S mode and PSC clock */
    __raw_writel(PSC_CTRL_DISABLE, PSC_CTRL(wd));
    wmb(); /* drain writebuffer */
    __raw_writel(0, PSC_SEL(wd));
    wmb(); /* drain writebuffer */
    __raw_writel((*wd).pm[0], PSC_SEL(wd));
    wmb(); /* drain writebuffer */

    0
}

static au1xpsci2s_pmops: dev_pm_ops = dev_pm_ops {
    suspend: Some(au1xpsc_i2s_drvsuspend),
    resume: Some(au1xpsc_i2s_drvresume),
};

static au1xpsc_i2s_driver_name: &[u8] = b"au1xpsc_i2s\0";

static mut au1xpsc_i2s_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: au1xpsc_i2s_driver_name.as_ptr() as *const c_char,
        pm: &au1xpsci2s_pmops,
    },
    probe: Some(au1xpsc_i2s_drvprobe),
    remove: Some(au1xpsc_i2s_drvremove),
};

// module_platform_driver(au1xpsc_i2s_driver);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Au12x0/Au1550 PSC I2S ALSA ASoC audio driver");
// MODULE_AUTHOR("Manuel Lauss");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
