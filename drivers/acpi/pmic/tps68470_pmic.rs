// SPDX-License-Identifier: GPL-2.0
/*
 * TI TPS68470 PMIC operation region driver
 *
 * Copyright (C) 2017 Intel Corporation. All rights reserved.
 *
 * Author: Rajmohan Mani <rajmohan.mani@intel.com>
 *
 * Based on drivers/acpi/pmic/intel_pmic* drivers
 */

// Dependencies supplied by the surrounding kernel Rust bindings.

#[repr(C)]
struct Tps68470PmicTable {
    address: u32,
    reg: u32,
    bitmask: u32,
}

const TI_PMIC_POWER_OPREGION_ID: u32 = 0xB0;
const TI_PMIC_VR_VAL_OPREGION_ID: u32 = 0xB1;
const TI_PMIC_CLOCK_OPREGION_ID: u32 = 0xB2;
const TI_PMIC_CLKFREQ_OPREGION_ID: u32 = 0xB3;

#[repr(C)]
struct Tps68470PmicOpregion {
    lock: Mutex,
    regmap: *mut Regmap,
}

const S_IO_I2C_EN: u32 = (1 << 0) | (1 << 1);

static POWER_TABLE: [Tps68470PmicTable; 6] = [
    Tps68470PmicTable { address: 0x00, reg: TPS68470_REG_S_I2C_CTL, bitmask: S_IO_I2C_EN },
    Tps68470PmicTable { address: 0x04, reg: TPS68470_REG_VCMCTL, bitmask: 1 << 0 },
    Tps68470PmicTable { address: 0x08, reg: TPS68470_REG_VAUX1CTL, bitmask: 1 << 0 },
    Tps68470PmicTable { address: 0x0C, reg: TPS68470_REG_VAUX2CTL, bitmask: 1 << 0 },
    Tps68470PmicTable { address: 0x10, reg: TPS68470_REG_VACTL, bitmask: 1 << 0 },
    Tps68470PmicTable { address: 0x14, reg: TPS68470_REG_VDCTL, bitmask: 1 << 0 },
];

/* Table to set voltage regulator value */
static VR_VAL_TABLE: [Tps68470PmicTable; 7] = [
    Tps68470PmicTable { address: 0x00, reg: TPS68470_REG_VSIOVAL, bitmask: TPS68470_VSIOVAL_IOVOLT_MASK },
    Tps68470PmicTable { address: 0x04, reg: TPS68470_REG_VIOVAL, bitmask: TPS68470_VIOVAL_IOVOLT_MASK },
    Tps68470PmicTable { address: 0x08, reg: TPS68470_REG_VCMVAL, bitmask: TPS68470_VCMVAL_VCVOLT_MASK },
    Tps68470PmicTable { address: 0x0C, reg: TPS68470_REG_VAUX1VAL, bitmask: TPS68470_VAUX1VAL_AUX1VOLT_MASK },
    Tps68470PmicTable { address: 0x10, reg: TPS68470_REG_VAUX2VAL, bitmask: TPS68470_VAUX2VAL_AUX2VOLT_MASK },
    Tps68470PmicTable { address: 0x14, reg: TPS68470_REG_VAVAL, bitmask: TPS68470_VAVAL_AVOLT_MASK },
    Tps68470PmicTable { address: 0x18, reg: TPS68470_REG_VDVAL, bitmask: TPS68470_VDVAL_DVOLT_MASK },
];

/* Table to configure clock frequency */
static CLK_FREQ_TABLE: [Tps68470PmicTable; 7] = [
    Tps68470PmicTable { address: 0x00, reg: TPS68470_REG_POSTDIV2, bitmask: (1 << 0) | (1 << 1) },
    Tps68470PmicTable { address: 0x04, reg: TPS68470_REG_BOOSTDIV, bitmask: 0x1F },
    Tps68470PmicTable { address: 0x08, reg: TPS68470_REG_BUCKDIV, bitmask: 0x0F },
    Tps68470PmicTable { address: 0x0C, reg: TPS68470_REG_PLLSWR, bitmask: 0x13 },
    Tps68470PmicTable { address: 0x10, reg: TPS68470_REG_XTALDIV, bitmask: 0xFF },
    Tps68470PmicTable { address: 0x14, reg: TPS68470_REG_PLLDIV, bitmask: 0xFF },
    Tps68470PmicTable { address: 0x18, reg: TPS68470_REG_POSTDIV, bitmask: 0x83 },
];

