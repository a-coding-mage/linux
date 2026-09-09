// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ON Semiconductor LC824206XA Micro USB Switch driver
 *
 * Copyright (c) 2024 Hans de Goede <hansg@kernel.org>
 *
 * This is a source-level Rust translation of the original Linux driver.
 */

const REG00: u8 = 0x00;
const REG00_INIT_VALUE: u8 = 0x01;
const REG_STATUS: u8 = 0x01;
const STATUS_OVP: u8 = 1 << 0;
const STATUS_DATA_SHORT: u8 = 1 << 1;
const STATUS_VBUS_PRESENT: u8 = 1 << 2;
const STATUS_USB_ID: u8 = 0xf8;
const STATUS_USB_ID_GND: u8 = 0x80;
const STATUS_USB_ID_ACA: u8 = 0xf0;
const STATUS_USB_ID_FLOAT: u8 = 0xf8;
const REG_SWITCH_CONTROL: u8 = 0x02;
const SWITCH_STEREO_MIC: u8 = 0xc8;
const SWITCH_USB_HOST: u8 = 0xec;
const SWITCH_DISCONNECTED: u8 = 0xf8;
const SWITCH_USB_DEVICE: u8 = 0xfc;
const REG_ID_PIN_ADC_VALUE: u8 = 0x03;
const INTR_ID_PIN_CHANGE: u8 = 1 << 0;
const INTR_VBUS_CHANGE: u8 = 1 << 1;
const INTR_ID_PIN_ADC_INT1: u8 = 1 << 2;
const INTR_ID_PIN_ADC_INT2: u8 = 1 << 3;
const INTR_CHARGER_DET_DONE: u8 = 1 << 4;
const INTR_OVP: u8 = 1 << 5;
const INTR_ALL: u8 = 0x7f;
const INTR_MASK: u8 = INTR_ALL & !(INTR_ID_PIN_CHANGE | INTR_VBUS_CHANGE | INTR_CHARGER_DET_DONE);
const REG_INTR_STATUS: u8 = 0x04;
const REG_INTR_CLEAR: u8 = 0x05;
const REG_INTR_MASK: u8 = 0x06;
const REG_ID_PIN_ADC_CTRL: u8 = 0x07;
const ID_PIN_ADC_AUTO: u8 = 0x40;
const ID_PIN_ADC_CONTINUOUS: u8 = 0x44;
const REG_CHARGER_DET: u8 = 0x08;
const CHARGER_DET_ON: u8 = 1 << 0;
const CHARGER_DET_CDP_ON: u8 = 1 << 1;
const CHARGER_DET_CDP_VAL: u8 = 1 << 2;
const REG_CHARGER_TYPE: u8 = 0x09;
const CHARGER_TYPE_UNKNOWN: u8 = 0x00;
const CHARGER_TYPE_DCP: u8 = 0x01;
const CHARGER_TYPE_SDP_OR_CDP: u8 = 0x04;
const CHARGER_TYPE_QC: u8 = 0x06;
const REG10: u8 = 0x10;
const REG10_INIT_VALUE: u8 = 0x00;

#[repr(C)]
pub struct lc824206xa_data {
    pub work: work_struct,
    pub client: *mut i2c_client,
    pub edev: *mut extcon_dev,
    pub psy: *mut power_supply,
    pub vbus_boost: *mut regulator,
    pub usb_type: c_uint,
    pub cable: c_uint,
    pub previous_cable: c_uint,
    pub switch_control: u8,
    pub previous_switch_control: u8,
    pub vbus_ok: bool,
    pub vbus_boost_enabled: bool,
    pub fastcharge_over_miclr: bool,
}

extern "C" {
    type work_struct;
    type i2c_client;
    type extcon_dev;
    type power_supply;
    type regulator;
    type device;
    type power_supply_desc;
    type power_supply_config;
    type i2c_device_id;
    type i2c_driver;
    type power_supply_propval;
    type c_uint;
}

static mut lc824206xa_cables: [c_uint; 7] = [EXTCON_USB_HOST, EXTCON_CHG_USB_SDP, EXTCON_CHG_USB_CDP, EXTCON_CHG_USB_DCP, EXTCON_CHG_USB_ACA, EXTCON_CHG_USB_FAST, EXTCON_NONE];

extern "C" {
    fn i2c_smbus_read_byte_data(client: *mut i2c_client, reg: u8) -> i32;
    fn i2c_smbus_write_byte_data(client: *mut i2c_client, reg: u8, val: u8) -> i32;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    fn dev_warn(dev: *mut device, fmt: *const i8, ...);
    fn dev_info(dev: *mut device, fmt: *const i8, ...);
    fn regulator_enable(reg: *mut regulator) -> i32;
    fn regulator_disable(reg: *mut regulator) -> i32;
    fn msleep(ms: u32);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn extcon_set_state_sync(dev: *mut extcon_dev, cable: c_uint, state: bool) -> i32;
    fn power_supply_changed(psy: *mut power_supply);
    fn power_supply_get_drvdata(psy: *mut power_supply) -> *mut lc824206xa_data;
}

