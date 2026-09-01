// SPDX-License-Identifier: GPL-2.0-only
//
// aw88395.c --  ALSA SoC AW88395 codec support
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;

const NULL: *mut c_void = ptr::null_mut();
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const I2C_FUNC_I2C: c_uint = 0x0000_0001;
const GPIOD_OUT_LOW: c_uint = 0;
const REGMAP_ENDIAN_LITTLE: c_uint = 1;
const REGMAP_ENDIAN_BIG: c_uint = 2;
const SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_uint = 3;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_NOPM: c_int = -1;

extern "C" {
    static mut system_dfl_wq: *mut workqueue_struct;

    static AW88395_REG_MAX: c_uint;
    static AW88395_START_RETRIES: c_int;
    static AW88395_DSP_FW_UPDATE_ON: c_int;
    static AW88395_DSP_FW_UPDATE_OFF: c_int;
    static AW88395_DEV_FW_OK: c_int;
    static AW88395_DEV_FW_FAILED: c_int;
    static AW88395_DEV_PW_ON: c_int;
    static AW88395_SYNC_START: bool_;
    static AW88395_ASYNC_START: bool_;
    static AW88395_START_WORK_DELAY_MS: c_ulong;
    static AW88395_RATES: c_uint;
    static AW88395_FORMATS: c_ulong;
    static AW88395_MUTE_VOL: c_int;
    static FADE_TIME_MAX: c_int;
    static FADE_TIME_MIN: c_int;
    static AW88395_CALI_RE_MAX: c_int;
    static AW88395_SYSCTRL2_REG: c_uint;
    static AW88395_1000_US: c_uint;
    static AW88395_ACF_FILE: *const c_char;
    static AW88395_I2C_NAME: *const c_char;
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct delayed_work {
    work: work_struct,
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
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
struct i2c_client {
    dev: device,
    adapter: *mut i2c_adapter,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_ulong,
}

#[repr(C)]
struct snd_kcontrol {
    private_value: c_ulong,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
union snd_ctl_elem_info_value {
    enumerated: snd_ctl_elem_info_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_info_enumerated {
    items: c_uint,
    item: c_uint,
    name: [c_char; 64],
}

#[repr(C)]
struct soc_mixer_control {
    min: c_int,
    max: c_int,
}

#[repr(C)]
struct aw_volume_desc {
    ctl_volume: c_int,
}

#[repr(C)]
struct aw_cali_desc {
    cali_re: c_int,
}

#[repr(C)]
struct aw_device {
    dev: *mut device,
    fw_status: c_int,
    status: c_int,
    fade_in_time: c_int,
    fade_out_time: c_int,
    fade_step: c_int,
    volume_desc: aw_volume_desc,
    cali_desc: aw_cali_desc,
}

#[repr(C)]
struct aw_container {
    len: c_int,
    data: [u8; 0],
}

#[repr(C)]
struct firmware {
    size: usize,
    data: *const u8,
}

#[repr(C)]
struct aw88395 {
    aw_pa: *mut aw_device,
    lock: mutex,
    start_work: delayed_work,
    reset_gpio: *mut gpio_desc,
    regmap: *mut regmap,
    aw_cfg: *mut aw_container,
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
}

#[repr(C)]
struct i2c_device_id {
    name: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
}

#[repr(C)]
struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
}

extern "C" {
    fn aw88395_dev_start(aw_dev: *mut aw_device) -> c_int;
    fn aw88395_dev_fw_update(aw_dev: *mut aw_device, update: c_int, force: bool_) -> c_int;
    fn aw88395_dev_get_profile_count(aw_dev: *mut aw_device) -> c_int;
    fn aw88395_dev_get_prof_name(aw_dev: *mut aw_device, index: c_int, name: *mut *mut c_char) -> c_int;
    fn aw88395_dev_get_profile_index(aw_dev: *mut aw_device) -> c_int;
    fn aw88395_dev_set_profile_index(aw_dev: *mut aw_device, index: c_long) -> c_int;
    fn aw88395_dev_stop(aw_dev: *mut aw_device);
    fn aw88395_dev_set_volume(aw_dev: *mut aw_device, volume: c_int);
    fn aw88395_dev_load_acf_check(aw_dev: *mut aw_device, cfg: *mut aw_container) -> c_int;
    fn aw88395_dev_init(aw_dev: *mut aw_device, cfg: *mut aw_container) -> c_int;
    fn aw88395_init(aw_dev: *mut *mut aw_device, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widgets: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        routes: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num: c_int,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn queue_delayed_work(wq: *mut workqueue_struct, dwork: *mut delayed_work, delay: c_ulong) -> bool_;
    fn cancel_delayed_work_sync(dwork: *mut delayed_work) -> bool_;
    fn INIT_DELAYED_WORK(dwork: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn usleep_range(min: c_uint, max: c_uint);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn i2c_check_functionality(adapter: *mut i2c_adapter, functionality: c_uint) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn module_i2c_driver(driver: *mut i2c_driver);
}

unsafe fn aw_container_data(cfg: *mut aw_container) -> *mut u8 {
    (cfg as *mut u8).add(size_of::<aw_container>())
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        mutex_lock(lock);
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.lock);
        }
    }
}

static aw88395_remap_config: regmap_config = regmap_config {
    val_bits: 16,
    reg_bits: 8,
    max_register: unsafe { AW88395_REG_MAX - 1 },
    reg_format_endian: REGMAP_ENDIAN_LITTLE,
    val_format_endian: REGMAP_ENDIAN_BIG,
};

unsafe extern "C" fn aw88395_start_pa(aw88395: *mut aw88395) {
    let mut ret: c_int;
    let mut i: c_int = 0;

    while i < AW88395_START_RETRIES {
        ret = aw88395_dev_start((*aw88395).aw_pa);
        if ret != 0 {
            dev_err((*(*aw88395).aw_pa).dev, b"aw88395 device start failed. retry = %d\0".as_ptr() as *const c_char, i);
            ret = aw88395_dev_fw_update((*aw88395).aw_pa, AW88395_DSP_FW_UPDATE_ON, true);
            if ret < 0 {
                dev_err((*(*aw88395).aw_pa).dev, b"fw update failed\0".as_ptr() as *const c_char);
                i += 1;
                continue;
            }
        } else {
            dev_info((*(*aw88395).aw_pa).dev, b"start success\n\0".as_ptr() as *const c_char);
            break;
        }
        i += 1;
    }
}

unsafe extern "C" fn aw88395_startup_work(work: *mut work_struct) {
    let aw88395 = container_of_start_work_work(work);

    let _guard = MutexGuard::new(&mut (*aw88395).lock);
    aw88395_start_pa(aw88395);
}

unsafe fn container_of_start_work_work(work: *mut work_struct) -> *mut aw88395 {
    let offset = core::mem::offset_of!(aw88395, start_work) + core::mem::offset_of!(delayed_work, work);
    (work as *mut u8).sub(offset) as *mut aw88395
}

unsafe extern "C" fn aw88395_start(aw88395: *mut aw88395, sync_start: bool_) {
    let mut ret: c_int;

    if (*(*aw88395).aw_pa).fw_status != AW88395_DEV_FW_OK {
        return;
    }

    if (*(*aw88395).aw_pa).status == AW88395_DEV_PW_ON {
        return;
    }

    ret = aw88395_dev_fw_update((*aw88395).aw_pa, AW88395_DSP_FW_UPDATE_OFF, true);
    if ret < 0 {
        dev_err((*(*aw88395).aw_pa).dev, b"fw update failed.\0".as_ptr() as *const c_char);
        return;
    }

    if sync_start == AW88395_SYNC_START {
        aw88395_start_pa(aw88395);
    } else {
        queue_delayed_work(
            system_dfl_wq,
            &mut (*aw88395).start_work,
            AW88395_START_WORK_DELAY_MS,
        );
    }
}

static mut aw88395_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"aw88395-aif\0".as_ptr() as *const c_char,
    id: 1,
    playback: snd_soc_pcm_stream {
        stream_name: b"Speaker_Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { AW88395_RATES },
        formats: unsafe { AW88395_FORMATS },
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Speaker_Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { AW88395_RATES },
        formats: unsafe { AW88395_FORMATS },
    },
}];

