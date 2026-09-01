// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  OSS emulation layer for the mixer interface
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// Original C dependencies:
// linux/init.h, linux/slab.h, linux/time.h, linux/string.h, linux/module.h,
// linux/compat.h, sound/core.h, sound/minors.h, sound/control.h,
// sound/info.h, sound/mixer_oss.h, linux/soundcard.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const OSS_ALSAEMULVER: c_uint = _SIOR('M' as c_uint, 249, size_of::<c_int>() as c_uint);

extern "C" {
    static mut snd_mixer_oss_notify_callback:
        Option<unsafe extern "C" fn(*mut snd_card, c_int) -> c_int>;
    static THIS_MODULE: *mut module;

    fn _SIOR(type_: c_uint, nr: c_uint, size: c_uint) -> c_uint;
    fn nonseekable_open(inode: *mut inode, file: *mut file) -> c_int;
    fn iminor(inode: *mut inode) -> c_int;
    fn snd_lookup_oss_minor_data(minor: c_int, type_: c_int) -> *mut snd_card;
    fn snd_card_unref(card: *mut snd_card);
    fn snd_card_ref(idx: c_int) -> *mut snd_card;
    fn snd_card_file_add(card: *mut snd_card, file: *mut file) -> c_int;
    fn snd_card_file_remove(card: *mut snd_card, file: *mut file) -> c_int;
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn get_user_int(x: *mut c_int, ptr: *const c_int) -> c_int;
    fn put_user_int(x: c_int, ptr: *mut c_int) -> c_int;
    fn snd_BUG_ON(condition: bool) -> bool;
    fn ffz(word: c_ulong) -> c_uint;
    fn DIV_ROUND_CLOSEST(x: c_long, divisor: c_long) -> c_long;
    fn snd_ctl_find_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> *mut snd_kcontrol;
    fn snd_ctl_find_numid(card: *mut snd_card, numid: c_uint) -> *mut snd_kcontrol;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_register_oss_device(
        type_: c_int,
        card: *mut snd_card,
        dev: c_int,
        f_ops: *const file_operations,
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_unregister_oss_device(type_: c_int, card: *mut snd_card, dev: c_int);
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_register(entry: *mut snd_info_entry) -> c_int;
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn snd_info_get_str(dst: *mut c_char, src: *const c_char, len: c_int) -> *const c_char;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn simple_strtoul(cp: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn mutex_init(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn snd_oss_info_register(type_: c_int, card: c_int, name: *const c_char);
    fn snd_oss_info_unregister(type_: c_int, card: c_int);
}

#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
}

#[repr(C)]
pub struct snd_card {
    pub mixer_oss: *mut snd_mixer_oss,
    pub module: *mut module,
    pub driver: [c_char; 16],
    pub mixername: [c_char; 80],
    pub mixer_oss_change_count: c_int,
    pub controls_rwsem: rw_semaphore,
    pub shutdown: bool,
    pub proc_root: *mut snd_info_entry,
    pub number: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_mixer_oss_file {
    pub card: *mut snd_card,
    pub mixer: *mut snd_mixer_oss,
}

#[repr(C)]
pub struct snd_mixer_oss {
    pub card: *mut snd_card,
    pub id: [c_char; 16],
    pub name: [c_char; 80],
    pub reg_mutex: mutex,
    pub slots: [snd_mixer_oss_slot; SNDRV_OSS_MAX_MIXERS],
    pub get_recsrc: Option<unsafe extern "C" fn(*mut snd_mixer_oss_file, *mut c_uint) -> c_int>,
    pub put_recsrc: Option<unsafe extern "C" fn(*mut snd_mixer_oss_file, c_uint) -> c_int>,
    pub mask_recsrc: c_int,
    pub oss_recsrc: c_int,
    pub proc_entry: *mut snd_info_entry,
    pub oss_dev_alloc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mixer_oss_slot {
    pub number: c_int,
    pub stereo: c_int,
    pub volume: [c_int; 2],
    pub get_volume: Option<unsafe extern "C" fn(*mut snd_mixer_oss_file, *mut snd_mixer_oss_slot, *mut c_int, *mut c_int) -> c_int>,
    pub put_volume: Option<unsafe extern "C" fn(*mut snd_mixer_oss_file, *mut snd_mixer_oss_slot, c_int, c_int) -> c_int>,
    pub get_recsrc: Option<unsafe extern "C" fn(*mut snd_mixer_oss_file, *mut snd_mixer_oss_slot, *mut c_int) -> c_int>,
    pub put_recsrc: Option<unsafe extern "C" fn(*mut snd_mixer_oss_file, *mut snd_mixer_oss_slot, c_int) -> c_int>,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_mixer_oss_slot)>,
}

#[repr(C)]
pub struct mixer_info {
    pub id: [c_char; 16],
    pub name: [c_char; 32],
    pub modify_counter: c_int,
}

#[repr(C)]
pub struct _old_mixer_info {
    pub id: [c_char; 16],
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
    pub info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int,
    pub get: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
    pub put: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
    pub enumerated: snd_ctl_elem_info_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_enumerated {
    pub items: c_uint,
    pub item: c_uint,
    pub name: [c_char; 64],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    pub write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub union snd_info_entry_c {
    pub text: core::mem::ManuallyDrop<snd_info_entry_text>,
}

#[repr(C)]
pub struct snd_info_entry {
    pub content: c_int,
    pub mode: c_uint,
    pub c: snd_info_entry_c,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_mixer_oss_assign_table {
    pub oss_id: c_int,
    pub name: *const c_char,
    pub index: c_int,
}

const GFP_KERNEL: c_uint = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EIO: c_int = 5;
const ENXIO: c_int = 6;
const ENOENT: c_int = 2;
const SIOC_IN: c_uint = 0x80000000;
const SIOC_OUT: c_uint = 0x40000000;
const SNDRV_OSS_DEVICE_TYPE_MIXER: c_int = 0;
const SNDRV_MINOR_OSS_MIXER: c_int = 0;
const SOUND_CAP_EXCL_INPUT: c_int = 0x00000040;
const SOUND_MIXER_INFO: c_uint = 0;
const SOUND_OLD_MIXER_INFO: c_uint = 0;
const SOUND_MIXER_WRITE_RECSRC: c_uint = 0;
const OSS_GETVERSION: c_uint = 0;
const SNDRV_OSS_VERSION: c_int = 0;
const SOUND_MIXER_READ_DEVMASK: c_uint = 0;
const SOUND_MIXER_READ_STEREODEVS: c_uint = 0;
const SOUND_MIXER_READ_RECMASK: c_uint = 0;
const SOUND_MIXER_READ_CAPS: c_uint = 0;
const SOUND_MIXER_READ_RECSRC: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_INFO_CONTENT_TEXT: c_int = 0;
const S_IFREG: c_uint = 0o100000;
const SNDRV_CARDS: c_int = 8;
const SND_MIXER_OSS_NOTIFY_REGISTER: c_int = 0;
const SND_MIXER_OSS_NOTIFY_DISCONNECT: c_int = 1;
const SND_MIXER_OSS_NOTIFY_FREE: c_int = 2;
const SNDRV_OSS_MAX_MIXERS: usize = 32;

const SOUND_MIXER_VOLUME: c_int = 0;
const SOUND_MIXER_BASS: c_int = 1;
const SOUND_MIXER_TREBLE: c_int = 2;
const SOUND_MIXER_SYNTH: c_int = 3;
const SOUND_MIXER_PCM: c_int = 4;
const SOUND_MIXER_SPEAKER: c_int = 5;
const SOUND_MIXER_LINE: c_int = 6;
const SOUND_MIXER_MIC: c_int = 7;
const SOUND_MIXER_CD: c_int = 8;
const SOUND_MIXER_IMIX: c_int = 9;
const SOUND_MIXER_ALTPCM: c_int = 10;
const SOUND_MIXER_RECLEV: c_int = 11;
const SOUND_MIXER_IGAIN: c_int = 12;
const SOUND_MIXER_OGAIN: c_int = 13;
const SOUND_MIXER_LINE1: c_int = 14;
const SOUND_MIXER_LINE2: c_int = 15;
const SOUND_MIXER_LINE3: c_int = 16;
const SOUND_MIXER_DIGITAL1: c_int = 17;
const SOUND_MIXER_DIGITAL2: c_int = 18;
const SOUND_MIXER_DIGITAL3: c_int = 19;
const SOUND_MIXER_PHONEIN: c_int = 20;
const SOUND_MIXER_PHONEOUT: c_int = 21;
const SOUND_MIXER_VIDEO: c_int = 22;
const SOUND_MIXER_RADIO: c_int = 23;
const SOUND_MIXER_MONITOR: c_int = 24;

struct MutexGuard(*mut mutex);
impl Drop for MutexGuard {
    fn drop(&mut self) { unsafe { mutex_unlock(self.0) } }
}
unsafe fn guard_mutex(m: *mut mutex) -> MutexGuard {
    mutex_lock(m);
    MutexGuard(m)
}

struct RwsemReadGuard(*mut rw_semaphore);
impl Drop for RwsemReadGuard {
    fn drop(&mut self) { unsafe { up_read(self.0) } }
}
unsafe fn guard_rwsem_read(s: *mut rw_semaphore) -> RwsemReadGuard {
    down_read(s);
    RwsemReadGuard(s)
}

unsafe extern "C" fn snd_mixer_oss_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut card: *mut snd_card;
    let fmixer: *mut snd_mixer_oss_file;
    let err: c_int;

    nonseekable_open(inode, file);
    card = snd_lookup_oss_minor_data(iminor(inode), SNDRV_OSS_DEVICE_TYPE_MIXER);
    if card.is_null() {
        return -ENODEV;
    }
    if (*card).mixer_oss.is_null() {
        snd_card_unref(card);
        return -ENODEV;
    }
    err = snd_card_file_add(card, file);
    if err < 0 {
        snd_card_unref(card);
        return err;
    }
    fmixer = kzalloc(size_of::<snd_mixer_oss_file>(), GFP_KERNEL) as *mut snd_mixer_oss_file;
    if fmixer.is_null() {
        snd_card_file_remove(card, file);
        snd_card_unref(card);
        return -ENOMEM;
    }
    (*fmixer).card = card;
    (*fmixer).mixer = (*card).mixer_oss;
    (*file).private_data = fmixer as *mut c_void;
    if !try_module_get((*card).module) {
        kfree(fmixer as *mut c_void);
        snd_card_file_remove(card, file);
        snd_card_unref(card);
        return -ENODEV;
    }
    snd_card_unref(card);
    0
}

unsafe extern "C" fn snd_mixer_oss_release(_inode: *mut inode, file: *mut file) -> c_int {
    if !(*file).private_data.is_null() {
        let fmixer = (*file).private_data as *mut snd_mixer_oss_file;
        module_put((*(*fmixer).card).module);
        snd_card_file_remove((*fmixer).card, file);
        kfree(fmixer as *mut c_void);
    }
    0
}

unsafe fn snd_mixer_oss_info(fmixer: *mut snd_mixer_oss_file, _info: *mut mixer_info) -> c_int {
    let card = (*fmixer).card;
    let mixer = (*fmixer).mixer;
    let mut info: mixer_info = core::mem::zeroed();
    let id = if !mixer.is_null() && (*mixer).id[0] != 0 { (*mixer).id.as_ptr() } else { (*card).driver.as_ptr() };
    let name = if !mixer.is_null() && (*mixer).name[0] != 0 { (*mixer).name.as_ptr() } else { (*card).mixername.as_ptr() };
    strscpy(info.id.as_mut_ptr(), id, size_of::<[c_char; 16]>());
    strscpy(info.name.as_mut_ptr(), name, size_of::<[c_char; 32]>());
    info.modify_counter = (*card).mixer_oss_change_count;
    if copy_to_user(_info as *mut c_void, &info as *const _ as *const c_void, size_of::<mixer_info>()) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn snd_mixer_oss_info_obsolete(fmixer: *mut snd_mixer_oss_file, _info: *mut _old_mixer_info) -> c_int {
    let card = (*fmixer).card;
    let mixer = (*fmixer).mixer;
    let mut info: _old_mixer_info = core::mem::zeroed();
    let id = if !mixer.is_null() && (*mixer).id[0] != 0 { (*mixer).id.as_ptr() } else { (*card).driver.as_ptr() };
    let name = if !mixer.is_null() && (*mixer).name[0] != 0 { (*mixer).name.as_ptr() } else { (*card).mixername.as_ptr() };
    strscpy(info.id.as_mut_ptr(), id, size_of::<[c_char; 16]>());
    strscpy(info.name.as_mut_ptr(), name, size_of::<[c_char; 32]>());
    if copy_to_user(_info as *mut c_void, &info as *const _ as *const c_void, size_of::<_old_mixer_info>()) != 0 {
        return -EFAULT;
    }
    0
}

unsafe fn snd_mixer_oss_caps(fmixer: *mut snd_mixer_oss_file) -> c_int {
    let mixer = (*fmixer).mixer;
    let mut result = 0;
    if mixer.is_null() { return -EIO; }
    if (*mixer).get_recsrc.is_some() && (*mixer).put_recsrc.is_some() {
        result |= SOUND_CAP_EXCL_INPUT;
    }
    result
}

unsafe fn snd_mixer_oss_devmask(fmixer: *mut snd_mixer_oss_file) -> c_int {
    let mixer = (*fmixer).mixer;
    let mut result = 0;
    if mixer.is_null() { return -EIO; }
    let _guard = guard_mutex(&mut (*mixer).reg_mutex);
    for chn in 0..31 {
        let pslot = &mut (*mixer).slots[chn];
        if pslot.put_volume.is_some() || pslot.put_recsrc.is_some() {
            result |= 1 << chn;
        }
    }
    result
}

unsafe fn snd_mixer_oss_stereodevs(fmixer: *mut snd_mixer_oss_file) -> c_int {
    let mixer = (*fmixer).mixer;
    let mut result = 0;
    if mixer.is_null() { return -EIO; }
    let _guard = guard_mutex(&mut (*mixer).reg_mutex);
    for chn in 0..31 {
        let pslot = &mut (*mixer).slots[chn];
        if pslot.put_volume.is_some() && pslot.stereo != 0 {
            result |= 1 << chn;
        }
    }
    result
}

unsafe fn snd_mixer_oss_recmask(fmixer: *mut snd_mixer_oss_file) -> c_int {
    let mixer = (*fmixer).mixer;
    let mut result = 0;
    if mixer.is_null() { return -EIO; }
    let _guard = guard_mutex(&mut (*mixer).reg_mutex);
    if (*mixer).put_recsrc.is_some() && (*mixer).get_recsrc.is_some() {
        result = (*mixer).mask_recsrc;
    } else {
        for chn in 0..31 {
            let pslot = &mut (*mixer).slots[chn];
            if pslot.put_recsrc.is_some() {
                result |= 1 << chn;
            }
        }
    }
    result
}

unsafe fn snd_mixer_oss_get_recsrc(fmixer: *mut snd_mixer_oss_file) -> c_int {
    let mixer = (*fmixer).mixer;
    let mut result = 0;
    if mixer.is_null() { return -EIO; }
    let _guard = guard_mutex(&mut (*mixer).reg_mutex);
    if let (Some(get_recsrc), Some(_put_recsrc)) = ((*mixer).get_recsrc, (*mixer).put_recsrc) {
        let mut index: c_uint = 0;
        result = get_recsrc(fmixer, &mut index);
        if result < 0 { return result; }
        result = 1 << index;
    } else {
        for chn in 0..31 {
            let pslot = &mut (*mixer).slots[chn];
            if let Some(get_recsrc) = pslot.get_recsrc {
                let mut active = 0;
                get_recsrc(fmixer, pslot, &mut active);
                if active != 0 { result |= 1 << chn; }
            }
        }
    }
    (*mixer).oss_recsrc = result;
    result
}

unsafe fn snd_mixer_oss_set_recsrc(fmixer: *mut snd_mixer_oss_file, mut recsrc: c_int) -> c_int {
    let mixer = (*fmixer).mixer;
    let mut result = 0;
    if mixer.is_null() { return -EIO; }
    let _guard = guard_mutex(&mut (*mixer).reg_mutex);
    if let (Some(get_recsrc), Some(put_recsrc)) = ((*mixer).get_recsrc, (*mixer).put_recsrc) {
        let mut index: c_uint = 0;
        if (recsrc & !(*mixer).oss_recsrc) != 0 {
            recsrc &= !(*mixer).oss_recsrc;
        }
        put_recsrc(fmixer, ffz(!(recsrc as c_ulong)));
        get_recsrc(fmixer, &mut index);
        result = 1 << index;
    }
    for chn in 0..31 {
        let pslot = &mut (*mixer).slots[chn];
        if let Some(put_recsrc) = pslot.put_recsrc {
            let active = if (recsrc & (1 << chn)) != 0 { 1 } else { 0 };
            put_recsrc(fmixer, pslot, active);
        }
    }
    if result == 0 {
        for chn in 0..31 {
            let pslot = &mut (*mixer).slots[chn];
            if let Some(get_recsrc) = pslot.get_recsrc {
                let mut active = 0;
                get_recsrc(fmixer, pslot, &mut active);
                if active != 0 { result |= 1 << chn; }
            }
        }
    }
    result
}

unsafe fn snd_mixer_oss_get_volume(fmixer: *mut snd_mixer_oss_file, slot: c_int) -> c_int {
    let mixer = (*fmixer).mixer;
    let mut result = 0;
    if mixer.is_null() || slot > 30 { return -EIO; }
    let _guard = guard_mutex(&mut (*mixer).reg_mutex);
    let pslot = &mut (*mixer).slots[slot as usize];
    let mut left = pslot.volume[0];
    let mut right = pslot.volume[1];
    if let Some(get_volume) = pslot.get_volume {
        result = get_volume(fmixer, pslot, &mut left, &mut right);
    }
    if pslot.stereo == 0 { right = left; }
    if snd_BUG_ON(left < 0 || left > 100) { return -EIO; }
    if snd_BUG_ON(right < 0 || right > 100) { return -EIO; }
    if result >= 0 {
        pslot.volume[0] = left;
        pslot.volume[1] = right;
        result = (left & 0xff) | ((right & 0xff) << 8);
    }
    result
}

unsafe fn snd_mixer_oss_set_volume(fmixer: *mut snd_mixer_oss_file, slot: c_int, volume: c_int) -> c_int {
    let mixer = (*fmixer).mixer;
    let mut result = 0;
    let mut left = volume & 0xff;
    let mut right = (volume >> 8) & 0xff;
    if mixer.is_null() || slot > 30 { return -EIO; }
    let _guard = guard_mutex(&mut (*mixer).reg_mutex);
    let pslot = &mut (*mixer).slots[slot as usize];
    if left > 100 { left = 100; }
    if right > 100 { right = 100; }
    if pslot.stereo == 0 { right = left; }
    if let Some(put_volume) = pslot.put_volume {
        result = put_volume(fmixer, pslot, left, right);
    }
    if result < 0 { return result; }
    pslot.volume[0] = left;
    pslot.volume[1] = right;
    (left & 0xff) | ((right & 0xff) << 8)
}

unsafe fn snd_mixer_oss_ioctl1(fmixer: *mut snd_mixer_oss_file, cmd: c_uint, arg: c_ulong) -> c_int {
    let argp = arg as *mut c_void;
    let p = argp as *mut c_int;
    let mut tmp: c_int = 0;

    if snd_BUG_ON(fmixer.is_null()) { return -ENXIO; }
    if ((cmd >> 8) & 0xff) == 'M' as c_uint {
        match cmd {
            SOUND_MIXER_INFO => return snd_mixer_oss_info(fmixer, argp as *mut mixer_info),
            SOUND_OLD_MIXER_INFO => return snd_mixer_oss_info_obsolete(fmixer, argp as *mut _old_mixer_info),
            SOUND_MIXER_WRITE_RECSRC => {
                if get_user_int(&mut tmp, p) != 0 { return -EFAULT; }
                tmp = snd_mixer_oss_set_recsrc(fmixer, tmp);
                if tmp < 0 { return tmp; }
                return put_user_int(tmp, p);
            }
            OSS_GETVERSION => return put_user_int(SNDRV_OSS_VERSION, p),
            OSS_ALSAEMULVER => return put_user_int(1, p),
            SOUND_MIXER_READ_DEVMASK => { tmp = snd_mixer_oss_devmask(fmixer); if tmp < 0 { return tmp; } return put_user_int(tmp, p); }
            SOUND_MIXER_READ_STEREODEVS => { tmp = snd_mixer_oss_stereodevs(fmixer); if tmp < 0 { return tmp; } return put_user_int(tmp, p); }
            SOUND_MIXER_READ_RECMASK => { tmp = snd_mixer_oss_recmask(fmixer); if tmp < 0 { return tmp; } return put_user_int(tmp, p); }
            SOUND_MIXER_READ_CAPS => { tmp = snd_mixer_oss_caps(fmixer); if tmp < 0 { return tmp; } return put_user_int(tmp, p); }
            SOUND_MIXER_READ_RECSRC => { tmp = snd_mixer_oss_get_recsrc(fmixer); if tmp < 0 { return tmp; } return put_user_int(tmp, p); }
            _ => {}
        }
    }
    if (cmd & SIOC_IN) != 0 {
        if get_user_int(&mut tmp, p) != 0 { return -EFAULT; }
        tmp = snd_mixer_oss_set_volume(fmixer, (cmd & 0xff) as c_int, tmp);
        if tmp < 0 { return tmp; }
        return put_user_int(tmp, p);
    } else if (cmd & SIOC_OUT) != 0 {
        tmp = snd_mixer_oss_get_volume(fmixer, (cmd & 0xff) as c_int);
        if tmp < 0 { return tmp; }
        return put_user_int(tmp, p);
    }
    -ENXIO
}

unsafe extern "C" fn snd_mixer_oss_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    snd_mixer_oss_ioctl1((*file).private_data as *mut snd_mixer_oss_file, cmd, arg) as c_long
}

#[no_mangle]
pub unsafe extern "C" fn snd_mixer_oss_ioctl_card(card: *mut snd_card, cmd: c_uint, arg: c_ulong) -> c_int {
    let mut fmixer: snd_mixer_oss_file = core::mem::zeroed();
    if snd_BUG_ON(card.is_null()) { return -ENXIO; }
    if (*card).mixer_oss.is_null() { return -ENXIO; }
    fmixer.card = card;
    fmixer.mixer = (*card).mixer_oss;
    snd_mixer_oss_ioctl1(&mut fmixer, cmd, arg)
}

// EXPORT_SYMBOL(snd_mixer_oss_ioctl_card);

// CONFIG_COMPAT: all compatible. Else this is NULL.
unsafe extern "C" fn snd_mixer_oss_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    snd_mixer_oss_ioctl1((*file).private_data as *mut snd_mixer_oss_file, cmd, arg as c_ulong) as c_long
}

static mut snd_mixer_oss_f_ops: file_operations = file_operations {
    owner: ptr::null_mut(),
    open: Some(snd_mixer_oss_open),
    release: Some(snd_mixer_oss_release),
    unlocked_ioctl: Some(snd_mixer_oss_ioctl),
    compat_ioctl: Some(snd_mixer_oss_ioctl_compat),
};

unsafe fn snd_mixer_oss_conv(val: c_long, omin: c_long, omax: c_long, nmin: c_long, nmax: c_long) -> c_long {
    let orange = omax - omin;
    let nrange = nmax - nmin;
    if orange == 0 { return 0; }
    DIV_ROUND_CLOSEST(nrange * (val - omin), orange) + nmin
}

/* convert from alsa native to oss values (0-100) */
unsafe fn snd_mixer_oss_conv1(val: c_long, min: c_long, max: c_long, old: *mut c_int) -> c_long {
    if val == snd_mixer_oss_conv((*old) as c_long, 0, 100, min, max) {
        return (*old) as c_long;
    }
    snd_mixer_oss_conv(val, min, max, 0, 100)
}

/* convert from oss to alsa native values */
unsafe fn snd_mixer_oss_conv2(val: c_long, min: c_long, max: c_long) -> c_long {
    snd_mixer_oss_conv(val, 0, 100, min, max)
}

/*
#if 0
static void snd_mixer_oss_recsrce_set(struct snd_card *card, int slot) { ... }
static int snd_mixer_oss_recsrce_get(struct snd_card *card, int slot) { ... }
#endif
*/

const SNDRV_MIXER_OSS_SIGNATURE: c_uint = 0x65999250;
const SNDRV_MIXER_OSS_ITEM_GLOBAL: usize = 0;
const SNDRV_MIXER_OSS_ITEM_GSWITCH: usize = 1;
const SNDRV_MIXER_OSS_ITEM_GROUTE: usize = 2;
const SNDRV_MIXER_OSS_ITEM_GVOLUME: usize = 3;
const SNDRV_MIXER_OSS_ITEM_PSWITCH: usize = 4;
const SNDRV_MIXER_OSS_ITEM_PROUTE: usize = 5;
const SNDRV_MIXER_OSS_ITEM_PVOLUME: usize = 6;
const SNDRV_MIXER_OSS_ITEM_CSWITCH: usize = 7;
const SNDRV_MIXER_OSS_ITEM_CROUTE: usize = 8;
const SNDRV_MIXER_OSS_ITEM_CVOLUME: usize = 9;
const SNDRV_MIXER_OSS_ITEM_CAPTURE: usize = 10;
const SNDRV_MIXER_OSS_ITEM_COUNT: usize = 11;

const SNDRV_MIXER_OSS_PRESENT_GLOBAL: c_uint = 1 << 0;
const SNDRV_MIXER_OSS_PRESENT_GSWITCH: c_uint = 1 << 1;
const SNDRV_MIXER_OSS_PRESENT_GROUTE: c_uint = 1 << 2;
const SNDRV_MIXER_OSS_PRESENT_GVOLUME: c_uint = 1 << 3;
const SNDRV_MIXER_OSS_PRESENT_PSWITCH: c_uint = 1 << 4;
const SNDRV_MIXER_OSS_PRESENT_PROUTE: c_uint = 1 << 5;
const SNDRV_MIXER_OSS_PRESENT_PVOLUME: c_uint = 1 << 6;
const SNDRV_MIXER_OSS_PRESENT_CSWITCH: c_uint = 1 << 7;
const SNDRV_MIXER_OSS_PRESENT_CROUTE: c_uint = 1 << 8;
const SNDRV_MIXER_OSS_PRESENT_CVOLUME: c_uint = 1 << 9;
const SNDRV_MIXER_OSS_PRESENT_CAPTURE: c_uint = 1 << 10;

#[repr(C)]
pub struct slot {
    pub signature: c_uint,
    pub present: c_uint,
    pub channels: c_uint,
    pub numid: [c_uint; SNDRV_MIXER_OSS_ITEM_COUNT],
    pub capture_item: c_uint,
    pub assigned: *const snd_mixer_oss_assign_table,
    pub allocated: c_uint,
}

const ID_UNKNOWN: c_uint = c_uint::MAX;

unsafe fn snd_mixer_oss_test_id(mixer: *mut snd_mixer_oss, name: *const c_char, index: c_int) -> *mut snd_kcontrol {
    let card = (*mixer).card;
    let mut id: snd_ctl_elem_id = core::mem::zeroed();
    id.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    strscpy(id.name.as_mut_ptr(), name, size_of::<[c_char; 44]>());
    id.index = index as c_uint;
    snd_ctl_find_id(card, &mut id)
}

unsafe extern "C" fn snd_mixer_oss_get_volume1_vol(
    fmixer: *mut snd_mixer_oss_file,
    pslot: *mut snd_mixer_oss_slot,
    numid: c_uint,
    left: *mut c_int,
    right: *mut c_int,
) {
    if numid == ID_UNKNOWN { return; }
    let card = (*fmixer).card;
    let _guard = guard_rwsem_read(&mut (*card).controls_rwsem);
    if (*card).shutdown { return; }
    let kctl = snd_ctl_find_numid(card, numid);
    if kctl.is_null() { return; }
    let uinfo = kzalloc(size_of::<snd_ctl_elem_info>(), GFP_KERNEL) as *mut snd_ctl_elem_info;
    let uctl = kzalloc(size_of::<snd_ctl_elem_value>(), GFP_KERNEL) as *mut snd_ctl_elem_value;
    if uinfo.is_null() || uctl.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if ((*kctl).info)(kctl, uinfo) != 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if ((*kctl).get)(kctl, uctl) != 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if (*uinfo).type_ == SNDRV_CTL_ELEM_TYPE_BOOLEAN
        && (*uinfo).value.integer.min == 0
        && (*uinfo).value.integer.max == 1 {
        kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return;
    }
    (*left) = snd_mixer_oss_conv1((*uctl).value.integer.value[0], (*uinfo).value.integer.min, (*uinfo).value.integer.max, &mut (*pslot).volume[0]) as c_int;
    if (*uinfo).count > 1 {
        (*right) = snd_mixer_oss_conv1((*uctl).value.integer.value[1], (*uinfo).value.integer.min, (*uinfo).value.integer.max, &mut (*pslot).volume[1]) as c_int;
    }
    kfree(uinfo as *mut c_void);
    kfree(uctl as *mut c_void);
}

unsafe extern "C" fn snd_mixer_oss_get_volume1_sw(
    fmixer: *mut snd_mixer_oss_file,
    _pslot: *mut snd_mixer_oss_slot,
    numid: c_uint,
    left: *mut c_int,
    right: *mut c_int,
    route: c_int,
) {
    if numid == ID_UNKNOWN { return; }
    let card = (*fmixer).card;
    let _guard = guard_rwsem_read(&mut (*card).controls_rwsem);
    if (*card).shutdown { return; }
    let kctl = snd_ctl_find_numid(card, numid);
    if kctl.is_null() { return; }
    let uinfo = kzalloc(size_of::<snd_ctl_elem_info>(), GFP_KERNEL) as *mut snd_ctl_elem_info;
    let uctl = kzalloc(size_of::<snd_ctl_elem_value>(), GFP_KERNEL) as *mut snd_ctl_elem_value;
    if uinfo.is_null() || uctl.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if ((*kctl).info)(kctl, uinfo) != 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if ((*kctl).get)(kctl, uctl) != 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if (*uctl).value.integer.value[0] == 0 {
        *left = 0;
        if (*uinfo).count == 1 { *right = 0; }
    }
    if (*uinfo).count > 1 && (*uctl).value.integer.value[if route != 0 { 3 } else { 1 }] == 0 {
        *right = 0;
    }
    kfree(uinfo as *mut c_void);
    kfree(uctl as *mut c_void);
}

unsafe extern "C" fn snd_mixer_oss_get_volume1(fmixer: *mut snd_mixer_oss_file, pslot: *mut snd_mixer_oss_slot, left: *mut c_int, right: *mut c_int) -> c_int {
    let slot = (*pslot).private_data as *mut slot;
    *right = 100;
    *left = *right;
    if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_PVOLUME) != 0 {
        snd_mixer_oss_get_volume1_vol(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_PVOLUME], left, right);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GVOLUME) != 0 {
        snd_mixer_oss_get_volume1_vol(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GVOLUME], left, right);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GLOBAL) != 0 {
        snd_mixer_oss_get_volume1_vol(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GLOBAL], left, right);
    }
    if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_PSWITCH) != 0 {
        snd_mixer_oss_get_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_PSWITCH], left, right, 0);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GSWITCH) != 0 {
        snd_mixer_oss_get_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GSWITCH], left, right, 0);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_PROUTE) != 0 {
        snd_mixer_oss_get_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_PROUTE], left, right, 1);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GROUTE) != 0 {
        snd_mixer_oss_get_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GROUTE], left, right, 1);
    }
    0
}

