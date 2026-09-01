// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Advanced Linux Sound Architecture
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// Original C dependencies:
// linux/init.h, linux/slab.h, linux/time.h, linux/device.h, linux/module.h,
// linux/debugfs.h, sound/core.h, sound/minors.h, sound/info.h,
// sound/control.h, sound/initval.h, linux/kmod.h, linux/mutex.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static CONFIG_SND_MAJOR: c_int;
    static SNDRV_OS_MINORS: usize;
    static SNDRV_MINOR_SEQUENCER: c_int;
    static SNDRV_MINOR_TIMER: c_int;
    static SNDRV_MINOR_CONTROL: c_int;
    static SNDRV_MINOR_GLOBAL: c_int;
    static SNDRV_MINOR_HWDEP: c_int;
    static SNDRV_MINOR_COMPRESS: c_int;
    static SNDRV_DEVICE_TYPE_CONTROL: c_int;
    static SNDRV_DEVICE_TYPE_HWDEP: c_int;
    static SNDRV_DEVICE_TYPE_RAWMIDI: c_int;
    static SNDRV_DEVICE_TYPE_PCM_PLAYBACK: c_int;
    static SNDRV_DEVICE_TYPE_PCM_CAPTURE: c_int;
    static SNDRV_DEVICE_TYPE_SEQUENCER: c_int;
    static SNDRV_DEVICE_TYPE_TIMER: c_int;
    static SNDRV_DEVICE_TYPE_COMPRESS: c_int;
    static THIS_MODULE: *mut module;

    fn snd_card_locked(card: c_int) -> c_int;
    fn request_module(name: *const c_char, ...) -> c_int;
    fn get_device(dev: *mut device) -> *mut device;
    fn snd_card_ref(card: c_int) -> *mut snd_card;
    fn snd_card_unref(card: *mut snd_card);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn iminor(inode: *mut inode) -> c_uint;
    fn fops_get(fops: *const file_operations) -> *const file_operations;
    fn replace_fops(file: *mut file, fops: *const file_operations);
    fn noop_llseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn snd_BUG_ON(condition: bool) -> c_int;
    fn SNDRV_MINOR_DEVICE(minor: c_uint) -> c_int;
    fn SNDRV_MINOR_CARD(minor: c_uint) -> c_int;
    fn SNDRV_MINOR(card: c_int, device: c_int) -> c_int;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn MKDEV(major: c_int, minor: c_int) -> dev_t;
    fn device_add(dev: *mut device) -> c_int;
    fn device_del(dev: *mut device);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_create_module_entry(
        module: *mut module,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn register_chrdev(major: c_int, name: *const c_char, fops: *const file_operations) -> c_int;
    fn unregister_chrdev(major: c_int, name: *const c_char);
    fn snd_info_init() -> c_int;
    fn snd_info_done();
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
}

type dev_t = u64;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_uint = 0;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub devt: dev_t,
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
    pub card_dev: device,
}

#[repr(C)]
pub struct file {
    pub f_op: *const file_operations,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, i64, c_int) -> i64>,
}

