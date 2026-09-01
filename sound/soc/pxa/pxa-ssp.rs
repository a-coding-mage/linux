// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pxa-ssp.c  --  ALSA Soc Audio Layer
 *
 * Copyright 2005,2008 Wolfson Microelectronics PLC.
 * Author: Liam Girdwood
 *         Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 * TODO:
 *  o Test network mode for > 16bit sample size
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct ssp_device {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub phys_base: c_ulong,
    pub mmio_base: *mut c_void,
    pub type_: c_int,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr_width: c_uint,
    pub maxburst: c_uint,
    pub addr: c_ulong,
    pub chan_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
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
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub pcm_new: Option<unsafe extern "C" fn() -> c_int>,
    pub open: Option<unsafe extern "C" fn() -> c_int>,
    pub close: Option<unsafe extern "C" fn() -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
    pub prepare: Option<unsafe extern "C" fn() -> c_int>,
    pub trigger: Option<unsafe extern "C" fn() -> c_int>,
    pub pointer: Option<unsafe extern "C" fn() -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub legacy_dai_naming: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

/*
 * External constants, macros, and functions supplied by kernel/ALSA/PXA headers.
 */
extern "C" {
    static mut SSCR0: c_uint;
    static mut SSCR1: c_uint;
    static mut SSTO: c_uint;
    static mut SSPSP: c_uint;
    static mut SSSR: c_uint;
    static mut SSACD: c_uint;
    static mut SSDR: c_ulong;
    static mut SSACDD: c_uint;
    static mut SSTSA: c_uint;
    static mut SSRSA: c_uint;

    static mut DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint;
    static mut DMA_SLAVE_BUSWIDTH_2_BYTES: c_uint;
    static mut SNDRV_PCM_STREAM_PLAYBACK: c_int;

    static mut ENOMEM: c_int;
    static mut ENODEV: c_int;
    static mut EINVAL: c_int;

    static mut PXA25x_SSP: c_int;
    static mut PXA3xx_SSP: c_int;
    static mut PXA_SSP_CLK_NET_PLL: c_int;
    static mut PXA_SSP_CLK_PLL: c_int;
    static mut PXA_SSP_CLK_EXT: c_int;
    static mut PXA_SSP_CLK_NET: c_int;
    static mut PXA_SSP_CLK_AUDIO: c_int;

    static mut SSCR0_ECS: u32;
    static mut SSCR0_NCS: u32;
    static mut SSCR0_MOD: u32;
    static mut SSCR0_ACS: u32;
    static mut SSCR0_SSE: u32;
    static mut SSCR0_PSP: u32;
    static mut SSCR0_EDSS: u32;
    static mut SSCR0_DSS: u32;
    static mut SSCR0_FPCKE: u32;
    static mut SSCR1_TTE: u32;
    static mut SSCR1_SCLKDIR: u32;
    static mut SSCR1_SFRMDIR: u32;
    static mut SSCR1_SCFR: u32;
    static mut SSCR1_RWOT: u32;
    static mut SSCR1_TRAIL: u32;
    static mut SSCR1_TFT: u32;
    static mut SSCR1_RFT: u32;
    static mut SSCR1_TSRE: u32;
    static mut SSCR1_RSRE: u32;
    static mut SSPSP_SFRMP: u32;
    static mut SSPSP_FSRT: u32;
    static mut SSSR_ROR: u32;
    static mut SSSR_TUR: u32;
    static mut SSSR_BCE: u32;
    static mut SSSR_BSY: u32;
    static mut SSACD_ACDS_32: u8;
    static mut SSACD_ACDS_16: u8;
    static mut SSACD_ACDS_4: u8;
    static mut SSACD_ACDS_2: u8;
    static mut SSACD_SCDB_4X: u8;
    static mut SSACD_SCDB_1X: u8;

    static mut SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static mut SND_SOC_DAIFMT_BC_FC: c_uint;
    static mut SND_SOC_DAIFMT_BC_FP: c_uint;
    static mut SND_SOC_DAIFMT_BP_FP: c_uint;
    static mut SND_SOC_DAIFMT_INV_MASK: c_uint;
    static mut SND_SOC_DAIFMT_NB_NF: c_uint;
    static mut SND_SOC_DAIFMT_NB_IF: c_uint;
    static mut SND_SOC_DAIFMT_IB_IF: c_uint;
    static mut SND_SOC_DAIFMT_IB_NF: c_uint;
    static mut SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static mut SND_SOC_DAIFMT_I2S: c_uint;
    static mut SND_SOC_DAIFMT_DSP_A: c_uint;
    static mut SND_SOC_DAIFMT_DSP_B: c_uint;

    static mut SNDRV_PCM_FORMAT_S16_LE: c_int;
    static mut SNDRV_PCM_FORMAT_S24_LE: c_int;
    static mut SNDRV_PCM_FORMAT_S32_LE: c_int;
    static mut SNDRV_PCM_TRIGGER_RESUME: c_int;
    static mut SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static mut SNDRV_PCM_TRIGGER_START: c_int;
    static mut SNDRV_PCM_TRIGGER_STOP: c_int;
    static mut SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static mut SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static mut SNDRV_PCM_RATE_8000: c_uint;
    static mut SNDRV_PCM_RATE_11025: c_uint;
    static mut SNDRV_PCM_RATE_16000: c_uint;
    static mut SNDRV_PCM_RATE_22050: c_uint;
    static mut SNDRV_PCM_RATE_32000: c_uint;
    static mut SNDRV_PCM_RATE_44100: c_uint;
    static mut SNDRV_PCM_RATE_48000: c_uint;
    static mut SNDRV_PCM_RATE_64000: c_uint;
    static mut SNDRV_PCM_RATE_88200: c_uint;
    static mut SNDRV_PCM_RATE_96000: c_uint;
    static mut SNDRV_PCM_FMTBIT_S16_LE: u64;
    static mut SNDRV_PCM_FMTBIT_S32_LE: u64;

    fn pxa_ssp_read_reg(ssp: *mut ssp_device, reg: c_uint) -> u32;
    fn pxa_ssp_write_reg(ssp: *mut ssp_device, reg: c_uint, val: u32);
    fn pxa_ssp_disable(ssp: *mut ssp_device);
    fn pxa_ssp_enable(ssp: *mut ssp_device);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_set_drvdata(dai: *mut snd_soc_dai, data: *mut c_void);
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut snd_dmaengine_dai_dma_data;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_active(component: *mut snd_soc_component) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_physical_width(format: c_int) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn pxa_ssp_request_of(np: *mut device_node, label: *const c_char) -> *mut ssp_device;
    fn pxa_ssp_request(port: c_int, label: *const c_char) -> *mut ssp_device;
    fn pxa_ssp_free(ssp: *mut ssp_device);
    fn devm_snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pxa2xx_soc_pcm_new() -> c_int;
    fn pxa2xx_soc_pcm_open() -> c_int;
    fn pxa2xx_soc_pcm_close() -> c_int;
    fn pxa2xx_soc_pcm_hw_params() -> c_int;
    fn pxa2xx_soc_pcm_prepare() -> c_int;
    fn pxa2xx_soc_pcm_trigger() -> c_int;
    fn pxa2xx_soc_pcm_pointer() -> c_int;
    fn __raw_readl(addr: *mut c_void) -> u32;
    fn __raw_writel(val: u32, addr: *mut c_void);
    fn cpu_relax();
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

extern "C" {
    static mut GFP_KERNEL: c_uint;
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn reg_addr(base: *mut c_void, off: c_uint) -> *mut c_void {
    (base as *mut u8).add(off as usize) as *mut c_void
}

extern "C" {
    fn SSCR0_SlotsPerFrm(slots: c_int) -> u32;
    fn SSCR0_DataSize(size: c_int) -> u32;
    fn SSCR1_RxTresh(val: c_int) -> u32;
    fn SSCR1_TxTresh(val: c_int) -> u32;
    fn SSPSP_SCMODE(val: c_int) -> u32;
    fn SSPSP_SFRMWDTH(val: c_int) -> u32;
    fn SSPSP_SFRMDLY(val: c_int) -> u32;
    fn SSPSP_EDMYSTOP(val: c_int) -> u32;
    fn SSPSP_DMYSTOP(val: c_int) -> u32;
    fn SSPSP_DMYSTRT(val: c_int) -> u32;
    fn SSACD_ACDS(val: u8) -> u32;
}

/*
 * SSP audio private data
 */
#[repr(C)]
struct ssp_priv {
    ssp: *mut ssp_device,
    extclk: *mut clk,
    ssp_clk: c_ulong,
    sysclk: c_uint,
    dai_fmt: c_uint,
    configured_dai_fmt: c_uint,
    /* CONFIG_PM */
    cr0: u32,
    cr1: u32,
    to: u32,
    psp: u32,
}

unsafe extern "C" fn dump_registers(ssp: *mut ssp_device) {
    dev_dbg(
        (*ssp).dev,
        b"SSCR0 0x%08x SSCR1 0x%08x SSTO 0x%08x\n\0".as_ptr() as *const c_char,
        pxa_ssp_read_reg(ssp, SSCR0),
        pxa_ssp_read_reg(ssp, SSCR1),
        pxa_ssp_read_reg(ssp, SSTO),
    );

    dev_dbg(
        (*ssp).dev,
        b"SSPSP 0x%08x SSSR 0x%08x SSACD 0x%08x\n\0".as_ptr() as *const c_char,
        pxa_ssp_read_reg(ssp, SSPSP),
        pxa_ssp_read_reg(ssp, SSSR),
        pxa_ssp_read_reg(ssp, SSACD),
    );
}

unsafe extern "C" fn pxa_ssp_set_dma_params(
    ssp: *mut ssp_device,
    width4: c_int,
    _out: c_int,
    dma: *mut snd_dmaengine_dai_dma_data,
) {
    (*dma).addr_width = if width4 != 0 {
        DMA_SLAVE_BUSWIDTH_4_BYTES
    } else {
        DMA_SLAVE_BUSWIDTH_2_BYTES
    };
    (*dma).maxburst = 16;
    (*dma).addr = (*ssp).phys_base + SSDR;
}

unsafe extern "C" fn pxa_ssp_startup(substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai) as *mut ssp_priv;
    let ssp = (*priv_).ssp;
    let dma: *mut snd_dmaengine_dai_dma_data;
    let ret: c_int = 0;

    if snd_soc_dai_active(cpu_dai) == 0 {
        clk_prepare_enable((*ssp).clk);
        pxa_ssp_disable(ssp);
    }

    clk_prepare_enable((*priv_).extclk);

    dma = kzalloc_obj::<snd_dmaengine_dai_dma_data>();
    if dma.is_null() {
        return -ENOMEM;
    }
    (*dma).chan_name = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        b"tx\0".as_ptr() as *const c_char
    } else {
        b"rx\0".as_ptr() as *const c_char
    };

    snd_soc_dai_set_dma_data(cpu_dai, substream, dma);

    ret
}

unsafe extern "C" fn pxa_ssp_shutdown(substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai) {
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai) as *mut ssp_priv;
    let ssp = (*priv_).ssp;

    if snd_soc_dai_active(cpu_dai) == 0 {
        pxa_ssp_disable(ssp);
        clk_disable_unprepare((*ssp).clk);
    }

    clk_disable_unprepare((*priv_).extclk);

    kfree(snd_soc_dai_get_dma_data(cpu_dai, substream) as *mut c_void);
    snd_soc_dai_set_dma_data(cpu_dai, substream, ptr::null_mut());
}