unsafe extern "C" fn aw88395_get_fade_in_time(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(component) as *mut aw88395;
    let aw_dev = (*aw88395).aw_pa;

    (*ucontrol).value.integer.value[0] = (*aw_dev).fade_in_time as c_long;

    0
}

unsafe extern "C" fn aw88395_set_fade_in_time(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(component) as *mut aw88395;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev = (*aw88395).aw_pa;
    let time: c_int;

    time = (*ucontrol).value.integer.value[0] as c_int;

    if time < (*mc).min || time > (*mc).max {
        return -EINVAL;
    }

    if time != (*aw_dev).fade_in_time {
        (*aw_dev).fade_in_time = time;
        return 1;
    }

    0
}

unsafe extern "C" fn aw88395_get_fade_out_time(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(component) as *mut aw88395;
    let aw_dev = (*aw88395).aw_pa;

    (*ucontrol).value.integer.value[0] = (*aw_dev).fade_out_time as c_long;

    0
}

unsafe extern "C" fn aw88395_set_fade_out_time(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(component) as *mut aw88395;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev = (*aw88395).aw_pa;
    let time: c_int;

    time = (*ucontrol).value.integer.value[0] as c_int;
    if time < (*mc).min || time > (*mc).max {
        return -EINVAL;
    }

    if time != (*aw_dev).fade_out_time {
        (*aw_dev).fade_out_time = time;
        return 1;
    }

    0
}

