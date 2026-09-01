// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/sound/soc/ep93xx-i2s.c
 * EP93xx I2S driver
 *
 * Copyright (C) 2010 Ryan Mallon
 *
 * Based on the original driver by:
 *   Copyright (C) 2007 Chase Douglas <chasedouglas@gmail>
 *   Copyright (C) 2006 Lennert Buytenhek <buytenh@wantstofly.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

const EP93XX_I2S_TXCLKCFG: c_uint = 0x00;
const EP93XX_I2S_RXCLKCFG: c_uint = 0x04;
const EP93XX_I2S_GLSTS: c_uint = 0x08;
const EP93XX_I2S_GLCTRL: c_uint = 0x0C;

const EP93XX_I2S_I2STX0LFT: c_uint = 0x10;
const EP93XX_I2S_I2STX0RT: c_uint = 0x14;

const EP93XX_I2S_TXLINCTRLDATA: c_uint = 0x28;
const EP93XX_I2S_TXCTRL: c_uint = 0x2C;
const EP93XX_I2S_TXWRDLEN: c_uint = 0x30;
const EP93XX_I2S_TX0EN: c_uint = 0x34;

const EP93XX_I2S_RXLINCTRLDATA: c_uint = 0x58;
const EP93XX_I2S_RXCTRL: c_uint = 0x5C;
const EP93XX_I2S_RXWRDLEN: c_uint = 0x60;
const EP93XX_I2S_RX0EN: c_uint = 0x64;

const EP93XX_I2S_WRDLEN_16: c_uint = 0 << 0;
const EP93XX_I2S_WRDLEN_24: c_uint = 1 << 0;
const EP93XX_I2S_WRDLEN_32: c_uint = 2 << 0;

const EP93XX_I2S_RXLINCTRLDATA_R_JUST: c_uint = BIT(1); /* Right justify */

const EP93XX_I2S_TXLINCTRLDATA_R_JUST: c_uint = BIT(2); /* Right justify */

/*
 * Transmit empty interrupt level select:
 * 0 - Generate interrupt when FIFO is half empty
 * 1 - Generate interrupt when FIFO is empty
 */
const EP93XX_I2S_TXCTRL_TXEMPTY_LVL: c_uint = BIT(0);
const EP93XX_I2S_TXCTRL_TXUFIE: c_uint = BIT(1); /* Transmit interrupt enable */

const EP93XX_I2S_CLKCFG_LRS: c_uint = 1 << 0; /* lrclk polarity */
const EP93XX_I2S_CLKCFG_CKP: c_uint = 1 << 1; /* Bit clock polarity */
const EP93XX_I2S_CLKCFG_REL: c_uint = 1 << 2; /* First bit transition */
const EP93XX_I2S_CLKCFG_MASTER: c_uint = 1 << 3; /* Master mode */
const EP93XX_I2S_CLKCFG_NBCG: c_uint = 1 << 4; /* Not bit clock gating */

const EP93XX_I2S_GLSTS_TX0_FIFO_FULL: c_uint = BIT(12);

#[repr(C)]
struct ep93xx_i2s_info {
    mclk: *mut clk,
    sclk: *mut clk,
    lrclk: *mut clk,
    regs: *mut c_void,
    dma_params_rx: snd_dmaengine_dai_dma_data,
    dma_params_tx: snd_dmaengine_dai_dma_data,
}

#[inline]
unsafe fn ep93xx_i2s_write_reg(info: *mut ep93xx_i2s_info, reg: c_uint, val: c_uint) {
    __raw_writel(val, ((*info).regs as *mut u8).add(reg as usize) as *mut c_void);
}

#[inline]
unsafe fn ep93xx_i2s_read_reg(info: *mut ep93xx_i2s_info, reg: c_uint) -> c_uint {
    __raw_readl(((*info).regs as *mut u8).add(reg as usize) as *const c_void)
}

