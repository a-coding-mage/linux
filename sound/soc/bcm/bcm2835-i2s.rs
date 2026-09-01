// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC I2S Audio Layer for Broadcom BCM2835 SoC
 *
 * Author:	Florian Meier <florian.meier@koalo.de>
 *		Copyright 2013
 *
 * Based on
 *	Raspberry Pi PCM I2S ALSA Driver
 *	Copyright (c) by Phil Poole 2013
 *
 *	ALSA SoC I2S (McBSP) Audio Layer for TI DAVINCI processor
 *      Vladimir Barinov, <vbarinov@embeddedalley.com>
 *	Copyright (C) 2007 MontaVista Software, Inc., <source@mvista.com>
 *
 *	OMAP ALSA SoC DAI driver using McBSP port
 *	Copyright (C) 2008 Nokia Corporation
 *	Contact: Jarkko Nikula <jarkko.nikula@bitmer.com>
 *		 Peter Ujfalusi <peter.ujfalusi@ti.com>
 *
 *	Freescale SSI ALSA SoC Digital Audio Interface (DAI) driver
 *	Author: Timur Tabi <timur@freescale.com>
 *	Copyright 2007-2010 Freescale Semiconductor, Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u64 = u64;
type u32 = u32;
type dma_addr_t = usize;
type bool_t = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: c_uint,
    pub maxburst: c_uint,
    pub flags: c_uint,
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
pub struct snd_soc_dai {
    _private: [u8; 0],
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_sample_bits: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub cache_type: c_uint,
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

unsafe extern "C" {
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn of_get_address(np: *mut c_void, index: c_int, size: *mut u64, flags: *mut c_uint) -> *const u32;
    fn be32_to_cpup(p: *const u32) -> u32;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(dev: *mut device, config: *const c_void, flags: c_uint) -> c_int;
}

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

fn hweight_long(v: usize) -> c_uint {
    v.count_ones()
}

fn ffs(v: c_uint) -> c_int {
    if v == 0 { 0 } else { v.trailing_zeros() as c_int + 1 }
}

fn fls(v: c_uint) -> c_int {
    if v == 0 { 0 } else { 32 - v.leading_zeros() as c_int }
}

fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

/* I2S registers */
const BCM2835_I2S_CS_A_REG: c_uint = 0x00;
const BCM2835_I2S_FIFO_A_REG: c_uint = 0x04;
const BCM2835_I2S_MODE_A_REG: c_uint = 0x08;
const BCM2835_I2S_RXC_A_REG: c_uint = 0x0c;
const BCM2835_I2S_TXC_A_REG: c_uint = 0x10;
const BCM2835_I2S_DREQ_A_REG: c_uint = 0x14;
const BCM2835_I2S_INTEN_A_REG: c_uint = 0x18;
const BCM2835_I2S_INTSTC_A_REG: c_uint = 0x1c;
const BCM2835_I2S_GRAY_REG: c_uint = 0x20;

/* I2S register settings */
const BCM2835_I2S_STBY: c_uint = BIT(25);
const BCM2835_I2S_SYNC: c_uint = BIT(24);
const BCM2835_I2S_RXSEX: c_uint = BIT(23);
const BCM2835_I2S_RXF: c_uint = BIT(22);
const BCM2835_I2S_TXE: c_uint = BIT(21);
const BCM2835_I2S_RXD: c_uint = BIT(20);
const BCM2835_I2S_TXD: c_uint = BIT(19);
const BCM2835_I2S_RXR: c_uint = BIT(18);
const BCM2835_I2S_TXW: c_uint = BIT(17);
const BCM2835_I2S_CS_RXERR: c_uint = BIT(16);
const BCM2835_I2S_CS_TXERR: c_uint = BIT(15);
const BCM2835_I2S_RXSYNC: c_uint = BIT(14);
const BCM2835_I2S_TXSYNC: c_uint = BIT(13);
const BCM2835_I2S_DMAEN: c_uint = BIT(9);
const fn BCM2835_I2S_RXTHR(v: c_uint) -> c_uint { v << 7 }
const fn BCM2835_I2S_TXTHR(v: c_uint) -> c_uint { v << 5 }
const BCM2835_I2S_RXCLR: c_uint = BIT(4);
const BCM2835_I2S_TXCLR: c_uint = BIT(3);
const BCM2835_I2S_TXON: c_uint = BIT(2);
const BCM2835_I2S_RXON: c_uint = BIT(1);
const BCM2835_I2S_EN: c_uint = 1;

const BCM2835_I2S_CLKDIS: c_uint = BIT(28);
const BCM2835_I2S_PDMN: c_uint = BIT(27);
const BCM2835_I2S_PDME: c_uint = BIT(26);
const BCM2835_I2S_FRXP: c_uint = BIT(25);
const BCM2835_I2S_FTXP: c_uint = BIT(24);
const BCM2835_I2S_CLKM: c_uint = BIT(23);
const BCM2835_I2S_CLKI: c_uint = BIT(22);
const BCM2835_I2S_FSM: c_uint = BIT(21);
const BCM2835_I2S_FSI: c_uint = BIT(20);
const fn BCM2835_I2S_FLEN(v: c_uint) -> c_uint { v << 10 }
const fn BCM2835_I2S_FSLEN(v: c_uint) -> c_uint { v }

const BCM2835_I2S_CHWEX: c_uint = BIT(15);
const BCM2835_I2S_CHEN: c_uint = BIT(14);
const fn BCM2835_I2S_CHPOS(v: c_uint) -> c_uint { v << 4 }
const fn BCM2835_I2S_CHWID(v: c_uint) -> c_uint { v }
const fn BCM2835_I2S_CH1(v: c_uint) -> c_uint { v << 16 }
const fn BCM2835_I2S_CH2(v: c_uint) -> c_uint { v }
const fn BCM2835_I2S_CH1_POS(v: c_uint) -> c_uint { BCM2835_I2S_CH1(BCM2835_I2S_CHPOS(v)) }
const fn BCM2835_I2S_CH2_POS(v: c_uint) -> c_uint { BCM2835_I2S_CH2(BCM2835_I2S_CHPOS(v)) }

const fn BCM2835_I2S_TX_PANIC(v: c_uint) -> c_uint { v << 24 }
const fn BCM2835_I2S_RX_PANIC(v: c_uint) -> c_uint { v << 16 }
const fn BCM2835_I2S_TX(v: c_uint) -> c_uint { v << 8 }
const fn BCM2835_I2S_RX(v: c_uint) -> c_uint { v }

const BCM2835_I2S_INT_RXERR: c_uint = BIT(3);
const BCM2835_I2S_INT_TXERR: c_uint = BIT(2);
const BCM2835_I2S_INT_RXR: c_uint = BIT(1);
const BCM2835_I2S_INT_TXW: c_uint = BIT(0);

/* Frame length register is 10 bit, maximum length 1024 */
const BCM2835_I2S_MAX_FRAME_LENGTH: c_int = 1024;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0;
const SND_SOC_DAIFMT_BP_FC: c_uint = 0;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_CONT: c_uint = 0;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_DSP_B: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_NB_IF: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_IB_NF: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_IB_IF: u64 = 0;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;
const REGCACHE_MAPLE: c_uint = 0;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint = 0;
const SND_DMAENGINE_PCM_DAI_FLAG_PACK: c_uint = 0;

/* General device struct */
#[repr(C)]
pub struct bcm2835_i2s_dev {
    pub dev: *mut device,
    pub dma_data: [snd_dmaengine_dai_dma_data; 2],
    pub fmt: c_uint,
    pub tdm_slots: c_uint,
    pub rx_mask: c_uint,
    pub tx_mask: c_uint,
    pub slot_width: c_uint,
    pub frame_length: c_uint,
    pub i2s_regmap: *mut regmap,
    pub clk: *mut clk,
    pub clk_prepared: bool_t,
    pub clk_rate: c_int,
}

unsafe extern "C" fn bcm2835_i2s_start_clock(dev: *mut bcm2835_i2s_dev) {
    let provider: c_uint = (*dev).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;

    if (*dev).clk_prepared {
        return;
    }

    match provider {
        SND_SOC_DAIFMT_BP_FP | SND_SOC_DAIFMT_BP_FC => {
            clk_prepare_enable((*dev).clk);
            (*dev).clk_prepared = true;
        }
        _ => {}
    }
}

unsafe extern "C" fn bcm2835_i2s_stop_clock(dev: *mut bcm2835_i2s_dev) {
    if (*dev).clk_prepared {
        clk_disable_unprepare((*dev).clk);
    }
    (*dev).clk_prepared = false;
}

unsafe extern "C" fn bcm2835_i2s_clear_fifos(dev: *mut bcm2835_i2s_dev, tx: bool_t, rx: bool_t) {
    let mut timeout: c_int = 1000;
    let mut syncval: u32 = 0;
    let mut csreg: u32 = 0;
    let i2s_active_state: u32;
    let clk_was_prepared: bool_t;
    let mut off: u32;
    let mut clr: u32;

    off = if tx { BCM2835_I2S_TXON } else { 0 };
    off |= if rx { BCM2835_I2S_RXON } else { 0 };

    clr = if tx { BCM2835_I2S_TXCLR } else { 0 };
    clr |= if rx { BCM2835_I2S_RXCLR } else { 0 };

    /* Backup the current state */
    regmap_read((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, &mut csreg);
    i2s_active_state = csreg & (BCM2835_I2S_RXON | BCM2835_I2S_TXON);

    /* Start clock if not running */
    clk_was_prepared = (*dev).clk_prepared;
    if !clk_was_prepared {
        bcm2835_i2s_start_clock(dev);
    }

    /* Stop I2S module */
    regmap_update_bits((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, off, 0);

    /*
     * Clear the FIFOs
     * Requires at least 2 PCM clock cycles to take effect
     */
    regmap_update_bits((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, clr, clr);

    /* Wait for 2 PCM clock cycles */

    /*
     * Toggle the SYNC flag. After 2 PCM clock cycles it can be read back
     * FIXME: This does not seem to work for slave mode!
     */
    regmap_read((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, &mut syncval);
    syncval &= BCM2835_I2S_SYNC;

    regmap_update_bits(
        (*dev).i2s_regmap,
        BCM2835_I2S_CS_A_REG,
        BCM2835_I2S_SYNC,
        !syncval,
    );

    /* Wait for the SYNC flag changing it's state */
    loop {
        timeout -= 1;
        if timeout == 0 {
            break;
        }
        regmap_read((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, &mut csreg);
        if (csreg & BCM2835_I2S_SYNC) != syncval {
            break;
        }
    }

    if timeout == 0 {
        dev_err((*dev).dev, c"I2S SYNC error!\n".as_ptr());
    }

    /* Stop clock if it was not running before */
    if !clk_was_prepared {
        bcm2835_i2s_stop_clock(dev);
    }

    /* Restore I2S state */
    regmap_update_bits(
        (*dev).i2s_regmap,
        BCM2835_I2S_CS_A_REG,
        BCM2835_I2S_RXON | BCM2835_I2S_TXON,
        i2s_active_state,
    );
}

unsafe extern "C" fn bcm2835_i2s_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;
    (*dev).fmt = fmt;
    0
}

unsafe extern "C" fn bcm2835_i2s_set_dai_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;

    if ratio == 0 {
        (*dev).tdm_slots = 0;
        return 0;
    }

    if ratio > BCM2835_I2S_MAX_FRAME_LENGTH as c_uint {
        return -EINVAL;
    }

    (*dev).tdm_slots = 2;
    (*dev).rx_mask = 0x03;
    (*dev).tx_mask = 0x03;
    (*dev).slot_width = ratio / 2;
    (*dev).frame_length = ratio;

    0
}

unsafe extern "C" fn bcm2835_i2s_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    mut rx_mask: c_uint,
    slots: c_int,
    width: c_int,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;

    if slots != 0 {
        if slots < 0 || width < 0 {
            return -EINVAL;
        }

        /* Limit masks to available slots */
        rx_mask &= GENMASK((slots - 1) as c_uint, 0);
        tx_mask &= GENMASK((slots - 1) as c_uint, 0);

        /*
         * The driver is limited to 2-channel setups.
         * Check that exactly 2 bits are set in the masks.
         */
        if hweight_long(rx_mask as usize) != 2 || hweight_long(tx_mask as usize) != 2 {
            return -EINVAL;
        }

        if slots * width > BCM2835_I2S_MAX_FRAME_LENGTH {
            return -EINVAL;
        }
    }

    (*dev).tdm_slots = slots as c_uint;

    (*dev).rx_mask = rx_mask;
    (*dev).tx_mask = tx_mask;
    (*dev).slot_width = width as c_uint;
    (*dev).frame_length = (slots * width) as c_uint;

    0
}

/*
 * Convert logical slot number into physical slot number.
 *
 * If odd_offset is 0 sequential number is identical to logical number.
 * This is used for DSP modes with slot numbering 0 1 2 3 ...
 *
 * Otherwise odd_offset defines the physical offset for odd numbered
 * slots. This is used for I2S and left/right justified modes to
 * translate from logical slot numbers 0 1 2 3 ... into physical slot
 * numbers 0 2 ... 3 4 ...
 */
unsafe extern "C" fn bcm2835_i2s_convert_slot(slot: c_uint, odd_offset: c_uint) -> c_int {
    if odd_offset == 0 {
        return slot as c_int;
    }

    if (slot & 1) != 0 {
        return ((slot >> 1) + odd_offset) as c_int;
    }

    (slot >> 1) as c_int
}

/*
 * Calculate channel position from mask and slot width.
 *
 * Mask must contain exactly 2 set bits.
 * Lowest set bit is channel 1 position, highest set bit channel 2.
 * The constant offset is added to both channel positions.
 *
 * If odd_offset is > 0 slot positions are translated to
 * I2S-style TDM slot numbering ( 0 2 ... 3 4...) with odd
 * logical slot numbers starting at physical slot odd_offset.
 */
unsafe extern "C" fn bcm2835_i2s_calc_channel_pos(
    ch1_pos: *mut c_uint,
    ch2_pos: *mut c_uint,
    mask: c_uint,
    width: c_uint,
    bit_offset: c_uint,
    odd_offset: c_uint,
) {
    *ch1_pos = (bcm2835_i2s_convert_slot((ffs(mask) - 1) as c_uint, odd_offset) as c_uint)
        * width + bit_offset;
    *ch2_pos = (bcm2835_i2s_convert_slot((fls(mask) - 1) as c_uint, odd_offset) as c_uint)
        * width + bit_offset;
}

unsafe extern "C" fn bcm2835_i2s_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;
    let mut data_length: c_uint;
    let mut data_delay: c_uint;
    let framesync_length: c_uint;
    let slots: c_uint;
    let slot_width: c_uint;
    let mut odd_slot_offset: c_uint;
    let frame_length: c_int;
    let bclk_rate: c_int;
    let rx_mask: c_uint;
    let tx_mask: c_uint;
    let mut rx_ch1_pos: c_uint = 0;
    let mut rx_ch2_pos: c_uint = 0;
    let mut tx_ch1_pos: c_uint = 0;
    let mut tx_ch2_pos: c_uint = 0;
    let mut mode: c_uint;
    let mut format: c_uint;
    let mut bit_clock_provider: bool_t = false;
    let mut frame_sync_provider: bool_t = false;
    let mut frame_start_falling_edge: bool_t = false;
    let mut csreg: u32 = 0;
    let mut ret: c_int = 0;

    /*
     * If a stream is already enabled,
     * the registers are already set properly.
     */
    regmap_read((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, &mut csreg);

    if (csreg & (BCM2835_I2S_TXON | BCM2835_I2S_RXON)) != 0 {
        return 0;
    }

    data_length = params_width(params);
    data_delay = 0;
    odd_slot_offset = 0;
    mode = 0;

    if (*dev).tdm_slots != 0 {
        slots = (*dev).tdm_slots;
        slot_width = (*dev).slot_width;
        frame_length = (*dev).frame_length as c_int;
        rx_mask = (*dev).rx_mask;
        tx_mask = (*dev).tx_mask;
        bclk_rate = ((*dev).frame_length * params_rate(params)) as c_int;
    } else {
        slots = 2;
        slot_width = params_width(params);
        rx_mask = 0x03;
        tx_mask = 0x03;

        frame_length = snd_soc_params_to_frame_size(params);
        if frame_length < 0 {
            return frame_length;
        }

        bclk_rate = snd_soc_params_to_bclk(params);
        if bclk_rate < 0 {
            return bclk_rate;
        }
    }

    /* Check if data fits into slots */
    if data_length > slot_width {
        return -EINVAL;
    }

    /* Check if CPU is bit clock provider */
    match (*dev).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP | SND_SOC_DAIFMT_BP_FC => {
            bit_clock_provider = true;
        }
        SND_SOC_DAIFMT_BC_FP | SND_SOC_DAIFMT_BC_FC => {
            bit_clock_provider = false;
        }
        _ => return -EINVAL,
    }

    /* Check if CPU is frame sync provider */
    match (*dev).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP | SND_SOC_DAIFMT_BC_FP => {
            frame_sync_provider = true;
        }
        SND_SOC_DAIFMT_BP_FC | SND_SOC_DAIFMT_BC_FC => {
            frame_sync_provider = false;
        }
        _ => return -EINVAL,
    }

    /* Clock should only be set up here if CPU is clock master */
    if bit_clock_provider && (!(*dev).clk_prepared || (*dev).clk_rate != bclk_rate) {
        if (*dev).clk_prepared {
            bcm2835_i2s_stop_clock(dev);
        }

        if (*dev).clk_rate != bclk_rate {
            ret = clk_set_rate((*dev).clk, bclk_rate);
            if ret != 0 {
                return ret;
            }
            (*dev).clk_rate = bclk_rate;
        }

        bcm2835_i2s_start_clock(dev);
    }

    /* Setup the frame format */
    format = BCM2835_I2S_CHEN;

    if data_length >= 24 {
        format |= BCM2835_I2S_CHWEX;
    }

    format |= BCM2835_I2S_CHWID((data_length - 8) & 0xf);

    /* CH2 format is the same as for CH1 */
    format = BCM2835_I2S_CH1(format) | BCM2835_I2S_CH2(format);

    match (*dev).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            /* I2S mode needs an even number of slots */
            if (slots & 1) != 0 {
                return -EINVAL;
            }

            /*
             * Use I2S-style logical slot numbering: even slots
             * are in first half of frame, odd slots in second half.
             */
            odd_slot_offset = slots >> 1;

            /* MSB starts one cycle after frame start */
            data_delay = 1;

            /* Setup frame sync signal for 50% duty cycle */
            framesync_length = frame_length as c_uint / 2;
            frame_start_falling_edge = true;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            if (slots & 1) != 0 {
                return -EINVAL;
            }

            odd_slot_offset = slots >> 1;
            data_delay = 0;
            framesync_length = frame_length as c_uint / 2;
            frame_start_falling_edge = false;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            if (slots & 1) != 0 {
                return -EINVAL;
            }

            /* Odd frame lengths aren't supported */
            if (frame_length & 1) != 0 {
                return -EINVAL;
            }

            odd_slot_offset = slots >> 1;
            data_delay = slot_width - data_length;
            framesync_length = frame_length as c_uint / 2;
            frame_start_falling_edge = false;
        }
        SND_SOC_DAIFMT_DSP_A => {
            data_delay = 1;
            framesync_length = 1;
            frame_start_falling_edge = false;
        }
        SND_SOC_DAIFMT_DSP_B => {
            data_delay = 0;
            framesync_length = 1;
            frame_start_falling_edge = false;
        }
        _ => return -EINVAL,
    }

    bcm2835_i2s_calc_channel_pos(
        &mut rx_ch1_pos,
        &mut rx_ch2_pos,
        rx_mask,
        slot_width,
        data_delay,
        odd_slot_offset,
    );
    bcm2835_i2s_calc_channel_pos(
        &mut tx_ch1_pos,
        &mut tx_ch2_pos,
        tx_mask,
        slot_width,
        data_delay,
        odd_slot_offset,
    );

    /*
     * Transmitting data immediately after frame start, eg
     * in left-justified or DSP mode A, only works stable
     * if bcm2835 is the frame clock provider.
     */
    if (rx_ch1_pos == 0 || tx_ch1_pos == 0) && !frame_sync_provider {
        dev_warn(
            (*dev).dev,
            c"Unstable consumer config detected, L/R may be swapped".as_ptr(),
        );
    }

    /*
     * Set format for both streams.
     * We cannot set another frame length
     * (and therefore word length) anyway,
     * so the format will be the same.
     */
    regmap_write(
        (*dev).i2s_regmap,
        BCM2835_I2S_RXC_A_REG,
        format | BCM2835_I2S_CH1_POS(rx_ch1_pos) | BCM2835_I2S_CH2_POS(rx_ch2_pos),
    );
    regmap_write(
        (*dev).i2s_regmap,
        BCM2835_I2S_TXC_A_REG,
        format | BCM2835_I2S_CH1_POS(tx_ch1_pos) | BCM2835_I2S_CH2_POS(tx_ch2_pos),
    );

    /* Setup the I2S mode */

    if data_length <= 16 {
        /*
         * Use frame packed mode (2 channels per 32 bit word)
         * We cannot set another frame length in the second stream
         * (and therefore word length) anyway,
         * so the format will be the same.
         */
        mode |= BCM2835_I2S_FTXP | BCM2835_I2S_FRXP;
    }

    mode |= BCM2835_I2S_FLEN((frame_length - 1) as c_uint);
    mode |= BCM2835_I2S_FSLEN(framesync_length);

    /* CLKM selects bcm2835 clock slave mode */
    if !bit_clock_provider {
        mode |= BCM2835_I2S_CLKM;
    }

    /* FSM selects bcm2835 frame sync slave mode */
    if !frame_sync_provider {
        mode |= BCM2835_I2S_FSM;
    }

    /* CLKI selects normal clocking mode, sampling on rising edge */
    match (*dev).fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_NB_IF => {
            mode |= BCM2835_I2S_CLKI;
        }
        SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_IB_IF => {}
        _ => return -EINVAL,
    }

    /* FSI selects frame start on falling edge */
    match (*dev).fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_IB_NF => {
            if frame_start_falling_edge {
                mode |= BCM2835_I2S_FSI;
            }
        }
        SND_SOC_DAIFMT_NB_IF | SND_SOC_DAIFMT_IB_IF => {
            if !frame_start_falling_edge {
                mode |= BCM2835_I2S_FSI;
            }
        }
        _ => return -EINVAL,
    }

    regmap_write((*dev).i2s_regmap, BCM2835_I2S_MODE_A_REG, mode);

    /* Setup the DMA parameters */
    regmap_update_bits(
        (*dev).i2s_regmap,
        BCM2835_I2S_CS_A_REG,
        BCM2835_I2S_RXTHR(1) | BCM2835_I2S_TXTHR(1) | BCM2835_I2S_DMAEN,
        0xffffffff,
    );

    regmap_update_bits(
        (*dev).i2s_regmap,
        BCM2835_I2S_DREQ_A_REG,
        BCM2835_I2S_TX_PANIC(0x10)
            | BCM2835_I2S_RX_PANIC(0x30)
            | BCM2835_I2S_TX(0x30)
            | BCM2835_I2S_RX(0x20),
        0xffffffff,
    );

    /* Clear FIFOs */
    bcm2835_i2s_clear_fifos(dev, true, true);

    dev_dbg(
        (*dev).dev,
        c"slots: %d width: %d rx mask: 0x%02x tx_mask: 0x%02x\n".as_ptr(),
        slots,
        slot_width,
        rx_mask,
        tx_mask,
    );

    dev_dbg(
        (*dev).dev,
        c"frame len: %d sync len: %d data len: %d\n".as_ptr(),
        frame_length,
        framesync_length,
        data_length,
    );

    dev_dbg(
        (*dev).dev,
        c"rx pos: %d,%d tx pos: %d,%d\n".as_ptr(),
        rx_ch1_pos,
        rx_ch2_pos,
        tx_ch1_pos,
        tx_ch2_pos,
    );

    dev_dbg(
        (*dev).dev,
        c"sampling rate: %d bclk rate: %d\n".as_ptr(),
        params_rate(params),
        bclk_rate,
    );

    dev_dbg(
        (*dev).dev,
        c"CLKM: %d CLKI: %d FSM: %d FSI: %d frame start: %s edge\n".as_ptr(),
        ((mode & BCM2835_I2S_CLKM) != 0) as c_int,
        ((mode & BCM2835_I2S_CLKI) != 0) as c_int,
        ((mode & BCM2835_I2S_FSM) != 0) as c_int,
        ((mode & BCM2835_I2S_FSI) != 0) as c_int,
        if (mode & BCM2835_I2S_FSI) != 0 { c"falling".as_ptr() } else { c"rising".as_ptr() },
    );

    ret
}