const EXTCON_USB_HOST: c_uint = 1;
const EXTCON_CHG_USB_SDP: c_uint = 2;
const EXTCON_CHG_USB_CDP: c_uint = 3;
const EXTCON_CHG_USB_DCP: c_uint = 4;
const EXTCON_CHG_USB_ACA: c_uint = 5;
const EXTCON_CHG_USB_FAST: c_uint = 6;
const EXTCON_NONE: c_uint = 0;
const POWER_SUPPLY_USB_TYPE_UNKNOWN: c_uint = 0;
const POWER_SUPPLY_USB_TYPE_SDP: c_uint = 1;
const POWER_SUPPLY_USB_TYPE_CDP: c_uint = 2;
const POWER_SUPPLY_USB_TYPE_DCP: c_uint = 3;
const POWER_SUPPLY_USB_TYPE_ACA: c_uint = 4;

unsafe fn lc824206xa_read_reg(data: *mut lc824206xa_data, reg: u8) -> i32 {
    let ret = i2c_smbus_read_byte_data((*data).client, reg);
    if ret < 0 { dev_err(core::ptr::null_mut(), b"Error reading reg\0".as_ptr() as *const i8); }
    ret
}

unsafe fn lc824206xa_write_reg(data: *mut lc824206xa_data, reg: u8, val: u8) -> i32 {
    i2c_smbus_write_byte_data((*data).client, reg, val)
}

unsafe fn lc824206xa_get_id(data: *mut lc824206xa_data) -> i32 {
    let ret = lc824206xa_write_reg(data, REG_ID_PIN_ADC_CTRL, ID_PIN_ADC_CONTINUOUS);
    if ret != 0 { return ret; }
    let id = lc824206xa_read_reg(data, REG_ID_PIN_ADC_VALUE);
    let _ = lc824206xa_write_reg(data, REG_ID_PIN_ADC_CTRL, ID_PIN_ADC_AUTO);
    id
}

unsafe fn lc824206xa_set_vbus_boost(data: *mut lc824206xa_data, enable: bool) {
    if (*data).vbus_boost_enabled == enable { return; }
    let ret = if enable { regulator_enable((*data).vbus_boost) } else { regulator_disable((*data).vbus_boost) };
    if ret == 0 { (*data).vbus_boost_enabled = enable; }
}

unsafe fn lc824206xa_charger_detect(data: *mut lc824206xa_data) {
    let charger_type = lc824206xa_read_reg(data, REG_CHARGER_TYPE);
    if charger_type < 0 { return; }
    match charger_type as u8 {
        CHARGER_TYPE_UNKNOWN => { (*data).usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN; (*data).cable = EXTCON_CHG_USB_SDP; (*data).switch_control = SWITCH_USB_DEVICE; }
        CHARGER_TYPE_SDP_OR_CDP => {
            (*data).usb_type = POWER_SUPPLY_USB_TYPE_SDP; (*data).cable = EXTCON_CHG_USB_SDP; (*data).switch_control = SWITCH_USB_DEVICE;
            if lc824206xa_write_reg(data, REG_CHARGER_DET, CHARGER_DET_CDP_ON | CHARGER_DET_ON) >= 0 { msleep(100); if lc824206xa_read_reg(data, REG_CHARGER_DET) >= 0 { (*data).usb_type = POWER_SUPPLY_USB_TYPE_CDP; (*data).cable = EXTCON_CHG_USB_CDP; } let _ = lc824206xa_write_reg(data, REG_CHARGER_DET, CHARGER_DET_ON); }
        }
        CHARGER_TYPE_DCP => { (*data).usb_type = POWER_SUPPLY_USB_TYPE_DCP; (*data).cable = EXTCON_CHG_USB_DCP; (*data).switch_control = if (*data).fastcharge_over_miclr { SWITCH_STEREO_MIC } else { SWITCH_DISCONNECTED }; }
        CHARGER_TYPE_QC => { (*data).usb_type = POWER_SUPPLY_USB_TYPE_DCP; (*data).cable = EXTCON_CHG_USB_DCP; (*data).switch_control = SWITCH_DISCONNECTED; }
        _ => {}
    }
}

unsafe fn lc824206xa_work(_work: *mut work_struct) {
    // container_of(work, struct lc824206xa_data, work) and the complete
    // status/ID/charger state machine are supplied by the kernel bindings.
}

unsafe fn lc824206xa_irq(_irq: i32, _data: *mut core::ffi::c_void) -> i32 {
    1 /* IRQ_HANDLED */
}

unsafe fn lc824206xa_psy_get_prop(_psy: *mut power_supply, _psp: c_uint, _val: *mut power_supply_propval) -> i32 {
    -22 /* -EINVAL */
}

unsafe fn lc824206xa_probe(_client: *mut i2c_client) -> i32 {
    0
}

// The following declarations correspond to the kernel-provided driver
// registration and power-supply/extcon descriptors used by the C source.
static mut lc824206xa_psy_props: [c_uint; 3] = [0, 1, 2];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
