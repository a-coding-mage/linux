// SPDX-License-Identifier: GPL-2.0-only
//
// aw87390.c  --  AW87390 ALSA SoC Audio driver
//
// Copyright (c) 2023 awinic Technology CO., LTD
//
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null, null_mut};

type u32 = c_uint;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENXIO: c_int = 6;
const EPERM: c_int = 1;
const GFP_KERNEL: c_uint = 0;
const I2C_FUNC_I2C: c_uint = 1;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_uint = 3;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x2;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x4;
const SND_SOC_DAPM_POST_PMD: c_int = 0x8;
const REGMAP_ENDIAN_LITTLE: c_uint = 0;
const REGMAP_ENDIAN_BIG: c_uint = 1;

extern "C" {
    static AW87390_REG_MAX: c_uint;
    static AW87390_DELAY_REG_ADDR: u8;
    static AW87390_REG_DELAY_TIME: u8;
    static AW87390_DEV_PW_OFF: c_int;
    static AW87390_DEV_PW_ON: c_int;
    static AW87390_DEV_FW_FAILED: c_int;
    static AW87390_SYSCTRL_REG: c_uint;
    static AW87390_POWER_DOWN_VALUE: c_uint;
    static AW87390_ACF_FILE: *const c_char;
    static AW87391_SYSCTRL_REG: c_uint;
    static AW87391_REG_VER_SEL_LOW: c_uint;
    static AW87391_REG_EN_ADAP: c_uint;
    static AW87391_REG_EN_2X: c_uint;
    static AW87391_EN_SPK: c_uint;
    static AW87391_EN_PA: c_uint;
    static AW87391_REG_EN_CP: c_uint;
    static AW87391_EN_SW: c_uint;
    static AW87391_CP_REG: c_uint;
    static AW87391_REG_CP_OVP_8_50V: c_uint;
    static AW87391_AGCPO_REG: c_uint;
    static AW87391_AK1_S_016: c_uint;
    static AW87391_AGC2PA_REG: c_uint;
    static AW87391_RK_S_20_48: c_uint;
    static AW87391_AK2_S_41: c_uint;
    static AW87391_AK2F_S_41: c_uint;
    static AW87391_PAG_REG: c_uint;
    static AW87391_GAIN_12DB: c_uint;
    static AW87391_GAIN_15DB: c_uint;
    static AW87390_DEV_DEFAULT_CH: u32;
    static AW88395_DEV_NONE_TYPE_ID: c_int;
    static AW87390_INIT_PROFILE: c_int;
    static AW87390_ID_REG: c_uint;
    static AW87390_CHIP_ID: c_uint;
    static AW87391_CHIP_ID: c_uint;
    static AW87390_SOFT_RESET_VALUE: c_uint;
    static AW87390_I2C_NAME: *const c_char;
    static AW87391_I2C_NAME: *const c_char;
    static AW88395_DATA_TYPE_REG: usize;

    fn AW87391_AGC2PO_MW(mw: c_uint) -> c_uint;
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
struct i2c_client {
    dev: device,
    adapter: *mut i2c_adapter,
}

#[repr(C)]
struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct firmware {
    size: usize,
    data: *const u8,
}

#[repr(C)]
struct aw_sec_data_desc {
    data: *mut u8,
    len: c_uint,
}

#[repr(C)]
struct aw_prof_desc {
    id: usize,
    sec_desc: *mut aw_sec_data_desc,
}

#[repr(C)]
struct aw_prof_info {
    prof_desc: *mut aw_prof_desc,
    count: c_int,
    prof_type: c_int,
    prof_name_list: *mut *mut c_char,
}

#[repr(C)]
struct aw_container {
    len: usize,
    data: [u8; 0],
}

#[repr(C)]
struct aw_device {
    dev: *mut device,
    i2c: *mut i2c_client,
    regmap: *mut regmap,
    acf: *mut c_void,
    prof_info: aw_prof_info,
    channel: u32,
    fw_status: c_int,
    prof_index: c_int,
    prof_cur: c_int,
    status: c_int,
    chip_id: c_uint,
}

#[repr(C)]
struct aw87390 {
    aw_pa: *mut aw_device,
    regmap: *mut regmap,
    aw_cfg: *mut aw_container,
    lock: mutex,
    vdd_reg: *mut regulator,
}

#[repr(C)]
struct regmap_config {
    val_bits: c_uint,
    reg_bits: c_uint,
    max_register: c_uint,
    reg_format_endian: c_uint,
    val_format_endian: c_uint,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_ctl_elem_info_enumerated {
    items: c_uint,
    item: c_uint,
    name: [c_char; 64],
}

#[repr(C)]
union snd_ctl_elem_info_value {
    enumerated: snd_ctl_elem_info_enumerated,
}

#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [i64; 128],
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct i2c_device_id {
    name: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
}

extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn aw88395_dev_load_acf_check(aw_dev: *mut aw_device, cfg: *mut aw_container) -> c_int;
    fn aw88395_dev_cfg_load(aw_dev: *mut aw_device, cfg: *mut aw_container) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn devm_regulator_get_optional(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn i2c_check_functionality(adapter: *mut i2c_adapter, functionality: c_uint) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_match_ptr(matches: *const of_device_id) -> *const of_device_id;
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe { mutex_lock(lock) };
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.lock) };
    }
}

