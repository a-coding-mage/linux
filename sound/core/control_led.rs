// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  LED state routines for driver control interface
 *  Copyright (c) 2021 by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type bool_t = bool;
type snd_ctl_elem_iface_t = c_uint;

// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("ALSA control interface to LED trigger code.");
// MODULE_LICENSE("GPL");

const SNDRV_CARDS: usize = 32;
const NUM_AUDIO_LEDS: usize = 2;
const LED_AUDIO_MUTE: led_audio = 0;
const LED_AUDIO_MICMUTE: led_audio = 1;
const LED_OFF: c_uint = 0;
const LED_ON: c_uint = 1;

const SNDRV_CTL_ELEM_ACCESS_LED_SHIFT: c_uint = 8;
const SNDRV_CTL_ELEM_ACCESS_SPK_LED: c_uint = 1 << SNDRV_CTL_ELEM_ACCESS_LED_SHIFT;
const SNDRV_CTL_ELEM_ACCESS_MIC_LED: c_uint = 2 << SNDRV_CTL_ELEM_ACCESS_LED_SHIFT;
const SNDRV_CTL_ELEM_ACCESS_LED_MASK: c_uint =
    SNDRV_CTL_ELEM_ACCESS_SPK_LED | SNDRV_CTL_ELEM_ACCESS_MIC_LED;
const MAX_LED: usize = (((SNDRV_CTL_ELEM_ACCESS_MIC_LED - SNDRV_CTL_ELEM_ACCESS_SPK_LED)
    >> SNDRV_CTL_ELEM_ACCESS_LED_SHIFT)
    + 1) as usize;

const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_TYPE_INTEGER64: c_uint = 6;
const SNDRV_CTL_ELEM_IFACE_CARD: snd_ctl_elem_iface_t = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: snd_ctl_elem_iface_t = 2;

const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1 << 0;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 1 << 1;
const SNDRV_CTL_EVENT_MASK_ADD: c_uint = 1 << 2;
const SNDRV_CTL_EVENT_MASK_REMOVE: c_uint = !0;

const GFP_KERNEL: c_uint = 0;
const ENXIO: c_int = 6;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const EXDEV: c_int = 18;
const E2BIG: c_int = 7;
const ENOMEM: c_int = 12;

type led_audio = c_uint;

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
struct class {
    _private: [u8; 0],
}

#[repr(C)]
struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
struct attribute_group {
    attrs: *mut *mut attribute,
}

#[repr(C)]
struct device_attribute {
    attr: attribute,
}

#[repr(C)]
struct device {
    kobj: kobject,
    parent: *mut device,
    class: *mut class,
    release: Option<unsafe extern "C" fn(*mut device)>,
    groups: *const *const attribute_group,
}

#[repr(C)]
struct led_trigger {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_id {
    numid: c_uint,
    iface: snd_ctl_elem_iface_t,
    device: c_uint,
    subdevice: c_uint,
    name: [c_char; 44],
    index: c_uint,
}

#[repr(C)]
struct snd_ctl_elem_integer_info {
    min: i64,
}

#[repr(C)]
struct snd_ctl_elem_integer64_info {
    min: i64,
}

#[repr(C)]
union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_integer_info,
    integer64: snd_ctl_elem_integer64_info,
}

