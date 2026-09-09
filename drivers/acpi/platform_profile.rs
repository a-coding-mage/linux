// SPDX-License-Identifier: GPL-2.0-or-later

/* Platform profile sysfs interface */
// C includes and kernel macros are supplied by the surrounding kernel bindings.

use core::ffi::{c_char, c_int, c_ulong, c_void};

const PLATFORM_PROFILE_LOW_POWER: usize = 0;
const PLATFORM_PROFILE_COOL: usize = 1;
const PLATFORM_PROFILE_QUIET: usize = 2;
const PLATFORM_PROFILE_BALANCED: usize = 3;
const PLATFORM_PROFILE_BALANCED_PERFORMANCE: usize = 4;
const PLATFORM_PROFILE_PERFORMANCE: usize = 5;
const PLATFORM_PROFILE_MAX_POWER: usize = 6;
const PLATFORM_PROFILE_CUSTOM: usize = 7;
const PLATFORM_PROFILE_LAST: usize = 8;
const PROFILE_BITS: usize = 1;

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct attribute { pub mode: u16 }
#[repr(C)] pub struct device_attribute { pub attr: attribute }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute }
#[repr(C)] pub struct attribute_group { pub attrs: *mut *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> u16> }
#[repr(C)] pub struct class { pub name: *const c_char, pub dev_groups: *const *const attribute_group, pub dev_release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct device { pub kobj: kobject, pub class: *const class, pub parent: *mut device }
#[repr(C)] pub struct platform_profile_ops {
    pub profile_set: Option<unsafe extern "C" fn(*mut device, c_int) -> c_int>,
    pub profile_get: Option<unsafe extern "C" fn(*mut device, *mut c_int) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut c_void, *mut c_ulong) -> c_int>,
    pub hidden_choices: Option<unsafe extern "C" fn(*mut c_void, *mut c_ulong) -> c_int>,
}
#[repr(C)] struct platform_profile_handler { name: *const c_char, dev: device, minor: c_int, choices: [c_ulong; PROFILE_BITS], hidden_choices: [c_ulong; PROFILE_BITS], ops: *const platform_profile_ops }
#[repr(C)] struct aggregate_choices_data { aggregate: [c_ulong; PROFILE_BITS], count: c_int }

extern "C" {
    static mut profile_lock: mutex;
    static mut platform_profile_ida: ida;
    static mut acpi_kobj: *mut kobject;
    static acpi_disabled: bool;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sysfs_emit_at(buf: *mut c_char, at: isize, fmt: *const c_char, ...) -> isize;
    fn sysfs_match_string(strings: *const *const c_char, buf: *const c_char) -> c_int;
    fn sysfs_notify(kobj: *mut kobject, dir: *const c_char, attr: *const c_char);
    fn kobject_uevent(kobj: *mut kobject, action: c_int) -> c_int;
    fn class_for_each_device(class: *const class, start: *mut device, data: *mut c_void, f: unsafe extern "C" fn(*mut device, *mut c_void) -> c_int) -> c_int;
    fn class_find_device(class: *const class, start: *mut device, data: *mut c_void, f: unsafe extern "C" fn(*mut device, *const c_void) -> c_int) -> *mut device;
    fn put_device(dev: *mut device);
    fn device_register(dev: *mut device) -> c_int;
    fn device_unregister(dev: *mut device);
    fn class_register(class: *const class) -> c_int;
    fn class_unregister(class: *const class);
    fn sysfs_create_group(kobj: *mut kobject, group: *const attribute_group) -> c_int;
    fn sysfs_remove_group(kobj: *mut kobject, group: *const attribute_group);
    fn sysfs_update_group(kobj: *mut kobject, group: *const attribute_group) -> c_int;
    fn ida_alloc(ida: *mut ida, flags: c_ulong) -> c_int;
    fn ida_free(ida: *mut ida, id: c_int);
    fn devres_alloc(release: unsafe extern "C" fn(*mut device, *mut c_void), size: usize, flags: c_ulong) -> *mut c_void;
    fn devres_free(res: *mut c_void);
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn kfree(ptr: *mut c_void);
}

static PROFILE_NAMES: [*const c_char; PLATFORM_PROFILE_LAST] = [b"low-power\0".as_ptr() as _, b"cool\0".as_ptr() as _, b"quiet\0".as_ptr() as _, b"balanced\0".as_ptr() as _, b"balanced-performance\0".as_ptr() as _, b"performance\0".as_ptr() as _, b"max-power\0".as_ptr() as _, b"custom\0".as_ptr() as _];

