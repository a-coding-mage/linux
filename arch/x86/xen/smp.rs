// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux and Xen environments are intentionally
// left as external symbols.

static mut XEN_RESCHED_IRQ: XenCommonIrq = XenCommonIrq { irq: -1, name: core::ptr::null_mut() };
static mut XEN_CALLFUNC_IRQ: XenCommonIrq = XenCommonIrq { irq: -1, name: core::ptr::null_mut() };
static mut XEN_CALLFUNCSINGLE_IRQ: XenCommonIrq = XenCommonIrq { irq: -1, name: core::ptr::null_mut() };
static mut XEN_DEBUG_IRQ: XenCommonIrq = XenCommonIrq { irq: -1, name: core::ptr::null_mut() };

/*
 * Reschedule call back.
 */
unsafe extern "C" fn xen_reschedule_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn {
    inc_irq_stat(RESCHEDULE);
    scheduler_ipi();

    IRQ_HANDLED
}

pub unsafe fn xen_smp_intr_free(cpu: u32) {
    kfree(per_cpu(&raw mut XEN_RESCHED_IRQ, cpu).name);
    per_cpu(&raw mut XEN_RESCHED_IRQ, cpu).name = core::ptr::null_mut();
    if per_cpu(&raw mut XEN_RESCHED_IRQ, cpu).irq >= 0 {
        unbind_from_irqhandler(per_cpu(&raw mut XEN_RESCHED_IRQ, cpu).irq, core::ptr::null_mut());
        per_cpu(&raw mut XEN_RESCHED_IRQ, cpu).irq = -1;
    }
    kfree(per_cpu(&raw mut XEN_CALLFUNC_IRQ, cpu).name);
    per_cpu(&raw mut XEN_CALLFUNC_IRQ, cpu).name = core::ptr::null_mut();
    if per_cpu(&raw mut XEN_CALLFUNC_IRQ, cpu).irq >= 0 {
        unbind_from_irqhandler(per_cpu(&raw mut XEN_CALLFUNC_IRQ, cpu).irq, core::ptr::null_mut());
        per_cpu(&raw mut XEN_CALLFUNC_IRQ, cpu).irq = -1;
    }
    kfree(per_cpu(&raw mut XEN_DEBUG_IRQ, cpu).name);
    per_cpu(&raw mut XEN_DEBUG_IRQ, cpu).name = core::ptr::null_mut();
    if per_cpu(&raw mut XEN_DEBUG_IRQ, cpu).irq >= 0 {
        unbind_from_irqhandler(per_cpu(&raw mut XEN_DEBUG_IRQ, cpu).irq, core::ptr::null_mut());
        per_cpu(&raw mut XEN_DEBUG_IRQ, cpu).irq = -1;
    }
    kfree(per_cpu(&raw mut XEN_CALLFUNCSINGLE_IRQ, cpu).name);
    per_cpu(&raw mut XEN_CALLFUNCSINGLE_IRQ, cpu).name = core::ptr::null_mut();
    if per_cpu(&raw mut XEN_CALLFUNCSINGLE_IRQ, cpu).irq >= 0 {
        unbind_from_irqhandler(per_cpu(&raw mut XEN_CALLFUNCSINGLE_IRQ, cpu).irq, core::ptr::null_mut());
        per_cpu(&raw mut XEN_CALLFUNCSINGLE_IRQ, cpu).irq = -1;
    }
}

pub unsafe fn xen_smp_intr_init(cpu: u32) -> i32 {
    let rc = (|| -> i32 {
        let resched_name = kasprintf(GFP_KERNEL, c"resched%d".as_ptr(), cpu);
        if resched_name.is_null() { return -ENOMEM; }
        per_cpu(&raw mut XEN_RESCHED_IRQ, cpu).name = resched_name;
        let mut rc = bind_ipi_to_irqhandler(XEN_RESCHEDULE_VECTOR, cpu, xen_reschedule_interrupt, IRQF_PERCPU | IRQF_NOBALANCING, resched_name, core::ptr::null_mut());
        if rc < 0 { return rc; }
        per_cpu(&raw mut XEN_RESCHED_IRQ, cpu).irq = rc;
        let callfunc_name = kasprintf(GFP_KERNEL, c"callfunc%d".as_ptr(), cpu);
        if callfunc_name.is_null() { return -ENOMEM; }
        per_cpu(&raw mut XEN_CALLFUNC_IRQ, cpu).name = callfunc_name;
        rc = bind_ipi_to_irqhandler(XEN_CALL_FUNCTION_VECTOR, cpu, xen_call_function_interrupt, IRQF_PERCPU | IRQF_NOBALANCING, callfunc_name, core::ptr::null_mut());
        if rc < 0 { return rc; }
        per_cpu(&raw mut XEN_CALLFUNC_IRQ, cpu).irq = rc;
        if !xen_fifo_events {
            let debug_name = kasprintf(GFP_KERNEL, c"debug%d".as_ptr(), cpu);
            if debug_name.is_null() { return -ENOMEM; }
            per_cpu(&raw mut XEN_DEBUG_IRQ, cpu).name = debug_name;
            rc = bind_virq_to_irqhandler(VIRQ_DEBUG, cpu, xen_debug_interrupt, IRQF_PERCPU | IRQF_NOBALANCING, debug_name, core::ptr::null_mut());
            if rc < 0 { return rc; }
            per_cpu(&raw mut XEN_DEBUG_IRQ, cpu).irq = rc;
        }
        let callfunc_name = kasprintf(GFP_KERNEL, c"callfuncsingle%d".as_ptr(), cpu);
        if callfunc_name.is_null() { return -ENOMEM; }
        per_cpu(&raw mut XEN_CALLFUNCSINGLE_IRQ, cpu).name = callfunc_name;
        rc = bind_ipi_to_irqhandler(XEN_CALL_FUNCTION_SINGLE_VECTOR, cpu, xen_call_function_single_interrupt, IRQF_PERCPU | IRQF_NOBALANCING, callfunc_name, core::ptr::null_mut());
        if rc < 0 { return rc; }
        per_cpu(&raw mut XEN_CALLFUNCSINGLE_IRQ, cpu).irq = rc;
        0
    })();
    if rc < 0 { xen_smp_intr_free(cpu); }
    rc
}