/* CONFIG_PM */
unsafe extern "C" fn pxa_ssp_suspend(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ssp_priv;
    let ssp = (*priv_).ssp;

    if snd_soc_component_active(component) == 0 {
        clk_prepare_enable((*ssp).clk);
    }

    (*priv_).cr0 = __raw_readl(reg_addr((*ssp).mmio_base, SSCR0));
    (*priv_).cr1 = __raw_readl(reg_addr((*ssp).mmio_base, SSCR1));
    (*priv_).to = __raw_readl(reg_addr((*ssp).mmio_base, SSTO));
    (*priv_).psp = __raw_readl(reg_addr((*ssp).mmio_base, SSPSP));

    pxa_ssp_disable(ssp);
    clk_disable_unprepare((*ssp).clk);
    0
}

unsafe extern "C" fn pxa_ssp_resume(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ssp_priv;
    let ssp = (*priv_).ssp;
    let sssr: u32 = SSSR_ROR | SSSR_TUR | SSSR_BCE;

    clk_prepare_enable((*ssp).clk);

    __raw_writel(sssr, reg_addr((*ssp).mmio_base, SSSR));
    __raw_writel((*priv_).cr0 & !SSCR0_SSE, reg_addr((*ssp).mmio_base, SSCR0));
    __raw_writel((*priv_).cr1, reg_addr((*ssp).mmio_base, SSCR1));
    __raw_writel((*priv_).to, reg_addr((*ssp).mmio_base, SSTO));
    __raw_writel((*priv_).psp, reg_addr((*ssp).mmio_base, SSPSP));

    if snd_soc_component_active(component) != 0 {
        pxa_ssp_enable(ssp);
    } else {
        clk_disable_unprepare((*ssp).clk);
    }

    0
}

