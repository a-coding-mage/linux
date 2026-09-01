// SPDX-License-Identifier: GPL-2.0-only
//
// Freescale MPC5200 PSC in I2S mode
// ALSA SoC Digital Audio Interface (DAI) driver
//
// Copyright (C) 2008 Secret Lab Technologies Ltd.
// Copyright (C) 2009 Jon Smirl, Digispeaker

// C includes translated as external dependencies:
// linux/module.h, linux/of.h
// sound/pcm.h, sound/pcm_params.h, sound/soc.h
// asm/mpc52xx_psc.h
// "mpc5200_dma.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

/**
 * PSC_I2S_RATES: sample rates supported by the I2S
 *
 * This driver currently only supports the PSC running in I2S slave mode,
 * which means the codec determines the sample rate.  Therefore, we tell
 * ALSA that we support all rates and let the codec driver decide what rates
 * are really supported.
 */
pub const PSC_I2S_RATES: c_uint = SNDRV_PCM_RATE_CONTINUOUS;

/**
 * PSC_I2S_FORMATS: audio formats supported by the PSC I2S mode
 */
pub const PSC_I2S_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_BE
    | SNDRV_PCM_FMTBIT_S24_BE
    | SNDRV_PCM_FMTBIT_S32_BE;

type u8_t = u8;
type u32_t = u32;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
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
pub union ipcr_acr_union {
    pub ipcr: u8_t,
}

#[repr(C)]
pub struct mpc52xx_psc {
    pub _prefix: [u8; 0],
    pub sicr: u32_t,
    pub ipcr_acr: ipcr_acr_union,
    pub command: u8_t,
}

