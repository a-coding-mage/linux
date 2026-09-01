// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * kirkwood-i2s.c
 *
 * (c) 2010 Arnaud Patard <apatard@mandriva.com>
 * (c) 2010 Arnaud Patard <arnaud.patard@rtp-net.org>
 */

// Rust translation of the C implementation source. Includes from Linux, ALSA
// SoC, platform data, OF, and "kirkwood.h" are represented by external
// declarations used below.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type uint32_t = u32;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const KIRKWOOD_I2S_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

const KIRKWOOD_SPDIF_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

/* These registers are relative to the second register region -
 * audio pll configuration.
 */
const A38X_PLL_CONF_REG0: c_ulong = 0x0;
const A38X_PLL_FB_CLK_DIV_OFFSET: c_uint = 10;
const A38X_PLL_FB_CLK_DIV_MASK: c_ulong = 0x7fc00;
const A38X_PLL_CONF_REG1: c_ulong = 0x4;
const A38X_PLL_FREQ_OFFSET_MASK: c_ulong = 0xffff;
const A38X_PLL_FREQ_OFFSET_VALID: c_ulong = BIT(16) as c_ulong;
const A38X_PLL_SW_RESET: c_ulong = BIT(31) as c_ulong;
const A38X_PLL_CONF_REG2: c_ulong = 0x8;
const A38X_PLL_AUDIO_POSTDIV_MASK: c_ulong = 0x7f;

/* Bit below belongs to SoC control register corresponding to the third
 * register region.
 */
const A38X_SPDIF_MODE_ENABLE: u32 = BIT(27);

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub no_period_wakeup: bool,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub id: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kirkwood_asoc_platform_data {
    pub burst: c_uint,
}

#[repr(C)]
pub struct kirkwood_dma_data {
    pub pll_config: *mut c_void,
    pub soc_control: *mut c_void,
    pub io: *mut c_void,
    pub irq: c_int,
    pub burst: c_uint,
    pub clk: *mut clk,
    pub extclk: *mut clk,
    pub ctl_play: uint32_t,
    pub ctl_rec: uint32_t,
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
    static kirkwood_soc_component: snd_soc_component_driver;
    static DRV_NAME: c_char;

    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_FORMAT_S32_LE: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;

