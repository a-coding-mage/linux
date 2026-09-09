// Translation of ../tpm.h is supplied by the surrounding translation unit.

unsafe extern "C" {
    pub static tpm1_ascii_b_measurements_seqops: seq_operations;
    pub static tpm1_binary_b_measurements_seqops: seq_operations;
    pub static tpm2_binary_b_measurements_seqops: seq_operations;
}

#[cfg(CONFIG_ACPI)]
unsafe extern "C" {
    pub fn tpm_read_log_acpi(chip: *mut tpm_chip) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_ACPI))]
#[inline]
pub unsafe fn tpm_read_log_acpi(_chip: *mut tpm_chip) -> ::core::ffi::c_int {
    -ENODEV
}

#[cfg(CONFIG_OF)]
unsafe extern "C" {
    pub fn tpm_read_log_of(chip: *mut tpm_chip) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn tpm_read_log_of(_chip: *mut tpm_chip) -> ::core::ffi::c_int {
    -ENODEV
}

#[cfg(CONFIG_EFI)]
unsafe extern "C" {
    pub fn tpm_read_log_efi(chip: *mut tpm_chip) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_EFI))]
#[inline]
pub unsafe fn tpm_read_log_efi(_chip: *mut tpm_chip) -> ::core::ffi::c_int {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
