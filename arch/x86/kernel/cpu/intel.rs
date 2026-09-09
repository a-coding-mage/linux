// SPDX-License-Identifier: GPL-2.0

// C headers and build-time configuration are supplied by the surrounding kernel.

unsafe fn check_memory_type_self_snoop_errata(c: *mut cpuinfo_x86) {
    match (*c).x86_vfm {
        INTEL_CORE_YONAH | INTEL_CORE2_MEROM | INTEL_CORE2_MEROM_L |
        INTEL_CORE2_PENRYN | INTEL_CORE2_DUNNINGTON | INTEL_NEHALEM |
        INTEL_NEHALEM_G | INTEL_NEHALEM_EP | INTEL_NEHALEM_EX |
        INTEL_WESTMERE | INTEL_WESTMERE_EP | INTEL_SANDYBRIDGE =>
            setup_clear_cpu_cap(X86_FEATURE_SELFSNOOP),
        _ => {}
    }
}

static mut ring3mwait_disabled: bool = false;
unsafe fn ring3mwait_disable(_: *mut i8) -> i32 { ring3mwait_disabled = true; 1 }
// __setup("ring3mwait=disable", ring3mwait_disable)

unsafe fn probe_xeon_phi_r3mwait(c: *mut cpuinfo_x86) {
    if (*c).x86 != 6 { return; }
    match (*c).x86_vfm { INTEL_XEON_PHI_KNL | INTEL_XEON_PHI_KNM => {}, _ => return }
    if ring3mwait_disabled { return; }
    set_cpu_cap(c, X86_FEATURE_RING3MWAIT);
    this_cpu_or(msr_misc_features_shadow, 1u64 << MSR_MISC_FEATURES_ENABLES_RING3MWAIT_BIT);
    if c == &mut boot_cpu_data { ELF_HWCAP2 |= HWCAP2_RING3MWAIT; }
}

#[repr(C)]
struct sku_microcode { vfm: u32, stepping: u8, microcode: u32 }
static spectre_bad_microcodes: &[sku_microcode] = &[
    sku_microcode{vfm:INTEL_KABYLAKE,stepping:0x0b,microcode:0x80}, sku_microcode{vfm:INTEL_KABYLAKE,stepping:0x0a,microcode:0x80}, sku_microcode{vfm:INTEL_KABYLAKE,stepping:0x09,microcode:0x80},
    sku_microcode{vfm:INTEL_KABYLAKE_L,stepping:0x0a,microcode:0x80}, sku_microcode{vfm:INTEL_KABYLAKE_L,stepping:0x09,microcode:0x80}, sku_microcode{vfm:INTEL_SKYLAKE_X,stepping:3,microcode:0x0100013e}, sku_microcode{vfm:INTEL_SKYLAKE_X,stepping:4,microcode:0x0200003c},
    sku_microcode{vfm:INTEL_BROADWELL,stepping:4,microcode:0x28}, sku_microcode{vfm:INTEL_BROADWELL_G,stepping:1,microcode:0x1b}, sku_microcode{vfm:INTEL_BROADWELL_D,stepping:2,microcode:0x14}, sku_microcode{vfm:INTEL_BROADWELL_D,stepping:3,microcode:0x07000011}, sku_microcode{vfm:INTEL_BROADWELL_X,stepping:1,microcode:0x0b000025},
    sku_microcode{vfm:INTEL_HASWELL_L,stepping:1,microcode:0x21}, sku_microcode{vfm:INTEL_HASWELL_G,stepping:1,microcode:0x18}, sku_microcode{vfm:INTEL_HASWELL,stepping:3,microcode:0x23}, sku_microcode{vfm:INTEL_HASWELL_X,stepping:2,microcode:0x3b}, sku_microcode{vfm:INTEL_HASWELL_X,stepping:4,microcode:0x10}, sku_microcode{vfm:INTEL_IVYBRIDGE_X,stepping:4,microcode:0x42a},
    sku_microcode{vfm:INTEL_SANDYBRIDGE_X,stepping:6,microcode:0x61b}, sku_microcode{vfm:INTEL_SANDYBRIDGE_X,stepping:7,microcode:0x712},
];
unsafe fn bad_spectre_microcode(c: *mut cpuinfo_x86) -> bool {
    if cpu_has(c, X86_FEATURE_HYPERVISOR) { return false; }
    for x in spectre_bad_microcodes { if (*c).x86_vfm == x.vfm && (*c).x86_stepping == x.stepping { return (*c).microcode <= x.microcode; } }
    false
}

