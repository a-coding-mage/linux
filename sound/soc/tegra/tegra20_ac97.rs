// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra20_ac97.c - Tegra20 AC97 platform driver
 *
 * Copyright (c) 2012 Lucas Stach <dev@lynxeye.de>
 *
 * Partly based on code copyright/by:
 *
 * Copyright (c) 2011,2012 Toradex Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const DRV_NAME: &[u8] = b"tegra20-ac97\0";

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: c_ulong,
    pub addr_width: c_int,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct tegra20_ac97 {
    pub reset: *mut reset_control,
    pub clk_ac97: *mut clk,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub sync_gpio: *mut gpio_desc,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> c_ushort>,
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>,
    pub reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub warm_reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}

pub type c_ushort = u16;
pub type bool_ = bool;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
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
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_int,
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
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    static mut jiffies: c_ulong;

    static TEGRA20_AC97_STATUS1: c_uint;
    static TEGRA20_AC97_STATUS1_CODEC1_RDY: u32;
    static TEGRA20_AC97_CMD: c_uint;
    static TEGRA20_AC97_CMD_CMD_ADDR_SHIFT: u32;
    static TEGRA20_AC97_CMD_CMD_ADDR_MASK: u32;
    static TEGRA20_AC97_CMD_BUSY: u32;
    static TEGRA20_AC97_STATUS1_STA_VALID1: u32;
    static TEGRA20_AC97_STATUS1_STA_DATA1_MASK: u32;
    static TEGRA20_AC97_STATUS1_STA_DATA1_SHIFT: u32;
    static TEGRA20_AC97_CMD_CMD_DATA_SHIFT: u32;
    static TEGRA20_AC97_CMD_CMD_DATA_MASK: u32;
    static TEGRA20_AC97_FIFO1_SCR: c_uint;
    static TEGRA20_AC97_FIFO_SCR_PB_QRT_MT_EN: u32;
    static TEGRA20_AC97_CTRL: c_uint;
    static TEGRA20_AC97_CTRL_PCM_DAC_EN: u32;
    static TEGRA20_AC97_CTRL_STM_EN: u32;
    static TEGRA20_AC97_FIFO_SCR_REC_FULL_EN: u32;
    static TEGRA20_AC97_FIFO_TX1: c_uint;
    static TEGRA20_AC97_FIFO_RX1: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static REGCACHE_FLAT: c_int;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_int;
    static DMA_SLAVE_BUSWIDTH_4_BYTES: c_int;

    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn gpiod_direction_output(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn udelay(usecs: c_ulong);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn time_after(a: c_ulong, b: c_ulong) -> bool_;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: u32, val: u32) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char)
        -> *mut reset_control;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_set_ac97_ops(ops: *mut snd_ac97_bus_ops) -> c_int;
    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn tegra_pcm_platform_register(dev: *mut device) -> c_int;
    fn tegra_pcm_platform_unregister(dev: *mut device);
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

static mut workdata: *mut tegra20_ac97 = core::ptr::null_mut();

unsafe extern "C" fn tegra20_ac97_codec_reset(_ac97: *mut snd_ac97) {
    let mut readback: u32 = 0;
    let timeout: c_ulong;

    /*
     * The reset line is not driven by DAC pad group, have to toggle GPIO.
     * The RESET line is active low but this is abstracted by the GPIO
     * library.
     */
    unsafe {
        gpiod_set_value((*workdata).reset_gpio, 1);
        udelay(2);

        gpiod_set_value((*workdata).reset_gpio, 0);
        udelay(2);

        timeout = jiffies.wrapping_add(msecs_to_jiffies(100));

        loop {
            regmap_read((*workdata).regmap, TEGRA20_AC97_STATUS1, &mut readback);
            if readback & TEGRA20_AC97_STATUS1_CODEC1_RDY != 0 {
                break;
            }
            usleep_range(1000, 2000);
            if time_after(jiffies, timeout) {
                break;
            }
        }
    }
}

