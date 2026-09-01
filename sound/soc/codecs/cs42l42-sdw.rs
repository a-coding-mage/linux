// SPDX-License-Identifier: GPL-2.0-only
// cs42l42-sdw.c -- CS42L42 ALSA SoC audio driver SoundWire driver
//
// Copyright (C) 2022 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u8 = u8;
type bool_ = bool;

const CS42L42_SDW_CAPTURE_PORT: c_uint = 1;
const CS42L42_SDW_PLAYBACK_PORT: c_uint = 2;

/* Register addresses are offset when sent over SoundWire */
const CS42L42_SDW_ADDR_OFFSET: c_uint = 0x8000;

const CS42L42_SDW_MEM_ACCESS_STATUS: c_uint = 0xd0;
const CS42L42_SDW_MEM_READ_DATA: c_uint = 0xd8;

const CS42L42_SDW_LAST_LATE: u8 = BIT(3) as u8;
const CS42L42_SDW_CMD_IN_PROGRESS: u8 = BIT(2) as u8;
const CS42L42_SDW_RDATA_RDY: u8 = BIT(0) as u8;

const CS42L42_DELAYED_READ_POLL_US: c_uint = 1;
const CS42L42_DELAYED_READ_TIMEOUT_US: c_uint = 100;

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
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
pub struct regmap {
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_stream_config {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_port_config {
    pub num: c_uint,
}

#[repr(C)]
pub struct sdw_prepare_ch {
    pub num: c_uint,
}

pub type sdw_port_prep_ops = c_uint;
const SDW_OPS_PORT_PRE_PREP: sdw_port_prep_ops = 0;
const SDW_OPS_PORT_POST_DEPREP: sdw_port_prep_ops = 1;

pub type sdw_slave_status = c_uint;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 1;

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_uint,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub source_ports: c_uint,
    pub sink_ports: c_uint,
    pub quirks: c_uint,
    pub scp_int1_mask: c_uint,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub unattach_request: bool_,
}

#[repr(C)]
pub struct sdw_bus_params {
    pub curr_dr_freq: c_uint,
    pub col: c_uint,
    pub row: c_uint,
}

#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
    pub delay_us: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn()>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub symmetric_rate: c_uint,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub num_ranges: c_uint,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave_ops {
    /* No interrupt callback because only hardware INT is supported for Jack Detect in the CS42L42 */
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub bus_config: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_bus_params) -> c_int>,
    pub port_prep: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_prepare_ch, sdw_port_prep_ops) -> c_int>,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdw_slave)>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

#[repr(C)]
pub struct cs42l42_private {
    pub init_done: bool_,
    pub sample_rate: c_uint,
    pub sclk: c_uint,
    pub stream_use: bool_,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub sdw_peripheral: *mut sdw_slave,
    pub irq: c_int,
    pub devid: c_uint,
    pub sdw_waiting_first_unattach: bool_,
    pub reset_gpio: *mut gpio_desc,
    pub ts_dbnc_rise: c_uint,
    pub ts_dbnc_fall: c_uint,
}

