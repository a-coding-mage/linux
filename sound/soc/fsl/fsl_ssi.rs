// SPDX-License-Identifier: GPL-2.0
//
// Freescale SSI ALSA SoC Digital Audio Interface (DAI) driver
//
// Author: Timur Tabi <timur@freescale.com>
//
// Copyright 2007-2010 Freescale Semiconductor, Inc.
//
// Some notes why imx-pcm-fiq is used instead of DMA on some boards:
//
// The i.MX SSI core has some nasty limitations in AC97 mode. While most
// sane processor vendors have a FIFO per AC97 slot, the i.MX has only
// one FIFO which combines all valid receive slots. We cannot even select
// which slots we want to receive. The WM9712 with which this driver
// was developed with always sends GPIO status data in slot 12 which
// we receive in our (PCM-) data stream. The only chance we have is to
// manually skip this data in the FIQ handler. With sampling rates different
// from 48000Hz not every frame has valid receive data, so the ratio
// between pcm data and GPIO status data changes. Our FIQ handler is not
// able to handle this, hence this driver only works with 48000Hz sampling
// rate.
// Reading and writing AC97 registers is another challenge. The core
// provides us status bits when the read register is updated with *another*
// value. When we read the same register two times (and the register still
// contains the same value) these status bits are not set. We work
// around this by not polling these bits but only wait a fixed delay.
//
// Dependencies from Linux, ALSA SoC, fsl_ssi.h, and imx-pcm.h are expected
// to be supplied by surrounding bindings.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = ::core::primitive::u8;
type u32 = ::core::primitive::u32;
type u64 = ::core::primitive::u64;
type bool_t = bool;
type dma_addr_t = usize;
type irqreturn_t = c_uint;

const RX: usize = 0;
const TX: usize = 1;

// FSLSSI_I2S_FORMATS is endian-dependent in C:
// big-endian uses S16/S18_3/S20_3/S24_3/S24 BE formats; otherwise LE formats.
#[cfg(target_endian = "big")]
const FSLSSI_I2S_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_BE
    | SNDRV_PCM_FMTBIT_S18_3BE
    | SNDRV_PCM_FMTBIT_S20_3BE
    | SNDRV_PCM_FMTBIT_S24_3BE
    | SNDRV_PCM_FMTBIT_S24_BE;
#[cfg(not(target_endian = "big"))]
const FSLSSI_I2S_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE;

/*
 * In AC97 mode, TXDIR bit is forced to 0 and TFDIR bit is forced to 1:
 *  - SSI inputs external bit clock and outputs frame sync clock -- CBM_CFS
 *  - Also have NB_NF to mark these two clocks will not be inverted
 */
const FSLSSI_AC97_DAIFMT: c_uint =
    SND_SOC_DAIFMT_AC97 | SND_SOC_DAIFMT_BC_FP | SND_SOC_DAIFMT_NB_NF;

const FSLSSI_SIER_DBG_RX_FLAGS: u32 =
    SSI_SIER_RFF0_EN | SSI_SIER_RLS_EN | SSI_SIER_RFS_EN | SSI_SIER_ROE0_EN | SSI_SIER_RFRC_EN;
const FSLSSI_SIER_DBG_TX_FLAGS: u32 =
    SSI_SIER_TFE0_EN | SSI_SIER_TLS_EN | SSI_SIER_TFS_EN | SSI_SIER_TUE0_EN | SSI_SIER_TFRC_EN;

#[repr(C)]
enum fsl_ssi_type {
    FSL_SSI_MCP8610,
    FSL_SSI_MX21,
    FSL_SSI_MX35,
    FSL_SSI_MX51,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct fsl_ssi_regvals {
    sier: u32,
    srcr: u32,
    stcr: u32,
    scr: u32,
}

unsafe extern "C" {
    static mut fsl_ssi_debugfs_create: unsafe extern "C" fn(*mut fsl_ssi_dbg, *mut device);
    static mut fsl_ssi_debugfs_remove: unsafe extern "C" fn(*mut fsl_ssi_dbg);
}

unsafe fn fsl_ssi_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_SSI_SACCEN | REG_SSI_SACCDIS => false,
        _ => true,
    }
}

unsafe fn fsl_ssi_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_SSI_STX0 | REG_SSI_STX1 | REG_SSI_SRX0 | REG_SSI_SRX1 | REG_SSI_SISR
        | REG_SSI_SFCSR | REG_SSI_SACNT | REG_SSI_SACADD | REG_SSI_SACDAT | REG_SSI_SATAG
        | REG_SSI_SACCST | REG_SSI_SOR => true,
        _ => false,
    }
}

unsafe fn fsl_ssi_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_SSI_SRX0 | REG_SSI_SRX1 | REG_SSI_SISR | REG_SSI_SACADD | REG_SSI_SACDAT
        | REG_SSI_SATAG => true,
        _ => false,
    }
}

unsafe fn fsl_ssi_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_SSI_SRX0 | REG_SSI_SRX1 | REG_SSI_SACCST => false,
        _ => true,
    }
}

static fsl_ssi_regconfig: regmap_config = regmap_config {
    max_register: REG_SSI_SACCDIS,
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    val_format_endian: REGMAP_ENDIAN_NATIVE,
    num_reg_defaults_raw: REG_SSI_SACCDIS / size_of::<u32>() as c_uint + 1,
    readable_reg: Some(fsl_ssi_readable_reg),
    volatile_reg: Some(fsl_ssi_volatile_reg),
    precious_reg: Some(fsl_ssi_precious_reg),
    writeable_reg: Some(fsl_ssi_writeable_reg),
    cache_type: REGCACHE_FLAT,
};

#[repr(C)]
struct fsl_ssi_soc_data {
    imx: bool,
    imx21regs: bool, /* imx21-class SSI - no SACC{ST,EN,DIS} regs */
    offline_config: bool,
    sisr_write_mask: u32,
}

