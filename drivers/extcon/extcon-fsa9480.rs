// SPDX-License-Identifier: GPL-2.0+
/*
 * extcon-fsa9480.c - Fairchild Semiconductor FSA9480 extcon driver
 *
 * Copyright (c) 2019 Tomasz Figa <tomasz.figa@gmail.com>
 *
 * Loosely based on old fsa9480 misc-device driver.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const FSA9480_REG_DEVID: u32 = 0x01;
const FSA9480_REG_CTRL: u32 = 0x02;
const FSA9480_REG_INT1: u32 = 0x03;
const FSA9480_REG_INT2: u32 = 0x04;
const FSA9480_REG_INT1_MASK: u32 = 0x05;
const FSA9480_REG_INT2_MASK: u32 = 0x06;
const FSA9480_REG_ADC: u32 = 0x07;
const FSA9480_REG_TIMING1: u32 = 0x08;
const FSA9480_REG_TIMING2: u32 = 0x09;
const FSA9480_REG_DEV_T1: u32 = 0x0a;
const FSA9480_REG_DEV_T2: u32 = 0x0b;
const FSA9480_REG_BTN1: u32 = 0x0c;
const FSA9480_REG_BTN2: u32 = 0x0d;
const FSA9480_REG_CK: u32 = 0x0e;
const FSA9480_REG_CK_INT1: u32 = 0x0f;
const FSA9480_REG_CK_INT2: u32 = 0x10;
const FSA9480_REG_CK_INTMASK1: u32 = 0x11;
const FSA9480_REG_CK_INTMASK2: u32 = 0x12;
const FSA9480_REG_MANSW1: u32 = 0x13;
const FSA9480_REG_MANSW2: u32 = 0x14;
const FSA9480_REG_END: u32 = 0x15;

const CON_SWITCH_OPEN: u32 = 1 << 4;
const CON_RAW_DATA: u32 = 1 << 3;
const CON_MANUAL_SW: u32 = 1 << 2;
const CON_WAIT: u32 = 1 << 1;
const CON_INT_MASK: u32 = 1 << 0;
const CON_MASK: u32 = CON_SWITCH_OPEN | CON_RAW_DATA | CON_MANUAL_SW | CON_WAIT;

const DEV_USB_OTG: u32 = 7;
const DEV_DEDICATED_CHG: u32 = 6;
const DEV_USB_CHG: u32 = 5;
const DEV_CAR_KIT: u32 = 4;
const DEV_UART: u32 = 3;
const DEV_USB: u32 = 2;
const DEV_AUDIO_2: u32 = 1;
const DEV_AUDIO_1: u32 = 0;
const DEV_T1_USB_MASK: u32 = DEV_USB_OTG | DEV_USB;
const DEV_T1_UART_MASK: u32 = DEV_UART;
const DEV_T1_CHARGER_MASK: u32 = DEV_DEDICATED_CHG | DEV_USB_CHG;
const DEV_AV: u32 = 14;
const DEV_TTY: u32 = 13;
const DEV_PPD: u32 = 12;
const DEV_JIG_UART_OFF: u32 = 11;
const DEV_JIG_UART_ON: u32 = 10;
const DEV_JIG_USB_OFF: u32 = 9;
const DEV_JIG_USB_ON: u32 = 8;
const DEV_T2_USB_MASK: u32 = DEV_JIG_USB_OFF | DEV_JIG_USB_ON;
const DEV_T2_UART_MASK: u32 = DEV_JIG_UART_OFF | DEV_JIG_UART_ON;
const DEV_T2_JIG_MASK: u32 = DEV_JIG_USB_OFF | DEV_JIG_USB_ON | DEV_JIG_UART_OFF | DEV_JIG_UART_ON;

const SW_VAUDIO: u32 = (4 << 5) | (4 << 2);
const SW_UART: u32 = (3 << 5) | (3 << 2);
const SW_AUDIO: u32 = (2 << 5) | (2 << 2);
const SW_DHOST: u32 = (1 << 5) | (1 << 2);
const SW_AUTO: u32 = 0;
const INT1_MASK: u32 = 0xff;
const INT_DETACH: u32 = 1 << 1;
const INT_ATTACH: u32 = 1 << 0;
const INT2_MASK: u32 = 0x1f;
const TIMING1_ADC_500MS: u32 = 0x6;

#[repr(C)]
struct fsa9480_usbsw {
    dev: *mut device,
    regmap: *mut regmap,
    edev: *mut extcon_dev,
    cable: u16,
}

#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct extcon_dev { _private: [u8; 0] }
#[repr(C)] struct i2c_client { dev: device, irq: i32 }
#[repr(C)] struct regmap_config { reg_bits: u32, val_bits: u32, volatile_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>, max_register: u32 }
#[repr(C)] struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] struct i2c_device_id { name: *const u8 }
#[repr(C)] struct of_device_id { compatible: *const u8 }
#[repr(C)] struct i2c_driver { _private: [u8; 0] }

extern "C" {
    fn regmap_write(map: *mut regmap, reg: i32, val: i32) -> i32;
    fn regmap_read(map: *mut regmap, reg: i32, val: *mut i32) -> i32;
    fn regmap_bulk_read(map: *mut regmap, reg: u32, val: *mut u8, count: usize) -> i32;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
    fn extcon_set_state_sync(dev: *mut extcon_dev, cable: i32, attached: bool) -> i32;
    fn fls64(x: u64) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut fsa9480_usbsw;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut fsa9480_usbsw);
    fn devm_extcon_dev_allocate(dev: *mut device, cables: *const u32) -> *mut extcon_dev;
    fn devm_extcon_dev_register(dev: *mut device, edev: *mut extcon_dev) -> i32;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_request_threaded_irq(dev: *mut device, irq: i32, handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32>, thread_fn: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32>, flags: u32, name: *const u8, data: *mut fsa9480_usbsw) -> i32;
    fn devm_device_init_wakeup(dev: *mut device);
    fn device_may_wakeup(dev: *mut device) -> bool;
    fn enable_irq_wake(irq: i32) -> i32;
    fn disable_irq_wake(irq: i32) -> i32;
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn i2c_add_driver(driver: *mut i2c_driver) -> i32;
    fn i2c_del_driver(driver: *mut i2c_driver);
}

static FSA9480_EXTCON_CABLE: [u32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 0xffffffff];
static CABLE_TYPES: [u64; 15] = [1 << 5, 1 << 4 | 1 << 2, 1 << 4 | 1 << 3, 1 << 4 | 1 << 3 | 1 << 5, 1 << 7, 1 << 4 | 1 << 3, 1 << 5, 1 << 5, 1 << 6 | 1 << 5, 1 << 7, 1 << 5 | 1 << 4, 1 << 7, 1 << 7, 1 << 4 | 1 << 7, 1 << 4 | 1 << 7];

unsafe extern "C" fn fsa9480_volatile_reg(_dev: *mut device, reg: u32) -> bool { reg == FSA9480_REG_INT1_MASK }
static FSA9480_REGMAP_CONFIG: regmap_config = regmap_config { reg_bits: 8, val_bits: 8, volatile_reg: Some(fsa9480_volatile_reg), max_register: FSA9480_REG_END };

unsafe fn fsa9480_write_reg(usbsw: *mut fsa9480_usbsw, reg: i32, value: i32) -> i32 { let ret = regmap_write((*usbsw).regmap, reg, value); if ret < 0 { dev_err((*usbsw).dev, b"fsa9480_write_reg: err %d\0".as_ptr(), ret); } ret }
unsafe fn fsa9480_read_reg(usbsw: *mut fsa9480_usbsw, reg: i32) -> i32 { let mut val = 0; let ret = regmap_read((*usbsw).regmap, reg, &mut val); if ret < 0 { dev_err((*usbsw).dev, b"fsa9480_read_reg: err %d\0".as_ptr(), ret); return ret; } val }
unsafe fn fsa9480_read_irq(usbsw: *mut fsa9480_usbsw, value: *mut i32) -> i32 { let mut regs = [0u8; 2]; let ret = regmap_bulk_read((*usbsw).regmap, FSA9480_REG_INT1, regs.as_mut_ptr(), 2); if ret < 0 { dev_err((*usbsw).dev, b"fsa9480_read_irq: err %d\0".as_ptr(), ret); } *value = ((regs[1] as i32) << 8) | regs[0] as i32; ret }
unsafe fn fsa9480_handle_change(usbsw: *mut fsa9480_usbsw, mut mask: u16, attached: bool) { while mask != 0 { let dev = fls64(mask as u64) - 1; let mut cables = CABLE_TYPES[dev as usize]; while cables != 0 { let cable = fls64(cables) - 1; extcon_set_state_sync((*usbsw).edev, cable, attached); cables &= !(1u64 << cable); } mask &= !(1u16 << dev); } }
unsafe fn fsa9480_detect_dev(usbsw: *mut fsa9480_usbsw) { let val1 = fsa9480_read_reg(usbsw, FSA9480_REG_DEV_T1 as i32); let val2 = fsa9480_read_reg(usbsw, FSA9480_REG_DEV_T2 as i32); if val1 < 0 || val2 < 0 { dev_err((*usbsw).dev, b"fsa9480_detect_dev: failed to read registers\0".as_ptr()); return; } let val = ((val2 << 8) | val1) as u16; dev_info((*usbsw).dev, b"dev1: 0x%x, dev2: 0x%x\n\0".as_ptr(), val1, val2); fsa9480_handle_change(usbsw, (*usbsw).cable & !val, false); fsa9480_handle_change(usbsw, val & !(*usbsw).cable, true); (*usbsw).cable = val; }
unsafe extern "C" fn fsa9480_irq_handler(_irq: i32, data: *mut core::ffi::c_void) -> i32 { let usbsw = data as *mut fsa9480_usbsw; let mut intr = 0; fsa9480_read_irq(usbsw, &mut intr); if intr == 0 { return 0; } fsa9480_detect_dev(usbsw); 1 }

// Probe, PM, driver registration, and module metadata retain their C interfaces.
// CONFIG_PM_SLEEP conditional code is intentionally preserved for the kernel build.
unsafe extern "C" fn fsa9480_probe(client: *mut i2c_client) -> i32 {
    if (*client).irq == 0 { dev_err(&mut (*client).dev, b"no interrupt provided\n\0".as_ptr()); return -22; }
    let info = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<fsa9480_usbsw>(), 0);
    if info.is_null() { return -12; }
    (*info).dev = &mut (*client).dev;
    i2c_set_clientdata(client, info);
    (*info).edev = devm_extcon_dev_allocate((*info).dev, FSA9480_EXTCON_CABLE.as_ptr());
    if (*info).edev.is_null() { dev_err((*info).dev, b"failed to allocate memory for extcon\n\0".as_ptr()); return -12; }
    let mut ret = devm_extcon_dev_register((*info).dev, (*info).edev); if ret != 0 { return ret; }
    (*info).regmap = devm_regmap_init_i2c(client, &FSA9480_REGMAP_CONFIG);
    if (*info).regmap.is_null() { return -12; }
    fsa9480_write_reg(info, FSA9480_REG_TIMING1 as i32, TIMING1_ADC_500MS as i32);
    fsa9480_write_reg(info, FSA9480_REG_CTRL as i32, CON_MASK as i32);
    fsa9480_write_reg(info, FSA9480_REG_INT1_MASK as i32, (INT1_MASK & !(INT_ATTACH | INT_DETACH)) as i32);
    fsa9480_write_reg(info, FSA9480_REG_INT2_MASK as i32, INT2_MASK as i32);
    ret = devm_request_threaded_irq((*info).dev, (*client).irq, None, Some(fsa9480_irq_handler), 0, b"fsa9480\0".as_ptr(), info);
    if ret != 0 { return ret; }
    devm_device_init_wakeup((*info).dev); fsa9480_detect_dev(info); 0
}

unsafe extern "C" fn fsa9480_suspend(dev: *mut device) -> i32 { let client = to_i2c_client(dev); if device_may_wakeup(&mut (*client).dev) && (*client).irq != 0 { enable_irq_wake((*client).irq); } 0 }
unsafe extern "C" fn fsa9480_resume(dev: *mut device) -> i32 { let client = to_i2c_client(dev); if device_may_wakeup(&mut (*client).dev) && (*client).irq != 0 { disable_irq_wake((*client).irq); } 0 }

unsafe extern "C" fn fsa9480_module_init() -> i32 { i2c_add_driver(core::ptr::null_mut()) }
unsafe extern "C" fn fsa9480_module_exit() { i2c_del_driver(core::ptr::null_mut()); }

// MODULE_DEVICE_TABLE(i2c, fsa9480_id);
// MODULE_DEVICE_TABLE(of, fsa9480_of_match);
// subsys_initcall(fsa9480_module_init);
// module_exit(fsa9480_module_exit);
// MODULE_DESCRIPTION("Fairchild Semiconductor FSA9480 extcon driver");
// MODULE_AUTHOR("Tomasz Figa <tomasz.figa@gmail.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
