// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2022, Analog Devices Inc.

// Rust translation of soc/codecs/max98363.c.
// External Linux, ASoC, regmap, SoundWire, and MAX98363 header symbols are
// expected to be supplied by the surrounding kernel Rust bindings.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

extern "C" {
    static max98363_pm: dev_pm_ops;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: c_uint,
    ) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_sdw(slave: *mut sdw_slave, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;

    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_stream_add_slave(
        slave: *mut sdw_slave,
        stream_config: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
        num_ports: c_int,
        stream: *mut sdw_stream_runtime,
    ) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut sdw_stream_runtime;
    fn snd_soc_dai_dma_data_set(
        dai: *mut snd_soc_dai,
        direction: c_int,
        sdw_stream: *mut c_void,
    );
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;

    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn hweight32(w: c_uint) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub scp_int1_mask: c_uint,
    pub sink_ports: c_ulong,
    pub paging_support: bool,
    pub clk_stop_timeout: c_uint,
    pub simple_clk_stop_capable: bool,
    pub clock_reg_supported: bool,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_uint,
    pub simple_ch_prep_sm: bool,
    pub ch_prep_timeout: c_uint,
}

#[repr(C)]
pub struct sdw_stream_config {
    pub frame_rate: c_int,
    pub bps: c_int,
    pub direction: sdw_data_direction,
    pub ch_count: c_uint,
}

#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
    pub ch_mask: c_uint,
}

pub type sdw_data_direction = c_uint;
pub type sdw_slave_status = c_uint;

#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub class_id: c_uint,
}

#[repr(C)]
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub update_status:
        Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
}

#[repr(C)]
pub struct sdw_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: sdw_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub hw_free:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
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
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct max98363_priv {
    pub regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub hw_init: bool,
    pub first_hw_init: bool,
}

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SDW_SCP_INT1_BUS_CLASH: c_uint = 1 << 0;
const SDW_SCP_INT1_PARITY: c_uint = 1 << 1;
const SDW_DPN_FULL: c_uint = 0;
const SDW_DATA_DIR_RX: sdw_data_direction = 0;
const SDW_SLAVE_UNATTACHED: sdw_slave_status = 0;
const SDW_SLAVE_ATTACHED: sdw_slave_status = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 1;
const SND_SOC_NOPM: c_int = 0;

const MAX98363_R2001_INTR_RAW: c_uint = 0x2001;
const MAX98363_R2003_INTR_STATE: c_uint = 0x2003;
const MAX98363_R2005_INTR_FALG: c_uint = 0x2005;
const MAX98363_R2007_INTR_EN: c_uint = 0x2007;
const MAX98363_R2009_INTR_CLR: c_uint = 0x2009;
const MAX98363_R2021_ERR_MON_CTRL: c_uint = 0x2021;
const MAX98363_R2022_SPK_MON_THRESH: c_uint = 0x2022;
const MAX98363_R2023_SPK_MON_DURATION: c_uint = 0x2023;
const MAX98363_R2030_TONE_GEN_CFG: c_uint = 0x2030;
const MAX98363_R203F_TONE_GEN_EN: c_uint = 0x203f;
const MAX98363_R2040_AMP_VOL: c_uint = 0x2040;
const MAX98363_R2041_AMP_GAIN: c_uint = 0x2041;
const MAX98363_R2042_DSP_CFG: c_uint = 0x2042;
const MAX98363_R21FF_REV_ID: c_uint = 0x21ff;
const MAX98363_AMP_DSP_CFG_RMP_SHIFT: c_uint = 0;
const MAX98363_CLOCK_MON_SHIFT: c_uint = 0;
const MAX98363_SPKMON_SHIFT: c_uint = 0;

const fn bit(n: c_uint) -> c_ulong {
    1 as c_ulong << n
}

const fn genmask(high: c_int, low: c_int) -> c_uint {
    ((!0u32) << low) & ((!0u32) >> (31 - high))
}