#[repr(C)]
pub struct snd_minor {
    pub type_: c_int,
    pub card: c_int,
    pub device: c_int,
    pub f_ops: *const file_operations,
    pub private_data: *mut c_void,
    pub card_ptr: *mut snd_card,
    pub dev: *mut device,
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
pub union snd_info_entry_c {
    pub text: snd_info_entry_text,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

static mut major: c_int = unsafe { CONFIG_SND_MAJOR };
#[no_mangle]
pub static mut snd_major: c_int = 0;

static mut cards_limit: c_int = 1;

// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("Advanced Linux Sound Architecture driver for soundcards.");
// MODULE_LICENSE("GPL");
// module_param(major, int, 0444);
// MODULE_PARM_DESC(major, "Major # for sound driver.");
// module_param(cards_limit, int, 0444);
// MODULE_PARM_DESC(cards_limit, "Count of auto-loadable soundcards.");
// MODULE_ALIAS_CHARDEV_MAJOR(CONFIG_SND_MAJOR);

/* this one holds the actual max. card number currently available.
 * as default, it's identical with cards_limit option.  when more
 * modules are loaded manually, this limit number increases, too.
 */
#[no_mangle]
pub static mut snd_ecards_limit: c_int = 0;

// CONFIG_SND_DEBUG
#[no_mangle]
pub static mut sound_debugfs_root: *mut dentry = core::ptr::null_mut();

static mut snd_minors: [*mut snd_minor; 256] = [core::ptr::null_mut(); 256];
static mut sound_mutex: mutex = mutex { _private: [] };

/**
 * snd_request_card - try to load the card module
 * @card: the card number
 *
 * Tries to load the module "snd-card-X" for the given card number
 * via request_module.  Returns immediately if already loaded.
 */
// CONFIG_MODULES
#[no_mangle]
pub unsafe extern "C" fn snd_request_card(card: c_int) {
    if snd_card_locked(card) != 0 {
        return;
    }
    if card < 0 || card >= cards_limit {
        return;
    }
    request_module(c"snd-card-%i".as_ptr(), card);
}

// CONFIG_MODULES
unsafe fn snd_request_other(minor: c_int) {
    let str_: *const c_char;

    match minor {
        x if x == SNDRV_MINOR_SEQUENCER => str_ = c"snd-seq".as_ptr(),
        x if x == SNDRV_MINOR_TIMER => str_ = c"snd-timer".as_ptr(),
        _ => return,
    }
    request_module(str_);
}

/**
 * snd_lookup_minor_data - get user data of a registered device
 * @minor: the minor number
 * @type: device type (SNDRV_DEVICE_TYPE_XXX)
 *
 * Checks that a minor device with the specified type is registered, and returns
 * its user data pointer.
 *
 * This function increments the reference counter of the card instance
 * if an associated instance with the given minor number and type is found.
 * The caller must call snd_card_unref() appropriately later.
 *
 * Return: The user data pointer if the specified device is found. %NULL
 * otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_lookup_minor_data(minor: c_uint, type_: c_int) -> *mut c_void {
    let mreg: *mut snd_minor;
    let private_data: *mut c_void;

    if minor as usize >= snd_minors.len() {
        return core::ptr::null_mut();
    }
    mutex_lock(&raw mut sound_mutex);
    mreg = snd_minors[minor as usize];
    if !mreg.is_null() && (*mreg).type_ == type_ {
        private_data = (*mreg).private_data;
        if !private_data.is_null() && !(*mreg).card_ptr.is_null() {
            get_device(&mut (*(*mreg).card_ptr).card_dev);
        }
    } else {
        private_data = core::ptr::null_mut();
    }
    mutex_unlock(&raw mut sound_mutex);
    private_data
}

// CONFIG_MODULES
unsafe fn autoload_device(minor: c_uint) -> *mut snd_minor {
    let dev: c_int;
    mutex_unlock(&raw mut sound_mutex); /* release lock temporarily */
    dev = SNDRV_MINOR_DEVICE(minor);
    if dev == SNDRV_MINOR_CONTROL {
        /* /dev/aloadC? */
        let card = SNDRV_MINOR_CARD(minor);
        let ref_ = snd_card_ref(card);
        if ref_.is_null() {
            snd_request_card(card);
        } else {
            snd_card_unref(ref_);
        }
    } else if dev == SNDRV_MINOR_GLOBAL {
        /* /dev/aloadSEQ */
        snd_request_other(minor as c_int);
    }
    mutex_lock(&raw mut sound_mutex); /* reacquire lock */
    snd_minors[minor as usize]
}

unsafe extern "C" fn snd_open(inode: *mut inode, file: *mut file) -> c_int {
    let minor = iminor(inode);
    let mut mptr: *mut snd_minor = core::ptr::null_mut();
    let new_fops: *const file_operations;
    let mut err: c_int = 0;

    if minor as usize >= snd_minors.len() {
        return -ENODEV;
    }
    mutex_lock(&raw mut sound_mutex);
    mptr = snd_minors[minor as usize];
    if mptr.is_null() {
        mptr = autoload_device(minor);
        if mptr.is_null() {
            mutex_unlock(&raw mut sound_mutex);
            return -ENODEV;
        }
    }
    new_fops = fops_get((*mptr).f_ops);
    mutex_unlock(&raw mut sound_mutex);
    if new_fops.is_null() {
        return -ENODEV;
    }
    replace_fops(file, new_fops);

    if let Some(open) = (*(*file).f_op).open {
        err = open(inode, file);
    }
    err
}

static snd_fops: file_operations = file_operations {
    owner: unsafe { THIS_MODULE },
    open: Some(snd_open),
    llseek: Some(noop_llseek),
};

// CONFIG_SND_DYNAMIC_MINORS version preserved as conditional intent:
// static int snd_find_free_minor(int type, struct snd_card *card, int dev)
// {
//     int minor;
//
//     /* static minors for module auto loading */
//     if (type == SNDRV_DEVICE_TYPE_SEQUENCER)
//         return SNDRV_MINOR_SEQUENCER;
//     if (type == SNDRV_DEVICE_TYPE_TIMER)
//         return SNDRV_MINOR_TIMER;
//
//     for (minor = 0; minor < ARRAY_SIZE(snd_minors); ++minor) {
//         /* skip static minors still used for module auto loading */
//         if (SNDRV_MINOR_DEVICE(minor) == SNDRV_MINOR_CONTROL)
//             continue;
//         if (minor == SNDRV_MINOR_SEQUENCER ||
//             minor == SNDRV_MINOR_TIMER)
//             continue;
//         if (!snd_minors[minor])
//             return minor;
//     }
//     return -EBUSY;
// }

unsafe fn snd_find_free_minor(type_: c_int, card: *mut snd_card, dev: c_int) -> c_int {
    let minor: c_int;

    match type_ {
        x if x == SNDRV_DEVICE_TYPE_SEQUENCER || x == SNDRV_DEVICE_TYPE_TIMER => {
            minor = type_;
        }
        x if x == SNDRV_DEVICE_TYPE_CONTROL => {
            if snd_BUG_ON(card.is_null()) != 0 {
                return -EINVAL;
            }
            minor = SNDRV_MINOR((*card).number, type_);
        }
        x if x == SNDRV_DEVICE_TYPE_HWDEP
            || x == SNDRV_DEVICE_TYPE_RAWMIDI
            || x == SNDRV_DEVICE_TYPE_PCM_PLAYBACK
            || x == SNDRV_DEVICE_TYPE_PCM_CAPTURE =>
        {
            if snd_BUG_ON(card.is_null()) != 0 {
                return -EINVAL;
            }
            minor = SNDRV_MINOR((*card).number, type_ + dev);
        }
        x if x == SNDRV_DEVICE_TYPE_COMPRESS => {
            if snd_BUG_ON(card.is_null()) != 0 {
                return -EINVAL;
            }
            if dev < 0 || dev >= SNDRV_MINOR_HWDEP - SNDRV_MINOR_COMPRESS {
                return -EINVAL;
            }
            minor = SNDRV_MINOR((*card).number, type_ + dev);
        }
        _ => return -EINVAL,
    }
    if snd_BUG_ON(minor < 0 || minor >= SNDRV_OS_MINORS as c_int) != 0 {
        return -EINVAL;
    }
    if !snd_minors[minor as usize].is_null() {
        return -EBUSY;
    }
    minor
}

/**
 * snd_register_device - Register the ALSA device file for the card
 * @type: the device type, SNDRV_DEVICE_TYPE_XXX
 * @card: the card instance
 * @dev: the device index
 * @f_ops: the file operations
 * @private_data: user pointer for f_ops->open()
 * @device: the device to register
 *
 * Registers an ALSA device file for the given card.
 * The operators have to be set in reg parameter.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_register_device(
    type_: c_int,
    card: *mut snd_card,
    dev: c_int,
    f_ops: *const file_operations,
    private_data: *mut c_void,
    device: *mut device,
) -> c_int {
    let minor: c_int;
    let mut err: c_int = 0;
    let preg: *mut snd_minor;

    if snd_BUG_ON(device.is_null()) != 0 {
        return -EINVAL;
    }

    preg = kmalloc(core::mem::size_of::<snd_minor>(), GFP_KERNEL) as *mut snd_minor;
    if preg.is_null() {
        return -ENOMEM;
    }
    (*preg).type_ = type_;
    (*preg).card = if !card.is_null() { (*card).number } else { -1 };
    (*preg).device = dev;
    (*preg).f_ops = f_ops;
    (*preg).private_data = private_data;
    (*preg).card_ptr = card;
    mutex_lock(&raw mut sound_mutex);
    minor = snd_find_free_minor(type_, card, dev);
    if minor < 0 {
        err = minor;
    } else {
        (*preg).dev = device;
        (*device).devt = MKDEV(major, minor);
        err = device_add(device);
        if err >= 0 {
            snd_minors[minor as usize] = preg;
        }
    }
    mutex_unlock(&raw mut sound_mutex);
    if err < 0 {
        kfree(preg as *mut c_void);
    }
    err
}

/**
 * snd_unregister_device - unregister the device on the given card
 * @dev: the device instance
 *
 * Unregisters the device file already registered via
 * snd_register_device().
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_unregister_device(dev: *mut device) -> c_int {
    let mut minor: usize;
    let mut preg: *mut snd_minor;

    mutex_lock(&raw mut sound_mutex);
    minor = 0;
    while minor < snd_minors.len() {
        preg = snd_minors[minor];
        if !preg.is_null() && (*preg).dev == dev {
            snd_minors[minor] = core::ptr::null_mut();
            device_del(dev);
            kfree(preg as *mut c_void);
            break;
        }
        minor += 1;
    }
    mutex_unlock(&raw mut sound_mutex);
    if minor >= snd_minors.len() {
        return -ENOENT;
    }
    0
}

// CONFIG_SND_PROC_FS
/*
 *  INFO PART
 */
unsafe fn snd_device_type_name(type_: c_int) -> *const c_char {
    match type_ {
        x if x == SNDRV_DEVICE_TYPE_CONTROL => c"control".as_ptr(),
        x if x == SNDRV_DEVICE_TYPE_HWDEP => c"hardware dependent".as_ptr(),
        x if x == SNDRV_DEVICE_TYPE_RAWMIDI => c"raw midi".as_ptr(),
        x if x == SNDRV_DEVICE_TYPE_PCM_PLAYBACK => c"digital audio playback".as_ptr(),
        x if x == SNDRV_DEVICE_TYPE_PCM_CAPTURE => c"digital audio capture".as_ptr(),
        x if x == SNDRV_DEVICE_TYPE_SEQUENCER => c"sequencer".as_ptr(),
        x if x == SNDRV_DEVICE_TYPE_TIMER => c"timer".as_ptr(),
        x if x == SNDRV_DEVICE_TYPE_COMPRESS => c"compress".as_ptr(),
        _ => c"?".as_ptr(),
    }
}

unsafe extern "C" fn snd_minor_info_read(
    _entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let mut minor: usize;
    let mut mptr: *mut snd_minor;

    mutex_lock(&raw mut sound_mutex);
    minor = 0;
    while minor < snd_minors.len() {
        mptr = snd_minors[minor];
        if !mptr.is_null() {
            if (*mptr).card >= 0 {
                if (*mptr).device >= 0 {
                    snd_iprintf(
                        buffer,
                        c"%3i: [%2i-%2i]: %s\n".as_ptr(),
                        minor as c_int,
                        (*mptr).card,
                        (*mptr).device,
                        snd_device_type_name((*mptr).type_),
                    );
                } else {
                    snd_iprintf(
                        buffer,
                        c"%3i: [%2i]   : %s\n".as_ptr(),
                        minor as c_int,
                        (*mptr).card,
                        snd_device_type_name((*mptr).type_),
                    );
                }
            } else {
                snd_iprintf(
                    buffer,
                    c"%3i:        : %s\n".as_ptr(),
                    minor as c_int,
                    snd_device_type_name((*mptr).type_),
                );
            }
        }
        minor += 1;
    }
    mutex_unlock(&raw mut sound_mutex);
}

#[no_mangle]
pub unsafe extern "C" fn snd_minor_info_init() -> c_int {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_module_entry(THIS_MODULE, c"devices".as_ptr(), core::ptr::null_mut());
    if entry.is_null() {
        return -ENOMEM;
    }
    (*entry).c.text.read = Some(snd_minor_info_read);
    snd_info_register(entry) /* freed in error path */
}

/*
 *  INIT PART
 */

unsafe extern "C" fn alsa_sound_init() -> c_int {
    let mut err: c_int;

    snd_major = major;
    snd_ecards_limit = cards_limit;

    err = register_chrdev(major, c"alsa".as_ptr(), &snd_fops);
    if err < 0 {
        pr_err(
            c"ALSA core: unable to register native major device number %d\n".as_ptr(),
            major,
        );
        return err;
    }

    err = snd_info_init();
    if err < 0 {
        unregister_chrdev(major, c"alsa".as_ptr());
        return err;
    }

    // CONFIG_SND_DEBUG
    sound_debugfs_root = debugfs_create_dir(c"sound".as_ptr(), core::ptr::null_mut());
    // !MODULE
    pr_info(c"Advanced Linux Sound Architecture Driver Initialized.\n".as_ptr());
    0
}

unsafe extern "C" fn alsa_sound_exit() {
    // CONFIG_SND_DEBUG
    debugfs_remove(sound_debugfs_root);
    snd_info_done();
    unregister_chrdev(major, c"alsa".as_ptr());
}

// subsys_initcall(alsa_sound_init);
// module_exit(alsa_sound_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
