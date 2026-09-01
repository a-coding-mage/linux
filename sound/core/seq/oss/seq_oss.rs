// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * registration of device and proc
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null_mut};

/*
 * Includes in the original C source:
 * <linux/init.h>, <linux/module.h>, <linux/mutex.h>, <linux/compat.h>,
 * <sound/core.h>, <sound/minors.h>, <sound/initval.h>,
 * "seq_oss_device.h", and "seq_oss_synth.h".
 *
 * Module metadata:
 * MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
 * MODULE_DESCRIPTION("OSS-compatible sequencer module");
 * MODULE_LICENSE("GPL");
 * MODULE_ALIAS_SNDRV_MINOR(SNDRV_MINOR_OSS_SEQUENCER);
 * MODULE_ALIAS_SNDRV_MINOR(SNDRV_MINOR_OSS_MUSIC);
 */

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type __poll_t = c_uint;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_oss_devinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_oss_reg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_seq_driver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub driver: snd_seq_driver_inner,
    pub id: c_int,
    pub argsize: size_t,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub union snd_info_entry_c {
    pub text: snd_info_entry_text,
}

#[repr(C)]
pub struct snd_info_entry {
    pub content: c_int,
    pub private_data: *mut c_void,
    pub c: snd_info_entry_c,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static KBUILD_MODNAME: [c_char; 0];
    static mut snd_seq_root: *mut snd_info_entry;

    static SNDRV_MINOR_OSS_MUSIC: c_int;
    static SNDRV_SEQ_OSS_MODE_MUSIC: c_int;
    static SNDRV_SEQ_OSS_MODE_SYNTH: c_int;
    static SNDCTL_SEQ_SYNC: c_uint;
    static ENXIO: c_int;
    static ERESTARTSYS: c_int;
    static EPOLLERR: __poll_t;
    static SNDRV_SEQ_DEV_ID_OSS: c_int;
    static SNDRV_OSS_DEVICE_TYPE_SEQUENCER: c_int;
    static SNDRV_OSS_DEVICE_TYPE_MUSIC: c_int;
    static ENOMEM: c_int;
    static SNDRV_INFO_CONTENT_TEXT: c_int;
    static SNDRV_SEQ_OSS_PROCNAME: *const c_char;
    static SNDRV_SEQ_OSS_VERSION_STR: *const c_char;

    fn snd_seq_oss_synth_probe(arg: *mut c_void) -> c_int;
    fn snd_seq_oss_synth_remove(arg: *mut c_void) -> c_int;
    fn snd_seq_oss_create_client() -> c_int;
    fn snd_seq_oss_delete_client();
    fn snd_seq_driver_register(driver: *mut snd_seq_driver) -> c_int;
    fn snd_seq_driver_unregister(driver: *mut snd_seq_driver);
    fn snd_seq_oss_synth_init();

    fn iminor(inode: *mut inode) -> c_int;
    fn snd_seq_oss_open(file: *mut file, level: c_int) -> c_int;
    fn snd_seq_oss_release(dp: *mut seq_oss_devinfo);
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn snd_seq_oss_read(dp: *mut seq_oss_devinfo, buf: *mut c_char, count: size_t) -> ssize_t;
    fn snd_seq_oss_write(
        dp: *mut seq_oss_devinfo,
        buf: *const c_char,
        count: size_t,
        file: *mut file,
    ) -> ssize_t;
    fn mutex_lock_interruptible(lock: *mut mutex) -> c_int;
    fn mutex_unlock(lock: *mut mutex);
    fn snd_seq_oss_ioctl(dp: *mut seq_oss_devinfo, cmd: c_uint, arg: c_ulong) -> c_long;
    fn compat_ptr(arg: c_ulong) -> *mut c_void;
    fn snd_seq_oss_poll(dp: *mut seq_oss_devinfo, file: *mut file, wait: *mut poll_table) -> __poll_t;
    fn noop_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;

