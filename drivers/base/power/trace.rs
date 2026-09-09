// SPDX-License-Identifier: GPL-2.0
/*
 * drivers/base/power/trace.c
 *
 * Copyright (C) 2006 Linus Torvalds
 *
 * Trace facility for suspend/resume problems, when none of the
 * devices may be working.
 */

// C headers and build-time registration macros are supplied by the kernel
// environment and are intentionally not reproduced here.

const USERHASH: u32 = 16;
const FILEHASH: u32 = 997;
const DEVHASH: u32 = 1009;
const DEVSEED: u32 = 7919;

pub static mut pm_trace_rtc_abused: bool = false;
static mut dev_hash_value: u32 = 0;

unsafe fn set_magic_time(user: u32, file: u32, device: u32) -> i32 {
    let mut n = user + USERHASH * (file + FILEHASH * device);

    // June 7th, 2006
    static mut time: rtc_time = rtc_time {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 7,
        tm_mon: 5, // June - counting from zero
        tm_year: 106,
        tm_wday: 3,
        tm_yday: 160,
        tm_isdst: 1,
    };

    time.tm_year = n % 100;
    n /= 100;
    time.tm_mon = n % 12;
    n /= 12;
    time.tm_mday = (n % 28) + 1;
    n /= 28;
    time.tm_hour = n % 24;
    n /= 24;
    time.tm_min = (n % 20) * 3;
    n /= 20;
    mc146818_set_time(&mut time);
    pm_trace_rtc_abused = true;
    if n != 0 { -1 } else { 0 }
}

unsafe fn read_magic_time() -> u32 {
    let mut time: rtc_time = core::mem::zeroed();

    if mc146818_get_time(&mut time, 1000) < 0 {
        pr_err!("Unable to read current time from RTC\n");
        return 0;
    }

    pr_info!("RTC time: %ptRt, date: %ptRd\n", &time, &time);
    let mut val = time.tm_year; // 100 years
    if val > 100 {
        val -= 100;
    }
    val += time.tm_mon * 100; // 12 months
    val += (time.tm_mday - 1) * 100 * 12; // 28 month-days
    val += time.tm_hour * 100 * 12 * 28; // 24 hours
    val += (time.tm_min / 3) * 100 * 12 * 28 * 24; // 20 3-minute intervals
    val
}

/*
 * This is just the sdbm hash function with a user-supplied
 * seed and final size parameter.
 */
unsafe fn hash_string(mut seed: u32, mut data: *const i8, modulus: u32) -> u32 {
    loop {
        let c = *(data as *const u8);
        data = data.add(1);
        if c == 0 {
            break;
        }
        seed = (seed << 16).wrapping_add(seed << 6).wrapping_sub(seed).wrapping_add(c as u32);
    }
    seed % modulus
}

pub unsafe fn set_trace_device(dev: *mut device) {
    dev_hash_value = hash_string(DEVSEED, dev_name(dev), DEVHASH);
}

/*
 * We could just take the "tracedata" index into the .tracedata
 * section instead. Generating a hash of the data gives us a
 * chance to work across kernel versions, and perhaps more
 * importantly it also gives us valid/invalid check (ie we will
 * likely not give totally bogus reports - if the hash matches,
 * it's not any guarantee, but it's a high _likelihood_ that the
 * match is valid).
 */
pub unsafe fn generate_pm_trace(tracedata: *const core::ffi::c_void, user: u32) {
    let p = tracedata as *const u8;
    let lineno = *(p as *const u16);
    let file = *(p.add(2) as *const *const i8);

    if !x86_platform.legacy.rtc {
        return;
    }
    let user_hash_value = user % USERHASH;
    let file_hash_value = hash_string(lineno as u32, file, FILEHASH);
    set_magic_time(user_hash_value, file_hash_value, dev_hash_value);
}

extern "C" {
    static mut __tracedata_start: u8;
    static mut __tracedata_end: u8;
}

unsafe fn show_file_hash(value: u32) -> i32 {
    let mut match_count = 0;
    let mut tracedata = &raw mut __tracedata_start;
    let end = &raw mut __tracedata_end;
    while tracedata < end {
        let lineno = *(tracedata as *const u16);
        let file = *((tracedata.add(2)) as *const *const i8);
        let hash = hash_string(lineno as u32, file, FILEHASH);
        if hash == value {
            pr_info!("  hash matches %s:%u\n", file, lineno);
            match_count += 1;
        }
        tracedata = tracedata.add(2 + core::mem::size_of::<usize>());
    }
    match_count
}

unsafe fn show_dev_hash(value: u32) -> i32 {
    let mut match_count = 0;
    device_pm_lock();
    let mut entry = dpm_list.prev;
    while entry != &raw mut dpm_list {
        let dev = to_device(entry);
        let hash = hash_string(DEVSEED, dev_name(dev), DEVHASH);
        if hash == value {
            dev_info!(dev, "hash matches\n");
            match_count += 1;
        }
        entry = (*entry).prev;
    }
    device_pm_unlock();
    match_count
}

static mut hash_value_early_read: u32 = 0;

pub unsafe fn show_trace_dev_match(mut buf: *mut i8, mut size: usize) -> i32 {
    let value = hash_value_early_read / (USERHASH * FILEHASH);
    let mut ret = 0;
    device_pm_lock();
    let mut entry = dpm_list.prev;
    while size != 0 && entry != &raw mut dpm_list {
        let dev = to_device(entry);
        let hash = hash_string(DEVSEED, dev_name(dev), DEVHASH);
        if hash == value {
            let len = scnprintf!(buf, size, "%s\n", dev_driver_string(dev));
            buf = buf.add(len);
            ret += len as i32;
            size -= len;
        }
        entry = (*entry).prev;
    }
    device_pm_unlock();
    ret
}

unsafe fn pm_trace_notify(_nb: *mut notifier_block, mode: usize, _unused: *mut core::ffi::c_void) -> i32 {
    match mode {
        PM_POST_HIBERNATION | PM_POST_SUSPEND => {
            if pm_trace_rtc_abused {
                pm_trace_rtc_abused = false;
                pr_warn!("Possible incorrect RTC due to pm_trace, please use 'ntpdate' or 'rdate' to reset it.\n");
            }
        }
        _ => {}
    }
    0
}

static mut pm_trace_nb: notifier_block = notifier_block {
    notifier_call: Some(pm_trace_notify),
};

unsafe fn early_resume_init() -> i32 {
    if !x86_platform.legacy.rtc {
        return 0;
    }
    hash_value_early_read = read_magic_time();
    register_pm_notifier(&mut pm_trace_nb);
    0
}

unsafe fn late_resume_init() -> i32 {
    let mut val = hash_value_early_read;
    if !x86_platform.legacy.rtc {
        return 0;
    }
    let user = val % USERHASH;
    val /= USERHASH;
    let file = val % FILEHASH;
    val /= FILEHASH;
    let dev = val; // % DEVHASH
    pr_info!("  Magic number: %d:%d:%d\n", user, file, dev);
    show_file_hash(file);
    show_dev_hash(dev);
    0
}

// core_initcall(early_resume_init);
// late_initcall(late_resume_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
