// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HD audio Component Binding Interface
 *
 * Copyright (C) 2021, 2023 Cirrus Logic, Inc. and
 *			Cirrus Logic International Semiconductor Ltd.
 */

// C dependencies:
// linux/acpi.h, linux/component.h, linux/module.h, linux/slab.h,
// sound/hda_codec.h, hda_component.h, hda_local.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type acpi_handle = *mut c_void;
type acpi_notify_handler = Option<unsafe extern "C" fn(acpi_handle, u32, *mut c_void)>;

const GFP_KERNEL: c_int = 0;
const ACPI_DEVICE_NOTIFY: u32 = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
}

#[repr(C)]
pub struct hda_codec_core {
    pub subsystem_id: u32,
}

#[repr(C)]
pub struct hda_component_parent {
    pub codec: *mut hda_codec,
    pub mutex: mutex,
    pub comps: [hda_component; HDA_COMPONENT_ARRAY_SIZE],
}

#[repr(C)]
pub struct hda_component {
    pub dev: *mut device,
    pub adev: *mut acpi_device,
    pub acpi_notify: Option<unsafe extern "C" fn(acpi_handle, u32, *mut device)>,
    pub acpi_notifications_supported: bool_,
    pub pre_playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
    pub playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
    pub post_playback_hook: Option<unsafe extern "C" fn(*mut device, c_int)>,
}

#[repr(C)]
pub struct acpi_device {
    pub handle: acpi_handle,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct component_master_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct component_match {
    _private: [u8; 0],
}

const HDA_COMPONENT_ARRAY_SIZE: usize = 0;

unsafe extern "C" {
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn hda_component_from_index(
        parent: *mut hda_component_parent,
        index: c_int,
    ) -> *mut hda_component;
    fn hda_codec_dev(cdc: *mut hda_codec) -> *mut device;

    fn acpi_device_handle(adev: *mut acpi_device) -> acpi_handle;
    fn acpi_install_notify_handler(
        handle: acpi_handle,
        typ: u32,
        handler: acpi_notify_handler,
        data: *mut c_void,
    ) -> c_int;
    fn acpi_remove_notify_handler(
        handle: acpi_handle,
        typ: u32,
        handler: acpi_notify_handler,
    ) -> c_int;

    fn component_bind_all(master: *mut device, data: *mut c_void) -> c_int;
    fn component_match_add(
        master: *mut device,
        matchptr: *mut *mut component_match,
        compare: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> c_int>,
        compare_data: *mut c_void,
    );
    fn component_master_add_with_match(
        master: *mut device,
        ops: *const component_master_ops,
        matchptr: *mut component_match,
    ) -> c_int;
    fn component_master_del(master: *mut device, ops: *const component_master_ops);

    fn devm_kmalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn dev_name(dev: *mut device) -> *const c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn isdigit(c: c_int) -> c_int;

    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_long;

    fn codec_warn(cdc: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_dbg(cdc: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_err(cdc: *mut hda_codec, fmt: *const c_char, ...);
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe {
            mutex_lock(lock);
        }
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.lock);
        }
    }
}

// CONFIG_ACPI
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_component_acpi_device_notify(
    parent: *mut hda_component_parent,
    _handle: acpi_handle,
    event: u32,
    _data: *mut c_void,
) {
    let mut comp: *mut hda_component;
    let mut i: c_int;

    let _guard = unsafe { MutexGuard::new(ptr::addr_of_mut!((*parent).mutex)) };
    i = 0;
    while (i as usize) < HDA_COMPONENT_ARRAY_SIZE {
        comp = unsafe { hda_component_from_index(parent, i) };
        if unsafe { !(*comp).dev.is_null() && (*comp).acpi_notify.is_some() } {
            unsafe {
                ((*comp).acpi_notify.unwrap())(
                    acpi_device_handle((*comp).adev),
                    event,
                    (*comp).dev,
                );
            }
        }
        i += 1;
    }
}
// EXPORT_SYMBOL_NS_GPL(hda_component_acpi_device_notify, "SND_HDA_SCODEC_COMPONENT");

