// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//

/*
 * Generic Hardware interface for ACP Audio I2S controller
 */

// Dependencies from the original C includes:
// linux/platform_device.h, linux/module.h, linux/err.h, linux/io.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-dai.h, linux/dma-mapping.h,
// linux/bitfield.h, and "amd.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;

const DRV_NAME: &[u8] = b"acp_i2s_playcap\0";
const I2S_MASTER_MODE_ENABLE: u32 = 1;
const LRCLK_DIV_FIELD: u32 = GENMASK(10, 2);
const BCLK_DIV_FIELD: u32 = GENMASK(23, 11);
const ACP63_LRCLK_DIV_FIELD: u32 = GENMASK(12, 2);
const ACP63_BCLK_DIV_FIELD: u32 = GENMASK(23, 13);

const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

const fn BIT(n: u32) -> u32 {
    1u32.wrapping_shl(n)
}

const fn FIELD_PREP(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub id: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub driver: *mut snd_soc_dai_driver,
    pub id: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut core::ffi::c_void,
    pub period_size: u32,
    pub buffer_size: u32,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acp_resource {
    pub soc_mclk: bool,
    pub sram_pte_offset: u32,
    pub irqp_used: u32,
    pub offset: u32,
}

#[repr(C)]
pub struct acp_stream {
    pub list: list_head,
    pub dai_id: c_int,
    pub dir: c_uint,
    pub bytescount: u64,
    pub reg_offset: u32,
    pub pte_offset: u32,
    pub fifo_offset: u32,
    pub id: c_uint,
    pub irq_bit: c_uint,
}

#[repr(C)]
pub struct acp_chip_info {
    pub tdm_mode: c_int,
    pub acp_rev: c_int,
    pub lrclk_div: u32,
    pub bclk_div: u32,
    pub base: *mut u8,
    pub rsrc: *mut acp_resource,
    pub acp_lock: core::ffi::c_void,
    pub stream_list: list_head,
    pub tdm_tx_fmt: [u32; 3],
    pub tdm_rx_fmt: [u32; 3],
    pub xfer_tx_resolution: [u32; 3],
    pub xfer_rx_resolution: [u32; 3],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, u32, u32, c_int, c_int) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_int,
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn readl(addr: *mut u8) -> u32;
    fn writel(val: u32, addr: *mut u8);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: u32) -> u32;
    fn acp_get_byte_count(chip: *mut acp_chip_info, dai_id: c_int, stream: c_uint) -> u64;
    fn spin_lock_irq(lock: *mut core::ffi::c_void);
    fn spin_unlock_irq(lock: *mut core::ffi::c_void);
}

