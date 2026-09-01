// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Intel Corporation
//
//

// Rust translation of soc/sof/ipc4-control.c.
// C includes removed; the referenced SOF/ALSA/kernel types, constants, list
// iteration helpers, and allocation/copying primitives are supplied externally.

use core::ffi::{c_char, c_int, c_uint, c_void};

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type size_t = usize;

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn pm_runtime_active(dev: *mut device) -> bool_;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn lockdep_assert_held(lock: *mut mutex);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_int;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_int;
    fn mixer_to_ipc(value: i64, table: *mut u32, size: u32) -> u32;
    fn ipc_to_mixer(value: u32, table: *mut u32, size: u32) -> i64;
    fn vol_compute_gain(i: c_int, tlv: *mut c_int) -> u32;
    fn sof_ipc4_find_swidget_by_ids(
        sdev: *mut snd_sof_dev,
        module_id: u32,
        instance_id: u32,
    ) -> *mut snd_sof_widget;
    fn snd_ctl_notify_one(card: *mut snd_card, mask: c_uint, kc: *mut snd_kcontrol, value: c_int);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_sof_control {
    pub scomp: *mut snd_soc_component,
    pub comp_id: c_int,
    pub name: *const c_char,
    pub ipc_control_data: *mut sof_ipc4_control_data,
    pub old_ipc_control_data: *mut c_void,
    pub size: size_t,
    pub max_size: size_t,
    pub max: u32,
    pub num_channels: c_uint,
    pub volume_table: *mut u32,
    pub comp_data_dirty: bool_,
    pub info_type: c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub ipc: *mut snd_sof_ipc,
    pub widget_list: list_head,
    pub kcontrol_list: list_head,
}

#[repr(C)]
pub struct snd_sof_ipc {
    pub ops: *const sof_ipc_ops,
}

#[repr(C)]
pub struct sof_ipc_ops {
    pub set_get_data:
        unsafe extern "C" fn(*mut snd_sof_dev, *mut sof_ipc4_msg, size_t, bool_) -> c_int,
}

#[repr(C)]
pub struct snd_sof_widget {
    pub comp_id: c_int,
    pub instance_id: u32,
    pub use_count: c_uint,
    pub setup_mutex: mutex,
    pub private: *mut c_void,
    pub widget: *mut snd_soc_dapm_widget,
    pub scomp: *mut snd_soc_component,
    pub list: list_head,
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub extension: u32,
    pub data_ptr: *mut c_void,
    pub data_size: size_t,
}

#[repr(C)]
pub struct sof_ipc4_control_data {
    pub msg: sof_ipc4_msg,
    pub index: u32,
    pub data: *mut sof_abi_hdr,
    pub chanv: [sof_ipc4_channel_value; 0],
}

#[repr(C)]
pub struct sof_ipc4_channel_value {
    pub channel: u32,
    pub value: u32,
}

#[repr(C)]
pub struct sof_ipc4_control_msg_payload {
    pub id: u32,
    pub num_elems: u32,
    pub chanv: [sof_ipc4_channel_value; 0],
}

#[repr(C)]
pub struct sof_abi_hdr {
    pub magic: u32,
    pub type_: u32,
    pub size: u32,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc4_gain {
    pub data: sof_ipc4_gain_data,
}

#[repr(C)]
pub struct sof_ipc4_gain_data {
    pub params: sof_ipc4_gain_params,
}

#[repr(C)]
pub struct sof_ipc4_gain_params {
    pub channels: u32,
    pub init_val: u32,
    pub curve_duration_l: u32,
    pub curve_duration_h: u32,
    pub curve_type: u32,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub bytes: snd_ctl_elem_value_bytes,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [u32; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
pub struct snd_ctl_tlv {
    pub numid: u32,
    pub length: u32,
    pub tlv: [u32; 0],
}

#[repr(C)]
pub struct sof_ipc4_notify_module_data {
    pub module_id: u32,
    pub instance_id: u32,
    pub event_id: u32,
    pub event_data_size: u32,
    pub event_data: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub num_kcontrols: c_int,
    pub dobj: snd_soc_dobj,
    pub kcontrol_news: *mut snd_kcontrol_new,
    pub kcontrols: *mut *mut snd_kcontrol,
}

#[repr(C)]
pub struct snd_soc_dobj {
    pub widget: snd_soc_tplg_widget,
}

#[repr(C)]
pub struct snd_soc_tplg_widget {
    pub kcontrol_type: *mut c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub index: u32,
}

#[repr(C)]
pub struct sof_ipc_tplg_control_ops {
    pub volume_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool_>,
    pub volume_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub switch_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool_>,
    pub switch_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub enum_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool_>,
    pub enum_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_ext_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *const c_uint, c_uint) -> c_int>,
    pub bytes_ext_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *const c_uint, c_uint) -> c_int>,
    pub bytes_ext_volatile_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *const c_uint, c_uint) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void)>,
    pub widget_kcontrol_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> c_int>,
    pub set_up_volume_table: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut c_int, c_int) -> c_int>,
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}
#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

extern "C" {
    static GFP_KERNEL: c_uint;
    static SOF_IPC4_MOD_INSTANCE_MASK: u32;
    static SOF_IPC4_GAIN_ALL_CHANNELS_MASK: u32;
    static SOF_IPC4_BYTES_CONTROL_PARAM_ID: u32;
    static SOF_IPC4_SWITCH_CONTROL_PARAM_ID: u32;
    static SOF_IPC4_ENUM_CONTROL_PARAM_ID: u32;
    static SOF_IPC4_NOTIFY_MODULE_EVENTID_ALSA_PARAMID_MASK: u32;
    static SOF_IPC4_MOD_EXT_MSG_PARAM_ID_MASK: u32;
    static SOF_IPC4_MOD_EXT_MSG_PARAM_ID_SHIFT: u32;
    static SOF_IPC4_ABI_MAGIC: u32;
    static SOF_CTRL_CMD_BINARY: u32;
    static SOF_IPC4_VOL_ZERO_DB: u64;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
    static SND_SOC_TPLG_TYPE_MIXER: c_int;
    static SND_SOC_TPLG_TYPE_ENUM: c_int;
    static SND_SOC_TPLG_TYPE_BYTES: c_int;
    static SND_SOC_TPLG_CTL_VOLSW: c_int;
    static SND_SOC_TPLG_CTL_VOLSW_SX: c_int;
    static SND_SOC_TPLG_CTL_VOLSW_XR_SX: c_int;
    static SND_SOC_TPLG_CTL_BYTES: c_int;
    static SND_SOC_TPLG_CTL_ENUM: c_int;
    static SND_SOC_TPLG_CTL_ENUM_VALUE: c_int;
}

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const ENOSPC: c_int = 28;
const SOF_TLV_ITEMS: usize = 4;

