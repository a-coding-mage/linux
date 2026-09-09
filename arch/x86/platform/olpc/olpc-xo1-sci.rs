// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for OLPC XO-1 System Control Interrupts (SCI)
 *
 * Copyright (C) 2010 One Laptop per Child
 * Copyright (C) 2006 Red Hat, Inc.
 * Copyright (C) 2006 Advanced Micro Devices, Inc.
 */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel/Rust integration.

const DRV_NAME: &[u8] = b"olpc-xo1-sci\0";
const PFX: &[u8] = b"olpc-xo1-sci: \0";

static mut acpi_base: c_ulong = 0;
static mut power_button_idev: *mut input_dev = core::ptr::null_mut();
static mut ebook_switch_idev: *mut input_dev = core::ptr::null_mut();
static mut lid_switch_idev: *mut input_dev = core::ptr::null_mut();
static mut sci_irq: c_int = 0;
static mut lid_open: bool = false;
static mut lid_inverted: bool = false;
static mut lid_wake_mode: c_int = 0;

#[repr(C)]
pub struct input_dev { pub sw: *mut c_ulong, pub swbit: *mut c_ulong, pub evbit: *mut c_ulong, pub keybit: *mut c_ulong, pub name: *const c_char, pub phys: *const c_char, pub dev: device }
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct resource { pub start: c_ulong }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct device_attribute;
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
pub type c_int = i32; pub type c_uint = u32; pub type c_ulong = usize;
pub type c_ushort = u16; pub type c_char = i8; pub type ssize_t = isize;
pub type pm_message_t = c_uint; pub type irqreturn_t = c_int;

const LID_WAKE_ALWAYS: c_int = 0;
const LID_WAKE_OPEN: c_int = 1;
const LID_WAKE_CLOSE: c_int = 2;
static lid_wake_mode_names: [&[u8]; 3] = [b"always", b"open", b"close"];

extern "C" {
    fn power_supply_get_by_name(name: *const c_char) -> *mut c_void;
    fn power_supply_changed(psy: *mut c_void); fn power_supply_put(psy: *mut c_void);
    fn olpc_ec_cmd(cmd: c_int, in_buf: *const c_void, in_len: c_int, out_buf: *mut u8, out_len: c_int) -> c_int;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> c_int;
    fn input_report_switch(dev: *mut input_dev, code: c_int, value: c_int);
    fn input_sync(dev: *mut input_dev); fn input_report_key(dev: *mut input_dev, code: c_int, value: c_int);
    fn pm_wakeup_event(dev: *mut device, msec: c_uint);
    fn cs5535_gpio_clear(gpio: c_int, mask: c_int); fn cs5535_gpio_set(gpio: c_int, mask: c_int);
    fn cs5535_gpio_isset(gpio: c_int, mask: c_int) -> c_int;
    fn olpc_ec_sci_query(data: *mut c_ushort) -> c_int;
    fn schedule_work(work: *mut work_struct) -> c_int; fn cancel_work_sync(work: *mut work_struct) -> c_int;
    fn inl(port: c_ulong) -> c_uint; fn outl(value: c_uint, port: c_ulong);
    fn inb(port: c_ulong) -> u8; fn outb(value: u8, port: c_ulong);
    fn bus_find_device_by_name(bus: *mut c_void, start: *mut c_void, name: *const c_char) -> *mut device;
    fn put_device(dev: *mut device); fn device_may_wakeup(dev: *mut device) -> bool;
    fn olpc_xo1_pm_wakeup_set(x: c_int); fn olpc_xo1_pm_wakeup_clear(x: c_int);
    fn olpc_ec_wakeup_set(x: c_int); fn olpc_ec_wakeup_clear(x: c_int);
    fn rdmsrq(msr: c_uint, value: *mut u64); fn wrmsrq(msr: c_uint, value: u64);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void); fn gpio_request(gpio: c_int, name: *const c_char) -> c_int;
    fn gpio_direction_input(gpio: c_int); fn gpio_free(gpio: c_int);
    fn cs5535_gpio_setup_event(gpio: c_int, group: c_int, enable: c_int);
    fn cs5535_pic_unreqz_select_high(group: c_int, irq: c_int); fn cs5535_gpio_set_irq(group: c_int, irq: c_int);
    fn olpc_ec_mask_write(mask: c_int); fn input_allocate_device() -> *mut input_dev;
    fn input_register_device(dev: *mut input_dev) -> c_int; fn input_free_device(dev: *mut input_dev);
    fn input_unregister_device(dev: *mut input_dev); fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn device_init_wakeup(dev: *mut device, enable: c_int); fn device_set_wakeup_capable(dev: *mut device, enable: bool);
    fn platform_get_resource(dev: *mut platform_device, typ: c_uint, num: c_uint) -> *mut resource;
    fn machine_is_olpc() -> bool; fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}
