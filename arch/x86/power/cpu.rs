// SPDX-License-Identifier: GPL-2.0-only
/*
 * Suspend support specific for i386/x86-64.
 *
 * Copyright (c) 2007 Rafael J. Wysocki <rjw@sisk.pl>
 * Copyright (c) 2002 Pavel Machek <pavel@ucw.cz>
 * Copyright (c) 2001 Patrick Mochel <mochel@osdl.org>
 */

// Linux and x86 headers supplied by the surrounding translation unit.

#[cfg(CONFIG_X86_32)]
#[no_mangle]
pub static mut saved_context_ebx: c_ulong = 0;
#[cfg(CONFIG_X86_32)]
#[no_mangle]
pub static mut saved_context_esp: c_ulong = 0;
#[cfg(CONFIG_X86_32)]
#[no_mangle]
pub static mut saved_context_ebp: c_ulong = 0;
#[cfg(CONFIG_X86_32)]
#[no_mangle]
pub static mut saved_context_esi: c_ulong = 0;
#[cfg(CONFIG_X86_32)]
#[no_mangle]
pub static mut saved_context_edi: c_ulong = 0;
#[cfg(CONFIG_X86_32)]
#[no_mangle]
pub static mut saved_context_eflags: c_ulong = 0;

#[no_mangle]
pub static mut saved_context: saved_context = unsafe { core::mem::zeroed() };

unsafe fn msr_save_context(ctxt: *mut saved_context) {
    let mut msr = (*ctxt).saved_msrs.array;
    let end = msr.add((*ctxt).saved_msrs.num as usize);
    while msr < end {
        if (*msr).valid { rdmsrq((*msr).info.msr_no, (*msr).info.reg.q); }
        msr = msr.add(1);
    }
}

unsafe fn msr_restore_context(ctxt: *mut saved_context) {
    let mut msr = (*ctxt).saved_msrs.array;
    let end = msr.add((*ctxt).saved_msrs.num as usize);
    while msr < end {
        if (*msr).valid { wrmsrq((*msr).info.msr_no, (*msr).info.reg.q); }
        msr = msr.add(1);
    }
}

unsafe fn __save_processor_state(ctxt: *mut saved_context) {
    #[cfg(CONFIG_X86_32)] { mtrr_save_fixed_ranges(core::ptr::null_mut()); }
    kernel_fpu_begin();
    store_idt(&mut (*ctxt).idt);
    (*ctxt).gdt_desc.size = GDT_SIZE - 1;
    (*ctxt).gdt_desc.address = get_cpu_gdt_rw(smp_processor_id()) as c_ulong;
    store_tr((*ctxt).tr);
    savesegment!(gs, (*ctxt).gs);
    #[cfg(CONFIG_X86_64)] {
        savesegment!(fs, (*ctxt).fs); savesegment!(ds, (*ctxt).ds); savesegment!(es, (*ctxt).es);
        rdmsrq(MSR_FS_BASE, (*ctxt).fs_base);
        rdmsrq(MSR_GS_BASE, (*ctxt).kernelmode_gs_base);
        rdmsrq(MSR_KERNEL_GS_BASE, (*ctxt).usermode_gs_base);
        mtrr_save_fixed_ranges(core::ptr::null_mut());
        rdmsrq(MSR_EFER, (*ctxt).efer);
    }
    (*ctxt).cr0 = read_cr0(); (*ctxt).cr2 = read_cr2(); (*ctxt).cr3 = __read_cr3(); (*ctxt).cr4 = __read_cr4();
    (*ctxt).misc_enable_saved = !rdmsrq_safe(MSR_IA32_MISC_ENABLE, &mut (*ctxt).misc_enable);
    msr_save_context(ctxt);
}

#[no_mangle]
pub unsafe extern "C" fn save_processor_state() { __save_processor_state(&raw mut saved_context); x86_platform.save_sched_clock_state(); }

unsafe fn do_fpu_end() { kernel_fpu_end(); }

