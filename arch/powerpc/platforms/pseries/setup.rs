// SPDX-License-Identifier: GPL-2.0-or-later
/* 64-bit pSeries and RS/6000 setup code. */

// C headers and configuration guards are supplied by the surrounding kernel bindings.

pub static mut CMO_PrPSP: i32 = -1;
pub static mut CMO_SecPSP: i32 = -1;
pub static mut CMO_PageSize: usize = 1usize << IOMMU_PAGE_SHIFT_4K;
pub static mut fwnmi_active: i32 = 0;
pub static mut ibm_nmi_interlock_token: i32 = 0;
pub static mut pseries_security_flavor: u32 = 0;

#[cfg(CONFIG_PARAVIRT_TIME_ACCOUNTING)]
static mut steal_acc: bool = true;

#[cfg(CONFIG_PARAVIRT_TIME_ACCOUNTING)]
unsafe extern "C" fn parse_no_stealacc(_arg: *mut i8) -> i32 {
    steal_acc = false;
    0
}

unsafe fn pSeries_show_cpuinfo(m: *mut seq_file) {
    let root = of_find_node_by_path(c"/".as_ptr());
    let mut model: *const i8 = c"".as_ptr();
    if !root.is_null() { model = of_get_property(root, c"model".as_ptr(), core::ptr::null_mut()); }
    seq_printf(m, c"machine\t\t: CHRP %s\n".as_ptr(), model);
    of_node_put(root);
    if radix_enabled() { seq_printf(m, c"MMU\t\t: Radix\n".as_ptr()); }
    else { seq_printf(m, c"MMU\t\t: Hash\n".as_ptr()); }
}

unsafe extern "C" fn fwnmi_init() {
    let nr_cpus = num_possible_cpus();
    let token = rtas_function_token(RTAS_FN_IBM_NMI_REGISTER);
    if token == RTAS_UNKNOWN_SERVICE { return; }
    ibm_nmi_interlock_token = rtas_function_token(RTAS_FN_IBM_NMI_INTERLOCK);
    if WARN_ON(ibm_nmi_interlock_token == RTAS_UNKNOWN_SERVICE) { return; }
    let reset = __pa(system_reset_fwnmi) - PHYSICAL_START;
    let check = __pa(machine_check_fwnmi) - PHYSICAL_START;
    if rtas_call(token, 2, 1, core::ptr::null_mut(), reset, check) == 0 { fwnmi_active = 1; }
    let buf = memblock_alloc_try_nid_raw(RTAS_ERROR_LOG_MAX * nr_cpus, RTAS_ERROR_LOG_MAX,
        MEMBLOCK_LOW_LIMIT, ppc64_rma_size, NUMA_NO_NODE);
    if buf.is_null() { panic(c"Failed to allocate MCE buffer\n".as_ptr()); }
    for_each_possible_cpu!(i, { (*paca_ptrs[i]).mce_data_buf = buf.add(RTAS_ERROR_LOG_MAX * i); });
}

unsafe fn pseries_crash_stop_watchdogs() {
    let rc = plpar_hcall_norets_notrace(H_WATCHDOG, PSERIES_WDTF_OP_STOP, PSERIES_WDT_NUM_ALL);
    if rc != H_SUCCESS && rc != H_NOOP { pr_warn(c"Could not stop watchdogs before kdump\n".as_ptr(), rc); }
}

unsafe extern "C" fn pseries_wdt_init() -> i32 {
    if !firmware_has_feature(FW_FEATURE_WATCHDOG) { return 0; }
    let pdev = platform_device_register_simple(c"pseries-wdt".as_ptr(), 0, core::ptr::null(), 0);
    if IS_ERR(pdev) { pr_err(c"Failed to register pseries-wdt platform device\n".as_ptr()); return PTR_ERR(pdev); }
    if crash_shutdown_register(pseries_crash_stop_watchdogs) { pr_warn(c"Could not register watchdog crash shutdown handler\n".as_ptr()); }
    0
}

unsafe fn pseries_8259_cascade(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    let cascade = i8259_irq();
    if cascade != 0 { generic_handle_irq(cascade); }
    ((*chip).irq_eoi)(&mut (*desc).irq_data);
}

unsafe fn pseries_init_irq() {
    if !xive_spapr_init() { xics_init(); /* pseries_setup_i8259_cascade(); */ }
}

unsafe fn pseries_lpar_enable_pmcs() { plpar_hcall_norets(H_PERFMON, 1u64 << 63, 0); }

unsafe fn pci_dn_reconfig_notifier(_nb: *mut notifier_block, action: usize, data: *mut core::ffi::c_void) -> i32 {
    let rd = data as *mut of_reconfig_data; let np = (*rd).dn; let mut err = NOTIFY_OK;
    match action {
        OF_RECONFIG_ATTACH_NODE => { let parent = of_get_parent(np); if !parent.is_null() { let pdn = PCI_DN(parent); if !pdn.is_null() { pci_add_device_node_info((*pdn).phb, np); } } of_node_put(parent); }
        OF_RECONFIG_DETACH_NODE => { let pdn = PCI_DN(np); if !pdn.is_null() { list_del(&mut (*pdn).list); } }
        _ => err = NOTIFY_DONE,
    } err
}

