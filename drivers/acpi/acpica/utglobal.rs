// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: utglobal - Global variables for the ACPI subsystem
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

use core::ffi::c_char;

/* Dependencies supplied by the ACPI headers and common implementation. */

pub const acpi_gbl_sleep_state_names: [*const c_char; ACPI_S_STATE_COUNT as usize] = [
    b"\\_S0_\0".as_ptr() as *const c_char,
    b"\\_S1_\0".as_ptr() as *const c_char,
    b"\\_S2_\0".as_ptr() as *const c_char,
    b"\\_S3_\0".as_ptr() as *const c_char,
    b"\\_S4_\0".as_ptr() as *const c_char,
    b"\\_S5_\0".as_ptr() as *const c_char,
];

pub const acpi_gbl_lowest_dstate_names: [*const c_char; ACPI_NUM_sx_w_METHODS as usize] = [
    b"_S0W\0".as_ptr() as *const c_char,
    b"_S1W\0".as_ptr() as *const c_char,
    b"_S2W\0".as_ptr() as *const c_char,
    b"_S3W\0".as_ptr() as *const c_char,
    b"_S4W\0".as_ptr() as *const c_char,
];

pub const acpi_gbl_highest_dstate_names: [*const c_char; ACPI_NUM_sx_d_METHODS as usize] = [
    b"_S1D\0".as_ptr() as *const c_char,
    b"_S2D\0".as_ptr() as *const c_char,
    b"_S3D\0".as_ptr() as *const c_char,
    b"_S4D\0".as_ptr() as *const c_char,
];

pub const acpi_gbl_lower_hex_digits: [c_char; 17] = *b"0123456789abcdef\0";
pub const acpi_gbl_upper_hex_digits: [c_char; 17] = *b"0123456789ABCDEF\0";

