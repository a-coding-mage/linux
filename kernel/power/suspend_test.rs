// SPDX-License-Identifier: GPL-2.0-only
/*
 * kernel/power/suspend_test.c - Suspend to RAM and standby test facility.
 *
 * Copyright (c) 2009 Pavel Machek <pavel@ucw.cz>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const TEST_SUSPEND_SECONDS: i64 = 10;

static mut suspend_test_start_time: c_ulong = 0;
static mut test_repeat_count_max: u32 = 1;
static mut test_repeat_count_current: u32 = 0;

pub unsafe fn suspend_test_start() {
	/* FIXME Use better timebase than "jiffies", ideally a clocksource.
	 * What we want is a hardware counter that will work correctly even
	 * during the irqs-are-off stages of the suspend/resume cycle...
	 */
	suspend_test_start_time = jiffies;
}

pub unsafe fn suspend_test_finish(label: *const c_char) {
	let nj: c_long = (jiffies as c_long).wrapping_sub(suspend_test_start_time as c_long);
	let msec: c_uint = jiffies_to_msecs(nj.unsigned_abs() as c_ulong);
	pr_info!(b"PM: %s took %d.%03d seconds\n\0".as_ptr() as *const c_char,
		label, msec / 1000, msec % 1000);

	/* Warning on suspend means the RTC alarm period needs to be
	 * larger -- the system was sooo slooowwww to suspend that the
	 * alarm (should have) fired before the system went to sleep!
	 *
	 * Warning on either suspend or resume also means the system
	 * has some performance issues.  The stack dump of a WARN_ON
	 * is more likely to get the right attention than a printk...
	 */
	WARN!(msec > (TEST_SUSPEND_SECONDS as c_uint * 1000),
		b"Component: %s, time: %u\n\0".as_ptr() as *const c_char, label, msec);
}

/*
 * To test system suspend, we need a hands-off mechanism to resume the
 * system.  RTCs wake alarms are a common self-contained mechanism.
 */

unsafe fn test_wakealarm(rtc: *mut rtc_device, mut state: suspend_state_t) {
	static mut err_readtime: [c_char; 39] = *b"PM: can't read %s time, err %d\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
	static mut err_wakealarm: [c_char; 40] = *b"PM: can't set %s wakealarm, err %d\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
	static mut err_suspend: [c_char; 38] = *b"PM: suspend test failed, error %d\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
	static mut info_test: [c_char; 43] = *b"PM: test RTC wakeup from '%s' suspend\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

	let mut now: time64_t;
	let mut alm: rtc_wkalrm = core::mem::zeroed();
	let mut status: c_int;

repeat: loop {
		status = rtc_read_time(rtc, &mut alm.time);
		if status < 0 {
			printk!(err_readtime.as_ptr(), dev_name((&(*rtc).dev) as *const device), status);
			return;
		}
		now = rtc_tm_to_time64(&alm.time);
		alm = core::mem::zeroed();
		rtc_time64_to_tm(now + TEST_SUSPEND_SECONDS, &mut alm.time);
		alm.enabled = true;

		status = rtc_set_alarm(rtc, &mut alm);
		if status < 0 {
			printk!(err_wakealarm.as_ptr(), dev_name((&(*rtc).dev) as *const device), status);
			return;
		}

		if state == PM_SUSPEND_MEM {
			printk!(info_test.as_ptr(), pm_states[state as usize]);
			status = pm_suspend(state);
			if status == -ENODEV { state = PM_SUSPEND_STANDBY; }
		}
		if state == PM_SUSPEND_STANDBY {
			printk!(info_test.as_ptr(), pm_states[state as usize]);
			status = pm_suspend(state);
			if status < 0 { state = PM_SUSPEND_TO_IDLE; }
		}
		if state == PM_SUSPEND_TO_IDLE {
			printk!(info_test.as_ptr(), pm_states[state as usize]);
			status = pm_suspend(state);
		}
		if status < 0 { printk!(err_suspend.as_ptr(), status); }
		test_repeat_count_current += 1;
		if test_repeat_count_current < test_repeat_count_max { continue 'repeat; }
		alm.enabled = false;
		rtc_set_alarm(rtc, &mut alm);
		break;
	}
}

unsafe fn has_wakealarm(dev: *mut device, _data: *const c_void) -> c_int {
	let candidate = to_rtc_device(dev);
	if !test_bit(RTC_FEATURE_ALARM, (*candidate).features) { return 0; }
	if !device_may_wakeup((*candidate).dev.parent) { return 0; }
	1
}

static mut test_state_label: *const c_char = core::ptr::null();
static mut warn_bad_state: [c_char; 36] = *b"PM: can't test '%s' suspend state\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

unsafe fn setup_test_suspend(mut value: *mut c_char) -> c_int {
	value = value.add(1);
	let suspend_type = strsep(&mut value, b",\0".as_ptr() as *const c_char);
	if suspend_type.is_null() { return 1; }
	let repeat = strsep(&mut value, b",\0".as_ptr() as *const c_char);
	if !repeat.is_null() && kstrtou32(repeat, 0, &mut test_repeat_count_max) != 0 { return 1; }
	for i in PM_SUSPEND_MIN..PM_SUSPEND_MAX {
		if strcmp(pm_labels[i as usize], suspend_type) == 0 {
			test_state_label = pm_labels[i as usize];
			return 1;
		}
	}
	printk!(warn_bad_state.as_ptr(), suspend_type);
	1
}

unsafe fn test_suspend() -> c_int {
	static mut warn_no_rtc: [c_char; 51] = *b"PM: no wakealarm-capable RTC driver is ready\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
	if test_state_label.is_null() { return 0; }
	let mut test_state = PM_SUSPEND_MIN;
	while test_state < PM_SUSPEND_MAX {
		let state_label = pm_states[test_state as usize];
		if !state_label.is_null() && strcmp(test_state_label, state_label) == 0 { break; }
		test_state += 1;
	}
	if test_state == PM_SUSPEND_MAX { printk!(warn_bad_state.as_ptr(), test_state_label); return 0; }
	let dev = class_find_device(&rtc_class, core::ptr::null_mut(), core::ptr::null(), Some(has_wakealarm));
	if dev.is_null() { printk!(warn_no_rtc.as_ptr()); return 0; }
	let rtc = rtc_class_open(dev_name(dev));
	put_device(dev);
	if rtc.is_null() { printk!(warn_no_rtc.as_ptr()); return 0; }
	test_wakealarm(rtc, test_state);
	rtc_class_close(rtc);
	0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