#[repr(C)]
struct snd_ctl_elem_info {
    id: snd_ctl_elem_id,
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
struct snd_ctl_elem_integer {
    value: [i64; 128],
}

#[repr(C)]
struct snd_ctl_elem_integer64 {
    value: [i64; 64],
}

#[repr(C)]
union snd_ctl_elem_value_data {
    integer: snd_ctl_elem_integer,
    integer64: snd_ctl_elem_integer64,
}

#[repr(C)]
struct snd_ctl_elem_value {
    id: snd_ctl_elem_id,
    value: snd_ctl_elem_value_data,
}

#[repr(C)]
struct snd_kcontrol_volatile {
    access: c_uint,
}

#[repr(C)]
struct snd_kcontrol {
    list: list_head,
    id: snd_ctl_elem_id,
    count: c_uint,
    vd: *mut snd_kcontrol_volatile,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
struct snd_ctl_device {
    kobj: kobject,
}

#[repr(C)]
struct snd_card {
    number: c_int,
    controls_rwsem: rw_semaphore,
    controls: list_head,
    ctl_dev: *mut snd_ctl_device,
    card_dev: device,
    dev: *mut device,
}

#[repr(C)]
struct snd_ctl_layer_ops {
    module_name: *const c_char,
    lregister: Option<unsafe extern "C" fn(*mut snd_card)>,
    ldisconnect: Option<unsafe extern "C" fn(*mut snd_card)>,
    lnotify: Option<unsafe extern "C" fn(*mut snd_card, c_uint, *mut snd_kcontrol, c_uint)>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum snd_ctl_led_mode {
    MODE_FOLLOW_MUTE = 0,
    MODE_FOLLOW_ROUTE,
    MODE_OFF,
    MODE_ON,
}

#[repr(C)]
struct snd_ctl_led_card {
    dev: device,
    number: c_int,
    led: *mut snd_ctl_led,
}

#[repr(C)]
struct snd_ctl_led {
    dev: device,
    controls: list_head,
    name: *const c_char,
    group: c_uint,
    trigger_type: led_audio,
    mode: snd_ctl_led_mode,
    cards: [*mut snd_ctl_led_card; SNDRV_CARDS],
}

#[repr(C)]
struct snd_ctl_led_ctl {
    list: list_head,
    card: *mut snd_card,
    access: c_uint,
    kctl: *mut snd_kcontrol,
    index_offset: c_uint,
}

static mut snd_ctl_led_mutex: mutex = mutex { _private: [] };
static mut snd_ctl_led_card_valid: [bool_t; SNDRV_CARDS] = [false; SNDRV_CARDS];
static mut snd_ctl_ledtrig_audio: [*mut led_trigger; NUM_AUDIO_LEDS] = [ptr::null_mut(); NUM_AUDIO_LEDS];
static speaker_name: &[u8] = b"speaker\0";
static mic_name: &[u8] = b"mic\0";
static SND_CTL_LAYER_MODULE_LED: &[u8] = b"led\0";

static mut snd_ctl_leds: [snd_ctl_led; MAX_LED] = [
    snd_ctl_led {
        dev: unsafe { core::mem::zeroed() },
        controls: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
        name: speaker_name.as_ptr() as *const c_char,
        group: ((SNDRV_CTL_ELEM_ACCESS_SPK_LED >> SNDRV_CTL_ELEM_ACCESS_LED_SHIFT) - 1),
        trigger_type: LED_AUDIO_MUTE,
        mode: snd_ctl_led_mode::MODE_FOLLOW_MUTE,
        cards: [ptr::null_mut(); SNDRV_CARDS],
    },
    snd_ctl_led {
        dev: unsafe { core::mem::zeroed() },
        controls: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
        name: mic_name.as_ptr() as *const c_char,
        group: ((SNDRV_CTL_ELEM_ACCESS_MIC_LED >> SNDRV_CTL_ELEM_ACCESS_LED_SHIFT) - 1),
        trigger_type: LED_AUDIO_MICMUTE,
        mode: snd_ctl_led_mode::MODE_FOLLOW_MUTE,
        cards: [ptr::null_mut(); SNDRV_CARDS],
    },
];

extern "C" {
    static mut sound_class: class;
    static mut dev_attr_mode: device_attribute;
    static mut dev_attr_brightness: device_attribute;
    static mut dev_attr_attach: device_attribute;
    static mut dev_attr_detach: device_attribute;
    static mut dev_attr_reset: device_attribute;
    static mut dev_attr_list: device_attribute;

    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: size_t) -> isize;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn kstrtoull(s: *const c_char, base: c_uint, res: *mut c_ulonglong) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn led_trigger_event(trigger: *mut led_trigger, event: c_uint);
    fn led_trigger_get_brightness(trigger: *mut led_trigger) -> c_uint;
    fn led_trigger_register_simple(name: *const c_char, trigger: *mut *mut led_trigger);
    fn led_trigger_unregister_simple(trigger: *mut led_trigger);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn sysfs_emit_at(buf: *mut c_char, at: size_t, fmt: *const c_char, ...) -> ssize_t;
    fn sysfs_create_link(kobj: *mut kobject, target: *mut kobject, name: *const c_char) -> c_int;
    fn sysfs_remove_link(kobj: *mut kobject, name: *const c_char);
    fn device_initialize(dev: *mut device);
    fn device_add(dev: *mut device) -> c_int;
    fn device_unregister(dev: *mut device);
    fn put_device(dev: *mut device);
    fn dev_set_name(dev: *mut device, name: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_card_ref(card_number: c_int) -> *mut snd_card;
    fn snd_card_unref(card: *mut snd_card);
    fn snd_ctl_find_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> *mut snd_kcontrol;
    fn snd_ctl_get_ioff(kctl: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> c_uint;
    fn snd_ctl_register_layer(ops: *mut snd_ctl_layer_ops);
    fn snd_ctl_disconnect_layer(ops: *mut snd_ctl_layer_ops);
    fn snd_BUG_ON(cond: bool_t) -> bool_t;
}

unsafe fn container_of<T, U>(ptr: *mut U, member_offset: usize) -> *mut T {
    (ptr as *mut u8).sub(member_offset) as *mut T
}

unsafe fn list_entry<T>(ptr: *mut list_head, member_offset: usize) -> *mut T {
    container_of(ptr, member_offset)
}

unsafe fn access_to_group(access: c_uint) -> c_uint {
    ((access & SNDRV_CTL_ELEM_ACCESS_LED_MASK) >> SNDRV_CTL_ELEM_ACCESS_LED_SHIFT) - 1
}

unsafe fn group_to_access(group: c_uint) -> c_uint {
    (group + 1) << SNDRV_CTL_ELEM_ACCESS_LED_SHIFT
}

unsafe fn snd_ctl_led_get_by_access(access: c_uint) -> *mut snd_ctl_led {
    let group = access_to_group(access);
    if group >= MAX_LED as c_uint {
        return ptr::null_mut();
    }
    &mut snd_ctl_leds[group as usize]
}

/*
 * A note for callers:
 *   The two static variables info and value are protected using snd_ctl_led_mutex.
 */
unsafe fn snd_ctl_led_get(lctl: *mut snd_ctl_led_ctl) -> c_int {
    static mut info: snd_ctl_elem_info = unsafe { core::mem::zeroed() };
    static mut value: snd_ctl_elem_value = unsafe { core::mem::zeroed() };
    let kctl = (*lctl).kctl;
    let mut i: c_uint;
    let mut result: c_int;

    memset(&mut info as *mut _ as *mut c_void, 0, size_of::<snd_ctl_elem_info>());
    info.id = (*kctl).id;
    info.id.index = info.id.index.wrapping_add((*lctl).index_offset);
    info.id.numid = info.id.numid.wrapping_add((*lctl).index_offset);
    result = ((*kctl).info.unwrap())(kctl, &mut info);
    if result < 0 {
        return -1;
    }
    memset(&mut value as *mut _ as *mut c_void, 0, size_of::<snd_ctl_elem_value>());
    value.id = info.id;
    result = ((*kctl).get.unwrap())(kctl, &mut value);
    if result < 0 {
        return -1;
    }
    if info.type_ == SNDRV_CTL_ELEM_TYPE_BOOLEAN || info.type_ == SNDRV_CTL_ELEM_TYPE_INTEGER {
        i = 0;
        while i < info.count {
            if value.value.integer.value[i as usize] != info.value.integer.min {
                return 1;
            }
            i += 1;
        }
    } else if info.type_ == SNDRV_CTL_ELEM_TYPE_INTEGER64 {
        i = 0;
        while i < info.count {
            if value.value.integer64.value[i as usize] != info.value.integer64.min {
                return 1;
            }
            i += 1;
        }
    }
    0
}

unsafe fn update_route(route: &mut c_int, cb: c_int) {
    let route2 = cb;
    if route2 >= 0 {
        *route = if *route < 0 { route2 } else { *route | route2 };
    }
}

unsafe fn snd_ctl_led_set_state(
    card: *mut snd_card,
    access: c_uint,
    kctl: *mut snd_kcontrol,
    ioff: c_uint,
) {
    let led = snd_ctl_led_get_by_access(access);
    let mut lctl: *mut snd_ctl_led_ctl;
    let mut route: c_int;
    let mut found: bool_t;

    if led.is_null() {
        return;
    }
    route = -1;
    found = false;
    mutex_lock(&mut snd_ctl_led_mutex);
    if !card.is_null() && !snd_ctl_led_card_valid[(*card).number as usize] {
        mutex_unlock(&mut snd_ctl_led_mutex);
        return;
    }
    let controls = &mut (*led).controls as *mut list_head;
    let mut pos = (*controls).next;
    while pos != controls {
        lctl = list_entry(pos, offset_of!(snd_ctl_led_ctl, list));
        if (*lctl).kctl == kctl && (*lctl).index_offset == ioff {
            found = true;
        }
        update_route(&mut route, snd_ctl_led_get(lctl));
        pos = (*pos).next;
    }
    if !found && !kctl.is_null() && !card.is_null() {
        lctl = kzalloc(size_of::<snd_ctl_led_ctl>(), GFP_KERNEL) as *mut snd_ctl_led_ctl;
        if !lctl.is_null() {
            (*lctl).card = card;
            (*lctl).access = access;
            (*lctl).kctl = kctl;
            (*lctl).index_offset = ioff;
            list_add(&mut (*lctl).list, &mut (*led).controls);
            update_route(&mut route, snd_ctl_led_get(lctl));
        }
    }
    mutex_unlock(&mut snd_ctl_led_mutex);

    match (*led).mode {
        snd_ctl_led_mode::MODE_OFF => route = 1,
        snd_ctl_led_mode::MODE_ON => route = 0,
        snd_ctl_led_mode::MODE_FOLLOW_ROUTE => {
            if route >= 0 {
                route ^= 1;
            }
        }
        snd_ctl_led_mode::MODE_FOLLOW_MUTE => {}
    }
    if route >= 0 {
        let trig = snd_ctl_ledtrig_audio[(*led).trigger_type as usize];
        led_trigger_event(trig, if route != 0 { LED_OFF } else { LED_ON });
    }
}

unsafe fn snd_ctl_led_find(kctl: *mut snd_kcontrol, ioff: c_uint) -> *mut snd_ctl_led_ctl {
    let mut group: c_uint = 0;
    while group < MAX_LED as c_uint {
        let controls = &mut snd_ctl_leds[group as usize].controls as *mut list_head;
        let mut pos = (*controls).next;
        while pos != controls {
            let lctl: *mut snd_ctl_led_ctl = list_entry(pos, offset_of!(snd_ctl_led_ctl, list));
            if (*lctl).kctl == kctl && (*lctl).index_offset == ioff {
                return lctl;
            }
            pos = (*pos).next;
        }
        group += 1;
    }
    ptr::null_mut()
}

unsafe fn snd_ctl_led_remove(kctl: *mut snd_kcontrol, ioff: c_uint, access: c_uint) -> c_uint {
    let mut ret: c_uint = 0;

    mutex_lock(&mut snd_ctl_led_mutex);
    let lctl = snd_ctl_led_find(kctl, ioff);
    if !lctl.is_null() && (access == 0 || access != (*lctl).access) {
        ret = (*lctl).access;
        list_del(&mut (*lctl).list);
        kfree(lctl as *mut c_void);
    }
    mutex_unlock(&mut snd_ctl_led_mutex);
    ret
}

unsafe fn snd_ctl_led_notify(
    card: *mut snd_card,
    mask: c_uint,
    kctl: *mut snd_kcontrol,
    ioff: c_uint,
) {
    let vd: *mut snd_kcontrol_volatile;
    let mut access: c_uint;
    let access2: c_uint;

    if mask == SNDRV_CTL_EVENT_MASK_REMOVE {
        access = snd_ctl_led_remove(kctl, ioff, 0);
        if access != 0 {
            snd_ctl_led_set_state(card, access, ptr::null_mut(), 0);
        }
    } else if (mask & SNDRV_CTL_EVENT_MASK_INFO) != 0 {
        vd = (*kctl).vd.add(ioff as usize);
        access = (*vd).access & SNDRV_CTL_ELEM_ACCESS_LED_MASK;
        access2 = snd_ctl_led_remove(kctl, ioff, access);
        if access2 != 0 {
            snd_ctl_led_set_state(card, access2, ptr::null_mut(), 0);
        }
        if access != 0 {
            snd_ctl_led_set_state(card, access, kctl, ioff);
        }
    } else if (mask & (SNDRV_CTL_EVENT_MASK_ADD | SNDRV_CTL_EVENT_MASK_VALUE)) != 0 {
        vd = (*kctl).vd.add(ioff as usize);
        access = (*vd).access & SNDRV_CTL_ELEM_ACCESS_LED_MASK;
        if access != 0 {
            snd_ctl_led_set_state(card, access, kctl, ioff);
        }
    }
}

unsafe fn snd_ctl_led_set_id(
    card_number: c_int,
    id: *mut snd_ctl_elem_id,
    group: c_uint,
    set: bool_t,
) -> c_int {
    let card = snd_card_ref(card_number);
    if card.is_null() {
        return -ENXIO;
    }
    down_write(&mut (*card).controls_rwsem);
    let kctl = snd_ctl_find_id(card, id);
    if kctl.is_null() {
        up_write(&mut (*card).controls_rwsem);
        snd_card_unref(card);
        return -ENOENT;
    }
    if (*kctl).info.is_none() || (*kctl).get.is_none() {
        up_write(&mut (*card).controls_rwsem);
        snd_card_unref(card);
        return -EINVAL;
    }
    let ioff = snd_ctl_get_ioff(kctl, id);
    let vd = (*kctl).vd.add(ioff as usize);
    let access = (*vd).access & SNDRV_CTL_ELEM_ACCESS_LED_MASK;
    if access != 0 && access != group_to_access(group) {
        up_write(&mut (*card).controls_rwsem);
        snd_card_unref(card);
        return -EXDEV;
    }
    let mut new_access = (*vd).access & !SNDRV_CTL_ELEM_ACCESS_LED_MASK;
    if set {
        new_access |= group_to_access(group);
    }
    if new_access != (*vd).access {
        (*vd).access = new_access;
        snd_ctl_led_notify(card, SNDRV_CTL_EVENT_MASK_INFO, kctl, ioff);
    }
    up_write(&mut (*card).controls_rwsem);
    snd_card_unref(card);
    0
}

unsafe fn snd_ctl_led_refresh() {
    let mut group: c_uint = 0;
    while group < MAX_LED as c_uint {
        snd_ctl_led_set_state(ptr::null_mut(), group_to_access(group), ptr::null_mut(), 0);
        group += 1;
    }
}

unsafe fn snd_ctl_led_ctl_destroy(lctl: *mut snd_ctl_led_ctl) {
    list_del(&mut (*lctl).list);
    kfree(lctl as *mut c_void);
}

unsafe fn snd_ctl_led_clean(card: *mut snd_card) {
    let mut group: c_uint = 0;
    while group < MAX_LED as c_uint {
        let led = &mut snd_ctl_leds[group as usize] as *mut snd_ctl_led;
        let head = &mut (*led).controls as *mut list_head;
        let mut pos = (*head).next;
        while pos != head {
            let next = (*pos).next;
            let lctl: *mut snd_ctl_led_ctl = list_entry(pos, offset_of!(snd_ctl_led_ctl, list));
            if card.is_null() || (*lctl).card == card {
                snd_ctl_led_ctl_destroy(lctl);
            }
            pos = next;
        }
        group += 1;
    }
}

unsafe fn snd_ctl_led_reset(card_number: c_int, group: c_uint) -> c_int {
    let mut change: bool_t = false;
    let card = snd_card_ref(card_number);

    if card.is_null() {
        return -ENXIO;
    }
    mutex_lock(&mut snd_ctl_led_mutex);
    if !snd_ctl_led_card_valid[card_number as usize] {
        mutex_unlock(&mut snd_ctl_led_mutex);
        snd_card_unref(card);
        return -ENXIO;
    }
    let led = &mut snd_ctl_leds[group as usize] as *mut snd_ctl_led;
    let head = &mut (*led).controls as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        let lctl: *mut snd_ctl_led_ctl = list_entry(pos, offset_of!(snd_ctl_led_ctl, list));
        if (*lctl).card == card {
            let vd = (*(*lctl).kctl).vd.add((*lctl).index_offset as usize);
            (*vd).access &= !group_to_access(group);
            snd_ctl_led_ctl_destroy(lctl);
            change = true;
        }
        pos = next;
    }
    mutex_unlock(&mut snd_ctl_led_mutex);
    snd_card_unref(card);
    if change {
        snd_ctl_led_set_state(ptr::null_mut(), group_to_access(group), ptr::null_mut(), 0);
    }
    0
}

unsafe fn snd_ctl_led_register(card: *mut snd_card) {
    if snd_BUG_ON((*card).number < 0 || (*card).number as usize >= SNDRV_CARDS) {
        return;
    }
    mutex_lock(&mut snd_ctl_led_mutex);
    snd_ctl_led_card_valid[(*card).number as usize] = true;
    mutex_unlock(&mut snd_ctl_led_mutex);
    /* the register callback is already called with held card->controls_rwsem */
    let head = &mut (*card).controls as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let kctl: *mut snd_kcontrol = list_entry(pos, offset_of!(snd_kcontrol, list));
        let mut ioff: c_uint = 0;
        while ioff < (*kctl).count {
            snd_ctl_led_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, kctl, ioff);
            ioff += 1;
        }
        pos = (*pos).next;
    }
    snd_ctl_led_refresh();
    snd_ctl_led_sysfs_add(card);
}

unsafe fn snd_ctl_led_disconnect(card: *mut snd_card) {
    snd_ctl_led_sysfs_remove(card);
    mutex_lock(&mut snd_ctl_led_mutex);
    snd_ctl_led_card_valid[(*card).number as usize] = false;
    snd_ctl_led_clean(card);
    mutex_unlock(&mut snd_ctl_led_mutex);
    snd_ctl_led_refresh();
}

unsafe extern "C" fn snd_ctl_led_card_release(dev: *mut device) {
    let led_card: *mut snd_ctl_led_card = container_of(dev, offset_of!(snd_ctl_led_card, dev));
    kfree(led_card as *mut c_void);
}

unsafe extern "C" fn snd_ctl_led_release(_dev: *mut device) {}

unsafe extern "C" fn snd_ctl_led_dev_release(_dev: *mut device) {}

/*
 * sysfs
 */

unsafe extern "C" fn mode_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let led: *mut snd_ctl_led = container_of(dev, offset_of!(snd_ctl_led, dev));
    let str_: *const c_char = match (*led).mode {
        snd_ctl_led_mode::MODE_FOLLOW_MUTE => b"follow-mute\0".as_ptr() as *const c_char,
        snd_ctl_led_mode::MODE_FOLLOW_ROUTE => b"follow-route\0".as_ptr() as *const c_char,
        snd_ctl_led_mode::MODE_ON => b"on\0".as_ptr() as *const c_char,
        snd_ctl_led_mode::MODE_OFF => b"off\0".as_ptr() as *const c_char,
    };
    sysfs_emit(buf, b"%s\n\0".as_ptr() as *const c_char, str_)
}

