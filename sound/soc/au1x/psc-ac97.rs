// SPDX-License-Identifier: GPL-2.0-only
/*
 * Au12x0/Au1550 PSC ALSA ASoC audio support.
 *
 * (c) 2007-2009 MSC Vertriebsges.m.b.H.,
 *	Manuel Lauss <manuel.lauss@gmail.com>
 *
 * Au1xxx-PSC AC97 glue.
 */

/* Dependencies from the original C source:
 * linux/init.h, linux/module.h, linux/slab.h, linux/device.h, linux/delay.h,
 * linux/mutex.h, linux/suspend.h, sound/core.h, sound/pcm.h, sound/initval.h,
 * sound/soc.h, asm/mach-au1x00/au1000.h, asm/mach-au1x00/au1xxx_psc.h,
 * and "psc.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ushort, c_void};

const AC97_RW_RETRIES: u16 = 5;

const AC97_DIR: c_ulong = SND_SOC_DAIDIR_PLAYBACK | SND_SOC_DAIDIR_CAPTURE;

const AC97_RATES: c_ulong = SNDRV_PCM_RATE_8000_48000;

const AC97_FMTS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3BE;

#[inline]
unsafe fn AC97PCR_START(stype: c_int) -> c_ulong {
    if stype == SNDRV_PCM_STREAM_PLAYBACK {
        PSC_AC97PCR_TS
    } else {
        PSC_AC97PCR_RS
    }
}

#[inline]
unsafe fn AC97PCR_STOP(stype: c_int) -> c_ulong {
    if stype == SNDRV_PCM_STREAM_PLAYBACK {
        PSC_AC97PCR_TP
    } else {
        PSC_AC97PCR_RP
    }
}

#[inline]
unsafe fn AC97PCR_CLRFIFO(stype: c_int) -> c_ulong {
    if stype == SNDRV_PCM_STREAM_PLAYBACK {
        PSC_AC97PCR_TC
    } else {
        PSC_AC97PCR_RC
    }
}

#[inline]
unsafe fn AC97STAT_BUSY(stype: c_int) -> c_ulong {
    if stype == SNDRV_PCM_STREAM_PLAYBACK {
        PSC_AC97STAT_TB
    } else {
        PSC_AC97STAT_RB
    }
}

/* instance data. There can be only one, MacLeod!!!! */
static mut au1xpsc_ac97_workdata: *mut au1xpsc_audio_data = core::ptr::null_mut();

/* #if 0
 *
 * this could theoretically work, but ac97->bus->card->private_data can be NULL
 * when snd_ac97_mixer() is called; I don't know if the rest further down the
 * chain are always valid either.
 *
 * static inline struct au1xpsc_audio_data *ac97_to_pscdata(struct snd_ac97 *x)
 * {
 *	struct snd_soc_card *c = x->bus->card->private_data;
 *	return snd_soc_dai_get_drvdata(c->snd_soc_rtd_to_cpu(rtd, 0));
 * }
 *
 * #else
 */

#[inline]
unsafe fn ac97_to_pscdata(_x: *mut snd_ac97) -> *mut au1xpsc_audio_data {
    au1xpsc_ac97_workdata
}

