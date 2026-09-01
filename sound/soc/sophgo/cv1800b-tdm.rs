// SPDX-License-Identifier: GPL-2.0

// Translated from Linux kernel C source. Includes are represented by the
// external declarations and opaque C-compatible types below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8 = u8;
type u32 = u32;
type u64 = u64;
type bool_ = bool;
type phys_addr_t = c_ulong;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ERANGE: c_int = 34;
const EOVERFLOW: c_int = 75;
const GFP_KERNEL: c_uint = 0;
const IORESOURCE_MEM: c_uint = 0x00000200;
const ULONG_MAX: c_ulong = c_ulong::MAX;

const DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint = 4;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SND_SOC_TRIGGER_ORDER_LDC: c_int = 1;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0xf000;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x4000;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x1000;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;

const fn genmask(h: u32, l: u32) -> u32 {
    if h == 31 && l == 0 {
        u32::MAX
    } else {
        (((1u64 << (h - l + 1)) - 1) << l) as u32
    }
}

fn u32_replace_bits(old: u32, val: impl Into<u64>, mask: u32) -> u32 {
    let shift = mask.trailing_zeros();
    (old & !mask) | ((((val.into()) as u32) << shift) & mask)
}

fn div_round_closest(n: u32, d: u32) -> u32 {
    (n.wrapping_add(d / 2)) / d
}

