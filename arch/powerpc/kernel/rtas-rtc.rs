// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

const MAX_RTC_WAIT: u64 = 5000; // 5 sec

pub unsafe fn rtas_get_boot_time() -> time64_t {
    let mut ret: [i32; 8] = [0; 8];
    let mut error: i32;
    let mut wait_time: u32;
    let max_wait_tb: u64 = get_tb()
        .wrapping_add(tb_ticks_per_usec.wrapping_mul(1000).wrapping_mul(MAX_RTC_WAIT));

    loop {
        error = rtas_call(
            rtas_function_token(RTAS_FN_GET_TIME_OF_DAY),
            0,
            8,
            ret.as_mut_ptr(),
        );

        wait_time = rtas_busy_delay_time(error);
        if wait_time != 0 {
            // This is boot time so we spin.
            udelay(wait_time.wrapping_mul(1000));
        }

        if !(wait_time != 0 && get_tb() < max_wait_tb) {
            break;
        }
    }

    if error != 0 {
        printk_ratelimited(KERN_WARNING, "error: reading the clock failed (%d)\n", error);
        return 0;
    }

    mktime64(
        ret[0] as i64,
        ret[1] as i64,
        ret[2] as i64,
        ret[3] as i64,
        ret[4] as i64,
        ret[5] as i64,
    )
}

/* NOTE: get_rtc_time will get an error if executed in interrupt context
 * and if a delay is needed to read the clock.  In this case we just
 * silently return without updating rtc_tm.
 */
pub unsafe fn rtas_get_rtc_time(rtc_tm: *mut rtc_time) {
    let mut ret: [i32; 8] = [0; 8];
    let mut error: i32;
    let mut wait_time: u32;
    let max_wait_tb: u64 = get_tb()
        .wrapping_add(tb_ticks_per_usec.wrapping_mul(1000).wrapping_mul(MAX_RTC_WAIT));

    loop {
        error = rtas_call(
            rtas_function_token(RTAS_FN_GET_TIME_OF_DAY),
            0,
            8,
            ret.as_mut_ptr(),
        );

        wait_time = rtas_busy_delay_time(error);
        if wait_time != 0 {
            if in_interrupt() {
                memset(
                    rtc_tm as *mut core::ffi::c_void,
                    0,
                    core::mem::size_of::<rtc_time>(),
                );
                printk_ratelimited(
                    KERN_WARNING,
                    "error: reading clock would delay interrupt\n",
                );
                return; // delay not allowed
            }
            msleep(wait_time);
        }

        if !(wait_time != 0 && get_tb() < max_wait_tb) {
            break;
        }
    }

    if error != 0 {
        printk_ratelimited(KERN_WARNING, "error: reading the clock failed (%d)\n", error);
        return;
    }

    (*rtc_tm).tm_sec = ret[5];
    (*rtc_tm).tm_min = ret[4];
    (*rtc_tm).tm_hour = ret[3];
    (*rtc_tm).tm_mday = ret[2];
    (*rtc_tm).tm_mon = ret[1] - 1;
    (*rtc_tm).tm_year = ret[0] - 1900;
}

pub unsafe fn rtas_set_rtc_time(tm: *mut rtc_time) -> i32 {
    let mut error: i32;
    let mut wait_time: i32;
    let max_wait_tb: u64 = get_tb()
        .wrapping_add(tb_ticks_per_usec.wrapping_mul(1000).wrapping_mul(MAX_RTC_WAIT));

    loop {
        error = rtas_call(
            rtas_function_token(RTAS_FN_SET_TIME_OF_DAY),
            7,
            1,
            core::ptr::null_mut(),
            (*tm).tm_year + 1900,
            (*tm).tm_mon + 1,
            (*tm).tm_mday,
            (*tm).tm_hour,
            (*tm).tm_min,
            (*tm).tm_sec,
            0,
        );

        wait_time = rtas_busy_delay_time(error) as i32;
        if wait_time != 0 {
            if in_interrupt() {
                return 1; // probably decrementer
            }
            msleep(wait_time as u32);
        }

        if !(wait_time != 0 && get_tb() < max_wait_tb) {
            break;
        }
    }

    if error != 0 {
        printk_ratelimited(KERN_WARNING, "error: setting the clock failed (%d)\n", error);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