    static KIRKWOOD_I2S_CTL_RJ: c_ulong;
    static KIRKWOOD_I2S_CTL_LJ: c_ulong;
    static KIRKWOOD_I2S_CTL_I2S: c_ulong;
    static KIRKWOOD_I2S_CTL_JUST_MASK: c_ulong;
    static KIRKWOOD_I2S_PLAYCTL: c_uint;
    static KIRKWOOD_I2S_RECCTL: c_uint;
    static KIRKWOOD_DCO_CTL_OFFSET_0: c_ulong;
    static KIRKWOOD_DCO_CTL_FREQ_11: c_ulong;
    static KIRKWOOD_DCO_CTL_FREQ_12: c_ulong;
    static KIRKWOOD_DCO_CTL_FREQ_24: c_ulong;
    static KIRKWOOD_DCO_CTL: c_uint;
    static KIRKWOOD_DCO_SPCR_STATUS: c_uint;
    static KIRKWOOD_DCO_SPCR_STATUS_DCO_LOCK: c_ulong;
    static KIRKWOOD_MCLK_SOURCE_DCO: uint32_t;
    static KIRKWOOD_MCLK_SOURCE_EXTCLK: uint32_t;
    static KIRKWOOD_CLOCKS_CTRL: c_uint;
    static KIRKWOOD_I2S_CTL_SIZE_MASK: c_ulong;
    static KIRKWOOD_I2S_CTL_SIZE_16: c_ulong;
    static KIRKWOOD_I2S_CTL_SIZE_24: c_ulong;
    static KIRKWOOD_I2S_CTL_SIZE_32: c_ulong;
    static KIRKWOOD_PLAYCTL_SIZE_16_C: uint32_t;
    static KIRKWOOD_PLAYCTL_SIZE_24: uint32_t;
    static KIRKWOOD_PLAYCTL_SIZE_32: uint32_t;
    static KIRKWOOD_PLAYCTL_I2S_EN: uint32_t;
    static KIRKWOOD_PLAYCTL_SPDIF_EN: uint32_t;
    static KIRKWOOD_RECCTL_SIZE_16_C: uint32_t;
    static KIRKWOOD_RECCTL_SIZE_24: uint32_t;
    static KIRKWOOD_RECCTL_SIZE_32: uint32_t;
    static KIRKWOOD_RECCTL_I2S_EN: uint32_t;
    static KIRKWOOD_RECCTL_SPDIF_EN: uint32_t;
    static KIRKWOOD_PLAYCTL_MONO_BOTH: uint32_t;
    static KIRKWOOD_PLAYCTL_MONO_OFF: uint32_t;
    static KIRKWOOD_PLAYCTL_MONO_MASK: uint32_t;
    static KIRKWOOD_PLAYCTL_ENABLE_MASK: uint32_t;
    static KIRKWOOD_PLAYCTL_SIZE_MASK: uint32_t;
    static KIRKWOOD_RECCTL_ENABLE_MASK: uint32_t;
    static KIRKWOOD_RECCTL_SIZE_MASK: uint32_t;
    static KIRKWOOD_PLAYCTL_I2S_MUTE: c_uint;
    static KIRKWOOD_PLAYCTL_SPDIF_MUTE: c_uint;
    static KIRKWOOD_PLAYCTL: c_uint;
    static KIRKWOOD_PLAYCTL_PLAY_BUSY: uint32_t;
    static KIRKWOOD_PLAYCTL_PAUSE: uint32_t;
    static KIRKWOOD_INT_MASK: c_uint;
    static KIRKWOOD_INT_CAUSE_PLAY_BYTES: uint32_t;
    static KIRKWOOD_INT_CAUSE_REC_BYTES: uint32_t;
    static KIRKWOOD_RECCTL: c_uint;
    static KIRKWOOD_RECCTL_PAUSE: uint32_t;
    static KIRKWOOD_RECCTL_MUTE: uint32_t;
    static KIRKWOOD_INT_CAUSE: c_uint;
    static KIRKWOOD_PLAYCTL_BURST_32: uint32_t;
    static KIRKWOOD_RECCTL_BURST_32: uint32_t;
    static KIRKWOOD_PLAYCTL_BURST_128: uint32_t;
    static KIRKWOOD_RECCTL_BURST_128: uint32_t;

    fn devm_platform_ioremap_resource_byname(
        pdev: *mut platform_device,
        name: *const c_char,
    ) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: c_ulong, addr: *mut c_void);
    fn of_property_read_bool(np: *const device_node, propname: *const c_char) -> bool;
    fn of_device_is_compatible(np: *const device_node, compatible: *const c_char) -> bool;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_notice(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn udelay(usecs: c_ulong);
    fn msleep(msecs: c_uint);
    fn cpu_relax();
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut kirkwood_dma_data;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut kirkwood_dma_data,
    );
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_is_match(p: *mut clk, q: *mut clk) -> bool;
    fn devm_clk_put(dev: *mut device, clk: *mut clk);
    fn ERR_PTR(error: c_int) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
}

unsafe fn ptr_add(base: *mut c_void, offset: c_ulong) -> *mut c_void {
    (base as *mut u8).add(offset as usize) as *mut c_void
}

unsafe extern "C" fn armada_38x_i2s_init_quirk(
    pdev: *mut platform_device,
    priv_: *mut kirkwood_dma_data,
    dai_drv: *mut snd_soc_dai_driver,
) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut reg_val: u32;
    let mut i: c_int;

    (*priv_).pll_config = devm_platform_ioremap_resource_byname(pdev, c"pll_regs".as_ptr());
    if IS_ERR((*priv_).pll_config) {
        return -ENOMEM;
    }

    (*priv_).soc_control = devm_platform_ioremap_resource_byname(pdev, c"soc_ctrl".as_ptr());
    if IS_ERR((*priv_).soc_control) {
        return -ENOMEM;
    }

    /* Select one of exceptive modes: I2S or S/PDIF */
    reg_val = readl((*priv_).soc_control);
    if of_property_read_bool(np, c"spdif-mode".as_ptr()) {
        reg_val |= A38X_SPDIF_MODE_ENABLE;
        dev_info(&mut (*pdev).dev, c"using S/PDIF mode\n".as_ptr());
    } else {
        reg_val &= !A38X_SPDIF_MODE_ENABLE;
        dev_info(&mut (*pdev).dev, c"using I2S mode\n".as_ptr());
    }
    writel(reg_val as c_ulong, (*priv_).soc_control);

    /* Update available rates of mclk's fs */
    i = 0;
    while i < 2 {
        (*dai_drv.add(i as usize)).playback.rates |= SNDRV_PCM_RATE_192000;
        (*dai_drv.add(i as usize)).capture.rates |= SNDRV_PCM_RATE_192000;
        i += 1;
    }

    0
}

