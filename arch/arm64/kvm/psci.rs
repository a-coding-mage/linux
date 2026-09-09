// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

/* External kernel declarations and constants are supplied by the surrounding tree. */

unsafe fn kvm_psci_vcpu_suspend(vcpu: *mut kvm_vcpu) -> c_ulong {
    kvm_vcpu_wfi(vcpu);
    PSCI_RET_SUCCESS
}

unsafe fn kvm_psci_vcpu_on(source_vcpu: *mut kvm_vcpu) -> c_ulong {
    let mut reset_state: *mut vcpu_reset_state;
    let kvm = (*source_vcpu).kvm;
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    let mut ret: c_int = PSCI_RET_SUCCESS as c_int;
    let cpu_id: c_ulong = smccc_get_arg1(source_vcpu);

    if !kvm_psci_valid_affinity(source_vcpu, cpu_id) {
        return PSCI_RET_INVALID_PARAMS;
    }
    vcpu = kvm_mpidr_to_vcpu(kvm, cpu_id);
    if vcpu.is_null() {
        return PSCI_RET_INVALID_PARAMS;
    }

    spin_lock(&mut (*(*vcpu).arch).mp_state_lock);
    if !kvm_arm_vcpu_stopped(vcpu) {
        if kvm_psci_version(source_vcpu) != KVM_ARM_PSCI_0_1 {
            ret = PSCI_RET_ALREADY_ON as c_int;
        } else {
            ret = PSCI_RET_INVALID_PARAMS as c_int;
        }
        spin_unlock(&mut (*(*vcpu).arch).mp_state_lock);
        return ret as c_ulong;
    }

    reset_state = &mut (*(*vcpu).arch).reset_state;
    (*reset_state).pc = smccc_get_arg2(source_vcpu);
    (*reset_state).be = kvm_vcpu_is_be(source_vcpu);
    (*reset_state).r0 = smccc_get_arg3(source_vcpu);
    (*reset_state).reset = true;
    kvm_make_request(KVM_REQ_VCPU_RESET, vcpu);
    smp_wmb();
    WRITE_ONCE((*(*(*vcpu).arch).mp_state).mp_state, KVM_MP_STATE_RUNNABLE);
    kvm_vcpu_wake_up(vcpu);
    spin_unlock(&mut (*(*vcpu).arch).mp_state_lock);
    ret as c_ulong
}

unsafe fn kvm_psci_vcpu_affinity_info(vcpu: *mut kvm_vcpu) -> c_ulong {
    let mut matching_cpus = 0;
    let mut mpidr: c_ulong;
    let mut i: c_ulong = 0;
    let target_affinity = smccc_get_arg1(vcpu);
    let lowest_affinity_level = smccc_get_arg2(vcpu);
    let mut target_affinity_mask = kvm_psci_affinity_mask(lowest_affinity_level);
    let kvm = (*vcpu).kvm;
    let mut tmp: *mut kvm_vcpu = core::ptr::null_mut();

    if !kvm_psci_valid_affinity(vcpu, target_affinity) || target_affinity_mask == 0 {
        return PSCI_RET_INVALID_PARAMS;
    }
    let target_affinity = target_affinity & target_affinity_mask;
    kvm_for_each_vcpu(i, tmp, kvm) {
        mpidr = kvm_vcpu_get_mpidr_aff(tmp);
        if (mpidr & target_affinity_mask) == target_affinity {
            matching_cpus += 1;
            if !kvm_arm_vcpu_stopped(tmp) {
                return PSCI_0_2_AFFINITY_LEVEL_ON;
            }
        }
    }
    if matching_cpus == 0 { PSCI_RET_INVALID_PARAMS } else { PSCI_0_2_AFFINITY_LEVEL_OFF }
}

unsafe fn kvm_prepare_system_event(vcpu: *mut kvm_vcpu, type_: u32, flags: u64) {
    let mut i: c_ulong = 0;
    let mut tmp: *mut kvm_vcpu = core::ptr::null_mut();
    kvm_for_each_vcpu(i, tmp, (*vcpu).kvm) {
        spin_lock(&mut (*(*tmp).arch).mp_state_lock);
        WRITE_ONCE((*(*(*tmp).arch).mp_state).mp_state, KVM_MP_STATE_STOPPED);
        spin_unlock(&mut (*(*tmp).arch).mp_state_lock);
    }
    kvm_make_all_cpus_request((*vcpu).kvm, KVM_REQ_SLEEP);
    memset(&mut (*(*vcpu).run).system_event, 0, core::mem::size_of_val(&(*(*vcpu).run).system_event));
    (*(*vcpu).run).system_event.type_ = type_;
    (*(*vcpu).run).system_event.ndata = 1;
    (*(*vcpu).run).system_event.data[0] = flags;
    (*(*vcpu).run).exit_reason = KVM_EXIT_SYSTEM_EVENT;
}

