// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 Freescale Semiconductor, Inc.
 */

/*
 * Translated from the isolated C implementation source. Linux kernel and local
 * header dependencies are declared as external symbols/types and are expected
 * to be supplied by the surrounding repository.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const MXS_SET_ADDR: usize = 0x4;
const MXS_CLR_ADDR: usize = 0x8;

const MXS_SAIF_BUSY_TIMEOUT_US: c_uint = 10000;

const EBUSY: c_int = 16;
const EEXIST: c_int = 17;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const GFP_KERNEL: c_uint = 0;
const USEC_PER_SEC: c_uint = 1000000;

const MXS_SAIF_MCLK: c_int = 0;
const MXS_SAIF_STATE_RUNNING: c_int = 1;
const MXS_SAIF_STATE_STOPPED: c_int = 0;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S20_3LE: c_int = 4;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0010;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0020;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0030;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x1000;

const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S20_3LE;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << SNDRV_PCM_FORMAT_S24_LE;

const MXS_SAIF_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const MXS_SAIF_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

extern "C" {
    static SAIF_CTRL: usize;
    static SAIF_STAT: usize;
    static SAIF_DATA: usize;
    static BM_SAIF_STAT_BUSY: u32;
    static BM_SAIF_CTRL_CLKGATE: u32;
    static BM_SAIF_CTRL_RUN: u32;
    static BM_SAIF_CTRL_SFTRST: u32;
    static BM_SAIF_CTRL_BITCLK_MULT_RATE: u32;
    static BM_SAIF_CTRL_BITCLK_BASE_RATE: u32;
    static BM_SAIF_CTRL_BITCLK_EDGE: u32;
    static BM_SAIF_CTRL_LRCLK_POLARITY: u32;
    static BM_SAIF_CTRL_JUSTIFY: u32;
    static BM_SAIF_CTRL_DELAY: u32;
    static BM_SAIF_CTRL_SLAVE_MODE: u32;
    static BM_SAIF_CTRL_WORD_LENGTH: u32;
    static BM_SAIF_CTRL_BITCLK_48XFS_ENABLE: u32;
    static BM_SAIF_CTRL_READ_MODE: u32;
    static BM_SAIF_CTRL_FIFO_ERROR_IRQ_EN: u32;
    static BM_SAIF_STAT_FIFO_UNDERFLOW_IRQ: u32;
    static BM_SAIF_STAT_FIFO_OVERFLOW_IRQ: u32;
    static BP_SAIF_CTRL_BITCLK_MULT_RATE: c_uint;
    static BP_SAIF_CTRL_WORD_LENGTH: c_uint;
}

type irqreturn_t = c_uint;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
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
pub struct mxs_saif {
    pub base: *mut c_void,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub id: c_int,
    pub master_id: c_int,
    pub mclk: c_uint,
    pub mclk_in_use: c_int,
    pub cur_rate: c_uint,
    pub ongoing: c_int,
    pub state: c_int,
    pub fifo_underrun: c_uint,
    pub fifo_overrun: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int,
    >,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub prepare:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
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
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_inner,
}

extern "C" {
    fn __raw_readl(addr: *mut c_void) -> u32;
    fn __raw_writel(val: u32, addr: *mut c_void);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare(clk: *mut clk) -> c_int;
    fn clk_unprepare(clk: *mut clk);
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn clk_register_divider(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_uint,
        reg: *mut c_void,
        shift: c_uint,
        width: c_uint,
        clk_divider_flags: c_uint,
        lock: *mut c_void,
    ) -> *mut clk;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn of_clk_add_provider(
        np: *mut device_node,
        clk_src_get: *mut c_void,
        data: *mut clk,
    ) -> c_int;
    static mut of_clk_src_simple_get: c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_alias_get_id(np: *mut device_node, stem: *const c_char) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn mxs_pcm_platform_register(dev: *mut device) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn udelay(usecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

static mut mxs_saif: [*mut mxs_saif; 2] = [ptr::null_mut(); 2];

unsafe fn addr(base: *mut c_void, off: usize) -> *mut c_void {
    (base as *mut u8).add(off) as *mut c_void
}

unsafe fn BF_SAIF_CTRL_BITCLK_MULT_RATE(v: u32) -> u32 {
    v << BP_SAIF_CTRL_BITCLK_MULT_RATE
}

unsafe fn BF_SAIF_CTRL_WORD_LENGTH(v: u32) -> u32 {
    v << BP_SAIF_CTRL_WORD_LENGTH
}

/*
 * Since SAIF may work on EXTMASTER mode, IOW, it's working BITCLK&LRCLK
 * is provided by other SAIF, we provide a interface here to get its master
 * from its master_id.
 * Note that the master could be itself.
 */