unsafe fn armada_38x_set_pll(base: *mut c_void, rate: c_ulong) {
    let mut reg_val: u32;
    let mut freq_offset: u16 = 0x22b0;
    let audio_postdiv: u8;
    let mut fb_clk_div: u8 = 0x1d;

    /* Set frequency offset value to not valid and enable PLL reset */
    reg_val = readl(ptr_add(base, A38X_PLL_CONF_REG1));
    reg_val &= !(A38X_PLL_FREQ_OFFSET_VALID as u32);
    reg_val &= !(A38X_PLL_SW_RESET as u32);
    writel(reg_val as c_ulong, ptr_add(base, A38X_PLL_CONF_REG1));

    udelay(1);

    /* Update PLL parameters */
    match rate {
        44100 => {
            freq_offset = 0x735;
            fb_clk_div = 0x1b;
            audio_postdiv = 0xc;
        }
        48000 => {
            audio_postdiv = 0xc;
        }
        96000 => {
            audio_postdiv = 0x6;
        }
        192000 => {
            audio_postdiv = 0x3;
        }
        _ => {
            freq_offset = 0x735;
            fb_clk_div = 0x1b;
            audio_postdiv = 0xc;
        }
    }

    reg_val = readl(ptr_add(base, A38X_PLL_CONF_REG0));
    reg_val &= !(A38X_PLL_FB_CLK_DIV_MASK as u32);
    reg_val |= (fb_clk_div as u32) << A38X_PLL_FB_CLK_DIV_OFFSET;
    writel(reg_val as c_ulong, ptr_add(base, A38X_PLL_CONF_REG0));

    reg_val = readl(ptr_add(base, A38X_PLL_CONF_REG2));
    reg_val &= !(A38X_PLL_AUDIO_POSTDIV_MASK as u32);
    reg_val |= audio_postdiv as u32;
    writel(reg_val as c_ulong, ptr_add(base, A38X_PLL_CONF_REG2));

    reg_val = readl(ptr_add(base, A38X_PLL_CONF_REG1));
    reg_val &= !(A38X_PLL_FREQ_OFFSET_MASK as u32);
    reg_val |= freq_offset as u32;
    writel(reg_val as c_ulong, ptr_add(base, A38X_PLL_CONF_REG1));

    udelay(1);

    /* Disable reset */
    reg_val |= A38X_PLL_SW_RESET as u32;
    writel(reg_val as c_ulong, ptr_add(base, A38X_PLL_CONF_REG1));

    /* Wait 50us for PLL to lock */
    udelay(50);

    /* Restore frequency offset value validity */
    reg_val |= A38X_PLL_FREQ_OFFSET_VALID as u32;
    writel(reg_val as c_ulong, ptr_add(base, A38X_PLL_CONF_REG1));
}

unsafe extern "C" fn kirkwood_i2s_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(cpu_dai);
    let mask: c_ulong;
    let mut value: c_ulong;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            mask = KIRKWOOD_I2S_CTL_RJ;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            mask = KIRKWOOD_I2S_CTL_LJ;
        }
        x if x == SND_SOC_DAIFMT_I2S => {
            mask = KIRKWOOD_I2S_CTL_I2S;
        }
        _ => {
            return -EINVAL;
        }
    }

    /*
     * Set same format for playback and record
     * This avoids some troubles.
     */
    value = readl(ptr_add((*priv_).io, KIRKWOOD_I2S_PLAYCTL as c_ulong)) as c_ulong;
    value &= !KIRKWOOD_I2S_CTL_JUST_MASK;
    value |= mask;
    writel(value, ptr_add((*priv_).io, KIRKWOOD_I2S_PLAYCTL as c_ulong));

    value = readl(ptr_add((*priv_).io, KIRKWOOD_I2S_RECCTL as c_ulong)) as c_ulong;
    value &= !KIRKWOOD_I2S_CTL_JUST_MASK;
    value |= mask;
    writel(value, ptr_add((*priv_).io, KIRKWOOD_I2S_RECCTL as c_ulong));

    0
}

