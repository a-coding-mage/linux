// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: hwregs - Read/write access functions for ACPI control and
// status registers.

// Dependencies supplied by the surrounding ACPI translation.

unsafe fn acpi_hw_get_access_bit_width(
    address: u64,
    reg: *mut acpi_generic_address,
    max_bit_width: u8,
) -> u8 {
    let mut access_bit_width: u8;

    if (*reg).bit_offset == 0
        && (*reg).bit_width != 0
        && ((*reg).bit_width & ((*reg).bit_width - 1)) == 0
        && (*reg).bit_width % 8 == 0
    {
        access_bit_width = (*reg).bit_width;
    } else if (*reg).access_width != 0 {
        access_bit_width = 1u8 << ((*reg).access_width - 1);
    } else {
        let mut v = (*reg).bit_offset.wrapping_add((*reg).bit_width);
        v = if v <= 8 { 8 } else { v.next_power_of_two() };
        access_bit_width = v;
        if access_bit_width <= 8 {
            access_bit_width = 8;
        } else {
            while address % ((access_bit_width >> 3) as u64) != 0 {
                access_bit_width >>= 1;
            }
        }
    }

    if (*reg).space_id == ACPI_ADR_SPACE_SYSTEM_IO {
        access_bit_width = 32;
    }
    if access_bit_width < max_bit_width {
        access_bit_width
    } else {
        max_bit_width
    }
}

pub unsafe fn acpi_hw_validate_register(
    reg: *mut acpi_generic_address,
    max_bit_width: u8,
    address: *mut u64,
) -> acpi_status {
    if reg.is_null() {
        return AE_BAD_PARAMETER;
    }
    *address = (*reg).address;
    if *address == 0 {
        return AE_BAD_ADDRESS;
    }
    if (*reg).space_id != ACPI_ADR_SPACE_SYSTEM_MEMORY
        && (*reg).space_id != ACPI_ADR_SPACE_SYSTEM_IO
    {
        return AE_SUPPORT;
    }
    if (*reg).access_width > 4 {
        return AE_SUPPORT;
    }
    let access_width = acpi_hw_get_access_bit_width(*address, reg, max_bit_width);
    let bit_width = ((*reg).bit_offset as u32 + (*reg).bit_width as u32 + access_width as u32 - 1)
        / access_width as u32 * access_width as u32;
    if max_bit_width as u32 < bit_width {
        return AE_SUPPORT;
    }
    AE_OK
}

pub unsafe fn acpi_hw_read(value: *mut u64, reg: *mut acpi_generic_address) -> acpi_status {
    let mut address = 0u64;
    let status = acpi_hw_validate_register(reg, 64, &mut address);
    if ACPI_FAILURE(status) { return status; }

    *value = 0;
    let access_width = acpi_hw_get_access_bit_width(address, reg, 64);
    let mut bit_width = (*reg).bit_offset as u32 + (*reg).bit_width as u32;
    let mut bit_offset = (*reg).bit_offset;
    let mut index = 0u8;
    let mut status = AE_OK;
    while bit_width != 0 {
        let mut value64 = 0u64;
        if bit_offset >= access_width {
            bit_offset -= access_width;
        } else if (*reg).space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY {
            status = acpi_os_read_memory(
                address + index as u64 * (access_width as u64 / 8),
                &mut value64,
                access_width,
            );
        } else {
            let mut value32 = 0u32;
            status = acpi_hw_read_port(
                address + index as u64 * (access_width as u64 / 8),
                &mut value32,
                access_width,
            );
            value64 = value32 as u64;
        }
        let mask = if access_width == 64 { u64::MAX } else { (1u64 << access_width) - 1 };
        *value |= (value64 & mask) << (index as u32 * access_width as u32);
        bit_width -= bit_width.min(access_width as u32);
        index += 1;
    }
    status
}

