// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: utexcep - Exception code support
 *
 ******************************************************************************/

// EXPORT_ACPI_INTERFACES
// ACPI_DEFINE_EXCEPTION_TABLE
// Dependencies are supplied by the surrounding ACPICA translation unit.

/// Translate an ACPI exception into an ASCII string.
///
/// A valid pointer is always returned. Unknown codes produce the literal
/// `UNKNOWN_STATUS_CODE` string.
#[no_mangle]
pub unsafe extern "C" fn acpi_format_exception(status: acpi_status) -> *const core::ffi::c_char {
    let exception = acpi_ut_validate_exception(status);
    if exception.is_null() {
        // Exception code was not recognized.
        // ACPI_ERROR((AE_INFO, "Unknown exception code: 0x%8.8X", status));
        return b"UNKNOWN_STATUS_CODE\0".as_ptr() as *const core::ffi::c_char;
    }

    (*exception).name
}

/// Validate and translate an ACPI exception into an exception descriptor.
/// Returns null if the exception is not valid.
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_validate_exception(
    status: acpi_status,
) -> *const acpi_exception_info {
    let sub_status: u32 = status & !AE_CODE_MASK;
    let mut exception: *const acpi_exception_info = core::ptr::null();

    // Status is composed of two parts, a "type" and an actual code.
    match status & AE_CODE_MASK {
        AE_CODE_ENVIRONMENTAL => {
            if sub_status <= AE_CODE_ENV_MAX {
                exception = &acpi_gbl_exception_names_env[sub_status as usize];
            }
        }

        AE_CODE_PROGRAMMER => {
            if sub_status <= AE_CODE_PGM_MAX {
                exception = &acpi_gbl_exception_names_pgm[sub_status as usize];
            }
        }

        AE_CODE_ACPI_TABLES => {
            if sub_status <= AE_CODE_TBL_MAX {
                exception = &acpi_gbl_exception_names_tbl[sub_status as usize];
            }
        }

        AE_CODE_AML => {
            if sub_status <= AE_CODE_AML_MAX {
                exception = &acpi_gbl_exception_names_aml[sub_status as usize];
            }
        }

        AE_CODE_CONTROL => {
            if sub_status <= AE_CODE_CTRL_MAX {
                exception = &acpi_gbl_exception_names_ctrl[sub_status as usize];
            }
        }

        _ => {}
    }

    if exception.is_null() || (*exception).name.is_null() {
        return core::ptr::null();
    }

    exception
}

extern "C" {
    static acpi_gbl_exception_names_env: [acpi_exception_info; AE_CODE_ENV_MAX as usize + 1];
    static acpi_gbl_exception_names_pgm: [acpi_exception_info; AE_CODE_PGM_MAX as usize + 1];
    static acpi_gbl_exception_names_tbl: [acpi_exception_info; AE_CODE_TBL_MAX as usize + 1];
    static acpi_gbl_exception_names_aml: [acpi_exception_info; AE_CODE_AML_MAX as usize + 1];
    static acpi_gbl_exception_names_ctrl: [acpi_exception_info; AE_CODE_CTRL_MAX as usize + 1];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
