// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Intel Corporation
//
//

// Rust translation of soc/sof/ipc3-control.c.
// C includes removed: "sof-priv.h", "sof-audio.h", "ipc3-priv.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type size_t = usize;

const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const ENOSPC: c_int = 28;
const GFP_KERNEL: c_uint = 0;

extern "C" {
    static SOF_ABI_MAGIC: u32;
    static SOF_ABI_VERSION: u32;
    static SOF_TLV_ITEMS: usize;
    static SOF_CTRL_CMD_BINARY: u32;
    static SOF_CTRL_CMD_VOLUME: u32;
    static SOF_CTRL_CMD_SWITCH: u32;
    static SOF_CTRL_CMD_ENUM: u32;
    static SOF_IPC_COMP_SET_DATA: u32;
    static SOF_IPC_COMP_GET_DATA: u32;
    static SOF_IPC_COMP_SET_VALUE: u32;
    static SOF_IPC_COMP_GET_VALUE: u32;
    static SOF_IPC_GLB_COMP_MSG: u32;
    static SOF_CTRL_TYPE_DATA_SET: sof_ipc_ctrl_type;
    static SOF_CTRL_TYPE_DATA_GET: sof_ipc_ctrl_type;
    static SOF_CTRL_TYPE_VALUE_CHAN_SET: sof_ipc_ctrl_type;
    static SOF_CTRL_TYPE_VALUE_CHAN_GET: sof_ipc_ctrl_type;
    static SOF_CTRL_TYPE_VALUE_COMP_GET: sof_ipc_ctrl_type;
    static SOF_CTRL_TYPE_VALUE_COMP_SET: sof_ipc_ctrl_type;
    static SND_SOC_TPLG_TYPE_MIXER: c_int;
    static SND_SOC_TPLG_TYPE_BYTES: c_int;
    static SND_SOC_TPLG_TYPE_ENUM: c_int;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
    pub set_get_data: Option<
        unsafe extern "C" fn(
            *mut snd_sof_dev,
            *mut sof_ipc_ctrl_data,
            u32,
            bool_,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_sof_control {
    pub scomp: *mut snd_soc_component,
    pub ipc_control_data: *mut sof_ipc_ctrl_data,
    pub old_ipc_control_data: *mut c_void,
    pub comp_id: c_int,
    pub num_channels: c_uint,
    pub max: c_uint,
    pub max_size: size_t,
    pub volume_table: *mut u32,
    pub comp_data_dirty: bool_,
    pub name: *const c_char,
    pub list: list_head,
}

#[repr(C)]
pub struct snd_sof_widget {
    pub comp_id: c_int,
    pub setup_mutex: mutex,
    pub use_count: c_int,
    pub widget: *mut snd_soc_dapm_widget,
    pub dynamic_pipeline_widget: bool_,
    pub scomp: *mut snd_soc_component,
    pub list: list_head,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub num_kcontrols: c_int,
    pub dobj: snd_soc_tplg_dobj,
    pub kcontrol_news: *mut snd_kcontrol_new,
    pub kcontrols: *mut *mut snd_kcontrol,
}

#[repr(C)]
pub struct snd_soc_tplg_dobj {
    pub widget: snd_soc_tplg_widget,
    pub private: *mut snd_sof_control,
}

#[repr(C)]
pub struct snd_soc_tplg_widget {
    pub kcontrol_type: *mut c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub dobj: snd_soc_tplg_dobj,
}

#[repr(C)]
pub struct soc_bytes_ext {
    pub dobj: snd_soc_tplg_dobj,
}

#[repr(C)]
pub struct soc_enum {
    pub dobj: snd_soc_tplg_dobj,
}

#[repr(C)]
pub struct snd_ctl_tlv {
    pub numid: c_uint,
    pub length: c_uint,
    pub tlv: [c_uint; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub bytes: snd_ctl_elem_value_bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct sof_ipc_ctrl_type(pub u32);

#[repr(C)]
pub struct sof_ipc_ctrl_value_chan {
    pub channel: u32,
    pub value: u32,
}

#[repr(C)]
pub struct sof_ipc_hdr {
    pub size: u32,
    pub cmd: u32,
}

#[repr(C)]
pub struct sof_ipc_reply_hdr {
    pub hdr: sof_ipc_hdr,
}

#[repr(C)]
pub struct sof_abi_hdr {
    pub magic: u32,
    pub type_: u32,
    pub size: u32,
    pub abi: u32,
}

#[repr(C)]
pub struct sof_ipc_ctrl_data {
    pub rhdr: sof_ipc_reply_hdr,
    pub type_: sof_ipc_ctrl_type,
    pub cmd: u32,
    pub comp_id: c_int,
    pub msg_index: u32,
    pub num_elems: u32,
    pub elems_remaining: u32,
    pub index: u32,
    pub chanv: [sof_ipc_ctrl_value_chan; 0],
    pub data: *mut sof_abi_hdr,
}

#[repr(C)]
pub struct sof_ipc_tplg_control_ops {
    pub volume_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool_>,
    pub volume_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub switch_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool_>,
    pub switch_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub enum_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool_>,
    pub enum_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_ext_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *const c_uint, c_uint) -> c_int>,
    pub bytes_ext_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *const c_uint, c_uint) -> c_int>,
    pub bytes_ext_volatile_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *const c_uint, c_uint) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void)>,
    pub widget_kcontrol_setup:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> c_int>,
    pub set_up_volume_table:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut c_int, c_int) -> c_int>,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn lockdep_assert_held(lock: *mut mutex);
    fn pm_runtime_active(dev: *mut device) -> bool_;
    fn ipc_to_mixer(value: u32, table: *mut u32, size: u32) -> i64;
    fn mixer_to_ipc(value: i64, table: *mut u32, size: u32) -> u32;
    fn vol_compute_gain(i: c_int, tlv: *mut c_int) -> u32;
    fn kcalloc(n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_int;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, n: size_t) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn snd_ctl_notify_one(
        card: *mut c_void,
        mask: c_uint,
        kcontrol: *mut snd_kcontrol,
        index: c_uint,
    );
    fn SOF_ABI_VERSION_INCOMPATIBLE(current: u32, found: u32) -> bool_;
    fn check_mul_overflow(a: size_t, b: size_t, d: *mut size_t) -> bool_;
    fn check_add_overflow(a: size_t, b: size_t, d: *mut size_t) -> bool_;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// External kernel list iteration primitive. This macro is intentionally left as