static max98363_reg: [reg_default; 8] = [
    reg_default {
        reg: MAX98363_R2021_ERR_MON_CTRL,
        def: 0x0,
    },
    reg_default {
        reg: MAX98363_R2022_SPK_MON_THRESH,
        def: 0x0,
    },
    reg_default {
        reg: MAX98363_R2023_SPK_MON_DURATION,
        def: 0x0,
    },
    reg_default {
        reg: MAX98363_R2030_TONE_GEN_CFG,
        def: 0x0,
    },
    reg_default {
        reg: MAX98363_R203F_TONE_GEN_EN,
        def: 0x0,
    },
    reg_default {
        reg: MAX98363_R2040_AMP_VOL,
        def: 0x0,
    },
    reg_default {
        reg: MAX98363_R2041_AMP_GAIN,
        def: 0x5,
    },
    reg_default {
        reg: MAX98363_R2042_DSP_CFG,
        def: 0x0,
    },
];

unsafe extern "C" fn max98363_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MAX98363_R2001_INTR_RAW
        | MAX98363_R2003_INTR_STATE
        | MAX98363_R2005_INTR_FALG
        | MAX98363_R2007_INTR_EN
        | MAX98363_R2009_INTR_CLR
        | MAX98363_R2021_ERR_MON_CTRL..=MAX98363_R2023_SPK_MON_DURATION
        | MAX98363_R2030_TONE_GEN_CFG
        | MAX98363_R203F_TONE_GEN_EN
        | MAX98363_R2040_AMP_VOL
        | MAX98363_R2041_AMP_GAIN
        | MAX98363_R2042_DSP_CFG
        | MAX98363_R21FF_REV_ID => true,
        _ => false,
    }
}

unsafe extern "C" fn max98363_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MAX98363_R2001_INTR_RAW
        | MAX98363_R2003_INTR_STATE
        | MAX98363_R2005_INTR_FALG
        | MAX98363_R2007_INTR_EN
        | MAX98363_R2009_INTR_CLR
        | MAX98363_R21FF_REV_ID => true,
        _ => false,
    }
}

static max98363_sdw_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 8,
    max_register: MAX98363_R21FF_REV_ID,
    reg_defaults: max98363_reg.as_ptr(),
    num_reg_defaults: max98363_reg.len() as c_uint,
    readable_reg: Some(max98363_readable_register),
    volatile_reg: Some(max98363_volatile_reg),
    cache_type: REGCACHE_RBTREE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn max98363_suspend(dev: *mut device) -> c_int {
    let max98363 = dev_get_drvdata(dev) as *mut max98363_priv;

    regcache_cache_only((*max98363).regmap, true);
    regcache_mark_dirty((*max98363).regmap);

    0
}

const MAX98363_PROBE_TIMEOUT: c_int = 5000;

unsafe extern "C" fn max98363_resume(dev: *mut device) -> c_int {
    let slave = dev_to_sdw_dev(dev);
    let max98363 = dev_get_drvdata(dev) as *mut max98363_priv;
    let mut ret: c_int;

    if !(*max98363).first_hw_init {
        return 0;
    }

    ret = sdw_slave_wait_for_init(slave, MAX98363_PROBE_TIMEOUT);
    if ret != 0 {
        return ret;
    }

    regcache_cache_only((*max98363).regmap, false);
    ret = regcache_sync((*max98363).regmap);
    if ret != 0 {
        regcache_cache_only((*max98363).regmap, true);
        regcache_mark_dirty((*max98363).regmap);
        return ret;
    }

    0
}

// C used DEFINE_RUNTIME_DEV_PM_OPS(max98363_pm, max98363_suspend, max98363_resume, NULL).

unsafe fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave {
    dev as *mut sdw_slave
}

