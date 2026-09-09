// SPDX-License-Identifier: GPL-2.0

// Linux/Xen dependencies are supplied by the surrounding translation unit.

extern "C" {
    static mut boot_cpu_data: CpuInfo;
    static mut HYPERVISOR_shared_info: *mut shared_info;
}

// DEFINE_STATIC_CALL(xen_hypercall, xen_hypercall_hvm);
// EXPORT_STATIC_CALL_TRAMP(xen_hypercall);

// Pointer to the xen_vcpu_info structure or shared_info->vcpu_info[cpu].
// It is switched during boot and is used to acknowledge pending events.
static mut xen_vcpu: *mut vcpu_info = core::ptr::null_mut();
static mut xen_vcpu_info: vcpu_info = unsafe { core::mem::zeroed() };

// Linux <-> Xen vCPU id mapping
static mut xen_vcpu_id: u32 = 0;

static mut machine_to_phys_mapping: *mut c_ulong = MACH2PHYS_VIRT_START as *mut c_ulong;
static mut machine_to_phys_nr: c_ulong = 0;
static mut xen_start_info: *mut start_info = core::ptr::null_mut();
static mut xen_dummy_shared_info: shared_info = unsafe { core::mem::zeroed() };
static mut xen_have_vector_callback: bool = true;

// These need to live in data because xen_prepare_pvh() runs before bss is cleared.
static mut xen_domain_type: xen_domain_type = XEN_NATIVE;
static mut xen_start_flags: u32 = 0;

// Point at empty memory initially; the real shared_info page is mapped later.
static mut HYPERVISOR_SHARED_INFO_PTR: *mut shared_info = unsafe { &mut xen_dummy_shared_info };

// Number of pages released from the initial allocation.
static mut xen_released_pages: c_ulong = 0;

unsafe fn xen_get_vendor() {
    init_cpu_devs();
    cpuid_scan_cpu(&mut boot_cpu_data);
    cpu_detect(&mut boot_cpu_data);
    get_cpu_vendor(&mut boot_cpu_data);
}

pub unsafe fn xen_hypercall_setfunc() {
    if static_call_query(xen_hypercall) != xen_hypercall_hvm {
        return;
    }
    if boot_cpu_data.x86_vendor == X86_VENDOR_AMD || boot_cpu_data.x86_vendor == X86_VENDOR_HYGON {
        static_call_update(xen_hypercall, xen_hypercall_amd);
    } else {
        static_call_update(xen_hypercall, xen_hypercall_intel);
    }
}

pub unsafe fn __xen_hypercall_setfunc() -> *mut core::ffi::c_void {
    let func: unsafe extern "C" fn();
    instrumentation_begin();
    xen_get_vendor();
    if boot_cpu_data.x86_vendor == X86_VENDOR_AMD || boot_cpu_data.x86_vendor == X86_VENDOR_HYGON {
        func = xen_hypercall_amd;
    } else {
        func = xen_hypercall_intel;
    }
    static_call_update_early(xen_hypercall, func);
    instrumentation_end();
    func as *mut core::ffi::c_void
}

unsafe fn xen_cpu_up_online(cpu: c_uint) -> c_int {
    xen_init_lock_cpu(cpu);
    0
}

pub unsafe fn xen_cpuhp_setup(cpu_up_prepare_cb: Option<unsafe extern "C" fn(c_uint) -> c_int>, cpu_dead_cb: Option<unsafe extern "C" fn(c_uint) -> c_int>) -> c_int {
    let mut rc = cpuhp_setup_state_nocalls(CPUHP_XEN_PREPARE, "x86/xen/guest:prepare", cpu_up_prepare_cb, cpu_dead_cb);
    if rc >= 0 {
        rc = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "x86/xen/guest:online", Some(xen_cpu_up_online), None);
        if rc < 0 { cpuhp_remove_state_nocalls(CPUHP_XEN_PREPARE); }
    }
    if rc >= 0 { 0 } else { rc }
}

