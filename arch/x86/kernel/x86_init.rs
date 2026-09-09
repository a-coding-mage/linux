/*
 * Copyright (C) 2009 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
 *
 *  For licencing details see kernel-base/COPYING
 */

// Declarations supplied by the Linux kernel headers are external dependencies.

pub unsafe extern "C" fn x86_init_noop() {}
pub unsafe extern "C" fn x86_init_uint_noop(_unused: ::core::ffi::c_uint) {}
unsafe extern "C" fn iommu_init_noop() -> ::core::ffi::c_int { 0 }
unsafe extern "C" fn iommu_shutdown_noop() {}
pub unsafe extern "C" fn bool_x86_init_noop() -> bool { false }
pub unsafe extern "C" fn x86_op_int_noop(_cpu: ::core::ffi::c_int) {}
pub unsafe extern "C" fn set_rtc_noop(now: *const timespec64) -> ::core::ffi::c_int {
    let _ = now;
    -EINVAL
}
pub unsafe extern "C" fn get_rtc_noop(now: *mut timespec64) { let _ = now; }

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const ::core::ffi::c_char,
}

#[used]
static OF_CMOS_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"motorola,mc146818\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

/*
 * Allow devicetree configured systems to disable the RTC by setting the
 * corresponding DT node's status property to disabled. Code is optimized
 * out for CONFIG_OF=n builds.
 */
unsafe extern "C" fn x86_wallclock_init() {
    let node = of_find_matching_node(core::ptr::null_mut(), OF_CMOS_MATCH.as_ptr());

    if !node.is_null() && !of_device_is_available(node) {
        x86_platform.get_wallclock = Some(get_rtc_noop);
        x86_platform.set_wallclock = Some(set_rtc_noop);
    }
}

/*
 * The platform setup functions are preset with the default functions
 * for standard PC hardware.
 */
pub static mut x86_init: x86_init_ops = x86_init_ops {
    resources: x86_init_resources {
        probe_roms: Some(probe_roms),
        reserve_resources: Some(reserve_standard_io_resources),
        memory_setup: Some(e820__memory_setup_default),
        dmi_setup: Some(dmi_setup),
        realmode_limit: SZ_1M,
    },
    mpparse: x86_init_mpparse {
        setup_ioapic_ids: Some(x86_init_noop),
        find_mptable: Some(mpparse_find_mptable),
        early_parse_smp_cfg: Some(mpparse_parse_early_smp_config),
        parse_smp_cfg: Some(mpparse_parse_smp_config),
    },
    irqs: x86_init_irqs {
        pre_vector_init: Some(init_ISA_irqs),
        intr_init: Some(native_init_IRQ),
        intr_mode_select: Some(apic_intr_mode_select),
        intr_mode_init: Some(apic_intr_mode_init),
        create_pci_msi_domain: Some(native_create_pci_msi_domain),
    },
    oem: x86_init_oem { arch_setup: Some(x86_init_noop), banner: Some(default_banner) },
    paging: x86_init_paging { pagetable_init: Some(native_pagetable_init) },
    timers: x86_init_timers {
        setup_percpu_clockev: Some(setup_boot_APIC_clock),
        timer_init: Some(hpet_time_init),
        wallclock_init: Some(x86_wallclock_init),
    },
    iommu: x86_init_iommu { iommu_init: Some(iommu_init_noop) },
    pci: x86_init_pci {
        init: Some(x86_default_pci_init),
        init_irq: Some(x86_default_pci_init_irq),
        fixup_irqs: Some(x86_default_pci_fixup_irqs),
    },
    hyper: x86_init_hyper {
        init_platform: Some(x86_init_noop), guest_late_init: Some(x86_init_noop),
        x2apic_available: Some(bool_x86_init_noop), msi_ext_dest_id: Some(bool_x86_init_noop),
        init_mem_mapping: Some(x86_init_noop), init_after_bootmem: Some(x86_init_noop),
    },
    acpi: x86_init_acpi {
        set_root_pointer: Some(x86_default_set_root_pointer),
        get_root_pointer: Some(x86_default_get_root_pointer),
        reduced_hw_early_init: Some(acpi_generic_reduced_hw_init),
    },
};

unsafe extern "C" fn default_nmi_init() {}
unsafe extern "C" fn enc_status_change_prepare_noop(_vaddr: ::core::ffi::c_ulong, _npages: ::core::ffi::c_int, _enc: bool) -> ::core::ffi::c_int { 0 }
unsafe extern "C" fn enc_status_change_finish_noop(_vaddr: ::core::ffi::c_ulong, _npages: ::core::ffi::c_int, _enc: bool) -> ::core::ffi::c_int { 0 }
unsafe extern "C" fn enc_tlb_flush_required_noop(_enc: bool) -> bool { false }
unsafe extern "C" fn enc_cache_flush_required_noop() -> bool { false }
unsafe extern "C" fn enc_kexec_begin_noop() {}
unsafe extern "C" fn enc_kexec_finish_noop() {}
unsafe extern "C" fn is_private_mmio_noop(_addr: u64) -> bool { false }

pub static mut x86_platform: x86_platform_ops = x86_platform_ops {
    calibrate_cpu: Some(native_calibrate_cpu_early), calibrate_tsc: Some(native_calibrate_tsc),
    get_wallclock: Some(mach_get_cmos_time), set_wallclock: Some(mach_set_cmos_time),
    iommu_shutdown: Some(iommu_shutdown_noop), is_untracked_pat_range: Some(is_ISA_range),
    nmi_init: Some(default_nmi_init), get_nmi_reason: Some(default_get_nmi_reason),
    save_sched_clock_state: Some(tsc_save_sched_clock_state), restore_sched_clock_state: Some(tsc_restore_sched_clock_state),
    realmode_reserve: Some(reserve_real_mode), realmode_init: Some(init_real_mode),
    hyper: x86_platform_hyper { pin_vcpu: Some(x86_op_int_noop), is_private_mmio: Some(is_private_mmio_noop) },
    guest: x86_platform_guest {
        enc_status_change_prepare: Some(enc_status_change_prepare_noop), enc_status_change_finish: Some(enc_status_change_finish_noop),
        enc_tlb_flush_required: Some(enc_tlb_flush_required_noop), enc_cache_flush_required: Some(enc_cache_flush_required_noop),
        enc_kexec_begin: Some(enc_kexec_begin_noop), enc_kexec_finish: Some(enc_kexec_finish_noop),
    },
};

pub static mut x86_cpuinit: x86_cpuinit_ops = x86_cpuinit_ops {
    early_percpu_clock_init: Some(x86_init_noop),
    setup_percpu_clockev: Some(setup_secondary_APIC_clock),
    parallel_bringup: true,
};

pub static mut x86_apic_ops: x86_apic_ops = x86_apic_ops {
    io_apic_read: Some(native_io_apic_read),
    restore: Some(native_restore_boot_irq_mode),
};

// EXPORT_SYMBOL_GPL(x86_platform);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
