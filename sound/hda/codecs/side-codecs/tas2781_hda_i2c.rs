// SPDX-License-Identifier: GPL-2.0
//
// TAS2781 HDA I2C driver
//
// Copyright 2023 - 2026 Texas Instruments, Inc.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
// Current maintainer: Baojun Xu <baojun.xu@ti.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type uint32_t = u32;
type efi_status_t = c_long;
type efi_char16_t = u16;
type efi_guid_t = [u8; 16];
type __be32 = u32;

const TAS2563_CAL_VAR_NAME_MAX: usize = 16;
const TAS2563_CAL_ARRAY_SIZE: usize = 80;
const TAS2563_CAL_DATA_SIZE: usize = 4;
const TAS2563_MAX_CHANNELS: usize = 4;
const TAS2563_CAL_CH_SIZE: usize = 20;

const fn TASDEVICE_REG(book: u32, page: u32, reg: u32) -> u32 {
    (book << 16) | (page << 8) | reg
}

const TAS2563_CAL_R0_LOW: u32 = TASDEVICE_REG(0, 0x0f, 0x48);
const TAS2563_CAL_POWER: u32 = TASDEVICE_REG(0, 0x0d, 0x3c);
const TAS2563_CAL_INVR0: u32 = TASDEVICE_REG(0, 0x0f, 0x40);
const TAS2563_CAL_TLIM: u32 = TASDEVICE_REG(0, 0x10, 0x14);
const TAS2563_CAL_R0: u32 = TASDEVICE_REG(0, 0x0f, 0x34);

const HDA_TAS2563: c_int = 0;
const HDA_TAS2770: c_int = 1;
const HDA_TAS2781: c_int = 2;
const HDA_TAS5825: c_int = 3;
const HDA_OTHERS: c_int = 4;

#[repr(C)]
struct device_chip_id {
    _unused: [u8; 0],
}

#[repr(C)]
struct tas2781_hda_i2c_priv {
    snd_ctls: [*mut snd_kcontrol; 2],
    save_calibration: Option<unsafe extern "C" fn(h: *mut tas2781_hda) -> c_int>,
    hda_chip_id: c_int,
}

#[repr(C)]
struct acpi_gpio_params {
    crs_entry_index: c_uint,
    line_index: c_uint,
    active_low: bool_,
}

#[repr(C)]
struct acpi_gpio_mapping {
    name: *const c_char,
    data: *const acpi_gpio_params,
    size: c_uint,
}

#[repr(C)]
struct snd_kcontrol_new {
    name: *const c_char,
    iface: c_int,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: c_ulong,
    tlv: *const c_uint,
}

#[repr(C)]
struct i2c_device_id {
    name: [c_char; 20],
}

#[repr(C)]
struct acpi_device_id {
    id: [c_char; 16],
    driver_data: c_ulong,
}

