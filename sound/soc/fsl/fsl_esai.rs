// SPDX-License-Identifier: GPL-2.0
//
// Freescale ESAI ALSA SoC Digital Audio Interface (DAI) driver
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Dependencies originally provided by Linux and local C headers:
// <linux/clk.h>, <linux/dmaengine.h>, <linux/module.h>, <linux/of_irq.h>,
// <linux/of_platform.h>, <linux/pm_runtime.h>, <sound/dmaengine_pcm.h>,
// <sound/pcm_params.h>, "fsl_esai.h", and "imx-pcm.h".

const FSL_ESAI_FORMATS: u32 =
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: resource_size_t,
    pub maxburst: u32,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub channels: u32,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32, u32, c_int, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_driver,
}

pub type irqreturn_t = c_int;
pub type resource_size_t = u64;
pub type __be32 = u32;

/**
 * struct fsl_esai_soc_data - soc specific data
 * @reset_at_xrun: flags for enable reset operaton
 */
#[repr(C)]
pub struct fsl_esai_soc_data {
    reset_at_xrun: bool,
}

/**
 * struct fsl_esai - ESAI private data
 * @dma_params_rx: DMA parameters for receive channel
 * @dma_params_tx: DMA parameters for transmit channel
 * @pdev: platform device pointer
 * @regmap: regmap handler
 * @coreclk: clock source to access register
 * @extalclk: esai clock source to derive HCK, SCK and FS
 * @fsysclk: system clock source to derive HCK, SCK and FS
 * @spbaclk: SPBA clock (optional, depending on SoC design)
 * @work: work to handle the reset operation
 * @soc: soc specific data
 * @lock: spin lock between hw_reset() and trigger()
 * @fifo_depth: depth of tx/rx FIFO
 * @slot_width: width of each DAI slot
 * @slots: number of slots
 * @tx_mask: slot mask for TX
 * @rx_mask: slot mask for RX
 * @channels: channel num for tx or rx
 * @hck_rate: clock rate of desired HCKx clock
 * @sck_rate: clock rate of desired SCKx clock
 * @hck_dir: the direction of HCKx pads
 * @sck_div: if using PSR/PM dividers for SCKx clock
 * @consumer_mode: if fully using DAI clock consumer mode
 * @synchronous: if using tx/rx synchronous mode
 * @name: driver name
 */
#[repr(C)]
pub struct fsl_esai {
    dma_params_rx: snd_dmaengine_dai_dma_data,
    dma_params_tx: snd_dmaengine_dai_dma_data,
    pdev: *mut platform_device,
    regmap: *mut regmap,
    coreclk: *mut clk,
    extalclk: *mut clk,
    fsysclk: *mut clk,
    spbaclk: *mut clk,
    work: work_struct,
    soc: *const fsl_esai_soc_data,
    lock: spinlock_t, /* Protect hw_reset and trigger */
    fifo_depth: u32,
    slot_width: u32,
    slots: u32,
    tx_mask: u32,
    rx_mask: u32,
    channels: [u32; 2],
    hck_rate: [u32; 2],
    sck_rate: [u32; 2],
    hck_dir: [bool; 2],
    sck_div: [bool; 2],
    consumer_mode: bool,
    synchronous: bool,
    name: [c_char; 32],
}

static mut fsl_esai_vf610: fsl_esai_soc_data = fsl_esai_soc_data {
    reset_at_xrun: true,
};

static mut fsl_esai_imx35: fsl_esai_soc_data = fsl_esai_soc_data {
    reset_at_xrun: true,
};

static mut fsl_esai_imx6ull: fsl_esai_soc_data = fsl_esai_soc_data {
    reset_at_xrun: false,
};

#[inline]
fn bidx(v: bool) -> usize {
    v as usize
}

unsafe extern "C" fn esai_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let esai_priv = devid as *mut fsl_esai;
    let pdev = (*esai_priv).pdev;
    let mut esr: u32 = 0;
    let mut saisr: u32 = 0;

    regmap_read((*esai_priv).regmap, REG_ESAI_ESR, &mut esr);
    regmap_read((*esai_priv).regmap, REG_ESAI_SAISR, &mut saisr);

    if (saisr & (ESAI_SAISR_TUE | ESAI_SAISR_ROE)) != 0 && (*(*esai_priv).soc).reset_at_xrun {
        dev_dbg(&mut (*pdev).dev, c"reset module for xrun\n".as_ptr());
        regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCR, ESAI_xCR_xEIE_MASK, 0);
        regmap_update_bits((*esai_priv).regmap, REG_ESAI_RCR, ESAI_xCR_xEIE_MASK, 0);
        schedule_work(&mut (*esai_priv).work);
    }

    if (esr & ESAI_ESR_TINIT_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Transmission Initialized\n".as_ptr());
    }
    if (esr & ESAI_ESR_RFF_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Receiving overrun\n".as_ptr());
    }
    if (esr & ESAI_ESR_TFE_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Transmission underrun\n".as_ptr());
    }
    if (esr & ESAI_ESR_TLS_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Just transmitted the last slot\n".as_ptr());
    }
    if (esr & ESAI_ESR_TDE_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Transmission data exception\n".as_ptr());
    }
    if (esr & ESAI_ESR_TED_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Transmitting even slots\n".as_ptr());
    }
    if (esr & ESAI_ESR_TD_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Transmitting data\n".as_ptr());
    }
    if (esr & ESAI_ESR_RLS_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Just received the last slot\n".as_ptr());
    }
    if (esr & ESAI_ESR_RDE_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Receiving data exception\n".as_ptr());
    }
    if (esr & ESAI_ESR_RED_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Receiving even slots\n".as_ptr());
    }
    if (esr & ESAI_ESR_RD_MASK) != 0 {
        dev_dbg(&mut (*pdev).dev, c"isr: Receiving data\n".as_ptr());
    }

    IRQ_HANDLED
}

/**
 * fsl_esai_divisor_cal - This function is used to calculate the
 * divisors of psr, pm, fp and it is supposed to be called in
 * set_dai_sysclk() and set_bclk().
 *
 * @dai: pointer to DAI
 * @tx: current setting is for playback or capture
 * @ratio: desired overall ratio for the paticipating dividers
 * @usefp: for HCK setting, there is no need to set fp divider
 * @fp: bypass other dividers by setting fp directly if fp != 0
 */