unsafe fn mxs_saif_get_master(saif: *mut mxs_saif) -> *mut mxs_saif {
    mxs_saif[(*saif).master_id as usize]
}

unsafe fn __mxs_saif_put_mclk(saif: *mut mxs_saif) -> c_int {
    let mut stat: u32;
    let mut ret: c_int = 0;
    let mut elapsed: c_uint = 0;

    loop {
        stat = __raw_readl(addr((*saif).base, SAIF_STAT));
        if (stat & BM_SAIF_STAT_BUSY) == 0 {
            break;
        }
        if elapsed >= USEC_PER_SEC {
            ret = -EBUSY;
            break;
        }
        udelay(MXS_SAIF_BUSY_TIMEOUT_US);
        elapsed = elapsed.wrapping_add(MXS_SAIF_BUSY_TIMEOUT_US);
    }
    if ret != 0 {
        dev_err((*saif).dev, c"error: busy\n".as_ptr());
        return -EBUSY;
    }

    /* disable MCLK output */
    __raw_writel(
        BM_SAIF_CTRL_CLKGATE,
        addr((*saif).base, SAIF_CTRL + MXS_SET_ADDR),
    );
    __raw_writel(
        BM_SAIF_CTRL_RUN,
        addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
    );

    (*saif).mclk_in_use = 0;

    0
}

unsafe fn __mxs_saif_get_mclk(saif: *mut mxs_saif) -> c_int {
    let stat: u32;
    let master_saif: *mut mxs_saif;

    if saif.is_null() {
        return -EINVAL;
    }

    /* Clear Reset */
    __raw_writel(
        BM_SAIF_CTRL_SFTRST,
        addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
    );

    /* FIXME: need clear clk gate for register r/w */
    __raw_writel(
        BM_SAIF_CTRL_CLKGATE,
        addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
    );

    master_saif = mxs_saif_get_master(saif);
    if saif != master_saif {
        dev_err(
            (*saif).dev,
            c"can not get mclk from a non-master saif\n".as_ptr(),
        );
        return -EINVAL;
    }

    stat = __raw_readl(addr((*saif).base, SAIF_STAT));
    if (stat & BM_SAIF_STAT_BUSY) != 0 {
        dev_err((*saif).dev, c"error: busy\n".as_ptr());
        return -EBUSY;
    }

    (*saif).mclk_in_use = 1;

    0
}

/*
 * SAIF is a little different with other normal SOC DAIs on clock using.
 *
 * For MXS, two SAIF modules are instantiated on-chip.
 * Each SAIF has a set of clock pins and can be operating in master
 * mode simultaneously if they are connected to different off-chip codecs.
 * Also, one of the two SAIFs can master or drive the clock pins while the
 * other SAIF, in slave mode, receives clocking from the master SAIF.
 * This also means that both SAIFs must operate at the same sample rate.
 *
 * We abstract this as each saif has a master, the master could be
 * itself or other saifs. In the generic saif driver, saif does not need
 * to know the different clkmux. Saif only needs to know who is its master
 * and operating its master to generate the proper clock rate for it.
 * The master id is provided in mach-specific layer according to different
 * clkmux setting.
 */

