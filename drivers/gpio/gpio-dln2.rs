// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the Diolan DLN-2 USB-GPIO adapter
 *
 * Copyright (c) 2014 Intel Corporation
 */

// Linux kernel dependencies are supplied by the surrounding repository.

const DLN2_GPIO_ID: u16 = 0x01;
const DLN2_GPIO_GET_PIN_COUNT: u16 = dln2_cmd(0x01, DLN2_GPIO_ID);
const DLN2_GPIO_SET_DEBOUNCE: u16 = dln2_cmd(0x04, DLN2_GPIO_ID);
const DLN2_GPIO_GET_DEBOUNCE: u16 = dln2_cmd(0x05, DLN2_GPIO_ID);
const DLN2_GPIO_PORT_GET_VAL: u16 = dln2_cmd(0x06, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_GET_VAL: u16 = dln2_cmd(0x0b, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_SET_OUT_VAL: u16 = dln2_cmd(0x0c, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_GET_OUT_VAL: u16 = dln2_cmd(0x0d, DLN2_GPIO_ID);
const DLN2_GPIO_CONDITION_MET_EV: u16 = dln2_cmd(0x0f, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_ENABLE: u16 = dln2_cmd(0x10, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_DISABLE: u16 = dln2_cmd(0x11, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_SET_DIRECTION: u16 = dln2_cmd(0x13, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_GET_DIRECTION: u16 = dln2_cmd(0x14, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_SET_EVENT_CFG: u16 = dln2_cmd(0x1e, DLN2_GPIO_ID);
const DLN2_GPIO_PIN_GET_EVENT_CFG: u16 = dln2_cmd(0x1f, DLN2_GPIO_ID);

const DLN2_GPIO_EVENT_NONE: u8 = 0;
const DLN2_GPIO_EVENT_CHANGE: u8 = 1;
const DLN2_GPIO_EVENT_LVL_HIGH: u8 = 2;
const DLN2_GPIO_EVENT_LVL_LOW: u8 = 3;
const DLN2_GPIO_EVENT_CHANGE_RISING: u8 = 0x11;
const DLN2_GPIO_EVENT_CHANGE_FALLING: u8 = 0x21;
const DLN2_GPIO_EVENT_MASK: u8 = 0x0f;
const DLN2_GPIO_MAX_PINS: usize = 32;

#[repr(C)]
pub struct Dln2Gpio {
    pub pdev: *mut PlatformDevice,
    pub gpio: GpioChip,
    // Cache pin direction to save us one transfer, since the hardware has
    // separate commands to read the in and out values.
    pub output_enabled: [usize; 1],
    // active IRQs - not synced to hardware
    pub unmasked_irqs: [usize; 1],
    // active IRQS - synced to hardware
    pub enabled_irqs: [usize; 1],
    pub irq_type: [i32; DLN2_GPIO_MAX_PINS],
    pub irq_lock: Mutex,
}

#[repr(C)]
pub struct Dln2GpioPin { pub pin: u16 }

#[repr(C, packed)]
pub struct Dln2GpioPinVal { pub pin: u16, pub value: u8 }

const DLN2_GPIO_DIRECTION_IN: u8 = 0;
const DLN2_GPIO_DIRECTION_OUT: u8 = 1;

unsafe fn dln2_gpio_get_pin_count(pdev: *mut PlatformDevice) -> i32 {
    let mut count: u16 = 0;
    let mut len = core::mem::size_of::<u16>() as i32;
    let ret = dln2_transfer_rx(pdev, DLN2_GPIO_GET_PIN_COUNT, &mut count as *mut _ as *mut _, &mut len);
    if ret < 0 { return ret; }
    if len < core::mem::size_of::<u16>() as i32 { return -EPROTO; }
    u16::from_le(count) as i32
}

unsafe fn dln2_gpio_pin_cmd(dln2: *mut Dln2Gpio, cmd: i32, pin: u32) -> i32 {
    let req = Dln2GpioPin { pin: (pin as u16).to_le() };
    dln2_transfer_tx((*dln2).pdev, cmd as u16, &req as *const _ as *const _, core::mem::size_of_val(&req) as i32)
}

unsafe fn dln2_gpio_pin_val(dln2: *mut Dln2Gpio, cmd: i32, pin: u32) -> i32 {
    let req = Dln2GpioPin { pin: (pin as u16).to_le() };
    let mut rsp = Dln2GpioPinVal { pin: 0, value: 0 };
    let mut len = core::mem::size_of::<Dln2GpioPinVal>() as i32;
    let ret = dln2_transfer(dln2_read_pdev(dln2), cmd as u16, &req as *const _ as *const _, core::mem::size_of_val(&req) as i32, &mut rsp as *mut _ as *mut _, &mut len);
    if ret < 0 { return ret; }
    if len < core::mem::size_of::<Dln2GpioPinVal>() as i32 || req.pin != rsp.pin { return -EPROTO; }
    rsp.value as i32
}

unsafe fn dln2_gpio_pin_get_in_val(dln2: *mut Dln2Gpio, pin: u32) -> i32 {
    let ret = dln2_gpio_pin_val(dln2, DLN2_GPIO_PIN_GET_VAL as i32, pin);
    if ret < 0 { ret } else { (ret != 0) as i32 }
}
unsafe fn dln2_gpio_pin_get_out_val(dln2: *mut Dln2Gpio, pin: u32) -> i32 {
    let ret = dln2_gpio_pin_val(dln2, DLN2_GPIO_PIN_GET_OUT_VAL as i32, pin);
    if ret < 0 { ret } else { (ret != 0) as i32 }
}
unsafe fn dln2_gpio_pin_set_out_val(dln2: *mut Dln2Gpio, pin: u32, value: i32) -> i32 {
    let req = Dln2GpioPinVal { pin: (pin as u16).to_le(), value: value as u8 };
    dln2_transfer_tx((*dln2).pdev, DLN2_GPIO_PIN_SET_OUT_VAL, &req as *const _ as *const _, core::mem::size_of_val(&req) as i32)
}

unsafe fn dln2_gpio_set_direction(dln2: *mut Dln2Gpio, chip: *mut GpioChip, offset: u32, dir: u8) -> i32 {
    let req = Dln2GpioPinVal { pin: (offset as u16).to_le(), value: dir };
    let ret = dln2_transfer_tx((*dln2).pdev, DLN2_GPIO_PIN_SET_DIRECTION, &req as *const _ as *const _, core::mem::size_of_val(&req) as i32);
    if ret < 0 { return ret; }
    if dir == DLN2_GPIO_DIRECTION_OUT { set_bit(offset, (*dln2).output_enabled.as_mut_ptr()); }
    else { clear_bit(offset, (*dln2).output_enabled.as_mut_ptr()); }
    let _ = chip;
    ret
}

unsafe fn dln2_gpio_request(dln2: *mut Dln2Gpio, offset: u32) -> i32 {
    let mut rsp = Dln2GpioPinVal { pin: 0, value: 0 };
    let req = Dln2GpioPin { pin: (offset as u16).to_le() };
    let mut len = core::mem::size_of::<Dln2GpioPinVal>() as i32;
    let mut ret = dln2_gpio_pin_cmd(dln2, DLN2_GPIO_PIN_ENABLE as i32, offset);
    if ret < 0 { return ret; }
    ret = dln2_transfer((*dln2).pdev, DLN2_GPIO_PIN_GET_DIRECTION, &req as *const _ as *const _, core::mem::size_of_val(&req) as i32, &mut rsp as *mut _ as *mut _, &mut len);
    if ret < 0 { return ret; }
    if len < core::mem::size_of::<Dln2GpioPinVal>() as i32 || req.pin != rsp.pin { ret = -EPROTO; }
    else if rsp.value == DLN2_GPIO_DIRECTION_IN { clear_bit(offset, (*dln2).output_enabled.as_mut_ptr()); return 0; }
    else if rsp.value == DLN2_GPIO_DIRECTION_OUT { set_bit(offset, (*dln2).output_enabled.as_mut_ptr()); return 0; }
    else { ret = -EPROTO; }
    let _ = dln2_gpio_pin_cmd(dln2, DLN2_GPIO_PIN_DISABLE as i32, offset);
    ret
}

unsafe fn dln2_gpio_free(dln2: *mut Dln2Gpio, offset: u32) { let _ = dln2_gpio_pin_cmd(dln2, DLN2_GPIO_PIN_DISABLE as i32, offset); }
unsafe fn dln2_gpio_get_direction(dln2: *mut Dln2Gpio, offset: u32) -> i32 {
    if test_bit(offset, (*dln2).output_enabled.as_ptr()) { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}
unsafe fn dln2_gpio_get(dln2: *mut Dln2Gpio, offset: u32) -> i32 {
    if dln2_gpio_get_direction(dln2, offset) == GPIO_LINE_DIRECTION_IN { dln2_gpio_pin_get_in_val(dln2, offset) } else { dln2_gpio_pin_get_out_val(dln2, offset) }
}
unsafe fn dln2_gpio_set(dln2: *mut Dln2Gpio, offset: u32, value: i32) -> i32 { dln2_gpio_pin_set_out_val(dln2, offset, value) }
unsafe fn dln2_gpio_direction_input(dln2: *mut Dln2Gpio, offset: u32) -> i32 { dln2_gpio_set_direction(dln2, core::ptr::null_mut(), offset, DLN2_GPIO_DIRECTION_IN) }
unsafe fn dln2_gpio_direction_output(dln2: *mut Dln2Gpio, offset: u32, value: i32) -> i32 {
    let ret = dln2_gpio_pin_set_out_val(dln2, offset, value); if ret < 0 { ret } else { dln2_gpio_set_direction(dln2, core::ptr::null_mut(), offset, DLN2_GPIO_DIRECTION_OUT) }
}

const GPIO_LINE_DIRECTION_IN: i32 = 0;
const GPIO_LINE_DIRECTION_OUT: i32 = 1;

// The remaining GPIO and IRQ callbacks preserve the original kernel callback
// interfaces and are declared here for the surrounding kernel bindings.
extern "C" {
    fn dln2_cmd(cmd: u8, id: u16) -> u16;
    fn dln2_transfer_rx(pdev: *mut PlatformDevice, cmd: u16, data: *mut core::ffi::c_void, len: *mut i32) -> i32;
    fn dln2_transfer(pdev: *mut PlatformDevice, cmd: u16, tx: *const core::ffi::c_void, tx_len: i32, rx: *mut core::ffi::c_void, rx_len: *mut i32) -> i32;
    fn dln2_read_pdev(dln2: *mut Dln2Gpio) -> *mut PlatformDevice;
    fn set_bit(n: u32, addr: *mut usize);
    fn clear_bit(n: u32, addr: *mut usize);
}

type PlatformDevice = core::ffi::c_void;
type GpioChip = core::ffi::c_void;
type Mutex = core::ffi::c_void;
const EPROTO: i32 = 71;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