unsafe extern "C" fn snd_mixer_oss_put_volume1_vol(fmixer: *mut snd_mixer_oss_file, _pslot: *mut snd_mixer_oss_slot, numid: c_uint, left: c_int, right: c_int) {
    if numid == ID_UNKNOWN { return; }
    let card = (*fmixer).card;
    let _guard = guard_rwsem_read(&mut (*card).controls_rwsem);
    if (*card).shutdown { return; }
    let kctl = snd_ctl_find_numid(card, numid);
    if kctl.is_null() { return; }
    let uinfo = kzalloc(size_of::<snd_ctl_elem_info>(), GFP_KERNEL) as *mut snd_ctl_elem_info;
    let uctl = kzalloc(size_of::<snd_ctl_elem_value>(), GFP_KERNEL) as *mut snd_ctl_elem_value;
    if uinfo.is_null() || uctl.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if ((*kctl).info)(kctl, uinfo) != 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if (*uinfo).type_ == SNDRV_CTL_ELEM_TYPE_BOOLEAN && (*uinfo).value.integer.min == 0 && (*uinfo).value.integer.max == 1 {
        kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return;
    }
    (*uctl).value.integer.value[0] = snd_mixer_oss_conv2(left as c_long, (*uinfo).value.integer.min, (*uinfo).value.integer.max);
    if (*uinfo).count > 1 {
        (*uctl).value.integer.value[1] = snd_mixer_oss_conv2(right as c_long, (*uinfo).value.integer.min, (*uinfo).value.integer.max);
    }
    let res = ((*kctl).put)(kctl, uctl);
    if res > 0 { snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kctl).id); }
    kfree(uinfo as *mut c_void);
    kfree(uctl as *mut c_void);
}