// a dependency supplied by the translated build environment.
macro_rules! list_for_each_entry {
    ($pos:ident, $head:expr, $member:ident, $body:block) => {
        compile_error!("list_for_each_entry! must be provided by the kernel Rust binding layer");
    };
}

/* IPC set()/get() for kcontrols. */
unsafe extern "C" fn sof_ipc3_set_get_kcontrol_data(
    scontrol: *mut snd_sof_control,
    set: bool_,
    lock: bool_,
) -> c_int {
    let sdev = snd_soc_component_get_drvdata((*scontrol).scomp);
    let cdata = (*scontrol).ipc_control_data;
    let iops = (*(*sdev).ipc).ops;
    let ctrl_type: sof_ipc_ctrl_type;
    let mut swidget: *mut snd_sof_widget = ptr::null_mut();
    let mut widget_found = false;
    let ipc_cmd: u32;
    let msg_bytes: u32;
    let mut ret: c_int = 0;

    list_for_each_entry!(swidget, &mut (*sdev).widget_list, list, {
        if (*swidget).comp_id == (*scontrol).comp_id {
            widget_found = true;
            break;
        }
    });

    if !widget_found {
        dev_err(
            (*sdev).dev,
            c_str!("%s: can't find widget with id %d\n"),
            c_str!("sof_ipc3_set_get_kcontrol_data"),
            (*scontrol).comp_id,
        );
        return -EINVAL;
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
        goto_unlock(lock, swidget);
        return ret;
    }

    /*
     * Select the IPC cmd and the ctrl_type based on the ctrl_cmd and the
     * direction
     * Note: SOF_CTRL_TYPE_VALUE_COMP_* is not used and supported currently
     *	 for ctrl_type
     */
    if (*cdata).cmd == SOF_CTRL_CMD_BINARY {
        ipc_cmd = if set { SOF_IPC_COMP_SET_DATA } else { SOF_IPC_COMP_GET_DATA };
        ctrl_type = if set { SOF_CTRL_TYPE_DATA_SET } else { SOF_CTRL_TYPE_DATA_GET };
    } else {
        ipc_cmd = if set { SOF_IPC_COMP_SET_VALUE } else { SOF_IPC_COMP_GET_VALUE };
        ctrl_type = if set {
            SOF_CTRL_TYPE_VALUE_CHAN_SET
        } else {
            SOF_CTRL_TYPE_VALUE_CHAN_GET
        };
    }

    (*cdata).rhdr.hdr.cmd = SOF_IPC_GLB_COMP_MSG | ipc_cmd;
    (*cdata).type_ = ctrl_type;
    (*cdata).comp_id = (*scontrol).comp_id;
    (*cdata).msg_index = 0;

    /* calculate header and data size */
    if (*cdata).type_ == SOF_CTRL_TYPE_VALUE_CHAN_GET
        || (*cdata).type_ == SOF_CTRL_TYPE_VALUE_CHAN_SET
    {
        (*cdata).num_elems = (*scontrol).num_channels;

        msg_bytes = (*scontrol).num_channels
            .wrapping_mul(size_of::<sof_ipc_ctrl_value_chan>() as u32)
            .wrapping_add(size_of::<sof_ipc_ctrl_data>() as u32);
    } else if (*cdata).type_ == SOF_CTRL_TYPE_DATA_GET || (*cdata).type_ == SOF_CTRL_TYPE_DATA_SET {
        (*cdata).num_elems = (*(*cdata).data).size;

        msg_bytes = (*(*cdata).data)
            .size
            .wrapping_add(size_of::<sof_ipc_ctrl_data>() as u32)
            .wrapping_add(size_of::<sof_abi_hdr>() as u32);
    } else {
        ret = -EINVAL;
        goto_unlock(lock, swidget);
        return ret;
    }

    (*cdata).rhdr.hdr.size = msg_bytes;
    (*cdata).elems_remaining = 0;

    ret = ((*iops).set_get_data.unwrap())(sdev, cdata, (*cdata).rhdr.hdr.size, set);
    if !set {
        goto_unlock(lock, swidget);
        return ret;
    }

    /* It is a set-data operation, and we have a backup that we can restore */
    if ret < 0 {
        if (*scontrol).old_ipc_control_data.is_null() {
            goto_unlock(lock, swidget);
            return ret;
        }
        /*
         * Current ipc_control_data is not valid, we use the last known good
         * configuration
         */
        memcpy(
            (*scontrol).ipc_control_data as *mut c_void,
            (*scontrol).old_ipc_control_data as *const c_void,
            (*scontrol).max_size,
        );
        kfree((*scontrol).old_ipc_control_data);
        (*scontrol).old_ipc_control_data = ptr::null_mut();
        /* Send the last known good configuration to firmware */
        ret = ((*iops).set_get_data.unwrap())(sdev, cdata, (*cdata).rhdr.hdr.size, set);
        if ret < 0 {
            goto_unlock(lock, swidget);
            return ret;
        }
    }

    goto_unlock(lock, swidget);
    ret
}