#[repr(C)]
struct component_ops {
    bind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void) -> c_int>,
    unbind: Option<unsafe extern "C" fn(*mut device, *mut device, *mut c_void)>,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    system_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    system_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    acpi_match_table: *const acpi_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct i2c_driver {
    driver: device_driver,
    id_table: *const i2c_device_id,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

#[repr(C)]
struct acpi_resource_i2c_serialbus {
    slave_address: u16,
}

#[repr(C)]
struct tasdevice {
    dev_addr: c_uint,
    cur_book: c_int,
    cur_prog: c_int,
    cur_conf: c_int,
}

#[repr(C)]
struct cali_reg {
    r0_reg: c_uint,
    invr0_reg: c_uint,
    r0_low_reg: c_uint,
    pow_reg: c_uint,
    tlimit_reg: c_uint,
}

#[repr(C)]
struct calidata {
    cali_dat_sz_per_dev: c_ulong,
    total_sz: c_ulong,
    data: *mut u8,
    cali_reg_array: cali_reg,
}

#[repr(C)]
struct rcabin {
    init_profile_id: c_int,
}

#[repr(C)]
struct tasdevice_fmw {
    nr_programs: c_int,
    nr_configurations: c_int,
}

#[repr(C)]
struct tasdevice_priv {
    dev: *mut device,
    codec: *mut hda_codec,
    codec_lock: mutex,
    ndev: c_int,
    tasdevice: [tasdevice; TASDEVICE_MAX_CHANNELS as usize],
    global_addr: c_uint,
    speaker_id: c_int,
    dev_name: [c_char; 32],
    playback_started: bool_,
    force_fwload_status: bool_,
    cali_data: calidata,
    fw_state: c_int,
    coef_binaryname: [c_char; 64],
    fmw: *mut tasdevice_fmw,
    cur_prog: c_int,
    cur_conf: c_int,
    rcabin: rcabin,
    index: c_int,
    chip_id: c_int,
    irq: c_int,
}

#[repr(C)]
struct tas2781_hda {
    snd_ctls: [*mut snd_kcontrol; 2],
    dsp_prog_ctl: *mut snd_kcontrol,
    dsp_conf_ctl: *mut snd_kcontrol,
    prof_ctl: *mut snd_kcontrol,
    hda_priv: *mut tas2781_hda_i2c_priv,
    priv_: *mut tasdevice_priv,
    dev: *mut device,
    catlog_id: c_int,
}

#[repr(C)]
struct hda_component_parent {
    codec: *mut hda_codec,
}

#[repr(C)]
struct hda_component {
    dev: *mut device,
    name: [c_char; 32],
    playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
}

#[repr(C)]
struct hda_codec_core {
    subsystem_id: c_uint,
}

#[repr(C)]
struct hda_codec {
    card: *mut snd_card,
    core: hda_codec_core,
}

#[repr(C)]
struct i2c_client {
    dev: device,
    irq: c_int,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
struct snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
struct snd_kcontrol_id {
    name: [c_char; 44],
}

#[repr(C)]
struct snd_kcontrol {
    id: snd_kcontrol_id,
    private_value: c_ulong,
}

#[repr(C)]
struct soc_mixer_control {
    _opaque: [u8; 0],
}

#[repr(C)]
struct efi {
    get_variable: unsafe extern "C" fn(*mut efi_char16_t, *mut efi_guid_t, *mut c_uint, *mut c_ulong, *mut u8) -> efi_status_t,
}

#[repr(C)]
struct device {
    _opaque: [u8; 0],
}
#[repr(C)]
struct acpi_resource {
    _opaque: [u8; 0],
}
#[repr(C)]
struct acpi_device {
    _opaque: [u8; 0],
}
#[repr(C)]
struct gpio_desc {
    _opaque: [u8; 0],
}
#[repr(C)]
struct firmware {
    _opaque: [u8; 0],
}
#[repr(C)]
struct snd_card {
    _opaque: [u8; 0],
}
#[repr(C)]
struct snd_ctl_elem_info {
    _opaque: [u8; 0],
}
#[repr(C)]
struct mutex {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    static mut efi: efi;
    static tasdev_fct_efi_guid: [efi_guid_t; 8];
    static tas2770_amp_tlv: [c_uint; 0];
    static tas2770_dvc_tlv: [c_uint; 0];
    static tas2781_amp_tlv: [c_uint; 0];
    static tas5825_amp_tlv: [c_uint; 0];
    static tas5825_dvc_tlv: [c_uint; 0];
    static THIS_MODULE: *mut c_void;

    fn i2c_acpi_get_i2c_resource(ares: *mut acpi_resource, sb: *mut *mut acpi_resource_i2c_serialbus) -> bool_;
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_void, hrv: c_long) -> *mut acpi_device;
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn acpi_dev_get_resources(adev: *mut acpi_device, resources: *mut list_head, func: Option<unsafe extern "C" fn(*mut acpi_resource, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    fn acpi_get_subsystem_id(handle: *mut c_void) -> *const c_char;
    fn acpi_dev_add_driver_gpios(adev: *mut acpi_device, gpios: *const acpi_gpio_mapping) -> c_int;
    fn acpi_fwnode_handle(adev: *mut acpi_device) -> *mut c_void;
    fn fwnode_gpiod_get_index(fwnode: *mut c_void, con_id: *const c_char, index: c_uint, flags: c_int, label: *const c_char) -> *mut gpio_desc;
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_put(desc: *mut gpio_desc);
    fn acpi_dev_remove_driver_gpios(adev: *mut acpi_device);
    fn acpi_dev_free_resource_list(resources: *mut list_head);
    fn acpi_dev_put(adev: *mut acpi_device);
    fn kstrtou32(s: *const c_char, base: c_uint, res: *mut u32) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool_;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snprintf(buf: *mut u8, size: usize, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol);
    fn tasdevice_amp_getvol(tas_priv: *mut tasdevice_priv, ucontrol: *mut snd_ctl_elem_value, mc: *mut soc_mixer_control) -> c_int;
    fn tasdevice_amp_putvol(tas_priv: *mut tasdevice_priv, ucontrol: *mut snd_ctl_elem_value, mc: *mut soc_mixer_control) -> c_int;
    fn tasdevice_info_profile(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;
    fn tasdevice_get_profile_id(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_set_profile_id(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_info_programs(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;
    fn tasdevice_program_get(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_program_put(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_info_config(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;
    fn tasdevice_config_get(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
    fn tasdevice_config_put(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
    fn tas2781_save_calibration(h: *mut tas2781_hda) -> c_int;
    fn tasdevice_dsp_remove(tas_priv: *mut tasdevice_priv);
    fn tasdevice_dsp_parser(tas_priv: *mut tasdevice_priv) -> c_int;
    fn tasdevice_prmg_load(tas_priv: *mut tasdevice_priv, prog: c_int);
    fn tasdevice_select_cfg_blk(tas_priv: *mut tasdevice_priv, profile_id: c_int, blk: c_int);
    fn tasdevice_rca_parser(tas_priv: *mut tasdevice_priv, fmw: *const firmware) -> c_int;
    fn release_firmware(fmw: *const firmware);
    fn hda_component_from_index(parent: *mut hda_component_parent, index: c_int) -> *mut hda_component;
    fn tascodec_init(tas_priv: *mut tasdevice_priv, codec: *mut hda_codec, module: *mut c_void, cb: Option<unsafe extern "C" fn(*const firmware, *mut c_void)>) -> c_int;
    fn request_firmware_nowait_cancel(dev: *mut device, context: *mut tasdevice_priv, cb: Option<unsafe extern "C" fn(*const firmware, *mut c_void)>);
    fn tasdevice_config_info_remove(tas_priv: *mut tasdevice_priv);
    fn tasdevice_kzalloc(clt: *mut i2c_client) -> *mut tasdevice_priv;
    fn tas2781_hda_remove(dev: *mut device, ops: *const component_ops);
    fn tasdevice_init(tas_priv: *mut tasdevice_priv) -> c_int;
    fn tasdevice_reset(tas_priv: *mut tasdevice_priv);
    fn component_add(dev: *mut device, ops: *const component_ops) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn tasdevice_tuning_switch(tas_priv: *mut tasdevice_priv, state: c_int, force: bool_);
    fn efi_rt_services_supported(mask: c_ulong) -> bool_;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

const TASDEVICE_MAX_CHANNELS: c_int = 4;
const TASDEV_CALIB_N: c_int = 5;
const LENOVO: usize = 0;
const DELL: c_int = 1;
const HP: c_int = 2;
const PCI_VENDOR_ID_ASUSTEK: u32 = 0x1043;
const EFI_RT_SUPPORTED_GET_VARIABLE: c_ulong = 0x0001;
const EFI_SUCCESS: efi_status_t = 0;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const GPIOD_IN: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_CARD: c_int = 0;
const HDA_GEN_PCM_ACT_OPEN: c_int = 0;
const HDA_GEN_PCM_ACT_CLOSE: c_int = 1;
const TASDEVICE_DSP_FW_PENDING: c_int = 0;
const TASDEVICE_DSP_FW_FAIL: c_int = 1;
const TASDEVICE_DSP_FW_ALL_OK: c_int = 2;
const TASDEVICE_RCA_FW_OK: c_int = 3;
const TASDEVICE_BIN_BLK_PRE_POWER_UP: c_int = 0;
const TAS2781: c_int = 2781;
const TAS5825: c_int = 5825;
const TAS2781_GLOBAL_ADDR: c_uint = 0;
const TAS2563_GLOBAL_ADDR: c_uint = 0;
const TAS2770_AMP_LEVEL: c_uint = 0;
const TAS2770_DVC_LEVEL: c_uint = 0;
const TAS2781_AMP_LEVEL: c_uint = 0;
const TAS5825_AMP_LEVEL: c_uint = 0;
const TAS5825_DVC_LEVEL: c_uint = 0;

unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

fn upper_16_bits(x: u32) -> u32 {
    x >> 16
}

fn lower_16_bits(x: u32) -> u32 {
    x & 0xffff
}

fn cpu_to_be32(x: u32) -> u32 {
    x.to_be()
}

struct MutexGuard(*mut mutex);

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.0) }
    }
}

unsafe fn guard_mutex(lock: *mut mutex) -> MutexGuard {
    mutex_lock(lock);
    MutexGuard(lock)
}

const speakerid_gpios: acpi_gpio_params = acpi_gpio_params {
    crs_entry_index: 0,
    line_index: 0,
    active_low: false,
};

static tas2781_speaker_id_gpios: [acpi_gpio_mapping; 2] = [
    acpi_gpio_mapping {
        name: b"speakerid-gpios\0".as_ptr() as *const c_char,
        data: &speakerid_gpios,
        size: 1,
    },
    acpi_gpio_mapping {
        name: ptr::null(),
        data: ptr::null(),
        size: 0,
    },
];

unsafe extern "C" fn tas2781_get_i2c_res(ares: *mut acpi_resource, data: *mut c_void) -> c_int {
    let tas_priv = data as *mut tasdevice_priv;
    let mut sb: *mut acpi_resource_i2c_serialbus = ptr::null_mut();

    if i2c_acpi_get_i2c_resource(ares, &mut sb) {
        if (*tas_priv).ndev < TASDEVICE_MAX_CHANNELS
            && (*sb).slave_address as c_uint != (*tas_priv).global_addr
        {
            (*tas_priv).tasdevice[(*tas_priv).ndev as usize].dev_addr = (*sb).slave_address as c_uint;
            (*tas_priv).ndev += 1;
        }
    }
    1
}

unsafe extern "C" fn tas2781_read_acpi(p: *mut tasdevice_priv, hid: *const c_char) -> c_int {
    let mut speaker_id: *mut gpio_desc;
    let adev: *mut acpi_device;
    let mut resources = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    let sub: *const c_char;
    let mut subid: uint32_t = 0;
    let mut ret: c_int;

    adev = acpi_dev_get_first_match_dev(hid, ptr::null(), -1);
    if adev.is_null() {
        dev_err((*p).dev, b"Failed to find an ACPI device for %s\n\0".as_ptr() as *const c_char, hid);
        return -ENODEV;
    }

    let physdev = get_device(acpi_get_first_physical_node(adev));
    ret = acpi_dev_get_resources(adev, &mut resources, Some(tas2781_get_i2c_res), p as *mut c_void);
    if ret < 0 {
        dev_err((*p).dev, b"Failed to get ACPI resource.\n\0".as_ptr() as *const c_char);
        put_device(physdev);
        dev_err((*p).dev, b"read acpi error, ret: %d\n\0".as_ptr() as *const c_char, ret);
        acpi_dev_put(adev);
        return ret;
    }
    sub = acpi_get_subsystem_id(physdev as *mut c_void);
    if IS_ERR(sub) {
        /* No subsys id in older tas2563 projects. */
        if strncmp(hid, b"INT8866\0".as_ptr() as *const c_char, size_of::<[u8; 8]>()) == 0 {
            (*p).speaker_id = -1;
            acpi_dev_free_resource_list(&mut resources);
            strscpy((*p).dev_name.as_mut_ptr(), hid, (*p).dev_name.len());
            acpi_dev_put(adev);
            put_device(physdev);
            return 0;
        }
        dev_err((*p).dev, b"Failed to get SUBSYS ID.\n\0".as_ptr() as *const c_char);
        ret = PTR_ERR(sub);
        put_device(physdev);
        dev_err((*p).dev, b"read acpi error, ret: %d\n\0".as_ptr() as *const c_char, ret);
        acpi_dev_put(adev);
        return ret;
    }
    /* Speaker id was needed for ASUS projects. */
    ret = kstrtou32(sub, 16, &mut subid);
    if ret == 0 && upper_16_bits(subid) == PCI_VENDOR_ID_ASUSTEK {
        ret = acpi_dev_add_driver_gpios(adev, tas2781_speaker_id_gpios.as_ptr());
        if ret < 0 {
            dev_err((*p).dev, b"Failed to add driver gpio %d.\n\0".as_ptr() as *const c_char, ret);
            (*p).speaker_id = -1;
            acpi_dev_free_resource_list(&mut resources);
            strscpy((*p).dev_name.as_mut_ptr(), hid, (*p).dev_name.len());
            acpi_dev_put(adev);
            put_device(physdev);
            return 0;
        }

        speaker_id = fwnode_gpiod_get_index(
            acpi_fwnode_handle(adev),
            b"speakerid\0".as_ptr() as *const c_char,
            0,
            GPIOD_IN,
            ptr::null(),
        );
        if !IS_ERR(speaker_id) {
            (*p).speaker_id = gpiod_get_value_cansleep(speaker_id);
            dev_dbg((*p).dev, b"Got speaker id gpio from ACPI: %d.\n\0".as_ptr() as *const c_char, (*p).speaker_id);
            gpiod_put(speaker_id);
        } else {
            (*p).speaker_id = -1;
            ret = PTR_ERR(speaker_id);
            dev_err((*p).dev, b"Get speaker id gpio failed %d.\n\0".as_ptr() as *const c_char, ret);
        }

        acpi_dev_remove_driver_gpios(adev);
    } else {
        (*p).speaker_id = -1;
    }

    acpi_dev_free_resource_list(&mut resources);
    strscpy((*p).dev_name.as_mut_ptr(), hid, (*p).dev_name.len());
    acpi_dev_put(adev);
    put_device(physdev);

    0
}

unsafe extern "C" fn tas2781_hda_playback_hook(dev: *mut device, action: c_int) {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;

    dev_dbg((*tas_hda).dev, b"%s: action = %d\n\0".as_ptr() as *const c_char, b"tas2781_hda_playback_hook\0".as_ptr() as *const c_char, action);
    match action {
        HDA_GEN_PCM_ACT_OPEN => {
            pm_runtime_get_sync(dev);
            let _guard = guard_mutex(&mut (*(*tas_hda).priv_).codec_lock);
            tasdevice_tuning_switch((*tas_hda).priv_, 0, false);
            (*(*tas_hda).priv_).playback_started = true;
        }
        HDA_GEN_PCM_ACT_CLOSE => {
            {
                let _guard = guard_mutex(&mut (*(*tas_hda).priv_).codec_lock);
                tasdevice_tuning_switch((*tas_hda).priv_, 1, false);
                (*(*tas_hda).priv_).playback_started = false;
            }
            pm_runtime_put_autosuspend(dev);
        }
        _ => {}
    }
}

unsafe extern "C" fn tas2781_amp_getvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);

    let ret = tasdevice_amp_getvol(tas_priv, ucontrol, mc);

    dev_dbg(
        (*tas_priv).dev,
        b"%s: kcontrol %s: %ld\n\0".as_ptr() as *const c_char,
        b"tas2781_amp_getvol\0".as_ptr() as *const c_char,
        (*kcontrol).id.name.as_ptr(),
        (*ucontrol).value.integer.value[0],
    );

    ret
}

unsafe extern "C" fn tas2781_amp_putvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;

    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);

    dev_dbg(
        (*tas_priv).dev,
        b"%s: kcontrol %s: -> %ld\n\0".as_ptr() as *const c_char,
        b"tas2781_amp_putvol\0".as_ptr() as *const c_char,
        (*kcontrol).id.name.as_ptr(),
        (*ucontrol).value.integer.value[0],
    );

    /* The check of the given value is in tasdevice_amp_putvol. */
    tasdevice_amp_putvol(tas_priv, ucontrol, mc)
}

unsafe extern "C" fn tas2781_force_fwload_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;

    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);

    (*ucontrol).value.integer.value[0] = (*tas_priv).force_fwload_status as c_int as c_long;
    dev_dbg(
        (*tas_priv).dev,
        b"%s: kcontrol %s: %d\n\0".as_ptr() as *const c_char,
        b"tas2781_force_fwload_get\0".as_ptr() as *const c_char,
        (*kcontrol).id.name.as_ptr(),
        (*tas_priv).force_fwload_status as c_int,
    );

    0
}

unsafe extern "C" fn tas2781_force_fwload_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let tas_priv = snd_kcontrol_chip(kcontrol) as *mut tasdevice_priv;
    let val = (*ucontrol).value.integer.value[0] != 0;
    let change: bool_;

    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);

    dev_dbg(
        (*tas_priv).dev,
        b"%s: kcontrol %s: %d -> %d\n\0".as_ptr() as *const c_char,
        b"tas2781_force_fwload_put\0".as_ptr() as *const c_char,
        (*kcontrol).id.name.as_ptr(),
        (*tas_priv).force_fwload_status as c_int,
        val as c_int,
    );

    if (*tas_priv).force_fwload_status == val {
        change = false;
    } else {
        change = true;
        (*tas_priv).force_fwload_status = val;
    }

    change as c_int
}

const fn snd_control(name: *const c_char, get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, tlv: *const c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new { name, iface: SNDRV_CTL_ELEM_IFACE_CARD, info: None, get, put, private_value: 0, tlv }
}

static tas2770_snd_controls: [snd_kcontrol_new; 2] = [
    snd_control(b"Speaker Analog Volume\0".as_ptr() as *const c_char, Some(tas2781_amp_getvol), Some(tas2781_amp_putvol), unsafe { tas2770_amp_tlv.as_ptr() }),
    snd_control(b"Speaker Digital Volume\0".as_ptr() as *const c_char, Some(tas2781_amp_getvol), Some(tas2781_amp_putvol), unsafe { tas2770_dvc_tlv.as_ptr() }),
];

static tas2781_snd_controls: [snd_kcontrol_new; 2] = [
    snd_control(b"Speaker Analog Volume\0".as_ptr() as *const c_char, Some(tas2781_amp_getvol), Some(tas2781_amp_putvol), unsafe { tas2781_amp_tlv.as_ptr() }),
    snd_control(b"Speaker Force Firmware Load\0".as_ptr() as *const c_char, Some(tas2781_force_fwload_get), Some(tas2781_force_fwload_put), ptr::null()),
];

static tas5825_snd_controls: [snd_kcontrol_new; 3] = [
    snd_control(b"Speaker Analog Volume\0".as_ptr() as *const c_char, Some(tas2781_amp_getvol), Some(tas2781_amp_putvol), unsafe { tas5825_amp_tlv.as_ptr() }),
    snd_control(b"Speaker Digital Volume\0".as_ptr() as *const c_char, Some(tas2781_amp_getvol), Some(tas2781_amp_putvol), unsafe { tas5825_dvc_tlv.as_ptr() }),
    snd_control(b"Speaker Force Firmware Load\0".as_ptr() as *const c_char, Some(tas2781_force_fwload_get), Some(tas2781_force_fwload_put), ptr::null()),
];

static tasdevice_prof_ctrl: snd_kcontrol_new = snd_kcontrol_new {
    name: b"Speaker Profile Id\0".as_ptr() as *const c_char,
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    info: Some(tasdevice_info_profile),
    get: Some(tasdevice_get_profile_id),
    put: Some(tasdevice_set_profile_id),
    private_value: 0,
    tlv: ptr::null(),
};

static tasdevice_dsp_prog_ctrl: snd_kcontrol_new = snd_kcontrol_new {
    name: b"Speaker Program Id\0".as_ptr() as *const c_char,
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    info: Some(tasdevice_info_programs),
    get: Some(tasdevice_program_get),
    put: Some(tasdevice_program_put),
    private_value: 0,
    tlv: ptr::null(),
};

static tasdevice_dsp_conf_ctrl: snd_kcontrol_new = snd_kcontrol_new {
    name: b"Speaker Config Id\0".as_ptr() as *const c_char,
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    info: Some(tasdevice_info_config),
    get: Some(tasdevice_config_get),
    put: Some(tasdevice_config_put),
    private_value: 0,
    tlv: ptr::null(),
};

unsafe extern "C" fn tas2563_save_calibration(h: *mut tas2781_hda) -> c_int {
    let mut efi_guid = tasdev_fct_efi_guid[LENOVO];
    let vars: [*const c_char; TASDEV_CALIB_N as usize] = [
        b"R0_%d\0".as_ptr() as *const c_char,
        b"R0_Low_%d\0".as_ptr() as *const c_char,
        b"InvR0_%d\0".as_ptr() as *const c_char,
        b"Power_%d\0".as_ptr() as *const c_char,
        b"TLim_%d\0".as_ptr() as *const c_char,
    ];
    let mut efi_name: [efi_char16_t; TAS2563_CAL_VAR_NAME_MAX] = [0; TAS2563_CAL_VAR_NAME_MAX];
    let mut max_size: c_ulong = TAS2563_CAL_DATA_SIZE as c_ulong;
    let mut var8: [u8; TAS2563_CAL_VAR_NAME_MAX] = [0; TAS2563_CAL_VAR_NAME_MAX];
    let p = (*h).priv_;
    let cd = &mut (*p).cali_data as *mut calidata;
    let r = &mut (*cd).cali_reg_array as *mut cali_reg;
    let mut offset: c_uint = 0;
    let data: *mut u8;
    let mut bedata: __be32;
    let mut status: efi_status_t;
    let mut attr: c_uint = 0;
    let mut ret: c_int;

    if !efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE) {
        dev_err((*p).dev, b"%s: NO EFI FOUND!\n\0".as_ptr() as *const c_char, b"tas2563_save_calibration\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    (*cd).cali_dat_sz_per_dev = (TAS2563_CAL_DATA_SIZE as c_int * TASDEV_CALIB_N) as c_ulong;

    /* extra byte for each device is the device number */
    (*cd).total_sz = ((*cd).cali_dat_sz_per_dev + 1) * (*p).ndev as c_ulong;
    data = devm_kzalloc((*p).dev, (*cd).total_sz as usize, GFP_KERNEL) as *mut u8;
    (*cd).data = data;
    if data.is_null() {
        return -ENOMEM;
    }

    for i in 0..(*p).ndev {
        *data.add(offset as usize) = i as u8;
        offset += 1;
        for j in 0..TASDEV_CALIB_N {
            /* EFI name for calibration started with 1, not 0 */
            ret = snprintf(var8.as_mut_ptr(), var8.len(), vars[j as usize], i + 1);
            if ret < 0 || ret as usize >= var8.len() - 1 {
                dev_err((*p).dev, b"%s: Read %s failed\n\0".as_ptr() as *const c_char, b"tas2563_save_calibration\0".as_ptr() as *const c_char, var8.as_ptr());
                (*cd).total_sz = 0;
                return -EINVAL;
            }
            /*
             * Our variable names are ASCII by construction, but
             * EFI names are wide chars.  Convert and zero-pad.
             */
            memset(efi_name.as_mut_ptr() as *mut c_void, 0, size_of::<[efi_char16_t; TAS2563_CAL_VAR_NAME_MAX]>());
            let mut k = 0usize;
            while k < var8.len() && var8[k] != 0 {
                efi_name[k] = var8[k] as efi_char16_t;
                k += 1;
            }
            status = (efi.get_variable)(
                efi_name.as_mut_ptr(),
                &mut efi_guid,
                &mut attr,
                &mut max_size,
                data.add(offset as usize),
            );
            if status != EFI_SUCCESS || max_size != TAS2563_CAL_DATA_SIZE as c_ulong {
                dev_warn((*p).dev, b"Dev %d: Caldat[%d] read failed %ld\n\0".as_ptr() as *const c_char, i, j, status);
                (*cd).total_sz = 0;
                return -EINVAL;
            }
            bedata = cpu_to_be32(*(data.add(offset as usize) as *mut uint32_t));
            memcpy(data.add(offset as usize) as *mut c_void, &mut bedata as *mut __be32 as *const c_void, size_of::<__be32>());
            offset += TAS2563_CAL_DATA_SIZE as c_uint;
        }
    }

    if (*cd).total_sz != offset as c_ulong {
        dev_err((*p).dev, b"%s: tot_size(%lu) and offset(%u) mismatch\n\0".as_ptr() as *const c_char, b"tas2563_save_calibration\0".as_ptr() as *const c_char, (*cd).total_sz, offset);
        (*cd).total_sz = 0;
        return -EINVAL;
    }

    (*r).r0_reg = TAS2563_CAL_R0;
    (*r).invr0_reg = TAS2563_CAL_INVR0;
    (*r).r0_low_reg = TAS2563_CAL_R0_LOW;
    (*r).pow_reg = TAS2563_CAL_POWER;
    (*r).tlimit_reg = TAS2563_CAL_TLIM;

    0
}

unsafe extern "C" fn tas2781_hda_remove_controls(tas_hda: *mut tas2781_hda) {
    let hda_priv = (*tas_hda).hda_priv;
    let codec = (*(*tas_hda).priv_).codec;

    snd_ctl_remove((*codec).card, (*tas_hda).dsp_prog_ctl);
    snd_ctl_remove((*codec).card, (*tas_hda).dsp_conf_ctl);

    let mut i = (*hda_priv).snd_ctls.len() as c_int - 1;
    while i >= 0 {
        snd_ctl_remove((*codec).card, (*hda_priv).snd_ctls[i as usize]);
        i -= 1;
    }

    snd_ctl_remove((*codec).card, (*tas_hda).prof_ctl);
}

unsafe extern "C" fn tasdev_add_kcontrols(tas_priv: *mut tasdevice_priv, ctls: *mut *mut snd_kcontrol, codec: *mut hda_codec, tas_snd_ctrls: *const snd_kcontrol_new, num_ctls: c_int) {
    let mut i: c_int = 0;

    while i < num_ctls {
        *ctls.add(i as usize) = snd_ctl_new1(tas_snd_ctrls.add(i as usize), tas_priv as *mut c_void);
        let ret = snd_ctl_add((*codec).card, *ctls.add(i as usize));
        if ret != 0 {
            dev_err((*tas_priv).dev, b"Failed to add KControl %s = %d\n\0".as_ptr() as *const c_char, (*tas_snd_ctrls.add(i as usize)).name, ret);
            break;
        }
        i += 1;
    }
}

unsafe extern "C" fn tasdevice_dspfw_init(context: *mut c_void) {
    let tas_priv = context as *mut tasdevice_priv;
    let tas_hda = dev_get_drvdata((*tas_priv).dev) as *mut tas2781_hda;
    let hda_priv = (*tas_hda).hda_priv;
    let codec = (*tas_priv).codec;
    let mut ret: c_int;

    tasdevice_dsp_remove(tas_priv);
    (*tas_priv).fw_state = TASDEVICE_DSP_FW_PENDING;
    if (*tas_priv).speaker_id >= 0 {
        snprintf(
            (*tas_priv).coef_binaryname.as_mut_ptr() as *mut u8,
            (*tas_priv).coef_binaryname.len(),
            b"TAS2XXX%04X%d.bin\0".as_ptr() as *const c_char,
            lower_16_bits((*codec).core.subsystem_id),
            (*tas_priv).speaker_id,
        );
    } else {
        snprintf(
            (*tas_priv).coef_binaryname.as_mut_ptr() as *mut u8,
            (*tas_priv).coef_binaryname.len(),
            b"TAS2XXX%04X.bin\0".as_ptr() as *const c_char,
            lower_16_bits((*codec).core.subsystem_id),
        );
    }
    ret = tasdevice_dsp_parser(tas_priv);
    if ret != 0 {
        dev_err((*tas_priv).dev, b"dspfw load %s error\n\0".as_ptr() as *const c_char, (*tas_priv).coef_binaryname.as_ptr());
        (*tas_priv).fw_state = TASDEVICE_DSP_FW_FAIL;
        return;
    }
    tasdev_add_kcontrols(tas_priv, &mut (*tas_hda).dsp_prog_ctl, codec, &tasdevice_dsp_prog_ctrl, 1);
    tasdev_add_kcontrols(tas_priv, &mut (*tas_hda).dsp_conf_ctl, codec, &tasdevice_dsp_conf_ctrl, 1);

    (*tas_priv).fw_state = TASDEVICE_DSP_FW_ALL_OK;
    tasdevice_prmg_load(tas_priv, 0);
    if (*(*tas_priv).fmw).nr_programs > 0 {
        (*tas_priv).cur_prog = 0;
    }
    if (*(*tas_priv).fmw).nr_configurations > 0 {
        (*tas_priv).cur_conf = 0;
    }

    /* Init common setting for different audio profiles */
    if (*tas_priv).rcabin.init_profile_id >= 0 {
        tasdevice_select_cfg_blk(tas_priv, (*tas_priv).rcabin.init_profile_id, TASDEVICE_BIN_BLK_PRE_POWER_UP);
    }

    /* If calibrated data occurs error, dsp will still works with default
     * calibrated data inside algo.
     */
    if let Some(save_calibration) = (*hda_priv).save_calibration {
        save_calibration(tas_hda);
    }
}

unsafe extern "C" fn tasdev_fw_ready(fmw: *const firmware, context: *mut c_void) {
    let tas_priv = context as *mut tasdevice_priv;
    let tas_hda = dev_get_drvdata((*tas_priv).dev) as *mut tas2781_hda;
    let hda_priv = (*tas_hda).hda_priv;
    let codec = (*tas_priv).codec;
    let ret: c_int;

    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);

    ret = tasdevice_rca_parser(tas_priv, fmw);
    if ret != 0 {
        release_firmware(fmw);
        return;
    }

    (*tas_priv).fw_state = TASDEVICE_RCA_FW_OK;
    tasdev_add_kcontrols(tas_priv, &mut (*tas_hda).prof_ctl, codec, &tasdevice_prof_ctrl, 1);

    match (*hda_priv).hda_chip_id {
        HDA_TAS2770 => {
            tasdev_add_kcontrols(tas_priv, (*hda_priv).snd_ctls.as_mut_ptr(), codec, tas2770_snd_controls.as_ptr(), tas2770_snd_controls.len() as c_int);
        }
        HDA_TAS2781 => {
            tasdev_add_kcontrols(tas_priv, (*hda_priv).snd_ctls.as_mut_ptr(), codec, tas2781_snd_controls.as_ptr(), tas2781_snd_controls.len() as c_int);
            tasdevice_dspfw_init(context);
        }
        HDA_TAS5825 => {
            tasdev_add_kcontrols(tas_priv, (*hda_priv).snd_ctls.as_mut_ptr(), codec, tas5825_snd_controls.as_ptr(), tas5825_snd_controls.len() as c_int);
            tasdevice_dspfw_init(context);
        }
        HDA_TAS2563 => {
            tasdevice_dspfw_init(context);
        }
        _ => {}
    }

    release_firmware(fmw);
}

unsafe extern "C" fn tas2781_hda_bind(dev: *mut device, _master: *mut device, master_data: *mut c_void) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let parent = master_data as *mut hda_component_parent;
    let comp: *mut hda_component;
    let codec: *mut hda_codec;
    let subid: c_uint;
    let ret: c_int;

    comp = hda_component_from_index(parent, (*(*tas_hda).priv_).index);
    if comp.is_null() {
        return -EINVAL;
    }

    if !(*comp).dev.is_null() {
        return -EBUSY;
    }

    codec = (*parent).codec;
    subid = (*codec).core.subsystem_id >> 16;

    match subid {
        0x1028 => (*tas_hda).catlog_id = DELL,
        0x103C => (*tas_hda).catlog_id = HP,
        _ => (*tas_hda).catlog_id = LENOVO as c_int,
    }

    (*comp).dev = dev;

    strscpy((*comp).name.as_mut_ptr(), dev_name(dev), (*comp).name.len());

    ret = tascodec_init((*tas_hda).priv_, codec, THIS_MODULE, Some(tasdev_fw_ready));
    if ret == 0 {
        (*comp).playback_hook = Some(tas2781_hda_playback_hook);
    }

    ret
}