unsafe extern "C" fn snd_mixer_oss_put_volume1_sw(fmixer: *mut snd_mixer_oss_file, _pslot: *mut snd_mixer_oss_slot, numid: c_uint, left: c_int, right: c_int, route: c_int) {
    if numid == ID_UNKNOWN { return; }
    let card = (*fmixer).card;
    let _guard = guard_rwsem_read(&mut (*card).controls_rwsem);
    if (*card).shutdown { return; }
    let kctl = snd_ctl_find_numid(card, numid);
    if kctl.is_null() { return; }
    let uinfo = kzalloc(size_of::<snd_ctl_elem_info>(), GFP_KERNEL) as *mut snd_ctl_elem_info;
    let uctl = kzalloc(size_of::<snd_ctl_elem_value>(), GFP_KERNEL) as *mut snd_ctl_elem_value;
    if uinfo.is_null() || uctl.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if ((*kctl).info)(kctl, uinfo) != 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return; }
    if (*uinfo).count > 1 {
        (*uctl).value.integer.value[0] = if left > 0 { 1 } else { 0 };
        (*uctl).value.integer.value[if route != 0 { 3 } else { 1 }] = if right > 0 { 1 } else { 0 };
        if route != 0 {
            (*uctl).value.integer.value[1] = 0;
            (*uctl).value.integer.value[2] = 0;
        }
    } else {
        (*uctl).value.integer.value[0] = if left > 0 || right > 0 { 1 } else { 0 };
    }
    let res = ((*kctl).put)(kctl, uctl);
    if res > 0 { snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kctl).id); }
    kfree(uinfo as *mut c_void);
    kfree(uctl as *mut c_void);
}