unsafe fn to_pprof_handler(dev: *mut device) -> *mut platform_profile_handler { dev as *mut platform_profile_handler }
unsafe extern "C" fn _commmon_choices_show(choices: *mut c_ulong, buf: *mut c_char) -> isize { let mut len = 0; for i in 0..PLATFORM_PROFILE_LAST { if (*choices & (1 << i)) != 0 { len += sysfs_emit_at(buf, len, if len == 0 { b"%s\0".as_ptr() as _ } else { b" %s\0".as_ptr() as _ }, PROFILE_NAMES[i]); } } len += sysfs_emit_at(buf, len, b"\n\0".as_ptr() as _); len }
unsafe extern "C" fn _store_class_profile(dev: *mut device, data: *mut c_void) -> c_int { let h = &mut *to_pprof_handler(dev); let bit = *(data as *mut c_int) as usize; if (h.choices[0] & (1 << bit)) == 0 && (h.hidden_choices[0] & (1 << bit)) == 0 { return -95; } ((*h.ops).profile_set.unwrap())(dev, bit as c_int) }
unsafe extern "C" fn _notify_class_profile(dev: *mut device, _data: *mut c_void) -> c_int { let h = &mut *to_pprof_handler(dev); sysfs_notify(&mut h.dev.kobj, core::ptr::null(), b"profile\0".as_ptr() as _); kobject_uevent(&mut h.dev.kobj, 2); 0 }
unsafe fn get_class_profile(dev: *mut device, profile: *mut c_int) -> c_int { let h = &mut *to_pprof_handler(dev); let mut val = 0; let err = ((*h.ops).profile_get.unwrap())(dev, &mut val); if err != 0 { return err; } if val < 0 || val as usize >= PLATFORM_PROFILE_LAST { return -22; } *profile = val; 0 }
unsafe extern "C" fn name_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { sysfs_emit(buf, b"%s\n\0".as_ptr() as _, (*to_pprof_handler(dev)).name) }
unsafe extern "C" fn choices_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { _commmon_choices_show((*to_pprof_handler(dev)).choices.as_mut_ptr(), buf) }
unsafe extern "C" fn profile_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize { let mut p = PLATFORM_PROFILE_LAST as c_int; let e = get_class_profile(dev, &mut p); if e != 0 { return e as isize; } sysfs_emit(buf, b"%s\n\0".as_ptr() as _, PROFILE_NAMES[p as usize]) }
unsafe extern "C" fn profile_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize { let i = sysfs_match_string(PROFILE_NAMES.as_ptr(), buf); if i < 0 { return -22; } let e = _store_class_profile(dev, &i as *const _ as *mut c_void); if e != 0 { return e as isize; } sysfs_notify(acpi_kobj, core::ptr::null(), b"platform_profile\0".as_ptr() as _); count as isize }

unsafe extern "C" fn _aggregate_choices(dev: *mut device, arg: *mut c_void) -> c_int { let d = &mut *(arg as *mut aggregate_choices_data); let h = &*to_pprof_handler(dev); d.aggregate[0] &= h.choices[0] | h.hidden_choices[0]; d.count += 1; 0 }
unsafe extern "C" fn _remove_hidden_choices(dev: *mut device, arg: *mut c_void) -> c_int { let d = &mut *(arg as *mut aggregate_choices_data); let h = &*to_pprof_handler(dev); d.aggregate[0] = h.choices[0] & !h.hidden_choices[0]; 0 }
unsafe extern "C" fn platform_profile_choices_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> isize { let mut d = aggregate_choices_data { aggregate: [!0], count: 0 }; let e = class_for_each_device(core::ptr::null(), core::ptr::null_mut(), &mut d as *mut _ as _, _aggregate_choices); if e != 0 { return e as isize; } if d.count == 1 { let _ = class_for_each_device(core::ptr::null(), core::ptr::null_mut(), &mut d as *mut _ as _, _remove_hidden_choices); } if d.aggregate[0] == 0 { return -22; } _commmon_choices_show(d.aggregate.as_mut_ptr(), buf) }
unsafe extern "C" fn _aggregate_profiles(dev: *mut device, data: *mut c_void) -> c_int { let p = &mut *(data as *mut c_int); let mut v = 0; let e = get_class_profile(dev, &mut v); if e != 0 { return e; } if *p != PLATFORM_PROFILE_LAST as c_int && *p != v { *p = PLATFORM_PROFILE_CUSTOM as c_int; } else { *p = v; } 0 }
unsafe extern "C" fn _store_and_notify(dev: *mut device, data: *mut c_void) -> c_int { let e = _store_class_profile(dev, data); if e != 0 { e } else { _notify_class_profile(dev, core::ptr::null_mut()) } }
unsafe extern "C" fn platform_profile_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> isize { let mut p = PLATFORM_PROFILE_LAST as c_int; let e = class_for_each_device(core::ptr::null(), core::ptr::null_mut(), &mut p as *mut _ as _, _aggregate_profiles); if e != 0 { return e as isize; } if p == PLATFORM_PROFILE_LAST as c_int { return -22; } sysfs_emit(buf, b"%s\n\0".as_ptr() as _, PROFILE_NAMES[p as usize]) }
unsafe extern "C" fn platform_profile_store(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, count: usize) -> isize { let i = sysfs_match_string(PROFILE_NAMES.as_ptr(), buf); if i < 0 || i == PLATFORM_PROFILE_CUSTOM as c_int { return -22; } let mut d = aggregate_choices_data { aggregate: [!0], count: 0 }; let e = class_for_each_device(core::ptr::null(), core::ptr::null_mut(), &mut d as *mut _ as _, _aggregate_choices); if e != 0 { return e as isize; } if d.aggregate[0] & (1 << i) == 0 { return -95; } let e = class_for_each_device(core::ptr::null(), core::ptr::null_mut(), &i as *const _ as *mut c_void, _store_and_notify); if e != 0 { return e as isize; } sysfs_notify(acpi_kobj, core::ptr::null(), b"platform_profile\0".as_ptr() as _); count as isize }

