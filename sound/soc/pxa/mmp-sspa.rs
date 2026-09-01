// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * linux/sound/soc/pxa/mmp-sspa.c
 * Base on pxa2xx-ssp.c
 *
 * Copyright (C) 2011 Marvell International Ltd.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u32 = u32;
type dma_addr_t = c_ulong;
type resource_size_t = c_ulong;
type pgprot_t = c_ulong;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
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
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_pcm_dma_buffer {
    pub addr: dma_addr_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub dma_buffer: snd_pcm_dma_buffer,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: c_ulong,
    pub vm_end: c_ulong,
    pub vm_page_prot: pgprot_t,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_pll:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_ulong,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<unsafe extern "C" fn()>,
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub mmap: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut vm_area_struct,
        ) -> c_int,
    >,
    pub open:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

/*
 * SSPA audio private data
 */
#[repr(C)]
pub struct sspa_priv {
    pub tx_base: *mut c_void,
    pub rx_base: *mut c_void,

    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub clk: *mut clk,
    pub audio_clk: *mut clk,
    pub sysclk: *mut clk,

    pub running_cnt: c_int,
    pub sp: u32,
    pub ctrl: u32,
}

unsafe extern "C" {
    static snd_dmaengine_pcm_prepare_slave_config: unsafe extern "C" fn();

    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);

    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: c_uint,
        num: c_uint,
    ) -> *mut resource;
    fn devm_ioremap(dev: *mut device, offset: resource_size_t, size: c_ulong) -> *mut c_void;

    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;

    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);

    fn vm_flags_set(vma: *mut vm_area_struct, flags: c_ulong);
    fn pgprot_noncached(prot: pgprot_t) -> pgprot_t;
    fn remap_pfn_range(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        pfn: c_ulong,
        size: c_ulong,
        prot: pgprot_t,
    ) -> c_int;

    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_match_ptr(match_table: *const of_device_id) -> *const of_device_id;
}

unsafe fn __raw_writel(value: c_uint, addr: *mut c_void) {
    ptr::write_volatile(addr as *mut c_uint, value);
}

unsafe fn __raw_readl(addr: *mut c_void) -> c_uint {
    ptr::read_volatile(addr as *const c_uint)
}

unsafe fn offset(base: *mut c_void, off: c_ulong) -> *mut c_void {
    (base as *mut u8).add(off as usize) as *mut c_void
}

const ENOTSUPP: c_int = 524;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;

const GFP_KERNEL: c_uint = 0;
const IORESOURCE_IO: c_uint = 0x00000100;
const PAGE_SHIFT: c_uint = 12;
const VM_DONTEXPAND: c_ulong = 0x0004_0000;
const VM_DONTDUMP: c_ulong = 0x0400_0000;

const SSPA_SP: c_ulong = 0;
const SSPA_CTL: c_ulong = 4;
const SSPA_FIFO_UL: c_ulong = 8;
const SSPA_D: c_ulong = 0x10;

const SSPA_SP_MSL: c_uint = 1 << 0;
const SSPA_SP_S_EN: c_uint = 1 << 1;
const SSPA_SP_WEN: c_uint = 1 << 2;
const SSPA_SP_S_RST: c_uint = 1 << 3;
const SSPA_SP_FFLUSH: c_uint = 1 << 4;
const SSPA_SP_FSP: c_uint = 1 << 5;
const SSPA_SP_FWID_MASK: c_uint = 0xff << 8;
const SSPA_TXSP_FPER_MASK: c_uint = 0xff << 16;

const SSPA_CTL_XPH: c_uint = 1 << 0;
const SSPA_CTL_XWDLEN1_MASK: c_uint = 0x7 << 4;
const SSPA_CTL_XWDLEN2_MASK: c_uint = 0x7 << 8;
const SSPA_CTL_XSSZ1_MASK: c_uint = 0x7 << 12;
const SSPA_CTL_XSSZ2_MASK: c_uint = 0x7 << 16;