/**
 * struct fsl_ssi - per-SSI private data
 * @regs: Pointer to the regmap registers
 * @irq: IRQ of this SSI
 * @cpu_dai_drv: CPU DAI driver for this device
 * @dai_fmt: DAI configuration this device is currently used with
 * @streams: Mask of current active streams: BIT(TX) and BIT(RX)
 * @i2s_net: I2S and Network mode configurations of SCR register
 *           (this is the initial settings based on the DAI format)
 * @synchronous: Use synchronous mode - both of TX and RX use STCK and SFCK
 * @use_dma: DMA is used or FIQ with stream filter
 * @use_dual_fifo: DMA with support for dual FIFO mode
 * @use_dyna_fifo: DMA with support for multi FIFO script
 * @has_ipg_clk_name: If "ipg" is in the clock name list of device tree
 * @fifo_depth: Depth of the SSI FIFOs
 * @slot_width: Width of each DAI slot
 * @slots: Number of slots
 * @regvals: Specific RX/TX register settings
 * @clk: Clock source to access register
 * @baudclk: Clock source to generate bit and frame-sync clocks
 * @baudclk_streams: Active streams that are using baudclk
 * @regcache_sfcsr: Cache sfcsr register value during suspend and resume
 * @regcache_sacnt: Cache sacnt register value during suspend and resume
 * @dma_params_tx: DMA transmit parameters
 * @dma_params_rx: DMA receive parameters
 * @ssi_phys: physical address of the SSI registers
 * @fiq_params: FIQ stream filtering parameters
 * @card_pdev: Platform_device pointer to register a sound card for PowerPC or
 *             to register a CODEC platform device for AC97
 * @card_name: Platform_device name to register a sound card for PowerPC or
 *             to register a CODEC platform device for AC97
 * @card_idx: The index of SSI to register a sound card for PowerPC or
 *            to register a CODEC platform device for AC97
 * @dbg_stats: Debugging statistics
 * @soc: SoC specific data
 * @dev: Pointer to &pdev->dev
 * @fifo_watermark: The FIFO watermark setting. Notifies DMA when there are
 *                  @fifo_watermark or fewer words in TX fifo or
 *                  @fifo_watermark or more empty words in RX fifo.
 * @dma_maxburst: Max number of words to transfer in one go. So far,
 *                this is always the same as fifo_watermark.
 * @ac97_reg_lock: Mutex lock to serialize AC97 register access operations
 * @audio_config: configure for dma multi fifo script
 */
#[repr(C)]
struct fsl_ssi {
    regs: *mut regmap,
    irq: c_int,
    cpu_dai_drv: snd_soc_dai_driver,
    dai_fmt: c_uint,
    streams: u8,
    i2s_net: u8,
    synchronous: bool,
    use_dma: bool,
    use_dual_fifo: bool,
    use_dyna_fifo: bool,
    has_ipg_clk_name: bool,
    fifo_depth: c_uint,
    slot_width: c_uint,
    slots: c_uint,
    regvals: [fsl_ssi_regvals; 2],
    clk: *mut clk,
    baudclk: *mut clk,
    baudclk_streams: c_uint,
    regcache_sfcsr: u32,
    regcache_sacnt: u32,
    dma_params_tx: snd_dmaengine_dai_dma_data,
    dma_params_rx: snd_dmaengine_dai_dma_data,
    ssi_phys: dma_addr_t,
    fiq_params: imx_pcm_fiq_params,
    card_pdev: *mut platform_device,
    card_name: [c_char; 32],
    card_idx: u32,
    dbg_stats: fsl_ssi_dbg,
    soc: *const fsl_ssi_soc_data,
    dev: *mut device,
    fifo_watermark: u32,
    dma_maxburst: u32,
    ac97_reg_lock: mutex,
    audio_config: [sdma_peripheral_config; 2],
}

/*
 * SoC specific data
 *
 * Notes:
 * 1) SSI in earlier SoCS has critical bits in control registers that
 *    cannot be changed after SSI starts running -- a software reset
 *    (set SSIEN to 0) is required to change their values. So adding
 *    an offline_config flag for these SoCs.
 * 2) SDMA is available since imx35. However, imx35 does not support
 *    DMA bits changing when SSI is running, so set offline_config.
 * 3) imx51 and later versions support register configurations when
 *    SSI is running (SSIEN); For these versions, DMA needs to be
 *    configured before SSI sends DMA request to avoid an undefined
 *    DMA request on the SDMA side.
 */
static mut fsl_ssi_mpc8610: fsl_ssi_soc_data = fsl_ssi_soc_data {
    imx: false,
    imx21regs: false,
    offline_config: true,
    sisr_write_mask: SSI_SISR_RFRC | SSI_SISR_TFRC | SSI_SISR_ROE0 | SSI_SISR_ROE1 | SSI_SISR_TUE0 | SSI_SISR_TUE1,
};
static mut fsl_ssi_imx21: fsl_ssi_soc_data = fsl_ssi_soc_data {
    imx: true,
    imx21regs: true,
    offline_config: true,
    sisr_write_mask: 0,
};
static mut fsl_ssi_imx35: fsl_ssi_soc_data = fsl_ssi_soc_data {
    imx: true,
    imx21regs: false,
    offline_config: true,
    sisr_write_mask: SSI_SISR_RFRC | SSI_SISR_TFRC | SSI_SISR_ROE0 | SSI_SISR_ROE1 | SSI_SISR_TUE0 | SSI_SISR_TUE1,
};
static mut fsl_ssi_imx51: fsl_ssi_soc_data = fsl_ssi_soc_data {
    imx: true,
    imx21regs: false,
    offline_config: false,
    sisr_write_mask: SSI_SISR_ROE0 | SSI_SISR_ROE1 | SSI_SISR_TUE0 | SSI_SISR_TUE1,
};

static fsl_ssi_ids: [of_device_id; 5] = [
    of_device_id { compatible: c"fsl,mpc8610-ssi".as_ptr(), data: unsafe { &fsl_ssi_mpc8610 as *const _ as *const c_void } },
    of_device_id { compatible: c"fsl,imx51-ssi".as_ptr(), data: unsafe { &fsl_ssi_imx51 as *const _ as *const c_void } },
    of_device_id { compatible: c"fsl,imx35-ssi".as_ptr(), data: unsafe { &fsl_ssi_imx35 as *const _ as *const c_void } },
    of_device_id { compatible: c"fsl,imx21-ssi".as_ptr(), data: unsafe { &fsl_ssi_imx21 as *const _ as *const c_void } },
    of_device_id::zeroed(),
];
// MODULE_DEVICE_TABLE(of, fsl_ssi_ids);