unsafe extern "C" fn tas2781_hda_unbind(dev: *mut device, _master: *mut device, master_data: *mut c_void) {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;
    let parent = master_data as *mut hda_component_parent;
    let comp: *mut hda_component;

    comp = hda_component_from_index(parent, (*(*tas_hda).priv_).index);
    if !comp.is_null() && (*comp).dev == dev {
        (*comp).dev = ptr::null_mut();
        memset((*comp).name.as_mut_ptr() as *mut c_void, 0, (*comp).name.len());
        (*comp).playback_hook = None;
    }

    request_firmware_nowait_cancel((*(*tas_hda).priv_).dev, (*tas_hda).priv_, Some(tasdev_fw_ready));

    tas2781_hda_remove_controls(tas_hda);

    tasdevice_config_info_remove((*tas_hda).priv_);
    tasdevice_dsp_remove((*tas_hda).priv_);

    (*(*tas_hda).priv_).fw_state = TASDEVICE_DSP_FW_PENDING;
}

static tas2781_hda_comp_ops: component_ops = component_ops {
    bind: Some(tas2781_hda_bind),
    unbind: Some(tas2781_hda_unbind),
};

unsafe extern "C" fn tas2781_hda_i2c_probe(clt: *mut i2c_client) -> c_int {
    let hda_priv: *mut tas2781_hda_i2c_priv;
    let tas_hda: *mut tas2781_hda;
    let device_name: *const c_char;
    let mut ret: c_int;

    tas_hda = devm_kzalloc(&mut (*clt).dev, size_of::<tas2781_hda>(), GFP_KERNEL) as *mut tas2781_hda;
    if tas_hda.is_null() {
        return -ENOMEM;
    }

    hda_priv = devm_kzalloc(&mut (*clt).dev, size_of::<tas2781_hda_i2c_priv>(), GFP_KERNEL) as *mut tas2781_hda_i2c_priv;
    if hda_priv.is_null() {
        return -ENOMEM;
    }

    (*tas_hda).hda_priv = hda_priv;

    dev_set_drvdata(&mut (*clt).dev, tas_hda as *mut c_void);
    (*tas_hda).dev = &mut (*clt).dev;

    (*tas_hda).priv_ = tasdevice_kzalloc(clt);
    if (*tas_hda).priv_.is_null() {
        return -ENOMEM;
    }

    if !strstr(dev_name(&mut (*clt).dev), b"TIAS2781\0".as_ptr() as *const c_char).is_null() {
        /*
         * TAS2781, integrated on-chip DSP with
         * global I2C address supported.
         */
        device_name = b"TIAS2781\0".as_ptr() as *const c_char;
        (*hda_priv).hda_chip_id = HDA_TAS2781;
        (*(*tas_hda).priv_).chip_id = TAS2781;
        (*hda_priv).save_calibration = Some(tas2781_save_calibration);
        (*(*tas_hda).priv_).global_addr = TAS2781_GLOBAL_ADDR;
    } else if strstarts(dev_name(&mut (*clt).dev), b"i2c-TXNW2770\0".as_ptr() as *const c_char) {
        /*
         * TAS2770, has no on-chip DSP, so no calibration data
         * required; has no global I2C address supported.
         */
        device_name = b"TXNW2770\0".as_ptr() as *const c_char;
        (*hda_priv).hda_chip_id = HDA_TAS2770;
    } else if strstarts(dev_name(&mut (*clt).dev), b"i2c-TXNW2781:00-tas2781-hda.0\0".as_ptr() as *const c_char) {
        device_name = b"TXNW2781\0".as_ptr() as *const c_char;
        (*hda_priv).hda_chip_id = HDA_TAS2781;
        (*(*tas_hda).priv_).chip_id = TAS2781;
        (*hda_priv).save_calibration = Some(tas2781_save_calibration);
        (*(*tas_hda).priv_).global_addr = TAS2781_GLOBAL_ADDR;
    } else if !strstr(dev_name(&mut (*clt).dev), b"INT8866\0".as_ptr() as *const c_char).is_null() {
        /*
         * TAS2563, integrated on-chip DSP with
         * global I2C address supported.
         */
        device_name = b"INT8866\0".as_ptr() as *const c_char;
        (*hda_priv).hda_chip_id = HDA_TAS2563;
        (*hda_priv).save_calibration = Some(tas2563_save_calibration);
        (*(*tas_hda).priv_).global_addr = TAS2563_GLOBAL_ADDR;
    } else if strstarts(dev_name(&mut (*clt).dev), b"i2c-TXNW5825\0".as_ptr() as *const c_char) {
        /*
         * TAS5825, integrated on-chip DSP without
         * global I2C address and calibration supported.
         */
        device_name = b"TXNW5825\0".as_ptr() as *const c_char;
        (*hda_priv).hda_chip_id = HDA_TAS5825;
        (*(*tas_hda).priv_).chip_id = TAS5825;
    } else {
        return -ENODEV;
    }

    (*(*tas_hda).priv_).irq = (*clt).irq;
    ret = tas2781_read_acpi((*tas_hda).priv_, device_name);
    if ret != 0 {
        return dev_err_probe((*tas_hda).dev, ret, b"Platform not supported\n\0".as_ptr() as *const c_char);
    }

    ret = tasdevice_init((*tas_hda).priv_);
    if ret != 0 {
        if ret != 0 {
            tas2781_hda_remove(&mut (*clt).dev, &tas2781_hda_comp_ops);
        }
        return ret;
    }

    pm_runtime_set_autosuspend_delay((*tas_hda).dev, 3000);
    pm_runtime_use_autosuspend((*tas_hda).dev);
    pm_runtime_mark_last_busy((*tas_hda).dev);
    pm_runtime_set_active((*tas_hda).dev);
    pm_runtime_enable((*tas_hda).dev);

    tasdevice_reset((*tas_hda).priv_);

    ret = component_add((*tas_hda).dev, &tas2781_hda_comp_ops);
    if ret != 0 {
        dev_err((*tas_hda).dev, b"Register component failed: %d\n\0".as_ptr() as *const c_char, ret);
        pm_runtime_disable((*tas_hda).dev);
    }

    if ret != 0 {
        tas2781_hda_remove(&mut (*clt).dev, &tas2781_hda_comp_ops);
    }
    ret
}

