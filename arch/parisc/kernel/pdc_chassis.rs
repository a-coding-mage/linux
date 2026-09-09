// SPDX-License-Identifier: GPL-2.0-only
/*
 * interfaces to Chassis Codes via PDC (firmware)
 *
 * Copyright (C) 2002 Laurent Canet <canetl@esiee.fr>
 * Copyright (C) 2002-2006 Thibaut VARENE <varenet@parisc-linux.org>
 *
 * TODO: poll chassis warns, trigger (configurable) machine shutdown when
 * needed. Find out how to get Chassis warnings out of PAT boxes?
 */

// PDC_CHASSIS_DEBUG is undefined in the original source.
// C includes supply the external kernel, PDC, processor, PAT, and LED symbols.

const PDC_CHASSIS_VER: &str = "0.05";

#[cfg(feature = "CONFIG_PDC_CHASSIS")]
static mut pdc_chassis_enabled: u32 = 1;

/// pdc_chassis_setup() - Enable/disable pdc_chassis code at boot time.
/// `str`: configuration param: 0 to disable chassis log.
/// Returns 1.
#[cfg(feature = "CONFIG_PDC_CHASSIS")]
unsafe fn pdc_chassis_setup(mut str_: *mut core::ffi::c_char) -> i32 {
    // panic_timeout = simple_strtoul(str, NULL, 0);
    get_option(&mut str_, &raw mut pdc_chassis_enabled);
    1
}

// __setup("pdcchassis=", pdc_chassis_setup);

/*
 * pdc_chassis_checkold() checked compatibility with old PDC_CHASSIS.
 * Currently, only E class and A180 were known to work with this.
 * The original implementation was compiled out with #if 0.
 */

/// pdc_chassis_panic_event() - Called by the panic handler.
#[cfg(feature = "CONFIG_PDC_CHASSIS")]
unsafe extern "C" fn pdc_chassis_panic_event(
    _this: *mut notifier_block,
    _event: u64,
    _ptr: *mut core::ffi::c_void,
) -> i32 {
    pdc_chassis_send_status(PDC_CHASSIS_DIRECT_PANIC);
    NOTIFY_DONE
}

#[cfg(feature = "CONFIG_PDC_CHASSIS")]
static mut pdc_chassis_panic_block: notifier_block = notifier_block {
    notifier_call: Some(pdc_chassis_panic_event),
    priority: INT_MAX,
};

/// pdc_chassis_reboot_event() - Called by the reboot handler.
#[cfg(feature = "CONFIG_PDC_CHASSIS")]
unsafe extern "C" fn pdc_chassis_reboot_event(
    _this: *mut notifier_block,
    _event: u64,
    _ptr: *mut core::ffi::c_void,
) -> i32 {
    pdc_chassis_send_status(PDC_CHASSIS_DIRECT_SHUTDOWN);
    NOTIFY_DONE
}

#[cfg(feature = "CONFIG_PDC_CHASSIS")]
static mut pdc_chassis_reboot_block: notifier_block = notifier_block {
    notifier_call: Some(pdc_chassis_reboot_event),
    priority: INT_MAX,
};

/// parisc_pdc_chassis_init() - Called at boot time.
pub unsafe extern "C" fn parisc_pdc_chassis_init() {
    #[cfg(feature = "CONFIG_PDC_CHASSIS")]
    if pdc_chassis_enabled != 0 {
        printk(
            KERN_INFO,
            "Enabling %s chassis codes support v%s\n",
            if is_pdc_pat() != 0 { "PDC_PAT" } else { "regular" },
            PDC_CHASSIS_VER,
        );

        atomic_notifier_chain_register(&raw mut panic_notifier_list, &raw mut pdc_chassis_panic_block);
        register_reboot_notifier(&raw mut pdc_chassis_reboot_block);
    }
}

