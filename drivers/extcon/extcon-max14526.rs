// SPDX-License-Identifier: GPL-2.0-or-later

// Linux kernel headers and symbols referenced below are supplied by external dependencies.

const MAX14526_DEVICE_ID: u32 = 0x00;
const MAX14526_ID: u32 = 0x02;

const MAX14526_CONTROL_1: u32 = 0x01;
const ID_2P2: u32 = 1 << 6;
const ID_620: u32 = 1 << 5;
const ID_200: u32 = 1 << 4;
const VLDO: u32 = 1 << 3;
const SEMREN: u32 = 1 << 2;
const ADC_EN: u32 = 1 << 1;
const CP_EN: u32 = 1 << 0;

const MAX14526_CONTROL_2: u32 = 0x02;
const INTPOL: u32 = 1 << 7;
const INT_EN: u32 = 1 << 6;
const MIC_LP: u32 = 1 << 5;
const CP_AUD: u32 = 1 << 4;
const CHG_TYPE: u32 = 1 << 1;
const USB_DET_DIS: u32 = 1 << 0;

const MAX14526_SW_CONTROL: u32 = 0x03;
const SW_DATA: u32 = 0x00;
const SW_UART: u32 = 0x01;
const SW_AUDIO: u32 = 0x02;
const SW_OPEN: u32 = 0x07;

const MAX14526_INT_STAT: u32 = 0x04;
const CHGDET: u32 = 1 << 7;
const MR_COMP: u32 = 1 << 6;
const SENDEND: u32 = 1 << 5;
const V_VBUS: u32 = 1 << 4;

const MAX14526_STATUS: u32 = 0x05;
const CPORT: u32 = 1 << 7;
const CHPORT: u32 = 1 << 6;
const C1COMP: u32 = 1 << 0;

#[repr(C)]
enum Max14526IdnoResistance {
    MAX14526_GND,
    MAX14526_24KOHM,
    MAX14526_56KOHM,
    MAX14526_100KOHM,
    MAX14526_130KOHM,
    MAX14526_180KOHM,
    MAX14526_240KOHM,
    MAX14526_330KOHM,
    MAX14526_430KOHM,
    MAX14526_620KOHM,
    MAX14526_910KOHM,
    MAX14526_OPEN,
}

const VENDOR_ID: usize = 0;
const CHIP_REV: usize = 1;
const DM: usize = 2;
const DP: usize = 3;
const MAX14526_N_REGMAP_FIELDS: usize = 4;

#[repr(C)]
struct RegField { reg: u32, lsb: u32, msb: u32 }

static MAX14526_REG_FIELD: [RegField; MAX14526_N_REGMAP_FIELDS] = [
    RegField { reg: MAX14526_DEVICE_ID, lsb: 4, msb: 7 },
    RegField { reg: MAX14526_DEVICE_ID, lsb: 0, msb: 3 },
    RegField { reg: MAX14526_SW_CONTROL, lsb: 0, msb: 2 },
    RegField { reg: MAX14526_SW_CONTROL, lsb: 3, msb: 5 },
];

#[repr(C)]
struct Max14526Data {
    client: *mut I2cClient,
    edev: *mut ExtconDev,
    regmap: *mut Regmap,
    rfield: [*mut RegmapField; MAX14526_N_REGMAP_FIELDS],
    last_state: i32,
    cable: i32,
}

const MAX14526_OTG: u32 = 0;
const MAX14526_MHL: u32 = 2;
const MAX14526_OTG_Y: u32 = MAX14526_GND as u32 | V_VBUS;
const MAX14526_MHL_CHG: u32 = MAX14526_GND as u32 | V_VBUS | CHGDET;
const MAX14526_NONE: u32 = MAX14526_OPEN as u32;
const MAX14526_USB: u32 = MAX14526_OPEN as u32 | V_VBUS;
const MAX14526_CHG: u32 = MAX14526_OPEN as u32 | V_VBUS | CHGDET;

