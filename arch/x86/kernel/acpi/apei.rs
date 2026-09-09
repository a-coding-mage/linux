// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Arch-specific APEI-related functions.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

extern "C" {
    fn mce_save_apei_thr_limit(error_threshold_value: u32);
    fn mce_disable_bank(bank_number: u8);
    fn apei_mce_report_mem_error(sev: i32, mem_err: *mut cper_sec_mem_err);
    fn apei_smca_report_x86_error(ctx_info: *mut cper_ia_proc_ctx, lapic_id: u64) -> i32;
}

pub unsafe fn arch_apei_enable_cmcff(
    hest_hdr: *mut acpi_hest_header,
    _data: *mut core::ffi::c_void,
) -> i32 {
    // CONFIG_X86_MCE conditional from the original source.
    let cmc = hest_hdr as *mut acpi_hest_ia_corrected;
    if (*cmc).enabled == 0 {
        return 0;
    }

    mce_save_apei_thr_limit((*cmc).notify.error_threshold_value);

    /*
     * We expect HEST to provide a list of MC banks that report errors
     * in firmware first mode. Otherwise, return non-zero value to
     * indicate that we are done parsing HEST.
     */
    if ((*cmc).flags & ACPI_HEST_FIRMWARE_FIRST) == 0
        || (*cmc).num_hardware_banks == 0
    {
        return 1;
    }

    pr_info!("HEST: Enabling Firmware First mode for corrected errors.\n");

    let mut mc_bank = (cmc.add(1)) as *mut acpi_hest_ia_error_bank;
    let mut i = 0;
    while i < (*cmc).num_hardware_banks {
        mce_disable_bank((*mc_bank).bank_number);
        i += 1;
        mc_bank = mc_bank.add(1);
    }
    1
}

pub unsafe fn arch_apei_report_mem_error(
    sev: i32,
    mem_err: *mut cper_sec_mem_err,
) {
    // CONFIG_X86_MCE conditional from the original source.
    apei_mce_report_mem_error(sev, mem_err);
}

pub unsafe fn arch_apei_report_x86_error(
    ctx_info: *mut cper_ia_proc_ctx,
    lapic_id: u64,
) -> i32 {
    apei_smca_report_x86_error(ctx_info, lapic_id)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
