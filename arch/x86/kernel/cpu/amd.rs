// SPDX-License-Identifier: GPL-2.0-only
// Translated from amd.c; Linux kernel dependencies are supplied externally.

pub static mut invlpgb_count_max: u16 = 1;

#[inline]
unsafe fn rdmsrq_amd_safe(msr: u32, p: *mut u64) -> i32 {
    let mut gprs = [0u32; 8];
    WARN_ONCE(boot_cpu_data.x86 != 0xf, "%s should only be used on K8!\n", "rdmsrq_amd_safe");
    gprs[1] = msr;
    gprs[7] = 0x9c5a203a;
    let err = rdmsr_safe_regs(gprs.as_mut_ptr());
    *p = gprs[0] as u64 | ((gprs[2] as u64) << 32);
    err
}

#[inline]
unsafe fn wrmsrq_amd_safe(msr: u32, val: u64) -> i32 {
    let mut gprs = [0u32; 8];
    WARN_ONCE(boot_cpu_data.x86 != 0xf, "%s should only be used on K8!\n", "wrmsrq_amd_safe");
    gprs[0] = val as u32;
    gprs[1] = msr;
    gprs[2] = (val >> 32) as u32;
    gprs[7] = 0x9c5a203a;
    wrmsr_safe_regs(gprs.as_mut_ptr())
}

#[cfg(CONFIG_X86_32)]
extern "C" { fn vide(); }

unsafe fn init_amd_k5(c: *mut cpuinfo_x86) {
    #[cfg(CONFIG_X86_32)] {
        const CBAR: u32 = 0xfffc;
        const CBAR_ENB: u32 = 0x80000000;
        const CBAR_KEY: u32 = 0x000000cb;
        if (*c).x86_model == 9 || (*c).x86_model == 10 {
            if inl(CBAR) & CBAR_ENB != 0 { outl(CBAR_KEY, CBAR); }
        }
    }
}

unsafe fn init_amd_k6(c: *mut cpuinfo_x86) {
    #[cfg(CONFIG_X86_32)] {
        let mut val: msr = core::mem::zeroed();
        let mut mbytes = get_num_physpages() >> (20 - PAGE_SHIFT);
        if (*c).x86_model < 6 {
            if (*c).x86_model == 0 { clear_cpu_cap(c, X86_FEATURE_APIC); set_cpu_cap(c, X86_FEATURE_PGE); }
            return;
        }
        if (*c).x86_model == 6 && (*c).x86_stepping == 1 {
            const K6_BUG_LOOP: i32 = 1_000_000;
            pr_info("AMD K6 stepping B detected - ");
            let f_vide: unsafe extern "C" fn() = vide;
            OPTIMIZER_HIDE_VAR(f_vide);
            let d = rdtsc();
            let mut n = K6_BUG_LOOP;
            while n != 0 { f_vide(); n -= 1; }
            let d = rdtsc() - d;
            if d > 20 * K6_BUG_LOOP as u64 { pr_cont("system stability may be impaired when more than 32 MB are used.\n"); }
            else { pr_cont("probably OK (after B9730xxxx).\n"); }
        }
        if (*c).x86_model < 8 || ((*c).x86_model == 8 && (*c).x86_stepping < 8) {
            if mbytes > 508 { mbytes = 508; }
            rdmsrq(MSR_K6_WHCR, val.q);
            if val.l & 0x0000ffff == 0 {
                let mut flags: c_ulong = 0;
                val.l = 1 | ((mbytes / 4) << 1);
                local_irq_save(&mut flags); wbinvd(); wrmsrq(MSR_K6_WHCR, val.q); local_irq_restore(flags);
                pr_info("Enabling old style K6 write allocation for %d Mb\n", mbytes);
            }
            return;
        }
        if ((*c).x86_model == 8 && (*c).x86_stepping > 7) || (*c).x86_model == 9 || (*c).x86_model == 13 {
            if mbytes > 4092 { mbytes = 4092; }
            rdmsrq(MSR_K6_WHCR, val.q);
            if val.l & 0xffff0000 == 0 {
                let mut flags: c_ulong = 0;
                val.l = ((mbytes >> 2) << 22) | (1 << 16);
                local_irq_save(&mut flags); wbinvd(); wrmsrq(MSR_K6_WHCR, val.q); local_irq_restore(flags);
                pr_info("Enabling new style K6 write allocation for %d Mb\n", mbytes);
            }
            return;
        }
        if (*c).x86_model == 10 { return; }
    }
}

