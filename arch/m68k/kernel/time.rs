// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/m68k/kernel/time.c
 *
 *  Copyright (C) 1991, 1992, 1995  Linus Torvalds
 *
 * This file contains the m68k-specific time handling details.
 * Most of the stuff is located in the machine specific files.
 *
 * 1997-09-10	Updated NTP code according to technical memorandum Jan '96
 *		"A Kernel Model for Precision Timekeeping" by Dave Mills
 */

// C header dependencies are supplied by other translation units.

extern "C" {
    pub static mut mach_random_get_entropy: Option<unsafe extern "C" fn() -> c_ulong>;
}

// EXPORT_SYMBOL_GPL(mach_random_get_entropy);

#[cfg(CONFIG_HEARTBEAT)]
pub unsafe extern "C" fn timer_heartbeat() {
    /* use power LED as a heartbeat instead -- much more useful
       for debugging -- based on the version for PReP by Cort */
    /* acts like an actual heart beat -- ie thump-thump-pause... */
    if mach_heartbeat.is_some() {
        static mut CNT: c_uint = 0;
        static mut PERIOD: c_uint = 0;
        static mut DIST: c_uint = 0;

        if CNT == 0 || CNT == DIST {
            mach_heartbeat.unwrap()(1);
        } else if CNT == 7 || CNT == DIST + 7 {
            mach_heartbeat.unwrap()(0);
        }

        CNT = CNT.wrapping_add(1);
        if CNT > PERIOD {
            CNT = 0;
            /* The hyperbolic function below modifies the heartbeat period
             * length in dependency of the current (5min) load. It goes
             * through the points f(0)=126, f(1)=86, f(5)=51,
             * f(inf)->30. */
            PERIOD = ((672u32 << FSHIFT) / (5 * avenrun[0] + (7u32 << FSHIFT))) + 30;
            DIST = PERIOD / 4;
        }
    }
}

#[cfg(any(CONFIG_M68KCLASSIC, CONFIG_SUN3))]
pub static mut mach_hwclk: Option<unsafe extern "C" fn(c_int, *mut rtc_time) -> c_int> = None;
// EXPORT_SYMBOL(mach_hwclk);

#[cfg(any(CONFIG_M68KCLASSIC, CONFIG_SUN3))]
pub static mut mach_get_rtc_pll: Option<unsafe extern "C" fn(*mut rtc_pll_info) -> c_int> = None;
#[cfg(any(CONFIG_M68KCLASSIC, CONFIG_SUN3))]
pub static mut mach_set_rtc_pll: Option<unsafe extern "C" fn(*mut rtc_pll_info) -> c_int> = None;
// EXPORT_SYMBOL(mach_get_rtc_pll);
// EXPORT_SYMBOL(mach_set_rtc_pll);

#[cfg(all(any(CONFIG_M68KCLASSIC, CONFIG_SUN3), not(CONFIG_RTC_DRV_GENERIC)))]
pub unsafe extern "C" fn read_persistent_clock64(ts: *mut timespec64) {
    let mut time: rtc_time = core::mem::zeroed();

    (*ts).tv_sec = 0;
    (*ts).tv_nsec = 0;

    if mach_hwclk.is_none() {
        return;
    }

    mach_hwclk.unwrap()(0, &mut time);
    (*ts).tv_sec = mktime64(
        (time.tm_year + 1900) as c_int,
        (time.tm_mon + 1) as c_int,
        time.tm_mday as c_int,
        time.tm_hour as c_int,
        time.tm_min as c_int,
        time.tm_sec as c_int,
    );
}

#[cfg(all(any(CONFIG_M68KCLASSIC, CONFIG_SUN3), CONFIG_RTC_DRV_GENERIC))]
unsafe extern "C" fn rtc_generic_get_time(_dev: *mut device, tm: *mut rtc_time) -> c_int {
    mach_hwclk.unwrap()(0, tm);
    0
}

#[cfg(all(any(CONFIG_M68KCLASSIC, CONFIG_SUN3), CONFIG_RTC_DRV_GENERIC))]
unsafe extern "C" fn rtc_generic_set_time(_dev: *mut device, tm: *mut rtc_time) -> c_int {
    if mach_hwclk.unwrap()(1, tm) < 0 {
        return -EOPNOTSUPP;
    }
    0
}

#[cfg(all(any(CONFIG_M68KCLASSIC, CONFIG_SUN3), CONFIG_RTC_DRV_GENERIC))]
unsafe extern "C" fn rtc_ioctl(_dev: *mut device, cmd: c_uint, arg: c_ulong) -> c_int {
    let mut pll: rtc_pll_info = core::mem::zeroed();
    let argp = arg as *mut rtc_pll_info;

    match cmd {
        RTC_PLL_GET => {
            if mach_get_rtc_pll.is_none() || mach_get_rtc_pll.unwrap()(&mut pll) != 0 {
                return -EINVAL;
            }
            if copy_to_user(argp as *mut c_void, &pll as *const _ as *const c_void, core::mem::size_of::<rtc_pll_info>()) != 0 { -EFAULT } else { 0 }
        }
        RTC_PLL_SET => {
            if mach_set_rtc_pll.is_none() {
                return -EINVAL;
            }
            if capable(CAP_SYS_TIME) == 0 {
                return -EACCES;
            }
            if copy_from_user(&mut pll as *mut _ as *mut c_void, argp as *const c_void, core::mem::size_of::<rtc_pll_info>()) != 0 {
                return -EFAULT;
            }
            mach_set_rtc_pll.unwrap()(&mut pll)
        }
        _ => -ENOIOCTLCMD,
    }
}

#[cfg(all(any(CONFIG_M68KCLASSIC, CONFIG_SUN3), CONFIG_RTC_DRV_GENERIC))]
static generic_rtc_ops: rtc_class_ops = rtc_class_ops {
    ioctl: Some(rtc_ioctl),
    read_time: Some(rtc_generic_get_time),
    set_time: Some(rtc_generic_set_time),
};

#[cfg(all(any(CONFIG_M68KCLASSIC, CONFIG_SUN3), CONFIG_RTC_DRV_GENERIC))]
unsafe extern "C" fn rtc_init() -> c_int {
    if mach_hwclk.is_none() {
        return -ENODEV;
    }
    let pdev = platform_device_register_data(
        core::ptr::null_mut(),
        b"rtc-generic\\0".as_ptr() as *const c_char,
        -1,
        &generic_rtc_ops as *const _ as *const c_void,
        core::mem::size_of::<rtc_class_ops>(),
    );
    PTR_ERR_OR_ZERO(pdev)
}

// module_init(rtc_init);

pub unsafe extern "C" fn time_init() {
    mach_sched_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