extern "C" {
    static cs42l42_regmap: regmap_config;
    static cs42l42_soc_component: snd_soc_component_driver;
    static cs42l42_mute_stream: unsafe extern "C" fn();

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_sdw_params_to_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn sdw_stream_add_slave(peripheral: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_uint, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(peripheral: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn cs42l42_src_config(component: *mut snd_soc_component, rate: c_uint);
    fn cs42l42_pll_config(component: *mut snd_soc_component, sclk: c_uint, sample_rate: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, mask: c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, mask: c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn dev_get_drvdata(dev: *const device) -> *mut c_void;
    fn dev_err(dev: *const device, fmt: *const c_char, ...);
    fn dev_warn(dev: *const device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *const device, fmt: *const c_char, ...);
    fn sdw_read_no_pm(peripheral: *mut sdw_slave, reg: c_uint) -> c_int;
    fn sdw_write_no_pm(peripheral: *mut sdw_slave, reg: c_uint, val: u8) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn cs42l42_init(cs42l42: *mut cs42l42_private) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regmap_multi_reg_write_bypassed(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn sdw_slave_wait_for_init(peripheral: *mut sdw_slave, timeout: c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn msleep(msecs: c_uint);
    fn regcache_sync_region(map: *mut regmap, min: c_uint, max: c_uint) -> c_int;
    fn cs42l42_resume(dev: *mut device) -> c_int;
    fn cs42l42_resume_restore(dev: *mut device);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn has_acpi_companion(dev: *mut device) -> bool_;
    fn ACPI_COMPANION(dev: *mut device) -> *mut c_void;
    fn acpi_dev_gpio_irq_get(adev: *mut c_void, index: c_int) -> c_int;
    fn of_irq_get(node: *mut c_void, index: c_int) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init(dev: *mut device, bus: *const c_void, bus_context: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device) -> c_int;
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device) -> c_int;
    fn cs42l42_common_probe(cs42l42: *mut cs42l42_private, component_drv: *mut snd_soc_component_driver, dai: *mut snd_soc_dai_driver) -> c_int;
    fn cs42l42_common_remove(cs42l42: *mut cs42l42_private);
    fn pm_runtime_disable(dev: *mut device);
    fn cs42l42_suspend(dev: *mut device) -> c_int;
}

extern "C" {
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static EINVAL: c_int;
    static ENODEV: c_int;
    static ENOMEM: c_int;
    static ENOENT: c_int;
    static EBUSY: c_int;
    static GFP_KERNEL: c_uint;
    static CS42L42_HP_PDN_MASK: c_uint;
    static CS42L42_ADC_PDN_MASK: c_uint;
    static CS42L42_PWR_CTL1: c_uint;
    static CS42L42_HP_ADC_EN_TIME_US: c_uint;
    static CS42L42_PWR_CTL3: c_uint;
    static CS42L42_SW_CLK_STP_STAT_SEL_MASK: c_uint;
    static SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY: c_uint;
    static SDW_SCP_INT1_BUS_CLASH: c_uint;
    static SDW_SCP_INT1_PARITY: c_uint;
    static SDW_DPN_FULL: c_uint;
    static CS42L42_SOFT_RESET_REBOOT: c_uint;
    static CS42L42_BOOT_TIME_US: c_uint;
    static CS42L42_MIC_DET_CTL1: c_uint;
    static CS42L42_CHIP_ID: c_uint;
}

const NULL: *const c_char = ptr::null();

static cs42l42_sdw_audio_map: [snd_soc_dapm_route; 5] = [
    /* Playback Path */
    snd_soc_dapm_route { sink: c"HP".as_ptr(), control: NULL, source: c"MIXER".as_ptr() },
    snd_soc_dapm_route { sink: c"MIXER".as_ptr(), control: NULL, source: c"DACSRC".as_ptr() },
    snd_soc_dapm_route { sink: c"DACSRC".as_ptr(), control: NULL, source: c"Playback".as_ptr() },

    /* Capture Path */
    snd_soc_dapm_route { sink: c"ADCSRC".as_ptr(), control: NULL, source: c"HS".as_ptr() },
    snd_soc_dapm_route { sink: c"Capture".as_ptr(), control: NULL, source: c"ADCSRC".as_ptr() },
];

unsafe extern "C" fn cs42l42_sdw_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let cs42l42 = snd_soc_component_get_drvdata((*dai).component) as *mut cs42l42_private;

    if !(*cs42l42).init_done {
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn cs42l42_sdw_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let cs42l42 = snd_soc_component_get_drvdata((*dai).component) as *mut cs42l42_private;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let ret: c_int;

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    /* Needed for PLL configuration when we are notified of new bus config */
    (*cs42l42).sample_rate = params_rate(params);

    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        port_config.num = CS42L42_SDW_PLAYBACK_PORT;
    } else {
        port_config.num = CS42L42_SDW_CAPTURE_PORT;
    }

    ret = sdw_stream_add_slave((*cs42l42).sdw_peripheral, &mut stream_config, &mut port_config, 1, sdw_stream);
    if ret != 0 {
        dev_err((*dai).dev, c"Failed to add sdw stream: %d\n".as_ptr(), ret);
        return ret;
    }

    cs42l42_src_config((*dai).component, params_rate(params));

    0
}

unsafe extern "C" fn cs42l42_sdw_dai_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let cs42l42 = snd_soc_component_get_drvdata((*dai).component) as *mut cs42l42_private;

    dev_dbg((*dai).dev, c"dai_prepare: sclk=%u rate=%u\n".as_ptr(), (*cs42l42).sclk, (*cs42l42).sample_rate);

    if (*cs42l42).sclk == 0 || (*cs42l42).sample_rate == 0 {
        return -EINVAL;
    }

    /*
     * At this point we know the sample rate from hw_params, and the SWIRE_CLK from bus_config()
     * callback. This could only fail if the ACPI or machine driver are misconfigured to allow
     * an unsupported SWIRE_CLK and sample_rate combination.
     */

    cs42l42_pll_config((*dai).component, (*cs42l42).sclk, (*cs42l42).sample_rate)
}

unsafe extern "C" fn cs42l42_sdw_dai_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let cs42l42 = snd_soc_component_get_drvdata((*dai).component) as *mut cs42l42_private;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);

    sdw_stream_remove_slave((*cs42l42).sdw_peripheral, sdw_stream);
    (*cs42l42).sample_rate = 0;

    0
}