unsafe extern "C" fn mode_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let led: *mut snd_ctl_led = container_of(dev, offset_of!(snd_ctl_led, dev));
    let mut _buf = [0 as c_char; 16];
    let l = core::cmp::min(count, size_of::<[c_char; 16]>() - 1);
    let mode: snd_ctl_led_mode;

    memcpy(_buf.as_mut_ptr() as *mut c_void, buf as *const c_void, l);
    _buf[l] = 0;
    if !strstr(_buf.as_ptr(), b"mute\0".as_ptr() as *const c_char).is_null() {
        mode = snd_ctl_led_mode::MODE_FOLLOW_MUTE;
    } else if !strstr(_buf.as_ptr(), b"route\0".as_ptr() as *const c_char).is_null() {
        mode = snd_ctl_led_mode::MODE_FOLLOW_ROUTE;
    } else if strncmp(_buf.as_ptr(), b"off\0".as_ptr() as *const c_char, 3) == 0
        || strncmp(_buf.as_ptr(), b"0\0".as_ptr() as *const c_char, 1) == 0
    {
        mode = snd_ctl_led_mode::MODE_OFF;
    } else if strncmp(_buf.as_ptr(), b"on\0".as_ptr() as *const c_char, 2) == 0
        || strncmp(_buf.as_ptr(), b"1\0".as_ptr() as *const c_char, 1) == 0
    {
        mode = snd_ctl_led_mode::MODE_ON;
    } else {
        return count as ssize_t;
    }

    mutex_lock(&mut snd_ctl_led_mutex);
    (*led).mode = mode;
    mutex_unlock(&mut snd_ctl_led_mutex);

    snd_ctl_led_set_state(ptr::null_mut(), group_to_access((*led).group), ptr::null_mut(), 0);
    count as ssize_t
}

