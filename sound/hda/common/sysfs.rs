// SPDX-License-Identifier: GPL-2.0-only
/*
 * sysfs interface for HD-audio codec
 *
 * Copyright (c) 2014 Takashi Iwai <tiwai@suse.de>
 *
 * split from hda_hwdep.c
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type ssize_t = isize;
type size_t = usize;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

/* hint string pair */
#[repr(C)]
pub struct hda_hint {
    pub key: *const c_char,
    pub val: *const c_char, /* contained in the same alloc as key */
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_array {
    pub list: *mut c_void,
    pub used: c_uint,
}

#[repr(C)]
pub struct hda_core {
    pub vendor_id: c_uint,
    pub subsystem_id: c_uint,
    pub revision_id: c_uint,
    pub afg: c_uint,
    pub mfg: c_uint,
    pub vendor_name: *mut c_char,
    pub chip_name: *mut c_char,
    pub addr: c_int,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hda_core,
    pub modelname: *mut c_char,
    pub power_on_acct: c_ulong,
    pub power_off_acct: c_ulong,
    pub user_mutex: mutex,
    pub init_pins: snd_array,
    pub driver_pins: snd_array,
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    pub init_verbs: snd_array,
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    pub hints: snd_array,
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    pub user_pins: snd_array,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct hda_pincfg {
    pub nid: c_int,
    pub cfg: c_int,
}

#[repr(C)]
pub struct hda_verb {
    pub nid: c_int,
    pub verb: c_int,
    pub param: c_int,
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_hda_update_power_acct(codec: *mut hda_codec);
    fn jiffies_to_msecs(j: c_ulong) -> c_uint;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn sysfs_emit_at(buf: *mut c_char, at: c_int, fmt: *const c_char, ...) -> ssize_t;
    fn mutex_init(lock: *mut mutex);
    fn snd_array_init(array: *mut snd_array, elem_size: size_t, alloc_align: c_uint);
    fn snd_array_free(array: *mut snd_array);
    fn snd_array_new(array: *mut snd_array) -> *mut c_void;
    fn snd_hda_add_pincfg(
        codec: *mut hda_codec,
        list: *mut snd_array,
        nid: c_int,
        cfg: c_int,
    ) -> c_int;
    fn snd_hda_codec_reset(codec: *mut hda_codec) -> c_int;
    fn snd_hda_sysfs_clear(codec: *mut hda_codec);
    fn device_reprobe(dev: *mut device) -> c_int;
    fn hda_codec_dev(codec: *mut hda_codec) -> *mut device;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn codec_err(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_info(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn kstrndup(src: *const c_char, len: size_t, gfp: c_uint) -> *mut c_char;
    fn kstrdup(src: *const c_char, gfp: c_uint) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncasecmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn skip_spaces(str_: *const c_char) -> *mut c_char;
    fn isspace(c: c_int) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn snd_hda_codec_set_name(codec: *mut hda_codec, name: *const c_char);
}

unsafe fn snd_array_ptr<T>(array: *mut snd_array, i: c_int) -> *mut T {
    ((*array).list as *mut T).add(i as usize)
}

unsafe fn power_on_acct_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    snd_hda_update_power_acct(codec);
    sysfs_emit(
        buf,
        c"%u\n".as_ptr(),
        jiffies_to_msecs((*codec).power_on_acct),
    )
}

unsafe fn power_off_acct_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    snd_hda_update_power_acct(codec);
    sysfs_emit(
        buf,
        c"%u\n".as_ptr(),
        jiffies_to_msecs((*codec).power_off_acct),
    )
}

/* static DEVICE_ATTR_RO(power_on_acct); */
/* static DEVICE_ATTR_RO(power_off_acct); */
unsafe fn vendor_id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    sysfs_emit(buf, c"0x%x\n".as_ptr(), (*codec).core.vendor_id)
}

unsafe fn subsystem_id_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    sysfs_emit(buf, c"0x%x\n".as_ptr(), (*codec).core.subsystem_id)
}

unsafe fn revision_id_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    sysfs_emit(buf, c"0x%x\n".as_ptr(), (*codec).core.revision_id)
}

