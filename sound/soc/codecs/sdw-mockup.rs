// SPDX-License-Identifier: GPL-2.0-only
//
// sdw-mockup.c -- a mockup SoundWire codec for tests where only the host
// drives the bus.
//
// Copyright(c) 2021 Intel Corporation
//
//

// C include dependencies translated as external Rust dependencies:
// <linux/device.h>, <linux/module.h>, <linux/soundwire/sdw.h>,
// <linux/soundwire/sdw_type.h>, <linux/soundwire/sdw_registers.h>,
// <sound/core.h>, <sound/pcm.h>, <sound/pcm_params.h>, <sound/sdw.h>,
// <sound/soc.h>

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SDW_DPN_FULL: c_int = 0;

type c_uint = u32;
type u32_t = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct sdw_stream_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_dpn_prop {
    pub num: c_uint,
    pub type_: c_int,
    pub simple_ch_prep_sm: bool,
}

#[repr(C)]
pub struct sdw_slave_prop {
    pub paging_support: bool,
    pub source_ports: u32_t,
    pub sink_ports: u32_t,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub simple_clk_stop_capable: bool,
    pub wake_capable: c_int,
}

#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub prop: sdw_slave_prop,
    pub is_mockup_device: bool,
}

#[repr(C)]
pub struct sdw_bus_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave_intr_status {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_device_id {
    pub mfg_id: c_uint,
    pub part_id: c_uint,
    pub class_id: c_uint,
    pub unique_id: c_uint,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
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
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
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
pub struct sdw_slave_ops {
    pub read_prop: Option<unsafe extern "C" fn(*mut sdw_slave) -> c_int>,
    pub interrupt_callback:
        Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_slave_intr_status) -> c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut sdw_slave, sdw_slave_status) -> c_int>,
    pub bus_config: Option<unsafe extern "C" fn(*mut sdw_slave, *mut sdw_bus_params) -> c_int>,
}

pub type sdw_slave_status = c_int;

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
}

#[repr(C)]
pub struct sdw_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut sdw_slave, *const sdw_device_id) -> c_int>,
    pub ops: *const sdw_slave_ops,
    pub id_table: *const sdw_device_id,
}

#[repr(C)]
pub struct sdw_mockup_priv {
    pub slave: *mut sdw_slave,
}

unsafe extern "C" {
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_void,
    );
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut c_void;
    fn snd_sdw_params_to_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        stream_config: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
    );
    fn sdw_stream_add_slave(
        slave: *mut sdw_slave,
        stream_config: *mut sdw_stream_config,
        port_config: *mut sdw_port_config,
        num_ports: c_uint,
        sdw_stream: *mut sdw_stream_runtime,
    ) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, sdw_stream: *mut sdw_stream_runtime);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const fn BIT(n: c_uint) -> u32_t {
    1u32 << n
}

fn hweight32(value: u32_t) -> c_int {
    value.count_ones() as c_int
}

unsafe extern "C" fn sdw_mockup_component_probe(_component: *mut snd_soc_component) -> c_int {
    0
}

unsafe extern "C" fn sdw_mockup_component_remove(_component: *mut snd_soc_component) {}

static snd_soc_sdw_mockup_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(sdw_mockup_component_probe),
    remove: Some(sdw_mockup_component_remove),
    endianness: 1,
};

unsafe extern "C" fn sdw_mockup_set_sdw_stream(
    dai: *mut snd_soc_dai,
    sdw_stream: *mut c_void,
    direction: c_int,
) -> c_int {
    unsafe {
        snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    }

    0
}

unsafe extern "C" fn sdw_mockup_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    unsafe {
        snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
    }
}