unsafe extern "C" fn tegra20_ac97_codec_warm_reset(_ac97: *mut snd_ac97) {
    let mut readback: u32 = 0;
    let timeout: c_ulong;

    /*
     * although sync line is driven by the DAC pad group warm reset using
     * the controller cmd is not working, have to toggle sync line
     * manually.
     */
    unsafe {
        gpiod_direction_output((*workdata).sync_gpio, 1);
        udelay(2);
        gpiod_set_value((*workdata).sync_gpio, 0);
        udelay(2);

        timeout = jiffies.wrapping_add(msecs_to_jiffies(100));

        loop {
            regmap_read((*workdata).regmap, TEGRA20_AC97_STATUS1, &mut readback);
            if readback & TEGRA20_AC97_STATUS1_CODEC1_RDY != 0 {
                break;
            }
            usleep_range(1000, 2000);
            if time_after(jiffies, timeout) {
                break;
            }
        }
    }
}

unsafe extern "C" fn tegra20_ac97_codec_read(
    _ac97_snd: *mut snd_ac97,
    reg: c_ushort,
) -> c_ushort {
    let mut readback: u32 = 0;
    let timeout: c_ulong;

    unsafe {
        regmap_write(
            (*workdata).regmap,
            TEGRA20_AC97_CMD,
            ((((reg as u32) | 0x80) << TEGRA20_AC97_CMD_CMD_ADDR_SHIFT)
                & TEGRA20_AC97_CMD_CMD_ADDR_MASK)
                | TEGRA20_AC97_CMD_BUSY,
        );

        timeout = jiffies.wrapping_add(msecs_to_jiffies(100));

        loop {
            regmap_read((*workdata).regmap, TEGRA20_AC97_STATUS1, &mut readback);
            if readback & TEGRA20_AC97_STATUS1_STA_VALID1 != 0 {
                break;
            }
            usleep_range(1000, 2000);
            if time_after(jiffies, timeout) {
                break;
            }
        }

        ((readback & TEGRA20_AC97_STATUS1_STA_DATA1_MASK)
            >> TEGRA20_AC97_STATUS1_STA_DATA1_SHIFT) as c_ushort
    }
}

unsafe extern "C" fn tegra20_ac97_codec_write(
    _ac97_snd: *mut snd_ac97,
    reg: c_ushort,
    val: c_ushort,
) {
    let mut readback: u32 = 0;
    let timeout: c_ulong;

    unsafe {
        regmap_write(
            (*workdata).regmap,
            TEGRA20_AC97_CMD,
            (((reg as u32) << TEGRA20_AC97_CMD_CMD_ADDR_SHIFT) & TEGRA20_AC97_CMD_CMD_ADDR_MASK)
                | (((val as u32) << TEGRA20_AC97_CMD_CMD_DATA_SHIFT)
                    & TEGRA20_AC97_CMD_CMD_DATA_MASK)
                | TEGRA20_AC97_CMD_BUSY,
        );

        timeout = jiffies.wrapping_add(msecs_to_jiffies(100));

        loop {
            regmap_read((*workdata).regmap, TEGRA20_AC97_CMD, &mut readback);
            if readback & TEGRA20_AC97_CMD_BUSY == 0 {
                break;
            }
            usleep_range(1000, 2000);
            if time_after(jiffies, timeout) {
                break;
            }
        }
    }
}

static mut tegra20_ac97_ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
    read: Some(tegra20_ac97_codec_read),
    write: Some(tegra20_ac97_codec_write),
    reset: Some(tegra20_ac97_codec_reset),
    warm_reset: Some(tegra20_ac97_codec_warm_reset),
};