unsafe fn fsl_ssi_is_ac97(ssi: *mut fsl_ssi) -> bool {
    ((*ssi).dai_fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_AC97
}

unsafe fn fsl_ssi_is_i2s_clock_provider(ssi: *mut fsl_ssi) -> bool {
    ((*ssi).dai_fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BP_FP
}

unsafe fn fsl_ssi_is_i2s_bc_fp(ssi: *mut fsl_ssi) -> bool {
    ((*ssi).dai_fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BC_FP
}

/**
 * fsl_ssi_isr - Interrupt handler to gather states
 * @irq: irq number
 * @dev_id: context
 */
unsafe extern "C" fn fsl_ssi_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let ssi = dev_id as *mut fsl_ssi;
    let regs = (*ssi).regs;
    let mut sisr: u32 = 0;
    regmap_read(regs, REG_SSI_SISR, &mut sisr);
    let sisr2 = sisr & (*(*ssi).soc).sisr_write_mask;
    /* Clear the bits that we set */
    if sisr2 != 0 {
        regmap_write(regs, REG_SSI_SISR, sisr2);
    }
    fsl_ssi_dbg_isr(&mut (*ssi).dbg_stats, sisr);
    IRQ_HANDLED
}

unsafe fn fsl_ssi_config_enable(ssi: *mut fsl_ssi, tx: bool) {
    let vals = (*ssi).regvals.as_mut_ptr();
    let dir = if tx { TX } else { RX };
    let (sier, srcr, stcr): (u32, u32, u32);
    regmap_update_bits((*ssi).regs, REG_SSI_SOR, SSI_SOR_xX_CLR(tx), SSI_SOR_xX_CLR(tx));
    if !((*(*ssi).soc).offline_config && (*ssi).streams != 0) {
        if (*(*ssi).soc).offline_config {
            srcr = (*vals.add(RX)).srcr | (*vals.add(TX)).srcr;
            stcr = (*vals.add(RX)).stcr | (*vals.add(TX)).stcr;
            sier = (*vals.add(RX)).sier | (*vals.add(TX)).sier;
        } else {
            srcr = (*vals.add(dir)).srcr;
            stcr = (*vals.add(dir)).stcr;
            sier = (*vals.add(dir)).sier;
        }
        regmap_update_bits((*ssi).regs, REG_SSI_SRCR, srcr, srcr);
        regmap_update_bits((*ssi).regs, REG_SSI_STCR, stcr, stcr);
        regmap_update_bits((*ssi).regs, REG_SSI_SIER, sier, sier);
    }
    if (*ssi).use_dma && tx {
        let mut tries: c_int = 100;
        let mut sfcsr: u32 = 0;
        regmap_update_bits((*ssi).regs, REG_SSI_SCR, SSI_SCR_SSIEN, SSI_SCR_SSIEN);
        loop {
            regmap_read((*ssi).regs, REG_SSI_SFCSR, &mut sfcsr);
            if SSI_SFCSR_TFCNT0(sfcsr) != 0 {
                break;
            }
            tries -= 1;
            if tries == 0 {
                break;
            }
        }
        if SSI_SFCSR_TFCNT0(sfcsr) == 0 {
            dev_warn((*ssi).dev, c"Timeout waiting TX FIFO filling\n".as_ptr());
        }
    }
    regmap_update_bits((*ssi).regs, REG_SSI_SCR, (*vals.add(dir)).scr, (*vals.add(dir)).scr);
    (*ssi).streams |= BIT(dir as c_uint) as u8;
}

fn _ssi_xor_shared_bits(vals: u32, avals: u32, aactive: bool) -> u32 {
    vals ^ avals.wrapping_mul(aactive as u32)
}
fn ssi_excl_shared_bits(vals: u32, avals: u32, aactive: bool) -> u32 {
    vals & _ssi_xor_shared_bits(vals, avals, aactive)
}

unsafe fn fsl_ssi_config_disable(ssi: *mut fsl_ssi, tx: bool) {
    let adir = if tx { RX } else { TX };
    let dir = if tx { TX } else { RX };
    let aactive = ((*ssi).streams & BIT(adir as c_uint) as u8) != 0;
    let vals = &mut (*ssi).regvals[dir] as *mut fsl_ssi_regvals;
    let avals = &mut (*ssi).regvals[adir] as *mut fsl_ssi_regvals;
    let scr = ssi_excl_shared_bits((*vals).scr, (*avals).scr, aactive);
    regmap_update_bits((*ssi).regs, REG_SSI_SCR, scr, 0);
    (*ssi).streams &= !(BIT(dir as c_uint) as u8);
    if !((*(*ssi).soc).offline_config && aactive) {
        let (sier, srcr, stcr) = if (*(*ssi).soc).offline_config {
            ((*vals).sier | (*avals).sier, (*vals).srcr | (*avals).srcr, (*vals).stcr | (*avals).stcr)
        } else {
            (
                ssi_excl_shared_bits((*vals).sier, (*avals).sier, aactive),
                ssi_excl_shared_bits((*vals).srcr, (*avals).srcr, aactive),
                ssi_excl_shared_bits((*vals).stcr, (*avals).stcr, aactive),
            )
        };
        regmap_update_bits((*ssi).regs, REG_SSI_SRCR, srcr, 0);
        regmap_update_bits((*ssi).regs, REG_SSI_STCR, stcr, 0);
        regmap_update_bits((*ssi).regs, REG_SSI_SIER, sier, 0);
    }
    regmap_update_bits((*ssi).regs, REG_SSI_SOR, SSI_SOR_xX_CLR(tx), SSI_SOR_xX_CLR(tx));
}

unsafe fn fsl_ssi_tx_ac97_saccst_setup(ssi: *mut fsl_ssi) {
    let regs = (*ssi).regs;
    /* no SACC{ST,EN,DIS} regs on imx21-class SSI */
    if !(*(*ssi).soc).imx21regs {
        regmap_write(regs, REG_SSI_SACCDIS, 0xff);
        regmap_write(regs, REG_SSI_SACCEN, 0x300);
    }
}

unsafe fn fsl_ssi_setup_regvals(ssi: *mut fsl_ssi) {
    let vals = (*ssi).regvals.as_mut_ptr();
    (*vals.add(RX)).sier = SSI_SIER_RFF0_EN | FSLSSI_SIER_DBG_RX_FLAGS;
    (*vals.add(RX)).srcr = SSI_SRCR_RFEN0;
    (*vals.add(RX)).scr = SSI_SCR_SSIEN | SSI_SCR_RE;
    (*vals.add(TX)).sier = SSI_SIER_TFE0_EN | FSLSSI_SIER_DBG_TX_FLAGS;
    (*vals.add(TX)).stcr = SSI_STCR_TFEN0;
    (*vals.add(TX)).scr = SSI_SCR_SSIEN | SSI_SCR_TE;
    if fsl_ssi_is_ac97(ssi) {
        (*vals.add(RX)).scr = 0;
        (*vals.add(TX)).scr = 0;
    }
    if (*ssi).use_dual_fifo {
        (*vals.add(RX)).srcr |= SSI_SRCR_RFEN1;
        (*vals.add(TX)).stcr |= SSI_STCR_TFEN1;
    }
    if (*ssi).use_dma {
        (*vals.add(RX)).sier |= SSI_SIER_RDMAE;
        (*vals.add(TX)).sier |= SSI_SIER_TDMAE;
    } else {
        (*vals.add(RX)).sier |= SSI_SIER_RIE;
        (*vals.add(TX)).sier |= SSI_SIER_TIE;
    }
}

unsafe fn fsl_ssi_setup_ac97(ssi: *mut fsl_ssi) {
    let regs = (*ssi).regs;
    regmap_write(regs, REG_SSI_STCCR, SSI_SxCCR_WL(17) | SSI_SxCCR_DC(13));
    regmap_write(regs, REG_SSI_SRCCR, SSI_SxCCR_WL(17) | SSI_SxCCR_DC(13));
    regmap_write(regs, REG_SSI_SACNT, SSI_SACNT_AC97EN | SSI_SACNT_FV);
    regmap_update_bits(regs, REG_SSI_SCR, SSI_SCR_SSIEN | SSI_SCR_TE | SSI_SCR_RE, SSI_SCR_SSIEN | SSI_SCR_TE | SSI_SCR_RE);
    regmap_write(regs, REG_SSI_SOR, SSI_SOR_WAIT(3));
}

unsafe extern "C" fn fsl_ssi_startup(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let ssi = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_ssi;
    let ret = clk_prepare_enable((*ssi).clk);
    if ret != 0 {
        return ret;
    }
    if (*ssi).use_dual_fifo || (*ssi).use_dyna_fifo {
        snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 2);
    }
    0
}

unsafe extern "C" fn fsl_ssi_shutdown(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let ssi = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_ssi;
    clk_disable_unprepare((*ssi).clk);
}

unsafe fn fsl_ssi_set_bclk(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let ssi = snd_soc_dai_get_drvdata(dai) as *mut fsl_ssi;
    let regs = (*ssi).regs;
    let mut pm: u32 = 999;
    let div2: u32 = 0;
    let psr: u32 = 0;
    let mut baudrate: c_ulong = 0;
    let mut savesub: u64 = 100000;
    let mut channels = params_channels(hw_params);
    let mut slot_width = params_width(hw_params);
    let mut slots: c_uint = 2;
    if (*ssi).slots != 0 {
        slots = (*ssi).slots;
    }
    if (*ssi).slot_width != 0 {
        slot_width = (*ssi).slot_width;
    }
    if channels == 2 && ((*ssi).i2s_net as u32 & SSI_SCR_I2S_MODE_MASK) == SSI_SCR_I2S_MODE_MASTER {
        slot_width = 32;
    }
    let freq = slots.wrapping_mul(slot_width).wrapping_mul(params_rate(hw_params));
    if IS_ERR((*ssi).baudclk as *const c_void) {
        return -EINVAL;
    }
    if freq.wrapping_mul(5) as c_ulong > clk_get_rate((*ssi).clk) {
        dev_err((*dai).dev, c"bitclk > ipgclk / 5\n".as_ptr());
        return -EINVAL;
    }
    let baudclk_is_used = ((*ssi).baudclk_streams & !BIT((*substream).stream as c_uint)) != 0;
    let factor = (div2 + 1).wrapping_mul(7u32.wrapping_mul(psr) + 1).wrapping_mul(2);
    let mut i: u32 = 0;
    while i < 255 {
        let tmprate = (freq as c_ulong).wrapping_mul(factor as c_ulong).wrapping_mul((i + 1) as c_ulong);
        let mut clkrate = if baudclk_is_used {
            clk_get_rate((*ssi).baudclk)
        } else {
            clk_round_rate((*ssi).baudclk, tmprate)
        };
        clkrate /= factor as c_ulong;
        let afreq = (clkrate / (i + 1) as c_ulong) as u32;
        let mut sub: u64;
        if freq == afreq {
            sub = 0;
        } else if freq / afreq == 1 {
            sub = (freq - afreq) as u64;
        } else if afreq / freq == 1 {
            sub = (afreq - freq) as u64;
        } else {
            i += 1;
            continue;
        }
        sub = sub.wrapping_mul(100000) / freq as u64;
        if sub < savesub && i != 0 {
            baudrate = tmprate;
            savesub = sub;
            pm = i;
        }
        if savesub == 0 {
            break;
        }
        i += 1;
    }
    if pm == 999 {
        dev_err((*dai).dev, c"failed to handle the required sysclk\n".as_ptr());
        return -EINVAL;
    }
    let stccr = SSI_SxCCR_PM(pm + 1);
    let mask = SSI_SxCCR_PM_MASK | SSI_SxCCR_DIV2 | SSI_SxCCR_PSR;
    let tx2 = tx || (*ssi).synchronous;
    regmap_update_bits(regs, REG_SSI_SxCCR(tx2), mask, stccr);
    if !baudclk_is_used {
        let ret = clk_set_rate((*ssi).baudclk, baudrate);
        if ret != 0 {
            dev_err((*dai).dev, c"failed to set baudclk rate\n".as_ptr());
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn fsl_ssi_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let ssi = snd_soc_dai_get_drvdata(dai) as *mut fsl_ssi;
    let vals = (*ssi).regvals.as_mut_ptr();
    let regs = (*ssi).regs;
    let channels = params_channels(hw_params);
    let sample_size = params_width(hw_params);
    let wl = SSI_SxCCR_WL(sample_size);
    if fsl_ssi_is_i2s_clock_provider(ssi) {
        let ret = fsl_ssi_set_bclk(substream, dai, hw_params);
        if ret != 0 {
            return ret;
        }
        if ((*ssi).baudclk_streams & BIT((*substream).stream as c_uint)) == 0 {
            let ret = clk_prepare_enable((*ssi).baudclk);
            if ret != 0 {
                return ret;
            }
            (*ssi).baudclk_streams |= BIT((*substream).stream as c_uint);
        }
    }
    if (*ssi).streams != 0 && (*ssi).synchronous {
        return 0;
    }
    if !fsl_ssi_is_ac97(ssi) {
        let mut i2s_net = (*ssi).i2s_net as u32;
        if fsl_ssi_is_i2s_bc_fp(ssi) && sample_size == 16 {
            i2s_net = SSI_SCR_I2S_MODE_NORMAL | SSI_SCR_NET;
        }
        if channels == 1 {
            i2s_net = SSI_SCR_I2S_MODE_NORMAL;
        }
        regmap_update_bits(regs, REG_SSI_SCR, SSI_SCR_I2S_NET_MASK, i2s_net);
    }
    let tx2 = tx || (*ssi).synchronous;
    regmap_update_bits(regs, REG_SSI_SxCCR(tx2), SSI_SxCCR_WL_MASK, wl);
    if (*ssi).use_dyna_fifo {
        if channels == 1 {
            (*ssi).audio_config[0].n_fifos_dst = 1;
            (*ssi).audio_config[1].n_fifos_src = 1;
            (*vals.add(RX)).srcr &= !SSI_SRCR_RFEN1;
            (*vals.add(TX)).stcr &= !SSI_STCR_TFEN1;
            (*vals.add(RX)).scr &= !SSI_SCR_TCH_EN;
            (*vals.add(TX)).scr &= !SSI_SCR_TCH_EN;
        } else {
            (*ssi).audio_config[0].n_fifos_dst = 2;
            (*ssi).audio_config[1].n_fifos_src = 2;
            (*vals.add(RX)).srcr |= SSI_SRCR_RFEN1;
            (*vals.add(TX)).stcr |= SSI_STCR_TFEN1;
            (*vals.add(RX)).scr |= SSI_SCR_TCH_EN;
            (*vals.add(TX)).scr |= SSI_SCR_TCH_EN;
        }
        (*ssi).dma_params_tx.peripheral_config = &mut (*ssi).audio_config[0] as *mut _ as *mut c_void;
        (*ssi).dma_params_tx.peripheral_size = size_of::<sdma_peripheral_config>();
        (*ssi).dma_params_rx.peripheral_config = &mut (*ssi).audio_config[1] as *mut _ as *mut c_void;
        (*ssi).dma_params_rx.peripheral_size = size_of::<sdma_peripheral_config>();
    }
    0
}

unsafe extern "C" fn fsl_ssi_hw_free(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let ssi = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_ssi;
    if fsl_ssi_is_i2s_clock_provider(ssi) && ((*ssi).baudclk_streams & BIT((*substream).stream as c_uint)) != 0 {
        clk_disable_unprepare((*ssi).baudclk);
        (*ssi).baudclk_streams &= !BIT((*substream).stream as c_uint);
    }
    0
}

unsafe fn _fsl_ssi_set_dai_fmt(ssi: *mut fsl_ssi, fmt: c_uint) -> c_int {
    let mut strcr: u32 = 0;
    let mut scr: u32 = 0;
    let stcr: u32;
    let mut srcr: u32;
    (*ssi).dai_fmt = fmt;
    scr |= SSI_SCR_SYNC_TX_FS;
    strcr |= SSI_STCR_TXBIT0;
    (*ssi).i2s_net = SSI_SCR_NET as u8;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
                SND_SOC_DAIFMT_BP_FP => {
                    if IS_ERR((*ssi).baudclk as *const c_void) {
                        dev_err((*ssi).dev, c"missing baudclk for master mode\n".as_ptr());
                        return -EINVAL;
                    }
                    (*ssi).i2s_net |= SSI_SCR_I2S_MODE_MASTER as u8;
                }
                SND_SOC_DAIFMT_BC_FP => (*ssi).i2s_net |= SSI_SCR_I2S_MODE_MASTER as u8,
                SND_SOC_DAIFMT_BC_FC => (*ssi).i2s_net |= SSI_SCR_I2S_MODE_SLAVE as u8,
                _ => return -EINVAL,
            }
            let slots = if (*ssi).slots != 0 { (*ssi).slots } else { 2 };
            regmap_update_bits((*ssi).regs, REG_SSI_STCCR, SSI_SxCCR_DC_MASK, SSI_SxCCR_DC(slots));
            regmap_update_bits((*ssi).regs, REG_SSI_SRCCR, SSI_SxCCR_DC_MASK, SSI_SxCCR_DC(slots));
            strcr |= SSI_STCR_TFSI | SSI_STCR_TSCKP | SSI_STCR_TEFS;
        }
        SND_SOC_DAIFMT_LEFT_J => strcr |= SSI_STCR_TSCKP,
        SND_SOC_DAIFMT_DSP_A => strcr |= SSI_STCR_TFSL | SSI_STCR_TSCKP | SSI_STCR_TEFS,
        SND_SOC_DAIFMT_DSP_B => strcr |= SSI_STCR_TFSL | SSI_STCR_TSCKP,
        SND_SOC_DAIFMT_AC97 => strcr |= SSI_STCR_TEFS,
        _ => return -EINVAL,
    }
    scr |= (*ssi).i2s_net as u32;
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => strcr ^= SSI_STCR_TSCKP,
        SND_SOC_DAIFMT_NB_IF => strcr ^= SSI_STCR_TFSI,
        SND_SOC_DAIFMT_IB_IF => {
            strcr ^= SSI_STCR_TSCKP;
            strcr ^= SSI_STCR_TFSI;
        }
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            strcr |= SSI_STCR_TFDIR | SSI_STCR_TXDIR;
            scr |= SSI_SCR_SYS_CLK_EN;
        }
        SND_SOC_DAIFMT_BC_FC => {}
        SND_SOC_DAIFMT_BC_FP => strcr |= SSI_STCR_TFDIR,
        _ => return -EINVAL,
    }
    stcr = strcr;
    srcr = strcr;
    if (*ssi).synchronous || fsl_ssi_is_ac97(ssi) {
        srcr &= !SSI_SRCR_RXDIR;
        scr |= SSI_SCR_SYN;
    }
    let mut mask = SSI_STCR_TFDIR | SSI_STCR_TXDIR | SSI_STCR_TSCKP | SSI_STCR_TFSL | SSI_STCR_TFSI | SSI_STCR_TEFS | SSI_STCR_TXBIT0;
    regmap_update_bits((*ssi).regs, REG_SSI_STCR, mask, stcr);
    regmap_update_bits((*ssi).regs, REG_SSI_SRCR, mask, srcr);
    mask = SSI_SCR_SYNC_TX_FS | SSI_SCR_I2S_MODE_MASK | SSI_SCR_SYS_CLK_EN | SSI_SCR_SYN;
    regmap_update_bits((*ssi).regs, REG_SSI_SCR, mask, scr);
    0
}