fn check_mul_overflow_u32(a: u32, b: u32, dst: *mut u32) -> bool {
    match a.checked_mul(b) {
        Some(v) => unsafe {
            *dst = v;
            false
        },
        None => true,
    }
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct resource {
    pub start: phys_addr_t,
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
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub trigger_stop: c_int,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: phys_addr_t,
    pub addr_width: c_uint,
    pub fifo_size: c_uint,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
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
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_inner,
}

unsafe extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_dmaengine_pcm_prepare_slave_config();
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_get_resource(pdev: *mut platform_device, ty: c_uint, num: c_uint) -> *mut resource;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

const TX_FIFO_SIZE: u32 = 1024;
const RX_FIFO_SIZE: u32 = 1024;
const TX_MAX_BURST: u32 = 8;
const RX_MAX_BURST: u32 = 8;

const CV1800B_DEF_FREQ: u32 = 24576000;
const CV1800B_DEF_MCLK_FS_RATIO: u32 = 256;

/* tdm registers */
const CV1800B_BLK_MODE_SETTING: usize = 0x000;
const CV1800B_FRAME_SETTING: usize = 0x004;
const CV1800B_SLOT_SETTING1: usize = 0x008;
const CV1800B_SLOT_SETTING2: usize = 0x00C;
const CV1800B_DATA_FORMAT: usize = 0x010;
const CV1800B_BLK_CFG: usize = 0x014;
const CV1800B_I2S_ENABLE: usize = 0x018;
const CV1800B_I2S_RESET: usize = 0x01C;
const CV1800B_I2S_INT_EN: usize = 0x020;
const CV1800B_I2S_INT: usize = 0x024;
const CV1800B_FIFO_THRESHOLD: usize = 0x028;
const CV1800B_LRCK_MASTER: usize = 0x02C; /* special clock only mode */
const CV1800B_FIFO_RESET: usize = 0x030;
const CV1800B_RX_STATUS: usize = 0x040;
const CV1800B_TX_STATUS: usize = 0x048;
const CV1800B_CLK_CTRL0: usize = 0x060;
const CV1800B_CLK_CTRL1: usize = 0x064;
const CV1800B_PCM_SYNTH: usize = 0x068;
const CV1800B_RX_RD_PORT: phys_addr_t = 0x080;
const CV1800B_TX_WR_PORT: phys_addr_t = 0x0C0;

/* CV1800B_BLK_MODE_SETTING (0x000) */
const BLK_TX_MODE_MASK: u32 = genmask(0, 0);
const BLK_MASTER_MODE_MASK: u32 = genmask(1, 1);
const BLK_DMA_MODE_MASK: u32 = genmask(7, 7);

/* CV1800B_CLK_CTRL1 (0x064) */
const CLK_MCLK_DIV_MASK: u32 = genmask(15, 0);
const CLK_BCLK_DIV_MASK: u32 = genmask(31, 16);

/* CV1800B_CLK_CTRL0 (0x060) */
const CLK_AUD_CLK_SEL_MASK: u32 = genmask(0, 0);
const CLK_BCLK_OUT_CLK_FORCE_EN_MASK: u32 = genmask(6, 6);
const CLK_MCLK_OUT_EN_MASK: u32 = genmask(7, 7);
const CLK_AUD_EN_MASK: u32 = genmask(8, 8);

/* CV1800B_I2S_RESET (0x01C) */
const RST_I2S_RESET_RX_MASK: u32 = genmask(0, 0);
const RST_I2S_RESET_TX_MASK: u32 = genmask(1, 1);

/* CV1800B_FIFO_RESET (0x030) */
const FIFO_RX_RESET_MASK: u32 = genmask(0, 0);
const FIFO_TX_RESET_MASK: u32 = genmask(16, 16);

/* CV1800B_I2S_ENABLE (0x018) */
const I2S_ENABLE_MASK: u32 = genmask(0, 0);

/* CV1800B_BLK_CFG (0x014) */
const BLK_AUTO_DISABLE_WITH_CH_EN_MASK: u32 = genmask(4, 4);
const BLK_RX_BLK_CLK_FORCE_EN_MASK: u32 = genmask(8, 8);
const BLK_RX_FIFO_DMA_CLK_FORCE_EN_MASK: u32 = genmask(9, 9);
const BLK_TX_BLK_CLK_FORCE_EN_MASK: u32 = genmask(16, 16);
const BLK_TX_FIFO_DMA_CLK_FORCE_EN_MASK: u32 = genmask(17, 17);

/* CV1800B_FRAME_SETTING (0x004) */
const FRAME_LENGTH_MASK: u32 = genmask(8, 0);
const FS_ACTIVE_LENGTH_MASK: u32 = genmask(23, 16);

/* CV1800B_I2S_INT_EN (0x020) */
const INT_I2S_INT_EN_MASK: u32 = genmask(8, 8);

/* CV1800B_SLOT_SETTING2 (0x00C) */
const SLOT_EN_MASK: u32 = genmask(15, 0);

/* CV1800B_LRCK_MASTER (0x02C) */
const LRCK_MASTER_ENABLE_MASK: u32 = genmask(0, 0);

/* CV1800B_DATA_FORMAT (0x010) */
const DF_WORD_LENGTH_MASK: u32 = genmask(2, 1);
const DF_TX_SOURCE_LEFT_ALIGN_MASK: u32 = genmask(6, 6);

/* CV1800B_FIFO_THRESHOLD (0x028) */
const FIFO_RX_THRESHOLD_MASK: u32 = genmask(4, 0);
const FIFO_TX_THRESHOLD_MASK: u32 = genmask(20, 16);
const FIFO_TX_HIGH_THRESHOLD_MASK: u32 = genmask(28, 24);

/* CV1800B_SLOT_SETTING1 (0x008) */
const SLOT_NUM_MASK: u32 = genmask(3, 0);
const SLOT_SIZE_MASK: u32 = genmask(13, 8);
const DATA_SIZE_MASK: u32 = genmask(20, 16);
const FB_OFFSET_MASK: u32 = genmask(28, 24);

#[repr(C)]
enum cv1800b_tdm_word_length {
    CV1800B_WORD_LENGTH_8_BIT = 0,
    CV1800B_WORD_LENGTH_16_BIT = 1,
    CV1800B_WORD_LENGTH_32_BIT = 2,
}

#[repr(C)]
struct cv1800b_i2s {
    base: *mut c_void,
    clk: *mut clk,
    sysclk: *mut clk,
    dev: *mut device,
    playback_dma: snd_dmaengine_dai_dma_data,
    capture_dma: snd_dmaengine_dai_dma_data,
    mclk_rate: u32,
    bclk_ratio_fixed: bool_,
    bclk_ratio: u32,
}

unsafe fn reg(base: *mut c_void, off: usize) -> *mut c_void {
    (base as *mut u8).add(off) as *mut c_void
}

unsafe fn cv1800b_setup_dma_struct(i2s: *mut cv1800b_i2s, phys_base: phys_addr_t) {
    (*i2s).playback_dma.addr = phys_base.wrapping_add(CV1800B_TX_WR_PORT);
    (*i2s).playback_dma.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*i2s).playback_dma.fifo_size = TX_FIFO_SIZE;
    (*i2s).playback_dma.maxburst = TX_MAX_BURST;

    (*i2s).capture_dma.addr = phys_base.wrapping_add(CV1800B_RX_RD_PORT);
    (*i2s).capture_dma.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*i2s).capture_dma.fifo_size = RX_FIFO_SIZE;
    (*i2s).capture_dma.maxburst = RX_MAX_BURST;
}