unsafe extern "C" fn max98363_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop = &mut (*slave).prop as *mut sdw_slave_prop;
    let mut i: c_int;
    let mut bit_idx: c_uint;
    let mut addr: c_ulong;
    let dpn: *mut sdw_dpn_prop;
    let nval: c_int;

    (*prop).scp_int1_mask = SDW_SCP_INT1_BUS_CLASH | SDW_SCP_INT1_PARITY;

    /* BITMAP: 00000010  Dataport 1 is active */
    (*prop).sink_ports = bit(1);
    (*prop).paging_support = true;
    (*prop).clk_stop_timeout = 20;
    (*prop).simple_clk_stop_capable = true;
    (*prop).clock_reg_supported = true;

    nval = hweight32((*prop).sink_ports as c_uint);
    (*prop).sink_dpn_prop = devm_kcalloc(
        &mut (*slave).dev,
        nval as usize,
        core::mem::size_of::<sdw_dpn_prop>(),
        GFP_KERNEL,
    ) as *mut sdw_dpn_prop;
    if (*prop).sink_dpn_prop.is_null() {
        return -ENOMEM;
    }

    i = 0;
    dpn = (*prop).sink_dpn_prop;
    addr = (*prop).sink_ports;
    bit_idx = 0;
    while bit_idx < 32 {
        if (addr & bit(bit_idx)) != 0 {
            (*dpn.add(i as usize)).num = bit_idx;
            (*dpn.add(i as usize)).type_ = SDW_DPN_FULL;
            (*dpn.add(i as usize)).simple_ch_prep_sm = true;
            (*dpn.add(i as usize)).ch_prep_timeout = 10;
            i += 1;
        }
        bit_idx += 1;
    }

    0
}

unsafe extern "C" fn max98363_io_init(slave: *mut sdw_slave) -> c_int {
    let dev = &mut (*slave).dev as *mut device;
    let max98363 = dev_get_drvdata(dev) as *mut max98363_priv;
    let mut ret: c_int;
    let mut reg: c_int = 0;

    regcache_cache_only((*max98363).regmap, false);
    if (*max98363).first_hw_init {
        regcache_cache_bypass((*max98363).regmap, true);
    }

    /*
     * PM runtime status is marked as 'active' only when a Slave reports as Attached
     */
    if !(*max98363).first_hw_init {
        /* update count of parent 'active' children */
        pm_runtime_set_active(dev);
    }

    pm_runtime_get_noresume(dev);

    ret = regmap_read((*max98363).regmap, MAX98363_R21FF_REV_ID, &mut reg);
    if ret == 0 {
        dev_info(dev, b"Revision ID: %X\n\0".as_ptr() as *const c_char, reg);
    } else {
        goto_out(dev);
        return ret;
    }

    if (*max98363).first_hw_init {
        regcache_cache_bypass((*max98363).regmap, false);
        regcache_mark_dirty((*max98363).regmap);
    }

    (*max98363).first_hw_init = true;
    (*max98363).hw_init = true;

    goto_out(dev);
    ret
}

unsafe fn goto_out(dev: *mut device) {
    pm_runtime_put_autosuspend(dev);
}

const MAX98363_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const MAX98363_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

unsafe extern "C" fn max98363_sdw_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let max98363 = snd_soc_component_get_drvdata(component) as *mut max98363_priv;

    let mut stream_config = sdw_stream_config {
        frame_rate: 0,
        bps: 0,
        direction: 0,
        ch_count: 0,
    };
    let mut port_config = sdw_port_config { num: 0, ch_mask: 0 };
    let direction: sdw_data_direction;
    let stream: *mut sdw_stream_runtime;
    let runtime = (*substream).runtime;

    let ret: c_int;

    stream = snd_soc_dai_get_dma_data(dai, substream);

    if stream.is_null() {
        return -EINVAL;
    }

    if (*max98363).slave.is_null() {
        return -EINVAL;
    }

    if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
        return -EINVAL;
    }

    direction = SDW_DATA_DIR_RX;
    port_config.num = 1;

    stream_config.frame_rate = params_rate(params);
    stream_config.bps = snd_pcm_format_width(params_format(params));
    stream_config.direction = direction;
    stream_config.ch_count = 1;

    if stream_config.ch_count > (*runtime).hw.channels_max {
        stream_config.ch_count = (*runtime).hw.channels_max;
        dev_info(
            (*dai).dev,
            b"Number of channels: %d (requested: %d)\n\0".as_ptr() as *const c_char,
            stream_config.ch_count,
            params_channels(params),
        );
    }
    port_config.ch_mask = genmask(stream_config.ch_count as c_int - 1, 0);

    ret = sdw_stream_add_slave(
        (*max98363).slave,
        &mut stream_config,
        &mut port_config,
        1,
        stream,
    );
    if ret != 0 {
        dev_err(
            (*dai).dev,
            b"Unable to configure port\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    dev_dbg(
        (*component).dev,
        b"Format supported %d\0".as_ptr() as *const c_char,
        params_format(params),
    );

    0
}