unsafe extern "C" fn cs42l42_sdw_port_prep(
    slave: *mut sdw_slave,
    prepare_ch: *mut sdw_prepare_ch,
    state: sdw_port_prep_ops,
) -> c_int {
    let cs42l42 = dev_get_drvdata(&(*slave).dev) as *mut cs42l42_private;
    let pdn_mask: c_uint;

    if (*prepare_ch).num == CS42L42_SDW_PLAYBACK_PORT {
        pdn_mask = CS42L42_HP_PDN_MASK;
    } else {
        pdn_mask = CS42L42_ADC_PDN_MASK;
    }

    if state == SDW_OPS_PORT_PRE_PREP {
        dev_dbg((*cs42l42).dev, c"Prep Port pdn_mask:%x\n".as_ptr(), pdn_mask);
        regmap_clear_bits((*cs42l42).regmap, CS42L42_PWR_CTL1, pdn_mask);
        usleep_range(CS42L42_HP_ADC_EN_TIME_US, CS42L42_HP_ADC_EN_TIME_US + 1000);
    } else if state == SDW_OPS_PORT_POST_DEPREP {
        dev_dbg((*cs42l42).dev, c"Deprep Port pdn_mask:%x\n".as_ptr(), pdn_mask);
        regmap_set_bits((*cs42l42).regmap, CS42L42_PWR_CTL1, pdn_mask);
    }

    0
}

unsafe extern "C" fn cs42l42_sdw_dai_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

unsafe extern "C" fn cs42l42_sdw_dai_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

static cs42l42_sdw_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(cs42l42_sdw_dai_startup),
    shutdown: Some(cs42l42_sdw_dai_shutdown),
    hw_params: Some(cs42l42_sdw_dai_hw_params),
    prepare: Some(cs42l42_sdw_dai_prepare),
    hw_free: Some(cs42l42_sdw_dai_hw_free),
    mute_stream: Some(cs42l42_mute_stream),
    set_stream: Some(cs42l42_sdw_dai_set_sdw_stream),
};

static mut cs42l42_sdw_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"cs42l42-sdw".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        /* Restrict which rates and formats are supported */
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 1,
        /* Restrict which rates and formats are supported */
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
    },
    symmetric_rate: 1,
    ops: &cs42l42_sdw_dai_ops,
};

unsafe fn cs42l42_sdw_poll_status(peripheral: *mut sdw_slave, mask: u8, match_: u8) -> c_int {
    let mut ret: c_int;
    let mut sdwret: c_int;

    loop {
        sdwret = sdw_read_no_pm(peripheral, CS42L42_SDW_MEM_ACCESS_STATUS);
        if (sdwret < 0) || (((sdwret as u8) & mask) == match_) {
            ret = 0;
            break;
        }
        /* read_poll_timeout(sdw_read_no_pm, ..., CS42L42_DELAYED_READ_POLL_US,
         * CS42L42_DELAYED_READ_TIMEOUT_US, false, peripheral,
         * CS42L42_SDW_MEM_ACCESS_STATUS)
         */
        ret = -EINVAL;
        break;
    }
    if ret == 0 {
        ret = sdwret;
    }

    if ret < 0 {
        dev_err(&(*peripheral).dev, c"MEM_ACCESS_STATUS & %#x for %#x fail: %d\n".as_ptr(), mask as c_uint, match_ as c_uint, ret);
    }

    ret
}

