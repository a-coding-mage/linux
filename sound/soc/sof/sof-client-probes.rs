// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2019-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//
// SOF client support:
//  Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//  Peter Ujfalusi <peter.ujfalusi@linux.intel.com>
//

// Dependencies from the original C includes:
// linux/debugfs.h, linux/module.h, linux/pm_runtime.h,
// linux/string_helpers.h, linux/stddef.h, sound/soc.h, sound/sof/header.h,
// sound/sof/ipc4/header.h, sof-client.h, sof-client-probes.h, sof-audio.h.
// CONFIG_SND_SOC_SOF_IPC4 also included ipc4-priv.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type u32 = u32;

const SOF_PROBES_SUSPEND_DELAY_MS: c_int = 3000;
/* only extraction supported for now */
const SOF_PROBES_NUM_DAI_LINKS: usize = 1;

const UINT_MAX: c_uint = c_uint::MAX;
const SOF_PROBES_INVALID_NODE_ID: c_uint = UINT_MAX;

const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const ENXIO: c_int = 6;
const GFP_KERNEL: c_uint = 0;
const PAGE_SIZE: usize = 4096;
const SNDRV_DMA_TYPE_DEV_SG: c_int = 2;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 14;

#[repr(C)]
pub struct snd_compr_stream {
    pub runtime: *mut snd_compr_runtime,
    pub dma_buffer: snd_dma_buffer,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_device,
}

#[repr(C)]
pub struct snd_dma_device {
    pub type_: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_compr_runtime {
    pub buffer_size: size_t,
    pub total_bytes_transferred: u64,
    pub dma_area: *mut c_void,
    pub dma_bytes: size_t,
}

#[repr(C)]
pub struct snd_compr_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
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
pub struct snd_soc_card {
    pub dev: *mut device,
    pub name: *const c_char,
    pub owner: *mut module,
    pub num_links: c_int,
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub id: c_int,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub nonatomic: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub compress_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_cdai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_soc_dai) -> c_int>,
    pub set_params:
        Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_params, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_compr_stream, c_int, *mut snd_soc_dai) -> c_int>,
    pub pointer:
        Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_tstamp64, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_compress_ops {
    pub copy:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut c_char, size_t) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver_capture {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub ops: *const snd_soc_dai_ops,
    pub cops: *const snd_soc_cdai_ops,
    pub capture: snd_soc_dai_driver_capture,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub compress_ops: *const snd_compress_ops,
    pub module_get_upon_open: c_uint,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_client_dev {
    pub auxdev: auxiliary_device,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct sof_probes_priv {
    pub host_ops: *const sof_probes_host_ops,
    pub ipc_ops: *const sof_probes_ipc_ops,
    pub extractor_stream_tag: c_uint,
    pub dfs_points: *mut dentry,
    pub dfs_points_remove: *mut dentry,
    pub card: snd_soc_card,
}

#[repr(C)]
pub struct sof_probes_host_ops {
    pub startup:
        Option<unsafe extern "C" fn(*mut sof_client_dev, *mut snd_compr_stream, *mut snd_soc_dai, *mut c_uint) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut sof_client_dev, *mut snd_compr_stream, *mut snd_soc_dai) -> c_int>,
    pub set_params: Option<
        unsafe extern "C" fn(*mut sof_client_dev, *mut snd_compr_stream, *mut snd_compr_params, *mut snd_soc_dai) -> c_int,
    >,
    pub trigger:
        Option<unsafe extern "C" fn(*mut sof_client_dev, *mut snd_compr_stream, c_int, *mut snd_soc_dai) -> c_int>,
    pub pointer: Option<
        unsafe extern "C" fn(*mut sof_client_dev, *mut snd_compr_stream, *mut snd_compr_tstamp64, *mut snd_soc_dai) -> c_int,
    >,
}

#[repr(C)]
pub struct sof_probes_ipc_ops {
    pub init: Option<unsafe extern "C" fn(*mut sof_client_dev, c_uint, size_t) -> c_int>,
    pub deinit: Option<unsafe extern "C" fn(*mut sof_client_dev) -> c_int>,
    pub points_info: Option<
        unsafe extern "C" fn(
            *mut sof_client_dev,
            *mut *mut sof_probe_point_desc,
            *mut size_t,
            sof_probe_info_type,
        ) -> c_int,
    >,
    pub points_add: Option<unsafe extern "C" fn(*mut sof_client_dev, *mut sof_probe_point_desc, size_t) -> c_int>,
    pub points_remove: Option<unsafe extern "C" fn(*mut sof_client_dev, *mut u32, size_t) -> c_int>,
    pub point_print:
        Option<unsafe extern "C" fn(*mut sof_client_dev, *mut c_char, c_int, *mut sof_probe_point_desc) -> c_int>,
}

#[repr(C)]
pub struct sof_probe_point_desc {
    pub buffer_id: u32,
    pub purpose: u32,
    pub stream_tag: u32,
}

type sof_probe_info_type = c_uint;
const PROBES_INFO_ACTIVE_PROBES: sof_probe_info_type = 0;
const PROBES_INFO_AVAILABE_PROBES: sof_probe_info_type = 1;

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    pub owner: *mut module,
}