unsafe extern "C" fn brightness_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let led: *mut snd_ctl_led = container_of(dev, offset_of!(snd_ctl_led, dev));
    let trig = snd_ctl_ledtrig_audio[(*led).trigger_type as usize];

    sysfs_emit(buf, b"%u\n\0".as_ptr() as *const c_char, led_trigger_get_brightness(trig))
}

// DEVICE_ATTR_RW(mode);
// DEVICE_ATTR_RO(brightness);

static mut snd_ctl_led_dev_attrs: [*mut attribute; 3] = unsafe {
    [
        &mut dev_attr_mode.attr as *mut attribute,
        &mut dev_attr_brightness.attr as *mut attribute,
        ptr::null_mut(),
    ]
};

static mut snd_ctl_led_dev_attr_group: attribute_group = attribute_group {
    attrs: unsafe { snd_ctl_led_dev_attrs.as_mut_ptr() },
};

static mut snd_ctl_led_dev_attr_groups: [*const attribute_group; 2] = unsafe {
    [
        &snd_ctl_led_dev_attr_group as *const attribute_group,
        ptr::null(),
    ]
};

unsafe fn find_eos(mut s: *mut c_char) -> *mut c_char {
    while *s != 0 && *s != b',' as c_char {
        s = s.add(1);
    }
    if *s != 0 {
        s = s.add(1);
    }
    s
}