pub unsafe fn xen_smp_cpus_done(max_cpus: u32) {
    if xen_hvm_domain() { native_smp_cpus_done(max_cpus); }
}

pub unsafe fn xen_smp_send_reschedule(cpu: i32) { xen_send_IPI_one(cpu, XEN_RESCHEDULE_VECTOR); }

unsafe fn __xen_send_IPI_mask(mask: *const CpuMask, vector: i32) {
    let mut cpu: u32 = 0;
    for_each_cpu_and!(cpu, mask, cpu_online_mask, { xen_send_IPI_one(cpu as i32, vector); });
}

pub unsafe fn xen_smp_send_call_function_ipi(mask: *const CpuMask) {
    __xen_send_IPI_mask(mask, XEN_CALL_FUNCTION_VECTOR);
    let mut cpu: i32 = 0;
    for_each_cpu!(cpu, mask, {
        if xen_vcpu_stolen(cpu) { HYPERVISOR_sched_op(SCHEDOP_yield, core::ptr::null_mut()); break; }
    });
}

pub unsafe fn xen_smp_send_call_function_single_ipi(cpu: i32) {
    __xen_send_IPI_mask(cpumask_of(cpu), XEN_CALL_FUNCTION_SINGLE_VECTOR);
}

unsafe fn xen_map_vector(vector: i32) -> i32 {
    match vector {
        RESCHEDULE_VECTOR => XEN_RESCHEDULE_VECTOR,
        CALL_FUNCTION_VECTOR => XEN_CALL_FUNCTION_VECTOR,
        CALL_FUNCTION_SINGLE_VECTOR => XEN_CALL_FUNCTION_SINGLE_VECTOR,
        IRQ_WORK_VECTOR => XEN_IRQ_WORK_VECTOR,
        #[cfg(target_arch = "x86_64")]
        NMI_VECTOR | APIC_DM_NMI => XEN_NMI_VECTOR,
        _ => { printk(KERN_ERR, c"xen: vector 0x%x is not implemented\n".as_ptr(), vector); -1 }
    }
}

pub unsafe fn xen_send_IPI_mask(mask: *const CpuMask, vector: i32) { let v = xen_map_vector(vector); if v >= 0 { __xen_send_IPI_mask(mask, v); } }
pub unsafe fn xen_send_IPI_all(vector: i32) { let v = xen_map_vector(vector); if v >= 0 { __xen_send_IPI_mask(cpu_online_mask, v); } }
pub unsafe fn xen_send_IPI_self(vector: i32) { let v = xen_map_vector(vector); if v >= 0 { xen_send_IPI_one(smp_processor_id(), v); } }

pub unsafe fn xen_send_IPI_mask_allbutself(mask: *const CpuMask, vector: i32) {
    let this_cpu = smp_processor_id();
    let xen_vector = xen_map_vector(vector);
    if !(num_online_cpus() > 1) || xen_vector < 0 { return; }
    let mut cpu: u32 = 0;
    for_each_cpu_and!(cpu, mask, cpu_online_mask, {
        if this_cpu == cpu as i32 { continue; }
        xen_send_IPI_one(cpu as i32, xen_vector);
    });
}

pub unsafe fn xen_send_IPI_allbutself(vector: i32) { xen_send_IPI_mask_allbutself(cpu_online_mask, vector); }

unsafe extern "C" fn xen_call_function_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn { generic_smp_call_function_interrupt(); inc_irq_stat(CALL_FUNCTION); IRQ_HANDLED }
unsafe extern "C" fn xen_call_function_single_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn { generic_smp_call_function_single_interrupt(); inc_irq_stat(CALL_FUNCTION); IRQ_HANDLED }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