unsafe extern "C" fn aw88395_profile_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;
    let mut prof_name: *mut c_char = ptr::null_mut();
    let mut count: c_int;
    let ret: c_int;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
    (*uinfo).count = 1;

    count = aw88395_dev_get_profile_count((*aw88395).aw_pa);
    if count <= 0 {
        (*uinfo).value.enumerated.items = 0;
        return 0;
    }

    (*uinfo).value.enumerated.items = count as c_uint;

    if (*uinfo).value.enumerated.item >= count as c_uint {
        (*uinfo).value.enumerated.item = (count - 1) as c_uint;
    }

    count = (*uinfo).value.enumerated.item as c_int;

    ret = aw88395_dev_get_prof_name((*aw88395).aw_pa, count, &mut prof_name);
    if ret != 0 {
        strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), b"null\0".as_ptr() as *const c_char);
        return 0;
    }

    strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), prof_name);

    0
}

unsafe extern "C" fn aw88395_profile_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;

    (*ucontrol).value.integer.value[0] = aw88395_dev_get_profile_index((*aw88395).aw_pa) as c_long;

    0
}

unsafe extern "C" fn aw88395_profile_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;
    let ret: c_int;

    /* pa stop or stopping just set profile */
    let _guard = MutexGuard::new(&mut (*aw88395).lock);
    ret = aw88395_dev_set_profile_index((*aw88395).aw_pa, (*ucontrol).value.integer.value[0]);
    if ret < 0 {
        dev_dbg((*codec).dev, b"profile index does not change\0".as_ptr() as *const c_char);
        return 0;
    }

    if (*(*aw88395).aw_pa).status != 0 {
        aw88395_dev_stop((*aw88395).aw_pa);
        aw88395_start(aw88395, AW88395_SYNC_START);
    }

    1
}

unsafe extern "C" fn aw88395_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;
    let vol_desc = &mut (*(*aw88395).aw_pa).volume_desc;

    (*ucontrol).value.integer.value[0] = vol_desc.ctl_volume as c_long;

    0
}

unsafe extern "C" fn aw88395_volume_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;
    let vol_desc = &mut (*(*aw88395).aw_pa).volume_desc;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let value: c_int;

    value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max {
        return -EINVAL;
    }

    if vol_desc.ctl_volume != value {
        vol_desc.ctl_volume = value;
        aw88395_dev_set_volume((*aw88395).aw_pa, vol_desc.ctl_volume);

        return 1;
    }

    0
}

unsafe extern "C" fn aw88395_get_fade_step(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;

    (*ucontrol).value.integer.value[0] = (*(*aw88395).aw_pa).fade_step as c_long;

    0
}

unsafe extern "C" fn aw88395_set_fade_step(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let value: c_int;

    value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max {
        return -EINVAL;
    }

    if (*(*aw88395).aw_pa).fade_step != value {
        (*(*aw88395).aw_pa).fade_step = value;
        return 1;
    }

    0
}

unsafe extern "C" fn aw88395_re_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;
    let aw_dev = (*aw88395).aw_pa;

    (*ucontrol).value.integer.value[0] = (*aw_dev).cali_desc.cali_re as c_long;

    0
}

unsafe extern "C" fn aw88395_re_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88395 = snd_soc_component_get_drvdata(codec) as *mut aw88395;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev = (*aw88395).aw_pa;
    let value: c_int;

    value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max {
        return -EINVAL;
    }

    if (*aw_dev).cali_desc.cali_re != value {
        (*aw_dev).cali_desc.cali_re = value;
        return 1;
    }

    0
}