#[repr(C)]
pub struct auxiliary_device {
    pub dev: device,
}

#[repr(C)]
pub struct auxiliary_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct auxiliary_driver {
    pub probe: Option<unsafe extern "C" fn(*mut auxiliary_device, *const auxiliary_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut auxiliary_device)>,
    pub id_table: *const auxiliary_device_id,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

const SOF_FW_CRASHED: c_int = 0;
const SOF_IPC_TYPE_4: c_int = 4;
const SOF_IPC_TYPE_3: c_int = 3;

static mut sof_probes_enabled: bool_ = false;

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static ipc4_probe_ops: sof_probes_ipc_ops;
    static ipc3_probe_ops: sof_probes_ipc_ops;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn sof_client_get_fw_state(cdev: *mut sof_client_dev) -> c_int;
    fn sof_client_core_module_get(cdev: *mut sof_client_dev) -> c_int;
    fn sof_client_core_module_put(cdev: *mut sof_client_dev);
    fn sof_client_get_dma_dev(cdev: *mut sof_client_dev) -> *mut device;
    fn snd_compr_malloc_pages(cstream: *mut snd_compr_stream, size: size_t) -> c_int;
    fn snd_compr_free_pages(cstream: *mut snd_compr_stream);
    fn sof_client_boot_dsp(cdev: *mut sof_client_dev) -> c_int;
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: size_t) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_void, available: size_t)
        -> ssize_t;
    fn parse_int_array_user(from: *const c_char, count: size_t, array: *mut *mut c_int) -> c_int;
    fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
    fn default_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn snd_soc_new_compress(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> c_int;
    fn auxiliary_dev_to_sof_client_dev(auxdev: *mut auxiliary_device) -> *mut sof_client_dev;
    fn sof_client_get_debugfs_root(cdev: *mut sof_client_dev) -> *mut dentry;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, idle_bias_on: bool_);
    fn sof_client_get_ipc_type(cdev: *mut sof_client_dev) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn div_u64_rem(dividend: u64, divisor: size_t, remainder: *mut c_uint) -> u64 {
    *remainder = (dividend % divisor as u64) as c_uint;
    dividend / divisor as u64
}

unsafe extern "C" fn sof_probes_compr_startup(
    cstream: *mut snd_compr_stream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card = snd_soc_component_get_drvdata((*dai).component) as *mut snd_soc_card;
    let cdev = snd_soc_card_get_drvdata(card) as *mut sof_client_dev;
    let priv_ = (*cdev).data as *mut sof_probes_priv;
    let ops = (*priv_).host_ops;
    let mut ret: c_int;

    if sof_client_get_fw_state(cdev) == SOF_FW_CRASHED {
        return -ENODEV;
    }

    ret = sof_client_core_module_get(cdev);
    if ret != 0 {
        return ret;
    }

    ret = ((*ops).startup.unwrap())(cdev, cstream, dai, &mut (*priv_).extractor_stream_tag);
    if ret != 0 {
        dev_err((*dai).dev, c"Failed to startup probe stream: %d\n".as_ptr(), ret);
        (*priv_).extractor_stream_tag = SOF_PROBES_INVALID_NODE_ID;
        sof_client_core_module_put(cdev);
    }

    ret
}

