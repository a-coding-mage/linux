// SPDX-License-Identifier: GPL-2.0-only
// ALSA SoC I2S (McBSP) Audio Layer for TI DAVINCI processor
//
// Author:      Vladimir Barinov, <vbarinov@embeddedalley.com>
// Copyright:   (C) 2007 MontaVista Software, Inc., <source@mvista.com>
//
// DT support	(c) 2016 Petr Kulhavy, Barix AG <petr@barix.com>
//		based on davinci-mcasp.c DT support
//
// TODO:
// on DA850 implement HW FIFOs instead of DMA into DXR and DRR registers

// External dependencies from linux kernel
extern "C" {
    // linux/init.h
    // linux/module.h
    // linux/device.h
    // linux/slab.h - memory allocation
    // linux/delay.h - udelay
    fn udelay(usecs: u32);

    // linux/io.h - I/O access
    fn __raw_writel(val: u32, addr: *mut u32);
    fn __raw_readl(addr: *const u32) -> u32;

    // linux/clk.h - clock management
    type clk;
    fn clk_get_rate(clk: *const clk) -> u64;
    fn clk_prepare_enable(clk: *const clk) -> i32;
    fn clk_disable_unprepare(clk: *const clk);

    // sound/core.h, sound/pcm.h, sound/soc.h - ALSA/ASoC
    type snd_soc_dai;
    type snd_pcm_substream;
    type snd_pcm_hw_params;
    type snd_soc_dai_driver;
    type snd_soc_component_driver;
    type snd_dmaengine_dai_dma_data;
    type device;
    type platform_device;
    type resource;

    fn snd_soc_dai_get_drvdata(dai: *const snd_soc_dai) -> *mut core::ffi::c_void;
    fn dev_dbg(dev: *const device, fmt: *const u8, ...);
    fn dev_err(dev: *const device, fmt: *const u8, ...);
    fn dev_warn(dev: *const device, fmt: *const u8, ...);
    fn dev_err_probe(dev: *const device, err: i32, fmt: *const u8, ...) -> i32;
    fn dev_set_drvdata(dev: *const device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *const device) -> *mut core::ffi::c_void;
    fn printk(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);

    fn platform_get_resource_byname(
        pdev: *const platform_device,
        ty: u32,
        name: *const u8,
    ) -> *mut resource;
    fn platform_get_resource(pdev: *const platform_device, ty: u32, num: u32) -> *mut resource;
    fn devm_ioremap_resource(dev: *const device, res: *mut resource) -> *mut u8;
    fn devm_kzalloc(dev: *const device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn of_property_read_bool(np: *const core::ffi::c_void, propname: *const u8) -> bool;

    fn devm_clk_get_optional(dev: *const device, id: *const u8) -> *const clk;
    fn devm_clk_get(dev: *const device, id: *const u8) -> *const clk;

    fn snd_soc_dai_dma_data_set(
        dai: *const snd_soc_dai,
        stream: i32,
        data: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_register_component(
        dev: *const device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const snd_soc_dai_driver,
        num_dais: i32,
    ) -> i32;
    fn snd_soc_unregister_component(dev: *const device);

    fn params_format(params: *const snd_pcm_hw_params) -> i32;
    fn params_channels(params: *const snd_pcm_hw_params) -> i32;
    fn hw_param_interval(
        params: *const snd_pcm_hw_params,
        var: i32,
    ) -> *mut core::ffi::c_void;
    fn snd_interval_value(i: *const core::ffi::c_void) -> u32;

    fn edma_pcm_platform_register(dev: *const device) -> i32;

    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
}

const DRV_NAME: &[u8] = b"davinci-i2s\0";

// NOTE:  terminology here is confusing.
//
//  - This driver supports the "Audio Serial Port" (ASP),
//    found on dm6446, dm355, and other DaVinci chips.
//
//  - But it labels it a "Multi-channel Buffered Serial Port"
//    (McBSP) as on older chips like the dm642 ... which was
//    backward-compatible, possibly explaining that confusion.
//
//  - OMAP chips have a controller called McBSP, which is
//    incompatible with the DaVinci flavor of McBSP.
//
//  - Newer DaVinci chips have a controller called McASP,
//    incompatible with ASP and with either McBSP.
//
// In short:  this uses ASP to implement I2S, not McBSP.
// And it won't be the only DaVinci implemention of I2S.

const DAVINCI_MCBSP_DRR_REG: i32 = 0x00;
const DAVINCI_MCBSP_DXR_REG: i32 = 0x04;
const DAVINCI_MCBSP_SPCR_REG: i32 = 0x08;
const DAVINCI_MCBSP_RCR_REG: i32 = 0x0c;
const DAVINCI_MCBSP_XCR_REG: i32 = 0x10;
const DAVINCI_MCBSP_SRGR_REG: i32 = 0x14;
const DAVINCI_MCBSP_PCR_REG: i32 = 0x24;

const DAVINCI_MCBSP_SPCR_RRST: u32 = 1 << 0;
const DAVINCI_MCBSP_SPCR_RJUST_Z_LE: u32 = 0 << 13;
const DAVINCI_MCBSP_SPCR_RJUST_S_LE: u32 = 1 << 13;
const DAVINCI_MCBSP_SPCR_XRST: u32 = 1 << 16;
const DAVINCI_MCBSP_SPCR_GRST: u32 = 1 << 22;
const DAVINCI_MCBSP_SPCR_FRST: u32 = 1 << 23;
const DAVINCI_MCBSP_SPCR_FREE: u32 = 1 << 25;

const DAVINCI_MCBSP_RCR_RFIG: u32 = 1 << 18;
const DAVINCI_MCBSP_RCR_RPHASE: u32 = 1 << 31;

const DAVINCI_MCBSP_XCR_XFIG: u32 = 1 << 18;
const DAVINCI_MCBSP_XCR_XPHASE: u32 = 1 << 31;

const DAVINCI_MCBSP_SRGR_FSGM: u32 = 1 << 28;
const DAVINCI_MCBSP_SRGR_CLKSM: u32 = 1 << 29;

const DAVINCI_MCBSP_PCR_CLKRP: u32 = 1 << 0;
const DAVINCI_MCBSP_PCR_CLKXP: u32 = 1 << 1;
const DAVINCI_MCBSP_PCR_FSRP: u32 = 1 << 2;
const DAVINCI_MCBSP_PCR_FSXP: u32 = 1 << 3;
const DAVINCI_MCBSP_PCR_SCLKME: u32 = 1 << 7;
const DAVINCI_MCBSP_PCR_CLKRM: u32 = 1 << 8;
const DAVINCI_MCBSP_PCR_CLKXM: u32 = 1 << 9;
const DAVINCI_MCBSP_PCR_FSRM: u32 = 1 << 10;
const DAVINCI_MCBSP_PCR_FSXM: u32 = 1 << 11;

const MOD_DSP_A: i32 = 0;
const MOD_DSP_B: i32 = 1;

#[repr(C)]
enum DavinciMcbspWord {
    Word8 = 0,
    Word12 = 1,
    Word16 = 2,
    Word20 = 3,
    Word24 = 4,
    Word32 = 5,
}

// Placeholder for external enum values
const SNDRV_PCM_FORMAT_S8: i32 = 0;
const SNDRV_PCM_FORMAT_S16_LE: i32 = 2;
const SNDRV_PCM_FORMAT_S24_LE: i32 = 5;
const SNDRV_PCM_FORMAT_S32_LE: i32 = 6;

const SNDRV_PCM_FORMAT_LAST: usize = (SNDRV_PCM_FORMAT_S32_LE + 1) as usize;

const SNDRV_PCM_STREAM_PLAYBACK: i32 = 0;
const SNDRV_PCM_STREAM_CAPTURE: i32 = 1;

const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_STOP: i32 = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32 = 2;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32 = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 4;
const SNDRV_PCM_TRIGGER_RESUME: i32 = 5;

const SND_SOC_DAIFMT_CONT: u32 = 1 << 0;
const SND_SOC_DAIFMT_GATED: u32 = 2 << 0;
const SND_SOC_DAIFMT_CLOCK_MASK: u32 = 0xf;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: u32 = 0xf << 12;
const SND_SOC_DAIFMT_BP_FP: u32 = 3 << 12;
const SND_SOC_DAIFMT_BC_FP: u32 = 2 << 12;
const SND_SOC_DAIFMT_BP_FC: u32 = 1 << 12;
const SND_SOC_DAIFMT_BC_FC: u32 = 0 << 12;

const SND_SOC_DAIFMT_FORMAT_MASK: u32 = 0xf << 4;
const SND_SOC_DAIFMT_I2S: u32 = 0 << 4;
const SND_SOC_DAIFMT_DSP_A: u32 = 1 << 4;
const SND_SOC_DAIFMT_DSP_B: u32 = 2 << 4;

const SND_SOC_DAIFMT_INV_MASK: u32 = 0xf << 8;
const SND_SOC_DAIFMT_NB_NF: u32 = 0 << 8;
const SND_SOC_DAIFMT_NB_IF: u32 = 1 << 8;
const SND_SOC_DAIFMT_IB_NF: u32 = 2 << 8;
const SND_SOC_DAIFMT_IB_IF: u32 = 3 << 8;

const DAVINCI_MCBSP_CLKGDV: i32 = 0;

const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: i32 = 10;
const SNDRV_PCM_HW_PARAM_FRAME_BITS: i32 = 11;

// External constants - build configuration
const IORESOURCE_MEM: u32 = 0;
const IORESOURCE_DMA: u32 = 3;
const GFP_KERNEL: u32 = 0x10u32;
const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;

static ASP_WORD_LENGTH: [u8; SNDRV_PCM_FORMAT_LAST] = {
    let mut arr = [0u8; SNDRV_PCM_FORMAT_LAST];
    arr[SNDRV_PCM_FORMAT_S8 as usize] = DavinciMcbspWord::Word8 as u8;
    arr[SNDRV_PCM_FORMAT_S16_LE as usize] = DavinciMcbspWord::Word16 as u8;
    arr[SNDRV_PCM_FORMAT_S24_LE as usize] = DavinciMcbspWord::Word24 as u8;
    arr[SNDRV_PCM_FORMAT_S32_LE as usize] = DavinciMcbspWord::Word32 as u8;
    arr
};

static DOUBLE_FMT: [i32; SNDRV_PCM_FORMAT_LAST] = {
    let mut arr = [0i32; SNDRV_PCM_FORMAT_LAST];
    arr[SNDRV_PCM_FORMAT_S8 as usize] = SNDRV_PCM_FORMAT_S16_LE;
    arr[SNDRV_PCM_FORMAT_S16_LE as usize] = SNDRV_PCM_FORMAT_S32_LE;
    arr
};

#[repr(C)]
struct DavinciMcbspDev {
    dev: *mut device,
    dma_data: [snd_dmaengine_dai_dma_data; 2],
    dma_request: [i32; 2],
    base: *mut u8,
    mode: i32,
    pcr: u32,
    clk: *const clk,
    ext_clk: *const clk,
    enable_channel_combine: u32,
    fmt: u32,
    clk_div: i32,
    i2s_accurate_sck: bool,
    tdm_slots: i32,
    slot_width: i32,
    tx_framing_bit: bool,
    rx_framing_bit: bool,
}

fn davinci_mcbsp_write_reg(dev: *mut DavinciMcbspDev, reg: i32, val: u32) {
    unsafe {
        __raw_writel(val, (*dev).base.add(reg as usize) as *mut u32);
    }
}

fn davinci_mcbsp_read_reg(dev: *mut DavinciMcbspDev, reg: i32) -> u32 {
    unsafe { __raw_readl((*dev).base.add(reg as usize) as *const u32) }
}

fn toggle_clock(dev: *mut DavinciMcbspDev, playback: i32) {
    let m = if playback != 0 {
        DAVINCI_MCBSP_PCR_CLKXP
    } else {
        DAVINCI_MCBSP_PCR_CLKRP
    };
    unsafe {
        davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_PCR_REG, (*dev).pcr ^ m);
        davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_PCR_REG, (*dev).pcr);
    }
}