unsafe fn kirkwood_set_dco(io: *mut c_void, rate: c_ulong) {
    let mut value: c_ulong;

    value = KIRKWOOD_DCO_CTL_OFFSET_0;
    match rate {
        44100 => {
            value |= KIRKWOOD_DCO_CTL_FREQ_11;
        }
        48000 => {
            value |= KIRKWOOD_DCO_CTL_FREQ_12;
        }
        96000 => {
            value |= KIRKWOOD_DCO_CTL_FREQ_24;
        }
        _ => {
            value |= KIRKWOOD_DCO_CTL_FREQ_11;
        }
    }
    writel(value, ptr_add(io, KIRKWOOD_DCO_CTL as c_ulong));

    /* wait for dco locked */
    loop {
        cpu_relax();
        value = readl(ptr_add(io, KIRKWOOD_DCO_SPCR_STATUS as c_ulong)) as c_ulong;
        value &= KIRKWOOD_DCO_SPCR_STATUS_DCO_LOCK;
        if value != 0 {
            break;
        }
    }
}

unsafe fn kirkwood_set_rate(dai: *mut snd_soc_dai, priv_: *mut kirkwood_dma_data, rate: c_ulong) {
    let clks_ctrl: uint32_t;

    if IS_ERR((*priv_).extclk as *const c_void) {
        /* use internal dco for the supported rates
         * defined in kirkwood_i2s_dai */
        dev_dbg((*dai).dev, c"%s: dco set rate = %lu\n".as_ptr(), c"kirkwood_set_rate".as_ptr(), rate);
        if !(*priv_).pll_config.is_null() {
            armada_38x_set_pll((*priv_).pll_config, rate);
        } else {
            kirkwood_set_dco((*priv_).io, rate);
        }

        clks_ctrl = KIRKWOOD_MCLK_SOURCE_DCO;
    } else {
        /* use the external clock for the other rates
         * defined in kirkwood_i2s_dai_extclk */
        dev_dbg(
            (*dai).dev,
            c"%s: extclk set rate = %lu -> %lu\n".as_ptr(),
            c"kirkwood_set_rate".as_ptr(),
            rate,
            256 * rate,
        );
        clk_set_rate((*priv_).extclk, 256 * rate);

        clks_ctrl = KIRKWOOD_MCLK_SOURCE_EXTCLK;
    }
    writel(clks_ctrl as c_ulong, ptr_add((*priv_).io, KIRKWOOD_CLOCKS_CTRL as c_ulong));
}

unsafe extern "C" fn kirkwood_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);

    snd_soc_dai_set_dma_data(dai, substream, priv_);
    0
}

