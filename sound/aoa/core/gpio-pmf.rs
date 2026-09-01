// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio pmf GPIOs
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

// Dependencies originally provided by:
// #include <linux/slab.h>
// #include <asm/pmac_feature.h>
// #include <asm/pmac_pfunc.h>
// #include "../aoa.h"

const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const KERN_WARNING: &[u8] = b"\x014\0";
const KERN_ERR: &[u8] = b"\x013\0";

type gfp_t = c_uint;
type notify_func_t = Option<unsafe extern "C" fn(data: *mut c_void)>;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub union pmf_arg {
    pub v: c_int,
    pub p: *mut c_int,
}

#[repr(C)]
pub struct pmf_args {
    pub count: c_int,
    pub u: [pmf_arg; 1],
}

#[repr(C)]
pub struct pmf_irq_client {
    pub data: *mut c_void,
    pub handler: Option<unsafe extern "C" fn(data: *mut c_void)>,
    pub owner: *mut module,
}

#[repr(C)]
pub enum notify_type {
    AOA_NOTIFY_HEADPHONE,
    AOA_NOTIFY_LINE_IN,
    AOA_NOTIFY_LINE_OUT,
}

#[repr(C)]
pub struct gpio_notification {
    pub work: delayed_work,
    pub mutex: mutex,
    pub notify: notify_func_t,
    pub data: *mut c_void,
    pub gpio_private: *mut pmf_irq_client,
}

#[repr(C)]
pub struct gpio_runtime {
    pub node: *mut device_node,
    pub implementation_private: c_int,
    pub headphone_notify: gpio_notification,
    pub line_in_notify: gpio_notification,
    pub line_out_notify: gpio_notification,
}

#[repr(C)]
pub struct gpio_methods {
    pub init: Option<unsafe extern "C" fn(rt: *mut gpio_runtime)>,
    pub exit: Option<unsafe extern "C" fn(rt: *mut gpio_runtime)>,
    pub all_amps_off: Option<unsafe extern "C" fn(rt: *mut gpio_runtime)>,
    pub all_amps_restore: Option<unsafe extern "C" fn(rt: *mut gpio_runtime)>,
    pub set_headphone: Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: c_int)>,
    pub set_speakers: Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: c_int)>,
    pub set_lineout: Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: c_int)>,
    pub set_hw_reset: Option<unsafe extern "C" fn(rt: *mut gpio_runtime, on: c_int)>,
    pub get_headphone: Option<unsafe extern "C" fn(rt: *mut gpio_runtime) -> c_int>,
    pub get_speakers: Option<unsafe extern "C" fn(rt: *mut gpio_runtime) -> c_int>,
    pub get_lineout: Option<unsafe extern "C" fn(rt: *mut gpio_runtime) -> c_int>,
    pub set_notify: Option<
        unsafe extern "C" fn(
            rt: *mut gpio_runtime,
            type_: notify_type,
            notify: notify_func_t,
            data: *mut c_void,
        ) -> c_int,
    >,
    pub get_detect:
        Option<unsafe extern "C" fn(rt: *mut gpio_runtime, type_: notify_type) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static GFP_KERNEL: gfp_t;

    fn printk(fmt: *const c_char, ...) -> c_int;
    fn pmf_call_function(
        node: *mut device_node,
        name: *const c_char,
        args: *mut pmf_args,
    ) -> c_int;
    fn pmf_register_irq_client(
        node: *mut device_node,
        name: *const c_char,
        client: *mut pmf_irq_client,
    ) -> c_int;
    fn pmf_unregister_irq_client(client: *mut pmf_irq_client);
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn init_delayed_work(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_uint) -> bool;
}