unsafe extern "C" fn mxs_saif_set_dai_sysclk(
    cpu_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let saif = snd_soc_dai_get_drvdata(cpu_dai) as *mut mxs_saif;
    let ret: c_int;

    match clk_id {
        MXS_SAIF_MCLK => {
            (*saif).mclk = freq;
        }
        _ => return -EINVAL,
    }

    if (*saif).mclk_in_use == 0 && freq != 0 {
        ret = __mxs_saif_get_mclk(saif);
        if ret != 0 {
            return ret;
        }

        /* enable MCLK output */
        __raw_writel(
            BM_SAIF_CTRL_RUN,
            addr((*saif).base, SAIF_CTRL + MXS_SET_ADDR),
        );
    } else if (*saif).mclk_in_use != 0 && freq == 0 {
        ret = __mxs_saif_put_mclk(saif);
        if ret != 0 {
            return ret;
        }
    }

    0
}

/*
 * Set SAIF clock and MCLK
 */
unsafe fn mxs_saif_set_clk(saif: *mut mxs_saif, mclk: c_uint, rate: c_uint) -> c_int {
    let mut scr: u32;
    let mut ret: c_int;
    let master_saif: *mut mxs_saif;

    dev_dbg((*saif).dev, c"mclk %d rate %d\n".as_ptr(), mclk, rate);

    /* Set master saif to generate proper clock */
    master_saif = mxs_saif_get_master(saif);
    if master_saif.is_null() {
        return -EINVAL;
    }

    dev_dbg((*saif).dev, c"master saif%d\n".as_ptr(), (*master_saif).id);

    /* Checking if can playback and capture simutaneously */
    if (*master_saif).ongoing != 0 && rate != (*master_saif).cur_rate {
        dev_err(
            (*saif).dev,
            c"can not change clock, master saif%d(rate %d) is ongoing\n".as_ptr(),
            (*master_saif).id,
            (*master_saif).cur_rate,
        );
        return -EINVAL;
    }

    scr = __raw_readl(addr((*master_saif).base, SAIF_CTRL));
    scr &= !BM_SAIF_CTRL_BITCLK_MULT_RATE;
    scr &= !BM_SAIF_CTRL_BITCLK_BASE_RATE;

    /*
     * Set SAIF clock
     *
     * The SAIF clock should be either 384*fs or 512*fs.
     * If MCLK is used, the SAIF clk ratio needs to match mclk ratio.
     *  For 256x, 128x, 64x, and 32x sub-rates, set saif clk as 512*fs.
     *  For 192x, 96x, and 48x sub-rates, set saif clk as 384*fs.
     *
     * If MCLK is not used, we just set saif clk to 512*fs.
     */
    ret = clk_prepare_enable((*master_saif).clk);
    if ret != 0 {
        return ret;
    }

    if (*master_saif).mclk_in_use != 0 {
        match mclk / rate {
            32 | 64 | 128 | 256 | 512 => {
                scr &= !BM_SAIF_CTRL_BITCLK_BASE_RATE;
                ret = clk_set_rate((*master_saif).clk, 512 * rate);
            }
            48 | 96 | 192 | 384 => {
                scr |= BM_SAIF_CTRL_BITCLK_BASE_RATE;
                ret = clk_set_rate((*master_saif).clk, 384 * rate);
            }
            _ => {
                /* SAIF MCLK should be a sub-rate of 512x or 384x */
                clk_disable_unprepare((*master_saif).clk);
                return -EINVAL;
            }
        }
    } else {
        ret = clk_set_rate((*master_saif).clk, 512 * rate);
        scr &= !BM_SAIF_CTRL_BITCLK_BASE_RATE;
    }

    clk_disable_unprepare((*master_saif).clk);

    if ret != 0 {
        return ret;
    }

    (*master_saif).cur_rate = rate;

    if (*master_saif).mclk_in_use == 0 {
        __raw_writel(scr, addr((*master_saif).base, SAIF_CTRL));
        return 0;
    }

    /*
     * Program the over-sample rate for MCLK output
     *
     * The available MCLK range is 32x, 48x... 512x. The rate
     * could be from 8kHz to 192kH.
     */
    match mclk / rate {
        32 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(4),
        64 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(3),
        128 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(2),
        256 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(1),
        512 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(0),
        48 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(3),
        96 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(2),
        192 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(1),
        384 => scr |= BF_SAIF_CTRL_BITCLK_MULT_RATE(0),
        _ => return -EINVAL,
    }

    __raw_writel(scr, addr((*master_saif).base, SAIF_CTRL));

    0
}

