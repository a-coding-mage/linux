// SPDX-License-Identifier: GPL-2.0-only
/*
 * Au1000/Au1500/Au1100 I2S controller driver for ASoC
 *
 * (c) 2011 Manuel Lauss <manuel.lauss@googlemail.com>
 *
 * Note: clock supplied to the I2S controller must be 256x samplerate.
 */

// C dependencies: linux/init.h, linux/module.h, linux/slab.h,
// linux/suspend.h, sound/core.h, sound/pcm.h, sound/initval.h,
// sound/soc.h, asm/mach-au1x00/au1000.h, and "psc.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of, addr_of_mut};

const I2S_RXTX: c_int = 0x00;
const I2S_CFG: c_int = 0x04;
const I2S_ENABLE: c_int = 0x08;

const CFG_XU: c_ulong = 1 << 25; /* tx underflow */
const CFG_XO: c_ulong = 1 << 24;
const CFG_RU: c_ulong = 1 << 23;
const CFG_RO: c_ulong = 1 << 22;
const CFG_TR: c_ulong = 1 << 21;
const CFG_TE: c_ulong = 1 << 20;
const CFG_TF: c_ulong = 1 << 19;
const CFG_RR: c_ulong = 1 << 18;
const CFG_RF: c_ulong = 1 << 17;
const CFG_ICK: c_ulong = 1 << 12; /* clock invert */
const CFG_PD: c_ulong = 1 << 11; /* set to make I2SDIO INPUT */
const CFG_LB: c_ulong = 1 << 10; /* loopback */
const CFG_IC: c_ulong = 1 << 9; /* word select invert */
const CFG_FM_I2S: c_ulong = 0 << 7; /* I2S format */
const CFG_FM_LJ: c_ulong = 1 << 7; /* left-justified */
const CFG_FM_RJ: c_ulong = 2 << 7; /* right-justified */
const CFG_FM_MASK: c_ulong = 3 << 7;
const CFG_TN: c_ulong = 1 << 6; /* tx fifo en */
const CFG_RN: c_ulong = 1 << 5; /* rx fifo en */
const CFG_SZ_8: c_ulong = 0x08;
const CFG_SZ_16: c_ulong = 0x10;
const CFG_SZ_18: c_ulong = 0x12;
const CFG_SZ_20: c_ulong = 0x14;
const CFG_SZ_24: c_ulong = 0x18;
const CFG_SZ_MASK: c_ulong = 0x1f;
const EN_D: c_ulong = 1 << 1; /* DISable */
const EN_CE: c_ulong = 1 << 0; /* clock enable */

/* only limited by clock generator and board design */
const AU1XI2SC_RATES: c_uint = SNDRV_PCM_RATE_CONTINUOUS;

const AU1XI2SC_FMTS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S16_BE
    | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_U16_BE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_U18_3LE
    | SNDRV_PCM_FMTBIT_S18_3BE
    | SNDRV_PCM_FMTBIT_U18_3BE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_U20_3LE
    | SNDRV_PCM_FMTBIT_S20_3BE
    | SNDRV_PCM_FMTBIT_U20_3BE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S24_BE
    | SNDRV_PCM_FMTBIT_U24_LE
    | SNDRV_PCM_FMTBIT_U24_BE
    | 0;