// External constants and register helper functions are supplied by translated dependencies.
unsafe extern "C" {
    static I2S_SP_INSTANCE: c_int;
    static I2S_BT_INSTANCE: c_int;
    static I2S_HS_INSTANCE: c_int;
    static ACP_I2STDM0_MSTRCLKGEN: u32;
    static ACP_I2STDM1_MSTRCLKGEN: u32;
    static ACP_I2STDM2_MSTRCLKGEN: u32;
    static ACP63_PCI_ID: c_int;
    static ACP70_PCI_ID: c_int;
    static ACP71_PCI_ID: c_int;
    static ACP72_PCI_ID: c_int;
    static ACP_RN_PCI_ID: c_int;
    static ACP_RMB_PCI_ID: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_int;
    static SND_SOC_DAIFMT_DSP_A: c_int;
    static TDM_DISABLE: c_int;
    static TDM_ENABLE: c_int;
    static EINVAL: c_int;
    static SLOT_WIDTH_8: c_int;
    static SLOT_WIDTH_16: c_int;
    static SLOT_WIDTH_24: c_int;
    static SLOT_WIDTH_32: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_uint;
    static FRM_LEN: u32;
    static SNDRV_PCM_FORMAT_U8: c_int;
    static SNDRV_PCM_FORMAT_S8: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static ACP3x_ITER_IRER_SAMP_LEN_MASK: u32;
    static ACP_BTTDM_ITER: u32;
    static ACP_BTTDM_TXFRMT: u32;
    static ACP_I2STDM_ITER: u32;
    static ACP_I2STDM_TXFRMT: u32;
    static ACP_HSTDM_ITER: u32;
    static ACP_HSTDM_TXFRMT: u32;
    static ACP_BTTDM_IRER: u32;
    static ACP_BTTDM_RXFRMT: u32;
    static ACP_I2STDM_IRER: u32;
    static ACP_I2STDM_RXFRMT: u32;
    static ACP_HSTDM_IRER: u32;
    static ACP_HSTDM_RXFRMT: u32;
    static ACP_BTTDM_IER: u32;
    static ACP_I2STDM_IER: u32;
    static ACP_HSTDM_IER: u32;
    static ACP_HS_TX_INTR_WATERMARK_SIZE: u32;
    static ACP_HS_TX_RINGBUFSIZE: u32;
    static ACP_HS_RX_INTR_WATERMARK_SIZE: u32;
    static ACP_HS_RX_RINGBUFSIZE: u32;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SP_PB_FIFO_ADDR_OFFSET: u32;
    static SP_CAPT_FIFO_ADDR_OFFSET: u32;
    static BT_PB_FIFO_ADDR_OFFSET: u32;
    static BT_CAPT_FIFO_ADDR_OFFSET: u32;
    static HS_PB_FIFO_ADDR_OFFSET: u32;
    static HS_CAPT_FIFO_ADDR_OFFSET: u32;
    static ACP7x_I2S_SP_TX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_SP_RX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_BT_TX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_BT_RX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_HS_TX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_HS_RX_MEM_WINDOW_START: u32;
    static I2S_SP_TX_MEM_WINDOW_START: u32;
    static I2S_SP_RX_MEM_WINDOW_START: u32;
    static I2S_BT_TX_MEM_WINDOW_START: u32;
    static I2S_HS_TX_MEM_WINDOW_START: u32;
    static I2S_HS_RX_MEM_WINDOW_START: u32;
    static DMA_SIZE: u32;
    static FIFO_SIZE: u32;
    static ACP_HS_TX_DMA_SIZE: u32;
    static ACP_HS_TX_FIFOADDR: u32;
    static ACP_HS_TX_FIFOSIZE: u32;
    static ACP_HS_TX_RINGBUFADDR: u32;
    static ACP_HS_RX_DMA_SIZE: u32;
    static ACP_HS_RX_FIFOADDR: u32;
    static ACP_HS_RX_FIFOSIZE: u32;
    static ACP_HS_RX_RINGBUFADDR: u32;
    static ACP_SRAM_SP_PB_PTE_OFFSET: u32;
    static ACP_SRAM_SP_CP_PTE_OFFSET: u32;
    static ACP_SRAM_BT_PB_PTE_OFFSET: u32;
    static ACP_SRAM_BT_CP_PTE_OFFSET: u32;
    static ACP_SRAM_HS_PB_PTE_OFFSET: u32;
    static ACP_SRAM_HS_CP_PTE_OFFSET: u32;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64;

    fn ACP_BT_TX_INTR_WATERMARK_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_INTR_WATERMARK_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_INTR_WATERMARK_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_INTR_WATERMARK_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_RINGBUFSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_RINGBUFSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_RINGBUFSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_RINGBUFSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_DMA_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_FIFOADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_FIFOSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_RINGBUFADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_DMA_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_FIFOADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_FIFOSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_RINGBUFADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_DMA_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_FIFOADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_FIFOSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_RINGBUFADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_DMA_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_FIFOADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_FIFOSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_RINGBUFADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_EXTERNAL_INTR_CNTL(chip: *mut acp_chip_info, irqp_used: u32) -> *mut u8;
    fn I2S_RX_THRESHOLD(offset: u32) -> u32;
    fn BT_RX_THRESHOLD(offset: u32) -> u32;
    fn I2S_TX_THRESHOLD(offset: u32) -> u32;
    fn BT_TX_THRESHOLD(offset: u32) -> u32;
    fn HS_RX_THRESHOLD(offset: u32) -> u32;
    fn HS_TX_THRESHOLD(offset: u32) -> u32;
}

unsafe fn ptr_add(base: *mut u8, offset: u32) -> *mut u8 {
    unsafe { base.add(offset as usize) }
}

