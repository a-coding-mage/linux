/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for talking to the PMU.  The PMU is a microcontroller
 * which controls battery charging and system power on PowerBook 3400
 * and 2400 models as well as the RTC and various other things.
 *
 * Copyright (C) 1998 Paul Mackerras.
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/rtc.h and uapi/linux/pmu.h

extern "C" {
    pub fn find_via_pmu() -> ::core::ffi::c_int;

    pub fn pmu_request(
        req: *mut adb_request,
        done: Option<unsafe extern "C" fn(*mut adb_request)>,
        nbytes: ::core::ffi::c_int,
        ...,
    ) -> ::core::ffi::c_int;
    pub fn pmu_queue_request(req: *mut adb_request) -> ::core::ffi::c_int;
    pub fn pmu_poll();
    pub fn pmu_poll_adb(); /* For use by xmon */
    pub fn pmu_wait_complete(req: *mut adb_request);

    /* For use before switching interrupts off for a long time;
     * warning: not stackable
     */
    #[cfg(CONFIG_ADB_PMU)]
    pub fn pmu_suspend();
    #[cfg(CONFIG_ADB_PMU)]
    pub fn pmu_resume();

    pub fn pmu_enable_irled(on: ::core::ffi::c_int);

    pub fn pmu_get_time() -> time64_t;
    pub fn pmu_set_rtc_time(tm: *mut rtc_time) -> ::core::ffi::c_int;

    pub fn pmu_restart();
    pub fn pmu_shutdown();
    pub fn pmu_unlock();

    pub fn pmu_present() -> ::core::ffi::c_int;
    pub fn pmu_get_model() -> ::core::ffi::c_int;

    pub fn pmu_backlight_set_sleep(sleep: ::core::ffi::c_int);

    pub static mut pmu_battery_count: ::core::ffi::c_int;
    pub static mut pmu_batteries: [pmu_battery_info; PMU_MAX_BATTERIES as usize];
    pub static mut pmu_power_flags: ::core::ffi::c_uint;

    /* Backlight */
    pub fn pmu_backlight_init();

    /* some code needs to know if the PMU was suspended for hibernation */
    #[cfg(all(CONFIG_SUSPEND, CONFIG_PPC32))]
    pub static mut pmu_sys_suspended: ::core::ffi::c_int;
}

#[cfg(not(CONFIG_ADB_PMU))]
#[inline]
pub unsafe fn pmu_suspend() {}

#[cfg(not(CONFIG_ADB_PMU))]
#[inline]
pub unsafe fn pmu_resume() {}

pub const PMU_MAX_BATTERIES: u32 = 2;

/* values for pmu_power_flags */
pub const PMU_PWR_AC_PRESENT: u32 = 0x00000001;

/* values for pmu_battery_info.flags */
pub const PMU_BATT_PRESENT: u32 = 0x00000001;
pub const PMU_BATT_CHARGING: u32 = 0x00000002;
pub const PMU_BATT_TYPE_MASK: u32 = 0x000000f0;
pub const PMU_BATT_TYPE_SMART: u32 = 0x00000010; /* Smart battery */
pub const PMU_BATT_TYPE_HOOPER: u32 = 0x00000020; /* 3400/3500 */
pub const PMU_BATT_TYPE_COMET: u32 = 0x00000030; /* 2400 */

#[repr(C)]
pub struct pmu_battery_info {
    pub flags: ::core::ffi::c_uint,
    pub charge: ::core::ffi::c_uint, /* current charge */
    pub max_charge: ::core::ffi::c_uint, /* maximum charge */
    pub amperage: ::core::ffi::c_int, /* current, positive if charging */
    pub voltage: ::core::ffi::c_uint, /* voltage */
    pub time_remaining: ::core::ffi::c_uint, /* remaining time */
}

#[cfg(not(all(CONFIG_SUSPEND, CONFIG_PPC32)))]
pub const pmu_sys_suspended: ::core::ffi::c_int = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
