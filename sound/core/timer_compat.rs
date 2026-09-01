// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   32bit -> 64bit ioctl wrapper for timer API
 *   Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

/* This file included from timer.c */

// Rust translation note: the original C file depends on <linux/compat.h> and
// on declarations from the including timer.c translation unit.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;

pub type u32 = u32;
pub type s32 = i32;

pub const EFAULT: c_int = 14;
pub const EBADFD: c_int = 77;
pub const ENOIOCTLCMD: c_int = 515;

pub const SNDRV_TIMER_HW_SLAVE: u32 = 1 << 0;
pub const SNDRV_TIMER_FLG_SLAVE: u32 = 1 << 0;

/*
 * ILP32/LP64 has different size for 'long' type. Additionally, the size
 * of storage alignment differs depending on architectures. Here, '__packed'
 * qualifier is used so that the size of this structure is multiple of 4 and
 * it fits to any architectures with 32 bit storage alignment.
 */
#[repr(C, packed)]
pub struct snd_timer_gparams32 {
    pub tid: snd_timer_id,
    pub period_num: u32,
    pub period_den: u32,
    pub reserved: [u8; 32],
}

#[repr(C)]
pub struct snd_timer_info32 {
    pub flags: u32,
    pub card: s32,
    pub id: [u8; 64],
    pub name: [u8; 80],
    pub reserved0: u32,
    pub resolution: u32,
    pub reserved: [u8; 64],
}