fn davinci_mcbsp_start(dev: *mut DavinciMcbspDev, substream: *const snd_pcm_substream) {
    unsafe {
        let playback = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            1
        } else {
            0
        };
        let mask = if playback != 0 {
            DAVINCI_MCBSP_SPCR_XRST
        } else {
            DAVINCI_MCBSP_SPCR_RRST
        };

        let mut spcr = davinci_mcbsp_read_reg(dev, DAVINCI_MCBSP_SPCR_REG);
        spcr |= mask;

        if ((*dev).pcr & (DAVINCI_MCBSP_PCR_FSXM | DAVINCI_MCBSP_PCR_FSRM)) != 0 {
            spcr |= DAVINCI_MCBSP_SPCR_FRST;
        }
        davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr);
    }
}

fn davinci_mcbsp_stop(dev: *mut DavinciMcbspDev, playback: i32) {
    unsafe {
        let mut spcr = davinci_mcbsp_read_reg(dev, DAVINCI_MCBSP_SPCR_REG);
        spcr &= !(DAVINCI_MCBSP_SPCR_GRST | DAVINCI_MCBSP_SPCR_FRST);
        spcr &= if playback != 0 {
            !DAVINCI_MCBSP_SPCR_XRST
        } else {
            !DAVINCI_MCBSP_SPCR_RRST
        };
        davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr);
        toggle_clock(dev, playback);
    }
}