unsafe fn goto_unlock(lock: bool_, swidget: *mut snd_sof_widget) {
    if lock {
        mutex_unlock(&mut (*swidget).setup_mutex);
    }
}

unsafe extern "C" fn sof_ipc3_refresh_control(scontrol: *mut snd_sof_control) {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let ret: c_int;

    if !(*scontrol).comp_data_dirty {
        return;
    }

    if !pm_runtime_active((*scomp).dev) {
        return;
    }

    /* set the ABI header values */
    (*(*cdata).data).magic = SOF_ABI_MAGIC;
    (*(*cdata).data).abi = SOF_ABI_VERSION;

    /* refresh the component data from DSP */
    (*scontrol).comp_data_dirty = false;
    ret = sof_ipc3_set_get_kcontrol_data(scontrol, false, true);
    if ret < 0 {
        dev_err((*scomp).dev, c_str!("Failed to get control data: %d\n"), ret);

        /* Set the flag to re-try next time to get the data */
        (*scontrol).comp_data_dirty = true;
    }
}

unsafe extern "C" fn sof_ipc3_volume_get(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let channels = (*scontrol).num_channels;
    let mut i: c_uint;

    sof_ipc3_refresh_control(scontrol);

    /* read back each channel */
    i = 0;
    while i < channels {
        (*ucontrol).value.integer.value[i as usize] = ipc_to_mixer(
            (*(*cdata).chanv.as_ptr().add(i as usize)).value,
            (*scontrol).volume_table,
            (*scontrol).max + 1,
        );
        i += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc3_volume_put(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> bool_ {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let channels = (*scontrol).num_channels;
    let mut i: c_uint;
    let mut change = false;

    /* update each channel */
    i = 0;
    while i < channels {
        let value = mixer_to_ipc(
            (*ucontrol).value.integer.value[i as usize],
            (*scontrol).volume_table,
            (*scontrol).max + 1,
        );

        change = change || value != (*(*cdata).chanv.as_ptr().add(i as usize)).value;
        (*(*cdata).chanv.as_mut_ptr().add(i as usize)).channel = i;
        (*(*cdata).chanv.as_mut_ptr().add(i as usize)).value = value;
        i += 1;
    }

    /* notify DSP of mixer updates */
    if pm_runtime_active((*scomp).dev) {
        let ret = sof_ipc3_set_get_kcontrol_data(scontrol, true, true);

        if ret < 0 {
            dev_err(
                (*scomp).dev,
                c_str!("Failed to set mixer updates for %s\n"),
                (*scontrol).name,
            );
            return false;
        }
    }

    change
}

unsafe extern "C" fn sof_ipc3_switch_get(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let channels = (*scontrol).num_channels;
    let mut i: c_uint;

    sof_ipc3_refresh_control(scontrol);

    /* read back each channel */
    i = 0;
    while i < channels {
        (*ucontrol).value.integer.value[i as usize] =
            (*(*cdata).chanv.as_ptr().add(i as usize)).value as i64;
        i += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc3_switch_put(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> bool_ {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let channels = (*scontrol).num_channels;
    let mut i: c_uint;
    let mut change = false;
    let mut value: u32;

    /* update each channel */
    i = 0;
    while i < channels {
        value = (*ucontrol).value.integer.value[i as usize] as u32;
        change = change || value != (*(*cdata).chanv.as_ptr().add(i as usize)).value;
        (*(*cdata).chanv.as_mut_ptr().add(i as usize)).channel = i;
        (*(*cdata).chanv.as_mut_ptr().add(i as usize)).value = value;
        i += 1;
    }

    /* notify DSP of mixer updates */
    if pm_runtime_active((*scomp).dev) {
        let ret = sof_ipc3_set_get_kcontrol_data(scontrol, true, true);

        if ret < 0 {
            dev_err(
                (*scomp).dev,
                c_str!("Failed to set mixer updates for %s\n"),
                (*scontrol).name,
            );
            return false;
        }
    }

    change
}

unsafe extern "C" fn sof_ipc3_enum_get(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let channels = (*scontrol).num_channels;
    let mut i: c_uint;

    sof_ipc3_refresh_control(scontrol);

    /* read back each channel */
    i = 0;
    while i < channels {
        (*ucontrol).value.enumerated.item[i as usize] =
            (*(*cdata).chanv.as_ptr().add(i as usize)).value;
        i += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc3_enum_put(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> bool_ {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let channels = (*scontrol).num_channels;
    let mut i: c_uint;
    let mut change = false;
    let mut value: u32;

    /* update each channel */
    i = 0;
    while i < channels {
        value = (*ucontrol).value.enumerated.item[i as usize];
        change = change || value != (*(*cdata).chanv.as_ptr().add(i as usize)).value;
        (*(*cdata).chanv.as_mut_ptr().add(i as usize)).channel = i;
        (*(*cdata).chanv.as_mut_ptr().add(i as usize)).value = value;
        i += 1;
    }

    /* notify DSP of enum updates */
    if pm_runtime_active((*scomp).dev) {
        let ret = sof_ipc3_set_get_kcontrol_data(scontrol, true, true);

        if ret < 0 {
            dev_err(
                (*scomp).dev,
                c_str!("Failed to set enum updates for %s\n"),
                (*scontrol).name,
            );
            return false;
        }
    }

    change
}

unsafe extern "C" fn sof_ipc3_bytes_get(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let data = (*cdata).data;
    let size: size_t;

    sof_ipc3_refresh_control(scontrol);

    if (*scontrol).max_size > size_of::<snd_ctl_elem_value_bytes>() {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("data max %zu exceeds ucontrol data array size\n"),
            (*scontrol).max_size,
        );
        return -EINVAL;
    }

    /* be->max has been verified to be >= sizeof(struct sof_abi_hdr) */
    if (*data).size as usize
        > (*scontrol)
            .max_size
            .wrapping_sub(size_of::<sof_ipc_ctrl_data>())
            .wrapping_sub(size_of::<sof_abi_hdr>())
    {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("%u bytes of control data is invalid, max is %zu\n"),
            (*data).size,
            (*scontrol)
                .max_size
                .wrapping_sub(size_of::<sof_ipc_ctrl_data>())
                .wrapping_sub(size_of::<sof_abi_hdr>()),
        );
        return -EINVAL;
    }

    size = (*data).size as usize + size_of::<sof_abi_hdr>();

    /* copy back to kcontrol */
    memcpy(
        (*ucontrol).value.bytes.data.as_mut_ptr() as *mut c_void,
        data as *const c_void,
        size,
    );

    0
}

unsafe extern "C" fn sof_ipc3_bytes_put(
    scontrol: *mut snd_sof_control,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let data = (*cdata).data;
    let new_hdr = (*ucontrol).value.bytes.data.as_ptr() as *const sof_abi_hdr;
    let size: size_t;

    if (*scontrol).max_size > size_of::<snd_ctl_elem_value_bytes>() {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("data max %zu exceeds ucontrol data array size\n"),
            (*scontrol).max_size,
        );
        return -EINVAL;
    }

    /* Validate the new data's size, not the old one */
    if (*new_hdr).size as usize
        > (*scontrol)
            .max_size
            .wrapping_sub(size_of::<sof_ipc_ctrl_data>())
            .wrapping_sub(size_of::<sof_abi_hdr>())
    {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("data size too big %u bytes max is %zu\n"),
            (*new_hdr).size,
            (*scontrol)
                .max_size
                .wrapping_sub(size_of::<sof_ipc_ctrl_data>())
                .wrapping_sub(size_of::<sof_abi_hdr>()),
        );
        return -EINVAL;
    }

    size = (*new_hdr).size as usize + size_of::<sof_abi_hdr>();

    /* copy from kcontrol */
    memcpy(
        data as *mut c_void,
        (*ucontrol).value.bytes.data.as_ptr() as *const c_void,
        size,
    );

    /* notify DSP of byte control updates */
    if pm_runtime_active((*scomp).dev) {
        return sof_ipc3_set_get_kcontrol_data(scontrol, true, true);
    }

    0
}

unsafe extern "C" fn sof_ipc3_bytes_ext_put(
    scontrol: *mut snd_sof_control,
    binary_data: *const c_uint,
    size: c_uint,
) -> c_int {
    let tlvd = binary_data as *const snd_ctl_tlv;
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let mut header: snd_ctl_tlv = core::mem::zeroed();
    let mut ret = -EINVAL;

    /*
     * The beginning of bytes data contains a header from where
     * the length (as bytes) is needed to know the correct copy
     * length of data from tlvd->tlv.
     */
    if copy_from_user(
        &mut header as *mut snd_ctl_tlv as *mut c_void,
        tlvd as *const c_void,
        size_of::<snd_ctl_tlv>(),
    ) != 0
    {
        return -EFAULT;
    }

    /* make sure TLV info is consistent */
    if header.length as usize + size_of::<snd_ctl_tlv>() > size as usize {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("Inconsistent TLV, data %d + header %zu > %d\n"),
            header.length,
            size_of::<snd_ctl_tlv>(),
            size,
        );
        return -EINVAL;
    }

    /* be->max is coming from topology */
    if header.length as usize > (*scontrol).max_size - size_of::<sof_ipc_ctrl_data>() {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("Bytes data size %u exceeds max %zu\n"),
            header.length,
            (*scontrol).max_size - size_of::<sof_ipc_ctrl_data>(),
        );
        return -EINVAL;
    }

    /* Ensure the data is large enough to contain the ABI header */
    if (header.length as usize) < size_of::<sof_abi_hdr>() {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("Bytes data size %u less than ABI header %zu\n"),
            header.length,
            size_of::<sof_abi_hdr>(),
        );
        return -EINVAL;
    }

    /* Check that header id matches the command */
    if header.numid != (*cdata).cmd {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("Incorrect command for bytes put %d\n"),
            header.numid,
        );
        return -EINVAL;
    }

    if (*scontrol).old_ipc_control_data.is_null() {
        /* Create a backup of the current, valid bytes control */
        (*scontrol).old_ipc_control_data = kmemdup(
            (*scontrol).ipc_control_data as *const c_void,
            (*scontrol).max_size,
            GFP_KERNEL,
        );
        if (*scontrol).old_ipc_control_data.is_null() {
            return -ENOMEM;
        }
    }

    if copy_from_user(
        (*cdata).data as *mut c_void,
        (*tlvd).tlv.as_ptr() as *const c_void,
        header.length as usize,
    ) != 0
    {
        ret = -EFAULT;
        goto_err_restore(scontrol, cdata);
        return ret;
    }

    if (*(*cdata).data).magic != SOF_ABI_MAGIC {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("Wrong ABI magic 0x%08x\n"),
            (*(*cdata).data).magic,
        );
        goto_err_restore(scontrol, cdata);
        return ret;
    }

    if SOF_ABI_VERSION_INCOMPATIBLE(SOF_ABI_VERSION, (*(*cdata).data).abi) {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("Incompatible ABI version 0x%08x\n"),
            (*(*cdata).data).abi,
        );
        goto_err_restore(scontrol, cdata);
        return ret;
    }

    /* be->max has been verified to be >= sizeof(struct sof_abi_hdr) */
    if (*(*cdata).data).size as usize
        > (*scontrol).max_size - size_of::<sof_ipc_ctrl_data>() - size_of::<sof_abi_hdr>()
    {
        dev_err_ratelimited((*scomp).dev, c_str!("Mismatch in ABI data size (truncated?)\n"));
        goto_err_restore(scontrol, cdata);
        return ret;
    }

    /* notify DSP of byte control updates */
    if pm_runtime_active((*scomp).dev) {
        /* Actually send the data to the DSP; this is an opportunity to validate the data */
        return sof_ipc3_set_get_kcontrol_data(scontrol, true, true);
    }

    0
}

