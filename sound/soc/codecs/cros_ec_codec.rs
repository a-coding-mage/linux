// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 Google, Inc.
 *
 * ChromeOS Embedded Controller codec driver.
 *
 * This driver uses the cros-ec interface to communicate with the ChromeOS
 * EC for audio function.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type snd_pcm_uframes_t = c_ulong;
type bool_t = bool;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EAGAIN: c_int = 11;
const EIO: c_int = 5;
const EFAULT: c_int = 14;
const ENOPROTOOPT: c_int = 92;
const PAGE_SIZE: u32 = 4096;
const SHA256_DIGEST_SIZE: usize = 32;

const DMIC_CTL_GAIN: usize = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cros_ec_device {
    pub event_notifier: blocking_notifier_head,
}

#[repr(C)]
pub struct blocking_notifier_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cros_ec_command {
    pub version: u32,
    pub command: u32,
    pub outsize: size_t,
    pub insize: size_t,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

type c_long = isize;

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub max: c_int,
    pub platform_max: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut u8,
    pub dma_bytes: size_t,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: u64,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *mut snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *mut snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *mut snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub pointer:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u64,
    pub formats: u64,
    pub rates: u64,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device_with_parent,
}

#[repr(C)]
pub struct device_with_parent {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: u64,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct cros_ec_codec_priv {
    pub dev: *mut device,
    pub ec_device: *mut cros_ec_device,

    /* common */
    pub ec_capabilities: u32,

    pub ec_shm_addr: u64,
    pub ec_shm_len: u32,

    pub ap_shm_phys_addr: u64,
    pub ap_shm_len: u32,
    pub ap_shm_addr: u64,
    pub ap_shm_last_alloc: u64,

    /* DMIC */
    pub dmic_probed: atomic_t,

    /* I2S_RX */
    pub i2s_rx_bclk_ratio: u32,

    /* WoV */
    pub wov_enabled: bool_t,
    pub wov_audio_shm_p: *mut u8,
    pub wov_audio_shm_len: u32,
    pub wov_audio_shm_type: u8,
    pub wov_lang_shm_p: *mut u8,
    pub wov_lang_shm_len: u32,
    pub wov_lang_shm_type: u8,

    pub wov_dma_lock: mutex,
    pub wov_buf: [u8; 64000],
    pub wov_rp: u32,
    pub wov_wp: u32,
    pub wov_dma_offset: size_t,
    pub wov_burst_read: bool_t,
    pub wov_substream: *mut snd_pcm_substream,
    pub wov_copy_work: delayed_work,
    pub wov_notifier: notifier_block,
}

#[repr(C)]
pub struct ec_param_ec_codec_dmic {
    pub cmd: u8,
    pub get_gain_idx_param: ec_codec_dmic_get_gain_idx_param,
    pub set_gain_idx_param: ec_codec_dmic_set_gain_idx_param,
}

#[repr(C)]
pub struct ec_codec_dmic_get_gain_idx_param {
    pub channel: u8,
}

#[repr(C)]
pub struct ec_codec_dmic_set_gain_idx_param {
    pub channel: u8,
    pub gain: c_int,
}

#[repr(C)]
pub struct ec_response_ec_codec_dmic_get_gain_idx {
    pub gain: c_int,
}

#[repr(C)]
pub struct ec_response_ec_codec_dmic_get_max_gain {
    pub max_gain: c_int,
}

#[repr(C)]
pub struct ec_param_ec_codec_i2s_rx {
    pub cmd: u8,
    pub set_sample_depth_param: ec_codec_i2s_rx_set_sample_depth_param,
    pub set_bclk_param: ec_codec_i2s_rx_set_bclk_param,
    pub set_daifmt_param: ec_codec_i2s_rx_set_daifmt_param,
}

#[repr(C)]
pub struct ec_codec_i2s_rx_set_sample_depth_param {
    pub depth: ec_codec_i2s_rx_sample_depth,
}

#[repr(C)]
pub struct ec_codec_i2s_rx_set_bclk_param {
    pub bclk: u32,
}

#[repr(C)]
pub struct ec_codec_i2s_rx_set_daifmt_param {
    pub daifmt: ec_codec_i2s_rx_daifmt,
}

type ec_codec_i2s_rx_sample_depth = u32;
type ec_codec_i2s_rx_daifmt = u32;

#[repr(C)]
pub struct ec_param_ec_codec {
    pub cmd: u8,
    pub get_shm_addr_param: ec_codec_get_shm_addr_param,
    pub set_shm_addr_param: ec_codec_set_shm_addr_param,
}

#[repr(C)]
pub struct ec_codec_get_shm_addr_param {
    pub shm_id: u8,
}

#[repr(C)]
pub struct ec_codec_set_shm_addr_param {
    pub phys_addr: u64,
    pub len: u32,
    pub shm_id: u8,
}

#[repr(C)]
pub struct ec_response_ec_codec_get_shm_addr {
    pub phys_addr: u64,
    pub len: u32,
    pub type_: u8,
}

#[repr(C)]
pub struct ec_response_ec_codec_get_capabilities {
    pub capabilities: u32,
}

#[repr(C)]
pub struct ec_param_ec_codec_wov {
    pub cmd: u8,
    pub set_lang_shm_param: ec_param_ec_codec_wov_set_lang_shm,
    pub set_lang_param: ec_param_ec_codec_wov_set_lang,
}

#[repr(C)]
pub struct ec_response_ec_codec_wov_read_audio_shm {
    pub offset: u32,
    pub len: u32,
}

#[repr(C)]
pub struct ec_response_ec_codec_wov_read_audio {
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct ec_param_ec_codec_wov_set_lang_shm {
    pub hash: [u8; SHA256_DIGEST_SIZE],
    pub total_len: size_t,
}

#[repr(C)]
pub struct ec_param_ec_codec_wov_set_lang {
    pub hash: [u8; SHA256_DIGEST_SIZE],
    pub total_len: size_t,
    pub offset: size_t,
    pub buf: [u8; 0],
    pub len: size_t,
}

#[repr(C)]
pub struct ec_response_ec_codec_wov_get_lang {
    pub hash: [u8; SHA256_DIGEST_SIZE],
}

unsafe extern "C" {
    static dmic_gain_tlv: c_uint;

    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, n: size_t);
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, n: size_t);
    fn memset_io(dst: *mut c_void, c: c_int, n: size_t);
    fn wmb();
    fn cros_ec_cmd_xfer_status(ec_dev: *mut cros_ec_device, msg: *mut cros_ec_command) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut cros_ec_codec_priv;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *mut snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn params_width(params: *mut snd_pcm_hw_params) -> u32;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> u32;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn devm_ioremap_wc(dev: *mut device, offset: u64, size: u32) -> *mut c_void;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn str_enable_disable(enable: c_int) -> *const c_char;
    fn sha256(buf: *const u8, size: size_t, digest: *mut u8);
    fn memdup_user(src: *const c_uint, size: c_uint) -> *mut u8;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn cros_ec_get_host_event(ec_dev: *mut cros_ec_device) -> u32;
    fn blocking_notifier_chain_register(
        nh: *mut blocking_notifier_head,
        nb: *mut notifier_block,
    ) -> c_int;
    fn blocking_notifier_chain_unregister(
        nh: *mut blocking_notifier_head,
        nb: *mut notifier_block,
    ) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        data: *mut c_void,
        size: size_t,
        max: size_t,
    );
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn of_get_address(
        np: *mut device_node,
        index: c_int,
        size: *mut u64,
        flags: *mut c_uint,
    ) -> *const u32;
    fn of_read_number(cell: *const u32, size: c_int) -> u64;
    fn of_reserved_mem_region_to_resource(
        np: *mut device_node,
        index: c_int,
        res: *mut resource,
    ) -> c_int;
    fn resource_size(res: *const resource) -> u64;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

