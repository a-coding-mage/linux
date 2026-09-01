// SPDX-License-Identifier: GPL-2.0-only
// omap-mcbsp.rs  -- OMAP ALSA SoC DAI driver using McBSP port
//
// Copyright (C) 2008 Nokia Corporation
//
// Contact: Jarkko Nikula <jarkko.nikula@bitmer.com>
//          Peter Ujfalusi <peter.ujfalusi@ti.com>

// Dependencies: linux kernel audio subsystem (ALSA SoC)
// Includes: linux/init.h, linux/module.h, linux/device.h, linux/pm_runtime.h,
//           linux/of.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
//           sound/initval.h, sound/soc.h, sound/dmaengine_pcm.h
// Internal headers: omap-mcbsp-priv.h, omap-mcbsp.h, sdma-pcm.h

const OMAP_MCBSP_RATES: u32 = 0; // SNDRV_PCM_RATE_8000_96000

#[repr(u32)]
pub enum OmapMcbspWord {
    Word8 = 0,
    Word12 = 1,
    Word16 = 2,
    Word20 = 3,
    Word24 = 4,
    Word32 = 5,
}

// External structures and types from dependencies
pub struct OmapMcbsp {
    pub dev: *mut std::ffi::c_void,
    pub id: i32,
    pub io_base: *mut std::ffi::c_void,
    pub phys_base: usize,
    pub phys_dma_base: usize,
    pub reg_cache_size: usize,
    pub reg_cache: *mut std::ffi::c_void,
    pub irq: i32,
    pub tx_irq: i32,
    pub rx_irq: i32,
    pub fclk: *mut std::ffi::c_void,
    pub lock: std::sync::Mutex<()>,
    pub free: bool,
    pub active: i32,
    pub configured: i32,
    pub st_data: *mut std::ffi::c_void,
    pub pdata: *const OmapMcbspPlatformData,
    pub fmt: u32,
    pub clk_div: i32,
    pub in_freq: u32,
    pub wlen: i32,
    pub max_tx_thres: u16,
    pub max_rx_thres: u16,
    pub dma_op_mode: i32,
    pub dma_data: [SndDmaengineDaiDmaData; 2],
    pub dma_req: [u32; 2],
    pub cfg_regs: OmapMcbspRegCfg,
    pub latency: [i32; 2],
    pub pm_qos_req: std::ffi::c_void,
}

pub struct OmapMcbspRegCfg {
    pub spcr2: u16,
    pub spcr1: u16,
    pub rcr2: u16,
    pub rcr1: u16,
    pub xcr2: u16,
    pub xcr1: u16,
    pub srgr2: u16,
    pub srgr1: u16,
    pub mcr2: u16,
    pub mcr1: u16,
    pub pcr0: u16,
    pub rccr: u16,
    pub xccr: u16,
}

pub struct OmapMcbspPlatformData {
    pub reg_step: u32,
    pub reg_size: u32,
    pub has_ccr: bool,
    pub has_wakeup: bool,
    pub buffer_size: u32,
    pub force_ick_on: bool,
    pub ops: *const OmapMcbspOps,
}

pub struct OmapMcbspOps {
    pub request: Option<extern "C" fn(u32) -> i32>,
    pub free: Option<extern "C" fn(u32) -> i32>,
}

pub struct SndDmaengineDaiDmaData {
    pub filter_data: *mut std::ffi::c_void,
    pub addr: usize,
    pub maxburst: u32,
}

// Macro helpers for register access
// MCBSP_READ, MCBSP_WRITE, MCBSP_READ_CACHE are external macros
// MCBSP_WRITE(mcbsp, reg, val) - write to register
// MCBSP_READ(mcbsp, reg) -> u16 - read from register
// MCBSP_READ_CACHE(mcbsp, reg) -> u16 - read from cache