unsafe extern "C" fn acp_set_i2s_clk(chip: *mut acp_chip_info, dai_id: c_int) {
    let i2s_clk_reg: u32;
    let mut val: u32;

    unsafe {
        if dai_id == I2S_SP_INSTANCE {
            i2s_clk_reg = ACP_I2STDM0_MSTRCLKGEN;
        } else if dai_id == I2S_BT_INSTANCE {
            i2s_clk_reg = ACP_I2STDM1_MSTRCLKGEN;
        } else if dai_id == I2S_HS_INSTANCE {
            i2s_clk_reg = ACP_I2STDM2_MSTRCLKGEN;
        } else {
            i2s_clk_reg = ACP_I2STDM0_MSTRCLKGEN;
        }

        val = I2S_MASTER_MODE_ENABLE;
        if (*chip).tdm_mode != 0 {
            val |= BIT(1);
        }

        if (*chip).acp_rev == ACP63_PCI_ID
            || (*chip).acp_rev == ACP70_PCI_ID
            || (*chip).acp_rev == ACP71_PCI_ID
            || (*chip).acp_rev == ACP72_PCI_ID
        {
            val |= FIELD_PREP(ACP63_LRCLK_DIV_FIELD, (*chip).lrclk_div);
            val |= FIELD_PREP(ACP63_BCLK_DIV_FIELD, (*chip).bclk_div);
        } else {
            val |= FIELD_PREP(LRCLK_DIV_FIELD, (*chip).lrclk_div);
            val |= FIELD_PREP(BCLK_DIV_FIELD, (*chip).bclk_div);
        }
        writel(val, ptr_add((*chip).base, i2s_clk_reg));
    }
}

unsafe extern "C" fn acp_i2s_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let dev = (*(*cpu_dai).component).dev;
        let chip = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
        let mode = fmt & SND_SOC_DAIFMT_FORMAT_MASK;

        if mode as c_int == SND_SOC_DAIFMT_I2S {
            (*chip).tdm_mode = TDM_DISABLE;
        } else if mode as c_int == SND_SOC_DAIFMT_DSP_A {
            (*chip).tdm_mode = TDM_ENABLE;
        } else {
            return -EINVAL;
        }
        0
    }
}

unsafe extern "C" fn acp_i2s_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: u32,
    rx_mask: u32,
    mut slots: c_int,
    slot_width: c_int,
) -> c_int {
    unsafe {
        let dev = (*(*dai).component).dev;
        let chip = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
        let slot_len: c_int;
        let no_of_slots: c_int;

        if slot_width == SLOT_WIDTH_8 {
            slot_len = 8;
        } else if slot_width == SLOT_WIDTH_16 {
            slot_len = 16;
        } else if slot_width == SLOT_WIDTH_24 {
            slot_len = 24;
        } else if slot_width == SLOT_WIDTH_32 {
            slot_len = 0;
        } else {
            dev_err(dev, b"Unsupported bitdepth %d\n\0".as_ptr(), slot_width);
            return -EINVAL;
        }

        if (*chip).acp_rev == ACP_RN_PCI_ID || (*chip).acp_rev == ACP_RMB_PCI_ID {
            if (1..=7).contains(&slots) {
                no_of_slots = slots;
            } else if slots == 8 {
                no_of_slots = 0;
            } else {
                dev_err(dev, b"Unsupported slots %d\n\0".as_ptr(), slots);
                return -EINVAL;
            }
        } else if (*chip).acp_rev == ACP63_PCI_ID
            || (*chip).acp_rev == ACP70_PCI_ID
            || (*chip).acp_rev == ACP71_PCI_ID
            || (*chip).acp_rev == ACP72_PCI_ID
        {
            if (1..=31).contains(&slots) {
                no_of_slots = slots;
            } else if slots == 32 {
                no_of_slots = 0;
            } else {
                dev_err(dev, b"Unsupported slots %d\n\0".as_ptr(), slots);
                return -EINVAL;
            }
        } else {
            dev_err(dev, b"Unknown chip revision %d\n\0".as_ptr(), (*chip).acp_rev);
            return -EINVAL;
        }

        slots = no_of_slots;

        spin_lock_irq(&mut (*chip).acp_lock);
        let mut pos = (*chip).stream_list.next;
        while pos != &mut (*chip).stream_list {
            let stream = (pos as *mut u8).sub(core::mem::offset_of!(acp_stream, list)) as *mut acp_stream;
            pos = (*pos).next;

            if (*dai).id != (*stream).dai_id {
                continue;
            }
            if (*chip).acp_rev == ACP_RN_PCI_ID || (*chip).acp_rev == ACP_RMB_PCI_ID {
                if tx_mask != 0 && (*stream).dir == SNDRV_PCM_STREAM_PLAYBACK {
                    (*chip).tdm_tx_fmt[((*stream).dai_id - 1) as usize] =
                        FRM_LEN | ((slots as u32) << 15) | ((slot_len as u32) << 18);
                } else if rx_mask != 0 && (*stream).dir == SNDRV_PCM_STREAM_CAPTURE {
                    (*chip).tdm_rx_fmt[((*stream).dai_id - 1) as usize] =
                        FRM_LEN | ((slots as u32) << 15) | ((slot_len as u32) << 18);
                }
            } else if (*chip).acp_rev == ACP63_PCI_ID
                || (*chip).acp_rev == ACP70_PCI_ID
                || (*chip).acp_rev == ACP71_PCI_ID
                || (*chip).acp_rev == ACP72_PCI_ID
            {
                if tx_mask != 0 && (*stream).dir == SNDRV_PCM_STREAM_PLAYBACK {
                    (*chip).tdm_tx_fmt[((*stream).dai_id - 1) as usize] =
                        FRM_LEN | ((slots as u32) << 13) | ((slot_len as u32) << 18);
                } else if rx_mask != 0 && (*stream).dir == SNDRV_PCM_STREAM_CAPTURE {
                    (*chip).tdm_rx_fmt[((*stream).dai_id - 1) as usize] =
                        FRM_LEN | ((slots as u32) << 13) | ((slot_len as u32) << 18);
                }
            } else {
                dev_err(dev, b"Unknown chip revision %d\n\0".as_ptr(), (*chip).acp_rev);
                spin_unlock_irq(&mut (*chip).acp_lock);
                return -EINVAL;
            }
        }
        spin_unlock_irq(&mut (*chip).acp_lock);
        0
    }
}