unsafe extern "C" fn tas2781_hda_i2c_remove(clt: *mut i2c_client) {
    tas2781_hda_remove(&mut (*clt).dev, &tas2781_hda_comp_ops);
}

unsafe extern "C" fn tas2781_runtime_suspend(dev: *mut device) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;

    dev_dbg((*tas_hda).dev, b"Runtime Suspend\n\0".as_ptr() as *const c_char);

    let _guard = guard_mutex(&mut (*(*tas_hda).priv_).codec_lock);

    /* The driver powers up the amplifiers at module load time.
     * Stop the playback if it's unused.
     */
    if (*(*tas_hda).priv_).playback_started {
        tasdevice_tuning_switch((*tas_hda).priv_, 1, false);
        (*(*tas_hda).priv_).playback_started = false;
    }

    0
}

unsafe extern "C" fn tas2781_runtime_resume(dev: *mut device) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;

    dev_dbg((*tas_hda).dev, b"Runtime Resume\n\0".as_ptr() as *const c_char);

    let _guard = guard_mutex(&mut (*(*tas_hda).priv_).codec_lock);

    tasdevice_prmg_load((*tas_hda).priv_, (*(*tas_hda).priv_).cur_prog);

    0
}

unsafe extern "C" fn tas2781_system_suspend(dev: *mut device) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;

    dev_dbg((*(*tas_hda).priv_).dev, b"System Suspend\n\0".as_ptr() as *const c_char);

    let _guard = guard_mutex(&mut (*(*tas_hda).priv_).codec_lock);

    /* Shutdown chip before system suspend */
    if (*(*tas_hda).priv_).playback_started {
        tasdevice_tuning_switch((*tas_hda).priv_, 1, false);
    }

    /*
     * Reset GPIO may be shared, so cannot reset here.
     * However beyond this point, amps may be powered down.
     */
    0
}

