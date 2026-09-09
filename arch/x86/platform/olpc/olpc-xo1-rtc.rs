// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for OLPC XO-1 Real Time Clock (RTC)
 *
 * Copyright (C) 2011 One Laptop per Child
 */

// Dependencies supplied by the kernel headers and other translation units.

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct CmosRtcBoardInfo {
    pub rtc_day_alarm: u8,
    pub rtc_mon_alarm: u8,
    pub rtc_century: u8,
    pub wake_on: Option<unsafe extern "C" fn(*mut Device)>,
    pub wake_off: Option<unsafe extern "C" fn(*mut Device)>,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const u8,
    pub id: i32,
    pub num_resources: usize,
    pub dev: Device,
    pub platform_data: *mut CmosRtcBoardInfo,
    pub resource: *mut Resource,
}

extern "C" {
    fn olpc_xo1_pm_wakeup_set(value: u32);
    fn olpc_xo1_pm_wakeup_clear(value: u32);
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        type_: *const u8,
        compatible: *const u8,
    ) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn platform_device_register(device: *mut PlatformDevice) -> i32;
    fn device_init_wakeup(device: *mut Device, enable: i32);
    fn pr_info(format: *const u8, ...);
    fn rdmsrq(msr: u32, value: *mut u8);
}

// These values and macros are supplied by the corresponding kernel headers.
const IORESOURCE_IO: u64 = 0;
const IORESOURCE_IRQ: u64 = 0;
const RTC_IRQ: usize = 0;
const CS5536_PM_RTC: u32 = 0;
const MSR_RTC_DOMA_OFFSET: u32 = 0;
const MSR_RTC_MONA_OFFSET: u32 = 0;
const MSR_RTC_CEN_OFFSET: u32 = 0;

// RTC_PORT(0) and RTC_PORT(1) are header macros; their values are preserved
// here as the platform resource endpoints.
const RTC_PORT_0: usize = 0;
const RTC_PORT_1: usize = 1;

unsafe extern "C" fn rtc_wake_on(_dev: *mut Device) {
    olpc_xo1_pm_wakeup_set(CS5536_PM_RTC);
}

unsafe extern "C" fn rtc_wake_off(_dev: *mut Device) {
    olpc_xo1_pm_wakeup_clear(CS5536_PM_RTC);
}

static mut RTC_PLATFORM_RESOURCE: [Resource; 2] = [
    Resource {
        start: RTC_PORT_0,
        end: RTC_PORT_1,
        flags: IORESOURCE_IO,
    },
    Resource {
        start: RTC_IRQ,
        end: RTC_IRQ,
        flags: IORESOURCE_IRQ,
    },
];

static mut RTC_INFO: CmosRtcBoardInfo = CmosRtcBoardInfo {
    rtc_day_alarm: 0,
    rtc_mon_alarm: 0,
    rtc_century: 0,
    wake_on: Some(rtc_wake_on),
    wake_off: Some(rtc_wake_off),
};

static mut XO1_RTC_DEVICE: PlatformDevice = PlatformDevice {
    name: b"rtc_cmos\0".as_ptr(),
    id: -1,
    num_resources: 2,
    dev: Device { _private: [] },
    platform_data: unsafe { &raw mut RTC_INFO },
    resource: unsafe { RTC_PLATFORM_RESOURCE.as_mut_ptr() },
};

unsafe fn xo1_rtc_init() -> i32 {
    let mut r: i32;
    let node: *mut DeviceNode;

    node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"olpc,xo1-rtc\0".as_ptr(),
    );
    if node.is_null() {
        return 0;
    }
    of_node_put(node);

    pr_info(b"olpc-xo1-rtc: Initializing OLPC XO-1 RTC\n\0".as_ptr());
    rdmsrq(MSR_RTC_DOMA_OFFSET, &raw mut RTC_INFO.rtc_day_alarm);
    rdmsrq(MSR_RTC_MONA_OFFSET, &raw mut RTC_INFO.rtc_mon_alarm);
    rdmsrq(MSR_RTC_CEN_OFFSET, &raw mut RTC_INFO.rtc_century);

    r = platform_device_register(&raw mut XO1_RTC_DEVICE);
    if r != 0 {
        return r;
    }

    // x86_platform.legacy.rtc = 0;
    device_init_wakeup(&raw mut XO1_RTC_DEVICE.dev, 1);
    0
}

// arch_initcall(xo1_rtc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