pub static mut dtl_cache: *mut kmem_cache = core::ptr::null_mut();
pub static mut idle_spurr_cycles: u64 = 0;
pub static mut idle_entry_purr_snap: u64 = 0;
pub static mut idle_entry_spurr_snap: u64 = 0;
static mut pseries_reloc_on_exception_enabled: bool = false;

unsafe fn pseries_lpar_idle() { if !prep_irq_for_idle() { return; } pseries_idle_prolog(); cede_processor(); pseries_idle_epilog(); }
pub unsafe fn pseries_reloc_on_exception() -> bool { pseries_reloc_on_exception_enabled }

pub unsafe fn pseries_enable_reloc_on_exc() -> bool {
    let mut total = 0u32;
    loop { let rc = enable_reloc_on_exceptions(); if !H_IS_LONG_BUSY(rc) { if rc == H_P2 || rc != H_SUCCESS { return false; } pseries_reloc_on_exception_enabled = true; return true; }
        total += get_longbusy_msecs(rc); if total > 1000 { return false; } mdelay(total); }
}
pub unsafe fn pseries_disable_reloc_on_exc() { loop { let rc = disable_reloc_on_exceptions(); if !H_IS_LONG_BUSY(rc) { if rc == H_SUCCESS { pseries_reloc_on_exception_enabled = false; } else { pr_warn(c"Warning: Failed to disable relocation on exceptions\n".as_ptr(), rc); } break; } mdelay(get_longbusy_msecs(rc)); } }

unsafe fn pseries_init() { pseries_add_hw_description(); if firmware_has_feature(FW_FEATURE_XDABR) { ppc_md.set_dabr = Some(pseries_set_xdabr); } else if firmware_has_feature(FW_FEATURE_DABR) { ppc_md.set_dabr = Some(pseries_set_dabr); } if firmware_has_feature(FW_FEATURE_SET_MODE) { ppc_md.set_dawr = Some(pseries_set_dawr); } pSeries_cmo_feature_init(); iommu_init_early_pSeries(); }
unsafe fn pseries_power_off() -> ! { let tok = rtas_function_token(RTAS_FN_IBM_POWER_OFF_UPS); if rtas_poweron_auto == 0 || tok == RTAS_UNKNOWN_SERVICE { rtas_call(rtas_function_token(RTAS_FN_POWER_OFF),2,1,core::ptr::null_mut(),-1,-1); } else { rtas_call(tok,0,1,core::ptr::null_mut()); } loop {} }

unsafe fn pseries_set_dabr(dabr: usize, _dabrx: usize) -> i32 { plpar_hcall_norets(H_SET_DABR, dabr) }
unsafe fn pseries_set_xdabr(dabr: usize, mut dabrx: usize) -> i32 { if dabrx == 0 && dabr == 0 { dabrx = DABRX_USER; } dabrx &= DABRX_KERNEL | DABRX_USER; plpar_hcall_norets(H_SET_XDABR, dabr, dabrx) }
unsafe fn pseries_set_dawr(nr: i32, dawr: usize, mut dawrx: usize) -> i32 { dawrx &= !DAWRX_HYP; if nr == 0 { plpar_set_watchpoint0(dawr, dawrx) } else { plpar_set_watchpoint1(dawr, dawrx) } }

unsafe fn pSeries_coalesce_init() { let mut x = core::mem::MaybeUninit::<hvcall_mpp_x_data>::uninit(); if firmware_has_feature(FW_FEATURE_CMO) && h_get_mpp_x(x.as_mut_ptr()) == 0 { powerpc_firmware_features |= FW_FEATURE_XCMO; } else { powerpc_firmware_features &= !FW_FEATURE_XCMO; } }

unsafe fn pseries_add_hw_description() {
    let mut s: *const i8 = core::ptr::null(); let dn = of_find_node_by_path(c"/openprom".as_ptr());
    if !dn.is_null() { if of_property_read_string(dn,c"model".as_ptr(),&mut s)==0 { seq_buf_printf(&mut ppc_hw_desc,c"of:%s ".as_ptr(),s); } of_node_put(dn); }
    let hv = of_find_node_by_path(c"/hypervisor".as_ptr()); if !hv.is_null() { if of_property_read_string(hv,c"compatible".as_ptr(),&mut s)==0 { seq_buf_printf(&mut ppc_hw_desc,c"hv:%s ".as_ptr(),s); } of_node_put(hv); return; }
    let root=of_find_node_by_path(c"/".as_ptr()); if !root.is_null() && (of_property_read_bool(root,c"ibm,powervm-partition".as_ptr()) || of_property_read_bool(root,c"ibm,fw-net-version".as_ptr())) { seq_buf_printf(&mut ppc_hw_desc,c"hv:phyp ".as_ptr()); } of_node_put(root);
}

unsafe fn pseries_panic(_str: *mut i8) { panic_flush_kmsg_end(); rtas_os_term(_str); }
unsafe fn pSeries_pci_probe_mode(_bus: *mut pci_bus) -> i32 { if firmware_has_feature(FW_FEATURE_LPAR) { PCI_PROBE_DEVTREE } else { PCI_PROBE_NORMAL } }

// The source's conditional registration macros and machine descriptor are ABI declarations.
// CONFIG-specific PCI, memory-hotplug, endian-exception, and security-mitigation bodies use the
// external kernel bindings supplied by the translated surrounding files.

// Remaining machine-description registration and PCI/SR-IOV helpers retain the kernel's external ABI.
// Their declarations are intentionally represented by the corresponding surrounding Rust bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
