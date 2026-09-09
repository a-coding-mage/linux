// SPDX-License-Identifier: GPL-2.0-only
// Xen/Linux dependencies are supplied by the surrounding translation unit.

static mut _xen_start_info: start_info = unsafe { core::mem::zeroed() };
pub static mut xen_start_info: *mut start_info = unsafe { &_xen_start_info };

pub static mut xen_domain_type: xen_domain_type = XEN_NATIVE;
pub static mut xen_dummy_shared_info: shared_info = unsafe { core::mem::zeroed() };
pub static mut HYPERVISOR_shared_info: *mut shared_info = unsafe { &mut xen_dummy_shared_info };

// DEFINE_PER_CPU(struct vcpu_info *, xen_vcpu);
// DEFINE_PER_CPU(uint32_t, xen_vcpu_id);
static mut xen_vcpu_info: *mut vcpu_info = core::ptr::null_mut();
pub static mut xen_released_pages: c_ulong = 0;
pub static mut xen_extra_mem: [xen_memory_region; XEN_EXTRA_MEM_MAX_REGIONS] = unsafe { core::mem::zeroed() };
static mut xen_events_irq: c_uint = 0;
static mut xen_grant_frames: phys_addr_t = 0;
const GRANT_TABLE_INDEX: usize = 0;
const EXT_REGION_INDEX: usize = 1;
pub static mut xen_start_flags: u32 = 0;

pub unsafe fn xen_unmap_domain_gfn_range(vma: *mut vm_area_struct, nr: c_int, pages: *mut *mut page) -> c_int {
    xen_xlate_unmap_gfn_range(vma, nr, pages)
}

unsafe fn xen_read_wallclock(ts: *mut timespec64) {
    let mut version: u32;
    let mut now: timespec64 = core::mem::zeroed();
    let mut ts_monotonic: timespec64 = core::mem::zeroed();
    let s = HYPERVISOR_shared_info;
    let wall_clock = &mut (*s).wc;
    loop {
        version = wall_clock.version;
        rmb();
        now.tv_sec = (((wall_clock.sec_hi as u64) << 32) | wall_clock.sec as u64) as i64;
        now.tv_nsec = wall_clock.nsec as i32;
        rmb();
        if (wall_clock.version & 1) == 0 && version == wall_clock.version { break; }
    }
    ktime_get_ts64(&mut ts_monotonic);
    *ts = timespec64_add(now, ts_monotonic);
}

unsafe fn xen_pvclock_gtod_notify(_nb: *mut notifier_block, was_set: c_ulong, priv_: *mut c_void) -> c_int {
    static mut next_sync: timespec64 = timespec64 { tv_sec: 0, tv_nsec: 0 };
    let mut op: xen_platform_op = core::mem::zeroed();
    let mut now: timespec64 = core::mem::zeroed();
    let mut system_time: timespec64;
    let tk = &* (priv_ as *mut timekeeper);
    now.tv_sec = tk.xtime_sec;
    now.tv_nsec = (tk.tkr_mono.xtime_nsec >> tk.tkr_mono.shift) as i32;
    system_time = timespec64_add(now, tk.wall_to_monotonic);
    if was_set == 0 && timespec64_compare(&now, &next_sync) < 0 { return NOTIFY_OK; }
    op.cmd = XENPF_settime64;
    op.u.settime64.mbz = 0;
    op.u.settime64.secs = now.tv_sec;
    op.u.settime64.nsecs = now.tv_nsec;
    op.u.settime64.system_time = timespec64_to_ns(&system_time);
    let _ = HYPERVISOR_platform_op(&mut op);
    next_sync = now;
    next_sync.tv_sec += 11 * 60;
    NOTIFY_OK
}

static mut xen_pvclock_gtod_notifier: notifier_block = notifier_block { notifier_call: Some(xen_pvclock_gtod_notify) };

unsafe fn xen_starting_cpu(cpu: c_uint) -> c_int {
    let mut info: vcpu_register_vcpu_info = core::mem::zeroed();
    let vcpup: *mut vcpu_info;
    if per_cpu_xen_vcpu(cpu) != core::ptr::null_mut() { goto_after_register_vcpu_info(); return 0; }
    pr_info!("Xen: initializing cpu{}\n", cpu);
    vcpup = per_cpu_ptr(xen_vcpu_info, cpu);
    info.mfn = percpu_to_gfn(vcpup);
    info.offset = xen_offset_in_page(vcpup);
    let err = HYPERVISOR_vcpu_op(VCPUOP_register_vcpu_info, xen_vcpu_nr(cpu), &mut info);
    BUG_ON(err != 0);
    set_per_cpu_xen_vcpu(cpu, vcpup);
    goto_after_register_vcpu_info();
    enable_percpu_irq(xen_events_irq, 0);
    0
}