unsafe fn goto_err_restore(scontrol: *mut snd_sof_control, cdata: *mut sof_ipc_ctrl_data) {
    /* If we have an issue, we restore the old, valid bytes control data */
    if !(*scontrol).old_ipc_control_data.is_null() {
        memcpy(
            cdata as *mut c_void,
            (*scontrol).old_ipc_control_data as *const c_void,
            (*scontrol).max_size,
        );
        kfree((*scontrol).old_ipc_control_data);
        (*scontrol).old_ipc_control_data = ptr::null_mut();
    }
}

unsafe extern "C" fn _sof_ipc3_bytes_ext_get(
    scontrol: *mut snd_sof_control,
    binary_data: *const c_uint,
    mut size: c_uint,
    from_dsp: bool_,
) -> c_int {
    let tlvd = binary_data as *mut snd_ctl_tlv;
    let cdata = (*scontrol).ipc_control_data;
    let scomp = (*scontrol).scomp;
    let mut header: snd_ctl_tlv = core::mem::zeroed();
    let data_size: size_t;

    /*
     * Decrement the limit by ext bytes header size to
     * ensure the user space buffer is not exceeded.
     */
    if (size as usize) < size_of::<snd_ctl_tlv>() {
        return -ENOSPC;
    }

    size -= size_of::<snd_ctl_tlv>() as c_uint;

    /* set the ABI header values */
    (*(*cdata).data).magic = SOF_ABI_MAGIC;
    (*(*cdata).data).abi = SOF_ABI_VERSION;

    /* get all the component data from DSP */
    if from_dsp {
        let ret = sof_ipc3_set_get_kcontrol_data(scontrol, false, true);

        if ret < 0 {
            return ret;
        }
    }

    /* check data size doesn't exceed max coming from topology */
    if (*(*cdata).data).size as usize
        > (*scontrol).max_size - size_of::<sof_ipc_ctrl_data>() - size_of::<sof_abi_hdr>()
    {
        dev_err_ratelimited(
            (*scomp).dev,
            c_str!("User data size %u exceeds max size %zu\n"),
            (*(*cdata).data).size,
            (*scontrol).max_size - size_of::<sof_ipc_ctrl_data>() - size_of::<sof_abi_hdr>(),
        );
        return -EINVAL;
    }

    data_size = (*(*cdata).data).size as usize + size_of::<sof_abi_hdr>();

    /* make sure we don't exceed size provided by user space for data */
    if data_size > size as usize {
        return -ENOSPC;
    }

    header.numid = (*cdata).cmd;
    header.length = data_size as c_uint;
    if copy_to_user(
        tlvd as *mut c_void,
        &header as *const snd_ctl_tlv as *const c_void,
        size_of::<snd_ctl_tlv>(),
    ) != 0
    {
        return -EFAULT;
    }

    if copy_to_user(
        (*tlvd).tlv.as_mut_ptr() as *mut c_void,
        (*cdata).data as *const c_void,
        data_size,
    ) != 0
    {
        return -EFAULT;
    }

    0
}

