/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 1999-2002 Vojtech Pavlik */
// Translated from the Linux input header. Included headers provide the
// referenced kernel types, constants, and helper macros.

pub const ABS_MT_FIRST: u32 = ABS_MT_TOUCH_MAJOR;
pub const ABS_MT_LAST: u32 = ABS_MT_TOOL_Y;

pub struct input_dev_poller;

#[repr(C)]
pub struct input_value {
    pub type_: u16,
    pub code: u16,
    pub value: i32,
}

#[repr(C)]
pub enum input_clock_type {
    INPUT_CLK_REAL = 0,
    INPUT_CLK_MONO,
    INPUT_CLK_BOOT,
    INPUT_CLK_MAX,
}

#[repr(C)]
pub struct input_dev {
    pub name: *const ::std::os::raw::c_char,
    pub phys: *const ::std::os::raw::c_char,
    pub uniq: *const ::std::os::raw::c_char,
    pub id: input_id,
    pub propbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(INPUT_PROP_CNT)],
    pub evbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(EV_CNT)],
    pub keybit: [::std::os::raw::c_ulong; BITS_TO_LONGS(KEY_CNT)],
    pub relbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(REL_CNT)],
    pub absbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(ABS_CNT)],
    pub mscbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(MSC_CNT)],
    pub ledbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(LED_CNT)],
    pub sndbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(SND_CNT)],
    pub ffbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(FF_CNT)],
    pub swbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(SW_CNT)],
    pub hint_events_per_packet: u32,
    pub keycodemax: u32,
    pub keycodesize: u32,
    pub keycode: *mut ::std::ffi::c_void,
    pub setkeycode: Option<unsafe extern "C" fn(*mut input_dev, *const input_keymap_entry, *mut u32) -> i32>,
    pub getkeycode: Option<unsafe extern "C" fn(*mut input_dev, *mut input_keymap_entry) -> i32>,
    pub ff: *mut ff_device,
    pub poller: *mut input_dev_poller,
    pub repeat_key: u32,
    pub timer: timer_list,
    pub rep: [i32; REP_CNT],
    pub mt: *mut input_mt,
    pub absinfo: *mut input_absinfo,
    pub key: [::std::os::raw::c_ulong; BITS_TO_LONGS(KEY_CNT)],
    pub led: [::std::os::raw::c_ulong; BITS_TO_LONGS(LED_CNT)],
    pub snd: [::std::os::raw::c_ulong; BITS_TO_LONGS(SND_CNT)],
    pub sw: [::std::os::raw::c_ulong; BITS_TO_LONGS(SW_CNT)],
    pub open: Option<unsafe extern "C" fn(*mut input_dev) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut input_dev)>,
    pub flush: Option<unsafe extern "C" fn(*mut input_dev, *mut file) -> i32>,
    pub event: Option<unsafe extern "C" fn(*mut input_dev, u32, u32, i32) -> i32>,
    pub grab: *mut input_handle,
    pub event_lock: spinlock_t,
    pub mutex: mutex,
    pub users: u32,
    pub going_away: bool,
    pub dev: device,
    pub h_list: list_head,
    pub node: list_head,
    pub num_vals: u32,
    pub max_vals: u32,
    pub vals: *mut input_value,
    pub devres_managed: bool,
    pub timestamp: [ktime_t; INPUT_CLK_MAX],
    pub inhibited: bool,
    pub ready: bool,
}

#[inline]
pub unsafe fn to_input_dev(d: *mut device) -> *mut input_dev {
    container_of!(d, input_dev, dev)
}

pub const INPUT_DEVICE_ID_MATCH_DEVICE: u32 = INPUT_DEVICE_ID_MATCH_BUS | INPUT_DEVICE_ID_MATCH_VENDOR | INPUT_DEVICE_ID_MATCH_PRODUCT;
pub const INPUT_DEVICE_ID_MATCH_DEVICE_AND_VERSION: u32 = INPUT_DEVICE_ID_MATCH_DEVICE | INPUT_DEVICE_ID_MATCH_VERSION;

pub struct input_handle;

#[repr(C)]
pub struct input_handler {
    pub private: *mut ::std::ffi::c_void,
    pub event: Option<unsafe extern "C" fn(*mut input_handle, u32, u32, i32)>,
    pub events: Option<unsafe extern "C" fn(*mut input_handle, *mut input_value, u32) -> u32>,
    pub filter: Option<unsafe extern "C" fn(*mut input_handle, u32, u32, i32) -> bool>,
    pub match_: Option<unsafe extern "C" fn(*mut input_handler, *mut input_dev) -> bool>,
    pub connect: Option<unsafe extern "C" fn(*mut input_handler, *mut input_dev, *const input_device_id) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut input_handle)>,
    pub start: Option<unsafe extern "C" fn(*mut input_handle)>,
    pub passive_observer: bool,
    pub legacy_minors: bool,
    pub minor: i32,
    pub name: *const ::std::os::raw::c_char,
    pub id_table: *const input_device_id,
    pub h_list: list_head,
    pub node: list_head,
}