unsafe extern "C" fn sdw_mockup_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let sdw_mockup: *mut sdw_mockup_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut sdw_mockup_priv };
    let mut stream_config: sdw_stream_config = unsafe { core::mem::zeroed() };
    let mut port_config: sdw_port_config = unsafe { core::mem::zeroed() };
    let sdw_stream: *mut sdw_stream_runtime =
        unsafe { snd_soc_dai_get_dma_data(dai, substream) as *mut sdw_stream_runtime };
    let ret: c_int;

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    if unsafe { (*sdw_mockup).slave.is_null() } {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    unsafe {
        snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);
    }

    if unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK } {
        port_config.num = 1;
    } else {
        port_config.num = 8;
    }

    ret = unsafe {
        sdw_stream_add_slave(
            (*sdw_mockup).slave,
            &mut stream_config,
            &mut port_config,
            1,
            sdw_stream,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err((*dai).dev, c"Unable to configure port\n".as_ptr());
        }
    }

    ret
}

unsafe extern "C" fn sdw_mockup_pcm_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let sdw_mockup: *mut sdw_mockup_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut sdw_mockup_priv };
    let sdw_stream: *mut sdw_stream_runtime =
        unsafe { snd_soc_dai_get_dma_data(dai, substream) as *mut sdw_stream_runtime };

    if unsafe { (*sdw_mockup).slave.is_null() } {
        return -EINVAL;
    }

    unsafe {
        sdw_stream_remove_slave((*sdw_mockup).slave, sdw_stream);
    }
    0
}

static sdw_mockup_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(sdw_mockup_pcm_hw_params),
    hw_free: Some(sdw_mockup_pcm_hw_free),
    set_stream: Some(sdw_mockup_set_sdw_stream),
    shutdown: Some(sdw_mockup_shutdown),
};

static mut sdw_mockup_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"sdw-mockup-aif1".as_ptr(),
    id: 1,
    playback: snd_soc_pcm_stream {
        stream_name: c"DP1 Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"DP8 Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
    },
    ops: &sdw_mockup_ops,
}];

unsafe extern "C" fn sdw_mockup_update_status(
    _slave: *mut sdw_slave,
    _status: sdw_slave_status,
) -> c_int {
    0
}

unsafe extern "C" fn sdw_mockup_read_prop(slave: *mut sdw_slave) -> c_int {
    let prop: *mut sdw_slave_prop = unsafe { &mut (*slave).prop };
    let mut nval: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut bit: u32_t;
    let mut addr: c_ulong;
    let mut dpn: *mut sdw_dpn_prop;

    unsafe {
        (*prop).paging_support = false;
    }

    /*
     * first we need to allocate memory for set bits in port lists
     * the port allocation is completely arbitrary:
     * DP0 is not supported
     * DP1 is sink
     * DP8 is source
     */
    unsafe {
        (*prop).source_ports = BIT(8);
        (*prop).sink_ports = BIT(1);
    }

    nval = hweight32(unsafe { (*prop).source_ports });
    unsafe {
        (*prop).src_dpn_prop = devm_kcalloc(
            &mut (*slave).dev,
            nval as usize,
            size_of::<sdw_dpn_prop>(),
            GFP_KERNEL,
        ) as *mut sdw_dpn_prop;
    }
    if unsafe { (*prop).src_dpn_prop.is_null() } {
        return -ENOMEM;
    }

    i = 0;
    dpn = unsafe { (*prop).src_dpn_prop };
    addr = unsafe { (*prop).source_ports as c_ulong };
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            unsafe {
                (*dpn.offset(i as isize)).num = bit;
                (*dpn.offset(i as isize)).type_ = SDW_DPN_FULL;
                (*dpn.offset(i as isize)).simple_ch_prep_sm = true;
            }
            i += 1;
        }
        bit += 1;
    }

    /* do this again for sink now */
    nval = hweight32(unsafe { (*prop).sink_ports });
    unsafe {
        (*prop).sink_dpn_prop = devm_kcalloc(
            &mut (*slave).dev,
            nval as usize,
            size_of::<sdw_dpn_prop>(),
            GFP_KERNEL,
        ) as *mut sdw_dpn_prop;
    }
    if unsafe { (*prop).sink_dpn_prop.is_null() } {
        return -ENOMEM;
    }

    j = 0;
    dpn = unsafe { (*prop).sink_dpn_prop };
    addr = unsafe { (*prop).sink_ports as c_ulong };
    bit = 0;
    while bit < 32 {
        if (addr & (1 as c_ulong).wrapping_shl(bit)) != 0 {
            unsafe {
                (*dpn.offset(j as isize)).num = bit;
                (*dpn.offset(j as isize)).type_ = SDW_DPN_FULL;
                (*dpn.offset(j as isize)).simple_ch_prep_sm = true;
            }
            j += 1;
        }
        bit += 1;
    }

    unsafe {
        (*prop).simple_clk_stop_capable = true;
    }

    /* wake-up event */
    unsafe {
        (*prop).wake_capable = 0;
    }

    0
}