unsafe fn xen_vcpu_setup_restore(cpu: c_int) {
    xen_vcpu_info_reset(cpu);
    if xen_pv_domain() || (xen_hvm_domain() && cpu_online(cpu)) { xen_vcpu_setup(cpu); }
}

pub unsafe fn xen_vcpu_restore() {
    for_each_possible_cpu(|cpu| {
        let other_cpu = cpu != smp_processor_id();
        if xen_vcpu_nr(cpu) == XEN_VCPU_ID_INVALID { return; }
        let is_up = HYPERVISOR_vcpu_op(VCPUOP_is_up, xen_vcpu_nr(cpu), core::ptr::null_mut()) > 0;
        if other_cpu && is_up && HYPERVISOR_vcpu_op(VCPUOP_down, xen_vcpu_nr(cpu), core::ptr::null_mut()) != 0 { BUG(); }
        if xen_pv_domain() || xen_feature(XENFEAT_hvm_safe_pvclock) { xen_setup_runstate_info(cpu); }
        xen_vcpu_setup_restore(cpu);
        if other_cpu && is_up && HYPERVISOR_vcpu_op(VCPUOP_up, xen_vcpu_nr(cpu), core::ptr::null_mut()) != 0 { BUG(); }
    });
}

pub unsafe fn xen_vcpu_info_reset(cpu: c_int) {
    if xen_vcpu_nr(cpu) < MAX_VIRT_CPUS {
        xen_vcpu = &mut (*HYPERVISOR_SHARED_INFO_PTR).vcpu_info[xen_vcpu_nr(cpu) as usize];
    } else { xen_vcpu = core::ptr::null_mut(); }
}

pub unsafe fn xen_vcpu_setup(cpu: c_int) {
    let mut info: vcpu_register_vcpu_info = core::mem::zeroed();
    let vcpup = &mut xen_vcpu_info;
    if xen_hvm_domain() && xen_vcpu == vcpup { return; }
    info.mfn = arbitrary_virt_to_mfn(vcpup);
    info.offset = offset_in_page(vcpup);
    let err = HYPERVISOR_vcpu_op(VCPUOP_register_vcpu_info, xen_vcpu_nr(cpu), &mut info);
    if err != 0 { panic!("register_vcpu_info failed: cpu={} err={}\n", cpu, err); }
    xen_vcpu = vcpup;
}

pub unsafe fn xen_banner() {
    let version = HYPERVISOR_xen_version(XENVER_version, core::ptr::null_mut());
    let mut extra: xen_extraversion = core::mem::zeroed();
    HYPERVISOR_xen_version(XENVER_extraversion, &mut extra);
    pr_info!("Booting kernel on {}\n", pv_info.name);
    pr_info!("Xen version: {}.{}{}{}\n", version >> 16, version & 0xffff, extra.extraversion, if xen_feature(XENFEAT_mmu_pt_update_preserve_ad) { " (preserve-AD)" } else { "" });
}

pub unsafe fn xen_running_on_version_or_later(major: c_uint, minor: c_uint) -> bool {
    if !xen_domain() { return false; }
    let version = HYPERVISOR_xen_version(XENVER_version, core::ptr::null_mut());
    ((version >> 16 == major) && ((version & 0xffff) >= minor)) || (version >> 16 > major)
}

pub unsafe fn xen_add_preferred_consoles() {
    add_preferred_console("xenboot", 0, core::ptr::null());
    if !boot_params.screen_info.orig_video_isVGA { add_preferred_console("tty", 0, core::ptr::null()); }
    add_preferred_console("hvc", 0, core::ptr::null());
    if boot_params.screen_info.orig_video_isVGA { add_preferred_console("tty", 0, core::ptr::null()); }
}

pub unsafe fn xen_reboot(reason: c_int) {
    let mut r = sched_shutdown { reason };
    for_each_online_cpu(|cpu| xen_pmu_finish(cpu));
    if HYPERVISOR_sched_op(SCHEDOP_shutdown, &mut r) != 0 { BUG(); }
}

