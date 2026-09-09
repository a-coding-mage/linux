// SPDX-License-Identifier: GPL-2.0
/*
 * extcon driver for Basin Cove PMIC
 *
 * Copyright (c) 2019, Intel Corporation.
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 */

// The following declarations are supplied by the kernel and related headers.
use core::ffi::c_void;

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct extcon_dev;
#[repr(C)]
pub struct platform_device;

pub type CInt = i32;
pub type Uint = u32;
pub type Bool = bool;
pub type IrqreturnT = CInt;

pub const IRQ_NONE: IrqreturnT = 0;
pub const IRQ_HANDLED: IrqreturnT = 1;
pub const ENODATA: CInt = 61;
pub const ENOMEM: CInt = 12;

pub const BCOVE_USBIDCTRL: Uint = 0x19;
pub const BCOVE_USBIDCTRL_ID: Uint = 1 << 0;
pub const BCOVE_USBIDCTRL_ACA: Uint = 1 << 1;
pub const BCOVE_USBIDCTRL_ALL: Uint = BCOVE_USBIDCTRL_ID | BCOVE_USBIDCTRL_ACA;

pub const BCOVE_USBIDSTS: Uint = 0x1a;
pub const BCOVE_USBIDSTS_GND: Uint = 1 << 0;
pub const BCOVE_USBIDSTS_RARBRC_MASK: Uint = 0b11 << 1;
pub const BCOVE_USBIDSTS_RARBRC_SHIFT: Uint = 1;
pub const BCOVE_USBIDSTS_NO_ACA: Uint = 0;
pub const BCOVE_USBIDSTS_R_ID_A: Uint = 1;
pub const BCOVE_USBIDSTS_R_ID_B: Uint = 2;
pub const BCOVE_USBIDSTS_R_ID_C: Uint = 3;
pub const BCOVE_USBIDSTS_FLOAT: Uint = 1 << 3;
pub const BCOVE_USBIDSTS_SHORT: Uint = 1 << 4;

// Defined by the Intel PMIC headers.
pub const BCOVE_CHGRIRQ_ALL: Uint = BCOVE_CHGRIRQ_VBUSDET | BCOVE_CHGRIRQ_DCDET |
    BCOVE_CHGRIRQ_BATTDET | BCOVE_CHGRIRQ_USBIDDET;
pub const BCOVE_CHGRCTRL0: Uint = 0x4b;
pub const BCOVE_CHGRCTRL0_CHGRRESET: Uint = 1 << 0;
pub const BCOVE_CHGRCTRL0_EMRGCHREN: Uint = 1 << 1;
pub const BCOVE_CHGRCTRL0_EXTCHRDIS: Uint = 1 << 2;
pub const BCOVE_CHGRCTRL0_SWCONTROL: Uint = 1 << 3;
pub const BCOVE_CHGRCTRL0_TTLCK: Uint = 1 << 4;
pub const BCOVE_CHGRCTRL0_BIT_5: Uint = 1 << 5;
pub const BCOVE_CHGRCTRL0_BIT_6: Uint = 1 << 6;
pub const BCOVE_CHGRCTRL0_CHR_WDT_NOKICK: Uint = 1 << 7;

#[repr(C)]
pub struct mrfld_extcon_data {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub edev: *mut extcon_dev,
    pub status: Uint,
    pub id: Uint,
}

pub const mrfld_extcon_cable: [Uint; 7] = [
    EXTCON_USB, EXTCON_USB_HOST, EXTCON_CHG_USB_SDP, EXTCON_CHG_USB_CDP,
    EXTCON_CHG_USB_DCP, EXTCON_CHG_USB_ACA, EXTCON_NONE,
];

extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: Uint, mask: Uint, val: Uint) -> CInt;
    fn regmap_read(map: *mut regmap, reg: Uint, val: *mut Uint) -> CInt;
    fn dev_err(dev: *mut device, fmt: *const i8, ...) -> CInt;
    fn extcon_set_state_sync(edev: *mut extcon_dev, cable: Uint, state: Bool) -> CInt;
}

unsafe fn mrfld_extcon_clear(data: *mut mrfld_extcon_data, reg: Uint, mask: Uint) -> CInt {
    regmap_update_bits((*data).regmap, reg, mask, 0x00)
}

unsafe fn mrfld_extcon_set(data: *mut mrfld_extcon_data, reg: Uint, mask: Uint) -> CInt {
    regmap_update_bits((*data).regmap, reg, mask, 0xff)
}

unsafe fn mrfld_extcon_sw_control(data: *mut mrfld_extcon_data, enable: Bool) -> CInt {
    let mask = BCOVE_CHGRCTRL0_SWCONTROL;
    let ret = if enable { mrfld_extcon_set(data, BCOVE_CHGRCTRL0, mask) }
              else { mrfld_extcon_clear(data, BCOVE_CHGRCTRL0, mask) };
    if ret != 0 { dev_err((*data).dev, b"can't set SW control: %d\0".as_ptr() as *const i8, ret); }
    ret
}