#[repr(C)]
pub struct au1xpsc_audio_data {
    pub mmio: *mut u8,
    pub cfg: c_ulong,
    pub dmaids: [c_ulong; 2],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
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
    pub name: *const c_char,
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub rates: c_uint,
    pub formats: u64,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub symmetric_rate: c_uint,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S16_BE: u64;
    static SNDRV_PCM_FMTBIT_U16_LE: u64;
    static SNDRV_PCM_FMTBIT_U16_BE: u64;
    static SNDRV_PCM_FMTBIT_S18_3LE: u64;
    static SNDRV_PCM_FMTBIT_U18_3LE: u64;
    static SNDRV_PCM_FMTBIT_S18_3BE: u64;
    static SNDRV_PCM_FMTBIT_U18_3BE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_U20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3BE: u64;
    static SNDRV_PCM_FMTBIT_U20_3BE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_BE: u64;
    static SNDRV_PCM_FMTBIT_U24_LE: u64;
    static SNDRV_PCM_FMTBIT_U24_BE: u64;

    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_MSB: c_uint;
    static SND_SOC_DAIFMT_LSB: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;

    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64;
    static SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64;
    static SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64;
    static SND_SOC_POSSIBLE_DAIFMT_NB_IF: u64;
    static SND_SOC_POSSIBLE_DAIFMT_IB_NF: u64;
    static SND_SOC_POSSIBLE_DAIFMT_IB_IF: u64;

    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
    static EBUSY: c_int;
    static GFP_KERNEL: c_uint;
    static IORESOURCE_MEM: c_uint;
    static IORESOURCE_DMA: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: usize;
    static SNDRV_PCM_STREAM_CAPTURE: usize;
    static PCM_TX: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;

    fn __raw_readl(addr: *mut u8) -> c_ulong;
    fn __raw_writel(v: c_ulong, addr: *mut u8);
    fn wmb();
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut au1xpsc_audio_data;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_ulong,
    );
    fn SUBSTREAM_TYPE(substream: *mut snd_pcm_substream) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: c_uint,
        num: c_uint,
    ) -> *mut resource;
    fn resource_size(res: *mut resource) -> c_ulong;
    fn devm_request_mem_region(
        dev: *mut device,
        start: c_ulong,
        n: c_ulong,
        name: *const c_char,
    ) -> *mut c_void;
    fn devm_ioremap(dev: *mut device, offset: c_ulong, size: c_ulong) -> *mut u8;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut au1xpsc_audio_data);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut au1xpsc_audio_data;
    fn dev_get_drvdata(dev: *mut device) -> *mut au1xpsc_audio_data;
    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
}

#[inline]
unsafe fn RD(ctx: *mut au1xpsc_audio_data, reg: c_int) -> c_ulong {
    unsafe { __raw_readl((*ctx).mmio.add(reg as usize)) }
}

#[inline]
unsafe fn WR(ctx: *mut au1xpsc_audio_data, reg: c_int, v: c_ulong) {
    unsafe {
        __raw_writel(v, (*ctx).mmio.add(reg as usize));
        wmb();
    }
}

unsafe extern "C" fn au1xi2s_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let ctx = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };
    let mut c: c_ulong;
    let mut ret: c_int;

    ret = unsafe { -EINVAL };
    c = unsafe { (*ctx).cfg };

    c &= !CFG_FM_MASK;
    match unsafe { fmt & SND_SOC_DAIFMT_FORMAT_MASK } {
        x if x == unsafe { SND_SOC_DAIFMT_I2S } => {
            c |= CFG_FM_I2S;
        }
        x if x == unsafe { SND_SOC_DAIFMT_MSB } => {
            c |= CFG_FM_RJ;
        }
        x if x == unsafe { SND_SOC_DAIFMT_LSB } => {
            c |= CFG_FM_LJ;
        }
        _ => {
            return ret;
        }
    }

    c &= !(CFG_IC | CFG_ICK); /* IB-IF */
    match unsafe { fmt & SND_SOC_DAIFMT_INV_MASK } {
        x if x == unsafe { SND_SOC_DAIFMT_NB_NF } => {
            c |= CFG_IC | CFG_ICK;
        }
        x if x == unsafe { SND_SOC_DAIFMT_NB_IF } => {
            c |= CFG_IC;
        }
        x if x == unsafe { SND_SOC_DAIFMT_IB_NF } => {
            c |= CFG_ICK;
        }
        x if x == unsafe { SND_SOC_DAIFMT_IB_IF } => {}
        _ => {
            return ret;
        }
    }

    /* I2S controller only supports provider */
    match unsafe { fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK } {
        x if x == unsafe { SND_SOC_DAIFMT_BP_FP } => {
            /* CODEC consumer */
        }
        _ => {
            return ret;
        }
    }

    ret = 0;
    unsafe {
        (*ctx).cfg = c;
    }
    ret
}