/*
 * Put and disable MCLK.
 */
#[no_mangle]
pub unsafe extern "C" fn mxs_saif_put_mclk(saif_id: c_uint) -> c_int {
    let saif = mxs_saif[saif_id as usize];
    let stat: u32;

    if saif.is_null() {
        return -EINVAL;
    }

    stat = __raw_readl(addr((*saif).base, SAIF_STAT));
    if (stat & BM_SAIF_STAT_BUSY) != 0 {
        dev_err((*saif).dev, c"error: busy\n".as_ptr());
        return -EBUSY;
    }

    clk_disable_unprepare((*saif).clk);

    /* disable MCLK output */
    __raw_writel(
        BM_SAIF_CTRL_CLKGATE,
        addr((*saif).base, SAIF_CTRL + MXS_SET_ADDR),
    );
    __raw_writel(
        BM_SAIF_CTRL_RUN,
        addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
    );

    (*saif).mclk_in_use = 0;
    0
}

/*
 * Get MCLK and set clock rate, then enable it
 *
 * This interface is used for codecs who are using MCLK provided
 * by saif.
 */
#[no_mangle]
pub unsafe extern "C" fn mxs_saif_get_mclk(saif_id: c_uint, mclk: c_uint, rate: c_uint) -> c_int {
    let saif = mxs_saif[saif_id as usize];
    let mut ret: c_int;

    if saif.is_null() {
        return -EINVAL;
    }

    ret = __mxs_saif_get_mclk(saif);
    if ret != 0 {
        return ret;
    }

    ret = mxs_saif_set_clk(saif, mclk, rate);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*saif).clk);
    if ret != 0 {
        return ret;
    }

    /* enable MCLK output */
    __raw_writel(
        BM_SAIF_CTRL_RUN,
        addr((*saif).base, SAIF_CTRL + MXS_SET_ADDR),
    );

    0
}

/*
 * SAIF DAI format configuration.
 * Should only be called when port is inactive.
 */
