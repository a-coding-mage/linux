// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by the surrounding kernel translation.

static mut SHARED_INFO_PFN: ::core::primitive::c_ulong = 0;

#[link_name = "xen_percpu_upcall"]
#[no_mangle]
pub static mut xen_percpu_upcall: bool = false;

pub unsafe fn xen_hvm_init_shared_info() {
    let mut xatp: xen_add_to_physmap = ::core::mem::zeroed();

    xatp.domid = DOMID_SELF;
    xatp.idx = 0;
    xatp.space = XENMAPSPACE_shared_info;
    xatp.gpfn = SHARED_INFO_PFN;
    if HYPERVISOR_memory_op(XENMEM_add_to_physmap, &mut xatp) != 0 {
        BUG();
    }
}

unsafe fn reserve_shared_info() {
    let mut pa: u64 = PAGE_SIZE as u64;

    while !e820__mapped_all(pa, pa + PAGE_SIZE as u64, E820_TYPE_RAM)
        || memblock_is_reserved(pa)
    {
        pa = pa.wrapping_add(PAGE_SIZE as u64);
    }

    SHARED_INFO_PFN = PHYS_PFN(pa);
    memblock_reserve(pa, PAGE_SIZE);
    HYPERVISOR_shared_info = early_memremap(pa, PAGE_SIZE);
}

unsafe fn xen_hvm_init_mem_mapping() {
    early_memunmap(HYPERVISOR_shared_info, PAGE_SIZE);
    HYPERVISOR_shared_info = __va(PFN_PHYS(SHARED_INFO_PFN));

    // The virtual address of shared_info changed; reset VCPU 0's stale pointer.
    xen_vcpu_info_reset(0);
}

unsafe fn init_hvm_pv_info() {
    let mut major: i32;
    let mut minor: i32;
    let mut eax: u32;
    let mut ebx: u32 = 0;
    let mut ecx: u32 = 0;
    let mut edx: u32 = 0;
    let base = xen_cpuid_base();

    eax = cpuid_eax(base + 1);
    major = (eax >> 16) as i32;
    minor = (eax & 0xffff) as i32;
    printk(KERN_INFO, "Xen version %d.%d.\n", major, minor);

    xen_domain_type = XEN_HVM_DOMAIN;
    if xen_pvh_domain() {
        pv_info.name = "Xen PVH";
    } else {
        pv_info.name = "Xen HVM";
    }

    xen_setup_features();
    cpuid(base + 4, &mut eax, &mut ebx, &mut ecx, &mut edx);
    if eax & XEN_HVM_CPUID_VCPU_ID_PRESENT != 0 {
        this_cpu_write(xen_vcpu_id, ebx);
    } else {
        this_cpu_write(xen_vcpu_id, smp_processor_id());
    }
}

pub unsafe fn sysvec_xen_hvm_callback(regs: *mut pt_regs) {
    let old_regs = set_irq_regs(regs);
    if xen_percpu_upcall {
        apic_eoi();
    }
    inc_irq_stat(HYPERVISOR_CALLBACK);
    xen_evtchn_do_upcall();
    set_irq_regs(old_regs);
}

