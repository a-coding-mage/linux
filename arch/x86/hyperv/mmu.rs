// Hyper-V MMU implementation translated from C.
// External types, constants, globals, and functions are supplied by the surrounding kernel bindings.

const HV_TLB_FLUSH_UNIT: u64 = 4096 * PAGE_SIZE;

#[inline]
unsafe fn fill_gva_list(
    gva_list: *mut u64,
    offset: i32,
    start: usize,
    end: usize,
) -> i32 {
    let mut gva_n = offset;
    let mut cur = start;

    loop {
        let diff = if end > cur { end - cur } else { 0 };

        *gva_list.add(gva_n as usize) = (cur as u64) & PAGE_MASK;
        // Lower 12 bits encode the number of additional pages to flush.
        if diff as u64 >= HV_TLB_FLUSH_UNIT {
            *gva_list.add(gva_n as usize) |= !PAGE_MASK;
            cur += HV_TLB_FLUSH_UNIT as usize;
        } else if diff != 0 {
            *gva_list.add(gva_n as usize) |= ((diff - 1) >> PAGE_SHIFT) as u64;
            cur = end;
        }

        gva_n += 1;
        if cur >= end {
            break;
        }
    }

    gva_n - offset
}

unsafe fn cpu_is_lazy(cpu: i32) -> bool {
    per_cpu(cpu_tlbstate_shared.is_lazy, cpu)
}

unsafe fn hyperv_flush_tlb_multi(cpus: *const cpumask, info: *const flush_tlb_info) {
    let mut cpu: i32;
    let mut vcpu: i32;
    let mut gva_n: i32;
    let mut max_gvas: i32;
    let flush: *mut hv_tlb_flush;
    let mut status: u64;
    let mut flags: ulong;
    let do_lazy = !(*info).freed_tables;

    trace_hyperv_mmu_flush_tlb_multi(cpus, info);

    if !hv_hypercall_pg {
        native_flush_tlb_multi(cpus, info);
        return;
    }

    local_irq_save(&mut flags);
    flush = *this_cpu_ptr(hyperv_pcpu_input_arg);

    if flush.is_null() {
        local_irq_restore(flags);
        native_flush_tlb_multi(cpus, info);
        return;
    }

    if !(*info).mm.is_null() {
        (*flush).address_space = virt_to_phys((*(*info).mm).pgd) & CR3_ADDR_MASK;
        (*flush).flags = 0;
    } else {
        (*flush).address_space = 0;
        (*flush).flags = HV_FLUSH_ALL_VIRTUAL_ADDRESS_SPACES;
    }

    (*flush).processor_mask = 0;
    if cpumask_equal(cpus, cpu_present_mask) {
        (*flush).flags |= HV_FLUSH_ALL_PROCESSORS;
    } else {
        cpu = cpumask_last(cpus);
        if cpu < nr_cpumask_bits && hv_cpu_number_to_vp_number(cpu) >= 64 {
            status = hyperv_flush_tlb_others_ex(cpus, info);
            local_irq_restore(flags);
            if hv_result_success(status) { return; }
            native_flush_tlb_multi(cpus, info);
            return;
        }

        for_each_cpu!(cpu, cpus) {
            if do_lazy && cpu_is_lazy(cpu) { continue; }
            vcpu = hv_cpu_number_to_vp_number(cpu);
            if vcpu == VP_INVAL {
                local_irq_restore(flags);
                native_flush_tlb_multi(cpus, info);
                return;
            }
            if vcpu >= 64 {
                status = hyperv_flush_tlb_others_ex(cpus, info);
                local_irq_restore(flags);
                if hv_result_success(status) { return; }
                native_flush_tlb_multi(cpus, info);
                return;
            }
            __set_bit(vcpu, &mut (*flush).processor_mask as *mut _ as *mut ulong);
        }

        if (*flush).processor_mask == 0 {
            local_irq_restore(flags);
            return;
        }
    }

    max_gvas = ((PAGE_SIZE as usize - core::mem::size_of::<hv_tlb_flush>()) /
        core::mem::size_of::<u64>()) as i32;
    if (*info).end == TLB_FLUSH_ALL {
        (*flush).flags |= HV_FLUSH_NON_GLOBAL_MAPPINGS_ONLY;
        status = hv_do_hypercall(HVCALL_FLUSH_VIRTUAL_ADDRESS_SPACE, flush, core::ptr::null_mut());
    } else if (*info).end != 0 && (((*info).end - (*info).start) / HV_TLB_FLUSH_UNIT as usize) > max_gvas as usize {
        status = hv_do_hypercall(HVCALL_FLUSH_VIRTUAL_ADDRESS_SPACE, flush, core::ptr::null_mut());
    } else {
        gva_n = fill_gva_list((*flush).gva_list.as_mut_ptr(), 0, (*info).start, (*info).end);
        status = hv_do_rep_hypercall(HVCALL_FLUSH_VIRTUAL_ADDRESS_LIST, gva_n, 0, flush, core::ptr::null_mut());
    }

    local_irq_restore(flags);
    if hv_result_success(status) { return; }
    native_flush_tlb_multi(cpus, info);
}

