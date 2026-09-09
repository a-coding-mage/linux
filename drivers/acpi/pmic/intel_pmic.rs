// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel_pmic.c - Intel PMIC operation region driver
 *
 * Copyright (C) 2014 Intel Corporation. All rights reserved.
 */

const PMIC_POWER_OPREGION_ID: u32 = 0x8d;
const PMIC_THERMAL_OPREGION_ID: u32 = 0x8c;
const PMIC_REGS_OPREGION_ID: u32 = 0x8f;

#[repr(C)]
struct intel_pmic_regs_handler_ctx {
    val: u32,
    addr: u16,
}

#[repr(C)]
struct intel_pmic_opregion {
    lock: mutex,
    lpat_table: *mut acpi_lpat_conversion_table,
    regmap: *mut regmap,
    data: *const intel_pmic_opregion_data,
    ctx: intel_pmic_regs_handler_ctx,
}

static mut intel_pmic_opregion: *mut intel_pmic_opregion = core::ptr::null_mut();

unsafe fn pmic_get_reg_bit(
    address: i32,
    table: *const pmic_table,
    count: i32,
    reg: *mut i32,
    bit: *mut i32,
) -> i32 {
    let mut i = 0;
    while i < count {
        let entry = &*table.add(i as usize);
        if entry.address == address {
            *reg = entry.reg;
            if !bit.is_null() {
                *bit = entry.bit;
            }
            return 0;
        }
        i += 1;
    }
    -ENOENT
}

unsafe extern "C" fn intel_pmic_power_handler(
    function: u32,
    address: acpi_physical_address,
    bits: u32,
    value64: *mut u64,
    _handler_context: *mut core::ffi::c_void,
    region_context: *mut core::ffi::c_void,
) -> acpi_status {
    let opregion = region_context as *mut intel_pmic_opregion;
    let regmap = (*opregion).regmap;
    let d = &*((*opregion).data);
    let mut reg = 0;
    let mut bit = 0;
    let result;

    if bits != 32 || value64.is_null() { return AE_BAD_PARAMETER; }
    if function == ACPI_WRITE && !(*value64 == 0 || *value64 == 1) { return AE_BAD_PARAMETER; }
    result = pmic_get_reg_bit(address as i32, d.power_table, d.power_table_count, &mut reg, &mut bit);
    if result == -ENOENT { return AE_BAD_PARAMETER; }

    let _guard = (*opregion).lock.lock();
    let result = if function == ACPI_READ {
        (d.get_power)(regmap, reg, bit, value64)
    } else {
        (d.update_power)(regmap, reg, bit, *value64 == 1)
    };
    if result != 0 { AE_ERROR } else { AE_OK }
}

unsafe fn pmic_read_temp(opregion: *mut intel_pmic_opregion, reg: i32, value: *mut u64) -> i32 {
    let d = &*((*opregion).data);
    if d.get_raw_temp.is_none() { return -ENXIO; }
    let raw_temp = d.get_raw_temp.unwrap()((*opregion).regmap, reg);
    if raw_temp < 0 { return raw_temp; }
    if (*opregion).lpat_table.is_null() { *value = raw_temp as u64; return 0; }
    let temp = (d.lpat_raw_to_temp)((*opregion).lpat_table, raw_temp);
    if temp < 0 { return temp; }
    *value = temp as u64;
    0
}

unsafe fn pmic_thermal_temp(opregion: *mut intel_pmic_opregion, reg: i32, function: u32, value: *mut u64) -> i32 {
    if function == ACPI_READ { pmic_read_temp(opregion, reg, value) } else { -EINVAL }
}

unsafe fn pmic_thermal_aux(opregion: *mut intel_pmic_opregion, reg: i32, function: u32, value: *mut u64) -> i32 {
    if function == ACPI_READ { return pmic_read_temp(opregion, reg, value); }
    let d = &*((*opregion).data);
    if d.update_aux.is_none() { return -ENXIO; }
    let raw_temp = if !(*opregion).lpat_table.is_null() {
        let v = acpi_lpat_temp_to_raw((*opregion).lpat_table, *value as i32);
        if v < 0 { return v; }
        v
    } else { *value as i32 };
    d.update_aux.unwrap()((*opregion).regmap, reg, raw_temp)
}