/* AC97 controller reads codec register */
unsafe extern "C" fn au1xpsc_ac97_read(
    ac97: *mut snd_ac97,
    reg: c_ushort,
) -> c_ushort {
    let pscdata: *mut au1xpsc_audio_data = ac97_to_pscdata(ac97);
    let mut retry: c_ushort;
    let mut tmo: c_ushort;
    let data: c_ulong;

    __raw_writel(PSC_AC97EVNT_CD, AC97_EVNT(pscdata));
    wmb(); /* drain writebuffer */

    retry = AC97_RW_RETRIES;
    loop {
        mutex_lock(core::ptr::addr_of_mut!((*pscdata).lock));

        __raw_writel(
            PSC_AC97CDC_RD | PSC_AC97CDC_INDX(reg),
            AC97_CDC(pscdata),
        );
        wmb(); /* drain writebuffer */

        tmo = 20;
        loop {
            udelay(21);
            if (__raw_readl(AC97_EVNT(pscdata)) & PSC_AC97EVNT_CD) != 0 {
                break;
            }
            tmo = tmo.wrapping_sub(1);
            if tmo == 0 {
                break;
            }
        }

        data = __raw_readl(AC97_CDC(pscdata));

        __raw_writel(PSC_AC97EVNT_CD, AC97_EVNT(pscdata));
        wmb(); /* drain writebuffer */

        mutex_unlock(core::ptr::addr_of_mut!((*pscdata).lock));

        if reg as c_ulong != ((data >> 16) & 0x7f) {
            tmo = 1; /* wrong register, try again */
        }

        retry = retry.wrapping_sub(1);
        if !(retry != 0 && tmo == 0) {
            break;
        }
    }

    if retry != 0 {
        (data & 0xffff) as c_ushort
    } else {
        0xffff
    }
}

/* AC97 controller writes to codec register */
unsafe extern "C" fn au1xpsc_ac97_write(
    ac97: *mut snd_ac97,
    reg: c_ushort,
    val: c_ushort,
) {
    let pscdata: *mut au1xpsc_audio_data = ac97_to_pscdata(ac97);
    let mut tmo: c_uint;
    let mut retry: c_uint;

    __raw_writel(PSC_AC97EVNT_CD, AC97_EVNT(pscdata));
    wmb(); /* drain writebuffer */

    retry = AC97_RW_RETRIES as c_uint;
    loop {
        mutex_lock(core::ptr::addr_of_mut!((*pscdata).lock));

        __raw_writel(
            PSC_AC97CDC_INDX(reg) | ((val & 0xffff) as c_ulong),
            AC97_CDC(pscdata),
        );
        wmb(); /* drain writebuffer */

        tmo = 20;
        loop {
            udelay(21);
            if (__raw_readl(AC97_EVNT(pscdata)) & PSC_AC97EVNT_CD) != 0 {
                break;
            }
            tmo = tmo.wrapping_sub(1);
            if tmo == 0 {
                break;
            }
        }

        __raw_writel(PSC_AC97EVNT_CD, AC97_EVNT(pscdata));
        wmb(); /* drain writebuffer */

        mutex_unlock(core::ptr::addr_of_mut!((*pscdata).lock));

        retry = retry.wrapping_sub(1);
        if !(retry != 0 && tmo == 0) {
            break;
        }
    }
}

/* AC97 controller asserts a warm reset */
unsafe extern "C" fn au1xpsc_ac97_warm_reset(ac97: *mut snd_ac97) {
    let pscdata: *mut au1xpsc_audio_data = ac97_to_pscdata(ac97);

    __raw_writel(PSC_AC97RST_SNC, AC97_RST(pscdata));
    wmb(); /* drain writebuffer */
    msleep(10);
    __raw_writel(0, AC97_RST(pscdata));
    wmb(); /* drain writebuffer */
}