unsafe extern "C" fn tas2781_system_resume(dev: *mut device) -> c_int {
    let tas_hda = dev_get_drvdata(dev) as *mut tas2781_hda;

    dev_dbg((*(*tas_hda).priv_).dev, b"System Resume\n\0".as_ptr() as *const c_char);

    let _guard = guard_mutex(&mut (*(*tas_hda).priv_).codec_lock);

    for i in 0..(*(*tas_hda).priv_).ndev {
        (*(*tas_hda).priv_).tasdevice[i as usize].cur_book = -1;
        (*(*tas_hda).priv_).tasdevice[i as usize].cur_prog = -1;
        (*(*tas_hda).priv_).tasdevice[i as usize].cur_conf = -1;
    }
    tasdevice_reset((*tas_hda).priv_);
    tasdevice_prmg_load((*tas_hda).priv_, (*(*tas_hda).priv_).cur_prog);

    /* Init common setting for different audio profiles */
    if (*(*tas_hda).priv_).rcabin.init_profile_id >= 0 {
        tasdevice_select_cfg_blk(
            (*tas_hda).priv_,
            (*(*tas_hda).priv_).rcabin.init_profile_id,
            TASDEVICE_BIN_BLK_PRE_POWER_UP,
        );
    }

    if (*(*tas_hda).priv_).playback_started {
        tasdevice_tuning_switch((*tas_hda).priv_, 0, false);
    }

    0
}