static mut reboot_reason: c_int = SHUTDOWN_reboot;
static mut xen_legacy_crash: bool = false;
pub unsafe fn xen_emergency_restart() { xen_reboot(reboot_reason); }

unsafe fn xen_panic_event(_this: *mut notifier_block, _event: c_ulong, _ptr: *mut c_void) -> c_int {
    if !kexec_crash_loaded() {
        if xen_legacy_crash { xen_reboot(SHUTDOWN_crash); }
        reboot_reason = SHUTDOWN_crash;
        if panic_timeout == 0 { panic_timeout = -1; }
    }
    NOTIFY_DONE
}

unsafe fn parse_xen_legacy_crash(_arg: *mut c_char) -> c_int { xen_legacy_crash = true; 0 }

static mut xen_panic_block: notifier_block = notifier_block { notifier_call: Some(xen_panic_event), priority: INT_MIN };

pub unsafe fn xen_panic_handler_init() -> c_int {
    atomic_notifier_chain_register(&mut panic_notifier_list, &mut xen_panic_block);
    0
}

pub unsafe fn xen_pin_vcpu(cpu: c_int) {
    static mut disable_pinning: bool = false;
    if disable_pinning { return; }
    let mut pin_override = sched_pin_override { pcpu: cpu };
    let ret = HYPERVISOR_sched_op(SCHEDOP_pin_override, &mut pin_override);
    if cpu < 0 { return; }
    match ret {
        -ENOSYS => { pr_warn!("Unable to pin on physical cpu {}. In case of problems consider vcpu pinning.\n", cpu); disable_pinning = true; }
        -EPERM => { WARN!(1, "Trying to pin vcpu without having privilege to do so\n"); disable_pinning = true; }
        -EINVAL | -EBUSY => pr_warn!("Physical cpu {} not available for pinning. Check Xen cpu configuration.\n", cpu),
        0 => (),
        _ => { WARN!(1, "rc {} while trying to pin vcpu\n", ret); disable_pinning = true; }
    }
}

// CONFIG_HOTPLUG_CPU
pub unsafe fn xen_arch_register_cpu(num: c_int) { arch_register_cpu(num); }
pub unsafe fn xen_arch_unregister_cpu(num: c_int) { arch_unregister_cpu(num); }

// Amount of extra memory space added to the e820 ranges.
static mut xen_extra_mem: [xen_memory_region; XEN_EXTRA_MEM_MAX_REGIONS as usize] = unsafe { core::mem::zeroed() };

pub unsafe fn xen_add_extra_mem(start_pfn: c_ulong, n_pfns: c_ulong) {
    let mut i = 0;
    while i < XEN_EXTRA_MEM_MAX_REGIONS {
        let region = &mut xen_extra_mem[i as usize];
        if region.n_pfns == 0 { region.start_pfn = start_pfn; region.n_pfns = n_pfns; break; }
        if region.start_pfn + region.n_pfns == start_pfn { region.n_pfns += n_pfns; break; }
        i += 1;
    }
    if i == XEN_EXTRA_MEM_MAX_REGIONS { printk!(KERN_WARNING "Warning: not enough extra memory regions\n"); }
    memblock_reserve(PFN_PHYS(start_pfn), PFN_PHYS(n_pfns));
}

// CONFIG_XEN_UNPOPULATED_ALLOC
pub unsafe fn arch_xen_unpopulated_init(res: *mut *mut resource) -> c_int {
    if !xen_domain() { return -ENODEV; }
    *res = &mut iomem_resource;
    for i in 0..XEN_EXTRA_MEM_MAX_REGIONS as usize {
        for j in 0..xen_extra_mem[i].n_pfns {
            let mut pg = pfn_to_page(xen_extra_mem[i].start_pfn + j);
            xen_free_unpopulated_pages(1, &mut pg);
        }
        xen_unpopulated_pages += xen_extra_mem[i].n_pfns;
        xen_extra_mem[i].n_pfns = 0;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