fn davinci_i2s_tdm_word_length(tdm_slot_width: i32) -> i32 {
    match tdm_slot_width {
        8 => DavinciMcbspWord::Word8 as i32,
        12 => DavinciMcbspWord::Word12 as i32,
        16 => DavinciMcbspWord::Word16 as i32,
        20 => DavinciMcbspWord::Word20 as i32,
        24 => DavinciMcbspWord::Word24 as i32,
        32 => DavinciMcbspWord::Word32 as i32,
        _ => -EINVAL,
    }
}

#[no_mangle]
extern "C" fn davinci_i2s_set_tdm_slot(
    cpu_dai: *mut snd_soc_dai,
    tx_mask: u32,
    rx_mask: u32,
    slots: i32,
    slot_width: i32,
) -> i32 {
    unsafe {
        let dev = snd_soc_dai_get_drvdata(cpu_dai) as *mut DavinciMcbspDev;
        dev_dbg(
            (*dev).dev,
            b"slots %d, slot_width %d\n\0".as_ptr(),
            slots,
            slot_width,
        );

        if slots > 128 || slots == 0 {
            dev_err((*dev).dev, b"Invalid number of slots\n\0".as_ptr());
            return -EINVAL;
        }

        if rx_mask != ((1u32 << slots as u32).wrapping_sub(1)) {
            dev_err(
                (*dev).dev,
                b"Invalid RX mask (0x%08x) : all slots must be used by McBSP\n\0".as_ptr(),
                rx_mask,
            );
            return -EINVAL;
        }

        if tx_mask != ((1u32 << slots as u32).wrapping_sub(1)) {
            dev_err(
                (*dev).dev,
                b"Invalid TX mask (0x%08x) : all slots must be used by McBSP\n\0".as_ptr(),
                tx_mask,
            );
            return -EINVAL;
        }

        if davinci_i2s_tdm_word_length(slot_width) < 0 {
            dev_err(
                (*dev).dev,
                b"%s: Unsupported slot_width %d\n\0".as_ptr(),
                b"davinci_i2s_set_tdm_slot\0".as_ptr(),
                slot_width,
            );
            return -EINVAL;
        }

        (*dev).tdm_slots = slots;
        (*dev).slot_width = slot_width;

        0
    }
}

const DEFAULT_BITPERSAMPLE: u32 = 16;

