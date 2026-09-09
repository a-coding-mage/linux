// SPDX-License-Identifier: GPL-2.0-only
/*
 * Default generic APIC driver. This handles up to 8 CPUs.
 *
 * Copyright 2003 Andi Kleen, SuSE Labs.
 *
 * Generic x86 APIC driver probe layer.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

unsafe fn default_get_apic_id(x: u32) -> u32 {
    let ver: u32 = GET_APIC_VERSION(apic_read(APIC_LVR));

    if APIC_XAPIC(ver) || boot_cpu_has(X86_FEATURE_EXTD_APICID) {
        (x >> 24) & 0xFF
    } else {
        (x >> 24) & 0x0F
    }
}

/* should be called last. */
unsafe fn probe_default() -> i32 {
    1
}

static mut apic_default: apic = apic {
    name: "default",
    probe: Some(probe_default),

    dest_mode_logical: true,

    disable_esr: 0,

    init_apic_ldr: Some(default_init_apic_ldr),
    cpu_present_to_apicid: Some(default_cpu_present_to_apicid),

    max_apic_id: 0xFE,
    get_apic_id: Some(default_get_apic_id),

    calc_dest_apicid: Some(apic_flat_calc_apicid),

    send_IPI: Some(default_send_IPI_single),
    send_IPI_mask: Some(default_send_IPI_mask_logical),
    send_IPI_mask_allbutself: Some(default_send_IPI_mask_allbutself_logical),
    send_IPI_allbutself: Some(default_send_IPI_allbutself),
    send_IPI_all: Some(default_send_IPI_all),
    send_IPI_self: Some(default_send_IPI_self),

    read: Some(native_apic_mem_read),
    write: Some(native_apic_mem_write),
    eoi: Some(native_apic_mem_eoi),
    icr_read: Some(native_apic_icr_read),
    icr_write: Some(native_apic_icr_write),
    wait_icr_idle: Some(apic_mem_wait_icr_idle),
    safe_wait_icr_idle: Some(apic_mem_wait_icr_idle_timeout),
};

// apic_driver(apic_default);

static mut apic: *mut apic = unsafe { &raw mut apic_default };
// EXPORT_SYMBOL_GPL(apic);

static mut cmdline_apic: i32 = 0;
unsafe fn parse_apic(arg: *mut core::ffi::c_char) -> i32 {
    let mut drv: *mut *mut apic;

    if arg.is_null() {
        return -EINVAL;
    }

    drv = __apicdrivers;
    while drv < __apicdrivers_end {
        if strcmp((*drv).as_ref().unwrap().name, arg) == 0 {
            apic_install_driver(*drv);
            cmdline_apic = 1;
            return 0;
        }
        drv = drv.add(1);
    }

    /* Parsed again by __setup for debug/verbose */
    0
}

// early_param("apic", parse_apic);

unsafe fn x86_32_probe_apic() {
    if cmdline_apic == 0 {
        let mut drv: *mut *mut apic;

        drv = __apicdrivers;
        while drv < __apicdrivers_end {
            if ((*drv).as_ref().unwrap().probe.unwrap())() != 0 {
                apic_install_driver(*drv);
                break;
            }
            drv = drv.add(1);
        }
        /* Not visible without early console */
        if drv == __apicdrivers_end {
            panic!("Didn't find an APIC driver");
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