    fn snd_register_oss_device(
        typ: c_int,
        card: *mut c_void,
        dev: c_int,
        f_ops: *const file_operations,
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_unregister_oss_device(typ: c_int, card: *mut c_void, dev: c_int) -> c_int;
    fn pr_err(fmt: *const c_char, ...);

    fn snd_info_create_module_entry(
        module: *mut module,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn snd_iprintf(buf: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_seq_oss_system_info_read(buf: *mut snd_info_buffer);
    fn snd_seq_oss_synth_info_read(buf: *mut snd_info_buffer);
    fn snd_seq_oss_midi_info_read(buf: *mut snd_info_buffer);
}

const ALSA_SEQ_OSS_CANT_REGISTER_DEVICE_SEQ: &[u8] =
    b"ALSA: seq_oss: can't register device seq\n\0";
const ALSA_SEQ_OSS_CANT_REGISTER_DEVICE_MUSIC: &[u8] =
    b"ALSA: seq_oss: can't register device music\n\0";
const ALSA_SEQ_OSS_ERROR_UNREGISTER_DEVICE_MUSIC: &[u8] =
    b"ALSA: seq_oss: error unregister device music\n\0";
const ALSA_SEQ_OSS_ERROR_UNREGISTER_DEVICE_SEQ: &[u8] =
    b"ALSA: seq_oss: error unregister device seq\n\0";
const OSS_SEQUENCER_EMULATION_VERSION_FMT: &[u8] =
    b"OSS sequencer emulation version %s\n\0";

/*
 * module interface
 */

static mut seq_oss_synth_driver: snd_seq_driver = snd_seq_driver {
    probe: Some(snd_seq_oss_synth_probe),
    remove: Some(snd_seq_oss_synth_remove),
    driver: snd_seq_driver_inner {
        name: unsafe { KBUILD_MODNAME.as_ptr() },
    },
    id: unsafe { SNDRV_SEQ_DEV_ID_OSS },
    argsize: size_of::<snd_seq_oss_reg>(),
};

unsafe extern "C" fn alsa_seq_oss_init() -> c_int {
    let mut rc: c_int;

    rc = register_device();
    if rc < 0 {
        return rc;
    }
    rc = register_proc();
    if rc < 0 {
        unregister_device();
        return rc;
    }
    rc = snd_seq_oss_create_client();
    if rc < 0 {
        unregister_proc();
        unregister_device();
        return rc;
    }

    rc = snd_seq_driver_register(addr_of_mut!(seq_oss_synth_driver));
    if rc < 0 {
        snd_seq_oss_delete_client();
        unregister_proc();
        unregister_device();
        return rc;
    }

    /* success */
    snd_seq_oss_synth_init();

    rc
}

unsafe extern "C" fn alsa_seq_oss_exit() {
    snd_seq_driver_unregister(addr_of_mut!(seq_oss_synth_driver));
    snd_seq_oss_delete_client();
    unregister_proc();
    unregister_device();
}

/* module_init(alsa_seq_oss_init) */
/* module_exit(alsa_seq_oss_exit) */

/*
 * ALSA minor device interface
 */

static mut register_mutex: mutex = mutex { _private: [] };

unsafe extern "C" fn odev_open(inode: *mut inode, file: *mut file) -> c_int {
    let level: c_int;

    if iminor(inode) == SNDRV_MINOR_OSS_MUSIC {
        level = SNDRV_SEQ_OSS_MODE_MUSIC;
    } else {
        level = SNDRV_SEQ_OSS_MODE_SYNTH;
    }

    mutex_lock_interruptible(addr_of_mut!(register_mutex));
    let rc = snd_seq_oss_open(file, level);
    mutex_unlock(addr_of_mut!(register_mutex));
    rc
}

unsafe extern "C" fn odev_release(_inode: *mut inode, file: *mut file) -> c_int {
    let dp: *mut seq_oss_devinfo;

    dp = (*file).private_data as *mut seq_oss_devinfo;
    if dp.is_null() {
        return 0;
    }

    mutex_lock_interruptible(addr_of_mut!(register_mutex));
    snd_seq_oss_release(dp);
    mutex_unlock(addr_of_mut!(register_mutex));
    0
}

unsafe extern "C" fn odev_read(
    file: *mut file,
    buf: *mut c_char,
    count: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    let dp: *mut seq_oss_devinfo;
    dp = (*file).private_data as *mut seq_oss_devinfo;
    if snd_BUG_ON(dp.is_null()) != 0 {
        return -ENXIO as ssize_t;
    }
    snd_seq_oss_read(dp, buf, count)
}

unsafe extern "C" fn odev_write(
    file: *mut file,
    buf: *const c_char,
    count: size_t,
    _offset: *mut loff_t,
) -> ssize_t {
    let dp: *mut seq_oss_devinfo;
    dp = (*file).private_data as *mut seq_oss_devinfo;
    if snd_BUG_ON(dp.is_null()) != 0 {
        return -ENXIO as ssize_t;
    }
    snd_seq_oss_write(dp, buf, count, file)
}

unsafe extern "C" fn odev_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let dp: *mut seq_oss_devinfo;
    let rc: c_long;

    dp = (*file).private_data as *mut seq_oss_devinfo;
    if snd_BUG_ON(dp.is_null()) != 0 {
        return -ENXIO as c_long;
    }

    if cmd != SNDCTL_SEQ_SYNC && mutex_lock_interruptible(addr_of_mut!(register_mutex)) != 0 {
        return -ERESTARTSYS as c_long;
    }
    rc = snd_seq_oss_ioctl(dp, cmd, arg);
    if cmd != SNDCTL_SEQ_SYNC {
        mutex_unlock(addr_of_mut!(register_mutex));
    }
    rc
}

#[cfg(CONFIG_COMPAT)]
unsafe extern "C" fn odev_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    odev_ioctl(file, cmd, compat_ptr(arg) as c_ulong)
}

#[cfg(CONFIG_COMPAT)]
const odev_ioctl_compat_ptr: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long> =
    Some(odev_ioctl_compat);

#[cfg(not(CONFIG_COMPAT))]
const odev_ioctl_compat_ptr: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long> =
    None;

unsafe extern "C" fn odev_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let dp: *mut seq_oss_devinfo;
    dp = (*file).private_data as *mut seq_oss_devinfo;
    if snd_BUG_ON(dp.is_null()) != 0 {
        return EPOLLERR;
    }
    snd_seq_oss_poll(dp, file, wait)
}