#[no_mangle]
extern "C" fn davinci_i2s_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    unsafe {
        let dev = snd_soc_dai_get_drvdata(cpu_dai) as *mut DavinciMcbspDev;
        let mut pcr: u32;
        let mut srgr: u32;
        let mut inv_fs = false;

        srgr = DAVINCI_MCBSP_SRGR_FSGM
            | ((DEFAULT_BITPERSAMPLE * 2 - 1) << 16)
            | ((DEFAULT_BITPERSAMPLE - 1) << 8);

        (*dev).fmt = fmt;

        let mut spcr = davinci_mcbsp_read_reg(dev, DAVINCI_MCBSP_SPCR_REG);
        match fmt & SND_SOC_DAIFMT_CLOCK_MASK {
            SND_SOC_DAIFMT_CONT => {
                spcr |= DAVINCI_MCBSP_SPCR_FREE;
                dev_dbg((*dev).dev, b"Free-running mode ON\n\0".as_ptr());
            }
            SND_SOC_DAIFMT_GATED => {
                spcr &= !DAVINCI_MCBSP_SPCR_FREE;
                dev_dbg((*dev).dev, b"Free-running mode OFF\n\0".as_ptr());
            }
            _ => {
                dev_err((*dev).dev, b"Invalid clock gating\n\0".as_ptr());
                return -EINVAL;
            }
        }
        davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr);

        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_BP_FP => {
                pcr = DAVINCI_MCBSP_PCR_FSXM
                    | DAVINCI_MCBSP_PCR_FSRM
                    | DAVINCI_MCBSP_PCR_CLKXM
                    | DAVINCI_MCBSP_PCR_CLKRM;
            }
            SND_SOC_DAIFMT_BC_FP => {
                if (*dev).tdm_slots != 0 || (*dev).slot_width != 0 {
                    dev_err(
                        (*dev).dev,
                        b"TDM is not supported for BC_FP format\n\0".as_ptr(),
                    );
                    return -EINVAL;
                }
                pcr = DAVINCI_MCBSP_PCR_FSRM | DAVINCI_MCBSP_PCR_FSXM;
                pcr |= DAVINCI_MCBSP_PCR_SCLKME;
            }
            SND_SOC_DAIFMT_BP_FC => {
                pcr = DAVINCI_MCBSP_PCR_CLKXM | DAVINCI_MCBSP_PCR_CLKRM;
            }
            SND_SOC_DAIFMT_BC_FC => {
                if (*dev).tdm_slots != 0 || (*dev).slot_width != 0 {
                    dev_err(
                        (*dev).dev,
                        b"TDM is not supported for BC_FC format\n\0".as_ptr(),
                    );
                    return -EINVAL;
                }
                pcr = 0;
            }
            _ => {
                printk(b"%s:bad master\n\0".as_ptr(), b"davinci_i2s_set_dai_fmt\0".as_ptr());
                return -EINVAL;
            }
        }

        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => {
                inv_fs = true;
                (*dev).mode = MOD_DSP_A;
            }
            SND_SOC_DAIFMT_DSP_A => {
                (*dev).mode = MOD_DSP_A;
            }
            SND_SOC_DAIFMT_DSP_B => {
                (*dev).mode = MOD_DSP_B;
            }
            _ => {
                printk(b"%s:bad format\n\0".as_ptr(), b"davinci_i2s_set_dai_fmt\0".as_ptr());
                return -EINVAL;
            }
        }

        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {
                pcr |= DAVINCI_MCBSP_PCR_CLKXP | DAVINCI_MCBSP_PCR_CLKRP;
            }
            SND_SOC_DAIFMT_IB_IF => {
                pcr |= DAVINCI_MCBSP_PCR_FSXP | DAVINCI_MCBSP_PCR_FSRP;
            }
            SND_SOC_DAIFMT_NB_IF => {
                pcr |= DAVINCI_MCBSP_PCR_CLKXP
                    | DAVINCI_MCBSP_PCR_CLKRP
                    | DAVINCI_MCBSP_PCR_FSXP
                    | DAVINCI_MCBSP_PCR_FSRP;
            }
            SND_SOC_DAIFMT_IB_NF => {}
            _ => {
                return -EINVAL;
            }
        }

        if inv_fs {
            pcr ^= DAVINCI_MCBSP_PCR_FSXP | DAVINCI_MCBSP_PCR_FSRP;
        }
        davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SRGR_REG, srgr);
        (*dev).pcr = pcr;
        davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_PCR_REG, pcr);
        0
    }
}

#[no_mangle]
extern "C" fn davinci_i2s_dai_set_clkdiv(
    cpu_dai: *mut snd_soc_dai,
    div_id: i32,
    div: i32,
) -> i32 {
    unsafe {
        let dev = snd_soc_dai_get_drvdata(cpu_dai) as *mut DavinciMcbspDev;

        if div_id != DAVINCI_MCBSP_CLKGDV {
            return -ENODEV;
        }

        (*dev).clk_div = div;
        0
    }
}

