// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

/* Mixer Controls */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const EINVAL: c_int = 22;
pub const EACCES: c_int = 13;
pub const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
pub const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_tlv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub dobj: snd_soc_dobj,
    pub min: c_int,
    pub max: c_int,
    pub platform_max: c_int,
}

#[repr(C)]
pub struct soc_enum {
    pub dobj: snd_soc_dobj,
}

#[repr(C)]
pub struct soc_bytes_ext {
    pub dobj: snd_soc_dobj,
}

#[repr(C)]
pub struct snd_soc_dobj {
    pub private: *mut snd_sof_control,
}

#[repr(C)]
pub struct snd_sof_control {
    pub scomp: *mut snd_soc_component,
    pub num_channels: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
    pub control: *const sof_ipc_tplg_control_ops,
}

#[repr(C)]
pub struct sof_ipc_tplg_control_ops {
    pub volume_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub volume_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub switch_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub switch_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub enum_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub enum_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> c_int>,
    pub bytes_ext_put:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *const c_uint, c_uint) -> c_int>,
    pub bytes_ext_volatile_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut c_uint, c_uint) -> c_int>,
    pub bytes_ext_get:
        Option<unsafe extern "C" fn(*mut snd_sof_control, *mut c_uint, c_uint) -> c_int>,
}

unsafe extern "C" {
    pub fn snd_soc_component_get_drvdata(scomp: *mut snd_soc_component) -> *mut snd_sof_dev;
    /* C source used sof_ipc_get_ops(sdev, tplg). */
    pub fn sof_ipc_get_ops_tplg(sdev: *mut snd_sof_dev) -> *const sof_ipc_tplg_ops;
    pub fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    pub fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    pub fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    pub fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int;
    pub fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
}

#[inline]
unsafe fn sof_ipc_get_ops(sdev: *mut snd_sof_dev) -> *const sof_ipc_tplg_ops {
    unsafe { sof_ipc_get_ops_tplg(sdev) }
}

