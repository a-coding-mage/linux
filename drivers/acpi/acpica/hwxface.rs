// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Public ACPICA hardware interfaces.
//
// C headers and ACPICA macros/types referenced here are supplied by the
// surrounding translation unit.

extern "C" {
    static mut acpi_gbl_FADT: acpi_fadt;
    static mut acpi_gbl_hardware_lock: acpi_handle;
    static acpi_gbl_sleep_state_names: [*const i8; ACPI_S_STATES_MAX as usize + 1];

    fn acpi_os_write_port(address: acpi_io_address, value: u32, width: u32) -> acpi_status;
    fn acpi_hw_write(value: u64, reg: *mut acpi_generic_address) -> acpi_status;
    fn acpi_hw_read(value: *mut u64, reg: *mut acpi_generic_address) -> acpi_status;
    fn acpi_hw_get_bit_register_info(id: u32) -> *mut acpi_bit_register_info;
    fn acpi_hw_register_read(id: u32, value: *mut u32) -> acpi_status;
    fn acpi_hw_register_write(id: u32, value: u32) -> acpi_status;
    fn acpi_os_acquire_raw_lock(lock: acpi_handle) -> acpi_cpu_flags;
    fn acpi_os_release_raw_lock(lock: acpi_handle, flags: acpi_cpu_flags);
    fn acpi_ns_evaluate(info: *mut acpi_evaluate_info) -> acpi_status;
    fn acpi_ut_remove_reference(object: *mut acpi_operand_object);
    fn acpi_allocate_zeroed(size: usize) -> *mut core::ffi::c_void;
    fn acpi_free(pointer: *mut core::ffi::c_void);
}

pub unsafe fn acpi_reset() -> acpi_status {
    let reset_reg: *mut acpi_generic_address = &mut acpi_gbl_FADT.reset_register;
    if (acpi_gbl_FADT.flags & ACPI_FADT_RESET_REGISTER) == 0 || (*reset_reg).address == 0 {
        return AE_NOT_EXIST;
    }

    if (*reset_reg).space_id == ACPI_ADR_SPACE_SYSTEM_IO {
        acpi_os_write_port(
            (*reset_reg).address as acpi_io_address,
            acpi_gbl_FADT.reset_value,
            ACPI_RESET_REGISTER_WIDTH,
        )
    } else {
        acpi_hw_write(acpi_gbl_FADT.reset_value as u64, reset_reg)
    }
}

pub unsafe fn acpi_read(return_value: *mut u64, reg: *mut acpi_generic_address) -> acpi_status {
    acpi_hw_read(return_value, reg)
}

pub unsafe fn acpi_write(value: u64, reg: *mut acpi_generic_address) -> acpi_status {
    acpi_hw_write(value, reg)
}

#[cfg(not(feature = "ACPI_REDUCED_HARDWARE"))]
pub unsafe fn acpi_read_bit_register(register_id: u32, return_value: *mut u32) -> acpi_status {
    let bit_reg_info = acpi_hw_get_bit_register_info(register_id);
    if bit_reg_info.is_null() {
        return AE_BAD_PARAMETER;
    }

    let mut register_value: u32 = 0;
    let status = acpi_hw_register_read((*bit_reg_info).parent_register, &mut register_value);
    if ACPI_FAILURE(status) {
        return status;
    }

    let value = (register_value & (*bit_reg_info).access_bit_mask) >> (*bit_reg_info).bit_position;
    *return_value = value;
    AE_OK
}

#[cfg(not(feature = "ACPI_REDUCED_HARDWARE"))]
pub unsafe fn acpi_write_bit_register(register_id: u32, value: u32) -> acpi_status {
    let bit_reg_info = acpi_hw_get_bit_register_info(register_id);
    if bit_reg_info.is_null() {
        return AE_BAD_PARAMETER;
    }

    let lock_flags = acpi_os_acquire_raw_lock(acpi_gbl_hardware_lock);
    let mut register_value: u32 = 0;
    let mut status = AE_OK;

    if (*bit_reg_info).parent_register != ACPI_REGISTER_PM1_STATUS {
        status = acpi_hw_register_read((*bit_reg_info).parent_register, &mut register_value);
        if ACPI_FAILURE(status) {
            acpi_os_release_raw_lock(acpi_gbl_hardware_lock, lock_flags);
            return status;
        }

        register_value = (register_value & !(*bit_reg_info).access_bit_mask)
            | ((value << (*bit_reg_info).bit_position) & (*bit_reg_info).access_bit_mask);
        status = acpi_hw_register_write((*bit_reg_info).parent_register, register_value);
    } else {
        register_value = (value << (*bit_reg_info).bit_position) & (*bit_reg_info).access_bit_mask;
        if register_value != 0 {
            status = acpi_hw_register_write(ACPI_REGISTER_PM1_STATUS, register_value);
        }
    }

    acpi_os_release_raw_lock(acpi_gbl_hardware_lock, lock_flags);
    status
}

pub unsafe fn acpi_get_sleep_type_data(
    sleep_state: u8,
    sleep_type_a: *mut u8,
    sleep_type_b: *mut u8,
) -> acpi_status {
    if sleep_state > ACPI_S_STATES_MAX || sleep_type_a.is_null() || sleep_type_b.is_null() {
        return AE_BAD_PARAMETER;
    }

    let info = acpi_allocate_zeroed(core::mem::size_of::<acpi_evaluate_info>())
        as *mut acpi_evaluate_info;
    if info.is_null() {
        return AE_NO_MEMORY;
    }

    (*info).relative_pathname = acpi_gbl_sleep_state_names[sleep_state as usize];
    let mut status = acpi_ns_evaluate(info);
    if ACPI_FAILURE(status) {
        if status == AE_NOT_FOUND {
            acpi_free(info as *mut core::ffi::c_void);
            return status;
        }
        acpi_free(info as *mut core::ffi::c_void);
        return status;
    }

    if (*info).return_object.is_null() {
        status = AE_AML_NO_RETURN_VALUE;
        acpi_free(info as *mut core::ffi::c_void);
        return status;
    }

    let object = (*info).return_object;
    if (*object).common.type_ != ACPI_TYPE_PACKAGE {
        status = AE_AML_OPERAND_TYPE;
        acpi_ut_remove_reference(object);
        acpi_free(info as *mut core::ffi::c_void);
        return status;
    }

    let count = (*object).package.count;
    let elements = (*object).package.elements;
    if count == 0 {
        status = AE_AML_PACKAGE_LIMIT;
    } else if count == 1 {
        if (*elements).common.type_ != ACPI_TYPE_INTEGER {
            status = AE_AML_OPERAND_TYPE;
        } else {
            *sleep_type_a = (*elements).integer.value as u8;
            *sleep_type_b = ((*elements).integer.value >> 8) as u8;
        }
    } else {
        if (*elements).common.type_ != ACPI_TYPE_INTEGER
            || (*elements.add(1)).common.type_ != ACPI_TYPE_INTEGER
        {
            status = AE_AML_OPERAND_TYPE;
        } else {
            *sleep_type_a = (*elements).integer.value as u8;
            *sleep_type_b = (*elements.add(1)).integer.value as u8;
        }
    }

    acpi_ut_remove_reference(object);
    acpi_free(info as *mut core::ffi::c_void);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