unsafe extern "C" fn au1xpsc_ac97_cold_reset(ac97: *mut snd_ac97) {
    let pscdata: *mut au1xpsc_audio_data = ac97_to_pscdata(ac97);
    let mut i: c_int;

    /* disable PSC during cold reset */
    __raw_writel(0, AC97_CFG(au1xpsc_ac97_workdata));
    wmb(); /* drain writebuffer */
    __raw_writel(PSC_CTRL_DISABLE, PSC_CTRL(pscdata));
    wmb(); /* drain writebuffer */

    /* issue cold reset */
    __raw_writel(PSC_AC97RST_RST, AC97_RST(pscdata));
    wmb(); /* drain writebuffer */
    msleep(500);
    __raw_writel(0, AC97_RST(pscdata));
    wmb(); /* drain writebuffer */

    /* enable PSC */
    __raw_writel(PSC_CTRL_ENABLE, PSC_CTRL(pscdata));
    wmb(); /* drain writebuffer */

    /* wait for PSC to indicate it's ready */
    i = 1000;
    while !((__raw_readl(AC97_STAT(pscdata)) & PSC_AC97STAT_SR) != 0) && {
        i -= 1;
        i != 0
    } {
        msleep(1);
    }

    if i == 0 {
        printk(KERN_ERR, b"au1xpsc-ac97: PSC not ready!\n\0".as_ptr() as *const c_char);
        return;
    }

    /* enable the ac97 function */
    __raw_writel((*pscdata).cfg | PSC_AC97CFG_DE_ENABLE, AC97_CFG(pscdata));
    wmb(); /* drain writebuffer */

    /* wait for AC97 core to become ready */
    i = 1000;
    while !((__raw_readl(AC97_STAT(pscdata)) & PSC_AC97STAT_DR) != 0) && {
        i -= 1;
        i != 0
    } {
        msleep(1);
    }
    if i == 0 {
        printk(KERN_ERR, b"au1xpsc-ac97: AC97 ctrl not ready\n\0".as_ptr() as *const c_char);
    }
}

/* AC97 controller operations */
static mut psc_ac97_ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
    read: Some(au1xpsc_ac97_read),
    write: Some(au1xpsc_ac97_write),
    reset: Some(au1xpsc_ac97_cold_reset),
    warm_reset: Some(au1xpsc_ac97_warm_reset),
};

unsafe extern "C" fn au1xpsc_ac97_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pscdata: *mut au1xpsc_audio_data = snd_soc_dai_get_drvdata(dai);
    let mut r: c_ulong;
    let ro: c_ulong;
    let stat: c_ulong;
    let mut t: c_int;
    let stype: c_int = (*substream).stream;

    r = __raw_readl(AC97_CFG(pscdata));
    ro = r;
    stat = __raw_readl(AC97_STAT(pscdata));

    /* already active? */
    if (stat & (PSC_AC97STAT_TB | PSC_AC97STAT_RB)) != 0 {
        /* reject parameters not currently set up */
        if PSC_AC97CFG_GET_LEN(r) != (*params).msbits as c_ulong
            || (*pscdata).rate != params_rate(params)
        {
            return -EINVAL;
        }
    } else {
        /* set sample bitdepth: REG[24:21]=(BITS-2)/2 */
        r &= !PSC_AC97CFG_LEN_MASK;
        r |= PSC_AC97CFG_SET_LEN((*params).msbits);

        /* channels: enable slots for front L/R channel */
        if stype == SNDRV_PCM_STREAM_PLAYBACK {
            r &= !PSC_AC97CFG_TXSLOT_MASK;
            r |= PSC_AC97CFG_TXSLOT_ENA(3);
            r |= PSC_AC97CFG_TXSLOT_ENA(4);
        } else {
            r &= !PSC_AC97CFG_RXSLOT_MASK;
            r |= PSC_AC97CFG_RXSLOT_ENA(3);
            r |= PSC_AC97CFG_RXSLOT_ENA(4);
        }

        /* do we need to poke the hardware? */
        if (r ^ ro) == 0 {
            return 0;
        }

        /* ac97 engine is about to be disabled */
        mutex_lock(core::ptr::addr_of_mut!((*pscdata).lock));

        /* disable AC97 device controller first... */
        __raw_writel(r & !PSC_AC97CFG_DE_ENABLE, AC97_CFG(pscdata));
        wmb(); /* drain writebuffer */

        /* ...wait for it... */
        t = 100;
        while (__raw_readl(AC97_STAT(pscdata)) & PSC_AC97STAT_DR) != 0 && {
            t -= 1;
            t != 0
        } {
            msleep(1);
        }

        if t == 0 {
            printk(KERN_ERR, b"PSC-AC97: can't disable!\n\0".as_ptr() as *const c_char);
        }

        /* ...write config... */
        __raw_writel(r, AC97_CFG(pscdata));
        wmb(); /* drain writebuffer */

        /* ...enable the AC97 controller again... */
        __raw_writel(r | PSC_AC97CFG_DE_ENABLE, AC97_CFG(pscdata));
        wmb(); /* drain writebuffer */

        /* ...and wait for ready bit */
        t = 100;
        while !((__raw_readl(AC97_STAT(pscdata)) & PSC_AC97STAT_DR) != 0) && {
            t -= 1;
            t != 0
        } {
            msleep(1);
        }

        if t == 0 {
            printk(KERN_ERR, b"PSC-AC97: can't enable!\n\0".as_ptr() as *const c_char);
        }

        mutex_unlock(core::ptr::addr_of_mut!((*pscdata).lock));

        (*pscdata).cfg = r;
        (*pscdata).rate = params_rate(params);
    }

    0
}