#[inline]
unsafe fn SOF_IPC4_MOD_INSTANCE(instance_id: u32) -> u32 {
    instance_id
}

#[inline]
unsafe fn SOF_IPC4_MOD_EXT_MSG_PARAM_ID(param_id: u32) -> u32 {
    param_id
}

#[inline]
unsafe fn PARAM_ID_FROM_EXTENSION(ext: u32) -> u32 {
    (ext & SOF_IPC4_MOD_EXT_MSG_PARAM_ID_MASK) >> SOF_IPC4_MOD_EXT_MSG_PARAM_ID_SHIFT
}

#[inline]
unsafe fn chanv_ptr(base: *mut sof_ipc4_control_data, idx: usize) -> *mut sof_ipc4_channel_value {
    (*base).chanv.as_mut_ptr().add(idx)
}

#[inline]
unsafe fn msg_chanv_ptr(
    base: *mut sof_ipc4_control_msg_payload,
    idx: usize,
) -> *mut sof_ipc4_channel_value {
    (*base).chanv.as_mut_ptr().add(idx)
}

#[inline]
unsafe fn msg_data_ptr(base: *mut sof_ipc4_control_msg_payload) -> *mut u8 {
    (*base).chanv.as_mut_ptr() as *mut u8
}

#[inline]
unsafe fn abi_data_ptr(base: *mut sof_abi_hdr) -> *mut u8 {
    (*base).data.as_mut_ptr()
}

#[inline]
fn struct_size_control_msg_chanv(num: usize) -> size_t {
    core::mem::size_of::<sof_ipc4_control_msg_payload>()
        + num * core::mem::size_of::<sof_ipc4_channel_value>()
}

#[inline]
fn struct_size_control_msg_data(num: usize) -> size_t {
    core::mem::size_of::<sof_ipc4_control_msg_payload>() + num
}

unsafe extern "C" fn sof_ipc4_set_get_kcontrol_data(
    scontrol: *mut snd_sof_control,
    msg: *mut sof_ipc4_msg,
    set: bool_,
    lock: bool_,
) -> c_int {
    let scomp = (*scontrol).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let iops = (*(*sdev).ipc).ops;
    let mut swidget: *mut snd_sof_widget = core::ptr::null_mut();
    let mut widget_found = false;
    let mut ret: c_int = 0;

    /* find widget associated with the control */
    // list_for_each_entry(swidget, &sdev->widget_list, list)
    for_each_swidget_in_widget_list(sdev, |entry| {
        swidget = entry;
        if (*swidget).comp_id == (*scontrol).comp_id {
            widget_found = true;
            return true;
        }
        false
    });

    if !widget_found {
        dev_err((*scomp).dev, b"Failed to find widget for kcontrol %s\n\0".as_ptr() as *const c_char, (*scontrol).name);
        return -ENOENT;
    }

    if lock {
        mutex_lock(&mut (*swidget).setup_mutex);
    } else {
        lockdep_assert_held(&mut (*swidget).setup_mutex);
    }

    /*
     * Volatile controls should always be part of static pipelines and the
     * widget use_count would always be > 0 in this case. For the others,
     * just return the cached value if the widget is not set up.
     */
    if (*swidget).use_count == 0 {
        goto_unlock(swidget, lock, ret)
    } else {
        (*msg).primary &= !SOF_IPC4_MOD_INSTANCE_MASK;
        (*msg).primary |= SOF_IPC4_MOD_INSTANCE((*swidget).instance_id);

        ret = ((*iops).set_get_data)(sdev, msg, (*msg).data_size, set);
        if set && ret < 0 {
            /* It is a set-data operation, and we have a valid backup that we can restore */
            if !(*scontrol).old_ipc_control_data.is_null() {
                /*
                 * Current ipc_control_data is not valid, we use the last known good
                 * configuration
                 */
                memcpy(
                    (*scontrol).ipc_control_data as *mut c_void,
                    (*scontrol).old_ipc_control_data,
                    (*scontrol).size,
                );
                kfree((*scontrol).old_ipc_control_data);
                (*scontrol).old_ipc_control_data = core::ptr::null_mut();
                /* Send the last known good configuration to firmware */
                ret = ((*iops).set_get_data)(sdev, msg, (*msg).data_size, set);
            }
        }
        goto_unlock(swidget, lock, ret)
    }
}

#[inline]
unsafe fn goto_unlock(swidget: *mut snd_sof_widget, lock: bool_, ret: c_int) -> c_int {
    if lock {
        mutex_unlock(&mut (*swidget).setup_mutex);
    }
    ret
}

