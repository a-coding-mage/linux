// SPDX-License-Identifier: GPL-2.0+
/*
 * Hygon Processor Support for Linux
 *
 * Copyright (C) 2018 Chengdu Haiguang IC Design Co., Ltd.
 *
 * Author: Pu Wen <puwen@hygon.cn>
 */
// Dependencies: linux/io.h, asm/apic.h, asm/cpu.h, asm/cpuid/api.h,
// asm/smp.h, asm/numa.h, asm/cacheinfo.h, asm/spec-ctrl.h, asm/delay.h,
// asm/msr.h, asm/resctrl.h, "cpu.h".

use core::ffi::c_int;
#[cfg(CONFIG_NUMA)]
use core::ffi::c_uint;
use core::ptr::addr_of_mut;

/*
 * To workaround broken NUMA config.  Read the comment in
 * srat_detect_node().
 */
#[cfg(CONFIG_NUMA)]
unsafe fn nearby_node(apicid: c_int) -> c_int {
    let mut i = apicid - 1;
    while i >= 0 {
        let node = __apicid_to_node[i as usize] as c_int;
        if node != NUMA_NO_NODE && node_online(node) {
            return node;
        }
        i -= 1;
    }
    i = apicid + 1;
    while i < MAX_LOCAL_APIC as c_int {
        let node = __apicid_to_node[i as usize] as c_int;
        if node != NUMA_NO_NODE && node_online(node) {
            return node;
        }
        i += 1;
    }
    first_node(node_online_map) /* Shouldn't happen */
}

#[allow(unused_variables)]
unsafe fn srat_detect_node(c: *mut cpuinfo_x86) {
    #[cfg(CONFIG_NUMA)]
    {
        let cpu: c_int = smp_processor_id();
        let apicid: c_uint = (*c).topo.apicid;

        let mut node: c_int = numa_cpu_node(cpu);
        if node == NUMA_NO_NODE {
            node = (*c).topo.llc_id as c_int;
        }

        /*
         * On multi-fabric platform (e.g. Numascale NumaChip) a
         * platform-specific handler needs to be called to fixup some
         * IDs of the CPU.
         */
        if let Some(fixup_cpu_id) = x86_cpuinit.fixup_cpu_id {
            fixup_cpu_id(c, node);
        }

        if !node_online(node) {
            /*
             * Two possibilities here:
             *
             * - The CPU is missing memory and no node was created.  In
             *   that case try picking one from a nearby CPU.
             *
             * - The APIC IDs differ from the HyperTransport node IDs.
             *   Assume they are all increased by a constant offset, but
             *   in the same order as the HT nodeids.  If that doesn't
             *   result in a usable node fall back to the path for the
             *   previous case.
             *
             * This workaround operates directly on the mapping between
             * APIC ID and NUMA node, assuming certain relationship
             * between APIC ID, HT node ID and NUMA topology.  As going
             * through CPU mapping may alter the outcome, directly
             * access __apicid_to_node[].
             */
            let ht_nodeid = (*c).topo.initial_apicid as usize;

            if __apicid_to_node[ht_nodeid] as c_int != NUMA_NO_NODE {
                node = __apicid_to_node[ht_nodeid] as c_int;
            }
            /* Pick a nearby node */
            if !node_online(node) {
                node = nearby_node(apicid as c_int);
            }
        }
        numa_set_node(cpu, node);
    }
}

unsafe extern "C" fn bsp_init_hygon(c: *mut cpuinfo_x86) {
    if cpu_has(c, X86_FEATURE_CONSTANT_TSC) {
        let mut val: u64 = 0;

        rdmsrq(MSR_K7_HWCR, &mut val);
        if val & (1 << 24) == 0 {
            pr_warn!("[Firmware Bug]: TSC doesn't count with P0 frequency!\n");
        }
    }

    if cpu_has(c, X86_FEATURE_MWAITX) {
        use_mwaitx_delay();
    }

    if !boot_cpu_has(X86_FEATURE_AMD_SSBD) && !boot_cpu_has(X86_FEATURE_VIRT_SSBD) {
        /*
         * Try to cache the base value so further operations can
         * avoid RMW. If that faults, do not enable SSBD.
         */
        if rdmsrq_safe(MSR_AMD64_LS_CFG, addr_of_mut!(x86_amd_ls_cfg_base)) == 0 {
            setup_force_cpu_cap(X86_FEATURE_LS_CFG_SSBD);
            setup_force_cpu_cap(X86_FEATURE_SSBD);
            x86_amd_ls_cfg_ssbd_mask = 1u64 << 10;
        }
    }

    resctrl_cpu_detect(c);
}

