// SPDX-License-Identifier: GPL-2.0+
// Copyright 2018-2021 NXP

// Rust translation of soc/fsl/fsl_rpmsg.c.
// External Linux/ALSA symbols and types are declared here as dependencies
// supplied by the surrounding kernel bindings.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const FSL_RPMSG_RATES: c_uint =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000;
const FSL_RPMSG_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE;

/* 192kHz/32bit/2ch/60s size is 0x574e00 */
const LPA_LARGE_BUFFER_SIZE: c_uint = 0x6000000;
/* 16kHz/32bit/8ch/1s size is 0x7D000 */
const LPA_CAPTURE_BUFFER_SIZE: c_uint = 0x100000;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_11025: c_uint = 1 << 1;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 3;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 5;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 6;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 7;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 8;
const SNDRV_PCM_RATE_176400: c_uint = 1 << 9;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 10;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 31;
const SNDRV_PCM_RATE_8000_48000: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_DSD_U8: c_ulong = 1 << 3;
const SNDRV_PCM_FMTBIT_DSD_U16_LE: c_ulong = 1 << 4;
const SNDRV_PCM_FMTBIT_DSD_U32_LE: c_ulong = 1 << 5;
const IMX_DEFAULT_DMABUF_SIZE: c_uint = 0;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
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
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub hw_free: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct fsl_rpmsg_soc_data {
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct fsl_rpmsg {
    pub mclk: *mut clk,
    pub pll8k: *mut clk,
    pub pll11k: *mut clk,
    pub mclk_streams: c_ulong,
    pub soc_data: *const fsl_rpmsg_soc_data,
    pub enable_lpa: c_int,
    pub buffer_size: [c_uint; 2],
    pub ipg: *mut clk,
    pub dma: *mut clk,
    pub card_pdev: *mut platform_device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub idle: *const c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_inner,
}

unsafe extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u64;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_is_match(p: *mut clk, q: *mut clk) -> bool;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_property_read_string(
        np: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn of_device_is_compatible(np: *mut device_node, compatible: *const c_char) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const fn BIT(nr: c_int) -> c_ulong {
    (1 as c_ulong) << nr
}

static fsl_rpmsg_rates: [c_uint; 17] = [
    8000, 11025, 16000, 22050, 44100, 32000, 48000, 96000, 88200, 176400, 192000, 352800, 384000,
    705600, 768000, 1411200, 2822400,
];

static fsl_rpmsg_rate_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: fsl_rpmsg_rates.len() as c_uint,
    list: fsl_rpmsg_rates.as_ptr(),
};

unsafe extern "C" fn fsl_rpmsg_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rpmsg = snd_soc_dai_get_drvdata(dai) as *mut fsl_rpmsg;
    let mut p = (*rpmsg).mclk;
    let mut pll: *mut clk = ptr::null_mut();
    let mut npll: *mut clk;
    let mut rate = params_rate(params);
    let mut ret: c_int = 0;

    /* Get current pll parent */
    while !p.is_null() && !(*rpmsg).pll8k.is_null() && !(*rpmsg).pll11k.is_null() {
        let pp = clk_get_parent(p);

        if clk_is_match(pp, (*rpmsg).pll8k) || clk_is_match(pp, (*rpmsg).pll11k) {
            pll = pp;
            break;
        }
        p = pp;
    }

    /* Switch to another pll parent if needed. */
    if !pll.is_null() {
        let rem = rate % 8000;
        rate /= 8000;
        npll = if rem != 0 { (*rpmsg).pll11k } else { (*rpmsg).pll8k };
        if !clk_is_match(pll, npll) {
            ret = clk_set_parent(p, npll);
            if ret < 0 {
                dev_warn(
                    (*dai).dev,
                    c"failed to set parent %s: %d\n".as_ptr(),
                    __clk_get_name(npll),
                    ret,
                );
            }
        }
    }

    if ((*rpmsg).mclk_streams & BIT((*substream).stream)) == 0 {
        ret = clk_prepare_enable((*rpmsg).mclk);
        if ret != 0 {
            dev_err((*dai).dev, c"failed to enable mclk: %d\n".as_ptr(), ret);
            return ret;
        }

        (*rpmsg).mclk_streams |= BIT((*substream).stream);
    }

    ret
}

unsafe extern "C" fn fsl_rpmsg_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rpmsg = snd_soc_dai_get_drvdata(dai) as *mut fsl_rpmsg;

    if ((*rpmsg).mclk_streams & BIT((*substream).stream)) != 0 {
        clk_disable_unprepare((*rpmsg).mclk);
        (*rpmsg).mclk_streams &= !BIT((*substream).stream);
    }

    0
}

unsafe extern "C" fn fsl_rpmsg_startup(
    substream: *mut snd_pcm_substream,
    _cpu_dai: *mut snd_soc_dai,
) -> c_int {
    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &fsl_rpmsg_rate_constraints,
    )
}