unsafe extern "C" fn sof_probes_compr_shutdown(
    cstream: *mut snd_compr_stream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card = snd_soc_component_get_drvdata((*dai).component) as *mut snd_soc_card;
    let cdev = snd_soc_card_get_drvdata(card) as *mut sof_client_dev;
    let priv_ = (*cdev).data as *mut sof_probes_priv;
    let ops = (*priv_).host_ops;
    let ipc = (*priv_).ipc_ops;
    let mut desc: *mut sof_probe_point_desc = ptr::null_mut();
    let mut num_desc: size_t = 0;
    let mut i: c_int;
    let mut ret: c_int;

    /* disconnect all probe points */
    ret = ((*ipc).points_info.unwrap())(cdev, &mut desc, &mut num_desc, PROBES_INFO_ACTIVE_PROBES);
    if ret < 0 {
        dev_err((*dai).dev, c"Failed to get probe points: %d\n".as_ptr(), ret);
    } else {
        i = 0;
        while (i as size_t) < num_desc {
            ((*ipc).points_remove.unwrap())(cdev, &mut (*desc.add(i as usize)).buffer_id, 1);
            i += 1;
        }
        kfree(desc as *mut c_void);
    }

    ret = ((*ipc).deinit.unwrap())(cdev);
    if ret < 0 {
        dev_err((*dai).dev, c"Failed to deinit probe: %d\n".as_ptr(), ret);
    }

    (*priv_).extractor_stream_tag = SOF_PROBES_INVALID_NODE_ID;
    snd_compr_free_pages(cstream);

    ret = ((*ops).shutdown.unwrap())(cdev, cstream, dai);

    sof_client_core_module_put(cdev);

    ret
}