unsafe extern "C" fn max98363_pcm_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let max98363 = snd_soc_component_get_drvdata(component) as *mut max98363_priv;
    let stream = snd_soc_dai_get_dma_data(dai, substream);

    if (*max98363).slave.is_null() {
        return -EINVAL;
    }

    sdw_stream_remove_slave((*max98363).slave, stream);

    0
}

unsafe extern "C" fn max98363_set_sdw_stream(
    dai: *mut snd_soc_dai,
    sdw_stream: *mut c_void,
    direction: c_int,
) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);

    0
}

static max98363_dai_sdw_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(max98363_sdw_dai_hw_params),
    hw_free: Some(max98363_pcm_hw_free),
    set_stream: Some(max98363_set_sdw_stream),
};

static mut max98363_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"max98363-aif1\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"HiFi Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 1,
        rates: MAX98363_RATES,
        formats: MAX98363_FORMATS,
    },
    ops: &max98363_dai_sdw_ops,
}];

unsafe extern "C" fn max98363_update_status(
    slave: *mut sdw_slave,
    status: sdw_slave_status,
) -> c_int {
    let max98363 = dev_get_drvdata(&mut (*slave).dev) as *mut max98363_priv;

    if status == SDW_SLAVE_UNATTACHED {
        (*max98363).hw_init = false;
    }

    /*
     * Perform initialization only if slave status is SDW_SLAVE_ATTACHED
     */
    if (*max98363).hw_init || status != SDW_SLAVE_ATTACHED {
        return 0;
    }

    /* perform I/O transfers required for Slave initialization */
    max98363_io_init(slave)
}

static max98363_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(max98363_read_prop),
    update_status: Some(max98363_update_status),
};

// C used DECLARE_TLV_DB_SCALE(max98363_digital_tlv, -6350, 50, 1).
static max98363_digital_tlv: [c_uint; 4] = [0, (-6350i32) as c_uint, 50, 1];

// C used DECLARE_TLV_DB_RANGE(max98363_spk_tlv, 0, 5, TLV_DB_SCALE_ITEM(-300, 300, 0)).
static max98363_spk_tlv: [c_uint; 5] = [0, 0, 5, (-300i32) as c_uint, 300];

static max98363_tone_cfg_text: [*const c_char; 15] = [
    b"Reserved\0".as_ptr() as *const c_char,
    b"0\0".as_ptr() as *const c_char,
    b"+FS/2\0".as_ptr() as *const c_char,
    b"-FS/2\0".as_ptr() as *const c_char,
    b"1KHz\0".as_ptr() as *const c_char,
    b"12KHz\0".as_ptr() as *const c_char,
    b"8KHz\0".as_ptr() as *const c_char,
    b"6KHz\0".as_ptr() as *const c_char,
    b"4KHz\0".as_ptr() as *const c_char,
    b"3KHz\0".as_ptr() as *const c_char,
    b"2KHz\0".as_ptr() as *const c_char,
    b"1.5KHz\0".as_ptr() as *const c_char,
    b"Reserved\0".as_ptr() as *const c_char,
    b"500Hz\0".as_ptr() as *const c_char,
    b"250Hz\0".as_ptr() as *const c_char,
];

// C used SOC_ENUM_SINGLE_DECL(max98363_tone_cfg_enum, MAX98363_R2030_TONE_GEN_CFG, 0,
// max98363_tone_cfg_text).
static max98363_tone_cfg_enum: soc_enum = soc_enum { _private: [] };

static max98363_spkmon_duration_text: [*const c_char; 16] = [
    b"8ms\0".as_ptr() as *const c_char,
    b"20ms\0".as_ptr() as *const c_char,
    b"40ms\0".as_ptr() as *const c_char,
    b"60ms\0".as_ptr() as *const c_char,
    b"80ms\0".as_ptr() as *const c_char,
    b"160ms\0".as_ptr() as *const c_char,
    b"240ms\0".as_ptr() as *const c_char,
    b"320ms\0".as_ptr() as *const c_char,
    b"400ms\0".as_ptr() as *const c_char,
    b"480ms\0".as_ptr() as *const c_char,
    b"560ms\0".as_ptr() as *const c_char,
    b"640ms\0".as_ptr() as *const c_char,
    b"720ms\0".as_ptr() as *const c_char,
    b"800ms\0".as_ptr() as *const c_char,
    b"880ms\0".as_ptr() as *const c_char,
    b"960ms\0".as_ptr() as *const c_char,
];

