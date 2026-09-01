// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Information interface for ALSA driver
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/*
 *  OSS compatible part
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of_val;
use core::ptr;

pub const SNDRV_CARDS: usize = 32;
pub const SNDRV_OSS_INFO_DEV_AUDIO: c_int = 0;
pub const SNDRV_OSS_INFO_DEV_SYNTH: c_int = 1;
pub const SNDRV_OSS_INFO_DEV_MIDI: c_int = 2;
pub const SNDRV_OSS_INFO_DEV_TIMERS: c_int = 3;
pub const SNDRV_OSS_INFO_DEV_MIXERS: c_int = 4;
pub const SNDRV_OSS_INFO_DEV_COUNT: usize = 5;

pub const ENXIO: c_int = 6;
pub const ENOMEM: c_int = 12;
pub const GFP_KERNEL: c_int = 0;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry {
    pub c: snd_info_entry_c,
}

#[repr(C)]
pub struct snd_info_entry_c {
    pub text: snd_info_entry_text,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct new_utsname {
    pub sysname: *const c_char,
    pub nodename: *const c_char,
    pub release: *const c_char,
    pub version: *const c_char,
    pub machine: *const c_char,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut snd_oss_root: *mut snd_info_entry;

    fn kfree(ptr: *const c_void);
    fn kstrdup(s: *const c_char, flags: c_int) -> *mut c_char;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn init_utsname() -> *mut new_utsname;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...) -> c_int;
    fn snd_card_info_read_oss(buffer: *mut snd_info_buffer);
    fn snd_info_create_module_entry(
        module: *mut c_void,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
}

/* DEFINE_MUTEX(strings); */
static mut strings: mutex = mutex { _private: [] };
static mut snd_sndstat_strings: [[*mut c_char; SNDRV_OSS_INFO_DEV_COUNT]; SNDRV_CARDS] =
    [[ptr::null_mut(); SNDRV_OSS_INFO_DEV_COUNT]; SNDRV_CARDS];

#[inline]
unsafe fn snd_BUG_ON(condition: bool) -> bool {
    condition
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_oss_info_register(
    dev: c_int,
    num: c_int,
    string: *mut c_char,
) -> c_int {
    let x: *mut c_char;

    if unsafe { snd_BUG_ON(dev < 0 || dev >= SNDRV_OSS_INFO_DEV_COUNT as c_int) } {
        return -ENXIO;
    }
    if unsafe { snd_BUG_ON(num < 0 || num >= SNDRV_CARDS as c_int) } {
        return -ENXIO;
    }
    unsafe { mutex_lock(&raw mut strings) };
    if string.is_null() {
        x = unsafe { snd_sndstat_strings[num as usize][dev as usize] };
        unsafe { kfree(x as *const c_void) };
        x = ptr::null_mut();
    } else {
        x = unsafe { kstrdup(string, GFP_KERNEL) };
        if x.is_null() {
            unsafe { mutex_unlock(&raw mut strings) };
            return -ENOMEM;
        }
    }
    unsafe {
        snd_sndstat_strings[num as usize][dev as usize] = x;
        mutex_unlock(&raw mut strings);
    }
    0
}

/* EXPORT_SYMBOL(snd_oss_info_register); */

unsafe extern "C" fn snd_sndstat_show_strings(
    buf: *mut snd_info_buffer,
    id: *mut c_char,
    dev: c_int,
) -> c_int {
    let mut idx: c_int;
    let mut ok: c_int = -1;
    let mut str_: *mut c_char;

    unsafe { snd_iprintf(buf, c"\n%s:".as_ptr(), id) };
    unsafe { mutex_lock(&raw mut strings) };
    idx = 0;
    while idx < SNDRV_CARDS as c_int {
        str_ = unsafe { snd_sndstat_strings[idx as usize][dev as usize] };
        if !str_.is_null() {
            if ok < 0 {
                unsafe { snd_iprintf(buf, c"\n".as_ptr()) };
                ok += 1;
            }
            unsafe { snd_iprintf(buf, c"%i: %s\n".as_ptr(), idx, str_) };
        }
        idx += 1;
    }
    unsafe { mutex_unlock(&raw mut strings) };
    if ok < 0 {
        unsafe { snd_iprintf(buf, c" NOT ENABLED IN CONFIG\n".as_ptr()) };
    }
    ok
}

unsafe extern "C" fn snd_sndstat_proc_read(
    _entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    unsafe {
        snd_iprintf(
            buffer,
            c"Sound Driver:3.8.1a-980706 (ALSA emulation code)\n".as_ptr(),
        );
        snd_iprintf(
            buffer,
            c"Kernel: %s %s %s %s %s\n".as_ptr(),
            (*init_utsname()).sysname,
            (*init_utsname()).nodename,
            (*init_utsname()).release,
            (*init_utsname()).version,
            (*init_utsname()).machine,
        );
        snd_iprintf(buffer, c"Config options: 0\n".as_ptr());
        snd_iprintf(buffer, c"\nInstalled drivers: \n".as_ptr());
        snd_iprintf(buffer, c"Type 10: ALSA emulation\n".as_ptr());
        snd_iprintf(buffer, c"\nCard config: \n".as_ptr());
        snd_card_info_read_oss(buffer);
        snd_sndstat_show_strings(
            buffer,
            c"Audio devices".as_ptr() as *mut c_char,
            SNDRV_OSS_INFO_DEV_AUDIO,
        );
        snd_sndstat_show_strings(
            buffer,
            c"Synth devices".as_ptr() as *mut c_char,
            SNDRV_OSS_INFO_DEV_SYNTH,
        );
        snd_sndstat_show_strings(
            buffer,
            c"Midi devices".as_ptr() as *mut c_char,
            SNDRV_OSS_INFO_DEV_MIDI,
        );
        snd_sndstat_show_strings(
            buffer,
            c"Timers".as_ptr() as *mut c_char,
            SNDRV_OSS_INFO_DEV_TIMERS,
        );
        snd_sndstat_show_strings(
            buffer,
            c"Mixers".as_ptr() as *mut c_char,
            SNDRV_OSS_INFO_DEV_MIXERS,
        );
    }
}

/* __init */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_info_minor_register() -> c_int {
    let entry: *mut snd_info_entry;

    unsafe {
        memset(
            (&raw mut snd_sndstat_strings) as *mut c_void,
            0,
            size_of_val(&*(&raw const snd_sndstat_strings)),
        );
        entry = snd_info_create_module_entry(THIS_MODULE, c"sndstat".as_ptr(), snd_oss_root);
        if entry.is_null() {
            return -ENOMEM;
        }
        (*entry).c.text.read = Some(snd_sndstat_proc_read);
        snd_info_register(entry)
    } /* freed in error path */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