/*
 * ssp_set_clkdiv - set SSP clock divider
 * @div: serial clock rate divider
 */
unsafe extern "C" fn pxa_ssp_set_scr(ssp: *mut ssp_device, div: u32) {
    let mut sscr0 = pxa_ssp_read_reg(ssp, SSCR0);

    if (*ssp).type_ == PXA25x_SSP {
        sscr0 &= !0x0000ff00;
        sscr0 |= ((div - 2) / 2) << 8; /* 2..512 */
    } else {
        sscr0 &= !0x000fff00;
        sscr0 |= (div - 1) << 8; /* 1..4096 */
    }
    pxa_ssp_write_reg(ssp, SSCR0, sscr0);
}

/*
 * Set the SSP ports SYSCLK.
 */
unsafe extern "C" fn pxa_ssp_set_dai_sysclk(
    cpu_dai: *mut snd_soc_dai,
    mut clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai) as *mut ssp_priv;
    let ssp = (*priv_).ssp;

    let mut sscr0 = pxa_ssp_read_reg(ssp, SSCR0) & !(SSCR0_ECS | SSCR0_NCS | SSCR0_MOD | SSCR0_ACS);

    if !(*priv_).extclk.is_null() {
        let ret: c_int;

        /*
         * For DT based boards, if an extclk is given, use it
         * here and configure PXA_SSP_CLK_EXT.
         */
        ret = clk_set_rate((*priv_).extclk, freq);
        if ret < 0 {
            return ret;
        }

        clk_id = PXA_SSP_CLK_EXT;
    }

    dev_dbg(
        (*ssp).dev,
        b"pxa_ssp_set_dai_sysclk id: %d, clk_id %d, freq %u\n\0".as_ptr() as *const c_char,
        (*cpu_dai).id,
        clk_id,
        freq,
    );

    if clk_id == PXA_SSP_CLK_NET_PLL {
        sscr0 |= SSCR0_MOD;
    } else if clk_id == PXA_SSP_CLK_PLL {
        /* Internal PLL is fixed */
        if (*ssp).type_ == PXA25x_SSP {
            (*priv_).sysclk = 1843200;
        } else {
            (*priv_).sysclk = 13000000;
        }
    } else if clk_id == PXA_SSP_CLK_EXT {
        (*priv_).sysclk = freq;
        sscr0 |= SSCR0_ECS;
    } else if clk_id == PXA_SSP_CLK_NET {
        (*priv_).sysclk = freq;
        sscr0 |= SSCR0_NCS | SSCR0_MOD;
    } else if clk_id == PXA_SSP_CLK_AUDIO {
        (*priv_).sysclk = 0;
        pxa_ssp_set_scr(ssp, 1);
        sscr0 |= SSCR0_ACS;
    } else {
        return -ENODEV;
    }

    /* The SSP clock must be disabled when changing SSP clock mode
     * on PXA2xx.  On PXA3xx it must be enabled when doing so. */
    if (*ssp).type_ != PXA3xx_SSP {
        clk_disable_unprepare((*ssp).clk);
    }
    pxa_ssp_write_reg(ssp, SSCR0, sscr0);
    if (*ssp).type_ != PXA3xx_SSP {
        clk_prepare_enable((*ssp).clk);
    }

    0
}

