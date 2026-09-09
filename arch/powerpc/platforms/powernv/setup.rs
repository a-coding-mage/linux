// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV setup code.
 *
 * Copyright 2011 IBM Corp.
 */

// C includes and build-time configuration supplied by the surrounding kernel
// are intentionally represented by external symbols below.

unsafe fn fw_feature_is(state: *const core::ffi::c_char,
                        name: *const core::ffi::c_char,
                        fw_features: *mut device_node) -> bool {
    let np = of_get_child_by_name(fw_features, name);
    if !np.is_null() {
        let rc = of_property_read_bool(np, state);
        of_node_put(np);
        rc
    } else {
        false
    }
}

unsafe fn init_fw_feat_flags(np: *mut device_node) {
    if fw_feature_is(c"enabled".as_ptr(), c"inst-spec-barrier-ori31,31,0".as_ptr(), np) { security_ftr_set(SEC_FTR_SPEC_BAR_ORI31); }
    if fw_feature_is(c"enabled".as_ptr(), c"fw-bcctrl-serialized".as_ptr(), np) { security_ftr_set(SEC_FTR_BCCTRL_SERIALISED); }
    if fw_feature_is(c"enabled".as_ptr(), c"inst-l1d-flush-ori30,30,0".as_ptr(), np) { security_ftr_set(SEC_FTR_L1D_FLUSH_ORI30); }
    if fw_feature_is(c"enabled".as_ptr(), c"inst-l1d-flush-trig2".as_ptr(), np) { security_ftr_set(SEC_FTR_L1D_FLUSH_TRIG2); }
    if fw_feature_is(c"enabled".as_ptr(), c"fw-l1d-thread-split".as_ptr(), np) { security_ftr_set(SEC_FTR_L1D_THREAD_PRIV); }
    if fw_feature_is(c"enabled".as_ptr(), c"fw-count-cache-disabled".as_ptr(), np) { security_ftr_set(SEC_FTR_COUNT_CACHE_DISABLED); }
    if fw_feature_is(c"enabled".as_ptr(), c"fw-count-cache-flush-bcctr2,0,0".as_ptr(), np) { security_ftr_set(SEC_FTR_BCCTR_FLUSH_ASSIST); }
    if fw_feature_is(c"enabled".as_ptr(), c"needs-count-cache-flush-on-context-switch".as_ptr(), np) { security_ftr_set(SEC_FTR_FLUSH_COUNT_CACHE); }

    /* These are enabled by default; clear them when firmware disables them. */
    if fw_feature_is(c"disabled".as_ptr(), c"speculation-policy-favor-security".as_ptr(), np) { security_ftr_clear(SEC_FTR_FAVOUR_SECURITY); }
    if fw_feature_is(c"disabled".as_ptr(), c"needs-l1d-flush-msr-pr-0-to-1".as_ptr(), np) { security_ftr_clear(SEC_FTR_L1D_FLUSH_PR); }
    if fw_feature_is(c"disabled".as_ptr(), c"needs-l1d-flush-msr-hv-1-to-0".as_ptr(), np) { security_ftr_clear(SEC_FTR_L1D_FLUSH_HV); }
    if fw_feature_is(c"disabled".as_ptr(), c"needs-spec-barrier-for-bound-checks".as_ptr(), np) { security_ftr_clear(SEC_FTR_BNDS_CHK_SPEC_BAR); }
    if fw_feature_is(c"enabled".as_ptr(), c"no-need-l1d-flush-msr-pr-1-to-0".as_ptr(), np) { security_ftr_clear(SEC_FTR_L1D_FLUSH_ENTRY); }
    if fw_feature_is(c"enabled".as_ptr(), c"no-need-l1d-flush-kernel-on-user-access".as_ptr(), np) { security_ftr_clear(SEC_FTR_L1D_FLUSH_UACCESS); }
    if fw_feature_is(c"enabled".as_ptr(), c"no-need-store-drain-on-priv-state-switch".as_ptr(), np) { security_ftr_clear(SEC_FTR_STF_BARRIER); }
}

