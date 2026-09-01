// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//          Cezary Rojewski <cezary.rojewski@intel.com>
//

// C dependencies translated as crate-level dependencies:
// <linux/cleanup.h>, <sound/soc.h>, "avs.h", "control.h", "messages.h", "path.h"
use crate::*;

unsafe fn avs_get_kcontrol_adev(kcontrol: *mut snd_kcontrol) -> *mut avs_dev {
    let dapm: *mut snd_soc_dapm_context = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let dev: *mut device = snd_soc_dapm_to_dev(dapm);

    to_avs_dev(dev)
}

unsafe fn avs_get_volume_module(adev: *mut avs_dev, id: u32) -> *mut avs_path_module {
    let mut path: *mut avs_path;
    let mut ppl: *mut avs_path_pipeline;
    let mut mod_: *mut avs_path_module;

    // C source uses guard(spinlock)(&adev->path_list_lock).
    let _guard = guard_spinlock!(&mut (*adev).path_list_lock);
    list_for_each_entry!(path, &mut (*adev).path_list, node, {
        list_for_each_entry!(ppl, &mut (*path).ppl_list, node, {
            list_for_each_entry!(mod_, &mut (*ppl).mod_list, node, {
                let type_: *mut guid_t = &mut (*(*(*mod_).template).cfg_ext).type_;

                if (guid_equal(type_, &AVS_PEAKVOL_MOD_UUID as *const _)
                    || guid_equal(type_, &AVS_GAIN_MOD_UUID as *const _))
                    && (*(*mod_).template).ctl_id == id
                {
                    return mod_;
                }
            });
        });
    });

    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn avs_control_volume_get(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> i32 {
    let mc: *mut soc_mixer_control = (*kctl).private_value as *mut soc_mixer_control;
    let ctl_data: *mut avs_control_data = (*mc).dobj.private as *mut avs_control_data;
    let active_module: *mut avs_path_module;
    let mut dspvols: *mut avs_volume_cfg = core::ptr::null_mut();
    let adev: *mut avs_dev;
    let mut num_dspvols: usize = 0;
    let mut ret: i32;
    let mut i: i32;

    adev = avs_get_kcontrol_adev(kctl);

    /* Prevent access to modules while path is being constructed. */
    let _guard = guard_mutex!(&mut (*adev).path_mutex);

    active_module = avs_get_volume_module(adev, (*ctl_data).id);
    if !active_module.is_null() {
        ret = avs_ipc_peakvol_get_volume(
            adev,
            (*active_module).module_id,
            (*active_module).instance_id,
            &mut dspvols,
            &mut num_dspvols,
        );
        if ret != 0 {
            return AVS_IPC_RET(ret);
        }

        /* Do not copy more than the control can store. */
        num_dspvols = min_t!(u32, num_dspvols, SND_SOC_TPLG_MAX_CHAN) as usize;
        i = 0;
        while i < num_dspvols as i32 {
            (*ctl_data).values[i as usize] = (*dspvols.add(i as usize)).target_volume;
            i += 1;
        }
        kfree(dspvols as *mut core::ffi::c_void);
    }

    memcpy(
        (*uctl).value.integer.value.as_mut_ptr() as *mut core::ffi::c_void,
        (*ctl_data).values.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ctl_data).values),
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_control_volume_put(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> i32 {
    let active_module: *mut avs_path_module;
    let ctl_data: *mut avs_control_data;
    let mc: *mut soc_mixer_control;
    let adev: *mut avs_dev;
    let input: *mut core::ffi::c_long;
    let mut ret: i32;
    let mut i: i32;

    mc = (*kctl).private_value as *mut soc_mixer_control;
    ctl_data = (*mc).dobj.private as *mut avs_control_data;
    adev = avs_get_kcontrol_adev(kctl);
    input = (*uctl).value.integer.value.as_mut_ptr();
    i = 0;

    /* mc->num_channels can be 0. */
    loop {
        if *input.add(i as usize) < (*mc).min || *input.add(i as usize) > (*mc).max {
            return -EINVAL;
        }
        i += 1;
        if i >= (*mc).num_channels {
            break;
        }
    }

    if memcmp(
        (*ctl_data).values.as_ptr() as *const core::ffi::c_void,
        input as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ctl_data).values),
    ) == 0
    {
        return 0;
    }

    /* Prevent access to modules while path is being constructed. */
    let _guard = guard_mutex!(&mut (*adev).path_mutex);

    active_module = avs_get_volume_module(adev, (*ctl_data).id);
    if !active_module.is_null() {
        ret = avs_peakvol_set_volume(adev, active_module, mc, input);
        if ret != 0 {
            return ret;
        }
    }

    memcpy(
        (*ctl_data).values.as_mut_ptr() as *mut core::ffi::c_void,
        input as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ctl_data).values),
    );
    1
}