/*
 * Configure the PLL frequency pxa27x and (afaik - pxa320 only)
 */
unsafe extern "C" fn pxa_ssp_set_pll(priv_: *mut ssp_priv, freq: c_uint) -> c_int {
    let ssp = (*priv_).ssp;
    let mut ssacd = pxa_ssp_read_reg(ssp, SSACD) & !0x70;

    if (*ssp).type_ == PXA3xx_SSP {
        pxa_ssp_write_reg(ssp, SSACDD, 0);
    }

    match freq {
        5622000 => {}
        11345000 => ssacd |= 0x1 << 4,
        12235000 => ssacd |= 0x2 << 4,
        14857000 => ssacd |= 0x3 << 4,
        32842000 => ssacd |= 0x4 << 4,
        48000000 => ssacd |= 0x5 << 4,
        0 => {
            /* Disable */
        }
        _ => {
            /* PXA3xx has a clock ditherer which can be used to generate
             * a wider range of frequencies - calculate a value for it.
             */
            if (*ssp).type_ == PXA3xx_SSP {
                let mut val: u32;
                let mut tmp: u64 = 19968;

                tmp = tmp.wrapping_mul(1000000);
                tmp /= freq as u64;
                val = tmp as u32;

                val = (val << 16) | 64;
                pxa_ssp_write_reg(ssp, SSACDD, val);

                ssacd |= 0x6 << 4;

                dev_dbg(
                    (*ssp).dev,
                    b"Using SSACDD %x to supply %uHz\n\0".as_ptr() as *const c_char,
                    val,
                    freq,
                );
            } else {
                return -EINVAL;
            }
        }
    }

    pxa_ssp_write_reg(ssp, SSACD, ssacd);

    0
}

/*
 * Set the active slots in TDM/Network mode
 */
unsafe extern "C" fn pxa_ssp_set_dai_tdm_slot(
    cpu_dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai) as *mut ssp_priv;
    let ssp = (*priv_).ssp;
    let mut sscr0: u32;

    sscr0 = pxa_ssp_read_reg(ssp, SSCR0);
    sscr0 &= !(SSCR0_MOD | SSCR0_SlotsPerFrm(8) | SSCR0_EDSS | SSCR0_DSS);

    /* set slot width */
    if slot_width > 16 {
        sscr0 |= SSCR0_EDSS | SSCR0_DataSize(slot_width - 16);
    } else {
        sscr0 |= SSCR0_DataSize(slot_width);
    }

    if slots > 1 {
        /* enable network mode */
        sscr0 |= SSCR0_MOD;

        /* set number of active slots */
        sscr0 |= SSCR0_SlotsPerFrm(slots);

        /* set active slot mask */
        pxa_ssp_write_reg(ssp, SSTSA, tx_mask);
        pxa_ssp_write_reg(ssp, SSRSA, rx_mask);
    }
    pxa_ssp_write_reg(ssp, SSCR0, sscr0);

    0
}