unsafe extern "C" fn fsl_esai_divisor_cal(
    dai: *mut snd_soc_dai,
    tx: bool,
    mut ratio: u32,
    usefp: bool,
    mut fp: u32,
) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;
    let mut pm: u32 = 999;
    let maxfp: u32;
    let mut prod: u32;
    let mut sub: u32;
    let mut savesub: u32;
    let mut i: u32;
    let mut j: u32;
    let psr: u32;

    maxfp = if usefp { 16 } else { 1 };

    if usefp && fp != 0 {
        return out_fp(esai_priv, tx, maxfp, fp);
    }

    if ratio > 2 * 8 * 256 * maxfp || ratio < 2 {
        dev_err((*dai).dev, c"the ratio is out of range (2 ~ %d)\n".as_ptr(), 2 * 8 * 256 * maxfp);
        return -EINVAL;
    } else if ratio % 2 != 0 {
        dev_err((*dai).dev, c"the raio must be even if using upper divider\n".as_ptr());
        return -EINVAL;
    }

    ratio /= 2;

    psr = if ratio <= 256 * maxfp {
        ESAI_xCCR_xPSR_BYPASS
    } else {
        ESAI_xCCR_xPSR_DIV8
    };

    /* Do not loop-search if PM (1 ~ 256) alone can serve the ratio */
    if ratio <= 256 {
        pm = ratio;
        fp = 1;
    } else {
        /* Set the max fluctuation -- 0.1% of the max devisor */
        savesub = (if psr != 0 { 1 } else { 8 }) * 256 * maxfp / 1000;

        /* Find the best value for PM */
        i = 1;
        'outer: while i <= 256 {
            j = 1;
            while j <= maxfp {
                /* PSR (1 or 8) * PM (1 ~ 256) * FP (1 ~ 16) */
                prod = (if psr != 0 { 1 } else { 8 }) * i * j;

                if prod == ratio {
                    sub = 0;
                } else if prod / ratio == 1 {
                    sub = prod - ratio;
                } else if ratio / prod == 1 {
                    sub = ratio - prod;
                } else {
                    j += 1;
                    continue;
                }

                /* Calculate the fraction */
                sub = sub * 1000 / ratio;
                if sub < savesub {
                    savesub = sub;
                    pm = i;
                    fp = j;
                }

                /* We are lucky */
                if savesub == 0 {
                    break 'outer;
                }
                j += 1;
            }
            i += 1;
        }

        if pm == 999 {
            dev_err((*dai).dev, c"failed to calculate proper divisors\n".as_ptr());
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_xCCR(tx),
        ESAI_xCCR_xPSR_MASK | ESAI_xCCR_xPM_MASK,
        psr | ESAI_xCCR_xPM(pm),
    );

    out_fp(esai_priv, tx, maxfp, fp)
}

unsafe fn out_fp(esai_priv: *mut fsl_esai, tx: bool, maxfp: u32, fp: u32) -> c_int {
    /* Bypass fp if not being required */
    if maxfp <= 1 {
        return 0;
    }

    regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_xCCR(tx),
        ESAI_xCCR_xFP_MASK,
        ESAI_xCCR_xFP(fp),
    );

    0
}

/**
 * fsl_esai_set_dai_sysclk - configure the clock frequency of MCLK (HCKT/HCKR)
 * @dai: pointer to DAI
 * @clk_id: The clock source of HCKT/HCKR
 *	  (Input from outside; output from inside, FSYS or EXTAL)
 * @freq: The required clock rate of HCKT/HCKR
 * @dir: The clock direction of HCKT/HCKR
 *
 * Note: If the direction is input, we do not care about clk_id.
 */
unsafe extern "C" fn fsl_esai_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;
    let mut clksrc = (*esai_priv).extalclk;
    let tx = clk_id <= ESAI_HCKT_EXTAL || (*esai_priv).synchronous;
    let in_ = dir == SND_SOC_CLOCK_IN;
    let mut ecr: u32 = 0;
    let clk_rate: c_ulong;
    let ratio: u32;
    let mut ret: c_int;

    if freq == 0 {
        dev_err(
            (*dai).dev,
            c"%sput freq of HCK%c should not be 0Hz\n".as_ptr(),
            if in_ { c"in".as_ptr() } else { c"out".as_ptr() },
            if tx { 'T' as c_int } else { 'R' as c_int },
        );
        return -EINVAL;
    }

    /* Bypass divider settings if the requirement doesn't change */
    if freq == (*esai_priv).hck_rate[bidx(tx)] && dir == (*esai_priv).hck_dir[bidx(tx)] as c_int {
        return 0;
    }

    /* sck_div can be only bypassed if ETO/ERO=0 and SNC_SOC_CLOCK_OUT */
    (*esai_priv).sck_div[bidx(tx)] = true;

    /* Set the direction of HCKT/HCKR pins */
    regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_xCCR(tx),
        ESAI_xCCR_xHCKD,
        if in_ { 0 } else { ESAI_xCCR_xHCKD },
    );

    if !in_ {
        match clk_id {
            ESAI_HCKT_FSYS | ESAI_HCKR_FSYS => clksrc = (*esai_priv).fsysclk,
            ESAI_HCKT_EXTAL => ecr |= ESAI_ECR_ETI,
            ESAI_HCKR_EXTAL => ecr |= if (*esai_priv).synchronous { ESAI_ECR_ETI } else { ESAI_ECR_ERI },
            _ => return -EINVAL,
        }

        if IS_ERR(clksrc as *const c_void) {
            dev_err(
                (*dai).dev,
                c"no assigned %s clock\n".as_ptr(),
                if clk_id % 2 != 0 { c"extal".as_ptr() } else { c"fsys".as_ptr() },
            );
            return PTR_ERR(clksrc as *const c_void) as c_int;
        }
        clk_rate = clk_get_rate(clksrc);

        ratio = (clk_rate / freq as c_ulong) as u32;
        if (ratio as c_ulong) * freq as c_ulong > clk_rate {
            ret = ((ratio as c_ulong) * freq as c_ulong - clk_rate) as c_int;
        } else if (ratio as c_ulong) * freq as c_ulong < clk_rate {
            ret = (clk_rate - (ratio as c_ulong) * freq as c_ulong) as c_int;
        } else {
            ret = 0;
        }

        /* Block if clock source can not be divided into the required rate */
        if ret != 0 && clk_rate / ret as c_ulong < 1000 {
            dev_err((*dai).dev, c"failed to derive required HCK%c rate\n".as_ptr(), if tx { 'T' as c_int } else { 'R' as c_int });
            return -EINVAL;
        }

        /* Only EXTAL source can be output directly without using PSR and PM */
        if ratio == 1 && clksrc == (*esai_priv).extalclk {
            /* Bypass all the dividers if not being needed */
            ecr |= if tx { ESAI_ECR_ETO } else { ESAI_ECR_ERO };
        } else if ratio < 2 {
            /* The ratio should be no less than 2 if using other sources */
            dev_err((*dai).dev, c"failed to derive required HCK%c rate\n".as_ptr(), if tx { 'T' as c_int } else { 'R' as c_int });
            return -EINVAL;
        } else {
            ret = fsl_esai_divisor_cal(dai, tx, ratio, false, 0);
            if ret != 0 {
                return ret;
            }
            (*esai_priv).sck_div[bidx(tx)] = false;
        }
    }

    (*esai_priv).hck_dir[bidx(tx)] = dir != 0;
    (*esai_priv).hck_rate[bidx(tx)] = freq;

    regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_ECR,
        if tx { ESAI_ECR_ETI | ESAI_ECR_ETO } else { ESAI_ECR_ERI | ESAI_ECR_ERO },
        ecr,
    );

    0
}