unsafe extern "C" fn sof_probes_compr_set_params(
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card = snd_soc_component_get_drvdata((*dai).component) as *mut snd_soc_card;
    let cdev = snd_soc_card_get_drvdata(card) as *mut sof_client_dev;
    let rtd = (*cstream).runtime;
    let priv_ = (*cdev).data as *mut sof_probes_priv;
    let ops = (*priv_).host_ops;
    let ipc = (*priv_).ipc_ops;
    let mut ret: c_int;

    (*cstream).dma_buffer.dev.type_ = SNDRV_DMA_TYPE_DEV_SG;
    (*cstream).dma_buffer.dev.dev = sof_client_get_dma_dev(cdev);
    ret = snd_compr_malloc_pages(cstream, (*rtd).buffer_size);
    if ret < 0 {
        return ret;
    }

    ret = ((*ops).set_params.unwrap())(cdev, cstream, params, dai);
    if ret != 0 {
        return ret;
    }

    ret = sof_client_boot_dsp(cdev);
    if ret != 0 {
        return ret;
    }

    ret = ((*ipc).init.unwrap())(cdev, (*priv_).extractor_stream_tag, (*rtd).dma_bytes);
    if ret < 0 {
        dev_err((*dai).dev, c"Failed to init probe: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn sof_probes_compr_trigger(
    cstream: *mut snd_compr_stream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card = snd_soc_component_get_drvdata((*dai).component) as *mut snd_soc_card;
    let cdev = snd_soc_card_get_drvdata(card) as *mut sof_client_dev;
    let priv_ = (*cdev).data as *mut sof_probes_priv;
    let ops = (*priv_).host_ops;

    ((*ops).trigger.unwrap())(cdev, cstream, cmd, dai)
}

unsafe extern "C" fn sof_probes_compr_pointer(
    cstream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card = snd_soc_component_get_drvdata((*dai).component) as *mut snd_soc_card;
    let cdev = snd_soc_card_get_drvdata(card) as *mut sof_client_dev;
    let priv_ = (*cdev).data as *mut sof_probes_priv;
    let ops = (*priv_).host_ops;

    ((*ops).pointer.unwrap())(cdev, cstream, tstamp, dai)
}

static sof_probes_compr_ops: snd_soc_cdai_ops = snd_soc_cdai_ops {
    startup: Some(sof_probes_compr_startup),
    shutdown: Some(sof_probes_compr_shutdown),
    set_params: Some(sof_probes_compr_set_params),
    trigger: Some(sof_probes_compr_trigger),
    pointer: Some(sof_probes_compr_pointer),
};

unsafe extern "C" fn sof_probes_compr_copy(
    _component: *mut snd_soc_component,
    cstream: *mut snd_compr_stream,
    buf: *mut c_char,
    mut count: size_t,
) -> c_int {
    let rtd = (*cstream).runtime;
    let mut offset: c_uint = 0;
    let n: c_uint;
    let ptr_: *mut c_void;
    let mut ret: c_int;

    if count > (*rtd).buffer_size {
        count = (*rtd).buffer_size;
    }

    div_u64_rem((*rtd).total_bytes_transferred, (*rtd).buffer_size, &mut offset);
    ptr_ = ((*rtd).dma_area as *mut u8).add(offset as usize) as *mut c_void;
    n = ((*rtd).buffer_size as c_uint).wrapping_sub(offset);

    if count < n as size_t {
        ret = copy_to_user(buf, ptr_, count);
    } else {
        ret = copy_to_user(buf, ptr_, n as size_t);
        ret += copy_to_user(buf.add(n as usize), (*rtd).dma_area, count - n as size_t);
    }

    if ret != 0 {
        return (count as c_int) - ret;
    }
    count as c_int
}

static sof_probes_compressed_ops: snd_compress_ops = snd_compress_ops {
    copy: Some(sof_probes_compr_copy),
};

unsafe extern "C" fn sof_probes_dfs_points_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
    type_: sof_probe_info_type,
) -> ssize_t {
    let cdev = (*file).private_data as *mut sof_client_dev;
    let priv_ = (*cdev).data as *mut sof_probes_priv;
    let dev = &mut (*cdev).auxdev.dev as *mut device;
    let mut desc: *mut sof_probe_point_desc = ptr::null_mut();
    let ipc = (*priv_).ipc_ops;
    let mut remaining: c_int;
    let mut offset: c_int;
    let mut num_desc: size_t = 0;
    let buf: *mut c_char;
    let mut i: c_int;
    let mut ret: c_int;
    let mut err: c_int;

    if (*priv_).extractor_stream_tag == SOF_PROBES_INVALID_NODE_ID {
        dev_warn(dev, c"no extractor stream running\n".as_ptr());
        return -(ENOENT as ssize_t);
    }

    buf = kzalloc(PAGE_SIZE, GFP_KERNEL);
    if buf.is_null() {
        return -(ENOMEM as ssize_t);
    }

    ret = pm_runtime_resume_and_get(dev);
    if ret < 0 && ret != -EACCES {
        dev_err_ratelimited(dev, c"debugfs read failed to resume %d\n".as_ptr(), ret);
        kfree(buf as *mut c_void);
        return ret as ssize_t;
    }

    ret = sof_client_boot_dsp(cdev);
    if ret == 0 {
        ret = ((*ipc).points_info.unwrap())(cdev, &mut desc, &mut num_desc, type_);
    }

    if ret >= 0 {
        i = 0;
        while (i as size_t) < num_desc {
            offset = strlen(buf) as c_int;
            remaining = PAGE_SIZE as c_int - offset;
            if let Some(point_print) = (*ipc).point_print {
                ret = point_print(cdev, buf.add(offset as usize), remaining, &mut *desc.add(i as usize));
            } else {
                ret = snprintf(
                    buf.add(offset as usize),
                    remaining as size_t,
                    c"Id: %#010x  Purpose: %u  Node id: %#x\n".as_ptr(),
                    (*desc.add(i as usize)).buffer_id,
                    (*desc.add(i as usize)).purpose,
                    (*desc.add(i as usize)).stream_tag,
                );
            }

            if ret < 0 || ret >= remaining {
                /* truncate the output buffer at the last full line */
                *buf.add(offset as usize) = 0;
                break;
            }
            i += 1;
        }

        ret = simple_read_from_buffer(to, count, ppos, buf as *const c_void, strlen(buf)) as c_int;

        kfree(desc as *mut c_void);
    }

    err = pm_runtime_put_autosuspend(dev);
    if err < 0 {
        dev_err_ratelimited(dev, c"debugfs read failed to idle %d\n".as_ptr(), err);
    }

    kfree(buf as *mut c_void);
    ret as ssize_t
}

unsafe extern "C" fn sof_probes_dfs_active_points_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    sof_probes_dfs_points_read(file, to, count, ppos, PROBES_INFO_ACTIVE_PROBES)
}

