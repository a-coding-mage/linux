// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2024 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 * soc_sdw_rt_dmic - Helpers to handle Realtek SDW DMIC from generic machine driver
 */

use core::ffi::{c_char, c_int, c_void};

pub type gfp_t = c_uint;
pub type c_uint = u32;

pub const ENOMEM: c_int = 12;
pub const SDCA_FUNCTION_TYPE_SMART_MIC: c_int = 0;

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub components: *mut c_char,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub name_prefix: *mut c_char,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    pub id: sdw_slave_id,
    pub sdca_data: sdca_data,
}

#[repr(C)]
pub struct sdw_slave_id {
    pub part_id: c_int,
}

#[repr(C)]
pub struct sdca_data {
    pub num_functions: c_int,
    pub function: *mut sdca_function,
}

#[repr(C)]
pub struct sdca_function {
    pub type_: c_int,
}

unsafe extern "C" {
    static GFP_KERNEL: gfp_t;

    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn devm_kasprintf(dev: *mut device, gfp: gfp_t, fmt: *const c_char, ...) -> *mut c_char;
    fn is_sdw_slave(dev: *mut device) -> bool;
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "Rust" {
    fn EXPORT_SYMBOL_NS(symbol: unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_soc_dai) -> c_int,
                        namespace: *const c_char);
}

unsafe extern "C" fn asoc_sdw_rt_dmic_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let mut component: *mut snd_soc_component;
    let mut sdw_peripheral: *mut sdw_slave = core::ptr::null_mut();
    let mic_name: *mut c_char;
    let mut rt1320_dmic_num: c_int = 0;
    let part_id: c_int;
    let mut i: c_int;

    component = (*dai).component;

    /*
     * rt715-sdca (aka rt714) is a special case that uses different name in card->components
     * and component->name_prefix.
     */
    if strcmp((*component).name_prefix, c"rt714".as_ptr()) == 0 {
        mic_name = devm_kasprintf((*card).dev, GFP_KERNEL, c"rt715-sdca".as_ptr());
    } else {
        mic_name = devm_kasprintf((*card).dev, GFP_KERNEL, c"%s".as_ptr(), (*component).name_prefix);
    }
    if mic_name.is_null() {
        return -ENOMEM;
    }

    /*
     * If there is any rt1320/rt1321 DMIC belonging to this card, try to count the `cfg-mics`
     * to be used in card->components.
     * Note: The rt1320 drivers register the peripheral dev to component->dev, so get the
     * sdw_peripheral from component->dev.
     */
    if is_sdw_slave((*component).dev) {
        sdw_peripheral = dev_to_sdw_dev((*component).dev);
    }
    if !sdw_peripheral.is_null()
        && ((*sdw_peripheral).id.part_id == 0x1320 || (*sdw_peripheral).id.part_id == 0x1321)
    {
        part_id = (*sdw_peripheral).id.part_id;
        /*
         * This rtd init callback is called once, so count the rt1320/rt1321 with SDCA
         * function SmartMic type in this card.
         */
        for_each_card_components!(card, component, {
            if !is_sdw_slave((*component).dev) {
                continue;
            }
            sdw_peripheral = dev_to_sdw_dev((*component).dev);
            if (*sdw_peripheral).id.part_id != part_id {
                continue;
            }
            i = 0;
            while i < (*sdw_peripheral).sdca_data.num_functions {
                if (*(*sdw_peripheral).sdca_data.function.offset(i as isize)).type_
                    == SDCA_FUNCTION_TYPE_SMART_MIC
                {
                    rt1320_dmic_num += 1;
                    break;
                }
                i += 1;
            }
        });
        (*card).components = devm_kasprintf(
            (*card).dev,
            GFP_KERNEL,
            c"%s mic:%s cfg-mics:%d".as_ptr(),
            (*card).components,
            mic_name,
            rt1320_dmic_num,
        );
    } else {
        (*card).components = devm_kasprintf(
            (*card).dev,
            GFP_KERNEL,
            c"%s mic:%s".as_ptr(),
            (*card).components,
            mic_name,
        );
    }

    if (*card).components.is_null() {
        return -ENOMEM;
    }

    dev_dbg((*card).dev, c"card->components: %s\n".as_ptr(), (*card).components);

    0
}

EXPORT_SYMBOL_NS(asoc_sdw_rt_dmic_rtd_init, c"SND_SOC_SDW_UTILS".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