const fn BIT(n: u8) -> u32 {
    1u32 << n
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const fn round_up(x: u32, y: u32) -> u32 {
    ((x + y - 1) / y) * y
}

unsafe fn min<T: Ord>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn ec_codec_capable(priv_: *mut cros_ec_codec_priv, cap: u8) -> c_int {
    ((*priv_).ec_capabilities & BIT(cap)) as c_int
}

unsafe fn send_ec_host_command(
    ec_dev: *mut cros_ec_device,
    cmd: u32,
    out: *mut u8,
    outsize: size_t,
    in_: *mut u8,
    insize: size_t,
) -> c_int {
    let mut ret: c_int;
    let msg: *mut cros_ec_command;

    msg = kmalloc(size_of::<cros_ec_command>() + min(outsize, insize), GFP_KERNEL)
        as *mut cros_ec_command;
    if msg.is_null() {
        return -ENOMEM;
    }

    (*msg).version = 0;
    (*msg).command = cmd;
    (*msg).outsize = outsize;
    (*msg).insize = insize;

    if outsize != 0 {
        memcpy((*msg).data.as_mut_ptr() as *mut c_void, out as *const c_void, outsize);
    }

    ret = cros_ec_cmd_xfer_status(ec_dev, msg);
    if ret < 0 {
        kfree(msg as *mut c_void);
        return ret;
    }

    if !in_.is_null() && insize != 0 {
        memcpy(in_ as *mut c_void, (*msg).data.as_ptr() as *const c_void, insize);
    }

    ret = 0;
    kfree(msg as *mut c_void);
    ret
}

unsafe extern "C" fn dmic_get_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component);
    let mut p: ec_param_ec_codec_dmic = zeroed();
    let mut r: ec_response_ec_codec_dmic_get_gain_idx = zeroed();
    let mut ret: c_int;

    p.cmd = EC_CODEC_DMIC_GET_GAIN_IDX;
    p.get_gain_idx_param.channel = EC_CODEC_DMIC_CHANNEL_0;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_DMIC,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_dmic>(),
        &mut r as *mut _ as *mut u8,
        size_of::<ec_response_ec_codec_dmic_get_gain_idx>(),
    );
    if ret < 0 {
        return ret;
    }
    (*ucontrol).value.integer.value[0] = r.gain as c_long;

    p.cmd = EC_CODEC_DMIC_GET_GAIN_IDX;
    p.get_gain_idx_param.channel = EC_CODEC_DMIC_CHANNEL_1;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_DMIC,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_dmic>(),
        &mut r as *mut _ as *mut u8,
        size_of::<ec_response_ec_codec_dmic_get_gain_idx>(),
    );
    if ret < 0 {
        return ret;
    }
    (*ucontrol).value.integer.value[1] = r.gain as c_long;

    0
}

unsafe extern "C" fn dmic_put_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component);
    let control = (*kcontrol).private_value as *mut soc_mixer_control;
    let max_dmic_gain = (*control).max;
    let left = (*ucontrol).value.integer.value[0] as c_int;
    let right = (*ucontrol).value.integer.value[1] as c_int;
    let mut p: ec_param_ec_codec_dmic = zeroed();
    let mut ret: c_int;

    if left > max_dmic_gain || right > max_dmic_gain {
        return -EINVAL;
    }

    dev_dbg((*component).dev, c"set mic gain to %u, %u\n".as_ptr(), left, right);

    p.cmd = EC_CODEC_DMIC_SET_GAIN_IDX;
    p.set_gain_idx_param.channel = EC_CODEC_DMIC_CHANNEL_0;
    p.set_gain_idx_param.gain = left;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_DMIC,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_dmic>(),
        ptr::null_mut(),
        0,
    );
    if ret < 0 {
        return ret;
    }

    p.cmd = EC_CODEC_DMIC_SET_GAIN_IDX;
    p.set_gain_idx_param.channel = EC_CODEC_DMIC_CHANNEL_1;
    p.set_gain_idx_param.gain = right;
    send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_DMIC,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_dmic>(),
        ptr::null_mut(),
        0,
    )
}

static mut dmic_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { private_value: 0 }];