unsafe extern "C" fn fsl_ssi_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let ssi = snd_soc_dai_get_drvdata(dai) as *mut fsl_ssi;
    if fsl_ssi_is_ac97(ssi) {
        return 0;
    }
    _fsl_ssi_set_dai_fmt(ssi, fmt)
}

unsafe extern "C" fn fsl_ssi_set_dai_tdm_slot(dai: *mut snd_soc_dai, tx_mask: u32, rx_mask: u32, slots: c_int, slot_width: c_int) -> c_int {
    let ssi = snd_soc_dai_get_drvdata(dai) as *mut fsl_ssi;
    let regs = (*ssi).regs;
    let mut val: u32 = 0;
    if (slot_width & 1) != 0 || slot_width < 8 || slot_width > 24 {
        dev_err((*dai).dev, c"invalid slot width: %d\n".as_ptr(), slot_width);
        return -EINVAL;
    }
    if (*ssi).i2s_net != 0 && slots < 2 {
        dev_err((*dai).dev, c"slot number should be >= 2 in I2S or NET\n".as_ptr());
        return -EINVAL;
    }
    regmap_update_bits(regs, REG_SSI_STCCR, SSI_SxCCR_DC_MASK, SSI_SxCCR_DC(slots as c_uint));
    regmap_update_bits(regs, REG_SSI_SRCCR, SSI_SxCCR_DC_MASK, SSI_SxCCR_DC(slots as c_uint));
    regmap_read(regs, REG_SSI_SCR, &mut val);
    regmap_update_bits(regs, REG_SSI_SCR, SSI_SCR_SSIEN, SSI_SCR_SSIEN);
    regmap_write(regs, REG_SSI_STMSK, !tx_mask);
    regmap_write(regs, REG_SSI_SRMSK, !rx_mask);
    regmap_update_bits(regs, REG_SSI_SCR, SSI_SCR_SSIEN, val);
    (*ssi).slot_width = slot_width as c_uint;
    (*ssi).slots = slots as c_uint;
    0
}