unsafe extern "C" fn sof_ipc3_bytes_ext_get(
    scontrol: *mut snd_sof_control,
    binary_data: *const c_uint,
    size: c_uint,
) -> c_int {
    _sof_ipc3_bytes_ext_get(scontrol, binary_data, size, false)
}

unsafe extern "C" fn sof_ipc3_bytes_ext_volatile_get(
    scontrol: *mut snd_sof_control,
    binary_data: *const c_uint,
    size: c_uint,
) -> c_int {
    _sof_ipc3_bytes_ext_get(scontrol, binary_data, size, true)
}

unsafe extern "C" fn snd_sof_update_control(
    scontrol: *mut snd_sof_control,
    cdata: *mut sof_ipc_ctrl_data,
) {
    let scomp = (*scontrol).scomp;
    let local_cdata: *mut sof_ipc_ctrl_data;
    let mut i: c_int;

    local_cdata = (*scontrol).ipc_control_data;

    if (*cdata).cmd == SOF_CTRL_CMD_BINARY {
        if (*cdata).num_elems != (*(*local_cdata).data).size {
            dev_err(
                (*scomp).dev,
                c_str!("cdata binary size mismatch %u - %u\n"),
                (*cdata).num_elems,
                (*(*local_cdata).data).size,
            );
            return;
        }

        /* Verify the size fits within the allocation */
        if (*cdata).num_elems as usize
            > (*scontrol).max_size
                - size_of::<sof_ipc_ctrl_data>()
                - size_of::<sof_abi_hdr>()
        {
            dev_err(
                (*scomp).dev,
                c_str!("cdata binary size %u exceeds buffer\n"),
                (*cdata).num_elems,
            );
            return;
        }

        /* copy the new binary data */
        memcpy(
            (*local_cdata).data as *mut c_void,
            (*cdata).data as *const c_void,
            (*cdata).num_elems as usize,
        );
    } else if (*cdata).num_elems != (*scontrol).num_channels {
        dev_err(
            (*scomp).dev,
            c_str!("cdata channel count mismatch %u - %d\n"),
            (*cdata).num_elems,
            (*scontrol).num_channels,
        );
    } else {
        /* copy the new values */
        i = 0;
        while i < (*cdata).num_elems as c_int {
            (*(*local_cdata).chanv.as_mut_ptr().add(i as usize)).value =
                (*(*cdata).chanv.as_ptr().add(i as usize)).value;
            i += 1;
        }
    }
}

