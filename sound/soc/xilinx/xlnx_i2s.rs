// SPDX-License-Identifier: GPL-2.0
//
// Xilinx ASoC I2S audio support
//
// Copyright (C) 2018 Xilinx, Inc.
//
// Author: Praveen Vuppala <praveenv@xilinx.com>
// Author: Maruthi Srinivas Bayyavarapu <maruthis@xilinx.com>

use core::ffi::c_void;

// External kernel functions and types (declared but not implemented)
extern "C" {
    fn writel(val: u32, addr: *mut u32);
    fn readl(addr: *const u32) -> u32;
    fn snd_soc_dai_get_drvdata(dai: *mut c_void) -> *mut c_void;
    fn snd_pcm_hw_constraint_ratnums(
        runtime: *mut c_void,
        cond: u32,
        var: u32,
        r: *const c_void,
    ) -> i32;
    fn params_rate(p: *const c_void) -> u32;
    fn params_channels(p: *const c_void) -> u32;
    fn device_property_read_u32(dev: *mut c_void, propname: *const u8, val: *mut u32) -> i32;
    fn dev_err_probe(dev: *mut c_void, err: i32, fmt: *const u8, ...) -> i32;
    fn device_is_compatible(device: *const c_void, compat: *const u8) -> i32;
    fn dev_warn(dev: *mut c_void, fmt: *const u8, ...);
    fn dev_info(dev: *mut c_void, fmt: *const u8, ...);
    fn dev_set_drvdata(dev: *mut c_void, data: *mut c_void);
    fn devm_kzalloc(dev: *mut c_void, size: usize, gfp: u32) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut c_void, index: u32) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut c_void,
        cmpnt_drv: *const c_void,
        dai_drv: *const c_void,
        num_dai: u32,
    ) -> i32;
}

const DRV_NAME: &[u8] = b"xlnx_i2s\0";

const I2S_CORE_CTRL_OFFSET: u32 = 0x08;
const I2S_CORE_CTRL_32BIT_LRCLK: u32 = 1 << 3;
const I2S_CORE_CTRL_ENABLE: u32 = 1 << 0;
const I2S_I2STIM_OFFSET: u32 = 0x20;
const I2S_CH0_OFFSET: u32 = 0x30;
const I2S_I2STIM_VALID_MASK: u32 = 0xff;

const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_STOP: i32 = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32 = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32 = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 5;
const SNDRV_PCM_TRIGGER_RESUME: i32 = 6;

const SNDRV_PCM_HW_PARAM_RATE: u32 = 10;

const SNDRV_PCM_FMTBIT_S16_LE: u32 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u32 = 1 << 6;

const SNDRV_PCM_RATE_8000_192000: u32 = 0x0ff0;

const GFP_KERNEL: u32 = 0xd0;

#[repr(C)]
pub struct SndRatnum {
    pub num: u32,
    pub den_step: u32,
    pub den_min: u32,
    pub den_max: u32,
}

#[repr(C)]
pub struct SndPcmHwConstraintRatnums {
    pub rats: *const SndRatnum,
    pub nrats: u32,
}

#[repr(C)]
pub struct SndSocDaiStream {
    pub stream_name: *const u8,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u32,
}

#[repr(C)]
pub struct SndSocDaiOps {
    pub trigger: Option<unsafe extern "C" fn(*mut c_void, i32, *mut c_void) -> i32>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut c_void, i32, u32, i32) -> i32>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut c_void, i32, i32) -> i32>,
    pub startup: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32>,
    pub hw_params: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> i32>,
}

#[repr(C)]
pub struct SndSocDaiDriver {
    pub name: *const u8,
    pub playback: SndSocDaiStream,
    pub capture: SndSocDaiStream,
    pub ops: *const SndSocDaiOps,
}

#[repr(C)]
pub struct XlnxI2sDrvData {
    pub dai_drv: SndSocDaiDriver,
    pub base: *mut u32,
    pub sysclk: u32,
    pub data_width: u32,
    pub channels: u32,
    pub is_32bit_lrclk: bool,
    pub ratnum: SndRatnum,
    pub rate_constraints: SndPcmHwConstraintRatnums,
}

#[repr(C)]
pub struct SndSocComponentDriver {
    pub name: *const u8,
    pub legacy_dai_naming: i32,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
    pub data: *const c_void,
}

#[repr(C)]
pub struct PlatformDriver {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
}

unsafe extern "C" fn xlnx_i2s_set_sclkout_div(
    cpu_dai: *mut c_void,
    div_id: i32,
    div: i32,
) -> i32 {
    let drv_data = snd_soc_dai_get_drvdata(cpu_dai) as *mut XlnxI2sDrvData;

    if div == 0 || ((div as u32) & !I2S_I2STIM_VALID_MASK) != 0 {
        return -22;
    }

    (*drv_data).sysclk = 0;

    writel(div as u32, (*drv_data).base.add((I2S_I2STIM_OFFSET >> 2) as usize));

    0
}