const MSR_IA32_TME_ACTIVATE: u32 = 0x982;
#[inline] fn TME_ACTIVATE_LOCKED(x:u64)->bool { x & 1 != 0 }
#[inline] fn TME_ACTIVATE_ENABLED(x:u64)->bool { x & 2 != 0 }
#[inline] fn TME_ACTIVATE_KEYID_BITS(x:u64)->i32 { ((x >> 32) & 0xf) as i32 }
unsafe fn detect_tme_early(c:*mut cpuinfo_x86) { let mut x=0u64; rdmsrq(MSR_IA32_TME_ACTIVATE,&mut x); if !TME_ACTIVATE_LOCKED(x)||!TME_ACTIVATE_ENABLED(x) { pr_info_once!("x86/tme: not enabled by BIOS\n"); clear_cpu_cap(c,X86_FEATURE_TME); return } pr_info_once!("x86/tme: enabled by BIOS\n"); let k=TME_ACTIVATE_KEYID_BITS(x); if k==0{return} (*c).x86_phys_bits-=k; pr_info_once!("x86/mktme: BIOS enabled: x86_phys_bits reduced by %d\n",k); }

pub unsafe fn intel_unlock_cpuid_leafs(c:*mut cpuinfo_x86) { if boot_cpu_data.x86_vendor!=X86_VENDOR_INTEL||(*c).x86_vfm<INTEL_PENTIUM_M_DOTHAN{return} if msr_clear_bit(MSR_IA32_MISC_ENABLE,MSR_IA32_MISC_ENABLE_LIMIT_CPUID_BIT)>0 {(*c).cpuid_level=cpuid_eax(0);} }
unsafe fn intel_cpuid_vfm()->u32 { let e=cpuid_eax(1); IFM(x86_family(e),x86_model(e)) }
pub unsafe fn intel_get_platform_id()->u32 { let mut v=[0u32;2]; if x86_hypervisor_present||intel_cpuid_vfm()<=INTEL_PENTIUM_II_KLAMATH{return 0} native_rdmsr(MSR_IA32_PLATFORM_ID,&mut v[0],&mut v[1]); (v[1]>>18)&7 }