unsafe extern "C" fn sof_ipc3_control_update(
    sdev: *mut snd_sof_dev,
    ipc_control_message: *mut c_void,
) {
    let cdata = ipc_control_message as *mut sof_ipc_ctrl_data;
    let widget: *mut snd_soc_dapm_widget;
    let mut scontrol: *mut snd_sof_control;
    let mut swidget: *mut snd_sof_widget = ptr::null_mut();
    let mut kc: *mut snd_kcontrol = ptr::null_mut();
    let sm: *mut soc_mixer_control;
    let be: *mut soc_bytes_ext;
    let mut expected_size: size_t = 0;
    let se: *mut soc_enum;
    let mut found = false;
    let mut i: c_int;
    let type_: c_int;

    if (*cdata).type_ == SOF_CTRL_TYPE_VALUE_COMP_GET
        || (*cdata).type_ == SOF_CTRL_TYPE_VALUE_COMP_SET
    {
        dev_err(
            (*sdev).dev,
            c_str!("Component data is not supported in control notification\n"),
        );
        return;
    }

    /* Find the swidget first */
    list_for_each_entry!(swidget, &mut (*sdev).widget_list, list, {
        if (*swidget).comp_id == (*cdata).comp_id {
            found = true;
            break;
        }
    });

    if !found {
        return;
    }

    /* Translate SOF cmd to TPLG type */
    if (*cdata).cmd == SOF_CTRL_CMD_VOLUME || (*cdata).cmd == SOF_CTRL_CMD_SWITCH {
        type_ = SND_SOC_TPLG_TYPE_MIXER;
    } else if (*cdata).cmd == SOF_CTRL_CMD_BINARY {
        type_ = SND_SOC_TPLG_TYPE_BYTES;
    } else if (*cdata).cmd == SOF_CTRL_CMD_ENUM {
        type_ = SND_SOC_TPLG_TYPE_ENUM;
    } else {
        dev_err(
            (*sdev).dev,
            c_str!("Unknown cmd %u in %s\n"),
            (*cdata).cmd,
            c_str!("sof_ipc3_control_update"),
        );
        return;
    }

    widget = (*swidget).widget;
    i = 0;
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

    if (*cdata).cmd == SOF_CTRL_CMD_VOLUME || (*cdata).cmd == SOF_CTRL_CMD_SWITCH {
        sm = (*kc).private_value as *mut soc_mixer_control;
        scontrol = (*sm).dobj.private;
    } else if (*cdata).cmd == SOF_CTRL_CMD_BINARY {
        be = (*kc).private_value as *mut soc_bytes_ext;
        scontrol = (*be).dobj.private;
    } else if (*cdata).cmd == SOF_CTRL_CMD_ENUM {
        se = (*kc).private_value as *mut soc_enum;
        scontrol = (*se).dobj.private;
    } else {
        return;
    }

    if (*cdata).type_ == SOF_CTRL_TYPE_VALUE_CHAN_GET
        || (*cdata).type_ == SOF_CTRL_TYPE_VALUE_CHAN_SET
    {
        if check_mul_overflow(
            (*cdata).num_elems as size_t,
            size_of::<sof_ipc_ctrl_value_chan>(),
            &mut expected_size,
        ) {
            return;
        }
        if check_add_overflow(
            expected_size,
            size_of::<sof_ipc_ctrl_data>(),
            &mut expected_size,
        ) {
            return;
        }
    } else if (*cdata).type_ == SOF_CTRL_TYPE_DATA_GET || (*cdata).type_ == SOF_CTRL_TYPE_DATA_SET {
        if check_add_overflow(
            (*cdata).num_elems as size_t,
            size_of::<sof_abi_hdr>(),
            &mut expected_size,
        ) {
            return;
        }
        if check_add_overflow(
            expected_size,
            size_of::<sof_ipc_ctrl_data>(),
            &mut expected_size,
        ) {
            return;
        }
    } else {
        return;
    }

    if (*cdata).rhdr.hdr.size as usize != expected_size {
        dev_err((*sdev).dev, c_str!("Component notification size mismatch\n"));
        return;
    }

    if (*cdata).num_elems != 0 {
        /*
         * The message includes the updated value/data, update the
         * control's local cache using the received notification
         */
        snd_sof_update_control(scontrol, cdata);
    } else {
        /* Mark the scontrol that the value/data is changed in SOF */
        (*scontrol).comp_data_dirty = true;
    }

    snd_ctl_notify_one(
        (*(*(*swidget).scomp).card).snd_card,
        SNDRV_CTL_EVENT_MASK_VALUE,
        kc,
        0,
    );
}