static fsl_rpmsg_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(fsl_rpmsg_startup),
    hw_params: Some(fsl_rpmsg_hw_params),
    hw_free: Some(fsl_rpmsg_hw_free),
};

static mut fsl_rpmsg_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: ptr::null(),
    playback: snd_soc_pcm_stream {
        stream_name: c"CPU-Playback".as_ptr(),
        channels_min: 2,
        channels_max: 32,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: FSL_RPMSG_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"CPU-Capture".as_ptr(),
        channels_min: 2,
        channels_max: 32,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: FSL_RPMSG_FORMATS,
    },
    symmetric_rate: 1,
    symmetric_channels: 1,
    symmetric_sample_bits: 1,
    ops: &fsl_rpmsg_dai_ops,
};

static fsl_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"fsl-rpmsg".as_ptr(),
};

static imx7ulp_data: fsl_rpmsg_soc_data = fsl_rpmsg_soc_data {
    rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
};

static imx8mm_data: fsl_rpmsg_soc_data = fsl_rpmsg_soc_data {
    rates: SNDRV_PCM_RATE_KNOT,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_DSD_U8
        | SNDRV_PCM_FMTBIT_DSD_U16_LE
        | SNDRV_PCM_FMTBIT_DSD_U32_LE,
};

static imx8mn_data: fsl_rpmsg_soc_data = fsl_rpmsg_soc_data {
    rates: SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
};

static imx8mp_data: fsl_rpmsg_soc_data = fsl_rpmsg_soc_data {
    rates: SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
};

static imx93_data: fsl_rpmsg_soc_data = fsl_rpmsg_soc_data {
    rates: SNDRV_PCM_RATE_16000
        | SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_96000,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
};

static imx95_data: fsl_rpmsg_soc_data = fsl_rpmsg_soc_data {
    rates: SNDRV_PCM_RATE_16000
        | SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
};

static fsl_rpmsg_ids: [of_device_id; 8] = [
    of_device_id {
        compatible: c"fsl,imx7ulp-rpmsg-audio".as_ptr(),
        data: &imx7ulp_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx8mm-rpmsg-audio".as_ptr(),
        data: &imx8mm_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx8mn-rpmsg-audio".as_ptr(),
        data: &imx8mn_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx8mp-rpmsg-audio".as_ptr(),
        data: &imx8mp_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx8ulp-rpmsg-audio".as_ptr(),
        data: &imx7ulp_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx93-rpmsg-audio".as_ptr(),
        data: &imx93_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx95-rpmsg-audio".as_ptr(),
        data: &imx95_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, fsl_rpmsg_ids);

unsafe extern "C" fn fsl_rpmsg_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut dai_drv: *mut snd_soc_dai_driver;
    let mut dai_name: *const c_char;
    let rpmsg: *mut fsl_rpmsg;
    let mut ret: c_int;

    dai_drv = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if dai_drv.is_null() {
        return -ENOMEM;
    }
    memcpy(
        dai_drv as *mut c_void,
        &raw const fsl_rpmsg_dai as *const c_void,
        size_of::<snd_soc_dai_driver>(),
    );

    rpmsg = devm_kzalloc(&mut (*pdev).dev, size_of::<fsl_rpmsg>(), GFP_KERNEL) as *mut fsl_rpmsg;
    if rpmsg.is_null() {
        return -ENOMEM;
    }

    (*rpmsg).soc_data = of_device_get_match_data(&mut (*pdev).dev) as *const fsl_rpmsg_soc_data;

    if !(*rpmsg).soc_data.is_null() {
        (*dai_drv).playback.rates = (*(*rpmsg).soc_data).rates;
        (*dai_drv).capture.rates = (*(*rpmsg).soc_data).rates;
        (*dai_drv).playback.formats = (*(*rpmsg).soc_data).formats;
        (*dai_drv).capture.formats = (*(*rpmsg).soc_data).formats;
    }

    /* Use rpmsg channel name as cpu dai name */
    ret = of_property_read_string(
        np,
        c"fsl,rpmsg-channel-name".as_ptr(),
        &mut dai_name,
    );
    if ret != 0 {
        if ret == -EINVAL {
            dai_name = c"rpmsg-audio-channel".as_ptr();
        } else {
            dev_err(
                &mut (*pdev).dev,
                c"Failed to get rpmsg channel name: %d!\n".as_ptr(),
                ret,
            );
            return ret;
        }
    }
    (*dai_drv).name = dai_name;

    /* Setup cpu dai for sound card that sits on rpmsg-micfil-channel */
    if strcmp(dai_name, c"rpmsg-micfil-channel".as_ptr()) == 0 {
        (*dai_drv).capture.channels_min = 1;
        (*dai_drv).capture.channels_max = 8;
        (*dai_drv).capture.rates = SNDRV_PCM_RATE_8000_48000;
        (*dai_drv).capture.formats = SNDRV_PCM_FMTBIT_S32_LE;
        if of_device_is_compatible(np, c"fsl,imx8mm-rpmsg-audio".as_ptr()) != 0 {
            (*dai_drv).capture.formats = SNDRV_PCM_FMTBIT_S16_LE;
        }
    }

    if of_property_read_bool(np, c"fsl,enable-lpa".as_ptr()) {
        (*rpmsg).enable_lpa = 1;
        (*rpmsg).buffer_size[SNDRV_PCM_STREAM_PLAYBACK] = LPA_LARGE_BUFFER_SIZE;
        (*rpmsg).buffer_size[SNDRV_PCM_STREAM_CAPTURE] = LPA_CAPTURE_BUFFER_SIZE;
    } else {
        (*rpmsg).buffer_size[SNDRV_PCM_STREAM_PLAYBACK] = IMX_DEFAULT_DMABUF_SIZE;
        (*rpmsg).buffer_size[SNDRV_PCM_STREAM_CAPTURE] = IMX_DEFAULT_DMABUF_SIZE;
    }

    /* Get the optional clocks */
    (*rpmsg).ipg = devm_clk_get_optional(&mut (*pdev).dev, c"ipg".as_ptr());
    if IS_ERR((*rpmsg).ipg as *const c_void) {
        return PTR_ERR((*rpmsg).ipg as *const c_void);
    }

    (*rpmsg).mclk = devm_clk_get_optional(&mut (*pdev).dev, c"mclk".as_ptr());
    if IS_ERR((*rpmsg).mclk as *const c_void) {
        return PTR_ERR((*rpmsg).mclk as *const c_void);
    }

    (*rpmsg).dma = devm_clk_get_optional(&mut (*pdev).dev, c"dma".as_ptr());
    if IS_ERR((*rpmsg).dma as *const c_void) {
        return PTR_ERR((*rpmsg).dma as *const c_void);
    }

    (*rpmsg).pll8k = devm_clk_get_optional(&mut (*pdev).dev, c"pll8k".as_ptr());
    if IS_ERR((*rpmsg).pll8k as *const c_void) {
        return PTR_ERR((*rpmsg).pll8k as *const c_void);
    }

    (*rpmsg).pll11k = devm_clk_get_optional(&mut (*pdev).dev, c"pll11k".as_ptr());
    if IS_ERR((*rpmsg).pll11k as *const c_void) {
        return PTR_ERR((*rpmsg).pll11k as *const c_void);
    }

    platform_set_drvdata(pdev, rpmsg as *mut c_void);
    pm_runtime_enable(&mut (*pdev).dev);

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &fsl_component, dai_drv, 1);
    if ret != 0 {
        goto_err_pm_disable(&mut (*pdev).dev);
        return ret;
    }

    0
}