unsafe fn early_init_intel(c:*mut cpuinfo_x86) {
    if (*c).x86>=6&&!cpu_has(c,X86_FEATURE_IA64){(*c).microcode=intel_get_microcode_revision();} (*c).intel_platform_id=intel_get_platform_id();
    if (cpu_has(c,X86_FEATURE_SPEC_CTRL)||cpu_has(c,X86_FEATURE_INTEL_STIBP)||cpu_has(c,X86_FEATURE_IBRS)||cpu_has(c,X86_FEATURE_IBPB)||cpu_has(c,X86_FEATURE_STIBP))&&bad_spectre_microcode(c){pr_warn!("Intel Spectre v2 broken microcode detected; disabling Speculation Control\n"); for f in [X86_FEATURE_IBRS,X86_FEATURE_IBPB,X86_FEATURE_STIBP,X86_FEATURE_SPEC_CTRL,X86_FEATURE_MSR_SPEC_CTRL,X86_FEATURE_INTEL_STIBP,X86_FEATURE_SSBD,X86_FEATURE_SPEC_CTRL_SSBD]{setup_clear_cpu_cap(f);}}
    if (*c).x86_vfm==INTEL_ATOM_BONNELL&&(*c).x86_stepping<=2&&(*c).microcode<0x20e{pr_warn!("Atom PSE erratum detected, BIOS microcode update recommended\n");clear_cpu_cap(c,X86_FEATURE_PSE);}
    if (*c).x86_vfm==INTEL_P4_PRESCOTT&&((*c).x86_stepping==3||(*c).x86_stepping==4){(*c).x86_phys_bits=36;}
    if (*c).x86_power&(1<<8)!=0{set_cpu_cap(c,X86_FEATURE_CONSTANT_TSC);set_cpu_cap(c,X86_FEATURE_NONSTOP_TSC);}else if ((*c).x86_vfm>=INTEL_P4_PRESCOTT&&(*c).x86_vfm<=INTEL_P4_CEDARMILL)||((*c).x86_vfm>=INTEL_CORE_YONAH&&(*c).x86_vfm<=INTEL_IVYBRIDGE){set_cpu_cap(c,X86_FEATURE_CONSTANT_TSC);}
    match (*c).x86_vfm{INTEL_ATOM_SALTWELL_MID|INTEL_ATOM_SALTWELL_TABLET|INTEL_ATOM_SILVERMONT_MID|INTEL_ATOM_AIRMONT_NP=>set_cpu_cap(c,X86_FEATURE_NONSTOP_TSC_S3),_-> {}}
    if (*c).x86_vfm>=INTEL_PENTIUM_PRO&&(*c).x86_vfm<=INTEL_CORE_YONAH{clear_cpu_cap(c,X86_FEATURE_PAT);}
    if (*c).x86_vfm>=INTEL_PENTIUM_M_DOTHAN{let mut m=0;rdmsrq(MSR_IA32_MISC_ENABLE,&mut m);if m&MSR_IA32_MISC_ENABLE_FAST_STRING!=0{set_cpu_cap(c,X86_FEATURE_REP_GOOD);}else{pr_info!("Disabled fast string operations\n");setup_clear_cpu_cap(X86_FEATURE_REP_GOOD);setup_clear_cpu_cap(X86_FEATURE_ERMS);}}
    if (*c).x86_vfm==INTEL_QUARK_X1000{pr_info!("Disabling PGE capability bit\n");setup_clear_cpu_cap(X86_FEATURE_PGE);} check_memory_type_self_snoop_errata(c); if cpu_has(c,X86_FEATURE_TME){detect_tme_early(c);}
}
unsafe fn bsp_init_intel(c:*mut cpuinfo_x86){resctrl_cpu_detect(c);}