unsafe fn afg_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    sysfs_emit(buf, c"0x%x\n".as_ptr(), (*codec).core.afg)
}

unsafe fn mfg_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    sysfs_emit(buf, c"0x%x\n".as_ptr(), (*codec).core.mfg)
}

unsafe fn vendor_name_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    sysfs_emit(
        buf,
        c"%s\n".as_ptr(),
        if !(*codec).core.vendor_name.is_null() {
            (*codec).core.vendor_name
        } else {
            c"".as_ptr() as *mut c_char
        },
    )
}

unsafe fn chip_name_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    sysfs_emit(
        buf,
        c"%s\n".as_ptr(),
        if !(*codec).core.chip_name.is_null() {
            (*codec).core.chip_name
        } else {
            c"".as_ptr() as *mut c_char
        },
    )
}

unsafe fn modelname_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    sysfs_emit(
        buf,
        c"%s\n".as_ptr(),
        if !(*codec).modelname.is_null() {
            (*codec).modelname
        } else {
            c"".as_ptr() as *mut c_char
        },
    )
}

unsafe fn pin_configs_show(codec: *mut hda_codec, list: *mut snd_array, buf: *mut c_char) -> ssize_t {
    let mut i: c_int = 0;
    let mut len: c_int = 0;

    /* guard(mutex)(&codec->user_mutex); */
    while (i as c_uint) < (*list).used {
        let pin = snd_array_ptr::<hda_pincfg>(list, i);
        len += sysfs_emit_at(buf, len, c"0x%02x 0x%08x\n".as_ptr(), (*pin).nid, (*pin).cfg)
            as c_int;
        i += 1;
    }
    len as ssize_t
}

unsafe fn init_pin_configs_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    pin_configs_show(codec, &mut (*codec).init_pins, buf)
}