unsafe extern "C" fn sof_probes_dfs_available_points_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    sof_probes_dfs_points_read(file, to, count, ppos, PROBES_INFO_AVAILABE_PROBES)
}

unsafe extern "C" fn sof_probes_dfs_points_write(
    file: *mut file,
    from: *const c_char,
    count: size_t,
    _ppos: *mut loff_t,
) -> ssize_t {
    let cdev = (*file).private_data as *mut sof_client_dev;
    let priv_ = (*cdev).data as *mut sof_probes_priv;
    let ipc = (*priv_).ipc_ops;
    let dev = &mut (*cdev).auxdev.dev as *mut device;
    let desc: *mut sof_probe_point_desc;
    let num_elems: u32;
    let mut array: *mut u32 = ptr::null_mut();
    let bytes: size_t;
    let mut ret: c_int;
    let mut err: c_int;

    if (*priv_).extractor_stream_tag == SOF_PROBES_INVALID_NODE_ID {
        dev_warn(dev, c"no extractor stream running\n".as_ptr());
        return -(ENOENT as ssize_t);
    }

    ret = parse_int_array_user(from, count, &mut array as *mut *mut u32 as *mut *mut c_int);
    if ret < 0 {
        return ret as ssize_t;
    }

    num_elems = *array;
    bytes = size_of::<u32>() * num_elems as usize;
    if bytes % size_of::<sof_probe_point_desc>() != 0 {
        ret = -EINVAL;
        kfree(array as *mut c_void);
        return ret as ssize_t;
    }

    desc = array.add(1) as *mut sof_probe_point_desc;

    ret = pm_runtime_resume_and_get(dev);
    if ret < 0 && ret != -EACCES {
        dev_err_ratelimited(dev, c"debugfs write failed to resume %d\n".as_ptr(), ret);
        kfree(array as *mut c_void);
        return ret as ssize_t;
    }

    ret = sof_client_boot_dsp(cdev);
    if ret == 0 {
        ret = ((*ipc).points_add.unwrap())(cdev, desc, bytes / size_of::<sof_probe_point_desc>());
        if ret == 0 {
            ret = count as c_int;
        }
    }

    err = pm_runtime_put_autosuspend(dev);
    if err < 0 {
        dev_err_ratelimited(dev, c"debugfs write failed to idle %d\n".as_ptr(), err);
    }
    kfree(array as *mut c_void);
    ret as ssize_t
}

static sof_probes_active_points_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(sof_probes_dfs_active_points_read),
    write: Some(sof_probes_dfs_points_write),
    llseek: Some(default_llseek),

    owner: unsafe { THIS_MODULE },
};

static sof_probes_available_points_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(sof_probes_dfs_available_points_read),
    write: None,
    llseek: Some(default_llseek),

    owner: unsafe { THIS_MODULE },
};

unsafe extern "C" fn sof_probes_dfs_points_remove_write(
    file: *mut file,
    from: *const c_char,
    count: size_t,
    _ppos: *mut loff_t,
) -> ssize_t {
    let cdev = (*file).private_data as *mut sof_client_dev;
    let priv_ = (*cdev).data as *mut sof_probes_priv;
    let ipc = (*priv_).ipc_ops;
    let dev = &mut (*cdev).auxdev.dev as *mut device;
    let mut ret: c_int;
    let mut err: c_int;
    let mut array: *mut u32 = ptr::null_mut();

    if (*priv_).extractor_stream_tag == SOF_PROBES_INVALID_NODE_ID {
        dev_warn(dev, c"no extractor stream running\n".as_ptr());
        return -(ENOENT as ssize_t);
    }

    ret = parse_int_array_user(from, count, &mut array as *mut *mut u32 as *mut *mut c_int);
    if ret < 0 {
        return ret as ssize_t;
    }

    ret = pm_runtime_resume_and_get(dev);
    if ret < 0 {
        dev_err_ratelimited(dev, c"debugfs write failed to resume %d\n".as_ptr(), ret);
        kfree(array as *mut c_void);
        return ret as ssize_t;
    }

    ret = sof_client_boot_dsp(cdev);
    if ret == 0 {
        ret = ((*ipc).points_remove.unwrap())(cdev, array.add(1), *array as size_t);
        if ret == 0 {
            ret = count as c_int;
        }
    }

    err = pm_runtime_put_autosuspend(dev);
    if err < 0 {
        dev_err_ratelimited(dev, c"debugfs write failed to idle %d\n".as_ptr(), err);
    }
    kfree(array as *mut c_void);
    ret as ssize_t
}