unsafe extern "C" fn xlnx_i2s_set_sysclk(
    dai: *mut c_void,
    clk_id: i32,
    freq: u32,
    dir: i32,
) -> i32 {
    let drv_data = snd_soc_dai_get_drvdata(dai) as *mut XlnxI2sDrvData;

    (*drv_data).sysclk = freq;
    if freq != 0 {
        let bits_per_sample = if (*drv_data).is_32bit_lrclk {
            32
        } else {
            (*drv_data).data_width
        };

        (*drv_data).ratnum.num = freq / (bits_per_sample * (*drv_data).channels) / 2;
        (*drv_data).ratnum.den_step = 1;
        (*drv_data).ratnum.den_min = 1;
        (*drv_data).ratnum.den_max = 255;
        (*drv_data).rate_constraints.rats = &(*drv_data).ratnum;
        (*drv_data).rate_constraints.nrats = 1;
    }
    0
}

unsafe extern "C" fn xlnx_i2s_startup(
    substream: *mut c_void,
    dai: *mut c_void,
) -> i32 {
    let drv_data = snd_soc_dai_get_drvdata(dai) as *mut XlnxI2sDrvData;

    if (*drv_data).sysclk != 0 {
        let runtime = *(substream as *mut *mut c_void);
        return snd_pcm_hw_constraint_ratnums(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            &(*drv_data).rate_constraints as *const _ as *const c_void,
        );
    }

    0
}

unsafe extern "C" fn xlnx_i2s_hw_params(
    substream: *mut c_void,
    params: *mut c_void,
    i2s_dai: *mut c_void,
) -> i32 {
    let mut reg_off: u32;
    let mut chan_id: u32;
    let drv_data = snd_soc_dai_get_drvdata(i2s_dai) as *mut XlnxI2sDrvData;

    if (*drv_data).sysclk != 0 {
        let bits_per_sample = if (*drv_data).is_32bit_lrclk {
            32
        } else {
            (*drv_data).data_width
        };

        let sclk = params_rate(params) * bits_per_sample * params_channels(params);
        let sclk_div = (*drv_data).sysclk / sclk / 2;

        if ((*drv_data).sysclk % sclk != 0) || sclk_div == 0 || (sclk_div & !I2S_I2STIM_VALID_MASK) != 0 {
            dev_warn(
                *(i2s_dai as *mut *mut c_void),
                b"invalid SCLK divisor for sysclk %u and sclk %u\n\0".as_ptr(),
                (*drv_data).sysclk,
                sclk,
            );
            return -22;
        }
        writel(sclk_div, (*drv_data).base.add((I2S_I2STIM_OFFSET >> 2) as usize));
    }

    chan_id = params_channels(params) / 2;

    while chan_id > 0 {
        reg_off = I2S_CH0_OFFSET + ((chan_id - 1) * 4);
        writel(chan_id, (*drv_data).base.add((reg_off >> 2) as usize));
        chan_id -= 1;
    }

    0
}

unsafe extern "C" fn xlnx_i2s_trigger(
    substream: *mut c_void,
    cmd: i32,
    i2s_dai: *mut c_void,
) -> i32 {
    let drv_data = snd_soc_dai_get_drvdata(i2s_dai) as *mut XlnxI2sDrvData;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            writel(I2S_CORE_CTRL_ENABLE, (*drv_data).base.add((I2S_CORE_CTRL_OFFSET >> 2) as usize));
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            writel(0, (*drv_data).base.add((I2S_CORE_CTRL_OFFSET >> 2) as usize));
        }
        _ => {
            return -22;
        }
    }

    0
}

#[used]
static XLNX_I2S_DAI_OPS: SndSocDaiOps = SndSocDaiOps {
    trigger: Some(xlnx_i2s_trigger),
    set_sysclk: Some(xlnx_i2s_set_sysclk),
    set_clkdiv: Some(xlnx_i2s_set_sclkout_div),
    startup: Some(xlnx_i2s_startup),
    hw_params: Some(xlnx_i2s_hw_params),
};

#[used]
static XLNX_I2S_COMPONENT: SndSocComponentDriver = SndSocComponentDriver {
    name: DRV_NAME.as_ptr(),
    legacy_dai_naming: 1,
};

#[repr(C)]
struct OfDeviceIdTable {
    compatibles: [OfDeviceId; 3],
}

