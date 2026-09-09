// SPDX-License-Identifier: GPL-2.0-only
/*
 * NBUS driver for TS-4600 based boards
 *
 * Copyright (c) 2016 - Savoir-faire Linux
 * Author: Sebastien Bourdelin <sebastien.bourdelin@savoirfairelinux.com>
 *
 * This driver implements a GPIOs bit-banged bus, called the NBUS by Technologic
 * Systems. It is used to communicate with the peripherals in the FPGA on the
 * TS-4600 SoM.
 */

const TS_NBUS_DIRECTION_IN: i32 = 0;
const TS_NBUS_DIRECTION_OUT: i32 = 1;
const TS_NBUS_WRITE_ADR: i32 = 0;
const TS_NBUS_WRITE_VAL: i32 = 1;

#[repr(C)]
pub struct pwm_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_descs {
    pub desc: *mut *mut gpio_desc,
    pub info: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pwm_state {
    pub period: u64,
    pub duty_cycle: u64,
    pub enabled: bool,
}

#[repr(C)]
pub struct ts_nbus {
    pwm: *mut pwm_device,
    data: *mut gpio_descs,
    csn: *mut gpio_desc,
    txrx: *mut gpio_desc,
    strobe: *mut gpio_desc,
    ale: *mut gpio_desc,
    rdy: *mut gpio_desc,
    lock: mutex,
}

extern "C" {
    fn devm_gpiod_get_array(dev: *mut device, con_id: *const u8, flags: u32) -> *mut gpio_descs;
    fn devm_gpiod_get(dev: *mut device, con_id: *const u8, flags: u32) -> *mut gpio_desc;
    fn devm_gpiod_get_array_error(descs: *mut gpio_descs) -> i32;
    fn devm_gpiod_get_error(desc: *mut gpio_desc) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8, ... ) -> i32;
    fn gpiod_direction_input(desc: *mut gpio_desc) -> i32;
    fn gpiod_direction_output(desc: *mut gpio_desc, value: i32) -> i32;
    fn gpiod_set_array_value_cansleep(count: u32, desc: *mut *mut gpio_desc, info: *mut core::ffi::c_void, values: *const usize);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: i32);
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> i32;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn devm_pwm_get(dev: *mut device, con_id: *const u8) -> *mut pwm_device;
    fn pwm_init_state(pwm: *mut pwm_device, state: *mut pwm_state);
    fn pwm_apply_might_sleep(pwm: *mut pwm_device, state: *mut pwm_state) -> i32;
    fn pwm_disable(pwm: *mut pwm_device);
    fn dev_set_drvdata(dev: *mut device, data: *mut ts_nbus);
    fn dev_get_drvdata(dev: *mut device) -> *mut ts_nbus;
    fn of_platform_populate(node: *mut core::ffi::c_void, matches: *const core::ffi::c_void, lookup: *const core::ffi::c_void, parent: *mut device) -> i32;
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
}

/* request all gpios required by the bus. */
unsafe fn ts_nbus_init_pdata(pdev: *mut platform_device, ts_nbus: *mut ts_nbus) -> i32 {
    let dev = pdev as *mut device;
    (*ts_nbus).data = devm_gpiod_get_array(dev, b"ts,data\0".as_ptr(), 0);
    if (*ts_nbus).data.is_null() { return dev_err_probe(dev, -1, b"failed to retrieve ts,data-gpio from dts\n\0".as_ptr()); }
    (*ts_nbus).csn = devm_gpiod_get(dev, b"ts,csn\0".as_ptr(), 0);
    if (*ts_nbus).csn.is_null() { return dev_err_probe(dev, -1, b"failed to retrieve ts,csn-gpio from dts\n\0".as_ptr()); }
    (*ts_nbus).txrx = devm_gpiod_get(dev, b"ts,txrx\0".as_ptr(), 0);
    if (*ts_nbus).txrx.is_null() { return dev_err_probe(dev, -1, b"failed to retrieve ts,txrx-gpio from dts\n\0".as_ptr()); }
    (*ts_nbus).strobe = devm_gpiod_get(dev, b"ts,strobe\0".as_ptr(), 0);
    if (*ts_nbus).strobe.is_null() { return dev_err_probe(dev, -1, b"failed to retrieve ts,strobe-gpio from dts\n\0".as_ptr()); }
    (*ts_nbus).ale = devm_gpiod_get(dev, b"ts,ale\0".as_ptr(), 0);
    if (*ts_nbus).ale.is_null() { return dev_err_probe(dev, -1, b"failed to retrieve ts,ale-gpio from dts\n\0".as_ptr()); }
    (*ts_nbus).rdy = devm_gpiod_get(dev, b"ts,rdy\0".as_ptr(), 0);
    if (*ts_nbus).rdy.is_null() { return dev_err_probe(dev, -1, b"failed to retrieve ts,rdy-gpio from dts\n\0".as_ptr()); }
    0
}