unsafe fn ep93xx_i2s_enable(info: *mut ep93xx_i2s_info, stream: c_int) -> c_int {
    let base_reg: c_uint;
    let mut err: c_int;

    if (ep93xx_i2s_read_reg(info, EP93XX_I2S_TX0EN) & 0x1) == 0
        && (ep93xx_i2s_read_reg(info, EP93XX_I2S_RX0EN) & 0x1) == 0
    {
        /* Enable clocks */
        err = clk_prepare_enable((*info).mclk);
        if err != 0 {
            return err;
        }
        err = clk_prepare_enable((*info).sclk);
        if err != 0 {
            clk_disable_unprepare((*info).mclk);
            return err;
        }
        err = clk_prepare_enable((*info).lrclk);
        if err != 0 {
            clk_disable_unprepare((*info).sclk);
            clk_disable_unprepare((*info).mclk);
            return err;
        }

        /* Enable i2s */
        ep93xx_i2s_write_reg(info, EP93XX_I2S_GLCTRL, 1);
    }

    /* Enable fifo */
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        base_reg = EP93XX_I2S_TX0EN;
    } else {
        base_reg = EP93XX_I2S_RX0EN;
    }
    ep93xx_i2s_write_reg(info, base_reg, 1);

    /* Enable TX IRQs (FIFO empty or underflow) */
    if IS_ENABLED_CONFIG_SND_EP93XX_SOC_I2S_WATCHDOG && stream == SNDRV_PCM_STREAM_PLAYBACK {
        ep93xx_i2s_write_reg(
            info,
            EP93XX_I2S_TXCTRL,
            EP93XX_I2S_TXCTRL_TXEMPTY_LVL | EP93XX_I2S_TXCTRL_TXUFIE,
        );
    }

    0
}

unsafe fn ep93xx_i2s_disable(info: *mut ep93xx_i2s_info, stream: c_int) {
    let base_reg: c_uint;

    /* Disable IRQs */
    if IS_ENABLED_CONFIG_SND_EP93XX_SOC_I2S_WATCHDOG && stream == SNDRV_PCM_STREAM_PLAYBACK {
        ep93xx_i2s_write_reg(info, EP93XX_I2S_TXCTRL, 0);
    }

    /* Disable fifo */
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        base_reg = EP93XX_I2S_TX0EN;
    } else {
        base_reg = EP93XX_I2S_RX0EN;
    }
    ep93xx_i2s_write_reg(info, base_reg, 0);

    if (ep93xx_i2s_read_reg(info, EP93XX_I2S_TX0EN) & 0x1) == 0
        && (ep93xx_i2s_read_reg(info, EP93XX_I2S_RX0EN) & 0x1) == 0
    {
        /* Disable i2s */
        ep93xx_i2s_write_reg(info, EP93XX_I2S_GLCTRL, 0);

        /* Disable clocks */
        clk_disable_unprepare((*info).lrclk);
        clk_disable_unprepare((*info).sclk);
        clk_disable_unprepare((*info).mclk);
    }
}

/*
 * According to documentation I2S controller can handle underflow conditions
 * just fine, but in reality the state machine is sometimes confused so that
 * the whole stream is shifted by one byte. The watchdog below disables the TX
 * FIFO, fills the buffer with zeroes and re-enables the FIFO. State machine
 * is being reset and by filling the buffer we get some time before next
 * underflow happens.
 */
unsafe extern "C" fn ep93xx_i2s_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let info = dev_id as *mut ep93xx_i2s_info;

    /* Disable FIFO */
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TX0EN, 0);
    /*
     * Fill TX FIFO with zeroes, this way we can defer next IRQs as much as
     * possible and get more time for DMA to catch up. Actually there are
     * only 8 samples in this FIFO, so even on 8kHz maximum deferral here is
     * 1ms.
     */
    while (ep93xx_i2s_read_reg(info, EP93XX_I2S_GLSTS) & EP93XX_I2S_GLSTS_TX0_FIFO_FULL) == 0 {
        ep93xx_i2s_write_reg(info, EP93XX_I2S_I2STX0LFT, 0);
        ep93xx_i2s_write_reg(info, EP93XX_I2S_I2STX0RT, 0);
    }
    /* Re-enable FIFO */
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TX0EN, 1);

    IRQ_HANDLED
}

unsafe extern "C" fn ep93xx_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let info = snd_soc_dai_get_drvdata(dai) as *mut ep93xx_i2s_info;

    snd_soc_dai_init_dma_data(dai, &mut (*info).dma_params_tx, &mut (*info).dma_params_rx);

    0
}

unsafe extern "C" fn ep93xx_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let info = snd_soc_dai_get_drvdata(dai) as *mut ep93xx_i2s_info;

    ep93xx_i2s_enable(info, (*substream).stream)
}

unsafe extern "C" fn ep93xx_i2s_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let info = snd_soc_dai_get_drvdata(dai) as *mut ep93xx_i2s_info;

    ep93xx_i2s_disable(info, (*substream).stream);
}

