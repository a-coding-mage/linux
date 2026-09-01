// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2013, Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const AXI_I2S_REG_RESET: c_uint = 0x00;
const AXI_I2S_REG_CTRL: c_uint = 0x04;
const AXI_I2S_REG_CLK_CTRL: c_uint = 0x08;
const AXI_I2S_REG_STATUS: c_uint = 0x10;

const AXI_I2S_REG_RX_FIFO: c_ulong = 0x28;
const AXI_I2S_REG_TX_FIFO: c_ulong = 0x2C;

const AXI_I2S_RESET_GLOBAL: c_uint = BIT(0);
const AXI_I2S_RESET_TX_FIFO: c_uint = BIT(1);
const AXI_I2S_RESET_RX_FIFO: c_uint = BIT(2);

const AXI_I2S_CTRL_TX_EN: c_uint = BIT(0);
const AXI_I2S_CTRL_RX_EN: c_uint = BIT(1);

/* The frame size is configurable, but for now we always set it 64 bit */
const AXI_I2S_BITS_PER_FRAME: c_ulong = 64;

#[repr(C)]
pub struct axi_i2s {
    regmap: *mut regmap,
    clk: *mut clk,
    clk_ref: *mut clk,

    has_capture: bool,
    has_playback: bool,

    dai_driver: snd_soc_dai_driver,

    capture_dma_data: snd_dmaengine_dai_dma_data,
    playback_dma_data: snd_dmaengine_dai_dma_data,

    ratnum: snd_ratnum,
    rate_constraints: snd_pcm_hw_constraint_ratnums,
}

unsafe extern "C" fn axi_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s: *mut axi_i2s = snd_soc_dai_get_drvdata(dai) as *mut axi_i2s;
    let mask: c_uint;
    let val: c_uint;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        mask = AXI_I2S_CTRL_RX_EN;
    } else {
        mask = AXI_I2S_CTRL_TX_EN;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            val = mask;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            val = 0;
        }
        _ => {
            return -EINVAL;
        }
    }

    regmap_update_bits((*i2s).regmap, AXI_I2S_REG_CTRL, mask, val);

    0
}

unsafe extern "C" fn axi_i2s_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s: *mut axi_i2s = snd_soc_dai_get_drvdata(dai) as *mut axi_i2s;
    let bclk_div: c_uint;
    let word_size: c_uint;
    let bclk_rate: c_uint;

    bclk_rate = params_rate(params).wrapping_mul(AXI_I2S_BITS_PER_FRAME as c_uint);

    word_size = (AXI_I2S_BITS_PER_FRAME / 2 - 1) as c_uint;
    bclk_div = (DIV_ROUND_UP(clk_get_rate((*i2s).clk_ref), bclk_rate as c_ulong) / 2 - 1) as c_uint;

    regmap_write(
        (*i2s).regmap,
        AXI_I2S_REG_CLK_CTRL,
        (word_size << 16) | bclk_div,
    );

    0
}

unsafe extern "C" fn axi_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s: *mut axi_i2s = snd_soc_dai_get_drvdata(dai) as *mut axi_i2s;
    let mask: u32;
    let ret: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        mask = AXI_I2S_RESET_RX_FIFO;
    } else {
        mask = AXI_I2S_RESET_TX_FIFO;
    }

    regmap_write((*i2s).regmap, AXI_I2S_REG_RESET, mask);

    ret = snd_pcm_hw_constraint_ratnums(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &mut (*i2s).rate_constraints,
    );
    if ret != 0 {
        return ret;
    }

    clk_prepare_enable((*i2s).clk_ref)
}

unsafe extern "C" fn axi_i2s_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let i2s: *mut axi_i2s = snd_soc_dai_get_drvdata(dai) as *mut axi_i2s;

    clk_disable_unprepare((*i2s).clk_ref);
}

unsafe extern "C" fn axi_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s: *mut axi_i2s = snd_soc_dai_get_drvdata(dai) as *mut axi_i2s;

    snd_soc_dai_init_dma_data(
        dai,
        if (*i2s).has_playback {
            &mut (*i2s).playback_dma_data
        } else {
            ptr::null_mut()
        },
        if (*i2s).has_capture {
            &mut (*i2s).capture_dma_data
        } else {
            ptr::null_mut()
        },
    );

    0
}

static axi_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(axi_i2s_dai_probe),
    startup: Some(axi_i2s_startup),
    shutdown: Some(axi_i2s_shutdown),
    trigger: Some(axi_i2s_trigger),
    hw_params: Some(axi_i2s_hw_params),
};

static mut axi_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    ops: &axi_i2s_dai_ops,
    symmetric_rate: 1,
    ..unsafe { core::mem::zeroed() }
};

static axi_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"axi-i2s".as_ptr(),
    legacy_dai_naming: 1,
};

static axi_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: AXI_I2S_REG_STATUS,
};