/* Table to configure and enable clocks */
static CLK_TABLE: [Tps68470PmicTable; 4] = [
    Tps68470PmicTable { address: 0x00, reg: TPS68470_REG_PLLCTL, bitmask: 0xF5 },
    Tps68470PmicTable { address: 0x04, reg: TPS68470_REG_PLLCTL2, bitmask: 1 << 0 },
    Tps68470PmicTable { address: 0x08, reg: TPS68470_REG_CLKCFG1, bitmask: TPS68470_CLKCFG1_MODE_A_MASK | TPS68470_CLKCFG1_MODE_B_MASK },
    Tps68470PmicTable { address: 0x0C, reg: TPS68470_REG_CLKCFG2, bitmask: TPS68470_CLKCFG1_MODE_A_MASK | TPS68470_CLKCFG1_MODE_B_MASK },
];

unsafe fn pmic_get_reg_bit(address: u64, table: *const Tps68470PmicTable,
                           table_size: usize, reg: *mut i32, bitmask: *mut i32) -> i32 {
    let i = address / 4;
    if i >= table_size as u64 { return -ENOENT; }
    if reg.is_null() || bitmask.is_null() { return -EINVAL; }
    *reg = (*table.add(i as usize)).reg as i32;
    *bitmask = (*table.add(i as usize)).bitmask as i32;
    0
}

unsafe fn tps68470_pmic_get_power(regmap: *mut Regmap, reg: i32, bitmask: i32, value: *mut u64) -> i32 {
    let mut data = 0u32;
    if regmap_read(regmap, reg, &mut data) != 0 { return -EIO; }
    *value = if (data & bitmask as u32) != 0 { 1 } else { 0 };
    0
}

unsafe fn tps68470_pmic_get_vr_val(regmap: *mut Regmap, reg: i32, bitmask: i32, value: *mut u64) -> i32 {
    let mut data = 0u32;
    if regmap_read(regmap, reg, &mut data) != 0 { return -EIO; }
    *value = (data & bitmask as u32) as u64;
    0
}

unsafe fn tps68470_pmic_get_clk(regmap: *mut Regmap, reg: i32, bitmask: i32, value: *mut u64) -> i32 {
    tps68470_pmic_get_power(regmap, reg, bitmask, value)
}

unsafe fn tps68470_pmic_get_clk_freq(regmap: *mut Regmap, reg: i32, bitmask: i32, value: *mut u64) -> i32 {
    tps68470_pmic_get_vr_val(regmap, reg, bitmask, value)
}

unsafe fn ti_tps68470_regmap_update_bits(regmap: *mut Regmap, reg: i32, bitmask: i32, value: u64) -> i32 {
    regmap_update_bits(regmap, reg, bitmask, value as u32)
}

unsafe fn tps68470_pmic_common_handler(
    function: u32, address: u64, bits: u32, value: *mut u64, region_context: *mut core::ffi::c_void,
    get: unsafe fn(*mut Regmap, i32, i32, *mut u64) -> i32,
    update: unsafe fn(*mut Regmap, i32, i32, u64) -> i32,
    tbl: *const Tps68470PmicTable, tbl_size: usize) -> AcpiStatus {
    let opregion = region_context as *mut Tps68470PmicOpregion;
    let regmap = (*opregion).regmap;
    let mut reg = 0i32;
    let mut bitmask = 0i32;
    if bits != 32 { return AE_BAD_PARAMETER; }
    if pmic_get_reg_bit(address, tbl, tbl_size, &mut reg, &mut bitmask) < 0 { return AE_BAD_PARAMETER; }
    if function == ACPI_WRITE && *value > bitmask as u64 { return AE_BAD_PARAMETER; }
    mutex_lock(&mut (*opregion).lock);
    let ret = if function == ACPI_READ { get(regmap, reg, bitmask, value) } else { update(regmap, reg, bitmask, *value) };
    mutex_unlock(&mut (*opregion).lock);
    if ret != 0 { AE_ERROR } else { AE_OK }
}

unsafe fn tps68470_pmic_cfreq_handler(function: u32, address: u64, bits: u32, value: *mut u64, _handler_context: *mut core::ffi::c_void, region_context: *mut core::ffi::c_void) -> AcpiStatus {
    tps68470_pmic_common_handler(function, address, bits, value, region_context, tps68470_pmic_get_clk_freq, ti_tps68470_regmap_update_bits, CLK_FREQ_TABLE.as_ptr(), CLK_FREQ_TABLE.len())
}

unsafe fn tps68470_pmic_clk_handler(function: u32, address: u64, bits: u32, value: *mut u64, _handler_context: *mut core::ffi::c_void, region_context: *mut core::ffi::c_void) -> AcpiStatus {
    tps68470_pmic_common_handler(function, address, bits, value, region_context, tps68470_pmic_get_clk, ti_tps68470_regmap_update_bits, CLK_TABLE.as_ptr(), CLK_TABLE.len())
}