// External functions to be declared
extern "C" {
    fn dev_dbg(dev: *mut std::ffi::c_void, fmt: *const u8, ...);
    fn dev_err(dev: *mut std::ffi::c_void, fmt: *const u8, ...);
    fn dev_info(dev: *mut std::ffi::c_void, fmt: *const u8, ...);
    fn clk_get(dev: *mut std::ffi::c_void, id: *const u8) -> *mut std::ffi::c_void;
    fn clk_put(clk: *mut std::ffi::c_void);
    fn clk_set_parent(clk: *mut std::ffi::c_void, parent: *mut std::ffi::c_void) -> i32;
    fn pm_runtime_put_sync(dev: *mut std::ffi::c_void) -> i32;
    fn pm_runtime_get_sync(dev: *mut std::ffi::c_void) -> i32;
    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut std::ffi::c_void) -> i32,
        flags: u32,
        name: *const u8,
        dev: *mut std::ffi::c_void,
    ) -> i32;
    fn free_irq(irq: u32, dev: *mut std::ffi::c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut std::ffi::c_void;
    fn kfree(ptr: *mut std::ffi::c_void);
    fn IS_ERR(ptr: *const std::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const std::ffi::c_void) -> i32;
    fn platform_get_drvdata(pdev: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut std::ffi::c_void, data: *mut std::ffi::c_void);
    fn platform_get_resource(
        pdev: *mut std::ffi::c_void,
        typ: u32,
        num: u32,
    ) -> *mut std::ffi::c_void;
    fn platform_get_resource_byname(
        pdev: *mut std::ffi::c_void,
        typ: u32,
        name: *const u8,
    ) -> *mut std::ffi::c_void;
    fn platform_get_irq_byname(pdev: *mut std::ffi::c_void, name: *const u8) -> i32;
    fn platform_get_irq(pdev: *mut std::ffi::c_void, num: u32) -> i32;
    fn devm_ioremap_resource(dev: *mut std::ffi::c_void, res: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn devm_clk_get(dev: *mut std::ffi::c_void, id: *const u8) -> *mut std::ffi::c_void;
    fn devm_kzalloc(dev: *mut std::ffi::c_void, size: usize, flags: u32) -> *mut std::ffi::c_void;
    fn devm_kmemdup(
        dev: *mut std::ffi::c_void,
        src: *const std::ffi::c_void,
        size: usize,
        flags: u32,
    ) -> *mut std::ffi::c_void;
    fn devm_device_add_group(
        dev: *mut std::ffi::c_void,
        grp: *const std::ffi::c_void,
    ) -> i32;
    fn devm_snd_soc_register_component(
        dev: *mut std::ffi::c_void,
        component: *const std::ffi::c_void,
        dai: *const std::ffi::c_void,
        num_dai: i32,
    ) -> i32;
    fn pm_runtime_enable(dev: *mut std::ffi::c_void);
    fn pm_runtime_disable(dev: *mut std::ffi::c_void);
    fn spin_lock_init(lock: *mut std::ffi::c_void);
    fn snd_soc_dai_init_dma_data(
        dai: *mut std::ffi::c_void,
        playback: *const std::ffi::c_void,
        capture: *const std::ffi::c_void,
    );
    fn snd_soc_dai_get_drvdata(dai: *const std::ffi::c_void) -> *mut std::ffi::c_void;
    fn snd_soc_substream_to_rtd(substream: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn snd_soc_rtd_to_cpu(rtd: *mut std::ffi::c_void, num: u32) -> *mut std::ffi::c_void;
    fn snd_soc_dai_get_dma_data(
        dai: *const std::ffi::c_void,
        substream: *mut std::ffi::c_void,
    ) -> *mut SndDmaengineDaiDmaData;
    fn snd_soc_dai_active(dai: *const std::ffi::c_void) -> i32;
    fn snd_pcm_hw_rule_add(
        runtime: *mut std::ffi::c_void,
        cond: u32,
        var: u32,
        func: unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32,
        private: *mut std::ffi::c_void,
        dep: u32,
        rest: i32,
    ) -> i32;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut std::ffi::c_void,
        cond: u32,
        var: u32,
        step: u32,
    ) -> i32;
    fn hw_param_interval(
        params: *mut std::ffi::c_void,
        var: u32,
    ) -> *mut std::ffi::c_void;
    fn snd_interval_any(interval: *mut std::ffi::c_void);
    fn snd_interval_refine(
        interval: *mut std::ffi::c_void,
        v: *const std::ffi::c_void,
    ) -> i32;
    fn params_channels(params: *const std::ffi::c_void) -> u32;
    fn params_format(params: *const std::ffi::c_void) -> u32;
    fn params_period_bytes(params: *const std::ffi::c_void) -> u32;
    fn params_rate(params: *const std::ffi::c_void) -> u32;
    fn sysfs_emit(buf: *mut u8, fmt: *const u8, ...) -> i32;
    fn sysfs_emit_at(buf: *mut u8, offset: usize, fmt: *const u8, ...) -> i32;
    fn kstrtoul(s: *const u8, base: i32, res: *mut usize) -> i32;
    fn sysfs_match_string(array: *const *const u8, string: *const u8) -> i32;
    fn resource_size(res: *const std::ffi::c_void) -> usize;
    fn of_property_read_u32(
        np: *const std::ffi::c_void,
        propname: *const u8,
        out_value: *mut u32,
    ) -> i32;
    fn device_get_match_data(dev: *const std::ffi::c_void) -> *const std::ffi::c_void;
    fn dev_get_platdata(dev: *const std::ffi::c_void) -> *mut std::ffi::c_void;
    fn dev_get_drvdata(dev: *const std::ffi::c_void) -> *mut std::ffi::c_void;
    fn sdma_pcm_platform_register(dev: *mut std::ffi::c_void, tx: *const u8, rx: *const u8) -> i32;
    fn omap_mcbsp_st_start(mcbsp: *mut OmapMcbsp);
    fn omap_mcbsp_st_stop(mcbsp: *mut OmapMcbsp);
    fn omap_mcbsp_st_init(pdev: *mut std::ffi::c_void) -> i32;
    fn mcbsp_omap1() -> bool;
    fn printk(fmt: *const u8, ...);
    fn udelay(usecs: u32);
}

unsafe fn omap_mcbsp_dump_reg(mcbsp: *mut OmapMcbsp) {
    // dev_dbg calls with register reads
    // Note: MCBSP_READ macro would need to be defined
}

unsafe fn omap2_mcbsp_set_clks_src(mcbsp: *mut OmapMcbsp, fck_src_id: u8) -> i32 {
    let src: *const u8 = if fck_src_id == 0 {
        b"pad_fck\0" as *const u8
    } else if fck_src_id == 1 {
        b"prcm_fck\0" as *const u8
    } else {
        return -22; // -EINVAL
    };

    let fck_src = clk_get((*mcbsp).dev, src);
    if IS_ERR(fck_src as *const std::ffi::c_void) {
        dev_info((*mcbsp).dev, b"CLKS: could not clk_get() %s\0" as *const u8, src);
        return 0;
    }

    if (*mcbsp).active != 0 {
        pm_runtime_put_sync((*mcbsp).dev);
    }

    let r = clk_set_parent((*mcbsp).fclk, fck_src);
    if r != 0 {
        dev_err((*mcbsp).dev, b"CLKS: could not clk_set_parent() to %s\0" as *const u8, src);
    }

    if (*mcbsp).active != 0 {
        pm_runtime_get_sync((*mcbsp).dev);
    }

    clk_put(fck_src);

    r
}

unsafe extern "C" fn omap_mcbsp_irq_handler(irq: i32, data: *mut std::ffi::c_void) -> i32 {
    let mcbsp = data as *mut OmapMcbsp;
    // irqst = MCBSP_READ(mcbsp, IRQST);
    // dev_dbg calls with various IRQ status checks
    // MCBSP_WRITE(mcbsp, IRQST, irqst);
    2 // IRQ_HANDLED
}

unsafe extern "C" fn omap_mcbsp_tx_irq_handler(irq: i32, data: *mut std::ffi::c_void) -> i32 {
    let mcbsp = data as *mut OmapMcbsp;
    // irqst_spcr2 = MCBSP_READ(mcbsp, SPCR2);
    // dev_dbg calls
    // if irqst_spcr2 & XSYNC_ERR: MCBSP_WRITE(mcbsp, SPCR2, ...);
    2 // IRQ_HANDLED
}

unsafe extern "C" fn omap_mcbsp_rx_irq_handler(irq: i32, data: *mut std::ffi::c_void) -> i32 {
    let mcbsp = data as *mut OmapMcbsp;
    // irqst_spcr1 = MCBSP_READ(mcbsp, SPCR1);
    // dev_dbg calls
    // if irqst_spcr1 & RSYNC_ERR: MCBSP_WRITE(mcbsp, SPCR1, ...);
    2 // IRQ_HANDLED
}

unsafe fn omap_mcbsp_config(mcbsp: *mut OmapMcbsp, config: *const OmapMcbspRegCfg) {
    dev_dbg(
        (*mcbsp).dev,
        b"Configuring McBSP%d  phys_base: 0x%08lx\0" as *const u8,
        (*mcbsp).id,
        (*mcbsp).phys_base,
    );

    // MCBSP_WRITE calls with config values
    // if mcbsp->pdata->has_ccr: additional writes
    // if mcbsp->pdata->has_wakeup: wakeup enable
    // if mcbsp->irq: enable interrupt sources
}

unsafe fn omap_mcbsp_dma_reg_params(mcbsp: *mut OmapMcbsp, stream: u32) -> usize {
    let data_reg: u32;

    if stream == 0 { // SNDRV_PCM_STREAM_PLAYBACK
        if (*(*mcbsp).pdata).reg_size == 2 {
            data_reg = 0; // OMAP_MCBSP_REG_DXR1
        } else {
            data_reg = 0; // OMAP_MCBSP_REG_DXR
        }
    } else {
        if (*(*mcbsp).pdata).reg_size == 2 {
            data_reg = 0; // OMAP_MCBSP_REG_DRR1
        } else {
            data_reg = 0; // OMAP_MCBSP_REG_DRR
        }
    }

    (*mcbsp).phys_dma_base + (data_reg * (*(*mcbsp).pdata).reg_step) as usize
}

unsafe fn omap_mcbsp_set_tx_threshold(mcbsp: *mut OmapMcbsp, threshold: u16) {
    if threshold > 0 && threshold <= (*mcbsp).max_tx_thres {
        // MCBSP_WRITE(mcbsp, THRSH2, threshold - 1);
    }
}

unsafe fn omap_mcbsp_set_rx_threshold(mcbsp: *mut OmapMcbsp, threshold: u16) {
    if threshold > 0 && threshold <= (*mcbsp).max_rx_thres {
        // MCBSP_WRITE(mcbsp, THRSH1, threshold - 1);
    }
}

unsafe fn omap_mcbsp_get_tx_delay(mcbsp: *mut OmapMcbsp) -> u16 {
    // buffstat = MCBSP_READ(mcbsp, XBUFFSTAT);
    // return (*mcbsp).pdata->buffer_size - buffstat;
    0
}

unsafe fn omap_mcbsp_get_rx_delay(mcbsp: *mut OmapMcbsp) -> u16 {
    // buffstat = MCBSP_READ(mcbsp, RBUFFSTAT);
    // threshold = MCBSP_READ(mcbsp, THRSH1);
    // if threshold <= buffstat: return 0
    // else: return threshold - buffstat
    0
}

unsafe fn omap_mcbsp_request(mcbsp: *mut OmapMcbsp) -> i32 {
    let reg_cache = kzalloc((*mcbsp).reg_cache_size, 0x200); // GFP_KERNEL
    if reg_cache.is_null() {
        return -12; // -ENOMEM
    }

    // scoped_guard(spinlock, &mcbsp->lock)
    if !(*mcbsp).free {
        dev_err((*mcbsp).dev, b"McBSP%d is currently in use\0" as *const u8, (*mcbsp).id);
        kfree(reg_cache);
        return -16; // -EBUSY
    }

    (*mcbsp).free = false;
    (*mcbsp).reg_cache = reg_cache;

    if !(*(*mcbsp).pdata).ops.is_null() && !(*(*(*mcbsp).pdata).ops).request.is_none() {
        (*(*(*mcbsp).pdata).ops).request.unwrap()((*mcbsp).id as u32 - 1);
    }

    // MCBSP_WRITE calls to reset transmitter/receiver

    // Handle IRQ registration
    if (*mcbsp).irq != 0 {
        let err = request_irq(
            (*mcbsp).irq as u32,
            omap_mcbsp_irq_handler,
            0,
            b"McBSP\0" as *const u8,
            mcbsp as *mut std::ffi::c_void,
        );
        if err != 0 {
            dev_err((*mcbsp).dev, b"Unable to request IRQ\0" as *const u8);
            return err;
        }
    } else {
        let err = request_irq(
            (*mcbsp).tx_irq as u32,
            omap_mcbsp_tx_irq_handler,
            0,
            b"McBSP TX\0" as *const u8,
            mcbsp as *mut std::ffi::c_void,
        );
        if err != 0 {
            dev_err((*mcbsp).dev, b"Unable to request TX IRQ\0" as *const u8);
            return err;
        }

        let err = request_irq(
            (*mcbsp).rx_irq as u32,
            omap_mcbsp_rx_irq_handler,
            0,
            b"McBSP RX\0" as *const u8,
            mcbsp as *mut std::ffi::c_void,
        );
        if err != 0 {
            dev_err((*mcbsp).dev, b"Unable to request RX IRQ\0" as *const u8);
            free_irq((*mcbsp).tx_irq as u32, mcbsp as *mut std::ffi::c_void);
            return err;
        }
    }

    0
}

unsafe fn omap_mcbsp_free(mcbsp: *mut OmapMcbsp) {
    if !(*(*mcbsp).pdata).ops.is_null() && !(*(*(*mcbsp).pdata).ops).free.is_none() {
        (*(*(*mcbsp).pdata).ops).free.unwrap()((*mcbsp).id as u32 - 1);
    }

    // Disable wakeup behavior
    // if mcbsp->pdata->has_wakeup: MCBSP_WRITE

    if (*mcbsp).irq != 0 {
        // MCBSP_WRITE(mcbsp, IRQEN, 0);
        free_irq((*mcbsp).irq as u32, mcbsp as *mut std::ffi::c_void);
    } else {
        free_irq((*mcbsp).rx_irq as u32, mcbsp as *mut std::ffi::c_void);
        free_irq((*mcbsp).tx_irq as u32, mcbsp as *mut std::ffi::c_void);
    }

    let reg_cache = (*mcbsp).reg_cache;

    if !mcbsp_omap1() {
        omap2_mcbsp_set_clks_src(mcbsp, 1); // MCBSP_CLKS_PRCM_SRC
    }

    // scoped_guard(spinlock, &mcbsp->lock)
    if (*mcbsp).free {
        dev_err((*mcbsp).dev, b"McBSP%d was not reserved\0" as *const u8, (*mcbsp).id);
    } else {
        (*mcbsp).free = true;
    }
    (*mcbsp).reg_cache = std::ptr::null_mut();

    kfree(reg_cache);
}

unsafe fn omap_mcbsp_start(mcbsp: *mut OmapMcbsp, stream: u32) {
    let tx = if stream == 0 { 1 } else { 0 }; // 0 = SNDRV_PCM_STREAM_PLAYBACK
    let rx = if tx != 0 { 0 } else { 1 };
    let mut enable_srg = 0;

    if !(*mcbsp).st_data.is_null() {
        omap_mcbsp_st_start(mcbsp);
    }

    // Only enable SRG if McBSP is master
    // w = MCBSP_READ_CACHE(mcbsp, PCR0);
    // if w & (FSXM | FSRM | CLKXM | CLKRM)
    //     enable_srg = !((MCBSP_READ_CACHE(...SPCR2) | MCBSP_READ_CACHE(...SPCR1)) & 1)

    if enable_srg != 0 {
        // w = MCBSP_READ_CACHE(mcbsp, SPCR2);
        // MCBSP_WRITE(mcbsp, SPCR2, w | (1 << 6));
    }

    // Enable transmitter and receiver
    // MCBSP_WRITE calls for SPCR2 and SPCR1

    udelay(500);

    if enable_srg != 0 {
        // w = MCBSP_READ_CACHE(mcbsp, SPCR2);
        // MCBSP_WRITE(mcbsp, SPCR2, w | (1 << 7));
    }

    if (*(*mcbsp).pdata).has_ccr {
        // Release transmitter and receiver
        // MCBSP_WRITE calls for XCCR and RCCR
    }

    omap_mcbsp_dump_reg(mcbsp);
}

unsafe fn omap_mcbsp_stop(mcbsp: *mut OmapMcbsp, stream: u32) {
    let tx = if stream == 0 { 1 } else { 0 }; // 0 = SNDRV_PCM_STREAM_PLAYBACK
    let rx = if tx != 0 { 0 } else { 1 };

    // Reset transmitter
    if (*(*mcbsp).pdata).has_ccr {
        // MCBSP_WRITE calls for XCCR
    }
    // MCBSP_WRITE(mcbsp, SPCR2, ...);

    // Reset receiver
    if (*(*mcbsp).pdata).has_ccr {
        // MCBSP_WRITE calls for RCCR
    }
    // MCBSP_WRITE(mcbsp, SPCR1, ...);

    // idle = !((MCBSP_READ_CACHE(...) | MCBSP_READ_CACHE(...)) & 1)
    let idle = false;

    if idle {
        // Reset sample rate generator
        // MCBSP_WRITE(mcbsp, SPCR2, w & ~(1 << 6));
    }

    if !(*mcbsp).st_data.is_null() {
        omap_mcbsp_st_stop(mcbsp);
    }
}

unsafe fn omap_mcbsp_set_threshold(substream: *mut std::ffi::c_void, packet_size: u32) {
    // rtd = snd_soc_substream_to_rtd(substream);
    // cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    // mcbsp = snd_soc_dai_get_drvdata(cpu_dai);

    // if mcbsp->pdata->buffer_size == 0: return

    // if packet_size: words = packet_size
    // else: words = 1

    // if substream->stream == SNDRV_PCM_STREAM_PLAYBACK
    //     omap_mcbsp_set_tx_threshold(mcbsp, words);
    // else
    //     omap_mcbsp_set_rx_threshold(mcbsp, words);
}

unsafe extern "C" fn omap_mcbsp_hwrule_min_buffersize(
    params: *mut std::ffi::c_void,
    rule: *mut std::ffi::c_void,
) -> i32 {
    // let buffer_size = hw_param_interval(params, SNDRV_PCM_HW_PARAM_BUFFER_SIZE);
    // let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    // let mcbsp = (*rule).private as *mut OmapMcbsp;
    // let mut frames: SndInterval;

    // snd_interval_any(&mut frames);
    // frames.min = (*mcbsp).pdata->buffer_size / channels.min;
    // frames.integer = 1;
    // return snd_interval_refine(buffer_size, &frames);

    0
}

unsafe extern "C" fn omap_mcbsp_dai_startup(
    substream: *mut std::ffi::c_void,
    cpu_dai: *mut std::ffi::c_void,
) -> i32 {
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;
    let mut err = 0;

    if snd_soc_dai_active(cpu_dai) == 0 {
        err = omap_mcbsp_request(mcbsp);
    }

    // if mcbsp->pdata->buffer_size: add constraints

    err
}

unsafe extern "C" fn omap_mcbsp_dai_shutdown(
    substream: *mut std::ffi::c_void,
    cpu_dai: *mut std::ffi::c_void,
) {
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;

    // Handle QoS requests
    // if mcbsp->latency[stream2]: update request
    // else if mcbsp->latency[stream1]: remove request

    // mcbsp->latency[stream1] = 0;

    if snd_soc_dai_active(cpu_dai) == 0 {
        omap_mcbsp_free(mcbsp);
        (*mcbsp).configured = 0;
    }
}

unsafe extern "C" fn omap_mcbsp_dai_prepare(
    substream: *mut std::ffi::c_void,
    cpu_dai: *mut std::ffi::c_void,
) -> i32 {
    // let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;

    // Handle QoS request setup

    0
}

unsafe extern "C" fn omap_mcbsp_dai_trigger(
    substream: *mut std::ffi::c_void,
    cmd: i32,
    cpu_dai: *mut std::ffi::c_void,
) -> i32 {
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;

    match cmd {
        0 | 1 | 4 => { // SNDRV_PCM_TRIGGER_START, RESUME, PAUSE_RELEASE
            (*mcbsp).active += 1;
            // omap_mcbsp_start(mcbsp, substream->stream);
        }
        1 | 2 | 3 => { // SNDRV_PCM_TRIGGER_STOP, SUSPEND, PAUSE_PUSH
            // omap_mcbsp_stop(mcbsp, substream->stream);
            (*mcbsp).active -= 1;
        }
        _ => return -22, // -EINVAL
    }

    0
}

unsafe extern "C" fn omap_mcbsp_dai_delay(
    substream: *mut std::ffi::c_void,
    dai: *mut std::ffi::c_void,
) -> i64 {
    // rtd = snd_soc_substream_to_rtd(substream);
    // cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    // mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;

    // if mcbsp->pdata->buffer_size == 0: return 0

    // if substream->stream == SNDRV_PCM_STREAM_PLAYBACK
    //     fifo_use = omap_mcbsp_get_tx_delay(mcbsp);
    // else
    //     fifo_use = omap_mcbsp_get_rx_delay(mcbsp);

    // delay = fifo_use / substream->runtime->channels;

    0
}

unsafe extern "C" fn omap_mcbsp_dai_hw_params(
    substream: *mut std::ffi::c_void,
    params: *mut std::ffi::c_void,
    cpu_dai: *mut std::ffi::c_void,
) -> i32 {
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;
    let channels = params_channels(params);

    match params_format(params) {
        0 => { // SNDRV_PCM_FORMAT_S16_LE
            // wlen = 16
        }
        1 => { // SNDRV_PCM_FORMAT_S32_LE
            // wlen = 32
        }
        _ => return -22, // -EINVAL
    }

    // Handle buffer size and DMA threshold configuration

    if (*mcbsp).configured != 0 {
        return 0;
    }

    // Configure registers based on format and parameters

    (*mcbsp).configured = 1;

    0
}

unsafe extern "C" fn omap_mcbsp_dai_set_dai_fmt(cpu_dai: *mut std::ffi::c_void, fmt: u32) -> i32 {
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;

    if (*mcbsp).configured != 0 {
        return 0;
    }

    (*mcbsp).fmt = fmt;
    // memset(&(*mcbsp).cfg_regs, 0, size_of::<OmapMcbspRegCfg>());

    // Configure register bits based on format

    0
}

unsafe extern "C" fn omap_mcbsp_dai_set_clkdiv(cpu_dai: *mut std::ffi::c_void, div_id: i32, div: i32) -> i32 {
    if div_id != 0 { // OMAP_MCBSP_CLKGDV
        return -19; // -ENODEV
    }

    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;
    (*mcbsp).clk_div = div;

    0
}

unsafe extern "C" fn omap_mcbsp_dai_set_dai_sysclk(
    cpu_dai: *mut std::ffi::c_void,
    clk_id: i32,
    freq: u32,
    dir: i32,
) -> i32 {
    let mcbsp = snd_soc_dai_get_drvdata(cpu_dai) as *mut OmapMcbsp;

    if (*mcbsp).active != 0 {
        if freq == (*mcbsp).in_freq {
            return 0;
        } else {
            return -16; // -EBUSY
        }
    }

    (*mcbsp).in_freq = freq;

    match clk_id {
        0 => { // OMAP_MCBSP_SYSCLK_CLK
            // Configure clock
        }
        1 => { // OMAP_MCBSP_SYSCLK_CLKS_FCLK
            if mcbsp_omap1() {
                return -22; // -EINVAL
            }
            // omap2_mcbsp_set_clks_src(mcbsp, MCBSP_CLKS_PRCM_SRC);
        }
        2 => { // OMAP_MCBSP_SYSCLK_CLKS_EXT
            if mcbsp_omap1() {
                return 0;
            }
            // omap2_mcbsp_set_clks_src(mcbsp, MCBSP_CLKS_PAD_SRC);
        }
        3 => { // OMAP_MCBSP_SYSCLK_CLKX_EXT
            // Configure for external CLKX
        }
        4 => { // OMAP_MCBSP_SYSCLK_CLKR_EXT
            // Configure for external CLKR
        }
        _ => return -19, // -ENODEV
    }

    0
}

unsafe extern "C" fn omap_mcbsp_probe(dai: *mut std::ffi::c_void) -> i32 {
    let mcbsp = snd_soc_dai_get_drvdata(dai) as *mut OmapMcbsp;

    pm_runtime_enable((*mcbsp).dev);

    snd_soc_dai_init_dma_data(
        dai,
        &(*mcbsp).dma_data[0] as *const _ as *const std::ffi::c_void,
        &(*mcbsp).dma_data[1] as *const _ as *const std::ffi::c_void,
    );

    0
}

unsafe extern "C" fn omap_mcbsp_remove(dai: *mut std::ffi::c_void) -> i32 {
    let mcbsp = snd_soc_dai_get_drvdata(dai) as *mut OmapMcbsp;

    pm_runtime_disable((*mcbsp).dev);

    0
}

pub struct SndSocDaiOps {
    pub probe: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> i32>,
    pub startup: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
    pub prepare: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut std::ffi::c_void, i32, *mut std::ffi::c_void) -> i32>,
    pub delay: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i64>,
    pub hw_params: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void, *mut std::ffi::c_void) -> i32>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut std::ffi::c_void, u32) -> i32>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut std::ffi::c_void, i32, i32) -> i32>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut std::ffi::c_void, i32, u32, i32) -> i32>,
}