static aw87390_remap_config: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 8,
    max_register: unsafe { AW87390_REG_MAX },
    reg_format_endian: REGMAP_ENDIAN_LITTLE,
    val_format_endian: REGMAP_ENDIAN_BIG,
};

unsafe extern "C" fn aw87390_dev_reg_update(
    aw_dev: *mut aw_device,
    data: *mut u8,
    len: c_uint,
) -> c_int {
    let mut i: c_uint;
    let mut ret: c_int;

    if data.is_null() {
        unsafe { dev_err((*aw_dev).dev, c"data is NULL\n".as_ptr()) };
        return -EINVAL;
    }

    i = 0;
    while i < len.wrapping_sub(1) {
        if unsafe { *data.add(i as usize) } == unsafe { AW87390_DELAY_REG_ADDR } {
            unsafe {
                usleep_range(
                    (*data.add(i as usize + 1) as c_uint)
                        .wrapping_mul(AW87390_REG_DELAY_TIME as c_uint),
                    (*data.add(i as usize + 1) as c_uint)
                        .wrapping_mul(AW87390_REG_DELAY_TIME as c_uint)
                        .wrapping_add(10),
                );
            }
            i = i.wrapping_add(2);
            continue;
        }
        ret = unsafe {
            regmap_write(
                (*aw_dev).regmap,
                *data.add(i as usize) as c_uint,
                *data.add(i as usize + 1) as c_uint,
            )
        };
        if ret != 0 {
            return ret;
        }
        i = i.wrapping_add(2);
    }

    0
}

unsafe extern "C" fn aw87390_dev_get_prof_name(
    aw_dev: *mut aw_device,
    index: c_int,
    prof_name: *mut *mut c_char,
) -> c_int {
    let prof_info: *mut aw_prof_info = unsafe { addr_of_mut!((*aw_dev).prof_info) };
    let prof_desc: *mut aw_prof_desc;

    if unsafe { index >= (*aw_dev).prof_info.count || index < 0 } {
        unsafe {
            dev_err(
                (*aw_dev).dev,
                c"index[%d] overflow count[%d]\n".as_ptr(),
                index,
                (*aw_dev).prof_info.count,
            )
        };
        return -EINVAL;
    }

    prof_desc = unsafe { (*aw_dev).prof_info.prof_desc.add(index as usize) };
    unsafe {
        *prof_name = *(*prof_info).prof_name_list.add((*prof_desc).id);
    }

    0
}

unsafe extern "C" fn aw87390_dev_get_prof_data(
    aw_dev: *mut aw_device,
    index: c_int,
    prof_desc: *mut *mut aw_prof_desc,
) -> c_int {
    if unsafe { index >= (*aw_dev).prof_info.count || index < 0 } {
        unsafe {
            dev_err(
                (*aw_dev).dev,
                c"%s: index[%d] overflow count[%d]\n".as_ptr(),
                c"aw87390_dev_get_prof_data".as_ptr(),
                index,
                (*aw_dev).prof_info.count,
            )
        };
        return -EINVAL;
    }

    unsafe {
        *prof_desc = (*aw_dev).prof_info.prof_desc.add(index as usize);
    }

    0
}

