// SPDX-License-Identifier: GPL-2.0
//
// Hyper-V specific APIC code.
// Translated from hv_apic.c; Linux headers and external symbols are supplied
// by the surrounding translation.

static mut orig_apic: apic = unsafe { core::mem::zeroed() };

unsafe fn hv_apic_icr_read() -> u64 {
    let mut reg_val: u64 = 0;
    rdmsrq(HV_X64_MSR_ICR, &mut reg_val);
    reg_val
}

unsafe fn hv_apic_icr_write(low: u32, id: u32) {
    let mut reg_val = SET_XAPIC_DEST_FIELD(id);
    reg_val = reg_val << 32;
    reg_val |= low as u64;
    wrmsrq(HV_X64_MSR_ICR, reg_val);
}

pub unsafe fn hv_enable_coco_interrupt(cpu: u32, vector: u32, set: bool) {
    apic_update_vector(cpu, vector, set);
}

unsafe fn hv_apic_read(reg: u32) -> u32 {
    let mut reg_val: msr = core::mem::zeroed();
    match reg {
        APIC_EOI => { rdmsrq(HV_X64_MSR_EOI, &mut reg_val.q); reg_val.l }
        APIC_TASKPRI => { rdmsrq(HV_X64_MSR_TPR, &mut reg_val.q); reg_val.l }
        _ => native_apic_mem_read(reg),
    }
}

unsafe fn hv_apic_write(reg: u32, val: u32) {
    match reg {
        APIC_EOI => wrmsrq(HV_X64_MSR_EOI, val as u64),
        APIC_TASKPRI => wrmsrq(HV_X64_MSR_TPR, val as u64),
        _ => native_apic_mem_write(reg, val),
    }
}

unsafe fn hv_apic_eoi_write() {
    let hvp = hv_vp_assist_page[smp_processor_id()];
    if !hvp.is_null() && (xchg(&mut (*hvp).apic_assist, 0) & 0x1) != 0 { return; }
    wrmsrq(HV_X64_MSR_EOI, APIC_EOI_ACK as u64);
}

unsafe fn cpu_is_self(cpu: i32) -> bool { cpu == smp_processor_id() }

unsafe fn __send_ipi_mask_ex(mask: *const cpumask, vector: i32, exclude_self: bool) -> bool {
    let mut ipi_arg: *mut hv_send_ipi_ex;
    let mut flags: ulong = 0;
    let mut nr_bank: i32 = 0;
    let mut status: u64 = HV_STATUS_INVALID_PARAMETER;
    if (ms_hyperv.hints & HV_X64_EX_PROCESSOR_MASKS_RECOMMENDED) == 0 { return false; }
    local_irq_save(&mut flags);
    ipi_arg = *this_cpu_ptr(hyperv_pcpu_input_arg);
    if ipi_arg.is_null() { local_irq_restore(flags); return hv_result_success(status); }
    (*ipi_arg).vector = vector;
    (*ipi_arg).reserved = 0;
    (*ipi_arg).vp_set.valid_bank_mask = 0;
    if !cpumask_equal(mask, cpu_present_mask) || exclude_self {
        (*ipi_arg).vp_set.format = HV_GENERIC_SET_SPARSE_4K;
        nr_bank = cpumask_to_vpset_skip(&mut (*ipi_arg).vp_set, mask,
            if exclude_self { Some(cpu_is_self) } else { None });
        if nr_bank <= 0 { local_irq_restore(flags); return hv_result_success(status); }
    } else { (*ipi_arg).vp_set.format = HV_GENERIC_SET_ALL; }
    status = hv_do_rep_hypercall(HVCALL_SEND_IPI_EX, 0, nr_bank, ipi_arg, core::ptr::null_mut());
    local_irq_restore(flags);
    hv_result_success(status)
}