const SSPA_CTL_8_BITS: c_int = 0;
const SSPA_CTL_16_BITS: c_int = 1;
const SSPA_CTL_24_BITS: c_int = 2;
const SSPA_CTL_32_BITS: c_int = 3;

const MMP_SSPA_CLK_AUDIO: c_int = 0;
const MMP_SSPA_CLK_PLL: c_int = 1;
const MMP_SSPA_CLK_VCXO: c_int = 2;
const MMP_SYSCLK: c_int = 3;
const MMP_SSPA_CLK: c_int = 4;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x1000;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x2000;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0100;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x00ff;
const SND_SOC_DAIFMT_I2S: c_uint = 1;

const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S24_3LE: c_int = 6;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: c_ulong = 1 << SNDRV_PCM_FORMAT_S8;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S24_3LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S24_3LE;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << SNDRV_PCM_FORMAT_S32_LE;

const SNDRV_PCM_INFO_MMAP: c_ulong = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_ulong = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_ulong = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: c_ulong = 1 << 3;
const SNDRV_PCM_INFO_RESUME: c_ulong = 1 << 4;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: c_ulong = 1 << 5;

const fn SSPA_CTL_XDATDLY(x: c_uint) -> c_uint {
    x << 20
}

const fn SSPA_CTL_XWDLEN1(x: c_int) -> c_uint {
    (x as c_uint) << 4
}

const fn SSPA_CTL_XWDLEN2(x: c_int) -> c_uint {
    (x as c_uint) << 8
}

const fn SSPA_CTL_XSSZ1(x: c_int) -> c_uint {
    (x as c_uint) << 12
}

const fn SSPA_CTL_XSSZ2(x: c_int) -> c_uint {
    (x as c_uint) << 16
}

const fn SSPA_SP_FWID(x: c_int) -> c_uint {
    (x as c_uint) << 8
}

const fn SSPA_TXSP_FPER(x: c_int) -> c_uint {
    (x as c_uint) << 16
}

unsafe extern "C" fn mmp_sspa_tx_enable(sspa: *mut sspa_priv) {
    let mut sspa_sp: c_uint = (*sspa).sp;

    sspa_sp &= !SSPA_SP_MSL;
    sspa_sp |= SSPA_SP_S_EN;
    sspa_sp |= SSPA_SP_WEN;
    __raw_writel(sspa_sp, offset((*sspa).tx_base, SSPA_SP));
}

unsafe extern "C" fn mmp_sspa_tx_disable(sspa: *mut sspa_priv) {
    let mut sspa_sp: c_uint = (*sspa).sp;

    sspa_sp &= !SSPA_SP_MSL;
    sspa_sp &= !SSPA_SP_S_EN;
    sspa_sp |= SSPA_SP_WEN;
    __raw_writel(sspa_sp, offset((*sspa).tx_base, SSPA_SP));
}

unsafe extern "C" fn mmp_sspa_rx_enable(sspa: *mut sspa_priv) {
    let mut sspa_sp: c_uint = (*sspa).sp;

    sspa_sp |= SSPA_SP_S_EN;
    sspa_sp |= SSPA_SP_WEN;
    __raw_writel(sspa_sp, offset((*sspa).rx_base, SSPA_SP));
}

unsafe extern "C" fn mmp_sspa_rx_disable(sspa: *mut sspa_priv) {
    let mut sspa_sp: c_uint = (*sspa).sp;

    sspa_sp &= !SSPA_SP_S_EN;
    sspa_sp |= SSPA_SP_WEN;
    __raw_writel(sspa_sp, offset((*sspa).rx_base, SSPA_SP));
}

unsafe extern "C" fn mmp_sspa_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let sspa = snd_soc_dai_get_drvdata(dai) as *mut sspa_priv;

    clk_prepare_enable((*sspa).sysclk);
    clk_prepare_enable((*sspa).clk);

    0
}