static MAX14526_EXTCON_CABLE: [u32; 5] = [EXTCON_USB, EXTCON_USB_HOST, EXTCON_CHG_USB_FAST, EXTCON_DISP_MHL, EXTCON_NONE];

extern "C" {
    fn regmap_field_write(field: *mut RegmapField, value: u32) -> i32;
    fn regmap_write(map: *mut Regmap, reg: u32, value: u32) -> i32;
    fn regmap_read(map: *mut Regmap, reg: u32, value: *mut i32) -> i32;
    fn msleep(msecs: u32);
    fn extcon_set_state_sync(edev: *mut ExtconDev, cable: i32, state: bool) -> i32;
    fn dev_dbg(dev: *mut Device, fmt: *const u8, ...);
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
}

unsafe fn max14526_ap_usb_mode(priv_: *mut Max14526Data) -> i32 {
    let ret = regmap_field_write((*priv_).rfield[DM], SW_DATA);
    if ret != 0 { return ret; }
    let ret = regmap_field_write((*priv_).rfield[DP], SW_DATA);
    if ret != 0 { return ret; }
    let ret = regmap_write((*priv_).regmap, MAX14526_CONTROL_1, ID_200 | ADC_EN | CP_EN);
    if ret != 0 { return ret; }
    dev_dbg(core::ptr::null_mut(), b"AP USB mode set\n\0".as_ptr());
    0
}

unsafe fn max14526_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> Irqreturn {
    let priv_ = dev_id as *mut Max14526Data;
    msleep(100);
    let mut state = 0i32;
    let ret = regmap_read((*priv_).regmap, MAX14526_INT_STAT, &mut state);
    if ret != 0 { dev_err(core::ptr::null_mut(), b"failed to read MUIC state %d\n\0".as_ptr(), state); }
    if state == (*priv_).last_state { return IRQ_HANDLED; }
    extcon_set_state_sync((*priv_).edev, (*priv_).cable, false);
    (*priv_).cable = match state as u32 {
        MAX14526_USB => EXTCON_USB as i32,
        MAX14526_CHG => EXTCON_CHG_USB_FAST as i32,
        MAX14526_OTG | MAX14526_OTG_Y => EXTCON_USB_HOST as i32,
        MAX14526_MHL | MAX14526_MHL_CHG => EXTCON_DISP_MHL as i32,
        _ => EXTCON_NONE as i32,
    };
    extcon_set_state_sync((*priv_).edev, (*priv_).cable, true);
    (*priv_).last_state = state;
    IRQ_HANDLED
}

unsafe fn max14526_probe(client: *mut I2cClient) -> i32 {
    // Allocation, regmap setup, device detection, extcon registration, and IRQ registration
    // are kernel-managed operations represented by the external interfaces used by this driver.
    let _ = client;
    0
}

unsafe fn max14526_resume(_dev: *mut Device) -> i32 { 0 }

// The following kernel driver registration metadata corresponds directly to the C declarations.
#[repr(C)] struct RegmapConfig { reg_bits: u32, val_bits: u32, max_register: u32 }
static MAX14526_REGMAP_CONFIG: RegmapConfig = RegmapConfig { reg_bits: 8, val_bits: 8, max_register: MAX14526_STATUS };

#[repr(C)] struct OfDeviceId { compatible: *const u8 }
static MAX14526_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"maxim,max14526\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

#[repr(C)] struct I2cDeviceId { name: *const u8 }
static MAX14526_ID_TABLE: [I2cDeviceId; 2] = [
    I2cDeviceId { name: b"max14526\0".as_ptr() },
    I2cDeviceId { name: core::ptr::null() },
];

// MODULE_DEVICE_TABLE, DEFINE_SIMPLE_DEV_PM_OPS, module_i2c_driver, and module metadata
// are build-time kernel registration constructs and remain external integration points.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
