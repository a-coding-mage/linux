// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding decompressor implementation:
// error.h, misc.h, tdx.h, sev.h, and asm/shared/tdx.h.

/*
 * accept_memory() and process_unaccepted_memory() called from EFI stub which
 * runs before decompressor and its early_tdx_detect().
 *
 * Enumerate TDX directly from the early users.
 */
unsafe fn early_is_tdx_guest() -> bool {
    static mut ONCE: bool = false;
    static mut IS_TDX: bool = false;

    // Build-time CONFIG_INTEL_TDX_GUEST condition is supplied by the target.
    if !IS_ENABLED!(CONFIG_INTEL_TDX_GUEST) {
        return false;
    }

    if !ONCE {
        let mut eax: u32 = 0;
        let mut sig: [u32; 3] = [0; 3];

        cpuid_count(
            TDX_CPUID_LEAF_ID,
            0,
            &mut eax,
            &mut sig[0],
            &mut sig[2],
            &mut sig[1],
        );
        IS_TDX = !memcmp(
            TDX_IDENT.as_ptr(),
            sig.as_ptr().cast(),
            core::mem::size_of_val(&sig),
        );
        ONCE = true;
    }

    IS_TDX
}

pub unsafe fn arch_accept_memory(start: phys_addr_t, end: phys_addr_t) {
    /* Platform-specific memory-acceptance call goes here */
    if early_is_tdx_guest() {
        if !tdx_accept_memory(start, end) {
            panic!("TDX: Failed to accept memory\n");
        }
    } else if early_is_sevsnp_guest() {
        snp_accept_memory(start, end);
    } else {
        error("Cannot accept memory: unknown platform\n");
    }
}

pub unsafe fn init_unaccepted_memory() -> bool {
    let guid: guid_t = LINUX_EFI_UNACCEPTED_MEM_TABLE_GUID;
    let mut table: *mut efi_unaccepted_memory;
    let mut cfg_table_pa: c_ulong = 0;
    let mut cfg_table_len: c_uint = 0;
    let et: efi_type;
    let ret: c_int;

    et = efi_get_type(boot_params_ptr);
    if et == EFI_TYPE_NONE {
        return false;
    }

    ret = efi_get_conf_table(boot_params_ptr, &mut cfg_table_pa, &mut cfg_table_len);
    if ret != 0 {
        warn("EFI config table not found.");
        return false;
    }

    table = efi_find_vendor_table(
        boot_params_ptr,
        cfg_table_pa,
        cfg_table_len,
        guid,
    ) as *mut efi_unaccepted_memory;
    if table.is_null() {
        return false;
    }

    if (*table).version != 1 {
        error("Unknown version of unaccepted memory table\n");
    }

    /*
     * In many cases unaccepted_table is already set by EFI stub, but it
     * has to be initialized again to cover cases when the table is not
     * allocated by EFI stub or EFI stub copied the kernel image with
     * efi_relocate_kernel() before the variable is set.
     *
     * It must be initialized before the first usage of accept_memory().
     */
    unaccepted_table = table;

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
