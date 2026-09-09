// Platform-specific memory acceptance declarations and helpers.
//
// The C header includes Linux EFI, TDX, and SEV definitions.  Those symbols
// are supplied by the surrounding translation unit.

/// Accept a physical memory range using the active confidential-computing
/// platform.
#[inline]
pub unsafe fn arch_accept_memory(start: phys_addr_t, end: phys_addr_t) {
    if cpu_feature_enabled(X86_FEATURE_TDX_GUEST) {
        if !tdx_accept_memory(start, end) {
            panic!("TDX: Failed to accept memory\n");
        }
    } else if cc_platform_has(CC_ATTR_GUEST_SEV_SNP) {
        snp_accept_memory(start, end);
    } else {
        panic!("Cannot accept memory: unknown platform\n");
    }
}

/// Return the virtual address of the EFI unaccepted-memory table, if present.
#[inline]
pub unsafe fn efi_get_unaccepted_table() -> *mut efi_unaccepted_memory {
    if efi.unaccepted == EFI_INVALID_TABLE_ADDR {
        core::ptr::null_mut()
    } else {
        __va(efi.unaccepted)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
