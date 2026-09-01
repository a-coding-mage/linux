// SPDX-License-Identifier: GPL-2.0-only
/*
 * Au1000/Au1500/Au1100 AC97C controller driver for ASoC
 *
 * (c) 2011 Manuel Lauss <manuel.lauss@googlemail.com>
 *
 * based on the old ALSA driver originally written by
 *			Charles Eidsness <charles@cooper-street.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

/* dependencies from linux/init.h, linux/module.h, linux/slab.h,
 * linux/device.h, linux/delay.h, linux/mutex.h, linux/platform_device.h,
 * linux/suspend.h, sound/core.h, sound/pcm.h, sound/initval.h, sound/soc.h,
 * asm/mach-au1x00/au1000.h, and psc.h.
 */

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct au1xpsc_audio_data {
    pub lock: mutex,
    pub mmio: *mut u8,
    pub cfg: c_ulong,
    pub dmaids: [c_ulong; 2],
}

pub type ac97_read_fn = unsafe extern "C" fn(*mut snd_ac97, u16) -> u16;
pub type ac97_write_fn = unsafe extern "C" fn(*mut snd_ac97, u16, u16);
pub type ac97_reset_fn = unsafe extern "C" fn(*mut snd_ac97);
pub type dai_probe_fn = unsafe extern "C" fn(*mut snd_soc_dai) -> c_int;
pub type dai_startup_fn =
    unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int;
pub type platform_probe_fn = unsafe extern "C" fn(*mut platform_device) -> c_int;
pub type platform_remove_fn = unsafe extern "C" fn(*mut platform_device);
pub type dev_pm_fn = unsafe extern "C" fn(*mut device) -> c_int;

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub read: Option<ac97_read_fn>,
    pub write: Option<ac97_write_fn>,
    pub reset: Option<ac97_reset_fn>,
    pub warm_reset: Option<ac97_reset_fn>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<dai_probe_fn>,
    pub startup: Option<dai_startup_fn>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub rates: c_ulong,
    pub formats: c_ulong,
    pub channels_min: u32,
    pub channels_max: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_int,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<dev_pm_fn>,
    pub resume: Option<dev_pm_fn>,
}

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_inner,
    pub probe: Option<platform_probe_fn>,
    pub remove: Option<platform_remove_fn>,
}

unsafe extern "C" {
    fn __raw_readl(addr: *const c_void) -> c_ulong;
    fn __raw_writel(val: c_ulong, addr: *mut c_void);
    fn wmb();
    fn udelay(usecs: c_ulong);
    fn msleep(msecs: c_ulong);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn pr_debug(fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_ulong) -> *mut c_void;
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: c_ulong,
        num: c_ulong,
    ) -> *mut resource;
    fn devm_request_mem_region(
        dev: *mut device,
        start: c_ulong,
        n: c_ulong,
        name: *const c_char,
    ) -> *mut c_void;
    fn resource_size(res: *mut resource) -> c_ulong;
    fn devm_ioremap(dev: *mut device, offset: c_ulong, size: c_ulong) -> *mut u8;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_set_ac97_ops(ops: *mut snd_ac97_bus_ops) -> c_int;
    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_void,
    );
}

const GFP_KERNEL: c_ulong = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const IORESOURCE_MEM: c_ulong = 0x0000_0200;
const IORESOURCE_DMA: c_ulong = 0x0000_0800;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_PCM_RATE_CONTINUOUS: c_ulong = 1 << 30;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S16_BE: c_ulong = 1 << 3;
const KERN_ERR: &[u8] = b"\x013";

/* register offsets and bits */
const AC97_CONFIG: c_int = 0x00;
const AC97_STATUS: c_int = 0x04;
const AC97_DATA: c_int = 0x08;
const AC97_CMDRESP: c_int = 0x0c;
const AC97_ENABLE: c_int = 0x10;