unsafe fn pnv_setup_security_mitigations() {
    let mut ty = L1D_FLUSH_FALLBACK;
    let np = of_find_node_by_name(core::ptr::null_mut(), c"ibm,opal".as_ptr());
    let fw_features = of_get_child_by_name(np, c"fw-features".as_ptr());
    of_node_put(np);
    if !fw_features.is_null() {
        init_fw_feat_flags(fw_features);
        of_node_put(fw_features);
        if security_ftr_enabled(SEC_FTR_L1D_FLUSH_TRIG2) { ty = L1D_FLUSH_MTTRIG; }
        if security_ftr_enabled(SEC_FTR_L1D_FLUSH_ORI30) { ty = L1D_FLUSH_ORI; }
    }
    if pvr_version_is(PVR_POWER7) || pvr_version_is(PVR_POWER7p) || pvr_version_is(PVR_POWER8E) || pvr_version_is(PVR_POWER8NVL) || pvr_version_is(PVR_POWER8) {
        security_ftr_clear(SEC_FTR_L1D_FLUSH_ENTRY);
        security_ftr_clear(SEC_FTR_L1D_FLUSH_UACCESS);
    }
    let mut enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) && (security_ftr_enabled(SEC_FTR_L1D_FLUSH_PR) || security_ftr_enabled(SEC_FTR_L1D_FLUSH_HV));
    setup_rfi_flush(ty, enable);
    setup_count_cache_flush();
    enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) && security_ftr_enabled(SEC_FTR_L1D_FLUSH_ENTRY);
    setup_entry_flush(enable);
    enable = security_ftr_enabled(SEC_FTR_FAVOUR_SECURITY) && security_ftr_enabled(SEC_FTR_L1D_FLUSH_UACCESS);
    setup_uaccess_flush(enable);
    setup_stf_barrier();
}

unsafe fn pnv_check_guarded_cores() {
    let mut bad_count = 0;
    for_each_node_by_type!(dn, c"cpu", {
        if of_property_match_string(dn, c"status".as_ptr(), c"bad".as_ptr()) >= 0 { bad_count += 1; }
    });
    if bad_count != 0 {
        printk(c"  _     _______________\n".as_ptr());
        pr_cont(c" | |   /               \\\n".as_ptr());
        pr_cont(c" | |   |    WARNING!   |\n".as_ptr());
        pr_cont(c" | |   |               |\n".as_ptr());
        pr_cont(c" | |   | It looks like |\n".as_ptr());
        pr_cont(c" |_|   |  you have %*d |\n".as_ptr(), 3, bad_count);
        pr_cont(c"  _    | guarded cores |\n".as_ptr());
        pr_cont(c" (_)   \\_______________/\n".as_ptr());
    }
}

unsafe fn pnv_setup_arch() {
    set_arch_panic_timeout(10, ARCH_PANIC_TIMEOUT);
    pnv_setup_security_mitigations();
    pnv_smp_init();
    if firmware_has_feature(FW_FEATURE_OPAL) { opal_nvram_init(); }
    powersave_nap = 1;
    pnv_check_guarded_cores();
    pnv_rng_init();
}

unsafe fn pnv_add_hw_description() {
    let dn = of_find_node_by_path(c"/ibm,opal/firmware".as_ptr());
    if dn.is_null() { return; }
    let mut s: *const core::ffi::c_char = core::ptr::null();
    if of_property_read_string(dn, c"version".as_ptr(), &mut s) == 0 || of_property_read_string(dn, c"git-id".as_ptr(), &mut s) == 0 { seq_buf_printf(&mut ppc_hw_desc, c"opal:%s ".as_ptr(), s); }
    if of_property_read_string(dn, c"mi-version".as_ptr(), &mut s) == 0 { seq_buf_printf(&mut ppc_hw_desc, c"mi:%s ".as_ptr(), s); }
    of_node_put(dn);
}

unsafe fn pnv_init() {
    pnv_add_hw_description();
    opal_lpc_init();
    if firmware_has_feature(FW_FEATURE_OPAL) { hvc_opal_init_early(); } else { add_preferred_console(c"hvc".as_ptr(), 0, core::ptr::null_mut()); }
    // CONFIG_PPC_64S_HASH_MMU: allocate per-CPU SLB storage when radix is disabled.
    if !radix_enabled() {
        let size = core::mem::size_of::<slb_entry>() * mmu_slb_size;
        for_each_possible_cpu!(i, { paca_ptrs[i].mce_faulty_slbs = memblock_alloc_node(size, core::mem::align_of::<slb_entry>(), cpu_to_node(i)); });
    }
}

unsafe fn pnv_init_IRQ() { if !xive_native_init() { xics_init(); } WARN_ON(!ppc_md.get_irq); }

