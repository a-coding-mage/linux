/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Hardware dependent layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/* C dependencies: <sound/asound.h>, <linux/poll.h> */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/* hwdep file ops; all ops can be NULL */
#[repr(C)]
pub struct snd_hwdep_ops {
    pub llseek: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        file: *mut file,
        offset: i64,
        orig: c_int,
    ) -> i64>,
    pub read: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        buf: *mut c_char,
        count: c_long,
        offset: *mut loff_t,
    ) -> c_long>,
    pub write: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        buf: *const c_char,
        count: c_long,
        offset: *mut loff_t,
    ) -> c_long>,
    pub open: Option<unsafe extern "C" fn(hw: *mut snd_hwdep, file: *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(hw: *mut snd_hwdep, file: *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        file: *mut file,
        wait: *mut poll_table,
    ) -> __poll_t>,
    pub ioctl: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        file: *mut file,
        cmd: c_uint,
        arg: c_ulong,
    ) -> c_int>,
    pub ioctl_compat: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        file: *mut file,
        cmd: c_uint,
        arg: c_ulong,
    ) -> c_int>,
    pub mmap: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        file: *mut file,
        vma: *mut vm_area_struct,
    ) -> c_int>,
    pub dsp_status: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        status: *mut snd_hwdep_dsp_status,
    ) -> c_int>,
    pub dsp_load: Option<unsafe extern "C" fn(
        hw: *mut snd_hwdep,
        image: *mut snd_hwdep_dsp_image,
    ) -> c_int>,
}

#[repr(C)]
pub struct snd_hwdep {
    pub card: *mut snd_card,
    pub list: list_head,
    pub device: c_int,
    pub id: [c_char; 32],
    pub name: [c_char; 80],
    pub iface: c_int,

    #[cfg(feature = "CONFIG_SND_OSSEMUL")]
    pub oss_type: c_int,
    #[cfg(feature = "CONFIG_SND_OSSEMUL")]
    pub ossreg: c_int,

    pub ops: snd_hwdep_ops,
    pub open_wait: wait_queue_head_t,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(hwdep: *mut snd_hwdep)>,
    pub dev: *mut device,

    pub open_mutex: mutex,
    pub used: c_int,                 /* reference counter */
    pub dsp_loaded: c_uint,          /* bit fields of loaded dsp indices */
    pub exclusive: c_uint,           /* exclusive access mode; C bit-field width: 1 */
}

extern "C" {
    pub fn snd_hwdep_new(
        card: *mut snd_card,
        id: *mut c_char,
        device: c_int,
        rhwdep: *mut *mut snd_hwdep,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