unsafe fn tps68470_pmic_vrval_handler(function: u32, address: u64, bits: u32, value: *mut u64, _handler_context: *mut core::ffi::c_void, region_context: *mut core::ffi::c_void) -> AcpiStatus {
    tps68470_pmic_common_handler(function, address, bits, value, region_context, tps68470_pmic_get_vr_val, ti_tps68470_regmap_update_bits, VR_VAL_TABLE.as_ptr(), VR_VAL_TABLE.len())
}

unsafe fn tps68470_pmic_pwr_handler(function: u32, address: u64, bits: u32, value: *mut u64, _handler_context: *mut core::ffi::c_void, region_context: *mut core::ffi::c_void) -> AcpiStatus {
    if bits != 32 { return AE_BAD_PARAMETER; }
    if function == ACPI_WRITE && *value != 0 && *value != 1 && *value != 3 { return AE_BAD_PARAMETER; }
    tps68470_pmic_common_handler(function, address, bits, value, region_context, tps68470_pmic_get_power, ti_tps68470_regmap_update_bits, POWER_TABLE.as_ptr(), POWER_TABLE.len())
}

unsafe fn tps68470_pmic_opregion_probe(pdev: *mut PlatformDevice) -> i32 {
    let tps68470_regmap = dev_get_drvdata((*pdev).dev.parent);
    let handle = acpi_handle((*pdev).dev.parent);
    let dev = &mut (*pdev).dev;
    if tps68470_regmap.is_null() { return dev_err_probe(dev, -EINVAL, "regmap is missing\0".as_ptr()); }
    if handle.is_null() { dev_warn(dev, "acpi handle is NULL\0".as_ptr()); return -ENODEV; }
    let opregion = devm_kzalloc(dev, core::mem::size_of::<Tps68470PmicOpregion>(), GFP_KERNEL) as *mut Tps68470PmicOpregion;
    if opregion.is_null() { return -ENOMEM; }
    mutex_init(&mut (*opregion).lock);
    (*opregion).regmap = tps68470_regmap;
    let status = acpi_install_address_space_handler(handle, TI_PMIC_POWER_OPREGION_ID, tps68470_pmic_pwr_handler, core::ptr::null_mut(), opregion as *mut _);
    if ACPI_FAILURE(status) { mutex_destroy(&mut (*opregion).lock); return -ENODEV; }
    let status = acpi_install_address_space_handler(handle, TI_PMIC_VR_VAL_OPREGION_ID, tps68470_pmic_vrval_handler, core::ptr::null_mut(), opregion as *mut _);
    if ACPI_FAILURE(status) { acpi_remove_address_space_handler(handle, TI_PMIC_POWER_OPREGION_ID, tps68470_pmic_pwr_handler); mutex_destroy(&mut (*opregion).lock); return -ENODEV; }
    let status = acpi_install_address_space_handler(handle, TI_PMIC_CLOCK_OPREGION_ID, tps68470_pmic_clk_handler, core::ptr::null_mut(), opregion as *mut _);
    if ACPI_FAILURE(status) { acpi_remove_address_space_handler(handle, TI_PMIC_VR_VAL_OPREGION_ID, tps68470_pmic_vrval_handler); acpi_remove_address_space_handler(handle, TI_PMIC_POWER_OPREGION_ID, tps68470_pmic_pwr_handler); mutex_destroy(&mut (*opregion).lock); return -ENODEV; }
    let status = acpi_install_address_space_handler(handle, TI_PMIC_CLKFREQ_OPREGION_ID, tps68470_pmic_cfreq_handler, core::ptr::null_mut(), opregion as *mut _);
    if ACPI_FAILURE(status) { acpi_remove_address_space_handler(handle, TI_PMIC_CLOCK_OPREGION_ID, tps68470_pmic_clk_handler); acpi_remove_address_space_handler(handle, TI_PMIC_VR_VAL_OPREGION_ID, tps68470_pmic_vrval_handler); acpi_remove_address_space_handler(handle, TI_PMIC_POWER_OPREGION_ID, tps68470_pmic_pwr_handler); mutex_destroy(&mut (*opregion).lock); return -ENODEV; }
    0
}

static mut Tps68470PmicOpregionDriver: PlatformDriver = PlatformDriver {
    probe: Some(tps68470_pmic_opregion_probe),
    driver: Driver { name: "tps68470_pmic_opregion\0".as_ptr() },
};

// Equivalent of builtin_platform_driver(tps68470_pmic_opregion_driver).
builtin_platform_driver!(Tps68470PmicOpregionDriver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