unsafe fn goto_err_pm_disable(dev: *mut device) {
    pm_runtime_disable(dev);
}

unsafe extern "C" fn fsl_rpmsg_remove(pdev: *mut platform_device) {
    let rpmsg = platform_get_drvdata(pdev) as *mut fsl_rpmsg;

    pm_runtime_disable(&mut (*pdev).dev);

    if !(*rpmsg).card_pdev.is_null() {
        platform_device_unregister((*rpmsg).card_pdev);
    }
}

unsafe extern "C" fn fsl_rpmsg_runtime_resume(dev: *mut device) -> c_int {
    let rpmsg = dev_get_drvdata(dev) as *mut fsl_rpmsg;
    let mut ret: c_int;

    ret = clk_prepare_enable((*rpmsg).ipg);
    if ret != 0 {
        dev_err(dev, c"failed to enable ipg clock: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = clk_prepare_enable((*rpmsg).dma);
    if ret != 0 {
        dev_err(dev, c"Failed to enable dma clock %d\n".as_ptr(), ret);
        clk_disable_unprepare((*rpmsg).ipg);
        return ret;
    }

    0
}

unsafe extern "C" fn fsl_rpmsg_runtime_suspend(dev: *mut device) -> c_int {
    let rpmsg = dev_get_drvdata(dev) as *mut fsl_rpmsg;

    clk_disable_unprepare((*rpmsg).dma);
    clk_disable_unprepare((*rpmsg).ipg);

    0
}

static fsl_rpmsg_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(fsl_rpmsg_runtime_suspend),
    runtime_resume: Some(fsl_rpmsg_runtime_resume),
    idle: ptr::null(),
};

static mut fsl_rpmsg_driver: platform_driver = platform_driver {
    probe: Some(fsl_rpmsg_probe),
    remove: Some(fsl_rpmsg_remove),
    driver: platform_driver_inner {
        name: c"fsl_rpmsg".as_ptr(),
        pm: &fsl_rpmsg_pm_ops,
        of_match_table: fsl_rpmsg_ids.as_ptr(),
    },
};
// module_platform_driver(fsl_rpmsg_driver);

// MODULE_DESCRIPTION("Freescale SoC Audio PRMSG CPU Interface");
// MODULE_AUTHOR("Shengjiu Wang <shengjiu.wang@nxp.com>");
// MODULE_ALIAS("platform:fsl_rpmsg");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
