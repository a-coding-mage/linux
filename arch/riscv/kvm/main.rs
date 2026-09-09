// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Anup Patel <anup.patel@wdc.com>
 */

// Linux kernel and RISC-V headers provide the declarations used below.

static mut KVM_RISCV_VIRTUALIZATION_ENABLED: bool = false;

// DEFINE_STATIC_KEY_FALSE(kvm_riscv_vsstage_tlb_no_gpa)
static mut KVM_RISCV_VSSTAGE_TLB_NO_GPA: bool = false;

unsafe fn kvm_riscv_setup_vendor_features() {
    /* Andes AX66: split two-stage TLBs */
    if riscv_cached_mvendorid(0) == ANDES_VENDOR_ID
        && (riscv_cached_marchid(0) & 0xFFFF) == 0x8A66
    {
        static_branch_enable(&mut KVM_RISCV_VSSTAGE_TLB_NO_GPA);
        kvm_info!("VS-stage TLB does not cache guest physical address and VMID\n");
    }
}

unsafe fn kvm_arch_dev_ioctl(_filp: *mut file, _ioctl: u32, _arg: c_ulong) -> c_long {
    -EINVAL
}

/* Initialize hypervisor CSRs - called during CPU online and non-retention idle resume */
unsafe fn kvm_riscv_csr_init() {
    csr_write(CSR_HEDELEG, 0);
    csr_write(CSR_HIDELEG, 0);

    /* VS should access only the time counter directly. Everything else should trap */
    csr_write(CSR_HCOUNTEREN, 0x02);

    csr_write(CSR_HVIP, 0);
}

/* Clear hypervisor CSRs - called during CPU offline and non-retention idle entry */
unsafe fn kvm_riscv_csr_cleanup() {
    /*
     * After clearing the hideleg CSR, the host kernel will receive
     * spurious interrupts if hvip CSR has pending interrupts and the
     * corresponding enable bits in vsie CSR are asserted. To avoid it,
     * hvip CSR and vsie CSR must be cleared before clearing hideleg CSR.
     */
    csr_write(CSR_VSIE, 0);
    csr_write(CSR_HVIP, 0);
    csr_write(CSR_HEDELEG, 0);
    csr_write(CSR_HIDELEG, 0);

    kvm_riscv_clear_former_vcpu();
}

unsafe fn kvm_arch_enable_virtualization_cpu() -> c_int {
    let rc = kvm_riscv_nacl_enable();
    if rc != 0 {
        return rc;
    }

    kvm_riscv_csr_init();
    kvm_riscv_aia_enable();

    this_cpu_write(&mut KVM_RISCV_VIRTUALIZATION_ENABLED, true);

    0
}

unsafe fn kvm_arch_disable_virtualization_cpu() {
    kvm_riscv_aia_disable();
    kvm_riscv_csr_cleanup();
    kvm_riscv_nacl_disable();

    this_cpu_write(&mut KVM_RISCV_VIRTUALIZATION_ENABLED, false);
}

unsafe fn kvm_riscv_cpu_pm_notifier(
    _self: *mut notifier_block,
    cmd: c_ulong,
    _v: *mut c_void,
) -> c_int {
    match cmd {
        CPU_PM_EXIT | CPU_PM_ENTER_FAILED => {
            /* Only restore hypervisor state if KVM virtualization is enabled on this CPU. */
            if this_cpu_read(&KVM_RISCV_VIRTUALIZATION_ENABLED) {
                kvm_riscv_csr_init();
                kvm_riscv_aia_pm_exit();
            }
            NOTIFY_OK
        }
        CPU_PM_ENTER => {
            /* Only save and clear hypervisor state if KVM virtualization is enabled on this CPU. */
            if this_cpu_read(&KVM_RISCV_VIRTUALIZATION_ENABLED) {
                kvm_riscv_aia_pm_enter();
                kvm_riscv_csr_cleanup();
            }
            NOTIFY_OK
        }
        _ => NOTIFY_DONE,
    }
}

static mut KVM_RISCV_CPU_PM_NB: notifier_block = notifier_block {
    notifier_call: Some(kvm_riscv_cpu_pm_notifier),
};