unsafe extern "C" fn cs42l42_sdw_read(context: *mut c_void, mut reg: c_uint, val: *mut c_uint) -> c_int {
    let peripheral = context as *mut sdw_slave;
    let data: u8;
    let mut ret: c_int;

    reg += CS42L42_SDW_ADDR_OFFSET;

    ret = cs42l42_sdw_poll_status(peripheral, CS42L42_SDW_CMD_IN_PROGRESS, 0);
    if ret < 0 {
        return ret;
    }

    ret = sdw_read_no_pm(peripheral, reg);
    if ret < 0 {
        dev_err(&(*peripheral).dev, c"Failed to issue read @0x%x: %d\n".as_ptr(), reg, ret);
        return ret;
    }

    data = ret as u8; /* possible non-delayed read value */
    ret = sdw_read_no_pm(peripheral, CS42L42_SDW_MEM_ACCESS_STATUS);
    if ret < 0 {
        dev_err(&(*peripheral).dev, c"Failed to read MEM_ACCESS_STATUS: %d\n".as_ptr(), ret);
        return ret;
    }

    /* If read was not delayed we already have the result */
    if (ret & CS42L42_SDW_LAST_LATE as c_int) == 0 {
        *val = data as c_uint;
        return 0;
    }

    /* Poll for delayed read completion */
    if (ret & CS42L42_SDW_RDATA_RDY as c_int) == 0 {
        ret = cs42l42_sdw_poll_status(peripheral, CS42L42_SDW_RDATA_RDY, CS42L42_SDW_RDATA_RDY);
        if ret < 0 {
            return ret;
        }
    }

    ret = sdw_read_no_pm(peripheral, CS42L42_SDW_MEM_READ_DATA);
    if ret < 0 {
        dev_err(&(*peripheral).dev, c"Failed to read READ_DATA: %d\n".as_ptr(), ret);
        return ret;
    }

    *val = ret as u8 as c_uint;

    0
}

unsafe extern "C" fn cs42l42_sdw_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    let peripheral = context as *mut sdw_slave;
    let ret: c_int;

    ret = cs42l42_sdw_poll_status(peripheral, CS42L42_SDW_CMD_IN_PROGRESS, 0);
    if ret < 0 {
        return ret;
    }

    sdw_write_no_pm(peripheral, reg + CS42L42_SDW_ADDR_OFFSET, val as u8)
}

/* Initialise cs42l42 using SoundWire - this is only called once, during initialisation */
unsafe fn cs42l42_sdw_init(peripheral: *mut sdw_slave) {
    let cs42l42 = dev_get_drvdata(&(*peripheral).dev) as *mut cs42l42_private;
    let mut ret: c_int;

    regcache_cache_only((*cs42l42).regmap, false);

    ret = cs42l42_init(cs42l42);
    if ret < 0 {
        regcache_cache_only((*cs42l42).regmap, true);
        goto_err(cs42l42);
        return;
    }

    /* Write out any cached changes that happened between probe and attach */
    ret = regcache_sync((*cs42l42).regmap);
    if ret < 0 {
        dev_warn((*cs42l42).dev, c"Failed to sync cache: %d\n".as_ptr(), ret);
    }

    /* Disable internal logic that makes clock-stop conditional */
    regmap_clear_bits((*cs42l42).regmap, CS42L42_PWR_CTL3, CS42L42_SW_CLK_STP_STAT_SEL_MASK);

    goto_err(cs42l42);
}

unsafe fn goto_err(cs42l42: *mut cs42l42_private) {
    /* This cancels the pm_runtime_get_noresume() call from cs42l42_sdw_probe(). */
    pm_runtime_put_autosuspend((*cs42l42).dev);
}

