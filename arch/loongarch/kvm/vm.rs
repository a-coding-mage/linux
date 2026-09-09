// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub static kvm_vm_stats_desc: [kvm_stats_desc; 3] = [
    KVM_GENERIC_VM_STATS!(),
    STATS_DESC_ICOUNTER!(VM, pages),
    STATS_DESC_ICOUNTER!(VM, hugepages),
];

pub static kvm_vm_stats_header: kvm_stats_header = kvm_stats_header {
    name_size: KVM_STATS_NAME_SIZE,
    num_desc: ARRAY_SIZE!(kvm_vm_stats_desc),
    id_offset: core::mem::size_of::<kvm_stats_header>(),
    desc_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE,
    data_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE
        + core::mem::size_of_val(&kvm_vm_stats_desc),
};

unsafe fn kvm_vm_init_features(kvm: *mut kvm) {
    let mut val: c_ulong;

    if cpu_has_lsx {
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_LSX);
    }
    if cpu_has_lasx {
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_LASX);
    }
    if cpu_has_lbt_x86 {
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_X86BT);
    }
    if cpu_has_lbt_arm {
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_ARMBT);
    }
    if cpu_has_lbt_mips {
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_MIPSBT);
    }
    if cpu_has_ptw {
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_PTW);
    }
    if cpu_has_msgint {
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_MSGINT);
    }

    val = read_csr_gcfg();
    if val & CSR_GCFG_GPMP != 0 {
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_PMU);
    }

    // Enable all PV features by default.
    (*kvm).arch.pv_features |= BIT!(KVM_FEATURE_IPI);
    (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_PV_IPI);
    if kvm_pvtime_supported() {
        (*kvm).arch.pv_features |= BIT!(KVM_FEATURE_PREEMPT);
        (*kvm).arch.pv_features |= BIT!(KVM_FEATURE_STEAL_TIME);
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_PV_PREEMPT);
        (*kvm).arch.kvm_features |= BIT!(KVM_LOONGARCH_VM_FEAT_PV_STEALTIME);
    }
}

pub unsafe fn kvm_arch_init_vm(kvm: *mut kvm, _type: c_ulong) -> c_int {
    // Allocate page table to map GPA -> RPA.
    (*kvm).arch.pgd = kvm_pgd_alloc();
    if (*kvm).arch.pgd.is_null() {
        return -ENOMEM;
    }

    (*kvm).arch.phyid_map = kvzalloc_obj!(kvm_phyid_map, GFP_KERNEL_ACCOUNT);
    if (*kvm).arch.phyid_map.is_null() {
        free_page((*kvm).arch.pgd as c_ulong);
        (*kvm).arch.pgd = core::ptr::null_mut();
        return -ENOMEM;
    }
    spin_lock_init!(&mut (*kvm).arch.phyid_map_lock);

    kvm_init_vmcs(kvm);
    kvm_vm_init_features(kvm);

    // cpu_vabits means user address space only (a half of total).
    // GPA size of VM is the same with the size of user address space.
    (*kvm).arch.gpa_size = BIT!(cpu_vabits);
    (*kvm).arch.root_level = CONFIG_PGTABLE_LEVELS - 1;
    (*kvm).arch.invalid_ptes[0] = 0;
    (*kvm).arch.invalid_ptes[1] = invalid_pte_table as c_ulong;
    #[cfg(CONFIG_PGTABLE_LEVELS > 2)]
    { (*kvm).arch.invalid_ptes[2] = invalid_pmd_table as c_ulong; }
    #[cfg(CONFIG_PGTABLE_LEVELS > 3)]
    { (*kvm).arch.invalid_ptes[3] = invalid_pud_table as c_ulong; }
    let mut i = 0;
    while i <= (*kvm).arch.root_level {
        (*kvm).arch.pte_shifts[i] = PAGE_SHIFT + i * (PAGE_SHIFT - 3);
        i += 1;
    }
    0
}

pub unsafe fn kvm_arch_destroy_vm(kvm: *mut kvm) {
    kvm_destroy_vcpus(kvm);
    free_page((*kvm).arch.pgd as c_ulong);
    (*kvm).arch.pgd = core::ptr::null_mut();
    kvfree((*kvm).arch.phyid_map);
    (*kvm).arch.phyid_map = core::ptr::null_mut();
}