/* Predefined ACPI Names (Built-in to the Interpreter). */
pub const acpi_gbl_pre_defined_names: [struct_acpi_predefined_names; 11] = [
    struct_acpi_predefined_names { name: b"_GPE\0".as_ptr() as *const c_char, type_: ACPI_TYPE_LOCAL_SCOPE, value: core::ptr::null_mut() },
    struct_acpi_predefined_names { name: b"_PR_\0".as_ptr() as *const c_char, type_: ACPI_TYPE_LOCAL_SCOPE, value: core::ptr::null_mut() },
    struct_acpi_predefined_names { name: b"_SB_\0".as_ptr() as *const c_char, type_: ACPI_TYPE_DEVICE, value: core::ptr::null_mut() },
    struct_acpi_predefined_names { name: b"_SI_\0".as_ptr() as *const c_char, type_: ACPI_TYPE_LOCAL_SCOPE, value: core::ptr::null_mut() },
    struct_acpi_predefined_names { name: b"_TZ_\0".as_ptr() as *const c_char, type_: ACPI_TYPE_DEVICE, value: core::ptr::null_mut() },
    struct_acpi_predefined_names { name: b"_REV\0".as_ptr() as *const c_char, type_: ACPI_TYPE_INTEGER, value: 2 as *mut c_char },
    struct_acpi_predefined_names { name: b"_OS_\0".as_ptr() as *const c_char, type_: ACPI_TYPE_STRING, value: ACPI_OS_NAME as *mut c_char },
    struct_acpi_predefined_names { name: b"_GL_\0".as_ptr() as *const c_char, type_: ACPI_TYPE_MUTEX, value: 1 as *mut c_char },
    struct_acpi_predefined_names { name: b"_OSI\0".as_ptr() as *const c_char, type_: ACPI_TYPE_METHOD, value: 1 as *mut c_char },
    struct_acpi_predefined_names { name: core::ptr::null(), type_: ACPI_TYPE_ANY, value: core::ptr::null_mut() },
];

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub static mut acpi_gbl_bit_register_info: [struct_acpi_bit_register_info; ACPI_NUM_BITREG as usize] = [
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_STATUS, bit_position: ACPI_BITPOSITION_TIMER_STATUS, access_bit_mask: ACPI_BITMASK_TIMER_STATUS },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_STATUS, bit_position: ACPI_BITPOSITION_BUS_MASTER_STATUS, access_bit_mask: ACPI_BITMASK_BUS_MASTER_STATUS },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_STATUS, bit_position: ACPI_BITPOSITION_GLOBAL_LOCK_STATUS, access_bit_mask: ACPI_BITMASK_GLOBAL_LOCK_STATUS },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_STATUS, bit_position: ACPI_BITPOSITION_POWER_BUTTON_STATUS, access_bit_mask: ACPI_BITMASK_POWER_BUTTON_STATUS },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_STATUS, bit_position: ACPI_BITPOSITION_SLEEP_BUTTON_STATUS, access_bit_mask: ACPI_BITMASK_SLEEP_BUTTON_STATUS },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_STATUS, bit_position: ACPI_BITPOSITION_RT_CLOCK_STATUS, access_bit_mask: ACPI_BITMASK_RT_CLOCK_STATUS },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_STATUS, bit_position: ACPI_BITPOSITION_WAKE_STATUS, access_bit_mask: ACPI_BITMASK_WAKE_STATUS },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_STATUS, bit_position: ACPI_BITPOSITION_PCIEXP_WAKE_STATUS, access_bit_mask: ACPI_BITMASK_PCIEXP_WAKE_STATUS },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_ENABLE, bit_position: ACPI_BITPOSITION_TIMER_ENABLE, access_bit_mask: ACPI_BITMASK_TIMER_ENABLE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_ENABLE, bit_position: ACPI_BITPOSITION_GLOBAL_LOCK_ENABLE, access_bit_mask: ACPI_BITMASK_GLOBAL_LOCK_ENABLE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_ENABLE, bit_position: ACPI_BITPOSITION_POWER_BUTTON_ENABLE, access_bit_mask: ACPI_BITMASK_POWER_BUTTON_ENABLE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_ENABLE, bit_position: ACPI_BITPOSITION_SLEEP_BUTTON_ENABLE, access_bit_mask: ACPI_BITMASK_SLEEP_BUTTON_ENABLE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_ENABLE, bit_position: ACPI_BITPOSITION_RT_CLOCK_ENABLE, access_bit_mask: ACPI_BITMASK_RT_CLOCK_ENABLE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_ENABLE, bit_position: ACPI_BITPOSITION_PCIEXP_WAKE_DISABLE, access_bit_mask: ACPI_BITMASK_PCIEXP_WAKE_DISABLE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_CONTROL, bit_position: ACPI_BITPOSITION_SCI_ENABLE, access_bit_mask: ACPI_BITMASK_SCI_ENABLE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_CONTROL, bit_position: ACPI_BITPOSITION_BUS_MASTER_RLD, access_bit_mask: ACPI_BITMASK_BUS_MASTER_RLD },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_CONTROL, bit_position: ACPI_BITPOSITION_GLOBAL_LOCK_RELEASE, access_bit_mask: ACPI_BITMASK_GLOBAL_LOCK_RELEASE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_CONTROL, bit_position: ACPI_BITPOSITION_SLEEP_TYPE, access_bit_mask: ACPI_BITMASK_SLEEP_TYPE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM1_CONTROL, bit_position: ACPI_BITPOSITION_SLEEP_ENABLE, access_bit_mask: ACPI_BITMASK_SLEEP_ENABLE },
    struct_acpi_bit_register_info { parent_register: ACPI_REGISTER_PM2_CONTROL, bit_position: ACPI_BITPOSITION_ARB_DISABLE, access_bit_mask: ACPI_BITMASK_ARB_DISABLE },
];

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub static mut acpi_gbl_fixed_event_info: [struct_acpi_fixed_event_info; ACPI_NUM_FIXED_EVENTS as usize] = [
    struct_acpi_fixed_event_info { status_register: ACPI_BITREG_TIMER_STATUS, enable_register: ACPI_BITREG_TIMER_ENABLE, status_bit_mask: ACPI_BITMASK_TIMER_STATUS, enable_bit_mask: ACPI_BITMASK_TIMER_ENABLE },
    struct_acpi_fixed_event_info { status_register: ACPI_BITREG_GLOBAL_LOCK_STATUS, enable_register: ACPI_BITREG_GLOBAL_LOCK_ENABLE, status_bit_mask: ACPI_BITMASK_GLOBAL_LOCK_STATUS, enable_bit_mask: ACPI_BITMASK_GLOBAL_LOCK_ENABLE },
    struct_acpi_fixed_event_info { status_register: ACPI_BITREG_POWER_BUTTON_STATUS, enable_register: ACPI_BITREG_POWER_BUTTON_ENABLE, status_bit_mask: ACPI_BITMASK_POWER_BUTTON_STATUS, enable_bit_mask: ACPI_BITMASK_POWER_BUTTON_ENABLE },
    struct_acpi_fixed_event_info { status_register: ACPI_BITREG_SLEEP_BUTTON_STATUS, enable_register: ACPI_BITREG_SLEEP_BUTTON_ENABLE, status_bit_mask: ACPI_BITMASK_SLEEP_BUTTON_STATUS, enable_bit_mask: ACPI_BITMASK_SLEEP_BUTTON_ENABLE },
    struct_acpi_fixed_event_info { status_register: ACPI_BITREG_RT_CLOCK_STATUS, enable_register: ACPI_BITREG_RT_CLOCK_ENABLE, status_bit_mask: ACPI_BITMASK_RT_CLOCK_STATUS, enable_bit_mask: ACPI_BITMASK_RT_CLOCK_ENABLE },
];

