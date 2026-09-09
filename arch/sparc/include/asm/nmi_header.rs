/* SPDX-License-Identifier: GPL-2.0 */

// `__init` is a build/link-time annotation in the C source.
extern "C" {
    pub fn nmi_init() -> i32;
    pub fn perfctr_irq(irq: i32, regs: *mut pt_regs);
    pub fn nmi_adjust_hz(new_hz: u32);

    pub static mut nmi_active: atomic_t;

    pub fn start_nmi_watchdog(unused: *mut core::ffi::c_void);
    pub fn stop_nmi_watchdog(unused: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