#[repr(C)]
pub struct psc_dma {
    pub dev: *mut device,
    pub psc_regs: *mut mpc52xx_psc,
    pub sicr: u32_t,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk: Option<
        unsafe extern "C" fn(
            cpu_dai: *mut snd_soc_dai,
            clk_id: c_int,
            freq: c_uint,
            dir: c_int,
        ) -> c_int,
    >,
    pub set_fmt:
        Option<unsafe extern "C" fn(cpu_dai: *mut snd_soc_dai, format: c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
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
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(op: *mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(op: *mut platform_device)>,
    pub driver: driver_private,
}

unsafe extern "C" {
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_FMTBIT_S8: c_uint;
    static SNDRV_PCM_FMTBIT_S16_BE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_BE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_BE: c_uint;

    static SNDRV_PCM_FORMAT_S8: c_int;
    static SNDRV_PCM_FORMAT_S16_BE: c_int;
    static SNDRV_PCM_FORMAT_S24_BE: c_int;
    static SNDRV_PCM_FORMAT_S32_BE: c_int;

    static MPC52xx_PSC_SICR_SIM_CODEC_8: u32_t;
    static MPC52xx_PSC_SICR_SIM_CODEC_16: u32_t;
    static MPC52xx_PSC_SICR_SIM_CODEC_24: u32_t;
    static MPC52xx_PSC_SICR_SIM_CODEC_32: u32_t;
    static MPC52xx_PSC_SICR_DTS1: u32_t;
    static MPC52xx_PSC_SICR_I2S: u32_t;
    static MPC52xx_PSC_SICR_CLKPOL: u32_t;
    static MPC52xx_PSC_TX_ENABLE: u8_t;
    static MPC52xx_PSC_RX_ENABLE: u8_t;

    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static EINVAL: c_int;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;

    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_periods(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;

    fn out_be32(addr: *mut u32_t, value: u32_t);
    fn in_8(addr: *mut u8_t) -> u8_t;
    fn out_8(addr: *mut u8_t, value: u8_t);

    fn mpc5200_audio_dma_create(op: *mut platform_device) -> c_int;
    fn mpc5200_audio_dma_destroy(op: *mut platform_device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: usize,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn module_platform_driver(driver: *mut platform_driver);
}

unsafe extern "C" fn psc_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let psc_dma: *mut psc_dma = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut psc_dma;
    let mode: u32_t;

    dev_dbg(
        (*psc_dma).dev,
        c"%s(substream=%p) p_size=%i p_bytes=%i periods=%i buffer_size=%i  buffer_bytes=%i\n"
            .as_ptr(),
        c"psc_i2s_hw_params".as_ptr(),
        substream,
        params_period_size(params),
        params_period_bytes(params),
        params_periods(params),
        params_buffer_size(params),
        params_buffer_bytes(params),
    );

    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S8 => {
            mode = MPC52xx_PSC_SICR_SIM_CODEC_8;
        }
        x if x == SNDRV_PCM_FORMAT_S16_BE => {
            mode = MPC52xx_PSC_SICR_SIM_CODEC_16;
        }
        x if x == SNDRV_PCM_FORMAT_S24_BE => {
            mode = MPC52xx_PSC_SICR_SIM_CODEC_24;
        }
        x if x == SNDRV_PCM_FORMAT_S32_BE => {
            mode = MPC52xx_PSC_SICR_SIM_CODEC_32;
        }
        _ => {
            dev_dbg((*psc_dma).dev, c"invalid format\n".as_ptr());
            return -EINVAL;
        }
    }
    out_be32(
        core::ptr::addr_of_mut!((*(*psc_dma).psc_regs).sicr),
        (*psc_dma).sicr | mode,
    );

    0
}

/**
 * psc_i2s_set_sysclk: set the clock frequency and direction
 *
 * This function is called by the machine driver to tell us what the clock
 * frequency and direction are.
 *
 * Currently, we only support operating as a clock slave (SND_SOC_CLOCK_IN),
 * and we don't care about the frequency.  Return an error if the direction
 * is not SND_SOC_CLOCK_IN.
 *
 * @cpu_dai: DAI runtime data pointer
 * @clk_id: reserved, should be zero
 * @freq: the frequency of the given clock ID, currently ignored
 * @dir: SND_SOC_CLOCK_IN (clock slave) or SND_SOC_CLOCK_OUT (clock master)
 *
 * Returns: %0 on success or %-EINVAL on failure.
 */
unsafe extern "C" fn psc_i2s_set_sysclk(
    cpu_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let psc_dma: *mut psc_dma = snd_soc_dai_get_drvdata(cpu_dai) as *mut psc_dma;
    dev_dbg(
        (*psc_dma).dev,
        c"psc_i2s_set_sysclk(cpu_dai=%p, dir=%i)\n".as_ptr(),
        cpu_dai,
        dir,
    );
    if dir == SND_SOC_CLOCK_IN {
        0
    } else {
        -EINVAL
    }
}

/**
 * psc_i2s_set_fmt: set the serial format.
 *
 * This function is called by the machine driver to tell us what serial
 * format to use.
 *
 * This driver only supports I2S mode.  Return an error if the format is
 * not SND_SOC_DAIFMT_I2S.
 *
 * @cpu_dai: DAI runtime data pointer
 * @format: one of SND_SOC_DAIFMT_xxx
 *
 * Returns: %0 on success or %-EINVAL on failure.
 */
unsafe extern "C" fn psc_i2s_set_fmt(cpu_dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let psc_dma: *mut psc_dma = snd_soc_dai_get_drvdata(cpu_dai) as *mut psc_dma;
    dev_dbg(
        (*psc_dma).dev,
        c"psc_i2s_set_fmt(cpu_dai=%p, format=%i)\n".as_ptr(),
        cpu_dai,
        format,
    );
    if format == SND_SOC_DAIFMT_I2S {
        0
    } else {
        -EINVAL
    }
}

/* ---------------------------------------------------------------------
 * ALSA SoC Bindings
 *
 * - Digital Audio Interface (DAI) template
 * - create/destroy dai hooks
 */

/**
 * var psc_i2s_dai_ops - template CPU Digital Audio Interface
 */
static psc_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(psc_i2s_hw_params),
    set_sysclk: Some(psc_i2s_set_sysclk),
    set_fmt: Some(psc_i2s_set_fmt),
};

static mut psc_i2s_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"mpc5200-psc-i2s.0".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"I2S Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: PSC_I2S_RATES,
        formats: PSC_I2S_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"I2S Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: PSC_I2S_RATES,
        formats: PSC_I2S_FORMATS,
    },
    ops: &psc_i2s_dai_ops,
}];