pub unsafe fn kvm_vm_ioctl_check_extension(_kvm: *mut kvm, ext: c_long) -> c_int {
    match ext {
        KVM_CAP_IRQCHIP | KVM_CAP_ONE_REG | KVM_CAP_ENABLE_CAP |
        KVM_CAP_READONLY_MEM | KVM_CAP_IMMEDIATE_EXIT | KVM_CAP_IOEVENTFD |
        KVM_CAP_MP_STATE | KVM_CAP_SET_GUEST_DEBUG | KVM_CAP_VCPU_ATTRIBUTES => 1,
        KVM_CAP_NR_VCPUS => min_t!(c_uint, num_online_cpus(), KVM_MAX_VCPUS) as c_int,
        KVM_CAP_MAX_VCPUS => KVM_MAX_VCPUS,
        KVM_CAP_MAX_VCPU_ID => KVM_MAX_VCPU_IDS,
        KVM_CAP_NR_MEMSLOTS => KVM_USER_MEM_SLOTS,
        KVM_CAP_STEAL_TIME => kvm_pvtime_supported() as c_int,
        _ => 0,
    }
}

unsafe fn kvm_vm_feature_has_attr(kvm: *mut kvm, attr: *mut kvm_device_attr) -> c_int {
    match (*attr).attr {
        KVM_LOONGARCH_VM_FEAT_LSX | KVM_LOONGARCH_VM_FEAT_LASX |
        KVM_LOONGARCH_VM_FEAT_X86BT | KVM_LOONGARCH_VM_FEAT_ARMBT |
        KVM_LOONGARCH_VM_FEAT_MIPSBT | KVM_LOONGARCH_VM_FEAT_PTW |
        KVM_LOONGARCH_VM_FEAT_MSGINT | KVM_LOONGARCH_VM_FEAT_PMU |
        KVM_LOONGARCH_VM_FEAT_PV_IPI | KVM_LOONGARCH_VM_FEAT_PV_PREEMPT |
        KVM_LOONGARCH_VM_FEAT_PV_STEALTIME => {
            if kvm_vm_support!(&(*kvm).arch, (*attr).attr) { 0 } else { -ENXIO }
        }
        _ => -ENXIO,
    }
}

unsafe fn kvm_vm_has_attr(kvm: *mut kvm, attr: *mut kvm_device_attr) -> c_int {
    match (*attr).group {
        KVM_LOONGARCH_VM_FEAT_CTRL => kvm_vm_feature_has_attr(kvm, attr),
        _ => -ENXIO,
    }
}

pub unsafe fn kvm_arch_vm_ioctl(filp: *mut file, ioctl: c_uint, arg: c_ulong) -> c_int {
    let argp = arg as *mut c_void;
    let kvm = (*filp).private_data as *mut kvm;
    let mut attr: kvm_device_attr = core::mem::zeroed();
    match ioctl {
        KVM_CREATE_IRQCHIP => 0,
        KVM_HAS_DEVICE_ATTR => {
            if copy_from_user!(&mut attr, argp, core::mem::size_of::<kvm_device_attr>()) != 0 { -EFAULT }
            else { kvm_vm_has_attr(kvm, &mut attr) }
        }
        _ => -ENOIOCTLCMD,
    }
}

pub unsafe fn kvm_vm_ioctl_irq_line(kvm: *mut kvm, irq_event: *mut kvm_irq_level, line_status: bool) -> c_int {
    if !kvm_arch_irqchip_in_kernel(kvm) { return -ENXIO; }
    (*irq_event).status = kvm_set_irq(kvm, KVM_USERSPACE_IRQ_SOURCE_ID,
        (*irq_event).irq, (*irq_event).level, line_status);
    0
}

pub unsafe fn kvm_arch_irqchip_in_kernel(kvm: *mut kvm) -> bool {
    !(*kvm).arch.ipi.is_null() && !(*kvm).arch.eiointc.is_null() && !(*kvm).arch.pch_pic.is_null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