unsafe extern "C" fn snd_mixer_oss_put_volume1(fmixer: *mut snd_mixer_oss_file, pslot: *mut snd_mixer_oss_slot, left: c_int, right: c_int) -> c_int {
    let slot = (*pslot).private_data as *mut slot;
    if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_PVOLUME) != 0 {
        snd_mixer_oss_put_volume1_vol(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_PVOLUME], left, right);
        if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_CVOLUME) != 0 {
            snd_mixer_oss_put_volume1_vol(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CVOLUME], left, right);
        }
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_CVOLUME) != 0 {
        snd_mixer_oss_put_volume1_vol(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CVOLUME], left, right);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GVOLUME) != 0 {
        snd_mixer_oss_put_volume1_vol(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GVOLUME], left, right);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GLOBAL) != 0 {
        snd_mixer_oss_put_volume1_vol(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GLOBAL], left, right);
    }
    if left != 0 || right != 0 {
        if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_PSWITCH) != 0 { snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_PSWITCH], left, right, 0); }
        if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_CSWITCH) != 0 { snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CSWITCH], left, right, 0); }
        if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GSWITCH) != 0 { snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GSWITCH], left, right, 0); }
        if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_PROUTE) != 0 { snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_PROUTE], left, right, 1); }
        if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_CROUTE) != 0 { snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CROUTE], left, right, 1); }
        if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GROUTE) != 0 { snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GROUTE], left, right, 1); }
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_PSWITCH) != 0 {
        snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_PSWITCH], left, right, 0);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_CSWITCH) != 0 {
        snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CSWITCH], left, right, 0);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GSWITCH) != 0 {
        snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GSWITCH], left, right, 0);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_PROUTE) != 0 {
        snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_PROUTE], left, right, 1);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_CROUTE) != 0 {
        snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CROUTE], left, right, 1);
    } else if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_GROUTE) != 0 {
        snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_GROUTE], left, right, 1);
    }
    0
}