#[no_mangle]
extern "C" fn davinci_i2s_hw_params(
    substream: *const snd_pcm_substream,
    params: *const snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    unsafe {
        let dev = snd_soc_dai_get_drvdata(dai) as *mut DavinciMcbspDev;
        let mut mcbsp_word_length: i32;
        let master: u32;
        let mut clk_div: u32;
        let mut freq: u64;
        let mut framesize: u32;
        let mut srgr: u32 = 0;
        let mut rcr: u32 = 0;
        let mut xcr: u32 = 0;
        let mut element_cnt: u32 = 1;

        let mut spcr = davinci_mcbsp_read_reg(dev, DAVINCI_MCBSP_SPCR_REG);

        let fmt = params_format(params);
        match fmt {
            SNDRV_PCM_FORMAT_S16_LE | SNDRV_PCM_FORMAT_S32_LE => {}
            SNDRV_PCM_FORMAT_S24_LE => {
                spcr |= DAVINCI_MCBSP_SPCR_RJUST_S_LE;
            }
            _ => {
                dev_warn(
                    (*dev).dev,
                    b"davinci-i2s: unsupported PCM format\n\0".as_ptr(),
                );
                return -EINVAL;
            }
        }

        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            spcr |= 3 << 4;
            davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr);
        } else {
            spcr |= 3 << 20;
            davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr);
        }

        master = (*dev).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
        let fmt = params_format(params);
        if (*dev).slot_width != 0 {
            mcbsp_word_length = davinci_i2s_tdm_word_length((*dev).slot_width);
        } else {
            mcbsp_word_length = ASP_WORD_LENGTH[fmt as usize] as i32;
        }

        if mcbsp_word_length < 0 {
            return mcbsp_word_length;
        }

        match master {
            SND_SOC_DAIFMT_BP_FP => {
                if !(*dev).ext_clk.is_null() {
                    freq = clk_get_rate((*dev).ext_clk);
                } else {
                    freq = clk_get_rate((*dev).clk);
                    srgr = DAVINCI_MCBSP_SRGR_CLKSM;
                }
                srgr |= DAVINCI_MCBSP_SRGR_FSGM;
                srgr |= (((mcbsp_word_length as u32) * 8 - 1) << 8);
                if (*dev).i2s_accurate_sck {
                    clk_div = 256;
                    loop {
                        clk_div -= 1;
                        framesize = ((freq / clk_div as u64) as u32) / (*params).rate_num
                            * (*params).rate_den;
                        if !((framesize < 33 || framesize > 4095) && clk_div != 0) {
                            break;
                        }
                    }
                    clk_div -= 1;
                    srgr |= (framesize - 1) << 16;
                } else {
                    clk_div = (freq / ((mcbsp_word_length as u64) * 16)) as u32
                        / (*params).rate_num as u32
                        * (*params).rate_den as u32;
                    srgr |= (((mcbsp_word_length as u32) * 16 - 1) << 16);
                }
                clk_div &= 0xFF;
                srgr |= clk_div;
            }
            SND_SOC_DAIFMT_BC_FP => {
                srgr = DAVINCI_MCBSP_SRGR_FSGM;
                clk_div = ((*dev).clk_div - 1) as u32;
                srgr |= (((mcbsp_word_length as u32) * 8 - 1) << 8);
                srgr |= (((mcbsp_word_length as u32) * 16 - 1) << 16);
                clk_div &= 0xFF;
                srgr |= clk_div;
            }
            SND_SOC_DAIFMT_BP_FC => {
                if !(*dev).ext_clk.is_null() {
                    freq = clk_get_rate((*dev).ext_clk);
                } else {
                    freq = clk_get_rate((*dev).clk);
                    srgr = DAVINCI_MCBSP_SRGR_CLKSM;
                }
                if (*dev).tdm_slots != 0 && (*dev).slot_width != 0 {
                    clk_div = (freq / ((*params).rate_num as u64 * (*params).rate_den as u64))
                        as u32
                        / ((*dev).tdm_slots as u32 * (*dev).slot_width as u32)
                        - 1;
                } else {
                    clk_div = (freq / ((mcbsp_word_length as u64) * 16)) as u32
                        / (*params).rate_num as u32
                        * (*params).rate_den as u32;
                }
                clk_div &= 0xFF;
                srgr |= clk_div;
            }
            SND_SOC_DAIFMT_BC_FC => {
                let i = hw_param_interval(params, SNDRV_PCM_HW_PARAM_SAMPLE_BITS);
                srgr = DAVINCI_MCBSP_SRGR_FSGM;
                srgr |= (snd_interval_value(i) - 1) << 8;
                pr_debug(
                    b"%s - %d  FWID set: re-read srgr = %X\n\0".as_ptr(),
                    b"davinci_i2s_hw_params\0".as_ptr(),
                    652,
                    snd_interval_value(i) - 1,
                );

                let i = hw_param_interval(params, SNDRV_PCM_HW_PARAM_FRAME_BITS);
                srgr |= (snd_interval_value(i) - 1) << 16;
            }
            _ => {
                return -EINVAL;
            }
        }
        davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SRGR_REG, srgr);

        if (*dev).mode == MOD_DSP_B {
            rcr |= 0 << 16;
            xcr |= 0 << 16;
        } else {
            rcr |= 1 << 16;
            xcr |= 1 << 16;
        }

        if (*dev).tx_framing_bit {
            xcr &= !(1u32 << 16);
            xcr |= 2 << 16;
        }
        if (*dev).rx_framing_bit {
            rcr &= !(1u32 << 16);
            rcr |= 2 << 16;
        }

        if params_channels(params) == 2 {
            element_cnt = 2;
            if DOUBLE_FMT[fmt as usize] != 0 && (*dev).enable_channel_combine != 0 {
                element_cnt = 1;
            }
            match master {
                SND_SOC_DAIFMT_BP_FP | SND_SOC_DAIFMT_BP_FC => {
                    rcr |= 0 << 24;
                    xcr |= 0 << 24;
                    rcr |= DAVINCI_MCBSP_RCR_RPHASE;
                    xcr |= DAVINCI_MCBSP_XCR_XPHASE;
                }
                SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_BC_FP => {
                    rcr |= (element_cnt - 1) << 24;
                    xcr |= (element_cnt - 1) << 24;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }

        match master {
            SND_SOC_DAIFMT_BP_FP | SND_SOC_DAIFMT_BP_FC => {
                if (*dev).tdm_slots > 0 {
                    rcr |= ((*dev).tdm_slots as u32 - 1) << 8;
                    xcr |= ((*dev).tdm_slots as u32 - 1) << 8;
                } else {
                    rcr |= 0 << 8;
                    xcr |= 0 << 8;
                }
            }
            SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_BC_FP => {
                rcr |= (element_cnt - 1) << 8;
                xcr |= (element_cnt - 1) << 8;
            }
            _ => {
                return -EINVAL;
            }
        }

        rcr |= (mcbsp_word_length as u32) << 5;
        rcr |= (mcbsp_word_length as u32) << 21;
        xcr |= (mcbsp_word_length as u32) << 5;
        xcr |= (mcbsp_word_length as u32) << 21;

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_XCR_REG, xcr);
        } else {
            davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_RCR_REG, rcr);
        }

        pr_debug(
            b"%s - %d  srgr=%X\n\0".as_ptr(),
            b"davinci_i2s_hw_params\0".as_ptr(),
            649,
            srgr,
        );
        pr_debug(
            b"%s - %d  xcr=%X\n\0".as_ptr(),
            b"davinci_i2s_hw_params\0".as_ptr(),
            650,
            xcr,
        );
        pr_debug(
            b"%s - %d  rcr=%X\n\0".as_ptr(),
            b"davinci_i2s_hw_params\0".as_ptr(),
            651,
            rcr,
        );
        0
    }
}