unsafe extern "C" fn kirkwood_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let mut ctl_play: uint32_t;
    let mut ctl_rec: uint32_t;
    let i2s_reg: c_uint;
    let mut i2s_value: c_ulong;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        i2s_reg = KIRKWOOD_I2S_PLAYCTL;
    } else {
        i2s_reg = KIRKWOOD_I2S_RECCTL;
    }

    kirkwood_set_rate(dai, priv_, params_rate(params));

    i2s_value = readl(ptr_add((*priv_).io, i2s_reg as c_ulong)) as c_ulong;
    i2s_value &= !KIRKWOOD_I2S_CTL_SIZE_MASK;

    /*
     * Size settings in play/rec i2s control regs and play/rec control
     * regs must be the same.
     */
    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S16_LE => {
            i2s_value |= KIRKWOOD_I2S_CTL_SIZE_16;
            ctl_play = KIRKWOOD_PLAYCTL_SIZE_16_C | KIRKWOOD_PLAYCTL_I2S_EN | KIRKWOOD_PLAYCTL_SPDIF_EN;
            ctl_rec = KIRKWOOD_RECCTL_SIZE_16_C | KIRKWOOD_RECCTL_I2S_EN | KIRKWOOD_RECCTL_SPDIF_EN;
        }
        /*
         * doesn't work... S20_3LE != kirkwood 20bit format ?
         *
        case SNDRV_PCM_FORMAT_S20_3LE:
            i2s_value |= KIRKWOOD_I2S_CTL_SIZE_20;
            ctl_play = KIRKWOOD_PLAYCTL_SIZE_20 |
                   KIRKWOOD_PLAYCTL_I2S_EN;
            ctl_rec = KIRKWOOD_RECCTL_SIZE_20 |
                  KIRKWOOD_RECCTL_I2S_EN;
            break;
        */
        x if x == SNDRV_PCM_FORMAT_S24_LE => {
            i2s_value |= KIRKWOOD_I2S_CTL_SIZE_24;
            ctl_play = KIRKWOOD_PLAYCTL_SIZE_24 | KIRKWOOD_PLAYCTL_I2S_EN | KIRKWOOD_PLAYCTL_SPDIF_EN;
            ctl_rec = KIRKWOOD_RECCTL_SIZE_24 | KIRKWOOD_RECCTL_I2S_EN | KIRKWOOD_RECCTL_SPDIF_EN;
        }
        x if x == SNDRV_PCM_FORMAT_S32_LE => {
            i2s_value |= KIRKWOOD_I2S_CTL_SIZE_32;
            ctl_play = KIRKWOOD_PLAYCTL_SIZE_32 | KIRKWOOD_PLAYCTL_I2S_EN;
            ctl_rec = KIRKWOOD_RECCTL_SIZE_32 | KIRKWOOD_RECCTL_I2S_EN;
        }
        _ => {
            return -EINVAL;
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if params_channels(params) == 1 {
            ctl_play |= KIRKWOOD_PLAYCTL_MONO_BOTH;
        } else {
            ctl_play |= KIRKWOOD_PLAYCTL_MONO_OFF;
        }

        (*priv_).ctl_play &= !(KIRKWOOD_PLAYCTL_MONO_MASK | KIRKWOOD_PLAYCTL_ENABLE_MASK | KIRKWOOD_PLAYCTL_SIZE_MASK);
        (*priv_).ctl_play |= ctl_play;
    } else {
        (*priv_).ctl_rec &= !(KIRKWOOD_RECCTL_ENABLE_MASK | KIRKWOOD_RECCTL_SIZE_MASK);
        (*priv_).ctl_rec |= ctl_rec;
    }

    writel(i2s_value, ptr_add((*priv_).io, i2s_reg as c_ulong));

    0
}

unsafe fn kirkwood_i2s_play_mute(mut ctl: c_uint) -> c_uint {
    if !(ctl & KIRKWOOD_PLAYCTL_I2S_EN) != 0 {
        ctl |= KIRKWOOD_PLAYCTL_I2S_MUTE;
    }
    if !(ctl & KIRKWOOD_PLAYCTL_SPDIF_EN) != 0 {
        ctl |= KIRKWOOD_PLAYCTL_SPDIF_MUTE;
    }
    ctl
}