const fn CFG_RC(x: c_ulong) -> c_ulong {
    ((x & 0x3ff) << 13) /* valid rx slots mask */
}
const fn CFG_XS(x: c_ulong) -> c_ulong {
    ((x & 0x3ff) << 3) /* valid tx slots mask */
}
const CFG_SG: c_ulong = 1 << 2; /* sync gate */
const CFG_SN: c_ulong = 1 << 1; /* sync control */
const CFG_RS: c_ulong = 1 << 0; /* acrst# control */
const STAT_XU: c_ulong = 1 << 11; /* tx underflow */
const STAT_XO: c_ulong = 1 << 10; /* tx overflow */
const STAT_RU: c_ulong = 1 << 9; /* rx underflow */
const STAT_RO: c_ulong = 1 << 8; /* rx overflow */
const STAT_RD: c_ulong = 1 << 7; /* codec ready */
const STAT_CP: c_ulong = 1 << 6; /* command pending */
const STAT_TE: c_ulong = 1 << 4; /* tx fifo empty */
const STAT_TF: c_ulong = 1 << 3; /* tx fifo full */
const STAT_RE: c_ulong = 1 << 1; /* rx fifo empty */
const STAT_RF: c_ulong = 1 << 0; /* rx fifo full */
const fn CMD_SET_DATA(x: c_ulong) -> c_ulong {
    ((x & 0xffff) << 16)
}
const fn CMD_GET_DATA(x: c_ulong) -> c_ulong {
    x & 0xffff
}
const CMD_READ: c_ulong = 1 << 7;
const CMD_WRITE: c_ulong = 0 << 7;
const fn CMD_IDX(x: c_ulong) -> c_ulong {
    x & 0x7f
}
const EN_D: c_ulong = 1 << 1; /* DISable bit */
const EN_CE: c_ulong = 1 << 0; /* clock enable bit */

/* how often to retry failed codec register reads/writes */
const AC97_RW_RETRIES: u32 = 5;

const AC97_RATES: c_ulong = SNDRV_PCM_RATE_CONTINUOUS;

const AC97_FMTS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE;

/* instance data. There can be only one, MacLeod!!!!, fortunately there IS only
 * once AC97C on early Alchemy chips. The newer ones aren't so lucky.
 */
static mut ac97c_workdata: *mut au1xpsc_audio_data = ptr::null_mut();

unsafe fn ac97_to_ctx(_x: *mut snd_ac97) -> *mut au1xpsc_audio_data {
    unsafe { ac97c_workdata }
}

#[inline]
unsafe fn RD(ctx: *mut au1xpsc_audio_data, reg: c_int) -> c_ulong {
    unsafe { __raw_readl((*ctx).mmio.add(reg as usize) as *const c_void) }
}

#[inline]
unsafe fn WR(ctx: *mut au1xpsc_audio_data, reg: c_int, v: c_ulong) {
    unsafe {
        __raw_writel(v, (*ctx).mmio.add(reg as usize) as *mut c_void);
        wmb();
    }
}

unsafe extern "C" fn au1xac97c_ac97_read(ac97: *mut snd_ac97, r: u16) -> u16 {
    let ctx: *mut au1xpsc_audio_data = unsafe { ac97_to_ctx(ac97) };
    let mut tmo: u32;
    let mut retry: u32;
    let mut data: c_ulong;

    data = !0;
    retry = AC97_RW_RETRIES;
    loop {
        unsafe {
            mutex_lock(&mut (*ctx).lock);
        }

        tmo = 6;
        while unsafe { (RD(ctx, AC97_STATUS) & STAT_CP) != 0 } && {
            tmo = tmo.wrapping_sub(1);
            tmo != 0
        } {
            unsafe {
                udelay(21);
            } /* wait an ac97 frame time */
        }
        if tmo == 0 {
            unsafe {
                pr_debug(c"ac97rd timeout #1\n".as_ptr());
            }
        } else {
            unsafe {
                WR(ctx, AC97_CMDRESP, CMD_IDX(r as c_ulong) | CMD_READ);
            }

            /* stupid errata: data is only valid for 21us, so
             * poll, Forrest, poll...
             */
            tmo = 0x10000;
            while unsafe { (RD(ctx, AC97_STATUS) & STAT_CP) != 0 } && {
                tmo = tmo.wrapping_sub(1);
                tmo != 0
            } {
                unsafe {
                    asm!("nop");
                }
            }
            data = unsafe { RD(ctx, AC97_CMDRESP) };

            if tmo == 0 {
                unsafe {
                    pr_debug(c"ac97rd timeout #2\n".as_ptr());
                }
            }
        }

        unsafe {
            mutex_unlock(&mut (*ctx).lock);
        }
        retry = retry.wrapping_sub(1);
        if !(retry != 0 && tmo == 0) {
            break;
        }
    }

    unsafe {
        pr_debug(
            c"AC97RD %04x %04lx %d\n".as_ptr(),
            r as c_int,
            data,
            retry as c_int,
        );
    }

    if retry != 0 {
        (data & 0xffff) as u16
    } else {
        0xffff
    }
}