#[no_mangle]
extern "C" fn davinci_i2s_prepare(
    substream: *const snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> i32 {
    unsafe {
        let dev = snd_soc_dai_get_drvdata(dai) as *mut DavinciMcbspDev;
        let playback = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            1
        } else {
            0
        };
        let mask = if playback != 0 {
            DAVINCI_MCBSP_SPCR_XRST
        } else {
            DAVINCI_MCBSP_SPCR_RRST
        };

        davinci_mcbsp_stop(dev, playback);

        let mut spcr = davinci_mcbsp_read_reg(dev, DAVINCI_MCBSP_SPCR_REG);
        if (spcr & mask) != 0 {
            davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr & !mask);
            toggle_clock(dev, playback);
        }
        if ((*dev).pcr
            & (DAVINCI_MCBSP_PCR_FSXM
                | DAVINCI_MCBSP_PCR_FSRM
                | DAVINCI_MCBSP_PCR_CLKXM
                | DAVINCI_MCBSP_PCR_CLKRM))
            != 0
        {
            spcr |= DAVINCI_MCBSP_SPCR_GRST;
            davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr);
        }

        if playback != 0 {
            spcr = davinci_mcbsp_read_reg(dev, DAVINCI_MCBSP_SPCR_REG);
            spcr |= DAVINCI_MCBSP_SPCR_XRST;
            davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr);

            udelay(100);

            spcr = davinci_mcbsp_read_reg(dev, DAVINCI_MCBSP_SPCR_REG);
            spcr &= !DAVINCI_MCBSP_SPCR_XRST;
            davinci_mcbsp_write_reg(dev, DAVINCI_MCBSP_SPCR_REG, spcr);
            toggle_clock(dev, playback);
        }

        0
    }
}

#[no_mangle]
extern "C" fn davinci_i2s_trigger(
    substream: *const snd_pcm_substream,
    cmd: i32,
    dai: *mut snd_soc_dai,
) -> i32 {
    unsafe {
        let dev = snd_soc_dai_get_drvdata(dai) as *mut DavinciMcbspDev;
        let playback = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            1
        } else {
            0
        };

        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                davinci_mcbsp_start(dev, substream);
                0
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                davinci_mcbsp_stop(dev, playback);
                0
            }
            _ => -EINVAL,
        }
    }
}

#[no_mangle]
extern "C" fn davinci_i2s_shutdown(
    substream: *const snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    unsafe {
        let dev = snd_soc_dai_get_drvdata(dai) as *mut DavinciMcbspDev;
        let playback = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            1
        } else {
            0
        };
        davinci_mcbsp_stop(dev, playback);
    }
}

const DAVINCI_I2S_RATES: u32 = 0x0fff;
const DAVINCI_I2S_FORMATS: u32 = (1 << (SNDRV_PCM_FORMAT_S16_LE as u32))
    | (1 << (SNDRV_PCM_FORMAT_S24_LE as u32))
    | (1 << (SNDRV_PCM_FORMAT_S32_LE as u32));

fn for_each_pcm_streams(mut f: impl FnMut(i32)) {
    f(SNDRV_PCM_STREAM_PLAYBACK);
    f(SNDRV_PCM_STREAM_CAPTURE);
}

#[no_mangle]
extern "C" fn davinci_i2s_dai_probe(dai: *mut snd_soc_dai) -> i32 {
    unsafe {
        let dev = snd_soc_dai_get_drvdata(dai) as *mut DavinciMcbspDev;

        for_each_pcm_streams(|stream| {
            snd_soc_dai_dma_data_set(
                dai,
                stream,
                &mut (*dev).dma_data[stream as usize] as *mut _,
            );
        });

        0
    }
}