unsafe fn driver_pin_configs_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    pin_configs_show(codec, &mut (*codec).driver_pins, buf)
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn clear_codec(codec: *mut hda_codec) -> c_int {
    let err = snd_hda_codec_reset(codec);
    if err < 0 {
        codec_err(codec, c"The codec is being used, can't free.\n".as_ptr());
        return err;
    }
    snd_hda_sysfs_clear(codec);
    0
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn reconfig_codec(codec: *mut hda_codec) -> c_int {
    let mut err: c_int;

    /* CLASS(snd_hda_power, pm)(codec); */
    codec_info(codec, c"hda-codec: reconfiguring\n".as_ptr());
    err = snd_hda_codec_reset(codec);
    if err < 0 {
        codec_err(
            codec,
            c"The codec is being used, can't reconfigure.\n".as_ptr(),
        );
        return err;
    }
    err = device_reprobe(hda_codec_dev(codec));
    if err < 0 {
        return err;
    }
    snd_card_register((*codec).card)
}

/*
 * allocate a string at most len chars, and remove the trailing EOL
 */
#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn kstrndup_noeol(src: *const c_char, len: size_t) -> *mut c_char {
    let s = kstrndup(src, len, GFP_KERNEL);
    let p: *mut c_char;
    if s.is_null() {
        return core::ptr::null_mut();
    }
    p = strchr(s, '\n' as c_int);
    if !p.is_null() {
        *p = 0;
    }
    s
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn vendor_id_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let mut val: c_ulong = 0;
    let err = kstrtoul(buf, 0, &mut val);
    if err < 0 {
        return err as ssize_t;
    }
    (*codec).core.vendor_id = val as c_uint;
    count as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn subsystem_id_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let mut val: c_ulong = 0;
    let err = kstrtoul(buf, 0, &mut val);
    if err < 0 {
        return err as ssize_t;
    }
    (*codec).core.subsystem_id = val as c_uint;
    count as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn revision_id_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let mut val: c_ulong = 0;
    let err = kstrtoul(buf, 0, &mut val);
    if err < 0 {
        return err as ssize_t;
    }
    (*codec).core.revision_id = val as c_uint;
    count as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn vendor_name_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let s = kstrndup_noeol(buf, 64);
    if s.is_null() {
        return -ENOMEM as ssize_t;
    }
    kfree((*codec).core.vendor_name as *const c_void);
    (*codec).core.vendor_name = s;
    count as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn chip_name_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let s = kstrndup_noeol(buf, 64);
    if s.is_null() {
        return -ENOMEM as ssize_t;
    }
    kfree((*codec).core.chip_name as *const c_void);
    (*codec).core.chip_name = s;
    count as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn modelname_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let s = kstrndup_noeol(buf, 64);
    if s.is_null() {
        return -ENOMEM as ssize_t;
    }
    kfree((*codec).modelname as *const c_void);
    (*codec).modelname = s;
    count as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn reconfig_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let mut err: c_int = 0;
    if *buf != 0 {
        err = reconfig_codec(codec);
    }
    if err < 0 { err as ssize_t } else { count as ssize_t }
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn clear_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let mut err: c_int = 0;
    if *buf != 0 {
        err = clear_codec(codec);
    }
    if err < 0 { err as ssize_t } else { count as ssize_t }
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn init_verbs_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let mut i: c_int = 0;
    let mut len: c_int = 0;

    /* guard(mutex)(&codec->user_mutex); */
    while (i as c_uint) < (*codec).init_verbs.used {
        let v = snd_array_ptr::<hda_verb>(&mut (*codec).init_verbs, i);
        len += sysfs_emit_at(
            buf,
            len,
            c"0x%02x 0x%03x 0x%04x\n".as_ptr(),
            (*v).nid,
            (*v).verb,
            (*v).param,
        ) as c_int;
        i += 1;
    }
    len as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn parse_init_verbs(codec: *mut hda_codec, buf: *const c_char) -> c_int {
    let v: *mut hda_verb;
    let mut nid: c_int = 0;
    let mut verb: c_int = 0;
    let mut param: c_int = 0;

    if sscanf(buf, c"%i %i %i".as_ptr(), &mut nid, &mut verb, &mut param) != 3 {
        return -EINVAL;
    }
    if nid == 0 || verb == 0 {
        return -EINVAL;
    }
    /* guard(mutex)(&codec->user_mutex); */
    v = snd_array_new(&mut (*codec).init_verbs) as *mut hda_verb;
    if v.is_null() {
        return -ENOMEM;
    }
    (*v).nid = nid;
    (*v).verb = verb;
    (*v).param = param;
    0
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn init_verbs_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let err = parse_init_verbs(codec, buf);
    if err < 0 {
        return err as ssize_t;
    }
    count as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn hints_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let mut i: c_int = 0;
    let mut len: c_int = 0;

    /* guard(mutex)(&codec->user_mutex); */
    while (i as c_uint) < (*codec).hints.used {
        let hint = snd_array_ptr::<hda_hint>(&mut (*codec).hints, i);
        len += sysfs_emit_at(buf, len, c"%s = %s\n".as_ptr(), (*hint).key, (*hint).val) as c_int;
        i += 1;
    }
    len as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn get_hint(codec: *mut hda_codec, key: *const c_char) -> *mut hda_hint {
    let mut i: c_int = 0;

    while (i as c_uint) < (*codec).hints.used {
        let hint = snd_array_ptr::<hda_hint>(&mut (*codec).hints, i);
        if strcmp((*hint).key, key) == 0 {
            return hint;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[cfg(any(CONFIG_SND_HDA_RECONFIG, CONFIG_SND_HDA_PATCH_LOADER))]
unsafe fn remove_trail_spaces(str_: *mut c_char) {
    let mut p: *mut c_char;
    if *str_ == 0 {
        return;
    }
    p = str_.add(strlen(str_) - 1);
    while isspace(*p as c_int) != 0 {
        *p = 0;
        if p == str_ {
            return;
        }
        p = p.sub(1);
    }
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
const MAX_HINTS: c_uint = 1024;

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn parse_hints(codec: *mut hda_codec, mut buf: *const c_char) -> c_int {
    let mut val: *mut c_char;
    let hint: *mut hda_hint;

    buf = skip_spaces(buf);
    if *buf == 0 || *buf == '#' as c_char || *buf == '\n' as c_char {
        return 0;
    }
    if *buf == '=' as c_char {
        return -EINVAL;
    }

    let key = kstrndup_noeol(buf, 1024);
    if key.is_null() {
        return -ENOMEM;
    }
    /* extract key and val */
    val = strchr(key, '=' as c_int);
    if val.is_null() {
        kfree(key as *const c_void);
        return -EINVAL;
    }
    *val = 0;
    val = val.add(1);
    val = skip_spaces(val) as *mut c_char;
    remove_trail_spaces(key);
    remove_trail_spaces(val);
    /* guard(mutex)(&codec->user_mutex); */
    let existing = get_hint(codec, key);
    if !existing.is_null() {
        /* replace */
        kfree((*existing).key as *const c_void);
        (*existing).key = key;
        (*existing).val = val;
        return 0;
    }
    /* allocate a new hint entry */
    if (*codec).hints.used >= MAX_HINTS {
        kfree(key as *const c_void);
        return -ENOMEM;
    }
    hint = snd_array_new(&mut (*codec).hints) as *mut hda_hint;
    if hint.is_null() {
        kfree(key as *const c_void);
        return -ENOMEM;
    }
    (*hint).key = key;
    (*hint).val = val;
    0
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn hints_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let err = parse_hints(codec, buf);
    if err < 0 {
        return err as ssize_t;
    }
    count as ssize_t
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn user_pin_configs_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    pin_configs_show(codec, &mut (*codec).user_pins, buf)
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn parse_user_pin_configs(codec: *mut hda_codec, buf: *const c_char) -> c_int {
    let mut nid: c_int = 0;
    let mut cfg: c_int = 0;

    if sscanf(buf, c"%i %i".as_ptr(), &mut nid, &mut cfg) != 2 {
        return -EINVAL;
    }
    if nid == 0 {
        return -EINVAL;
    }
    /* guard(mutex)(&codec->user_mutex); */
    snd_hda_add_pincfg(codec, &mut (*codec).user_pins, nid, cfg)
}

#[cfg(CONFIG_SND_HDA_RECONFIG)]
unsafe fn user_pin_configs_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let codec = dev_get_drvdata(dev) as *mut hda_codec;
    let err = parse_user_pin_configs(codec, buf);
    if err < 0 {
        return err as ssize_t;
    }
    count as ssize_t
}

/* sysfs attributes exposed only when CONFIG_SND_HDA_RECONFIG=y */
/* static DEVICE_ATTR_RW(init_verbs); */
/* static DEVICE_ATTR_RW(hints); */
/* static DEVICE_ATTR_RW(user_pin_configs); */
/* static DEVICE_ATTR_WO(reconfig); */
/* static DEVICE_ATTR_WO(clear); */

/**
 * snd_hda_get_hint - Look for hint string
 * @codec: the HDA codec
 * @key: the hint key string
 *
 * Look for a hint key/value pair matching with the given key string
 * and returns the value string.  If nothing found, returns NULL.
 */
#[cfg(CONFIG_SND_HDA_RECONFIG)]
#[no_mangle]
pub unsafe extern "C" fn snd_hda_get_hint(
    codec: *mut hda_codec,
    key: *const c_char,
) -> *const c_char {
    let hint = get_hint(codec, key);
    if !hint.is_null() { (*hint).val } else { core::ptr::null() }
}
/* EXPORT_SYMBOL_GPL(snd_hda_get_hint); */

/**
 * snd_hda_get_bool_hint - Get a boolean hint value
 * @codec: the HDA codec
 * @key: the hint key string
 *
 * Look for a hint key/value pair matching with the given key string
 * and returns a boolean value parsed from the value.  If no matching
 * key is found, return a negative value.
 */
#[cfg(CONFIG_SND_HDA_RECONFIG)]
#[no_mangle]
pub unsafe extern "C" fn snd_hda_get_bool_hint(
    codec: *mut hda_codec,
    key: *const c_char,
) -> c_int {
    let p: *const c_char;

    /* guard(mutex)(&codec->user_mutex); */
    p = snd_hda_get_hint(codec, key);
    if p.is_null() || *p == 0 {
        return -ENOENT;
    }
    match toupper(*p as c_int) {
        x if x == 'T' as c_int => 1, /* true */
        x if x == 'Y' as c_int => 1, /* yes */
        x if x == '1' as c_int => 1,
        _ => 0,
    }
}
/* EXPORT_SYMBOL_GPL(snd_hda_get_bool_hint); */

/**
 * snd_hda_get_int_hint - Get an integer hint value
 * @codec: the HDA codec
 * @key: the hint key string
 * @valp: pointer to store a value
 *
 * Look for a hint key/value pair matching with the given key string
 * and stores the integer value to @valp.  If no matching key is found,
 * return a negative error code.  Otherwise it returns zero.
 */
#[cfg(CONFIG_SND_HDA_RECONFIG)]
#[no_mangle]
pub unsafe extern "C" fn snd_hda_get_int_hint(
    codec: *mut hda_codec,
    key: *const c_char,
    valp: *mut c_int,
) -> c_int {
    let p: *const c_char;
    let mut val: c_ulong = 0;

    /* guard(mutex)(&codec->user_mutex); */
    p = snd_hda_get_hint(codec, key);
    if p.is_null() {
        -ENOENT
    } else if kstrtoul(p, 0, &mut val) != 0 {
        -EINVAL
    } else {
        *valp = val as c_int;
        0
    }
}
/* EXPORT_SYMBOL_GPL(snd_hda_get_int_hint); */

/*
 * common sysfs attributes
 */
/* RECONFIG_DEVICE_ATTR(name) maps to DEVICE_ATTR_RW with CONFIG_SND_HDA_RECONFIG,
 * otherwise DEVICE_ATTR_RO. */
/* static RECONFIG_DEVICE_ATTR(vendor_id); */
/* static RECONFIG_DEVICE_ATTR(subsystem_id); */
/* static RECONFIG_DEVICE_ATTR(revision_id); */
/* static DEVICE_ATTR_RO(afg); */
/* static DEVICE_ATTR_RO(mfg); */
/* static RECONFIG_DEVICE_ATTR(vendor_name); */
/* static RECONFIG_DEVICE_ATTR(chip_name); */
/* static RECONFIG_DEVICE_ATTR(modelname); */
/* static DEVICE_ATTR_RO(init_pin_configs); */
/* static DEVICE_ATTR_RO(driver_pin_configs); */

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_NONE: c_int = 0;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_CODEC: c_int = 1;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_MODEL: c_int = 2;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_PINCFG: c_int = 3;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_VERB: c_int = 4;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_HINT: c_int = 5;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_VENDOR_ID: c_int = 6;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_SUBSYSTEM_ID: c_int = 7;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_REVISION_ID: c_int = 8;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const LINE_MODE_CHIP_NAME: c_int = 9;
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
const NUM_LINE_MODES: usize = 10;

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe fn strmatch(a: *const c_char, b: *const c_char) -> c_int {
    (strncasecmp(a, b, strlen(b)) == 0) as c_int
}

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe fn parse_codec_mode(
    buf: *mut c_char,
    bus: *mut hda_bus,
    codecp: *mut *mut hda_codec,
) {
    let mut vendorid: c_int = 0;
    let mut subid: c_int = 0;
    let mut caddr: c_int = 0;
    let mut codec: *mut hda_codec;

    *codecp = core::ptr::null_mut();
    if sscanf(buf, c"%i %i %i".as_ptr(), &mut vendorid, &mut subid, &mut caddr) == 3 {
        codec = list_first_codec(bus);
        while !codec.is_null() {
            if (vendorid <= 0 || (*codec).core.vendor_id == vendorid as c_uint)
                && (subid <= 0 || (*codec).core.subsystem_id == subid as c_uint)
                && (*codec).core.addr == caddr
            {
                *codecp = codec;
                break;
            }
            codec = list_next_codec(codec, bus);
        }
    }
}

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe extern "C" {
    fn list_first_codec(bus: *mut hda_bus) -> *mut hda_codec;
    fn list_next_codec(codec: *mut hda_codec, bus: *mut hda_bus) -> *mut hda_codec;
}

/* parse the contents after the other command tags, [pincfg], [verb],
 * [vendor_id], [subsystem_id], [revision_id], [chip_name], [hint] and [model]
 * just pass to the sysfs helper (only when any codec was specified)
 */
#[cfg(all(CONFIG_SND_HDA_PATCH_LOADER, CONFIG_SND_HDA_RECONFIG))]
unsafe fn parse_pincfg_mode(buf: *mut c_char, _bus: *mut hda_bus, codecp: *mut *mut hda_codec) {
    parse_user_pin_configs(*codecp, buf);
}

#[cfg(all(CONFIG_SND_HDA_PATCH_LOADER, CONFIG_SND_HDA_RECONFIG))]
unsafe fn parse_verb_mode(buf: *mut c_char, _bus: *mut hda_bus, codecp: *mut *mut hda_codec) {
    parse_init_verbs(*codecp, buf);
}

#[cfg(all(CONFIG_SND_HDA_PATCH_LOADER, CONFIG_SND_HDA_RECONFIG))]
unsafe fn parse_hint_mode(buf: *mut c_char, _bus: *mut hda_bus, codecp: *mut *mut hda_codec) {
    parse_hints(*codecp, buf);
}

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe fn parse_model_mode(buf: *mut c_char, _bus: *mut hda_bus, codecp: *mut *mut hda_codec) {
    kfree((**codecp).modelname as *const c_void);
    (**codecp).modelname = kstrdup(buf, GFP_KERNEL);
}

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe fn parse_chip_name_mode(
    buf: *mut c_char,
    _bus: *mut hda_bus,
    codecp: *mut *mut hda_codec,
) {
    snd_hda_codec_set_name(*codecp, buf);
}

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe fn parse_vendor_id_mode(buf: *mut c_char, _bus: *mut hda_bus, codecp: *mut *mut hda_codec) {
    let mut val: c_ulong = 0;
    if kstrtoul(buf, 0, &mut val) == 0 {
        (**codecp).core.vendor_id = val as c_uint;
    }
}

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe fn parse_subsystem_id_mode(
    buf: *mut c_char,
    _bus: *mut hda_bus,
    codecp: *mut *mut hda_codec,
) {
    let mut val: c_ulong = 0;
    if kstrtoul(buf, 0, &mut val) == 0 {
        (**codecp).core.subsystem_id = val as c_uint;
    }
}

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe fn parse_revision_id_mode(
    buf: *mut c_char,
    _bus: *mut hda_bus,
    codecp: *mut *mut hda_codec,
) {
    let mut val: c_ulong = 0;
    if kstrtoul(buf, 0, &mut val) == 0 {
        (**codecp).core.revision_id = val as c_uint;
    }
}

#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
#[repr(C)]
struct hda_patch_item {
    tag: *const c_char,
    alias: *const c_char,
    parser: Option<unsafe fn(*mut c_char, *mut hda_bus, *mut *mut hda_codec)>,
}

#[cfg(all(CONFIG_SND_HDA_PATCH_LOADER, CONFIG_SND_HDA_RECONFIG))]
static patch_items: [hda_patch_item; NUM_LINE_MODES] = [
    hda_patch_item { tag: core::ptr::null(), alias: core::ptr::null(), parser: None },
    hda_patch_item { tag: c"[codec]".as_ptr(), alias: core::ptr::null(), parser: Some(parse_codec_mode) },
    hda_patch_item { tag: c"[model]".as_ptr(), alias: core::ptr::null(), parser: Some(parse_model_mode) },
    hda_patch_item { tag: c"[pincfg]".as_ptr(), alias: c"[user_pin_configs]".as_ptr(), parser: Some(parse_pincfg_mode) },
    hda_patch_item { tag: c"[verb]".as_ptr(), alias: c"[init_verbs]".as_ptr(), parser: Some(parse_verb_mode) },
    hda_patch_item { tag: c"[hint]".as_ptr(), alias: c"[hints]".as_ptr(), parser: Some(parse_hint_mode) },
    hda_patch_item { tag: c"[vendor_id]".as_ptr(), alias: core::ptr::null(), parser: Some(parse_vendor_id_mode) },
    hda_patch_item { tag: c"[subsystem_id]".as_ptr(), alias: core::ptr::null(), parser: Some(parse_subsystem_id_mode) },
    hda_patch_item { tag: c"[revision_id]".as_ptr(), alias: core::ptr::null(), parser: Some(parse_revision_id_mode) },
    hda_patch_item { tag: c"[chip_name]".as_ptr(), alias: core::ptr::null(), parser: Some(parse_chip_name_mode) },
];

/* check the line starting with '[' -- change the parser mode accordingly */
#[cfg(all(CONFIG_SND_HDA_PATCH_LOADER, CONFIG_SND_HDA_RECONFIG))]
unsafe fn parse_line_mode(buf: *mut c_char, _bus: *mut hda_bus) -> c_int {
    let mut i: usize = 0;
    while i < patch_items.len() {
        if patch_items[i].tag.is_null() {
            i += 1;
            continue;
        }
        if strmatch(buf, patch_items[i].tag) != 0 {
            return i as c_int;
        }
        if !patch_items[i].alias.is_null() && strmatch(buf, patch_items[i].alias) != 0 {
            return i as c_int;
        }
        i += 1;
    }
    LINE_MODE_NONE
}

/* copy one line from the buffer in fw, and update the fields in fw
 * return zero if it reaches to the end of the buffer, or non-zero
 * if successfully copied a line
 *
 * the spaces at the beginning and the end of the line are stripped
 */
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
unsafe fn get_line_from_fw(
    mut buf: *mut c_char,
    size: c_int,
    fw_size_p: *mut size_t,
    fw_data_p: *mut *const c_void,
) -> c_int {
    let mut len: c_int;
    let mut fw_size = *fw_size_p;
    let mut p = *fw_data_p as *const c_char;

    while fw_size != 0 && isspace(*p as c_int) != 0 {
        p = p.add(1);
        fw_size -= 1;
    }
    if fw_size == 0 {
        return 0;
    }

    len = 0;
    while (len as size_t) < fw_size {
        if *p == 0 {
            break;
        }
        if *p == '\n' as c_char {
            p = p.add(1);
            len += 1;
            break;
        }
        if len < size {
            *buf = *p;
            buf = buf.add(1);
        }
        p = p.add(1);
        len += 1;
    }
    *buf = 0;
    *fw_size_p = fw_size - len as size_t;
    *fw_data_p = p as *const c_void;
    remove_trail_spaces(buf);
    1
}

/**
 * snd_hda_load_patch - load a "patch" firmware file and parse it
 * @bus: HD-audio bus
 * @fw_size: the firmware byte size
 * @fw_buf: the firmware data
 */
#[cfg(all(CONFIG_SND_HDA_PATCH_LOADER, CONFIG_SND_HDA_RECONFIG))]
#[no_mangle]
pub unsafe extern "C" fn snd_hda_load_patch(
    bus: *mut hda_bus,
    mut fw_size: size_t,
    mut fw_buf: *const c_void,
) -> c_int {
    let mut buf: [c_char; 128] = [0; 128];
    let mut codec: *mut hda_codec;
    let mut line_mode: c_int;

    line_mode = LINE_MODE_NONE;
    codec = core::ptr::null_mut();
    while get_line_from_fw(
        buf.as_mut_ptr(),
        (core::mem::size_of_val(&buf) - 1) as c_int,
        &mut fw_size,
        &mut fw_buf,
    ) != 0
    {
        if buf[0] == 0 || buf[0] == '#' as c_char || buf[0] == '\n' as c_char {
            continue;
        }
        if buf[0] == '[' as c_char {
            line_mode = parse_line_mode(buf.as_mut_ptr(), bus);
        } else if patch_items[line_mode as usize].parser.is_some()
            && (!codec.is_null() || line_mode <= LINE_MODE_CODEC)
        {
            patch_items[line_mode as usize].parser.unwrap()(buf.as_mut_ptr(), bus, &mut codec);
        }
    }
    0
}
/* EXPORT_SYMBOL_GPL(snd_hda_load_patch); */

/*
 * sysfs entries
 */
unsafe extern "C" {
    static mut dev_attr_vendor_id: device_attribute;
    static mut dev_attr_subsystem_id: device_attribute;
    static mut dev_attr_revision_id: device_attribute;
    static mut dev_attr_afg: device_attribute;
    static mut dev_attr_mfg: device_attribute;
    static mut dev_attr_vendor_name: device_attribute;
    static mut dev_attr_chip_name: device_attribute;
    static mut dev_attr_modelname: device_attribute;
    static mut dev_attr_init_pin_configs: device_attribute;
    static mut dev_attr_driver_pin_configs: device_attribute;
    static mut dev_attr_power_on_acct: device_attribute;
    static mut dev_attr_power_off_acct: device_attribute;
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    static mut dev_attr_init_verbs: device_attribute;
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    static mut dev_attr_hints: device_attribute;
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    static mut dev_attr_user_pin_configs: device_attribute;
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    static mut dev_attr_reconfig: device_attribute;
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    static mut dev_attr_clear: device_attribute;
}

static mut hda_dev_attrs: [*mut attribute; 18] = [
    unsafe { &mut dev_attr_vendor_id.attr },
    unsafe { &mut dev_attr_subsystem_id.attr },
    unsafe { &mut dev_attr_revision_id.attr },
    unsafe { &mut dev_attr_afg.attr },
    unsafe { &mut dev_attr_mfg.attr },
    unsafe { &mut dev_attr_vendor_name.attr },
    unsafe { &mut dev_attr_chip_name.attr },
    unsafe { &mut dev_attr_modelname.attr },
    unsafe { &mut dev_attr_init_pin_configs.attr },
    unsafe { &mut dev_attr_driver_pin_configs.attr },
    unsafe { &mut dev_attr_power_on_acct.attr },
    unsafe { &mut dev_attr_power_off_acct.attr },
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    unsafe { &mut dev_attr_init_verbs.attr },
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    unsafe { &mut dev_attr_hints.attr },
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    unsafe { &mut dev_attr_user_pin_configs.attr },
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    unsafe { &mut dev_attr_reconfig.attr },
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    unsafe { &mut dev_attr_clear.attr },
    core::ptr::null_mut(),
];

static mut hda_dev_attr_group: attribute_group = attribute_group {
    attrs: unsafe { hda_dev_attrs.as_mut_ptr() },
};

#[no_mangle]
pub static mut snd_hda_dev_attr_groups: [*const attribute_group; 2] = [
    unsafe { &hda_dev_attr_group },
    core::ptr::null(),
];

#[no_mangle]
pub unsafe extern "C" fn snd_hda_sysfs_init(codec: *mut hda_codec) {
    mutex_init(&mut (*codec).user_mutex);
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    {
        snd_array_init(
            &mut (*codec).init_verbs,
            core::mem::size_of::<hda_verb>(),
            32,
        );
        snd_array_init(&mut (*codec).hints, core::mem::size_of::<hda_hint>(), 32);
        snd_array_init(
            &mut (*codec).user_pins,
            core::mem::size_of::<hda_pincfg>(),
            16,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_hda_sysfs_clear(codec: *mut hda_codec) {
    #[cfg(CONFIG_SND_HDA_RECONFIG)]
    {
        let mut i: c_int = 0;

        /* clear init verbs */
        snd_array_free(&mut (*codec).init_verbs);
        /* clear hints */
        while (i as c_uint) < (*codec).hints.used {
            let hint = snd_array_ptr::<hda_hint>(&mut (*codec).hints, i);
            kfree((*hint).key as *const c_void); /* we don't need to free hint->val */
            i += 1;
        }
        snd_array_free(&mut (*codec).hints);
        snd_array_free(&mut (*codec).user_pins);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