unsafe extern "C" fn sof_ipc3_widget_kcontrol_setup(
    sdev: *mut snd_sof_dev,
    swidget: *mut snd_sof_widget,
) -> c_int {
    let mut scontrol: *mut snd_sof_control = ptr::null_mut();
    let mut ret: c_int;

    /* set up all controls for the widget */
    list_for_each_entry!(scontrol, &mut (*sdev).kcontrol_list, list, {
        if (*scontrol).comp_id == (*swidget).comp_id {
            /* set kcontrol data in DSP */
            ret = sof_ipc3_set_get_kcontrol_data(scontrol, true, false);
            if ret < 0 {
                dev_err(
                    (*sdev).dev,
                    c_str!("kcontrol %d set up failed for widget %s\n"),
                    (*scontrol).comp_id,
                    (*(*swidget).widget).name,
                );
                return ret;
            }

            /*
             * Read back the data from the DSP for static widgets.
             * This is particularly useful for binary kcontrols
             * associated with static pipeline widgets to initialize
             * the data size to match that in the DSP.
             */
            if (*swidget).dynamic_pipeline_widget {
                continue;
            }

            ret = sof_ipc3_set_get_kcontrol_data(scontrol, false, false);
            if ret < 0 {
                dev_warn(
                    (*sdev).dev,
                    c_str!("kcontrol %d read failed for widget %s\n"),
                    (*scontrol).comp_id,
                    (*(*swidget).widget).name,
                );
            }
        }
    });

    0
}

