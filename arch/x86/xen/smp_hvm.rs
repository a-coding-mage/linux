// SPDX-License-Identifier: GPL-2.0
// Translated from smp_hvm.c. C header dependencies are supplied externally.

use core::ffi::c_uint;

extern "C" {
    fn smp_processor_id() -> c_uint;
    fn native_smp_prepare_boot_cpu();
    fn xen_vcpu_setup(cpu: c_uint);
    fn xen_hvm_init_time_ops();
    fn xen_init_spinlocks();
    fn native_smp_prepare_cpus(max_cpus: c_uint);
    fn xen_smp_intr_init(cpu: c_uint) -> i32;
    fn xen_init_lock_cpu(cpu: c_uint);
    fn xen_smp_intr_free(cpu: c_uint);
    fn xen_uninit_lock_cpu(cpu: c_uint);
    fn xen_teardown_timer(cpu: c_uint);
    fn xen_smp_cpus_done();
    fn xen_smp_send_reschedule(cpu: c_uint);
    fn xen_smp_send_call_function_ipi(mask: *const core::ffi::c_void);
    fn xen_smp_send_call_function_single_ipi(cpu: c_uint);
    fn bug();
    fn warn_on(condition: bool) -> bool;

    static mut xen_have_vector_callback: bool;
    static mut nopvspin: bool;
    static mut smp_ops: SmpOps;
}

// The kernel's per-CPU storage and possible-CPU iterator are supplied by the
// surrounding architecture code.
extern "C" {
    fn xen_for_each_possible_cpu_next(cpu: *mut c_int) -> bool;
    fn xen_vcpu_id_set(cpu: c_int, value: c_int);
}

use core::ffi::c_int;

#[repr(C)]
struct SmpOps {
    smp_prepare_boot_cpu: Option<unsafe extern "C" fn()>,
    smp_prepare_cpus: Option<unsafe extern "C" fn(c_uint)>,
    smp_cpus_done: Option<unsafe extern "C" fn()>,
    cleanup_dead_cpu: Option<unsafe extern "C" fn(c_uint)>,
    smp_send_reschedule: Option<unsafe extern "C" fn(c_uint)>,
    send_call_func_ipi: Option<unsafe extern "C" fn(*const core::ffi::c_void)>,
    send_call_func_single_ipi: Option<unsafe extern "C" fn(c_uint)>,
}

const XEN_VCPU_ID_INVALID: c_int = -1;

unsafe extern "C" fn xen_hvm_smp_prepare_boot_cpu() {
    if smp_processor_id() != 0 {
        bug();
    }
    native_smp_prepare_boot_cpu();

    /* Setup vcpu_info for boot CPU. Secondary CPUs get their vcpu_info
     * in xen_cpu_up_prepare_hvm(). */
    xen_vcpu_setup(0);

    /* Called again in case the kernel boots on vcpu >= MAX_VIRT_CPUS.
     * Refer to comments in xen_hvm_init_time_ops(). */
    xen_hvm_init_time_ops();

    /* The alternative logic (which patches the unlock/lock) runs before
     * the smp bootup up code is activated. Hence we need to set this up
     * the core kernel is being patched. Otherwise we will have only
     * modules patched but not core code. */
    xen_init_spinlocks();
}

unsafe extern "C" fn xen_hvm_smp_prepare_cpus(max_cpus: c_uint) {
    let mut cpu: c_int = 0;

    native_smp_prepare_cpus(max_cpus);

    if xen_have_vector_callback {
        let _ = warn_on(xen_smp_intr_init(0) != 0);
        xen_init_lock_cpu(0);
    }

    while xen_for_each_possible_cpu_next(&mut cpu) {
        if cpu == 0 {
            continue;
        }

        /* Set default vcpu_id to make sure that we don't use cpu-0's */
        xen_vcpu_id_set(cpu, XEN_VCPU_ID_INVALID);
    }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn xen_hvm_cleanup_dead_cpu(cpu: c_uint) {
    if xen_have_vector_callback {
        xen_smp_intr_free(cpu);
        xen_uninit_lock_cpu(cpu);
        xen_teardown_timer(cpu);
    }
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
unsafe extern "C" fn xen_hvm_cleanup_dead_cpu(_cpu: c_uint) {
    bug();
}

pub unsafe extern "C" fn xen_hvm_smp_init() {
    smp_ops.smp_prepare_boot_cpu = Some(xen_hvm_smp_prepare_boot_cpu);
    smp_ops.smp_prepare_cpus = Some(xen_hvm_smp_prepare_cpus);
    smp_ops.smp_cpus_done = Some(xen_smp_cpus_done);
    smp_ops.cleanup_dead_cpu = Some(xen_hvm_cleanup_dead_cpu);

    if !xen_have_vector_callback {
        #[cfg(feature = "CONFIG_PARAVIRT_SPINLOCKS")]
        {
            nopvspin = true;
        }
        return;
    }

    smp_ops.smp_send_reschedule = Some(xen_smp_send_reschedule);
    smp_ops.send_call_func_ipi = Some(xen_smp_send_call_function_ipi);
    smp_ops.send_call_func_single_ipi = Some(xen_smp_send_call_function_single_ipi);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