unsafe extern "C" fn fsl_ssi_trigger(substream: *mut snd_pcm_substream, cmd: c_int, _dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let ssi = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_ssi;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if tx && fsl_ssi_is_ac97(ssi) {
                fsl_ssi_tx_ac97_saccst_setup(ssi);
            }
            fsl_ssi_config_enable(ssi, tx);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            fsl_ssi_config_disable(ssi, tx);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn fsl_ssi_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let ssi = snd_soc_dai_get_drvdata(dai) as *mut fsl_ssi;
    if (*(*ssi).soc).imx && (*ssi).use_dma {
        snd_soc_dai_init_dma_data(dai, &mut (*ssi).dma_params_tx, &mut (*ssi).dma_params_rx);
    }
    0
}

static fsl_ssi_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_ssi_dai_probe),
    startup: Some(fsl_ssi_startup),
    shutdown: Some(fsl_ssi_shutdown),
    hw_params: Some(fsl_ssi_hw_params),
    hw_free: Some(fsl_ssi_hw_free),
    set_fmt: Some(fsl_ssi_set_dai_fmt),
    set_tdm_slot: Some(fsl_ssi_set_dai_tdm_slot),
    trigger: Some(fsl_ssi_trigger),
};

static mut fsl_ssi_dai_template: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream { stream_name: c"CPU-Playback".as_ptr(), channels_min: 1, channels_max: 32, rates: SNDRV_PCM_RATE_CONTINUOUS, formats: FSLSSI_I2S_FORMATS },
    capture: snd_soc_pcm_stream { stream_name: c"CPU-Capture".as_ptr(), channels_min: 1, channels_max: 32, rates: SNDRV_PCM_RATE_CONTINUOUS, formats: FSLSSI_I2S_FORMATS },
    ops: &fsl_ssi_dai_ops,
    ..snd_soc_dai_driver::zeroed()
};