unsafe fn pmic_thermal_pen(opregion: *mut intel_pmic_opregion, reg: i32, bit: i32, function: u32, value: *mut u64) -> i32 {
    let d = &*((*opregion).data);
    if d.get_policy.is_none() || d.update_policy.is_none() { return -ENXIO; }
    if function == ACPI_READ { return d.get_policy.unwrap()((*opregion).regmap, reg, bit, value); }
    if *value != 0 && *value != 1 { return -EINVAL; }
    d.update_policy.unwrap()((*opregion).regmap, reg, bit, *value)
}

fn pmic_thermal_is_temp(address: i32) -> bool { address <= 0x3c && address % 12 == 0 }
fn pmic_thermal_is_aux(address: i32) -> bool {
    (address >= 4 && address <= 0x40 && (address - 4) % 12 == 0) ||
    (address >= 8 && address <= 0x44 && (address - 8) % 12 == 0)
}
fn pmic_thermal_is_pen(address: i32) -> bool { address >= 0x48 && address <= 0x5c }

unsafe extern "C" fn intel_pmic_thermal_handler(function: u32, address: acpi_physical_address, bits: u32, value64: *mut u64, _handler_context: *mut core::ffi::c_void, region_context: *mut core::ffi::c_void) -> acpi_status {
    let opregion = region_context as *mut intel_pmic_opregion;
    let d = &*((*opregion).data);
    let mut reg = 0; let mut bit = 0;
    if bits != 32 || value64.is_null() { return AE_BAD_PARAMETER; }
    let mut result = pmic_get_reg_bit(address as i32, d.thermal_table, d.thermal_table_count, &mut reg, &mut bit);
    if result == -ENOENT { return AE_BAD_PARAMETER; }
    let _guard = (*opregion).lock.lock();
    if pmic_thermal_is_temp(address as i32) { result = pmic_thermal_temp(opregion, reg, function, value64); }
    else if pmic_thermal_is_aux(address as i32) { result = pmic_thermal_aux(opregion, reg, function, value64); }
    else if pmic_thermal_is_pen(address as i32) { result = pmic_thermal_pen(opregion, reg, bit, function, value64); }
    else { result = -EINVAL; }
    if result < 0 { if result == -EINVAL { AE_BAD_PARAMETER } else { AE_ERROR } } else { AE_OK }
}

unsafe extern "C" fn intel_pmic_regs_handler(function: u32, address: acpi_physical_address, _bits: u32, value64: *mut u64, _handler_context: *mut core::ffi::c_void, region_context: *mut core::ffi::c_void) -> acpi_status {
    let opregion = region_context as *mut intel_pmic_opregion;
    let mut result = -EINVAL;
    if function == ACPI_WRITE {
        match address {
            0 => return AE_OK,
            1 => { (*opregion).ctx.addr |= ((*value64 & 0xff) as u16) << 8; return AE_OK; },
            2 => { (*opregion).ctx.addr |= (*value64 & 0xff) as u16; return AE_OK; },
            3 => { (*opregion).ctx.val = *value64 as u32 & 0xff; return AE_OK; },
            4 => { if *value64 != 0 { result = regmap_write((*opregion).regmap, (*opregion).ctx.addr, (*opregion).ctx.val); } else { result = regmap_read((*opregion).regmap, (*opregion).ctx.addr, &mut (*opregion).ctx.val); } (*opregion).ctx.addr = 0; },
            _ => {}
        }
    }
    if function == ACPI_READ && address == 3 { *value64 = (*opregion).ctx.val as u64; return AE_OK; }
    if result < 0 { if result == -EINVAL { AE_BAD_PARAMETER } else { AE_ERROR } } else { AE_OK }
}