// C goto target retained as a small local-equivalent marker.
unsafe fn goto_after_register_vcpu_info() {}
unsafe fn xen_dying_cpu(_cpu: c_uint) -> c_int { disable_percpu_irq(xen_events_irq); 0 }

pub unsafe fn xen_reboot(reason: c_int) {
    let mut r = sched_shutdown { reason };
    let rc = HYPERVISOR_sched_op(SCHEDOP_shutdown, &mut r);
    BUG_ON(rc != 0);
}
unsafe fn xen_restart(_nb: *mut notifier_block, _action: c_ulong, _data: *mut c_void) -> c_int { xen_reboot(SHUTDOWN_reboot); NOTIFY_DONE }
static mut xen_restart_nb: notifier_block = notifier_block { notifier_call: Some(xen_restart), priority: 192 };
unsafe fn xen_power_off() { xen_reboot(SHUTDOWN_poweroff); }
unsafe fn xen_arm_callback(_irq: c_int, _arg: *mut c_void) -> irqreturn_t { xen_evtchn_do_upcall(); IRQ_HANDLED }

#[repr(C)]
struct HyperNode { compat: *const c_char, prefix: *const c_char, version: *const c_char, found: bool }
static mut hyper_node: HyperNode = HyperNode { compat: c"xen,xen".as_ptr(), prefix: c"xen,xen-".as_ptr(), version: core::ptr::null(), found: false };

unsafe fn fdt_find_hyper_node(node: c_ulong, uname: *const c_char, depth: c_int, _data: *mut c_void) -> c_int {
    if depth != 1 || strcmp(uname, c"hypervisor".as_ptr()) != 0 { return 0; }
    if of_flat_dt_is_compatible(node, hyper_node.compat) { hyper_node.found = true; }
    let mut len = 0; let s = of_get_flat_dt_prop(node, c"compatible".as_ptr(), &mut len);
    let prefix_len = strlen(hyper_node.prefix);
    if !s.is_null() && len > 0 && strnlen(s, len as usize) < len as usize && len as usize > prefix_len + 3 && strncmp(hyper_node.prefix, s, prefix_len) == 0 { hyper_node.version = s.add(prefix_len); }
    if IS_ENABLED(CONFIG_XEN_EFI) && of_get_flat_dt_subnode_by_name(node, c"uefi".as_ptr()) > 0 && !efi_runtime_disabled() { set_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); }
    0
}

pub unsafe fn xen_early_init() {
    of_scan_flat_dt(Some(fdt_find_hyper_node), core::ptr::null_mut());
    if !hyper_node.found { pr_debug!("No Xen support\n"); return; }
    if hyper_node.version.is_null() { pr_debug!("Xen version not found\n"); return; }
    pr_info!("Xen %s support found\n", hyper_node.version);
    xen_domain_type = XEN_HVM_DOMAIN; xen_setup_features();
    if xen_feature(XENFEAT_dom0) { xen_start_flags |= SIF_INITDOMAIN | SIF_PRIVILEGED; }
    if !console_set_on_cmdline && !xen_initial_domain() { add_preferred_console(c"hvc".as_ptr(), 0, core::ptr::null()); }
}

unsafe fn xen_acpi_guest_init() {
    if !IS_ENABLED(CONFIG_ACPI) { return; }
    let mut a: xen_hvm_param = core::mem::zeroed(); a.domid = DOMID_SELF; a.index = HVM_PARAM_CALLBACK_IRQ;
    if HYPERVISOR_hvm_op(HVMOP_get_param, &mut a) != 0 || (a.value >> 56) != HVM_PARAM_CALLBACK_TYPE_PPI { xen_events_irq = 0; return; }
    let interrupt = a.value & 0xff; let trigger = if (a.value >> 8) & 1 != 0 { ACPI_EDGE_SENSITIVE } else { ACPI_LEVEL_SENSITIVE }; let polarity = if (a.value >> 8) & 2 != 0 { ACPI_ACTIVE_LOW } else { ACPI_ACTIVE_HIGH };
    xen_events_irq = acpi_register_gsi(core::ptr::null_mut(), interrupt as u32, trigger, polarity);
}