/**
 * fsl_esai_set_bclk - configure the related dividers according to the bclk rate
 * @dai: pointer to DAI
 * @tx: direction boolean
 * @freq: bclk freq
 */
unsafe extern "C" fn fsl_esai_set_bclk(dai: *mut snd_soc_dai, tx: bool, freq: u32) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;
    let hck_rate = (*esai_priv).hck_rate[bidx(tx)];
    let mut sub: u32;
    let ratio = hck_rate / freq;
    let mut ret: c_int;

    /* Don't apply for fully consumer mode or unchanged bclk */
    if (*esai_priv).consumer_mode || (*esai_priv).sck_rate[bidx(tx)] == freq {
        return 0;
    }

    if ratio * freq > hck_rate {
        sub = ratio * freq - hck_rate;
    } else if ratio * freq < hck_rate {
        sub = hck_rate - ratio * freq;
    } else {
        sub = 0;
    }

    /* Block if clock source can not be divided into the required rate */
    if sub != 0 && hck_rate / sub < 1000 {
        dev_err((*dai).dev, c"failed to derive required SCK%c rate\n".as_ptr(), if tx { 'T' as c_int } else { 'R' as c_int });
        return -EINVAL;
    }

    /* The ratio should be contented by FP alone if bypassing PM and PSR */
    if !(*esai_priv).sck_div[bidx(tx)] && (ratio > 16 || ratio == 0) {
        dev_err((*dai).dev, c"the ratio is out of range (1 ~ 16)\n".as_ptr());
        return -EINVAL;
    }

    ret = fsl_esai_divisor_cal(
        dai,
        tx,
        ratio,
        true,
        if (*esai_priv).sck_div[bidx(tx)] { 0 } else { ratio },
    );
    if ret != 0 {
        return ret;
    }

    /* Save current bclk rate */
    (*esai_priv).sck_rate[bidx(tx)] = freq;

    0
}

unsafe extern "C" fn fsl_esai_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: u32,
    rx_mask: u32,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;

    regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCCR, ESAI_xCCR_xDC_MASK, ESAI_xCCR_xDC(slots as u32));
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_RCCR, ESAI_xCCR_xDC_MASK, ESAI_xCCR_xDC(slots as u32));

    (*esai_priv).slot_width = slot_width as u32;
    (*esai_priv).slots = slots as u32;
    (*esai_priv).tx_mask = tx_mask;
    (*esai_priv).rx_mask = rx_mask;

    0
}

unsafe extern "C" fn fsl_esai_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;
    let mut xcr: u32 = 0;
    let mut xccr: u32 = 0;
    let mut mask: u32;

    /* DAI mode */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            /* Data on rising edge of bclk, frame low, 1clk before data */
            xcr |= ESAI_xCR_xFSR;
            xccr |= ESAI_xCCR_xFSP | ESAI_xCCR_xCKP | ESAI_xCCR_xHCKP;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            /* Data on rising edge of bclk, frame high */
            xccr |= ESAI_xCCR_xCKP | ESAI_xCCR_xHCKP;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            /* Data on rising edge of bclk, frame high, right aligned */
            xccr |= ESAI_xCCR_xCKP | ESAI_xCCR_xHCKP;
            xcr |= ESAI_xCR_xWA;
        }
        SND_SOC_DAIFMT_DSP_A => {
            /* Data on rising edge of bclk, frame high, 1clk before data */
            xcr |= ESAI_xCR_xFSL | ESAI_xCR_xFSR;
            xccr |= ESAI_xCCR_xCKP | ESAI_xCCR_xHCKP;
        }
        SND_SOC_DAIFMT_DSP_B => {
            /* Data on rising edge of bclk, frame high */
            xcr |= ESAI_xCR_xFSL;
            xccr |= ESAI_xCCR_xCKP | ESAI_xCCR_xHCKP;
        }
        _ => return -EINVAL,
    }

    /* DAI clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => xccr ^= ESAI_xCCR_xCKP | ESAI_xCCR_xHCKP,
        SND_SOC_DAIFMT_NB_IF => xccr ^= ESAI_xCCR_xFSP,
        SND_SOC_DAIFMT_IB_IF => xccr ^= ESAI_xCCR_xCKP | ESAI_xCCR_xHCKP | ESAI_xCCR_xFSP,
        _ => return -EINVAL,
    }

    (*esai_priv).consumer_mode = false;

    /* DAI clock provider masks */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => (*esai_priv).consumer_mode = true,
        SND_SOC_DAIFMT_BP_FC => xccr |= ESAI_xCCR_xCKD,
        SND_SOC_DAIFMT_BC_FP => xccr |= ESAI_xCCR_xFSD,
        SND_SOC_DAIFMT_BP_FP => xccr |= ESAI_xCCR_xFSD | ESAI_xCCR_xCKD,
        _ => return -EINVAL,
    }

    mask = ESAI_xCR_xFSL | ESAI_xCR_xFSR | ESAI_xCR_xWA;
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCR, mask, xcr);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_RCR, mask, xcr);

    mask = ESAI_xCCR_xCKP | ESAI_xCCR_xHCKP | ESAI_xCCR_xFSP | ESAI_xCCR_xFSD | ESAI_xCCR_xCKD;
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCCR, mask, xccr);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_RCCR, mask, xccr);

    0
}