unsafe extern "C" fn mxs_saif_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let mut scr: u32;
    let stat: u32;
    let mut scr0: u32;
    let saif = snd_soc_dai_get_drvdata(cpu_dai) as *mut mxs_saif;

    stat = __raw_readl(addr((*saif).base, SAIF_STAT));
    if (stat & BM_SAIF_STAT_BUSY) != 0 {
        dev_err((*cpu_dai).dev, c"error: busy\n".as_ptr());
        return -EBUSY;
    }

    /* If SAIF1 is configured as slave, the clk gate needs to be cleared
     * before the register can be written.
     */
    if (*saif).id != (*saif).master_id {
        __raw_writel(
            BM_SAIF_CTRL_SFTRST,
            addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
        );
        __raw_writel(
            BM_SAIF_CTRL_CLKGATE,
            addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
        );
    }

    scr0 = __raw_readl(addr((*saif).base, SAIF_CTRL));
    scr0 = scr0
        & !BM_SAIF_CTRL_BITCLK_EDGE
        & !BM_SAIF_CTRL_LRCLK_POLARITY
        & !BM_SAIF_CTRL_JUSTIFY
        & !BM_SAIF_CTRL_DELAY;
    scr = 0;

    /* DAI mode */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            /* data frame low 1clk before data */
            scr |= BM_SAIF_CTRL_DELAY;
            scr &= !BM_SAIF_CTRL_LRCLK_POLARITY;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            /* data frame high with data */
            scr &= !BM_SAIF_CTRL_DELAY;
            scr &= !BM_SAIF_CTRL_LRCLK_POLARITY;
            scr &= !BM_SAIF_CTRL_JUSTIFY;
        }
        _ => return -EINVAL,
    }

    /* DAI clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_IF => {
            scr |= BM_SAIF_CTRL_BITCLK_EDGE;
            scr |= BM_SAIF_CTRL_LRCLK_POLARITY;
        }
        SND_SOC_DAIFMT_IB_NF => {
            scr |= BM_SAIF_CTRL_BITCLK_EDGE;
            scr &= !BM_SAIF_CTRL_LRCLK_POLARITY;
        }
        SND_SOC_DAIFMT_NB_IF => {
            scr &= !BM_SAIF_CTRL_BITCLK_EDGE;
            scr |= BM_SAIF_CTRL_LRCLK_POLARITY;
        }
        SND_SOC_DAIFMT_NB_NF => {
            scr &= !BM_SAIF_CTRL_BITCLK_EDGE;
            scr &= !BM_SAIF_CTRL_LRCLK_POLARITY;
        }
        _ => {}
    }

    /*
     * Note: We simply just support master mode since SAIF TX can only
     * work as master.
     * Here the master is relative to codec side.
     * Saif internally could be slave when working on EXTMASTER mode.
     * We just hide this to machine driver.
     */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            if (*saif).id == (*saif).master_id {
                scr &= !BM_SAIF_CTRL_SLAVE_MODE;
            } else {
                scr |= BM_SAIF_CTRL_SLAVE_MODE;
            }

            __raw_writel(scr | scr0, addr((*saif).base, SAIF_CTRL));
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn mxs_saif_startup(
    _substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let saif = snd_soc_dai_get_drvdata(cpu_dai) as *mut mxs_saif;
    let ret: c_int;

    /* clear error status to 0 for each re-open */
    (*saif).fifo_underrun = 0;
    (*saif).fifo_overrun = 0;

    /* Clear Reset for normal operations */
    __raw_writel(
        BM_SAIF_CTRL_SFTRST,
        addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
    );

    /* clear clock gate */
    __raw_writel(
        BM_SAIF_CTRL_CLKGATE,
        addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
    );

    ret = clk_prepare((*saif).clk);
    if ret != 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn mxs_saif_shutdown(
    _substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) {
    let saif = snd_soc_dai_get_drvdata(cpu_dai) as *mut mxs_saif;

    clk_unprepare((*saif).clk);
}

/*
 * Should only be called when port is inactive.
 * although can be called multiple times by upper layers.
 */
unsafe extern "C" fn mxs_saif_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let saif = snd_soc_dai_get_drvdata(cpu_dai) as *mut mxs_saif;
    let master_saif: *mut mxs_saif;
    let mut scr: u32;
    let stat: u32;
    let mut ret: c_int;

    master_saif = mxs_saif_get_master(saif);
    if master_saif.is_null() {
        return -EINVAL;
    }

    /* mclk should already be set */
    if (*saif).mclk == 0 && (*saif).mclk_in_use != 0 {
        dev_err((*cpu_dai).dev, c"set mclk first\n".as_ptr());
        return -EINVAL;
    }

    stat = __raw_readl(addr((*saif).base, SAIF_STAT));
    if (*saif).mclk_in_use == 0 && (stat & BM_SAIF_STAT_BUSY) != 0 {
        dev_err((*cpu_dai).dev, c"error: busy\n".as_ptr());
        return -EBUSY;
    }

    /*
     * Set saif clk based on sample rate.
     * If mclk is used, we also set mclk, if not, saif->mclk is
     * default 0, means not used.
     */
    ret = mxs_saif_set_clk(saif, (*saif).mclk, params_rate(params));
    if ret != 0 {
        dev_err((*cpu_dai).dev, c"unable to get proper clk\n".as_ptr());
        return ret;
    }

    if saif != master_saif {
        /*
        * Set an initial clock rate for the saif internal logic to work
        * properly. This is important when working in EXTMASTER mode
        * that uses the other saif's BITCLK&LRCLK but it still needs a
        * basic clock which should be fast enough for the internal
        * logic.
        */
        ret = clk_enable((*saif).clk);
        if ret != 0 {
            return ret;
        }

        ret = clk_set_rate((*saif).clk, 24000000);
        clk_disable((*saif).clk);
        if ret != 0 {
            return ret;
        }

        ret = clk_prepare((*master_saif).clk);
        if ret != 0 {
            return ret;
        }
    }

    scr = __raw_readl(addr((*saif).base, SAIF_CTRL));

    scr &= !BM_SAIF_CTRL_WORD_LENGTH;
    scr &= !BM_SAIF_CTRL_BITCLK_48XFS_ENABLE;
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            scr |= BF_SAIF_CTRL_WORD_LENGTH(0);
        }
        SNDRV_PCM_FORMAT_S20_3LE => {
            scr |= BF_SAIF_CTRL_WORD_LENGTH(4);
            scr |= BM_SAIF_CTRL_BITCLK_48XFS_ENABLE;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            scr |= BF_SAIF_CTRL_WORD_LENGTH(8);
            scr |= BM_SAIF_CTRL_BITCLK_48XFS_ENABLE;
        }
        _ => return -EINVAL,
    }

    /* Tx/Rx config */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        /* enable TX mode */
        scr &= !BM_SAIF_CTRL_READ_MODE;
    } else {
        /* enable RX mode */
        scr |= BM_SAIF_CTRL_READ_MODE;
    }

    __raw_writel(scr, addr((*saif).base, SAIF_CTRL));
    0
}