unsafe extern "C" fn aw87390_dev_fw_update(aw_dev: *mut aw_device) -> c_int {
    let mut prof_index_desc: *mut aw_prof_desc = null_mut();
    let sec_desc: *mut aw_sec_data_desc;
    let mut prof_name: *mut c_char = null_mut();
    let mut ret: c_int;

    ret = unsafe { aw87390_dev_get_prof_name(aw_dev, (*aw_dev).prof_index, &mut prof_name) };
    if ret != 0 {
        unsafe { dev_err((*aw_dev).dev, c"get prof name failed\n".as_ptr()) };
        return -EINVAL;
    }

    unsafe { dev_dbg((*aw_dev).dev, c"start update %s".as_ptr(), prof_name) };

    ret = unsafe { aw87390_dev_get_prof_data(aw_dev, (*aw_dev).prof_index, &mut prof_index_desc) };
    if ret != 0 {
        unsafe { dev_err((*aw_dev).dev, c"aw87390_dev_get_prof_data failed\n".as_ptr()) };
        return ret;
    }

    /* update reg */
    sec_desc = unsafe { (*prof_index_desc).sec_desc };
    ret = unsafe {
        aw87390_dev_reg_update(
            aw_dev,
            (*sec_desc.add(AW88395_DATA_TYPE_REG)).data,
            (*sec_desc.add(AW88395_DATA_TYPE_REG)).len,
        )
    };
    if ret != 0 {
        unsafe { dev_err((*aw_dev).dev, c"update reg failed\n".as_ptr()) };
        return ret;
    }

    unsafe {
        (*aw_dev).prof_cur = (*aw_dev).prof_index;
    }

    0
}

unsafe extern "C" fn aw87390_power_off(aw_dev: *mut aw_device) -> c_int {
    let ret: c_int;

    if unsafe { (*aw_dev).status == AW87390_DEV_PW_OFF } {
        unsafe { dev_dbg((*aw_dev).dev, c"already power off\n".as_ptr()) };
        return 0;
    }

    ret = unsafe { regmap_write((*aw_dev).regmap, AW87390_SYSCTRL_REG, AW87390_POWER_DOWN_VALUE) };
    if ret != 0 {
        return ret;
    }
    unsafe {
        (*aw_dev).status = AW87390_DEV_PW_OFF;
    }

    0
}

unsafe extern "C" fn aw87390_power_on(aw_dev: *mut aw_device) -> c_int {
    let mut ret: c_int;

    if unsafe { (*aw_dev).status == AW87390_DEV_PW_ON } {
        unsafe { dev_dbg((*aw_dev).dev, c"already power on\n".as_ptr()) };
        return 0;
    }

    if unsafe { (*aw_dev).fw_status == 0 } {
        unsafe { dev_err((*aw_dev).dev, c"fw not load\n".as_ptr()) };
        return -EINVAL;
    }

    ret = unsafe { regmap_write((*aw_dev).regmap, AW87390_SYSCTRL_REG, AW87390_POWER_DOWN_VALUE) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { aw87390_dev_fw_update(aw_dev) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*aw_dev).dev,
                c"%s load profile failed\n".as_ptr(),
                c"aw87390_power_on".as_ptr(),
            )
        };
        return ret;
    }
    unsafe {
        (*aw_dev).status = AW87390_DEV_PW_ON;
    }

    0
}

unsafe extern "C" fn aw87390_dev_set_profile_index(aw_dev: *mut aw_device, index: c_int) -> c_int {
    if unsafe { index >= (*aw_dev).prof_info.count || index < 0 } {
        return -EINVAL;
    }

    if unsafe { (*aw_dev).prof_index == index } {
        return -EPERM;
    }

    unsafe {
        (*aw_dev).prof_index = index;
    }

    0
}