unsafe extern "C" fn fsl_esai_startup(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;

    if snd_soc_dai_active(dai) == 0 {
        /* Set synchronous mode */
        regmap_update_bits(
            (*esai_priv).regmap,
            REG_ESAI_SAICR,
            ESAI_SAICR_SYNC,
            if (*esai_priv).synchronous { ESAI_SAICR_SYNC } else { 0 },
        );

        /* Set slots count */
        regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCCR, ESAI_xCCR_xDC_MASK, ESAI_xCCR_xDC((*esai_priv).slots));
        regmap_update_bits((*esai_priv).regmap, REG_ESAI_RCCR, ESAI_xCCR_xDC_MASK, ESAI_xCCR_xDC((*esai_priv).slots));
    }

    0
}

unsafe extern "C" fn fsl_esai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let width = params_width(params);
    let channels = params_channels(params);
    let pins = DIV_ROUND_UP(channels, (*esai_priv).slots);
    let mut slot_width = width;
    let bclk: u32;
    let mut mask: u32;
    let mut val: u32;
    let ret: c_int;

    /* Override slot_width if being specifically set */
    if (*esai_priv).slot_width != 0 {
        slot_width = (*esai_priv).slot_width;
    }

    bclk = params_rate(params) * slot_width * (*esai_priv).slots;

    ret = fsl_esai_set_bclk(dai, (*esai_priv).synchronous || tx, bclk);
    if ret != 0 {
        return ret;
    }

    mask = ESAI_xCR_xSWS_MASK;
    val = ESAI_xCR_xSWS(slot_width, width);

    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xCR(tx), mask, val);
    /* Recording in synchronous mode needs to set TCR also */
    if !tx && (*esai_priv).synchronous {
        regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCR, mask, val);
    }

    /* Use Normal mode to support monaural audio */
    regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_xCR(tx),
        ESAI_xCR_xMOD_MASK,
        if params_channels(params) > 1 { ESAI_xCR_xMOD_NETWORK } else { 0 },
    );

    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xFCR(tx), ESAI_xFCR_xFR_MASK, ESAI_xFCR_xFR);

    mask = ESAI_xFCR_xFR_MASK
        | ESAI_xFCR_xWA_MASK
        | ESAI_xFCR_xFWM_MASK
        | if tx { ESAI_xFCR_TE_MASK | ESAI_xFCR_TIEN } else { ESAI_xFCR_RE_MASK };
    val = ESAI_xFCR_xWA(width)
        | ESAI_xFCR_xFWM((*esai_priv).fifo_depth)
        | if tx { ESAI_xFCR_TE(pins) | ESAI_xFCR_TIEN } else { ESAI_xFCR_RE(pins) };

    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xFCR(tx), mask, val);

    if tx {
        regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCR, ESAI_xCR_PADC, ESAI_xCR_PADC);
    }

    /* Remove ESAI personal reset by configuring ESAI_PCRC and ESAI_PRRC */
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_PRRC, ESAI_PRRC_PDC_MASK, ESAI_PRRC_PDC(ESAI_GPIO));
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_PCRC, ESAI_PCRC_PC_MASK, ESAI_PCRC_PC(ESAI_GPIO));
    0
}

unsafe extern "C" fn fsl_esai_hw_init(esai_priv: *mut fsl_esai) -> c_int {
    let pdev = (*esai_priv).pdev;
    let mut ret: c_int;

    /* Reset ESAI unit */
    ret = regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_ECR,
        ESAI_ECR_ESAIEN_MASK | ESAI_ECR_ERST_MASK,
        ESAI_ECR_ESAIEN | ESAI_ECR_ERST,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"failed to reset ESAI: %d\n".as_ptr(), ret);
        return ret;
    }

    /*
     * We need to enable ESAI so as to access some of its registers.
     * Otherwise, we would fail to dump regmap from user space.
     */
    ret = regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_ECR,
        ESAI_ECR_ESAIEN_MASK | ESAI_ECR_ERST_MASK,
        ESAI_ECR_ESAIEN,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"failed to enable ESAI: %d\n".as_ptr(), ret);
        return ret;
    }

    regmap_update_bits((*esai_priv).regmap, REG_ESAI_PRRC, ESAI_PRRC_PDC_MASK, 0);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_PCRC, ESAI_PCRC_PC_MASK, 0);

    0
}

unsafe extern "C" fn fsl_esai_register_restore(esai_priv: *mut fsl_esai) -> c_int {
    let ret: c_int;

    /* FIFO reset for safety */
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_TFCR, ESAI_xFCR_xFR, ESAI_xFCR_xFR);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_RFCR, ESAI_xFCR_xFR, ESAI_xFCR_xFR);

    regcache_mark_dirty((*esai_priv).regmap);
    ret = regcache_sync((*esai_priv).regmap);
    if ret != 0 {
        return ret;
    }

    /* FIFO reset done */
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_TFCR, ESAI_xFCR_xFR, 0);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_RFCR, ESAI_xFCR_xFR, 0);

    0
}

unsafe extern "C" fn fsl_esai_trigger_start(esai_priv: *mut fsl_esai, tx: bool) {
    let mut i: u8;
    let channels = (*esai_priv).channels[bidx(tx)] as u8;
    let pins = DIV_ROUND_UP(channels as u32, (*esai_priv).slots);
    let mask: u32;

    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xFCR(tx), ESAI_xFCR_xFEN_MASK, ESAI_xFCR_xFEN);

    /* Write initial words reqiured by ESAI as normal procedure */
    i = 0;
    while tx && i < channels {
        regmap_write((*esai_priv).regmap, REG_ESAI_ETDR, 0x0);
        i = i.wrapping_add(1);
    }

    /*
     * When set the TE/RE in the end of enablement flow, there
     * will be channel swap issue for multi data line case.
     * In order to workaround this issue, we switch the bit
     * enablement sequence to below sequence
     * 1) clear the xSMB & xSMA: which is done in probe and
     *                           stop state.
     * 2) set TE/RE
     * 3) set xSMB
     * 4) set xSMA:  xSMA is the last one in this flow, which
     *               will trigger esai to start.
     */
    regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_xCR(tx),
        if tx { ESAI_xCR_TE_MASK } else { ESAI_xCR_RE_MASK },
        if tx { ESAI_xCR_TE(pins) } else { ESAI_xCR_RE(pins) },
    );
    mask = if tx { (*esai_priv).tx_mask } else { (*esai_priv).rx_mask };

    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xSMB(tx), ESAI_xSMB_xS_MASK, ESAI_xSMB_xS(mask));
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xSMA(tx), ESAI_xSMA_xS_MASK, ESAI_xSMA_xS(mask));

    /* Enable Exception interrupt */
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xCR(tx), ESAI_xCR_xEIE_MASK, ESAI_xCR_xEIE);
}