unsafe extern "C" fn ep93xx_i2s_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let info = snd_soc_dai_get_drvdata(cpu_dai) as *mut ep93xx_i2s_info;
    let mut clk_cfg: c_uint;
    let mut txlin_ctrl: c_uint = 0;
    let mut rxlin_ctrl: c_uint = 0;

    clk_cfg = ep93xx_i2s_read_reg(info, EP93XX_I2S_RXCLKCFG);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            clk_cfg |= EP93XX_I2S_CLKCFG_REL;
        }

        SND_SOC_DAIFMT_LEFT_J => {
            clk_cfg &= !EP93XX_I2S_CLKCFG_REL;
        }

        SND_SOC_DAIFMT_RIGHT_J => {
            clk_cfg &= !EP93XX_I2S_CLKCFG_REL;
            rxlin_ctrl |= EP93XX_I2S_RXLINCTRLDATA_R_JUST;
            txlin_ctrl |= EP93XX_I2S_TXLINCTRLDATA_R_JUST;
        }

        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            /* CPU is provider */
            clk_cfg |= EP93XX_I2S_CLKCFG_MASTER;
        }

        SND_SOC_DAIFMT_BC_FC => {
            /* Codec is provider */
            clk_cfg &= !EP93XX_I2S_CLKCFG_MASTER;
        }

        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            /* Negative bit clock, lrclk low on left word */
            clk_cfg &= !(EP93XX_I2S_CLKCFG_CKP | EP93XX_I2S_CLKCFG_LRS);
        }

        SND_SOC_DAIFMT_NB_IF => {
            /* Negative bit clock, lrclk low on right word */
            clk_cfg &= !EP93XX_I2S_CLKCFG_CKP;
            clk_cfg |= EP93XX_I2S_CLKCFG_LRS;
        }

        SND_SOC_DAIFMT_IB_NF => {
            /* Positive bit clock, lrclk low on left word */
            clk_cfg |= EP93XX_I2S_CLKCFG_CKP;
            clk_cfg &= !EP93XX_I2S_CLKCFG_LRS;
        }

        SND_SOC_DAIFMT_IB_IF => {
            /* Positive bit clock, lrclk low on right word */
            clk_cfg |= EP93XX_I2S_CLKCFG_CKP | EP93XX_I2S_CLKCFG_LRS;
        }

        _ => {}
    }

    /* Write new register values */
    ep93xx_i2s_write_reg(info, EP93XX_I2S_RXCLKCFG, clk_cfg);
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TXCLKCFG, clk_cfg);
    ep93xx_i2s_write_reg(info, EP93XX_I2S_RXLINCTRLDATA, rxlin_ctrl);
    ep93xx_i2s_write_reg(info, EP93XX_I2S_TXLINCTRLDATA, txlin_ctrl);
    0
}

unsafe extern "C" fn ep93xx_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let info = snd_soc_dai_get_drvdata(dai) as *mut ep93xx_i2s_info;
    let word_len: c_uint;
    let div: c_uint;
    let mut sdiv: c_uint;
    let lrdiv: c_uint;
    let mut err: c_int;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            word_len = EP93XX_I2S_WRDLEN_16;
        }

        SNDRV_PCM_FORMAT_S24_LE => {
            word_len = EP93XX_I2S_WRDLEN_24;
        }

        SNDRV_PCM_FORMAT_S32_LE => {
            word_len = EP93XX_I2S_WRDLEN_32;
        }

        _ => {
            return -EINVAL;
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ep93xx_i2s_write_reg(info, EP93XX_I2S_TXWRDLEN, word_len);
    } else {
        ep93xx_i2s_write_reg(info, EP93XX_I2S_RXWRDLEN, word_len);
    }

    /*
     * EP93xx I2S module can be setup so SCLK / LRCLK value can be
     * 32, 64, 128. MCLK / SCLK value can be 2 and 4.
     * We set LRCLK equal to `rate' and minimum SCLK / LRCLK
     * value is 64, because our sample size is 32 bit * 2 channels.
     * I2S standard permits us to transmit more bits than
     * the codec uses.
     */
    div = (clk_get_rate((*info).mclk) / params_rate(params) as c_ulonglong) as c_uint;
    sdiv = 4;
    if div > (256 + 512) / 2 {
        lrdiv = 128;
    } else {
        lrdiv = 64;
        if div < (128 + 256) / 2 {
            sdiv = 2;
        }
    }

    err = clk_set_rate((*info).sclk, clk_get_rate((*info).mclk) / sdiv as c_ulonglong);
    if err != 0 {
        return err;
    }

    err = clk_set_rate((*info).lrclk, clk_get_rate((*info).sclk) / lrdiv as c_ulonglong);
    if err != 0 {
        return err;
    }

    0
}

unsafe extern "C" fn ep93xx_i2s_set_sysclk(
    cpu_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let info = snd_soc_dai_get_drvdata(cpu_dai) as *mut ep93xx_i2s_info;

    if dir == SND_SOC_CLOCK_IN || clk_id != 0 {
        return -EINVAL;
    }
    if freq == 0 {
        return 0;
    }

    clk_set_rate((*info).mclk, freq as c_ulonglong)
}