unsafe extern "C" fn sdw_mockup_bus_config(
    _slave: *mut sdw_slave,
    _params: *mut sdw_bus_params,
) -> c_int {
    0
}

unsafe extern "C" fn sdw_mockup_interrupt_callback(
    _slave: *mut sdw_slave,
    _status: *mut sdw_slave_intr_status,
) -> c_int {
    0
}

static sdw_mockup_slave_ops: sdw_slave_ops = sdw_slave_ops {
    read_prop: Some(sdw_mockup_read_prop),
    interrupt_callback: Some(sdw_mockup_interrupt_callback),
    update_status: Some(sdw_mockup_update_status),
    bus_config: Some(sdw_mockup_bus_config),
};

unsafe extern "C" fn sdw_mockup_sdw_probe(
    slave: *mut sdw_slave,
    _id: *const sdw_device_id,
) -> c_int {
    let dev: *mut device = unsafe { &mut (*slave).dev };
    let sdw_mockup: *mut sdw_mockup_priv;
    let ret: c_int;

    sdw_mockup =
        unsafe { devm_kzalloc(dev, size_of::<sdw_mockup_priv>(), GFP_KERNEL) as *mut sdw_mockup_priv };
    if sdw_mockup.is_null() {
        return -ENOMEM;
    }

    unsafe {
        dev_set_drvdata(dev, sdw_mockup as *mut c_void);
        (*sdw_mockup).slave = slave;

        (*slave).is_mockup_device = true;

        ret = devm_snd_soc_register_component(
            dev,
            &snd_soc_sdw_mockup_component,
            sdw_mockup_dai.as_mut_ptr(),
            sdw_mockup_dai.len() as c_int,
        );
    }

    ret
}

/*
 * Intel reserved parts ID with the following mapping expected:
 * 0xAAAA: generic full-duplex codec
 * 0xAA55: headset codec (mock-up of RT711/RT5682) - full-duplex
 * 0x55AA: amplifier (mock-up of RT1308/Maxim 98373) - playback only with
 * IV feedback
 * 0x5555: mic codec (mock-up of RT715) - capture-only
 */
const fn SDW_SLAVE_ENTRY_EXT(
    mfg_id: c_uint,
    part_id: c_uint,
    class_id: c_uint,
    unique_id: c_uint,
    driver_data: c_ulong,
) -> sdw_device_id {
    sdw_device_id {
        mfg_id,
        part_id,
        class_id,
        unique_id,
        driver_data,
    }
}

static sdw_mockup_id: [sdw_device_id; 5] = [
    SDW_SLAVE_ENTRY_EXT(0x0105, 0xAAAA, 0x0, 0, 0),
    SDW_SLAVE_ENTRY_EXT(0x0105, 0xAA55, 0x0, 0, 0),
    SDW_SLAVE_ENTRY_EXT(0x0105, 0x55AA, 0x0, 0, 0),
    SDW_SLAVE_ENTRY_EXT(0x0105, 0x5555, 0x0, 0, 0),
    sdw_device_id {
        mfg_id: 0,
        part_id: 0,
        class_id: 0,
        unique_id: 0,
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(sdw, sdw_mockup_id);

static mut sdw_mockup_sdw_driver: sdw_driver = sdw_driver {
    driver: driver_private {
        name: c"sdw-mockup".as_ptr(),
    },
    probe: Some(sdw_mockup_sdw_probe),
    ops: &sdw_mockup_slave_ops,
    id_table: sdw_mockup_id.as_ptr(),
};

// module_sdw_driver(sdw_mockup_sdw_driver);

// MODULE_DESCRIPTION("ASoC SDW mockup codec driver");
// MODULE_AUTHOR("Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