pub struct SndSocDaiDriver {
    pub playback: SndPcmCapabilities,
    pub capture: SndPcmCapabilities,
    pub ops: *const SndSocDaiOps,
}

pub struct SndPcmCapabilities {
    pub channels_min: i32,
    pub channels_max: i32,
    pub rates: u32,
    pub formats: u32,
}

pub struct SndSocComponentDriver {
    pub name: *const u8,
    pub legacy_dai_naming: i32,
}

pub struct DeviceAttribute {
    pub attr: *const std::ffi::c_void,
}

pub struct AttributeGroup {
    pub attrs: *mut *const DeviceAttribute,
}

// Platform data instances
pub const OMAP2420_PDATA: OmapMcbspPlatformData = OmapMcbspPlatformData {
    reg_step: 4,
    reg_size: 2,
    has_ccr: false,
    has_wakeup: false,
    buffer_size: 0,
    force_ick_on: false,
    ops: std::ptr::null(),
};

pub const OMAP2430_PDATA: OmapMcbspPlatformData = OmapMcbspPlatformData {
    reg_step: 4,
    reg_size: 4,
    has_ccr: true,
    has_wakeup: false,
    buffer_size: 0,
    force_ick_on: false,
    ops: std::ptr::null(),
};

pub const OMAP3_PDATA: OmapMcbspPlatformData = OmapMcbspPlatformData {
    reg_step: 4,
    reg_size: 4,
    has_ccr: true,
    has_wakeup: true,
    buffer_size: 0,
    force_ick_on: false,
    ops: std::ptr::null(),
};

