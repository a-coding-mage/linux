/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the C header includes <linux/kdebug.h>.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine_ops {
    pub restart: Option<unsafe extern "C" fn(cmd: *mut core::ffi::c_char)>,
    pub halt: Option<unsafe extern "C" fn()>,
    pub power_off: Option<unsafe extern "C" fn()>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub crash_shutdown: Option<unsafe extern "C" fn(regs: *mut pt_regs)>,
    pub emergency_restart: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    pub static mut machine_ops: machine_ops;
    pub static mut crashing_cpu: core::ffi::c_int;

    pub fn native_machine_crash_shutdown(regs: *mut pt_regs);
    pub fn native_machine_shutdown();
    pub fn machine_real_restart(type_: core::ffi::c_uint) -> !;

    // These must match dispatch in arch/x86/realmode/rm/reboot.S.
    pub fn nmi_shootdown_cpus(callback: nmi_shootdown_cb);
    pub fn run_crash_ipi_callback(regs: *mut pt_regs);
}

pub const MRR_BIOS: core::ffi::c_int = 0;
pub const MRR_APM: core::ffi::c_int = 1;

pub type nmi_shootdown_cb = unsafe extern "C" fn(core::ffi::c_int, *mut pt_regs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
