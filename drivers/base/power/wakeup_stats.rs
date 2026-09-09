// SPDX-License-Identifier: GPL-2.0
/*
 * Wakeup statistics in sysfs
 *
 * Copyright (c) 2019 Linux Foundation
 * Copyright (c) 2019 Greg Kroah-Hartman <gregkh@linuxfoundation.org>
 * Copyright (c) 2019 Google Inc.
 */

// Dependencies supplied by the Linux kernel and power subsystem.

static mut wakeup_class: *mut class = core::ptr::null_mut();

macro_rules! wakeup_attr {
    ($name:ident, $show:ident) => {
        unsafe fn $show(
            dev: *mut device,
            _attr: *mut device_attribute,
            buf: *mut core::ffi::c_char,
        ) -> isize {
            let ws: *mut wakeup_source = dev_get_drvdata(dev);
            sysfs_emit(buf, "%lu\n", (*ws).$name)
        }
    };
}

wakeup_attr!(active_count, active_count_show);
wakeup_attr!(event_count, event_count_show);
wakeup_attr!(wakeup_count, wakeup_count_show);
wakeup_attr!(expire_count, expire_count_show);
wakeup_attr!(relax_count, relax_count_show);

unsafe fn active_time_ms_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let ws: *mut wakeup_source = dev_get_drvdata(dev);
    let active_time: ktime_t = if (*ws).active {
        ktime_sub(ktime_get(), (*ws).last_time)
    } else {
        0
    };
    sysfs_emit(buf, "%lld\n", ktime_to_ms(active_time))
}

unsafe fn total_time_ms_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let ws: *mut wakeup_source = dev_get_drvdata(dev);
    let mut total_time: ktime_t = (*ws).total_time;
    if (*ws).active {
        let active_time = ktime_sub(ktime_get(), (*ws).last_time);
        total_time = ktime_add(total_time, active_time);
    }
    sysfs_emit(buf, "%lld\n", ktime_to_ms(total_time))
}

unsafe fn max_time_ms_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let ws: *mut wakeup_source = dev_get_drvdata(dev);
    let mut max_time: ktime_t = (*ws).max_time;
    if (*ws).active {
        let active_time = ktime_sub(ktime_get(), (*ws).last_time);
        if active_time > max_time {
            max_time = active_time;
        }
    }
    sysfs_emit(buf, "%lld\n", ktime_to_ms(max_time))
}

unsafe fn last_change_ms_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let ws: *mut wakeup_source = dev_get_drvdata(dev);
    sysfs_emit(buf, "%lld\n", ktime_to_ms((*ws).last_time))
}

unsafe fn name_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let ws: *mut wakeup_source = dev_get_drvdata(dev);
    sysfs_emit(buf, "%s\n", (*ws).name)
}

unsafe fn prevent_suspend_time_ms_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let ws: *mut wakeup_source = dev_get_drvdata(dev);
    let mut prevent_sleep_time: ktime_t = (*ws).prevent_sleep_time;
    if (*ws).active && (*ws).autosleep_enabled {
        prevent_sleep_time = ktime_add(
            prevent_sleep_time,
            ktime_sub(ktime_get(), (*ws).start_prevent_time),
        );
    }
    sysfs_emit(buf, "%lld\n", ktime_to_ms(prevent_sleep_time))
}

// DEVICE_ATTR_RO and ATTRIBUTE_GROUPS declarations from the kernel.
static mut wakeup_source_attrs: [*mut attribute; 12] = [
    &mut dev_attr_name.attr,
    &mut dev_attr_active_count.attr,
    &mut dev_attr_event_count.attr,
    &mut dev_attr_wakeup_count.attr,
    &mut dev_attr_expire_count.attr,
    &mut dev_attr_relax_count.attr,
    &mut dev_attr_active_time_ms.attr,
    &mut dev_attr_total_time_ms.attr,
    &mut dev_attr_max_time_ms.attr,
    &mut dev_attr_last_change_ms.attr,
    &mut dev_attr_prevent_suspend_time_ms.attr,
    core::ptr::null_mut(),
];

unsafe fn device_create_release(dev: *mut device) {
    kfree(dev);
}

unsafe fn wakeup_source_device_create(
    parent: *mut device,
    ws: *mut wakeup_source,
) -> *mut device {
    let dev = kzalloc_obj::<device>();
    if dev.is_null() {
        return err_ptr(-12);
    }

    device_initialize(dev);
    (*dev).devt = mkdev(0, 0);
    (*dev).class = wakeup_class;
    (*dev).parent = parent;
    (*dev).groups = wakeup_source_groups;
    (*dev).release = Some(device_create_release);
    dev_set_drvdata(dev, ws);
    device_set_pm_not_required(dev);

    let retval = dev_set_name(dev, "wakeup%d", (*ws).id);
    if retval != 0 {
        put_device(dev);
        return err_ptr(retval);
    }
    let retval = device_add(dev);
    if retval != 0 {
        put_device(dev);
        return err_ptr(retval);
    }
    dev
}

/// wakeup_source_sysfs_add - Add wakeup_source attributes to sysfs.
/// @parent: Device given wakeup source is associated with (or NULL if virtual).
/// @ws: Wakeup source to be added in sysfs.
pub unsafe fn wakeup_source_sysfs_add(parent: *mut device, ws: *mut wakeup_source) -> i32 {
    let dev = wakeup_source_device_create(parent, ws);
    if is_err(dev) {
        return ptr_err(dev);
    }
    (*ws).dev = dev;
    0
}

/// pm_wakeup_source_sysfs_add - Add wakeup_source attributes to sysfs
/// for a device if they're missing.
/// @parent: Device given wakeup source is associated with
pub unsafe fn pm_wakeup_source_sysfs_add(parent: *mut device) -> i32 {
    if (*parent).power.wakeup.is_null() || !(*(*parent).power.wakeup).dev.is_null() {
        return 0;
    }
    wakeup_source_sysfs_add(parent, (*parent).power.wakeup)
}

/// wakeup_source_sysfs_remove - Remove wakeup_source attributes from sysfs.
/// @ws: Wakeup source to be removed from sysfs.
pub unsafe fn wakeup_source_sysfs_remove(ws: *mut wakeup_source) {
    device_unregister((*ws).dev);
}

unsafe fn wakeup_sources_sysfs_init() -> i32 {
    wakeup_class = class_create("wakeup");
    ptr_err_or_zero(wakeup_class)
}

// postcore_initcall(wakeup_sources_sysfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