unsafe extern "C" fn kirkwood_i2s_play_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let runtime = (*substream).runtime;
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let mut ctl: uint32_t;
    let mut value: uint32_t = 0;

    ctl = readl(ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));
    if (ctl & KIRKWOOD_PLAYCTL_ENABLE_MASK) == 0 {
        let mut timeout: c_uint = 5000;
        /*
         * The Armada510 spec says that if we enter pause mode, the
         * busy bit must be read back as clear _twice_.  Make sure
         * we respect that otherwise we get DMA underruns.
         */
        loop {
            value = ctl;
            ctl = readl(ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));
            if !((ctl | value) & KIRKWOOD_PLAYCTL_PLAY_BUSY) != 0 {
                break;
            }
            udelay(1);
            let old = timeout;
            timeout = timeout.wrapping_sub(1);
            if old == 0 {
                break;
            }
        }

        if ((ctl | value) & KIRKWOOD_PLAYCTL_PLAY_BUSY) != 0 {
            dev_notice(
                (*dai).dev,
                c"timed out waiting for busy to deassert: %08x\n".as_ptr(),
                ctl,
            );
        }
    }

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START => {
            /* configure */
            ctl = (*priv_).ctl_play;
            if (*dai).id == 0 {
                ctl &= !KIRKWOOD_PLAYCTL_SPDIF_EN; /* i2s */
            } else {
                ctl &= !KIRKWOOD_PLAYCTL_I2S_EN; /* spdif */
            }
            ctl = kirkwood_i2s_play_mute(ctl);
            value = ctl & !KIRKWOOD_PLAYCTL_ENABLE_MASK;
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));

            /* enable interrupts */
            if !(*runtime).no_period_wakeup {
                value = readl(ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));
                value |= KIRKWOOD_INT_CAUSE_PLAY_BYTES;
                writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));
            }

            /* enable playback */
            writel(ctl as c_ulong, ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));
        }

        x if x == SNDRV_PCM_TRIGGER_STOP => {
            /* stop audio, disable interrupts */
            ctl |= KIRKWOOD_PLAYCTL_PAUSE | KIRKWOOD_PLAYCTL_I2S_MUTE | KIRKWOOD_PLAYCTL_SPDIF_MUTE;
            writel(ctl as c_ulong, ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));

            value = readl(ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));
            value &= !KIRKWOOD_INT_CAUSE_PLAY_BYTES;
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));

            /* disable all playbacks */
            ctl &= !KIRKWOOD_PLAYCTL_ENABLE_MASK;
            writel(ctl as c_ulong, ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));
        }

        x if x == SNDRV_PCM_TRIGGER_PAUSE_PUSH || x == SNDRV_PCM_TRIGGER_SUSPEND => {
            ctl |= KIRKWOOD_PLAYCTL_PAUSE | KIRKWOOD_PLAYCTL_I2S_MUTE | KIRKWOOD_PLAYCTL_SPDIF_MUTE;
            writel(ctl as c_ulong, ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));
        }

        x if x == SNDRV_PCM_TRIGGER_RESUME || x == SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ctl &= !(KIRKWOOD_PLAYCTL_PAUSE | KIRKWOOD_PLAYCTL_I2S_MUTE | KIRKWOOD_PLAYCTL_SPDIF_MUTE);
            ctl = kirkwood_i2s_play_mute(ctl);
            writel(ctl as c_ulong, ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));
        }

        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn kirkwood_i2s_rec_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let mut ctl: uint32_t;
    let mut value: uint32_t;

    value = readl(ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START => {
            /* configure */
            ctl = (*priv_).ctl_rec;
            if (*dai).id == 0 {
                ctl &= !KIRKWOOD_RECCTL_SPDIF_EN; /* i2s */
            } else {
                ctl &= !KIRKWOOD_RECCTL_I2S_EN; /* spdif */
            }

            value = ctl & !KIRKWOOD_RECCTL_ENABLE_MASK;
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));

            /* enable interrupts */
            value = readl(ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));
            value |= KIRKWOOD_INT_CAUSE_REC_BYTES;
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));

            /* enable record */
            writel(ctl as c_ulong, ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));
        }

        x if x == SNDRV_PCM_TRIGGER_STOP => {
            /* stop audio, disable interrupts */
            value = readl(ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));
            value |= KIRKWOOD_RECCTL_PAUSE | KIRKWOOD_RECCTL_MUTE;
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));

            value = readl(ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));
            value &= !KIRKWOOD_INT_CAUSE_REC_BYTES;
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));

            /* disable all records */
            value = readl(ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));
            value &= !KIRKWOOD_RECCTL_ENABLE_MASK;
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));
        }

        x if x == SNDRV_PCM_TRIGGER_PAUSE_PUSH || x == SNDRV_PCM_TRIGGER_SUSPEND => {
            value = readl(ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));
            value |= KIRKWOOD_RECCTL_PAUSE | KIRKWOOD_RECCTL_MUTE;
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));
        }

        x if x == SNDRV_PCM_TRIGGER_RESUME || x == SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            value = readl(ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));
            value &= !(KIRKWOOD_RECCTL_PAUSE | KIRKWOOD_RECCTL_MUTE);
            writel(value as c_ulong, ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));
        }

        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn kirkwood_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        return kirkwood_i2s_play_trigger(substream, cmd, dai);
    } else {
        return kirkwood_i2s_rec_trigger(substream, cmd, dai);
    }

    #[allow(unreachable_code)]
    0
}

