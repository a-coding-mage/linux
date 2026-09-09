// SPDX-License-Identifier: GPL-2.0
/*
 * XPower AXP288 PMIC operation region driver
 *
 * Copyright (C) 2014 Intel Corporation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

const XPOWER_GPADC_LOW: i32 = 0x5b;
const XPOWER_GPI1_CTRL: i32 = 0x92;

const GPI1_LDO_MASK: i32 = 0x7;
const GPI1_LDO_ON: i32 = 3 << 0;
const GPI1_LDO_OFF: i32 = 4 << 0;

const AXP288_ADC_TS_CURRENT_ON_OFF_MASK: i32 = 0x3;
const AXP288_ADC_TS_CURRENT_OFF: i32 = 0 << 0;
const AXP288_ADC_TS_CURRENT_ON_WHEN_CHARGING: i32 = 1 << 0;
const AXP288_ADC_TS_CURRENT_ON_ONDEMAND: i32 = 2 << 0;
const AXP288_ADC_TS_CURRENT_ON: i32 = 3 << 0;

#[repr(C)]
struct PmicTable {
    address: u32,
    reg: u32,
    bit: u32,
}

static POWER_TABLE: &[PmicTable] = &[
    PmicTable { address: 0x00, reg: 0x13, bit: 0x05 }, // ALD1
    PmicTable { address: 0x04, reg: 0x13, bit: 0x06 }, // ALD2
    PmicTable { address: 0x08, reg: 0x13, bit: 0x07 }, // ALD3
    PmicTable { address: 0x0c, reg: 0x12, bit: 0x03 }, // DLD1
    PmicTable { address: 0x10, reg: 0x12, bit: 0x04 }, // DLD2
    PmicTable { address: 0x14, reg: 0x12, bit: 0x05 }, // DLD3
    PmicTable { address: 0x18, reg: 0x12, bit: 0x06 }, // DLD4
    PmicTable { address: 0x1c, reg: 0x12, bit: 0x00 }, // ELD1
    PmicTable { address: 0x20, reg: 0x12, bit: 0x01 }, // ELD2
    PmicTable { address: 0x24, reg: 0x12, bit: 0x02 }, // ELD3
    PmicTable { address: 0x28, reg: 0x13, bit: 0x02 }, // FLD1
    PmicTable { address: 0x2c, reg: 0x13, bit: 0x03 }, // FLD2
    PmicTable { address: 0x30, reg: 0x13, bit: 0x04 }, // FLD3
    PmicTable { address: 0x34, reg: 0x10, bit: 0x03 }, // BUC1
    PmicTable { address: 0x38, reg: 0x10, bit: 0x06 }, // BUC2
    PmicTable { address: 0x3c, reg: 0x10, bit: 0x05 }, // BUC3
    PmicTable { address: 0x40, reg: 0x10, bit: 0x04 }, // BUC4
    PmicTable { address: 0x44, reg: 0x10, bit: 0x01 }, // BUC5
    PmicTable { address: 0x48, reg: 0x10, bit: 0x00 }, // BUC6
    PmicTable { address: 0x4c, reg: 0x92, bit: 0 }, // GPI1
];

// TMP0 - TMP5 are the same, all from GPADC
static THERMAL_TABLE: &[PmicTable] = &[
    PmicTable { address: 0x00, reg: XPOWER_GPADC_LOW as u32, bit: 0 },
    PmicTable { address: 0x0c, reg: XPOWER_GPADC_LOW as u32, bit: 0 },
    PmicTable { address: 0x18, reg: XPOWER_GPADC_LOW as u32, bit: 0 },
    PmicTable { address: 0x24, reg: XPOWER_GPADC_LOW as u32, bit: 0 },
    PmicTable { address: 0x30, reg: XPOWER_GPADC_LOW as u32, bit: 0 },
    PmicTable { address: 0x3c, reg: XPOWER_GPADC_LOW as u32, bit: 0 },
];

unsafe fn intel_xpower_pmic_get_power(regmap: *mut Regmap, reg: i32, bit: i32, value: *mut u64) -> i32 {
    let mut data = 0i32;
    if regmap_read(regmap, reg, &mut data) != 0 { return -5; }
    if reg == XPOWER_GPI1_CTRL {
        *value = (((data & GPI1_LDO_MASK) == GPI1_LDO_ON) as u64);
    } else {
        *value = (((data & (1 << bit)) != 0) as u64);
    }
    0
}

unsafe fn intel_xpower_pmic_update_power(regmap: *mut Regmap, reg: i32, bit: i32, on: bool) -> i32 {
    let mut data = 0i32;
    let mut ret = iosf_mbi_block_punit_i2c_access();
    if ret != 0 { return ret; }
    if reg == XPOWER_GPI1_CTRL {
        ret = regmap_update_bits(regmap, reg, GPI1_LDO_MASK, if on { GPI1_LDO_ON } else { GPI1_LDO_OFF });
    } else {
        if regmap_read(regmap, reg, &mut data) != 0 { ret = -5; }
        else {
            if on { data |= 1 << bit; } else { data &= !(1 << bit); }
            if regmap_write(regmap, reg, data) != 0 { ret = -5; }
        }
    }
    iosf_mbi_unblock_punit_i2c_access();
    ret
}

unsafe fn intel_xpower_pmic_get_raw_temp(regmap: *mut Regmap, reg: i32) -> i32 {
    let mut adc_ts_pin_ctrl = 0i32;
    let mut buf = [0u8; 2];
    let mut ret = regmap_read(regmap, AXP288_ADC_TS_PIN_CTRL, &mut adc_ts_pin_ctrl);
    if ret != 0 { return ret; }
    if adc_ts_pin_ctrl & AXP288_ADC_TS_CURRENT_ON_OFF_MASK != 0 {
        ret = regmap_update_bits(regmap, AXP288_ADC_TS_PIN_CTRL, AXP288_ADC_TS_CURRENT_ON_OFF_MASK, AXP288_ADC_TS_CURRENT_ON_ONDEMAND);
        if ret != 0 { return ret; }
        usleep_range(6000, 10000);
    }
    ret = iosf_mbi_block_punit_i2c_access();
    if ret != 0 { return ret; }
    ret = regmap_bulk_read(regmap, AXP288_GP_ADC_H, buf.as_mut_ptr(), 2);
    if ret == 0 { ret = ((buf[0] as i32) << 4) + (((buf[1] >> 4) & 0x0f) as i32); }
    if adc_ts_pin_ctrl & AXP288_ADC_TS_CURRENT_ON_OFF_MASK != 0 {
        regmap_update_bits(regmap, AXP288_ADC_TS_PIN_CTRL, AXP288_ADC_TS_CURRENT_ON_OFF_MASK, AXP288_ADC_TS_CURRENT_ON);
    }
    iosf_mbi_unblock_punit_i2c_access();
    ret
}

unsafe fn intel_xpower_exec_mipi_pmic_seq_element(regmap: *mut Regmap, i2c_address: u16, reg_address: u32, value: u32, mask: u32) -> i32 {
    if i2c_address != 0x34 { return -6; }
    let ret = iosf_mbi_block_punit_i2c_access();
    if ret != 0 { return ret; }
    let ret = regmap_update_bits(regmap, reg_address as i32, mask as i32, value as i32);
    iosf_mbi_unblock_punit_i2c_access();
    ret
}

unsafe fn intel_xpower_lpat_raw_to_temp(lpat_table: *mut AcpiLpatConversionTable, mut raw: i32) -> i32 {
    let first = (*lpat_table).lpat[0];
    let last = (*lpat_table).lpat[(*lpat_table).lpat_count - 1];
    if first.raw < last.raw { raw = raw.clamp(first.raw, last.raw); }
    else { raw = raw.clamp(last.raw, first.raw); }
    acpi_lpat_raw_to_temp(lpat_table, raw)
}

// External kernel types and functions are supplied by other translated units.
#[repr(C)] struct Regmap;
#[repr(C)] struct AcpiLpat { raw: i32 }
#[repr(C)] struct AcpiLpatConversionTable { lpat: *mut AcpiLpat, lpat_count: usize }
extern "C" {
    static AXP288_ADC_TS_PIN_CTRL: i32;
    static AXP288_GP_ADC_H: i32;
    fn regmap_read(*mut Regmap, i32, *mut i32) -> i32;
    fn regmap_update_bits(*mut Regmap, i32, i32, i32) -> i32;
    fn regmap_write(*mut Regmap, i32, i32) -> i32;
    fn regmap_bulk_read(*mut Regmap, i32, *mut u8, usize) -> i32;
    fn iosf_mbi_block_punit_i2c_access() -> i32;
    fn iosf_mbi_unblock_punit_i2c_access();
    fn usleep_range(u32, u32);
    fn acpi_lpat_raw_to_temp(*mut AcpiLpatConversionTable, i32) -> i32;
}

unsafe fn intel_xpower_pmic_gpio_handler(
    _function: u32,
    _address: u64,
    _bit_width: u32,
    _value: *mut u64,
    _handler_context: *mut core::ffi::c_void,
    _region_context: *mut core::ffi::c_void,
) -> i32 {
    0 // AE_OK
}

#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Device { parent: *mut Device }
#[repr(C)] struct Axp20xDev { regmap: *mut Regmap }
#[repr(C)]
struct IntelPmicOpregionData {
    get_power: unsafe fn(*mut Regmap, i32, i32, *mut u64) -> i32,
    update_power: unsafe fn(*mut Regmap, i32, i32, bool) -> i32,
    get_raw_temp: unsafe fn(*mut Regmap, i32) -> i32,
    exec_mipi_pmic_seq_element: unsafe fn(*mut Regmap, u16, u32, u32, u32) -> i32,
    lpat_raw_to_temp: unsafe fn(*mut AcpiLpatConversionTable, i32) -> i32,
    power_table: *const PmicTable,
    power_table_count: usize,
    thermal_table: *const PmicTable,
    thermal_table_count: usize,
    pmic_i2c_address: u16,
}

static INTEL_XPOWER_PMIC_OPREGION_DATA: IntelPmicOpregionData = IntelPmicOpregionData {
    get_power: intel_xpower_pmic_get_power,
    update_power: intel_xpower_pmic_update_power,
    get_raw_temp: intel_xpower_pmic_get_raw_temp,
    exec_mipi_pmic_seq_element: intel_xpower_exec_mipi_pmic_seq_element,
    lpat_raw_to_temp: intel_xpower_lpat_raw_to_temp,
    power_table: POWER_TABLE.as_ptr(),
    power_table_count: POWER_TABLE.len(),
    thermal_table: THERMAL_TABLE.as_ptr(),
    thermal_table_count: THERMAL_TABLE.len(),
    pmic_i2c_address: 0x34,
};

extern "C" {
    fn dev_get_drvdata(*mut Device) -> *mut Axp20xDev;
    fn acpi_install_address_space_handler(*mut core::ffi::c_void, u32, unsafe fn(u32, u64, u32, *mut u64, *mut core::ffi::c_void, *mut core::ffi::c_void) -> i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> i32;
    fn acpi_remove_address_space_handler(*mut core::ffi::c_void, u32, unsafe fn(u32, u64, u32, *mut u64, *mut core::ffi::c_void, *mut core::ffi::c_void) -> i32);
    fn acpi_handle(*mut Device) -> *mut core::ffi::c_void;
    fn intel_pmic_install_opregion_handler(*mut Device, *mut core::ffi::c_void, *mut Regmap, *const IntelPmicOpregionData) -> i32;
}

unsafe fn intel_xpower_pmic_opregion_probe(pdev: *mut PlatformDevice) -> i32 {
    let parent = (*pdev).dev.parent;
    let axp20x = dev_get_drvdata(parent);
    let status = acpi_install_address_space_handler(
        acpi_handle(parent), 0x08, intel_xpower_pmic_gpio_handler, core::ptr::null_mut(), core::ptr::null_mut());
    if status != 0 { return -19; }
    let result = intel_pmic_install_opregion_handler(
        &mut (*pdev).dev, acpi_handle(parent), (*axp20x).regmap, &INTEL_XPOWER_PMIC_OPREGION_DATA);
    if result != 0 {
        acpi_remove_address_space_handler(acpi_handle(parent), 0x08, intel_xpower_pmic_gpio_handler);
    }
    result
}

// The operation-region data and platform-driver registration are consumed by
// the surrounding kernel bindings.
#[repr(C)] struct PlatformDriver;
extern "C" {
    static mut INTEL_XPOWER_PMIC_OPREGION_DRIVER: PlatformDriver;
    fn builtin_platform_driver(*mut PlatformDriver);
}

#[no_mangle]
unsafe extern "C" fn intel_xpower_pmic_opregion_driver_init() {
    builtin_platform_driver(&mut INTEL_XPOWER_PMIC_OPREGION_DRIVER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