use core::ffi::{c_void, c_char};

unsafe fn battery_status_changed() { let psy = power_supply_get_by_name(b"olpc_battery\0".as_ptr() as *const c_char); if !psy.is_null() { power_supply_changed(psy); power_supply_put(psy); } }
unsafe fn ac_status_changed() { let psy = power_supply_get_by_name(b"olpc_ac\0".as_ptr() as *const c_char); if !psy.is_null() { power_supply_changed(psy); power_supply_put(psy); } }

/* Report current ebook switch state through input layer */
unsafe fn send_ebook_state() { let mut state = 0u8; if olpc_ec_cmd(EC_READ_EB_MODE, core::ptr::null(), 0, &mut state, 1) != 0 { return; } if test_bit(SW_TABLET_MODE, (*ebook_switch_idev).sw) == (state != 0) as c_int { return; } input_report_switch(ebook_switch_idev, SW_TABLET_MODE, state as c_int); input_sync(ebook_switch_idev); pm_wakeup_event(&mut (*ebook_switch_idev).dev, 0); }
unsafe fn flip_lid_inverter() { if lid_inverted { cs5535_gpio_clear(OLPC_GPIO_LID, GPIO_INPUT_INVERT); } else { cs5535_gpio_set(OLPC_GPIO_LID, GPIO_INPUT_INVERT); } lid_inverted = !lid_inverted; }
unsafe fn detect_lid_state() { let state = cs5535_gpio_isset(OLPC_GPIO_LID, GPIO_READ_BACK); lid_open = (!state != 0) ^ (!lid_inverted); if state == 0 { return; } flip_lid_inverter(); }
/* Report current lid switch state through input layer */
unsafe fn send_lid_state() { if (test_bit(SW_LID, (*lid_switch_idev).sw) != 0) == !lid_open { return; } input_report_switch(lid_switch_idev, SW_LID, (!lid_open) as c_int); input_sync(lid_switch_idev); pm_wakeup_event(&mut (*lid_switch_idev).dev, 0); }

unsafe fn process_sci_queue(propagate_events: bool) { let mut r; let mut data = 0u16; loop { r = olpc_ec_sci_query(&mut data); if r != 0 || data == 0 { break; } match data as c_int { EC_SCI_SRC_BATERR | EC_SCI_SRC_BATSOC | EC_SCI_SRC_BATTERY | EC_SCI_SRC_BATCRIT => battery_status_changed(), EC_SCI_SRC_ACPWR => ac_status_changed(), _ => {} } if data as c_int == EC_SCI_SRC_EBOOK && propagate_events { send_ebook_state(); } } }
unsafe extern "C" fn process_sci_queue_work(_work: *mut work_struct) { process_sci_queue(true); }
static mut sci_work: work_struct = work_struct { };

unsafe extern "C" fn xo1_sci_intr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t { let pdev = dev_id as *mut platform_device; let sts = inl(acpi_base + CS5536_PM1_STS); outl(sts | 0xffff, acpi_base + CS5536_PM1_STS); let gpe = inl(acpi_base + CS5536_PM_GPE0_STS); outl(0xffff_ffff, acpi_base + CS5536_PM_GPE0_STS); if sts & CS5536_PWRBTN_FLAG != 0 { if sts & CS5536_WAK_FLAG == 0 { input_report_key(power_button_idev, KEY_POWER, 1); input_sync(power_button_idev); input_report_key(power_button_idev, KEY_POWER, 0); input_sync(power_button_idev); } pm_wakeup_event(&mut (*power_button_idev).dev, 0); } if sts & (CS5536_RTC_FLAG | CS5536_WAK_FLAG) == (CS5536_RTC_FLAG | CS5536_WAK_FLAG) { let rtc = bus_find_device_by_name(core::ptr::null_mut(), core::ptr::null_mut(), b"rtc_cmos\0".as_ptr() as *const c_char); if !rtc.is_null() { pm_wakeup_event(rtc, 0); put_device(rtc); } } if gpe & CS5536_GPIOM7_PME_FLAG != 0 { cs5535_gpio_set(OLPC_GPIO_ECSCI, GPIO_NEGATIVE_EDGE_STS); schedule_work(&mut sci_work); } cs5535_gpio_set(OLPC_GPIO_LID, GPIO_NEGATIVE_EDGE_STS); cs5535_gpio_set(OLPC_GPIO_LID, GPIO_POSITIVE_EDGE_STS); detect_lid_state(); send_lid_state(); IRQ_HANDLED }

