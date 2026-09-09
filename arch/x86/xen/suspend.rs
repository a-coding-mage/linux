// SPDX-License-Identifier: GPL-2.0

// Declarations supplied by the surrounding kernel/Xen code.
extern "C" {
    fn xen_save_time_memory_area();
    fn xen_pv_domain() -> bool;
    fn xen_pv_pre_suspend();
    fn xen_pv_post_suspend(cancelled: i32);
    fn xen_hvm_post_suspend(cancelled: i32);
    fn boot_cpu_has(feature: u32) -> bool;
    fn wrmsrq(msr: u32, value: u64);
    fn rdmsrq(msr: u32, value: *mut u64);
    fn smp_processor_id() -> i32;
    fn tick_resume_local();
    fn tick_suspend_local();
    fn xen_restore_time_memory_area();
    fn on_each_cpu(func: unsafe extern "C" fn(*mut core::ffi::c_void), info: *mut core::ffi::c_void, wait: i32);
    fn xen_pmu_init(cpu: i32);
    fn xen_pmu_finish(cpu: i32);
}

// DEFINE_PER_CPU(u64, spec_ctrl)
static mut SPEC_CTRL: u64 = 0;

pub unsafe fn xen_arch_pre_suspend() {
    xen_save_time_memory_area();

    if xen_pv_domain() {
        xen_pv_pre_suspend();
    }
}

pub unsafe fn xen_arch_post_suspend(cancelled: i32) {
    if xen_pv_domain() {
        xen_pv_post_suspend(cancelled);
    } else {
        xen_hvm_post_suspend(cancelled);
    }

    xen_restore_time_memory_area();
}

unsafe extern "C" fn xen_vcpu_notify_restore(_data: *mut core::ffi::c_void) {
    if xen_pv_domain() && boot_cpu_has(X86_FEATURE_SPEC_CTRL) {
        wrmsrq(MSR_IA32_SPEC_CTRL, SPEC_CTRL);
    }

    // Boot processor notified via generic timekeeping_resume()
    if smp_processor_id() == 0 {
        return;
    }

    tick_resume_local();
}

unsafe extern "C" fn xen_vcpu_notify_suspend(_data: *mut core::ffi::c_void) {
    let mut tmp: u64 = 0;

    tick_suspend_local();

    if xen_pv_domain() && boot_cpu_has(X86_FEATURE_SPEC_CTRL) {
        rdmsrq(MSR_IA32_SPEC_CTRL, &mut tmp);
        SPEC_CTRL = tmp;
        wrmsrq(MSR_IA32_SPEC_CTRL, 0);
    }
}

pub unsafe fn xen_arch_resume() {
    on_each_cpu(
        xen_vcpu_notify_restore,
        core::ptr::null_mut(),
        1,
    );

    // for_each_online_cpu(cpu)
    for_each_online_cpu!(cpu, {
        xen_pmu_init(cpu);
    });
}

pub unsafe fn xen_arch_suspend() {
    // for_each_online_cpu(cpu)
    for_each_online_cpu!(cpu, {
        xen_pmu_finish(cpu);
    });

    on_each_cpu(
        xen_vcpu_notify_suspend,
        core::ptr::null_mut(),
        1,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
