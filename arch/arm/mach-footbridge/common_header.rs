/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <linux/reboot.h> provides `enum reboot_mode`.

use core::ffi::c_char;

unsafe extern "C" {
    pub fn footbridge_timer_init();
    pub fn isa_timer_init();

    pub fn isa_rtc_init();

    pub fn footbridge_map_io();
    pub fn footbridge_init_irq();

    pub fn isa_init_irq(irq: u32);
    pub fn footbridge_restart(mode: reboot_mode, cmd: *const c_char);

    pub fn footbridge_sched_clock();
}

// External dependency supplied by the Linux reboot interface.
#[allow(non_camel_case_types)]
pub type reboot_mode = core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