unsafe fn kirkwood_i2s_init(priv_: *mut kirkwood_dma_data) -> c_int {
    let mut value: c_ulong;
    let mut reg_data: c_uint;

    /* put system in a "safe" state : */
    /* disable audio interrupts */
    writel(0xffffffff, ptr_add((*priv_).io, KIRKWOOD_INT_CAUSE as c_ulong));
    writel(0, ptr_add((*priv_).io, KIRKWOOD_INT_MASK as c_ulong));

    reg_data = readl(ptr_add((*priv_).io, 0x1200));
    reg_data &= !0x333FF8;
    reg_data |= 0x111D18;
    writel(reg_data as c_ulong, ptr_add((*priv_).io, 0x1200));

    msleep(500);

    reg_data = readl(ptr_add((*priv_).io, 0x1200));
    reg_data &= !0x333FF8;
    reg_data |= 0x111D18;
    writel(reg_data as c_ulong, ptr_add((*priv_).io, 0x1200));

    /* disable playback/record */
    value = readl(ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong)) as c_ulong;
    value &= !(KIRKWOOD_PLAYCTL_ENABLE_MASK as c_ulong);
    writel(value, ptr_add((*priv_).io, KIRKWOOD_PLAYCTL as c_ulong));

    value = readl(ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong)) as c_ulong;
    value &= !(KIRKWOOD_RECCTL_ENABLE_MASK as c_ulong);
    writel(value, ptr_add((*priv_).io, KIRKWOOD_RECCTL as c_ulong));

    0
}

static kirkwood_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(kirkwood_i2s_startup),
    trigger: Some(kirkwood_i2s_trigger),
    hw_params: Some(kirkwood_i2s_hw_params),
    set_fmt: Some(kirkwood_i2s_set_fmt),
};

static mut kirkwood_i2s_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"i2s".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000,
            rate_min: 0,
            rate_max: 0,
            formats: KIRKWOOD_I2S_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000,
            rate_min: 0,
            rate_max: 0,
            formats: KIRKWOOD_I2S_FORMATS,
        },
        ops: &kirkwood_i2s_dai_ops,
    },
    snd_soc_dai_driver {
        name: c"spdif".as_ptr(),
        id: 1,
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000,
            rate_min: 0,
            rate_max: 0,
            formats: KIRKWOOD_SPDIF_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000,
            rate_min: 0,
            rate_max: 0,
            formats: KIRKWOOD_SPDIF_FORMATS,
        },
        ops: &kirkwood_i2s_dai_ops,
    },
];

static mut kirkwood_i2s_dai_extclk: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"i2s".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 192000,
            formats: KIRKWOOD_I2S_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 192000,
            formats: KIRKWOOD_I2S_FORMATS,
        },
        ops: &kirkwood_i2s_dai_ops,
    },
    snd_soc_dai_driver {
        name: c"spdif".as_ptr(),
        id: 1,
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 192000,
            formats: KIRKWOOD_SPDIF_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 192000,
            formats: KIRKWOOD_SPDIF_FORMATS,
        },
        ops: &kirkwood_i2s_dai_ops,
    },
];