unsafe fn parse_uint(s: *mut c_char, val: *mut c_uint) -> *mut c_char {
    let mut res: c_ulonglong = 0;
    if kstrtoull(s, 10, &mut res) != 0 {
        res = 0;
    }
    *val = res as c_uint;
    find_eos(s)
}

unsafe fn parse_string(mut s: *mut c_char, mut val: *mut c_char, mut val_size: size_t) -> *mut c_char {
    if *s == b'"' as c_char || *s == b'\'' as c_char {
        let c = *s;
        s = s.add(1);
        while *s != 0 && *s != c {
            if val_size > 1 {
                *val = *s;
                val = val.add(1);
                val_size -= 1;
            }
            s = s.add(1);
        }
    } else {
        while *s != 0 && *s != b',' as c_char {
            if val_size > 1 {
                *val = *s;
                val = val.add(1);
                val_size -= 1;
            }
            s = s.add(1);
        }
    }
    *val = 0;
    if *s != 0 {
        s = s.add(1);
    }
    s
}

unsafe fn parse_iface(s: *mut c_char, val: *mut snd_ctl_elem_iface_t) -> *mut c_char {
    if strncasecmp(s, b"card\0".as_ptr() as *const c_char, 4) == 0 {
        *val = SNDRV_CTL_ELEM_IFACE_CARD;
    } else if strncasecmp(s, b"mixer\0".as_ptr() as *const c_char, 5) == 0 {
        *val = SNDRV_CTL_ELEM_IFACE_MIXER;
    }
    find_eos(s)
}