unsafe extern "C" fn mxs_saif_prepare(
    _substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let saif = snd_soc_dai_get_drvdata(cpu_dai) as *mut mxs_saif;

    /* enable FIFO error irqs */
    __raw_writel(
        BM_SAIF_CTRL_FIFO_ERROR_IRQ_EN,
        addr((*saif).base, SAIF_CTRL + MXS_SET_ADDR),
    );

    0
}

unsafe extern "C" fn mxs_saif_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let saif = snd_soc_dai_get_drvdata(cpu_dai) as *mut mxs_saif;
    let master_saif: *mut mxs_saif;
    let delay: u32;
    let mut ret: c_int;

    master_saif = mxs_saif_get_master(saif);
    if master_saif.is_null() {
        return -EINVAL;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*saif).state == MXS_SAIF_STATE_RUNNING {
                return 0;
            }

            dev_dbg((*cpu_dai).dev, c"start\n".as_ptr());

            ret = clk_enable((*master_saif).clk);
            if ret != 0 {
                dev_err((*saif).dev, c"Failed to enable master clock\n".as_ptr());
                return ret;
            }

            /*
             * If the saif's master is not itself, we also need to enable
             * itself clk for its internal basic logic to work.
             */
            if saif != master_saif {
                ret = clk_enable((*saif).clk);
                if ret != 0 {
                    dev_err((*saif).dev, c"Failed to enable master clock\n".as_ptr());
                    clk_disable((*master_saif).clk);
                    return ret;
                }

                __raw_writel(
                    BM_SAIF_CTRL_RUN,
                    addr((*saif).base, SAIF_CTRL + MXS_SET_ADDR),
                );
            }

            if (*master_saif).mclk_in_use == 0 {
                __raw_writel(
                    BM_SAIF_CTRL_RUN,
                    addr((*master_saif).base, SAIF_CTRL + MXS_SET_ADDR),
                );
            }

            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                /*
                 * write data to saif data register to trigger
                 * the transfer.
                 * For 24-bit format the 32-bit FIFO register stores
                 * only one channel, so we need to write twice.
                 * This is also safe for the other non 24-bit formats.
                 */
                __raw_writel(0, addr((*saif).base, SAIF_DATA));
                __raw_writel(0, addr((*saif).base, SAIF_DATA));
            } else {
                /*
                 * read data from saif data register to trigger
                 * the receive.
                 * For 24-bit format the 32-bit FIFO register stores
                 * only one channel, so we need to read twice.
                 * This is also safe for the other non 24-bit formats.
                 */
                __raw_readl(addr((*saif).base, SAIF_DATA));
                __raw_readl(addr((*saif).base, SAIF_DATA));
            }

            (*master_saif).ongoing = 1;
            (*saif).state = MXS_SAIF_STATE_RUNNING;

            dev_dbg(
                (*saif).dev,
                c"CTRL 0x%x STAT 0x%x\n".as_ptr(),
                __raw_readl(addr((*saif).base, SAIF_CTRL)),
                __raw_readl(addr((*saif).base, SAIF_STAT)),
            );

            dev_dbg(
                (*master_saif).dev,
                c"CTRL 0x%x STAT 0x%x\n".as_ptr(),
                __raw_readl(addr((*master_saif).base, SAIF_CTRL)),
                __raw_readl(addr((*master_saif).base, SAIF_STAT)),
            );
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if (*saif).state == MXS_SAIF_STATE_STOPPED {
                return 0;
            }

            dev_dbg((*cpu_dai).dev, c"stop\n".as_ptr());

            /* wait a while for the current sample to complete */
            delay = USEC_PER_SEC / (*master_saif).cur_rate;

            if (*master_saif).mclk_in_use == 0 {
                __raw_writel(
                    BM_SAIF_CTRL_RUN,
                    addr((*master_saif).base, SAIF_CTRL + MXS_CLR_ADDR),
                );
                udelay(delay);
            }
            clk_disable((*master_saif).clk);

            if saif != master_saif {
                __raw_writel(
                    BM_SAIF_CTRL_RUN,
                    addr((*saif).base, SAIF_CTRL + MXS_CLR_ADDR),
                );
                udelay(delay);
                clk_disable((*saif).clk);
            }

            (*master_saif).ongoing = 0;
            (*saif).state = MXS_SAIF_STATE_STOPPED;
        }
        _ => return -EINVAL,
    }

    0
}