unsafe fn tegra20_ac97_start_playback(ac97: *mut tegra20_ac97) {
    unsafe {
        regmap_update_bits(
            (*ac97).regmap,
            TEGRA20_AC97_FIFO1_SCR,
            TEGRA20_AC97_FIFO_SCR_PB_QRT_MT_EN,
            TEGRA20_AC97_FIFO_SCR_PB_QRT_MT_EN,
        );

        regmap_update_bits(
            (*ac97).regmap,
            TEGRA20_AC97_CTRL,
            TEGRA20_AC97_CTRL_PCM_DAC_EN | TEGRA20_AC97_CTRL_STM_EN,
            TEGRA20_AC97_CTRL_PCM_DAC_EN | TEGRA20_AC97_CTRL_STM_EN,
        );
    }
}

unsafe fn tegra20_ac97_stop_playback(ac97: *mut tegra20_ac97) {
    unsafe {
        regmap_update_bits(
            (*ac97).regmap,
            TEGRA20_AC97_FIFO1_SCR,
            TEGRA20_AC97_FIFO_SCR_PB_QRT_MT_EN,
            0,
        );

        regmap_update_bits(
            (*ac97).regmap,
            TEGRA20_AC97_CTRL,
            TEGRA20_AC97_CTRL_PCM_DAC_EN,
            0,
        );
    }
}

unsafe fn tegra20_ac97_start_capture(ac97: *mut tegra20_ac97) {
    unsafe {
        regmap_update_bits(
            (*ac97).regmap,
            TEGRA20_AC97_FIFO1_SCR,
            TEGRA20_AC97_FIFO_SCR_REC_FULL_EN,
            TEGRA20_AC97_FIFO_SCR_REC_FULL_EN,
        );
    }
}

unsafe fn tegra20_ac97_stop_capture(ac97: *mut tegra20_ac97) {
    unsafe {
        regmap_update_bits(
            (*ac97).regmap,
            TEGRA20_AC97_FIFO1_SCR,
            TEGRA20_AC97_FIFO_SCR_REC_FULL_EN,
            0,
        );
    }
}

unsafe extern "C" fn tegra20_ac97_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ac97: *mut tegra20_ac97 = unsafe { snd_soc_dai_get_drvdata(dai) as *mut tegra20_ac97 };

    unsafe {
        if cmd == SNDRV_PCM_TRIGGER_START
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
            || cmd == SNDRV_PCM_TRIGGER_RESUME
        {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                tegra20_ac97_start_playback(ac97);
            } else {
                tegra20_ac97_start_capture(ac97);
            }
        } else if cmd == SNDRV_PCM_TRIGGER_STOP
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
            || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                tegra20_ac97_stop_playback(ac97);
            } else {
                tegra20_ac97_stop_capture(ac97);
            }
        } else {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn tegra20_ac97_probe(dai: *mut snd_soc_dai) -> c_int {
    let ac97: *mut tegra20_ac97 = unsafe { snd_soc_dai_get_drvdata(dai) as *mut tegra20_ac97 };

    unsafe {
        snd_soc_dai_init_dma_data(
            dai,
            &mut (*ac97).playback_dma_data,
            &mut (*ac97).capture_dma_data,
        );
    }

    0
}

static tegra20_ac97_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(tegra20_ac97_probe),
    trigger: Some(tegra20_ac97_trigger),
};

static mut tegra20_ac97_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"tegra-ac97-pcm\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"PCM Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"PCM Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    ops: &tegra20_ac97_dai_ops,
};

static tegra20_ac97_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn tegra20_ac97_wr_rd_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    unsafe {
        if reg == TEGRA20_AC97_CTRL
            || reg == TEGRA20_AC97_CMD
            || reg == TEGRA20_AC97_STATUS1
            || reg == TEGRA20_AC97_FIFO1_SCR
            || reg == TEGRA20_AC97_FIFO_TX1
            || reg == TEGRA20_AC97_FIFO_RX1
        {
            return true;
        }
    }

    false
}

unsafe extern "C" fn tegra20_ac97_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    unsafe {
        if reg == TEGRA20_AC97_STATUS1
            || reg == TEGRA20_AC97_FIFO1_SCR
            || reg == TEGRA20_AC97_FIFO_TX1
            || reg == TEGRA20_AC97_FIFO_RX1
        {
            return true;
        }
    }

    false
}