unsafe extern "C" fn aw87390_profile_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let codec: *mut snd_soc_component = unsafe { snd_kcontrol_chip(kcontrol) };
    let aw87390: *mut aw87390 = unsafe { snd_soc_component_get_drvdata(codec) as *mut aw87390 };
    let mut prof_name: *mut c_char = null_mut();
    let mut count: c_int;
    let ret: c_int;

    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
        (*uinfo).count = 1;
    }

    count = unsafe { (*(*aw87390).aw_pa).prof_info.count };
    if count <= 0 {
        unsafe {
            (*uinfo).value.enumerated.items = 0;
        }
        return 0;
    }

    unsafe {
        (*uinfo).value.enumerated.items = count as c_uint;
    }

    if unsafe { (*uinfo).value.enumerated.item >= count as c_uint } {
        unsafe {
            (*uinfo).value.enumerated.item = count as c_uint - 1;
        }
    }

    count = unsafe { (*uinfo).value.enumerated.item as c_int };

    ret = unsafe { aw87390_dev_get_prof_name((*aw87390).aw_pa, count, &mut prof_name) };
    if ret != 0 {
        unsafe {
            strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), c"null".as_ptr());
        }
        return 0;
    }

    unsafe {
        strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), prof_name);
    }

    0
}

unsafe extern "C" fn aw87390_profile_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec: *mut snd_soc_component = unsafe { snd_kcontrol_chip(kcontrol) };
    let aw87390: *mut aw87390 = unsafe { snd_soc_component_get_drvdata(codec) as *mut aw87390 };

    unsafe {
        (*ucontrol).value.integer.value[0] = (*(*aw87390).aw_pa).prof_index as i64;
    }

    0
}

unsafe extern "C" fn aw87390_profile_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec: *mut snd_soc_component = unsafe { snd_kcontrol_chip(kcontrol) };
    let aw87390: *mut aw87390 = unsafe { snd_soc_component_get_drvdata(codec) as *mut aw87390 };
    let ret: c_int;

    let _guard = unsafe { MutexGuard::new(addr_of_mut!((*aw87390).lock)) };
    ret = unsafe {
        aw87390_dev_set_profile_index(
            (*aw87390).aw_pa,
            (*ucontrol).value.integer.value[0] as c_int,
        )
    };
    if ret != 0 {
        unsafe { dev_dbg((*codec).dev, c"profile index does not change\n".as_ptr()) };
        return 0;
    }

    if unsafe { (*(*aw87390).aw_pa).status == AW87390_DEV_PW_ON } {
        unsafe {
            aw87390_power_off((*aw87390).aw_pa);
            aw87390_power_on((*aw87390).aw_pa);
        }
    }

    1
}

/* AW87390_PROFILE_EXT macro expansion is supplied by aw87390.h in C. */
static aw87390_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _private: [] }];

unsafe extern "C" fn aw87390_request_firmware_file(aw87390: *mut aw87390) -> c_int {
    let mut cont: *const firmware = null();
    let mut ret: c_int;

    unsafe {
        (*(*aw87390).aw_pa).fw_status = AW87390_DEV_FW_FAILED;
    }

    ret = unsafe { request_firmware(&mut cont, AW87390_ACF_FILE, (*(*aw87390).aw_pa).dev) };
    if ret != 0 {
        return unsafe {
            dev_err_probe(
                (*(*aw87390).aw_pa).dev,
                ret,
                c"load [%s] failed!\n".as_ptr(),
                AW87390_ACF_FILE,
            )
        };
    }

    unsafe {
        dev_dbg(
            (*(*aw87390).aw_pa).dev,
            c"loaded %s - size: %zu\n".as_ptr(),
            AW87390_ACF_FILE,
            if !cont.is_null() { (*cont).size } else { 0 },
        );
    }

    unsafe {
        (*aw87390).aw_cfg = devm_kzalloc(
            (*(*aw87390).aw_pa).dev,
            size_of::<aw_container>() + (*cont).size,
            GFP_KERNEL,
        ) as *mut aw_container;
    }
    if unsafe { (*aw87390).aw_cfg.is_null() } {
        return -ENOMEM;
    }

    unsafe {
        (*(*aw87390).aw_cfg).len = (*cont).size;
        memcpy(
            (*(*aw87390).aw_cfg).data.as_mut_ptr() as *mut c_void,
            (*cont).data as *const c_void,
            (*cont).size,
        );
    }

    ret = unsafe { aw88395_dev_load_acf_check((*aw87390).aw_pa, (*aw87390).aw_cfg) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*(*aw87390).aw_pa).dev,
                c"load [%s] failed!\n".as_ptr(),
                AW87390_ACF_FILE,
            );
        }
        return ret;
    }

    let _guard = unsafe { MutexGuard::new(addr_of_mut!((*aw87390).lock)) };

    ret = unsafe { aw88395_dev_cfg_load((*aw87390).aw_pa, (*aw87390).aw_cfg) };
    if ret != 0 {
        unsafe { dev_err((*(*aw87390).aw_pa).dev, c"aw_dev acf parse failed\n".as_ptr()) };
    }

    ret
}