pub unsafe fn acpi_hw_write(value: u64, reg: *mut acpi_generic_address) -> acpi_status {
    let mut address = 0u64;
    let mut status = acpi_hw_validate_register(reg, 64, &mut address);
    if ACPI_FAILURE(status) { return status; }
    let access_width = acpi_hw_get_access_bit_width(address, reg, 64);
    let mut bit_width = (*reg).bit_offset as u32 + (*reg).bit_width as u32;
    let mut bit_offset = (*reg).bit_offset;
    let mut index = 0u8;
    while bit_width != 0 {
        let mask = if access_width == 64 { u64::MAX } else { (1u64 << access_width) - 1 };
        let value64 = (value >> (index as u32 * access_width as u32)) & mask;
        if bit_offset >= access_width {
            bit_offset -= access_width;
        } else if (*reg).space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY {
            status = acpi_os_write_memory(
                address + index as u64 * (access_width as u64 / 8), value64, access_width);
        } else {
            status = acpi_hw_write_port(
                address + index as u64 * (access_width as u64 / 8), value64 as u32, access_width);
        }
        bit_width -= bit_width.min(access_width as u32);
        index += 1;
    }
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_hw_clear_acpi_status() -> acpi_status {
    let lock_flags = acpi_os_acquire_raw_lock(acpi_gbl_hardware_lock);
    let mut status = acpi_hw_register_write(ACPI_REGISTER_PM1_STATUS, ACPI_BITMASK_ALL_FIXED_STATUS);
    acpi_os_release_raw_lock(acpi_gbl_hardware_lock, lock_flags);
    if ACPI_FAILURE(status) { return status; }
    status = acpi_ev_walk_gpe_list(acpi_hw_clear_gpe_block, core::ptr::null_mut());
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_hw_get_bit_register_info(register_id: u32) -> *mut acpi_bit_register_info {
    if register_id > ACPI_BITREG_MAX { return core::ptr::null_mut(); }
    acpi_gbl_bit_register_info.add(register_id as usize)
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_hw_write_pm1_control(pm1a_control: u32, pm1b_control: u32) -> acpi_status {
    let mut status = acpi_hw_write(pm1a_control as u64, &mut acpi_gbl_FADT.xpm1a_control_block);
    if ACPI_FAILURE(status) { return status; }
    if acpi_gbl_FADT.xpm1b_control_block.address != 0 {
        status = acpi_hw_write(pm1b_control as u64, &mut acpi_gbl_FADT.xpm1b_control_block);
    }
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_hw_register_read(register_id: u32, return_value: *mut u32) -> acpi_status {
    let mut value = 0u32;
    let mut value64 = 0u64;
    let status = match register_id {
        ACPI_REGISTER_PM1_STATUS => acpi_hw_read_multiple(&mut value, &mut acpi_gbl_xpm1a_status, &mut acpi_gbl_xpm1b_status),
        ACPI_REGISTER_PM1_ENABLE => acpi_hw_read_multiple(&mut value, &mut acpi_gbl_xpm1a_enable, &mut acpi_gbl_xpm1b_enable),
        ACPI_REGISTER_PM1_CONTROL => { let s = acpi_hw_read_multiple(&mut value, &mut acpi_gbl_FADT.xpm1a_control_block, &mut acpi_gbl_FADT.xpm1b_control_block); value &= !ACPI_PM1_CONTROL_WRITEONLY_BITS; s }
        ACPI_REGISTER_PM2_CONTROL => { let s = acpi_hw_read(&mut value64, &mut acpi_gbl_FADT.xpm2_control_block); if ACPI_SUCCESS(s) { value = value64 as u32; } s }
        ACPI_REGISTER_PM_TIMER => { let s = acpi_hw_read(&mut value64, &mut acpi_gbl_FADT.xpm_timer_block); if ACPI_SUCCESS(s) { value = value64 as u32; } s }
        ACPI_REGISTER_SMI_COMMAND_BLOCK => acpi_hw_read_port(acpi_gbl_FADT.smi_command, &mut value, 8),
        _ => AE_BAD_PARAMETER,
    };
    if ACPI_SUCCESS(status) { *return_value = value; }
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_hw_register_write(register_id: u32, mut value: u32) -> acpi_status {
    let mut read_value = 0u32;
    let mut read_value64 = 0u64;
    match register_id {
        ACPI_REGISTER_PM1_STATUS => { value &= !ACPI_PM1_STATUS_PRESERVED_BITS; acpi_hw_write_multiple(value, &mut acpi_gbl_xpm1a_status, &mut acpi_gbl_xpm1b_status) }
        ACPI_REGISTER_PM1_ENABLE => acpi_hw_write_multiple(value, &mut acpi_gbl_xpm1a_enable, &mut acpi_gbl_xpm1b_enable),
        ACPI_REGISTER_PM1_CONTROL => { let s = acpi_hw_read_multiple(&mut read_value, &mut acpi_gbl_FADT.xpm1a_control_block, &mut acpi_gbl_FADT.xpm1b_control_block); if ACPI_FAILURE(s) { return s; } value = (value & !ACPI_PM1_CONTROL_PRESERVED_BITS) | (read_value & ACPI_PM1_CONTROL_PRESERVED_BITS); acpi_hw_write_multiple(value, &mut acpi_gbl_FADT.xpm1a_control_block, &mut acpi_gbl_FADT.xpm1b_control_block) }
        ACPI_REGISTER_PM2_CONTROL => { let s = acpi_hw_read(&mut read_value64, &mut acpi_gbl_FADT.xpm2_control_block); if ACPI_FAILURE(s) { return s; } read_value = read_value64 as u32; value = (value & !ACPI_PM2_CONTROL_PRESERVED_BITS) | (read_value & ACPI_PM2_CONTROL_PRESERVED_BITS); acpi_hw_write(value as u64, &mut acpi_gbl_FADT.xpm2_control_block) }
        ACPI_REGISTER_PM_TIMER => acpi_hw_write(value as u64, &mut acpi_gbl_FADT.xpm_timer_block),
        ACPI_REGISTER_SMI_COMMAND_BLOCK => acpi_hw_write_port(acpi_gbl_FADT.smi_command, value, 8),
        _ => AE_BAD_PARAMETER,
    }
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
unsafe fn acpi_hw_read_multiple(value: *mut u32, register_a: *mut acpi_generic_address, register_b: *mut acpi_generic_address) -> acpi_status {
    let mut value64 = 0u64;
    let status = acpi_hw_read(&mut value64, register_a);
    if ACPI_FAILURE(status) { return status; }
    let value_a = value64 as u32;
    let mut value_b = 0u32;
    if (*register_b).address != 0 { let status = acpi_hw_read(&mut value64, register_b); if ACPI_FAILURE(status) { return status; } value_b = value64 as u32; }
    *value = value_a | value_b;
    AE_OK
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
unsafe fn acpi_hw_write_multiple(value: u32, register_a: *mut acpi_generic_address, register_b: *mut acpi_generic_address) -> acpi_status {
    let mut status = acpi_hw_write(value as u64, register_a);
    if ACPI_FAILURE(status) { return status; }
    if (*register_b).address != 0 { status = acpi_hw_write(value as u64, register_b); }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