unsafe extern "C" fn au1xi2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ctx = unsafe { snd_soc_dai_get_drvdata(dai) };
    let stype = unsafe { SUBSTREAM_TYPE(substream) };

    match cmd {
        x if x == unsafe { SNDRV_PCM_TRIGGER_START } || x == unsafe { SNDRV_PCM_TRIGGER_RESUME } => {
            /* power up */
            unsafe {
                WR(ctx, I2S_ENABLE, EN_D | EN_CE);
                WR(ctx, I2S_ENABLE, EN_CE);
                (*ctx).cfg |= if stype == PCM_TX { CFG_TN } else { CFG_RN };
                WR(ctx, I2S_CFG, (*ctx).cfg);
            }
        }
        x if x == unsafe { SNDRV_PCM_TRIGGER_STOP } || x == unsafe { SNDRV_PCM_TRIGGER_SUSPEND } => {
            unsafe {
                (*ctx).cfg &= !(if stype == PCM_TX { CFG_TN } else { CFG_RN });
                WR(ctx, I2S_CFG, (*ctx).cfg);
                WR(ctx, I2S_ENABLE, EN_D); /* power off */
            }
        }
        _ => {
            return unsafe { -EINVAL };
        }
    }

    0
}

fn msbits_to_reg(msbits: c_int) -> c_ulong {
    match msbits {
        8 => CFG_SZ_8,
        16 => CFG_SZ_16,
        18 => CFG_SZ_18,
        20 => CFG_SZ_20,
        24 => CFG_SZ_24,
        _ => 0,
    }
}

unsafe extern "C" fn au1xi2s_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ctx = unsafe { snd_soc_dai_get_drvdata(dai) };
    let v: c_ulong;

    v = msbits_to_reg(unsafe { (*params).msbits });
    if v == 0 {
        return unsafe { -EINVAL };
    }

    unsafe {
        (*ctx).cfg &= !CFG_SZ_MASK;
        (*ctx).cfg |= v;
    }
    0
}

unsafe extern "C" fn au1xi2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ctx = unsafe { snd_soc_dai_get_drvdata(dai) };
    unsafe {
        snd_soc_dai_set_dma_data(dai, substream, addr_of_mut!((*ctx).dmaids[0]));
    }
    0
}

static au1xi2s_selectable_formats: u64 = unsafe {
    SND_SOC_POSSIBLE_DAIFMT_I2S
        | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
        | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
        | SND_SOC_POSSIBLE_DAIFMT_NB_NF
        | SND_SOC_POSSIBLE_DAIFMT_NB_IF
        | SND_SOC_POSSIBLE_DAIFMT_IB_NF
        | SND_SOC_POSSIBLE_DAIFMT_IB_IF
};

static au1xi2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(au1xi2s_startup),
    trigger: Some(au1xi2s_trigger),
    hw_params: Some(au1xi2s_hw_params),
    set_fmt: Some(au1xi2s_set_fmt),
    auto_selectable_formats: addr_of!(au1xi2s_selectable_formats),
    num_auto_selectable_formats: 1,
};

static mut au1xi2s_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    symmetric_rate: 1,
    playback: snd_soc_pcm_stream {
        rates: AU1XI2SC_RATES,
        formats: AU1XI2SC_FMTS,
        channels_min: 2,
        channels_max: 2,
    },
    capture: snd_soc_pcm_stream {
        rates: AU1XI2SC_RATES,
        formats: AU1XI2SC_FMTS,
        channels_min: 2,
        channels_max: 2,
    },
    ops: addr_of!(au1xi2s_dai_ops),
};