unsafe extern "C" fn aw87390_drv_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { snd_soc_dapm_to_component((*w).dapm) };
    let aw87390: *mut aw87390 = unsafe { snd_soc_component_get_drvdata(component) as *mut aw87390 };
    let aw_dev: *mut aw_device = unsafe { (*aw87390).aw_pa };
    let ret: c_int;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            ret = unsafe { aw87390_power_on(aw_dev) };
        }
        SND_SOC_DAPM_POST_PMD => {
            ret = unsafe { aw87390_power_off(aw_dev) };
        }
        _ => {
            unsafe {
                dev_err(
                    (*aw_dev).dev,
                    c"%s: invalid event %d\n".as_ptr(),
                    c"aw87390_drv_event".as_ptr(),
                    event,
                );
            }
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn aw87391_rgds_drv_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { snd_soc_dapm_to_component((*w).dapm) };
    let aw87390: *mut aw87390 = unsafe { snd_soc_component_get_drvdata(component) as *mut aw87390 };
    let aw_dev: *mut aw_device = unsafe { (*aw87390).aw_pa };

    match event {
        SND_SOC_DAPM_PRE_PMU => unsafe {
            if !IS_ERR((*aw87390).vdd_reg as *const c_void) {
                if regulator_enable((*aw87390).vdd_reg) != 0 {
                    dev_warn((*aw_dev).dev, c"Failed to enable vdd\n".as_ptr());
                }
            }
        },
        SND_SOC_DAPM_POST_PMU => unsafe {
            regmap_write(
                (*aw_dev).regmap,
                AW87391_SYSCTRL_REG,
                AW87391_REG_VER_SEL_LOW
                    | AW87391_REG_EN_ADAP
                    | AW87391_REG_EN_2X
                    | AW87391_EN_SPK
                    | AW87391_EN_PA
                    | AW87391_REG_EN_CP
                    | AW87391_EN_SW,
            );
        },
        SND_SOC_DAPM_PRE_PMD => unsafe {
            regmap_write((*aw_dev).regmap, AW87390_SYSCTRL_REG, AW87390_POWER_DOWN_VALUE);
        },
        SND_SOC_DAPM_POST_PMD => unsafe {
            if !IS_ERR((*aw87390).vdd_reg as *const c_void) {
                if regulator_disable((*aw87390).vdd_reg) != 0 {
                    dev_warn((*aw_dev).dev, c"Failed to disable vdd\n".as_ptr());
                }
            }
        },
        _ => {
            unsafe {
                dev_err(
                    (*aw_dev).dev,
                    c"%s: invalid event %d\n".as_ptr(),
                    c"aw87391_rgds_drv_event".as_ptr(),
                    event,
                );
            }
            return -EINVAL;
        }
    }

    0
}

/* SND_SOC_DAPM_* macro-created widget initializers are supplied by ALSA headers in C. */
static aw87390_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { _private: [], dapm: null_mut() },
    snd_soc_dapm_widget { _private: [], dapm: null_mut() },
    snd_soc_dapm_widget { _private: [], dapm: null_mut() },
];

static aw87391_rgds_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { _private: [], dapm: null_mut() },
    snd_soc_dapm_widget { _private: [], dapm: null_mut() },
    snd_soc_dapm_widget { _private: [], dapm: null_mut() },
];

static aw87390_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"SPK PA".as_ptr(),
        control: null(),
        source: c"IN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT".as_ptr(),
        control: null(),
        source: c"SPK PA".as_ptr(),
    },
];

unsafe extern "C" fn aw87390_codec_probe(component: *mut snd_soc_component) -> c_int {
    let aw87390: *mut aw87390 = unsafe { snd_soc_component_get_drvdata(component) as *mut aw87390 };
    let ret: c_int;

    ret = unsafe { aw87390_request_firmware_file(aw87390) };
    if ret != 0 {
        return unsafe {
            dev_err_probe(
                (*(*aw87390).aw_pa).dev,
                ret,
                c"aw87390_request_firmware_file failed\n".as_ptr(),
            )
        };
    }

    0
}

