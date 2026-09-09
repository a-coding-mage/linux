// SPDX-License-Identifier: GPL-2.0
/* power.c: Power management driver.
 *
 * Copyright (C) 1999, 2007, 2008 David S. Miller (davem@davemloft.net)
 */

// Linux kernel dependencies:
// linux/kernel.h, linux/export.h, linux/init.h, linux/interrupt.h,
// linux/reboot.h, linux/of.h, linux/platform_device.h, asm/prom.h, asm/io.h

use core::ffi::c_void;

// Types and functions supplied by the surrounding kernel translation.
type IrqreturnT = i32;
type DevT = c_void;
type DeviceNode = c_void;
type Resource = c_void;
type PlatformDevice = c_void;
type OfDeviceId = c_void;
type PlatformDriver = c_void;

const IRQ_HANDLED: IrqreturnT = 1;

unsafe extern "C" {
    fn orderly_poweroff(force: bool);
    fn of_property_read_bool(np: *const DeviceNode, propname: *const u8) -> bool;
    fn of_ioremap(
        res: *mut Resource,
        offset: usize,
        size: usize,
        name: *const u8,
    ) -> *mut c_void;
    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> IrqreturnT,
        flags: u32,
        name: *const u8,
        dev_id: *mut c_void,
    ) -> i32;
    fn printk(fmt: *const u8, ...);
}

static mut power_reg: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn power_handler(_irq: i32, _dev_id: *mut c_void) -> IrqreturnT {
    orderly_poweroff(true);

    /* FIXME: Check registers for status... */
    IRQ_HANDLED
}

unsafe fn has_button_interrupt(irq: u32, dp: *mut DeviceNode) -> i32 {
    if irq == 0xffff_ffff {
        return 0;
    }
    if !of_property_read_bool(dp, b"button\0".as_ptr()) {
        return 0;
    }

    1
}

unsafe fn power_probe(op: *mut PlatformDevice) -> i32 {
    // struct resource *res = &op->resource[0];
    let res: *mut Resource = core::ptr::null_mut();
    // unsigned int irq = op->archdata.irqs[0];
    let irq: u32 = 0;

    power_reg = of_ioremap(res, 0, 0x4, b"power\0".as_ptr());

    // printk(KERN_INFO "%pOFn: Control reg at %llx\n",
    //        op->dev.of_node, res->start);

    if has_button_interrupt(irq, core::ptr::null_mut()) != 0 {
        if request_irq(irq, power_handler, 0, b"power\0".as_ptr(), core::ptr::null_mut()) < 0 {
            // printk(KERN_ERR "power: Cannot setup IRQ handler.\n");
        }
    }

    0
}

static power_match: [OfDeviceId; 2] = [
    // {
    //     .name = "power",
    // },
    // {},
    unsafe { core::mem::zeroed() },
    unsafe { core::mem::zeroed() },
];

static mut power_driver: *mut PlatformDriver = core::ptr::null_mut();

// builtin_platform_driver(power_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