unsafe fn intel_smp_check(c:*mut cpuinfo_x86){if (*c).cpu_index==0{return} if (*c).x86_vfm>=INTEL_FAM5_START&&(*c).x86_vfm<INTEL_PENTIUM_MMX&&(*c).x86_stepping>=1&&(*c).x86_stepping<=4{WARN_ONCE!(true,"WARNING: SMP operation may be unreliablewith B stepping processors.\n");}}
static mut forcepae:i32=0;
unsafe fn forcepae_setup(_: *mut i8)->i32{forcepae=1;1}
// __setup("forcepae", forcepae_setup)
unsafe fn intel_workarounds(c:*mut cpuinfo_x86){if IS_ENABLED!(CONFIG_X86_F00F_BUG)&&(*c).x86_vfm>=INTEL_FAM5_START&&(*c).x86_vfm<INTEL_QUARK_X1000{set_cpu_bug(c,X86_BUG_F00F);} if ((*c).x86_vfm==INTEL_PENTIUM_II_KLAMATH&&(*c).x86_stepping<3)||(*c).x86_vfm<INTEL_PENTIUM_II_KLAMATH{clear_cpu_cap(c,X86_FEATURE_SEP);} if forcepae!=0{pr_warn!("PAE forced!\n");set_cpu_cap(c,X86_FEATURE_PAE);add_taint(TAINT_CPU_OUT_OF_SPEC,LOCKDEP_NOW_UNRELIABLE);} if (*c).x86_vfm==INTEL_P4_WILLAMETTE&&(*c).x86_stepping==1&&msr_set_bit(MSR_IA32_MISC_ENABLE,MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE_BIT)>0{pr_info!("CPU: C0 stepping P4 Xeon detected.\n");pr_info!("CPU: Disabling hardware prefetching (Erratum 037)\n");} if boot_cpu_has(X86_FEATURE_APIC)&&(*c).x86_vfm==INTEL_PENTIUM_75&&((*c).x86_stepping<6||(*c).x86_stepping==0xb){set_cpu_bug(c,X86_BUG_11AP);} intel_smp_check(c);}
unsafe fn srat_detect_node(c:*mut cpuinfo_x86){#[cfg(CONFIG_NUMA)]{let cpu=smp_processor_id();let mut node=numa_cpu_node(cpu);if node==NUMA_NO_NODE||!node_online(node){node=cpu_to_node(cpu);}numa_set_node(cpu,node);}}
unsafe fn init_cpuid_fault(c:*mut cpuinfo_x86){let mut m=0;if rdmsrq_safe(MSR_PLATFORM_INFO,&mut m)==0&&m&MSR_PLATFORM_INFO_CPUID_FAULT!=0{set_cpu_cap(c,X86_FEATURE_CPUID_FAULT);}}
unsafe fn init_intel_misc_features(c:*mut cpuinfo_x86){let mut m=0;if rdmsrq_safe(MSR_MISC_FEATURES_ENABLES,&mut m)!=0{return}this_cpu_write(msr_misc_features_shadow,0);init_cpuid_fault(c);probe_xeon_phi_r3mwait(c);m=this_cpu_read(msr_misc_features_shadow);wrmsrq(MSR_MISC_FEATURES_ENABLES,m);}
unsafe fn init_intel(c:*mut cpuinfo_x86){early_init_intel(c);intel_workarounds(c);init_intel_cacheinfo(c);if (*c).cpuid_level>9{let e=cpuid_eax(10);if e&0xff!=0&&((e>>8)&0xff)>1{set_cpu_cap(c,X86_FEATURE_ARCH_PERFMON);}}if cpu_has(c,X86_FEATURE_XMM2){set_cpu_cap(c,X86_FEATURE_LFENCE_RDTSC);}if boot_cpu_has(X86_FEATURE_DS){let mut l=0;rdmsrq(MSR_IA32_MISC_ENABLE,&mut l);if l&MSR_IA32_MISC_ENABLE_BTS_UNAVAIL==0{set_cpu_cap(c,X86_FEATURE_BTS);}if l&MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL==0{set_cpu_cap(c,X86_FEATURE_PEBS);}}if boot_cpu_has(X86_FEATURE_CLFLUSH)&&((*c).x86_vfm==INTEL_CORE2_DUNNINGTON||(*c).x86_vfm==INTEL_NEHALEM_EX||(*c).x86_vfm==INTEL_WESTMERE_EX){set_cpu_bug(c,X86_BUG_CLFLUSH_MONITOR);}if boot_cpu_has(X86_FEATURE_MWAIT)&&((*c).x86_vfm==INTEL_ATOM_GOLDMONT||(*c).x86_vfm==INTEL_LUNARLAKE_M){set_cpu_bug(c,X86_BUG_MONITOR);}if x86_match_cpu(zmm_exclusion_list){set_cpu_cap(c,X86_FEATURE_PREFER_YMM);}srat_detect_node(c);init_ia32_feat_ctl(c);init_intel_misc_features(c);split_lock_init();intel_init_thermal(c);}
unsafe fn intel_tlb_lookup(d:*const leaf_0x2_table){let e=(*d).entries;match (*d).t_type{STLB_4K=>{tlb_lli_4k=max(tlb_lli_4k,e);tlb_lld_4k=max(tlb_lld_4k,e)},STLB_4K_2M=>{tlb_lli_4k=max(tlb_lli_4k,e);tlb_lld_4k=max(tlb_lld_4k,e);tlb_lli_2m=max(tlb_lli_2m,e);tlb_lld_2m=max(tlb_lld_2m,e);tlb_lli_4m=max(tlb_lli_4m,e);tlb_lld_4m=max(tlb_lld_4m,e)},TLB_INST_4K=>tlb_lli_4k=max(tlb_lli_4k,e),TLB_INST_4M=>tlb_lli_4m=max(tlb_lli_4m,e),TLB_DATA_4K|TLB_DATA0_4K=>tlb_lld_4k=max(tlb_lld_4k,e),TLB_DATA_4M|TLB_DATA0_4M=>tlb_lld_4m=max(tlb_lld_4m,e),TLB_DATA_1G=>tlb_lld_1g=max(tlb_lld_1g,e),_=>{}}}
unsafe fn intel_detect_tlb(c:*mut cpuinfo_x86){if (*c).cpuid_level<2{return}let mut regs=leaf_0x2_regs::default();cpuid_leaf_0x2(&mut regs);for_each_cpuid_0x2_desc!(regs,|d|intel_tlb_lookup(d));}
// struct cpu_dev intel_cpu_dev and cpu_dev_register(intel_cpu_dev)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