/*
 * Tristate the SSP DAI lines
 */
unsafe extern "C" fn pxa_ssp_set_dai_tristate(cpu_dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai) as *mut ssp_priv;
    let ssp = (*priv_).ssp;
    let mut sscr1: u32;

    sscr1 = pxa_ssp_read_reg(ssp, SSCR1);
    if tristate != 0 {
        sscr1 &= !SSCR1_TTE;
    } else {
        sscr1 |= SSCR1_TTE;
    }
    pxa_ssp_write_reg(ssp, SSCR1, sscr1);

    0
}

unsafe extern "C" fn pxa_ssp_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai) as *mut ssp_priv;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_BC_FC || x == SND_SOC_DAIFMT_BC_FP || x == SND_SOC_DAIFMT_BP_FP => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF
            || x == SND_SOC_DAIFMT_NB_IF
            || x == SND_SOC_DAIFMT_IB_IF
            || x == SND_SOC_DAIFMT_IB_NF => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {}
        _ => return -EINVAL,
    }

    /* Settings will be applied in hw_params() */
    (*priv_).dai_fmt = fmt;

    0
}

/*
 * Set up the SSP DAI format.
 * The SSP Port must be inactive before calling this function as the
 * physical interface format is changed.
 */
unsafe extern "C" fn pxa_ssp_configure_dai_fmt(priv_: *mut ssp_priv) -> c_int {
    let ssp = (*priv_).ssp;
    let mut sscr0: u32;
    let mut sscr1: u32;
    let mut sspsp: u32;
    let scfr: u32;

    /* check if we need to change anything at all */
    if (*priv_).configured_dai_fmt == (*priv_).dai_fmt {
        return 0;
    }

    /* reset port settings */
    sscr0 = pxa_ssp_read_reg(ssp, SSCR0) & !(SSCR0_PSP | SSCR0_MOD);
    sscr1 = pxa_ssp_read_reg(ssp, SSCR1)
        & !(SSCR1_SCLKDIR | SSCR1_SFRMDIR | SSCR1_SCFR | SSCR1_RWOT | SSCR1_TRAIL | SSCR1_TFT | SSCR1_RFT);
    sspsp = pxa_ssp_read_reg(ssp, SSPSP) & !(SSPSP_SFRMP | SSPSP_SCMODE(3));

    sscr1 |= SSCR1_RxTresh(8) | SSCR1_TxTresh(7);

    match (*priv_).dai_fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_BC_FC => sscr1 |= SSCR1_SCLKDIR | SSCR1_SFRMDIR | SSCR1_SCFR,
        x if x == SND_SOC_DAIFMT_BC_FP => sscr1 |= SSCR1_SCLKDIR | SSCR1_SCFR,
        x if x == SND_SOC_DAIFMT_BP_FP => {}
        _ => return -EINVAL,
    }

    match (*priv_).dai_fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => sspsp |= SSPSP_SFRMP,
        x if x == SND_SOC_DAIFMT_NB_IF => {}
        x if x == SND_SOC_DAIFMT_IB_IF => sspsp |= SSPSP_SCMODE(2),
        x if x == SND_SOC_DAIFMT_IB_NF => sspsp |= SSPSP_SCMODE(2) | SSPSP_SFRMP,
        _ => return -EINVAL,
    }

    match (*priv_).dai_fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            sscr0 |= SSCR0_PSP;
            sscr1 |= SSCR1_RWOT | SSCR1_TRAIL;
            /* See hw_params() */
        }
        x if x == SND_SOC_DAIFMT_DSP_A => {
            sspsp |= SSPSP_FSRT;
            sscr0 |= SSCR0_MOD | SSCR0_PSP;
            sscr1 |= SSCR1_TRAIL | SSCR1_RWOT;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => {
            sscr0 |= SSCR0_MOD | SSCR0_PSP;
            sscr1 |= SSCR1_TRAIL | SSCR1_RWOT;
        }
        _ => return -EINVAL,
    }

    pxa_ssp_write_reg(ssp, SSCR0, sscr0);
    pxa_ssp_write_reg(ssp, SSCR1, sscr1);
    pxa_ssp_write_reg(ssp, SSPSP, sspsp);

    match (*priv_).dai_fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_BC_FC || x == SND_SOC_DAIFMT_BC_FP => {
            scfr = pxa_ssp_read_reg(ssp, SSCR1) | SSCR1_SCFR;
            pxa_ssp_write_reg(ssp, SSCR1, scfr);

            while (pxa_ssp_read_reg(ssp, SSSR) & SSSR_BSY) != 0 {
                cpu_relax();
            }
        }
        _ => {}
    }

    dump_registers(ssp);

    /* Since we are configuring the timings for the format by hand
     * we have to defer some things until hw_params() where we
     * know parameters like the sample size.
     */
    (*priv_).configured_dai_fmt = (*priv_).dai_fmt;

    0
}