// CONFIG_ACPI
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_component_manager_bind_acpi_notifications(
    cdc: *mut hda_codec,
    parent: *mut hda_component_parent,
    handler: acpi_notify_handler,
    data: *mut c_void,
) -> c_int {
    let mut support_notifications: bool_ = false;
    let adev: *mut acpi_device;
    let mut comp: *mut hda_component;
    let ret: c_int;
    let mut i: c_int;

    adev = unsafe { (*parent).comps[0].adev };
    if unsafe { acpi_device_handle(adev).is_null() } {
        return 0;
    }

    i = 0;
    while (i as usize) < HDA_COMPONENT_ARRAY_SIZE {
        comp = unsafe { hda_component_from_index(parent, i) };
        support_notifications =
            support_notifications || unsafe { (*comp).acpi_notifications_supported };
        i += 1;
    }

    if support_notifications {
        ret = unsafe { acpi_install_notify_handler((*adev).handle, ACPI_DEVICE_NOTIFY, handler, data) };
        if ret < 0 {
            unsafe {
                codec_warn(
                    cdc,
                    c"Failed to install notify handler: %d\n".as_ptr(),
                    ret,
                );
            }
            return 0;
        }

        unsafe {
            codec_dbg(cdc, c"Notify handler installed\n".as_ptr());
        }
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(hda_component_manager_bind_acpi_notifications, "SND_HDA_SCODEC_COMPONENT");

// CONFIG_ACPI
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_component_manager_unbind_acpi_notifications(
    cdc: *mut hda_codec,
    parent: *mut hda_component_parent,
    handler: acpi_notify_handler,
) {
    let adev: *mut acpi_device;
    let ret: c_int;

    adev = unsafe { (*parent).comps[0].adev };
    if unsafe { acpi_device_handle(adev).is_null() } {
        return;
    }

    ret = unsafe { acpi_remove_notify_handler((*adev).handle, ACPI_DEVICE_NOTIFY, handler) };
    if ret < 0 {
        unsafe {
            codec_warn(
                cdc,
                c"Failed to uninstall notify handler: %d\n".as_ptr(),
                ret,
            );
        }
    }
}
// EXPORT_SYMBOL_NS_GPL(hda_component_manager_unbind_acpi_notifications, "SND_HDA_SCODEC_COMPONENT");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_component_manager_playback_hook(
    parent: *mut hda_component_parent,
    action: c_int,
) {
    let mut comp: *mut hda_component;
    let mut i: c_int;

    let _guard = unsafe { MutexGuard::new(ptr::addr_of_mut!((*parent).mutex)) };
    i = 0;
    while (i as usize) < HDA_COMPONENT_ARRAY_SIZE {
        comp = unsafe { hda_component_from_index(parent, i) };
        if unsafe { !(*comp).dev.is_null() && (*comp).pre_playback_hook.is_some() } {
            unsafe {
                ((*comp).pre_playback_hook.unwrap())((*comp).dev, action);
            }
        }
        i += 1;
    }
    i = 0;
    while (i as usize) < HDA_COMPONENT_ARRAY_SIZE {
        comp = unsafe { hda_component_from_index(parent, i) };
        if unsafe { !(*comp).dev.is_null() && (*comp).playback_hook.is_some() } {
            unsafe {
                ((*comp).playback_hook.unwrap())((*comp).dev, action);
            }
        }
        i += 1;
    }
    i = 0;
    while (i as usize) < HDA_COMPONENT_ARRAY_SIZE {
        comp = unsafe { hda_component_from_index(parent, i) };
        if unsafe { !(*comp).dev.is_null() && (*comp).post_playback_hook.is_some() } {
            unsafe {
                ((*comp).post_playback_hook.unwrap())((*comp).dev, action);
            }
        }
        i += 1;
    }
}
// EXPORT_SYMBOL_NS_GPL(hda_component_manager_playback_hook, "SND_HDA_SCODEC_COMPONENT");

#[repr(C)]
struct hda_scodec_match {
    bus: *const c_char,
    hid: *const c_char,
    match_str: *const c_char,
    index: c_int,
}

/* match the device name in a slightly relaxed manner */
unsafe extern "C" fn hda_comp_match_dev_name(dev: *mut device, data: *mut c_void) -> c_int {
    let p: *mut hda_scodec_match = data as *mut hda_scodec_match;
    let d: *const c_char = unsafe { dev_name(dev) };
    let mut n: c_int = unsafe { strlen((*p).bus) as c_int };
    let mut tmp: [c_char; 32] = [0; 32];

    /* check the bus name */
    if unsafe { strncmp(d, (*p).bus, n as usize) } != 0 {
        return 0;
    }
    /* skip the bus number */
    if unsafe { isdigit(*d.add(n as usize) as c_int) } != 0 {
        n += 1;
    }
    /* the rest must be exact matching */
    unsafe {
        snprintf(
            tmp.as_mut_ptr(),
            size_of::<[c_char; 32]>(),
            (*p).match_str,
            (*p).hid,
            (*p).index,
        );
    }
    (unsafe { strcmp(d.add(n as usize), tmp.as_ptr()) } == 0) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_component_manager_bind(
    cdc: *mut hda_codec,
    parent: *mut hda_component_parent,
) -> c_int {
    /* Init shared and component specific data */
    unsafe {
        memset(
            (*parent).comps.as_mut_ptr() as *mut c_void,
            0,
            size_of::<[hda_component; HDA_COMPONENT_ARRAY_SIZE]>(),
        );
    }

    let _guard = unsafe { MutexGuard::new(ptr::addr_of_mut!((*parent).mutex)) };
    unsafe { component_bind_all(hda_codec_dev(cdc), parent as *mut c_void) }
}
// EXPORT_SYMBOL_NS_GPL(hda_component_manager_bind, "SND_HDA_SCODEC_COMPONENT");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_component_manager_init(
    cdc: *mut hda_codec,
    parent: *mut hda_component_parent,
    count: c_int,
    bus: *const c_char,
    hid: *const c_char,
    match_str: *const c_char,
    ops: *const component_master_ops,
) -> c_int {
    let dev: *mut device = unsafe { hda_codec_dev(cdc) };
    let mut match_: *mut component_match = ptr::null_mut();
    let mut sm: *mut hda_scodec_match;
    let ret: c_int;
    let mut i: c_int;

    if unsafe { !(*parent).codec.is_null() } {
        unsafe {
            codec_err(
                cdc,
                c"Component binding already created (SSID: %x)\n".as_ptr(),
                (*cdc).core.subsystem_id,
            );
        }
        return -EINVAL;
    }
    unsafe {
        (*parent).codec = cdc;
    }

    unsafe {
        mutex_init(ptr::addr_of_mut!((*parent).mutex));
    }

    i = 0;
    while i < count {
        sm = unsafe { devm_kmalloc(dev, size_of::<hda_scodec_match>(), GFP_KERNEL) }
            as *mut hda_scodec_match;
        if sm.is_null() {
            return -ENOMEM;
        }

        unsafe {
            (*sm).bus = bus;
            (*sm).hid = hid;
            (*sm).match_str = match_str;
            (*sm).index = i;
            component_match_add(
                dev,
                ptr::addr_of_mut!(match_),
                Some(hda_comp_match_dev_name),
                sm as *mut c_void,
            );
        }
        if unsafe { IS_ERR(match_ as *const c_void) } {
            unsafe {
                codec_err(
                    cdc,
                    c"Fail to add component %ld\n".as_ptr(),
                    PTR_ERR(match_ as *const c_void),
                );
            }
            return unsafe { PTR_ERR(match_ as *const c_void) as c_int };
        }
        i += 1;
    }

    ret = unsafe { component_master_add_with_match(dev, ops, match_) };
    if ret != 0 {
        unsafe {
            codec_err(
                cdc,
                c"Fail to register component aggregator %d\n".as_ptr(),
                ret,
            );
        }
    }

    ret
}
// EXPORT_SYMBOL_NS_GPL(hda_component_manager_init, "SND_HDA_SCODEC_COMPONENT");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_component_manager_free(
    parent: *mut hda_component_parent,
    ops: *const component_master_ops,
) {
    let dev: *mut device;

    if unsafe { (*parent).codec.is_null() } {
        return;
    }

    dev = unsafe { hda_codec_dev((*parent).codec) };

    unsafe {
        component_master_del(dev, ops);

        (*parent).codec = ptr::null_mut();
    }
}
// EXPORT_SYMBOL_NS_GPL(hda_component_manager_free, "SND_HDA_SCODEC_COMPONENT");

// MODULE_DESCRIPTION("HD Audio component binding library");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