unsafe extern "C" fn pmf_gpio_set_headphone(rt: *mut gpio_runtime, on: c_int) {
    let mut args = pmf_args {
        count: 1,
        u: [pmf_arg { v: (!on) as c_int }],
    };
    let rc: c_int;

    if rt.is_null() {
        return;
    }
    rc = unsafe { pmf_call_function((*rt).node, c"headphone-mute".as_ptr(), &mut args) };
    if rc != 0 && rc != -ENODEV {
        unsafe {
            printk(
                c"%spmf_gpio_set_headphone failed, rc: %d\n".as_ptr(),
                KERN_WARNING.as_ptr() as *const c_char,
                rc,
            );
        }
    }
    unsafe {
        (*rt).implementation_private &= !(1 << 0);
        (*rt).implementation_private |= ((on != 0) as c_int) << 0;
    }
}

unsafe extern "C" fn pmf_gpio_get_headphone(rt: *mut gpio_runtime) -> c_int {
    if rt.is_null() {
        return 0;
    }
    unsafe { ((*rt).implementation_private >> 0) & 1 }
}

unsafe extern "C" fn pmf_gpio_set_amp(rt: *mut gpio_runtime, on: c_int) {
    let mut args = pmf_args {
        count: 1,
        u: [pmf_arg { v: (!on) as c_int }],
    };
    let rc: c_int;

    if rt.is_null() {
        return;
    }
    rc = unsafe { pmf_call_function((*rt).node, c"amp-mute".as_ptr(), &mut args) };
    if rc != 0 && rc != -ENODEV {
        unsafe {
            printk(
                c"%spmf_gpio_set_amp failed, rc: %d\n".as_ptr(),
                KERN_WARNING.as_ptr() as *const c_char,
                rc,
            );
        }
    }
    unsafe {
        (*rt).implementation_private &= !(1 << 1);
        (*rt).implementation_private |= ((on != 0) as c_int) << 1;
    }
}

unsafe extern "C" fn pmf_gpio_get_amp(rt: *mut gpio_runtime) -> c_int {
    if rt.is_null() {
        return 0;
    }
    unsafe { ((*rt).implementation_private >> 1) & 1 }
}

unsafe extern "C" fn pmf_gpio_set_lineout(rt: *mut gpio_runtime, on: c_int) {
    let mut args = pmf_args {
        count: 1,
        u: [pmf_arg { v: (!on) as c_int }],
    };
    let rc: c_int;

    if rt.is_null() {
        return;
    }
    rc = unsafe { pmf_call_function((*rt).node, c"lineout-mute".as_ptr(), &mut args) };
    if rc != 0 && rc != -ENODEV {
        unsafe {
            printk(
                c"%spmf_gpio_set_lineout failed, rc: %d\n".as_ptr(),
                KERN_WARNING.as_ptr() as *const c_char,
                rc,
            );
        }
    }
    unsafe {
        (*rt).implementation_private &= !(1 << 2);
        (*rt).implementation_private |= ((on != 0) as c_int) << 2;
    }
}

unsafe extern "C" fn pmf_gpio_get_lineout(rt: *mut gpio_runtime) -> c_int {
    if rt.is_null() {
        return 0;
    }
    unsafe { ((*rt).implementation_private >> 2) & 1 }
}

unsafe extern "C" fn pmf_gpio_set_hw_reset(rt: *mut gpio_runtime, on: c_int) {
    let mut args = pmf_args {
        count: 1,
        u: [pmf_arg {
            v: (on != 0) as c_int,
        }],
    };
    let rc: c_int;

    if rt.is_null() {
        return;
    }
    rc = unsafe { pmf_call_function((*rt).node, c"hw-reset".as_ptr(), &mut args) };
    if rc != 0 {
        unsafe {
            printk(
                c"%spmf_gpio_set_hw_reset failed, rc: %d\n".as_ptr(),
                KERN_WARNING.as_ptr() as *const c_char,
                rc,
            );
        }
    }
}

unsafe extern "C" fn pmf_gpio_all_amps_off(rt: *mut gpio_runtime) {
    let saved: c_int;

    if rt.is_null() {
        return;
    }
    unsafe {
        saved = (*rt).implementation_private;
        pmf_gpio_set_headphone(rt, 0);
        pmf_gpio_set_amp(rt, 0);
        pmf_gpio_set_lineout(rt, 0);
        (*rt).implementation_private = saved;
    }
}