static psc_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"mpc5200-i2s".as_ptr(),
    legacy_dai_naming: 1,
};

/* ---------------------------------------------------------------------
 * OF platform bus binding code:
 * - Probe/remove operations
 * - OF device match table
 */
unsafe extern "C" fn psc_i2s_of_probe(op: *mut platform_device) -> c_int {
    let mut rc: c_int;
    let psc_dma: *mut psc_dma;
    let regs: *mut mpc52xx_psc;

    rc = mpc5200_audio_dma_create(op);
    if rc != 0 {
        return rc;
    }

    rc = devm_snd_soc_register_component(
        core::ptr::addr_of_mut!((*op).dev),
        &psc_i2s_component,
        psc_i2s_dai.as_mut_ptr(),
        psc_i2s_dai.len(),
    );
    if rc != 0 {
        pr_err(c"Failed to register DAI\n".as_ptr());
        mpc5200_audio_dma_destroy(op);
        return rc;
    }

    psc_dma = dev_get_drvdata(core::ptr::addr_of_mut!((*op).dev)) as *mut psc_dma;
    regs = (*psc_dma).psc_regs;

    /* Configure the serial interface mode; defaulting to CODEC8 mode */
    (*psc_dma).sicr = MPC52xx_PSC_SICR_DTS1 | MPC52xx_PSC_SICR_I2S | MPC52xx_PSC_SICR_CLKPOL;
    out_be32(
        core::ptr::addr_of_mut!((*(*psc_dma).psc_regs).sicr),
        (*psc_dma).sicr | MPC52xx_PSC_SICR_SIM_CODEC_8,
    );

    /* Check for the codec handle.  If it is not present then we
     * are done */
    if !of_property_present((*op).dev.of_node, c"codec-handle".as_ptr()) {
        return 0;
    }

    /* Due to errata in the dma mode; need to line up enabling
     * the transmitter with a transition on the frame sync
     * line */

    /* first make sure it is low */
    while (in_8(core::ptr::addr_of_mut!((*regs).ipcr_acr.ipcr)) & 0x80) != 0 {}
    /* then wait for the transition to high */
    while (in_8(core::ptr::addr_of_mut!((*regs).ipcr_acr.ipcr)) & 0x80) == 0 {}
    /* Finally, enable the PSC.
     * Receiver must always be enabled; even when we only want
     * transmit.  (see 15.3.2.3 of MPC5200B User's Guide) */

    /* Go */
    out_8(
        core::ptr::addr_of_mut!((*(*psc_dma).psc_regs).command),
        MPC52xx_PSC_TX_ENABLE | MPC52xx_PSC_RX_ENABLE,
    );

    0
}

unsafe extern "C" fn psc_i2s_of_remove(op: *mut platform_device) {
    mpc5200_audio_dma_destroy(op);
}

/* Match table for of_platform binding */
static psc_i2s_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c"fsl,mpc5200-psc-i2s".as_ptr(),
    },
    of_device_id {
        compatible: c"fsl,mpc5200b-psc-i2s".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, psc_i2s_match);

static mut psc_i2s_driver: platform_driver = platform_driver {
    probe: Some(psc_i2s_of_probe),
    remove: Some(psc_i2s_of_remove),
    driver: driver_private {
        name: c"mpc5200-psc-i2s".as_ptr(),
        of_match_table: psc_i2s_match.as_ptr(),
    },
};

unsafe fn register_psc_i2s_driver() {
    module_platform_driver(core::ptr::addr_of_mut!(psc_i2s_driver));
}

// MODULE_AUTHOR("Grant Likely <grant.likely@secretlab.ca>");
// MODULE_DESCRIPTION("Freescale MPC5200 PSC in I2S mode ASoC Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