unsafe extern "C" fn fsl_esai_trigger_stop(esai_priv: *mut fsl_esai, tx: bool) {
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xCR(tx), ESAI_xCR_xEIE_MASK, 0);

    regmap_update_bits(
        (*esai_priv).regmap,
        REG_ESAI_xCR(tx),
        if tx { ESAI_xCR_TE_MASK } else { ESAI_xCR_RE_MASK },
        0,
    );
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xSMA(tx), ESAI_xSMA_xS_MASK, 0);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xSMB(tx), ESAI_xSMB_xS_MASK, 0);

    /* Disable and reset FIFO */
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xFCR(tx), ESAI_xFCR_xFR | ESAI_xFCR_xFEN, ESAI_xFCR_xFR);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_xFCR(tx), ESAI_xFCR_xFR, 0);
}

unsafe extern "C" fn fsl_esai_hw_reset(work: *mut work_struct) {
    let esai_priv = container_of_fsl_esai_work(work);
    let tx = true;
    let rx = false;
    let mut enabled = [false; 2];
    let mut tfcr: u32 = 0;
    let mut rfcr: u32 = 0;

    spin_lock_irqsave(&mut (*esai_priv).lock);
    /* Save the registers */
    regmap_read((*esai_priv).regmap, REG_ESAI_TFCR, &mut tfcr);
    regmap_read((*esai_priv).regmap, REG_ESAI_RFCR, &mut rfcr);
    enabled[bidx(tx)] = (tfcr & ESAI_xFCR_xFEN) != 0;
    enabled[bidx(rx)] = (rfcr & ESAI_xFCR_xFEN) != 0;

    /* Stop the tx & rx */
    fsl_esai_trigger_stop(esai_priv, tx);
    fsl_esai_trigger_stop(esai_priv, rx);

    /* Reset the esai, and ignore return value */
    fsl_esai_hw_init(esai_priv);

    /* Enforce ESAI personal resets for both TX and RX */
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCR, ESAI_xCR_xPR_MASK, ESAI_xCR_xPR);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_RCR, ESAI_xCR_xPR_MASK, ESAI_xCR_xPR);

    /* Restore registers by regcache_sync, and ignore return value */
    fsl_esai_register_restore(esai_priv);

    /* Remove ESAI personal resets by configuring PCRC and PRRC also */
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_TCR, ESAI_xCR_xPR_MASK, 0);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_RCR, ESAI_xCR_xPR_MASK, 0);
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_PRRC, ESAI_PRRC_PDC_MASK, ESAI_PRRC_PDC(ESAI_GPIO));
    regmap_update_bits((*esai_priv).regmap, REG_ESAI_PCRC, ESAI_PCRC_PC_MASK, ESAI_PCRC_PC(ESAI_GPIO));

    /* Restart tx / rx, if they already enabled */
    if enabled[bidx(tx)] {
        fsl_esai_trigger_start(esai_priv, tx);
    }
    if enabled[bidx(rx)] {
        fsl_esai_trigger_start(esai_priv, rx);
    }
    spin_unlock_irqrestore(&mut (*esai_priv).lock);
}

unsafe extern "C" fn fsl_esai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;

    (*esai_priv).channels[bidx(tx)] = (*(*substream).runtime).channels;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            spin_lock_irqsave(&mut (*esai_priv).lock);
            fsl_esai_trigger_start(esai_priv, tx);
            spin_unlock_irqrestore(&mut (*esai_priv).lock);
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            spin_lock_irqsave(&mut (*esai_priv).lock);
            fsl_esai_trigger_stop(esai_priv, tx);
            spin_unlock_irqrestore(&mut (*esai_priv).lock);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn fsl_esai_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let esai_priv = snd_soc_dai_get_drvdata(dai) as *mut fsl_esai;

    snd_soc_dai_init_dma_data(dai, &mut (*esai_priv).dma_params_tx, &mut (*esai_priv).dma_params_rx);

    0
}

static fsl_esai_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_esai_dai_probe),
    startup: Some(fsl_esai_startup),
    trigger: Some(fsl_esai_trigger),
    hw_params: Some(fsl_esai_hw_params),
    set_sysclk: Some(fsl_esai_set_dai_sysclk),
    set_fmt: Some(fsl_esai_set_dai_fmt),
    set_tdm_slot: Some(fsl_esai_set_dai_tdm_slot),
};

static mut fsl_esai_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: c"CPU-Playback".as_ptr(),
        channels_min: 1,
        channels_max: 12,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: FSL_ESAI_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"CPU-Capture".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: FSL_ESAI_FORMATS,
    },
    ops: &fsl_esai_dai_ops,
    symmetric_rate: 0,
    symmetric_channels: 0,
    symmetric_sample_bits: 0,
};

static fsl_esai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"fsl-esai".as_ptr(),
    legacy_dai_naming: 1,
};

static fsl_esai_reg_defaults: [reg_default; 23] = [
    reg_default { reg: REG_ESAI_ETDR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_ECR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TFCR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_RFCR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TX0, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TX1, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TX2, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TX3, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TX4, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TX5, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TSR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_SAICR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TCR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TCCR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_RCR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_RCCR, def: 0x00000000 },
    reg_default { reg: REG_ESAI_TSMA, def: 0x0000ffff },
    reg_default { reg: REG_ESAI_TSMB, def: 0x0000ffff },
    reg_default { reg: REG_ESAI_RSMA, def: 0x0000ffff },
    reg_default { reg: REG_ESAI_RSMB, def: 0x0000ffff },
    reg_default { reg: REG_ESAI_PRRC, def: 0x00000000 },
    reg_default { reg: REG_ESAI_PCRC, def: 0x00000000 },
    reg_default { reg: 0, def: 0 },
];

unsafe extern "C" fn fsl_esai_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_ESAI_ERDR | REG_ESAI_ECR | REG_ESAI_ESR | REG_ESAI_TFCR | REG_ESAI_TFSR | REG_ESAI_RFCR
        | REG_ESAI_RFSR | REG_ESAI_RX0 | REG_ESAI_RX1 | REG_ESAI_RX2 | REG_ESAI_RX3 | REG_ESAI_SAISR
        | REG_ESAI_SAICR | REG_ESAI_TCR | REG_ESAI_TCCR | REG_ESAI_RCR | REG_ESAI_RCCR | REG_ESAI_TSMA
        | REG_ESAI_TSMB | REG_ESAI_RSMA | REG_ESAI_RSMB | REG_ESAI_PRRC | REG_ESAI_PCRC => true,
        _ => false,
    }
}