unsafe fn ts_nbus_set_direction(bus: *mut ts_nbus, direction: i32) {
    for i in 0..8 { let d = *(*(*bus).data).desc.add(i); if direction == TS_NBUS_DIRECTION_IN { gpiod_direction_input(d); } else { gpiod_direction_output(d, 1); } }
}
unsafe fn ts_nbus_reset_bus(bus: *mut ts_nbus) {
    let values = [0usize; 1];
    gpiod_set_array_value_cansleep(8, (*(*bus).data).desc, (*(*bus).data).info, values.as_ptr());
    gpiod_set_value_cansleep((*bus).csn, 0); gpiod_set_value_cansleep((*bus).strobe, 0); gpiod_set_value_cansleep((*bus).ale, 0);
}
unsafe fn ts_nbus_start_transaction(bus: *mut ts_nbus) { gpiod_set_value_cansleep((*bus).strobe, 1); }
unsafe fn ts_nbus_read_byte(bus: *mut ts_nbus, val: *mut u8) -> i32 {
    *val = 0; for i in 0..8 { let ret = gpiod_get_value_cansleep(*(*(*bus).data).desc.add(i)); if ret < 0 { return ret; } if ret != 0 { *val |= 1u8 << i; } } 0
}
unsafe fn ts_nbus_write_byte(bus: *mut ts_nbus, byte: u8) { let values = [byte as usize]; gpiod_set_array_value_cansleep(8, (*(*bus).data).desc, (*(*bus).data).info, values.as_ptr()); }
unsafe fn ts_nbus_read_bus(bus: *mut ts_nbus, val: *mut u8) -> i32 { ts_nbus_reset_bus(bus); ts_nbus_start_transaction(bus); ts_nbus_read_byte(bus, val) }
unsafe fn ts_nbus_write_bus(bus: *mut ts_nbus, cmd: i32, val: u8) { ts_nbus_reset_bus(bus); if cmd == TS_NBUS_WRITE_ADR { gpiod_set_value_cansleep((*bus).ale, 1); } ts_nbus_write_byte(bus, val); ts_nbus_start_transaction(bus); }

pub unsafe fn ts_nbus_read(bus: *mut ts_nbus, adr: u8, val: *mut u16) -> i32 {
    let mut ret; let mut byte = 0u8; mutex_lock(&mut (*bus).lock); gpiod_set_value_cansleep((*bus).txrx, 0); ts_nbus_write_bus(bus, TS_NBUS_WRITE_ADR, adr); ts_nbus_set_direction(bus, TS_NBUS_DIRECTION_IN);
    loop { *val = 0; byte = 0; for i in (0..=1).rev() { ret = ts_nbus_read_bus(bus, &mut byte); if ret < 0 { ts_nbus_set_direction(bus, TS_NBUS_DIRECTION_OUT); mutex_unlock(&mut (*bus).lock); return ret; } *val |= (byte as u16) << (i * 8); } gpiod_set_value_cansleep((*bus).csn, 1); ret = gpiod_get_value_cansleep((*bus).rdy); if ret == 0 { ts_nbus_set_direction(bus, TS_NBUS_DIRECTION_OUT); mutex_unlock(&mut (*bus).lock); return ret; } }
}