unsafe extern "C" fn dmic_probe(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component);
    let dev = (*priv_).dev;
    let control: *mut soc_mixer_control;
    let mut p: ec_param_ec_codec_dmic = zeroed();
    let mut r: ec_response_ec_codec_dmic_get_max_gain = zeroed();
    let ret: c_int;

    if !atomic_add_unless(&mut (*priv_).dmic_probed, 1, 1) {
        return 0;
    }

    p.cmd = EC_CODEC_DMIC_GET_MAX_GAIN;

    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_DMIC,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_dmic>(),
        &mut r as *mut _ as *mut u8,
        size_of::<ec_response_ec_codec_dmic_get_max_gain>(),
    );
    if ret < 0 {
        dev_warn(dev, c"get_max_gain() unsupported\n".as_ptr());
        return 0;
    }

    dev_dbg(dev, c"max gain = %d\n".as_ptr(), r.max_gain);

    control = dmic_controls[DMIC_CTL_GAIN].private_value as *mut soc_mixer_control;
    (*control).max = r.max_gain;
    (*control).platform_max = r.max_gain;

    snd_soc_add_component_controls(component, &mut dmic_controls[DMIC_CTL_GAIN], 1)
}

unsafe extern "C" fn i2s_rx_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component);
    let mut p: ec_param_ec_codec_i2s_rx = zeroed();
    let depth: ec_codec_i2s_rx_sample_depth;
    let bclk: u32;
    let mut ret: c_int;

    if params_rate(params) != 48000 {
        return -EINVAL;
    }

    match params_width(params) {
        16 => depth = EC_CODEC_I2S_RX_SAMPLE_DEPTH_16,
        24 => depth = EC_CODEC_I2S_RX_SAMPLE_DEPTH_24,
        _ => return -EINVAL,
    }

    dev_dbg((*component).dev, c"set depth to %u\n".as_ptr(), depth);

    p.cmd = EC_CODEC_I2S_RX_SET_SAMPLE_DEPTH;
    p.set_sample_depth_param.depth = depth;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_I2S_RX,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_i2s_rx>(),
        ptr::null_mut(),
        0,
    );
    if ret < 0 {
        return ret;
    }

    if (*priv_).i2s_rx_bclk_ratio != 0 {
        bclk = params_rate(params) * (*priv_).i2s_rx_bclk_ratio;
    } else {
        bclk = snd_soc_params_to_bclk(params);
    }

    dev_dbg((*component).dev, c"set bclk to %u\n".as_ptr(), bclk);

    p.cmd = EC_CODEC_I2S_RX_SET_BCLK;
    p.set_bclk_param.bclk = bclk;
    send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_I2S_RX,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_i2s_rx>(),
        ptr::null_mut(),
        0,
    )
}

unsafe extern "C" fn i2s_rx_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component);

    (*priv_).i2s_rx_bclk_ratio = ratio;
    0
}

unsafe extern "C" fn i2s_rx_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component);
    let mut p: ec_param_ec_codec_i2s_rx = zeroed();
    let daifmt: ec_codec_i2s_rx_daifmt;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => daifmt = EC_CODEC_I2S_RX_DAIFMT_I2S,
        SND_SOC_DAIFMT_RIGHT_J => daifmt = EC_CODEC_I2S_RX_DAIFMT_RIGHT_J,
        SND_SOC_DAIFMT_LEFT_J => daifmt = EC_CODEC_I2S_RX_DAIFMT_LEFT_J,
        _ => return -EINVAL,
    }

    dev_dbg((*component).dev, c"set format to %u\n".as_ptr(), daifmt);

    p.cmd = EC_CODEC_I2S_RX_SET_DAIFMT;
    p.set_daifmt_param.daifmt = daifmt;
    send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_I2S_RX,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_i2s_rx>(),
        ptr::null_mut(),
        0,
    )
}

static i2s_rx_selectable_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
    | SND_SOC_POSSIBLE_DAIFMT_NB_NF;

static i2s_rx_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(i2s_rx_hw_params),
    set_fmt: Some(i2s_rx_set_fmt),
    set_bclk_ratio: Some(i2s_rx_set_bclk_ratio),
    auto_selectable_formats: &i2s_rx_selectable_formats,
    num_auto_selectable_formats: 1,
};

unsafe extern "C" fn i2s_rx_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(component);
    let mut p: ec_param_ec_codec_i2s_rx = zeroed();

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            dev_dbg((*component).dev, c"enable I2S RX\n".as_ptr());
            p.cmd = EC_CODEC_I2S_RX_ENABLE;
        }
        SND_SOC_DAPM_PRE_PMD => {
            dev_dbg((*component).dev, c"disable I2S RX\n".as_ptr());
            p.cmd = EC_CODEC_I2S_RX_DISABLE;
        }
        _ => return 0,
    }

    send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_I2S_RX,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_i2s_rx>(),
        ptr::null_mut(),
        0,
    )
}

static mut i2s_rx_dapm_widgets: [snd_soc_dapm_widget; 3] = unsafe { zeroed() };
static mut i2s_rx_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"I2S RX".as_ptr(),
        control: ptr::null(),
        source: c"DMIC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I2S RX".as_ptr(),
        control: ptr::null(),
        source: c"I2S RX Enable".as_ptr(),
    },
];

static mut i2s_rx_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"EC Codec I2S RX".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"I2S Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &i2s_rx_dai_ops,
};

unsafe extern "C" fn i2s_rx_probe(component: *mut snd_soc_component) -> c_int {
    dmic_probe(component)
}

static mut i2s_rx_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(i2s_rx_probe),
    remove: None,
    controls: ptr::null_mut(),
    num_controls: 0,
    dapm_widgets: unsafe { i2s_rx_dapm_widgets.as_mut_ptr() },
    num_dapm_widgets: 3,
    dapm_routes: unsafe { i2s_rx_dapm_routes.as_mut_ptr() },
    num_dapm_routes: 2,
    open: None,
    hw_params: None,
    hw_free: None,
    pointer: None,
    pcm_new: None,
    endianness: 1,
};