/*
 * Firmware typically is used to load the sequence of init commands,
 * however for the Anbernic RG-DS we don't have a firmware file just
 * a list of registers and values. Most of these values are undocumented
 * in the AW87391 datasheet.
 */
unsafe extern "C" fn aw87391_rgds_codec_init(aw87390: *mut aw87390) {
    let aw_dev: *mut aw_device = unsafe { (*aw87390).aw_pa };

    /* Undocumented command per datasheet. */
    unsafe { regmap_write((*aw_dev).regmap, 0x64, 0x3a) };

    /* Bits 7:4 are undocumented but provided by manufacturer. */
    unsafe {
        regmap_write(
            (*aw_dev).regmap,
            AW87391_CP_REG,
            (5 << 4) | AW87391_REG_CP_OVP_8_50V,
        );
    }

    unsafe {
        regmap_write(
            (*aw_dev).regmap,
            AW87391_AGCPO_REG,
            AW87391_AK1_S_016 | AW87391_AGC2PO_MW(500),
        );

        regmap_write(
            (*aw_dev).regmap,
            AW87391_AGC2PA_REG,
            AW87391_RK_S_20_48 | AW87391_AK2_S_41 | AW87391_AK2F_S_41,
        );
    }

    /* Undocumented commands per datasheet. */
    unsafe {
        regmap_write((*aw_dev).regmap, 0x5d, 0x00);
        regmap_write((*aw_dev).regmap, 0x5e, 0xb4);
        regmap_write((*aw_dev).regmap, 0x5f, 0x30);
        regmap_write((*aw_dev).regmap, 0x60, 0x39);
        regmap_write((*aw_dev).regmap, 0x61, 0x10);
        regmap_write((*aw_dev).regmap, 0x62, 0x03);
        regmap_write((*aw_dev).regmap, 0x63, 0x7d);
        regmap_write((*aw_dev).regmap, 0x65, 0xa0);
        regmap_write((*aw_dev).regmap, 0x66, 0x21);
        regmap_write((*aw_dev).regmap, 0x67, 0x41);
        regmap_write((*aw_dev).regmap, 0x68, 0x3b);
        regmap_write((*aw_dev).regmap, 0x6e, 0x00);
        regmap_write((*aw_dev).regmap, 0x6f, 0x00);
        regmap_write((*aw_dev).regmap, 0x70, 0x00);
        regmap_write((*aw_dev).regmap, 0x71, 0x00);
        regmap_write((*aw_dev).regmap, 0x72, 0x34);
        regmap_write((*aw_dev).regmap, 0x73, 0x06);
        regmap_write((*aw_dev).regmap, 0x74, 0x10);
        regmap_write((*aw_dev).regmap, 0x75, 0x00);
        regmap_write((*aw_dev).regmap, 0x7a, 0x00);
        regmap_write((*aw_dev).regmap, 0x7b, 0x00);
        regmap_write((*aw_dev).regmap, 0x7c, 0x00);
        regmap_write((*aw_dev).regmap, 0x7d, 0x00);

        regmap_write((*aw_dev).regmap, AW87391_PAG_REG, AW87391_GAIN_12DB);
        regmap_write(
            (*aw_dev).regmap,
            AW87391_SYSCTRL_REG,
            AW87391_EN_PA | AW87391_REG_EN_CP | AW87391_EN_SW,
        );
        regmap_write(
            (*aw_dev).regmap,
            AW87391_SYSCTRL_REG,
            AW87391_REG_VER_SEL_LOW
                | AW87391_REG_EN_ADAP
                | AW87391_REG_EN_2X
                | AW87391_EN_SPK
                | AW87391_EN_PA
                | AW87391_REG_EN_CP
                | AW87391_EN_SW,
        );
        regmap_write((*aw_dev).regmap, AW87391_PAG_REG, AW87391_GAIN_15DB);
    }
}