unsafe fn kvm_psci_system_off(vcpu: *mut kvm_vcpu) { kvm_prepare_system_event(vcpu, KVM_SYSTEM_EVENT_SHUTDOWN, 0); }
unsafe fn kvm_psci_system_off2(vcpu: *mut kvm_vcpu) { kvm_prepare_system_event(vcpu, KVM_SYSTEM_EVENT_SHUTDOWN, KVM_SYSTEM_EVENT_SHUTDOWN_FLAG_PSCI_OFF2); }
unsafe fn kvm_psci_system_reset(vcpu: *mut kvm_vcpu) { kvm_prepare_system_event(vcpu, KVM_SYSTEM_EVENT_RESET, 0); }
unsafe fn kvm_psci_system_reset2(vcpu: *mut kvm_vcpu) { kvm_prepare_system_event(vcpu, KVM_SYSTEM_EVENT_RESET, KVM_SYSTEM_EVENT_RESET_FLAG_PSCI_RESET2); }

unsafe fn kvm_psci_system_suspend(vcpu: *mut kvm_vcpu) {
    let run = (*vcpu).run;
    memset(&mut (*run).system_event, 0, core::mem::size_of_val(&(*run).system_event));
    (*run).system_event.type_ = KVM_SYSTEM_EVENT_SUSPEND;
    (*run).exit_reason = KVM_EXIT_SYSTEM_EVENT;
}

unsafe fn kvm_psci_check_allowed_function(vcpu: *mut kvm_vcpu, fn_: u32) -> c_ulong {
    if (fn_ & PSCI_0_2_64BIT) != 0 && vcpu_mode_is_32bit(vcpu) { PSCI_RET_NOT_SUPPORTED } else { 0 }
}

unsafe fn kvm_psci_0_2_call(vcpu: *mut kvm_vcpu) -> c_int {
    let psci_fn = smccc_get_function(vcpu);
    let mut val: c_ulong;
    let mut ret: c_int = 1;
    match psci_fn {
        PSCI_0_2_FN_PSCI_VERSION => { val = KVM_ARM_PSCI_0_2; }
        PSCI_0_2_FN_CPU_SUSPEND | PSCI_0_2_FN64_CPU_SUSPEND => { val = kvm_psci_vcpu_suspend(vcpu); }
        PSCI_0_2_FN_CPU_OFF => { kvm_arm_vcpu_power_off(vcpu); val = PSCI_RET_SUCCESS; }
        PSCI_0_2_FN_CPU_ON | PSCI_0_2_FN64_CPU_ON => { val = kvm_psci_vcpu_on(vcpu); }
        PSCI_0_2_FN_AFFINITY_INFO | PSCI_0_2_FN64_AFFINITY_INFO => { val = kvm_psci_vcpu_affinity_info(vcpu); }
        PSCI_0_2_FN_MIGRATE_INFO_TYPE => { val = PSCI_0_2_TOS_MP; }
        PSCI_0_2_FN_SYSTEM_OFF => { kvm_psci_system_off(vcpu); val = PSCI_RET_INTERNAL_FAILURE; ret = 0; }
        PSCI_0_2_FN_SYSTEM_RESET => { kvm_psci_system_reset(vcpu); val = PSCI_RET_INTERNAL_FAILURE; ret = 0; }
        _ => { val = PSCI_RET_NOT_SUPPORTED; }
    }
    smccc_set_retval(vcpu, val, 0, 0, 0);
    ret
}