unsafe extern "C" fn early_init_hygon(c: *mut cpuinfo_x86) {
    let mut val: u64 = 0;

    set_cpu_cap(c, X86_FEATURE_K8);

    rdmsrq_safe(MSR_AMD64_PATCH_LEVEL, &mut val);
    (*c).microcode = val as u32;

    /*
     * c->x86_power is 8000_0007 edx. Bit 8 is TSC runs at constant rate
     * with P/T states and does not stop in deep C-states
     */
    if (*c).x86_power & (1 << 8) != 0 {
        set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
        set_cpu_cap(c, X86_FEATURE_NONSTOP_TSC);
    }

    /* Bit 12 of 8000_0007 edx is accumulated power mechanism. */
    if (*c).x86_power & (1 << 12) != 0 {
        set_cpu_cap(c, X86_FEATURE_ACC_POWER);
    }

    /* Bit 14 indicates the Runtime Average Power Limit interface. */
    if (*c).x86_power & (1 << 14) != 0 {
        set_cpu_cap(c, X86_FEATURE_RAPL);
    }

    #[cfg(CONFIG_X86_64)]
    set_cpu_cap(c, X86_FEATURE_SYSCALL32);

    /*
     * ApicID can always be treated as an 8-bit value for Hygon APIC So, we
     * can safely set X86_FEATURE_EXTD_APICID unconditionally.
     */
    #[cfg(all(CONFIG_X86_LOCAL_APIC, CONFIG_PCI))]
    if boot_cpu_has(X86_FEATURE_APIC) {
        set_cpu_cap(c, X86_FEATURE_EXTD_APICID);
    }

    /*
     * This is only needed to tell the kernel whether to use VMCALL
     * and VMMCALL.  VMMCALL is never executed except under virt, so
     * we can set it unconditionally.
     */
    set_cpu_cap(c, X86_FEATURE_VMMCALL);
}

unsafe extern "C" fn init_hygon(c: *mut cpuinfo_x86) {
    let mut vm_cr: u64 = 0;

    early_init_hygon(c);

    set_cpu_cap(c, X86_FEATURE_REP_GOOD);

    /*
     * XXX someone from Hygon needs to confirm this DTRT
     *
    init_spectral_chicken(c);
     */

    set_cpu_cap(c, X86_FEATURE_ZEN);
    set_cpu_cap(c, X86_FEATURE_CPB);

    cpu_detect_cache_sizes(c);

    srat_detect_node(c);

    init_hygon_cacheinfo(c);

    if cpu_has(c, X86_FEATURE_SVM) {
        rdmsrq(MSR_VM_CR, &mut vm_cr);
        if vm_cr & SVM_VM_CR_SVM_DIS_MASK as u64 != 0 {
            pr_notice_once!("SVM disabled (by BIOS) in MSR_VM_CR\n");
            clear_cpu_cap(c, X86_FEATURE_SVM);
        }
    }

    if cpu_has(c, X86_FEATURE_XMM2) {
        /*
         * Use LFENCE for execution serialization.  On families which
         * don't have that MSR, LFENCE is already serializing.
         * msr_set_bit() uses the safe accessors, too, even if the MSR
         * is not present.
         */
        msr_set_bit(MSR_AMD64_DE_CFG, MSR_AMD64_DE_CFG_LFENCE_SERIALIZE_BIT as u8);

        /* A serializing LFENCE stops RDTSC speculation */
        set_cpu_cap(c, X86_FEATURE_LFENCE_RDTSC);
    }

    /*
     * Hygon processors have APIC timer running in deep C states.
     */
    set_cpu_cap(c, X86_FEATURE_ARAT);

    /* Hygon CPUs don't reset SS attributes on SYSRET, Xen does. */
    if !cpu_feature_enabled(X86_FEATURE_XENPV) {
        set_cpu_bug(c, X86_BUG_SYSRET_SS_ATTRS);
    }

    check_null_seg_clears_base(c);

    /* Hygon CPUs don't need fencing after x2APIC/TSC_DEADLINE MSR writes. */
    clear_cpu_cap(c, X86_FEATURE_APIC_MSRS_FENCE);
}

unsafe extern "C" fn cpu_detect_tlb_hygon(c: *mut cpuinfo_x86) {
    let (mut ebx, mut eax, mut ecx, mut edx): (u32, u32, u32, u32) = (0, 0, 0, 0);
    let mask: u16 = 0xfff;

    if (*c).extended_cpuid_level < 0x80000006 {
        return;
    }

    cpuid(0x80000006, &mut eax, &mut ebx, &mut ecx, &mut edx);

    tlb_lld_4k = ((ebx >> 16) as u16) & mask;
    tlb_lli_4k = (ebx as u16) & mask;

    /* Handle DTLB 2M and 4M sizes, fall back to L1 if L2 is disabled */
    if ((eax >> 16) as u16) & mask == 0 {
        tlb_lld_2m = ((cpuid_eax(0x80000005) >> 16) & 0xff) as u16;
    } else {
        tlb_lld_2m = ((eax >> 16) as u16) & mask;
    }

    /* a 4M entry uses two 2M entries */
    tlb_lld_4m = tlb_lld_2m >> 1;

    /* Handle ITLB 2M and 4M sizes, fall back to L1 if L2 is disabled */
    if (eax as u16) & mask == 0 {
        cpuid(0x80000005, &mut eax, &mut ebx, &mut ecx, &mut edx);
        tlb_lli_2m = (eax & 0xff) as u16;
    } else {
        tlb_lli_2m = (eax as u16) & mask;
    }

    tlb_lli_4m = tlb_lli_2m >> 1;
}

static hygon_cpu_dev: cpu_dev = cpu_dev {
    c_vendor: c"Hygon".as_ptr(),
    c_ident: [c"HygonGenuine".as_ptr(), core::ptr::null()],
    c_early_init: Some(early_init_hygon),
    c_bsp_init: Some(bsp_init_hygon),
    c_init: Some(init_hygon),
    c_identify: None,
    c_detect_tlb: Some(cpu_detect_tlb_hygon),
    c_x86_vendor: X86_VENDOR_HYGON as _,
    #[cfg(CONFIG_X86_32)]
    legacy_cache_size: None,
    #[cfg(CONFIG_X86_32)]
    // SAFETY: an all-zero model table is the C default (no named models).
    legacy_models: unsafe { core::mem::zeroed() },
};

cpu_dev_register!(hygon_cpu_dev);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