#[repr(C)]
struct pxa_ssp_clock_mode {
    rate: c_int,
    pll: c_int,
    acds: u8,
    scdb: u8,
}

static mut pxa_ssp_clock_modes: [pxa_ssp_clock_mode; 8] = [
    pxa_ssp_clock_mode { rate: 8000, pll: 32842000, acds: unsafe { SSACD_ACDS_32 }, scdb: unsafe { SSACD_SCDB_4X } },
    pxa_ssp_clock_mode { rate: 11025, pll: 5622000, acds: unsafe { SSACD_ACDS_4 }, scdb: unsafe { SSACD_SCDB_4X } },
    pxa_ssp_clock_mode { rate: 16000, pll: 32842000, acds: unsafe { SSACD_ACDS_16 }, scdb: unsafe { SSACD_SCDB_4X } },
    pxa_ssp_clock_mode { rate: 22050, pll: 5622000, acds: unsafe { SSACD_ACDS_2 }, scdb: unsafe { SSACD_SCDB_4X } },
    pxa_ssp_clock_mode { rate: 44100, pll: 11345000, acds: unsafe { SSACD_ACDS_2 }, scdb: unsafe { SSACD_SCDB_4X } },
    pxa_ssp_clock_mode { rate: 48000, pll: 12235000, acds: unsafe { SSACD_ACDS_2 }, scdb: unsafe { SSACD_SCDB_4X } },
    pxa_ssp_clock_mode { rate: 96000, pll: 12235000, acds: unsafe { SSACD_ACDS_4 }, scdb: unsafe { SSACD_SCDB_1X } },
    pxa_ssp_clock_mode { rate: 0, pll: 0, acds: 0, scdb: 0 },
];

/*
 * Set the SSP audio DMA parameters and sample size.
 * Can be called multiple times by oss emulation.
 */
