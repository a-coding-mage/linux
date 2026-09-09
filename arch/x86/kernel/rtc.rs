// SPDX-License-Identifier: GPL-2.0
/*
 * RTC related functions
 */

// C dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_X86_32)]
pub static mut cmos_lock: ::core::ffi::c_ulong = 0;

// DEFINE_SPINLOCK(rtc_lock);
extern "C" {
    pub static mut rtc_lock: ::core::ffi::c_void;
}

/*
 * In order to set the CMOS clock precisely, mach_set_cmos_time has to be
 * called 500 ms after the second nowtime has started, because when
 * nowtime is written into the registers of the CMOS clock, it will jump to
 * the next second precisely 500 ms later. Check the Motorola MC146818A or
 * Dallas DS12887 data sheet for details.
 */
pub unsafe fn mach_set_cmos_time(now: *const timespec64) -> ::core::ffi::c_int {
    let nowtime: ::core::ffi::c_ulonglong = (*now).tv_sec as ::core::ffi::c_ulonglong;
    let mut tm: rtc_time = ::core::mem::zeroed();
    let mut retval: ::core::ffi::c_int = 0;

    rtc_time64_to_tm(nowtime, &mut tm);
    if rtc_valid_tm(&tm) == 0 {
        retval = mc146818_set_time(&tm);
        if retval != 0 {
            printk(
                KERN_ERR,
                b"%s: RTC write failed with error %d\n\0".as_ptr(),
                b"mach_set_cmos_time\0".as_ptr(),
                retval,
            );
        }
    } else {
        printk(
            KERN_ERR,
            b"%s: Invalid RTC value: write of %llx to RTC failed\n\0".as_ptr(),
            b"mach_set_cmos_time\0".as_ptr(),
            nowtime,
        );
        retval = -EINVAL;
    }
    retval
}

pub unsafe fn mach_get_cmos_time(now: *mut timespec64) {
    let mut tm: rtc_time = ::core::mem::zeroed();

    /*
     * If pm_trace abused the RTC as storage, set the timespec to 0,
     * which tells the caller that this RTC value is unusable.
     */
    if pm_trace_rtc_valid() == 0 {
        (*now).tv_sec = 0;
        (*now).tv_nsec = 0;
        return;
    }

    if mc146818_get_time(&mut tm, 1000) != 0 {
        pr_err(b"Unable to read current time from RTC\n\0".as_ptr());
        (*now).tv_sec = 0;
        (*now).tv_nsec = 0;
        return;
    }

    (*now).tv_sec = rtc_tm_to_time64(&tm);
    (*now).tv_nsec = 0;
}

/* Routines for accessing the CMOS RAM/RTC. */
pub unsafe fn rtc_cmos_read(addr: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar {
    let val: ::core::ffi::c_uchar;

    lock_cmos_prefix(addr);
    outb(addr, RTC_PORT(0));
    val = inb(RTC_PORT(1));
    lock_cmos_suffix(addr);

    val
}

pub unsafe fn rtc_cmos_write(
    val: ::core::ffi::c_uchar,
    addr: ::core::ffi::c_uchar,
) {
    lock_cmos_prefix(addr);
    outb(addr, RTC_PORT(0));
    outb(val, RTC_PORT(1));
    lock_cmos_suffix(addr);
}

pub unsafe fn update_persistent_clock64(now: timespec64) -> ::core::ffi::c_int {
    x86_platform.set_wallclock(&now)
}

/* not static: needed by APM */
pub unsafe fn read_persistent_clock64(ts: *mut timespec64) {
    x86_platform.get_wallclock(ts);
}

static mut rtc_resources: [resource; 2] = [
    resource {
        start: RTC_PORT(0),
        end: RTC_PORT(1),
        flags: IORESOURCE_IO,
    },
    resource {
        start: RTC_IRQ,
        end: RTC_IRQ,
        flags: IORESOURCE_IRQ,
    },
];

static mut rtc_device: platform_device = platform_device {
    name: b"rtc_cmos\0".as_ptr(),
    id: -1,
    resource: rtc_resources.as_mut_ptr(),
    num_resources: rtc_resources.len(),
};

unsafe fn add_rtc_cmos() -> ::core::ffi::c_int {
    if cmos_rtc_platform_device_present {
        return 0;
    }

    if !x86_platform.legacy.rtc {
        return -ENODEV;
    }

    platform_device_register(&mut rtc_device);
    dev_info(
        &mut rtc_device.dev,
        b"registered fallback platform RTC device\n\0".as_ptr(),
    );

    0
}

// device_initcall(add_rtc_cmos);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