/*
 * C control macro initializers translated as dependency-provided data intent:
 * SOC_SINGLE_EXT("PCM Playback Volume", AW88395_SYSCTRL2_REG,
 *     6, AW88395_MUTE_VOL, 0, aw88395_volume_get, aw88395_volume_set)
 * SOC_SINGLE_EXT("Fade Step", 0, 0, AW88395_MUTE_VOL, 0,
 *     aw88395_get_fade_step, aw88395_set_fade_step)
 * SOC_SINGLE_EXT("Volume Ramp Up Step", 0, 0, FADE_TIME_MAX, FADE_TIME_MIN,
 *     aw88395_get_fade_in_time, aw88395_set_fade_in_time)
 * SOC_SINGLE_EXT("Volume Ramp Down Step", 0, 0, FADE_TIME_MAX, FADE_TIME_MIN,
 *     aw88395_get_fade_out_time, aw88395_set_fade_out_time)
 * SOC_SINGLE_EXT("Calib", 0, 0, AW88395_CALI_RE_MAX, 0,
 *     aw88395_re_get, aw88395_re_set)
 * AW88395_PROFILE_EXT("Profile Set", aw88395_profile_info,
 *     aw88395_profile_get, aw88395_profile_set)
 */
extern "C" {
    static aw88395_controls: [snd_kcontrol_new; 6];
}

unsafe extern "C" fn aw88395_playback_event(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let aw88395 = snd_soc_component_get_drvdata(component) as *mut aw88395;

    let _guard = MutexGuard::new(&mut (*aw88395).lock);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            aw88395_start(aw88395, AW88395_ASYNC_START);
        }
        SND_SOC_DAPM_POST_PMD => {
            aw88395_dev_stop((*aw88395).aw_pa);
        }
        _ => {}
    }

    0
}

/*
 * C DAPM widget macro initializers preserved as dependency intent:
 * SND_SOC_DAPM_AIF_IN_E("AIF_RX", "Speaker_Playback", 0, 0, 0, 0,
 *     aw88395_playback_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD)
 * SND_SOC_DAPM_OUTPUT("DAC Output")
 * SND_SOC_DAPM_AIF_OUT("AIF_TX", "Speaker_Capture", 0, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_INPUT("ADC Input")
 */
extern "C" {
    static aw88395_dapm_widgets: [snd_soc_dapm_widget; 4];
}

static aw88395_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"DAC Output\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AIF_RX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AIF_TX\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ADC Input\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn aw88395_codec_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let aw88395 = snd_soc_component_get_drvdata(component) as *mut aw88395;
    let mut ret: c_int;

    INIT_DELAYED_WORK(&mut (*aw88395).start_work, aw88395_startup_work);

    /* add widgets */
    ret = snd_soc_dapm_new_controls(
        dapm,
        aw88395_dapm_widgets.as_ptr(),
        aw88395_dapm_widgets.len() as c_int,
    );
    if ret < 0 {
        return ret;
    }

    /* add route */
    ret = snd_soc_dapm_add_routes(
        dapm,
        aw88395_audio_map.as_ptr(),
        aw88395_audio_map.len() as c_int,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_add_component_controls(
        component,
        aw88395_controls.as_ptr(),
        aw88395_controls.len() as c_int,
    );

    ret
}

unsafe extern "C" fn aw88395_codec_remove(aw_codec: *mut snd_soc_component) {
    let aw88395 = snd_soc_component_get_drvdata(aw_codec) as *mut aw88395;

    cancel_delayed_work_sync(&mut (*aw88395).start_work);
}

static soc_codec_dev_aw88395: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(aw88395_codec_probe),
    remove: Some(aw88395_codec_remove),
};

unsafe extern "C" fn aw88395_malloc_init(i2c: *mut i2c_client) -> *mut aw88395 {
    let aw88395 = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<aw88395>(),
        GFP_KERNEL,
    ) as *mut aw88395;
    if aw88395.is_null() {
        return ptr::null_mut();
    }

    mutex_init(&mut (*aw88395).lock);

    aw88395
}

unsafe extern "C" fn aw88395_hw_reset(aw88395: *mut aw88395) {
    if !(*aw88395).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*aw88395).reset_gpio, 0);
        usleep_range(AW88395_1000_US, AW88395_1000_US + 10);
        gpiod_set_value_cansleep((*aw88395).reset_gpio, 1);
        usleep_range(AW88395_1000_US, AW88395_1000_US + 10);
    }
}

