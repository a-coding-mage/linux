// SPDX-License-Identifier: GPL-2.0
/* reboot.c: reboot/shutdown/halt/poweroff handling
 *
 * Copyright (C) 2008 David S. Miller <davem@davemloft.net>
 */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

extern "C" {
    static mut of_console_device: *mut DeviceNode;
    static mut reboot_command: *mut c_char;

    fn of_node_is_type(node: *mut DeviceNode, typ: *const c_char) -> bool;
    fn prom_halt_power_off();
    fn prom_halt();
    fn prom_reboot(cmd: *const c_char);
    fn panic(fmt: *const c_char, ...) -> !;
}

/* sysctl - toggle power-off restriction for serial console
 * systems in machine_power_off()
 */
#[no_mangle]
pub static mut scons_pwroff: i32 = 1;

/* This isn't actually used, it exists merely to satisfy the
 * reference in kernel/sys.c
 */
#[no_mangle]
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = Some(machine_power_off);
// EXPORT_SYMBOL(pm_power_off);

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    if !of_node_is_type(of_console_device, b"serial\0".as_ptr() as *const c_char)
        || scons_pwroff != 0
    {
        prom_halt_power_off();
    }

    prom_halt();
}

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    prom_halt();
    panic(b"Halt failed!\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn machine_restart(cmd: *mut c_char) {
    let mut p = reboot_command;

    while *p != 0 && *p != b'\n' as c_char {
        p = p.add(1);
    }
    if *p == b'\n' as c_char {
        *p = 0;
    }
    if !cmd.is_null() {
        prom_reboot(cmd as *const c_char);
    }
    if !reboot_command.is_null() && *reboot_command != 0 {
        prom_reboot(reboot_command as *const c_char);
    }
    prom_reboot(b"\0".as_ptr() as *const c_char);
    panic(b"Reboot failed!\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
