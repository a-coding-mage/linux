/*
 * Microwatt FPGA-based SoC platform setup code.
 *
 * Copyright 2020 Paul Mackerras (paulus@ozlabs.org), IBM Corp.
 */

// Declarations supplied by the Linux/PowerPC and Microwatt dependencies.
unsafe extern "C" {
    fn xics_init();
    fn of_platform_default_populate(
        node: *const kernel::ffi::c_void,
        parent: *const kernel::ffi::c_void,
        data: *const kernel::ffi::c_void,
    ) -> kernel::ffi::c_int;
    fn microwatt_init_smp();
    fn microwatt_rng_init();
    fn prep_irq_for_idle_irqsoff() -> kernel::ffi::c_int;
    fn udbg_progress(message: *const kernel::ffi::c_char, hex: kernel::ffi::c_ulong);
}

unsafe fn microwatt_init_irq() {
    unsafe { xics_init() };
}

unsafe fn microwatt_populate() -> kernel::ffi::c_int {
    unsafe {
        of_platform_default_populate(
            core::ptr::null(),
            core::ptr::null(),
            core::ptr::null(),
        )
    }
}

// Corresponds to machine_arch_initcall(microwatt, microwatt_populate).

unsafe fn microwatt_probe() -> kernel::ffi::c_int {
    // Main reason for having this is to start the other CPU(s)
    // IS_ENABLED(CONFIG_SMP) is a build-time configuration condition.
    #[cfg(CONFIG_SMP)]
    unsafe {
        microwatt_init_smp();
    }
    1
}

unsafe fn microwatt_setup_arch() {
    unsafe { microwatt_rng_init() };
}

unsafe fn microwatt_idle() {
    if unsafe { prep_irq_for_idle_irqsoff() } == 0 {
        return;
    }

    unsafe {
        core::arch::asm!("wait", options(nomem, nostack, preserves_flags));
    }
}

// Corresponds to define_machine(microwatt).
#[repr(C)]
pub struct MachineDesc {
    pub name: *const kernel::ffi::c_char,
    pub compatible: *const kernel::ffi::c_char,
    pub probe: unsafe fn() -> kernel::ffi::c_int,
    pub init_irq: unsafe fn(),
    pub setup_arch: unsafe fn(),
    pub progress: unsafe extern "C" fn(*const kernel::ffi::c_char, kernel::ffi::c_ulong),
    pub power_save: unsafe fn(),
}

#[no_mangle]
pub static mut microwatt: MachineDesc = MachineDesc {
    name: b"microwatt\0".as_ptr() as *const kernel::ffi::c_char,
    compatible: b"microwatt-soc\0".as_ptr() as *const kernel::ffi::c_char,
    probe: microwatt_probe,
    init_irq: microwatt_init_irq,
    setup_arch: microwatt_setup_arch,
    progress: udbg_progress,
    power_save: microwatt_idle,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