#[no_mangle]
pub unsafe extern "C" fn avs_control_volume_info(
    kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let mc: *mut soc_mixer_control = (*kctl).private_value as *mut soc_mixer_control;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = max_t!(u32, 1, (*mc).num_channels);
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*mc).max;
    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_control_mute_get(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> i32 {
    let mc: *mut soc_mixer_control = (*kctl).private_value as *mut soc_mixer_control;
    let ctl_data: *mut avs_control_data = (*mc).dobj.private as *mut avs_control_data;
    let active_module: *mut avs_path_module;
    let mut dspmutes: *mut avs_mute_cfg = core::ptr::null_mut();
    let adev: *mut avs_dev;
    let mut num_dspmutes: usize = 0;
    let mut ret: i32;
    let mut i: i32;

    adev = avs_get_kcontrol_adev(kctl);

    /* Prevent access to modules while path is being constructed. */
    let _guard = guard_mutex!(&mut (*adev).path_mutex);

    active_module = avs_get_volume_module(adev, (*ctl_data).id);
    if !active_module.is_null() {
        ret = avs_ipc_peakvol_get_mute(
            adev,
            (*active_module).module_id,
            (*active_module).instance_id,
            &mut dspmutes,
            &mut num_dspmutes,
        );
        if ret != 0 {
            return AVS_IPC_RET(ret);
        }

        /* Do not copy more than the control can store. */
        num_dspmutes = min_t!(u32, num_dspmutes, SND_SOC_TPLG_MAX_CHAN) as usize;
        i = 0;
        while i < num_dspmutes as i32 {
            (*ctl_data).values[i as usize] = if (*dspmutes.add(i as usize)).mute == 0 {
                1
            } else {
                0
            };
            i += 1;
        }
        kfree(dspmutes as *mut core::ffi::c_void);
    }

    memcpy(
        (*uctl).value.integer.value.as_mut_ptr() as *mut core::ffi::c_void,
        (*ctl_data).values.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ctl_data).values),
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_control_mute_put(
    kctl: *mut snd_kcontrol,
    uctl: *mut snd_ctl_elem_value,
) -> i32 {
    let active_module: *mut avs_path_module;
    let ctl_data: *mut avs_control_data;
    let mc: *mut soc_mixer_control;
    let adev: *mut avs_dev;
    let input: *mut core::ffi::c_long;
    let mut ret: i32;
    let mut i: i32;

    mc = (*kctl).private_value as *mut soc_mixer_control;
    ctl_data = (*mc).dobj.private as *mut avs_control_data;
    adev = avs_get_kcontrol_adev(kctl);
    input = (*uctl).value.integer.value.as_mut_ptr();
    i = 0;

    /* mc->num_channels can be 0. */
    loop {
        if *input.add(i as usize) < (*mc).min || *input.add(i as usize) > (*mc).max {
            return -EINVAL;
        }
        i += 1;
        if i >= (*mc).num_channels {
            break;
        }
    }

    if memcmp(
        (*ctl_data).values.as_ptr() as *const core::ffi::c_void,
        input as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ctl_data).values),
    ) == 0
    {
        return 0;
    }

    /* Prevent access to modules while path is being constructed. */
    let _guard = guard_mutex!(&mut (*adev).path_mutex);

    active_module = avs_get_volume_module(adev, (*ctl_data).id);
    if !active_module.is_null() {
        ret = avs_peakvol_set_mute(adev, active_module, mc, input);
        if ret != 0 {
            return ret;
        }
    }

    memcpy(
        (*ctl_data).values.as_mut_ptr() as *mut core::ffi::c_void,
        input as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ctl_data).values),
    );
    1
}

#[no_mangle]
pub unsafe extern "C" fn avs_control_mute_info(
    kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let mc: *mut soc_mixer_control = (*kctl).private_value as *mut soc_mixer_control;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = max_t!(u32, 1, (*mc).num_channels);
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*mc).max;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