unsafe fn pnv_show_cpuinfo(m: *mut seq_file) {
    let root = of_find_node_by_path(c"/".as_ptr());
    let model = if !root.is_null() { of_get_property(root, c"model".as_ptr(), core::ptr::null_mut()) } else { c"".as_ptr() };
    seq_printf(m, c"machine\t\t: PowerNV %s\n".as_ptr(), model);
    if firmware_has_feature(FW_FEATURE_OPAL) { seq_printf(m, c"firmware\t: OPAL\n".as_ptr()); } else { seq_printf(m, c"firmware\t: BML\n".as_ptr()); }
    of_node_put(root);
    if radix_enabled() { seq_printf(m, c"MMU\t\t: Radix\n".as_ptr()); } else { seq_printf(m, c"MMU\t\t: Hash\n".as_ptr()); }
}

unsafe fn pnv_prepare_going_down() { opal_event_shutdown(); opal_flash_update_print_message(); smp_send_stop(); hard_irq_disable(); }

unsafe fn pnv_restart(mut cmd: *mut core::ffi::c_char) -> ! {
    pnv_prepare_going_down();
    loop {
        let mut rc = if cmd.is_null() || *cmd == 0 { opal_cec_reboot() } else if strcmp(cmd, c"full".as_ptr()) == 0 { opal_cec_reboot2(OPAL_REBOOT_FULL_IPL, core::ptr::null_mut()) } else if strcmp(cmd, c"mpipl".as_ptr()) == 0 { opal_cec_reboot2(OPAL_REBOOT_MPIPL, core::ptr::null_mut()) } else if strcmp(cmd, c"error".as_ptr()) == 0 { opal_cec_reboot2(OPAL_REBOOT_PLATFORM_ERROR, core::ptr::null_mut()) } else if strcmp(cmd, c"fast".as_ptr()) == 0 { opal_cec_reboot2(OPAL_REBOOT_FAST, core::ptr::null_mut()) } else { OPAL_UNSUPPORTED };
        if rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT { opal_poll_events(core::ptr::null_mut()); mdelay(10); }
        else if !cmd.is_null() && rc != 0 { if rc == OPAL_UNSUPPORTED { pr_err(c"Unsupported '%s' reboot.\n".as_ptr(), cmd); } else { pr_err(c"Unable to issue '%s' reboot. Err=%ld\n".as_ptr(), cmd, rc); } pr_info(c"Forcing a cec-reboot\n".as_ptr()); cmd = core::ptr::null_mut(); rc = OPAL_BUSY; }
        else if rc != OPAL_SUCCESS { pr_err(c"Unable to reboot. Err=%ld\n".as_ptr(), rc); }
        if rc != OPAL_BUSY && rc != OPAL_BUSY_EVENT { break; }
    }
    loop { opal_poll_events(core::ptr::null_mut()); }
}

unsafe fn pnv_power_off() -> ! { pnv_prepare_going_down(); let mut rc = OPAL_BUSY; while rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT { rc = opal_cec_power_down(0); if rc == OPAL_BUSY_EVENT { opal_poll_events(core::ptr::null_mut()); } else { mdelay(10); } } loop { opal_poll_events(core::ptr::null_mut()); } }
unsafe fn pnv_halt() -> ! { pnv_power_off(); }
unsafe fn pnv_progress(_s: *mut core::ffi::c_char, _hex: u16) {}
unsafe fn pnv_shutdown() { pnv_pci_shutdown(); opal_shutdown(); }

// CONFIG_KEXEC_CORE: the following hooks return CPUs to OPAL before kexec.
unsafe fn pnv_kexec_wait_secondaries_down() { let my_cpu = raw_smp_processor_id(); let mut notified = -1; for_each_online_cpu!(i, { if i != my_cpu { let mut status = 0u8; let mut timeout = 1000i64; loop { let rc = opal_query_cpu_status(get_hard_smp_processor_id(i), &mut status); if rc != OPAL_SUCCESS || status != OPAL_THREAD_STARTED { break; } barrier(); if i != notified { printk(KERN_INFO, c"kexec: waiting for cpu %d (physical %d) to enter OPAL\n".as_ptr(), i, paca_ptrs[i].hw_cpu_id); notified = i; } mdelay(1); if timeout == 0 { printk(KERN_ERR, c"kexec: timed out waiting for cpu %d (physical %d) to enter OPAL\n".as_ptr(), i, paca_ptrs[i].hw_cpu_id); break; } timeout -= 1; } } }); }