#[repr(C)]
pub struct input_handle {
    pub private: *mut ::std::ffi::c_void,
    pub open: i32,
    pub name: *const ::std::os::raw::c_char,
    pub dev: *mut input_dev,
    pub handler: *mut input_handler,
    pub handle_events: Option<unsafe extern "C" fn(*mut input_handle, *mut input_value, u32) -> u32>,
    pub d_node: list_head,
    pub h_node: list_head,
}

extern "C" {
    pub fn input_allocate_device() -> *mut input_dev;
    pub fn devm_input_allocate_device(dev: *mut device) -> *mut input_dev;
    pub fn input_free_device(dev: *mut input_dev);
    pub fn input_register_device(dev: *mut input_dev) -> i32;
    pub fn input_unregister_device(dev: *mut input_dev);
    pub fn input_reset_device(dev: *mut input_dev);
    pub fn get_device(dev: *mut device) -> *mut device;
    pub fn put_device(dev: *mut device);
    pub fn dev_get_drvdata(dev: *mut device) -> *mut ::std::ffi::c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut ::std::ffi::c_void);
    pub fn input_setup_polling(dev: *mut input_dev, poll_fn: Option<unsafe extern "C" fn(*mut input_dev)>) -> i32;
    pub fn input_set_poll_interval(dev: *mut input_dev, interval: u32);
    pub fn input_set_min_poll_interval(dev: *mut input_dev, interval: u32);
    pub fn input_set_max_poll_interval(dev: *mut input_dev, interval: u32);
    pub fn input_get_poll_interval(dev: *mut input_dev) -> i32;
    pub fn input_register_handler(handler: *mut input_handler) -> i32;
    pub fn input_unregister_handler(handler: *mut input_handler);
    pub fn input_get_new_minor(legacy_base: i32, legacy_num: u32, allow_dynamic: bool) -> i32;
    pub fn input_free_minor(minor: u32);
    pub fn input_handler_for_each_handle(handler: *mut input_handler, data: *mut ::std::ffi::c_void, f: Option<unsafe extern "C" fn(*mut input_handle, *mut ::std::ffi::c_void) -> i32>) -> i32;
    pub fn input_register_handle(handle: *mut input_handle) -> i32;
    pub fn input_unregister_handle(handle: *mut input_handle);
    pub fn input_grab_device(handle: *mut input_handle) -> i32;
    pub fn input_release_device(handle: *mut input_handle);
    pub fn input_open_device(handle: *mut input_handle) -> i32;
    pub fn input_close_device(handle: *mut input_handle);
    pub fn input_flush_device(handle: *mut input_handle, file: *mut file) -> i32;
    pub fn input_set_timestamp(dev: *mut input_dev, timestamp: ktime_t);
    pub fn input_get_timestamp(dev: *mut input_dev) -> *mut ktime_t;
    pub fn input_event(dev: *mut input_dev, type_: u32, code: u32, value: i32);
    pub fn input_inject_event(handle: *mut input_handle, type_: u32, code: u32, value: i32);
    pub fn input_set_capability(dev: *mut input_dev, type_: u32, code: u32);
    pub fn input_alloc_absinfo(dev: *mut input_dev);
    pub fn input_set_abs_params(dev: *mut input_dev, axis: u32, min: i32, max: i32, fuzz: i32, flat: i32);
    pub fn input_copy_abs(dst: *mut input_dev, dst_axis: u32, src: *const input_dev, src_axis: u32);
    pub fn input_scancode_to_scalar(ke: *const input_keymap_entry, scancode: *mut u32) -> i32;
    pub fn input_default_setkeycode(dev: *mut input_dev, ke: *const input_keymap_entry, old_keycode: *mut u32) -> i32;
    pub fn input_get_keycode(dev: *mut input_dev, ke: *mut input_keymap_entry) -> i32;
    pub fn input_set_keycode(dev: *mut input_dev, ke: *const input_keymap_entry) -> i32;
    pub fn input_match_device_id(dev: *const input_dev, id: *const input_device_id) -> bool;
    pub fn input_enable_softrepeat(dev: *mut input_dev, delay: i32, period: i32);
    pub fn input_device_enabled(dev: *mut input_dev) -> bool;
}

#[inline]
pub unsafe fn input_get_device(dev: *mut input_dev) -> *mut input_dev {
    if !dev.is_null() { to_input_dev(get_device(&mut (*dev).dev)) } else { core::ptr::null_mut() }
}
#[inline]
pub unsafe fn input_put_device(dev: *mut input_dev) { if !dev.is_null() { put_device(&mut (*dev).dev); } }
#[inline]
pub unsafe fn input_get_drvdata(dev: *mut input_dev) -> *mut ::std::ffi::c_void { dev_get_drvdata(&mut (*dev).dev) }
#[inline]
pub unsafe fn input_set_drvdata(dev: *mut input_dev, data: *mut ::std::ffi::c_void) { dev_set_drvdata(&mut (*dev).dev, data); }