static fsl_ssi_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"fsl-ssi".as_ptr(),
    legacy_dai_naming: 1,
};

static mut fsl_ssi_ac97_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    symmetric_channels: 1,
    playback: snd_soc_pcm_stream { stream_name: c"CPU AC97 Playback".as_ptr(), channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S20 },
    capture: snd_soc_pcm_stream { stream_name: c"CPU AC97 Capture".as_ptr(), channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S20 },
    ops: &fsl_ssi_dai_ops,
    ..snd_soc_dai_driver::zeroed()
};

static mut fsl_ac97_data: *mut fsl_ssi = ptr::null_mut();

unsafe extern "C" fn fsl_ssi_ac97_write(_ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let regs = (*fsl_ac97_data).regs;
    let ret: c_int;
    if reg > 0x7f {
        return;
    }
    mutex_lock(&mut (*fsl_ac97_data).ac97_reg_lock);
    ret = clk_prepare_enable((*fsl_ac97_data).clk);
    if ret != 0 {
        pr_err(c"ac97 write clk_prepare_enable failed: %d\n".as_ptr(), ret);
        mutex_unlock(&mut (*fsl_ac97_data).ac97_reg_lock);
        return;
    }
    let lreg: c_uint = (reg as c_uint) << 12;
    regmap_write(regs, REG_SSI_SACADD, lreg);
    let lval: c_uint = (val as c_uint) << 4;
    regmap_write(regs, REG_SSI_SACDAT, lval);
    regmap_update_bits(regs, REG_SSI_SACNT, SSI_SACNT_RDWR_MASK, SSI_SACNT_WR);
    udelay(100);
    clk_disable_unprepare((*fsl_ac97_data).clk);
    mutex_unlock(&mut (*fsl_ac97_data).ac97_reg_lock);
}

unsafe extern "C" fn fsl_ssi_ac97_read(_ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let regs = (*fsl_ac97_data).regs;
    let mut val: c_ushort = 0;
    let mut reg_val: u32 = 0;
    mutex_lock(&mut (*fsl_ac97_data).ac97_reg_lock);
    let ret = clk_prepare_enable((*fsl_ac97_data).clk);
    if ret != 0 {
        pr_err(c"ac97 read clk_prepare_enable failed: %d\n".as_ptr(), ret);
        mutex_unlock(&mut (*fsl_ac97_data).ac97_reg_lock);
        return val;
    }
    let lreg: c_uint = ((reg & 0x7f) as c_uint) << 12;
    regmap_write(regs, REG_SSI_SACADD, lreg);
    regmap_update_bits(regs, REG_SSI_SACNT, SSI_SACNT_RDWR_MASK, SSI_SACNT_RD);
    udelay(100);
    regmap_read(regs, REG_SSI_SACDAT, &mut reg_val);
    val = ((reg_val >> 4) & 0xffff) as c_ushort;
    clk_disable_unprepare((*fsl_ac97_data).clk);
    mutex_unlock(&mut (*fsl_ac97_data).ac97_reg_lock);
    val
}

static fsl_ssi_ac97_ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
    read: Some(fsl_ssi_ac97_read),
    write: Some(fsl_ssi_ac97_write),
};

unsafe fn fsl_ssi_hw_init(ssi: *mut fsl_ssi) -> c_int {
    let wm = (*ssi).fifo_watermark;
    fsl_ssi_setup_regvals(ssi);
    regmap_write((*ssi).regs, REG_SSI_SFCSR, SSI_SFCSR_TFWM0(wm) | SSI_SFCSR_RFWM0(wm) | SSI_SFCSR_TFWM1(wm) | SSI_SFCSR_RFWM1(wm));
    if (*ssi).use_dual_fifo {
        regmap_update_bits((*ssi).regs, REG_SSI_SCR, SSI_SCR_TCH_EN, SSI_SCR_TCH_EN);
    }
    if fsl_ssi_is_ac97(ssi) {
        _fsl_ssi_set_dai_fmt(ssi, (*ssi).dai_fmt);
        fsl_ssi_setup_ac97(ssi);
    }
    0
}

unsafe fn fsl_ssi_hw_clean(ssi: *mut fsl_ssi) {
    if fsl_ssi_is_ac97(ssi) {
        regmap_update_bits((*ssi).regs, REG_SSI_SCR, SSI_SCR_TE | SSI_SCR_RE, 0);
        regmap_write((*ssi).regs, REG_SSI_SACNT, 0);
        regmap_write((*ssi).regs, REG_SSI_SOR, 0);
        regmap_update_bits((*ssi).regs, REG_SSI_SCR, SSI_SCR_SSIEN, 0);
    }
}

/*
 * Make every character in a string lower-case
 */
unsafe fn make_lowercase(mut s: *mut c_char) {
    if s.is_null() {
        return;
    }
    while *s != 0 {
        *s = tolower(*s as c_int) as c_char;
        s = s.add(1);
    }
}