unsafe fn hyperv_flush_tlb_others_ex(cpus: *const cpumask, info: *const flush_tlb_info) -> u64 {
    if ms_hyperv.hints & HV_X64_EX_PROCESSOR_MASKS_RECOMMENDED == 0 { return HV_STATUS_INVALID_PARAMETER; }
    let flush: *mut hv_tlb_flush_ex = *this_cpu_ptr(hyperv_pcpu_input_arg);
    if !(*info).mm.is_null() {
        (*flush).address_space = virt_to_phys((*(*info).mm).pgd) & CR3_ADDR_MASK;
        (*flush).flags = 0;
    } else {
        (*flush).address_space = 0;
        (*flush).flags = HV_FLUSH_ALL_VIRTUAL_ADDRESS_SPACES;
    }
    (*flush).hv_vp_set.valid_bank_mask = 0;
    (*flush).hv_vp_set.format = HV_GENERIC_SET_SPARSE_4K;
    let nr_bank = cpumask_to_vpset_skip(&mut (*flush).hv_vp_set, cpus, if (*info).freed_tables { None } else { Some(cpu_is_lazy) });
    if nr_bank < 0 { return HV_STATUS_INVALID_PARAMETER; }
    let max_gvas = ((PAGE_SIZE as usize - core::mem::size_of::<hv_tlb_flush_ex>() - nr_bank as usize * core::mem::size_of::<u64>()) / core::mem::size_of::<u64>()) as usize;
    if (*info).end == TLB_FLUSH_ALL || ((*info).end != 0 && (((*info).end - (*info).start) / HV_TLB_FLUSH_UNIT as usize) > max_gvas) {
        (*flush).flags |= if (*info).end == TLB_FLUSH_ALL { HV_FLUSH_NON_GLOBAL_MAPPINGS_ONLY } else { 0 };
        hv_do_rep_hypercall(HVCALL_FLUSH_VIRTUAL_ADDRESS_SPACE_EX, 0, nr_bank, flush as *mut _, core::ptr::null_mut())
    } else {
        let gva_n = fill_gva_list((*flush).gva_list.as_mut_ptr(), nr_bank, (*info).start, (*info).end);
        hv_do_rep_hypercall(HVCALL_FLUSH_VIRTUAL_ADDRESS_LIST_EX, gva_n, nr_bank, flush as *mut _, core::ptr::null_mut())
    }
}

pub unsafe fn hyperv_setup_mmu_ops() {
    if ms_hyperv.hints & HV_X64_REMOTE_TLB_FLUSH_RECOMMENDED == 0 { return; }
    pr_info!("Using hypercall for remote TLB flush\n");
    pv_ops.mmu.flush_tlb_multi = Some(hyperv_flush_tlb_multi);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