unsafe fn wov_map_shm(
    priv_: *mut cros_ec_codec_priv,
    shm_id: u8,
    len: *mut u32,
    type_: *mut u8,
) -> *mut c_void {
    let mut p: ec_param_ec_codec = zeroed();
    let mut r: ec_response_ec_codec_get_shm_addr = zeroed();
    let req: u32;
    let offset: u32;

    p.cmd = EC_CODEC_GET_SHM_ADDR;
    p.get_shm_addr_param.shm_id = shm_id;
    if send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec>(),
        &mut r as *mut _ as *mut u8,
        size_of::<ec_response_ec_codec_get_shm_addr>(),
    ) < 0
    {
        dev_err((*priv_).dev, c"failed to EC_CODEC_GET_SHM_ADDR\n".as_ptr());
        return ptr::null_mut();
    }

    dev_dbg((*priv_).dev, c"phys_addr=%#llx, len=%#x\n".as_ptr(), r.phys_addr, r.len);

    *len = r.len;
    *type_ = r.type_;

    match r.type_ {
        EC_CODEC_SHM_TYPE_EC_RAM => {
            devm_ioremap_wc((*priv_).dev, r.phys_addr + (*priv_).ec_shm_addr, r.len)
        }
        EC_CODEC_SHM_TYPE_SYSTEM_RAM => {
            if r.phys_addr != 0 {
                dev_err((*priv_).dev, c"unknown status\n".as_ptr());
                return ptr::null_mut();
            }

            req = round_up(r.len, PAGE_SIZE);
            dev_dbg((*priv_).dev, c"round up from %u to %u\n".as_ptr(), r.len, req);

            if (*priv_).ap_shm_last_alloc + req as u64
                > (*priv_).ap_shm_phys_addr + (*priv_).ap_shm_len as u64
            {
                dev_err((*priv_).dev, c"insufficient space for AP SHM\n".as_ptr());
                return ptr::null_mut();
            }

            dev_dbg(
                (*priv_).dev,
                c"alloc AP SHM addr=%#llx, len=%#x\n".as_ptr(),
                (*priv_).ap_shm_last_alloc,
                req,
            );

            p.cmd = EC_CODEC_SET_SHM_ADDR;
            p.set_shm_addr_param.phys_addr = (*priv_).ap_shm_last_alloc;
            p.set_shm_addr_param.len = req;
            p.set_shm_addr_param.shm_id = shm_id;
            if send_ec_host_command(
                (*priv_).ec_device,
                EC_CMD_EC_CODEC,
                &mut p as *mut _ as *mut u8,
                size_of::<ec_param_ec_codec>(),
                ptr::null_mut(),
                0,
            ) < 0
            {
                dev_err((*priv_).dev, c"failed to EC_CODEC_SET_SHM_ADDR\n".as_ptr());
                return ptr::null_mut();
            }

            /*
             * Note: EC codec only requests for `r.len' but we allocate
             * round up PAGE_SIZE `req'.
             */
            offset = ((*priv_).ap_shm_last_alloc - (*priv_).ap_shm_phys_addr) as u32;
            (*priv_).ap_shm_last_alloc += req as u64;

            ((*priv_).ap_shm_addr + offset as u64) as usize as *mut c_void
        }
        _ => ptr::null_mut(),
    }
}

unsafe fn wov_queue_full(priv_: *mut cros_ec_codec_priv) -> bool {
    (((*priv_).wov_wp + 1) as usize % size_of::<[u8; 64000]>()) == (*priv_).wov_rp as usize
}

unsafe fn wov_queue_size(priv_: *mut cros_ec_codec_priv) -> size_t {
    if (*priv_).wov_wp >= (*priv_).wov_rp {
        ((*priv_).wov_wp - (*priv_).wov_rp) as size_t
    } else {
        size_of::<[u8; 64000]>() - (*priv_).wov_rp as size_t + (*priv_).wov_wp as size_t
    }
}

unsafe fn wov_queue_dequeue(priv_: *mut cros_ec_codec_priv, mut len: size_t) {
    let runtime = (*(*priv_).wov_substream).runtime;
    let mut req: size_t;

    while len != 0 {
        req = min(len, (*runtime).dma_bytes - (*priv_).wov_dma_offset);
        if (*priv_).wov_wp >= (*priv_).wov_rp {
            req = min(req, ((*priv_).wov_wp - (*priv_).wov_rp) as size_t);
        } else {
            req = min(req, size_of::<[u8; 64000]>() - (*priv_).wov_rp as size_t);
        }

        memcpy(
            (*runtime).dma_area.add((*priv_).wov_dma_offset) as *mut c_void,
            (*priv_).wov_buf.as_ptr().add((*priv_).wov_rp as size_t) as *const c_void,
            req,
        );

        (*priv_).wov_dma_offset += req;
        if (*priv_).wov_dma_offset == (*runtime).dma_bytes {
            (*priv_).wov_dma_offset = 0;
        }

        (*priv_).wov_rp += req as u32;
        if (*priv_).wov_rp as usize == size_of::<[u8; 64000]>() {
            (*priv_).wov_rp = 0;
        }

        len -= req;
    }

    snd_pcm_period_elapsed((*priv_).wov_substream);
}

unsafe fn wov_queue_try_dequeue(priv_: *mut cros_ec_codec_priv) {
    let mut period_bytes = snd_pcm_lib_period_bytes((*priv_).wov_substream);

    while period_bytes != 0 && wov_queue_size(priv_) >= period_bytes {
        wov_queue_dequeue(priv_, period_bytes);
        period_bytes = snd_pcm_lib_period_bytes((*priv_).wov_substream);
    }
}

unsafe fn wov_queue_enqueue(
    priv_: *mut cros_ec_codec_priv,
    mut addr: *mut u8,
    mut len: size_t,
    iomem: bool,
) {
    let mut req: size_t;

    while len != 0 {
        if wov_queue_full(priv_) {
            wov_queue_try_dequeue(priv_);

            if wov_queue_full(priv_) {
                dev_err((*priv_).dev, c"overrun detected\n".as_ptr());
                return;
            }
        }

        if (*priv_).wov_wp >= (*priv_).wov_rp {
            req = size_of::<[u8; 64000]>() - (*priv_).wov_wp as size_t;
        } else {
            /* Note: waste 1-byte to differentiate full and empty */
            req = ((*priv_).wov_rp - (*priv_).wov_wp - 1) as size_t;
        }
        req = min(req, len);

        if iomem {
            memcpy_fromio(
                (*priv_).wov_buf.as_mut_ptr().add((*priv_).wov_wp as size_t) as *mut c_void,
                addr as *const c_void,
                req,
            );
        } else {
            memcpy(
                (*priv_).wov_buf.as_mut_ptr().add((*priv_).wov_wp as size_t) as *mut c_void,
                addr as *const c_void,
                req,
            );
        }

        (*priv_).wov_wp += req as u32;
        if (*priv_).wov_wp as usize == size_of::<[u8; 64000]>() {
            (*priv_).wov_wp = 0;
        }

        addr = addr.add(req);
        len -= req;
    }

    wov_queue_try_dequeue(priv_);
}