pub unsafe fn intel_pmic_install_opregion_handler(dev: *mut device, handle: acpi_handle, regmap: *mut regmap, d: *const intel_pmic_opregion_data) -> i32 {
    let mut status = AE_OK;
    if dev.is_null() || regmap.is_null() || d.is_null() { return -EINVAL; }
    if handle.is_null() { return -ENODEV; }
    let opregion = devm_kzalloc(dev, core::mem::size_of::<intel_pmic_opregion>(), GFP_KERNEL) as *mut intel_pmic_opregion;
    if opregion.is_null() { return -ENOMEM; }
    mutex_init(&mut (*opregion).lock); (*opregion).regmap = regmap; (*opregion).lpat_table = acpi_lpat_get_conversion_table(handle);
    if (*d).power_table_count != 0 { status = acpi_install_address_space_handler(handle, PMIC_POWER_OPREGION_ID, Some(intel_pmic_power_handler), core::ptr::null_mut(), opregion as *mut _); }
    if ACPI_FAILURE(status) { acpi_lpat_free_conversion_table((*opregion).lpat_table); return -ENODEV; }
    if (*d).thermal_table_count != 0 { status = acpi_install_address_space_handler(handle, PMIC_THERMAL_OPREGION_ID, Some(intel_pmic_thermal_handler), core::ptr::null_mut(), opregion as *mut _); }
    if ACPI_FAILURE(status) { if (*d).power_table_count != 0 { acpi_remove_address_space_handler(handle, PMIC_POWER_OPREGION_ID, Some(intel_pmic_power_handler)); } acpi_lpat_free_conversion_table((*opregion).lpat_table); return -ENODEV; }
    status = acpi_install_address_space_handler(handle, PMIC_REGS_OPREGION_ID, Some(intel_pmic_regs_handler), core::ptr::null_mut(), opregion as *mut _);
    if ACPI_FAILURE(status) { if (*d).thermal_table_count != 0 { acpi_remove_address_space_handler(handle, PMIC_THERMAL_OPREGION_ID, Some(intel_pmic_thermal_handler)); } if (*d).power_table_count != 0 { acpi_remove_address_space_handler(handle, PMIC_POWER_OPREGION_ID, Some(intel_pmic_power_handler)); } acpi_lpat_free_conversion_table((*opregion).lpat_table); return -ENODEV; }
    (*opregion).data = d; intel_pmic_opregion = opregion; 0
}

pub unsafe fn intel_soc_pmic_exec_mipi_pmic_seq_element(i2c_address: u16, reg_address: u32, value: u32, mask: u32) -> i32 {
    if intel_pmic_opregion.is_null() { pr_warn("%s: No PMIC registered\n", "intel_soc_pmic_exec_mipi_pmic_seq_element"); return -ENXIO; }
    let d = &*((*intel_pmic_opregion).data); let _guard = (*intel_pmic_opregion).lock.lock();
    if let Some(f) = d.exec_mipi_pmic_seq_element { return f((*intel_pmic_opregion).regmap, i2c_address, reg_address, value, mask); }
    if d.pmic_i2c_address != 0 {
        if i2c_address == d.pmic_i2c_address { regmap_update_bits((*intel_pmic_opregion).regmap, reg_address, mask, value) }
        else { pr_err("%s: Unexpected i2c-addr: 0x%02x (reg-addr 0x%x value 0x%x mask 0x%x)\n", "intel_soc_pmic_exec_mipi_pmic_seq_element", i2c_address, reg_address, value, mask); -ENXIO }
    } else { pr_warn("%s: Not implemented\n", "intel_soc_pmic_exec_mipi_pmic_seq_element"); pr_warn("%s: i2c-addr: 0x%x reg-addr 0x%x value 0x%x mask 0x%x\n", "intel_soc_pmic_exec_mipi_pmic_seq_element", i2c_address, reg_address, value, mask); -EOPNOTSUPP }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