pub const OMAP4_PDATA: OmapMcbspPlatformData = OmapMcbspPlatformData {
    reg_step: 4,
    reg_size: 4,
    has_ccr: true,
    has_wakeup: true,
    buffer_size: 0,
    force_ick_on: false,
    ops: std::ptr::null(),
};

// Device match table
pub struct OfDeviceId {
    pub compatible: *const u8,
    pub data: *const std::ffi::c_void,
}

pub const OMAP_MCBSP_OF_MATCH: &[OfDeviceId] = &[
    OfDeviceId {
        compatible: b"ti,omap2420-mcbsp\0" as *const u8,
        data: &OMAP2420_PDATA as *const _ as *const std::ffi::c_void,
    },
    OfDeviceId {
        compatible: b"ti,omap2430-mcbsp\0" as *const u8,
        data: &OMAP2430_PDATA as *const _ as *const std::ffi::c_void,
    },
    OfDeviceId {
        compatible: b"ti,omap3-mcbsp\0" as *const u8,
        data: &OMAP3_PDATA as *const _ as *const std::ffi::c_void,
    },
    OfDeviceId {
        compatible: b"ti,omap4-mcbsp\0" as *const u8,
        data: &OMAP4_PDATA as *const _ as *const std::ffi::c_void,
    },
];