unsafe fn fsl_ssi_imx_probe(pdev: *mut platform_device, ssi: *mut fsl_ssi, iomem: *mut c_void) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: c_int;
    if (*ssi).has_ipg_clk_name {
        (*ssi).clk = devm_clk_get(dev, c"ipg".as_ptr());
    } else {
        (*ssi).clk = devm_clk_get(dev, ptr::null());
    }
    if IS_ERR((*ssi).clk as *const c_void) {
        ret = PTR_ERR((*ssi).clk as *const c_void) as c_int;
        dev_err(dev, c"failed to get clock: %d\n".as_ptr(), ret);
        return ret;
    }
    if !(*ssi).has_ipg_clk_name {
        ret = clk_prepare_enable((*ssi).clk);
        if ret != 0 {
            dev_err(dev, c"clk_prepare_enable failed: %d\n".as_ptr(), ret);
            return ret;
        }
    }
    (*ssi).baudclk = devm_clk_get(dev, c"baud".as_ptr());
    if IS_ERR((*ssi).baudclk as *const c_void) {
        dev_dbg(dev, c"failed to get baud clock: %ld\n".as_ptr(), PTR_ERR((*ssi).baudclk as *const c_void));
    }
    (*ssi).dma_params_tx.maxburst = (*ssi).dma_maxburst;
    (*ssi).dma_params_rx.maxburst = (*ssi).dma_maxburst;
    (*ssi).dma_params_tx.addr = (*ssi).ssi_phys + REG_SSI_STX0 as usize;
    (*ssi).dma_params_rx.addr = (*ssi).ssi_phys + REG_SSI_SRX0 as usize;
    if (*ssi).use_dual_fifo || (*ssi).use_dyna_fifo {
        (*ssi).dma_params_tx.maxburst &= !0x1;
        (*ssi).dma_params_rx.maxburst &= !0x1;
    }
    if !(*ssi).use_dma {
        (*ssi).fiq_params.irq = (*ssi).irq;
        (*ssi).fiq_params.base = iomem;
        (*ssi).fiq_params.dma_params_rx = &mut (*ssi).dma_params_rx;
        (*ssi).fiq_params.dma_params_tx = &mut (*ssi).dma_params_tx;
        ret = imx_pcm_fiq_init(pdev, &mut (*ssi).fiq_params);
        if ret != 0 {
            if !(*ssi).has_ipg_clk_name {
                clk_disable_unprepare((*ssi).clk);
            }
            return ret;
        }
    } else {
        ret = imx_pcm_dma_init(pdev);
        if ret != 0 {
            dev_err_probe(dev, ret, c"Failed to init PCM DMA\n".as_ptr());
            if !(*ssi).has_ipg_clk_name {
                clk_disable_unprepare((*ssi).clk);
            }
            return ret;
        }
    }
    0
}

unsafe fn fsl_ssi_imx_clean(pdev: *mut platform_device, ssi: *mut fsl_ssi) {
    if !(*ssi).use_dma {
        imx_pcm_fiq_exit(pdev);
    }
    if !(*ssi).has_ipg_clk_name {
        clk_disable_unprepare((*ssi).clk);
    }
}

unsafe fn fsl_ssi_probe_from_dt(ssi: *mut fsl_ssi) -> c_int {
    let dev = (*ssi).dev;
    let np = (*dev).of_node;
    let mut dmas: [u32; 4] = [0; 4];
    let mut ret = of_property_match_string(np, c"clock-names".as_ptr(), c"ipg".as_ptr());
    (*ssi).has_ipg_clk_name = ret >= 0;
    let mut sprop = of_get_property(np, c"fsl,mode".as_ptr(), ptr::null_mut()) as *const c_char;
    if !sprop.is_null() && strcmp(sprop, c"ac97-slave".as_ptr()) == 0 {
        (*ssi).dai_fmt = FSLSSI_AC97_DAIFMT;
        ret = of_property_read_u32(np, c"cell-index".as_ptr(), &mut (*ssi).card_idx);
        if ret != 0 {
            dev_err(dev, c"failed to get SSI index property\n".as_ptr());
            return -EINVAL;
        }
        strscpy((*ssi).card_name.as_mut_ptr(), c"ac97-codec".as_ptr());
    } else if !of_property_read_bool(np, c"fsl,ssi-asynchronous".as_ptr()) {
        (*ssi).synchronous = true;
    }
    (*ssi).use_dma = !of_property_read_bool(np, c"fsl,fiq-stream-filter".as_ptr());
    let iprop = of_get_property(np, c"fsl,fifo-depth".as_ptr(), ptr::null_mut()) as *const __be32;
    if !iprop.is_null() {
        (*ssi).fifo_depth = be32_to_cpup(iprop);
    } else {
        (*ssi).fifo_depth = 8;
    }
    ret = of_property_read_u32_array(np, c"dmas".as_ptr(), dmas.as_mut_ptr(), 4);
    if (*ssi).use_dma && ret == 0 && dmas[2] == IMX_DMATYPE_SSI_DUAL {
        (*ssi).use_dual_fifo = true;
    }
    if (*ssi).use_dma && ret == 0 && dmas[2] == IMX_DMATYPE_MULTI_SAI {
        (*ssi).use_dyna_fifo = true;
    }
    if (*ssi).card_name[0] == 0 && !of_get_property(np, c"codec-handle".as_ptr(), ptr::null_mut()).is_null() {
        let root = of_find_node_by_path(c"/".as_ptr());
        sprop = of_get_property(root, c"compatible".as_ptr(), ptr::null_mut()) as *const c_char;
        of_node_put(root);
        let p = strrchr(sprop, ',' as c_int);
        if !p.is_null() {
            sprop = p.add(1);
        }
        snprintf((*ssi).card_name.as_mut_ptr(), (*ssi).card_name.len(), c"snd-soc-%s".as_ptr(), sprop);
        make_lowercase((*ssi).card_name.as_mut_ptr());
        (*ssi).card_idx = 0;
    }
    0
}