unsafe fn wov_read_audio_shm(priv_: *mut cros_ec_codec_priv) -> c_int {
    let mut p: ec_param_ec_codec_wov = zeroed();
    let mut r: ec_response_ec_codec_wov_read_audio_shm = zeroed();
    let ret: c_int;

    p.cmd = EC_CODEC_WOV_READ_AUDIO_SHM;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_WOV,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_wov>(),
        &mut r as *mut _ as *mut u8,
        size_of::<ec_response_ec_codec_wov_read_audio_shm>(),
    );
    if ret != 0 {
        dev_err((*priv_).dev, c"failed to EC_CODEC_WOV_READ_AUDIO_SHM\n".as_ptr());
        return ret;
    }

    if r.len == 0 {
        dev_dbg((*priv_).dev, c"no data, sleep\n".as_ptr());
    } else {
        wov_queue_enqueue(
            priv_,
            (*priv_).wov_audio_shm_p.add(r.offset as size_t),
            r.len as size_t,
            (*priv_).wov_audio_shm_type == EC_CODEC_SHM_TYPE_EC_RAM,
        );
    }
    -EAGAIN
}

unsafe fn wov_read_audio(priv_: *mut cros_ec_codec_priv) -> c_int {
    let mut p: ec_param_ec_codec_wov = zeroed();
    let mut r: ec_response_ec_codec_wov_read_audio = zeroed();
    let mut remain: c_int = if (*priv_).wov_burst_read { 16000 } else { 320 };
    let mut ret: c_int;

    while remain >= 0 {
        p.cmd = EC_CODEC_WOV_READ_AUDIO;
        ret = send_ec_host_command(
            (*priv_).ec_device,
            EC_CMD_EC_CODEC_WOV,
            &mut p as *mut _ as *mut u8,
            size_of::<ec_param_ec_codec_wov>(),
            &mut r as *mut _ as *mut u8,
            size_of::<ec_response_ec_codec_wov_read_audio>(),
        );
        if ret != 0 {
            dev_err((*priv_).dev, c"failed to EC_CODEC_WOV_READ_AUDIO\n".as_ptr());
            return ret;
        }

        if r.len == 0 {
            dev_dbg((*priv_).dev, c"no data, sleep\n".as_ptr());
            (*priv_).wov_burst_read = false;
            break;
        }

        wov_queue_enqueue(priv_, r.buf, r.len as size_t, false);
        remain -= r.len;
    }

    -EAGAIN
}

unsafe extern "C" fn wov_copy_work(w: *mut work_struct) {
    let priv_ = container_of_wov_copy_work(w);
    let ret: c_int;

    mutex_lock(&mut (*priv_).wov_dma_lock);
    if (*priv_).wov_substream.is_null() {
        dev_warn((*priv_).dev, c"no pcm substream\n".as_ptr());
        mutex_unlock(&mut (*priv_).wov_dma_lock);
        return;
    }

    if ec_codec_capable(priv_, EC_CODEC_CAP_WOV_AUDIO_SHM) != 0 {
        ret = wov_read_audio_shm(priv_);
    } else {
        ret = wov_read_audio(priv_);
    }

    mutex_unlock(&mut (*priv_).wov_dma_lock);

    if ret == -EAGAIN {
        schedule_delayed_work(&mut (*priv_).wov_copy_work, msecs_to_jiffies(10));
    } else if ret != 0 {
        dev_err((*priv_).dev, c"failed to read audio data\n".as_ptr());
    }
}

unsafe extern "C" fn wov_enable_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let c = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(c);

    (*ucontrol).value.integer.value[0] = (*priv_).wov_enabled as c_long;
    0
}

unsafe extern "C" fn wov_enable_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let c = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(c);
    let enabled = (*ucontrol).value.integer.value[0] as c_int;
    let mut p: ec_param_ec_codec_wov = zeroed();
    let ret: c_int;

    if (*priv_).wov_enabled as c_int != enabled {
        if enabled != 0 {
            p.cmd = EC_CODEC_WOV_ENABLE;
        } else {
            p.cmd = EC_CODEC_WOV_DISABLE;
        }

        ret = send_ec_host_command(
            (*priv_).ec_device,
            EC_CMD_EC_CODEC_WOV,
            &mut p as *mut _ as *mut u8,
            size_of::<ec_param_ec_codec_wov>(),
            ptr::null_mut(),
            0,
        );
        if ret != 0 {
            dev_err(
                (*priv_).dev,
                c"failed to %s wov\n".as_ptr(),
                str_enable_disable(enabled),
            );
            return ret;
        }

        (*priv_).wov_enabled = enabled != 0;
    }

    0
}

unsafe fn wov_set_lang_shm(
    priv_: *mut cros_ec_codec_priv,
    buf: *mut u8,
    size: size_t,
    digest: *mut u8,
) -> c_int {
    let mut p: ec_param_ec_codec_wov = zeroed();
    let pp = &mut p.set_lang_shm_param as *mut ec_param_ec_codec_wov_set_lang_shm;
    let ret: c_int;

    if size > (*priv_).wov_lang_shm_len as size_t {
        dev_err((*priv_).dev, c"no enough SHM size: %d\n".as_ptr(), (*priv_).wov_lang_shm_len);
        return -EIO;
    }

    match (*priv_).wov_lang_shm_type {
        EC_CODEC_SHM_TYPE_EC_RAM => {
            memcpy_toio((*priv_).wov_lang_shm_p as *mut c_void, buf as *const c_void, size);
            memset_io(
                (*priv_).wov_lang_shm_p.add(size) as *mut c_void,
                0,
                (*priv_).wov_lang_shm_len as size_t - size,
            );
        }
        EC_CODEC_SHM_TYPE_SYSTEM_RAM => {
            memcpy((*priv_).wov_lang_shm_p as *mut c_void, buf as *const c_void, size);
            memset(
                (*priv_).wov_lang_shm_p.add(size) as *mut c_void,
                0,
                (*priv_).wov_lang_shm_len as size_t - size,
            );

            /* make sure write to memory before calling host command */
            wmb();
        }
        _ => {}
    }

    p.cmd = EC_CODEC_WOV_SET_LANG_SHM;
    memcpy((*pp).hash.as_mut_ptr() as *mut c_void, digest as *const c_void, SHA256_DIGEST_SIZE);
    (*pp).total_len = size;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_WOV,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_wov>(),
        ptr::null_mut(),
        0,
    );
    if ret != 0 {
        dev_err((*priv_).dev, c"failed to EC_CODEC_WOV_SET_LANG_SHM\n".as_ptr());
        return ret;
    }

    0
}