unsafe extern "C" fn fsl_esai_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_ESAI_ERDR | REG_ESAI_ESR | REG_ESAI_TFSR | REG_ESAI_RFSR | REG_ESAI_RX0 | REG_ESAI_RX1
        | REG_ESAI_RX2 | REG_ESAI_RX3 | REG_ESAI_SAISR => true,
        _ => false,
    }
}

unsafe extern "C" fn fsl_esai_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_ESAI_ETDR | REG_ESAI_ECR | REG_ESAI_TFCR | REG_ESAI_RFCR | REG_ESAI_TX0 | REG_ESAI_TX1
        | REG_ESAI_TX2 | REG_ESAI_TX3 | REG_ESAI_TX4 | REG_ESAI_TX5 | REG_ESAI_TSR | REG_ESAI_SAICR
        | REG_ESAI_TCR | REG_ESAI_TCCR | REG_ESAI_RCR | REG_ESAI_RCCR | REG_ESAI_TSMA | REG_ESAI_TSMB
        | REG_ESAI_RSMA | REG_ESAI_RSMB | REG_ESAI_PRRC | REG_ESAI_PCRC => true,
        _ => false,
    }
}

static fsl_esai_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: REG_ESAI_PCRC,
    reg_defaults: fsl_esai_reg_defaults.as_ptr(),
    num_reg_defaults: fsl_esai_reg_defaults.len() as c_uint,
    readable_reg: Some(fsl_esai_readable_reg),
    volatile_reg: Some(fsl_esai_volatile_reg),
    writeable_reg: Some(fsl_esai_writeable_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn fsl_esai_runtime_resume(dev: *mut device) -> c_int {
    let esai = dev_get_drvdata(dev) as *mut fsl_esai;
    let mut ret: c_int;

    /*
     * Some platforms might use the same bit to gate all three or two of
     * clocks, so keep all clocks open/close at the same time for safety
     */
    ret = clk_prepare_enable((*esai).coreclk);
    if ret != 0 {
        return ret;
    }
    if !IS_ERR((*esai).spbaclk as *const c_void) {
        ret = clk_prepare_enable((*esai).spbaclk);
        if ret != 0 {
            goto_err_spbaclk(esai);
            return ret;
        }
    }
    if !IS_ERR((*esai).extalclk as *const c_void) {
        ret = clk_prepare_enable((*esai).extalclk);
        if ret != 0 {
            goto_err_extalclk(esai);
            return ret;
        }
    }
    if !IS_ERR((*esai).fsysclk as *const c_void) {
        ret = clk_prepare_enable((*esai).fsysclk);
        if ret != 0 {
            goto_err_fsysclk(esai);
            return ret;
        }
    }

    regcache_cache_only((*esai).regmap, false);

    ret = fsl_esai_register_restore(esai);
    if ret != 0 {
        goto_err_regcache_sync(esai);
        return ret;
    }

    0
}

unsafe fn goto_err_regcache_sync(esai: *mut fsl_esai) {
    if !IS_ERR((*esai).fsysclk as *const c_void) {
        clk_disable_unprepare((*esai).fsysclk);
    }
    goto_err_fsysclk(esai);
}

unsafe fn goto_err_fsysclk(esai: *mut fsl_esai) {
    if !IS_ERR((*esai).extalclk as *const c_void) {
        clk_disable_unprepare((*esai).extalclk);
    }
    goto_err_extalclk(esai);
}

unsafe fn goto_err_extalclk(esai: *mut fsl_esai) {
    if !IS_ERR((*esai).spbaclk as *const c_void) {
        clk_disable_unprepare((*esai).spbaclk);
    }
    goto_err_spbaclk(esai);
}

unsafe fn goto_err_spbaclk(esai: *mut fsl_esai) {
    clk_disable_unprepare((*esai).coreclk);
}

unsafe extern "C" fn fsl_esai_runtime_suspend(dev: *mut device) -> c_int {
    let esai = dev_get_drvdata(dev) as *mut fsl_esai;

    regcache_cache_only((*esai).regmap, true);

    if !IS_ERR((*esai).fsysclk as *const c_void) {
        clk_disable_unprepare((*esai).fsysclk);
    }
    if !IS_ERR((*esai).extalclk as *const c_void) {
        clk_disable_unprepare((*esai).extalclk);
    }
    if !IS_ERR((*esai).spbaclk as *const c_void) {
        clk_disable_unprepare((*esai).spbaclk);
    }
    clk_disable_unprepare((*esai).coreclk);

    0
}

static fsl_esai_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

unsafe extern "C" fn fsl_esai_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut esai_priv: *mut fsl_esai;
    let mut res: *mut resource = core::ptr::null_mut();
    let mut iprop: *const __be32;
    let regs: *mut c_void;
    let irq: c_int;
    let mut ret: c_int;

    esai_priv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<fsl_esai>(), GFP_KERNEL) as *mut fsl_esai;
    if esai_priv.is_null() {
        return -ENOMEM;
    }

    (*esai_priv).pdev = pdev;
    snprintf((*esai_priv).name.as_mut_ptr(), (*esai_priv).name.len(), c"%pOFn".as_ptr(), np);

    (*esai_priv).soc = of_device_get_match_data(&mut (*pdev).dev) as *const fsl_esai_soc_data;

    /* Get the addresses and IRQ */
    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void) as c_int;
    }

    (*esai_priv).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &fsl_esai_regmap_config);
    if IS_ERR((*esai_priv).regmap as *const c_void) {
        dev_err(&mut (*pdev).dev, c"failed to init regmap: %ld\n".as_ptr(), PTR_ERR((*esai_priv).regmap as *const c_void));
        return PTR_ERR((*esai_priv).regmap as *const c_void) as c_int;
    }

    (*esai_priv).coreclk = devm_clk_get(&mut (*pdev).dev, c"core".as_ptr());
    if IS_ERR((*esai_priv).coreclk as *const c_void) {
        dev_err(&mut (*pdev).dev, c"failed to get core clock: %ld\n".as_ptr(), PTR_ERR((*esai_priv).coreclk as *const c_void));
        return PTR_ERR((*esai_priv).coreclk as *const c_void) as c_int;
    }

    (*esai_priv).extalclk = devm_clk_get(&mut (*pdev).dev, c"extal".as_ptr());
    if IS_ERR((*esai_priv).extalclk as *const c_void) {
        dev_warn(&mut (*pdev).dev, c"failed to get extal clock: %ld\n".as_ptr(), PTR_ERR((*esai_priv).extalclk as *const c_void));
    }

    (*esai_priv).fsysclk = devm_clk_get(&mut (*pdev).dev, c"fsys".as_ptr());
    if IS_ERR((*esai_priv).fsysclk as *const c_void) {
        dev_warn(&mut (*pdev).dev, c"failed to get fsys clock: %ld\n".as_ptr(), PTR_ERR((*esai_priv).fsysclk as *const c_void));
    }

    (*esai_priv).spbaclk = devm_clk_get(&mut (*pdev).dev, c"spba".as_ptr());
    if IS_ERR((*esai_priv).spbaclk as *const c_void) {
        dev_warn(&mut (*pdev).dev, c"failed to get spba clock: %ld\n".as_ptr(), PTR_ERR((*esai_priv).spbaclk as *const c_void));
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    ret = devm_request_irq(&mut (*pdev).dev, irq, Some(esai_isr), IRQF_SHARED, (*esai_priv).name.as_ptr(), esai_priv as *mut c_void);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"failed to claim irq %u\n".as_ptr(), irq);
        return ret;
    }

    /* Set a default slot number */
    (*esai_priv).slots = 2;

    /* Set a default clock provider state */
    (*esai_priv).consumer_mode = true;

    /* Determine the FIFO depth */
    iprop = of_get_property(np, c"fsl,fifo-depth".as_ptr(), core::ptr::null_mut()) as *const __be32;
    if !iprop.is_null() {
        (*esai_priv).fifo_depth = be32_to_cpup(iprop);
    } else {
        (*esai_priv).fifo_depth = 64;
    }

    (*esai_priv).dma_params_tx.maxburst = 16;
    (*esai_priv).dma_params_rx.maxburst = 16;
    (*esai_priv).dma_params_tx.addr = (*res).start + REG_ESAI_ETDR as resource_size_t;
    (*esai_priv).dma_params_rx.addr = (*res).start + REG_ESAI_ERDR as resource_size_t;

    (*esai_priv).synchronous = of_property_read_bool(np, c"fsl,esai-synchronous".as_ptr());

    /* Implement full symmetry for synchronous mode */
    if (*esai_priv).synchronous {
        fsl_esai_dai.symmetric_rate = 1;
        fsl_esai_dai.symmetric_channels = 1;
        fsl_esai_dai.symmetric_sample_bits = 1;
    }

    dev_set_drvdata(&mut (*pdev).dev, esai_priv as *mut c_void);
    spin_lock_init(&mut (*esai_priv).lock);
    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = fsl_esai_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            pm_runtime_disable(&mut (*pdev).dev);
            return ret;
        }
    }

    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret < 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_esai_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = fsl_esai_hw_init(esai_priv);
    if ret != 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_esai_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    (*esai_priv).tx_mask = 0xFFFFFFFF;
    (*esai_priv).rx_mask = 0xFFFFFFFF;

    /* Clear the TSMA, TSMB, RSMA, RSMB */
    regmap_write((*esai_priv).regmap, REG_ESAI_TSMA, 0);
    regmap_write((*esai_priv).regmap, REG_ESAI_TSMB, 0);
    regmap_write((*esai_priv).regmap, REG_ESAI_RSMA, 0);
    regmap_write((*esai_priv).regmap, REG_ESAI_RSMB, 0);

    ret = pm_runtime_put_sync(&mut (*pdev).dev);
    if ret < 0 && ret != -ENOSYS {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_esai_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    /*
     * Register platform component before registering cpu dai for there
     * is not defer probe for platform component in snd_soc_add_pcm_runtime().
     */
    ret = imx_pcm_dma_init(pdev);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"failed to init imx pcm dma: %d\n".as_ptr(), ret);
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_esai_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &fsl_esai_component, &mut fsl_esai_dai, 1);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"failed to register DAI: %d\n".as_ptr(), ret);
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_esai_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    INIT_WORK(&mut (*esai_priv).work, Some(fsl_esai_hw_reset));

    ret
}