unsafe extern "C" fn mmp_sspa_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let sspa = snd_soc_dai_get_drvdata(dai) as *mut sspa_priv;

    clk_disable_unprepare((*sspa).clk);
    clk_disable_unprepare((*sspa).sysclk);
}

/*
 * Set the SSP ports SYSCLK.
 */
unsafe extern "C" fn mmp_sspa_set_dai_sysclk(
    cpu_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let sspa = snd_soc_dai_get_drvdata(cpu_dai) as *mut sspa_priv;
    let dev = (*(*cpu_dai).component).dev;
    let mut ret: c_int = 0;

    if !(*dev).of_node.is_null() {
        return -ENOTSUPP;
    }

    match clk_id {
        MMP_SSPA_CLK_AUDIO => {
            ret = clk_set_rate((*sspa).audio_clk, freq);
            if ret != 0 {
                return ret;
            }
        }
        MMP_SSPA_CLK_PLL | MMP_SSPA_CLK_VCXO => {
            /* not support yet */
            return -EINVAL;
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn mmp_sspa_set_dai_pll(
    cpu_dai: *mut snd_soc_dai,
    pll_id: c_int,
    _source: c_int,
    _freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let sspa = snd_soc_dai_get_drvdata(cpu_dai) as *mut sspa_priv;
    let dev = (*(*cpu_dai).component).dev;
    let mut ret: c_int = 0;

    if !(*dev).of_node.is_null() {
        return -ENOTSUPP;
    }

    match pll_id {
        MMP_SYSCLK => {
            ret = clk_set_rate((*sspa).sysclk, freq_out);
            if ret != 0 {
                return ret;
            }
        }
        MMP_SSPA_CLK => {
            ret = clk_set_rate((*sspa).clk, freq_out);
            if ret != 0 {
                return ret;
            }
        }
        _ => return -ENODEV,
    }

    0
}

/*
 * Set up the sspa dai format.
 */
unsafe extern "C" fn mmp_sspa_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let sspa = snd_soc_dai_get_drvdata(cpu_dai) as *mut sspa_priv;

    /* reset port settings */
    (*sspa).sp = SSPA_SP_WEN | SSPA_SP_S_RST | SSPA_SP_FFLUSH;
    (*sspa).ctrl = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            (*sspa).sp |= SSPA_SP_MSL;
        }
        SND_SOC_DAIFMT_BC_FC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            (*sspa).sp |= SSPA_SP_FSP;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            (*sspa).ctrl |= SSPA_CTL_XDATDLY(1);
        }
        _ => return -EINVAL,
    }

    /* Since we are configuring the timings for the format by hand
     * we have to defer some things until hw_params() where we
     * know parameters like the sample size.
     */
    0
}

/*
 * Set the SSPA audio DMA parameters and sample size.
 * Can be called multiple times by oss emulation.
 */
