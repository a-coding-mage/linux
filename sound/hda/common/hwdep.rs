// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HWDEP Interface for HD-audio codec
 *
 * Copyright (c) 2007 Takashi Iwai <tiwai@suse.de>
 */

/*
 * Original C dependencies:
 * linux/init.h, linux/slab.h, linux/compat.h, linux/nospec.h,
 * sound/core.h, sound/hda_codec.h, hda_local.h, sound/hda_hwdep.h,
 * sound/minors.h
 */

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

type u32 = u32;

const EFAULT: c_int = 14;
const ENOIOCTLCMD: c_int = 515;
const EACCES: c_int = 13;
const CAP_SYS_RAWIO: c_int = 17;

extern "C" {
    static snd_hda_dev_attr_groups: *mut c_void;

    fn get_user_u32(value: *mut u32, ptr: *const u32) -> c_int;
    fn put_user_u32(value: u32, ptr: *mut u32) -> c_int;
    fn put_user_int(value: c_int, ptr: *mut c_int) -> c_int;
    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: u32,
        flags: c_int,
        verb: u32,
        parm: u32,
    ) -> u32;
    fn array_index_nospec(index: u32, size: u32) -> u32;
    fn capable(cap: c_int) -> bool;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_hwdep_new(
        card: *mut c_void,
        id: *const c_char,
        device: c_int,
        rhwdep: *mut *mut snd_hwdep,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);

    /*
     * CONFIG_COMPAT only: C used compat_ptr(arg) before forwarding to the
     * native ioctl handler.
     */
    fn compat_ptr(arg: c_ulong) -> *mut c_void;
}

extern "C" {
    static HDA_IOCTL_PVERSION: c_uint;
    static HDA_IOCTL_VERB_WRITE: c_uint;
    static HDA_IOCTL_GET_WCAP: c_uint;
    static HDA_HWDEP_VERSION: c_int;
    static SNDRV_HWDEP_IFACE_HDA: c_int;
}

#[repr(C)]
pub struct hda_codec_core {
    pub start_nid: u32,
    pub num_nodes: u32,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
    pub wcaps: *mut u32,
    pub addr: c_int,
    pub card: *mut c_void,
    pub hwdep: *mut snd_hwdep,
}

#[repr(C)]
pub struct hda_verb_ioctl {
    pub verb: u32,
    pub res: u32,
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub groups: *mut c_void,
}

#[repr(C)]
pub struct snd_hwdep_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file) -> c_int>,
    pub ioctl:
        Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int>,
    /*
     * CONFIG_COMPAT: present in the C struct when compatibility ioctls are
     * enabled by the kernel configuration.
     */
    pub ioctl_compat:
        Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int>,
}

#[repr(C)]
pub struct snd_hwdep {
    pub name: [c_char; 0],
    pub iface: c_int,
    pub private_data: *mut c_void,
    pub exclusive: c_int,
    pub ops: snd_hwdep_ops,
    pub dev: *mut device,
}

/*
 * write/read an out-of-bound verb
 */
unsafe extern "C" fn verb_write_ioctl(
    codec: *mut hda_codec,
    arg: *mut hda_verb_ioctl,
) -> c_int {
    let mut verb: u32 = 0;
    let res: u32;

    if get_user_u32(core::ptr::addr_of_mut!(verb), core::ptr::addr_of!((*arg).verb)) != 0 {
        return -EFAULT;
    }
    res = snd_hda_codec_read(
        codec,
        verb >> 24,
        0,
        (verb >> 8) & 0xffff,
        verb & 0xff,
    );
    if put_user_u32(res, core::ptr::addr_of_mut!((*arg).res)) != 0 {
        return -EFAULT;
    }
    0
}

unsafe extern "C" fn get_wcap_ioctl(
    codec: *mut hda_codec,
    arg: *mut hda_verb_ioctl,
) -> c_int {
    let mut verb: u32 = 0;
    let res: u32;

    if get_user_u32(core::ptr::addr_of_mut!(verb), core::ptr::addr_of!((*arg).verb)) != 0 {
        return -EFAULT;
    }
    /* open-code get_wcaps(verb>>24) with nospec */
    verb >>= 24;
    if verb < (*codec).core.start_nid
        || verb >= (*codec).core.start_nid.wrapping_add((*codec).core.num_nodes)
    {
        res = 0;
    } else {
        verb = verb.wrapping_sub((*codec).core.start_nid);
        verb = array_index_nospec(verb, (*codec).core.num_nodes);
        res = *(*codec).wcaps.add(verb as usize);
    }
    if put_user_u32(res, core::ptr::addr_of_mut!((*arg).res)) != 0 {
        return -EFAULT;
    }
    0
}

/*
 */
unsafe extern "C" fn hda_hwdep_ioctl(
    hw: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let codec: *mut hda_codec = (*hw).private_data as *mut hda_codec;
    let argp: *mut c_void = arg as *mut c_void;

    let _ = file;

    if cmd == HDA_IOCTL_PVERSION {
        return put_user_int(HDA_HWDEP_VERSION, argp as *mut c_int);
    }
    if cmd == HDA_IOCTL_VERB_WRITE {
        return verb_write_ioctl(codec, argp as *mut hda_verb_ioctl);
    }
    if cmd == HDA_IOCTL_GET_WCAP {
        return get_wcap_ioctl(codec, argp as *mut hda_verb_ioctl);
    }
    -ENOIOCTLCMD
}

/* CONFIG_COMPAT */
unsafe extern "C" fn hda_hwdep_ioctl_compat(
    hw: *mut snd_hwdep,
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    hda_hwdep_ioctl(hw, file, cmd, compat_ptr(arg) as c_ulong)
}

unsafe extern "C" fn hda_hwdep_open(hw: *mut snd_hwdep, file: *mut file) -> c_int {
    let _ = hw;
    let _ = file;

    if !capable(CAP_SYS_RAWIO) {
        return -EACCES;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_hda_create_hwdep(codec: *mut hda_codec) -> c_int {
    let mut hwname: [c_char; 16] = [0; 16];
    let mut hwdep: *mut snd_hwdep = core::ptr::null_mut();
    let err: c_int;

    err = sprintf(
        hwname.as_mut_ptr(),
        b"HDA Codec %d\0".as_ptr() as *const c_char,
        (*codec).addr,
    );
    let _ = err;

    let err = snd_hwdep_new(
        (*codec).card,
        hwname.as_ptr(),
        (*codec).addr,
        core::ptr::addr_of_mut!(hwdep),
    );
    if err < 0 {
        return err;
    }
    (*codec).hwdep = hwdep;
    sprintf(
        (*hwdep).name.as_mut_ptr(),
        b"HDA Codec %d\0".as_ptr() as *const c_char,
        (*codec).addr,
    );
    (*hwdep).iface = SNDRV_HWDEP_IFACE_HDA;
    (*hwdep).private_data = codec as *mut c_void;
    (*hwdep).exclusive = 1;

    (*hwdep).ops.open = Some(hda_hwdep_open);
    (*hwdep).ops.ioctl = Some(hda_hwdep_ioctl);
    /* CONFIG_COMPAT */
    (*hwdep).ops.ioctl_compat = Some(hda_hwdep_ioctl_compat);

    /* for sysfs */
    (*(*hwdep).dev).groups = snd_hda_dev_attr_groups;
    dev_set_drvdata((*hwdep).dev, codec as *mut c_void);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
