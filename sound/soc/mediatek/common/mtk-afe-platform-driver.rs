// SPDX-License-Identifier: GPL-2.0
/*
 * mtk-afe-platform-driver.c  --  Mediatek afe platform driver
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/dma-mapping.h>
// #include <sound/soc.h>
// #include "mtk-afe-platform-driver.h"
// #include "mtk-base-afe.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;

type size_t = usize;
type ssize_t = isize;
type snd_pcm_uframes_t = usize;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SNDRV_DMA_TYPE_DEV: c_int = 0;

extern "C" {
    static AFE_PCM_NAME: *const c_char;

    fn devm_kcalloc(
        dev: *mut device,
        n: size_t,
        size: size_t,
        flags: c_uint,
    ) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;

    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn snd_soc_component_to_dapm(
        component: *mut snd_soc_component,
    ) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_new_widgets(card: *mut snd_soc_card) -> c_int;

    fn snd_soc_substream_to_rtd(
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: ssize_t) -> snd_pcm_uframes_t;

    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: size_t,
        max: size_t,
    );
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct mtk_base_memif_data {
    pub reg_ofs_cur: c_uint,
    pub reg_ofs_cur_msb: c_uint,
    pub reg_ofs_base: c_uint,
    pub reg_ofs_base_msb: c_uint,
}

#[repr(C)]
pub struct mtk_base_afe_memif {
    pub data: *const mtk_base_memif_data,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    pub list: list_head,
    pub num_dai_drivers: size_t,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub sub_dais: list_head,
    pub num_dai_drivers: size_t,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub dev: *mut device,
    pub memif: *mut mtk_base_afe_memif,
    pub regmap: *mut regmap,
    pub mtk_afe_hardware: *mut snd_pcm_hardware,
    pub preallocate_buffers: bool,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub pointer: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
        ) -> snd_pcm_uframes_t,
    >,
    pub pcm_new: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            rtd: *mut snd_soc_pcm_runtime,
        ) -> c_int,
    >,
    pub probe: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
}

unsafe fn list_entry_mtk_base_afe_dai(ptr: *mut list_head) -> *mut mtk_base_afe_dai {
    (ptr as *mut u8).sub(core::mem::offset_of!(mtk_base_afe_dai, list)) as *mut mtk_base_afe_dai
}

fn MTK_ALIGN_16BYTES(x: u64) -> u64 {
    x & !0xf
}

#[no_mangle]
pub unsafe extern "C" fn mtk_afe_combine_sub_dai(afe: *mut mtk_base_afe) -> c_int {
    let mut dai: *mut mtk_base_afe_dai;
    let mut num_dai_drivers: size_t = 0;
    let mut dai_idx: size_t = 0;

    /* calcualte total dai driver size */
    let mut pos = (*afe).sub_dais.next;
    while pos != &mut (*afe).sub_dais {
        dai = list_entry_mtk_base_afe_dai(pos);
        num_dai_drivers = num_dai_drivers.wrapping_add((*dai).num_dai_drivers);
        pos = (*pos).next;
    }

    dev_info(
        (*afe).dev,
        b"%s(), num of dai %zd\n\0".as_ptr() as *const c_char,
        b"mtk_afe_combine_sub_dai\0".as_ptr() as *const c_char,
        num_dai_drivers,
    );

    /* combine sub_dais */
    (*afe).num_dai_drivers = num_dai_drivers;
    (*afe).dai_drivers = devm_kcalloc(
        (*afe).dev,
        num_dai_drivers,
        size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if (*afe).dai_drivers.is_null() {
        return -ENOMEM;
    }

    pos = (*afe).sub_dais.next;
    while pos != &mut (*afe).sub_dais {
        dai = list_entry_mtk_base_afe_dai(pos);
        /* dai driver */
        memcpy(
            (*afe).dai_drivers.add(dai_idx) as *mut c_void,
            (*dai).dai_drivers as *const c_void,
            (*dai)
                .num_dai_drivers
                .wrapping_mul(size_of::<snd_soc_dai_driver>()),
        );
        dai_idx = dai_idx.wrapping_add((*dai).num_dai_drivers);
        pos = (*pos).next;
    }
    0
}
// EXPORT_SYMBOL_GPL(mtk_afe_combine_sub_dai);

#[no_mangle]
pub unsafe extern "C" fn mtk_afe_add_sub_dai_control(
    component: *mut snd_soc_component,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;
    let mut dai: *mut mtk_base_afe_dai;

    let mut pos = (*afe).sub_dais.next;
    while pos != &mut (*afe).sub_dais {
        dai = list_entry_mtk_base_afe_dai(pos);
        if !(*dai).controls.is_null() {
            snd_soc_add_component_controls(component, (*dai).controls, (*dai).num_controls);
        }

        if !(*dai).dapm_widgets.is_null() {
            snd_soc_dapm_new_controls(dapm, (*dai).dapm_widgets, (*dai).num_dapm_widgets);
        }
        pos = (*pos).next;
    }
    /* add routes after all widgets are added */
    pos = (*afe).sub_dais.next;
    while pos != &mut (*afe).sub_dais {
        dai = list_entry_mtk_base_afe_dai(pos);
        if !(*dai).dapm_routes.is_null() {
            snd_soc_dapm_add_routes(dapm, (*dai).dapm_routes, (*dai).num_dapm_routes);
        }
        pos = (*pos).next;
    }

    snd_soc_dapm_new_widgets((*component).card);

    0
}
// EXPORT_SYMBOL_GPL(mtk_afe_add_sub_dai_control);

