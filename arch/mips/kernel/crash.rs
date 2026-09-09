// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation are referenced here.

use core::ffi::c_void;

extern "C" {
    static mut _crash_smp_send_stop: Option<unsafe extern "C" fn()>;
}

// This keeps a track of which one is crashing cpu.
static mut crashing_cpu: i32 = -1;
static mut cpus_in_crash: cpumask_t = CPU_MASK_NONE;

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    fn smp_processor_id() -> i32;
    fn get_irq_regs() -> *mut pt_regs;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn cpu_online(cpu: i32) -> bool;
    fn set_cpu_online(cpu: i32, online: bool);
    fn local_irq_disable();
    fn cpumask_test_cpu(cpu: i32, mask: *const cpumask_t) -> bool;
    fn crash_save_cpu(regs: *mut pt_regs, cpu: i32);
    fn cpumask_set_cpu(cpu: i32, mask: *mut cpumask_t);
    fn atomic_read(v: *const atomic_t) -> i32;
    fn cpu_relax();
    fn kexec_reboot();
    fn num_online_cpus() -> u32;
    fn smp_call_function(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: i32);
    fn smp_wmb();
    fn cpumask_weight(mask: *const cpumask_t) -> u32;
    fn mdelay(msecs: u32);
    fn pr_emerg(fmt: *const u8, ...);
}

#[cfg(feature = "CONFIG_SMP")]
#[allow(non_snake_case)]
unsafe extern "C" {
    static mut kexec_ready_to_reboot: atomic_t;
}

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" fn crash_shutdown_secondary(passed_regs: *mut c_void) {
    let mut regs = passed_regs as *mut pt_regs;
    let cpu = smp_processor_id();

    /*
     * If we are passed registers, use those.  Otherwise get the
     * regs from the last interrupt, which should be correct, as
     * we are in an interrupt.  But if the regs are not there,
     * pull them from the top of the stack.  They are probably
     * wrong, but we need something to keep from crashing again.
     */
    if regs.is_null() {
        regs = get_irq_regs();
    }
    if regs.is_null() {
        regs = task_pt_regs(current);
    }

    if !cpu_online(cpu) {
        return;
    }

    /* We won't be sent IPIs any more. */
    set_cpu_online(cpu, false);

    local_irq_disable();
    if !cpumask_test_cpu(cpu, &cpus_in_crash) {
        crash_save_cpu(regs, cpu);
    }
    cpumask_set_cpu(cpu, &mut cpus_in_crash);

    while (atomic_read(&kexec_ready_to_reboot) == 0) {
        cpu_relax();
    }

    kexec_reboot();

    /* NOTREACHED */
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn crash_kexec_prepare_cpus() {
    static mut cpus_stopped: i32 = 0;
    let mut msecs: u32;
    let ncpus: u32;

    if cpus_stopped != 0 {
        return;
    }

    ncpus = num_online_cpus() - 1; /* Excluding the panic cpu */

    smp_call_function(crash_shutdown_secondary, core::ptr::null_mut(), 0);
    smp_wmb();

    /*
     * The crash CPU sends an IPI and wait for other CPUs to
     * respond. Delay of at least 10 seconds.
     */
    pr_emerg(b"Sending IPI to other cpus...\n\0".as_ptr());
    msecs = 10000;
    while (cpumask_weight(&cpus_in_crash) < ncpus) && {
        msecs = msecs.wrapping_sub(1);
        msecs > 0
    } {
        cpu_relax();
        mdelay(1);
    }

    cpus_stopped = 1;
}

#[cfg(feature = "CONFIG_SMP")]
#[no_mangle]
pub unsafe extern "C" fn crash_smp_send_stop() {
    if let Some(func) = _crash_smp_send_stop {
        func();
    }

    crash_kexec_prepare_cpus();
}

#[cfg(not(feature = "CONFIG_SMP"))]
unsafe fn crash_kexec_prepare_cpus() {}

#[no_mangle]
pub unsafe extern "C" fn default_machine_crash_shutdown(regs: *mut pt_regs) {
    local_irq_disable();
    crashing_cpu = smp_processor_id();
    crash_save_cpu(regs, crashing_cpu);
    crash_kexec_prepare_cpus();
    cpumask_set_cpu(crashing_cpu, &mut cpus_in_crash);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
