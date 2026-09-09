// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Declarations supplied by the surrounding kernel environment are intentionally
// left external, corresponding to the C includes and configuration symbols.

pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    // CONFIG_SMP
    preempt_disable();
    smp_send_stop();

    local_irq_disable();
    clear_csr_ecfg(ECFG0_IM);

    pr_notice("\n\n** You can safely turn off the power now **\n\n");
    console_flush_on_panic(CONSOLE_FLUSH_PENDING);

    loop {
        core::arch::asm!("idle 0", options(nostack, preserves_flags));
    }
}

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    // CONFIG_SMP
    preempt_disable();
    smp_send_stop();

    // CONFIG_PM
    if !acpi_disabled {
        enable_pci_wakeup();
    }
    do_kernel_power_off();

    // CONFIG_EFI
    efi_reset_system(EFI_RESET_SHUTDOWN, EFI_SUCCESS, 0, core::ptr::null_mut());

    loop {
        core::arch::asm!("idle 0", options(nostack, preserves_flags));
    }
}

#[no_mangle]
pub unsafe extern "C" fn machine_restart(command: *mut core::ffi::c_char) {
    // CONFIG_SMP
    preempt_disable();
    smp_send_stop();

    do_kernel_restart(command);

    // CONFIG_EFI
    if efi_capsule_pending(core::ptr::null_mut()) {
        efi_reboot(REBOOT_WARM, core::ptr::null_mut());
    } else {
        efi_reboot(REBOOT_COLD, core::ptr::null_mut());
    }

    if !acpi_disabled {
        acpi_reboot();
    }

    loop {
        core::arch::asm!("idle 0", options(nostack, preserves_flags));
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