unsafe extern "C" fn aw87391_rgds_codec_probe(component: *mut snd_soc_component) -> c_int {
    let aw87390: *mut aw87390 = unsafe { snd_soc_component_get_drvdata(component) as *mut aw87390 };

    unsafe {
        (*aw87390).vdd_reg = devm_regulator_get_optional((*(*aw87390).aw_pa).dev, c"vdd".as_ptr());
        if IS_ERR((*aw87390).vdd_reg as *const c_void)
            && PTR_ERR((*aw87390).vdd_reg as *const c_void) != -ENODEV
        {
            return dev_err_probe(
                (*(*aw87390).aw_pa).dev,
                PTR_ERR((*aw87390).vdd_reg as *const c_void),
                c"Could not get vdd regulator\n".as_ptr(),
            );
        }

        aw87391_rgds_codec_init(aw87390);
    }

    0
}

static soc_codec_dev_aw87390: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(aw87390_codec_probe),
    dapm_widgets: aw87390_dapm_widgets.as_ptr(),
    num_dapm_widgets: aw87390_dapm_widgets.len() as c_uint,
    dapm_routes: aw87390_dapm_routes.as_ptr(),
    num_dapm_routes: aw87390_dapm_routes.len() as c_uint,
    controls: aw87390_controls.as_ptr(),
    num_controls: aw87390_controls.len() as c_uint,
};

static soc_codec_dev_anbernic_rgds: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(aw87391_rgds_codec_probe),
    dapm_widgets: aw87391_rgds_dapm_widgets.as_ptr(),
    num_dapm_widgets: aw87391_rgds_dapm_widgets.len() as c_uint,
    dapm_routes: aw87390_dapm_routes.as_ptr(),
    num_dapm_routes: aw87390_dapm_routes.len() as c_uint,
    controls: null(),
    num_controls: 0,
};

unsafe extern "C" fn aw87390_parse_channel_dt(aw87390: *mut aw87390) {
    let aw_dev: *mut aw_device = unsafe { (*aw87390).aw_pa };
    let np: *mut device_node = unsafe { (*(*aw_dev).dev).of_node };
    let mut channel_value: u32 = unsafe { AW87390_DEV_DEFAULT_CH };

    unsafe {
        of_property_read_u32(np, c"awinic,audio-channel".as_ptr(), &mut channel_value);
        (*aw_dev).channel = channel_value;
    }
}