unsafe extern "C" fn bcm2835_i2s_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;
    let mut cs_reg: u32 = 0;

    /*
     * Clear both FIFOs if the one that should be started
     * is not empty at the moment. This should only happen
     * after overrun. Otherwise, hw_params would have cleared
     * the FIFO.
     */
    regmap_read((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, &mut cs_reg);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK as c_int && (cs_reg & BCM2835_I2S_TXE) == 0 {
        bcm2835_i2s_clear_fifos(dev, true, false);
    } else if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE as c_int && (cs_reg & BCM2835_I2S_RXD) != 0 {
        bcm2835_i2s_clear_fifos(dev, false, true);
    }

    0
}

unsafe extern "C" fn bcm2835_i2s_stop(
    dev: *mut bcm2835_i2s_dev,
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let mask: u32;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE as c_int {
        mask = BCM2835_I2S_RXON;
    } else {
        mask = BCM2835_I2S_TXON;
    }

    regmap_update_bits((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, mask, 0);

    /* Stop also the clock when not SND_SOC_DAIFMT_CONT */
    if snd_soc_dai_active(dai) == 0 && ((*dev).fmt & SND_SOC_DAIFMT_CONT) == 0 {
        bcm2835_i2s_stop_clock(dev);
    }
}

unsafe extern "C" fn bcm2835_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;
    let mask: u32;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            bcm2835_i2s_start_clock(dev);

            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE as c_int {
                mask = BCM2835_I2S_RXON;
            } else {
                mask = BCM2835_I2S_TXON;
            }

            regmap_update_bits((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, mask, mask);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            bcm2835_i2s_stop(dev, substream, dai);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn bcm2835_i2s_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;

    if snd_soc_dai_active(dai) != 0 {
        return 0;
    }

    /* Should this still be running stop it */
    bcm2835_i2s_stop_clock(dev);

    /* Enable PCM block */
    regmap_update_bits((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, BCM2835_I2S_EN, BCM2835_I2S_EN);

    /*
     * Disable STBY.
     * Requires at least 4 PCM clock cycles to take effect.
     */
    regmap_update_bits((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, BCM2835_I2S_STBY, BCM2835_I2S_STBY);

    0
}

unsafe extern "C" fn bcm2835_i2s_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;

    bcm2835_i2s_stop(dev, substream, dai);

    /* If both streams are stopped, disable module and clock */
    if snd_soc_dai_active(dai) != 0 {
        return;
    }

    /* Disable the module */
    regmap_update_bits((*dev).i2s_regmap, BCM2835_I2S_CS_A_REG, BCM2835_I2S_EN, 0);

    /*
     * Stopping clock is necessary, because stop does
     * not stop the clock when SND_SOC_DAIFMT_CONT
     */
    bcm2835_i2s_stop_clock(dev);
}

unsafe extern "C" fn bcm2835_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let dev = snd_soc_dai_get_drvdata(dai) as *mut bcm2835_i2s_dev;

    snd_soc_dai_init_dma_data(
        dai,
        &mut (*dev).dma_data[SNDRV_PCM_STREAM_PLAYBACK],
        &mut (*dev).dma_data[SNDRV_PCM_STREAM_CAPTURE],
    );

    0
}

static bcm2835_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S
        | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
        | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
        | SND_SOC_POSSIBLE_DAIFMT_DSP_A
        | SND_SOC_POSSIBLE_DAIFMT_DSP_B
        | SND_SOC_POSSIBLE_DAIFMT_NB_NF
        | SND_SOC_POSSIBLE_DAIFMT_NB_IF
        | SND_SOC_POSSIBLE_DAIFMT_IB_NF
        | SND_SOC_POSSIBLE_DAIFMT_IB_IF;

static bcm2835_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(bcm2835_i2s_dai_probe),
    startup: Some(bcm2835_i2s_startup),
    shutdown: Some(bcm2835_i2s_shutdown),
    prepare: Some(bcm2835_i2s_prepare),
    trigger: Some(bcm2835_i2s_trigger),
    hw_params: Some(bcm2835_i2s_hw_params),
    set_fmt: Some(bcm2835_i2s_set_dai_fmt),
    set_bclk_ratio: Some(bcm2835_i2s_set_dai_bclk_ratio),
    set_tdm_slot: Some(bcm2835_i2s_set_dai_tdm_slot),
    auto_selectable_formats: &bcm2835_selectable_formats,
    num_auto_selectable_formats: 1,
};