unsafe extern "C" fn pmf_gpio_all_amps_restore(rt: *mut gpio_runtime) {
    let s: c_int;

    if rt.is_null() {
        return;
    }
    unsafe {
        s = (*rt).implementation_private;
        pmf_gpio_set_headphone(rt, (s >> 0) & 1);
        pmf_gpio_set_amp(rt, (s >> 1) & 1);
        pmf_gpio_set_lineout(rt, (s >> 2) & 1);
    }
}

unsafe extern "C" fn pmf_handle_notify(work: *mut work_struct) {
    let notif = (work as *mut u8).wrapping_sub(
        offset_of!(gpio_notification, work) + offset_of!(delayed_work, work),
    ) as *mut gpio_notification;

    unsafe {
        mutex_lock(&mut (*notif).mutex);
        if let Some(notify) = (*notif).notify {
            notify((*notif).data);
        }
        mutex_unlock(&mut (*notif).mutex);
    }
}

unsafe extern "C" fn pmf_gpio_init(rt: *mut gpio_runtime) {
    unsafe {
        pmf_gpio_all_amps_off(rt);
        (*rt).implementation_private = 0;
        init_delayed_work(&mut (*rt).headphone_notify.work, pmf_handle_notify);
        init_delayed_work(&mut (*rt).line_in_notify.work, pmf_handle_notify);
        init_delayed_work(&mut (*rt).line_out_notify.work, pmf_handle_notify);
        mutex_init(&mut (*rt).headphone_notify.mutex);
        mutex_init(&mut (*rt).line_in_notify.mutex);
        mutex_init(&mut (*rt).line_out_notify.mutex);
    }
}

unsafe extern "C" fn pmf_gpio_exit(rt: *mut gpio_runtime) {
    unsafe {
        pmf_gpio_all_amps_off(rt);
        (*rt).implementation_private = 0;

        if !(*rt).headphone_notify.gpio_private.is_null() {
            pmf_unregister_irq_client((*rt).headphone_notify.gpio_private);
        }
        if !(*rt).line_in_notify.gpio_private.is_null() {
            pmf_unregister_irq_client((*rt).line_in_notify.gpio_private);
        }
        if !(*rt).line_out_notify.gpio_private.is_null() {
            pmf_unregister_irq_client((*rt).line_out_notify.gpio_private);
        }

        /* make sure no work is pending before freeing
         * all things */
        cancel_delayed_work_sync(&mut (*rt).headphone_notify.work);
        cancel_delayed_work_sync(&mut (*rt).line_in_notify.work);
        cancel_delayed_work_sync(&mut (*rt).line_out_notify.work);

        mutex_destroy(&mut (*rt).headphone_notify.mutex);
        mutex_destroy(&mut (*rt).line_in_notify.mutex);
        mutex_destroy(&mut (*rt).line_out_notify.mutex);

        kfree((*rt).headphone_notify.gpio_private as *mut c_void);
        kfree((*rt).line_in_notify.gpio_private as *mut c_void);
        kfree((*rt).line_out_notify.gpio_private as *mut c_void);
    }
}

unsafe extern "C" fn pmf_handle_notify_irq(data: *mut c_void) {
    let notif = data as *mut gpio_notification;

    unsafe {
        schedule_delayed_work(&mut (*notif).work, 0);
    }
}

