// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/kernel/rtc.c
 *
 *  Copyright (C) 1991, 1992, 1995, 1999, 2000  Linus Torvalds
 *
 * This file contains date handling.
 */

/* C headers and proto.h provide the external kernel symbols used below. */

/*
 * Support for the RTC device.
 *
 * We don't want to use the rtc-cmos driver, because we don't want to support
 * alarms, as that would be indistinguishable from timer interrupts.
 *
 * Further, generic code is really, really tied to a 1900 epoch.  This is
 * true in __get_rtc_time as well as the users of struct rtc_time e.g.
 * rtc_tm_to_time.  Thankfully all of the other epochs in use are later
 * than 1900, and so it's easy to adjust.
 */

static mut rtc_epoch: ::core::ffi::c_ulong = 0;

#[allow(non_snake_case)]
unsafe extern "C" fn specifiy_epoch(str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let epoch = simple_strtoul(str_, core::ptr::null_mut(), 0);
    if epoch < 1900 {
        printk(b"Ignoring invalid user specified epoch %lu\n\0".as_ptr() as *const _, epoch);
    } else {
        rtc_epoch = epoch;
    }
    1
}

// __setup("epoch=", specifiy_epoch);

unsafe extern "C" fn init_rtc_epoch() {
    let mut epoch: ::core::ffi::c_int;
    let mut year: ::core::ffi::c_int;
    let ctrl: ::core::ffi::c_int;

    if rtc_epoch != 0 {
        /* The epoch was specified on the command-line.  */
        return;
    }

    /* Detect the epoch in use on this computer.  */
    ctrl = CMOS_READ(RTC_CONTROL);
    year = CMOS_READ(RTC_YEAR);
    if (ctrl & RTC_DM_BINARY) == 0 || RTC_ALWAYS_BCD != 0 {
        year = bcd2bin(year);
    }

    /* PC-like is standard; used for year >= 70 */
    epoch = 1900;
    if year < 20 {
        epoch = 2000;
    } else if year >= 20 && year < 48 {
        /* NT epoch */
        epoch = 1980;
    } else if year >= 48 && year < 70 {
        /* Digital UNIX epoch */
        epoch = 1952;
    }
    rtc_epoch = epoch as ::core::ffi::c_ulong;

    printk(KERN_INFO, b"Using epoch %d for rtc year %d\n\0".as_ptr() as *const _, epoch, year);
}

unsafe extern "C" fn alpha_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> ::core::ffi::c_int {
    let ret = mc146818_get_time(tm, 10);

    if ret < 0 {
        dev_err_ratelimited(dev, b"unable to read current time\n\0".as_ptr() as *const _);
        return ret;
    }

    /* Adjust for non-default epochs. */
    if rtc_epoch != 1900 {
        let mut year = (*tm).tm_year;
        /* Undo the century adjustment made in __get_rtc_time.  */
        if year >= 100 {
            year -= 100;
        }
        year += rtc_epoch as ::core::ffi::c_int - 1900;
        /* Redo the century adjustment with the epoch in place.  */
        if year <= 69 {
            year += 100;
        }
        (*tm).tm_year = year;
    }

    0
}

unsafe extern "C" fn alpha_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> ::core::ffi::c_int {
    let mut xtm: rtc_time;

    if rtc_epoch != 1900 {
        xtm = *tm;
        xtm.tm_year -= rtc_epoch as ::core::ffi::c_int - 1900;
        tm = &mut xtm;
    }

    mc146818_set_time(tm)
}

unsafe extern "C" fn alpha_rtc_ioctl(
    dev: *mut device,
    cmd: ::core::ffi::c_uint,
    arg: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    match cmd {
        RTC_EPOCH_READ => put_user(rtc_epoch, arg as *mut ::core::ffi::c_ulong),
        RTC_EPOCH_SET => {
            if arg < 1900 {
                return -EINVAL;
            }
            rtc_epoch = arg;
            0
        }
        _ => -ENOIOCTLCMD,
    }
}

static alpha_rtc_ops: rtc_class_ops = rtc_class_ops {
    read_time: Some(alpha_rtc_read_time),
    set_time: Some(alpha_rtc_set_time),
    ioctl: Some(alpha_rtc_ioctl),
};

/*
 * Similarly, except do the actual CMOS access on the boot cpu only.  The
 * access polls for the RTC update cycle and takes rtc_lock, so run it in a
 * worker on that cpu rather than from an interprocessor interrupt.
 */

// Conditional C build configuration: HAVE_REMOTE_RTC is enabled for SMP
// Alpha generic or Marvel configurations.

unsafe extern "C" fn do_remote_read(data: *mut ::core::ffi::c_void) -> ::core::ffi::c_long {
    alpha_rtc_read_time(core::ptr::null_mut(), data as *mut rtc_time) as ::core::ffi::c_long
}

unsafe extern "C" fn remote_read_time(dev: *mut device, tm: *mut rtc_time) -> ::core::ffi::c_int {
    if smp_processor_id() != boot_cpuid {
        return work_on_cpu(boot_cpuid, Some(do_remote_read), tm as *mut _);
    }
    alpha_rtc_read_time(core::ptr::null_mut(), tm)
}

unsafe extern "C" fn do_remote_set(data: *mut ::core::ffi::c_void) -> ::core::ffi::c_long {
    alpha_rtc_set_time(core::ptr::null_mut(), data as *mut rtc_time) as ::core::ffi::c_long
}

unsafe extern "C" fn remote_set_time(dev: *mut device, tm: *mut rtc_time) -> ::core::ffi::c_int {
    if smp_processor_id() != boot_cpuid {
        return work_on_cpu(boot_cpuid, Some(do_remote_set), tm as *mut _);
    }
    alpha_rtc_set_time(core::ptr::null_mut(), tm)
}

static remote_rtc_ops: rtc_class_ops = rtc_class_ops {
    read_time: Some(remote_read_time),
    set_time: Some(remote_set_time),
    ioctl: Some(alpha_rtc_ioctl),
};

unsafe extern "C" fn alpha_rtc_init() -> ::core::ffi::c_int {
    let pdev: *mut platform_device;
    let mut rtc: *mut rtc_device;

    init_rtc_epoch();

    pdev = platform_device_register_simple(b"rtc-alpha\0".as_ptr() as *const _, -1, core::ptr::null_mut(), 0);
    rtc = devm_rtc_allocate_device(&mut (*pdev).dev);
    if IS_ERR(rtc) {
        return PTR_ERR(rtc);
    }

    platform_set_drvdata(pdev, rtc);
    (*rtc).ops = &alpha_rtc_ops;

    // #ifdef HAVE_REMOTE_RTC
    if alpha_mv.rtc_boot_cpu_only {
        (*rtc).ops = &remote_rtc_ops;
    }
    // #endif

    devm_rtc_register_device(rtc)
}

// device_initcall(alpha_rtc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