unsafe fn omap_mcbsp_init(pdev: *mut std::ffi::c_void) -> i32 {
    let mcbsp = platform_get_drvdata(pdev) as *mut OmapMcbsp;

    spin_lock_init(&mut (*mcbsp).lock as *mut _ as *mut std::ffi::c_void);
    (*mcbsp).free = true;

    // Get resource and setup I/O base, physical base, and DMA base
    // Get IRQ resources
    // Setup DMA data
    // Get clock

    0
}

unsafe extern "C" fn asoc_mcbsp_probe(pdev: *mut std::ffi::c_void) -> i32 {
    let pdata = dev_get_platdata(pdev) as *mut OmapMcbspPlatformData;
    let match_pdata = device_get_match_data(pdev) as *const OmapMcbspPlatformData;

    let mut pdata_final = if !match_pdata.is_null() {
        // Copy platform data
        let pdata_copy = devm_kmemdup(
            pdev,
            match_pdata as *const std::ffi::c_void,
            std::mem::size_of::<OmapMcbspPlatformData>(),
            0x200, // GFP_KERNEL
        );
        if pdata_copy.is_null() {
            return -12; // -ENOMEM
        }
        pdata_copy as *mut OmapMcbspPlatformData
    } else if pdata.is_null() {
        dev_err(pdev, b"missing platform data.\0" as *const u8);
        return -22; // -EINVAL
    } else {
        pdata
    };

    let mcbsp = devm_kzalloc(pdev, std::mem::size_of::<OmapMcbsp>(), 0x200) as *mut OmapMcbsp;
    if mcbsp.is_null() {
        return -12; // -ENOMEM
    }

    // Setup mcbsp structure
    // (*mcbsp).id = pdev->id;
    // (*mcbsp).pdata = pdata_final;
    // (*mcbsp).dev = &pdev->dev;
    // platform_set_drvdata(pdev, mcbsp);

    let ret = omap_mcbsp_init(pdev);
    if ret != 0 {
        return ret;
    }

    // Register component and DAI
    let ret = devm_snd_soc_register_component(pdev, std::ptr::null(), std::ptr::null(), 1);
    if ret != 0 {
        return ret;
    }

    sdma_pcm_platform_register(pdev, b"tx\0" as *const u8, b"rx\0" as *const u8)
}

unsafe extern "C" fn asoc_mcbsp_remove(pdev: *mut std::ffi::c_void) {
    let mcbsp = platform_get_drvdata(pdev) as *mut OmapMcbsp;

    if !(*(*mcbsp).pdata).ops.is_null() && !(*(*(*mcbsp).pdata).ops).free.is_none() {
        (*(*(*mcbsp).pdata).ops).free.unwrap()((*mcbsp).id as u32);
    }

    // cpu_latency_qos_remove_request if active
}

pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
}

pub const ASOC_MCBSP_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(asoc_mcbsp_probe),
    remove: Some(asoc_mcbsp_remove),
};

// Module metadata (preserved as comments since they're Linux kernel specific)
// MODULE_AUTHOR("Jarkko Nikula <jarkko.nikula@bitmer.com>");
// MODULE_DESCRIPTION("OMAP I2S SoC Interface");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:omap-mcbsp");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