unsafe extern "C" fn pxa_ssp_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai) as *mut ssp_priv;
    let ssp = (*priv_).ssp;
    let chn = params_channels(params);
    let mut sscr0: u32;
    let mut sspsp: u32;
    let width = snd_pcm_format_physical_width(params_format(params));
    let ttsa = (pxa_ssp_read_reg(ssp, SSTSA) & 0xf) as c_int;
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let rate = params_rate(params);
    let bclk = rate * chn * (width / 8);
    let mut ret: c_int;

    dma_data = snd_soc_dai_get_dma_data(cpu_dai, substream);

    /* Network mode with one active slot (ttsa == 1) can be used
     * to force 16-bit frame width on the wire (for S16_LE), even
     * with two channels. Use 16-bit DMA transfers for this case.
     */
    pxa_ssp_set_dma_params(
        ssp,
        (((chn == 2) && (ttsa != 1)) || (width == 32)) as c_int,
        ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK) as c_int,
        dma_data,
    );

    /* we can only change the settings if the port is not in use */
    if (pxa_ssp_read_reg(ssp, SSCR0) & SSCR0_SSE) != 0 {
        return 0;
    }

    ret = pxa_ssp_configure_dai_fmt(priv_);
    if ret < 0 {
        return ret;
    }

    /* clear selected SSP bits */
    sscr0 = pxa_ssp_read_reg(ssp, SSCR0) & !(SSCR0_DSS | SSCR0_EDSS);

    /* bit size */
    if params_format(params) == SNDRV_PCM_FORMAT_S16_LE {
        if (*ssp).type_ == PXA3xx_SSP {
            sscr0 |= SSCR0_FPCKE;
        }
        sscr0 |= SSCR0_DataSize(16);
    } else if params_format(params) == SNDRV_PCM_FORMAT_S24_LE {
        sscr0 |= SSCR0_EDSS | SSCR0_DataSize(8);
    } else if params_format(params) == SNDRV_PCM_FORMAT_S32_LE {
        sscr0 |= SSCR0_EDSS | SSCR0_DataSize(16);
    }
    pxa_ssp_write_reg(ssp, SSCR0, sscr0);

    if (sscr0 & SSCR0_ACS) != 0 {
        ret = pxa_ssp_set_pll(priv_, bclk as c_uint);

        /*
         * If we were able to generate the bclk directly,
         * all is fine. Otherwise, look up the closest rate
         * from the table and also set the dividers.
         */
        if ret < 0 {
            let mut m = pxa_ssp_clock_modes.as_ptr();
            let mut ssacd: c_int;

            while (*m).rate != 0 {
                if (*m).rate == rate {
                    break;
                }
                m = m.add(1);
            }

            if (*m).rate == 0 {
                return -EINVAL;
            }

            ret = pxa_ssp_set_pll(priv_, bclk as c_uint);
            if ret < 0 {
                return ret;
            }

            ssacd = pxa_ssp_read_reg(ssp, SSACD) as c_int;
            ssacd &= !(SSACD_ACDS(7) | SSACD_SCDB_1X as u32) as c_int;
            ssacd |= SSACD_ACDS((*m).acds) as c_int;
            ssacd |= (*m).scdb as c_int;
            pxa_ssp_write_reg(ssp, SSACD, ssacd as u32);
        }
    } else if (sscr0 & SSCR0_ECS) != 0 {
        /*
         * For setups with external clocking, the PLL and its diviers
         * are not active. Instead, the SCR bits in SSCR0 can be used
         * to divide the clock.
         */
        pxa_ssp_set_scr(ssp, (bclk / rate) as u32);
    }

    match (*priv_).dai_fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            sspsp = pxa_ssp_read_reg(ssp, SSPSP);

            if (((*priv_).sysclk / bclk as c_uint) == 64) && (width == 16) {
                /* This is a special case where the bitclk is 64fs
                 * and we're not dealing with 2*32 bits of audio
                 * samples.
                 *
                 * The SSP values used for that are all found out by
                 * trying and failing a lot; some of the registers
                 * needed for that mode are only available on PXA3xx.
                 */
                if (*ssp).type_ != PXA3xx_SSP {
                    return -EINVAL;
                }

                sspsp |= SSPSP_SFRMWDTH(width * 2);
                sspsp |= SSPSP_SFRMDLY(width * 4);
                sspsp |= SSPSP_EDMYSTOP(3);
                sspsp |= SSPSP_DMYSTOP(3);
                sspsp |= SSPSP_DMYSTRT(1);
            } else {
                /* The frame width is the width the LRCLK is
                 * asserted for; the delay is expressed in
                 * half cycle units.  We need the extra cycle
                 * because the data starts clocking out one BCLK
                 * after LRCLK changes polarity.
                 */
                sspsp |= SSPSP_SFRMWDTH(width + 1);
                sspsp |= SSPSP_SFRMDLY((width + 1) * 2);
                sspsp |= SSPSP_DMYSTRT(1);
            }

            pxa_ssp_write_reg(ssp, SSPSP, sspsp);
        }
        _ => {}
    }

    /* When we use a network mode, we always require TDM slots
     * - complain loudly and fail if they've not been set up yet.
     */
    if ((sscr0 & SSCR0_MOD) != 0) && ttsa == 0 {
        dev_err((*ssp).dev, b"No TDM timeslot configured\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    dump_registers(ssp);

    0
}

unsafe extern "C" fn pxa_ssp_set_running_bit(substream: *mut snd_pcm_substream, ssp: *mut ssp_device, value: c_int) {
    let sscr0: u32 = pxa_ssp_read_reg(ssp, SSCR0);
    let mut sscr1: u32 = pxa_ssp_read_reg(ssp, SSCR1);
    let sspsp: u32 = pxa_ssp_read_reg(ssp, SSPSP);
    let sssr: u32 = pxa_ssp_read_reg(ssp, SSSR);

    if value != 0 && (sscr0 & SSCR0_SSE) != 0 {
        pxa_ssp_write_reg(ssp, SSCR0, sscr0 & !SSCR0_SSE);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if value != 0 {
            sscr1 |= SSCR1_TSRE;
        } else {
            sscr1 &= !SSCR1_TSRE;
        }
    } else if value != 0 {
        sscr1 |= SSCR1_RSRE;
    } else {
        sscr1 &= !SSCR1_RSRE;
    }

    pxa_ssp_write_reg(ssp, SSCR1, sscr1);

    if value != 0 {
        pxa_ssp_write_reg(ssp, SSSR, sssr);
        pxa_ssp_write_reg(ssp, SSPSP, sspsp);
        pxa_ssp_write_reg(ssp, SSCR0, sscr0 | SSCR0_SSE);
    }
}

unsafe extern "C" fn pxa_ssp_trigger(substream: *mut snd_pcm_substream, cmd: c_int, cpu_dai: *mut snd_soc_dai) -> c_int {
    let mut ret: c_int = 0;
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai) as *mut ssp_priv;
    let ssp = (*priv_).ssp;
    let val: c_int;

    if cmd == SNDRV_PCM_TRIGGER_RESUME {
        pxa_ssp_enable(ssp);
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE {
        pxa_ssp_set_running_bit(substream, ssp, 1);
        val = pxa_ssp_read_reg(ssp, SSSR) as c_int;
        pxa_ssp_write_reg(ssp, SSSR, val as u32);
    } else if cmd == SNDRV_PCM_TRIGGER_START {
        pxa_ssp_set_running_bit(substream, ssp, 1);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP {
        pxa_ssp_set_running_bit(substream, ssp, 0);
    } else if cmd == SNDRV_PCM_TRIGGER_SUSPEND {
        pxa_ssp_disable(ssp);
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH {
        pxa_ssp_set_running_bit(substream, ssp, 0);
    } else {
        ret = -EINVAL;
    }

    dump_registers(ssp);

    ret
}

unsafe extern "C" fn pxa_ssp_probe(dai: *mut snd_soc_dai) -> c_int {
    let dev = (*dai).dev;
    let priv_: *mut ssp_priv;
    let mut ret: c_int;

    priv_ = kzalloc_obj::<ssp_priv>();
    if priv_.is_null() {
        return -ENOMEM;
    }

    if !(*dev).of_node.is_null() {
        let ssp_handle: *mut device_node;

        ssp_handle = of_parse_phandle((*dev).of_node, b"port\0".as_ptr() as *const c_char, 0);
        if ssp_handle.is_null() {
            dev_err(dev, b"unable to get 'port' phandle\n\0".as_ptr() as *const c_char);
            ret = -ENODEV;
            kfree(priv_ as *mut c_void);
            return ret;
        }

        (*priv_).ssp = pxa_ssp_request_of(ssp_handle, b"SoC audio\0".as_ptr() as *const c_char);
        if (*priv_).ssp.is_null() {
            ret = -ENODEV;
            kfree(priv_ as *mut c_void);
            return ret;
        }

        (*priv_).extclk = devm_clk_get_optional(dev, b"extclk\0".as_ptr() as *const c_char);
        if IS_ERR((*priv_).extclk as *const c_void) {
            ret = PTR_ERR((*priv_).extclk as *const c_void);
            kfree(priv_ as *mut c_void);
            return ret;
        }
    } else {
        (*priv_).ssp = pxa_ssp_request((*dai).id + 1, b"SoC audio\0".as_ptr() as *const c_char);
        if (*priv_).ssp.is_null() {
            ret = -ENODEV;
            kfree(priv_ as *mut c_void);
            return ret;
        }
    }

    (*priv_).dai_fmt = -1i32 as c_uint;
    snd_soc_dai_set_drvdata(dai, priv_ as *mut c_void);

    0
}

unsafe extern "C" fn pxa_ssp_remove(dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut ssp_priv;

    pxa_ssp_free((*priv_).ssp);
    kfree(priv_ as *mut c_void);
    0
}

unsafe fn PXA_SSP_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000
        | SNDRV_PCM_RATE_11025
        | SNDRV_PCM_RATE_16000
        | SNDRV_PCM_RATE_22050
        | SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_64000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
}

unsafe fn PXA_SSP_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static pxa_ssp_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(pxa_ssp_probe),
    remove: Some(pxa_ssp_remove),
    startup: Some(pxa_ssp_startup),
    shutdown: Some(pxa_ssp_shutdown),
    trigger: Some(pxa_ssp_trigger),
    hw_params: Some(pxa_ssp_hw_params),
    set_sysclk: Some(pxa_ssp_set_dai_sysclk),
    set_fmt: Some(pxa_ssp_set_dai_fmt),
    set_tdm_slot: Some(pxa_ssp_set_dai_tdm_slot),
    set_tristate: Some(pxa_ssp_set_dai_tristate),
};

static mut pxa_ssp_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 8,
        rates: unsafe { PXA_SSP_RATES() },
        formats: unsafe { PXA_SSP_FORMATS() },
    },
    capture: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 8,
        rates: unsafe { PXA_SSP_RATES() },
        formats: unsafe { PXA_SSP_FORMATS() },
    },
    ops: &pxa_ssp_dai_ops,
};