unsafe extern "C" fn acp_i2s_hwparams(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let dev = (*(*dai).component).dev;
        let chip = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
        let rsrc = (*chip).rsrc;
        let mut val: u32;
        let xfer_resolution: u32;
        let reg_val: u32;
        let fmt_reg: u32;
        let tdm_fmt: u32;
        let mut lrclk_div_val: u32;
        let mut bclk_div_val: u32;

        /* These values are as per Hardware Spec */
        if params_format(params) == SNDRV_PCM_FORMAT_U8 || params_format(params) == SNDRV_PCM_FORMAT_S8 {
            xfer_resolution = 0x0;
        } else if params_format(params) == SNDRV_PCM_FORMAT_S16_LE {
            xfer_resolution = 0x02;
        } else if params_format(params) == SNDRV_PCM_FORMAT_S24_LE {
            xfer_resolution = 0x04;
        } else if params_format(params) == SNDRV_PCM_FORMAT_S32_LE {
            xfer_resolution = 0x05;
        } else {
            return -EINVAL;
        }

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            if (*(*dai).driver).id == I2S_BT_INSTANCE {
                reg_val = ACP_BTTDM_ITER;
                fmt_reg = ACP_BTTDM_TXFRMT;
            } else if (*(*dai).driver).id == I2S_SP_INSTANCE {
                reg_val = ACP_I2STDM_ITER;
                fmt_reg = ACP_I2STDM_TXFRMT;
            } else if (*(*dai).driver).id == I2S_HS_INSTANCE {
                reg_val = ACP_HSTDM_ITER;
                fmt_reg = ACP_HSTDM_TXFRMT;
            } else {
                dev_err(dev, b"Invalid dai id %x\n\0".as_ptr(), (*(*dai).driver).id);
                return -EINVAL;
            }
            (*chip).xfer_tx_resolution[((*(*dai).driver).id - 1) as usize] = xfer_resolution;
        } else {
            if (*(*dai).driver).id == I2S_BT_INSTANCE {
                reg_val = ACP_BTTDM_IRER;
                fmt_reg = ACP_BTTDM_RXFRMT;
            } else if (*(*dai).driver).id == I2S_SP_INSTANCE {
                reg_val = ACP_I2STDM_IRER;
                fmt_reg = ACP_I2STDM_RXFRMT;
            } else if (*(*dai).driver).id == I2S_HS_INSTANCE {
                reg_val = ACP_HSTDM_IRER;
                fmt_reg = ACP_HSTDM_RXFRMT;
            } else {
                dev_err(dev, b"Invalid dai id %x\n\0".as_ptr(), (*(*dai).driver).id);
                return -EINVAL;
            }
            (*chip).xfer_rx_resolution[((*(*dai).driver).id - 1) as usize] = xfer_resolution;
        }

        val = readl(ptr_add((*chip).base, reg_val));
        val &= !ACP3x_ITER_IRER_SAMP_LEN_MASK;
        val = val | (xfer_resolution << 3);
        writel(val, ptr_add((*chip).base, reg_val));

        if (*chip).tdm_mode != 0 {
            val = readl(ptr_add((*chip).base, reg_val));
            writel(val | BIT(1), ptr_add((*chip).base, reg_val));
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                tdm_fmt = (*chip).tdm_tx_fmt[((*(*dai).driver).id - 1) as usize];
            } else {
                tdm_fmt = (*chip).tdm_rx_fmt[((*(*dai).driver).id - 1) as usize];
            }
            writel(tdm_fmt, ptr_add((*chip).base, fmt_reg));
        }

        if (*rsrc).soc_mclk {
            if params_format(params) == SNDRV_PCM_FORMAT_S16_LE {
                match params_rate(params) {
                    8000 => bclk_div_val = 768,
                    16000 => bclk_div_val = 384,
                    24000 => bclk_div_val = 256,
                    32000 => bclk_div_val = 192,
                    44100 | 48000 => bclk_div_val = 128,
                    88200 | 96000 => bclk_div_val = 64,
                    192000 => bclk_div_val = 32,
                    _ => return -EINVAL,
                }
                lrclk_div_val = 32;
            } else if params_format(params) == SNDRV_PCM_FORMAT_S32_LE {
                match params_rate(params) {
                    8000 => bclk_div_val = 384,
                    16000 => bclk_div_val = 192,
                    24000 => bclk_div_val = 128,
                    32000 => bclk_div_val = 96,
                    44100 | 48000 => bclk_div_val = 64,
                    88200 | 96000 => bclk_div_val = 32,
                    192000 => bclk_div_val = 16,
                    _ => return -EINVAL,
                }
                lrclk_div_val = 64;
            } else {
                return -EINVAL;
            }

            match params_rate(params) {
                8000 | 16000 | 24000 | 48000 | 96000 | 192000 => {
                    match params_channels(params) {
                        2 => {}
                        4 => {
                            bclk_div_val >>= 1;
                            lrclk_div_val <<= 1;
                        }
                        8 => {
                            bclk_div_val >>= 2;
                            lrclk_div_val <<= 2;
                        }
                        16 => {
                            bclk_div_val >>= 3;
                            lrclk_div_val <<= 3;
                        }
                        32 => {
                            bclk_div_val >>= 4;
                            lrclk_div_val <<= 4;
                        }
                        _ => {
                            dev_err(
                                dev,
                                b"Unsupported channels %#x\n\0".as_ptr(),
                                params_channels(params),
                            );
                        }
                    }
                }
                _ => {}
            }
            (*chip).lrclk_div = lrclk_div_val;
            (*chip).bclk_div = bclk_div_val;
        }
        0
    }
}