unsafe extern "C" fn snd_mixer_oss_get_recsrc1_sw(fmixer: *mut snd_mixer_oss_file, pslot: *mut snd_mixer_oss_slot, active: *mut c_int) -> c_int {
    let slot = (*pslot).private_data as *mut slot;
    let mut left = 1;
    let mut right = 1;
    snd_mixer_oss_get_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CSWITCH], &mut left, &mut right, 0);
    *active = if left != 0 || right != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn snd_mixer_oss_get_recsrc1_route(fmixer: *mut snd_mixer_oss_file, pslot: *mut snd_mixer_oss_slot, active: *mut c_int) -> c_int {
    let slot = (*pslot).private_data as *mut slot;
    let mut left = 1;
    let mut right = 1;
    snd_mixer_oss_get_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CROUTE], &mut left, &mut right, 1);
    *active = if left != 0 || right != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn snd_mixer_oss_put_recsrc1_sw(fmixer: *mut snd_mixer_oss_file, pslot: *mut snd_mixer_oss_slot, active: c_int) -> c_int {
    let slot = (*pslot).private_data as *mut slot;
    snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CSWITCH], active, active, 0);
    0
}

unsafe extern "C" fn snd_mixer_oss_put_recsrc1_route(fmixer: *mut snd_mixer_oss_file, pslot: *mut snd_mixer_oss_slot, active: c_int) -> c_int {
    let slot = (*pslot).private_data as *mut slot;
    snd_mixer_oss_put_volume1_sw(fmixer, pslot, (*slot).numid[SNDRV_MIXER_OSS_ITEM_CROUTE], active, active, 1);
    0
}