static pxa_ssp_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"pxa-ssp\0".as_ptr() as *const c_char,
    pcm_new: Some(pxa2xx_soc_pcm_new),
    open: Some(pxa2xx_soc_pcm_open),
    close: Some(pxa2xx_soc_pcm_close),
    hw_params: Some(pxa2xx_soc_pcm_hw_params),
    prepare: Some(pxa2xx_soc_pcm_prepare),
    trigger: Some(pxa2xx_soc_pcm_trigger),
    pointer: Some(pxa2xx_soc_pcm_pointer),
    suspend: Some(pxa_ssp_suspend),
    resume: Some(pxa_ssp_resume),
    legacy_dai_naming: 1,
};

/* CONFIG_OF */
static pxa_ssp_of_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mrvl,pxa-ssp-dai\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, pxa_ssp_of_ids); */

unsafe extern "C" fn asoc_ssp_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(&mut (*pdev).dev, &pxa_ssp_component, &mut pxa_ssp_dai, 1)
}

static mut asoc_ssp_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"pxa-ssp-dai\0".as_ptr() as *const c_char,
        of_match_table: pxa_ssp_of_ids.as_ptr(),
    },
    probe: Some(asoc_ssp_probe),
};

/* module_platform_driver(asoc_ssp_driver); */

/* Module information */
/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_DESCRIPTION("PXA SSP/PCM SoC Interface"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:pxa-ssp-dai"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