// The remaining platform setup/teardown entry points retain the C driver's
// externally supplied constants and kernel helper calls.
unsafe fn xo1_sci_suspend(_pdev: *mut platform_device, _state: pm_message_t) -> c_int { if device_may_wakeup(&mut (*power_button_idev).dev) { olpc_xo1_pm_wakeup_set(CS5536_PM_PWRBTN); } else { olpc_xo1_pm_wakeup_clear(CS5536_PM_PWRBTN); } if device_may_wakeup(&mut (*ebook_switch_idev).dev) { olpc_ec_wakeup_set(EC_SCI_SRC_EBOOK); } else { olpc_ec_wakeup_clear(EC_SCI_SRC_EBOOK); } if !device_may_wakeup(&mut (*lid_switch_idev).dev) { cs5535_gpio_clear(OLPC_GPIO_LID, GPIO_EVENTS_ENABLE); } else if (lid_open && lid_wake_mode == LID_WAKE_OPEN) || (!lid_open && lid_wake_mode == LID_WAKE_CLOSE) { flip_lid_inverter(); cs5535_gpio_set(OLPC_GPIO_LID, GPIO_NEGATIVE_EDGE_STS); cs5535_gpio_set(OLPC_GPIO_LID, GPIO_POSITIVE_EDGE_STS); cs5535_gpio_set(OLPC_GPIO_LID, GPIO_EVENTS_ENABLE); } 0 }
unsafe fn xo1_sci_resume(_pdev: *mut platform_device) -> c_int { detect_lid_state(); send_lid_state(); cs5535_gpio_set(OLPC_GPIO_LID, GPIO_EVENTS_ENABLE); olpc_ec_mask_write(EC_SCI_SRC_ALL); battery_status_changed(); ac_status_changed(); 0 }