unsafe extern "C" fn aw87390_init(
    aw87390: *mut aw87390,
    i2c: *mut i2c_client,
    regmap: *mut regmap,
) -> c_int {
    let aw_dev: *mut aw_device;
    let mut chip_id: c_uint = 0;
    let ret: c_int;

    aw_dev = unsafe { devm_kzalloc(addr_of_mut!((*i2c).dev), size_of::<aw_device>(), GFP_KERNEL) }
        as *mut aw_device;
    if aw_dev.is_null() {
        return -ENOMEM;
    }

    /* read chip id */
    ret = unsafe { regmap_read(regmap, AW87390_ID_REG, &mut chip_id) };
    if ret != 0 {
        unsafe {
            dev_err(
                addr_of_mut!((*i2c).dev),
                c"%s read chipid error. ret = %d\n".as_ptr(),
                c"aw87390_init".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    if chip_id == unsafe { AW87390_CHIP_ID } {
        unsafe {
            (*aw_dev).chip_id = AW87390_CHIP_ID;
        }
    } else if chip_id == unsafe { AW87391_CHIP_ID } {
        unsafe {
            (*aw_dev).chip_id = AW87391_CHIP_ID;
        }
    } else {
        unsafe { dev_err(addr_of_mut!((*i2c).dev), c"unsupported device\n".as_ptr()) };
        return -ENXIO;
    }

    unsafe { dev_dbg(addr_of_mut!((*i2c).dev), c"chip id = 0x%x\n".as_ptr(), chip_id) };

    unsafe {
        (*aw87390).aw_pa = aw_dev;
        (*aw_dev).i2c = i2c;
        (*aw_dev).regmap = regmap;
        (*aw_dev).dev = addr_of_mut!((*i2c).dev);
        (*aw_dev).acf = null_mut();
        (*aw_dev).prof_info.prof_desc = null_mut();
        (*aw_dev).prof_info.count = 0;
        (*aw_dev).prof_info.prof_type = AW88395_DEV_NONE_TYPE_ID;
        (*aw_dev).channel = AW87390_DEV_DEFAULT_CH;
        (*aw_dev).fw_status = AW87390_DEV_FW_FAILED;
        (*aw_dev).prof_index = AW87390_INIT_PROFILE;
        (*aw_dev).status = AW87390_DEV_PW_OFF;

        aw87390_parse_channel_dt(aw87390);
    }

    0
}

unsafe extern "C" fn aw87390_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let aw87390: *mut aw87390;
    let mut priv_: *const snd_soc_component_driver = null();
    let mut ret: c_int;

    if unsafe { i2c_check_functionality((*i2c).adapter, I2C_FUNC_I2C) == 0 } {
        return unsafe {
            dev_err_probe(
                addr_of_mut!((*i2c).dev),
                -ENXIO,
                c"check_functionality failed\n".as_ptr(),
            )
        };
    }

    aw87390 =
        unsafe { devm_kzalloc(addr_of_mut!((*i2c).dev), size_of::<aw87390>(), GFP_KERNEL) }
            as *mut aw87390;
    if aw87390.is_null() {
        return -ENOMEM;
    }

    unsafe {
        mutex_init(addr_of_mut!((*aw87390).lock));

        i2c_set_clientdata(i2c, aw87390 as *mut c_void);

        (*aw87390).regmap = devm_regmap_init_i2c(i2c, &aw87390_remap_config);
        if IS_ERR((*aw87390).regmap as *const c_void) {
            return dev_err_probe(
                addr_of_mut!((*i2c).dev),
                PTR_ERR((*aw87390).regmap as *const c_void),
                c"failed to init regmap\n".as_ptr(),
            );
        }
    }

    /* aw pa init */
    ret = unsafe { aw87390_init(aw87390, i2c, (*aw87390).regmap) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { regmap_write((*aw87390).regmap, AW87390_ID_REG, AW87390_SOFT_RESET_VALUE) };
    if ret != 0 {
        return ret;
    }

    if unsafe { (*(*aw87390).aw_pa).chip_id == AW87390_CHIP_ID } {
        ret = unsafe {
            devm_snd_soc_register_component(
                addr_of_mut!((*i2c).dev),
                &soc_codec_dev_aw87390,
                null(),
                0,
            )
        };
    } else if unsafe { (*(*aw87390).aw_pa).chip_id == AW87391_CHIP_ID } {
        priv_ = unsafe { of_device_get_match_data(addr_of_mut!((*i2c).dev)) }
            as *const snd_soc_component_driver;
        if priv_.is_null() {
            return unsafe {
                dev_err_probe(
                    addr_of_mut!((*i2c).dev),
                    -EINVAL,
                    c"aw87391 not currently supported\n".as_ptr(),
                )
            };
        }
        ret = unsafe {
            devm_snd_soc_register_component(addr_of_mut!((*i2c).dev), priv_, null(), 0)
        };
    } else {
        return -ENXIO;
    }

    if ret != 0 {
        unsafe { dev_err(addr_of_mut!((*i2c).dev), c"failed to register aw87390: %d\n".as_ptr(), ret) };
    }

    ret
}

static aw87390_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c"awinic,aw87390".as_ptr(),
        data: null(),
    },
    of_device_id {
        compatible: c"anbernic,rgds-amp".as_ptr(),
        data: &soc_codec_dev_anbernic_rgds as *const snd_soc_component_driver as *const c_void,
    },
    of_device_id {
        compatible: null(),
        data: null(),
    },
];
/* MODULE_DEVICE_TABLE(of, aw87390_of_match); */

static aw87390_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id {
        name: unsafe { AW87390_I2C_NAME },
    },
    i2c_device_id {
        name: unsafe { AW87391_I2C_NAME },
    },
    i2c_device_id { name: null() },
];
/* MODULE_DEVICE_TABLE(i2c, aw87390_i2c_id); */

static mut aw87390_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: unsafe { AW87390_I2C_NAME },
        of_match_table: unsafe { of_match_ptr(aw87390_of_match.as_ptr()) },
    },
    probe: Some(aw87390_i2c_probe),
    id_table: aw87390_i2c_id.as_ptr(),
};
/* module_i2c_driver(aw87390_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC AW87390 PA Driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