unsafe extern "C" fn acp_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let stream = (*(*substream).runtime).private_data as *mut acp_stream;
        let dev = (*(*dai).component).dev;
        let chip = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
        let rsrc = (*chip).rsrc;
        let mut val: u32;
        let period_bytes: u32;
        let reg_val: u32;
        let ier_val: u32;
        let water_val: u32;
        let buf_size: u32;
        let buf_reg: u32;

        period_bytes = frames_to_bytes((*substream).runtime, (*(*substream).runtime).period_size);
        buf_size = frames_to_bytes((*substream).runtime, (*(*substream).runtime).buffer_size);

        if cmd == SNDRV_PCM_TRIGGER_START
            || cmd == SNDRV_PCM_TRIGGER_RESUME
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
        {
            (*stream).bytescount = acp_get_byte_count(chip, (*stream).dai_id, (*substream).stream);
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                if (*(*dai).driver).id == I2S_BT_INSTANCE {
                    water_val = ACP_BT_TX_INTR_WATERMARK_SIZE(chip);
                    reg_val = ACP_BTTDM_ITER;
                    ier_val = ACP_BTTDM_IER;
                    buf_reg = ACP_BT_TX_RINGBUFSIZE(chip);
                } else if (*(*dai).driver).id == I2S_SP_INSTANCE {
                    water_val = ACP_I2S_TX_INTR_WATERMARK_SIZE(chip);
                    reg_val = ACP_I2STDM_ITER;
                    ier_val = ACP_I2STDM_IER;
                    buf_reg = ACP_I2S_TX_RINGBUFSIZE(chip);
                } else if (*(*dai).driver).id == I2S_HS_INSTANCE {
                    water_val = ACP_HS_TX_INTR_WATERMARK_SIZE;
                    reg_val = ACP_HSTDM_ITER;
                    ier_val = ACP_HSTDM_IER;
                    buf_reg = ACP_HS_TX_RINGBUFSIZE;
                } else {
                    dev_err(dev, b"Invalid dai id %x\n\0".as_ptr(), (*(*dai).driver).id);
                    return -EINVAL;
                }
            } else {
                if (*(*dai).driver).id == I2S_BT_INSTANCE {
                    water_val = ACP_BT_RX_INTR_WATERMARK_SIZE(chip);
                    reg_val = ACP_BTTDM_IRER;
                    ier_val = ACP_BTTDM_IER;
                    buf_reg = ACP_BT_RX_RINGBUFSIZE(chip);
                } else if (*(*dai).driver).id == I2S_SP_INSTANCE {
                    water_val = ACP_I2S_RX_INTR_WATERMARK_SIZE(chip);
                    reg_val = ACP_I2STDM_IRER;
                    ier_val = ACP_I2STDM_IER;
                    buf_reg = ACP_I2S_RX_RINGBUFSIZE(chip);
                } else if (*(*dai).driver).id == I2S_HS_INSTANCE {
                    water_val = ACP_HS_RX_INTR_WATERMARK_SIZE;
                    reg_val = ACP_HSTDM_IRER;
                    ier_val = ACP_HSTDM_IER;
                    buf_reg = ACP_HS_RX_RINGBUFSIZE;
                } else {
                    dev_err(dev, b"Invalid dai id %x\n\0".as_ptr(), (*(*dai).driver).id);
                    return -EINVAL;
                }
            }

            writel(period_bytes, ptr_add((*chip).base, water_val));
            writel(buf_size, ptr_add((*chip).base, buf_reg));
            if (*rsrc).soc_mclk {
                acp_set_i2s_clk(chip, (*(*dai).driver).id);
            }
            val = readl(ptr_add((*chip).base, reg_val));
            val = val | BIT(0);
            writel(val, ptr_add((*chip).base, reg_val));
            writel(1, ptr_add((*chip).base, ier_val));
            return 0;
        } else if cmd == SNDRV_PCM_TRIGGER_STOP
            || cmd == SNDRV_PCM_TRIGGER_SUSPEND
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                if (*(*dai).driver).id == I2S_BT_INSTANCE {
                    reg_val = ACP_BTTDM_ITER;
                } else if (*(*dai).driver).id == I2S_SP_INSTANCE {
                    reg_val = ACP_I2STDM_ITER;
                } else if (*(*dai).driver).id == I2S_HS_INSTANCE {
                    reg_val = ACP_HSTDM_ITER;
                } else {
                    dev_err(dev, b"Invalid dai id %x\n\0".as_ptr(), (*(*dai).driver).id);
                    return -EINVAL;
                }
            } else {
                if (*(*dai).driver).id == I2S_BT_INSTANCE {
                    reg_val = ACP_BTTDM_IRER;
                } else if (*(*dai).driver).id == I2S_SP_INSTANCE {
                    reg_val = ACP_I2STDM_IRER;
                } else if (*(*dai).driver).id == I2S_HS_INSTANCE {
                    reg_val = ACP_HSTDM_IRER;
                } else {
                    dev_err(dev, b"Invalid dai id %x\n\0".as_ptr(), (*(*dai).driver).id);
                    return -EINVAL;
                }
            }
            val = readl(ptr_add((*chip).base, reg_val));
            val = val & !BIT(0);
            writel(val, ptr_add((*chip).base, reg_val));

            if (readl(ptr_add((*chip).base, ACP_BTTDM_ITER)) & BIT(0)) == 0
                && (readl(ptr_add((*chip).base, ACP_BTTDM_IRER)) & BIT(0)) == 0
            {
                writel(0, ptr_add((*chip).base, ACP_BTTDM_IER));
            }
            if (readl(ptr_add((*chip).base, ACP_I2STDM_ITER)) & BIT(0)) == 0
                && (readl(ptr_add((*chip).base, ACP_I2STDM_IRER)) & BIT(0)) == 0
            {
                writel(0, ptr_add((*chip).base, ACP_I2STDM_IER));
            }
            if (readl(ptr_add((*chip).base, ACP_HSTDM_ITER)) & BIT(0)) == 0
                && (readl(ptr_add((*chip).base, ACP_HSTDM_IRER)) & BIT(0)) == 0
            {
                writel(0, ptr_add((*chip).base, ACP_HSTDM_IER));
            }
            return 0;
        } else {
            return -EINVAL;
        }
    }
}

