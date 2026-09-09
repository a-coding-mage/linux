// SPDX-License-Identifier: GPL-2.0
/*
 * R-Car Generation 2 da9063(L)/da9210 regulator quirk
 *
 * Certain Gen2 development boards have an da9063 and one or more da9210
 * regulators. All of these regulators have their interrupt request lines
 * tied to the same interrupt pin (IRQ2) on the SoC.
 *
 * After cold boot or da9063-induced restart, both the da9063 and da9210 seem
 * to assert their interrupt request lines. Hence as soon as one driver
 * requests this irq, it gets stuck in an interrupt storm, as it only manages
 * to deassert its own interrupt request line, and the other driver hasn't
 * installed an interrupt handler yet.
 *
 * To handle this, install a quirk that masks the interrupts in both the
 * da9063 and da9210. This quirk has to run after the i2c master driver has
 * been initialized, but before the i2c slave drivers are initialized.
 *
 * Copyright (C) 2015 Glider bvba
 */

// Linux dependencies supplied externally.
use core::ffi::c_void;

const IRQC_BASE: usize = 0xe61c0000;
const IRQC_MONITOR: usize = 0x104;
const REGULATOR_IRQ_MASK: u32 = 1 << 2;
const DA9210_REG_MASK_A: u8 = 0x54;

#[repr(C)]
struct RegulatorQuirk {
    list: ListHead,
    id: *const OfDeviceId,
    np: *mut DeviceNode,
    irq_args: OfPhandleArgs,
    i2c_msg: I2cMsg,
    shared: bool,
}

#[repr(C)] struct ListHead { next: *mut ListHead, prev: *mut ListHead }
#[repr(C)] struct OfDeviceId { compatible: *const u8, data: *const c_void }
#[repr(C)] struct DeviceNode { parent: *mut DeviceNode }
#[repr(C)] struct OfPhandleArgs { _private: [u8; 32] }
#[repr(C)] struct I2cMsg { addr: u16, flags: u16, len: u16, buf: *mut u8 }
#[repr(C)] struct Device { type_: *const c_void, parent: *mut Device }
#[repr(C)] struct I2cClient { dev: Device, adapter: *mut c_void, name: *const u8 }
#[repr(C)] struct NotifierBlock { notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, usize, *mut c_void) -> i32> }

extern "C" {
    static mut i2c_adapter_type: c_void;
    static mut i2c_bus_type: c_void;
    static mut da9063_reg_irq_mask_a: u8;
    fn ioread32(addr: *mut c_void) -> u32;
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn i2c_transfer(adapter: *mut c_void, msg: *mut I2cMsg, num: i32) -> i32;
    fn of_machine_compatible_match(boards: *const *const u8) -> bool;
    fn bus_register_notifier(bus: *mut c_void, nb: *mut NotifierBlock) -> i32;
    fn of_node_put(np: *mut DeviceNode);
    fn of_node_get(np: *mut DeviceNode) -> *mut DeviceNode;
    fn of_device_is_available(np: *mut DeviceNode) -> bool;
    fn of_property_read_u32(np: *mut DeviceNode, name: *const u8, value: *mut u32) -> i32;
    fn of_irq_parse_one(np: *mut DeviceNode, index: i32, args: *mut OfPhandleArgs) -> i32;
    fn of_phandle_args_equal(a: *const OfPhandleArgs, b: *const OfPhandleArgs) -> bool;
    fn kfree(ptr: *mut RegulatorQuirk);
    fn printk_debug(fmt: *const u8, ...);
    fn printk_info(fmt: *const u8, ...);
    fn printk_device_debug(dev: *mut Device, fmt: *const u8, ...);
    fn printk_device_info(dev: *mut Device, fmt: *const u8, ...);
    fn printk_device_error(dev: *mut Device, fmt: *const u8, ...);
}

static mut QUIRK_LIST: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut irqc: *mut c_void = core::ptr::null_mut();
static mut da9063_irq_clr: [u8; 5] = [0, 0xff, 0xff, 0xff, 0xff];
static mut da9210_irq_clr: [u8; 3] = [DA9210_REG_MASK_A, 0xff, 0xff];
static mut da9063_msg: I2cMsg = I2cMsg { addr: 0, flags: 0, len: 5, buf: core::ptr::null_mut() };
static mut da9210_msg: I2cMsg = I2cMsg { addr: 0, flags: 0, len: 3, buf: core::ptr::null_mut() };
static mut regulator_quirk_nb: NotifierBlock = NotifierBlock { notifier_call: Some(regulator_quirk_notify) };

static rcar_gen2_quirk_match: [OfDeviceId; 4] = [
    OfDeviceId { compatible: b"dlg,da9063\0".as_ptr(), data: core::ptr::null() },
    OfDeviceId { compatible: b"dlg,da9063l\0".as_ptr(), data: core::ptr::null() },
    OfDeviceId { compatible: b"dlg,da9210\0".as_ptr(), data: core::ptr::null() },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn regulator_quirk_notify(_nb: *mut NotifierBlock, action: usize, data: *mut c_void) -> i32 {
    static mut done: bool = false;
    if done { return 0; }
    let mon = ioread32(irqc.add(IRQC_MONITOR));
    if mon & REGULATOR_IRQ_MASK != 0 { done = true; iounmap(irqc); return 0; }
    if action != 1 { return 0; }
    let client = data as *mut I2cClient;
    let mut pos = QUIRK_LIST.next;
    while pos != &mut QUIRK_LIST as *mut _ {
        let q = pos as *mut RegulatorQuirk;
        if (*q).shared { let _ = i2c_transfer((*client).adapter, &mut (*q).i2c_msg, 1); }
        pos = (*pos).next;
    }
    if ioread32(irqc.add(IRQC_MONITOR)) & REGULATOR_IRQ_MASK != 0 { done = true; iounmap(irqc); }
    0
}

#[allow(dead_code)]
unsafe extern "C" fn rcar_gen2_regulator_quirk() -> i32 {
    let boards = [b"renesas,koelsch\0".as_ptr(), b"renesas,lager\0".as_ptr(), b"renesas,porter\0".as_ptr(), b"renesas,stout\0".as_ptr(), b"renesas,gose\0".as_ptr(), core::ptr::null()];
    if !of_machine_compatible_match(boards.as_ptr()) { return -19; }
    irqc = ioremap(IRQC_BASE, 4096);
    if irqc.is_null() { return -12; }
    if ioread32(irqc.add(IRQC_MONITOR)) & REGULATOR_IRQ_MASK != 0 { iounmap(irqc); return 0; }
    bus_register_notifier(&mut i2c_bus_type, &mut regulator_quirk_nb);
    0
}

// arch_initcall(rcar_gen2_regulator_quirk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