/*
 * These types of input strings are accepted:
 *
 *   unsigned integer - numid (equivaled to numid=UINT)
 *   string - basic mixer name (equivalent to iface=MIXER,name=STR)
 *   numid=UINT
 *   [iface=MIXER,][device=UINT,][subdevice=UINT,]name=STR[,index=UINT]
 */
unsafe fn set_led_id(
    led_card: *mut snd_ctl_led_card,
    buf: *const c_char,
    count: size_t,
    attach: bool_t,
) -> ssize_t {
    let mut buf2 = [0 as c_char; 256];
    let mut id: snd_ctl_elem_id = core::mem::zeroed();
    let mut err: c_int;

    if strscpy(buf2.as_mut_ptr(), buf, size_of::<[c_char; 256]>()) < 0 {
        return -(E2BIG as ssize_t);
    }
    id.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    let mut s = buf2.as_mut_ptr();
    while *s != 0 {
        let os = s;
        if strncasecmp(s, b"numid=\0".as_ptr() as *const c_char, 6) == 0 {
            s = parse_uint(s.add(6), &mut id.numid);
        } else if strncasecmp(s, b"iface=\0".as_ptr() as *const c_char, 6) == 0 {
            s = parse_iface(s.add(6), &mut id.iface);
        } else if strncasecmp(s, b"device=\0".as_ptr() as *const c_char, 7) == 0 {
            s = parse_uint(s.add(7), &mut id.device);
        } else if strncasecmp(s, b"subdevice=\0".as_ptr() as *const c_char, 10) == 0 {
            s = parse_uint(s.add(10), &mut id.subdevice);
        } else if strncasecmp(s, b"name=\0".as_ptr() as *const c_char, 5) == 0 {
            s = parse_string(s.add(5), id.name.as_mut_ptr(), id.name.len());
        } else if strncasecmp(s, b"index=\0".as_ptr() as *const c_char, 6) == 0 {
            s = parse_uint(s.add(6), &mut id.index);
        } else if s == buf2.as_mut_ptr() {
            while *s != 0 {
                if *s < b'0' as c_char || *s > b'9' as c_char {
                    break;
                }
                s = s.add(1);
            }
            if *s == 0 {
                parse_uint(buf2.as_mut_ptr(), &mut id.numid);
            } else {
                while *s >= b' ' as c_char {
                    s = s.add(1);
                }
                *s = 0;
                strscpy(id.name.as_mut_ptr(), buf2.as_ptr(), id.name.len());
            }
            break;
        }
        if *s == b',' as c_char {
            s = s.add(1);
        }
        if s == os {
            break;
        }
    }

    err = snd_ctl_led_set_id((*led_card).number, &mut id, (*(*led_card).led).group, attach);
    if err < 0 {
        return err as ssize_t;
    }

    count as ssize_t
}

unsafe extern "C" fn attach_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let led_card: *mut snd_ctl_led_card = container_of(dev, offset_of!(snd_ctl_led_card, dev));
    set_led_id(led_card, buf, count, true)
}