static cv1800b_i2s_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
};

unsafe fn cv1800b_reset_fifo(i2s: *mut cv1800b_i2s) {
    let mut val: u32;

    val = readl(reg((*i2s).base, CV1800B_FIFO_RESET));
    val = u32_replace_bits(val, 1u32, FIFO_RX_RESET_MASK);
    val = u32_replace_bits(val, 1u32, FIFO_TX_RESET_MASK);
    writel(val, reg((*i2s).base, CV1800B_FIFO_RESET));

    usleep_range(10, 20);

    val = readl(reg((*i2s).base, CV1800B_FIFO_RESET));
    val = u32_replace_bits(val, 0u32, FIFO_RX_RESET_MASK);
    val = u32_replace_bits(val, 0u32, FIFO_TX_RESET_MASK);
    writel(val, reg((*i2s).base, CV1800B_FIFO_RESET));
}

unsafe fn cv1800b_reset_i2s(i2s: *mut cv1800b_i2s) {
    let mut val: u32;

    val = readl(reg((*i2s).base, CV1800B_I2S_RESET));
    val = u32_replace_bits(val, 1u32, RST_I2S_RESET_RX_MASK);
    val = u32_replace_bits(val, 1u32, RST_I2S_RESET_TX_MASK);
    writel(val, reg((*i2s).base, CV1800B_I2S_RESET));

    usleep_range(10, 20);

    val = readl(reg((*i2s).base, CV1800B_I2S_RESET));
    val = u32_replace_bits(val, 0u32, RST_I2S_RESET_RX_MASK);
    val = u32_replace_bits(val, 0u32, RST_I2S_RESET_TX_MASK);
    writel(val, reg((*i2s).base, CV1800B_I2S_RESET));
}

unsafe fn cv1800b_set_mclk_div(i2s: *mut cv1800b_i2s, mclk_div: u32) {
    let mut val: u32;

    val = readl(reg((*i2s).base, CV1800B_CLK_CTRL1));
    val = u32_replace_bits(val, mclk_div, CLK_MCLK_DIV_MASK);
    writel(val, reg((*i2s).base, CV1800B_CLK_CTRL1));
    dev_dbg((*i2s).dev, b"mclk_div is set to %u\n\0".as_ptr() as *const c_char, mclk_div);
}

unsafe fn cv1800b_set_tx_mode(i2s: *mut cv1800b_i2s, is_tx: bool) {
    let mut val: u32;

    val = readl(reg((*i2s).base, CV1800B_BLK_MODE_SETTING));
    val = u32_replace_bits(val, is_tx as u32, BLK_TX_MODE_MASK);
    writel(val, reg((*i2s).base, CV1800B_BLK_MODE_SETTING));
    dev_dbg((*i2s).dev, b"tx_mode is set to %u\n\0".as_ptr() as *const c_char, is_tx as c_uint);
}

unsafe fn cv1800b_set_bclk_div(i2s: *mut cv1800b_i2s, bclk_div: u32) -> c_int {
    let mut val: u32;

    if bclk_div == 0 || bclk_div > 0xFFFF {
        return -EINVAL;
    }

    val = readl(reg((*i2s).base, CV1800B_CLK_CTRL1));
    val = u32_replace_bits(val, bclk_div, CLK_BCLK_DIV_MASK);
    writel(val, reg((*i2s).base, CV1800B_CLK_CTRL1));
    dev_dbg((*i2s).dev, b"bclk_div is set to %u\n\0".as_ptr() as *const c_char, bclk_div);
    0
}