unsafe fn mrfld_extcon_get_id(data: *mut mrfld_extcon_data) -> CInt {
    let mut id = 0;
    let ret = regmap_read((*data).regmap, BCOVE_USBIDSTS, &mut id);
    if ret != 0 { return ret; }
    if id & BCOVE_USBIDSTS_FLOAT != 0 { return INTEL_USB_ID_FLOAT; }
    match (id & BCOVE_USBIDSTS_RARBRC_MASK) >> BCOVE_USBIDSTS_RARBRC_SHIFT {
        BCOVE_USBIDSTS_R_ID_A => return INTEL_USB_RID_A,
        BCOVE_USBIDSTS_R_ID_B => return INTEL_USB_RID_B,
        BCOVE_USBIDSTS_R_ID_C => return INTEL_USB_RID_C,
        _ => (),
    }
    let ground = id & BCOVE_USBIDSTS_GND != 0;
    match b'A' + BCOVE_MAJOR((*data).id) as u8 {
        b'A' => if ground { INTEL_USB_ID_GND } else { INTEL_USB_ID_FLOAT },
        b'B' => if ground { INTEL_USB_ID_FLOAT } else { INTEL_USB_ID_GND },
        _ => INTEL_USB_ID_FLOAT,
    }
}

unsafe fn mrfld_extcon_role_detect(data: *mut mrfld_extcon_data) -> CInt {
    let id = mrfld_extcon_get_id(data);
    if id < 0 { return id; }
    let usb_host = id == INTEL_USB_ID_GND || id == INTEL_USB_RID_A;
    extcon_set_state_sync((*data).edev, EXTCON_USB_HOST, usb_host);
    0
}

unsafe fn mrfld_extcon_cable_detect(data: *mut mrfld_extcon_data) -> CInt {
    let mut status = 0;
    let ret = regmap_read((*data).regmap, BCOVE_SCHGRIRQ1, &mut status);
    if ret != 0 { return ret; }
    let change = status ^ (*data).status;
    if change == 0 { return -ENODATA; }
    if change & BCOVE_CHGRIRQ_USBIDDET != 0 {
        let ret = mrfld_extcon_role_detect(data);
        if ret != 0 { return ret; }
    }
    (*data).status = status;
    0
}

pub unsafe extern "C" fn mrfld_extcon_interrupt(_irq: CInt, dev_id: *mut c_void) -> IrqreturnT {
    let data = dev_id as *mut mrfld_extcon_data;
    let ret = mrfld_extcon_cable_detect(data);
    mrfld_extcon_clear(data, BCOVE_MIRQLVL1, BCOVE_LVL1_CHGR);
    if ret != 0 { IRQ_NONE } else { IRQ_HANDLED }
}

extern "C" {
    fn platform_get_irq(pdev: *mut platform_device, index: Uint) -> CInt;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: Uint) -> *mut c_void;
    fn devm_extcon_dev_allocate(dev: *mut device, cables: *const Uint) -> *mut extcon_dev;
    fn devm_extcon_dev_register(dev: *mut device, edev: *mut extcon_dev) -> CInt;
    fn devm_request_threaded_irq(dev: *mut device, irq: CInt, handler: Option<unsafe extern "C" fn(CInt, *mut c_void) -> IrqreturnT>, thread_fn: Option<unsafe extern "C" fn(CInt, *mut c_void) -> IrqreturnT>, flags: Uint, name: *const i8, data: *mut c_void) -> CInt;
    fn dev_err_probe(dev: *mut device, err: CInt, fmt: *const i8, ...) -> CInt;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut mrfld_extcon_data;
}

pub unsafe extern "C" fn mrfld_extcon_probe(pdev: *mut platform_device) -> CInt {
    let dev = pdev as *mut device;
    let pmic = dev_get_drvdata((*dev).cast::<device>()); // parent device supplied by platform
    let regmap = pmic as *mut regmap;
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    let data = devm_kzalloc(dev, core::mem::size_of::<mrfld_extcon_data>(), 0) as *mut mrfld_extcon_data;
    if data.is_null() { return -ENOMEM; }
    (*data).dev = dev;
    (*data).regmap = regmap;
    (*data).edev = devm_extcon_dev_allocate(dev, mrfld_extcon_cable.as_ptr());
    if (*data).edev.is_null() { return -ENOMEM; }
    let mut ret = devm_extcon_dev_register(dev, (*data).edev);
    if ret < 0 { return dev_err_probe(dev, ret, b"can't register extcon device\n\0".as_ptr() as *const i8); }
    ret = devm_request_threaded_irq(dev, irq, None, Some(mrfld_extcon_interrupt), IRQF_ONESHOT | IRQF_SHARED, core::ptr::null(), data.cast());
    if ret != 0 { return dev_err_probe(dev, ret, b"can't register IRQ handler\n\0".as_ptr() as *const i8); }
    let mut id = 0;
    ret = regmap_read(regmap, BCOVE_ID, &mut id);
    if ret != 0 { return dev_err_probe(dev, ret, b"can't read PMIC ID\n\0".as_ptr() as *const i8); }
    (*data).id = id;
    ret = mrfld_extcon_sw_control(data, true);
    if ret != 0 { return ret; }
    mrfld_extcon_role_detect(data);
    let mut status = 0;
    regmap_read(regmap, BCOVE_SCHGRIRQ1, &mut status);
    (*data).status = status;
    mrfld_extcon_clear(data, BCOVE_MIRQLVL1, BCOVE_LVL1_CHGR);
    mrfld_extcon_clear(data, BCOVE_MCHGRIRQ1, BCOVE_CHGRIRQ_ALL);
    mrfld_extcon_set(data, BCOVE_USBIDCTRL, BCOVE_USBIDCTRL_ALL);
    platform_set_drvdata(pdev, data.cast());
    0
}

pub unsafe extern "C" fn mrfld_extcon_remove(pdev: *mut platform_device) {
    mrfld_extcon_sw_control(platform_get_drvdata(pdev), false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