unsafe extern "C" fn fsl_esai_remove(pdev: *mut platform_device) {
    let esai_priv = platform_get_drvdata(pdev) as *mut fsl_esai;

    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        fsl_esai_runtime_suspend(&mut (*pdev).dev);
    }

    cancel_work_sync(&mut (*esai_priv).work);
}

static fsl_esai_dt_ids: [of_device_id; 4] = [
    of_device_id { compatible: c"fsl,imx35-esai".as_ptr(), data: unsafe { &fsl_esai_imx35 as *const _ as *const c_void } },
    of_device_id { compatible: c"fsl,vf610-esai".as_ptr(), data: unsafe { &fsl_esai_vf610 as *const _ as *const c_void } },
    of_device_id { compatible: c"fsl,imx6ull-esai".as_ptr(), data: unsafe { &fsl_esai_imx6ull as *const _ as *const c_void } },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(of, fsl_esai_dt_ids);

static fsl_esai_driver: platform_driver = platform_driver {
    probe: Some(fsl_esai_probe),
    remove: Some(fsl_esai_remove),
    driver: platform_driver_driver {
        name: c"fsl-esai-dai".as_ptr(),
        pm: &fsl_esai_pm_ops,
        of_match_table: fsl_esai_dt_ids.as_ptr(),
    },
};

// module_platform_driver(fsl_esai_driver);
// MODULE_AUTHOR("Freescale Semiconductor, Inc.");
// MODULE_DESCRIPTION("Freescale ESAI CPU DAI driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:fsl-esai-dai");

unsafe extern "C" {
    static SNDRV_PCM_FMTBIT_S8: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u32;
    static SNDRV_PCM_FMTBIT_S20_3LE: u32;
    static SNDRV_PCM_FMTBIT_S24_LE: u32;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static IRQ_HANDLED: irqreturn_t;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENOSYS: c_int;
    static GFP_KERNEL: c_uint;
    static IRQF_SHARED: c_uint;
    static REGCACHE_FLAT: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BC_FC: c_uint;
    static SND_SOC_DAIFMT_BP_FC: c_uint;
    static SND_SOC_DAIFMT_BC_FP: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;

    static REG_ESAI_ETDR: c_uint;
    static REG_ESAI_ERDR: c_uint;
    static REG_ESAI_ECR: c_uint;
    static REG_ESAI_ESR: c_uint;
    static REG_ESAI_TFCR: c_uint;
    static REG_ESAI_TFSR: c_uint;
    static REG_ESAI_RFCR: c_uint;
    static REG_ESAI_RFSR: c_uint;
    static REG_ESAI_TX0: c_uint;
    static REG_ESAI_TX1: c_uint;
    static REG_ESAI_TX2: c_uint;
    static REG_ESAI_TX3: c_uint;
    static REG_ESAI_TX4: c_uint;
    static REG_ESAI_TX5: c_uint;
    static REG_ESAI_RX0: c_uint;
    static REG_ESAI_RX1: c_uint;
    static REG_ESAI_RX2: c_uint;
    static REG_ESAI_RX3: c_uint;
    static REG_ESAI_TSR: c_uint;
    static REG_ESAI_SAICR: c_uint;
    static REG_ESAI_SAISR: c_uint;
    static REG_ESAI_TCR: c_uint;
    static REG_ESAI_TCCR: c_uint;
    static REG_ESAI_RCR: c_uint;
    static REG_ESAI_RCCR: c_uint;
    static REG_ESAI_TSMA: c_uint;
    static REG_ESAI_TSMB: c_uint;
    static REG_ESAI_RSMA: c_uint;
    static REG_ESAI_RSMB: c_uint;
    static REG_ESAI_PRRC: c_uint;
    static REG_ESAI_PCRC: c_uint;

    static ESAI_SAISR_TUE: u32;
    static ESAI_SAISR_ROE: u32;
    static ESAI_ESR_TINIT_MASK: u32;
    static ESAI_ESR_RFF_MASK: u32;
    static ESAI_ESR_TFE_MASK: u32;
    static ESAI_ESR_TLS_MASK: u32;
    static ESAI_ESR_TDE_MASK: u32;
    static ESAI_ESR_TED_MASK: u32;
    static ESAI_ESR_TD_MASK: u32;
    static ESAI_ESR_RLS_MASK: u32;
    static ESAI_ESR_RDE_MASK: u32;
    static ESAI_ESR_RED_MASK: u32;
    static ESAI_ESR_RD_MASK: u32;
    static ESAI_xCR_xEIE_MASK: u32;
    static ESAI_xCR_xEIE: u32;
    static ESAI_xCCR_xPSR_BYPASS: u32;
    static ESAI_xCCR_xPSR_DIV8: u32;
    static ESAI_xCCR_xPSR_MASK: u32;
    static ESAI_xCCR_xPM_MASK: u32;
    static ESAI_xCCR_xFP_MASK: u32;
    static ESAI_xCCR_xHCKD: u32;
    static ESAI_ECR_ETI: u32;
    static ESAI_ECR_ERI: u32;
    static ESAI_ECR_ETO: u32;
    static ESAI_ECR_ERO: u32;
    static ESAI_ECR_ESAIEN_MASK: u32;
    static ESAI_ECR_ERST_MASK: u32;
    static ESAI_ECR_ESAIEN: u32;
    static ESAI_ECR_ERST: u32;
    static ESAI_HCKT_FSYS: c_int;
    static ESAI_HCKR_FSYS: c_int;
    static ESAI_HCKT_EXTAL: c_int;
    static ESAI_HCKR_EXTAL: c_int;
    static ESAI_xCCR_xDC_MASK: u32;
    static ESAI_xCR_xFSR: u32;
    static ESAI_xCCR_xFSP: u32;
    static ESAI_xCCR_xCKP: u32;
    static ESAI_xCCR_xHCKP: u32;
    static ESAI_xCR_xWA: u32;
    static ESAI_xCR_xFSL: u32;
    static ESAI_xCCR_xCKD: u32;
    static ESAI_xCCR_xFSD: u32;
    static ESAI_SAICR_SYNC: u32;
    static ESAI_xCR_xSWS_MASK: u32;
    static ESAI_xCR_xMOD_MASK: u32;
    static ESAI_xCR_xMOD_NETWORK: u32;
    static ESAI_xFCR_xFR_MASK: u32;
    static ESAI_xFCR_xFR: u32;
    static ESAI_xFCR_xWA_MASK: u32;
    static ESAI_xFCR_xFWM_MASK: u32;
    static ESAI_xFCR_TE_MASK: u32;
    static ESAI_xFCR_TIEN: u32;
    static ESAI_xFCR_RE_MASK: u32;
    static ESAI_xCR_PADC: u32;
    static ESAI_PRRC_PDC_MASK: u32;
    static ESAI_PCRC_PC_MASK: u32;
    static ESAI_GPIO: u32;
    static ESAI_xFCR_xFEN_MASK: u32;
    static ESAI_xFCR_xFEN: u32;
    static ESAI_xCR_TE_MASK: u32;
    static ESAI_xCR_RE_MASK: u32;
    static ESAI_xSMB_xS_MASK: u32;
    static ESAI_xSMA_xS_MASK: u32;
    static ESAI_xCR_xPR_MASK: u32;
    static ESAI_xCR_xPR: u32;

    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: u32, val: u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: u32) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn params_width(params: *mut snd_pcm_hw_params) -> u32;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_get_property(np: *mut device_node, name: *const c_char, lenp: *mut c_int) -> *const c_void;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn be32_to_cpup(p: *const __be32) -> u32;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn imx_pcm_dma_init(pdev: *mut platform_device) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn INIT_WORK(work: *mut work_struct, func: Option<unsafe extern "C" fn(*mut work_struct)>);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn REG_ESAI_xCCR(tx: bool) -> c_uint;
    fn REG_ESAI_xCR(tx: bool) -> c_uint;
    fn REG_ESAI_xFCR(tx: bool) -> c_uint;
    fn REG_ESAI_xSMB(tx: bool) -> c_uint;
    fn REG_ESAI_xSMA(tx: bool) -> c_uint;
    fn ESAI_xCCR_xPM(pm: u32) -> u32;
    fn ESAI_xCCR_xFP(fp: u32) -> u32;
    fn ESAI_xCCR_xDC(dc: u32) -> u32;
    fn ESAI_xCR_xSWS(slot_width: u32, width: u32) -> u32;
    fn ESAI_xFCR_xWA(width: u32) -> u32;
    fn ESAI_xFCR_xFWM(depth: u32) -> u32;
    fn ESAI_xFCR_TE(pins: u32) -> u32;
    fn ESAI_xFCR_RE(pins: u32) -> u32;
    fn ESAI_PRRC_PDC(gpio: u32) -> u32;
    fn ESAI_PCRC_PC(gpio: u32) -> u32;
    fn ESAI_xSMB_xS(mask: u32) -> u32;
    fn ESAI_xSMA_xS(mask: u32) -> u32;
    fn ESAI_xCR_TE(pins: u32) -> u32;
    fn ESAI_xCR_RE(pins: u32) -> u32;
    fn DIV_ROUND_UP(n: u32, d: u32) -> u32;
    fn container_of_fsl_esai_work(work: *mut work_struct) -> *mut fsl_esai;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