/* set memory width of audio data , reg word_length */
unsafe fn cv1800b_set_word_length(i2s: *mut cv1800b_i2s, physical_width: c_uint) -> c_int {
    let word_length_val: u8;
    let mut val: u32;

    match physical_width {
        8 => word_length_val = cv1800b_tdm_word_length::CV1800B_WORD_LENGTH_8_BIT as u8,
        16 => word_length_val = cv1800b_tdm_word_length::CV1800B_WORD_LENGTH_16_BIT as u8,
        32 => word_length_val = cv1800b_tdm_word_length::CV1800B_WORD_LENGTH_32_BIT as u8,
        _ => {
            dev_dbg((*i2s).dev, b"can't set word_length field\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    val = readl(reg((*i2s).base, CV1800B_DATA_FORMAT));
    val = u32_replace_bits(val, word_length_val as u32, DF_WORD_LENGTH_MASK);
    writel(val, reg((*i2s).base, CV1800B_DATA_FORMAT));
    0
}

unsafe fn cv1800b_enable_clocks(i2s: *mut cv1800b_i2s, enabled: bool) {
    let mut val: u32;

    val = readl(reg((*i2s).base, CV1800B_CLK_CTRL0));
    val = u32_replace_bits(val, enabled as u32, CLK_AUD_EN_MASK);
    writel(val, reg((*i2s).base, CV1800B_CLK_CTRL0));
}

unsafe fn cv1800b_set_slot_settings(
    i2s: *mut cv1800b_i2s,
    slots: u32,
    physical_width: u32,
    data_size: u32,
) -> c_int {
    let slot_num: u32;
    let slot_size: u32;
    let frame_length: u32;
    let frame_active_length: u32;
    let mut val: u32;

    if slots == 0 || physical_width == 0 || data_size == 0 {
        dev_err((*i2s).dev, b"frame or slot settings are not valid\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if slots > 16 || physical_width > 64 || data_size > 32 {
        dev_err((*i2s).dev, b"frame or slot settings are not valid\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    slot_num = slots - 1;
    slot_size = physical_width - 1;
    frame_length = physical_width.wrapping_mul(slots).wrapping_sub(1);
    frame_active_length = physical_width - 1;

    if frame_length > 511 || frame_active_length > 255 {
        dev_err((*i2s).dev, b"frame or slot settings are not valid\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    val = readl(reg((*i2s).base, CV1800B_SLOT_SETTING1));
    val = u32_replace_bits(val, slot_size, SLOT_SIZE_MASK);
    val = u32_replace_bits(val, data_size - 1, DATA_SIZE_MASK);
    val = u32_replace_bits(val, slot_num, SLOT_NUM_MASK);
    writel(val, reg((*i2s).base, CV1800B_SLOT_SETTING1));

    val = readl(reg((*i2s).base, CV1800B_FRAME_SETTING));
    val = u32_replace_bits(val, frame_length, FRAME_LENGTH_MASK);
    val = u32_replace_bits(val, frame_active_length, FS_ACTIVE_LENGTH_MASK);
    writel(val, reg((*i2s).base, CV1800B_FRAME_SETTING));

    dev_dbg(
        (*i2s).dev,
        b"slot settings num: %u width: %u\n\0".as_ptr() as *const c_char,
        slots,
        physical_width,
    );
    0
}

/*
 * calculate mclk_div.
 * if requested value is bigger than optimal
 * leave mclk_div as 1. cff clock is capable
 * to handle it
 */
unsafe fn cv1800b_calc_mclk_div(target_mclk: c_uint, mclk_div: *mut u32) -> c_int {
    *mclk_div = 1;

    if target_mclk == 0 {
        return -EINVAL;
    }

    /* optimal parent frequency is close to CV1800B_DEF_FREQ */
    if target_mclk < CV1800B_DEF_FREQ {
        *mclk_div = div_round_closest(CV1800B_DEF_FREQ, target_mclk);
        if *mclk_div == 0 || *mclk_div > 0xFFFF {
            return -EINVAL;
        }
    }
    0
}

/*
 * set CCF clock and divider for this clock
 * mclk_clock = ccf_clock / mclk_div
 */
unsafe fn cv1800b_i2s_set_rate_for_mclk(i2s: *mut cv1800b_i2s, target_mclk: c_uint) -> c_int {
    let mut mclk_div: u32 = 1;
    let tmp: u64;
    let mut ret: c_int;
    let clk_rate: c_ulong;
    let actual: c_ulong;

    ret = cv1800b_calc_mclk_div(target_mclk, &mut mclk_div);
    if ret != 0 {
        dev_dbg(
            (*i2s).dev,
            b"can't calc mclk_div for freq %u\n\0".as_ptr() as *const c_char,
            target_mclk,
        );
        return ret;
    }

    tmp = (target_mclk as u64).wrapping_mul(mclk_div as u64);
    if tmp > ULONG_MAX as u64 {
        dev_err(
            (*i2s).dev,
            b"clk_rate overflow: freq=%u div=%u\n\0".as_ptr() as *const c_char,
            target_mclk,
            mclk_div,
        );
        return -ERANGE;
    }

    clk_rate = tmp as c_ulong;

    cv1800b_enable_clocks(i2s, false);

    ret = clk_set_rate((*i2s).sysclk, clk_rate);
    if ret != 0 {
        return ret;
    }

    actual = clk_get_rate((*i2s).sysclk);
    if clk_rate != actual {
        dev_err_ratelimited(
            (*i2s).dev,
            b"clk_set_rate failed %lu, actual is %lu\n\0".as_ptr() as *const c_char,
            clk_rate,
            actual,
        );
    }

    cv1800b_set_mclk_div(i2s, mclk_div);
    cv1800b_enable_clocks(i2s, true);

    0
}

unsafe extern "C" fn cv1800b_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_i2s;
    let rate: c_uint = params_rate(params);
    let channels: c_uint = params_channels(params);
    let physical_width: c_uint = params_physical_width(params);
    let data_width: c_int = params_width(params);
    let tx_mode: bool = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { true } else { false };
    let mut ret: c_int;
    let bclk_div: u32;
    let bclk_ratio: u32;
    let mclk_rate: u32;
    let mut tmp: u32 = 0;

    if data_width < 0 {
        return data_width;
    }

    if channels == 0 || rate == 0 || physical_width == 0 {
        return -EINVAL;
    }

    ret = cv1800b_set_slot_settings(i2s, channels, physical_width, data_width as u32);
    if ret != 0 {
        return ret;
    }

    if (*i2s).mclk_rate != 0 {
        mclk_rate = (*i2s).mclk_rate;
    } else {
        dev_dbg((*i2s).dev, b"mclk is not set by machine driver\n\0".as_ptr() as *const c_char);
        ret = cv1800b_i2s_set_rate_for_mclk(i2s, rate.wrapping_mul(CV1800B_DEF_MCLK_FS_RATIO));
        if ret != 0 {
            return ret;
        }
        mclk_rate = rate.wrapping_mul(CV1800B_DEF_MCLK_FS_RATIO);
    }

    bclk_ratio = if (*i2s).bclk_ratio_fixed {
        (*i2s).bclk_ratio
    } else {
        physical_width.wrapping_mul(channels)
    };

    if check_mul_overflow_u32(rate, bclk_ratio, &mut tmp) {
        return -EOVERFLOW;
    }

    if tmp == 0 {
        return -EINVAL;
    }
    if mclk_rate % tmp != 0 {
        dev_warn((*i2s).dev, b"mclk rate is not aligned to bclk or rate\n\0".as_ptr() as *const c_char);
    }

    bclk_div = div_round_closest(mclk_rate, tmp);

    ret = cv1800b_set_bclk_div(i2s, bclk_div);
    if ret != 0 {
        return ret;
    }

    ret = cv1800b_set_word_length(i2s, physical_width);
    if ret != 0 {
        return ret;
    }

    cv1800b_set_tx_mode(i2s, tx_mode);

    cv1800b_reset_fifo(i2s);
    cv1800b_reset_i2s(i2s);
    0
}

unsafe extern "C" fn cv1800b_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_i2s;
    let mut val: u32;

    let _ = substream;
    val = readl(reg((*i2s).base, CV1800B_I2S_ENABLE));

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            val = u32_replace_bits(val, 1u32, I2S_ENABLE_MASK);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            val = u32_replace_bits(val, 0u32, I2S_ENABLE_MASK);
        }
        _ => return -EINVAL,
    }
    writel(val, reg((*i2s).base, CV1800B_I2S_ENABLE));
    0
}

unsafe extern "C" fn cv1800b_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_i2s;
    let dai_link: *mut snd_soc_dai_link = (*rtd).dai_link;

    dev_dbg(
        (*i2s).dev,
        b"%s: dai=%s substream=%d\n\0".as_ptr() as *const c_char,
        b"cv1800b_i2s_startup\0".as_ptr() as *const c_char,
        (*dai).name,
        (*substream).stream,
    );
    /**
     * Ensure DMA is stopped before DAI
     * shutdown (prevents DW AXI DMAC stop/busy on next open).
     */
    (*dai_link).trigger_stop = SND_SOC_TRIGGER_ORDER_LDC;
    0
}

unsafe extern "C" fn cv1800b_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_i2s;

    if i2s.is_null() {
        dev_err((*dai).dev, b"no drvdata in DAI probe\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    snd_soc_dai_init_dma_data(dai, &mut (*i2s).playback_dma, &mut (*i2s).capture_dma);
    0
}

unsafe extern "C" fn cv1800b_i2s_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_i2s;
    let mut val: u32;
    let master: u32;

    /* only i2s format is supported */
    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_I2S {
        return -EINVAL;
    }

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            dev_dbg((*i2s).dev, b"set to master mode\n\0".as_ptr() as *const c_char);
            master = 1;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            dev_dbg((*i2s).dev, b"set to slave mode\n\0".as_ptr() as *const c_char);
            master = 0;
        }
        _ => return -EINVAL,
    }

    val = readl(reg((*i2s).base, CV1800B_BLK_MODE_SETTING));
    val = u32_replace_bits(val, master, BLK_MASTER_MODE_MASK);
    writel(val, reg((*i2s).base, CV1800B_BLK_MODE_SETTING));
    0
}

unsafe extern "C" fn cv1800b_i2s_dai_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_i2s;

    if ratio == 0 {
        return -EINVAL;
    }
    (*i2s).bclk_ratio = ratio;
    (*i2s).bclk_ratio_fixed = true;
    0
}