static mxs_saif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mxs_saif_startup),
    shutdown: Some(mxs_saif_shutdown),
    trigger: Some(mxs_saif_trigger),
    prepare: Some(mxs_saif_prepare),
    hw_params: Some(mxs_saif_hw_params),
    set_sysclk: Some(mxs_saif_set_dai_sysclk),
    set_fmt: Some(mxs_saif_set_dai_fmt),
};

static mut mxs_saif_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"mxs-saif".as_ptr(),
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: MXS_SAIF_RATES,
        formats: MXS_SAIF_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: MXS_SAIF_RATES,
        formats: MXS_SAIF_FORMATS,
    },
    ops: &mxs_saif_dai_ops,
};

static mxs_saif_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"mxs-saif".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn mxs_saif_irq(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let saif = dev_id as *mut mxs_saif;
    let stat: c_uint;

    stat = __raw_readl(addr((*saif).base, SAIF_STAT));
    if (stat & (BM_SAIF_STAT_FIFO_UNDERFLOW_IRQ | BM_SAIF_STAT_FIFO_OVERFLOW_IRQ)) == 0 {
        return IRQ_NONE;
    }

    if (stat & BM_SAIF_STAT_FIFO_UNDERFLOW_IRQ) != 0 {
        (*saif).fifo_underrun = (*saif).fifo_underrun.wrapping_add(1);
        dev_dbg(
            (*saif).dev,
            c"underrun!!! %d\n".as_ptr(),
            (*saif).fifo_underrun,
        );
        __raw_writel(
            BM_SAIF_STAT_FIFO_UNDERFLOW_IRQ,
            addr((*saif).base, SAIF_STAT + MXS_CLR_ADDR),
        );
    }

    if (stat & BM_SAIF_STAT_FIFO_OVERFLOW_IRQ) != 0 {
        (*saif).fifo_overrun = (*saif).fifo_overrun.wrapping_add(1);
        dev_dbg(
            (*saif).dev,
            c"overrun!!! %d\n".as_ptr(),
            (*saif).fifo_overrun,
        );
        __raw_writel(
            BM_SAIF_STAT_FIFO_OVERFLOW_IRQ,
            addr((*saif).base, SAIF_STAT + MXS_CLR_ADDR),
        );
    }

    dev_dbg(
        (*saif).dev,
        c"SAIF_CTRL %x SAIF_STAT %x\n".as_ptr(),
        __raw_readl(addr((*saif).base, SAIF_CTRL)),
        __raw_readl(addr((*saif).base, SAIF_STAT)),
    );

    IRQ_HANDLED
}