unsafe extern "C" fn detach_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let led_card: *mut snd_ctl_led_card = container_of(dev, offset_of!(snd_ctl_led_card, dev));
    set_led_id(led_card, buf, count, false)
}

unsafe extern "C" fn reset_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let led_card: *mut snd_ctl_led_card = container_of(dev, offset_of!(snd_ctl_led_card, dev));
    let err: c_int;

    if count > 0 && *buf == b'1' as c_char {
        err = snd_ctl_led_reset((*led_card).number, (*(*led_card).led).group);
        if err < 0 {
            return err as ssize_t;
        }
    }
    count as ssize_t
}

unsafe extern "C" fn list_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let led_card: *mut snd_ctl_led_card = container_of(dev, offset_of!(snd_ctl_led_card, dev));
    let mut l: size_t = 0;
    let card = snd_card_ref((*led_card).number);

    if card.is_null() {
        return -(ENXIO as ssize_t);
    }
    down_read(&mut (*card).controls_rwsem);
    mutex_lock(&mut snd_ctl_led_mutex);
    if snd_ctl_led_card_valid[(*led_card).number as usize] {
        let head = &mut (*(*led_card).led).controls as *mut list_head;
        let mut pos = (*head).next;
        while pos != head {
            let lctl: *mut snd_ctl_led_ctl = list_entry(pos, offset_of!(snd_ctl_led_ctl, list));
            if (*lctl).card != card {
                pos = (*pos).next;
                continue;
            }
            if l != 0 {
                l += sysfs_emit_at(buf, l, b" \0".as_ptr() as *const c_char) as size_t;
            }
            l += sysfs_emit_at(
                buf,
                l,
                b"%u\0".as_ptr() as *const c_char,
                (*(*lctl).kctl).id.numid.wrapping_add((*lctl).index_offset),
            ) as size_t;
            pos = (*pos).next;
        }
    }
    mutex_unlock(&mut snd_ctl_led_mutex);
    up_read(&mut (*card).controls_rwsem);
    snd_card_unref(card);
    l as ssize_t
}

// DEVICE_ATTR_WO(attach);
// DEVICE_ATTR_WO(detach);
// DEVICE_ATTR_WO(reset);
// DEVICE_ATTR_RO(list);

static mut snd_ctl_led_card_attrs: [*mut attribute; 5] = unsafe {
    [
        &mut dev_attr_attach.attr as *mut attribute,
        &mut dev_attr_detach.attr as *mut attribute,
        &mut dev_attr_reset.attr as *mut attribute,
        &mut dev_attr_list.attr as *mut attribute,
        ptr::null_mut(),
    ]
};

static mut snd_ctl_led_card_attr_group: attribute_group = attribute_group {
    attrs: unsafe { snd_ctl_led_card_attrs.as_mut_ptr() },
};

static mut snd_ctl_led_card_attr_groups: [*const attribute_group; 2] = unsafe {
    [
        &snd_ctl_led_card_attr_group as *const attribute_group,
        ptr::null(),
    ]
};

static mut snd_ctl_led_dev: device = unsafe { core::mem::zeroed() };

unsafe fn snd_ctl_led_sysfs_add(card: *mut snd_card) {
    let mut group: c_uint = 0;
    let mut link_name = [0 as c_char; 32];

    while group < MAX_LED as c_uint {
        let led = &mut snd_ctl_leds[group as usize] as *mut snd_ctl_led;
        let led_card = kzalloc(size_of::<snd_ctl_led_card>(), GFP_KERNEL) as *mut snd_ctl_led_card;
        if led_card.is_null() {
            dev_err((*card).dev, b"snd_ctl_led: unable to add card%d\0".as_ptr() as *const c_char, (*card).number);
            group += 1;
            continue;
        }
        (*led_card).number = (*card).number;
        (*led_card).led = led;
        device_initialize(&mut (*led_card).dev);
        (*led_card).dev.release = Some(snd_ctl_led_card_release);
        if dev_set_name(&mut (*led_card).dev, b"card%d\0".as_ptr() as *const c_char, (*card).number) < 0 {
            put_device(&mut (*led_card).dev);
            dev_err((*card).dev, b"snd_ctl_led: unable to add card%d\0".as_ptr() as *const c_char, (*card).number);
            group += 1;
            continue;
        }
        (*led_card).dev.parent = &mut (*led).dev;
        (*led_card).dev.groups = snd_ctl_led_card_attr_groups.as_ptr();
        if device_add(&mut (*led_card).dev) != 0 {
            put_device(&mut (*led_card).dev);
            dev_err((*card).dev, b"snd_ctl_led: unable to add card%d\0".as_ptr() as *const c_char, (*card).number);
            group += 1;
            continue;
        }
        (*led).cards[(*card).number as usize] = led_card;
        snprintf(link_name.as_mut_ptr(), link_name.len(), b"led-%s\0".as_ptr() as *const c_char, (*led).name);
        if sysfs_create_link(&mut (*(*card).ctl_dev).kobj, &mut (*led_card).dev.kobj, link_name.as_ptr()) != 0 {
            dev_err(
                (*card).dev,
                b"%s: can't create symlink to controlC%i device\n\0".as_ptr() as *const c_char,
                b"snd_ctl_led_sysfs_add\0".as_ptr() as *const c_char,
                (*card).number,
            );
        }
        if sysfs_create_link(
            &mut (*led_card).dev.kobj,
            &mut (*card).card_dev.kobj,
            b"card\0".as_ptr() as *const c_char,
        ) != 0 {
            dev_err(
                (*card).dev,
                b"%s: can't create symlink to card%i\n\0".as_ptr() as *const c_char,
                b"snd_ctl_led_sysfs_add\0".as_ptr() as *const c_char,
                (*card).number,
            );
        }
        group += 1;
    }
}