unsafe fn pnv_kexec_cpu_down(_crash_shutdown: i32, secondary: i32) {
    let mut reinit_flags: u64;
    if xive_enabled() { xive_teardown_cpu(); } else { xics_kexec_teardown_cpu(secondary); }
    if !firmware_has_feature(FW_FEATURE_OPAL) { return; }
    if secondary != 0 { mb(); (*get_paca()).kexec_state = KEXEC_STATE_REAL_MODE; mb(); opal_return_cpu(); }
    else { pnv_kexec_wait_secondaries_down(); if xive_enabled() { xive_shutdown(); } reinit_flags = OPAL_REINIT_CPUS_HILE_BE; if cpu_has_feature(CPU_FTR_ARCH_300) { reinit_flags |= OPAL_REINIT_CPUS_MMU_RADIX | OPAL_REINIT_CPUS_MMU_HASH; } opal_reinit_cpus(reinit_flags); }
}

// CONFIG_MEMORY_HOTPLUG
unsafe fn pnv_memory_block_size() -> usize { memory_block_size }

unsafe fn pnv_setup_machdep_opal() {
    ppc_md.get_boot_time = opal_get_boot_time; ppc_md.restart = pnv_restart; pm_power_off = Some(pnv_power_off); ppc_md.halt = pnv_halt; ppc_md.machine_check_exception = opal_machine_check; ppc_md.mce_check_early_recovery = opal_mce_check_early_recovery;
    if opal_check_token(OPAL_HANDLE_HMI2) { ppc_md.hmi_exception_early = opal_hmi_exception_early2; } else { ppc_md.hmi_exception_early = opal_hmi_exception_early; }
    ppc_md.handle_hmi_exception = opal_handle_hmi_exception;
}
unsafe fn pnv_probe() -> i32 { if firmware_has_feature(FW_FEATURE_OPAL) { pnv_setup_machdep_opal(); } pr_debug(c"PowerNV detected !\n".as_ptr()); pnv_init(); 1 }

// CONFIG_PPC_TRANSACTIONAL_MEM
unsafe fn pnv_tm_init() { if !firmware_has_feature(FW_FEATURE_OPAL) || !pvr_version_is(PVR_POWER9) || early_cpu_has_feature(CPU_FTR_TM) { return; } if opal_reinit_cpus(OPAL_REINIT_CPUS_TM_SUSPEND_DISABLED) != OPAL_SUCCESS { return; } pr_info(c"Enabling TM (Transactional Memory) with Suspend Disabled\n".as_ptr()); cur_cpu_spec.cpu_features |= CPU_FTR_TM; cur_cpu_spec.cpu_user_features2 &= !PPC_FEATURE2_HTM; cur_cpu_spec.cpu_user_features2 |= PPC_FEATURE2_HTM_NO_SUSPEND | PPC_FEATURE2_HTM_NOSC; tm_suspend_disabled = true; }

/* Returns the cpu frequency for `cpu` in Hz. This is used by /proc/cpuinfo. */
unsafe fn pnv_get_proc_freq(cpu: u32) -> usize { let mut ret_freq = cpufreq_get(cpu) * 1000usize; if ret_freq == 0 { ret_freq = ppc_proc_freq; } ret_freq }
unsafe fn pnv_machine_check_early(regs: *mut pt_regs) -> i64 { if !cur_cpu_spec.is_null() && !(*cur_cpu_spec).machine_check_early.is_none() { (*cur_cpu_spec).machine_check_early.unwrap()(regs) } else { 0 } }

// define_machine(powernv)
#[allow(non_upper_case_globals)]
static mut powernv: machine_desc = machine_desc {
    name: c"PowerNV".as_ptr(), compatible: c"ibm,powernv".as_ptr(), probe: Some(pnv_probe), setup_arch: Some(pnv_setup_arch), init_IRQ: Some(pnv_init_IRQ), show_cpuinfo: Some(pnv_show_cpuinfo), get_proc_freq: Some(pnv_get_proc_freq), discover_phbs: Some(pnv_pci_init), progress: Some(pnv_progress), machine_shutdown: Some(pnv_shutdown), power_save: None, machine_check_early: Some(pnv_machine_check_early),
    // CONFIG_KEXEC_CORE: kexec_cpu_down: Some(pnv_kexec_cpu_down)
    // CONFIG_MEMORY_HOTPLUG: memory_block_size: Some(pnv_memory_block_size)
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