unsafe fn mxs_saif_mclk_init(pdev: *mut platform_device) -> c_int {
    let saif = platform_get_drvdata(pdev) as *mut mxs_saif;
    let np = (*pdev).dev.of_node;
    let clk: *mut clk;
    let ret: c_int;

    clk = clk_register_divider(
        &mut (*pdev).dev,
        c"mxs_saif_mclk".as_ptr(),
        __clk_get_name((*saif).clk),
        0,
        addr((*saif).base, SAIF_CTRL),
        BP_SAIF_CTRL_BITCLK_MULT_RATE,
        3,
        0,
        ptr::null_mut(),
    );
    if IS_ERR(clk as *const c_void) {
        let err = PTR_ERR(clk as *const c_void);
        if err == -EEXIST {
            return 0;
        }
        dev_err(
            &mut (*pdev).dev,
            c"failed to register mclk: %d\n".as_ptr(),
            err,
        );
        return PTR_ERR(clk as *const c_void);
    }

    ret = of_clk_add_provider(np, &mut of_clk_src_simple_get as *mut c_void, clk);
    if ret != 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn mxs_saif_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let saif: *mut mxs_saif;
    let mut irq: c_int;
    let mut ret: c_int;
    let master: *mut device_node;

    saif = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<mxs_saif>(),
        GFP_KERNEL,
    ) as *mut mxs_saif;
    if saif.is_null() {
        return -ENOMEM;
    }

    ret = of_alias_get_id(np, c"saif".as_ptr());
    if ret < 0 {
        return ret;
    } else {
        (*saif).id = ret;
    }

    if (*saif).id as usize >= mxs_saif.len() {
        dev_err(&mut (*pdev).dev, c"get wrong saif id\n".as_ptr());
        return -EINVAL;
    }

    /*
     * If there is no "fsl,saif-master" phandle, it's a saif
     * master.  Otherwise, it's a slave and its phandle points
     * to the master.
     */
    master = of_parse_phandle(np, c"fsl,saif-master".as_ptr(), 0);
    if master.is_null() {
        (*saif).master_id = (*saif).id;
    } else {
        ret = of_alias_get_id(master, c"saif".as_ptr());
        of_node_put(master);
        if ret < 0 {
            return ret;
        } else {
            (*saif).master_id = ret;
        }

        if (*saif).master_id as usize >= mxs_saif.len() {
            dev_err(&mut (*pdev).dev, c"get wrong master id\n".as_ptr());
            return -EINVAL;
        }
    }

    mxs_saif[(*saif).id as usize] = saif;

    (*saif).clk = devm_clk_get(&mut (*pdev).dev, ptr::null());
    if IS_ERR((*saif).clk as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*saif).clk as *const c_void),
            c"Cannot get the clock\n".as_ptr(),
        );
    }

    (*saif).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*saif).base as *const c_void) {
        return PTR_ERR((*saif).base as *const c_void);
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    (*saif).dev = &mut (*pdev).dev;
    ret = devm_request_irq(
        &mut (*pdev).dev,
        irq,
        Some(mxs_saif_irq),
        0,
        dev_name(&mut (*pdev).dev),
        saif as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    platform_set_drvdata(pdev, saif as *mut c_void);

    /* We only support saif0 being tx and clock master */
    if (*saif).id == 0 {
        ret = mxs_saif_mclk_init(pdev);
        if ret != 0 {
            dev_warn(&mut (*pdev).dev, c"failed to init clocks\n".as_ptr());
        }
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &mxs_saif_component,
        &raw mut mxs_saif_dai,
        1,
    );
    if ret != 0 {
        return ret;
    }

    ret = mxs_pcm_platform_register(&mut (*pdev).dev);
    if ret != 0 {
        return ret;
    }

    0
}

static mxs_saif_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"fsl,imx28-saif".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

static mut mxs_saif_driver: platform_driver = platform_driver {
    probe: Some(mxs_saif_probe),

    driver: platform_driver_inner {
        name: c"mxs-saif".as_ptr(),
        of_match_table: mxs_saif_dt_ids.as_ptr(),
    },
};

/* MODULE_DEVICE_TABLE(of, mxs_saif_dt_ids); */
/* module_platform_driver(mxs_saif_driver); */

/* MODULE_AUTHOR("Freescale Semiconductor, Inc."); */
/* MODULE_DESCRIPTION("MXS ASoC SAIF driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:mxs-saif"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
