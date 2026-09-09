// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2004 James Cleverdon, IBM.
 *
 * Flat APIC subarch code.
 *
 * Hacked for x86-64 by James Cleverdon from i386 architecture code by
 * Martin Bligh, Andi Kleen, James Bottomley, John Stultz, and
 * James Cleverdon.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/apic.h and local.h.

unsafe fn physflat_get_apic_id(x: u32) -> u32 {
    (x >> 24) & 0xFF
}

unsafe fn physflat_probe() -> i32 {
    1
}

unsafe fn physflat_acpi_madt_oem_check(_oem_id: *mut core::ffi::c_char,
                                       _oem_table_id: *mut core::ffi::c_char) -> i32 {
    1
}

static mut apic_physflat: apic = apic {
    name: b"physical flat\0".as_ptr() as *const core::ffi::c_char,
    probe: Some(physflat_probe),
    acpi_madt_oem_check: Some(physflat_acpi_madt_oem_check),

    dest_mode_logical: false,

    disable_esr: 0,

    cpu_present_to_apicid: Some(default_cpu_present_to_apicid),

    max_apic_id: 0xFE,
    get_apic_id: Some(physflat_get_apic_id),

    calc_dest_apicid: Some(apic_default_calc_apicid),

    send_IPI: Some(default_send_IPI_single_phys),
    send_IPI_mask: Some(default_send_IPI_mask_sequence_phys),
    send_IPI_mask_allbutself: Some(default_send_IPI_mask_allbutself_phys),
    send_IPI_allbutself: Some(default_send_IPI_allbutself),
    send_IPI_all: Some(default_send_IPI_all),
    send_IPI_self: Some(default_send_IPI_self),
    nmi_to_offline_cpu: true,

    read: Some(native_apic_mem_read),
    write: Some(native_apic_mem_write),
    eoi: Some(native_apic_mem_eoi),
    icr_read: Some(native_apic_icr_read),
    icr_write: Some(native_apic_icr_write),
    wait_icr_idle: Some(apic_mem_wait_icr_idle),
    safe_wait_icr_idle: Some(apic_mem_wait_icr_idle_timeout),
};

// C registration macro: apic_driver(apic_physflat);

pub static mut apic: *mut apic = unsafe { &raw mut apic_physflat };

// EXPORT_SYMBOL_GPL(apic);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