unsafe extern "C" fn au1xac97c_ac97_write(ac97: *mut snd_ac97, r: u16, v: u16) {
    let ctx: *mut au1xpsc_audio_data = unsafe { ac97_to_ctx(ac97) };
    let mut tmo: u32;
    let mut retry: u32;

    retry = AC97_RW_RETRIES;
    loop {
        unsafe {
            mutex_lock(&mut (*ctx).lock);
        }

        tmo = 5;
        while unsafe { (RD(ctx, AC97_STATUS) & STAT_CP) != 0 } && tmo != 0 {
            unsafe {
                udelay(21);
            }
            tmo = tmo.wrapping_sub(1);
        }
        if tmo == 0 {
            unsafe {
                pr_debug(c"ac97wr timeout #1\n".as_ptr());
            }
        } else {
            unsafe {
                WR(
                    ctx,
                    AC97_CMDRESP,
                    CMD_WRITE | CMD_IDX(r as c_ulong) | CMD_SET_DATA(v as c_ulong),
                );
            }

            tmo = 10;
            while unsafe { (RD(ctx, AC97_STATUS) & STAT_CP) != 0 } && tmo != 0 {
                unsafe {
                    udelay(21);
                }
                tmo = tmo.wrapping_sub(1);
            }
            if tmo == 0 {
                unsafe {
                    pr_debug(c"ac97wr timeout #2\n".as_ptr());
                }
            }
        }
        unsafe {
            mutex_unlock(&mut (*ctx).lock);
        }
        retry = retry.wrapping_sub(1);
        if !(retry != 0 && tmo == 0) {
            break;
        }
    }

    unsafe {
        pr_debug(
            c"AC97WR %04x %04x %d\n".as_ptr(),
            r as c_int,
            v as c_int,
            retry as c_int,
        );
    }
}

unsafe extern "C" fn au1xac97c_ac97_warm_reset(ac97: *mut snd_ac97) {
    let ctx: *mut au1xpsc_audio_data = unsafe { ac97_to_ctx(ac97) };

    unsafe {
        WR(ctx, AC97_CONFIG, (*ctx).cfg | CFG_SG | CFG_SN);
        msleep(20);
        WR(ctx, AC97_CONFIG, (*ctx).cfg | CFG_SG);
        WR(ctx, AC97_CONFIG, (*ctx).cfg);
    }
}

unsafe extern "C" fn au1xac97c_ac97_cold_reset(ac97: *mut snd_ac97) {
    let ctx: *mut au1xpsc_audio_data = unsafe { ac97_to_ctx(ac97) };
    let mut i: c_int;

    unsafe {
        WR(ctx, AC97_CONFIG, (*ctx).cfg | CFG_RS);
        msleep(500);
        WR(ctx, AC97_CONFIG, (*ctx).cfg);
    }

    /* wait for codec ready */
    i = 50;
    while unsafe { (RD(ctx, AC97_STATUS) & STAT_RD) == 0 } && {
        i -= 1;
        i != 0
    } {
        unsafe {
            msleep(20);
        }
    }
    if i == 0 {
        unsafe {
            printk(c"\x013ac97c: codec not ready after cold reset\n".as_ptr());
        }
    }
}

/* AC97 controller operations */
static mut ac97c_bus_ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
    read: Some(au1xac97c_ac97_read),
    write: Some(au1xac97c_ac97_write),
    reset: Some(au1xac97c_ac97_cold_reset),
    warm_reset: Some(au1xac97c_ac97_warm_reset),
};

unsafe extern "C" fn alchemy_ac97c_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ctx: *mut au1xpsc_audio_data = unsafe { snd_soc_dai_get_drvdata(dai) as *mut _ };
    unsafe {
        snd_soc_dai_set_dma_data(
            dai,
            substream,
            &mut (*ctx).dmaids[0] as *mut c_ulong as *mut c_void,
        );
    }
    0
}

unsafe extern "C" fn au1xac97c_dai_probe(_dai: *mut snd_soc_dai) -> c_int {
    if unsafe { !ac97c_workdata.is_null() } {
        0
    } else {
        -ENODEV
    }
}

static alchemy_ac97c_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(au1xac97c_dai_probe),
    startup: Some(alchemy_ac97c_startup),
};

static mut au1xac97c_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"alchemy-ac97c".as_ptr(),
    playback: snd_soc_pcm_stream {
        rates: AC97_RATES,
        formats: AC97_FMTS,
        channels_min: 2,
        channels_max: 2,
    },
    capture: snd_soc_pcm_stream {
        rates: AC97_RATES,
        formats: AC97_FMTS,
        channels_min: 2,
        channels_max: 2,
    },
    ops: &alchemy_ac97c_ops,
};

