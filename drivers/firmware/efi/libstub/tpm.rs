// SPDX-License-Identifier: GPL-2.0
/*
 * TPM handling.
 *
 * Copyright (C) 2016 CoreOS, Inc
 * Copyright (C) 2017 Google, Inc.
 *     Matthew Garrett <mjg59@google.com>
 *     Thiebaud Weksteen <tweek@google.com>
 */

// Dependencies supplied by the surrounding EFI stub and kernel bindings.

#[cfg(CONFIG_RESET_ATTACK_MITIGATION)]
static EFI_MEMORY_OVERWRITE_REQUEST_NAME: [efi_char16_t; 30] = [
    'M' as efi_char16_t, 'e' as efi_char16_t, 'm' as efi_char16_t,
    'o' as efi_char16_t, 'r' as efi_char16_t, 'y' as efi_char16_t,
    'O' as efi_char16_t, 'v' as efi_char16_t, 'e' as efi_char16_t,
    'r' as efi_char16_t, 'w' as efi_char16_t, 'r' as efi_char16_t,
    'i' as efi_char16_t, 't' as efi_char16_t, 'e' as efi_char16_t,
    'R' as efi_char16_t, 'e' as efi_char16_t, 'q' as efi_char16_t,
    'u' as efi_char16_t, 'e' as efi_char16_t, 's' as efi_char16_t,
    't' as efi_char16_t, 'C' as efi_char16_t, 'o' as efi_char16_t,
    'n' as efi_char16_t, 't' as efi_char16_t, 'r' as efi_char16_t,
    'o' as efi_char16_t, 'l' as efi_char16_t, 0,
];

#[cfg(CONFIG_RESET_ATTACK_MITIGATION)]
pub unsafe fn efi_enable_reset_attack_mitigation() {
    let mut val: u8 = 1;
    let mut var_guid: efi_guid_t = MEMORY_ONLY_RESET_CONTROL_GUID;
    let status: efi_status_t;
    let mut datasize: c_ulong = 0;

    status = get_efi_var(
        EFI_MEMORY_OVERWRITE_REQUEST_NAME.as_ptr(),
        &mut var_guid,
        core::ptr::null_mut(),
        &mut datasize,
        core::ptr::null_mut(),
    );

    if status == EFI_NOT_FOUND {
        return;
    }

    set_efi_var(
        EFI_MEMORY_OVERWRITE_REQUEST_NAME.as_ptr(),
        &mut var_guid,
        EFI_VARIABLE_NON_VOLATILE
            | EFI_VARIABLE_BOOTSERVICE_ACCESS
            | EFI_VARIABLE_RUNTIME_ACCESS,
        core::mem::size_of_val(&val),
        &mut val,
    );
}