unsafe fn init_amd_k7(c: *mut cpuinfo_x86) {
    #[cfg(CONFIG_X86_32)] {
        let mut val: msr = core::mem::zeroed();
        if (*c).x86_model >= 6 && (*c).x86_model <= 10 && !cpu_has(c, X86_FEATURE_XMM) {
            pr_info("Enabling disabled K7/SSE Support.\n"); msr_clear_bit(MSR_K7_HWCR, 15); set_cpu_cap(c, X86_FEATURE_XMM);
        }
        if ((*c).x86_model == 8 && (*c).x86_stepping >= 1) || (*c).x86_model > 8 {
            rdmsrq(MSR_K7_CLK_CTL, val.q);
            if val.l & 0xfff00000 != 0x20000000 { pr_info("CPU: CLK_CTL MSR was %x. Reprogramming to %x\n", val.l, (val.l & 0x000fffff) | 0x20000000); val.l = (val.l & 0x000fffff) | 0x20000000; wrmsrq(MSR_K7_CLK_CTL, val.q); }
        }
        if (*c).cpu_index == 0 { return; }
        if (*c).x86_model == 6 && (*c).x86_stepping <= 1 { return; }
        if (*c).x86_model == 7 && (*c).x86_stepping == 0 { return; }
        if ((*c).x86_model == 6 && (*c).x86_stepping >= 2) || ((*c).x86_model == 7 && (*c).x86_stepping >= 1) || (*c).x86_model > 7 { if cpu_has(c, X86_FEATURE_MP) { return; } }
        WARN_ONCE(true, "WARNING: This combination of AMD processors is not suitable for SMP.\n");
        add_taint(TAINT_CPU_OUT_OF_SPEC, LOCKDEP_NOW_UNRELIABLE);
    }
}

#[cfg(CONFIG_NUMA)]
unsafe fn nearby_node(apicid: i32) -> i32 {
    let mut i = apicid - 1;
    while i >= 0 { let node = __apicid_to_node[i as usize]; if node != NUMA_NO_NODE && node_online(node) { return node; } i -= 1; }
    i = apicid + 1;
    while i < MAX_LOCAL_APIC { let node = __apicid_to_node[i as usize]; if node != NUMA_NO_NODE && node_online(node) { return node; } i += 1; }
    first_node(node_online_map)
}

unsafe fn srat_detect_node(c: *mut cpuinfo_x86) {
    #[cfg(CONFIG_NUMA)] {
        let cpu = smp_processor_id();
        let mut node = numa_cpu_node(cpu);
        if node == NUMA_NO_NODE { node = per_cpu_llc_id(cpu); }
        if let Some(fixup) = x86_cpuinit.fixup_cpu_id { fixup(c, node); }
        if !node_online(node) {
            let ht_nodeid = (*c).topo.initial_apicid;
            if __apicid_to_node[ht_nodeid as usize] != NUMA_NO_NODE { node = __apicid_to_node[ht_nodeid as usize]; }
            if !node_online(node) { node = nearby_node((*c).topo.apicid as i32); }
        }
        numa_set_node(cpu, node);
    }
}

unsafe fn bsp_determine_snp(c: *mut cpuinfo_x86) {
    #[cfg(CONFIG_ARCH_HAS_CC_PLATFORM)] {
        cc_vendor = CC_VENDOR_AMD;
        if cpu_has(c, X86_FEATURE_SEV_SNP) {
            if !cpu_has(c, X86_FEATURE_HYPERVISOR) && (cpu_feature_enabled(X86_FEATURE_ZEN3) || cpu_feature_enabled(X86_FEATURE_ZEN4) || cpu_feature_enabled(X86_FEATURE_RMPREAD)) && snp_probe_rmptable_info() { cc_platform_set(CC_ATTR_HOST_SEV_SNP); }
            else { setup_clear_cpu_cap(X86_FEATURE_SEV_SNP); cc_platform_clear(CC_ATTR_HOST_SEV_SNP); }
        }
    }
}

