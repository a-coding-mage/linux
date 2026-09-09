/*
 * Copyright (C) 2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2009 PetaLogix
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Linux kernel dependencies supplied by other translation units.

pub unsafe fn machine_shutdown() {
    pr_notice!("Machine shutdown...\n");
    loop {}
}

pub unsafe fn machine_halt() {
    pr_notice!("Machine halt...\n");
    loop {}
}

pub unsafe fn machine_power_off() {
    pr_notice!("Machine power off...\n");
    loop {}
}

pub unsafe fn machine_restart(cmd: *mut core::ffi::c_char) {
    do_kernel_restart(cmd);
    /* Give the restart hook 1 s to take us down */
    mdelay(1000);
    pr_emerg!("Reboot failed -- System halted\n");
    loop {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
