// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utmath - Integer math support routines

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uint64Struct {
    pub lo: u32,
    pub hi: u32,
}

#[repr(C)]
pub union Uint64Overlay {
    pub full: u64,
    pub part: Uint64Struct,
}

// ACPI_USE_NATIVE_MATH64 and ACPI_USE_NATIVE_DIVIDE are build-time options.

#[cfg(not(feature = "ACPI_USE_NATIVE_MATH64"))]
pub unsafe fn acpi_ut_short_multiply(
    multiplicand: u64,
    multiplier: u32,
    out_product: *mut u64,
) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_short_multiply);
    let m = Uint64Overlay { full: multiplicand };
    let p = (multiplicand as u128).wrapping_mul(multiplier as u128) as u64;
    if !out_product.is_null() { *out_product = p; }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(feature = "ACPI_USE_NATIVE_MATH64")]
pub unsafe fn acpi_ut_short_multiply(
    multiplicand: u64,
    multiplier: u32,
    out_product: *mut u64,
) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_short_multiply);
    if !out_product.is_null() { *out_product = multiplicand.wrapping_mul(multiplier as u64); }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(not(feature = "ACPI_USE_NATIVE_MATH64"))]
pub unsafe fn acpi_ut_short_shift_left(operand: u64, mut count: u32, out_result: *mut u64) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_short_shift_left);
    count &= 63;
    if !out_result.is_null() { *out_result = operand.wrapping_shl(count); }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(feature = "ACPI_USE_NATIVE_MATH64")]
pub unsafe fn acpi_ut_short_shift_left(operand: u64, count: u32, out_result: *mut u64) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_short_shift_left);
    if !out_result.is_null() { *out_result = operand.wrapping_shl(count); }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(not(feature = "ACPI_USE_NATIVE_MATH64"))]
pub unsafe fn acpi_ut_short_shift_right(operand: u64, mut count: u32, out_result: *mut u64) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_short_shift_right);
    count &= 63;
    if !out_result.is_null() { *out_result = operand.wrapping_shr(count); }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(feature = "ACPI_USE_NATIVE_MATH64")]
pub unsafe fn acpi_ut_short_shift_right(operand: u64, count: u32, out_result: *mut u64) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_short_shift_right);
    if !out_result.is_null() { *out_result = operand.wrapping_shr(count); }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(not(feature = "ACPI_USE_NATIVE_DIVIDE"))]
pub unsafe fn acpi_ut_short_divide(
    dividend: u64, divisor: u32, out_quotient: *mut u64, out_remainder: *mut u32,
) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_short_divide);
    if divisor == 0 {
        ACPI_ERROR!((AE_INFO, "Divide by zero"));
        return_ACPI_STATUS!(AE_AML_DIVIDE_BY_ZERO)
    }
    if !out_quotient.is_null() { *out_quotient = dividend / divisor as u64; }
    if !out_remainder.is_null() { *out_remainder = (dividend % divisor as u64) as u32; }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(feature = "ACPI_USE_NATIVE_DIVIDE")]
pub unsafe fn acpi_ut_short_divide(
    dividend: u64, divisor: u32, out_quotient: *mut u64, out_remainder: *mut u32,
) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_short_divide);
    if divisor == 0 {
        ACPI_ERROR!((AE_INFO, "Divide by zero"));
        return_ACPI_STATUS!(AE_AML_DIVIDE_BY_ZERO)
    }
    if !out_quotient.is_null() { *out_quotient = dividend / divisor as u64; }
    if !out_remainder.is_null() { *out_remainder = (dividend % divisor as u64) as u32; }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(not(feature = "ACPI_USE_NATIVE_DIVIDE"))]
pub unsafe fn acpi_ut_divide(
    in_dividend: u64, in_divisor: u64, out_quotient: *mut u64, out_remainder: *mut u64,
) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_divide);
    if in_divisor == 0 {
        ACPI_ERROR!((AE_INFO, "Divide by zero"));
        return_ACPI_STATUS!(AE_AML_DIVIDE_BY_ZERO)
    }
    if !out_quotient.is_null() { *out_quotient = in_dividend / in_divisor; }
    if !out_remainder.is_null() { *out_remainder = in_dividend % in_divisor; }
    return_ACPI_STATUS!(AE_OK)
}

#[cfg(feature = "ACPI_USE_NATIVE_DIVIDE")]
pub unsafe fn acpi_ut_divide(
    in_dividend: u64, in_divisor: u64, out_quotient: *mut u64, out_remainder: *mut u64,
) -> acpi_status {
    ACPI_FUNCTION_TRACE!(ut_divide);
    if in_divisor == 0 {
        ACPI_ERROR!((AE_INFO, "Divide by zero"));
        return_ACPI_STATUS!(AE_AML_DIVIDE_BY_ZERO)
    }
    if !out_quotient.is_null() { *out_quotient = in_dividend / in_divisor; }
    if !out_remainder.is_null() { *out_remainder = in_dividend % in_divisor; }
    return_ACPI_STATUS!(AE_OK)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