unsafe fn control_ops(tplg_ops: *const sof_ipc_tplg_ops) -> *const sof_ipc_tplg_control_ops {
    if !tplg_ops.is_null() {
        unsafe { (*tplg_ops).control }
    } else {
        core::ptr::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sm = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let scontrol = unsafe { (*sm).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(volume_get) = unsafe { (*control).volume_get } {
            return unsafe { volume_get(scontrol, ucontrol) };
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sm = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let scontrol = unsafe { (*sm).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(volume_put) = unsafe { (*control).volume_put } {
            return unsafe { volume_put(scontrol, ucontrol) };
        }
    }

    false as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_volume_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let sm = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let scontrol = unsafe { (*sm).dobj.private };
    let channels = unsafe { (*scontrol).num_channels };
    let platform_max: c_int;

    if unsafe { (*sm).platform_max } == 0 {
        unsafe {
            (*sm).platform_max = (*sm).max;
        }
    }
    platform_max = unsafe { (*sm).platform_max };

    if platform_max == 1
        && unsafe { strstr((*kcontrol).id.name.as_ptr(), c" Volume".as_ptr()).is_null() }
    {
        unsafe {
            (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN as c_uint;
        }
    } else {
        unsafe {
            (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
        }
    }

    unsafe {
        (*uinfo).count = channels;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = (platform_max - (*sm).min) as i64;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_switch_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sm = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let scontrol = unsafe { (*sm).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(switch_get) = unsafe { (*control).switch_get } {
            return unsafe { switch_get(scontrol, ucontrol) };
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_switch_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sm = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let scontrol = unsafe { (*sm).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(switch_put) = unsafe { (*control).switch_put } {
            return unsafe { switch_put(scontrol, ucontrol) };
        }
    }

    false as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_enum_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let se = unsafe { (*kcontrol).private_value as *mut soc_enum };
    let scontrol = unsafe { (*se).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(enum_get) = unsafe { (*control).enum_get } {
            return unsafe { enum_get(scontrol, ucontrol) };
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_enum_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let se = unsafe { (*kcontrol).private_value as *mut soc_enum };
    let scontrol = unsafe { (*se).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(enum_put) = unsafe { (*control).enum_put } {
            return unsafe { enum_put(scontrol, ucontrol) };
        }
    }

    false as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_bytes_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let be = unsafe { (*kcontrol).private_value as *mut soc_bytes_ext };
    let scontrol = unsafe { (*be).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(bytes_get) = unsafe { (*control).bytes_get } {
            return unsafe { bytes_get(scontrol, ucontrol) };
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_bytes_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let be = unsafe { (*kcontrol).private_value as *mut soc_bytes_ext };
    let scontrol = unsafe { (*be).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(bytes_put) = unsafe { (*control).bytes_put } {
            return unsafe { bytes_put(scontrol, ucontrol) };
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_bytes_ext_put(
    kcontrol: *mut snd_kcontrol,
    binary_data: *const c_uint,
    size: c_uint,
) -> c_int {
    let be = unsafe { (*kcontrol).private_value as *mut soc_bytes_ext };
    let scontrol = unsafe { (*be).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    /* make sure we have at least a header */
    if (size as usize) < core::mem::size_of::<snd_ctl_tlv>() {
        return -EINVAL;
    }

    if !control.is_null() {
        if let Some(bytes_ext_put) = unsafe { (*control).bytes_ext_put } {
            return unsafe { bytes_ext_put(scontrol, binary_data, size) };
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_bytes_ext_volatile_get(
    kcontrol: *mut snd_kcontrol,
    binary_data: *mut c_uint,
    size: c_uint,
) -> c_int {
    let be = unsafe { (*kcontrol).private_value as *mut soc_bytes_ext };
    let scontrol = unsafe { (*be).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };
    let mut ret: c_int;
    let err: c_int;

    /* ignore the ext_volatile_get call if the callbacks are not provided */
    if control.is_null() || unsafe { (*control).bytes_ext_volatile_get.is_none() } {
        return 0;
    }

    ret = unsafe { pm_runtime_resume_and_get((*scomp).dev) };
    if ret < 0 && ret != -EACCES {
        unsafe {
            dev_err_ratelimited(
                (*scomp).dev,
                c"%s: failed to resume %d\n".as_ptr(),
                c"snd_sof_bytes_ext_volatile_get".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    /* Make sure the DSP/firmware is booted up */
    ret = unsafe { snd_sof_boot_dsp_firmware(sdev) };
    if ret == 0 {
        let bytes_ext_volatile_get = unsafe { (*control).bytes_ext_volatile_get.unwrap_unchecked() };
        ret = unsafe { bytes_ext_volatile_get(scontrol, binary_data, size) };
    }

    err = unsafe { pm_runtime_put_autosuspend((*scomp).dev) };
    if err < 0 {
        unsafe {
            dev_err_ratelimited(
                (*scomp).dev,
                c"%s: failed to idle %d\n".as_ptr(),
                c"snd_sof_bytes_ext_volatile_get".as_ptr(),
                err,
            );
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_sof_bytes_ext_get(
    kcontrol: *mut snd_kcontrol,
    binary_data: *mut c_uint,
    size: c_uint,
) -> c_int {
    let be = unsafe { (*kcontrol).private_value as *mut soc_bytes_ext };
    let scontrol = unsafe { (*be).dobj.private };
    let scomp = unsafe { (*scontrol).scomp };
    let sdev = unsafe { snd_soc_component_get_drvdata(scomp) };
    let tplg_ops = unsafe { sof_ipc_get_ops(sdev) };
    let control = unsafe { control_ops(tplg_ops) };

    if !control.is_null() {
        if let Some(bytes_ext_get) = unsafe { (*control).bytes_ext_get } {
            return unsafe { bytes_ext_get(scontrol, binary_data, size) };
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