static mut bcm2835_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"bcm2835-i2s".as_ptr(),
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 384000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    },
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 384000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &bcm2835_i2s_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
};

unsafe extern "C" fn bcm2835_i2s_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        BCM2835_I2S_CS_A_REG | BCM2835_I2S_FIFO_A_REG | BCM2835_I2S_INTSTC_A_REG | BCM2835_I2S_GRAY_REG => true,
        _ => false,
    }
}

unsafe extern "C" fn bcm2835_i2s_precious_reg(_dev: *mut device, reg: c_uint) -> bool_t {
    match reg {
        BCM2835_I2S_FIFO_A_REG => true,
        _ => false,
    }
}

static bcm2835_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: BCM2835_I2S_GRAY_REG,
    precious_reg: Some(bcm2835_i2s_precious_reg),
    volatile_reg: Some(bcm2835_i2s_volatile_reg),
    cache_type: REGCACHE_MAPLE,
};

static bcm2835_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"bcm2835-i2s-comp".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn bcm2835_i2s_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut bcm2835_i2s_dev;
    let mut ret: c_int;
    let base: *mut c_void;
    let addr: *const u32;
    let dma_base: dma_addr_t;

    dev = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<bcm2835_i2s_dev>(), GFP_KERNEL)
        as *mut bcm2835_i2s_dev;
    if dev.is_null() {
        return -ENOMEM;
    }

    /* get the clock */
    (*dev).clk_prepared = false;
    (*dev).clk = devm_clk_get(&mut (*pdev).dev, ptr::null());
    if IS_ERR((*dev).clk) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR((*dev).clk) as isize, c"could not get clk\n".as_ptr());
    }

    /* Request ioarea */
    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    (*dev).i2s_regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, &bcm2835_regmap_config);
    if IS_ERR((*dev).i2s_regmap) {
        return PTR_ERR((*dev).i2s_regmap);
    }

    /* Set the DMA address - we have to parse DT ourselves */
    addr = of_get_address(ptr::null_mut(), 0, ptr::null_mut(), ptr::null_mut());
    if addr.is_null() {
        dev_err(&mut (*pdev).dev, c"could not get DMA-register address\n".as_ptr());
        return -EINVAL;
    }
    dma_base = be32_to_cpup(addr) as dma_addr_t;

    (*dev).dma_data[SNDRV_PCM_STREAM_PLAYBACK].addr = dma_base + BCM2835_I2S_FIFO_A_REG as dma_addr_t;

    (*dev).dma_data[SNDRV_PCM_STREAM_CAPTURE].addr = dma_base + BCM2835_I2S_FIFO_A_REG as dma_addr_t;

    /* Set the bus width */
    (*dev).dma_data[SNDRV_PCM_STREAM_PLAYBACK].addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*dev).dma_data[SNDRV_PCM_STREAM_CAPTURE].addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;

    /* Set burst */
    (*dev).dma_data[SNDRV_PCM_STREAM_PLAYBACK].maxburst = 2;
    (*dev).dma_data[SNDRV_PCM_STREAM_CAPTURE].maxburst = 2;

    /*
     * Set the PACK flag to enable S16_LE support (2 S16_LE values
     * packed into 32-bit transfers).
     */
    (*dev).dma_data[SNDRV_PCM_STREAM_PLAYBACK].flags = SND_DMAENGINE_PCM_DAI_FLAG_PACK;
    (*dev).dma_data[SNDRV_PCM_STREAM_CAPTURE].flags = SND_DMAENGINE_PCM_DAI_FLAG_PACK;

    /* Store the pdev */
    (*dev).dev = &mut (*pdev).dev;
    dev_set_drvdata(&mut (*pdev).dev, dev as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &bcm2835_i2s_component,
        &raw mut bcm2835_i2s_dai,
        1,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Could not register DAI: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Could not register PCM: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

static bcm2835_i2s_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"brcm,bcm2835-i2s".as_ptr() },
    of_device_id { compatible: ptr::null() },
];

/* MODULE_DEVICE_TABLE(of, bcm2835_i2s_of_match); */

static mut bcm2835_i2s_driver: platform_driver = platform_driver {
    probe: Some(bcm2835_i2s_probe),
    driver: platform_driver_inner {
        name: c"bcm2835-i2s".as_ptr(),
        of_match_table: bcm2835_i2s_of_match.as_ptr(),
    },
};

/* module_platform_driver(bcm2835_i2s_driver); */

/* MODULE_ALIAS("platform:bcm2835-i2s"); */
/* MODULE_DESCRIPTION("BCM2835 I2S interface"); */
/* MODULE_AUTHOR("Florian Meier <florian.meier@koalo.de>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
