// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   32bit -> 64bit ioctl wrapper for hwdep API
 *   Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

/* This file is included from hwdep.c */

/* C dependency intent: #include <linux/compat.h> */

use core::ffi::{c_int, c_long, c_uint, c_ulong, c_void};

#[allow(non_camel_case_types)]
type u32 = core::ffi::c_uint;

#[allow(non_camel_case_types)]
type compat_caddr_t = u32;

const EFAULT: c_int = 14;
const ENOIOCTLCMD: c_int = 515;

#[repr(C)]
pub struct snd_hwdep_dsp_image32 {
    pub index: u32,
    pub name: [u8; 64],
    pub image: u32, /* pointer */
    pub length: u32,
    pub driver_data: u32,
} /* don't set packed attribute here */

#[repr(C)]
pub struct snd_hwdep_dsp_image {
    pub index: u32,
    pub name: [u8; 64],
    pub image: *mut c_void,
    pub length: u32,
    pub driver_data: u32,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_hwdep_ops {
    pub ioctl_compat: Option<
        unsafe extern "C" fn(
            hw: *mut snd_hwdep,
            file: *mut file,
            cmd: c_uint,
            arg: c_ulong,
        ) -> c_long,
    >,
}

#[repr(C)]
pub struct snd_hwdep {
    pub ops: snd_hwdep_ops,
}

unsafe extern "C" {
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;
    fn snd_hwdep_dsp_load(hw: *mut snd_hwdep, info: *mut snd_hwdep_dsp_image) -> c_int;
    fn snd_hwdep_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;

    static SNDRV_HWDEP_IOCTL_PVERSION: c_uint;
    static SNDRV_HWDEP_IOCTL_INFO: c_uint;
    static SNDRV_HWDEP_IOCTL_DSP_STATUS: c_uint;
}

#[inline]
unsafe fn compat_ptr(ptr: c_ulong) -> *mut c_void {
    ptr as usize as *mut c_void
}

#[inline]
unsafe fn get_user_u32(dst: *mut u32, src: *const u32) -> c_int {
    unsafe {
        *dst = core::ptr::read(src);
    }
    0
}

#[inline]
const fn ioc(dir: c_uint, type_: c_uint, nr: c_uint, size: c_uint) -> c_uint {
    const IOC_NRBITS: c_uint = 8;
    const IOC_TYPEBITS: c_uint = 8;
    const IOC_SIZEBITS: c_uint = 14;

    const IOC_NRSHIFT: c_uint = 0;
    const IOC_TYPESHIFT: c_uint = IOC_NRSHIFT + IOC_NRBITS;
    const IOC_SIZESHIFT: c_uint = IOC_TYPESHIFT + IOC_TYPEBITS;
    const IOC_DIRSHIFT: c_uint = IOC_SIZESHIFT + IOC_SIZEBITS;

    (dir << IOC_DIRSHIFT) | (type_ << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

#[inline]
const fn iow<T>(type_: c_uint, nr: c_uint) -> c_uint {
    const IOC_WRITE: c_uint = 1;
    ioc(IOC_WRITE, type_, nr, core::mem::size_of::<T>() as c_uint)
}

const SNDRV_HWDEP_IOCTL_DSP_LOAD32: c_uint = iow::<snd_hwdep_dsp_image32>('H' as c_uint, 0x03);

unsafe fn snd_hwdep_dsp_load_compat(
    hw: *mut snd_hwdep,
    src: *mut snd_hwdep_dsp_image32,
) -> c_int {
    let mut info: snd_hwdep_dsp_image = unsafe { core::mem::zeroed() };
    let mut ptr: compat_caddr_t = 0;

    if unsafe {
        copy_from_user(
            (&mut info as *mut snd_hwdep_dsp_image).cast::<c_void>(),
            src.cast::<c_void>(),
            (4 + 64) as c_ulong,
        ) != 0
            || get_user_u32(&mut ptr, core::ptr::addr_of!((*src).image)) != 0
            || get_user_u32(&mut info.length, core::ptr::addr_of!((*src).length)) != 0
            || get_user_u32(
                &mut info.driver_data,
                core::ptr::addr_of!((*src).driver_data),
            ) != 0
    } {
        return -EFAULT;
    }
    info.image = unsafe { compat_ptr(ptr as c_ulong) };

    unsafe { snd_hwdep_dsp_load(hw, &mut info) }
}

unsafe fn snd_hwdep_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let hw: *mut snd_hwdep = unsafe { (*file).private_data.cast::<snd_hwdep>() };
    let argp: *mut c_void = unsafe { compat_ptr(arg) };

    unsafe {
        if cmd == SNDRV_HWDEP_IOCTL_PVERSION
            || cmd == SNDRV_HWDEP_IOCTL_INFO
            || cmd == SNDRV_HWDEP_IOCTL_DSP_STATUS
        {
            return snd_hwdep_ioctl(file, cmd, argp as c_ulong);
        }
    }

    match cmd {
        SNDRV_HWDEP_IOCTL_DSP_LOAD32 => {
            return unsafe { snd_hwdep_dsp_load_compat(hw, argp.cast::<snd_hwdep_dsp_image32>()) }
                as c_long;
        }
        _ => {}
    }

    if let Some(ioctl_compat) = unsafe { (*hw).ops.ioctl_compat } {
        return unsafe { ioctl_compat(hw, file, cmd, arg) };
    }
    -(ENOIOCTLCMD as c_long)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