unsafe extern "C" fn mmp_sspa_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let sspa = snd_soc_dai_get_drvdata(dai) as *mut sspa_priv;
    let dev = (*(*dai).component).dev;
    let mut sspa_ctrl: u32 = (*sspa).ctrl;
    let bits: c_int;
    let bitval: c_int;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => {
            bits = 8;
            bitval = SSPA_CTL_8_BITS;
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            bits = 16;
            bitval = SSPA_CTL_16_BITS;
        }
        SNDRV_PCM_FORMAT_S24_3LE => {
            bits = 24;
            bitval = SSPA_CTL_24_BITS;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            bits = 32;
            bitval = SSPA_CTL_32_BITS;
        }
        _ => return -EINVAL,
    }

    sspa_ctrl &= !SSPA_CTL_XPH;
    if !(*dev).of_node.is_null() || params_channels(params) == 2 {
        sspa_ctrl |= SSPA_CTL_XPH;
    }

    sspa_ctrl &= !SSPA_CTL_XWDLEN1_MASK;
    sspa_ctrl |= SSPA_CTL_XWDLEN1(bitval);

    sspa_ctrl &= !SSPA_CTL_XWDLEN2_MASK;
    sspa_ctrl |= SSPA_CTL_XWDLEN2(bitval);

    sspa_ctrl &= !SSPA_CTL_XSSZ1_MASK;
    sspa_ctrl |= SSPA_CTL_XSSZ1(bitval);

    sspa_ctrl &= !SSPA_CTL_XSSZ2_MASK;
    sspa_ctrl |= SSPA_CTL_XSSZ2(bitval);

    (*sspa).sp &= !SSPA_SP_FWID_MASK;
    (*sspa).sp |= SSPA_SP_FWID(bits - 1);

    (*sspa).sp &= !SSPA_TXSP_FPER_MASK;
    (*sspa).sp |= SSPA_TXSP_FPER(bits * 2 - 1);

    if !(*dev).of_node.is_null() {
        clk_set_rate(
            (*sspa).clk,
            params_rate(params) * params_channels(params) * bits as c_uint,
        );
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        __raw_writel(sspa_ctrl, offset((*sspa).tx_base, SSPA_CTL));
        __raw_writel(0x1, offset((*sspa).tx_base, SSPA_FIFO_UL));
    } else {
        __raw_writel(sspa_ctrl, offset((*sspa).rx_base, SSPA_CTL));
        __raw_writel(0x0, offset((*sspa).rx_base, SSPA_FIFO_UL));
    }

    0
}

unsafe extern "C" fn mmp_sspa_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let sspa = snd_soc_dai_get_drvdata(dai) as *mut sspa_priv;
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            /*
             * whatever playback or capture, must enable rx.
             * this is a hw issue, so need check if rx has been
             * enabled or not; if has been enabled by another
             * stream, do not enable again.
             */
            if (*sspa).running_cnt == 0 {
                mmp_sspa_rx_enable(sspa);
            }

            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                mmp_sspa_tx_enable(sspa);
            }

            (*sspa).running_cnt += 1;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            (*sspa).running_cnt -= 1;

            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                mmp_sspa_tx_disable(sspa);
            }

            /* have no capture stream, disable rx port */
            if (*sspa).running_cnt == 0 {
                mmp_sspa_rx_disable(sspa);
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn mmp_sspa_probe(dai: *mut snd_soc_dai) -> c_int {
    let sspa = dev_get_drvdata((*dai).dev) as *mut sspa_priv;

    snd_soc_dai_init_dma_data(
        dai,
        &mut (*sspa).playback_dma_data,
        &mut (*sspa).capture_dma_data,
    );

    0
}

const MMP_SSPA_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const MMP_SSPA_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static mmp_sspa_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(mmp_sspa_probe),
    startup: Some(mmp_sspa_startup),
    shutdown: Some(mmp_sspa_shutdown),
    trigger: Some(mmp_sspa_trigger),
    hw_params: Some(mmp_sspa_hw_params),
    set_sysclk: Some(mmp_sspa_set_dai_sysclk),
    set_pll: Some(mmp_sspa_set_dai_pll),
    set_fmt: Some(mmp_sspa_set_dai_fmt),
};

static mut mmp_sspa_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 128,
        rates: MMP_SSPA_RATES,
        formats: MMP_SSPA_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 2,
        rates: MMP_SSPA_RATES,
        formats: MMP_SSPA_FORMATS,
    },
    ops: &mmp_sspa_dai_ops,
};

const MMP_PCM_INFO: c_ulong = SNDRV_PCM_INFO_MMAP
    | SNDRV_PCM_INFO_MMAP_VALID
    | SNDRV_PCM_INFO_INTERLEAVED
    | SNDRV_PCM_INFO_PAUSE
    | SNDRV_PCM_INFO_RESUME
    | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP;