unsafe fn kvm_psci_1_x_call(vcpu: *mut kvm_vcpu, minor: u32) -> c_int {
    let psci_fn = smccc_get_function(vcpu);
    let kvm = (*vcpu).kvm;
    let mut val = PSCI_RET_NOT_SUPPORTED;
    let mut ret: c_int = 1;
    match psci_fn {
        PSCI_0_2_FN_PSCI_VERSION => val = PSCI_VERSION(1, minor),
        PSCI_1_0_FN_PSCI_FEATURES => {
            let arg = smccc_get_arg1(vcpu);
            val = kvm_psci_check_allowed_function(vcpu, arg);
            if val == 0 {
                val = match arg {
                    PSCI_0_2_FN_PSCI_VERSION | PSCI_0_2_FN_CPU_SUSPEND |
                    PSCI_0_2_FN64_CPU_SUSPEND | PSCI_0_2_FN_CPU_OFF |
                    PSCI_0_2_FN_CPU_ON | PSCI_0_2_FN64_CPU_ON |
                    PSCI_0_2_FN_AFFINITY_INFO | PSCI_0_2_FN64_AFFINITY_INFO |
                    PSCI_0_2_FN_MIGRATE_INFO_TYPE | PSCI_0_2_FN_SYSTEM_OFF |
                    PSCI_0_2_FN_SYSTEM_RESET | PSCI_1_0_FN_PSCI_FEATURES |
                    ARM_SMCCC_VERSION_FUNC_ID => 0,
                    PSCI_1_0_FN_SYSTEM_SUSPEND | PSCI_1_0_FN64_SYSTEM_SUSPEND => {
                        if test_bit(KVM_ARCH_FLAG_SYSTEM_SUSPEND_ENABLED, &(*kvm).arch.flags) { 0 } else { PSCI_RET_NOT_SUPPORTED }
                    }
                    PSCI_1_1_FN_SYSTEM_RESET2 | PSCI_1_1_FN64_SYSTEM_RESET2 => {
                        if minor >= 1 { 0 } else { PSCI_RET_NOT_SUPPORTED }
                    }
                    PSCI_1_3_FN_SYSTEM_OFF2 | PSCI_1_3_FN64_SYSTEM_OFF2 => {
                        if minor >= 3 { PSCI_1_3_OFF_TYPE_HIBERNATE_OFF } else { PSCI_RET_NOT_SUPPORTED }
                    }
                    _ => PSCI_RET_NOT_SUPPORTED,
                };
            }
        }
        PSCI_1_0_FN_SYSTEM_SUSPEND | PSCI_1_0_FN64_SYSTEM_SUSPEND => {
            if test_bit(KVM_ARCH_FLAG_SYSTEM_SUSPEND_ENABLED, &(*kvm).arch.flags) { kvm_psci_system_suspend(vcpu); return 0; }
        }
        PSCI_1_1_FN_SYSTEM_RESET2 | PSCI_1_1_FN64_SYSTEM_RESET2 if minor >= 1 => {
            let arg = smccc_get_arg1(vcpu);
            if arg <= PSCI_1_1_RESET_TYPE_SYSTEM_WARM_RESET || arg >= PSCI_1_1_RESET_TYPE_VENDOR_START { kvm_psci_system_reset2(vcpu); vcpu_set_reg(vcpu, 0, PSCI_RET_INTERNAL_FAILURE); return 0; }
            val = PSCI_RET_INVALID_PARAMS;
        }
        PSCI_1_3_FN_SYSTEM_OFF2 | PSCI_1_3_FN64_SYSTEM_OFF2 if minor >= 3 => {
            let arg = smccc_get_arg1(vcpu);
            if (arg != 0 && arg != PSCI_1_3_OFF_TYPE_HIBERNATE_OFF) || smccc_get_arg2(vcpu) != 0 { val = PSCI_RET_INVALID_PARAMS; } else { kvm_psci_system_off2(vcpu); val = PSCI_RET_INTERNAL_FAILURE; ret = 0; }
        }
        _ => return kvm_psci_0_2_call(vcpu),
    }
    smccc_set_retval(vcpu, val, 0, 0, 0);
    ret
}

unsafe fn kvm_psci_0_1_call(vcpu: *mut kvm_vcpu) -> c_int {
    let val = match smccc_get_function(vcpu) { KVM_PSCI_FN_CPU_OFF => { kvm_arm_vcpu_power_off(vcpu); PSCI_RET_SUCCESS }, KVM_PSCI_FN_CPU_ON => kvm_psci_vcpu_on(vcpu), _ => PSCI_RET_NOT_SUPPORTED };
    smccc_set_retval(vcpu, val, 0, 0, 0); 1
}

pub unsafe fn kvm_psci_call(vcpu: *mut kvm_vcpu) -> c_int {
    let psci_fn = smccc_get_function(vcpu);
    let val = kvm_psci_check_allowed_function(vcpu, psci_fn);
    if val != 0 { smccc_set_retval(vcpu, val, 0, 0, 0); return 1; }
    match kvm_psci_version(vcpu) {
        KVM_ARM_PSCI_1_3 => kvm_psci_1_x_call(vcpu, 3), KVM_ARM_PSCI_1_2 => kvm_psci_1_x_call(vcpu, 2),
        KVM_ARM_PSCI_1_1 => kvm_psci_1_x_call(vcpu, 1), KVM_ARM_PSCI_1_0 => kvm_psci_1_x_call(vcpu, 0),
        KVM_ARM_PSCI_0_2 => kvm_psci_0_2_call(vcpu), KVM_ARM_PSCI_0_1 => kvm_psci_0_1_call(vcpu),
        _ => { WARN_ONCE!(true, "Unknown PSCI version {}", kvm_psci_version(vcpu)); smccc_set_retval(vcpu, SMCCC_RET_NOT_SUPPORTED, 0, 0, 0); 1 }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