unsafe fn wov_set_lang(
    priv_: *mut cros_ec_codec_priv,
    buf: *mut u8,
    size: size_t,
    digest: *mut u8,
) -> c_int {
    let mut p: ec_param_ec_codec_wov = zeroed();
    let pp = &mut p.set_lang_param as *mut ec_param_ec_codec_wov_set_lang;
    let mut i: size_t = 0;
    let mut req: size_t;
    let mut ret: c_int;

    while i < size {
        req = min(size - i, ARRAY_SIZE(&(*pp).buf));

        p.cmd = EC_CODEC_WOV_SET_LANG;
        memcpy((*pp).hash.as_mut_ptr() as *mut c_void, digest as *const c_void, SHA256_DIGEST_SIZE);
        (*pp).total_len = size;
        (*pp).offset = i;
        memcpy((*pp).buf.as_mut_ptr() as *mut c_void, buf.add(i) as *const c_void, req);
        (*pp).len = req;
        ret = send_ec_host_command(
            (*priv_).ec_device,
            EC_CMD_EC_CODEC_WOV,
            &mut p as *mut _ as *mut u8,
            size_of::<ec_param_ec_codec_wov>(),
            ptr::null_mut(),
            0,
        );
        if ret != 0 {
            dev_err((*priv_).dev, c"failed to EC_CODEC_WOV_SET_LANG\n".as_ptr());
            return ret;
        }

        i += req;
    }

    0
}

unsafe extern "C" fn wov_hotword_model_put(
    kcontrol: *mut snd_kcontrol,
    mut bytes: *const c_uint,
    mut size: c_uint,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component);
    let mut p: ec_param_ec_codec_wov = zeroed();
    let mut r: ec_response_ec_codec_wov_get_lang = zeroed();
    let mut digest = [0u8; SHA256_DIGEST_SIZE];
    let buf: *mut u8;
    let mut ret: c_int;

    /* Skips the TLV header. */
    bytes = bytes.add(2);
    size -= 8;

    dev_dbg((*priv_).dev, c"%s: size=%d\n".as_ptr(), c"wov_hotword_model_put".as_ptr(), size);

    buf = memdup_user(bytes, size);
    if IS_ERR(buf as *const c_void) {
        return PTR_ERR(buf as *const c_void);
    }

    sha256(buf, size as size_t, digest.as_mut_ptr());
    dev_dbg((*priv_).dev, c"hash=%*phN\n".as_ptr(), SHA256_DIGEST_SIZE, digest.as_ptr());

    p.cmd = EC_CODEC_WOV_GET_LANG;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_WOV,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec_wov>(),
        &mut r as *mut _ as *mut u8,
        size_of::<ec_response_ec_codec_wov_get_lang>(),
    );
    if ret != 0 {
        kfree(buf as *mut c_void);
        return ret;
    }

    if memcmp(digest.as_ptr() as *const c_void, r.hash.as_ptr() as *const c_void, SHA256_DIGEST_SIZE)
        == 0
    {
        dev_dbg((*priv_).dev, c"not updated".as_ptr());
        kfree(buf as *mut c_void);
        return ret;
    }

    if ec_codec_capable(priv_, EC_CODEC_CAP_WOV_LANG_SHM) != 0 {
        ret = wov_set_lang_shm(priv_, buf, size as size_t, digest.as_mut_ptr());
    } else {
        ret = wov_set_lang(priv_, buf, size as size_t, digest.as_mut_ptr());
    }

    kfree(buf as *mut c_void);
    ret
}

static mut wov_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { private_value: 0 },
    snd_kcontrol_new { private_value: 0 },
];

static mut wov_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"Wake on Voice".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"WoV Capture".as_ptr(),
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_16000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: ptr::null(),
};

unsafe extern "C" fn wov_host_event(
    nb: *mut notifier_block,
    _queued_during_suspend: c_ulong,
    _notify: *mut c_void,
) -> c_int {
    let priv_ = container_of_wov_notifier(nb);
    let host_event: u32;

    dev_dbg((*priv_).dev, c"%s\n".as_ptr(), c"wov_host_event".as_ptr());

    host_event = cros_ec_get_host_event((*priv_).ec_device);
    if host_event & EC_HOST_EVENT_MASK(EC_HOST_EVENT_WOV) != 0 {
        schedule_delayed_work(&mut (*priv_).wov_copy_work, 0);
        NOTIFY_OK
    } else {
        NOTIFY_DONE
    }
}

unsafe extern "C" fn wov_probe(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component);
    let mut ret: c_int;

    mutex_init(&mut (*priv_).wov_dma_lock);
    INIT_DELAYED_WORK(&mut (*priv_).wov_copy_work, Some(wov_copy_work));

    (*priv_).wov_notifier.notifier_call = Some(wov_host_event);
    ret = blocking_notifier_chain_register(
        &mut (*(*priv_).ec_device).event_notifier,
        &mut (*priv_).wov_notifier,
    );
    if ret != 0 {
        return ret;
    }

    if ec_codec_capable(priv_, EC_CODEC_CAP_WOV_LANG_SHM) != 0 {
        (*priv_).wov_lang_shm_p = wov_map_shm(
            priv_,
            EC_CODEC_SHM_ID_WOV_LANG,
            &mut (*priv_).wov_lang_shm_len,
            &mut (*priv_).wov_lang_shm_type,
        ) as *mut u8;
        if (*priv_).wov_lang_shm_p.is_null() {
            return -EFAULT;
        }
    }

    if ec_codec_capable(priv_, EC_CODEC_CAP_WOV_AUDIO_SHM) != 0 {
        (*priv_).wov_audio_shm_p = wov_map_shm(
            priv_,
            EC_CODEC_SHM_ID_WOV_AUDIO,
            &mut (*priv_).wov_audio_shm_len,
            &mut (*priv_).wov_audio_shm_type,
        ) as *mut u8;
        if (*priv_).wov_audio_shm_p.is_null() {
            return -EFAULT;
        }
    }

    dmic_probe(component)
}