unsafe fn axi_i2s_parse_of(i2s: *mut axi_i2s, np: *const device_node) {
    let mut dma_names: *mut property = ptr::null_mut();
    let mut dma_name: *const c_char = ptr::null();

    /*
     * C source used of_property_for_each_string(np, "dma-names",
     * dma_names, dma_name).
     */
    while of_property_for_each_string_next(
        np,
        c"dma-names".as_ptr(),
        &mut dma_names,
        &mut dma_name,
    ) != 0
    {
        if strcmp(dma_name, c"rx".as_ptr()) == 0 {
            (*i2s).has_capture = true;
        }
        if strcmp(dma_name, c"tx".as_ptr()) == 0 {
            (*i2s).has_playback = true;
        }
    }
}

unsafe extern "C" fn axi_i2s_probe(pdev: *mut platform_device) -> c_int {
    let mut res: *mut resource = ptr::null_mut();
    let i2s: *mut axi_i2s;
    let base: *mut c_void;
    let mut ret: c_int;

    i2s = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<axi_i2s>(),
        GFP_KERNEL,
    ) as *mut axi_i2s;
    if i2s.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, i2s as *mut c_void);

    axi_i2s_parse_of(i2s, (*pdev).dev.of_node);

    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    (*i2s).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, &axi_i2s_regmap_config);
    if IS_ERR((*i2s).regmap as *const c_void) {
        return PTR_ERR((*i2s).regmap as *const c_void);
    }

    (*i2s).clk = devm_clk_get(&mut (*pdev).dev, c"axi".as_ptr());
    if IS_ERR((*i2s).clk as *const c_void) {
        return PTR_ERR((*i2s).clk as *const c_void);
    }

    (*i2s).clk_ref = devm_clk_get(&mut (*pdev).dev, c"ref".as_ptr());
    if IS_ERR((*i2s).clk_ref as *const c_void) {
        return PTR_ERR((*i2s).clk_ref as *const c_void);
    }

    ret = clk_prepare_enable((*i2s).clk);
    if ret != 0 {
        return ret;
    }

    if (*i2s).has_playback {
        axi_i2s_dai.playback.channels_min = 2;
        axi_i2s_dai.playback.channels_max = 2;
        axi_i2s_dai.playback.rates = SNDRV_PCM_RATE_KNOT;
        axi_i2s_dai.playback.formats = SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_U32_LE;

        (*i2s).playback_dma_data.addr = (*res).start + AXI_I2S_REG_TX_FIFO;
        (*i2s).playback_dma_data.addr_width = 4;
        (*i2s).playback_dma_data.maxburst = 1;
    }

    if (*i2s).has_capture {
        axi_i2s_dai.capture.channels_min = 2;
        axi_i2s_dai.capture.channels_max = 2;
        axi_i2s_dai.capture.rates = SNDRV_PCM_RATE_KNOT;
        axi_i2s_dai.capture.formats = SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_U32_LE;

        (*i2s).capture_dma_data.addr = (*res).start + AXI_I2S_REG_RX_FIFO;
        (*i2s).capture_dma_data.addr_width = 4;
        (*i2s).capture_dma_data.maxburst = 1;
    }

    (*i2s).ratnum.num = clk_get_rate((*i2s).clk_ref) / 2 / AXI_I2S_BITS_PER_FRAME;
    (*i2s).ratnum.den_step = 1;
    (*i2s).ratnum.den_min = 1;
    (*i2s).ratnum.den_max = 64;

    (*i2s).rate_constraints.rats = &mut (*i2s).ratnum;
    (*i2s).rate_constraints.nrats = 1;

    regmap_write((*i2s).regmap, AXI_I2S_REG_RESET, AXI_I2S_RESET_GLOBAL);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &axi_i2s_component,
        &raw mut axi_i2s_dai,
        1,
    );
    if ret != 0 {
        clk_disable_unprepare((*i2s).clk);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if ret != 0 {
        clk_disable_unprepare((*i2s).clk);
        return ret;
    }

    dev_info(
        &mut (*pdev).dev,
        c"probed, capture %s, playback %s\n".as_ptr(),
        str_enabled_disabled((*i2s).has_capture),
        str_enabled_disabled((*i2s).has_playback),
    );

    0
}

unsafe extern "C" fn axi_i2s_dev_remove(pdev: *mut platform_device) {
    let i2s: *mut axi_i2s = platform_get_drvdata(pdev) as *mut axi_i2s;

    clk_disable_unprepare((*i2s).clk);
}

static axi_i2s_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"adi,axi-i2s-1.00.a".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, axi_i2s_of_match); */

static mut axi_i2s_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"axi-i2s".as_ptr(),
        of_match_table: axi_i2s_of_match.as_ptr(),
    },
    probe: Some(axi_i2s_probe),
    remove: Some(axi_i2s_dev_remove),
};
/* module_platform_driver(axi_i2s_driver); */

/* MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>"); */
/* MODULE_DESCRIPTION("AXI I2S driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
