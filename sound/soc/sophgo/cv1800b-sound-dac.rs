// SPDX-License-Identifier: GPL-2.0
/*
 * Internal DAC codec for cv1800b based CPUs
 */

// Rust translation of Linux kernel C source. External kernel, ASoC, platform,
// MMIO, bitfield, and module symbols are expected to be supplied by bindings.

const CV1800B_TXDAC_CTRL0: usize = 0x00;
const CV1800B_TXDAC_CTRL1: usize = 0x04;
const CV1800B_TXDAC_STATUS: usize = 0x08;
const CV1800B_TXDAC_AFE0: usize = 0x0c;
const CV1800B_TXDAC_AFE1: usize = 0x10;
const CV1800B_TXDAC_ANA0: usize = 0x20;
const CV1800B_TXDAC_ANA1: usize = 0x24;
const CV1800B_TXDAC_ANA2: usize = 0x28;

const fn genmask(high: u32, low: u32) -> u32 {
    if high == 31 && low == 0 {
        u32::MAX
    } else {
        (((1u64 << (high - low + 1)) - 1) << low) as u32
    }
}

/* cv1800b_TXDAC_CTRL0 */
const REG_TXDAC_EN: u32 = genmask(0, 0);
const REG_I2S_RX_EN: u32 = genmask(1, 1);

/* cv1800b_TXDAC_CTRL1 */
const REG_TXDAC_CIC_OPT: u32 = genmask(1, 0);

/* cv1800b_TXDAC_AFE0 */
const REG_TXDAC_INIT_DLY_CNT: u32 = genmask(5, 0);

/* cv1800b_TXDAC_ANA2 */
const TXDAC_OW_VAL_L_MASK: u32 = genmask(7, 0);
const TXDAC_OW_VAL_R_MASK: u32 = genmask(15, 8);
const TXDAC_OW_EN_L_MASK: u32 = genmask(16, 16);
const TXDAC_OW_EN_R_MASK: u32 = genmask(17, 17);

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const GFP_KERNEL: gfp_t = 0;

type u8_t = u8;
type u32_t = u32;
type gfp_t = u32;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
        ) -> i32,
    >,
    trigger: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cmd: i32,
            dai: *mut snd_soc_dai,
        ) -> i32,
    >,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const i8,
    channels_min: u32,
    channels_max: u32,
    rates: u32,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const i8,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const i8,
}

#[repr(C)]
struct of_device_id {
    compatible: *const i8,
}

#[repr(C)]
struct driver {
    name: *const i8,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> i32>,
    driver: driver,
}

#[repr(C)]
struct cv1800b_priv {
    regs: *mut core::ffi::c_void,
    dev: *mut device,
}

#[repr(i32)]
enum decimation_values {
    DECIMATION_64 = 0,
    DECIMATION_128,
    DECIMATION_256,
    DECIMATION_512,
}

extern "C" {
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_TRIGGER_START: i32;
    static SNDRV_PCM_TRIGGER_RESUME: i32;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32;
    static SNDRV_PCM_TRIGGER_STOP: i32;
    static SNDRV_PCM_TRIGGER_SUSPEND: i32;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32;

    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn u32_replace_bits(old: u32, val: u32, mask: u32) -> u32;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut core::ffi::c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: i32,
    ) -> i32;
}

unsafe fn cv1800b_dac_enable(priv_: *mut cv1800b_priv, enable: bool) {
    let mut val: u32_t;

    val = readl((*priv_).regs.add(CV1800B_TXDAC_CTRL0));
    val = u32_replace_bits(val, enable as u32, REG_TXDAC_EN);
    val = u32_replace_bits(val, enable as u32, REG_I2S_RX_EN);
    writel(val, (*priv_).regs.add(CV1800B_TXDAC_CTRL0));
}

/*
 * Control the DAC overwrite bits. When enabled, the DAC outputs the fixed
 * overwrite value instead of samples from the I2S input.
 */