static au1xac97c_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"au1xac97c".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn au1xac97c_drvprobe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mut iores: *mut resource;
    let mut dmares: *mut resource;
    let ctx: *mut au1xpsc_audio_data;

    ctx = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<au1xpsc_audio_data>(),
            GFP_KERNEL,
        ) as *mut au1xpsc_audio_data
    };
    if ctx.is_null() {
        return -ENOMEM;
    }

    unsafe {
        mutex_init(&mut (*ctx).lock);
    }

    iores = unsafe { platform_get_resource(pdev, IORESOURCE_MEM, 0) };
    if iores.is_null() {
        return -ENODEV;
    }

    if unsafe {
        devm_request_mem_region(
            &mut (*pdev).dev,
            (*iores).start,
            resource_size(iores),
            (*pdev).name,
        )
    }
    .is_null()
    {
        return -EBUSY;
    }

    unsafe {
        (*ctx).mmio = devm_ioremap(&mut (*pdev).dev, (*iores).start, resource_size(iores));
    }
    if unsafe { (*ctx).mmio.is_null() } {
        return -EBUSY;
    }

    dmares = unsafe { platform_get_resource(pdev, IORESOURCE_DMA, 0) };
    if dmares.is_null() {
        return -EBUSY;
    }
    unsafe {
        (*ctx).dmaids[SNDRV_PCM_STREAM_PLAYBACK] = (*dmares).start;
    }

    dmares = unsafe { platform_get_resource(pdev, IORESOURCE_DMA, 1) };
    if dmares.is_null() {
        return -EBUSY;
    }
    unsafe {
        (*ctx).dmaids[SNDRV_PCM_STREAM_CAPTURE] = (*dmares).start;
    }

    /* switch it on */
    unsafe {
        WR(ctx, AC97_ENABLE, EN_D | EN_CE);
        WR(ctx, AC97_ENABLE, EN_CE);

        (*ctx).cfg = CFG_RC(3) | CFG_XS(3);
        WR(ctx, AC97_CONFIG, (*ctx).cfg);

        platform_set_drvdata(pdev, ctx as *mut c_void);
    }

    ret = unsafe { snd_soc_set_ac97_ops(&mut ac97c_bus_ops) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe {
        snd_soc_register_component(
            &mut (*pdev).dev,
            &au1xac97c_component,
            &mut au1xac97c_dai_driver,
            1,
        )
    };
    if ret != 0 {
        return ret;
    }

    unsafe {
        ac97c_workdata = ctx;
    }
    0
}

unsafe extern "C" fn au1xac97c_drvremove(pdev: *mut platform_device) {
    let ctx: *mut au1xpsc_audio_data =
        unsafe { platform_get_drvdata(pdev) as *mut au1xpsc_audio_data };

    unsafe {
        snd_soc_unregister_component(&mut (*pdev).dev);

        WR(ctx, AC97_ENABLE, EN_D); /* clock off, disable */

        ac97c_workdata = ptr::null_mut(); /* MDEV */
    }
}

/* CONFIG_PM */
unsafe extern "C" fn au1xac97c_drvsuspend(dev: *mut device) -> c_int {
    let ctx: *mut au1xpsc_audio_data =
        unsafe { dev_get_drvdata(dev) as *mut au1xpsc_audio_data };

    unsafe {
        WR(ctx, AC97_ENABLE, EN_D); /* clock off, disable */
    }

    0
}

unsafe extern "C" fn au1xac97c_drvresume(dev: *mut device) -> c_int {
    let ctx: *mut au1xpsc_audio_data =
        unsafe { dev_get_drvdata(dev) as *mut au1xpsc_audio_data };

    unsafe {
        WR(ctx, AC97_ENABLE, EN_D | EN_CE);
        WR(ctx, AC97_ENABLE, EN_CE);
        WR(ctx, AC97_CONFIG, (*ctx).cfg);
    }

    0
}

static au1xpscac97_pmops: dev_pm_ops = dev_pm_ops {
    suspend: Some(au1xac97c_drvsuspend),
    resume: Some(au1xac97c_drvresume),
};

const AU1XPSCAC97_PMOPS: *const dev_pm_ops = &au1xpscac97_pmops;

/* without CONFIG_PM, AU1XPSCAC97_PMOPS is NULL. */

static mut au1xac97c_driver: platform_driver = platform_driver {
    driver: driver_inner {
        name: c"alchemy-ac97c".as_ptr(),
        pm: AU1XPSCAC97_PMOPS,
    },
    probe: Some(au1xac97c_drvprobe),
    remove: Some(au1xac97c_drvremove),
};

/* module_platform_driver(au1xac97c_driver); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("Au1000/1500/1100 AC97C ASoC driver"); */
/* MODULE_AUTHOR("Manuel Lauss"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