unsafe fn kvm_riscv_teardown() {
    kvm_riscv_aia_exit();
    kvm_riscv_nacl_exit();
    kvm_riscv_v_exit();
    kvm_unregister_perf_callbacks();
}

unsafe fn riscv_kvm_init() -> c_int {
    let mut rc: c_int;
    let mut slist = [0i8; 64];
    let str_: *const c_char;

    if !riscv_isa_extension_available(core::ptr::null_mut(), H) {
        kvm_info!("hypervisor extension not available\n");
        return -ENODEV;
    }
    if sbi_spec_is_0_1() {
        kvm_info!("require SBI v0.2 or higher\n");
        return -ENODEV;
    }
    if !sbi_probe_extension(SBI_EXT_RFENCE) {
        kvm_info!("require SBI RFENCE extension\n");
        return -ENODEV;
    }

    rc = kvm_riscv_nacl_init();
    if rc != 0 && rc != -ENODEV { return rc; }
    kvm_riscv_gstage_mode_detect();
    str_ = match kvm_riscv_gstage_max_pgd_levels {
        2 => c"Sv32x4".as_ptr(), 3 => c"Sv39x4".as_ptr(),
        4 => c"Sv48x4".as_ptr(), 5 => c"Sv57x4".as_ptr(),
        _ => { kvm_riscv_nacl_exit(); return -ENODEV; }
    };
    kvm_riscv_gstage_vmid_detect();
    rc = kvm_riscv_aia_init();
    if rc != 0 && rc != -ENODEV { kvm_riscv_nacl_exit(); return rc; }
    kvm_info!("hypervisor extension available\n");

    if kvm_riscv_nacl_available() {
        rc = 0;
        slist[0] = 0;
        if kvm_riscv_nacl_sync_csr_available() { strcat(slist.as_mut_ptr(), c"sync_csr".as_ptr()); rc += 1; }
        if kvm_riscv_nacl_sync_hfence_available() { if rc != 0 { strcat(slist.as_mut_ptr(), c", ".as_ptr()); } strcat(slist.as_mut_ptr(), c"sync_hfence".as_ptr()); rc += 1; }
        if kvm_riscv_nacl_sync_sret_available() { if rc != 0 { strcat(slist.as_mut_ptr(), c", ".as_ptr()); } strcat(slist.as_mut_ptr(), c"sync_sret".as_ptr()); rc += 1; }
        if kvm_riscv_nacl_autoswap_csr_available() { if rc != 0 { strcat(slist.as_mut_ptr(), c", ".as_ptr()); } strcat(slist.as_mut_ptr(), c"autoswap_csr".as_ptr()); rc += 1; }
        kvm_info!("using SBI nested acceleration with %s\n", if rc != 0 { slist.as_ptr() } else { c"no features".as_ptr() });
    }
    kvm_info!("highest G-stage page table mode is %s\n", str_);
    kvm_info!("VMID %ld bits available\n", kvm_riscv_gstage_vmid_bits());
    kvm_riscv_setup_vendor_features();
    kvm_riscv_v_init();
    kvm_register_perf_callbacks();

    if IS_ENABLED_CONFIG_CPU_PM {
        rc = cpu_pm_register_notifier(&mut KVM_RISCV_CPU_PM_NB);
        if rc != 0 { kvm_err!("Failed to register CPU PM notifier: %d\n", rc); goto_err_teardown(); return rc; }
    }
    rc = kvm_init(core::mem::size_of::<kvm_vcpu>(), 0, THIS_MODULE);
    if rc != 0 { if IS_ENABLED_CONFIG_CPU_PM { cpu_pm_unregister_notifier(&mut KVM_RISCV_CPU_PM_NB); } kvm_riscv_teardown(); return rc; }
    if kvm_riscv_aia_available() { kvm_info!("AIA available with %d guest external interrupts\n", atomic_read(&kvm_riscv_aia_nr_hgei)); }
    0
}

unsafe fn riscv_kvm_exit() {
    kvm_exit();
    if IS_ENABLED_CONFIG_CPU_PM { cpu_pm_unregister_notifier(&mut KVM_RISCV_CPU_PM_NB); }
    kvm_riscv_teardown();
}

// module_init(riscv_kvm_init);
// module_exit(riscv_kvm_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
