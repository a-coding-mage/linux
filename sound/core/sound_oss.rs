// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Advanced Linux Sound Architecture
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const SNDRV_OSS_MINORS: usize = 256;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
    pub card_dev: device,
}

#[repr(C)]
pub struct snd_minor {
    pub type_: c_int,
    pub card: c_int,
    pub device: c_int,
    pub f_ops: *const file_operations,
    pub private_data: *mut c_void,
    pub card_ptr: *mut snd_card,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub union snd_info_entry_c {
    pub text: core::mem::ManuallyDrop<snd_info_entry_text>,
}

#[repr(C)]
pub struct snd_info_entry {
    pub c: snd_info_entry_c,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

static mut snd_oss_minors: [*mut snd_minor; SNDRV_OSS_MINORS] =
    [core::ptr::null_mut(); SNDRV_OSS_MINORS];
static mut sound_oss_mutex: mutex = mutex { _private: [] };

unsafe extern "C" {
    static SNDRV_OSS_DEVICE_TYPE_MIXER: c_int;
    static SNDRV_OSS_DEVICE_TYPE_SEQUENCER: c_int;
    static SNDRV_OSS_DEVICE_TYPE_MUSIC: c_int;
    static SNDRV_OSS_DEVICE_TYPE_PCM: c_int;
    static SNDRV_OSS_DEVICE_TYPE_MIDI: c_int;
    static SNDRV_OSS_DEVICE_TYPE_DMFM: c_int;
    static SNDRV_OSS_DEVICE_TYPE_SNDSTAT: c_int;

    static SNDRV_MINOR_OSS_MIXER1: c_int;
    static SNDRV_MINOR_OSS_MIXER: c_int;
    static SNDRV_MINOR_OSS_SEQUENCER: c_int;
    static SNDRV_MINOR_OSS_MUSIC: c_int;
    static SNDRV_MINOR_OSS_PCM1: c_int;
    static SNDRV_MINOR_OSS_PCM: c_int;
    static SNDRV_MINOR_OSS_MIDI1: c_int;
    static SNDRV_MINOR_OSS_MIDI: c_int;
    static SNDRV_MINOR_OSS_DMFM: c_int;
    static SNDRV_MINOR_OSS_SNDSTAT: c_int;
    static SNDRV_MINOR_OSS_AUDIO: c_int;
    static SNDRV_MINOR_OSS_DMMIDI: c_int;
    static SNDRV_MINOR_OSS_DMMIDI1: c_int;
    static SNDRV_MINOR_OSS_DEVICES: c_int;

    static EINVAL: c_int;
    static ENOMEM: c_int;
    static EBUSY: c_int;
    static ENOENT: c_int;

    static THIS_MODULE: *mut c_void;
    static snd_oss_root: *mut c_void;

    fn SNDRV_MINOR_OSS(card: c_int, dev: c_int) -> c_int;
    fn SNDRV_MINOR_OSS_CARD(minor: c_int) -> c_int;
    fn SNDRV_MINOR_OSS_DEVICE(minor: c_int) -> c_int;

    fn snd_BUG_ON(condition: bool) -> c_int;
    fn get_device(dev: *mut device) -> *mut device;
    fn snd_card_get_device_link(card: *mut snd_card) -> *mut device;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn register_sound_special_device(
        f_ops: *const file_operations,
        minor: c_int,
        device: *mut device,
    ) -> c_int;
    fn unregister_sound_special(minor: c_int);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn snd_info_create_module_entry(
        module: *mut c_void,
        name: *const c_char,
        parent: *mut c_void,
    ) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...) -> c_int;
}

const GFP_KERNEL: c_uint = 0;

unsafe fn kmalloc_obj_snd_minor() -> *mut snd_minor {
    unsafe { kmalloc(core::mem::size_of::<snd_minor>(), GFP_KERNEL) as *mut snd_minor }
}

/* NOTE: This function increments the refcount of the associated card like
 * snd_lookup_minor_data(); the caller must call snd_card_unref() appropriately
 */
#[no_mangle]
pub unsafe extern "C" fn snd_lookup_oss_minor_data(
    minor: c_uint,
    type_: c_int,
) -> *mut c_void {
    let mreg: *mut snd_minor;
    let private_data: *mut c_void;

    if minor as usize >= SNDRV_OSS_MINORS {
        return core::ptr::null_mut();
    }
    unsafe {
        mutex_lock(core::ptr::addr_of_mut!(sound_oss_mutex));
        mreg = snd_oss_minors[minor as usize];
        if !mreg.is_null() && (*mreg).type_ == type_ {
            private_data = (*mreg).private_data;
            if !private_data.is_null() && !(*mreg).card_ptr.is_null() {
                get_device(core::ptr::addr_of_mut!((*(*mreg).card_ptr).card_dev));
            }
        } else {
            private_data = core::ptr::null_mut();
        }
        mutex_unlock(core::ptr::addr_of_mut!(sound_oss_mutex));
    }
    private_data
}
/* EXPORT_SYMBOL(snd_lookup_oss_minor_data); */

unsafe fn snd_oss_kernel_minor(type_: c_int, card: *mut snd_card, dev: c_int) -> c_int {
    let minor: c_int;

    unsafe {
        if type_ == SNDRV_OSS_DEVICE_TYPE_MIXER {
            if snd_BUG_ON(card.is_null() || dev < 0 || dev > 1) != 0 {
                return -EINVAL;
            }
            minor = SNDRV_MINOR_OSS(
                (*card).number,
                if dev != 0 {
                    SNDRV_MINOR_OSS_MIXER1
                } else {
                    SNDRV_MINOR_OSS_MIXER
                },
            );
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_SEQUENCER {
            minor = SNDRV_MINOR_OSS_SEQUENCER;
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_MUSIC {
            minor = SNDRV_MINOR_OSS_MUSIC;
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_PCM {
            if snd_BUG_ON(card.is_null() || dev < 0 || dev > 1) != 0 {
                return -EINVAL;
            }
            minor = SNDRV_MINOR_OSS(
                (*card).number,
                if dev != 0 {
                    SNDRV_MINOR_OSS_PCM1
                } else {
                    SNDRV_MINOR_OSS_PCM
                },
            );
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_MIDI {
            if snd_BUG_ON(card.is_null() || dev < 0 || dev > 1) != 0 {
                return -EINVAL;
            }
            minor = SNDRV_MINOR_OSS(
                (*card).number,
                if dev != 0 {
                    SNDRV_MINOR_OSS_MIDI1
                } else {
                    SNDRV_MINOR_OSS_MIDI
                },
            );
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_DMFM {
            minor = SNDRV_MINOR_OSS((*card).number, SNDRV_MINOR_OSS_DMFM);
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_SNDSTAT {
            minor = SNDRV_MINOR_OSS_SNDSTAT;
        } else {
            return -EINVAL;
        }
        if minor < 0 || minor >= SNDRV_OSS_MINORS as c_int {
            return -EINVAL;
        }
    }
    minor
}

#[no_mangle]
pub unsafe extern "C" fn snd_register_oss_device(
    type_: c_int,
    card: *mut snd_card,
    dev: c_int,
    f_ops: *const file_operations,
    private_data: *mut c_void,
) -> c_int {
    let minor = unsafe { snd_oss_kernel_minor(type_, card, dev) };
    let minor_unit: c_int;
    let preg: *mut snd_minor;
    let cidx = unsafe { SNDRV_MINOR_OSS_CARD(minor) };
    let mut track2: c_int = -1;
    let mut register1: c_int = -1;
    let mut register2: c_int = -1;
    let carddev = unsafe { snd_card_get_device_link(card) };

    unsafe {
        if !card.is_null() && (*card).number >= SNDRV_MINOR_OSS_DEVICES {
            return 0; /* ignore silently */
        }
        if minor < 0 {
            return minor;
        }
        preg = kmalloc_obj_snd_minor();
        if preg.is_null() {
            return -ENOMEM;
        }
        (*preg).type_ = type_;
        (*preg).card = if !card.is_null() { (*card).number } else { -1 };
        (*preg).device = dev;
        (*preg).f_ops = f_ops;
        (*preg).private_data = private_data;
        (*preg).card_ptr = card;
        mutex_lock(core::ptr::addr_of_mut!(sound_oss_mutex));
        snd_oss_minors[minor as usize] = preg;
        minor_unit = SNDRV_MINOR_OSS_DEVICE(minor);
        if minor_unit == SNDRV_MINOR_OSS_PCM {
            track2 = SNDRV_MINOR_OSS(cidx, SNDRV_MINOR_OSS_AUDIO);
        } else if minor_unit == SNDRV_MINOR_OSS_MIDI {
            track2 = SNDRV_MINOR_OSS(cidx, SNDRV_MINOR_OSS_DMMIDI);
        } else if minor_unit == SNDRV_MINOR_OSS_MIDI1 {
            track2 = SNDRV_MINOR_OSS(cidx, SNDRV_MINOR_OSS_DMMIDI1);
        }
        register1 = register_sound_special_device(f_ops, minor, carddev);
        if register1 != minor {
            goto_end(register2, register1, minor, preg);
            mutex_unlock(core::ptr::addr_of_mut!(sound_oss_mutex));
            return -EBUSY;
        }
        if track2 >= 0 {
            register2 = register_sound_special_device(f_ops, track2, carddev);
            if register2 != track2 {
                goto_end(register2, register1, minor, preg);
                mutex_unlock(core::ptr::addr_of_mut!(sound_oss_mutex));
                return -EBUSY;
            }
            snd_oss_minors[track2 as usize] = preg;
        }
        mutex_unlock(core::ptr::addr_of_mut!(sound_oss_mutex));
    }
    return 0;

    unsafe fn goto_end(register2: c_int, register1: c_int, minor: c_int, preg: *mut snd_minor) {
        unsafe {
            if register2 >= 0 {
                unregister_sound_special(register2);
            }
            if register1 >= 0 {
                unregister_sound_special(register1);
            }
            snd_oss_minors[minor as usize] = core::ptr::null_mut();
            kfree(preg as *mut c_void);
        }
    }
}
/* EXPORT_SYMBOL(snd_register_oss_device); */

#[no_mangle]
pub unsafe extern "C" fn snd_unregister_oss_device(
    type_: c_int,
    card: *mut snd_card,
    dev: c_int,
) -> c_int {
    let minor = unsafe { snd_oss_kernel_minor(type_, card, dev) };
    let cidx = unsafe { SNDRV_MINOR_OSS_CARD(minor) };
    let mut track2: c_int = -1;
    let mptr: *mut snd_minor;

    unsafe {
        if !card.is_null() && (*card).number >= SNDRV_MINOR_OSS_DEVICES {
            return 0;
        }
        if minor < 0 {
            return minor;
        }
        mutex_lock(core::ptr::addr_of_mut!(sound_oss_mutex));
        mptr = snd_oss_minors[minor as usize];
        if mptr.is_null() {
            mutex_unlock(core::ptr::addr_of_mut!(sound_oss_mutex));
            return -ENOENT;
        }
        let minor_device = SNDRV_MINOR_OSS_DEVICE(minor);
        if minor_device == SNDRV_MINOR_OSS_PCM {
            track2 = SNDRV_MINOR_OSS(cidx, SNDRV_MINOR_OSS_AUDIO);
        } else if minor_device == SNDRV_MINOR_OSS_MIDI {
            track2 = SNDRV_MINOR_OSS(cidx, SNDRV_MINOR_OSS_DMMIDI);
        } else if minor_device == SNDRV_MINOR_OSS_MIDI1 {
            track2 = SNDRV_MINOR_OSS(cidx, SNDRV_MINOR_OSS_DMMIDI1);
        }
        if track2 >= 0 {
            snd_oss_minors[track2 as usize] = core::ptr::null_mut();
        }
        snd_oss_minors[minor as usize] = core::ptr::null_mut();
        mutex_unlock(core::ptr::addr_of_mut!(sound_oss_mutex));

        /* call unregister_sound_special() outside sound_oss_mutex;
         * otherwise may deadlock, as it can trigger the release of a card
         */
        unregister_sound_special(minor);
        if track2 >= 0 {
            unregister_sound_special(track2);
        }

        kfree(mptr as *mut c_void);
    }
    0
}
/* EXPORT_SYMBOL(snd_unregister_oss_device); */

/*
 *  INFO PART
 */

/* CONFIG_SND_PROC_FS */
unsafe fn snd_oss_device_type_name(type_: c_int) -> *const c_char {
    unsafe {
        if type_ == SNDRV_OSS_DEVICE_TYPE_MIXER {
            c"mixer".as_ptr()
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_SEQUENCER
            || type_ == SNDRV_OSS_DEVICE_TYPE_MUSIC
        {
            c"sequencer".as_ptr()
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_PCM {
            c"digital audio".as_ptr()
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_MIDI {
            c"raw midi".as_ptr()
        } else if type_ == SNDRV_OSS_DEVICE_TYPE_DMFM {
            c"hardware dependent".as_ptr()
        } else {
            c"?".as_ptr()
        }
    }
}

unsafe extern "C" fn snd_minor_info_oss_read(
    _entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let mut minor: c_int;
    let mut mptr: *mut snd_minor;

    unsafe {
        mutex_lock(core::ptr::addr_of_mut!(sound_oss_mutex));
        minor = 0;
        while minor < SNDRV_OSS_MINORS as c_int {
            mptr = snd_oss_minors[minor as usize];
            if !mptr.is_null() {
                if (*mptr).card >= 0 {
                    snd_iprintf(
                        buffer,
                        c"%3i: [%i-%2i]: %s\n".as_ptr(),
                        minor,
                        (*mptr).card,
                        (*mptr).device,
                        snd_oss_device_type_name((*mptr).type_),
                    );
                } else {
                    snd_iprintf(
                        buffer,
                        c"%3i:       : %s\n".as_ptr(),
                        minor,
                        snd_oss_device_type_name((*mptr).type_),
                    );
                }
            }
            minor += 1;
        }
        mutex_unlock(core::ptr::addr_of_mut!(sound_oss_mutex));
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_minor_info_oss_init() -> c_int {
    let entry: *mut snd_info_entry;

    unsafe {
        entry = snd_info_create_module_entry(THIS_MODULE, c"devices".as_ptr(), snd_oss_root);
        if entry.is_null() {
            return -ENOMEM;
        }
        (*entry).c.text = core::mem::ManuallyDrop::new(snd_info_entry_text {
            read: Some(snd_minor_info_oss_read),
        });
        snd_info_register(entry) /* freed in error path */
    }
}
/* #endif CONFIG_SND_PROC_FS */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
