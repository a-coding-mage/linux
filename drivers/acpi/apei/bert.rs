// SPDX-License-Identifier: GPL-2.0-only
/*
 * APEI Boot Error Record Table (BERT) support
 *
 * Copyright 2011 Intel Corp.
 *   Author: Huang Ying <ying.huang@intel.com>
 *
 * Under normal circumstances, when a hardware error occurs, the error
 * handler receives control and processes the error. This gives OSPM a
 * chance to process the error condition, report it, and optionally attempt
 * recovery. In some cases, the system is unable to process an error.
 * For example, system firmware or a management controller may choose to
 * reset the system or the system might experience an uncontrolled crash
 * or reset.The boot error source is used to report unhandled errors that
 * occurred in a previous boot. This mechanism is described in the BERT
 * table.
 *
 * For more information about BERT, please refer to ACPI Specification
 * version 4.0, section 17.3.1
 */

// Dependencies supplied by the surrounding kernel translation unit.

const ACPI_BERT_PRINT_MAX_RECORDS: i32 = 5;
const ACPI_BERT_PRINT_MAX_LEN: u32 = 1024;

static mut bert_disable: i32 = 0;

/*
 * Print "all" the error records in the BERT table, but avoid huge spam to
 * the console if the BIOS included oversize records, or too many records.
 * Skipping some records here does not lose anything because the full
 * data is available to user tools in:
 *	/sys/firmware/acpi/tables/data/BERT
 */
unsafe fn bert_print_all(
    mut region: *mut acpi_bert_region,
    region_len: u32,
) {
    let mut estatus = region as *mut acpi_hest_generic_status;
    let mut remain: i32 = region_len as i32;
    let mut printed: i32 = 0;
    let mut skipped: i32 = 0;
    let mut estatus_len: u32;

    while remain >= core::mem::size_of::<acpi_bert_region>() as i32 {
        estatus_len = cper_estatus_len(estatus);
        if remain < estatus_len as i32 {
            pr_err!(FW_BUG, "Truncated status block (length: {}).\n", estatus_len);
            break;
        }

        /* No more error records. */
        if (*estatus).block_status == 0 {
            break;
        }

        if cper_estatus_check(estatus) != 0 {
            pr_err!(FW_BUG, "Invalid error record.\n");
            break;
        }

        if estatus_len < ACPI_BERT_PRINT_MAX_LEN
            && printed < ACPI_BERT_PRINT_MAX_RECORDS
        {
            pr_info_once!("Error records from previous boot:\n");
            cper_estatus_print(KERN_INFO, HW_ERR, estatus);
            printed += 1;
        } else {
            skipped += 1;
        }

        /*
         * Because the boot error source is "one-time polled" type,
         * clear Block Status of current Generic Error Status Block,
         * once it's printed.
         */
        (*estatus).block_status = 0;

        estatus = (estatus as *mut u8).add(estatus_len as usize)
            as *mut acpi_hest_generic_status;
        remain -= estatus_len as i32;
    }

    if skipped != 0 {
        pr_info!(HW_ERR, "Skipped {} error records\n", skipped);
    }

    if printed + skipped != 0 {
        pr_info!("Total records found: {}\n", printed + skipped);
    }
}

unsafe fn setup_bert_disable(_str: *mut i8) -> i32 {
    bert_disable = 1;
    1
}

__setup!("bert_disable", setup_bert_disable);

unsafe fn bert_check_table(bert_tab: *mut acpi_table_bert) -> i32 {
    if (*bert_tab).header.length < core::mem::size_of::<acpi_table_bert>() as u32
        || (*bert_tab).region_length < core::mem::size_of::<acpi_bert_region>() as u32
    {
        return -EINVAL;
    }

    0
}

unsafe fn bert_init() -> i32 {
    let mut bert_resources: apei_resources;
    let mut boot_error_region: *mut acpi_bert_region;
    let mut bert_tab: *mut acpi_table_bert = core::ptr::null_mut();
    let mut region_len: u32;
    let mut status: acpi_status;
    let mut rc: i32 = 0;

    if acpi_disabled {
        return 0;
    }

    if bert_disable != 0 {
        pr_info!("Boot Error Record Table support is disabled.\n");
        return 0;
    }

    status = acpi_get_table(
        ACPI_SIG_BERT,
        0,
        &mut bert_tab as *mut *mut acpi_table_bert as *mut *mut acpi_table_header,
    );
    if status == AE_NOT_FOUND {
        return 0;
    }

    if ACPI_FAILURE(status) {
        pr_err!("get table failed, {}.\n", acpi_format_exception(status));
        return -EINVAL;
    }

    rc = bert_check_table(bert_tab);
    if rc != 0 {
        pr_err!(FW_BUG, "table invalid.\n");
        goto_out_put_bert_tab!(bert_tab, rc);
    }

    region_len = (*bert_tab).region_length;
    apei_resources_init(&mut bert_resources);
    rc = apei_resources_add(
        &mut bert_resources,
        (*bert_tab).address,
        region_len,
        true,
    );
    if rc != 0 {
        goto_out_put_bert_tab!(bert_tab, rc);
    }
    rc = apei_resources_request(&mut bert_resources, "APEI BERT\0".as_ptr() as *const i8);
    if rc != 0 {
        goto_out_fini!(bert_resources, bert_tab, rc);
    }
    boot_error_region = ioremap_cache((*bert_tab).address, region_len);
    if !boot_error_region.is_null() {
        bert_print_all(boot_error_region, region_len);
        iounmap(boot_error_region as *mut core::ffi::c_void);
    } else {
        rc = -ENOMEM;
    }

    apei_resources_release(&mut bert_resources);
    apei_resources_fini(&mut bert_resources);
    acpi_put_table(bert_tab as *mut acpi_table_header);
    return rc;
}

late_initcall!(bert_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
