// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Google, Inc.
 *     Thiebaud Weksteen <tweek@google.com>
 */

// #include <asm/early_ioremap.h>
// #include <linux/efi.h>
// #include <linux/init.h>
// #include <linux/memblock.h>
// #include <linux/tpm_eventlog.h>

// #define TPM_MEMREMAP(start, size) early_memremap(start, size)
// #define TPM_MEMUNMAP(start, size) early_memunmap(start, size)

pub static mut efi_tpm_final_log_size: i32 = 0;
// EXPORT_SYMBOL(efi_tpm_final_log_size);

unsafe fn tpm2_calc_event_log_size(
    mut data: *mut core::ffi::c_void,
    mut count: i32,
    size_info: *mut core::ffi::c_void,
) -> i32 {
    let mut size: u32 = 0;

    while count > 0 {
        let header = data.add(size as usize) as *mut tcg_pcr_event2_head;
        let event_size = __calc_tpm2_event_size(header, size_info, true);
        if event_size == 0 {
            return -1;
        }
        size = size.wrapping_add(event_size);
        count -= 1;
    }

    size as i32
}

/*
 * Reserve the memory associated with the TPM Event Log configuration table.
 */
pub unsafe fn efi_tpm_eventlog_init() -> i32 {
    let mut ret: i32 = 0;

    if efi.tpm_log == EFI_INVALID_TABLE_ADDR {
        /*
         * We can't calculate the size of the final events without the
         * first entry in the TPM log, so bail here.
         */
        return 0;
    }

    let log_tbl = early_memremap(
        efi.tpm_log,
        core::mem::size_of::<linux_efi_tpm_eventlog>(),
    ) as *mut linux_efi_tpm_eventlog;
    if log_tbl.is_null() {
        pr_err!("Failed to map TPM Event Log table @ 0x%lx\n", efi.tpm_log);
        efi.tpm_log = EFI_INVALID_TABLE_ADDR;
        return -ENOMEM;
    }

    let tbl_size = core::mem::size_of::<linux_efi_tpm_eventlog>() + (*log_tbl).size as usize;
    if memblock_reserve(efi.tpm_log, tbl_size) != 0 {
        pr_err!(
            "TPM Event Log memblock reserve fails (0x%lx, 0x%x)\n",
            efi.tpm_log,
            tbl_size
        );
        ret = -ENOMEM;
        early_memunmap(log_tbl as *mut core::ffi::c_void, core::mem::size_of::<linux_efi_tpm_eventlog>());
        return ret;
    }

    let final_tbl = if efi.tpm_final_log == EFI_INVALID_TABLE_ADDR {
        pr_info!("TPM Final Events table not present\n");
        core::ptr::null_mut()
    } else if (*log_tbl).version != EFI_TCG2_EVENT_LOG_FORMAT_TCG_2 {
        pr_warn!("FW_BUG TPM Final Events table invalid\n");
        core::ptr::null_mut()
    } else {
        early_memremap(
            efi.tpm_final_log,
            core::mem::size_of::<efi_tcg2_final_events_table>(),
        ) as *mut efi_tcg2_final_events_table
    };

    if final_tbl.is_null() {
        if efi.tpm_final_log == EFI_INVALID_TABLE_ADDR
            || (*log_tbl).version != EFI_TCG2_EVENT_LOG_FORMAT_TCG_2
        {
            early_memunmap(log_tbl as *mut core::ffi::c_void, core::mem::size_of::<linux_efi_tpm_eventlog>());
            return ret;
        }
        pr_err!(
            "Failed to map TPM Final Event Log table @ 0x%lx\n",
            efi.tpm_final_log
        );
        efi.tpm_final_log = EFI_INVALID_TABLE_ADDR;
        ret = -ENOMEM;
        early_memunmap(log_tbl as *mut core::ffi::c_void, core::mem::size_of::<linux_efi_tpm_eventlog>());
        return ret;
    }

    let mut final_tbl_size: i32 = 0;
    if (*final_tbl).nr_events != 0 {
        let events = (efi.tpm_final_log as usize
            + core::mem::size_of_val(&(*final_tbl).version)
            + core::mem::size_of_val(&(*final_tbl).nr_events)) as *mut core::ffi::c_void;
        final_tbl_size = tpm2_calc_event_log_size(events, (*final_tbl).nr_events as i32, (*log_tbl).log as *mut core::ffi::c_void);
    }

    if final_tbl_size < 0 {
        pr_err!("FW_BUG Failed to parse event in TPM Final Events Log\n");
        ret = -EINVAL;
    } else {
        memblock_reserve(efi.tpm_final_log, final_tbl_size as usize + core::mem::size_of::<efi_tcg2_final_events_table>());
        efi_tpm_final_log_size = final_tbl_size;
    }

    early_memunmap(final_tbl as *mut core::ffi::c_void, core::mem::size_of::<efi_tcg2_final_events_table>());
    early_memunmap(log_tbl as *mut core::ffi::c_void, core::mem::size_of::<linux_efi_tpm_eventlog>());
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