// C used SOC_ENUM_SINGLE_DECL(max98363_spkmon_duration_enum,
// MAX98363_R2023_SPK_MON_DURATION, 0, max98363_spkmon_duration_text).
static max98363_spkmon_duration_enum: soc_enum = soc_enum { _private: [] };

// C used SOC_SINGLE_TLV, SOC_SINGLE, and SOC_ENUM initializers here.
static max98363_snd_controls: [snd_kcontrol_new; 9] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

// C used SND_SOC_DAPM_AIF_IN("AIFIN", "HiFi Playback", 0, SND_SOC_NOPM, 0, 0)
// and SND_SOC_DAPM_OUTPUT("BE_OUT").
static max98363_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static max98363_audio_map: [snd_soc_dapm_route; 1] = [
    /* Plabyack */
    snd_soc_dapm_route {
        sink: b"BE_OUT\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AIFIN\0".as_ptr() as *const c_char,
    },
];

static soc_codec_dev_max98363: snd_soc_component_driver = snd_soc_component_driver {
    controls: max98363_snd_controls.as_ptr(),
    num_controls: max98363_snd_controls.len() as c_uint,
    dapm_widgets: max98363_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98363_dapm_widgets.len() as c_uint,
    dapm_routes: max98363_audio_map.as_ptr(),
    num_dapm_routes: max98363_audio_map.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn max98363_init(slave: *mut sdw_slave, regmap: *mut regmap) -> c_int {
    let max98363: *mut max98363_priv;
    let ret: c_int;
    let dev = &mut (*slave).dev as *mut device;

    /*  Allocate and assign private driver data structure  */
    max98363 = devm_kzalloc(dev, core::mem::size_of::<max98363_priv>(), GFP_KERNEL)
        as *mut max98363_priv;
    if max98363.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, max98363 as *mut c_void);
    (*max98363).regmap = regmap;
    (*max98363).slave = slave;

    regcache_cache_only((*max98363).regmap, true);

    (*max98363).hw_init = false;
    (*max98363).first_hw_init = false;

    /* codec registration  */
    ret = devm_snd_soc_register_component(
        dev,
        &soc_codec_dev_max98363,
        max98363_dai.as_mut_ptr(),
        max98363_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(
            dev,
            b"Failed to register codec: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);

    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);

    pm_runtime_enable(dev);

    /* important note: the device is NOT tagged as 'active' and will remain
     * 'suspended' until the hardware is enumerated/initialized. This is required
     * to make sure the ASoC framework use of pm_runtime_get_sync() does not silently
     * fail with -EACCESS because of race conditions between card creation and enumeration
     */
    0
}

unsafe extern "C" fn max98363_sdw_probe(
    slave: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let regmap: *mut regmap;

    /* Regmap Initialization */
    regmap = devm_regmap_init_sdw(slave, &max98363_sdw_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    max98363_init(slave, regmap)
}

static max98363_id: [sdw_device_id; 2] = [
    sdw_device_id {
        mfg_id: 0x019f,
        part_id: 0x8363,
        class_id: 0,
    },
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        class_id: 0,
    },
];
// C used MODULE_DEVICE_TABLE(sdw, max98363_id).

static mut max98363_sdw_driver: sdw_driver = sdw_driver {
    driver: sdw_driver_driver {
        name: b"max98363\0".as_ptr() as *const c_char,
        pm: unsafe { &max98363_pm },
    },
    probe: Some(max98363_sdw_probe),
    ops: &max98363_slave_ops,
    id_table: max98363_id.as_ptr(),
};

// C used module_sdw_driver(max98363_sdw_driver).
// C module metadata:
// MODULE_DESCRIPTION("ASoC MAX98363 driver SDW");
// MODULE_AUTHOR("Ryan Lee <ryans.lee@analog.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