unsafe fn fix_processor_context() {
    let cpu = smp_processor_id();
    #[cfg(CONFIG_X86_64)] let desc = get_cpu_gdt_rw(cpu);
    set_tss_desc(cpu, &mut get_cpu_entry_area(cpu).tss.x86_tss);
    #[cfg(CONFIG_X86_64)] {
        let mut tss: tss_desc = core::mem::zeroed();
        memcpy(&mut tss as *mut _ as *mut c_void, desc.add(GDT_ENTRY_TSS), core::mem::size_of::<tss_desc>());
        tss.type_ = 0x9; write_gdt_entry(desc, GDT_ENTRY_TSS, &tss, DESC_TSS); syscall_init();
    }
    #[cfg(CONFIG_X86_32)] if boot_cpu_has(X86_FEATURE_SEP) { enable_sep_cpu(); }
    load_TR_desc(); load_mm_ldt(current.active_mm); initialize_tlbstate_and_flush(); fpu__resume_cpu(); load_fixmap_gdt(cpu);
}

unsafe fn __restore_processor_state(ctxt: *mut saved_context) {
    if (*ctxt).misc_enable_saved { wrmsrq(MSR_IA32_MISC_ENABLE, (*ctxt).misc_enable); }
    #[cfg(CONFIG_X86_32)] if (*ctxt).cr4 != 0 { __write_cr4((*ctxt).cr4); }
    #[cfg(CONFIG_X86_64)] { wrmsrq(MSR_EFER, (*ctxt).efer); __write_cr4((*ctxt).cr4); }
    write_cr3((*ctxt).cr3); write_cr2((*ctxt).cr2); write_cr0((*ctxt).cr0); load_idt(&(*ctxt).idt);
    loadsegment!(ss, __KERNEL_DS); loadsegment!(ds, __USER_DS); loadsegment!(es, __USER_DS);
    #[cfg(CONFIG_X86_64)] {
        wrmsrq(MSR_GS_BASE, (*ctxt).kernelmode_gs_base);
        if (*ctxt).cr4 & X86_CR4_FRED != 0 { cpu_init_fred_exceptions(); cpu_init_fred_rsps(); }
    }
    #[cfg(CONFIG_X86_32)] loadsegment!(fs, __KERNEL_PERCPU);
    fix_processor_context();
    #[cfg(CONFIG_X86_64)] {
        loadsegment!(ds, (*ctxt).es); loadsegment!(es, (*ctxt).es); loadsegment!(fs, (*ctxt).fs); load_gs_index((*ctxt).gs);
        wrmsrq(MSR_FS_BASE, (*ctxt).fs_base); wrmsrq(MSR_KERNEL_GS_BASE, (*ctxt).usermode_gs_base);
    }
    #[cfg(CONFIG_X86_32)] loadsegment!(gs, (*ctxt).gs);
    do_fpu_end(); tsc_verify_tsc_adjust(true); x86_platform.restore_sched_clock_state(); cache_bp_restore(); perf_restore_debug_store();
    let c = &mut cpu_data(smp_processor_id()); if cpu_has(c, X86_FEATURE_MSR_IA32_FEAT_CTL) { init_ia32_feat_ctl(c); }
    microcode_bsp_resume(); msr_restore_context(ctxt);
}

#[no_mangle]
pub unsafe extern "C" fn restore_processor_state() { __restore_processor_state(&raw mut saved_context); }

#[cfg(all(CONFIG_HIBERNATION, CONFIG_HOTPLUG_CPU))]
unsafe fn resume_play_dead() -> ! { play_dead_common(); tboot_shutdown(TB_SHUTDOWN_WFS); hlt_play_dead(); }

#[cfg(all(CONFIG_HIBERNATION, CONFIG_HOTPLUG_CPU))]
pub unsafe extern "C" fn hibernate_resume_nonboot_cpu_disable() -> c_int {
    let play_dead = smp_ops.play_dead; let mut ret = cpuhp_smt_enable(); if ret != 0 { return ret; }
    smp_ops.play_dead = Some(resume_play_dead); ret = freeze_secondary_cpus(0); smp_ops.play_dead = play_dead; ret
}