unsafe extern "C" fn au1xpsc_ac97_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pscdata: *mut au1xpsc_audio_data = snd_soc_dai_get_drvdata(dai);
    let mut ret: c_int;
    let stype: c_int = (*substream).stream;

    ret = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            __raw_writel(AC97PCR_CLRFIFO(stype), AC97_PCR(pscdata));
            wmb(); /* drain writebuffer */
            __raw_writel(AC97PCR_START(stype), AC97_PCR(pscdata));
            wmb(); /* drain writebuffer */
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            __raw_writel(AC97PCR_STOP(stype), AC97_PCR(pscdata));
            wmb(); /* drain writebuffer */

            while (__raw_readl(AC97_STAT(pscdata)) & AC97STAT_BUSY(stype)) != 0 {
                core::arch::asm!("nop");
            }

            __raw_writel(AC97PCR_CLRFIFO(stype), AC97_PCR(pscdata));
            wmb(); /* drain writebuffer */
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

unsafe extern "C" fn au1xpsc_ac97_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pscdata: *mut au1xpsc_audio_data = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_set_dma_data(
        dai,
        substream,
        core::ptr::addr_of_mut!((*pscdata).dmaids[0]) as *mut c_void,
    );
    0
}

unsafe extern "C" fn au1xpsc_ac97_probe(_dai: *mut snd_soc_dai) -> c_int {
    if !au1xpsc_ac97_workdata.is_null() {
        0
    } else {
        -ENODEV
    }
}

static au1xpsc_ac97_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(au1xpsc_ac97_probe),
    startup: Some(au1xpsc_ac97_startup),
    trigger: Some(au1xpsc_ac97_trigger),
    hw_params: Some(au1xpsc_ac97_hw_params),
};

static au1xpsc_ac97_dai_template: snd_soc_dai_driver = snd_soc_dai_driver {
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
    ops: core::ptr::addr_of!(au1xpsc_ac97_dai_ops),
    ..unsafe { core::mem::zeroed() }
};