unsafe extern "C" fn acp_i2s_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let dev = (*(*dai).component).dev;
        let chip = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
        let rsrc = (*chip).rsrc;
        let stream = (*(*substream).runtime).private_data as *mut acp_stream;
        let mut reg_dma_size: u32 = 0;
        let mut reg_fifo_size: u32 = 0;
        let mut reg_fifo_addr: u32 = 0;
        let mut phy_addr: u32 = 0;
        let mut acp_fifo_addr: u32 = 0;
        let mut ext_int_ctrl: u32;
        let dir: c_uint = (*substream).stream;

        if (*(*dai).driver).id == I2S_SP_INSTANCE {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                reg_dma_size = ACP_I2S_TX_DMA_SIZE(chip);
                acp_fifo_addr = (*rsrc).sram_pte_offset + SP_PB_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_I2S_TX_FIFOADDR(chip);
                reg_fifo_size = ACP_I2S_TX_FIFOSIZE(chip);

                if (*chip).acp_rev >= ACP70_PCI_ID {
                    phy_addr = ACP7x_I2S_SP_TX_MEM_WINDOW_START;
                } else {
                    phy_addr = I2S_SP_TX_MEM_WINDOW_START + (*stream).reg_offset;
                }
                writel(phy_addr, ptr_add((*chip).base, ACP_I2S_TX_RINGBUFADDR(chip)));
            } else {
                reg_dma_size = ACP_I2S_RX_DMA_SIZE(chip);
                acp_fifo_addr = (*rsrc).sram_pte_offset + SP_CAPT_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_I2S_RX_FIFOADDR(chip);
                reg_fifo_size = ACP_I2S_RX_FIFOSIZE(chip);

                if (*chip).acp_rev >= ACP70_PCI_ID {
                    phy_addr = ACP7x_I2S_SP_RX_MEM_WINDOW_START;
                } else {
                    phy_addr = I2S_SP_RX_MEM_WINDOW_START + (*stream).reg_offset;
                }
                writel(phy_addr, ptr_add((*chip).base, ACP_I2S_RX_RINGBUFADDR(chip)));
            }
        } else if (*(*dai).driver).id == I2S_BT_INSTANCE {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                reg_dma_size = ACP_BT_TX_DMA_SIZE(chip);
                acp_fifo_addr = (*rsrc).sram_pte_offset + BT_PB_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_BT_TX_FIFOADDR(chip);
                reg_fifo_size = ACP_BT_TX_FIFOSIZE(chip);

                if (*chip).acp_rev >= ACP70_PCI_ID {
                    phy_addr = ACP7x_I2S_BT_TX_MEM_WINDOW_START;
                } else {
                    phy_addr = I2S_BT_TX_MEM_WINDOW_START + (*stream).reg_offset;
                }
                writel(phy_addr, ptr_add((*chip).base, ACP_BT_TX_RINGBUFADDR(chip)));
            } else {
                reg_dma_size = ACP_BT_RX_DMA_SIZE(chip);
                acp_fifo_addr = (*rsrc).sram_pte_offset + BT_CAPT_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_BT_RX_FIFOADDR(chip);
                reg_fifo_size = ACP_BT_RX_FIFOSIZE(chip);

                if (*chip).acp_rev >= ACP70_PCI_ID {
                    phy_addr = ACP7x_I2S_BT_RX_MEM_WINDOW_START;
                } else {
                    phy_addr = I2S_BT_TX_MEM_WINDOW_START + (*stream).reg_offset;
                }
                writel(phy_addr, ptr_add((*chip).base, ACP_BT_RX_RINGBUFADDR(chip)));
            }
        } else if (*(*dai).driver).id == I2S_HS_INSTANCE {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                reg_dma_size = ACP_HS_TX_DMA_SIZE;
                acp_fifo_addr = (*rsrc).sram_pte_offset + HS_PB_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_HS_TX_FIFOADDR;
                reg_fifo_size = ACP_HS_TX_FIFOSIZE;

                if (*chip).acp_rev >= ACP70_PCI_ID {
                    phy_addr = ACP7x_I2S_HS_TX_MEM_WINDOW_START;
                } else {
                    phy_addr = I2S_HS_TX_MEM_WINDOW_START + (*stream).reg_offset;
                }
                writel(phy_addr, ptr_add((*chip).base, ACP_HS_TX_RINGBUFADDR));
            } else {
                reg_dma_size = ACP_HS_RX_DMA_SIZE;
                acp_fifo_addr = (*rsrc).sram_pte_offset + HS_CAPT_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_HS_RX_FIFOADDR;
                reg_fifo_size = ACP_HS_RX_FIFOSIZE;

                if (*chip).acp_rev >= ACP70_PCI_ID {
                    phy_addr = ACP7x_I2S_HS_RX_MEM_WINDOW_START;
                } else {
                    phy_addr = I2S_HS_RX_MEM_WINDOW_START + (*stream).reg_offset;
                }
                writel(phy_addr, ptr_add((*chip).base, ACP_HS_RX_RINGBUFADDR));
            }
        } else {
            dev_err(dev, b"Invalid dai id %x\n\0".as_ptr(), (*(*dai).driver).id);
            return -EINVAL;
        }

        writel(DMA_SIZE, ptr_add((*chip).base, reg_dma_size));
        writel(acp_fifo_addr, ptr_add((*chip).base, reg_fifo_addr));
        writel(FIFO_SIZE, ptr_add((*chip).base, reg_fifo_size));

        ext_int_ctrl = readl(ACP_EXTERNAL_INTR_CNTL(chip, (*rsrc).irqp_used));
        ext_int_ctrl |= BIT(I2S_RX_THRESHOLD((*rsrc).offset))
            | BIT(BT_RX_THRESHOLD((*rsrc).offset))
            | BIT(I2S_TX_THRESHOLD((*rsrc).offset))
            | BIT(BT_TX_THRESHOLD((*rsrc).offset))
            | BIT(HS_RX_THRESHOLD((*rsrc).offset))
            | BIT(HS_TX_THRESHOLD((*rsrc).offset));

        writel(ext_int_ctrl, ACP_EXTERNAL_INTR_CNTL(chip, (*rsrc).irqp_used));

        0
    }
}