unsafe extern "C" fn pmf_set_notify(
    rt: *mut gpio_runtime,
    type_: notify_type,
    notify: notify_func_t,
    data: *mut c_void,
) -> c_int {
    let notif: *mut gpio_notification;
    let old: notify_func_t;
    let mut irq_client: *mut pmf_irq_client;
    let name: *const c_char;
    let mut err: c_int = -EBUSY;

    unsafe {
        match type_ {
            notify_type::AOA_NOTIFY_HEADPHONE => {
                notif = &mut (*rt).headphone_notify;
                name = c"headphone-detect".as_ptr();
            }
            notify_type::AOA_NOTIFY_LINE_IN => {
                notif = &mut (*rt).line_in_notify;
                name = c"linein-detect".as_ptr();
            }
            notify_type::AOA_NOTIFY_LINE_OUT => {
                notif = &mut (*rt).line_out_notify;
                name = c"lineout-detect".as_ptr();
            }
        }

        mutex_lock(&mut (*notif).mutex);

        old = (*notif).notify;

        if old.is_none() && notify.is_none() {
            mutex_unlock(&mut (*notif).mutex);
            return 0;
        }

        if old.is_some() && notify.is_some() {
            if old == notify && (*notif).data == data {
                err = 0;
            }
            mutex_unlock(&mut (*notif).mutex);
            return err;
        }

        if old.is_some() && notify.is_none() {
            irq_client = (*notif).gpio_private;
            pmf_unregister_irq_client(irq_client);
            kfree(irq_client as *mut c_void);
            (*notif).gpio_private = ptr::null_mut();
        }
        if old.is_none() && notify.is_some() {
            irq_client = kzalloc(size_of::<pmf_irq_client>(), GFP_KERNEL) as *mut pmf_irq_client;
            if irq_client.is_null() {
                mutex_unlock(&mut (*notif).mutex);
                return -ENOMEM;
            }
            (*irq_client).data = notif as *mut c_void;
            (*irq_client).handler = Some(pmf_handle_notify_irq);
            (*irq_client).owner = THIS_MODULE;
            err = pmf_register_irq_client((*rt).node, name, irq_client);
            if err != 0 {
                printk(
                    c"%ssnd-aoa: gpio layer failed to register %s irq (%d)\n".as_ptr(),
                    KERN_ERR.as_ptr() as *const c_char,
                    name,
                    err,
                );
                kfree(irq_client as *mut c_void);
                mutex_unlock(&mut (*notif).mutex);
                return err;
            }
            (*notif).gpio_private = irq_client;
        }
        (*notif).notify = notify;
        (*notif).data = data;

        mutex_unlock(&mut (*notif).mutex);
    }

    0
}

unsafe extern "C" fn pmf_get_detect(rt: *mut gpio_runtime, type_: notify_type) -> c_int {
    let name: *const c_char;
    let mut ret: c_int = 0;
    let mut args = pmf_args {
        count: 1,
        u: [pmf_arg { p: &mut ret }],
    };
    let err: c_int;

    match type_ {
        notify_type::AOA_NOTIFY_HEADPHONE => {
            name = c"headphone-detect".as_ptr();
        }
        notify_type::AOA_NOTIFY_LINE_IN => {
            name = c"linein-detect".as_ptr();
        }
        notify_type::AOA_NOTIFY_LINE_OUT => {
            name = c"lineout-detect".as_ptr();
        }
    }

    err = unsafe { pmf_call_function((*rt).node, name, &mut args) };
    if err != 0 {
        return err;
    }
    ret
}

static mut methods: gpio_methods = gpio_methods {
    init: Some(pmf_gpio_init),
    exit: Some(pmf_gpio_exit),
    all_amps_off: Some(pmf_gpio_all_amps_off),
    all_amps_restore: Some(pmf_gpio_all_amps_restore),
    set_headphone: Some(pmf_gpio_set_headphone),
    set_speakers: Some(pmf_gpio_set_amp),
    set_lineout: Some(pmf_gpio_set_lineout),
    set_hw_reset: Some(pmf_gpio_set_hw_reset),
    get_headphone: Some(pmf_gpio_get_headphone),
    get_speakers: Some(pmf_gpio_get_amp),
    get_lineout: Some(pmf_gpio_get_lineout),
    set_notify: Some(pmf_set_notify),
    get_detect: Some(pmf_get_detect),
};

#[unsafe(no_mangle)]
pub static mut pmf_gpio_methods: *mut gpio_methods = unsafe { &raw mut methods };

// EXPORT_SYMBOL_GPL(pmf_gpio_methods);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