static mmp_pcm_hardware: [snd_pcm_hardware; 2] = [
    snd_pcm_hardware {
        info: MMP_PCM_INFO,
        period_bytes_min: 1024,
        period_bytes_max: 2048,
        periods_min: 2,
        periods_max: 32,
        buffer_bytes_max: 4096,
        fifo_size: 32,
    },
    snd_pcm_hardware {
        info: MMP_PCM_INFO,
        period_bytes_min: 1024,
        period_bytes_max: 2048,
        periods_min: 2,
        periods_max: 32,
        buffer_bytes_max: 4096,
        fifo_size: 32,
    },
];

static mmp_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    pcm_hardware: mmp_pcm_hardware.as_ptr(),
    prealloc_buffer_size: 4096,
};

unsafe extern "C" fn mmp_pcm_mmap(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    vma: *mut vm_area_struct,
) -> c_int {
    vm_flags_set(vma, VM_DONTEXPAND | VM_DONTDUMP);
    (*vma).vm_page_prot = pgprot_noncached((*vma).vm_page_prot);
    remap_pfn_range(
        vma,
        (*vma).vm_start,
        (*substream).dma_buffer.addr >> PAGE_SHIFT,
        (*vma).vm_end - (*vma).vm_start,
        (*vma).vm_page_prot,
    )
}

unsafe extern "C" fn mmp_sspa_open(
    component: *mut snd_soc_component,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    let sspa = snd_soc_component_get_drvdata(component) as *mut sspa_priv;

    pm_runtime_get_sync((*component).dev);

    /* we can only change the settings if the port is not in use */
    if (__raw_readl(offset((*sspa).tx_base, SSPA_SP)) & SSPA_SP_S_EN) != 0
        || (__raw_readl(offset((*sspa).rx_base, SSPA_SP)) & SSPA_SP_S_EN) != 0
    {
        dev_err(
            (*component).dev,
            b"can't change hardware dai format: stream is in use\n\0".as_ptr() as *const c_char,
        );
        return -EBUSY;
    }

    __raw_writel((*sspa).sp, offset((*sspa).tx_base, SSPA_SP));
    __raw_writel((*sspa).sp, offset((*sspa).rx_base, SSPA_SP));

    (*sspa).sp &= !(SSPA_SP_S_RST | SSPA_SP_FFLUSH);
    __raw_writel((*sspa).sp, offset((*sspa).tx_base, SSPA_SP));
    __raw_writel((*sspa).sp, offset((*sspa).rx_base, SSPA_SP));

    /*
     * FIXME: hw issue, for the tx serial port,
     * can not config the master/slave mode;
     * so must clean this bit.
     * The master/slave mode has been set in the
     * rx port.
     */
    __raw_writel((*sspa).sp & !SSPA_SP_MSL, offset((*sspa).tx_base, SSPA_SP));

    __raw_writel((*sspa).ctrl, offset((*sspa).tx_base, SSPA_CTL));
    __raw_writel((*sspa).ctrl, offset((*sspa).rx_base, SSPA_CTL));

    0
}

unsafe extern "C" fn mmp_sspa_close(
    component: *mut snd_soc_component,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    pm_runtime_put_sync((*component).dev);
    0
}