#[used]
static XLNX_I2S_OF_MATCH: [OfDeviceId; 3] = [
    OfDeviceId {
        compatible: b"xlnx,i2s-transmitter-1.0\0".as_ptr(),
        data: core::ptr::null(),
    },
    OfDeviceId {
        compatible: b"xlnx,i2s-receiver-1.0\0".as_ptr(),
        data: core::ptr::null(),
    },
    OfDeviceId {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

unsafe extern "C" fn xlnx_i2s_probe(pdev: *mut c_void) -> i32 {
    let mut drv_data: *mut XlnxI2sDrvData;
    let mut ret: i32;
    let mut format: u32;
    let dev = pdev as *mut c_void;

    drv_data = devm_kzalloc(dev, core::mem::size_of::<XlnxI2sDrvData>(), GFP_KERNEL)
        as *mut XlnxI2sDrvData;
    if drv_data.is_null() {
        return -12;
    }

    (*drv_data).base = devm_platform_ioremap_resource(pdev, 0) as *mut u32;
    if ((*drv_data).base as i32) < 0 {
        return (*drv_data).base as i32;
    }

    ret = device_property_read_u32(dev, b"xlnx,num-channels\0".as_ptr(), &mut (*drv_data).channels);
    if ret < 0 {
        return dev_err_probe(dev, ret, b"cannot get supported channels\n\0".as_ptr());
    }

    (*drv_data).channels *= 2;

    ret = device_property_read_u32(dev, b"xlnx,dwidth\0".as_ptr(), &mut (*drv_data).data_width);
    if ret < 0 {
        return dev_err_probe(dev, ret, b"cannot get data width\n\0".as_ptr());
    }

    format = match (*drv_data).data_width {
        16 => SNDRV_PCM_FMTBIT_S16_LE,
        24 => SNDRV_PCM_FMTBIT_S24_LE,
        _ => {
            return -22;
        }
    };

    if device_is_compatible(dev, b"xlnx,i2s-transmitter-1.0\0".as_ptr()) != 0 {
        (*drv_data).dai_drv.name = b"xlnx_i2s_playback\0".as_ptr();
        (*drv_data).dai_drv.playback.stream_name = b"Playback\0".as_ptr();
        (*drv_data).dai_drv.playback.formats = format;
        (*drv_data).dai_drv.playback.channels_min = (*drv_data).channels;
        (*drv_data).dai_drv.playback.channels_max = (*drv_data).channels;
        (*drv_data).dai_drv.playback.rates = SNDRV_PCM_RATE_8000_192000;
        (*drv_data).dai_drv.ops = &XLNX_I2S_DAI_OPS;
    } else if device_is_compatible(dev, b"xlnx,i2s-receiver-1.0\0".as_ptr()) != 0 {
        (*drv_data).dai_drv.name = b"xlnx_i2s_capture\0".as_ptr();
        (*drv_data).dai_drv.capture.stream_name = b"Capture\0".as_ptr();
        (*drv_data).dai_drv.capture.formats = format;
        (*drv_data).dai_drv.capture.channels_min = (*drv_data).channels;
        (*drv_data).dai_drv.capture.channels_max = (*drv_data).channels;
        (*drv_data).dai_drv.capture.rates = SNDRV_PCM_RATE_8000_192000;
        (*drv_data).dai_drv.ops = &XLNX_I2S_DAI_OPS;
    } else {
        return -19;
    }

    (*drv_data).is_32bit_lrclk = (readl((*drv_data).base.add((I2S_CORE_CTRL_OFFSET >> 2) as usize))
        & I2S_CORE_CTRL_32BIT_LRCLK)
        != 0;

    dev_set_drvdata(dev, drv_data as *mut c_void);

    ret = devm_snd_soc_register_component(
        dev,
        &XLNX_I2S_COMPONENT as *const _ as *const c_void,
        &(*drv_data).dai_drv as *const _ as *const c_void,
        1,
    );
    if ret != 0 {
        return ret;
    }

    dev_info(
        dev,
        b"%s DAI registered\n\0".as_ptr(),
        (*drv_data).dai_drv.name,
    );

    ret
}

#[repr(C)]
pub struct PlatformDeviceDriver {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

#[used]
static XLNX_I2S_DRIVER: PlatformDeviceDriver = PlatformDeviceDriver {
    name: DRV_NAME.as_ptr(),
    of_match_table: XLNX_I2S_OF_MATCH.as_ptr(),
};

// Module-level constants (kernel module metadata)
// MODULE_DESCRIPTION, MODULE_LICENSE, MODULE_AUTHOR are kernel module directives
// that would be handled by kernel build system. Platform driver registration
// would be handled via module_platform_driver macro equivalent.

// Note: module_platform_driver(xlnx_i2s_aud_driver) macro would register the
// platform driver and link probe function. In Rust, this requires kernel module
// infrastructure integration handled externally.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