unsafe extern "C" fn cs42l42_sdw_read_prop(peripheral: *mut sdw_slave) -> c_int {
    let cs42l42 = dev_get_drvdata(&(*peripheral).dev) as *mut cs42l42_private;
    let prop = &mut (*peripheral).prop as *mut sdw_slave_prop;
    let ports: *mut sdw_dpn_prop;

    ports = devm_kcalloc((*cs42l42).dev, 2, core::mem::size_of::<sdw_dpn_prop>(), GFP_KERNEL) as *mut sdw_dpn_prop;
    if ports.is_null() {
        return -ENOMEM;
    }

    (*prop).source_ports = BIT(CS42L42_SDW_CAPTURE_PORT);
    (*prop).sink_ports = BIT(CS42L42_SDW_PLAYBACK_PORT);
    (*prop).quirks = SDW_SLAVE_QUIRKS_INVALID_INITIAL_PARITY;
    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;

    /* DP1 - capture */
    (*ports.add(0)).num = CS42L42_SDW_CAPTURE_PORT;
    (*ports.add(0)).type_ = SDW_DPN_FULL;
    (*ports.add(0)).ch_prep_timeout = 10;
    (*prop).src_dpn_prop = ports.add(0);

    /* DP2 - playback */
    (*ports.add(1)).num = CS42L42_SDW_PLAYBACK_PORT;
    (*ports.add(1)).type_ = SDW_DPN_FULL;
    (*ports.add(1)).ch_prep_timeout = 10;
    (*prop).sink_dpn_prop = ports.add(1);

    0
}