unsafe extern "C" fn cv1800b_i2s_dai_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_i2s;
    let mut ret: c_int;
    let mut val: u32;
    let output_enable: bool = if dir == SND_SOC_CLOCK_OUT { true } else { false };

    let _ = clk_id;
    dev_dbg(
        (*i2s).dev,
        b"%s called with %u\n\0".as_ptr() as *const c_char,
        b"cv1800b_i2s_dai_set_sysclk\0".as_ptr() as *const c_char,
        freq,
    );
    ret = cv1800b_i2s_set_rate_for_mclk(i2s, freq);
    if ret != 0 {
        return ret;
    }

    val = readl(reg((*i2s).base, CV1800B_CLK_CTRL0));
    val = u32_replace_bits(val, output_enable as u32, CLK_MCLK_OUT_EN_MASK);
    writel(val, reg((*i2s).base, CV1800B_CLK_CTRL0));

    (*i2s).mclk_rate = freq;
    0
}

static cv1800b_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(cv1800b_i2s_dai_probe),
    startup: Some(cv1800b_i2s_startup),
    hw_params: Some(cv1800b_i2s_hw_params),
    trigger: Some(cv1800b_i2s_trigger),
    set_fmt: Some(cv1800b_i2s_dai_set_fmt),
    set_bclk_ratio: Some(cv1800b_i2s_dai_set_bclk_ratio),
    set_sysclk: Some(cv1800b_i2s_dai_set_sysclk),
};

