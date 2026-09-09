// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/devfreq/governor_userspace.c
 *
 *  Copyright (C) 2011 Samsung Electronics
 *	MyungJoo Ham <myungjoo.ham@samsung.com>
 */

use core::ffi::c_void;

#[repr(C)]
struct userspace_data {
    user_frequency: libc::c_ulong,
    valid: bool,
}

// Types and functions below are supplied by the surrounding kernel bindings.
#[allow(non_camel_case_types)]
type ssize_t = libc::ssize_t;

unsafe fn devfreq_userspace_func(df: *mut devfreq, freq: *mut libc::c_ulong) -> libc::c_int {
    let data = (*df).governor_data as *mut userspace_data;

    if (*data).valid {
        *freq = (*data).user_frequency;
    } else {
        *freq = (*df).previous_freq; /* No user freq specified yet */
    }

    0
}

unsafe fn set_freq_store(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const libc::c_char,
    count: libc::size_t,
) -> ssize_t {
    let devfreq = to_devfreq(dev);
    let mut data: *mut userspace_data;
    let mut wanted: libc::c_ulong = 0;
    let mut err: libc::c_int = 0;

    err = kstrtoul(buf, 0, &mut wanted);
    if err != 0 {
        return err as ssize_t;
    }

    mutex_lock(&mut (*devfreq).lock);
    data = (*devfreq).governor_data as *mut userspace_data;

    (*data).user_frequency = wanted;
    (*data).valid = true;
    err = update_devfreq(devfreq);
    if err == 0 {
        err = count as libc::c_int;
    }
    mutex_unlock(&mut (*devfreq).lock);
    err as ssize_t
}

unsafe fn set_freq_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut libc::c_char,
) -> ssize_t {
    let devfreq = to_devfreq(dev);
    let data: *mut userspace_data;
    let mut err: libc::c_int = 0;

    mutex_lock(&mut (*devfreq).lock);
    data = (*devfreq).governor_data as *mut userspace_data;

    if (*data).valid {
        err = sprintf(buf, "%lu\n", (*data).user_frequency);
    } else {
        err = sprintf(buf, "undefined\n");
    }
    mutex_unlock(&mut (*devfreq).lock);
    err as ssize_t
}

// DEVICE_ATTR_RW(set_freq);
static mut dev_entries: [*mut attribute; 2] = [
    &raw mut dev_attr_set_freq.attr,
    core::ptr::null_mut(),
];

static mut dev_attr_group: attribute_group = attribute_group {
    name: DEVFREQ_GOV_USERSPACE,
    attrs: unsafe { &raw mut dev_entries },
};

unsafe fn userspace_init(devfreq: *mut devfreq) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let data = kzalloc_obj::<userspace_data>();

    if data.is_null() {
        err = -ENOMEM;
        return err;
    }
    (*data).valid = false;
    (*devfreq).governor_data = data as *mut c_void;

    err = sysfs_create_group(&mut (*devfreq).dev.kobj, &raw mut dev_attr_group);
    err
}

unsafe fn userspace_exit(devfreq: *mut devfreq) {
    /*
     * Remove the sysfs entry, unless this is being called after
     * device_del(), which should have done this already via kobject_del().
     */
    if !(*devfreq).dev.kobj.sd.is_null() {
        sysfs_remove_group(&mut (*devfreq).dev.kobj, &raw mut dev_attr_group);
    }

    kfree((*devfreq).governor_data);
    (*devfreq).governor_data = core::ptr::null_mut();
}

unsafe fn devfreq_userspace_handler(
    devfreq: *mut devfreq,
    event: libc::c_uint,
    _data: *mut c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;

    match event {
        DEVFREQ_GOV_START => {
            ret = userspace_init(devfreq);
        }
        DEVFREQ_GOV_STOP => {
            userspace_exit(devfreq);
        }
        _ => {}
    }

    ret
}

static mut devfreq_userspace: devfreq_governor = devfreq_governor {
    name: DEVFREQ_GOV_USERSPACE,
    get_target_freq: Some(devfreq_userspace_func),
    event_handler: Some(devfreq_userspace_handler),
};

unsafe fn devfreq_userspace_init() -> libc::c_int {
    devfreq_add_governor(&raw mut devfreq_userspace)
}

// subsys_initcall(devfreq_userspace_init);

unsafe fn devfreq_userspace_exit() {
    let ret: libc::c_int;

    ret = devfreq_remove_governor(&raw mut devfreq_userspace);
    if ret != 0 {
        pr_err("%s: failed remove governor %d\n", "devfreq_userspace_exit", ret);
    }

    return;
}

// module_exit(devfreq_userspace_exit);
// MODULE_DESCRIPTION("DEVFREQ Userspace governor");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
