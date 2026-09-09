// SPDX-License-Identifier: GPL-2.0
/*
 * Performance event support for s390x
 *
 *  Copyright IBM Corp. 2012, 2013
 *  Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

// Dependency declarations and build-time configuration are supplied by the
// surrounding kernel translation unit.

unsafe fn sie_block(regs: *mut pt_regs) -> *mut kvm_s390_sie_block {
    let stack = (*regs).gprs[15] as *mut stack_frame;

    if stack.is_null() {
        return core::ptr::null_mut();
    }

    (*stack).sie_control_block as *mut kvm_s390_sie_block
}

unsafe fn is_in_guest(regs: *mut pt_regs) -> bool {
    if user_mode(regs) {
        return false;
    }
    // Preserve CONFIG_KVM conditional intent from the original source.
    #[cfg(feature = "CONFIG_KVM")]
    {
        return instruction_pointer(regs) == sie_exit as usize;
    }
    #[cfg(not(feature = "CONFIG_KVM"))]
    {
        false
    }
}

unsafe fn guest_is_user_mode(regs: *mut pt_regs) -> c_ulong {
    (*sie_block(regs)).gpsw.mask & PSW_MASK_PSTATE
}

unsafe fn instruction_pointer_guest(regs: *mut pt_regs) -> c_ulong {
    (*sie_block(regs)).gpsw.addr
}

pub unsafe fn perf_arch_instruction_pointer(regs: *mut pt_regs) -> c_ulong {
    if is_in_guest(regs) {
        instruction_pointer_guest(regs)
    } else {
        instruction_pointer(regs)
    }
}

unsafe fn perf_misc_guest_flags(regs: *mut pt_regs) -> c_ulong {
    if guest_is_user_mode(regs) != 0 {
        PERF_RECORD_MISC_GUEST_USER
    } else {
        PERF_RECORD_MISC_GUEST_KERNEL
    }
}

unsafe fn perf_misc_flags_sf(regs: *mut pt_regs) -> c_ulong {
    let sde_regs = &(*regs).int_parm_long as *const _ as *const perf_sf_sde_regs;
    if (*sde_regs).in_guest != 0 {
        if user_mode(regs) { PERF_RECORD_MISC_GUEST_USER } else { PERF_RECORD_MISC_GUEST_KERNEL }
    } else if user_mode(regs) {
        PERF_RECORD_MISC_USER
    } else {
        PERF_RECORD_MISC_KERNEL
    }
}

pub unsafe fn perf_arch_misc_flags(regs: *mut pt_regs) -> c_ulong {
    /* Check if the cpum_sf PMU has created the pt_regs structure.
     * In this case, perf misc flags can be easily extracted.  Otherwise,
     * do regular checks on the pt_regs content.
     */
    if (*regs).int_code == 0x1407 && (*regs).int_parm == CPU_MF_INT_SF_PRA {
        if (*regs).gprs[15] == 0 {
            return perf_misc_flags_sf(regs);
        }
    }

    if is_in_guest(regs) {
        return perf_misc_guest_flags(regs);
    }

    if user_mode(regs) { PERF_RECORD_MISC_USER } else { PERF_RECORD_MISC_KERNEL }
}

unsafe fn print_debug_cf() {
    let mut cf_info: cpumf_ctr_info = core::mem::zeroed();
    let cpu = smp_processor_id();

    if qctri(&mut cf_info) == 0 {
        pr_info!("CPU[{}] CPUM_CF: ver={}.{} A={:04x} E={:04x} C={:04x}\n",
            cpu, cf_info.cfvn, cf_info.csvn, cf_info.auth_ctl,
            cf_info.enable_ctl, cf_info.act_ctl);
    }
}

unsafe fn print_debug_sf() {
    let mut si: hws_qsi_info_block = core::mem::zeroed();
    let cpu = smp_processor_id();

    if qsi(&mut si) != 0 { return; }

    pr_info!("CPU[{}] CPUM_SF: basic={} diag={} min={} max={} cpu_speed={}\n",
        cpu, si.as_, si.ad, si.min_sampl_rate, si.max_sampl_rate, si.cpu_speed);
    if si.as_ != 0 {
        pr_info!("CPU[{}] CPUM_SF: Basic-sampling: a={} e={} c={} bsdes={} tear={:016x} dear={:016x}\n",
            cpu, si.as_, si.es, si.cs, si.bsdes, si.tear, si.dear);
    }
    if si.ad != 0 {
        pr_info!("CPU[{}] CPUM_SF: Diagnostic-sampling: a={} e={} c={} dsdes={} tear={:016x} dear={:016x}\n",
            cpu, si.ad, si.ed, si.cd, si.dsdes, si.tear, si.dear);
    }
}

pub unsafe fn perf_event_print_debug() {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    if cpum_cf_avail() { print_debug_cf(); }
    if cpum_sf_avail() { print_debug_sf(); }
    local_irq_restore(flags);
}

/* Service level infrastructure */
unsafe fn sl_print_counter(m: *mut seq_file) {
    let mut ci: cpumf_ctr_info = core::mem::zeroed();
    if qctri(&mut ci) != 0 { return; }
    seq_printf!(m, "CPU-MF: Counter facility: version={}.{} authorization={:04x}\n",
        ci.cfvn, ci.csvn, ci.auth_ctl);
}

unsafe fn sl_print_sampling(m: *mut seq_file) {
    let mut si: hws_qsi_info_block = core::mem::zeroed();
    if qsi(&mut si) != 0 || (si.as_ == 0 && si.ad == 0) { return; }
    seq_printf!(m, "CPU-MF: Sampling facility: min_rate={} max_rate={} cpu_speed={}\n",
        si.min_sampl_rate, si.max_sampl_rate, si.cpu_speed);
    if si.as_ != 0 { seq_printf!(m, "CPU-MF: Sampling facility: mode=basic sample_size={}\n", si.bsdes); }
    if si.ad != 0 { seq_printf!(m, "CPU-MF: Sampling facility: mode=diagnostic sample_size={}\n", si.dsdes); }
}

unsafe fn service_level_perf_print(m: *mut seq_file, _sl: *mut service_level) {
    if cpum_cf_avail() { sl_print_counter(m); }
    if cpum_sf_avail() { sl_print_sampling(m); }
}

static mut service_level_perf: service_level = service_level { seq_print: Some(service_level_perf_print) };

unsafe fn service_level_perf_register() -> c_int {
    register_service_level(&mut service_level_perf)
}

// Equivalent of arch_initcall(service_level_perf_register).

pub unsafe fn perf_callchain_kernel(entry: *mut perf_callchain_entry_ctx, regs: *mut pt_regs) {
    let mut state: unwind_state = core::mem::zeroed();
    let mut addr: c_ulong;
    unwind_for_each_frame!(&mut state, current, regs, 0, {
        addr = unwind_get_return_address(&mut state);
        if addr == 0 || perf_callchain_store(entry, addr) != 0 { return; }
    });
}

pub unsafe fn perf_callchain_user(entry: *mut perf_callchain_entry_ctx, regs: *mut pt_regs) {
    arch_stack_walk_user_common(core::ptr::null_mut(), core::ptr::null_mut(), entry, regs, true);
}

/* Perf definitions for PMU event attributes in sysfs */
pub unsafe fn cpumf_events_sysfs_show(
    _dev: *mut device,
    attr: *mut device_attribute,
    page: *mut c_char,
) -> ssize_t {
    let pmu_attr = container_of!(attr, perf_pmu_events_attr, attr);
    sysfs_emit!(page, "event=0x{:04x}\n", (*pmu_attr).id)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