#[no_mangle]
pub unsafe extern "C" fn mtk_afe_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;
    let memif = (*afe)
        .memif
        .add((*snd_soc_rtd_to_cpu(rtd, 0)).id as usize);
    let memif_data = (*memif).data;
    let regmap = (*afe).regmap;
    let dev = (*afe).dev;
    let mut hw_ptr_lower32: c_uint = 0;
    let mut hw_ptr_upper32: c_uint = 0;
    let mut hw_base_lower32: c_uint = 0;
    let mut hw_base_upper32: c_uint = 0;
    let mut hw_ptr: u64 = 0;
    let mut hw_base: u64 = 0;
    let mut ret: c_int;
    let mut pcm_ptr_bytes: u64 = 0;

    ret = regmap_read(regmap, (*memif_data).reg_ofs_cur, &mut hw_ptr_lower32);
    if ret != 0 {
        dev_err(
            dev,
            b"%s hw_ptr_lower32 err\n\0".as_ptr() as *const c_char,
            b"mtk_afe_pcm_pointer\0".as_ptr() as *const c_char,
        );
        return 0;
    }

    if (*memif_data).reg_ofs_cur_msb != 0 {
        ret = regmap_read(regmap, (*memif_data).reg_ofs_cur_msb, &mut hw_ptr_upper32);
        if ret != 0 {
            dev_err(
                dev,
                b"%s hw_ptr_upper32 err\n\0".as_ptr() as *const c_char,
                b"mtk_afe_pcm_pointer\0".as_ptr() as *const c_char,
            );
            return 0;
        }
    }

    ret = regmap_read(regmap, (*memif_data).reg_ofs_base, &mut hw_base_lower32);
    if ret != 0 {
        dev_err(
            dev,
            b"%s hw_base_lower32 err\n\0".as_ptr() as *const c_char,
            b"mtk_afe_pcm_pointer\0".as_ptr() as *const c_char,
        );
        return 0;
    }
    if (*memif_data).reg_ofs_base_msb != 0 {
        ret = regmap_read(regmap, (*memif_data).reg_ofs_base_msb, &mut hw_base_upper32);
        if ret != 0 {
            dev_err(
                dev,
                b"%s hw_base_upper32 err\n\0".as_ptr() as *const c_char,
                b"mtk_afe_pcm_pointer\0".as_ptr() as *const c_char,
            );
            return 0;
        }
    }

    hw_ptr = ((hw_ptr_upper32 as u64) << 32) | hw_ptr_lower32 as u64;
    hw_base = ((hw_base_upper32 as u64) << 32) | hw_base_lower32 as u64;

    if hw_ptr == 0 || hw_base == 0 {
        dev_err(
            dev,
            b"hw_ptr or hw_base = 0 err\n\0".as_ptr() as *const c_char,
        );
        return 0;
    }

    pcm_ptr_bytes = MTK_ALIGN_16BYTES(hw_ptr.wrapping_sub(hw_base));
    bytes_to_frames((*substream).runtime, pcm_ptr_bytes as ssize_t)
}
// EXPORT_SYMBOL_GPL(mtk_afe_pcm_pointer);

#[no_mangle]
pub unsafe extern "C" fn mtk_afe_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let size: size_t;
    let pcm = (*rtd).pcm;
    let afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;

    size = (*(*afe).mtk_afe_hardware).buffer_bytes_max;
    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        (*afe).dev as *mut c_void,
        if (*afe).preallocate_buffers { size } else { 0 },
        size,
    );

    0
}
// EXPORT_SYMBOL_GPL(mtk_afe_pcm_new);

unsafe extern "C" fn mtk_afe_component_probe(component: *mut snd_soc_component) -> c_int {
    let afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;
    let ret: c_int;

    snd_soc_component_init_regmap(component, (*afe).regmap);

    /* If the list was never initialized there are no sub-DAIs */
    if !(*afe).sub_dais.next.is_null() && !(*afe).sub_dais.prev.is_null() {
        ret = mtk_afe_add_sub_dai_control(component);
        if ret != 0 {
            return ret;
        }
    }

    0
}

#[no_mangle]
pub static mtk_afe_pcm_platform: snd_soc_component_driver = snd_soc_component_driver {
    name: unsafe { AFE_PCM_NAME },
    pointer: Some(mtk_afe_pcm_pointer),
    pcm_new: Some(mtk_afe_pcm_new),
    probe: Some(mtk_afe_component_probe),
};
// EXPORT_SYMBOL_GPL(mtk_afe_pcm_platform);

// MODULE_DESCRIPTION("Mediatek simple platform driver");
// MODULE_AUTHOR("Garlic Tseng <garlic.tseng@mediatek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
