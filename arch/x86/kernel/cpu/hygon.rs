// SPDX-License-Identifier: GPL-2.0+
/*
 * Hygon Processor Support for Linux
 *
 * Copyright (C) 2018 Chengdu Haiguang IC Design Co., Ltd.
 *
 * Author: Pu Wen <puwen@hygon.cn>
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[cfg(feature = "CONFIG_NUMA")]
unsafe fn nearby_node(apicid: i32) -> i32 {
    let mut i = apicid - 1;
    while i >= 0 {
        let node = __apicid_to_node[i as usize];
        if node != NUMA_NO_NODE && node_online(node) {
            return node;
        }
        i -= 1;
    }
    i = apicid + 1;
    while i < MAX_LOCAL_APIC {
        let node = __apicid_to_node[i as usize];
        if node != NUMA_NO_NODE && node_online(node) {
            return node;
        }
        i += 1;
    }
    first_node(node_online_map) // Shouldn't happen
}

unsafe fn srat_detect_node(c: *mut cpuinfo_x86) {
    #[cfg(feature = "CONFIG_NUMA")]
    {
        let cpu = smp_processor_id();
        let mut node;
        let apicid = (*c).topo.apicid;

        node = numa_cpu_node(cpu);
        if node == NUMA_NO_NODE {
            node = (*c).topo.llc_id;
        }

        if x86_cpuinit.fixup_cpu_id.is_some() {
            (x86_cpuinit.fixup_cpu_id.unwrap())(c, node);
        }

        if !node_online(node) {
            let ht_nodeid = (*c).topo.initial_apicid;
            if __apicid_to_node[ht_nodeid as usize] != NUMA_NO_NODE {
                node = __apicid_to_node[ht_nodeid as usize];
            }
            if !node_online(node) {
                node = nearby_node(apicid);
            }
        }
        numa_set_node(cpu, node);
    }
}

unsafe fn bsp_init_hygon(c: *mut cpuinfo_x86) {
    if cpu_has(c, X86_FEATURE_CONSTANT_TSC) {
        let mut val: u64 = 0;
        rdmsrq(MSR_K7_HWCR, &mut val);
        if val & BIT(24) == 0 {
            pr_warn!("TSC doesn't count with P0 frequency!\n");
        }
    }
    if cpu_has(c, X86_FEATURE_MWAITX) {
        use_mwaitx_delay();
    }
    if !boot_cpu_has(X86_FEATURE_AMD_SSBD) && !boot_cpu_has(X86_FEATURE_VIRT_SSBD) {
        if !rdmsrq_safe(MSR_AMD64_LS_CFG, &mut x86_amd_ls_cfg_base) {
            setup_force_cpu_cap(X86_FEATURE_LS_CFG_SSBD);
            setup_force_cpu_cap(X86_FEATURE_SSBD);
            x86_amd_ls_cfg_ssbd_mask = 1u64 << 10;
        }
    }
    resctrl_cpu_detect(c);
}

unsafe fn early_init_hygon(c: *mut cpuinfo_x86) {
    let mut val: u64 = 0;
    set_cpu_cap(c, X86_FEATURE_K8);
    rdmsrq_safe(MSR_AMD64_PATCH_LEVEL, &mut val);
    (*c).microcode = val as u32;
    if (*c).x86_power & (1 << 8) != 0 {
        set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
        set_cpu_cap(c, X86_FEATURE_NONSTOP_TSC);
    }
    if (*c).x86_power & BIT(12) != 0 { set_cpu_cap(c, X86_FEATURE_ACC_POWER); }
    if (*c).x86_power & BIT(14) != 0 { set_cpu_cap(c, X86_FEATURE_RAPL); }
    #[cfg(feature = "CONFIG_X86_64")]
    set_cpu_cap(c, X86_FEATURE_SYSCALL32);
    #[cfg(all(feature = "CONFIG_X86_LOCAL_APIC", feature = "CONFIG_PCI"))]
    if boot_cpu_has(X86_FEATURE_APIC) { set_cpu_cap(c, X86_FEATURE_EXTD_APICID); }
    set_cpu_cap(c, X86_FEATURE_VMMCALL);
}

unsafe fn init_hygon(c: *mut cpuinfo_x86) {
    let mut vm_cr: u64 = 0;
    early_init_hygon(c);
    set_cpu_cap(c, X86_FEATURE_REP_GOOD);
    set_cpu_cap(c, X86_FEATURE_ZEN);
    set_cpu_cap(c, X86_FEATURE_CPB);
    cpu_detect_cache_sizes(c);
    srat_detect_node(c);
    init_hygon_cacheinfo(c);
    if cpu_has(c, X86_FEATURE_SVM) {
        rdmsrq(MSR_VM_CR, &mut vm_cr);
        if vm_cr & SVM_VM_CR_SVM_DIS_MASK != 0 {
            pr_notice_once!("SVM disabled (by BIOS) in MSR_VM_CR\n");
            clear_cpu_cap(c, X86_FEATURE_SVM);
        }
    }
    if cpu_has(c, X86_FEATURE_XMM2) {
        msr_set_bit(MSR_AMD64_DE_CFG, MSR_AMD64_DE_CFG_LFENCE_SERIALIZE_BIT);
        set_cpu_cap(c, X86_FEATURE_LFENCE_RDTSC);
    }
    set_cpu_cap(c, X86_FEATURE_ARAT);
    if !cpu_feature_enabled(X86_FEATURE_XENPV) {
        set_cpu_bug(c, X86_BUG_SYSRET_SS_ATTRS);
    }
    check_null_seg_clears_base(c);
    clear_cpu_cap(c, X86_FEATURE_APIC_MSRS_FENCE);
}

unsafe fn cpu_detect_tlb_hygon(c: *mut cpuinfo_x86) {
    let mut ebx: u32 = 0;
    let mut eax: u32 = 0;
    let mut ecx: u32 = 0;
    let mut edx: u32 = 0;
    let mask: u32 = 0xfff;
    if (*c).extended_cpuid_level < 0x80000006 { return; }
    cpuid(0x80000006, &mut eax, &mut ebx, &mut ecx, &mut edx);
    tlb_lld_4k = (ebx >> 16) & mask;
    tlb_lli_4k = ebx & mask;
    if ((eax >> 16) & mask) == 0 { tlb_lld_2m = (cpuid_eax(0x80000005) >> 16) & 0xff; }
    else { tlb_lld_2m = (eax >> 16) & mask; }
    tlb_lld_4m = tlb_lld_2m >> 1;
    if (eax & mask) == 0 {
        cpuid(0x80000005, &mut eax, &mut ebx, &mut ecx, &mut edx);
        tlb_lli_2m = eax & 0xff;
    } else { tlb_lli_2m = eax & mask; }
    tlb_lli_4m = tlb_lli_2m >> 1;
}

static hygon_cpu_dev: cpu_dev = cpu_dev {
    c_vendor: "Hygon",
    c_ident: ["HygonGenuine"],
    c_early_init: Some(early_init_hygon),
    c_detect_tlb: Some(cpu_detect_tlb_hygon),
    c_bsp_init: Some(bsp_init_hygon),
    c_init: Some(init_hygon),
    c_x86_vendor: X86_VENDOR_HYGON,
};

cpu_dev_register(hygon_cpu_dev);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