static au1xi2s_component_name: &[u8; 8] = b"au1xi2s\0";
static au1xi2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: au1xi2s_component_name.as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn au1xi2s_drvprobe(pdev: *mut platform_device) -> c_int {
    let mut iores: *mut resource;
    let mut dmares: *mut resource;
    let ctx: *mut au1xpsc_audio_data;

    ctx = unsafe {
        devm_kzalloc(
            addr_of_mut!((*pdev).dev),
            size_of::<au1xpsc_audio_data>(),
            GFP_KERNEL,
        ) as *mut au1xpsc_audio_data
    };
    if ctx.is_null() {
        return unsafe { -ENOMEM };
    }

    iores = unsafe { platform_get_resource(pdev, IORESOURCE_MEM, 0) };
    if iores.is_null() {
        return unsafe { -ENODEV };
    }

    if unsafe {
        devm_request_mem_region(
            addr_of_mut!((*pdev).dev),
            (*iores).start,
            resource_size(iores),
            (*pdev).name,
        )
    }
    .is_null()
    {
        return unsafe { -EBUSY };
    }

    unsafe {
        (*ctx).mmio = devm_ioremap(
            addr_of_mut!((*pdev).dev),
            (*iores).start,
            resource_size(iores),
        );
    }
    if unsafe { (*ctx).mmio.is_null() } {
        return unsafe { -EBUSY };
    }

    dmares = unsafe { platform_get_resource(pdev, IORESOURCE_DMA, 0) };
    if dmares.is_null() {
        return unsafe { -EBUSY };
    }
    unsafe {
        (*ctx).dmaids[SNDRV_PCM_STREAM_PLAYBACK] = (*dmares).start;
    }

    dmares = unsafe { platform_get_resource(pdev, IORESOURCE_DMA, 1) };
    if dmares.is_null() {
        return unsafe { -EBUSY };
    }
    unsafe {
        (*ctx).dmaids[SNDRV_PCM_STREAM_CAPTURE] = (*dmares).start;
    }

    unsafe {
        platform_set_drvdata(pdev, ctx);
    }

    unsafe {
        snd_soc_register_component(
            addr_of_mut!((*pdev).dev),
            addr_of!(au1xi2s_component),
            addr_of_mut!(au1xi2s_dai_driver),
            1,
        )
    }
}

unsafe extern "C" fn au1xi2s_drvremove(pdev: *mut platform_device) {
    let ctx = unsafe { platform_get_drvdata(pdev) };

    unsafe {
        snd_soc_unregister_component(addr_of_mut!((*pdev).dev));

        WR(ctx, I2S_ENABLE, EN_D); /* clock off, disable */
    }
}

unsafe extern "C" fn au1xi2s_drvsuspend(dev: *mut device) -> c_int {
    let ctx = unsafe { dev_get_drvdata(dev) };

    unsafe {
        WR(ctx, I2S_ENABLE, EN_D); /* clock off, disable */
    }

    0
}

unsafe extern "C" fn au1xi2s_drvresume(_dev: *mut device) -> c_int {
    0
}

static au1xi2sc_pmops: dev_pm_ops = dev_pm_ops {
    suspend: Some(au1xi2s_drvsuspend),
    resume: Some(au1xi2s_drvresume),
};

static au1xi2s_driver_name: &[u8; 13] = b"alchemy-i2sc\0";
static mut au1xi2s_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: au1xi2s_driver_name.as_ptr() as *const c_char,
        pm: addr_of!(au1xi2sc_pmops),
    },
    probe: Some(au1xi2s_drvprobe),
    remove: Some(au1xi2s_drvremove),
};

// module_platform_driver(au1xi2s_driver);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Au1000/1500/1100 I2S ASoC driver");
// MODULE_AUTHOR("Manuel Lauss");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
