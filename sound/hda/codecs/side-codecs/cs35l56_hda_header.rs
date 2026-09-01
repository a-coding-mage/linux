// SPDX-License-Identifier: GPL-2.0-only
//
// HDA audio driver for Cirrus Logic CS35L56 smart amp
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// C dependencies:
// #include <linux/container_of.h>
// #include <linux/device.h>
// #include <linux/gpio/consumer.h>
// #include <linux/firmware/cirrus/cs_dsp.h>
// #include <linux/firmware/cirrus/wmfw.h>
// #include <linux/regulator/consumer.h>
// #include <linux/workqueue.h>
// #include <sound/cs35l56.h>

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct cs35l56_hda {
    pub base: cs35l56_base,
    pub codec: *mut hda_codec,
    pub dsp_work: work_struct,

    pub index: c_int,
    pub num_amps: c_int,
    pub system_name: *const c_char,
    pub amp_name: *const c_char,

    pub cs_dsp: cs_dsp,
    pub playing: bool,
    pub suspended: bool,
    pub asp_tx_mask: u8,

    pub posture_ctl: *mut snd_kcontrol,
    pub volume_ctl: *mut snd_kcontrol,
    pub mixer_ctl: [*mut snd_kcontrol; 4],

    // Present in C when IS_ENABLED(CONFIG_SND_DEBUG):
    // pub debugfs_root: *mut dentry,
}

#[inline]
pub unsafe fn cs35l56_hda_from_base(cs35l56_base: *mut cs35l56_base) -> *mut cs35l56_hda {
    unsafe {
        (cs35l56_base as *mut u8)
            .sub(core::mem::offset_of!(cs35l56_hda, base))
            as *mut cs35l56_hda
    }
}

unsafe extern "C" {
    pub static cs35l56_hda_pm_ops: dev_pm_ops;

    pub fn cs35l56_hda_common_probe(
        cs35l56: *mut cs35l56_hda,
        hid: c_int,
        id: c_int,
    ) -> c_int;
    pub fn cs35l56_hda_remove(dev: *mut device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