static sof_probes_points_remove_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: None,
    write: Some(sof_probes_dfs_points_remove_write),
    llseek: Some(default_llseek),

    owner: unsafe { THIS_MODULE },
};

static sof_probes_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    compress_new: Some(snd_soc_new_compress),
};

static mut sof_probes_dai_drv: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"Probe Extraction CPU DAI".as_ptr(),
    ops: &sof_probes_dai_ops,
    cops: &sof_probes_compr_ops,
    capture: snd_soc_dai_driver_capture {
        stream_name: c"Probe Extraction".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_48000,
        rate_min: 48000,
        rate_max: 48000,
    },
}];

static sof_probes_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"sof-probes-component".as_ptr(),
    compress_ops: &sof_probes_compressed_ops,
    module_get_upon_open: 1,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn sof_probes_client_probe(
    auxdev: *mut auxiliary_device,
    _id: *const auxiliary_device_id,
) -> c_int {
    let cdev = auxiliary_dev_to_sof_client_dev(auxdev);
    let dfsroot = sof_client_get_debugfs_root(cdev);
    let dev = &mut (*auxdev).dev as *mut device;
    let mut platform_component = [snd_soc_dai_link_component {
        name: dev_name(dev),
        dai_name: ptr::null(),
    }];
    let card: *mut snd_soc_card;
    let priv_: *mut sof_probes_priv;
    let cpus: *mut snd_soc_dai_link_component;
    let ops: *mut sof_probes_host_ops;
    let links: *mut snd_soc_dai_link;
    let mut ret: c_int;

    /* do not set up the probes support if it is not enabled */
    if !sof_probes_enabled {
        return -ENXIO;
    }

    ops = dev_get_platdata(dev) as *mut sof_probes_host_ops;
    if ops.is_null() {
        dev_err(dev, c"missing platform data\n".as_ptr());
        return -ENODEV;
    }
    if (*ops).startup.is_none()
        || (*ops).shutdown.is_none()
        || (*ops).set_params.is_none()
        || (*ops).trigger.is_none()
        || (*ops).pointer.is_none()
    {
        dev_err(dev, c"missing platform callback(s)\n".as_ptr());
        return -ENODEV;
    }

    priv_ = devm_kzalloc(dev, size_of::<sof_probes_priv>(), GFP_KERNEL) as *mut sof_probes_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).host_ops = ops;

    match sof_client_get_ipc_type(cdev) {
        // CONFIG_SND_SOC_SOF_IPC4
        SOF_IPC_TYPE_4 => {
            (*priv_).ipc_ops = &ipc4_probe_ops;
        }
        // CONFIG_SND_SOC_SOF_IPC3
        SOF_IPC_TYPE_3 => {
            (*priv_).ipc_ops = &ipc3_probe_ops;
        }
        _ => {
            dev_err(dev, c"Matching IPC ops not found.".as_ptr());
            return -ENODEV;
        }
    }

    (*cdev).data = priv_ as *mut c_void;

    /* register probes component driver and dai */
    ret = devm_snd_soc_register_component(
        dev,
        &sof_probes_component,
        sof_probes_dai_drv.as_mut_ptr(),
        sof_probes_dai_drv.len() as c_int,
    );
    if ret < 0 {
        dev_err(dev, c"failed to register SOF probes DAI driver %d\n".as_ptr(), ret);
        return ret;
    }

    /* set client data */
    (*priv_).extractor_stream_tag = SOF_PROBES_INVALID_NODE_ID;

    /* create read-write probes_points debugfs entry */
    (*priv_).dfs_points = debugfs_create_file(
        c"probe_points".as_ptr(),
        0o644,
        dfsroot,
        cdev as *mut c_void,
        &sof_probes_active_points_fops,
    );

    /* create read-write probe_points_remove debugfs entry */
    (*priv_).dfs_points_remove = debugfs_create_file(
        c"probe_points_remove".as_ptr(),
        0o644,
        dfsroot,
        cdev as *mut c_void,
        &sof_probes_points_remove_fops,
    );

    /* create read-write probes_points debugfs entry */
    (*priv_).dfs_points = debugfs_create_file(
        c"probe_points_available".as_ptr(),
        0o644,
        dfsroot,
        cdev as *mut c_void,
        &sof_probes_available_points_fops,
    );

    links = devm_kcalloc(
        dev,
        SOF_PROBES_NUM_DAI_LINKS,
        size_of::<snd_soc_dai_link>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    cpus = devm_kcalloc(
        dev,
        SOF_PROBES_NUM_DAI_LINKS,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if links.is_null() || cpus.is_null() {
        debugfs_remove((*priv_).dfs_points);
        debugfs_remove((*priv_).dfs_points_remove);
        return -ENOMEM;
    }

    /* extraction DAI link */
    (*links.add(0)).name = c"Compress Probe Capture".as_ptr();
    (*links.add(0)).id = 0;
    (*links.add(0)).cpus = &mut *cpus.add(0);
    (*links.add(0)).num_cpus = 1;
    (*(*links.add(0)).cpus).dai_name = c"Probe Extraction CPU DAI".as_ptr();
    (*links.add(0)).codecs = &mut snd_soc_dummy_dlc;
    (*links.add(0)).num_codecs = 1;
    (*links.add(0)).platforms = platform_component.as_mut_ptr();
    (*links.add(0)).num_platforms = platform_component.len() as c_uint;
    (*links.add(0)).nonatomic = 1;

    card = &mut (*priv_).card;

    (*card).dev = dev;
    (*card).name = c"sof-probes".as_ptr();
    (*card).owner = THIS_MODULE;
    (*card).num_links = SOF_PROBES_NUM_DAI_LINKS as c_int;
    (*card).dai_link = links;

    snd_soc_card_set_drvdata(card, cdev as *mut c_void);

    ret = devm_snd_soc_register_card(dev, card);
    if ret < 0 {
        debugfs_remove((*priv_).dfs_points);
        debugfs_remove((*priv_).dfs_points_remove);
        dev_err(dev, c"Probes card register failed %d\n".as_ptr(), ret);
        return ret;
    }

    /*
     * set idle_bias_off to prevent the core from resuming the card->dev
     * call it after snd_soc_register_card()
     */
    let dapm = snd_soc_card_to_dapm(card);

    snd_soc_dapm_set_idle_bias(dapm, false);

    /* enable runtime PM */
    pm_runtime_set_autosuspend_delay(dev, SOF_PROBES_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_enable(dev);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_idle(dev);

    0
}

unsafe extern "C" fn sof_probes_client_remove(auxdev: *mut auxiliary_device) {
    let cdev = auxiliary_dev_to_sof_client_dev(auxdev);
    let priv_ = (*cdev).data as *mut sof_probes_priv;

    if !sof_probes_enabled {
        return;
    }

    pm_runtime_disable(&mut (*auxdev).dev);
    debugfs_remove((*priv_).dfs_points);
    debugfs_remove((*priv_).dfs_points_remove);
}

static sof_probes_client_id_table: [auxiliary_device_id; 3] = [
    auxiliary_device_id {
        name: c"snd_sof.hda-probes".as_ptr(),
    },
    auxiliary_device_id {
        name: c"snd_sof.acp-probes".as_ptr(),
    },
    auxiliary_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(auxiliary, sof_probes_client_id_table);

/* driver name will be set based on KBUILD_MODNAME */
static mut sof_probes_client_drv: auxiliary_driver = auxiliary_driver {
    probe: Some(sof_probes_client_probe),
    remove: Some(sof_probes_client_remove),

    id_table: sof_probes_client_id_table.as_ptr(),
};

// module_auxiliary_driver(sof_probes_client_drv);

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("SOF Probes Client Driver");
// MODULE_IMPORT_NS("SND_SOC_SOF_CLIENT");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