unsafe extern "C" fn wov_remove(component: *mut snd_soc_component) {
    let priv_ = snd_soc_component_get_drvdata(component);

    blocking_notifier_chain_unregister(
        &mut (*(*priv_).ec_device).event_notifier,
        &mut (*priv_).wov_notifier,
    );
}

unsafe extern "C" fn wov_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    static hw_param: snd_pcm_hardware = snd_pcm_hardware {
        info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
        rates: SNDRV_PCM_RATE_16000,
        channels_min: 1,
        channels_max: 1,
        period_bytes_min: PAGE_SIZE as size_t,
        period_bytes_max: 0x20000 / 8,
        periods_min: 8,
        periods_max: 8,
        buffer_bytes_max: 0x20000,
    };

    snd_soc_set_runtime_hwparams(substream, &hw_param)
}

unsafe extern "C" fn wov_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    _hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component);

    mutex_lock(&mut (*priv_).wov_dma_lock);
    (*priv_).wov_substream = substream;
    (*priv_).wov_rp = 0;
    (*priv_).wov_wp = 0;
    (*priv_).wov_dma_offset = 0;
    (*priv_).wov_burst_read = true;
    mutex_unlock(&mut (*priv_).wov_dma_lock);

    0
}

unsafe extern "C" fn wov_pcm_hw_free(
    component: *mut snd_soc_component,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component);

    mutex_lock(&mut (*priv_).wov_dma_lock);
    wov_queue_dequeue(priv_, wov_queue_size(priv_));
    (*priv_).wov_substream = ptr::null_mut();
    mutex_unlock(&mut (*priv_).wov_dma_lock);

    cancel_delayed_work_sync(&mut (*priv_).wov_copy_work);

    0
}

unsafe extern "C" fn wov_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let priv_ = snd_soc_component_get_drvdata(component);

    bytes_to_frames(runtime, (*priv_).wov_dma_offset)
}

unsafe extern "C" fn wov_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_VMALLOC, ptr::null_mut(), 0, 0);
    0
}

static mut wov_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wov_probe),
    remove: Some(wov_remove),
    controls: unsafe { wov_controls.as_mut_ptr() },
    num_controls: 2,
    dapm_widgets: ptr::null_mut(),
    num_dapm_widgets: 0,
    dapm_routes: ptr::null_mut(),
    num_dapm_routes: 0,
    open: Some(wov_pcm_open),
    hw_params: Some(wov_pcm_hw_params),
    hw_free: Some(wov_pcm_hw_free),
    pointer: Some(wov_pcm_pointer),
    pcm_new: Some(wov_pcm_new),
    endianness: 0,
};

unsafe extern "C" fn cros_ec_codec_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device_with_parent as *mut device;
    let ec_device = dev_get_drvdata((*pdev).dev.parent) as *mut cros_ec_device;
    let priv_: *mut cros_ec_codec_priv;
    let mut p: ec_param_ec_codec = zeroed();
    let mut r: ec_response_ec_codec_get_capabilities = zeroed();
    let mut ret: c_int;

    /* CONFIG_OF: device tree shared-memory discovery is translated below. */
    let mut res: resource = zeroed();
    let mut ec_shm_size: u64 = 0;
    let mut regaddr_p: *const u32;

    priv_ = devm_kzalloc(dev, size_of::<cros_ec_codec_priv>(), GFP_KERNEL) as *mut cros_ec_codec_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    /* #ifdef CONFIG_OF */
    regaddr_p = of_get_address((*pdev).dev.of_node, 0, &mut ec_shm_size, ptr::null_mut());
    if !regaddr_p.is_null() {
        (*priv_).ec_shm_addr = of_read_number(regaddr_p, 2);
        (*priv_).ec_shm_len = ec_shm_size as u32;

        dev_dbg(
            dev,
            c"ec_shm_addr=%#llx len=%#x\n".as_ptr(),
            (*priv_).ec_shm_addr,
            (*priv_).ec_shm_len,
        );
    }

    ret = of_reserved_mem_region_to_resource((*pdev).dev.of_node, 0, &mut res);
    if ret == 0 {
        (*priv_).ap_shm_phys_addr = res.start;
        (*priv_).ap_shm_len = resource_size(&res) as u32;
        (*priv_).ap_shm_addr = devm_ioremap_wc(dev, (*priv_).ap_shm_phys_addr, (*priv_).ap_shm_len) as u64;
        (*priv_).ap_shm_last_alloc = (*priv_).ap_shm_phys_addr;

        dev_dbg(
            dev,
            c"ap_shm_phys_addr=%#llx len=%#x\n".as_ptr(),
            (*priv_).ap_shm_phys_addr,
            (*priv_).ap_shm_len,
        );
    }
    /* #endif */

    (*priv_).dev = dev;
    (*priv_).ec_device = ec_device;
    atomic_set(&mut (*priv_).dmic_probed, 0);

    p.cmd = EC_CODEC_GET_CAPABILITIES;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec>(),
        &mut r as *mut _ as *mut u8,
        size_of::<ec_response_ec_codec_get_capabilities>(),
    );
    if ret != 0 {
        dev_err(dev, c"failed to EC_CODEC_GET_CAPABILITIES\n".as_ptr());
        return ret;
    }
    (*priv_).ec_capabilities = r.capabilities;

    /* Reset EC codec i2s rx. */
    p.cmd = EC_CODEC_I2S_RX_RESET;
    ret = send_ec_host_command(
        (*priv_).ec_device,
        EC_CMD_EC_CODEC_I2S_RX,
        &mut p as *mut _ as *mut u8,
        size_of::<ec_param_ec_codec>(),
        ptr::null_mut(),
        0,
    );
    if ret == -ENOPROTOOPT {
        dev_info(dev, c"Missing reset command. Please update EC firmware.\n".as_ptr());
    } else if ret != 0 {
        dev_err(dev, c"failed to EC_CODEC_I2S_RESET: %d\n".as_ptr(), ret);
        return ret;
    }

    platform_set_drvdata(pdev, priv_ as *mut c_void);

    ret = devm_snd_soc_register_component(dev, &i2s_rx_component_driver, &mut i2s_rx_dai_driver, 1);
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(dev, &wov_component_driver, &mut wov_dai_driver, 1)
}