/* CONFIG_PM */
unsafe extern "C" fn ep93xx_i2s_suspend(component: *mut snd_soc_component) -> c_int {
    let info = snd_soc_component_get_drvdata(component) as *mut ep93xx_i2s_info;

    if snd_soc_component_active(component) == 0 {
        return 0;
    }

    ep93xx_i2s_disable(info, SNDRV_PCM_STREAM_PLAYBACK);
    ep93xx_i2s_disable(info, SNDRV_PCM_STREAM_CAPTURE);

    0
}

/* CONFIG_PM */
unsafe extern "C" fn ep93xx_i2s_resume(component: *mut snd_soc_component) -> c_int {
    let info = snd_soc_component_get_drvdata(component) as *mut ep93xx_i2s_info;
    let mut err: c_int;

    if snd_soc_component_active(component) == 0 {
        return 0;
    }

    err = ep93xx_i2s_enable(info, SNDRV_PCM_STREAM_PLAYBACK);
    if err != 0 {
        return err;
    }

    ep93xx_i2s_enable(info, SNDRV_PCM_STREAM_CAPTURE)
}

/* !CONFIG_PM maps ep93xx_i2s_suspend and ep93xx_i2s_resume to NULL in C. */

static mut ep93xx_selectable_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S as u64
    | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J as u64
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J as u64
    | SND_SOC_POSSIBLE_DAIFMT_NB_NF as u64
    | SND_SOC_POSSIBLE_DAIFMT_NB_IF as u64
    | SND_SOC_POSSIBLE_DAIFMT_IB_NF as u64
    | SND_SOC_POSSIBLE_DAIFMT_IB_IF as u64;

static mut ep93xx_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(ep93xx_i2s_dai_probe),
    startup: Some(ep93xx_i2s_startup),
    shutdown: Some(ep93xx_i2s_shutdown),
    hw_params: Some(ep93xx_i2s_hw_params),
    set_sysclk: Some(ep93xx_i2s_set_sysclk),
    set_fmt: Some(ep93xx_i2s_set_dai_fmt),
    auto_selectable_formats: unsafe { &ep93xx_selectable_formats as *const u64 },
    num_auto_selectable_formats: 1,
};

const EP93XX_I2S_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S32_LE;

static mut ep93xx_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    symmetric_rate: 1,
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: EP93XX_I2S_FORMATS as u64,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: EP93XX_I2S_FORMATS as u64,
    },
    ops: unsafe { &ep93xx_i2s_dai_ops as *const snd_soc_dai_ops },
};

static mut ep93xx_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"ep93xx-i2s\0".as_ptr() as *const c_char,
    suspend: Some(ep93xx_i2s_suspend),
    resume: Some(ep93xx_i2s_resume),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn ep93xx_i2s_probe(pdev: *mut platform_device) -> c_int {
    let mut info: *mut ep93xx_i2s_info;
    let mut err: c_int;

    info = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<ep93xx_i2s_info>(),
        GFP_KERNEL,
    ) as *mut ep93xx_i2s_info;
    if info.is_null() {
        return -ENOMEM;
    }

    (*info).regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*info).regs) {
        return PTR_ERR((*info).regs) as c_int;
    }

    if IS_ENABLED_CONFIG_SND_EP93XX_SOC_I2S_WATCHDOG {
        let irq: c_int = platform_get_irq(pdev, 0);
        if irq <= 0 {
            return if irq < 0 { irq } else { -ENODEV };
        }

        err = devm_request_irq(
            &mut (*pdev).dev,
            irq,
            Some(ep93xx_i2s_interrupt),
            0,
            (*pdev).name,
            info as *mut c_void,
        );
        if err != 0 {
            return err;
        }
    }

    (*info).mclk = clk_get(&mut (*pdev).dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*info).mclk as *const c_void) {
        err = PTR_ERR((*info).mclk as *const c_void) as c_int;
        goto_fail_put_mclk(info, err)
    } else {
        (*info).sclk = clk_get(&mut (*pdev).dev, b"sclk\0".as_ptr() as *const c_char);
        if IS_ERR((*info).sclk as *const c_void) {
            err = PTR_ERR((*info).sclk as *const c_void) as c_int;
            goto_fail_put_sclk(info, err)
        } else {
            (*info).lrclk = clk_get(&mut (*pdev).dev, b"lrclk\0".as_ptr() as *const c_char);
            if IS_ERR((*info).lrclk as *const c_void) {
                err = PTR_ERR((*info).lrclk as *const c_void) as c_int;
                goto_fail_put_lrclk(info, err)
            } else {
                dev_set_drvdata(&mut (*pdev).dev, info as *mut c_void);

                err = devm_snd_soc_register_component(
                    &mut (*pdev).dev,
                    &ep93xx_i2s_component,
                    &mut ep93xx_i2s_dai,
                    1,
                );
                if err != 0 {
                    goto_fail(info, err)
                } else {
                    err = devm_ep93xx_pcm_platform_register(&mut (*pdev).dev);
                    if err != 0 {
                        goto_fail(info, err)
                    } else {
                        0
                    }
                }
            }
        }
    }
}