unsafe fn efi_retrieve_tcg2_eventlog(
    version: c_int,
    log_location: efi_physical_addr_t,
    log_last_entry: efi_physical_addr_t,
    truncated: efi_bool_t,
    final_events_table: *mut efi_tcg2_final_events_table,
) {
    let mut linux_eventlog_guid: efi_guid_t = LINUX_EFI_TPM_EVENT_LOG_GUID;
    let status: efi_status_t;
    let mut log_tbl: *mut linux_efi_tpm_eventlog = core::ptr::null_mut();
    let first_entry_addr: c_ulong = log_location as c_ulong;
    let mut last_entry_addr: c_ulong;
    let log_size: usize;
    let last_entry_size: usize;
    let mut final_events_size: u32 = 0;

    let _ = truncated;
    if log_last_entry == 0 {
        log_size = 0;
    } else {
        last_entry_addr = log_last_entry as c_ulong;
        last_entry_size = if version > EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2 {
            __calc_tpm2_event_size(
                last_entry_addr as *mut c_void,
                log_location as *mut c_void,
                false,
            ) as usize
        } else {
            core::mem::size_of::<tcpa_event>()
                + (*(last_entry_addr as *mut tcpa_event)).event_size as usize
        };
        log_size = (log_last_entry - log_location) as usize + last_entry_size;
    }

    status = efi_bs_call!(allocate_pool, EFI_ACPI_RECLAIM_MEMORY,
        core::mem::size_of::<linux_efi_tpm_eventlog>() + log_size,
        &mut log_tbl as *mut _ as *mut c_void);
    if status != EFI_SUCCESS {
        efi_err!("Unable to allocate memory for event log\n");
        return;
    }

    if !final_events_table.is_null() && (*final_events_table).nr_events != 0 {
        let mut offset = (core::mem::size_of_val(&(*final_events_table).version)
            + core::mem::size_of_val(&(*final_events_table).nr_events)) as u32;
        let mut i = (*final_events_table).nr_events;
        let data = final_events_table as *mut c_void;
        while i > 0 {
            let header = (data as *mut u8).add((offset + final_events_size) as usize)
                as *mut tcg_pcr_event2_head;
            let event_size = __calc_tpm2_event_size(header, log_location as *mut c_void, false);
            if event_size == 0 { break; }
            final_events_size += event_size;
            i -= 1;
        }
    }

    core::ptr::write_bytes(log_tbl as *mut u8, 0,
        core::mem::size_of::<linux_efi_tpm_eventlog>() + log_size);
    (*log_tbl).size = log_size;
    (*log_tbl).final_events_preboot_size = final_events_size;
    (*log_tbl).version = version;
    core::ptr::copy_nonoverlapping(first_entry_addr as *const u8,
        (*log_tbl).log.as_mut_ptr() as *mut u8, log_size);

    status = efi_bs_call!(install_configuration_table, &mut linux_eventlog_guid, log_tbl);
    if status != EFI_SUCCESS {
        efi_bs_call!(free_pool, log_tbl);
    }
}

pub unsafe fn efi_retrieve_eventlog() {
    let mut final_events_table: *mut efi_tcg2_final_events_table = core::ptr::null_mut();
    let mut log_location: efi_physical_addr_t = 0;
    let mut log_last_entry: efi_physical_addr_t = 0;
    let mut tpm2_guid: efi_guid_t = EFI_TCG2_PROTOCOL_GUID;
    let mut version = EFI_TCG2_EVENT_LOG_FORMAT_TCG_2;
    let mut tpm2: *mut efi_tcg2_protocol_t = core::ptr::null_mut();
    let mut truncated: efi_bool_t = 0;
    let mut status = efi_bs_call!(locate_protocol, &mut tpm2_guid, core::ptr::null_mut(), &mut tpm2 as *mut _ as *mut c_void);

    if status == EFI_SUCCESS {
        status = efi_call_proto!(tpm2, get_event_log, version, &mut log_location, &mut log_last_entry, &mut truncated);
        if status != EFI_SUCCESS || log_location == 0 {
            version = EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2;
            status = efi_call_proto!(tpm2, get_event_log, version, &mut log_location, &mut log_last_entry, &mut truncated);
        } else {
            final_events_table = get_efi_config_table(EFI_TCG2_FINAL_EVENTS_TABLE_GUID);
        }
    } else {
        let mut cc_guid: efi_guid_t = EFI_CC_MEASUREMENT_PROTOCOL_GUID;
        let mut cc: *mut efi_cc_protocol_t = core::ptr::null_mut();
        status = efi_bs_call!(locate_protocol, &mut cc_guid, core::ptr::null_mut(), &mut cc as *mut _ as *mut c_void);
        if status != EFI_SUCCESS { return; }
        version = EFI_CC_EVENT_LOG_FORMAT_TCG_2;
        status = efi_call_proto!(cc, get_event_log, version, &mut log_location, &mut log_last_entry, &mut truncated);
        final_events_table = get_efi_config_table(EFI_CC_FINAL_EVENTS_TABLE_GUID);
    }
    if status != EFI_SUCCESS || log_location == 0 { return; }
    efi_retrieve_tcg2_eventlog(version, log_location, log_last_entry, truncated, final_events_table);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