unsafe extern "C" fn snd_mixer_oss_get_recsrc2(fmixer: *mut snd_mixer_oss_file, active_index: *mut c_uint) -> c_int {
    let card = (*fmixer).card;
    let mixer = (*fmixer).mixer;
    let uinfo = kzalloc(size_of::<snd_ctl_elem_info>(), GFP_KERNEL) as *mut snd_ctl_elem_info;
    let uctl = kzalloc(size_of::<snd_ctl_elem_value>(), GFP_KERNEL) as *mut snd_ctl_elem_value;
    if uinfo.is_null() || uctl.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return -ENOMEM; }
    let _guard = guard_rwsem_read(&mut (*card).controls_rwsem);
    if (*card).shutdown { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return -ENODEV; }
    let kctl = snd_mixer_oss_test_id(mixer, c"Capture Source".as_ptr(), 0);
    if kctl.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return -ENOENT; }
    let mut err = ((*kctl).info)(kctl, uinfo);
    if err < 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return err; }
    err = ((*kctl).get)(kctl, uctl);
    if err < 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return err; }
    for idx in 0..32 {
        if ((*mixer).mask_recsrc & (1 << idx)) == 0 { continue; }
        let pslot = &mut (*mixer).slots[idx as usize];
        let slot = pslot.private_data as *mut slot;
        if (*slot).signature != SNDRV_MIXER_OSS_SIGNATURE { continue; }
        if ((*slot).present & SNDRV_MIXER_OSS_PRESENT_CAPTURE) == 0 { continue; }
        if (*slot).capture_item == (*uctl).value.enumerated.item[0] {
            *active_index = idx as c_uint;
            break;
        }
    }
    kfree(uinfo as *mut c_void);
    kfree(uctl as *mut c_void);
    0
}

unsafe extern "C" fn snd_mixer_oss_put_recsrc2(fmixer: *mut snd_mixer_oss_file, active_index: c_uint) -> c_int {
    let card = (*fmixer).card;
    let mixer = (*fmixer).mixer;
    let mut slotp: *mut slot = ptr::null_mut();
    let uinfo = kzalloc(size_of::<snd_ctl_elem_info>(), GFP_KERNEL) as *mut snd_ctl_elem_info;
    let uctl = kzalloc(size_of::<snd_ctl_elem_value>(), GFP_KERNEL) as *mut snd_ctl_elem_value;
    if uinfo.is_null() || uctl.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return -ENOMEM; }
    let _guard = guard_rwsem_read(&mut (*card).controls_rwsem);
    if (*card).shutdown { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return -ENODEV; }
    let kctl = snd_mixer_oss_test_id(mixer, c"Capture Source".as_ptr(), 0);
    if kctl.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return -ENOENT; }
    let err = ((*kctl).info)(kctl, uinfo);
    if err < 0 { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return err; }
    for idx in 0..32 {
        if ((*mixer).mask_recsrc & (1 << idx)) == 0 { continue; }
        let pslot = &mut (*mixer).slots[idx as usize];
        slotp = pslot.private_data as *mut slot;
        if (*slotp).signature != SNDRV_MIXER_OSS_SIGNATURE { continue; }
        if ((*slotp).present & SNDRV_MIXER_OSS_PRESENT_CAPTURE) == 0 { continue; }
        if idx as c_uint == active_index { break; }
        slotp = ptr::null_mut();
    }
    if slotp.is_null() { kfree(uinfo as *mut c_void); kfree(uctl as *mut c_void); return 0; }
    for idx in 0..(*uinfo).count as usize {
        (*uctl).value.enumerated.item[idx] = (*slotp).capture_item;
    }
    let err2 = ((*kctl).put)(kctl, uctl);
    if err2 > 0 { snd_ctl_notify((*fmixer).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kctl).id); }
    kfree(uinfo as *mut c_void);
    kfree(uctl as *mut c_void);
    0
}

unsafe fn snd_mixer_oss_build_test(mixer: *mut snd_mixer_oss, slot: *mut slot, name: *const c_char, index: c_int, item: usize) -> c_int {
    let card = (*mixer).card;
    let info = kmalloc(size_of::<snd_ctl_elem_info>(), GFP_KERNEL) as *mut snd_ctl_elem_info;
    if info.is_null() { return -ENOMEM; }
    {
        let _guard = guard_rwsem_read(&mut (*card).controls_rwsem);
        if (*card).shutdown { kfree(info as *mut c_void); return -ENODEV; }
        let kcontrol = snd_mixer_oss_test_id(mixer, name, index);
        if kcontrol.is_null() { kfree(info as *mut c_void); return 0; }
        let err = ((*kcontrol).info)(kcontrol, info);
        if err < 0 { kfree(info as *mut c_void); return err; }
        (*slot).numid[item] = (*kcontrol).id.numid;
    }
    if (*info).count > (*slot).channels { (*slot).channels = (*info).count; }
    (*slot).present |= 1 << item;
    kfree(info as *mut c_void);
    0
}

unsafe extern "C" fn snd_mixer_oss_slot_free(chn: *mut snd_mixer_oss_slot) {
    let p = (*chn).private_data as *mut slot;
    if !p.is_null() {
        if (*p).allocated != 0 && !(*p).assigned.is_null() {
            kfree((*(*p).assigned).name as *mut c_void);
            kfree((*p).assigned as *mut c_void);
        }
        kfree(p as *mut c_void);
    }
}

unsafe fn mixer_slot_clear(rslot: *mut snd_mixer_oss_slot) {
    let idx = (*rslot).number;
    if let Some(private_free) = (*rslot).private_free {
        private_free(rslot);
    }
    memset(rslot as *mut c_void, 0, size_of::<snd_mixer_oss_slot>());
    (*rslot).number = idx;
}

unsafe fn snd_mixer_oss_build_test_all(mixer: *mut snd_mixer_oss, ptr: *const snd_mixer_oss_assign_table, slot: *mut slot) -> c_int {
    let mut strbuf = [0 as c_char; 64];
    let mut err = snd_mixer_oss_build_test(mixer, slot, (*ptr).name, (*ptr).index, SNDRV_MIXER_OSS_ITEM_GLOBAL);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Switch".as_ptr(), (*ptr).name);
    err = snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_GSWITCH);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Route".as_ptr(), (*ptr).name);
    err = snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_GROUTE);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Volume".as_ptr(), (*ptr).name);
    err = snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_GVOLUME);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Playback Switch".as_ptr(), (*ptr).name);
    err = snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_PSWITCH);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Playback Route".as_ptr(), (*ptr).name);
    err = snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_PROUTE);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Playback Volume".as_ptr(), (*ptr).name);
    err = snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_PVOLUME);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Capture Switch".as_ptr(), (*ptr).name);
    err = snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_CSWITCH);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Capture Route".as_ptr(), (*ptr).name);
    err = snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_CROUTE);
    if err != 0 { return err; }
    sprintf(strbuf.as_mut_ptr(), c"%s Capture Volume".as_ptr(), (*ptr).name);
    snd_mixer_oss_build_test(mixer, slot, strbuf.as_ptr(), (*ptr).index, SNDRV_MIXER_OSS_ITEM_CVOLUME)
}