unsafe extern "C" fn tegra20_ac97_precious_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    unsafe {
        if reg == TEGRA20_AC97_FIFO_TX1 || reg == TEGRA20_AC97_FIFO_RX1 {
            return true;
        }
    }

    false
}

static tegra20_ac97_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: unsafe { TEGRA20_AC97_FIFO_RX1 },
    writeable_reg: Some(tegra20_ac97_wr_rd_reg),
    readable_reg: Some(tegra20_ac97_wr_rd_reg),
    volatile_reg: Some(tegra20_ac97_volatile_reg),
    precious_reg: Some(tegra20_ac97_precious_reg),
    cache_type: unsafe { REGCACHE_FLAT },
};

unsafe extern "C" fn tegra20_ac97_platform_probe(pdev: *mut platform_device) -> c_int {
    let ac97: *mut tegra20_ac97;
    let mut mem: *mut resource = core::ptr::null_mut();
    let regs: *mut c_void;
    let mut ret: c_int = 0;

    unsafe {
        ac97 = devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<tegra20_ac97>(),
            GFP_KERNEL,
        ) as *mut tegra20_ac97;
        if ac97.is_null() {
            ret = -ENOMEM;
            goto_err(ret);
            return ret;
        }
        dev_set_drvdata(&mut (*pdev).dev, ac97 as *mut c_void);

        (*ac97).reset =
            devm_reset_control_get_exclusive(&mut (*pdev).dev, b"ac97\0".as_ptr() as *const c_char);
        if IS_ERR((*ac97).reset as *const c_void) {
            dev_err(
                &mut (*pdev).dev,
                b"Can't retrieve ac97 reset\n\0".as_ptr() as *const c_char,
            );
            ret = PTR_ERR((*ac97).reset as *const c_void);
            goto_err(ret);
            return ret;
        }

        (*ac97).clk_ac97 = devm_clk_get(&mut (*pdev).dev, core::ptr::null());
        if IS_ERR((*ac97).clk_ac97 as *const c_void) {
            dev_err(
                &mut (*pdev).dev,
                b"Can't retrieve ac97 clock\n\0".as_ptr() as *const c_char,
            );
            ret = PTR_ERR((*ac97).clk_ac97 as *const c_void);
            goto_err(ret);
            return ret;
        }

        regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem);
        if IS_ERR(regs as *const c_void) {
            ret = PTR_ERR(regs as *const c_void);
            goto_err_clk_put(ret);
            return ret;
        }

        (*ac97).regmap =
            devm_regmap_init_mmio(&mut (*pdev).dev, regs, &tegra20_ac97_regmap_config);
        if IS_ERR((*ac97).regmap as *const c_void) {
            dev_err(
                &mut (*pdev).dev,
                b"regmap init failed\n\0".as_ptr() as *const c_char,
            );
            ret = PTR_ERR((*ac97).regmap as *const c_void);
            goto_err_clk_put(ret);
            return ret;
        }

        /* Obtain RESET de-asserted */
        (*ac97).reset_gpio = devm_gpiod_get(
            &mut (*pdev).dev,
            b"nvidia,codec-reset\0".as_ptr() as *const c_char,
            GPIOD_OUT_LOW,
        );
        if IS_ERR((*ac97).reset_gpio as *const c_void) {
            ret = PTR_ERR((*ac97).reset_gpio as *const c_void);
            dev_err(
                &mut (*pdev).dev,
                b"no RESET GPIO supplied: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_err_clk_put(ret);
            return ret;
        }
        gpiod_set_consumer_name(
            (*ac97).reset_gpio,
            b"codec-reset\0".as_ptr() as *const c_char,
        );

        (*ac97).sync_gpio = devm_gpiod_get(
            &mut (*pdev).dev,
            b"nvidia,codec-sync\0".as_ptr() as *const c_char,
            GPIOD_OUT_LOW,
        );
        if IS_ERR((*ac97).sync_gpio as *const c_void) {
            ret = PTR_ERR((*ac97).sync_gpio as *const c_void);
            dev_err(
                &mut (*pdev).dev,
                b"no codec-sync GPIO supplied: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_err_clk_put(ret);
            return ret;
        }
        gpiod_set_consumer_name((*ac97).sync_gpio, b"codec-sync\0".as_ptr() as *const c_char);

        (*ac97).capture_dma_data.addr = (*mem).start.wrapping_add(TEGRA20_AC97_FIFO_RX1 as c_ulong);
        (*ac97).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*ac97).capture_dma_data.maxburst = 4;

        (*ac97).playback_dma_data.addr = (*mem).start.wrapping_add(TEGRA20_AC97_FIFO_TX1 as c_ulong);
        (*ac97).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*ac97).playback_dma_data.maxburst = 4;

        ret = reset_control_assert((*ac97).reset);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"Failed to assert AC'97 reset: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_err_clk_put(ret);
            return ret;
        }

        ret = clk_prepare_enable((*ac97).clk_ac97);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"clk_enable failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_err_clk_put(ret);
            return ret;
        }

        usleep_range(10, 100);

        ret = reset_control_deassert((*ac97).reset);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"Failed to deassert AC'97 reset: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_err_clk_disable_unprepare(ac97, ret);
            return ret;
        }

        ret = snd_soc_set_ac97_ops(&mut tegra20_ac97_ops);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"Failed to set AC'97 ops: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_err_clk_disable_unprepare(ac97, ret);
            return ret;
        }

        ret = snd_soc_register_component(
            &mut (*pdev).dev,
            &tegra20_ac97_component,
            &mut tegra20_ac97_dai,
            1,
        );
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"Could not register DAI: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            ret = -ENOMEM;
            goto_err_clk_disable_unprepare(ac97, ret);
            return ret;
        }

        ret = tegra_pcm_platform_register(&mut (*pdev).dev);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"Could not register PCM: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            snd_soc_unregister_component(&mut (*pdev).dev);
            goto_err_clk_disable_unprepare(ac97, ret);
            return ret;
        }

        /* XXX: crufty ASoC AC97 API - only one AC97 codec allowed */
        workdata = ac97;
    }

    return 0;

    unsafe fn goto_err_clk_disable_unprepare(ac97: *mut tegra20_ac97, ret: c_int) {
        clk_disable_unprepare((*ac97).clk_ac97);
        goto_err_clk_put(ret);
    }

    unsafe fn goto_err_clk_put(ret: c_int) {
        goto_err(ret);
    }

    unsafe fn goto_err(ret: c_int) {
        let _ = ret;
        snd_soc_set_ac97_ops(core::ptr::null_mut());
    }
}

unsafe extern "C" fn tegra20_ac97_platform_remove(pdev: *mut platform_device) {
    let ac97: *mut tegra20_ac97 = unsafe { dev_get_drvdata(&mut (*pdev).dev) as *mut tegra20_ac97 };

    unsafe {
        tegra_pcm_platform_unregister(&mut (*pdev).dev);
        snd_soc_unregister_component(&mut (*pdev).dev);

        clk_disable_unprepare((*ac97).clk_ac97);

        snd_soc_set_ac97_ops(core::ptr::null_mut());
    }
}

static tegra20_ac97_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"nvidia,tegra20-ac97\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, tegra20_ac97_of_match); */

static mut tegra20_ac97_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: DRV_NAME.as_ptr() as *const c_char,
        of_match_table: tegra20_ac97_of_match.as_ptr(),
    },
    probe: Some(tegra20_ac97_platform_probe),
    remove: Some(tegra20_ac97_platform_remove),
};
/* module_platform_driver(tegra20_ac97_driver); */

/* MODULE_AUTHOR("Lucas Stach"); */
/* MODULE_DESCRIPTION("Tegra20 AC97 ASoC driver"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:" DRV_NAME); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