unsafe extern "C" fn cs42l42_sdw_update_status(peripheral: *mut sdw_slave, status: sdw_slave_status) -> c_int {
    let cs42l42 = dev_get_drvdata(&(*peripheral).dev) as *mut cs42l42_private;

    match status {
        SDW_SLAVE_ATTACHED => {
            dev_dbg((*cs42l42).dev, c"ATTACHED\n".as_ptr());

            /*
             * The SoundWire core can report stale ATTACH notifications
             * if we hard-reset CS42L42 in probe() but it had already been
             * enumerated. Reject the ATTACH if we haven't yet seen an
             * UNATTACH report for the device being in reset.
             */
            if (*cs42l42).sdw_waiting_first_unattach {
                return 0;
            }

            /*
             * Initialise codec, this only needs to be done once.
             * When resuming from suspend, resume callback will handle re-init of codec,
             * using regcache_sync().
             */
            if !(*cs42l42).init_done {
                cs42l42_sdw_init(peripheral);
            }
        }
        SDW_SLAVE_UNATTACHED => {
            dev_dbg((*cs42l42).dev, c"UNATTACHED\n".as_ptr());

            if (*cs42l42).sdw_waiting_first_unattach {
                /*
                 * SoundWire core has seen that CS42L42 is not on
                 * the bus so release RESET and wait for ATTACH.
                 */
                (*cs42l42).sdw_waiting_first_unattach = false;
                gpiod_set_value_cansleep((*cs42l42).reset_gpio, 1);
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn cs42l42_sdw_bus_config(peripheral: *mut sdw_slave, params: *mut sdw_bus_params) -> c_int {
    let cs42l42 = dev_get_drvdata(&(*peripheral).dev) as *mut cs42l42_private;
    let new_sclk: c_uint = (*params).curr_dr_freq / 2;

    /* The cs42l42 cannot support a glitchless SWIRE_CLK change. */
    if (new_sclk != (*cs42l42).sclk) && (*cs42l42).stream_use {
        dev_warn((*cs42l42).dev, c"Rejected SCLK change while audio active\n".as_ptr());
        return -EBUSY;
    }

    (*cs42l42).sclk = new_sclk;

    dev_dbg((*cs42l42).dev, c"bus_config: sclk=%u c=%u r=%u\n".as_ptr(), (*cs42l42).sclk, (*params).col, (*params).row);

    0
}

static cs42l42_sdw_ops: sdw_slave_ops = sdw_slave_ops {
    /* No interrupt callback because only hardware INT is supported for Jack Detect in the CS42L42 */
    read_prop: Some(cs42l42_sdw_read_prop),
    update_status: Some(cs42l42_sdw_update_status),
    bus_config: Some(cs42l42_sdw_bus_config),
    port_prep: Some(cs42l42_sdw_port_prep),
};

unsafe extern "C" fn cs42l42_sdw_runtime_suspend(dev: *mut device) -> c_int {
    let cs42l42 = dev_get_drvdata(dev) as *mut cs42l42_private;

    dev_dbg(dev, c"Runtime suspend\n".as_ptr());

    if !(*cs42l42).init_done {
        return 0;
    }

    /* The host controller could suspend, which would mean no register access */
    regcache_cache_only((*cs42l42).regmap, true);

    0
}

static cs42l42_soft_reboot_seq: [reg_sequence; 1] = [
    reg_sequence { reg: unsafe { CS42L42_SOFT_RESET_REBOOT }, def: 0x1e, delay_us: 0 },
];

unsafe fn cs42l42_sdw_handle_unattach(cs42l42: *mut cs42l42_private) -> c_int {
    let peripheral = (*cs42l42).sdw_peripheral;
    let ret: c_int;

    if !(*peripheral).unattach_request {
        return 0;
    }

    /* Cannot access registers until master re-attaches. */
    dev_dbg(&(*peripheral).dev, c"Wait for initialization_complete\n".as_ptr());
    ret = sdw_slave_wait_for_init(peripheral, 5000);
    if ret != 0 {
        return ret;
    }

    /*
     * After a bus reset there must be a reconfiguration reset to
     * reinitialize the internal state of CS42L42.
     */
    regmap_multi_reg_write_bypassed(
        (*cs42l42).regmap,
        cs42l42_soft_reboot_seq.as_ptr(),
        cs42l42_soft_reboot_seq.len() as c_int,
    );
    usleep_range(CS42L42_BOOT_TIME_US, CS42L42_BOOT_TIME_US * 2);
    regcache_mark_dirty((*cs42l42).regmap);

    0
}

unsafe extern "C" fn cs42l42_sdw_runtime_resume(dev: *mut device) -> c_int {
    static ts_dbnce_ms: [c_uint; 8] = [0, 125, 250, 500, 750, 1000, 1250, 1500];
    let cs42l42 = dev_get_drvdata(dev) as *mut cs42l42_private;
    let dbnce: c_uint;
    let mut ret: c_int;

    dev_dbg(dev, c"Runtime resume\n".as_ptr());

    if !(*cs42l42).init_done {
        return 0;
    }

    ret = cs42l42_sdw_handle_unattach(cs42l42);
    if ret < 0 {
        return ret;
    } else if ret > 0 {
        dbnce = core::cmp::max((*cs42l42).ts_dbnc_rise, (*cs42l42).ts_dbnc_fall);

        if dbnce > 0 {
            msleep(ts_dbnce_ms[dbnce as usize]);
        }
    }

    regcache_cache_only((*cs42l42).regmap, false);

    /* Sync LATCH_TO_VP first so the VP domain registers sync correctly */
    ret = regcache_sync_region((*cs42l42).regmap, CS42L42_MIC_DET_CTL1, CS42L42_MIC_DET_CTL1);
    if ret != 0 {
        regcache_cache_only((*cs42l42).regmap, true);
        regcache_mark_dirty((*cs42l42).regmap);
        return ret;
    }

    ret = regcache_sync((*cs42l42).regmap);
    if ret != 0 {
        regcache_cache_only((*cs42l42).regmap, true);
        regcache_mark_dirty((*cs42l42).regmap);
        return ret;
    }

    0
}

unsafe extern "C" fn cs42l42_sdw_resume(dev: *mut device) -> c_int {
    let cs42l42 = dev_get_drvdata(dev) as *mut cs42l42_private;
    let mut ret: c_int;

    dev_dbg(dev, c"System resume\n".as_ptr());

    /* Power-up so it can re-enumerate */
    ret = cs42l42_resume(dev);
    if ret != 0 {
        return ret;
    }

    /* Wait for re-attach */
    ret = cs42l42_sdw_handle_unattach(cs42l42);
    if ret < 0 {
        return ret;
    }

    cs42l42_resume_restore(dev);

    0
}

unsafe extern "C" fn cs42l42_sdw_probe(peripheral: *mut sdw_slave, id: *const sdw_device_id) -> c_int {
    let component_drv: *mut snd_soc_component_driver;
    let dev = &mut (*peripheral).dev as *mut device;
    let cs42l42: *mut cs42l42_private;
    let regmap_conf: *mut regmap_config;
    let regmap: *mut regmap;
    let mut irq: c_int;
    let ret: c_int;

    cs42l42 = devm_kzalloc(dev, core::mem::size_of::<cs42l42_private>(), GFP_KERNEL) as *mut cs42l42_private;
    if cs42l42.is_null() {
        return -ENOMEM;
    }

    if has_acpi_companion(dev) {
        irq = acpi_dev_gpio_irq_get(ACPI_COMPANION(dev), 0);
    } else {
        /* dev->of_node */
        irq = of_irq_get(ptr::null_mut(), 0);
    }

    if irq == -ENOENT {
        irq = 0;
    } else if irq < 0 {
        return dev_err_probe(dev, irq, c"Failed to get IRQ\n".as_ptr());
    }

    regmap_conf = devm_kmemdup(
        dev,
        &cs42l42_regmap as *const regmap_config as *const c_void,
        core::mem::size_of_val(&cs42l42_regmap),
        GFP_KERNEL,
    ) as *mut regmap_config;
    if regmap_conf.is_null() {
        return -ENOMEM;
    }
    (*regmap_conf).reg_bits = 16;
    (*regmap_conf).num_ranges = 0;
    (*regmap_conf).reg_read = Some(cs42l42_sdw_read);
    (*regmap_conf).reg_write = Some(cs42l42_sdw_write);

    regmap = devm_regmap_init(dev, ptr::null(), peripheral as *mut c_void, regmap_conf);
    if IS_ERR(regmap as *const c_void) {
        return dev_err_probe(dev, PTR_ERR(regmap as *const c_void), c"Failed to allocate register map\n".as_ptr());
    }

    /* Start in cache-only until device is enumerated */
    regcache_cache_only(regmap, true);

    component_drv = devm_kmemdup(
        dev,
        &cs42l42_soc_component as *const snd_soc_component_driver as *const c_void,
        core::mem::size_of_val(&cs42l42_soc_component),
        GFP_KERNEL,
    ) as *mut snd_soc_component_driver;
    if component_drv.is_null() {
        return -ENOMEM;
    }

    (*component_drv).dapm_routes = cs42l42_sdw_audio_map.as_ptr();
    (*component_drv).num_dapm_routes = cs42l42_sdw_audio_map.len() as c_uint;

    (*cs42l42).dev = dev;
    (*cs42l42).regmap = regmap;
    (*cs42l42).sdw_peripheral = peripheral;
    (*cs42l42).irq = irq;
    (*cs42l42).devid = CS42L42_CHIP_ID;

    /*
     * pm_runtime is needed to control bus manager suspend, and to
     * recover from an unattach_request when the manager suspends.
     */
    pm_runtime_set_autosuspend_delay((*cs42l42).dev, 3000);
    pm_runtime_use_autosuspend((*cs42l42).dev);
    pm_runtime_mark_last_busy((*cs42l42).dev);
    pm_runtime_set_active((*cs42l42).dev);
    pm_runtime_get_noresume((*cs42l42).dev);
    pm_runtime_enable((*cs42l42).dev);

    ret = cs42l42_common_probe(cs42l42, component_drv, &raw mut cs42l42_sdw_dai);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn cs42l42_sdw_remove(peripheral: *mut sdw_slave) {
    let cs42l42 = dev_get_drvdata(&(*peripheral).dev) as *mut cs42l42_private;

    cs42l42_common_remove(cs42l42);
    pm_runtime_disable((*cs42l42).dev);
}

/* SYSTEM_SLEEP_PM_OPS(cs42l42_suspend, cs42l42_sdw_resume)
 * RUNTIME_PM_OPS(cs42l42_sdw_runtime_suspend, cs42l42_sdw_runtime_resume, NULL)
 */
static cs42l42_sdw_pm: dev_pm_ops = dev_pm_ops { _private: [] };

/* SDW_SLAVE_ENTRY(0x01FA, 0x4242, 0), {} */
static cs42l42_sdw_id: [sdw_device_id; 2] = [
    sdw_device_id { _private: [] },
    sdw_device_id { _private: [] },
];
/* MODULE_DEVICE_TABLE(sdw, cs42l42_sdw_id); */

static mut cs42l42_sdw_driver: sdw_driver = sdw_driver {
    driver: driver_private {
        name: c"cs42l42-sdw".as_ptr(),
        pm: &cs42l42_sdw_pm,
    },
    probe: Some(cs42l42_sdw_probe),
    remove: Some(cs42l42_sdw_remove),
    ops: &cs42l42_sdw_ops,
    id_table: cs42l42_sdw_id.as_ptr(),
};

/* module_sdw_driver(cs42l42_sdw_driver);
 *
 * MODULE_DESCRIPTION("ASoC CS42L42 SoundWire driver");
 * MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
 * MODULE_LICENSE("GPL");
 * MODULE_IMPORT_NS("SND_SOC_CS42L42_CORE");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
