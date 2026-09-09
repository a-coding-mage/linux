/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Reset an IP27.
 *
 * Copyright (C) 1997, 1998, 1999, 2000, 06 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

// Dependencies supplied by the Linux and SGI/IP27 environments are external.

extern "C" {
    pub fn machine_restart(command: *mut core::ffi::c_char) -> !;
    pub fn machine_halt() -> !;
    pub fn machine_power_off() -> !;

    static mut _machine_restart: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
}

#[inline(never)]
unsafe fn ip27_machine_restart(command: *mut core::ffi::c_char) -> ! {
    let _ = command;

    // XXX How to pass the reboot command to the firmware???
    unsafe {
        printk!("Reboot started from CPU %d\n", smp_processor_id());
    }

    // CONFIG_SMP conditional from the C source.
    #[cfg(feature = "CONFIG_SMP")]
    unsafe {
        smp_send_stop();
    }

    // The disabled CONFIG_SMP-independent PROM reboot loop is preserved by
    // the active local reset operation below.
    unsafe {
        LOCAL_HUB_S!(NI_PORT_RESET, NPR_PORTRESET | NPR_LOCALRESET);
    }
    loop {}
}

unsafe fn ip27_machine_halt() -> ! {
    let mut i: i32;

    // CONFIG_SMP conditional from the C source.
    #[cfg(feature = "CONFIG_SMP")]
    {
        smp_send_stop();
    }

    for_each_online_node!(i, {
        REMOTE_HUB_S!(i, PROMOP_REG, PROMOP_RESTART);
    });
    LOCAL_HUB_S!(NI_PORT_RESET, NPR_PORTRESET | NPR_LOCALRESET);
    loop {}
}

unsafe fn ip27_machine_power_off() -> ! {
    /* To do ...  */
    loop {}
}

pub unsafe fn ip27_reboot_setup() {
    _machine_restart = Some(ip27_machine_restart);
    _machine_halt = Some(ip27_machine_halt);
    pm_power_off = Some(ip27_machine_power_off);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
