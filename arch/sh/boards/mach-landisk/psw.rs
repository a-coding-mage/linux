// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/landisk/psw.c
 *
 * push switch support for LANDISK and USL-5P
 *
 * Copyright (C) 2006-2007  Paul Mundt
 * Copyright (C) 2007  kogiidena
 */
// Dependencies supplied by the kernel and architecture-specific headers.

use core::ffi::c_void;

extern "C" {
    static PA_STATUS: *mut u8;
    static PA_PWRINT_CLR: *mut u8;
    static jiffies: usize;

    fn __raw_readb(addr: *mut u8) -> u8;
    fn __raw_writeb(value: u8, addr: *mut u8);
    fn mod_timer(timer: *mut TimerList, expires: usize) -> i32;
    fn platform_get_drvdata(pdev: *mut PlatformDevice) -> *mut PushSwitch;
    fn platform_add_devices(devices: *mut *mut PlatformDevice, count: usize) -> i32;
}

type IrqReturnT = i32;
type IrqHandler = unsafe extern "C" fn(i32, *mut c_void) -> IrqReturnT;

#[repr(C)]
pub struct TimerList {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PushSwitch {
    pub state: i32,
    pub debounce: TimerList,
}

#[repr(C)]
pub struct PushSwitchPlatformInfo {
    pub name: *const u8,
    pub bit: u32,
    pub irq_flags: u32,
    pub irq_handler: Option<IrqHandler>,
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub flags: u32,
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const u8,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut Resource,
    pub dev: Device,
}

const IRQ_RETVAL_MASK: IrqReturnT = 1;
const IRQF_SHARED: u32 = 0x0000_0080;
const IORESOURCE_IRQ: u32 = 0x0000_0400;
const IRQ_POWER: usize = 0;
const IRQ_BUTTON: usize = 0;

unsafe extern "C" fn psw_irq_handler(irq: i32, arg: *mut c_void) -> IrqReturnT {
    let _ = irq;
    let pdev = arg as *mut PlatformDevice;
    let psw = platform_get_drvdata(pdev);
    let psw_info = (*pdev).dev.platform_data as *mut PushSwitchPlatformInfo;
    let sw_value: u32;
    let mut ret: IrqReturnT = 0;

    sw_value = 0x0ff & (!(unsafe { __raw_readb(unsafe { PA_STATUS }) } as u32));

    /* Nothing to do if there's no state change */
    if (*psw).state != 0 {
        ret = 1;
    } else if (sw_value & (1u32 << (*psw_info).bit)) != 0 {
        (*psw).state = 1;
        mod_timer(&mut (*psw).debounce, unsafe { jiffies } + 50);
        ret = 1;
    }

    /* Clear the switch IRQs */
    __raw_writeb(0x00, unsafe { PA_PWRINT_CLR });

    IRQ_RETVAL_MASK & ret
}

static mut psw_power_resources: [Resource; 1] = [Resource {
    start: IRQ_POWER,
    flags: IORESOURCE_IRQ,
}];

static mut psw_usl5p_resources: [Resource; 1] = [Resource {
    start: IRQ_BUTTON,
    flags: IORESOURCE_IRQ,
}];

static mut psw_power_platform_data: PushSwitchPlatformInfo = PushSwitchPlatformInfo {
    name: b"psw_power\0".as_ptr(),
    bit: 4,
    irq_flags: IRQF_SHARED,
    irq_handler: Some(psw_irq_handler),
};

static mut psw1_platform_data: PushSwitchPlatformInfo = PushSwitchPlatformInfo {
    name: b"psw1\0".as_ptr(),
    bit: 0,
    irq_flags: IRQF_SHARED,
    irq_handler: Some(psw_irq_handler),
};

static mut psw2_platform_data: PushSwitchPlatformInfo = PushSwitchPlatformInfo {
    name: b"psw2\0".as_ptr(),
    bit: 2,
    irq_flags: IRQF_SHARED,
    irq_handler: Some(psw_irq_handler),
};

static mut psw3_platform_data: PushSwitchPlatformInfo = PushSwitchPlatformInfo {
    name: b"psw3\0".as_ptr(),
    bit: 1,
    irq_flags: IRQF_SHARED,
    irq_handler: Some(psw_irq_handler),
};

static mut psw_power_switch_device: PlatformDevice = PlatformDevice {
    name: b"push-switch\0".as_ptr(), id: 0, num_resources: 1,
    resource: unsafe { psw_power_resources.as_mut_ptr() },
    dev: Device { platform_data: unsafe { &mut psw_power_platform_data as *mut _ as *mut c_void } },
};

static mut psw1_switch_device: PlatformDevice = PlatformDevice {
    name: b"push-switch\0".as_ptr(), id: 1, num_resources: 1,
    resource: unsafe { psw_usl5p_resources.as_mut_ptr() },
    dev: Device { platform_data: unsafe { &mut psw1_platform_data as *mut _ as *mut c_void } },
};

static mut psw2_switch_device: PlatformDevice = PlatformDevice {
    name: b"push-switch\0".as_ptr(), id: 2, num_resources: 1,
    resource: unsafe { psw_usl5p_resources.as_mut_ptr() },
    dev: Device { platform_data: unsafe { &mut psw2_platform_data as *mut _ as *mut c_void } },
};

static mut psw3_switch_device: PlatformDevice = PlatformDevice {
    name: b"push-switch\0".as_ptr(), id: 3, num_resources: 1,
    resource: unsafe { psw_usl5p_resources.as_mut_ptr() },
    dev: Device { platform_data: unsafe { &mut psw3_platform_data as *mut _ as *mut c_void } },
};

static mut psw_devices: [*mut PlatformDevice; 4] = [
    unsafe { &mut psw_power_switch_device },
    unsafe { &mut psw1_switch_device },
    unsafe { &mut psw2_switch_device },
    unsafe { &mut psw3_switch_device },
];

unsafe extern "C" fn psw_init() -> i32 {
    platform_add_devices(psw_devices.as_mut_ptr(), psw_devices.len())
}

// device_initcall(psw_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