unsafe fn setup_sci_interrupt(pdev: *mut platform_device) -> c_int { let mut msr=0u64; rdmsrq(0x51400020,&mut msr); sci_irq=((msr>>20)&15) as c_int; if sci_irq==0 { sci_irq=3; msr|=0x00300000; wrmsrq(0x51400020,msr); } let mut lo; if sci_irq<8 { lo=inb(CS5536_PIC_INT_SEL1); outb(lo | (1<<sci_irq),CS5536_PIC_INT_SEL1); } else { lo=inb(CS5536_PIC_INT_SEL2); outb(lo | (1<<(sci_irq-8)),CS5536_PIC_INT_SEL2); } let sts=inl(acpi_base+CS5536_PM1_STS); outl(((CS5536_PM_PWRBTN|CS5536_PM_RTC)<<16)|0xffff,acpi_base+CS5536_PM1_STS); request_irq(sci_irq,xo1_sci_intr,0,DRV_NAME.as_ptr() as *const c_char,pdev as *mut c_void) }
unsafe fn setup_ec_sci() -> c_int { let r=gpio_request(OLPC_GPIO_ECSCI,b"OLPC-ECSCI\0".as_ptr() as *const c_char); if r!=0{return r;} gpio_direction_input(OLPC_GPIO_ECSCI); cs5535_gpio_set(OLPC_GPIO_ECSCI,GPIO_NEGATIVE_EDGE_STS); cs5535_gpio_set(OLPC_GPIO_ECSCI,GPIO_POSITIVE_EDGE_STS); cs5535_gpio_set(OLPC_GPIO_ECSCI,GPIO_EVENTS_ENABLE); cs5535_gpio_setup_event(OLPC_GPIO_ECSCI,7,1); cs5535_pic_unreqz_select_high(7,sci_irq); 0 }
unsafe fn free_ec_sci(){gpio_free(OLPC_GPIO_ECSCI)}
unsafe fn setup_lid_events()->c_int { let r=gpio_request(OLPC_GPIO_LID,b"OLPC-LID\0".as_ptr() as *const c_char); if r!=0{return r;} gpio_direction_input(OLPC_GPIO_LID); cs5535_gpio_clear(OLPC_GPIO_LID,GPIO_INPUT_INVERT); lid_inverted=false; cs5535_gpio_clear(OLPC_GPIO_LID,GPIO_EVENTS_ENABLE|GPIO_NEGATIVE_EDGE_EN|GPIO_POSITIVE_EDGE_EN); cs5535_gpio_set(OLPC_GPIO_LID,GPIO_NEGATIVE_EDGE_STS); cs5535_gpio_set(OLPC_GPIO_LID,GPIO_POSITIVE_EDGE_STS); cs5535_gpio_setup_event(OLPC_GPIO_LID,6,1); cs5535_gpio_set_irq(6,sci_irq); cs5535_gpio_set(OLPC_GPIO_LID,GPIO_EVENTS_ENABLE); 0 }
unsafe fn free_lid_events(){gpio_free(OLPC_GPIO_LID)}
unsafe fn setup_power_button(pdev:*mut platform_device)->c_int { power_button_idev=input_allocate_device(); if power_button_idev.is_null(){return -ENOMEM;} (*power_button_idev).name=b"Power Button\0".as_ptr() as *const c_char; (*power_button_idev).phys=b"olpc-xo1-sci/input0\0".as_ptr() as *const c_char; set_bit(EV_KEY,(*power_button_idev).evbit); set_bit(KEY_POWER,(*power_button_idev).keybit); (*power_button_idev).dev.parent=&mut (*pdev).dev; device_init_wakeup(&mut (*power_button_idev).dev,1); let r=input_register_device(power_button_idev); if r!=0{input_free_device(power_button_idev);} r }
unsafe fn free_power_button(){input_unregister_device(power_button_idev)}
unsafe fn setup_ebook_switch(pdev:*mut platform_device)->c_int { ebook_switch_idev=input_allocate_device(); if ebook_switch_idev.is_null(){return -ENOMEM;} (*ebook_switch_idev).name=b"EBook Switch\0".as_ptr() as *const c_char; (*ebook_switch_idev).phys=b"olpc-xo1-sci/input1\0".as_ptr() as *const c_char; set_bit(EV_SW,(*ebook_switch_idev).evbit); set_bit(SW_TABLET_MODE,(*ebook_switch_idev).swbit); (*ebook_switch_idev).dev.parent=&mut (*pdev).dev; device_set_wakeup_capable(&mut (*ebook_switch_idev).dev,true); let r=input_register_device(ebook_switch_idev); if r!=0{input_free_device(ebook_switch_idev);} r }
unsafe fn free_ebook_switch(){input_unregister_device(ebook_switch_idev)}
unsafe fn setup_lid_switch(pdev:*mut platform_device)->c_int { lid_switch_idev=input_allocate_device(); if lid_switch_idev.is_null(){return -ENOMEM;} (*lid_switch_idev).name=b"Lid Switch\0".as_ptr() as *const c_char; (*lid_switch_idev).phys=b"olpc-xo1-sci/input2\0".as_ptr() as *const c_char; set_bit(EV_SW,(*lid_switch_idev).evbit); set_bit(SW_LID,(*lid_switch_idev).swbit); (*lid_switch_idev).dev.parent=&mut (*pdev).dev; device_set_wakeup_capable(&mut (*lid_switch_idev).dev,true); let r=input_register_device(lid_switch_idev); if r!=0{input_free_device(lid_switch_idev);} r }
unsafe fn free_lid_switch(){input_unregister_device(lid_switch_idev)}
unsafe fn xo1_sci_remove(pdev:*mut platform_device){free_irq(sci_irq,pdev as *mut c_void);cancel_work_sync(&mut sci_work);free_ec_sci();free_lid_events();free_lid_switch();free_ebook_switch();free_power_button();acpi_base=0;}
#[allow(dead_code)] unsafe fn xo1_sci_init()->c_int{platform_driver_register(core::ptr::null_mut())}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