/// pdc_chassis_send_status() - Sends a predefined message to the chassis,
/// and changes the front panel LEDs according to the new system state.
/// `message`: one of the PDC_CHASSIS_DIRECT_* values.
pub unsafe extern "C" fn pdc_chassis_send_status(message: i32) -> i32 {
    let mut retval: i32 = 0;

    #[cfg(feature = "CONFIG_PDC_CHASSIS")]
    if pdc_chassis_enabled != 0 {
        #[cfg(feature = "CONFIG_64BIT")]
        {
            if is_pdc_pat() != 0 {
                retval = match message {
                    PDC_CHASSIS_DIRECT_BSTART => pdc_pat_chassis_send_log(PDC_CHASSIS_PMSG_BSTART, PDC_CHASSIS_LSTATE_RUN_NORMAL),
                    PDC_CHASSIS_DIRECT_BCOMPLETE => pdc_pat_chassis_send_log(PDC_CHASSIS_PMSG_BCOMPLETE, PDC_CHASSIS_LSTATE_RUN_NORMAL),
                    PDC_CHASSIS_DIRECT_SHUTDOWN => pdc_pat_chassis_send_log(PDC_CHASSIS_PMSG_SHUTDOWN, PDC_CHASSIS_LSTATE_NONOS),
                    PDC_CHASSIS_DIRECT_PANIC => pdc_pat_chassis_send_log(PDC_CHASSIS_PMSG_PANIC, PDC_CHASSIS_LSTATE_RUN_CRASHREC),
                    PDC_CHASSIS_DIRECT_LPMC => pdc_pat_chassis_send_log(PDC_CHASSIS_PMSG_LPMC, PDC_CHASSIS_LSTATE_RUN_SYSINT),
                    PDC_CHASSIS_DIRECT_HPMC => pdc_pat_chassis_send_log(PDC_CHASSIS_PMSG_HPMC, PDC_CHASSIS_LSTATE_RUN_NCRIT),
                    _ => -1,
                };
            } else {
                retval = -1;
            }
        }

        #[cfg(not(feature = "CONFIG_64BIT"))]
        {
            retval = match message {
                PDC_CHASSIS_DIRECT_BSTART => pdc_chassis_disp(PDC_CHASSIS_DISP_DATA(OSTAT_INIT)),
                PDC_CHASSIS_DIRECT_BCOMPLETE => pdc_chassis_disp(PDC_CHASSIS_DISP_DATA(OSTAT_RUN)),
                PDC_CHASSIS_DIRECT_SHUTDOWN => pdc_chassis_disp(PDC_CHASSIS_DISP_DATA(OSTAT_SHUT)),
                PDC_CHASSIS_DIRECT_HPMC | PDC_CHASSIS_DIRECT_PANIC => pdc_chassis_disp(PDC_CHASSIS_DISP_DATA(OSTAT_FLT)),
                PDC_CHASSIS_DIRECT_LPMC => pdc_chassis_disp(PDC_CHASSIS_DISP_DATA(OSTAT_WARN)),
                _ => -1,
            };
        }
    }

    #[cfg(all(feature = "CONFIG_PDC_CHASSIS", feature = "CONFIG_CHASSIS_LCD_LED"))]
    if retval != -1 {
        lcd_print(core::ptr::null());
    }

    retval
}

#[cfg(all(feature = "CONFIG_PDC_CHASSIS_WARN", feature = "CONFIG_PROC_FS"))]
unsafe extern "C" fn pdc_chassis_warn_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let mut warn: u64 = 0;
    let warnreg: u32;

    if pdc_chassis_warn(&mut warn) != PDC_OK {
        return -EIO;
    }
    warnreg = warn as u32;

    if ((warnreg >> 24) & 0xff) != 0 {
        seq_printf(m, "Chassis component failure! (eg fan or PSU): 0x%.2x\n", (warnreg >> 24) & 0xff);
    }
    seq_printf(m, "Battery: %s\n", if warnreg & 0x04 != 0 { "Low!" } else { "OK" });
    seq_printf(m, "Temp low: %s\n", if warnreg & 0x02 != 0 { "Exceeded!" } else { "OK" });
    seq_printf(m, "Temp mid: %s\n", if warnreg & 0x01 != 0 { "Exceeded!" } else { "OK" });
    0
}

#[cfg(all(feature = "CONFIG_PDC_CHASSIS_WARN", feature = "CONFIG_PROC_FS"))]
unsafe extern "C" fn pdc_chassis_create_procfs() -> i32 {
    let mut test: u64 = 0;
    let ret = pdc_chassis_warn(&mut test);
    if ret == PDC_BAD_PROC || ret == PDC_BAD_OPTION {
        printk(KERN_INFO, "Chassis warnings not supported.\n");
        return 0;
    }
    printk(KERN_INFO, "Enabling PDC chassis warnings support v%s\n", PDC_CHASSIS_VER);
    proc_create_single("chassis", 0o400, core::ptr::null_mut(), Some(pdc_chassis_warn_show));
    0
}

// __initcall(pdc_chassis_create_procfs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