/*
 * registration of sequencer minor device
 */

static seq_oss_f_ops: file_operations = file_operations {
    owner: unsafe { THIS_MODULE },
    read: Some(odev_read),
    write: Some(odev_write),
    open: Some(odev_open),
    release: Some(odev_release),
    poll: Some(odev_poll),
    unlocked_ioctl: Some(odev_ioctl),
    compat_ioctl: odev_ioctl_compat_ptr,
    llseek: Some(noop_llseek),
};

unsafe extern "C" fn register_device() -> c_int {
    let mut rc: c_int;

    mutex_lock_interruptible(addr_of_mut!(register_mutex));
    rc = snd_register_oss_device(
        SNDRV_OSS_DEVICE_TYPE_SEQUENCER,
        null_mut(),
        0,
        &seq_oss_f_ops,
        null_mut(),
    );
    if rc < 0 {
        mutex_unlock(addr_of_mut!(register_mutex));
        pr_err(ALSA_SEQ_OSS_CANT_REGISTER_DEVICE_SEQ.as_ptr() as *const c_char);
        return rc;
    }
    rc = snd_register_oss_device(
        SNDRV_OSS_DEVICE_TYPE_MUSIC,
        null_mut(),
        0,
        &seq_oss_f_ops,
        null_mut(),
    );
    if rc < 0 {
        pr_err(ALSA_SEQ_OSS_CANT_REGISTER_DEVICE_MUSIC.as_ptr() as *const c_char);
        snd_unregister_oss_device(SNDRV_OSS_DEVICE_TYPE_SEQUENCER, null_mut(), 0);
        mutex_unlock(addr_of_mut!(register_mutex));
        return rc;
    }
    mutex_unlock(addr_of_mut!(register_mutex));
    0
}

unsafe extern "C" fn unregister_device() {
    mutex_lock_interruptible(addr_of_mut!(register_mutex));
    if snd_unregister_oss_device(SNDRV_OSS_DEVICE_TYPE_MUSIC, null_mut(), 0) < 0 {
        pr_err(ALSA_SEQ_OSS_ERROR_UNREGISTER_DEVICE_MUSIC.as_ptr() as *const c_char);
    }
    if snd_unregister_oss_device(SNDRV_OSS_DEVICE_TYPE_SEQUENCER, null_mut(), 0) < 0 {
        pr_err(ALSA_SEQ_OSS_ERROR_UNREGISTER_DEVICE_SEQ.as_ptr() as *const c_char);
    }
    mutex_unlock(addr_of_mut!(register_mutex));
}

/*
 * /proc interface
 */

#[cfg(CONFIG_SND_PROC_FS)]
static mut info_entry: *mut snd_info_entry = null_mut();

#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" fn info_read(_entry: *mut snd_info_entry, buf: *mut snd_info_buffer) {
    mutex_lock_interruptible(addr_of_mut!(register_mutex));
    snd_iprintf(
        buf,
        OSS_SEQUENCER_EMULATION_VERSION_FMT.as_ptr() as *const c_char,
        SNDRV_SEQ_OSS_VERSION_STR,
    );
    snd_seq_oss_system_info_read(buf);
    snd_seq_oss_synth_info_read(buf);
    snd_seq_oss_midi_info_read(buf);
    mutex_unlock(addr_of_mut!(register_mutex));
}

#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" fn register_proc() -> c_int {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_module_entry(THIS_MODULE, SNDRV_SEQ_OSS_PROCNAME, snd_seq_root);
    if entry.is_null() {
        return -ENOMEM;
    }

    (*entry).content = SNDRV_INFO_CONTENT_TEXT;
    (*entry).private_data = null_mut();
    (*entry).c.text.read = Some(info_read);
    if snd_info_register(entry) < 0 {
        snd_info_free_entry(entry);
        return -ENOMEM;
    }
    info_entry = entry;
    0
}

#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" fn unregister_proc() {
    snd_info_free_entry(info_entry);
    info_entry = null_mut();
}

#[cfg(not(CONFIG_SND_PROC_FS))]
unsafe extern "C" fn register_proc() -> c_int {
    0
}

#[cfg(not(CONFIG_SND_PROC_FS))]
unsafe extern "C" fn unregister_proc() {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
