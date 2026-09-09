// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Arm Ltd.

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to the C includes in the source file.

pub unsafe fn kvm_update_stolen_time(vcpu: *mut kvm_vcpu) {
    let kvm = (*vcpu).kvm;
    let base: u64 = (*vcpu).arch.steal.base;
    let last_steal: u64 = (*vcpu).arch.steal.last_steal;
    let offset: u64 = core::mem::offset_of!(pvclock_vcpu_stolen_time, stolen_time) as u64;
    let mut steal: u64 = 0;
    let idx: i32;

    if base == INVALID_GPA {
        return;
    }

    idx = srcu_read_lock(&(*kvm).srcu);
    if !kvm_get_guest(kvm, base.wrapping_add(offset), &mut steal) {
        steal = le64_to_cpu(steal);
        (*vcpu).arch.steal.last_steal = READ_ONCE((*current).sched_info.run_delay);
        steal = steal.wrapping_add((*vcpu).arch.steal.last_steal.wrapping_sub(last_steal));
        kvm_put_guest(kvm, base.wrapping_add(offset), cpu_to_le64(steal));
    }
    srcu_read_unlock(&(*kvm).srcu, idx);
}

pub unsafe fn kvm_hypercall_pv_features(vcpu: *mut kvm_vcpu) -> i64 {
    let feature: u32 = smccc_get_arg1(vcpu);
    let mut val: i64 = SMCCC_RET_NOT_SUPPORTED;

    match feature {
        ARM_SMCCC_HV_PV_TIME_FEATURES | ARM_SMCCC_HV_PV_TIME_ST => {
            if (*vcpu).arch.steal.base != INVALID_GPA {
                val = SMCCC_RET_SUCCESS;
            }
        }
        _ => {}
    }

    val
}

pub unsafe fn kvm_init_stolen_time(vcpu: *mut kvm_vcpu) -> gpa_t {
    let mut init_values: pvclock_vcpu_stolen_time = core::mem::zeroed();
    let kvm = (*vcpu).kvm;
    let base: u64 = (*vcpu).arch.steal.base;

    if base == INVALID_GPA {
        return base;
    }

    /*
     * Start counting stolen time from the time the guest requests
     * the feature enabled.
     */
    (*vcpu).arch.steal.last_steal = (*current).sched_info.run_delay;
    kvm_write_guest_lock(
        kvm,
        base,
        &mut init_values as *mut pvclock_vcpu_stolen_time as *const _,
        core::mem::size_of::<pvclock_vcpu_stolen_time>(),
    );

    base
}

pub unsafe fn kvm_arm_pvtime_supported() -> bool {
    sched_info_on() != 0
}

pub unsafe fn kvm_arm_pvtime_set_attr(
    vcpu: *mut kvm_vcpu,
    attr: *mut kvm_device_attr,
) -> i32 {
    let user: *mut u64 = (*attr).addr as *mut u64;
    let kvm = (*vcpu).kvm;
    let mut ipa: u64 = 0;
    let mut ret: i32 = 0;
    let idx: i32;

    if !kvm_arm_pvtime_supported()
        || (*attr).attr != KVM_ARM_VCPU_PVTIME_IPA
    {
        return -ENXIO;
    }

    if get_user(&mut ipa, user) != 0 {
        return -EFAULT;
    }
    if !ipa.is_multiple_of(64) {
        return -EINVAL;
    }
    if (*vcpu).arch.steal.base != INVALID_GPA {
        return -EEXIST;
    }

    /* Check the address is in a valid memslot */
    idx = srcu_read_lock(&(*kvm).srcu);
    if kvm_is_error_hva(gfn_to_hva(kvm, ipa >> PAGE_SHIFT)) {
        ret = -EINVAL;
    }
    srcu_read_unlock(&(*kvm).srcu, idx);

    if ret == 0 {
        (*vcpu).arch.steal.base = ipa;
    }

    ret
}

pub unsafe fn kvm_arm_pvtime_get_attr(
    vcpu: *mut kvm_vcpu,
    attr: *mut kvm_device_attr,
) -> i32 {
    let user: *mut u64 = (*attr).addr as *mut u64;
    let ipa: u64;

    if !kvm_arm_pvtime_supported()
        || (*attr).attr != KVM_ARM_VCPU_PVTIME_IPA
    {
        return -ENXIO;
    }

    ipa = (*vcpu).arch.steal.base;

    if put_user(ipa, user) != 0 {
        return -EFAULT;
    }
    0
}

pub unsafe fn kvm_arm_pvtime_has_attr(
    _vcpu: *mut kvm_vcpu,
    attr: *mut kvm_device_attr,
) -> i32 {
    match (*attr).attr {
        KVM_ARM_VCPU_PVTIME_IPA => {
            if kvm_arm_pvtime_supported() {
                return 0;
            }
        }
        _ => {}
    }
    -ENXIO
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