unsafe fn cv1800b_dac_mute(priv_: *mut cv1800b_priv, enable: bool) {
    let mut val: u32_t;

    val = readl((*priv_).regs.add(CV1800B_TXDAC_ANA2));
    val = u32_replace_bits(val, enable as u32, TXDAC_OW_EN_L_MASK);
    val = u32_replace_bits(val, enable as u32, TXDAC_OW_EN_R_MASK);
    writel(val, (*priv_).regs.add(CV1800B_TXDAC_ANA2));
}

unsafe fn cv1800b_dac_decimation(priv_: *mut cv1800b_priv, dec: u8_t) -> i32 {
    let mut val: u32_t;

    if dec > 3 {
        return -EINVAL;
    }

    val = readl((*priv_).regs.add(CV1800B_TXDAC_CTRL1));
    val = u32_replace_bits(val, dec as u32, REG_TXDAC_CIC_OPT);
    writel(val, (*priv_).regs.add(CV1800B_TXDAC_CTRL1));
    0
}

unsafe fn cv1800b_dac_dly(priv_: *mut cv1800b_priv, dly: u32_t) -> i32 {
    let mut val: u32_t;

    if dly > 63 {
        return -EINVAL;
    }

    val = readl((*priv_).regs.add(CV1800B_TXDAC_AFE0));
    val = u32_replace_bits(val, dly, REG_TXDAC_INIT_DLY_CNT);
    writel(val, (*priv_).regs.add(CV1800B_TXDAC_AFE0));
    0
}

unsafe extern "C" fn cv1800b_dac_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_priv;
    let mut ret: i32;
    let rate: u32 = params_rate(params);
    let _ = substream;

    if rate != 48000 {
        dev_err(
            (*priv_).dev,
            b"rate %u is not supported\n\0".as_ptr() as *const i8,
            rate,
        );
        return -EINVAL;
    }
    /* Clear DAC overwrite so playback uses I2S data. */
    cv1800b_dac_mute(priv_, false);
    /* minimal decimation for 48kHz is 64*/
    ret = cv1800b_dac_decimation(priv_, decimation_values::DECIMATION_64 as u8);
    if ret != 0 {
        return ret;
    }

    /* value is taken from vendors driver 48kHz
     * tested on sg2000 and sg2002.
     */
    ret = cv1800b_dac_dly(priv_, 0x19);
    if ret != 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn cv1800b_dac_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: i32,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_priv;
    let _ = substream;

    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        cv1800b_dac_enable(priv_, true);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
    {
        cv1800b_dac_enable(priv_, false);
    } else {
        return -EINVAL;
    }

    0
}

static cv1800b_dac_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cv1800b_dac_hw_params),
    trigger: Some(cv1800b_dac_dai_trigger),
};

static mut cv1800b_dac_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"dac-hifi\0".as_ptr() as *const i8,
    playback: snd_soc_pcm_stream {
        stream_name: b"DAC Playback\0".as_ptr() as *const i8,
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    ops: unsafe { &cv1800b_dac_dai_ops as *const snd_soc_dai_ops },
};

static cv1800b_dac_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"cv1800b-dac-codec\0".as_ptr() as *const i8,
};

unsafe extern "C" fn cv1800b_dac_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let priv_: *mut cv1800b_priv;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<cv1800b_priv>(), GFP_KERNEL) as *mut cv1800b_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).dev = dev;
    (*priv_).regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*priv_).regs) {
        return PTR_ERR((*priv_).regs);
    }

    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &cv1800b_dac_component,
        &mut cv1800b_dac_dai,
        1,
    )
}

static cv1800b_dac_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"sophgo,cv1800b-sound-dac\0".as_ptr() as *const i8,
    },
    of_device_id {
        /* sentinel */
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cv1800b_dac_of_match);

static mut cv1800b_dac_driver: platform_driver = platform_driver {
    probe: Some(cv1800b_dac_probe),
    driver: driver {
        name: b"cv1800b-dac-codec\0".as_ptr() as *const i8,
        of_match_table: cv1800b_dac_of_match.as_ptr(),
    },
};
// module_platform_driver(cv1800b_dac_driver);

// MODULE_DESCRIPTION("DAC codec for CV1800B");
// MODULE_AUTHOR("Anton D. Stavinskii <stavinsky@gmail.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