pub unsafe fn ts_nbus_write(bus: *mut ts_nbus, adr: u8, val: u16) -> i32 {
    mutex_lock(&mut (*bus).lock); gpiod_set_value_cansleep((*bus).txrx, 1); ts_nbus_write_bus(bus, TS_NBUS_WRITE_ADR, adr); for i in (0..=1).rev() { ts_nbus_write_bus(bus, TS_NBUS_WRITE_VAL, (val >> (i * 8)) as u8); }
    gpiod_set_value_cansleep((*bus).csn, 1); while gpiod_get_value_cansleep((*bus).rdy) != 0 { gpiod_set_value_cansleep((*bus).csn, 0); gpiod_set_value_cansleep((*bus).csn, 1); } mutex_unlock(&mut (*bus).lock); 0
}

unsafe fn ts_nbus_probe(pdev: *mut platform_device) -> i32 {
    let dev = pdev as *mut device;
    let bus = devm_kzalloc(dev, core::mem::size_of::<ts_nbus>(), 0);
    if bus.is_null() { return -12; }
    mutex_init(&mut (*bus).lock);
    let mut ret = ts_nbus_init_pdata(pdev, bus);
    if ret < 0 { return ret; }
    (*bus).pwm = devm_pwm_get(dev, core::ptr::null());
    if (*bus).pwm.is_null() { return dev_err_probe(dev, -1, b"unable to request PWM\n\0".as_ptr()); }
    let mut state = core::mem::zeroed::<pwm_state>();
    pwm_init_state((*bus).pwm, &mut state);
    if state.period == 0 { return dev_err_probe(dev, -22, b"invalid PWM period\n\0".as_ptr()); }
    state.duty_cycle = state.period; state.enabled = true;
    ret = pwm_apply_might_sleep((*bus).pwm, &mut state);
    if ret < 0 { return dev_err_probe(dev, ret, b"failed to configure PWM\n\0".as_ptr()); }
    dev_set_drvdata(dev, bus);
    ret = of_platform_populate(core::ptr::null_mut(), core::ptr::null(), core::ptr::null(), dev);
    if ret < 0 { return dev_err_probe(dev, ret, b"failed to populate platform devices on bus\n\0".as_ptr()); }
    dev_info(dev, b"initialized\n\0".as_ptr());
    0
}

unsafe fn ts_nbus_remove(pdev: *mut platform_device) {
    let bus = dev_get_drvdata(pdev as *mut device);
    mutex_lock(&mut (*bus).lock); pwm_disable((*bus).pwm); mutex_unlock(&mut (*bus).lock);
}

#[repr(C)]
struct of_device_id { compatible: *const u8 }
#[repr(C)]
struct platform_driver { probe: unsafe fn(*mut platform_device) -> i32, remove: unsafe fn(*mut platform_device), name: *const u8, of_match_table: *const of_device_id }

static TS_NBUS_OF_MATCH: &[of_device_id] = &[of_device_id { compatible: b"technologic,ts-nbus\0".as_ptr() }, of_device_id { compatible: core::ptr::null() }];
static TS_NBUS_DRIVER: platform_driver = platform_driver { probe: ts_nbus_probe, remove: ts_nbus_remove, name: b"ts_nbus\0".as_ptr(), of_match_table: TS_NBUS_OF_MATCH.as_ptr() };

extern "C" { fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut ts_nbus; }

// Equivalent kernel registration and exported module metadata:
// module_platform_driver(ts_nbus_driver);
// MODULE_ALIAS("platform:ts_nbus");
// MODULE_AUTHOR("Sebastien Bourdelin <sebastien.bourdelin@savoirfairelinux.com>");
// MODULE_DESCRIPTION("Technologic Systems NBUS");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