unsafe fn snd_mixer_oss_build_input(mixer: *mut snd_mixer_oss, ptr: *const snd_mixer_oss_assign_table, ptr_allocated: c_int, replace_old: c_int) -> c_int {
    let mut slotv: slot = core::mem::zeroed();
    if (*mixer).slots[(*ptr).oss_id as usize].get_volume.is_some() && replace_old == 0 {
        return 0;
    }
    memset(slotv.numid.as_mut_ptr() as *mut c_void, 0xff, size_of::<[c_uint; SNDRV_MIXER_OSS_ITEM_COUNT]>());
    if snd_mixer_oss_build_test_all(mixer, ptr, &mut slotv) != 0 {
        return 0;
    }
    let _guard = guard_rwsem_read(&mut (*(*mixer).card).controls_rwsem);
    if (*(*mixer).card).shutdown { return -ENODEV; }
    let mut kctl: *mut snd_kcontrol = ptr::null_mut();
    if (*ptr).index == 0 {
        kctl = snd_mixer_oss_test_id(mixer, c"Capture Source".as_ptr(), 0);
    }
    if !kctl.is_null() {
        let uinfo = kzalloc(size_of::<snd_ctl_elem_info>(), GFP_KERNEL) as *mut snd_ctl_elem_info;
        if uinfo.is_null() { return -ENOMEM; }
        if ((*kctl).info)(kctl, uinfo) != 0 { kfree(uinfo as *mut c_void); return 0; }
        let mut strp = (*ptr).name;
        if strcmp(strp, c"Master".as_ptr()) == 0 {
            strp = c"Mix".as_ptr();
        } else if strcmp(strp, c"Master Mono".as_ptr()) == 0 {
            strp = c"Mix Mono".as_ptr();
        }
        slotv.capture_item = 0;
        if strcmp((*uinfo).value.enumerated.name.as_ptr(), strp) == 0 {
            slotv.present |= SNDRV_MIXER_OSS_PRESENT_CAPTURE;
        } else {
            slotv.capture_item = 1;
            while slotv.capture_item < (*uinfo).value.enumerated.items {
                (*uinfo).value.enumerated.item = slotv.capture_item;
                if ((*kctl).info)(kctl, uinfo) != 0 { kfree(uinfo as *mut c_void); return 0; }
                if strcmp((*uinfo).value.enumerated.name.as_ptr(), strp) == 0 {
                    slotv.present |= SNDRV_MIXER_OSS_PRESENT_CAPTURE;
                    break;
                }
                slotv.capture_item += 1;
            }
        }
        kfree(uinfo as *mut c_void);
    }
    if slotv.present != 0 {
        let pslot = kmalloc(size_of::<slot>(), GFP_KERNEL) as *mut slot;
        if pslot.is_null() { return -ENOMEM; }
        ptr::write(pslot, slotv);
        (*pslot).signature = SNDRV_MIXER_OSS_SIGNATURE;
        (*pslot).assigned = ptr;
        (*pslot).allocated = ptr_allocated as c_uint;
        let rslot = &mut (*mixer).slots[(*ptr).oss_id as usize] as *mut snd_mixer_oss_slot;
        mixer_slot_clear(rslot);
        (*rslot).stereo = if (*pslot).channels > 1 { 1 } else { 0 };
        (*rslot).get_volume = Some(snd_mixer_oss_get_volume1);
        (*rslot).put_volume = Some(snd_mixer_oss_put_volume1);
        if ((*pslot).present & SNDRV_MIXER_OSS_PRESENT_CSWITCH) != 0 {
            (*rslot).get_recsrc = Some(snd_mixer_oss_get_recsrc1_sw);
            (*rslot).put_recsrc = Some(snd_mixer_oss_put_recsrc1_sw);
        } else if ((*pslot).present & SNDRV_MIXER_OSS_PRESENT_CROUTE) != 0 {
            (*rslot).get_recsrc = Some(snd_mixer_oss_get_recsrc1_route);
            (*rslot).put_recsrc = Some(snd_mixer_oss_put_recsrc1_route);
        } else if ((*pslot).present & SNDRV_MIXER_OSS_PRESENT_CAPTURE) != 0 {
            (*mixer).mask_recsrc |= 1 << (*ptr).oss_id;
        }
        (*rslot).private_data = pslot as *mut c_void;
        (*rslot).private_free = Some(snd_mixer_oss_slot_free);
        return 1;
    }
    0
}

// CONFIG_SND_PROC_FS block.
static mut oss_mixer_names: [*const c_char; SNDRV_OSS_MAX_MIXERS] = [
    c"VOLUME".as_ptr(), c"BASS".as_ptr(), c"TREBLE".as_ptr(), c"SYNTH".as_ptr(),
    c"PCM".as_ptr(), c"SPEAKER".as_ptr(), c"LINE".as_ptr(), c"MIC".as_ptr(),
    c"CD".as_ptr(), c"IMIX".as_ptr(), c"ALTPCM".as_ptr(), c"RECLEV".as_ptr(),
    c"IGAIN".as_ptr(), c"OGAIN".as_ptr(), c"LINE1".as_ptr(), c"LINE2".as_ptr(),
    c"LINE3".as_ptr(), c"DIGITAL1".as_ptr(), c"DIGITAL2".as_ptr(), c"DIGITAL3".as_ptr(),
    c"PHONEIN".as_ptr(), c"PHONEOUT".as_ptr(), c"VIDEO".as_ptr(), c"RADIO".as_ptr(),
    c"MONITOR".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(),
    ptr::null(), ptr::null(),
];

unsafe extern "C" fn snd_mixer_oss_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let mixer = (*entry).private_data as *mut snd_mixer_oss;
    let _guard = guard_mutex(&mut (*mixer).reg_mutex);
    for i in 0..SNDRV_OSS_MAX_MIXERS {
        if oss_mixer_names[i].is_null() { continue; }
        let p = (*mixer).slots[i].private_data as *mut slot;
        snd_iprintf(buffer, c"%s ".as_ptr(), oss_mixer_names[i]);
        if !p.is_null() && !(*p).assigned.is_null() {
            snd_iprintf(buffer, c"\"%s\" %d\n".as_ptr(), (*(*p).assigned).name, (*(*p).assigned).index);
        } else {
            snd_iprintf(buffer, c"\"\" 0\n".as_ptr());
        }
    }
}

unsafe extern "C" fn snd_mixer_oss_proc_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let mixer = (*entry).private_data as *mut snd_mixer_oss;
    let mut line = [0 as c_char; 128];
    let mut strbuf = [0 as c_char; 32];
    let mut idxstr = [0 as c_char; 16];
    while snd_info_get_line(buffer, line.as_mut_ptr(), size_of::<[c_char; 128]>() as c_int) == 0 {
        let mut cptr = snd_info_get_str(strbuf.as_mut_ptr(), line.as_ptr(), size_of::<[c_char; 32]>() as c_int);
        let mut ch = 0usize;
        while ch < SNDRV_OSS_MAX_MIXERS {
            if !oss_mixer_names[ch].is_null() && strcmp(oss_mixer_names[ch], strbuf.as_ptr()) == 0 { break; }
            ch += 1;
        }
        if ch >= SNDRV_OSS_MAX_MIXERS {
            pr_err(c"ALSA: mixer_oss: invalid OSS volume '%s'\n".as_ptr(), strbuf.as_ptr());
            continue;
        }
        cptr = snd_info_get_str(strbuf.as_mut_ptr(), cptr, size_of::<[c_char; 32]>() as c_int);
        if strbuf[0] == 0 {
            let _guard = guard_mutex(&mut (*mixer).reg_mutex);
            mixer_slot_clear(&mut (*mixer).slots[ch]);
            continue;
        }
        snd_info_get_str(idxstr.as_mut_ptr(), cptr, size_of::<[c_char; 16]>() as c_int);
        let idx = simple_strtoul(idxstr.as_ptr(), ptr::null_mut(), 10) as c_uint;
        if idx >= 0x4000 {
            pr_err(c"ALSA: mixer_oss: invalid index %d\n".as_ptr(), idx);
            continue;
        }
        let _guard = guard_mutex(&mut (*mixer).reg_mutex);
        let slotp = (*mixer).slots[ch].private_data as *mut slot;
        if !slotp.is_null() && !(*slotp).assigned.is_null()
            && (*(*slotp).assigned).index == idx as c_int
            && strcmp((*(*slotp).assigned).name, strbuf.as_ptr()) == 0 {
            break;
        }
        let tbl = kmalloc(size_of::<snd_mixer_oss_assign_table>(), GFP_KERNEL) as *mut snd_mixer_oss_assign_table;
        if tbl.is_null() { break; }
        (*tbl).oss_id = ch as c_int;
        (*tbl).name = kstrdup(strbuf.as_ptr(), GFP_KERNEL);
        if (*tbl).name.is_null() {
            kfree(tbl as *mut c_void);
            break;
        }
        (*tbl).index = idx as c_int;
        if snd_mixer_oss_build_input(mixer, tbl, 1, 1) <= 0 {
            kfree((*tbl).name as *mut c_void);
            kfree(tbl as *mut c_void);
        }
    }
}

