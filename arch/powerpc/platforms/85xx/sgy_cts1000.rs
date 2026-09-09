// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Servergy CTS-1000 Setup
 *
 * Maintained by Ben Collins <ben.c@servergy.com>
 *
 * Copyright 2012 by Servergy, Inc.
 */

// pr_fmt(fmt) = "gpio-halt: " fmt
// Linux kernel dependencies are supplied by the surrounding translation unit.

use core::ffi::c_void;

extern "C" {
    static mut ppc_md: MachDep;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    fn orderly_poweroff(force: bool);
    fn panic(fmt: *const u8) -> !;
    fn gpiod_set_value(desc: *mut GpioDesc, value: i32);
    fn schedule_work(work: *mut WorkStruct);
    fn cancel_work_sync(work: *mut WorkStruct);
    fn request_irq(
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> IrqReturn,
        flags: u32,
        name: *const u8,
        dev: *mut PlatformDevice,
    ) -> i32;
    fn free_irq(irq: i32, dev: *mut PlatformDevice);
    fn fwnode_gpiod_get_index(
        node: *mut c_void,
        con_id: *const u8,
        index: u32,
        flags: u32,
        label: *const u8,
    ) -> *mut GpioDesc;
    fn irq_of_parse_and_map(node: *mut DeviceNode, index: i32) -> i32;
    fn of_find_matching_node(from: *mut DeviceNode, matches: *const OfDeviceId) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn gpiod_put(desc: *mut GpioDesc);
}

#[repr(C)]
pub struct GpioDesc;
#[repr(C)]
pub struct DeviceNode;
#[repr(C)]
pub struct Device;
#[repr(C)]
pub struct PlatformDevice { pub dev: Device }
#[repr(C)]
pub struct WorkStruct;
#[repr(C)]
pub struct MachDep { pub halt: Option<unsafe extern "C" fn()> }
#[repr(C)]
pub struct OfDeviceId { pub compatible: *const u8 }
#[repr(C)]
pub struct PlatformDriver {
    pub driver: Driver,
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut PlatformDevice)>,
}
#[repr(C)]
pub struct Driver { pub name: *const u8, pub of_match_table: *const OfDeviceId }
#[repr(C)]
pub struct IrqReturn;

const ENODEV: i32 = 19;
const GPIOD_OUT_LOW: u32 = 0;
const IRQF_TRIGGER_RISING: u32 = 0x0000_0040;
const IRQF_TRIGGER_FALLING: u32 = 0x0000_0080;
const IRQ_HANDLED: IrqReturn = IrqReturn;

static mut halt_gpio: *mut GpioDesc = core::ptr::null_mut();
static mut halt_irq: i32 = 0;

static child_match: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"sgy,gpio-halt\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn gpio_halt_wfn(_work: *mut WorkStruct) {
    // Likely wont return
    orderly_poweroff(true);
}

static mut gpio_halt_wq: WorkStruct = WorkStruct;

unsafe extern "C" fn gpio_halt_cb() -> ! {
    // pr_info("triggering GPIO.\n");
    // Probably wont return
    gpiod_set_value(halt_gpio, 1);
    panic(b"Halt failed\n\0".as_ptr());
}

unsafe extern "C" fn gpio_halt_irq(_irq: i32, data: *mut c_void) -> IrqReturn {
    let pdev = data as *mut PlatformDevice;
    // dev_info(&pdev->dev, "scheduling shutdown due to power button IRQ\n");
    schedule_work(&mut gpio_halt_wq);
    IRQ_HANDLED
}

unsafe extern "C" fn __gpio_halt_probe(
    pdev: *mut PlatformDevice,
    halt_node: *mut DeviceNode,
) -> i32 {
    let mut err: i32;
    halt_gpio = fwnode_gpiod_get_index(
        halt_node as *mut c_void,
        core::ptr::null(),
        0,
        GPIOD_OUT_LOW,
        b"gpio-halt\0".as_ptr(),
    );
    err = if halt_gpio.is_null() { 0 } else { 0 };
    if err != 0 { return err; }

    halt_irq = irq_of_parse_and_map(halt_node, 0);
    err = request_irq(halt_irq, gpio_halt_irq,
        IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
        b"gpio-halt\0".as_ptr(), pdev);
    if err != 0 {
        gpiod_put(halt_gpio);
        halt_gpio = core::ptr::null_mut();
        return err;
    }
    ppc_md.halt = Some(gpio_halt_cb);
    pm_power_off = Some(gpio_halt_cb);
    0
}

unsafe extern "C" fn gpio_halt_probe(pdev: *mut PlatformDevice) -> i32 {
    let halt_node = of_find_matching_node(core::ptr::null_mut(), child_match.as_ptr());
    if halt_node.is_null() { return -ENODEV; }
    let ret = __gpio_halt_probe(pdev, halt_node);
    of_node_put(halt_node);
    ret
}

unsafe extern "C" fn gpio_halt_remove(pdev: *mut PlatformDevice) {
    free_irq(halt_irq, pdev);
    cancel_work_sync(&mut gpio_halt_wq);
    ppc_md.halt = None;
    pm_power_off = None;
    gpiod_put(halt_gpio);
    halt_gpio = core::ptr::null_mut();
}

static gpio_halt_match: [OfDeviceId; 2] = [
    // We match on the gpio bus itself and scan the children since they wont
    // be matched against us. We know the bus wont match until registered too.
    OfDeviceId { compatible: b"fsl,qoriq-gpio\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut gpio_halt_driver: PlatformDriver = PlatformDriver {
    driver: Driver { name: b"gpio-halt\0".as_ptr(), of_match_table: gpio_halt_match.as_ptr() },
    probe: Some(gpio_halt_probe),
    remove: Some(gpio_halt_remove),
};

// module_platform_driver(gpio_halt_driver);
// MODULE_DEVICE_TABLE(of, gpio_halt_match);
// MODULE_DESCRIPTION("Driver to support GPIO triggered system halt for Servergy CTS-1000 Systems.");
// MODULE_VERSION("1.0");
// MODULE_AUTHOR("Ben Collins <ben.c@servergy.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