// The following declarations preserve the remaining file-local implementation and
// its external kernel interfaces in a literal low-level form.
static mut rdrand_force: bool = false;

unsafe fn rdrand_cmdline(str_: *const c_char) -> i32 {
    if str_.is_null() { return -EINVAL; }
    if strcmp(str_, c"force".as_ptr()) == 0 { rdrand_force = true; 0 } else { -EINVAL }
}

unsafe fn clear_rdrand_cpuid_bit(c: *mut cpuinfo_x86) {
    if !IS_ENABLED(CONFIG_PM_SLEEP) { return; }
    if cpuid_ecx(1) & BIT(30) == 0 || rdrand_force { return; }
    msr_clear_bit(MSR_AMD64_CPUID_FN_1, 62);
    if cpuid_ecx(1) & BIT(30) != 0 { pr_info_once("BIOS may not properly restore RDRAND after suspend, but hypervisor does not support hiding RDRAND via CPUID.\n"); return; }
    clear_cpu_cap(c, X86_FEATURE_RDRAND);
    pr_info_once("BIOS may not properly restore RDRAND after suspend, hiding RDRAND via CPUID. Use rdrand=force to reenable.\n");
}

unsafe fn init_amd_jg(c: *mut cpuinfo_x86) { clear_rdrand_cpuid_bit(c); }

unsafe fn init_amd_bd(c: *mut cpuinfo_x86) {
    let mut value = 0u64;
    if (*c).x86_model >= 0x02 && (*c).x86_model < 0x20 && rdmsrq_safe(MSR_F15H_IC_CFG, &mut value) == 0 && value & 0x1e == 0 { value |= 0x1e; wrmsrq_safe(MSR_F15H_IC_CFG, value); }
    clear_rdrand_cpuid_bit(c);
}