static mut cv1800b_i2s_dai_template: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"cv1800b-i2s\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &cv1800b_i2s_dai_ops,
};

static cv1800b_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"cv1800b-i2s\0".as_ptr() as *const c_char,
};

unsafe fn cv1800b_i2s_hw_disable(i2s: *mut cv1800b_i2s) {
    let mut val: u32;

    val = readl(reg((*i2s).base, CV1800B_I2S_ENABLE));
    val = u32_replace_bits(val, 0u32, I2S_ENABLE_MASK);
    writel(val, reg((*i2s).base, CV1800B_I2S_ENABLE));

    val = readl(reg((*i2s).base, CV1800B_CLK_CTRL0));
    val = u32_replace_bits(val, 0u32, CLK_AUD_EN_MASK);
    val = u32_replace_bits(val, 0u32, CLK_MCLK_OUT_EN_MASK);
    writel(val, reg((*i2s).base, CV1800B_CLK_CTRL0));

    val = readl(reg((*i2s).base, CV1800B_I2S_RESET));
    val = u32_replace_bits(val, 1u32, RST_I2S_RESET_RX_MASK);
    val = u32_replace_bits(val, 1u32, RST_I2S_RESET_TX_MASK);
    writel(val, reg((*i2s).base, CV1800B_I2S_RESET));

    val = readl(reg((*i2s).base, CV1800B_FIFO_RESET));
    val = u32_replace_bits(val, 1u32, FIFO_RX_RESET_MASK);
    val = u32_replace_bits(val, 1u32, FIFO_TX_RESET_MASK);
    writel(val, reg((*i2s).base, CV1800B_FIFO_RESET));
}