unsafe fn goto_fail(info: *mut ep93xx_i2s_info, err: c_int) -> c_int {
    clk_put((*info).lrclk);
    clk_put((*info).sclk);
    clk_put((*info).mclk);
    err
}

unsafe fn goto_fail_put_lrclk(info: *mut ep93xx_i2s_info, err: c_int) -> c_int {
    clk_put((*info).sclk);
    clk_put((*info).mclk);
    err
}

unsafe fn goto_fail_put_sclk(info: *mut ep93xx_i2s_info, err: c_int) -> c_int {
    clk_put((*info).mclk);
    err
}

unsafe fn goto_fail_put_mclk(_info: *mut ep93xx_i2s_info, err: c_int) -> c_int {
    err
}

unsafe extern "C" fn ep93xx_i2s_remove(pdev: *mut platform_device) {
    let info = dev_get_drvdata(&mut (*pdev).dev) as *mut ep93xx_i2s_info;

    clk_put((*info).lrclk);
    clk_put((*info).sclk);
    clk_put((*info).mclk);
}

static mut ep93xx_i2s_of_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"cirrus,ep9301-i2s\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, ep93xx_i2s_of_ids); */

static mut ep93xx_i2s_driver: platform_driver = platform_driver {
    probe: Some(ep93xx_i2s_probe),
    remove: Some(ep93xx_i2s_remove),
    driver: device_driver {
        name: b"ep93xx-i2s\0".as_ptr() as *const c_char,
        of_match_table: unsafe { ep93xx_i2s_of_ids.as_ptr() },
    },
};

/* module_platform_driver(ep93xx_i2s_driver); */

/* MODULE_ALIAS("platform:ep93xx-i2s"); */
/* MODULE_AUTHOR("Ryan Mallon"); */
/* MODULE_DESCRIPTION("EP93XX I2S driver"); */
/* MODULE_LICENSE("GPL"); */

extern "C" {
    fn __raw_writel(val: c_uint, addr: *mut c_void);
    fn __raw_readl(addr: *const c_void) -> c_uint;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_ulonglong;
    fn clk_set_rate(clk: *mut clk, rate: c_ulonglong) -> c_int;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_active(component: *mut snd_soc_component) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_ulonglong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_ep93xx_pcm_platform_register(dev: *mut device) -> c_int;
}

extern "Rust" {
    type clk;
    type snd_dmaengine_dai_dma_data;
    type snd_soc_dai;
    type snd_pcm_hw_params;
    type snd_soc_component;
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    auto_selectable_formats: *const u64,
    num_auto_selectable_formats: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    symmetric_rate: c_uint,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
    name: *const c_char,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: device_driver,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

type irqreturn_t = c_uint;

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const IRQ_HANDLED: irqreturn_t = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x100;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x400;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0010;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0020;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0030;
const SNDRV_PCM_FORMAT_S16_LE: c_uint = 2;
const SNDRV_PCM_FORMAT_S24_LE: c_uint = 6;
const SNDRV_PCM_FORMAT_S32_LE: c_uint = 10;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_POSSIBLE_DAIFMT_I2S: c_ulonglong = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: c_ulonglong = 1 << 1;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: c_ulonglong = 1 << 2;
const SND_SOC_POSSIBLE_DAIFMT_NB_NF: c_ulonglong = 1 << 3;
const SND_SOC_POSSIBLE_DAIFMT_NB_IF: c_ulonglong = 1 << 4;
const SND_SOC_POSSIBLE_DAIFMT_IB_NF: c_ulonglong = 1 << 5;
const SND_SOC_POSSIBLE_DAIFMT_IB_IF: c_ulonglong = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << SNDRV_PCM_FORMAT_S32_LE;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

/* IS_ENABLED(CONFIG_SND_EP93XX_SOC_I2S_WATCHDOG) build-time condition. */
const IS_ENABLED_CONFIG_SND_EP93XX_SOC_I2S_WATCHDOG: bool = false;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