#[repr(C)]
struct SndSocDaiOps {
    probe: Option<extern "C" fn(*mut snd_soc_dai) -> i32>,
    shutdown: Option<extern "C" fn(*const snd_pcm_substream, *mut snd_soc_dai)>,
    prepare: Option<extern "C" fn(*const snd_pcm_substream, *mut snd_soc_dai) -> i32>,
    trigger: Option<extern "C" fn(*const snd_pcm_substream, i32, *mut snd_soc_dai) -> i32>,
    hw_params: Option<
        extern "C" fn(*const snd_pcm_substream, *const snd_pcm_hw_params, *mut snd_soc_dai) -> i32,
    >,
    set_fmt: Option<extern "C" fn(*mut snd_soc_dai, u32) -> i32>,
    set_clkdiv: Option<extern "C" fn(*mut snd_soc_dai, i32, i32) -> i32>,
    set_tdm_slot: Option<
        extern "C" fn(*mut snd_soc_dai, u32, u32, i32, i32) -> i32,
    >,
}

static DAVINCI_I2S_DAI_OPS: SndSocDaiOps = SndSocDaiOps {
    probe: Some(davinci_i2s_dai_probe),
    shutdown: Some(davinci_i2s_shutdown),
    prepare: Some(davinci_i2s_prepare),
    trigger: Some(davinci_i2s_trigger),
    hw_params: Some(davinci_i2s_hw_params),
    set_fmt: Some(davinci_i2s_set_dai_fmt),
    set_clkdiv: Some(davinci_i2s_dai_set_clkdiv),
    set_tdm_slot: Some(davinci_i2s_set_tdm_slot),
};

#[repr(C)]
struct SndSocDaiCapabilities {
    channels_min: i32,
    channels_max: i32,
    rates: u32,
    formats: u32,
}

#[repr(C)]
struct SndSocDaiDriver {
    playback: SndSocDaiCapabilities,
    capture: SndSocDaiCapabilities,
    ops: *const SndSocDaiOps,
}

static DAVINCI_I2S_DAI: SndSocDaiDriver = SndSocDaiDriver {
    playback: SndSocDaiCapabilities {
        channels_min: 2,
        channels_max: 128,
        rates: DAVINCI_I2S_RATES,
        formats: DAVINCI_I2S_FORMATS,
    },
    capture: SndSocDaiCapabilities {
        channels_min: 2,
        channels_max: 128,
        rates: DAVINCI_I2S_RATES,
        formats: DAVINCI_I2S_FORMATS,
    },
    ops: &DAVINCI_I2S_DAI_OPS,
};

#[repr(C)]
struct SndSocComponentDriver {
    name: *const u8,
    legacy_dai_naming: i32,
}

static DAVINCI_I2S_COMPONENT: SndSocComponentDriver = SndSocComponentDriver {
    name: DRV_NAME.as_ptr(),
    legacy_dai_naming: 1,
};