unsafe fn snd_mixer_oss_proc_init(mixer: *mut snd_mixer_oss) {
    let entry = snd_info_create_card_entry((*mixer).card, c"oss_mixer".as_ptr(), (*(*mixer).card).proc_root);
    if entry.is_null() { return; }
    (*entry).content = SNDRV_INFO_CONTENT_TEXT;
    (*entry).mode = S_IFREG | 0o644;
    (*entry).c.text = core::mem::ManuallyDrop::new(snd_info_entry_text {
        read: Some(snd_mixer_oss_proc_read),
        write: Some(snd_mixer_oss_proc_write),
    });
    (*entry).private_data = mixer as *mut c_void;
    if snd_info_register(entry) < 0 {
        snd_info_free_entry(entry);
        (*mixer).proc_entry = ptr::null_mut();
    } else {
        (*mixer).proc_entry = entry;
    }
}

unsafe fn snd_mixer_oss_proc_done(mixer: *mut snd_mixer_oss) {
    snd_info_free_entry((*mixer).proc_entry);
    (*mixer).proc_entry = ptr::null_mut();
}

unsafe fn snd_mixer_oss_build(mixer: *mut snd_mixer_oss) {
    static TABLE: [snd_mixer_oss_assign_table; 38] = [
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_VOLUME, name: c"Master".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_VOLUME, name: c"Front".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_BASS, name: c"Tone Control - Bass".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_TREBLE, name: c"Tone Control - Treble".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_SYNTH, name: c"Synth".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_SYNTH, name: c"FM".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_SYNTH, name: c"Music".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_PCM, name: c"PCM".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_SPEAKER, name: c"Beep".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_SPEAKER, name: c"PC Speaker".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_SPEAKER, name: c"Speaker".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_LINE, name: c"Line".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_MIC, name: c"Mic".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_CD, name: c"CD".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_IMIX, name: c"Monitor Mix".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_ALTPCM, name: c"PCM".as_ptr(), index: 1 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_ALTPCM, name: c"Headphone".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_ALTPCM, name: c"Wave".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_RECLEV, name: c"-- nothing --".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_IGAIN, name: c"Capture".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_OGAIN, name: c"Playback".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_LINE1, name: c"Aux".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_LINE2, name: c"Aux".as_ptr(), index: 1 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_LINE3, name: c"Aux".as_ptr(), index: 2 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_DIGITAL1, name: c"Digital".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_DIGITAL1, name: c"IEC958".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_DIGITAL1, name: c"IEC958 Optical".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_DIGITAL1, name: c"IEC958 Coaxial".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_DIGITAL2, name: c"Digital".as_ptr(), index: 1 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_DIGITAL3, name: c"Digital".as_ptr(), index: 2 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_PHONEIN, name: c"Phone".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_PHONEOUT, name: c"Master Mono".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_PHONEOUT, name: c"Speaker".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_PHONEOUT, name: c"Mono".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_PHONEOUT, name: c"Phone".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_VIDEO, name: c"Video".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_RADIO, name: c"Radio".as_ptr(), index: 0 },
        snd_mixer_oss_assign_table { oss_id: SOUND_MIXER_MONITOR, name: c"Monitor".as_ptr(), index: 0 },
    ];
    for idx in 0..TABLE.len() {
        snd_mixer_oss_build_input(mixer, &TABLE[idx], 0, 0);
    }
    if (*mixer).mask_recsrc != 0 {
        (*mixer).get_recsrc = Some(snd_mixer_oss_get_recsrc2);
        (*mixer).put_recsrc = Some(snd_mixer_oss_put_recsrc2);
    }
}

unsafe extern "C" fn snd_mixer_oss_free1(private: *mut c_void) -> c_int {
    let mixer = private as *mut snd_mixer_oss;
    if mixer.is_null() { return 0; }
    let card = (*mixer).card;
    if snd_BUG_ON(mixer != (*card).mixer_oss) { return -ENXIO; }
    (*card).mixer_oss = ptr::null_mut();
    for idx in 0..SNDRV_OSS_MAX_MIXERS {
        let chn = &mut (*mixer).slots[idx] as *mut snd_mixer_oss_slot;
        if let Some(private_free) = (*chn).private_free {
            private_free(chn);
        }
    }
    kfree(mixer as *mut c_void);
    0
}

unsafe extern "C" fn snd_mixer_oss_notify_handler(card: *mut snd_card, cmd: c_int) -> c_int {
    let mut mixer: *mut snd_mixer_oss;
    if cmd == SND_MIXER_OSS_NOTIFY_REGISTER {
        mixer = kzalloc(size_of::<snd_mixer_oss>() * 2, GFP_KERNEL) as *mut snd_mixer_oss;
        if mixer.is_null() { return -ENOMEM; }
        mutex_init(&mut (*mixer).reg_mutex);
        let err = snd_register_oss_device(SNDRV_OSS_DEVICE_TYPE_MIXER, card, 0, &snd_mixer_oss_f_ops, card as *mut c_void);
        if err < 0 {
            dev_err((*card).dev, c"unable to register OSS mixer device %i:%i\n".as_ptr(), (*card).number, 0);
            kfree(mixer as *mut c_void);
            return err;
        }
        (*mixer).oss_dev_alloc = 1;
        (*mixer).card = card;
        if (*card).mixername[0] != 0 {
            strscpy((*mixer).name.as_mut_ptr(), (*card).mixername.as_ptr(), size_of::<[c_char; 80]>());
        } else {
            snprintf((*mixer).name.as_mut_ptr(), size_of::<[c_char; 80]>(), c"mixer%i".as_ptr(), (*card).number);
        }
        // SNDRV_OSS_INFO_DEV_MIXERS conditional registration.
        for idx in 0..SNDRV_OSS_MAX_MIXERS {
            (*mixer).slots[idx].number = idx as c_int;
        }
        (*card).mixer_oss = mixer;
        snd_mixer_oss_build(mixer);
        snd_mixer_oss_proc_init(mixer);
    } else {
        mixer = (*card).mixer_oss;
        if mixer.is_null() { return 0; }
        if (*mixer).oss_dev_alloc != 0 {
            // SNDRV_OSS_INFO_DEV_MIXERS conditional unregistration.
            snd_unregister_oss_device(SNDRV_OSS_DEVICE_TYPE_MIXER, (*mixer).card, 0);
            (*mixer).oss_dev_alloc = 0;
        }
        if cmd == SND_MIXER_OSS_NOTIFY_DISCONNECT {
            return 0;
        }
        snd_mixer_oss_proc_done(mixer);
        return snd_mixer_oss_free1(mixer as *mut c_void);
    }
    0
}

unsafe extern "C" fn alsa_mixer_oss_init() -> c_int {
    snd_mixer_oss_notify_callback = Some(snd_mixer_oss_notify_handler);
    for idx in 0..SNDRV_CARDS {
        let card = snd_card_ref(idx);
        if !card.is_null() {
            snd_mixer_oss_notify_handler(card, SND_MIXER_OSS_NOTIFY_REGISTER);
            snd_card_unref(card);
        }
    }
    0
}

unsafe extern "C" fn alsa_mixer_oss_exit() {
    snd_mixer_oss_notify_callback = None;
    for idx in 0..SNDRV_CARDS {
        let card = snd_card_ref(idx);
        if !card.is_null() {
            snd_mixer_oss_notify_handler(card, SND_MIXER_OSS_NOTIFY_FREE);
            snd_card_unref(card);
        }
    }
}

// module_init(alsa_mixer_oss_init)
// module_exit(alsa_mixer_oss_exit)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