unsafe fn init_amd_zen_common() { setup_force_cpu_cap(X86_FEATURE_ZEN); #[cfg(CONFIG_NUMA)] { node_reclaim_distance = 32; } }
unsafe fn init_amd_zen1(c: *mut cpuinfo_x86) { fix_erratum_1386(c); if !cpu_has(c,X86_FEATURE_HYPERVISOR) && !cpu_has(c,X86_FEATURE_CPB) { set_cpu_cap(c,X86_FEATURE_CPB); } setup_force_cpu_bug(X86_BUG_DIV0); if (*c).x86_model < 0x30 { msr_clear_bit(MSR_K7_HWCR,MSR_K7_HWCR_IRPERF_EN_BIT); clear_cpu_cap(c,X86_FEATURE_IRPERF); } msr_set_bit(MSR_AMD64_FP_CFG,MSR_AMD64_FP_CFG_ZEN1_DENORM_FIX_BIT); }
unsafe fn init_amd_zen2(c: *mut cpuinfo_x86) { init_spectral_chicken(c); fix_erratum_1386(c); zen2_zenbleed_check(c); if (*c).x86_model==0x47 && (*c).x86_stepping==0 { clear_cpu_cap(c,X86_FEATURE_RDSEED); msr_clear_bit(MSR_AMD64_CPUID_FN_7,18); pr_emerg!("RDSEED is not reliable on this platform; disabling.\n"); } clear_cpu_cap(c,X86_FEATURE_INVLPGB); if !cpu_has(c,X86_FEATURE_HYPERVISOR) { msr_set_bit(MSR_ZEN4_BP_CFG,MSR_ZEN2_BP_CFG_BUG_FIX_BIT); } }
unsafe fn init_amd_zen3(c: *mut cpuinfo_x86) { if !cpu_has(c,X86_FEATURE_HYPERVISOR) && !cpu_has(c,X86_FEATURE_BTC_NO) { set_cpu_cap(c,X86_FEATURE_BTC_NO); } }
unsafe fn init_amd_zen4(c: *mut cpuinfo_x86) { if !cpu_has(c,X86_FEATURE_HYPERVISOR) { msr_set_bit(MSR_ZEN4_BP_CFG,MSR_ZEN4_BP_CFG_SHARED_BTB_FIX_BIT); } if ((*c).x86_model>=0x18&&(*c).x86_model<=0x1f)||((*c).x86_model>=0x60&&(*c).x86_model<=0x7f) { clear_cpu_cap(c,X86_FEATURE_V_VMSAVE_VMLOAD); } }
unsafe fn init_amd_zen5(c: *mut cpuinfo_x86) { if !x86_match_min_microcode_rev(zen5_rdseed_microcode) { clear_cpu_cap(c,X86_FEATURE_RDSEED); msr_clear_bit(MSR_AMD64_CPUID_FN_7,18); pr_emerg_once!("RDSEED32 is broken. Disabling the corresponding CPUID bit.\n"); } }

unsafe fn init_amd(c: *mut cpuinfo_x86) {
    early_init_amd(c); if (*c).x86>=0x10 { set_cpu_cap(c,X86_FEATURE_REP_GOOD); } if cpu_has(c,X86_FEATURE_FSRM) { set_cpu_cap(c,X86_FEATURE_FSRS); } if (*c).x86<6 { clear_cpu_cap(c,X86_FEATURE_MCE); }
    match (*c).x86 { 4=>init_amd_k5(c),5=>init_amd_k6(c),6=>init_amd_k7(c),0xf=>init_amd_k8(c),0x10=>init_amd_gh(c),0x12=>init_amd_ln(c),0x15=>init_amd_bd(c),0x16=>init_amd_jg(c),_=>{} }
    if (*c).x86>=0x17 { init_amd_zen_common(); } if boot_cpu_has(X86_FEATURE_ZEN1){init_amd_zen1(c)}else if boot_cpu_has(X86_FEATURE_ZEN2){init_amd_zen2(c)}else if boot_cpu_has(X86_FEATURE_ZEN3){init_amd_zen3(c)}else if boot_cpu_has(X86_FEATURE_ZEN4){init_amd_zen4(c)}else if boot_cpu_has(X86_FEATURE_ZEN5){init_amd_zen5(c)}
    if (*c).x86>=6&&!cpu_has(c,X86_FEATURE_XSAVEERPTR){set_cpu_bug(c,X86_BUG_FXSAVE_LEAK)} cpu_detect_cache_sizes(c); srat_detect_node(c); init_amd_cacheinfo(c);
    if cpu_has(c,X86_FEATURE_SVM){let mut vm_cr=0;rdmsrq(MSR_VM_CR,&mut vm_cr);if vm_cr&SVM_VM_CR_SVM_DIS_MASK!=0{clear_cpu_cap(c,X86_FEATURE_SVM);}}
    if !cpu_has(c,X86_FEATURE_LFENCE_RDTSC)&&cpu_has(c,X86_FEATURE_XMM2){msr_set_bit(MSR_AMD64_DE_CFG,MSR_AMD64_DE_CFG_LFENCE_SERIALIZE_BIT);set_cpu_cap(c,X86_FEATURE_LFENCE_RDTSC)} if (*c).x86>0x11{set_cpu_cap(c,X86_FEATURE_ARAT)} if !cpu_has(c,X86_FEATURE_3DNOWPREFETCH)&&(cpu_has(c,X86_FEATURE_3DNOW)||cpu_has(c,X86_FEATURE_LM)){set_cpu_cap(c,X86_FEATURE_3DNOWPREFETCH)} if !cpu_feature_enabled(X86_FEATURE_XENPV){set_cpu_bug(c,X86_BUG_SYSRET_SS_ATTRS)} if cpu_has(c,X86_FEATURE_IRPERF){msr_set_bit(MSR_K7_HWCR,MSR_K7_HWCR_IRPERF_EN_BIT)} check_null_seg_clears_base(c); clear_cpu_cap(c,X86_FEATURE_APIC_MSRS_FENCE); if cpu_has(c,X86_FEATURE_TCE){msr_set_bit(MSR_EFER,_EFER_TCE)}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
