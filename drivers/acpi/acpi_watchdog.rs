// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI watchdog table parsing support.
 *
 * Copyright (C) 2016, Intel Corporation
 * Author: Mika Westerberg <mika.westerberg@linux.intel.com>
 */

// #define pr_fmt(fmt) "ACPI: watchdog: " fmt
// Dependencies supplied by the Linux ACPI, I/O resource, platform-device,
// and internal headers are intentionally referenced but not implemented here.

#[cfg(CONFIG_RTC_MC146818_LIB)]
static unsafe fn acpi_watchdog_uses_rtc(wdat: *const acpi_table_wdat) -> bool {
    /*
     * There are several systems where the WDAT table is accessing RTC SRAM to
     * store persistent information. This does not work well with the Linux RTC
     * driver so on those systems we skip WDAT driver and prefer iTCO_wdt
     * instead.
     *
     * See also https://bugzilla.kernel.org/show_bug.cgi?id=199033.
     */
    let entries = (wdat.add(1)) as *const acpi_wdat_entry;
    let mut i = 0;
    while i < (*wdat).entries {
        let gas = &(*entries.add(i as usize)).register_region;
        if gas.space_id == ACPI_ADR_SPACE_SYSTEM_IO {
            match gas.address {
                RTC_PORT(0) | RTC_PORT(1) | RTC_PORT(2) | RTC_PORT(3) => {
                    return true;
                }
                _ => {}
            }
        }
        i += 1;
    }

    false
}

#[cfg(not(CONFIG_RTC_MC146818_LIB))]
static unsafe fn acpi_watchdog_uses_rtc(_wdat: *const acpi_table_wdat) -> bool {
    false
}

static mut acpi_no_watchdog: bool = false;

static unsafe fn acpi_watchdog_get_wdat() -> *const acpi_table_wdat {
    let mut wdat: *const acpi_table_wdat = core::ptr::null();
    let status: acpi_status;

    if acpi_disabled || acpi_no_watchdog {
        return core::ptr::null();
    }

    status = acpi_get_table(
        ACPI_SIG_WDAT,
        0,
        (&mut wdat as *mut *const acpi_table_wdat).cast::<*mut acpi_table_header>(),
    );
    if ACPI_FAILURE(status) {
        /* It is fine if there is no WDAT */
        return core::ptr::null();
    }

    if acpi_watchdog_uses_rtc(wdat) {
        acpi_put_table(wdat as *const acpi_table_header);
        pr_info!("Skipping WDAT on this system because it uses RTC SRAM\n");
        return core::ptr::null();
    }

    wdat
}

/*
 * Returns true if this system should prefer ACPI based watchdog instead of
 * the native one (which are typically the same hardware).
 */
pub unsafe fn acpi_has_watchdog() -> bool {
    !acpi_watchdog_get_wdat().is_null()
}

// EXPORT_SYMBOL_GPL(acpi_has_watchdog);

/* ACPI watchdog can be disabled on boot command line */
unsafe fn disable_acpi_watchdog(_str: *mut core::ffi::c_char) -> i32 {
    acpi_no_watchdog = true;
    1
}

// __setup("acpi_no_watchdog", disable_acpi_watchdog);

pub unsafe fn acpi_watchdog_init() {
    let mut resource_list = LIST_HEAD!();
    let mut wdat: *const acpi_table_wdat;
    let mut nresources: usize = 0;

    wdat = acpi_watchdog_get_wdat();
    if wdat.is_null() {
        /* It is fine if there is no WDAT */
        return;
    }

    /* Watchdog disabled by BIOS */
    if (*wdat).flags & ACPI_WDAT_ENABLED == 0 {
        goto_fail_put_wdat!(wdat);
    }

    /* Skip legacy PCI WDT devices */
    if (*wdat).pci_segment != 0xff
        || (*wdat).pci_bus != 0xff
        || (*wdat).pci_device != 0xff
        || (*wdat).pci_function != 0xff
    {
        goto_fail_put_wdat!(wdat);
    }

    let entries = (wdat.add(1)) as *const acpi_wdat_entry;
    let mut i = 0;
    while i < (*wdat).entries {
        let gas = &(*entries.add(i as usize)).register_region;
        let mut res = resource::default();
        res.start = gas.address;
        res.end = res.start + ACPI_ACCESS_BYTE_WIDTH(gas.access_width) - 1;
        if gas.space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY {
            res.flags = IORESOURCE_MEM;
        } else if gas.space_id == ACPI_ADR_SPACE_SYSTEM_IO {
            res.flags = IORESOURCE_IO;
        } else {
            pr_warn!("Unsupported address space: {}\n", gas.space_id);
            goto_fail_free_resource_list!(wdat, resource_list);
        }

        let mut found = false;
        resource_list_for_each_entry!(rentry, &resource_list, {
            if (*rentry).res.flags == res.flags
                && resource_union((*rentry).res, &res, (*rentry).res)
            {
                found = true;
                break;
            }
        });

        if !found {
            let rentry = resource_list_create_entry(core::ptr::null_mut(), 0);
            if rentry.is_null() {
                goto_fail_free_resource_list!(wdat, resource_list);
            }
            *(*rentry).res = res;
            resource_list_add_tail(rentry, &mut resource_list);
            nresources += 1;
        }
        i += 1;
    }

    let resources = kzalloc_objs::<resource>(nresources);
    if resources.is_null() {
        goto_fail_free_resource_list!(wdat, resource_list);
    }

    i = 0;
    resource_list_for_each_entry!(rentry, &resource_list, {
        *resources.add(i as usize) = *(*rentry).res;
        i += 1;
    });

    let pdev = platform_device_register_simple(
        c"wdat_wdt".as_ptr(),
        PLATFORM_DEVID_NONE,
        resources,
        nresources,
    );
    if IS_ERR(pdev) {
        pr_err!("Device creation failed: %pe\n", pdev);
    }

    kfree(resources);
    resource_list_free(&mut resource_list);
    acpi_put_table(wdat as *const acpi_table_header);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