#[no_mangle] pub unsafe extern "C" fn platform_profile_notify(dev: *mut device) { _notify_class_profile(dev, core::ptr::null_mut()); sysfs_notify(acpi_kobj, core::ptr::null(), b"platform_profile\0".as_ptr() as _); }
#[no_mangle] pub unsafe extern "C" fn platform_profile_cycle() -> c_int { let mut p = PLATFORM_PROFILE_LAST as c_int; let e = class_for_each_device(core::ptr::null(), core::ptr::null_mut(), &mut p as *mut _ as _, _aggregate_profiles); if e != 0 { return e; } if p == PLATFORM_PROFILE_MAX_POWER as c_int || p == PLATFORM_PROFILE_CUSTOM as c_int || p == PLATFORM_PROFILE_LAST as c_int { return -22; } let mut d = aggregate_choices_data { aggregate: [!0], count: 0 }; let e = class_for_each_device(core::ptr::null(), core::ptr::null_mut(), &mut d as *mut _ as _, _aggregate_choices); if e != 0 { return e; } d.aggregate[0] &= !(1 << PLATFORM_PROFILE_MAX_POWER); d.aggregate[0] &= !(1 << PLATFORM_PROFILE_CUSTOM); let mut next = ((p as usize + 1)..PLATFORM_PROFILE_LAST).find(|i| d.aggregate[0] & (1 << i) != 0).unwrap_or_else(|| (0..=p as usize).find(|i| d.aggregate[0] & (1 << i) != 0).unwrap_or(PLATFORM_PROFILE_LAST)); if next == PLATFORM_PROFILE_LAST { next = 0; } class_for_each_device(core::ptr::null(), core::ptr::null_mut(), &mut next as *mut _ as _, _store_and_notify) }

#[no_mangle] pub unsafe extern "C" fn platform_profile_register(dev: *mut device, name: *const c_char, drvdata: *mut c_void, ops: *const platform_profile_ops) -> *mut device { if dev.is_null() || name.is_null() || ops.is_null() { return (-22isize) as *mut device; } let p = Box::into_raw(Box::new(core::mem::zeroed::<platform_profile_handler>())); (*p).name = name; (*p).ops = ops; let e = ((*ops).probe.unwrap())(drvdata, (*p).choices.as_mut_ptr()); if e != 0 { kfree(p as _); return e as isize as *mut device; } p as *mut device }
#[no_mangle] pub unsafe extern "C" fn platform_profile_remove(dev: *mut device) { if !dev.is_null() { device_unregister(dev); } }
#[no_mangle] pub unsafe extern "C" fn devm_platform_profile_register(dev: *mut device, name: *const c_char, drvdata: *mut c_void, ops: *const platform_profile_ops) -> *mut device { platform_profile_register(dev, name, drvdata, ops) }

// Module initialization, class/attribute declarations, release callbacks, and exported symbol metadata
// correspond to the Linux module macros and are intentionally left as binding-provided integration points.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