static tas2781_hda_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(tas2781_runtime_suspend),
    runtime_resume: Some(tas2781_runtime_resume),
    system_suspend: Some(tas2781_system_suspend),
    system_resume: Some(tas2781_system_resume),
};

static tas2781_hda_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"tas2781-hda\0\0\0\0\0\0\0\0" as [u8; 20] as [c_char; 20] },
    i2c_device_id { name: [0; 20] },
];

static tas2781_acpi_hda_match: [acpi_device_id; 6] = [
    acpi_device_id { id: *b"INT8866\0\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: 0 },
    acpi_device_id { id: *b"TIAS2781\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: 0 },
    acpi_device_id { id: *b"TXNW2770\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: 0 },
    acpi_device_id { id: *b"TXNW2781\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: 0 },
    acpi_device_id { id: *b"TXNW5825\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: 0 },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(acpi, tas2781_acpi_hda_match); */

static mut tas2781_hda_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"tas2781-hda\0".as_ptr() as *const c_char,
        acpi_match_table: tas2781_acpi_hda_match.as_ptr(),
        pm: &tas2781_hda_pm_ops,
    },
    id_table: tas2781_hda_i2c_id.as_ptr(),
    probe: Some(tas2781_hda_i2c_probe),
    remove: Some(tas2781_hda_i2c_remove),
};
/* module_i2c_driver(tas2781_hda_i2c_driver); */

/* MODULE_DESCRIPTION("TAS2781 HDA Driver"); */
/* MODULE_AUTHOR("Shenghao Ding, TI, <shenghao-ding@ti.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_IMPORT_NS("SND_SOC_TAS2781_FMWLIB"); */
/* MODULE_IMPORT_NS("SND_HDA_SCODEC_TAS2781"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