unsafe fn __send_ipi_mask(mask: *const cpumask, vector: i32, exclude_self: bool) -> bool {
    let this_cpu = smp_processor_id();
    let mut ipi_arg: hv_send_ipi = core::mem::zeroed();
    trace_hyperv_send_ipi_mask(mask, vector);
    let weight = cpumask_weight(mask);
    if weight == 0 || (exclude_self && weight == 1 && cpumask_test_cpu(this_cpu, mask)) { return true; }
    if hv_hypercall_pg.is_null() && (ms_hyperv.paravisor_present || !hv_isolation_type_tdx()) { return false; }
    if vector < HV_IPI_LOW_VECTOR || vector > HV_IPI_HIGH_VECTOR { return false; }
    if hv_cpu_number_to_vp_number(cpumask_last(mask)) >= 64 { return __send_ipi_mask_ex(mask, vector, exclude_self); }
    ipi_arg.vector = vector; ipi_arg.cpu_mask = 0;
    for_each_cpu!(cur_cpu, mask, {
        if exclude_self && cur_cpu == this_cpu { continue; }
        let vcpu = hv_cpu_number_to_vp_number(cur_cpu);
        if vcpu == VP_INVAL { return false; }
        if vcpu >= 64 { return __send_ipi_mask_ex(mask, vector, exclude_self); }
        __set_bit(vcpu, &mut ipi_arg.cpu_mask);
    });
    hv_result_success(hv_do_fast_hypercall16(HVCALL_SEND_IPI, ipi_arg.vector, ipi_arg.cpu_mask))
}

unsafe fn __send_ipi_one(cpu: i32, vector: i32) -> bool {
    let vp = hv_cpu_number_to_vp_number(cpu);
    trace_hyperv_send_ipi_one(cpu, vector);
    if vp == VP_INVAL { return false; }
    if hv_hypercall_pg.is_null() && (ms_hyperv.paravisor_present || !hv_isolation_type_tdx()) { return false; }
    if vector < HV_IPI_LOW_VECTOR || vector > HV_IPI_HIGH_VECTOR { return false; }
    if vp >= 64 { return __send_ipi_mask_ex(cpumask_of(cpu), vector, false); }
    hv_result_success(hv_do_fast_hypercall16(HVCALL_SEND_IPI, vector, BIT_ULL(vp)))
}

unsafe fn hv_send_ipi(cpu: i32, vector: i32) { if !__send_ipi_one(cpu, vector) { orig_apic.send_IPI(cpu, vector); } }
unsafe fn hv_send_ipi_mask(mask: *const cpumask, vector: i32) { if !__send_ipi_mask(mask, vector, false) { orig_apic.send_IPI_mask(mask, vector); } }
unsafe fn hv_send_ipi_mask_allbutself(mask: *const cpumask, vector: i32) { if !__send_ipi_mask(mask, vector, true) { orig_apic.send_IPI_mask_allbutself(mask, vector); } }
unsafe fn hv_send_ipi_allbutself(vector: i32) { hv_send_ipi_mask_allbutself(cpu_online_mask, vector); }
unsafe fn hv_send_ipi_all(vector: i32) { if !__send_ipi_mask(cpu_online_mask, vector, false) { orig_apic.send_IPI_all(vector); } }
unsafe fn hv_send_ipi_self(vector: i32) { if !__send_ipi_one(smp_processor_id(), vector) { orig_apic.send_IPI_self(vector); } }

pub unsafe fn hv_apic_init() {
    if cc_platform_has(CC_ATTR_SNP_SECURE_AVIC) { return; }
    if (ms_hyperv.hints & HV_X64_CLUSTER_IPI_RECOMMENDED) != 0 {
        pr_info("Hyper-V: Using IPI hypercalls");
        orig_apic = *apic;
        apic_update_callback(send_IPI, hv_send_ipi);
        apic_update_callback(send_IPI_mask, hv_send_ipi_mask);
        apic_update_callback(send_IPI_mask_allbutself, hv_send_ipi_mask_allbutself);
        apic_update_callback(send_IPI_allbutself, hv_send_ipi_allbutself);
        apic_update_callback(send_IPI_all, hv_send_ipi_all);
        apic_update_callback(send_IPI_self, hv_send_ipi_self);
    }
    if (ms_hyperv.hints & HV_X64_APIC_ACCESS_RECOMMENDED) != 0 {
        pr_info!("Hyper-V: Using enlightened APIC (%s mode)", if x2apic_enabled() { "x2apic" } else { "xapic" });
        apic_update_callback(eoi, hv_apic_eoi_write);
        if !x2apic_enabled() {
            apic_update_callback(read, hv_apic_read);
            apic_update_callback(write, hv_apic_write);
            apic_update_callback(icr_write, hv_apic_icr_write);
            apic_update_callback(icr_read, hv_apic_icr_read);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