unsafe extern "C" fn aw88395_request_firmware_file(aw88395: *mut aw88395) -> c_int {
    let mut cont: *const firmware = ptr::null();
    let mut aw_cfg: *mut aw_container;
    let mut ret: c_int;

    (*(*aw88395).aw_pa).fw_status = AW88395_DEV_FW_FAILED;

    ret = request_firmware(&mut cont, AW88395_ACF_FILE, (*(*aw88395).aw_pa).dev);
    if ret < 0 || cont.is_null() {
        dev_err((*(*aw88395).aw_pa).dev, b"load [%s] failed!\0".as_ptr() as *const c_char, AW88395_ACF_FILE);
        return ret;
    }

    dev_info(
        (*(*aw88395).aw_pa).dev,
        b"loaded %s - size: %zu\n\0".as_ptr() as *const c_char,
        AW88395_ACF_FILE,
        if !cont.is_null() { (*cont).size } else { 0 },
    );

    aw_cfg = devm_kzalloc(
        (*(*aw88395).aw_pa).dev,
        size_of::<aw_container>() + (*cont).size,
        GFP_KERNEL,
    ) as *mut aw_container;
    if aw_cfg.is_null() {
        release_firmware(cont);
        return -ENOMEM;
    }

    (*aw_cfg).len = (*cont).size as c_int;
    memcpy(
        aw_container_data(aw_cfg) as *mut c_void,
        (*cont).data as *const c_void,
        (*cont).size,
    );

    (*aw88395).aw_cfg = aw_cfg;

    ret = aw88395_dev_load_acf_check((*aw88395).aw_pa, (*aw88395).aw_cfg);
    if ret < 0 {
        dev_err((*(*aw88395).aw_pa).dev, b"Load [%s] failed ....!\0".as_ptr() as *const c_char, AW88395_ACF_FILE);
        release_firmware(cont);
        return ret;
    }

    dev_dbg((*(*aw88395).aw_pa).dev, b"%s : bin load success\n\0".as_ptr() as *const c_char, b"aw88395_request_firmware_file\0".as_ptr() as *const c_char);

    {
        let _guard = MutexGuard::new(&mut (*aw88395).lock);
        /* aw device init */
        ret = aw88395_dev_init((*aw88395).aw_pa, (*aw88395).aw_cfg);
        if ret < 0 {
            dev_err((*(*aw88395).aw_pa).dev, b"dev init failed\0".as_ptr() as *const c_char);
        }
    }

    release_firmware(cont);
    ret
}

unsafe extern "C" fn aw88395_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let aw88395: *mut aw88395;
    let mut ret: c_int;

    if i2c_check_functionality((*i2c).adapter, I2C_FUNC_I2C) == 0 {
        dev_err(&mut (*i2c).dev, b"check_functionality failed\0".as_ptr() as *const c_char);
        return -EIO;
    }

    aw88395 = aw88395_malloc_init(i2c);
    if aw88395.is_null() {
        dev_err(&mut (*i2c).dev, b"malloc aw88395 failed\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    i2c_set_clientdata(i2c, aw88395 as *mut c_void);

    (*aw88395).reset_gpio = devm_gpiod_get_optional(&mut (*i2c).dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*aw88395).reset_gpio as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*aw88395).reset_gpio as *const c_void),
            b"failed to get reset gpio\n\0".as_ptr() as *const c_char,
        );
    }
    /* hardware reset */
    aw88395_hw_reset(aw88395);

    (*aw88395).regmap = devm_regmap_init_i2c(i2c, &aw88395_remap_config);
    if IS_ERR((*aw88395).regmap as *const c_void) {
        ret = PTR_ERR((*aw88395).regmap as *const c_void) as c_int;
        dev_err(&mut (*i2c).dev, b"Failed to init regmap: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* aw pa init */
    ret = aw88395_init(&mut (*aw88395).aw_pa, i2c, (*aw88395).regmap);
    if ret < 0 {
        return ret;
    }

    ret = aw88395_request_firmware_file(aw88395);
    if ret < 0 {
        dev_err(&mut (*i2c).dev, b"%s failed\n\0".as_ptr() as *const c_char, b"aw88395_i2c_probe\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_codec_dev_aw88395,
        aw88395_dai.as_mut_ptr(),
        aw88395_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(&mut (*i2c).dev, b"failed to register aw88395: %d\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

static aw88395_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: unsafe { AW88395_I2C_NAME },
    },
    i2c_device_id {
        name: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(i2c, aw88395_i2c_id); */

static mut aw88395_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: unsafe { AW88395_I2C_NAME },
    },
    probe: Some(aw88395_i2c_probe),
    id_table: aw88395_i2c_id.as_ptr(),
};

unsafe extern "C" fn aw88395_module_init() {
    module_i2c_driver(&mut aw88395_i2c_driver);
}

/* MODULE_DESCRIPTION("ASoC AW88395 Smart PA Driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
