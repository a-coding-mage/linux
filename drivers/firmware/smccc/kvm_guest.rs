// SPDX-License-Identifier: GPL-2.0

// pr_fmt(fmt) = "smccc: KVM: " fmt

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

static mut __kvm_arm_hyp_services: [bool; ARM_SMCCC_KVM_NUM_FUNCS as usize] =
    [false; ARM_SMCCC_KVM_NUM_FUNCS as usize];

pub unsafe fn kvm_init_hyp_services() {
    let kvm_uuid = ARM_SMCCC_VENDOR_HYP_UID_KVM;
    let mut res: arm_smccc_res = core::mem::zeroed();
    let mut val: [u32; 4] = [0; 4];

    if !arm_smccc_hypervisor_has_uuid(&kvm_uuid) {
        return;
    }

    arm_smccc_1_1_invoke(ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID, &mut res);

    val[0] = res.a0 as u32;
    val[1] = res.a1 as u32;
    val[2] = res.a2 as u32;
    val[3] = res.a3 as u32;

    for bit in 0..(ARM_SMCCC_KVM_NUM_FUNCS as usize) {
        __kvm_arm_hyp_services[bit] = (val[bit / 32] & (1u32 << (bit % 32))) != 0;
    }

    pr_info!(
        "hypervisor services detected (0x%08lx 0x%08lx 0x%08lx 0x%08lx)\n",
        res.a3, res.a2, res.a1, res.a0
    );

    kvm_arch_init_hyp_services();
}

pub unsafe fn kvm_arm_hyp_service_available(func_id: u32) -> bool {
    if func_id >= ARM_SMCCC_KVM_NUM_FUNCS {
        return false;
    }

    __kvm_arm_hyp_services[func_id as usize]
}

#[cfg(CONFIG_ARM64)]
pub unsafe fn kvm_arm_target_impl_cpu_init() {
    let mut i: i32;
    let mut ver: u32;
    let mut max_cpus: u64;
    let mut res: arm_smccc_res = core::mem::zeroed();
    let mut target: *mut target_impl_cpu;

    if !kvm_arm_hyp_service_available(ARM_SMCCC_KVM_FUNC_DISCOVER_IMPL_VER)
        || !kvm_arm_hyp_service_available(ARM_SMCCC_KVM_FUNC_DISCOVER_IMPL_CPUS)
    {
        return;
    }

    arm_smccc_1_1_invoke(
        ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_VER_FUNC_ID,
        0,
        &mut res,
    );
    if res.a0 != SMCCC_RET_SUCCESS {
        return;
    }

    // Version info is in lower 32 bits and is in SMMCCC_VERSION format
    ver = res.a1 as u32;
    if PSCI_VERSION_MAJOR(ver) != 1 {
        pr_warn!(
            "Unsupported target CPU implementation version v{}.{}\n",
            PSCI_VERSION_MAJOR(ver),
            PSCI_VERSION_MINOR(ver)
        );
        return;
    }

    if res.a2 == 0 {
        pr_warn!("No target implementation CPUs specified\n");
        return;
    }

    max_cpus = res.a2;
    target = memblock_alloc(
        core::mem::size_of::<target_impl_cpu>() as u64 * max_cpus,
        core::mem::align_of::<target_impl_cpu>(),
    ) as *mut target_impl_cpu;
    if target.is_null() {
        pr_warn!("Not enough memory for struct target_impl_cpu\n");
        return;
    }

    i = 0;
    while (i as u64) < max_cpus {
        arm_smccc_1_1_invoke(
            ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_CPUS_FUNC_ID,
            i,
            0,
            0,
            &mut res,
        );
        if res.a0 != SMCCC_RET_SUCCESS {
            pr_warn!("Discovering target implementation CPUs failed\n");
            memblock_free(
                target,
                core::mem::size_of::<target_impl_cpu>() as u64 * max_cpus,
            );
            return;
        }
        (*target.add(i as usize)).midr = res.a1;
        (*target.add(i as usize)).revidr = res.a2;
        (*target.add(i as usize)).aidr = res.a3;
        i += 1;
    }

    if !cpu_errata_set_target_impl(max_cpus, target) {
        pr_warn!("Failed to set target implementation CPUs\n");
        memblock_free(
            target,
            core::mem::size_of::<target_impl_cpu>() as u64 * max_cpus,
        );
        return;
    }

    pr_info!("Number of target implementation CPUs is {}\n", max_cpus);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