unsafe fn snd_ctl_led_sysfs_remove(card: *mut snd_card) {
    let mut group: c_uint = 0;
    let mut link_name = [0 as c_char; 32];

    while group < MAX_LED as c_uint {
        let led = &mut snd_ctl_leds[group as usize] as *mut snd_ctl_led;
        let led_card = (*led).cards[(*card).number as usize];
        if led_card.is_null() {
            group += 1;
            continue;
        }
        snprintf(link_name.as_mut_ptr(), link_name.len(), b"led-%s\0".as_ptr() as *const c_char, (*led).name);
        sysfs_remove_link(&mut (*(*card).ctl_dev).kobj, link_name.as_ptr());
        sysfs_remove_link(&mut (*led_card).dev.kobj, b"card\0".as_ptr() as *const c_char);
        device_unregister(&mut (*led_card).dev);
        (*led).cards[(*card).number as usize] = ptr::null_mut();
        group += 1;
    }
}

/*
 * Control layer registration
 */
static mut snd_ctl_led_lops: snd_ctl_layer_ops = snd_ctl_layer_ops {
    module_name: SND_CTL_LAYER_MODULE_LED.as_ptr() as *const c_char,
    lregister: Some(snd_ctl_led_register),
    ldisconnect: Some(snd_ctl_led_disconnect),
    lnotify: Some(snd_ctl_led_notify),
};

unsafe fn snd_ctl_led_init() -> c_int {
    led_trigger_register_simple(
        b"audio-mute\0".as_ptr() as *const c_char,
        &mut snd_ctl_ledtrig_audio[LED_AUDIO_MUTE as usize],
    );
    led_trigger_register_simple(
        b"audio-micmute\0".as_ptr() as *const c_char,
        &mut snd_ctl_ledtrig_audio[LED_AUDIO_MICMUTE as usize],
    );

    device_initialize(&mut snd_ctl_led_dev);
    snd_ctl_led_dev.class = &mut sound_class;
    snd_ctl_led_dev.release = Some(snd_ctl_led_dev_release);
    dev_set_name(&mut snd_ctl_led_dev, b"ctl-led\0".as_ptr() as *const c_char);
    if device_add(&mut snd_ctl_led_dev) != 0 {
        put_device(&mut snd_ctl_led_dev);
        return -ENOMEM;
    }
    let mut group: c_uint = 0;
    while group < MAX_LED as c_uint {
        let led = &mut snd_ctl_leds[group as usize] as *mut snd_ctl_led;
        INIT_LIST_HEAD(&mut (*led).controls);
        device_initialize(&mut (*led).dev);
        (*led).dev.parent = &mut snd_ctl_led_dev;
        (*led).dev.release = Some(snd_ctl_led_release);
        (*led).dev.groups = snd_ctl_led_dev_attr_groups.as_ptr();
        dev_set_name(&mut (*led).dev, (*led).name);
        if device_add(&mut (*led).dev) != 0 {
            put_device(&mut (*led).dev);
            while group > 0 {
                group -= 1;
                let prev_led = &mut snd_ctl_leds[group as usize] as *mut snd_ctl_led;
                device_unregister(&mut (*prev_led).dev);
            }
            device_unregister(&mut snd_ctl_led_dev);
            return -ENOMEM;
        }
        group += 1;
    }
    snd_ctl_register_layer(&mut snd_ctl_led_lops);
    0
}

unsafe fn snd_ctl_led_exit() {
    snd_ctl_disconnect_layer(&mut snd_ctl_led_lops);
    let mut card_number: c_uint = 0;
    while card_number < SNDRV_CARDS as c_uint {
        if snd_ctl_led_card_valid[card_number as usize] {
            let card = snd_card_ref(card_number as c_int);
            if !card.is_null() {
                snd_ctl_led_sysfs_remove(card);
                snd_card_unref(card);
            }
        }
        card_number += 1;
    }
    let mut group: c_uint = 0;
    while group < MAX_LED as c_uint {
        let led = &mut snd_ctl_leds[group as usize] as *mut snd_ctl_led;
        device_unregister(&mut (*led).dev);
        group += 1;
    }
    device_unregister(&mut snd_ctl_led_dev);
    snd_ctl_led_clean(ptr::null_mut());

    led_trigger_unregister_simple(snd_ctl_ledtrig_audio[LED_AUDIO_MUTE as usize]);
    led_trigger_unregister_simple(snd_ctl_ledtrig_audio[LED_AUDIO_MICMUTE as usize]);
}

// module_init(snd_ctl_led_init)
// module_exit(snd_ctl_led_exit)
// MODULE_ALIAS("ledtrig:audio-mute");
// MODULE_ALIAS("ledtrig:audio-micmute");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
