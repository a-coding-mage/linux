// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: exutils - interpreter/scanner utilities

// DEFINE_AML_GLOBALS is defined in the original compilation unit so that
// globals from amlcode.h are defined here rather than merely declared.

type AcpiStatus = u32;
type AcpiThreadId = usize;

const ACPI_MTX_INTERPRETER: u32 = 0;
const ACPI_MTX_NAMESPACE: u32 = 1;
const ACPI_WAIT_FOREVER: u32 = 0xffff_ffff;
const ACPI_DESC_TYPE_OPERAND: u8 = 1;
const ACPI_TYPE_INTEGER: u8 = 1;
const ACPI_UINT32_MAX: u64 = 0xffff_ffff;
const AML_FIELD_LOCK_RULE_MASK: u32 = 1;
const ACPI_NUM_PREDEFINED_REGIONS: u8 = 0x80;
const ACPI_USER_REGION_BEGIN: u8 = 0x80;
const ACPI_ADR_SPACE_DATA_TABLE: u8 = 0x7e;
const ACPI_ADR_SPACE_FIXED_HARDWARE: u8 = 0x7f;

#[repr(C)]
pub struct AcpiCommonObject {
    pub descriptor_type: u8,
    pub type_: u8,
}

#[repr(C)]
pub struct AcpiIntegerObject {
    pub common: AcpiCommonObject,
    pub value: u64,
}

#[repr(C)]
pub union AcpiOperandObject {
    pub common: AcpiCommonObject,
    pub integer: AcpiIntegerObject,
}

extern "C" {
    fn acpi_ut_acquire_mutex(mutex_id: u32) -> AcpiStatus;
    fn acpi_ut_release_mutex(mutex_id: u32) -> AcpiStatus;
    fn acpi_ex_acquire_mutex_object(timeout: u32, mutex: *mut AcpiOperandObject,
                                    thread_id: AcpiThreadId) -> AcpiStatus;
    fn acpi_ex_release_mutex_object(mutex: *mut AcpiOperandObject) -> AcpiStatus;
    fn acpi_os_get_thread_id() -> AcpiThreadId;
    fn acpi_ut_short_divide(dividend: u64, divisor: u32, quotient: *mut u64,
                            remainder: *mut u32) -> AcpiStatus;
    fn acpi_ut_dword_byte_swap(value: u32) -> u32;
    fn acpi_ut_hex_to_ascii_char(value: u64, position: u32) -> u8;
}

extern "C" {
    static mut acpi_gbl_integer_byte_width: u8;
    static mut acpi_gbl_global_lock_mutex: *mut AcpiOperandObject;
}

#[inline]
pub unsafe fn acpi_ex_enter_interpreter() {
    let _status = acpi_ut_acquire_mutex(ACPI_MTX_INTERPRETER);
    let _status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
}

#[inline]
pub unsafe fn acpi_ex_exit_interpreter() {
    let _status = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    let _status = acpi_ut_release_mutex(ACPI_MTX_INTERPRETER);
}

pub unsafe fn acpi_ex_truncate_for32bit_table(obj_desc: *mut AcpiOperandObject) -> u8 {
    if obj_desc.is_null()
        || (*obj_desc).common.descriptor_type != ACPI_DESC_TYPE_OPERAND
        || (*obj_desc).common.type_ != ACPI_TYPE_INTEGER
    {
        return 0;
    }

    if acpi_gbl_integer_byte_width == 4 && (*obj_desc).integer.value > ACPI_UINT32_MAX {
        (*obj_desc).integer.value &= ACPI_UINT32_MAX;
        return 1;
    }
    0
}

pub unsafe fn acpi_ex_acquire_global_lock(field_flags: u32) {
    if field_flags & AML_FIELD_LOCK_RULE_MASK == 0 {
        return;
    }
    let _status = acpi_ex_acquire_mutex_object(
        ACPI_WAIT_FOREVER,
        acpi_gbl_global_lock_mutex,
        acpi_os_get_thread_id(),
    );
}

pub unsafe fn acpi_ex_release_global_lock(field_flags: u32) {
    if field_flags & AML_FIELD_LOCK_RULE_MASK == 0 {
        return;
    }
    let _status = acpi_ex_release_mutex_object(acpi_gbl_global_lock_mutex);
}

unsafe fn acpi_ex_digits_needed(mut value: u64, base: u32) -> u32 {
    if value == 0 {
        return 1;
    }
    let mut num_digits = 0;
    while value != 0 {
        let mut quotient = 0;
        let _ = acpi_ut_short_divide(value, base, &mut quotient, core::ptr::null_mut());
        value = quotient;
        num_digits += 1;
    }
    num_digits
}

pub unsafe fn acpi_ex_eisa_id_to_string(out_string: *mut u8, compressed_id: u64) {
    let swapped_id = acpi_ut_dword_byte_swap(compressed_id as u32);
    *out_string.add(0) = (0x40 + ((swapped_id >> 26) & 0x1f)) as u8;
    *out_string.add(1) = (0x40 + ((swapped_id >> 21) & 0x1f)) as u8;
    *out_string.add(2) = (0x40 + ((swapped_id >> 16) & 0x1f)) as u8;
    *out_string.add(3) = acpi_ut_hex_to_ascii_char(swapped_id as u64, 12);
    *out_string.add(4) = acpi_ut_hex_to_ascii_char(swapped_id as u64, 8);
    *out_string.add(5) = acpi_ut_hex_to_ascii_char(swapped_id as u64, 4);
    *out_string.add(6) = acpi_ut_hex_to_ascii_char(swapped_id as u64, 0);
    *out_string.add(7) = 0;
}

pub unsafe fn acpi_ex_integer_to_string(out_string: *mut u8, mut value: u64) {
    let digits_needed = acpi_ex_digits_needed(value, 10);
    *out_string.add(digits_needed as usize) = 0;
    let mut count = digits_needed;
    while count > 0 {
        let mut quotient = 0;
        let mut remainder = 0;
        let _ = acpi_ut_short_divide(value, 10, &mut quotient, &mut remainder);
        value = quotient;
        *out_string.add((count - 1) as usize) = (b'0' + remainder as u8);
        count -= 1;
    }
}

pub unsafe fn acpi_ex_pci_cls_to_string(out_string: *mut u8, class_code: *const u8) {
    for i in 0..3usize {
        let value = *class_code.add(i) as u64;
        *out_string.add(i * 2) = acpi_ut_hex_to_ascii_char(value, 4);
        *out_string.add(i * 2 + 1) = acpi_ut_hex_to_ascii_char(value, 0);
    }
    *out_string.add(6) = 0;
}

pub fn acpi_is_valid_space_id(space_id: u8) -> u8 {
    if space_id >= ACPI_NUM_PREDEFINED_REGIONS
        && space_id < ACPI_USER_REGION_BEGIN
        && space_id != ACPI_ADR_SPACE_DATA_TABLE
        && space_id != ACPI_ADR_SPACE_FIXED_HARDWARE
    {
        0
    } else {
        1
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