#[inline] pub unsafe fn input_report_key(dev: *mut input_dev, code: u32, value: i32) { input_event(dev, EV_KEY, code, (value != 0) as i32); }
#[inline] pub unsafe fn input_report_rel(dev: *mut input_dev, code: u32, value: i32) { input_event(dev, EV_REL, code, value); }
#[inline] pub unsafe fn input_report_abs(dev: *mut input_dev, code: u32, value: i32) { input_event(dev, EV_ABS, code, value); }
#[inline] pub unsafe fn input_report_ff_status(dev: *mut input_dev, code: u32, value: i32) { input_event(dev, EV_FF_STATUS, code, value); }
#[inline] pub unsafe fn input_report_switch(dev: *mut input_dev, code: u32, value: i32) { input_event(dev, EV_SW, code, (value != 0) as i32); }
#[inline] pub unsafe fn input_sync(dev: *mut input_dev) { input_event(dev, EV_SYN, SYN_REPORT, 0); }
#[inline] pub unsafe fn input_mt_sync(dev: *mut input_dev) { input_event(dev, EV_SYN, SYN_MT_REPORT, 0); }

#[inline]
pub unsafe fn input_set_events_per_packet(dev: *mut input_dev, n_events: i32) { (*dev).hint_events_per_packet = n_events as u32; }

#[inline] pub unsafe fn input_abs_get_val(dev: *mut input_dev, axis: u32) -> i32 { if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).value } else { 0 } }
#[inline] pub unsafe fn input_abs_get_min(dev: *mut input_dev, axis: u32) -> i32 { if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).minimum } else { 0 } }
#[inline] pub unsafe fn input_abs_get_max(dev: *mut input_dev, axis: u32) -> i32 { if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).maximum } else { 0 } }
#[inline] pub unsafe fn input_abs_get_fuzz(dev: *mut input_dev, axis: u32) -> i32 { if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).fuzz } else { 0 } }
#[inline] pub unsafe fn input_abs_get_flat(dev: *mut input_dev, axis: u32) -> i32 { if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).flat } else { 0 } }
#[inline] pub unsafe fn input_abs_get_res(dev: *mut input_dev, axis: u32) -> i32 { if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).resolution } else { 0 } }
#[inline] pub unsafe fn input_abs_set_val(dev: *mut input_dev, axis: u32, val: i32) { input_alloc_absinfo(dev); if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).value = val; } }
#[inline] pub unsafe fn input_abs_set_min(dev: *mut input_dev, axis: u32, val: i32) { input_alloc_absinfo(dev); if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).minimum = val; } }
#[inline] pub unsafe fn input_abs_set_max(dev: *mut input_dev, axis: u32, val: i32) { input_alloc_absinfo(dev); if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).maximum = val; } }
#[inline] pub unsafe fn input_abs_set_fuzz(dev: *mut input_dev, axis: u32, val: i32) { input_alloc_absinfo(dev); if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).fuzz = val; } }
#[inline] pub unsafe fn input_abs_set_flat(dev: *mut input_dev, axis: u32, val: i32) { input_alloc_absinfo(dev); if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).flat = val; } }
#[inline] pub unsafe fn input_abs_set_res(dev: *mut input_dev, axis: u32, val: i32) { input_alloc_absinfo(dev); if !(*dev).absinfo.is_null() { (*(*dev).absinfo.add(axis as usize)).resolution = val; } }

extern "C" { pub static input_class: class; }

#[repr(C)]
pub struct ff_device {
    pub upload: Option<unsafe extern "C" fn(*mut input_dev, *mut ff_effect, *mut ff_effect) -> i32>,
    pub erase: Option<unsafe extern "C" fn(*mut input_dev, i32) -> i32>,
    pub playback: Option<unsafe extern "C" fn(*mut input_dev, i32, i32) -> i32>,
    pub set_gain: Option<unsafe extern "C" fn(*mut input_dev, u16)>,
    pub set_autocenter: Option<unsafe extern "C" fn(*mut input_dev, u16)>,
    pub destroy: Option<unsafe extern "C" fn(*mut ff_device)>,
    pub stop: Option<unsafe extern "C" fn(*mut ff_device)>,
    pub private: *mut ::std::ffi::c_void,
    pub ffbit: [::std::os::raw::c_ulong; BITS_TO_LONGS(FF_CNT)],
    pub mutex: mutex,
    pub max_effects: i32,
    pub effects: *mut ff_effect,
    pub effect_owners: [*mut file; 0],
}

extern "C" {
    pub fn input_ff_create(dev: *mut input_dev, max_effects: u32) -> i32;
    pub fn input_ff_destroy(dev: *mut input_dev);
    pub fn input_ff_event(dev: *mut input_dev, type_: u32, code: u32, value: i32) -> i32;
    pub fn input_ff_upload(dev: *mut input_dev, effect: *mut ff_effect, file: *mut file) -> i32;
    pub fn input_ff_erase(dev: *mut input_dev, effect_id: i32, file: *mut file) -> i32;
    pub fn input_ff_flush(dev: *mut input_dev, file: *mut file) -> i32;
    pub fn input_ff_create_memless(dev: *mut input_dev, data: *mut ::std::ffi::c_void, play_effect: Option<unsafe extern "C" fn(*mut input_dev, *mut ::std::ffi::c_void, *mut ff_effect) -> i32>) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