unsafe extern "C" fn sof_ipc3_set_up_volume_table(
    scontrol: *mut snd_sof_control,
    tlv: *mut c_int,
    size: c_int,
) -> c_int {
    let mut i: c_int;

    /* init the volume table */
    (*scontrol).volume_table = kcalloc(size as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if (*scontrol).volume_table.is_null() {
        return -ENOMEM;
    }

    /* populate the volume table */
    i = 0;
    while i < size {
        *(*scontrol).volume_table.add(i as usize) = vol_compute_gain(i, tlv);
        i += 1;
    }

    0
}

#[no_mangle]
pub static tplg_ipc3_control_ops: sof_ipc_tplg_control_ops = sof_ipc_tplg_control_ops {
    volume_put: Some(sof_ipc3_volume_put),
    volume_get: Some(sof_ipc3_volume_get),
    switch_put: Some(sof_ipc3_switch_put),
    switch_get: Some(sof_ipc3_switch_get),
    enum_put: Some(sof_ipc3_enum_put),
    enum_get: Some(sof_ipc3_enum_get),
    bytes_put: Some(sof_ipc3_bytes_put),
    bytes_get: Some(sof_ipc3_bytes_get),
    bytes_ext_put: Some(sof_ipc3_bytes_ext_put),
    bytes_ext_get: Some(sof_ipc3_bytes_ext_get),
    bytes_ext_volatile_get: Some(sof_ipc3_bytes_ext_volatile_get),
    update: Some(sof_ipc3_control_update),
    widget_kcontrol_setup: Some(sof_ipc3_widget_kcontrol_setup),
    set_up_volume_table: Some(sof_ipc3_set_up_volume_table),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