#[repr(C)]
pub struct snd_timer_id {
    pub _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct snd_timer_gparams {
    pub tid: snd_timer_id,
    pub period_num: c_ulong,
    pub period_den: c_ulong,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct mutex {
    pub _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct snd_timer_user {
    pub timeri: *mut snd_timer_instance,
    pub ioctl_lock: mutex,
}

#[repr(C)]
pub struct snd_timer_instance {
    pub _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
}

#[repr(C)]
pub struct snd_timer_hardware {
    pub flags: u32,
    pub resolution: u32,
}

#[repr(C)]
pub struct snd_timer {
    pub card: *mut snd_card,
    pub hw: snd_timer_hardware,
    pub id: [c_char; 64],
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_timer_status32 {
    pub _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct snd_timer_status64 {
    pub _bindgen_opaque_blob: [u8; 0],
}

unsafe extern "C" {
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn compat_ptr(arg: c_ulong) -> *mut c_void;
    fn timer_set_gparams(gparams: *mut snd_timer_gparams) -> c_int;
    fn snd_timeri_timer_get(timeri: *mut snd_timer_instance) -> *mut snd_timer;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dest: *mut u8, src: *const c_char, count: usize) -> isize;
    fn __snd_timer_user_ioctl(
        file: *mut file,
        cmd: c_uint,
        arg: c_ulong,
        compat: bool,
    ) -> c_long;
    fn snd_timer_user_status32(file: *mut file, user: *mut c_void) -> c_long;
    fn snd_timer_user_status64(file: *mut file, user: *mut c_void) -> c_long;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

unsafe fn get_user_u32(dst: *mut u32, src: *const u32) -> c_int {
    copy_from_user(
        dst.cast::<c_void>(),
        src.cast::<c_void>(),
        size_of::<u32>(),
    ) as c_int
}

unsafe fn _IOC(dir: c_uint, ty: c_uint, nr: c_uint, size: c_uint) -> c_uint {
    const IOC_NRBITS: c_uint = 8;
    const IOC_TYPEBITS: c_uint = 8;
    const IOC_SIZEBITS: c_uint = 14;
    const IOC_NRSHIFT: c_uint = 0;
    const IOC_TYPESHIFT: c_uint = IOC_NRSHIFT + IOC_NRBITS;
    const IOC_SIZESHIFT: c_uint = IOC_TYPESHIFT + IOC_TYPEBITS;
    const IOC_DIRSHIFT: c_uint = IOC_SIZESHIFT + IOC_SIZEBITS;

    (dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

unsafe fn _IOW<T>(ty: c_uint, nr: c_uint) -> c_uint {
    const IOC_WRITE: c_uint = 1;
    _IOC(IOC_WRITE, ty, nr, size_of::<T>() as c_uint)
}

unsafe fn _IOR<T>(ty: c_uint, nr: c_uint) -> c_uint {
    const IOC_READ: c_uint = 2;
    _IOC(IOC_READ, ty, nr, size_of::<T>() as c_uint)
}

static mut SNDRV_TIMER_IOCTL_PVERSION: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_TREAD_OLD: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_TREAD64: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_GINFO: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_GSTATUS: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_SELECT: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_PARAMS: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_START: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_START_OLD: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_STOP: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_STOP_OLD: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_CONTINUE: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_CONTINUE_OLD: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_PAUSE: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_PAUSE_OLD: c_uint = 0;
static mut SNDRV_TIMER_IOCTL_NEXT_DEVICE: c_uint = 0;

unsafe fn snd_timer_user_gparams_compat(
    file: *mut file,
    user: *mut snd_timer_gparams32,
) -> c_int {
    let mut gparams: snd_timer_gparams = core::mem::zeroed();

    if copy_from_user(
        (&mut gparams.tid as *mut snd_timer_id).cast::<c_void>(),
        (&raw const (*user).tid).cast::<c_void>(),
        size_of::<snd_timer_id>(),
    ) != 0
        || get_user_u32(
            (&mut gparams.period_num as *mut c_ulong).cast::<u32>(),
            &raw const (*user).period_num,
        ) != 0
        || get_user_u32(
            (&mut gparams.period_den as *mut c_ulong).cast::<u32>(),
            &raw const (*user).period_den,
        ) != 0
    {
        return -(EFAULT as c_int);
    }

    timer_set_gparams(&mut gparams)
}

unsafe fn snd_timer_user_info_compat(file: *mut file, _info: *mut snd_timer_info32) -> c_int {
    let mut tu: *mut snd_timer_user;
    let mut info: snd_timer_info32;

    tu = (*file).private_data.cast::<snd_timer_user>();
    if (*tu).timeri.is_null() {
        return -(EBADFD as c_int);
    }

    /*
     * Original C uses:
     * struct snd_timer *t __free(snd_timeri_timer) =
     *         snd_timeri_timer_get(tu->timeri);
     */
    let t: *mut snd_timer = snd_timeri_timer_get((*tu).timeri);
    if t.is_null() {
        return -(EBADFD as c_int);
    }
    info = core::mem::zeroed();
    memset(
        (&mut info as *mut snd_timer_info32).cast::<c_void>(),
        0,
        size_of::<snd_timer_info32>(),
    );
    info.card = if !(*t).card.is_null() {
        (*(*t).card).number
    } else {
        -1
    };
    if ((*t).hw.flags & SNDRV_TIMER_HW_SLAVE) != 0 {
        info.flags |= SNDRV_TIMER_FLG_SLAVE;
    }
    strscpy(
        info.id.as_mut_ptr(),
        (*t).id.as_ptr(),
        size_of::<[u8; 64]>(),
    );
    strscpy(
        info.name.as_mut_ptr(),
        (*t).name.as_ptr(),
        size_of::<[u8; 80]>(),
    );
    info.resolution = (*t).hw.resolution;
    if copy_to_user(
        _info.cast::<c_void>(),
        (&info as *const snd_timer_info32).cast::<c_void>(),
        size_of::<snd_timer_info32>(),
    ) != 0
    {
        return -(EFAULT as c_int);
    }
    0
}

unsafe fn SNDRV_TIMER_IOCTL_GPARAMS32() -> c_uint {
    _IOW::<snd_timer_gparams32>('T' as c_uint, 0x04)
}

unsafe fn SNDRV_TIMER_IOCTL_INFO32() -> c_uint {
    _IOR::<snd_timer_info32>('T' as c_uint, 0x11)
}

unsafe fn SNDRV_TIMER_IOCTL_STATUS_COMPAT32() -> c_uint {
    _IOW::<snd_timer_status32>('T' as c_uint, 0x14)
}

unsafe fn SNDRV_TIMER_IOCTL_STATUS_COMPAT64() -> c_uint {
    _IOW::<snd_timer_status64>('T' as c_uint, 0x14)
}

unsafe fn __snd_timer_user_ioctl_compat(
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_long {
    let argp: *mut c_void = compat_ptr(arg);

    if cmd == SNDRV_TIMER_IOCTL_PVERSION
        || cmd == SNDRV_TIMER_IOCTL_TREAD_OLD
        || cmd == SNDRV_TIMER_IOCTL_TREAD64
        || cmd == SNDRV_TIMER_IOCTL_GINFO
        || cmd == SNDRV_TIMER_IOCTL_GSTATUS
        || cmd == SNDRV_TIMER_IOCTL_SELECT
        || cmd == SNDRV_TIMER_IOCTL_PARAMS
        || cmd == SNDRV_TIMER_IOCTL_START
        || cmd == SNDRV_TIMER_IOCTL_START_OLD
        || cmd == SNDRV_TIMER_IOCTL_STOP
        || cmd == SNDRV_TIMER_IOCTL_STOP_OLD
        || cmd == SNDRV_TIMER_IOCTL_CONTINUE
        || cmd == SNDRV_TIMER_IOCTL_CONTINUE_OLD
        || cmd == SNDRV_TIMER_IOCTL_PAUSE
        || cmd == SNDRV_TIMER_IOCTL_PAUSE_OLD
        || cmd == SNDRV_TIMER_IOCTL_NEXT_DEVICE
    {
        return __snd_timer_user_ioctl(file, cmd, argp as c_ulong, true);
    }

    if cmd == SNDRV_TIMER_IOCTL_GPARAMS32() {
        return snd_timer_user_gparams_compat(file, argp.cast::<snd_timer_gparams32>()) as c_long;
    }
    if cmd == SNDRV_TIMER_IOCTL_INFO32() {
        return snd_timer_user_info_compat(file, argp.cast::<snd_timer_info32>()) as c_long;
    }
    if cmd == SNDRV_TIMER_IOCTL_STATUS_COMPAT32() {
        return snd_timer_user_status32(file, argp);
    }
    if cmd == SNDRV_TIMER_IOCTL_STATUS_COMPAT64() {
        return snd_timer_user_status64(file, argp);
    }

    -(ENOIOCTLCMD as c_long)
}

unsafe fn snd_timer_user_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let tu: *mut snd_timer_user = (*file).private_data.cast::<snd_timer_user>();

    /*
     * Original C uses guard(mutex)(&tu->ioctl_lock), which locks for the
     * lexical scope of this function.
     */
    mutex_lock(&mut (*tu).ioctl_lock);
    let ret = __snd_timer_user_ioctl_compat(file, cmd, arg);
    mutex_unlock(&mut (*tu).ioctl_lock);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
