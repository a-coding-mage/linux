// SPDX-License-Identifier: GPL-2.0
//
// Declarations supplied by the Linux kernel and architecture headers are
// intentionally left external to this translation unit.

extern "C" {
    fn sh_wdt_write_cnt(value: u8);
    fn sh_wdt_write_csr(value: u8);
    fn local_irq_disable();
    fn __flush_tlb_global();
    fn trigger_address_error();
    fn cpu_sleep();
    fn smp_send_stop();
    fn do_kernel_power_off();
    fn stop_this_cpu(arg: *mut core::ffi::c_void);
    #[cfg(feature = "CONFIG_KEXEC_CORE")]
    fn native_machine_crash_shutdown(regs: *mut pt_regs);
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine_ops {
    pub power_off: unsafe extern "C" fn(),
    pub shutdown: unsafe extern "C" fn(),
    pub restart: unsafe extern "C" fn(*mut core::ffi::c_char),
    pub halt: unsafe extern "C" fn(),
    #[cfg(feature = "CONFIG_KEXEC_CORE")]
    pub crash_shutdown: unsafe extern "C" fn(*mut pt_regs),
}

#[no_mangle]
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" fn watchdog_trigger_immediate() {
    sh_wdt_write_cnt(0xFF);
    sh_wdt_write_csr(0xC2);
}

unsafe extern "C" fn native_machine_restart(_unused: *mut core::ffi::c_char) {
    local_irq_disable();

    /* Destroy all of the TLBs in preparation for reset by MMU */
    __flush_tlb_global();

    /* Address error with SR.BL=1 first. */
    trigger_address_error();

    /* If that fails or is unsupported, go for the watchdog next. */
    watchdog_trigger_immediate();

    /*
     * Give up and sleep.
     */
    loop {
        cpu_sleep();
    }
}

unsafe extern "C" fn native_machine_shutdown() {
    smp_send_stop();
}

unsafe extern "C" fn native_machine_power_off() {
    do_kernel_power_off();
}

unsafe extern "C" fn native_machine_halt() {
    /* stop other cpus */
    machine_shutdown();

    /* stop this cpu */
    stop_this_cpu(core::ptr::null_mut());
}

#[no_mangle]
pub static mut machine_ops: machine_ops = machine_ops {
    power_off: native_machine_power_off,
    shutdown: native_machine_shutdown,
    restart: native_machine_restart,
    halt: native_machine_halt,
    #[cfg(feature = "CONFIG_KEXEC_CORE")]
    crash_shutdown: native_machine_crash_shutdown,
};

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    (machine_ops.power_off)();
}

#[no_mangle]
pub unsafe extern "C" fn machine_shutdown() {
    (machine_ops.shutdown)();
}

#[no_mangle]
pub unsafe extern "C" fn machine_restart(cmd: *mut core::ffi::c_char) {
    (machine_ops.restart)(cmd);
}

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    (machine_ops.halt)();
}

// Preserved build-time condition: CONFIG_KEXEC_CORE.
#[cfg(feature = "CONFIG_KEXEC_CORE")]
#[no_mangle]
pub unsafe extern "C" fn machine_crash_shutdown(regs: *mut pt_regs) {
    (machine_ops.crash_shutdown)(regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