unsafe fn cv1800b_i2s_setup_tdm(i2s: *mut cv1800b_i2s) {
    let mut val: u32;

    val = readl(reg((*i2s).base, CV1800B_BLK_MODE_SETTING));
    val = u32_replace_bits(val, 1u32, BLK_DMA_MODE_MASK);
    writel(val, reg((*i2s).base, CV1800B_BLK_MODE_SETTING));

    val = readl(reg((*i2s).base, CV1800B_CLK_CTRL0));
    val = u32_replace_bits(val, 0u32, CLK_AUD_CLK_SEL_MASK);
    val = u32_replace_bits(val, 0u32, CLK_MCLK_OUT_EN_MASK);
    val = u32_replace_bits(val, 0u32, CLK_AUD_EN_MASK);
    writel(val, reg((*i2s).base, CV1800B_CLK_CTRL0));

    val = readl(reg((*i2s).base, CV1800B_FIFO_THRESHOLD));
    val = u32_replace_bits(val, 4u32, FIFO_RX_THRESHOLD_MASK);
    val = u32_replace_bits(val, 4u32, FIFO_TX_THRESHOLD_MASK);
    val = u32_replace_bits(val, 4u32, FIFO_TX_HIGH_THRESHOLD_MASK);
    writel(val, reg((*i2s).base, CV1800B_FIFO_THRESHOLD));

    val = readl(reg((*i2s).base, CV1800B_I2S_ENABLE));
    val = u32_replace_bits(val, 0u32, I2S_ENABLE_MASK);
    writel(val, reg((*i2s).base, CV1800B_I2S_ENABLE));
}

unsafe extern "C" fn cv1800b_i2s_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let i2s: *mut cv1800b_i2s;
    let res: *mut resource;
    let regs: *mut c_void;
    let mut ret: c_int;

    i2s = devm_kzalloc(dev, core::mem::size_of::<cv1800b_i2s>(), GFP_KERNEL) as *mut cv1800b_i2s;
    if i2s.is_null() {
        return -ENOMEM;
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }
    (*i2s).dev = &mut (*pdev).dev;
    (*i2s).base = regs;

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        return -ENODEV;
    }
    cv1800b_setup_dma_struct(i2s, (*res).start);

    (*i2s).clk = devm_clk_get_enabled(dev, b"i2s\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).clk as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*i2s).clk as *const c_void),
            b"failed to get+enable i2s\n\0".as_ptr() as *const c_char,
        );
    }
    (*i2s).sysclk = devm_clk_get_enabled(dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*i2s).sysclk as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*i2s).sysclk as *const c_void),
            b"failed to get+enable mclk\n\0".as_ptr() as *const c_char,
        );
    }

    platform_set_drvdata(pdev, i2s as *mut c_void);
    cv1800b_i2s_setup_tdm(i2s);

    ret = devm_snd_soc_register_component(
        dev,
        &cv1800b_i2s_component,
        &mut cv1800b_i2s_dai_template,
        1,
    );
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(dev, &cv1800b_i2s_pcm_config, 0);
    if ret != 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn cv1800b_i2s_remove(pdev: *mut platform_device) {
    let i2s = platform_get_drvdata(pdev) as *mut cv1800b_i2s;

    if i2s.is_null() {
        return;
    }
    cv1800b_i2s_hw_disable(i2s);
}

static cv1800b_i2s_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"sophgo,cv1800b-i2s\0".as_ptr() as *const c_char,
    },
    of_device_id {
        /* sentinel */
        compatible: core::ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, cv1800b_i2s_of_match); */

static mut cv1800b_i2s_driver: platform_driver = platform_driver {
    probe: Some(cv1800b_i2s_probe),
    remove: Some(cv1800b_i2s_remove),
    driver: platform_driver_inner {
        name: b"cv1800b-i2s\0".as_ptr() as *const c_char,
        of_match_table: cv1800b_i2s_of_match.as_ptr(),
    },
};

/* module_platform_driver(cv1800b_i2s_driver); */

/* MODULE_DESCRIPTION("Sophgo cv1800b I2S/TDM driver"); */
/* MODULE_AUTHOR("Anton D. Stavinsky <stavinsky@gmail.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
