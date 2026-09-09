// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Intel Corporation
 * Author: Johannes Berg <johannes@sipsolutions.net>
 */

// External kernel/UML types, constants, globals, and functions are supplied by
// the surrounding translation unit and are intentionally not implemented here.

static mut UML_RTC_ALARM_TIME: time64_t = 0;
static mut UML_RTC_ALARM_ENABLED: bool = false;
static mut UML_RTC: *mut rtc_device = core::ptr::null_mut();
static mut UML_RTC_IRQ_FD: i32 = 0;
static mut UML_RTC_IRQ: i32 = 0;

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
unsafe extern "C" fn uml_rtc_time_travel_alarm(_ev: *mut time_travel_event) {
    uml_rtc_send_timetravel_alarm();
}

#[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
static mut UML_RTC_ALARM_EVENT: time_travel_event = time_travel_event {
    fn_: Some(uml_rtc_time_travel_alarm),
};

unsafe extern "C" fn uml_rtc_read_time(_dev: *mut device, tm: *mut rtc_time) -> i32 {
    let mut ts: timespec64 = core::mem::zeroed();

    // Use this to get correct time in time-travel mode
    read_persistent_clock64(&mut ts);
    rtc_time64_to_tm(timespec64_to_ktime(ts) / NSEC_PER_SEC, tm);

    0
}

unsafe extern "C" fn uml_rtc_read_alarm(
    _dev: *mut device,
    alrm: *mut rtc_wkalrm,
) -> i32 {
    rtc_time64_to_tm(UML_RTC_ALARM_TIME, &mut (*alrm).time);
    (*alrm).enabled = UML_RTC_ALARM_ENABLED;

    0
}

unsafe extern "C" fn uml_rtc_alarm_irq_enable(
    _dev: *mut device,
    enable: u32,
) -> i32 {
    let mut ts: timespec64 = core::mem::zeroed();
    let secs: u64;

    if enable == 0 && !UML_RTC_ALARM_ENABLED {
        return 0;
    }

    UML_RTC_ALARM_ENABLED = enable != 0;

    read_persistent_clock64(&mut ts);
    secs = UML_RTC_ALARM_TIME.wrapping_sub(ts.tv_sec as time64_t) as u64;

    if time_travel_mode == TT_MODE_OFF {
        if enable == 0 {
            uml_rtc_disable_alarm();
            return 0;
        }

        // enable or update
        return uml_rtc_enable_alarm(secs);
    } else {
        #[cfg(CONFIG_UML_TIME_TRAVEL_SUPPORT)]
        {
            time_travel_del_event(&mut UML_RTC_ALARM_EVENT);

            if enable != 0 {
                time_travel_add_event_rel(
                    &mut UML_RTC_ALARM_EVENT,
                    secs.wrapping_mul(NSEC_PER_SEC as u64)
                        .wrapping_sub(ts.tv_nsec as u64),
                );
            }
        }
    }

    0
}

unsafe extern "C" fn uml_rtc_set_alarm(
    dev: *mut device,
    alrm: *mut rtc_wkalrm,
) -> i32 {
    uml_rtc_alarm_irq_enable(dev, 0);
    UML_RTC_ALARM_TIME = rtc_tm_to_time64(&(*alrm).time);
    uml_rtc_alarm_irq_enable(dev, (*alrm).enabled as u32);

    0
}

static UML_RTC_OPS: rtc_class_ops = rtc_class_ops {
    read_time: Some(uml_rtc_read_time),
    read_alarm: Some(uml_rtc_read_alarm),
    alarm_irq_enable: Some(uml_rtc_alarm_irq_enable),
    set_alarm: Some(uml_rtc_set_alarm),
};

unsafe extern "C" fn uml_rtc_interrupt(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t {
    let mut c: u64 = 0;

    // alarm triggered, it's now off
    UML_RTC_ALARM_ENABLED = false;

    os_read_file(UML_RTC_IRQ_FD, &mut c as *mut u64 as *mut core::ffi::c_void, core::mem::size_of::<u64>());
    WARN_ON(c == 0);

    pm_system_wakeup();
    rtc_update_irq(UML_RTC, 1, RTC_IRQF | RTC_AF);

    IRQ_HANDLED
}

unsafe extern "C" fn uml_rtc_setup() -> i32 {
    let mut err: i32;

    err = uml_rtc_start(time_travel_mode != TT_MODE_OFF);
    if WARN(err < 0, c"err = %d\n") {
        return err;
    }

    UML_RTC_IRQ_FD = err;

    err = um_request_irq(UM_IRQ_ALLOC, UML_RTC_IRQ_FD, IRQ_READ, Some(uml_rtc_interrupt), 0, c"rtc", core::ptr::null_mut());
    if err < 0 {
        uml_rtc_stop(time_travel_mode != TT_MODE_OFF);
        return err;
    }

    irq_set_irq_wake(err, 1);

    UML_RTC_IRQ = err;
    0
}

unsafe extern "C" fn uml_rtc_cleanup() {
    um_free_irq(UML_RTC_IRQ, core::ptr::null_mut());
    uml_rtc_stop(time_travel_mode != TT_MODE_OFF);
}

unsafe extern "C" fn uml_rtc_probe(pdev: *mut platform_device) -> i32 {
    let mut err = uml_rtc_setup();
    if err != 0 { return err; }

    UML_RTC = devm_rtc_allocate_device(&mut (*pdev).dev);
    if IS_ERR(UML_RTC) {
        err = PTR_ERR(UML_RTC);
        uml_rtc_cleanup();
        return err;
    }

    (*UML_RTC).ops = &UML_RTC_OPS;
    device_init_wakeup(&mut (*pdev).dev, 1);

    err = devm_rtc_register_device(UML_RTC);
    if err != 0 { uml_rtc_cleanup(); return err; }
    0
}

unsafe extern "C" fn uml_rtc_remove(pdev: *mut platform_device) {
    device_init_wakeup(&mut (*pdev).dev, 0);
    uml_rtc_cleanup();
}

static mut UML_RTC_DRIVER: platform_driver = platform_driver {
    probe: Some(uml_rtc_probe),
    remove: Some(uml_rtc_remove),
    driver: driver { name: c"uml-rtc", ..unsafe { core::mem::zeroed() } },
};

unsafe extern "C" fn uml_rtc_init() -> i32 {
    let mut pdev: *mut platform_device;
    let mut err = platform_driver_register(&mut UML_RTC_DRIVER);
    if err != 0 { return err; }

    pdev = platform_device_alloc(c"uml-rtc", 0);
    if pdev.is_null() {
        err = -ENOMEM;
        platform_driver_unregister(&mut UML_RTC_DRIVER);
        return err;
    }

    err = platform_device_add(pdev);
    if err != 0 {
        platform_device_put(pdev);
        platform_driver_unregister(&mut UML_RTC_DRIVER);
        return err;
    }
    0
}

device_initcall!(uml_rtc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