#[no_mangle]
extern "C" fn davinci_i2s_probe(pdev: *mut platform_device) -> i32 {
    unsafe {
        let mem = platform_get_resource_byname(pdev, IORESOURCE_MEM, b"mpu\0".as_ptr());
        let mem = if mem.is_null() {
            dev_warn(
                &(*pdev).dev,
                b"\"mpu\" mem resource not found, using index 0\n\0".as_ptr(),
            );
            let m = platform_get_resource(pdev, IORESOURCE_MEM, 0);
            if m.is_null() {
                dev_err(&(*pdev).dev, b"no mem resource?\n\0".as_ptr());
                return -ENODEV;
            }
            m
        } else {
            mem
        };

        let io_base = devm_ioremap_resource(&(*pdev).dev, mem);
        if IS_ERR(io_base as *const core::ffi::c_void) {
            return PTR_ERR(io_base as *const core::ffi::c_void);
        }

        let dev = devm_kzalloc(
            &(*pdev).dev,
            core::mem::size_of::<DavinciMcbspDev>(),
            GFP_KERNEL,
        ) as *mut DavinciMcbspDev;
        if dev.is_null() {
            return -ENOMEM;
        }

        (*dev).base = io_base;

        (*dev).tx_framing_bit = of_property_read_bool(
            (*pdev).dev.of_node,
            b"ti,T1-framing-tx\0".as_ptr(),
        );
        (*dev).rx_framing_bit = of_property_read_bool(
            (*pdev).dev.of_node,
            b"ti,T1-framing-rx\0".as_ptr(),
        );

        let dma_data = &mut (*dev).dma_data[SNDRV_PCM_STREAM_PLAYBACK as usize];
        (*dma_data).addr = ((*mem).start + DAVINCI_MCBSP_DXR_REG as u64) as u64;

        let res = platform_get_resource(pdev, IORESOURCE_DMA, 0);
        if !res.is_null() {
            let dma = &mut (*dev).dma_request[SNDRV_PCM_STREAM_PLAYBACK as usize];
            *dma = (*res).start as i32;
            (*dma_data).filter_data = dma as *mut _ as *mut core::ffi::c_void;
        } else if IS_ENABLED(1) && !(*pdev).dev.of_node.is_null() {
            (*dma_data).filter_data = b"tx\0".as_ptr() as *mut core::ffi::c_void;
        } else {
            dev_err(&(*pdev).dev, b"Missing DMA tx resource\n\0".as_ptr());
            return -ENODEV;
        }

        let dma_data = &mut (*dev).dma_data[SNDRV_PCM_STREAM_CAPTURE as usize];
        (*dma_data).addr = ((*mem).start + DAVINCI_MCBSP_DRR_REG as u64) as u64;

        let res = platform_get_resource(pdev, IORESOURCE_DMA, 1);
        if !res.is_null() {
            let dma = &mut (*dev).dma_request[SNDRV_PCM_STREAM_CAPTURE as usize];
            *dma = (*res).start as i32;
            (*dma_data).filter_data = dma as *mut _ as *mut core::ffi::c_void;
        } else if IS_ENABLED(1) && !(*pdev).dev.of_node.is_null() {
            (*dma_data).filter_data = b"rx\0".as_ptr() as *mut core::ffi::c_void;
        } else {
            dev_err(&(*pdev).dev, b"Missing DMA rx resource\n\0".as_ptr());
            return -ENODEV;
        }

        (*dev).clk = devm_clk_get_optional(&(*pdev).dev, b"fck\0".as_ptr());
        if IS_ERR((*dev).clk as *const core::ffi::c_void) {
            return dev_err_probe(
                &(*pdev).dev,
                PTR_ERR((*dev).clk as *const core::ffi::c_void),
                b"Invalid functional clock\n\0".as_ptr(),
            );
        }
        if (*dev).clk.is_null() {
            (*dev).clk = devm_clk_get(&(*pdev).dev, b"\0".as_ptr());
            if IS_ERR((*dev).clk as *const core::ffi::c_void) {
                return dev_err_probe(
                    &(*pdev).dev,
                    PTR_ERR((*dev).clk as *const core::ffi::c_void),
                    b"Missing functional clock\n\0".as_ptr(),
                );
            }
        }

        (*dev).ext_clk = devm_clk_get_optional(&(*pdev).dev, b"clks\0".as_ptr());
        if IS_ERR((*dev).ext_clk as *const core::ffi::c_void) {
            return dev_err_probe(
                &(*pdev).dev,
                PTR_ERR((*dev).ext_clk as *const core::ffi::c_void),
                b"Invalid external clock\n\0".as_ptr(),
            );
        }

        let ret = clk_prepare_enable((*dev).clk);
        if ret != 0 {
            return ret;
        }

        if !(*dev).ext_clk.is_null() {
            dev_dbg(
                &(*pdev).dev,
                b"External clock used for sample rate generator\n\0".as_ptr(),
            );
            let ret = clk_prepare_enable((*dev).ext_clk);
            if ret != 0 {
                dev_err_probe(
                    &(*pdev).dev,
                    ret,
                    b"Failed to enable external clock\n\0".as_ptr(),
                );
                clk_disable_unprepare((*dev).clk);
                return ret;
            }
        }

        (*dev).dev = &mut (*pdev).dev;
        dev_set_drvdata(&(*pdev).dev, dev as *mut core::ffi::c_void);

        let ret = snd_soc_register_component(
            &(*pdev).dev,
            &DAVINCI_I2S_COMPONENT,
            &DAVINCI_I2S_DAI,
            1,
        );
        if ret != 0 {
            clk_disable_unprepare((*dev).ext_clk);
            clk_disable_unprepare((*dev).clk);
            return ret;
        }

        let ret = edma_pcm_platform_register(&(*pdev).dev);
        if ret != 0 {
            dev_err_probe(&(*pdev).dev, ret, b"register PCM failed\n\0".as_ptr());
            snd_soc_unregister_component(&(*pdev).dev);
            clk_disable_unprepare((*dev).ext_clk);
            clk_disable_unprepare((*dev).clk);
            return ret;
        }

        0
    }
}

#[no_mangle]
extern "C" fn davinci_i2s_remove(pdev: *mut platform_device) {
    unsafe {
        let dev = dev_get_drvdata(&(*pdev).dev) as *mut DavinciMcbspDev;

        snd_soc_unregister_component(&(*pdev).dev);

        clk_disable_unprepare((*dev).clk);

        clk_disable_unprepare((*dev).ext_clk);
    }
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const u8,
}

static DAVINCI_I2S_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"ti,da850-mcbsp\0".as_ptr(),
    },
    OfDeviceId {
        compatible: core::ptr::null(),
    },
];

#[repr(C)]
struct PlatformDriver {
    probe: Option<extern "C" fn(*mut platform_device) -> i32>,
    remove: Option<extern "C" fn(*mut platform_device)>,
    driver_name: *const u8,
    of_match_table: *const OfDeviceId,
}

static DAVINCI_MCBSP_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(davinci_i2s_probe),
    remove: Some(davinci_i2s_remove),
    driver_name: b"davinci-mcbsp\0".as_ptr(),
    of_match_table: DAVINCI_I2S_MATCH.as_ptr(),
};

fn IS_ENABLED(config: i32) -> bool {
    config != 0
}

// Placeholder to represent module registration
// In actual Rust kernel code, this would use kernel macros
#[no_mangle]
pub extern "C" fn davinci_mcbsp_init() -> i32 {
    // module_platform_driver equivalent
    0
}

#[no_mangle]
pub extern "C" fn davinci_mcbsp_exit() {
    // module cleanup
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