unsafe extern "C" fn acp_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let stream = (*(*substream).runtime).private_data as *mut acp_stream;
        let dev = (*(*dai).component).dev;
        let chip = dev_get_drvdata((*dev).parent) as *mut acp_chip_info;
        let rsrc = (*chip).rsrc;
        let dir: c_uint = (*substream).stream;
        let irq_bit: c_uint;

        if (*(*dai).driver).id == I2S_SP_INSTANCE {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                irq_bit = BIT(I2S_TX_THRESHOLD((*rsrc).offset));
                (*stream).pte_offset = ACP_SRAM_SP_PB_PTE_OFFSET;
                (*stream).fifo_offset = SP_PB_FIFO_ADDR_OFFSET;
            } else {
                irq_bit = BIT(I2S_RX_THRESHOLD((*rsrc).offset));
                (*stream).pte_offset = ACP_SRAM_SP_CP_PTE_OFFSET;
                (*stream).fifo_offset = SP_CAPT_FIFO_ADDR_OFFSET;
            }
        } else if (*(*dai).driver).id == I2S_BT_INSTANCE {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                irq_bit = BIT(BT_TX_THRESHOLD((*rsrc).offset));
                (*stream).pte_offset = ACP_SRAM_BT_PB_PTE_OFFSET;
                (*stream).fifo_offset = BT_PB_FIFO_ADDR_OFFSET;
            } else {
                irq_bit = BIT(BT_RX_THRESHOLD((*rsrc).offset));
                (*stream).pte_offset = ACP_SRAM_BT_CP_PTE_OFFSET;
                (*stream).fifo_offset = BT_CAPT_FIFO_ADDR_OFFSET;
            }
        } else if (*(*dai).driver).id == I2S_HS_INSTANCE {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                irq_bit = BIT(HS_TX_THRESHOLD((*rsrc).offset));
                (*stream).pte_offset = ACP_SRAM_HS_PB_PTE_OFFSET;
                (*stream).fifo_offset = HS_PB_FIFO_ADDR_OFFSET;
            } else {
                irq_bit = BIT(HS_RX_THRESHOLD((*rsrc).offset));
                (*stream).pte_offset = ACP_SRAM_HS_CP_PTE_OFFSET;
                (*stream).fifo_offset = HS_CAPT_FIFO_ADDR_OFFSET;
            }
        } else {
            dev_err(dev, b"Invalid dai id %x\n\0".as_ptr(), (*(*dai).driver).id);
            return -EINVAL;
        }

        /* Save runtime dai configuration in stream */
        (*stream).id = ((*(*dai).driver).id as c_uint) + dir;
        (*stream).dai_id = (*(*dai).driver).id;
        (*stream).irq_bit = irq_bit;
        (*stream).dir = (*substream).stream;

        0
    }
}

static acp_i2s_selectable_formats: u64 =
    unsafe { SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_DSP_A };

#[no_mangle]
pub static asoc_acp_cpu_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(acp_i2s_startup),
    hw_params: Some(acp_i2s_hwparams),
    prepare: Some(acp_i2s_prepare),
    trigger: Some(acp_i2s_trigger),
    set_fmt: Some(acp_i2s_set_fmt),
    set_tdm_slot: Some(acp_i2s_set_tdm_slot),
    auto_selectable_formats: unsafe { &acp_i2s_selectable_formats },
    num_auto_selectable_formats: 1,
};

// EXPORT_SYMBOL_NS_GPL(asoc_acp_cpu_dai_ops, "SND_SOC_ACP_COMMON");

// MODULE_DESCRIPTION("AMD ACP Audio I2S controller");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_ALIAS(DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
