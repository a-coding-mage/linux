// SPDX-License-Identifier: GPL-2.0
//
// CS42L43 CODEC driver SoundWire handling
//
// Copyright (C) 2022-2023 Cirrus Logic, Inc. and
//                         Cirrus Logic International Semiconductor Ltd.

// Dependencies originally included from:
// linux/errno.h, linux/mfd/cs42l43.h, linux/mfd/cs42l43-regs.h,
// linux/module.h, linux/soundwire/sdw.h, sound/pcm.h, sound/sdw.h,
// sound/soc-component.h, sound/soc-dai.h, sound/soc.h, and "cs42l43.h".

pub const EINVAL: i32 = 22;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct cs42l43_codec {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
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
    pub id: i32,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
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
    pub num: i32,
}

unsafe extern "C" {
    pub fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut core::ffi::c_void;
    pub fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut sdw_stream_runtime;
    pub fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    pub fn snd_sdw_params_to_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        sconfig: *mut sdw_stream_config,
        pconfig: *mut sdw_port_config,
    );
    pub fn sdw_stream_add_slave(
        sdw: *mut sdw_slave,
        sconfig: *mut sdw_stream_config,
        pconfig: *mut sdw_port_config,
        num_ports: i32,
        sdw_stream: *mut sdw_stream_runtime,
    ) -> i32;
    pub fn sdw_stream_remove_slave(
        sdw: *mut sdw_slave,
        sdw_stream: *mut sdw_stream_runtime,
    ) -> i32;
    pub fn snd_soc_dai_dma_data_set(
        dai: *mut snd_soc_dai,
        direction: i32,
        data: *mut core::ffi::c_void,
    );
    pub fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs42l43_sdw_add_peripheral(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_0 =
        unsafe { snd_soc_component_get_drvdata((*dai).component) as *mut cs42l43_codec };
    let sdw_stream = unsafe { snd_soc_dai_get_dma_data(dai, substream) };
    let sdw = unsafe { dev_to_sdw_dev((*(*priv_0).dev).parent) };
    let mut sconfig: sdw_stream_config = unsafe { core::mem::zeroed() };
    let mut pconfig: sdw_port_config = unsafe { core::mem::zeroed() };
    let ret: i32;

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    unsafe { snd_sdw_params_to_config(substream, params, &mut sconfig, &mut pconfig) };
    pconfig.num = unsafe { (*dai).id };

    ret = unsafe { sdw_stream_add_slave(sdw, &mut sconfig, &mut pconfig, 1, sdw_stream) };
    if ret != 0 {
        const FAILED_TO_ADD_SDW_STREAM: &[u8] = b"Failed to add sdw stream: %d\n\0";

        unsafe {
            dev_err(
                (*priv_0).dev,
                FAILED_TO_ADD_SDW_STREAM.as_ptr() as *const core::ffi::c_char,
                ret,
            )
        };
        return ret;
    }

    0
}

// EXPORT_SYMBOL_NS_GPL(cs42l43_sdw_add_peripheral, "SND_SOC_CS42L43");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs42l43_sdw_remove_peripheral(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> i32 {
    let priv_0 =
        unsafe { snd_soc_component_get_drvdata((*dai).component) as *mut cs42l43_codec };
    let sdw_stream = unsafe { snd_soc_dai_get_dma_data(dai, substream) };
    let sdw = unsafe { dev_to_sdw_dev((*(*priv_0).dev).parent) };

    if sdw_stream.is_null() {
        return -EINVAL;
    }

    unsafe { sdw_stream_remove_slave(sdw, sdw_stream) }
}

// EXPORT_SYMBOL_NS_GPL(cs42l43_sdw_remove_peripheral, "SND_SOC_CS42L43");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs42l43_sdw_set_stream(
    dai: *mut snd_soc_dai,
    sdw_stream: *mut core::ffi::c_void,
    direction: i32,
) -> i32 {
    unsafe { snd_soc_dai_dma_data_set(dai, direction, sdw_stream) };

    0
}

// EXPORT_SYMBOL_NS_GPL(cs42l43_sdw_set_stream, "SND_SOC_CS42L43");

// MODULE_DESCRIPTION("CS42L43 CODEC SoundWire Driver");
// MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