/* #ifdef CONFIG_OF */
static cros_ec_codec_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"google,cros-ec-codec".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, cros_ec_codec_of_match); */
/* #endif */

/* #ifdef CONFIG_ACPI */
static cros_ec_codec_acpi_id: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'G' as c_char,
            b'O' as c_char,
            b'O' as c_char,
            b'G' as c_char,
            b'0' as c_char,
            b'0' as c_char,
            b'1' as c_char,
            b'3' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 16],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(acpi, cros_ec_codec_acpi_id); */
/* #endif */

static mut cros_ec_codec_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"cros-ec-codec".as_ptr(),
        of_match_table: cros_ec_codec_of_match.as_ptr(),
        acpi_match_table: cros_ec_codec_acpi_id.as_ptr(),
    },
    probe: Some(cros_ec_codec_platform_probe),
};

/* module_platform_driver(cros_ec_codec_platform_driver); */

/* MODULE_LICENSE("GPL v2"); */
/* MODULE_DESCRIPTION("ChromeOS EC codec driver"); */
/* MODULE_AUTHOR("Cheng-Yi Chiang <cychiang@chromium.org>"); */
/* MODULE_ALIAS("platform:cros-ec-codec"); */

unsafe extern "C" {
    fn atomic_add_unless(v: *mut atomic_t, a: c_int, u: c_int) -> bool;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn INIT_DELAYED_WORK(
        work: *mut delayed_work,
        func: Option<unsafe extern "C" fn(*mut work_struct)>,
    );
    fn container_of_wov_copy_work(w: *mut work_struct) -> *mut cros_ec_codec_priv;
    fn container_of_wov_notifier(nb: *mut notifier_block) -> *mut cros_ec_codec_priv;
}

extern "C" {
    static EC_CODEC_DMIC_GET_GAIN_IDX: u8;
    static EC_CODEC_DMIC_SET_GAIN_IDX: u8;
    static EC_CODEC_DMIC_GET_MAX_GAIN: u8;
    static EC_CODEC_DMIC_CHANNEL_0: u8;
    static EC_CODEC_DMIC_CHANNEL_1: u8;
    static EC_CMD_EC_CODEC_DMIC: u32;
    static EC_CODEC_I2S_RX_SAMPLE_DEPTH_16: u32;
    static EC_CODEC_I2S_RX_SAMPLE_DEPTH_24: u32;
    static EC_CODEC_I2S_RX_SET_SAMPLE_DEPTH: u8;
    static EC_CMD_EC_CODEC_I2S_RX: u32;
    static EC_CODEC_I2S_RX_SET_BCLK: u8;
    static EC_CODEC_I2S_RX_DAIFMT_I2S: u32;
    static EC_CODEC_I2S_RX_DAIFMT_RIGHT_J: u32;
    static EC_CODEC_I2S_RX_DAIFMT_LEFT_J: u32;
    static EC_CODEC_I2S_RX_SET_DAIFMT: u8;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: u32;
    static SND_SOC_DAIFMT_CBC_CFC: u32;
    static SND_SOC_DAIFMT_INV_MASK: u32;
    static SND_SOC_DAIFMT_NB_NF: u32;
    static SND_SOC_DAIFMT_FORMAT_MASK: u32;
    static SND_SOC_DAIFMT_I2S: u32;
    static SND_SOC_DAIFMT_RIGHT_J: u32;
    static SND_SOC_DAIFMT_LEFT_J: u32;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64;
    static SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64;
    static SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static EC_CODEC_I2S_RX_ENABLE: u8;
    static EC_CODEC_I2S_RX_DISABLE: u8;
    static SND_SOC_NOPM: c_int;
    static SNDRV_PCM_RATE_48000: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static EC_CODEC_GET_SHM_ADDR: u8;
    static EC_CMD_EC_CODEC: u32;
    static EC_CODEC_SHM_TYPE_EC_RAM: u8;
    static EC_CODEC_SHM_TYPE_SYSTEM_RAM: u8;
    static EC_CODEC_SET_SHM_ADDR: u8;
    static EC_CODEC_WOV_READ_AUDIO_SHM: u8;
    static EC_CMD_EC_CODEC_WOV: u32;
    static EC_CODEC_WOV_READ_AUDIO: u8;
    static EC_CODEC_CAP_WOV_AUDIO_SHM: u8;
    static EC_CODEC_WOV_ENABLE: u8;
    static EC_CODEC_WOV_DISABLE: u8;
    static EC_CODEC_WOV_SET_LANG_SHM: u8;
    static EC_CODEC_WOV_SET_LANG: u8;
    static EC_CODEC_WOV_GET_LANG: u8;
    static EC_CODEC_CAP_WOV_LANG_SHM: u8;
    static EC_CODEC_SHM_ID_WOV_LANG: u8;
    static EC_CODEC_SHM_ID_WOV_AUDIO: u8;
    static EC_HOST_EVENT_WOV: u32;
    static NOTIFY_OK: c_int;
    static NOTIFY_DONE: c_int;
    static SNDRV_PCM_RATE_16000: u64;
    static SNDRV_PCM_INFO_MMAP: u64;
    static SNDRV_PCM_INFO_INTERLEAVED: u64;
    static SNDRV_PCM_INFO_MMAP_VALID: u64;
    static SNDRV_DMA_TYPE_VMALLOC: c_int;
    static EC_CODEC_GET_CAPABILITIES: u8;
    static EC_CODEC_I2S_RX_RESET: u8;
}

fn EC_HOST_EVENT_MASK(event: u32) -> u32 {
    1u32 << event
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