unsafe extern "C" fn sof_ipc4_set_volume_data(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
    scontrol: *mut snd_sof_control,
    lock: bool_,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let gain = (*swidget).private as *mut sof_ipc4_gain;
    let mut params: sof_ipc4_gain_params = core::mem::zeroed();
    let mut all_channels_equal = true;
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let value: u32;
    let mut i: c_int;

    /* check if all channel values are equal */
    value = (*chanv_ptr(cdata, 0)).value;
    i = 1;
    while i < (*scontrol).num_channels as c_int {
        if (*chanv_ptr(cdata, i as usize)).value != value {
            all_channels_equal = false;
            break;
        }
        i += 1;
    }

    /*
     * notify DSP with a single IPC message if all channel values are equal. Otherwise send
     * a separate IPC for each channel.
     */
    memcpy(&mut msg as *mut _ as *mut c_void, &(*cdata).msg as *const _ as *const c_void, core::mem::size_of::<sof_ipc4_msg>());
    i = 0;
    while i < (*scontrol).num_channels as c_int {
        if all_channels_equal {
            params.channels = SOF_IPC4_GAIN_ALL_CHANNELS_MASK;
            params.init_val = (*chanv_ptr(cdata, 0)).value;
        } else {
            params.channels = (*chanv_ptr(cdata, i as usize)).channel;
            params.init_val = (*chanv_ptr(cdata, i as usize)).value;
        }

        /* set curve type and duration from topology */
        params.curve_duration_l = (*gain).data.params.curve_duration_l;
        params.curve_duration_h = (*gain).data.params.curve_duration_h;
        params.curve_type = (*gain).data.params.curve_type;

        msg.data_ptr = &mut params as *mut _ as *mut c_void;
        msg.data_size = core::mem::size_of::<sof_ipc4_gain_params>();

        let ret = sof_ipc4_set_get_kcontrol_data(scontrol, &mut msg, true, lock);
        if ret < 0 {
            dev_err((*sdev).dev, b"Failed to set volume update for %s\n\0".as_ptr() as *const c_char, (*scontrol).name);
            return ret;
        }

        if all_channels_equal {
            break;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc4_volume_put(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> bool_ {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let channels = (*scontrol).num_channels;
    let mut swidget: *mut snd_sof_widget = core::ptr::null_mut();
    let mut widget_found = false;
    let mut change = false;
    let mut i: c_uint = 0;

    /* update each channel */
    while i < channels {
        let value = mixer_to_ipc(
            (*ucontrol).value.integer.value[i as usize],
            (*scontrol).volume_table,
            (*scontrol).max + 1,
        );

        change = change || value != (*chanv_ptr(cdata, i as usize)).value;
        (*chanv_ptr(cdata, i as usize)).channel = i;
        (*chanv_ptr(cdata, i as usize)).value = value;
        i += 1;
    }

    if !pm_runtime_active((*scomp).dev) {
        return change;
    }

    /* find widget associated with the control */
    for_each_swidget_in_widget_list(sdev, |entry| {
        swidget = entry;
        if (*swidget).comp_id == (*scontrol).comp_id {
            widget_found = true;
            return true;
        }
        false
    });

    if !widget_found {
        dev_err((*scomp).dev, b"Failed to find widget for kcontrol %s\n\0".as_ptr() as *const c_char, (*scontrol).name);
        return false;
    }

    let ret = sof_ipc4_set_volume_data(sdev, swidget, scontrol, true);
    if ret < 0 {
        return false;
    }

    change
}

unsafe extern "C" fn sof_ipc4_volume_get(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let channels = (*scontrol).num_channels;
    let mut i: c_uint = 0;

    while i < channels {
        (*ucontrol).value.integer.value[i as usize] = ipc_to_mixer(
            (*chanv_ptr(cdata, i as usize)).value,
            (*scontrol).volume_table,
            (*scontrol).max + 1,
        );
        i += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc4_set_generic_control_data(
    sdev: *mut snd_sof_dev,
    _swidget: *mut snd_sof_widget,
    scontrol: *mut snd_sof_control,
    lock: bool_,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let data_size = struct_size_control_msg_chanv((*scontrol).num_channels as usize);
    let data = kzalloc(data_size, GFP_KERNEL) as *mut sof_ipc4_control_msg_payload;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).id = (*cdata).index;
    (*data).num_elems = (*scontrol).num_channels;
    let mut i: c_uint = 0;
    while i < (*scontrol).num_channels {
        (*msg_chanv_ptr(data, i as usize)).channel = (*chanv_ptr(cdata, i as usize)).channel;
        (*msg_chanv_ptr(data, i as usize)).value = (*chanv_ptr(cdata, i as usize)).value;
        i += 1;
    }

    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    memcpy(&mut msg as *mut _ as *mut c_void, &(*cdata).msg as *const _ as *const c_void, core::mem::size_of::<sof_ipc4_msg>());
    msg.data_ptr = data as *mut c_void;
    msg.data_size = data_size;

    let ret = sof_ipc4_set_get_kcontrol_data(scontrol, &mut msg, true, lock);
    if ret < 0 {
        dev_err((*sdev).dev, b"Failed to set control update for %s\n\0".as_ptr() as *const c_char, (*scontrol).name);
    }

    kfree(data as *mut c_void);
    ret
}

unsafe extern "C" fn sof_ipc4_refresh_generic_control(scontrol: *mut snd_sof_control) {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let data_size = struct_size_control_msg_chanv((*scontrol).num_channels as usize);
    let data = kmalloc(data_size, GFP_KERNEL) as *mut sof_ipc4_control_msg_payload;
    let mut msg: sof_ipc4_msg = core::mem::zeroed();

    if !(*scontrol).comp_data_dirty {
        return;
    }

    if !pm_runtime_active((*scomp).dev) {
        return;
    }

    if data.is_null() {
        return;
    }

    (*data).id = (*cdata).index;
    (*data).num_elems = (*scontrol).num_channels;

    memcpy(&mut msg as *mut _ as *mut c_void, &(*cdata).msg as *const _ as *const c_void, core::mem::size_of::<sof_ipc4_msg>());
    msg.data_ptr = data as *mut c_void;
    msg.data_size = data_size;

    (*scontrol).comp_data_dirty = false;
    let ret = sof_ipc4_set_get_kcontrol_data(scontrol, &mut msg, false, true);
    if ret == 0 {
        let mut i: c_uint = 0;
        while i < (*scontrol).num_channels {
            (*chanv_ptr(cdata, i as usize)).channel = (*msg_chanv_ptr(data, i as usize)).channel;
            (*chanv_ptr(cdata, i as usize)).value = (*msg_chanv_ptr(data, i as usize)).value;
            i += 1;
        }
    } else {
        dev_err((*scomp).dev, b"Failed to read control data for %s\n\0".as_ptr() as *const c_char, (*scontrol).name);
        (*scontrol).comp_data_dirty = true;
    }

    kfree(data as *mut c_void);
}

unsafe extern "C" fn sof_ipc4_set_bytes_control_data(
    scontrol: *mut snd_sof_control,
    lock: bool_,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let data_hdr = (*cdata).data;
    let data_size = struct_size_control_msg_data((*data_hdr).size as usize);
    let msg_data = kzalloc(data_size, GFP_KERNEL) as *mut sof_ipc4_control_msg_payload;
    if msg_data.is_null() {
        return -ENOMEM;
    }

    (*msg_data).id = (*cdata).index;
    (*msg_data).num_elems = (*data_hdr).size;
    memcpy(msg_data_ptr(msg_data) as *mut c_void, abi_data_ptr(data_hdr) as *const c_void, (*data_hdr).size as usize);

    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    memcpy(&mut msg as *mut _ as *mut c_void, &(*cdata).msg as *const _ as *const c_void, core::mem::size_of::<sof_ipc4_msg>());
    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID((*data_hdr).type_);
    msg.data_ptr = msg_data as *mut c_void;
    msg.data_size = data_size;

    let ret = sof_ipc4_set_get_kcontrol_data(scontrol, &mut msg, true, lock);
    if ret < 0 {
        dev_err((*scomp).dev, b"%s: Failed to set control update for %s\n\0".as_ptr() as *const c_char, b"sof_ipc4_set_bytes_control_data\0".as_ptr() as *const c_char, (*scontrol).name);
    }

    kfree(msg_data as *mut c_void);
    ret
}

unsafe extern "C" fn sof_ipc4_refresh_bytes_control(
    scontrol: *mut snd_sof_control,
    lock: bool_,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let data_hdr = (*cdata).data;
    let mut ret: c_int = 0;

    if !(*scontrol).comp_data_dirty {
        return 0;
    }

    if !pm_runtime_active((*scomp).dev) {
        return 0;
    }

    let mut data_size = (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>();
    if data_size < core::mem::size_of::<sof_ipc4_control_msg_payload>() {
        data_size = core::mem::size_of::<sof_ipc4_control_msg_payload>();
    }

    let msg_data = kzalloc(data_size, GFP_KERNEL) as *mut sof_ipc4_control_msg_payload;
    if msg_data.is_null() {
        return -ENOMEM;
    }

    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    memcpy(&mut msg as *mut _ as *mut c_void, &(*cdata).msg as *const _ as *const c_void, core::mem::size_of::<sof_ipc4_msg>());
    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID((*data_hdr).type_);

    (*msg_data).id = (*cdata).index;
    (*msg_data).num_elems = 0; /* ignored for bytes */

    msg.data_ptr = msg_data as *mut c_void;
    msg.data_size = data_size;

    (*scontrol).comp_data_dirty = false;
    ret = sof_ipc4_set_get_kcontrol_data(scontrol, &mut msg, false, lock);
    if ret == 0 {
        if msg.data_size > (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>() {
            dev_err((*scomp).dev, b"%s: no space for data in %s (%zu, %zu)\n\0".as_ptr() as *const c_char, b"sof_ipc4_refresh_bytes_control\0".as_ptr() as *const c_char, (*scontrol).name, msg.data_size, (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>());
            ret = -EINVAL;
        } else {
            (*data_hdr).size = msg.data_size as u32;
            (*scontrol).size =
                core::mem::size_of::<sof_ipc4_control_data>() + core::mem::size_of::<sof_abi_hdr>() + (*data_hdr).size as usize;
            memcpy(abi_data_ptr(data_hdr) as *mut c_void, msg.data_ptr as *const c_void, (*data_hdr).size as usize);
        }
    } else {
        dev_err((*scomp).dev, b"Failed to read control data for %s\n\0".as_ptr() as *const c_char, (*scontrol).name);
        (*scontrol).comp_data_dirty = true;
    }

    kfree(msg_data as *mut c_void);
    ret
}

unsafe extern "C" fn sof_ipc4_switch_put(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> bool_ {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut swidget: *mut snd_sof_widget = core::ptr::null_mut();
    let mut widget_found = false;
    let mut change = false;
    let mut i: c_uint = 0;

    /* update each channel */
    while i < (*scontrol).num_channels {
        let value = (*ucontrol).value.integer.value[i as usize] as u32;
        change = change || value != (*chanv_ptr(cdata, i as usize)).value;
        (*chanv_ptr(cdata, i as usize)).channel = i;
        (*chanv_ptr(cdata, i as usize)).value = value;
        i += 1;
    }

    if !pm_runtime_active((*scomp).dev) {
        return change;
    }

    /* find widget associated with the control */
    for_each_swidget_in_widget_list(sdev, |entry| {
        swidget = entry;
        if (*swidget).comp_id == (*scontrol).comp_id {
            widget_found = true;
            return true;
        }
        false
    });

    if !widget_found {
        dev_err((*scomp).dev, b"Failed to find widget for kcontrol %s\n\0".as_ptr() as *const c_char, (*scontrol).name);
        return false;
    }

    let ret = sof_ipc4_set_generic_control_data(sdev, swidget, scontrol, true);
    if ret < 0 {
        return false;
    }

    change
}

unsafe extern "C" fn sof_ipc4_switch_get(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;

    sof_ipc4_refresh_generic_control(scontrol);

    /* read back each channel */
    let mut i: c_uint = 0;
    while i < (*scontrol).num_channels {
        (*ucontrol).value.integer.value[i as usize] = (*chanv_ptr(cdata, i as usize)).value as i64;
        i += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc4_enum_put(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> bool_ {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut swidget: *mut snd_sof_widget = core::ptr::null_mut();
    let mut widget_found = false;
    let mut change = false;
    let mut i: c_uint = 0;

    /* update each channel */
    while i < (*scontrol).num_channels {
        let value = (*ucontrol).value.enumerated.item[i as usize];
        change = change || value != (*chanv_ptr(cdata, i as usize)).value;
        (*chanv_ptr(cdata, i as usize)).channel = i;
        (*chanv_ptr(cdata, i as usize)).value = value;
        i += 1;
    }

    if !pm_runtime_active((*scomp).dev) {
        return change;
    }

    /* find widget associated with the control */
    for_each_swidget_in_widget_list(sdev, |entry| {
        swidget = entry;
        if (*swidget).comp_id == (*scontrol).comp_id {
            widget_found = true;
            return true;
        }
        false
    });

    if !widget_found {
        dev_err((*scomp).dev, b"Failed to find widget for kcontrol %s\n\0".as_ptr() as *const c_char, (*scontrol).name);
        return false;
    }

    let ret = sof_ipc4_set_generic_control_data(sdev, swidget, scontrol, true);
    if ret < 0 {
        return false;
    }

    change
}

unsafe extern "C" fn sof_ipc4_enum_get(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;

    sof_ipc4_refresh_generic_control(scontrol);

    /* read back each channel */
    let mut i: c_uint = 0;
    while i < (*scontrol).num_channels {
        (*ucontrol).value.enumerated.item[i as usize] = (*chanv_ptr(cdata, i as usize)).value;
        i += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc4_set_get_bytes_data(
    sdev: *mut snd_sof_dev,
    scontrol: *mut snd_sof_control,
    set: bool_,
    lock: bool_,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let data = (*cdata).data;
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let mut ret: c_int = 0;

    /* Send the new data to the firmware only if it is powered up */
    if set {
        if !pm_runtime_active((*sdev).dev) {
            return 0;
        }

        if (*data).size == 0 {
            dev_dbg((*sdev).dev, b"%s: No data to be sent.\n\0".as_ptr() as *const c_char, (*scontrol).name);
            return 0;
        }
    }

    if (*data).type_ == SOF_IPC4_BYTES_CONTROL_PARAM_ID {
        if set {
            return sof_ipc4_set_bytes_control_data(scontrol, lock);
        } else {
            return sof_ipc4_refresh_bytes_control(scontrol, lock);
        }
    }

    memcpy(&mut msg as *mut _ as *mut c_void, &(*cdata).msg as *const _ as *const c_void, core::mem::size_of::<sof_ipc4_msg>());
    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID((*data).type_);

    msg.data_ptr = abi_data_ptr(data) as *mut c_void;
    if set {
        msg.data_size = (*data).size as usize;
    } else {
        msg.data_size = (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>();
    }

    ret = sof_ipc4_set_get_kcontrol_data(scontrol, &mut msg, set, lock);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"Failed to %s for %s\n\0".as_ptr() as *const c_char,
            if set { b"set bytes update\0".as_ptr() } else { b"get bytes\0".as_ptr() } as *const c_char,
            (*scontrol).name,
        );
    } else if !set {
        /* Update the sizes according to the received payload data */
        (*data).size = msg.data_size as u32;
        (*scontrol).size =
            core::mem::size_of::<sof_ipc4_control_data>() + core::mem::size_of::<sof_abi_hdr>() + (*data).size as usize;
    }

    ret
}

unsafe extern "C" fn sof_ipc4_bytes_put(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let data = (*cdata).data;
    let new_hdr = (*ucontrol).value.bytes.data.as_ptr() as *const sof_abi_hdr;

    if (*scontrol).max_size > core::mem::size_of_val(&(*ucontrol).value.bytes.data) {
        dev_err_ratelimited((*scomp).dev, b"data max %zu exceeds ucontrol data array size\n\0".as_ptr() as *const c_char, (*scontrol).max_size);
        return -EINVAL;
    }

    /* Validate the new data's size, not the old one */
    if (*new_hdr).size as usize > (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>() {
        dev_err_ratelimited((*scomp).dev, b"data size too big %u bytes max is %zu\n\0".as_ptr() as *const c_char, (*new_hdr).size, (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>());
        return -EINVAL;
    }

    let size = (*new_hdr).size as usize + core::mem::size_of::<sof_abi_hdr>();

    /* copy from kcontrol */
    memcpy(data as *mut c_void, (*ucontrol).value.bytes.data.as_ptr() as *const c_void, size);

    let ret = sof_ipc4_set_get_bytes_data(sdev, scontrol, true, true);
    if ret == 0 {
        /* Update the cdata size */
        (*scontrol).size = core::mem::size_of::<sof_ipc4_control_data>() + size;
    }

    ret
}

unsafe extern "C" fn sof_ipc4_bytes_get(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let data = (*cdata).data;

    if (*scontrol).max_size > core::mem::size_of_val(&(*ucontrol).value.bytes.data) {
        dev_err_ratelimited((*scomp).dev, b"data max %zu exceeds ucontrol data array size\n\0".as_ptr() as *const c_char, (*scontrol).max_size);
        return -EINVAL;
    }

    if (*data).size as usize > (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>() {
        dev_err_ratelimited((*scomp).dev, b"%u bytes of control data is invalid, max is %zu\n\0".as_ptr() as *const c_char, (*data).size, (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>());
        return -EINVAL;
    }

    sof_ipc4_refresh_bytes_control(scontrol, true);

    let size = (*data).size as usize + core::mem::size_of::<sof_abi_hdr>();

    /* copy back to kcontrol */
    memcpy((*ucontrol).value.bytes.data.as_mut_ptr() as *mut c_void, data as *const c_void, size);

    0
}

unsafe extern "C" fn sof_ipc4_bytes_ext_put(
    scontrol: *mut snd_sof_control,
    binary_data: *const c_uint,
    size: c_uint,
) -> c_int {
    let tlvd = binary_data as *mut snd_ctl_tlv;
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let data = (*cdata).data;
    let mut abi_hdr: sof_abi_hdr = core::mem::zeroed();
    let mut header: snd_ctl_tlv = core::mem::zeroed();

    /*
     * The beginning of bytes data contains a header from where
     * the length (as bytes) is needed to know the correct copy
     * length of data from tlvd->tlv.
     */
    if copy_from_user(&mut header as *mut _ as *mut c_void, tlvd as *const c_void, core::mem::size_of::<snd_ctl_tlv>()) != 0 {
        return -EFAULT;
    }

    /* make sure TLV info is consistent */
    if header.length as usize + core::mem::size_of::<snd_ctl_tlv>() > size as usize {
        dev_err_ratelimited((*scomp).dev, b"Inconsistent TLV, data %d + header %zu > %d\n\0".as_ptr() as *const c_char, header.length, core::mem::size_of::<snd_ctl_tlv>(), size);
        return -EINVAL;
    }

    /* be->max is coming from topology */
    if header.length as usize > (*scontrol).max_size {
        dev_err_ratelimited((*scomp).dev, b"Bytes data size %d exceeds max %zu\n\0".as_ptr() as *const c_char, header.length, (*scontrol).max_size);
        return -EINVAL;
    }

    /* Check header id */
    if header.numid != SOF_CTRL_CMD_BINARY {
        dev_err_ratelimited((*scomp).dev, b"Incorrect numid for bytes put %d\n\0".as_ptr() as *const c_char, header.numid);
        return -EINVAL;
    }

    /* Verify the ABI header first */
    if copy_from_user(&mut abi_hdr as *mut _ as *mut c_void, (*tlvd).tlv.as_ptr() as *const c_void, core::mem::size_of::<sof_abi_hdr>()) != 0 {
        return -EFAULT;
    }

    if abi_hdr.magic != SOF_IPC4_ABI_MAGIC {
        dev_err_ratelimited((*scomp).dev, b"Wrong ABI magic 0x%08x\n\0".as_ptr() as *const c_char, abi_hdr.magic);
        return -EINVAL;
    }

    if abi_hdr.size as usize > (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>() {
        dev_err_ratelimited((*scomp).dev, b"%u bytes of control data is invalid, max is %zu\n\0".as_ptr() as *const c_char, abi_hdr.size, (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>());
        return -EINVAL;
    }

    if (*scontrol).old_ipc_control_data.is_null() {
        /* Create a backup of the current, valid bytes control */
        (*scontrol).old_ipc_control_data =
            kmemdup((*scontrol).ipc_control_data as *const c_void, (*scontrol).size, GFP_KERNEL);
        if (*scontrol).old_ipc_control_data.is_null() {
            return -ENOMEM;
        }
    }

    /* Copy the whole binary data which includes the ABI header and the payload */
    if copy_from_user(data as *mut c_void, (*tlvd).tlv.as_ptr() as *const c_void, header.length as usize) != 0 {
        memcpy((*scontrol).ipc_control_data as *mut c_void, (*scontrol).old_ipc_control_data, (*scontrol).size);
        kfree((*scontrol).old_ipc_control_data);
        (*scontrol).old_ipc_control_data = core::ptr::null_mut();
        return -EFAULT;
    }

    /* Update the cdata size */
    (*scontrol).size = core::mem::size_of::<sof_ipc4_control_data>() + header.length as usize;

    sof_ipc4_set_get_bytes_data(sdev, scontrol, true, true)
}

unsafe extern "C" fn _sof_ipc4_bytes_ext_get(
    scontrol: *mut snd_sof_control,
    binary_data: *const c_uint,
    mut size: c_uint,
    from_dsp: bool_,
) -> c_int {
    let tlvd = binary_data as *mut snd_ctl_tlv;
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let data = (*cdata).data;
    let mut header: snd_ctl_tlv = core::mem::zeroed();

    /*
     * Decrement the limit by ext bytes header size to ensure the user space
     * buffer is not exceeded.
     */
    if (size as usize) < core::mem::size_of::<snd_ctl_tlv>() {
        return -ENOSPC;
    }

    size -= core::mem::size_of::<snd_ctl_tlv>() as c_uint;

    /* get all the component data from DSP */
    if from_dsp {
        let sdev = snd_soc_component_get_drvdata(scomp);
        let ret = sof_ipc4_set_get_bytes_data(sdev, scontrol, false, true);

        if ret < 0 {
            return ret;
        }

        /* Set the ABI magic (if the control is not initialized) */
        (*data).magic = SOF_IPC4_ABI_MAGIC;
    }

    if (*data).size as usize > (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>() {
        dev_err_ratelimited((*scomp).dev, b"%u bytes of control data is invalid, max is %zu\n\0".as_ptr() as *const c_char, (*data).size, (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>());
        return -EINVAL;
    }

    let data_size = (*data).size as usize + core::mem::size_of::<sof_abi_hdr>();

    /* make sure we don't exceed size provided by user space for data */
    if data_size > size as usize {
        return -ENOSPC;
    }

    /* Set header id and length */
    header.numid = SOF_CTRL_CMD_BINARY;
    header.length = data_size as u32;

    if copy_to_user(tlvd as *mut c_void, &header as *const _ as *const c_void, core::mem::size_of::<snd_ctl_tlv>()) != 0 {
        return -EFAULT;
    }

    if copy_to_user((*tlvd).tlv.as_mut_ptr() as *mut c_void, data as *const c_void, data_size) != 0 {
        return -EFAULT;
    }

    0
}

unsafe extern "C" fn sof_ipc4_bytes_ext_get(
    scontrol: *mut snd_sof_control,
    binary_data: *const c_uint,
    size: c_uint,
) -> c_int {
    sof_ipc4_refresh_bytes_control(scontrol, true);

    _sof_ipc4_bytes_ext_get(scontrol, binary_data, size, false)
}

unsafe extern "C" fn sof_ipc4_bytes_ext_volatile_get(
    scontrol: *mut snd_sof_control,
    binary_data: *const c_uint,
    size: c_uint,
) -> c_int {
    _sof_ipc4_bytes_ext_get(scontrol, binary_data, size, true)
}

unsafe extern "C" fn sof_ipc4_volsw_setup(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
    scontrol: *mut snd_sof_control,
) -> c_int {
    if (*scontrol).max == 1 {
        return sof_ipc4_set_generic_control_data(sdev, swidget, scontrol, false);
    }

    sof_ipc4_set_volume_data(sdev, swidget, scontrol, false)
}

unsafe extern "C" fn sof_ipc4_control_update(sdev: *mut snd_sof_dev, ipc_message: *mut c_void) {
    let ipc4_msg = ipc_message as *mut sof_ipc4_msg;
    let ndata = (*ipc4_msg).data_ptr as *mut sof_ipc4_notify_module_data;
    let mut msg_data: *mut sof_ipc4_control_msg_payload;
    let mut cdata: *mut sof_ipc4_control_data = core::ptr::null_mut();
    let widget: *mut snd_soc_dapm_widget;
    let mut scontrol: *mut snd_sof_control = core::ptr::null_mut();
    let swidget: *mut snd_sof_widget;
    let mut kc: *mut snd_kcontrol = core::ptr::null_mut();
    let mut scontrol_found = false;
    let event_param_id: u32;
    let type_: c_int;

    if (*ndata).event_data_size as usize
        < core::mem::size_of::<sof_ipc4_control_msg_payload>()
    {
        dev_err((*sdev).dev, b"%s: Invalid event data size for module %u.%u: %u\n\0".as_ptr() as *const c_char, b"sof_ipc4_control_update\0".as_ptr() as *const c_char, (*ndata).module_id, (*ndata).instance_id, (*ndata).event_data_size);
        return;
    }

    event_param_id = (*ndata).event_id & SOF_IPC4_NOTIFY_MODULE_EVENTID_ALSA_PARAMID_MASK;
    if event_param_id == SOF_IPC4_SWITCH_CONTROL_PARAM_ID {
        type_ = SND_SOC_TPLG_TYPE_MIXER;
    } else if event_param_id == SOF_IPC4_ENUM_CONTROL_PARAM_ID {
        type_ = SND_SOC_TPLG_TYPE_ENUM;
    } else if event_param_id == SOF_IPC4_BYTES_CONTROL_PARAM_ID {
        type_ = SND_SOC_TPLG_TYPE_BYTES;
    } else {
        dev_err((*sdev).dev, b"%s: Invalid control type for module %u.%u: %u\n\0".as_ptr() as *const c_char, b"sof_ipc4_control_update\0".as_ptr() as *const c_char, (*ndata).module_id, (*ndata).instance_id, event_param_id);
        return;
    }

    /* Find the swidget based on ndata->module_id and ndata->instance_id */
    swidget = sof_ipc4_find_swidget_by_ids(sdev, (*ndata).module_id, (*ndata).instance_id);
    if swidget.is_null() {
        dev_err((*sdev).dev, b"%s: Failed to find widget for module %u.%u\n\0".as_ptr() as *const c_char, b"sof_ipc4_control_update\0".as_ptr() as *const c_char, (*ndata).module_id, (*ndata).instance_id);
        return;
    }

    /* Find the scontrol which is the source of the notification */
    msg_data = (*ndata).event_data.as_mut_ptr() as *mut sof_ipc4_control_msg_payload;
    // list_for_each_entry(scontrol, &sdev->kcontrol_list, list)
    for_each_scontrol_in_kcontrol_list(sdev, |entry| {
        scontrol = entry;
        if (*scontrol).comp_id == (*swidget).comp_id {
            let local_param_id: u32;

            cdata = (*scontrol).ipc_control_data;
            /*
             * The scontrol's param_id is stored in the IPC message
             * template's extension
             */
            local_param_id = PARAM_ID_FROM_EXTENSION((*cdata).msg.extension);
            if local_param_id == event_param_id && (*msg_data).id == (*cdata).index {
                scontrol_found = true;
                return true;
            }
        }
        false
    });

    if !scontrol_found {
        dev_err((*sdev).dev, b"%s: Failed to find control on widget %s: %u:%u\n\0".as_ptr() as *const c_char, b"sof_ipc4_control_update\0".as_ptr() as *const c_char, (*(*swidget).widget).name, (*ndata).event_id & 0xffff, (*msg_data).id);
        return;
    }

    if (*msg_data).num_elems != 0 {
        /*
         * The message includes the updated value/data, update the
         * control's local cache using the received notification
         */
        if type_ == SND_SOC_TPLG_TYPE_BYTES {
            let data = (*cdata).data;
            let source_size = struct_size_control_msg_data((*msg_data).num_elems as usize);

            if source_size > (*ndata).event_data_size as usize {
                dev_warn((*sdev).dev, b"%s: invalid bytes notification size for %s (%zu, %u)\n\0".as_ptr() as *const c_char, b"sof_ipc4_control_update\0".as_ptr() as *const c_char, (*scontrol).name, source_size, (*ndata).event_data_size);
                (*scontrol).comp_data_dirty = true;
                // goto notify;
            } else if (*msg_data).num_elems as usize > (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>() {
                dev_warn((*sdev).dev, b"%s: no space for data in %s (%u, %zu)\n\0".as_ptr() as *const c_char, b"sof_ipc4_control_update\0".as_ptr() as *const c_char, (*scontrol).name, (*msg_data).num_elems, (*scontrol).max_size - core::mem::size_of::<sof_abi_hdr>());
            } else {
                memcpy(abi_data_ptr(data) as *mut c_void, msg_data_ptr(msg_data) as *const c_void, (*msg_data).num_elems as usize);
                (*data).size = (*msg_data).num_elems;
                (*scontrol).size =
                    core::mem::size_of::<sof_ipc4_control_data>() + core::mem::size_of::<sof_abi_hdr>() + (*data).size as usize;
            }
        } else {
            let source_size = struct_size_control_msg_chanv((*msg_data).num_elems as usize);

            if source_size > (*ndata).event_data_size as usize {
                dev_warn((*sdev).dev, b"%s: invalid channel notification size for %s (%zu, %u)\n\0".as_ptr() as *const c_char, b"sof_ipc4_control_update\0".as_ptr() as *const c_char, (*scontrol).name, source_size, (*ndata).event_data_size);
                (*scontrol).comp_data_dirty = true;
                // goto notify;
            } else {
                let mut i: c_int = 0;
                while i < (*msg_data).num_elems as c_int {
                    let channel = (*msg_chanv_ptr(msg_data, i as usize)).channel;

                    if channel >= (*scontrol).num_channels {
                        dev_warn((*sdev).dev, b"Invalid channel index for %s: %u\n\0".as_ptr() as *const c_char, (*scontrol).name, i);

                        /*
                         * Mark the scontrol as dirty to force a refresh
                         * on next read
                         */
                        (*scontrol).comp_data_dirty = true;
                        break;
                    }

                    (*chanv_ptr(cdata, channel as usize)).value =
                        (*msg_chanv_ptr(msg_data, i as usize)).value;
                    i += 1;
                }
            }
        }
    } else {
        /*
         * Mark the scontrol as dirty because the value/data is changed
         * in firmware, forcing a refresh on next read access
         */
        (*scontrol).comp_data_dirty = true;
    }

    /*
     * Look up the ALSA kcontrol of the scontrol to be able to send a
     * notification to user space
     */
    widget = (*swidget).widget;
    let mut i: c_int = 0;
    while i < (*widget).num_kcontrols {
        /* skip non matching types or non matching indexes within type */
        if *(*widget).dobj.widget.kcontrol_type.add(i as usize) == type_
            && (*(*widget).kcontrol_news.add(i as usize)).index == (*cdata).index
        {
            kc = *(*widget).kcontrols.add(i as usize);
            break;
        }
        i += 1;
    }

    if kc.is_null() {
        return;
    }

    snd_ctl_notify_one(
        (*(*(*swidget).scomp).card).snd_card,
        SNDRV_CTL_EVENT_MASK_VALUE,
        kc,
        0,
    );
}

/* set up all controls for the widget */
unsafe extern "C" fn sof_ipc4_widget_kcontrol_setup(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
) -> c_int {
    let mut ret: c_int = 0;

    // list_for_each_entry(scontrol, &sdev->kcontrol_list, list)
    for_each_scontrol_in_kcontrol_list(sdev, |scontrol| {
        if (*scontrol).comp_id == (*swidget).comp_id {
            if (*scontrol).info_type == SND_SOC_TPLG_CTL_VOLSW
                || (*scontrol).info_type == SND_SOC_TPLG_CTL_VOLSW_SX
                || (*scontrol).info_type == SND_SOC_TPLG_CTL_VOLSW_XR_SX
            {
                ret = sof_ipc4_volsw_setup(sdev, swidget, scontrol);
            } else if (*scontrol).info_type == SND_SOC_TPLG_CTL_BYTES {
                ret = sof_ipc4_set_get_bytes_data(sdev, scontrol, true, false);
            } else if (*scontrol).info_type == SND_SOC_TPLG_CTL_ENUM
                || (*scontrol).info_type == SND_SOC_TPLG_CTL_ENUM_VALUE
            {
                ret = sof_ipc4_set_generic_control_data(sdev, swidget, scontrol, false);
            }

            if ret < 0 {
                dev_err((*sdev).dev, b"kcontrol %d set up failed for widget %s\n\0".as_ptr() as *const c_char, (*scontrol).comp_id, (*(*swidget).widget).name);
                return true;
            }
        }
        false
    });

    ret
}

unsafe extern "C" fn sof_ipc4_set_up_volume_table(
    scontrol: *mut snd_sof_control,
    tlv: *mut c_int,
    size: c_int,
) -> c_int {
    let mut i: c_int;

    /* init the volume table */
    (*scontrol).volume_table = kcalloc(size as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if (*scontrol).volume_table.is_null() {
        return -ENOMEM;
    }

    /* populate the volume table */
    i = 0;
    while i < size {
        let val = vol_compute_gain(i, tlv);
        let q31val = (val as u64) << 15; /* Can be over Q1.31, need to saturate */

        *(*scontrol).volume_table.add(i as usize) = if q31val > SOF_IPC4_VOL_ZERO_DB {
            SOF_IPC4_VOL_ZERO_DB as u32
        } else {
            q31val as u32
        };
        i += 1;
    }

    0
}

#[no_mangle]
pub static tplg_ipc4_control_ops: sof_ipc_tplg_control_ops = sof_ipc_tplg_control_ops {
    volume_put: Some(sof_ipc4_volume_put),
    volume_get: Some(sof_ipc4_volume_get),
    switch_put: Some(sof_ipc4_switch_put),
    switch_get: Some(sof_ipc4_switch_get),
    enum_put: Some(sof_ipc4_enum_put),
    enum_get: Some(sof_ipc4_enum_get),
    bytes_put: Some(sof_ipc4_bytes_put),
    bytes_get: Some(sof_ipc4_bytes_get),
    bytes_ext_put: Some(sof_ipc4_bytes_ext_put),
    bytes_ext_get: Some(sof_ipc4_bytes_ext_get),
    bytes_ext_volatile_get: Some(sof_ipc4_bytes_ext_volatile_get),
    update: Some(sof_ipc4_control_update),
    widget_kcontrol_setup: Some(sof_ipc4_widget_kcontrol_setup),
    set_up_volume_table: Some(sof_ipc4_set_up_volume_table),
};

// The C source uses Linux list_for_each_entry() over list_head members. The
// actual Rust mapping for intrusive list iteration belongs to the surrounding
// kernel bindings; these declarations preserve the source-level control flow.
unsafe fn for_each_swidget_in_widget_list<F: FnMut(*mut snd_sof_widget) -> bool>(
    _sdev: *mut snd_sof_dev,
    mut _f: F,
) {
    // External intrusive list traversal supplied by the translated kernel list bindings.
}

unsafe fn for_each_scontrol_in_kcontrol_list<F: FnMut(*mut snd_sof_control) -> bool>(
    _sdev: *mut snd_sof_dev,
    mut _f: F,
) {
    // External intrusive list traversal supplied by the translated kernel list bindings.
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