static mmp_sspa_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"mmp-sspa\0".as_ptr() as *const c_char,
    mmap: Some(mmp_pcm_mmap),
    open: Some(mmp_sspa_open),
    close: Some(mmp_sspa_close),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn asoc_mmp_sspa_probe(pdev: *mut platform_device) -> c_int {
    let sspa: *mut sspa_priv;
    let mut ret: c_int;

    sspa = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<sspa_priv>(),
        GFP_KERNEL,
    ) as *mut sspa_priv;
    if sspa.is_null() {
        return -ENOMEM;
    }

    if !(*pdev).dev.of_node.is_null() {
        (*sspa).rx_base = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR((*sspa).rx_base) {
            return PTR_ERR((*sspa).rx_base);
        }

        (*sspa).tx_base = devm_platform_ioremap_resource(pdev, 1);
        if IS_ERR((*sspa).tx_base) {
            return PTR_ERR((*sspa).tx_base);
        }

        (*sspa).clk = devm_clk_get(&mut (*pdev).dev, b"bitclk\0".as_ptr() as *const c_char);
        if IS_ERR((*sspa).clk as *const c_void) {
            return PTR_ERR((*sspa).clk as *const c_void);
        }

        (*sspa).audio_clk =
            devm_clk_get(&mut (*pdev).dev, b"audio\0".as_ptr() as *const c_char);
        if IS_ERR((*sspa).audio_clk as *const c_void) {
            return PTR_ERR((*sspa).audio_clk as *const c_void);
        }
    } else {
        let res: *mut resource;

        res = platform_get_resource(pdev, IORESOURCE_IO, 0);
        if res.is_null() {
            return -ENODEV;
        }

        (*sspa).rx_base = devm_ioremap(&mut (*pdev).dev, (*res).start, 0x30);
        if (*sspa).rx_base.is_null() {
            return -ENOMEM;
        }

        (*sspa).tx_base = devm_ioremap(&mut (*pdev).dev, (*res).start + 0x80, 0x30);
        if (*sspa).tx_base.is_null() {
            return -ENOMEM;
        }

        (*sspa).clk = devm_clk_get(&mut (*pdev).dev, ptr::null());
        if IS_ERR((*sspa).clk as *const c_void) {
            return PTR_ERR((*sspa).clk as *const c_void);
        }

        (*sspa).audio_clk = clk_get(ptr::null_mut(), b"mmp-audio\0".as_ptr() as *const c_char);
        if IS_ERR((*sspa).audio_clk as *const c_void) {
            return PTR_ERR((*sspa).audio_clk as *const c_void);
        }

        (*sspa).sysclk = clk_get(ptr::null_mut(), b"mmp-sysclk\0".as_ptr() as *const c_char);
        if IS_ERR((*sspa).sysclk as *const c_void) {
            clk_put((*sspa).audio_clk);
            return PTR_ERR((*sspa).sysclk as *const c_void);
        }
    }
    platform_set_drvdata(pdev, sspa as *mut c_void);

    (*sspa).playback_dma_data.maxburst = 4;
    (*sspa).capture_dma_data.maxburst = 4;
    /* You know, these addresses are actually ignored. */
    (*sspa).capture_dma_data.addr = SSPA_D;
    (*sspa).playback_dma_data.addr = 0x80 + SSPA_D;

    if !(*pdev).dev.of_node.is_null() {
        ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, &mmp_pcm_config, 0);
        if ret != 0 {
            return ret;
        }
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &mmp_sspa_component,
        &raw mut mmp_sspa_dai,
        1,
    );
    if ret != 0 {
        return ret;
    }

    pm_runtime_enable(&mut (*pdev).dev);
    clk_prepare_enable((*sspa).audio_clk);

    0
}

unsafe extern "C" fn asoc_mmp_sspa_remove(pdev: *mut platform_device) {
    let sspa = platform_get_drvdata(pdev) as *mut sspa_priv;

    clk_disable_unprepare((*sspa).audio_clk);
    pm_runtime_disable(&mut (*pdev).dev);

    if !(*pdev).dev.of_node.is_null() {
        return;
    }

    clk_put((*sspa).audio_clk);
    clk_put((*sspa).sysclk);
}

/* CONFIG_OF */
static mmp_sspa_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"marvell,mmp-sspa\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

static mut asoc_mmp_sspa_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: b"mmp-sspa-dai\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(mmp_sspa_of_match.as_ptr()) },
    },
    probe: Some(asoc_mmp_sspa_probe),
    remove: Some(asoc_mmp_sspa_remove),
};

/* module_platform_driver(asoc_mmp_sspa_driver); */

/* MODULE_AUTHOR("Leo Yan <leoy@marvell.com>"); */
/* MODULE_DESCRIPTION("MMP SSPA SoC Interface"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:mmp-sspa-dai"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
