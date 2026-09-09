// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV Real Time Clock.
 *
 * Copyright 2011 IBM Corp.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/time.h, linux/bcd.h, linux/rtc.h, linux/delay.h,
// linux/of.h, linux/of_platform.h, linux/platform_device.h, asm/opal.h,
// asm/firmware.h, and asm/machdep.h.

unsafe fn opal_to_tm(y_m_d: u32, h_m_s_ms: u64, tm: *mut rtc_time) {
    (*tm).tm_year = ((bcd2bin(y_m_d >> 24) * 100)
        + bcd2bin((y_m_d >> 16) & 0xff)) - 1900;
    (*tm).tm_mon = bcd2bin((y_m_d >> 8) & 0xff) - 1;
    (*tm).tm_mday = bcd2bin(y_m_d & 0xff);
    (*tm).tm_hour = bcd2bin((h_m_s_ms >> 56) & 0xff);
    (*tm).tm_min = bcd2bin((h_m_s_ms >> 48) & 0xff);
    (*tm).tm_sec = bcd2bin((h_m_s_ms >> 40) & 0xff);
    (*tm).tm_wday = -1;
}

unsafe fn opal_get_boot_time() -> time64_t {
    let mut tm: rtc_time;
    let mut y_m_d: u32;
    let mut h_m_s_ms: u64;
    let mut __y_m_d: __be32;
    let mut __h_m_s_ms: __be64;
    let mut rc: c_long = OPAL_BUSY;

    if opal_check_token(OPAL_RTC_READ) == 0 {
        return 0;
    }

    while rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT {
        rc = opal_rtc_read(&mut __y_m_d, &mut __h_m_s_ms);
        if rc == OPAL_BUSY_EVENT {
            mdelay(OPAL_BUSY_DELAY_MS);
            opal_poll_events(core::ptr::null_mut());
        } else if rc == OPAL_BUSY {
            mdelay(OPAL_BUSY_DELAY_MS);
        }
    }
    if rc != OPAL_SUCCESS {
        return 0;
    }

    y_m_d = be32_to_cpu(__y_m_d);
    h_m_s_ms = be64_to_cpu(__h_m_s_ms);
    opal_to_tm(y_m_d, h_m_s_ms, &mut tm);
    rtc_tm_to_time64(&tm)
}

unsafe fn opal_time_init() -> c_int {
    let mut pdev: *mut platform_device;
    let rtc: *mut device_node;

    rtc = of_find_node_by_path(c"/ibm,opal/rtc".as_ptr());
    if !rtc.is_null() {
        pdev = of_platform_device_create(rtc, c"opal-rtc".as_ptr(), core::ptr::null_mut());
        of_node_put(rtc);
    } else {
        if opal_check_token(OPAL_RTC_READ) != 0 || opal_check_token(OPAL_READ_TPO) != 0 {
            pdev = platform_device_register_simple(
                c"opal-rtc".as_ptr(),
                -1,
                core::ptr::null(),
                0,
            );
        } else {
            return -ENODEV;
        }
    }

    PTR_ERR_OR_ZERO(pdev)
}

// machine_subsys_initcall(powernv, opal_time_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