#[cfg(any(ACPI_DISASSEMBLER, ACPI_ASL_COMPILER))]
pub const acpi_gbl_pld_panel_list: [*const c_char; 8] = [b"TOP\0".as_ptr() as *const c_char, b"BOTTOM\0".as_ptr() as *const c_char, b"LEFT\0".as_ptr() as *const c_char, b"RIGHT\0".as_ptr() as *const c_char, b"FRONT\0".as_ptr() as *const c_char, b"BACK\0".as_ptr() as *const c_char, b"UNKNOWN\0".as_ptr() as *const c_char, core::ptr::null()];
#[cfg(any(ACPI_DISASSEMBLER, ACPI_ASL_COMPILER))]
pub const acpi_gbl_pld_vertical_position_list: [*const c_char; 4] = [b"UPPER\0".as_ptr() as *const c_char, b"CENTER\0".as_ptr() as *const c_char, b"LOWER\0".as_ptr() as *const c_char, core::ptr::null()];
#[cfg(any(ACPI_DISASSEMBLER, ACPI_ASL_COMPILER))]
pub const acpi_gbl_pld_horizontal_position_list: [*const c_char; 4] = [b"LEFT\0".as_ptr() as *const c_char, b"CENTER\0".as_ptr() as *const c_char, b"RIGHT\0".as_ptr() as *const c_char, core::ptr::null()];
#[cfg(any(ACPI_DISASSEMBLER, ACPI_ASL_COMPILER))]
pub const acpi_gbl_pld_shape_list: [*const c_char; 10] = [b"ROUND\0".as_ptr() as *const c_char, b"OVAL\0".as_ptr() as *const c_char, b"SQUARE\0".as_ptr() as *const c_char, b"VERTICALRECTANGLE\0".as_ptr() as *const c_char, b"HORIZONTALRECTANGLE\0".as_ptr() as *const c_char, b"VERTICALTRAPEZOID\0".as_ptr() as *const c_char, b"HORIZONTALTRAPEZOID\0".as_ptr() as *const c_char, b"UNKNOWN\0".as_ptr() as *const c_char, b"CHAMFERED\0".as_ptr() as *const c_char, core::ptr::null()];


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