unsafe extern "C" fn kirkwood_i2s_dev_probe(pdev: *mut platform_device) -> c_int {
    let data = (*pdev).dev.platform_data as *mut kirkwood_asoc_platform_data;
    let mut soc_dai = ptr::addr_of_mut!(kirkwood_i2s_dai) as *mut snd_soc_dai_driver;
    let priv_: *mut kirkwood_dma_data;
    let np = (*pdev).dev.of_node;
    let mut err: c_int;

    priv_ = devm_kzalloc(&mut (*pdev).dev, size_of::<kirkwood_dma_data>(), GFP_KERNEL) as *mut kirkwood_dma_data;
    if priv_.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut c_void);

    if of_device_is_compatible(np, c"marvell,armada-380-audio".as_ptr()) {
        (*priv_).io = devm_platform_ioremap_resource_byname(pdev, c"i2s_regs".as_ptr());
    } else {
        (*priv_).io = devm_platform_ioremap_resource(pdev, 0);
    }
    if IS_ERR((*priv_).io) {
        return PTR_ERR((*priv_).io);
    }

    (*priv_).irq = platform_get_irq(pdev, 0);
    if (*priv_).irq < 0 {
        return (*priv_).irq;
    }

    if of_device_is_compatible(np, c"marvell,armada-380-audio".as_ptr()) {
        err = armada_38x_i2s_init_quirk(pdev, priv_, soc_dai);
        if err < 0 {
            return err;
        }
        /* Set initial pll frequency */
        armada_38x_set_pll((*priv_).pll_config, 44100);
    }

    if !np.is_null() {
        (*priv_).burst = 128; /* might be 32 or 128 */
    } else if !data.is_null() {
        (*priv_).burst = (*data).burst;
    } else {
        dev_err(&mut (*pdev).dev, c"no DT nor platform data ?!\n".as_ptr());
        return -EINVAL;
    }

    (*priv_).clk = devm_clk_get(
        &mut (*pdev).dev,
        if !np.is_null() { c"internal".as_ptr() } else { ptr::null() },
    );
    if IS_ERR((*priv_).clk as *const c_void) {
        dev_err(&mut (*pdev).dev, c"no clock\n".as_ptr());
        return PTR_ERR((*priv_).clk as *const c_void);
    }

    (*priv_).extclk = devm_clk_get(&mut (*pdev).dev, c"extclk".as_ptr());
    if IS_ERR((*priv_).extclk as *const c_void) {
        if PTR_ERR((*priv_).extclk as *const c_void) == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }
    } else {
        if clk_is_match((*priv_).extclk, (*priv_).clk) {
            devm_clk_put(&mut (*pdev).dev, (*priv_).extclk);
            (*priv_).extclk = ERR_PTR(-EINVAL);
        } else {
            dev_info(&mut (*pdev).dev, c"found external clock\n".as_ptr());
            clk_prepare_enable((*priv_).extclk);
            soc_dai = ptr::addr_of_mut!(kirkwood_i2s_dai_extclk) as *mut snd_soc_dai_driver;
        }
    }

    err = clk_prepare_enable((*priv_).clk);
    if err < 0 {
        return err;
    }

    /* Some sensible defaults - this reflects the powerup values */
    (*priv_).ctl_play = KIRKWOOD_PLAYCTL_SIZE_24;
    (*priv_).ctl_rec = KIRKWOOD_RECCTL_SIZE_24;

    /* Select the burst size */
    if (*priv_).burst == 32 {
        (*priv_).ctl_play |= KIRKWOOD_PLAYCTL_BURST_32;
        (*priv_).ctl_rec |= KIRKWOOD_RECCTL_BURST_32;
    } else {
        (*priv_).ctl_play |= KIRKWOOD_PLAYCTL_BURST_128;
        (*priv_).ctl_rec |= KIRKWOOD_RECCTL_BURST_128;
    }

    err = snd_soc_register_component(&mut (*pdev).dev, &kirkwood_soc_component, soc_dai, 2);
    if err != 0 {
        dev_err(&mut (*pdev).dev, c"snd_soc_register_component failed\n".as_ptr());
        if !IS_ERR((*priv_).extclk as *const c_void) {
            clk_disable_unprepare((*priv_).extclk);
        }
        clk_disable_unprepare((*priv_).clk);

        return err;
    }

    kirkwood_i2s_init(priv_);

    0
}

unsafe extern "C" fn kirkwood_i2s_dev_remove(pdev: *mut platform_device) {
    let priv_ = dev_get_drvdata(&mut (*pdev).dev) as *mut kirkwood_dma_data;

    snd_soc_unregister_component(&mut (*pdev).dev);
    if !IS_ERR((*priv_).extclk as *const c_void) {
        clk_disable_unprepare((*priv_).extclk);
    }
    clk_disable_unprepare((*priv_).clk);
}

// #ifdef CONFIG_OF
static mvebu_audio_of_match: [of_device_id; 5] = [
    of_device_id {
        compatible: c"marvell,kirkwood-audio".as_ptr(),
    },
    of_device_id {
        compatible: c"marvell,dove-audio".as_ptr(),
    },
    of_device_id {
        compatible: c"marvell,armada370-audio".as_ptr(),
    },
    of_device_id {
        compatible: c"marvell,armada-380-audio".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, mvebu_audio_of_match);
// #endif

static mut kirkwood_i2s_driver: platform_driver = platform_driver {
    probe: Some(kirkwood_i2s_dev_probe),
    remove: Some(kirkwood_i2s_dev_remove),
    driver: platform_driver_inner {
        name: unsafe { &DRV_NAME as *const c_char },
        of_match_table: unsafe { of_match_ptr(mvebu_audio_of_match.as_ptr()) },
    },
};

// module_platform_driver(kirkwood_i2s_driver);

/* Module information */
// MODULE_AUTHOR("Arnaud Patard, <arnaud.patard@rtp-net.org>");
// MODULE_DESCRIPTION("Kirkwood I2S SoC Interface");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:mvebu-audio");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