static au1xpsc_ac97_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"au1xpsc-ac97\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn au1xpsc_ac97_drvprobe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mut dmares: *mut resource;
    let sel: c_ulong;
    let wd: *mut au1xpsc_audio_data;

    wd = devm_kzalloc(
        core::ptr::addr_of_mut!((*pdev).dev),
        core::mem::size_of::<au1xpsc_audio_data>(),
        GFP_KERNEL,
    ) as *mut au1xpsc_audio_data;
    if wd.is_null() {
        return -ENOMEM;
    }

    mutex_init(core::ptr::addr_of_mut!((*wd).lock));

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

    /* configuration: max dma trigger threshold, enable ac97 */
    (*wd).cfg = PSC_AC97CFG_RT_FIFO8 | PSC_AC97CFG_TT_FIFO8 | PSC_AC97CFG_DE_ENABLE;

    /* preserve PSC clock source set up by platform	 */
    sel = __raw_readl(PSC_SEL(wd)) & PSC_SEL_CLK_MASK;
    __raw_writel(PSC_CTRL_DISABLE, PSC_CTRL(wd));
    wmb(); /* drain writebuffer */
    __raw_writel(0, PSC_SEL(wd));
    wmb(); /* drain writebuffer */
    __raw_writel(PSC_SEL_PS_AC97MODE | sel, PSC_SEL(wd));
    wmb(); /* drain writebuffer */

    /* name the DAI like this device instance ("au1xpsc-ac97.PSCINDEX") */
    memcpy(
        core::ptr::addr_of_mut!((*wd).dai_drv) as *mut c_void,
        core::ptr::addr_of!(au1xpsc_ac97_dai_template) as *const c_void,
        core::mem::size_of::<snd_soc_dai_driver>(),
    );
    (*wd).dai_drv.name = dev_name(core::ptr::addr_of_mut!((*pdev).dev));

    platform_set_drvdata(pdev, wd as *mut c_void);

    ret = snd_soc_set_ac97_ops(core::ptr::addr_of_mut!(psc_ac97_ops));
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_register_component(
        core::ptr::addr_of_mut!((*pdev).dev),
        core::ptr::addr_of!(au1xpsc_ac97_component),
        core::ptr::addr_of_mut!((*wd).dai_drv),
        1,
    );
    if ret != 0 {
        return ret;
    }

    au1xpsc_ac97_workdata = wd;
    0
}

unsafe extern "C" fn au1xpsc_ac97_drvremove(pdev: *mut platform_device) {
    let wd: *mut au1xpsc_audio_data = platform_get_drvdata(pdev);

    snd_soc_unregister_component(core::ptr::addr_of_mut!((*pdev).dev));

    /* disable PSC completely */
    __raw_writel(0, AC97_CFG(wd));
    wmb(); /* drain writebuffer */
    __raw_writel(PSC_CTRL_DISABLE, PSC_CTRL(wd));
    wmb(); /* drain writebuffer */

    au1xpsc_ac97_workdata = core::ptr::null_mut(); /* MDEV */
}

unsafe extern "C" fn au1xpsc_ac97_drvsuspend(dev: *mut device) -> c_int {
    let wd: *mut au1xpsc_audio_data = dev_get_drvdata(dev);

    /* save interesting registers and disable PSC */
    (*wd).pm[0] = __raw_readl(PSC_SEL(wd));

    __raw_writel(0, AC97_CFG(wd));
    wmb(); /* drain writebuffer */
    __raw_writel(PSC_CTRL_DISABLE, PSC_CTRL(wd));
    wmb(); /* drain writebuffer */

    0
}

unsafe extern "C" fn au1xpsc_ac97_drvresume(dev: *mut device) -> c_int {
    let wd: *mut au1xpsc_audio_data = dev_get_drvdata(dev);

    /* restore PSC clock config */
    __raw_writel((*wd).pm[0] | PSC_SEL_PS_AC97MODE, PSC_SEL(wd));
    wmb(); /* drain writebuffer */

    /* after this point the ac97 core will cold-reset the codec.
     * During cold-reset the PSC is reinitialized and the last
     * configuration set up in hw_params() is restored.
     */
    0
}

static au1xpscac97_pmops: dev_pm_ops =
    DEFINE_SIMPLE_DEV_PM_OPS(au1xpsc_ac97_drvsuspend, au1xpsc_ac97_drvresume);

static mut au1xpsc_ac97_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"au1xpsc_ac97\0".as_ptr() as *const c_char,
        pm: pm_ptr(core::ptr::addr_of!(au1xpscac97_pmops)),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(au1xpsc_ac97_drvprobe),
    remove: Some(au1xpsc_ac97_drvremove),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(au1xpsc_ac97_driver);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("Au12x0/Au1550 PSC AC97 ALSA ASoC audio driver");
MODULE_AUTHOR!("Manuel Lauss");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
