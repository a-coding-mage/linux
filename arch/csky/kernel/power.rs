// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependency supplied by the surrounding kernel translation.

extern "C" {
    fn local_irq_disable();
    fn do_kernel_power_off();
    fn do_kernel_restart(cmd: *mut core::ffi::c_char);
}

pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;
// EXPORT_SYMBOL(pm_power_off);

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    local_irq_disable();
    do_kernel_power_off();
    core::arch::asm!("bkpt");
}

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    local_irq_disable();
    do_kernel_power_off();
    core::arch::asm!("bkpt");
}

#[no_mangle]
pub unsafe extern "C" fn machine_restart(cmd: *mut core::ffi::c_char) {
    local_irq_disable();
    do_kernel_restart(cmd);
    core::arch::asm!("bkpt");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