unsafe fn xen_dt_guest_init() {
    let xen_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"xen,xen".as_ptr());
    if xen_node.is_null() { pr_err!("Xen support was detected before, but it has disappeared\n"); return; }
    xen_events_irq = irq_of_parse_and_map(xen_node, 0);
    let mut res: resource = core::mem::zeroed(); if of_address_to_resource(xen_node, GRANT_TABLE_INDEX as u32, &mut res) != 0 { pr_err!("Xen grant table region is not found\n"); of_node_put(xen_node); return; }
    of_node_put(xen_node); xen_grant_frames = res.start;
}

unsafe fn xen_guest_init() -> c_int {
    if !xen_domain() { return 0; }
    if !acpi_disabled { xen_acpi_guest_init(); } else { xen_dt_guest_init(); }
    if xen_events_irq == 0 { pr_err!("Xen event channel interrupt not found\n"); return -ENODEV; }
    if efi_enabled(EFI_RUNTIME_SERVICES) { xen_efi_runtime_setup(); }
    let shared = get_zeroed_page(GFP_KERNEL) as *mut shared_info; if shared.is_null() { pr_err!("not enough memory\n"); return -ENOMEM; }
    let mut xatp: xen_add_to_physmap = core::mem::zeroed(); xatp.domid = DOMID_SELF; xatp.idx = 0; xatp.space = XENMAPSPACE_shared_info; xatp.gpfn = virt_to_gfn(shared);
    BUG_ON(HYPERVISOR_memory_op(XENMEM_add_to_physmap, &mut xatp) != 0); HYPERVISOR_shared_info = shared;
    xen_vcpu_info = __alloc_percpu(core::mem::size_of::<vcpu_info>(), 1 << fls((core::mem::size_of::<vcpu_info>() - 1) as c_int)) as *mut vcpu_info; if xen_vcpu_info.is_null() { return -ENOMEM; }
    for_each_possible_cpu!(cpu => set_per_cpu_xen_vcpu_id(cpu, cpu));
    let rc = if xen_grant_frames == 0 { xen_auto_xlat_grant_frames.count = gnttab_max_grant_frames(); xen_xlate_map_ballooned_pages(&mut xen_auto_xlat_grant_frames.pfn, &mut xen_auto_xlat_grant_frames.vaddr, xen_auto_xlat_grant_frames.count) } else { gnttab_setup_auto_xlat_frames(xen_grant_frames) }; if rc != 0 { free_percpu(xen_vcpu_info as *mut c_void); return rc; }
    gnttab_init(); disable_cpuidle(); disable_cpufreq(); xen_init_IRQ();
    if request_percpu_irq(xen_events_irq, Some(xen_arm_callback), c"events".as_ptr(), &raw mut xen_vcpu) != 0 { pr_err!("Error request IRQ %d\n", xen_events_irq); return -EINVAL; }
    if xen_initial_domain() { pvclock_gtod_register_notifier(&raw mut xen_pvclock_gtod_notifier); }
    cpuhp_setup_state(CPUHP_AP_ARM_XEN_STARTING, c"arm/xen:starting".as_ptr(), Some(xen_starting_cpu), Some(xen_dying_cpu))
}

unsafe fn xen_starting_runstate_cpu(cpu: c_uint) -> c_int { xen_setup_runstate_info(cpu); 0 }
unsafe fn xen_late_init() -> c_int { if !xen_domain() { return -ENODEV; } register_platform_power_off(Some(xen_power_off)); register_restart_handler(&raw mut xen_restart_nb); if !xen_initial_domain() { let mut ts = core::mem::zeroed(); xen_read_wallclock(&mut ts); do_settimeofday64(&ts); } if xen_kernel_unmapped_at_usr() { return 0; } xen_time_setup_guest(); cpuhp_setup_state(CPUHP_AP_ARM_XEN_RUNSTATE_STARTING, c"arm/xen_runstate:starting".as_ptr(), Some(xen_starting_runstate_cpu), None) }
pub unsafe fn xen_arch_pre_suspend() {} pub unsafe fn xen_arch_post_suspend(_suspend_cancelled: c_int) {} pub unsafe fn xen_timer_resume() {} pub unsafe fn xen_arch_resume() {} pub unsafe fn xen_arch_suspend() {}

// CONFIG_XEN_UNPOPULATED_ALLOC: the C implementation scans extended DT
// regions, builds xen_resource, inserts unavailable holes, and returns it.
// Its resource/device-node types and allocation helpers are external bindings.
#[allow(dead_code)]
static mut xen_resource: resource = resource { name: c"Xen unused space".as_ptr(), start: 0, end: 0 };

// early_initcall(xen_guest_init); late_initcall(xen_late_init);
// EXPORT_SYMBOL/EXPORT_SYMBOL_GPL declarations for the hypercalls and
// privcmd_call are linker/export metadata supplied by the kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