#[cfg(CONFIG_KEXEC_CORE)]
unsafe fn xen_hvm_shutdown() {
    native_machine_shutdown();
    if kexec_in_progress {
        xen_reboot(SHUTDOWN_soft_reset);
    }
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn xen_hvm_crash_shutdown(regs: *mut pt_regs) {
    native_machine_crash_shutdown(regs);
    xen_reboot(SHUTDOWN_soft_reset);
}

unsafe fn xen_cpu_up_prepare_hvm(cpu: ::core::primitive::c_uint) -> i32 {
    let mut cpu_uid: u32 = 0;
    let mut rc: i32 = 0;

    xen_uninit_lock_cpu(cpu);
    if acpi_get_cpu_uid(cpu, &mut cpu_uid) == 0 {
        per_cpu!(xen_vcpu_id, cpu) = cpu_uid;
    } else {
        per_cpu!(xen_vcpu_id, cpu) = cpu;
    }
    xen_vcpu_setup(cpu);
    if !xen_have_vector_callback {
        return 0;
    }
    if xen_percpu_upcall {
        rc = xen_set_upcall_vector(cpu);
        if rc != 0 {
            WARN!(1, "HVMOP_set_evtchn_upcall_vector for CPU %d failed: %d\n", cpu, rc);
            return rc;
        }
    }
    if xen_feature(XENFEAT_hvm_safe_pvclock) {
        xen_setup_timer(cpu);
    }
    rc = xen_smp_intr_init(cpu);
    if rc != 0 {
        WARN!(1, "xen_smp_intr_init() for CPU %d failed: %d\n", cpu, rc);
    }
    rc
}

unsafe fn xen_cpu_dead_hvm(cpu: ::core::primitive::c_uint) -> i32 {
    xen_smp_intr_free(cpu);
    if xen_have_vector_callback && xen_feature(XENFEAT_hvm_safe_pvclock) {
        xen_teardown_timer(cpu);
    }
    0
}

unsafe fn xen_hvm_guest_init() {
    if xen_pv_domain() {
        return;
    }
    if IS_ENABLED(CONFIG_XEN_VIRTIO_FORCE_GRANT) {
        virtio_set_mem_acc_cb(xen_virtio_restricted_mem_acc);
    }
    init_hvm_pv_info();
    reserve_shared_info();
    xen_hvm_init_shared_info();
    xen_vcpu_info_reset(0);
    xen_panic_handler_init();
    xen_hvm_smp_init();
    WARN_ON(xen_cpuhp_setup(xen_cpu_up_prepare_hvm, xen_cpu_dead_hvm));
    xen_unplug_emulated_devices();
    x86_init.irqs.intr_init = xen_init_IRQ;
    xen_hvm_init_time_ops();
    xen_hvm_init_mmu_ops();
    #[cfg(CONFIG_KEXEC_CORE)]
    { machine_ops.shutdown = xen_hvm_shutdown; }
    #[cfg(CONFIG_CRASH_DUMP)]
    { machine_ops.crash_shutdown = xen_hvm_crash_shutdown; }
}

unsafe fn xen_parse_nopv(_arg: *mut ::core::ffi::c_char) -> i32 {
    pr_notice!("\"xen_nopv\" is deprecated, please use \"nopv\" instead\n");
    if xen_cpuid_base() != 0 { nopv = true; }
    0
}

unsafe fn xen_parse_no_vector_callback(_arg: *mut ::core::ffi::c_char) -> i32 {
    xen_have_vector_callback = false;
    0
}

unsafe fn xen_x2apic_available() -> bool { x2apic_supported() }

unsafe fn msi_ext_dest_id() -> bool {
    cpuid_eax(xen_cpuid_base() + 4) & XEN_HVM_CPUID_EXT_DEST_ID != 0
}

unsafe fn xen_hvm_guest_late_init() {
    #[cfg(CONFIG_XEN_PVH)]
    {
        if !xen_pvh && (x86_platform.legacy.rtc || !x86_platform.legacy.no_vga) { return; }
        xen_pvh = true;
        if nopv { panic!("\"nopv\" and \"xen_nopv\" parameters are unsupported in PVH guest."); }
        if nr_ioapics == 0 && acpi_irq_model == ACPI_IRQ_MODEL_PIC {
            acpi_irq_model = ACPI_IRQ_MODEL_PLATFORM;
        }
        machine_ops.emergency_restart = xen_emergency_restart;
        pv_info.name = "Xen PVH";
    }
}

unsafe fn xen_platform_hvm() -> u32 {
    let xen_domain = xen_cpuid_base();
    let h = &mut x86_hyper_xen_hvm.init;
    if xen_pv_domain() { return 0; }
    if xen_domain != 0 { xen_hypercall_setfunc(); }
    if xen_pvh_domain() && nopv {
        pr_info!("\"nopv\" parameter is ignored in PVH guest\n");
        nopv = false;
    } else if nopv && xen_domain != 0 {
        h.init_platform = x86_init_noop;
        h.x2apic_available = bool_x86_init_noop;
        h.init_mem_mapping = x86_init_noop;
        h.init_after_bootmem = x86_init_noop;
        h.guest_late_init = xen_hvm_guest_late_init;
        x86_hyper_xen_hvm.runtime.pin_vcpu = x86_op_int_noop;
    }
    xen_domain
}

pub static mut x86_hyper_xen_hvm: hypervisor_x86 = hypervisor_x86 {
    name: "Xen HVM",
    detect: xen_platform_hvm,
    type_: X86_HYPER_XEN_HVM,
    init: hypervisor_x86_init {
        init_platform: xen_hvm_guest_init,
        x2apic_available: xen_x2apic_available,
        init_mem_mapping: xen_hvm_init_mem_mapping,
        guest_late_init: xen_hvm_guest_late_init,
        msi_ext_dest_id,
    },
    runtime: hypervisor_x86_runtime { pin_vcpu: xen_pin_vcpu },
    ignore_nopv: true,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