unsafe extern "C" fn fsl_ssi_probe(pdev: *mut platform_device) -> c_int {
    let mut regconfig = fsl_ssi_regconfig;
    let dev = &mut (*pdev).dev as *mut device;
    let mut res: *mut resource = ptr::null_mut();
    let mut ret: c_int = 0;
    let ssi = devm_kzalloc(dev, size_of::<fsl_ssi>(), GFP_KERNEL) as *mut fsl_ssi;
    if ssi.is_null() {
        return -ENOMEM;
    }
    (*ssi).dev = dev;
    (*ssi).soc = of_device_get_match_data(dev) as *const fsl_ssi_soc_data;
    ret = fsl_ssi_probe_from_dt(ssi);
    if ret != 0 {
        return ret;
    }
    if fsl_ssi_is_ac97(ssi) {
        memcpy(&mut (*ssi).cpu_dai_drv as *mut _ as *mut c_void, &raw const fsl_ssi_ac97_dai as *const c_void, size_of::<snd_soc_dai_driver>());
        fsl_ac97_data = ssi;
    } else {
        memcpy(&mut (*ssi).cpu_dai_drv as *mut _ as *mut c_void, &raw const fsl_ssi_dai_template as *const c_void, size_of::<snd_soc_dai_driver>());
    }
    (*ssi).cpu_dai_drv.name = dev_name(dev);
    let iomem = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(iomem as *const c_void) {
        return PTR_ERR(iomem as *const c_void) as c_int;
    }
    (*ssi).ssi_phys = (*res).start;
    if (*(*ssi).soc).imx21regs {
        regconfig.max_register = REG_SSI_SRMSK;
        regconfig.num_reg_defaults_raw = REG_SSI_SRMSK / size_of::<u32>() as c_uint + 1;
    }
    if (*ssi).has_ipg_clk_name {
        (*ssi).regs = devm_regmap_init_mmio_clk(dev, c"ipg".as_ptr(), iomem, &regconfig);
    } else {
        (*ssi).regs = devm_regmap_init_mmio(dev, iomem, &regconfig);
    }
    if IS_ERR((*ssi).regs as *const c_void) {
        dev_err(dev, c"failed to init register map\n".as_ptr());
        return PTR_ERR((*ssi).regs as *const c_void) as c_int;
    }
    (*ssi).irq = platform_get_irq(pdev, 0);
    if (*ssi).irq < 0 {
        return (*ssi).irq;
    }
    if (*ssi).synchronous && !fsl_ssi_is_ac97(ssi) {
        (*ssi).cpu_dai_drv.symmetric_rate = 1;
        (*ssi).cpu_dai_drv.symmetric_channels = 1;
        (*ssi).cpu_dai_drv.symmetric_sample_bits = 1;
    }
    match (*ssi).fifo_depth {
        15 => {
            (*ssi).fifo_watermark = 8;
            (*ssi).dma_maxburst = 8;
        }
        _ => {
            (*ssi).fifo_watermark = (*ssi).fifo_depth - 2;
            (*ssi).dma_maxburst = (*ssi).fifo_depth - 2;
        }
    }
    dev_set_drvdata(dev, ssi as *mut c_void);
    if (*(*ssi).soc).imx {
        ret = fsl_ssi_imx_probe(pdev, ssi, iomem);
        if ret != 0 {
            return ret;
        }
    }
    if fsl_ssi_is_ac97(ssi) {
        mutex_init(&mut (*ssi).ac97_reg_lock);
        ret = snd_soc_set_ac97_ops_of_reset(&fsl_ssi_ac97_ops, pdev);
        if ret != 0 {
            dev_err(dev, c"failed to set AC'97 ops\n".as_ptr());
            if fsl_ssi_is_ac97(ssi) {
                mutex_destroy(&mut (*ssi).ac97_reg_lock);
            }
            if (*(*ssi).soc).imx {
                fsl_ssi_imx_clean(pdev, ssi);
            }
            return ret;
        }
    }
    ret = devm_snd_soc_register_component(dev, &fsl_ssi_component, &mut (*ssi).cpu_dai_drv, 1);
    if ret != 0 {
        dev_err(dev, c"failed to register DAI: %d\n".as_ptr(), ret);
        if fsl_ssi_is_ac97(ssi) {
            snd_soc_set_ac97_ops(ptr::null());
            mutex_destroy(&mut (*ssi).ac97_reg_lock);
        }
        if (*(*ssi).soc).imx {
            fsl_ssi_imx_clean(pdev, ssi);
        }
        return ret;
    }
    if (*ssi).use_dma {
        ret = devm_request_irq(dev, (*ssi).irq, Some(fsl_ssi_isr), 0, dev_name(dev), ssi as *mut c_void);
        if ret < 0 {
            dev_err(dev, c"failed to claim irq %u\n".as_ptr(), (*ssi).irq);
            if fsl_ssi_is_ac97(ssi) {
                snd_soc_set_ac97_ops(ptr::null());
                mutex_destroy(&mut (*ssi).ac97_reg_lock);
            }
            if (*(*ssi).soc).imx {
                fsl_ssi_imx_clean(pdev, ssi);
            }
            return ret;
        }
    }
    fsl_ssi_debugfs_create(&mut (*ssi).dbg_stats, dev);
    fsl_ssi_hw_init(ssi);
    if (*ssi).card_name[0] != 0 {
        let mut parent = dev;
        if fsl_ssi_is_ac97(ssi) {
            parent = ptr::null_mut();
        }
        (*ssi).card_pdev = platform_device_register_data(parent, (*ssi).card_name.as_mut_ptr(), (*ssi).card_idx, ptr::null(), 0);
        if IS_ERR((*ssi).card_pdev as *const c_void) {
            ret = PTR_ERR((*ssi).card_pdev as *const c_void) as c_int;
            dev_err(dev, c"failed to register %s: %d\n".as_ptr(), (*ssi).card_name.as_mut_ptr(), ret);
            fsl_ssi_debugfs_remove(&mut (*ssi).dbg_stats);
            if fsl_ssi_is_ac97(ssi) {
                snd_soc_set_ac97_ops(ptr::null());
                mutex_destroy(&mut (*ssi).ac97_reg_lock);
            }
            if (*(*ssi).soc).imx {
                fsl_ssi_imx_clean(pdev, ssi);
            }
            return ret;
        }
    }
    0
}

unsafe extern "C" fn fsl_ssi_remove(pdev: *mut platform_device) {
    let ssi = dev_get_drvdata(&mut (*pdev).dev) as *mut fsl_ssi;
    fsl_ssi_debugfs_remove(&mut (*ssi).dbg_stats);
    if !(*ssi).card_pdev.is_null() {
        platform_device_unregister((*ssi).card_pdev);
    }
    fsl_ssi_hw_clean(ssi);
    if (*(*ssi).soc).imx {
        fsl_ssi_imx_clean(pdev, ssi);
    }
    if fsl_ssi_is_ac97(ssi) {
        snd_soc_set_ac97_ops(ptr::null());
        mutex_destroy(&mut (*ssi).ac97_reg_lock);
    }
}

unsafe extern "C" fn fsl_ssi_suspend(dev: *mut device) -> c_int {
    let ssi = dev_get_drvdata(dev) as *mut fsl_ssi;
    let regs = (*ssi).regs;
    regmap_read(regs, REG_SSI_SFCSR, &mut (*ssi).regcache_sfcsr);
    regmap_read(regs, REG_SSI_SACNT, &mut (*ssi).regcache_sacnt);
    regcache_cache_only(regs, true);
    regcache_mark_dirty(regs);
    0
}

unsafe extern "C" fn fsl_ssi_resume(dev: *mut device) -> c_int {
    let ssi = dev_get_drvdata(dev) as *mut fsl_ssi;
    let regs = (*ssi).regs;
    regcache_cache_only(regs, false);
    regmap_update_bits(regs, REG_SSI_SFCSR, SSI_SFCSR_RFWM1_MASK | SSI_SFCSR_TFWM1_MASK | SSI_SFCSR_RFWM0_MASK | SSI_SFCSR_TFWM0_MASK, (*ssi).regcache_sfcsr);
    regmap_write(regs, REG_SSI_SACNT, (*ssi).regcache_sacnt);
    regcache_sync(regs)
}

static fsl_ssi_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(fsl_ssi_suspend, fsl_ssi_resume)
    suspend: Some(fsl_ssi_suspend),
    resume: Some(fsl_ssi_resume),
};

static mut fsl_ssi_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"fsl-ssi-dai".as_ptr(),
        of_match_table: fsl_ssi_ids.as_ptr(),
        pm: &fsl_ssi_pm,
    },
    probe: Some(fsl_ssi_probe),
    remove: Some(fsl_ssi_remove),
};

// module_platform_driver(fsl_ssi_driver);
// MODULE_ALIAS("platform:fsl-ssi-dai");
// MODULE_AUTHOR("Timur Tabi <timur@freescale.com>");
// MODULE_DESCRIPTION("Freescale Synchronous Serial Interface (SSI) ASoC Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