unsafe fn bsp_check() -> c_int { if cpumask_first(cpu_online_mask) != 0 { pr_warn!("CPU0 is offline.\n"); return -ENODEV; } 0 }
unsafe fn bsp_pm_callback(_nb: *mut notifier_block, action: c_ulong, _ptr: *mut c_void) -> c_int {
    let ret = match action { PM_SUSPEND_PREPARE | PM_HIBERNATION_PREPARE => bsp_check(), _ => 0 }; notifier_from_errno(ret)
}
unsafe fn bsp_pm_check_init() -> c_int { pm_notifier(Some(bsp_pm_callback), -INT_MAX); 0 }
core_initcall!(bsp_pm_check_init);

unsafe fn msr_build_context(msr_id: *const u32, num: c_int) -> c_int {
    let saved_msrs = &mut saved_context.saved_msrs; let total_num = saved_msrs.num + num;
    let msr_array = kmalloc_objs::<saved_msr>(total_num); if msr_array.is_null() { pr_err!("x86/pm: Can not allocate memory to save/restore MSRs during suspend.\n"); return -ENOMEM; }
    if !saved_msrs.array.is_null() { memcpy(msr_array as *mut c_void, saved_msrs.array as *const c_void, core::mem::size_of::<saved_msr>() * saved_msrs.num as usize); kfree(saved_msrs.array as *mut c_void); }
    for j in 0..num { let i = saved_msrs.num + j; (*msr_array.add(i as usize)).info.msr_no = *msr_id.add(j as usize); let mut dummy = 0; (*msr_array.add(i as usize)).valid = !rdmsrq_safe(*msr_id.add(j as usize), &mut dummy); (*msr_array.add(i as usize)).info.reg.q = 0; }
    saved_msrs.num = total_num; saved_msrs.array = msr_array; 0
}

// BIOS quirk callbacks and registration tables retain the source interface.
unsafe fn msr_initialize_bdw(d: *const dmi_system_id) -> c_int { let ids = [MSR_IA32_THERM_CONTROL]; pr_info!("x86/pm: {} detected, MSR saving is needed during suspending.\n", (*d).ident); msr_build_context(ids.as_ptr(), 1) }
static msr_save_dmi_table: [dmi_system_id; 2] = [dmi_system_id { callback: Some(msr_initialize_bdw), ident: "BROADWELL BDX_EP\0".as_ptr() as *const c_char, matches: [] }, unsafe { core::mem::zeroed() }];
unsafe fn msr_save_cpuid_features(c: *const x86_cpu_id) -> c_int { let ids = [MSR_AMD64_CPUID_FN_1]; pr_info!("x86/pm: family %#hx cpu detected, MSR saving is needed during suspending.\n", (*c).family); msr_build_context(ids.as_ptr(), 1) }
static msr_save_cpu_table: [x86_cpu_id; 3] = [X86_MATCH_VENDOR_FAM!(AMD, 0x15, msr_save_cpuid_features), X86_MATCH_VENDOR_FAM!(AMD, 0x16, msr_save_cpuid_features), unsafe { core::mem::zeroed() }];
unsafe fn pm_cpu_check(_c: *const x86_cpu_id) -> c_int { let m = x86_match_cpu(msr_save_cpu_table.as_ptr()); if !m.is_null() { return core::mem::transmute::<*const c_void, unsafe fn(*const x86_cpu_id) -> c_int>((*m).driver_data)(m); } 0 }
unsafe fn pm_save_spec_msr() { let e = [(MSR_IA32_SPEC_CTRL,X86_FEATURE_MSR_SPEC_CTRL),(MSR_IA32_TSX_CTRL,X86_FEATURE_MSR_TSX_CTRL),(MSR_TSX_FORCE_ABORT,X86_FEATURE_TSX_FORCE_ABORT),(MSR_IA32_MCU_OPT_CTRL,X86_FEATURE_SRBDS_CTRL),(MSR_AMD64_LS_CFG,X86_FEATURE_LS_CFG_SSBD),(MSR_AMD64_DE_CFG,X86_FEATURE_LFENCE_RDTSC)]; for &(msr, feature) in &e { if boot_cpu_has(feature) { msr_build_context(&msr, 1); } } }
unsafe fn pm_check_save_msr() -> c_int { dmi_check_system(msr_save_dmi_table.as_ptr()); pm_cpu_check(core::ptr::null()); pm_save_spec_msr(); 0 }
device_initcall!(pm_check_save_msr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
