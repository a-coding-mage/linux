// SPDX-License-Identifier: GPL-2.0-or-later
/* Common boot and setup code. Direct translation of setup_64.c. */

// C headers and kernel-provided symbols are supplied by the surrounding tree.

pub static mut spinning_secondaries: i32 = 0;
pub static mut ppc64_pft_size: u64 = 0;

#[repr(C)]
pub struct ppc_cache_info {
    pub size: u32, pub sets: u32, pub line_size: u32, pub block_size: u32,
    pub log_block_size: u32, pub blocks_per_page: u32, pub assoc: u32,
}
#[repr(C)]
pub struct ppc64_caches_struct { pub l1d: ppc_cache_info, pub l1i: ppc_cache_info, pub l2: ppc_cache_info, pub l3: ppc_cache_info }

pub static mut ppc64_caches: ppc64_caches_struct = ppc64_caches_struct {
    l1d: ppc_cache_info { size: 0, sets: 0, line_size: 0, block_size: 0x40, log_block_size: 6, blocks_per_page: 0, assoc: 0 },
    l1i: ppc_cache_info { size: 0, sets: 0, line_size: 0, block_size: 0x40, log_block_size: 6, blocks_per_page: 0, assoc: 0 },
    l2: ppc_cache_info { size: 0, sets: 0, line_size: 0, block_size: 0, log_block_size: 0, blocks_per_page: 0, assoc: 0 },
    l3: ppc_cache_info { size: 0, sets: 0, line_size: 0, block_size: 0, log_block_size: 0, blocks_per_page: 0, assoc: 0 },
};

#[cfg(all(CONFIG_PPC_BOOK3E_64, CONFIG_SMP))]
pub unsafe fn setup_tlb_core_data() {
    BUILD_BUG_ON!(core::mem::offset_of!(tlb_core_data, lock) != 0);
    for_each_possible_cpu!(cpu) {
        let mut first = cpu_first_thread_sibling(cpu);
        if cpu_first_thread_sibling(boot_cpuid) == first { first = boot_cpuid; }
        (*paca_ptrs[cpu]).tcd_ptr = &mut (*paca_ptrs[first]).tcd;
        WARN_ONCE!(smt_enabled_at_boot >= 2 && book3e_htw_mode != PPC_HTW_E6500,
                   "%s: unsupported MMU configuration\n", __func__);
    }
}

#[cfg(CONFIG_SMP)]
static mut smt_enabled_cmdline: *mut i8 = core::ptr::null_mut();

#[cfg(CONFIG_SMP)]
pub unsafe fn check_smt_enabled() {
    let mut dn: *mut device_node;
    let smt_option: *const i8;
    smt_enabled_at_boot = threads_per_core;
    if !smt_enabled_cmdline.is_null() {
        if strcmp(smt_enabled_cmdline, c"on".as_ptr()) == 0 { smt_enabled_at_boot = threads_per_core; }
        else if strcmp(smt_enabled_cmdline, c"off".as_ptr()) == 0 { smt_enabled_at_boot = 0; }
        else { let mut smt = 0; if kstrtoint(smt_enabled_cmdline, 10, &mut smt) == 0 { smt_enabled_at_boot = core::cmp::min(threads_per_core, smt); } }
    } else {
        dn = of_find_node_by_path(c"/options".as_ptr());
        if !dn.is_null() {
            smt_option = of_get_property(dn, c"ibm,smt-enabled".as_ptr(), core::ptr::null_mut());
            if !smt_option.is_null() {
                if strcmp(smt_option, c"on".as_ptr()) == 0 { smt_enabled_at_boot = threads_per_core; }
                else if strcmp(smt_option, c"off".as_ptr()) == 0 { smt_enabled_at_boot = 0; }
            }
            of_node_put(dn);
        }
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn early_smt_enabled(p: *mut i8) -> i32 { smt_enabled_cmdline = p; 0 }
// early_param("smt-enabled", early_smt_enabled);

unsafe fn fixup_boot_paca(boot_paca: *mut paca_struct) {
    (*boot_paca).cpu_start = 1;
    #[cfg(CONFIG_PPC_BOOK3S_64)] { (*boot_paca).mc_emergency_sp = (&init_thread_union as *const _ as *mut u8).add(THREAD_SIZE / 2) as *mut _; }
    (*boot_paca).data_offset = 0;
    (*boot_paca).irq_soft_mask = IRQS_DISABLED;
    (*boot_paca).irq_happened = PACA_IRQ_HARD_DIS;
    WARN_ON!(mfmsr() & MSR_EE != 0);
}

unsafe fn configure_exceptions() {
    setup_kdump_trampoline();
    if firmware_has_feature(FW_FEATURE_SET_MODE) {
        if IS_ENABLED!(CONFIG_KVM_BOOK3S_PR_POSSIBLE) && !radix_enabled() { init_task.thread.fscr &= !FSCR_SCV; (*cur_cpu_spec).cpu_user_features2 &= !PPC_FEATURE2_SCV; }
        if !pseries_enable_reloc_on_exc() { init_task.thread.fscr &= !FSCR_SCV; (*cur_cpu_spec).cpu_user_features2 &= !PPC_FEATURE2_SCV; }
        #[cfg(target_endian = "little")] { pseries_little_endian_exceptions(); }
    } else {
        if firmware_has_feature(FW_FEATURE_OPAL) { opal_configure_cores(); }
    }
}

unsafe fn cpu_ready_for_interrupts() {
    if cpu_has_feature(CPU_FTR_HVMODE) {
        let lpcr = mfspr(SPRN_LPCR); let mut new_lpcr = lpcr;
        if cpu_has_feature(CPU_FTR_ARCH_31) { if pvr_version_is(PVR_POWER10) && (mfspr(SPRN_PVR) & 0xf00) == 0x100 { new_lpcr |= LPCR_AIL_3; } else { new_lpcr |= LPCR_HAIL; } }
        else if cpu_has_feature(CPU_FTR_ARCH_207S) { new_lpcr |= LPCR_AIL_3; }
        if new_lpcr != lpcr { mtspr(SPRN_LPCR, new_lpcr); }
    }
    if cpu_has_feature(CPU_FTR_HVMODE) { if cpu_has_feature(CPU_FTR_TM_COMP) { mtspr(SPRN_HFSCR, mfspr(SPRN_HFSCR) | HFSCR_TM); } else { mtspr(SPRN_HFSCR, mfspr(SPRN_HFSCR) & !HFSCR_TM); } }
    (*get_paca()).kernel_msr = MSR_KERNEL;
}

pub static mut spr_default_dscr: u64 = 0;
unsafe fn record_spr_defaults() { if early_cpu_has_feature(CPU_FTR_DSCR) { spr_default_dscr = mfspr(SPRN_DSCR); } }

pub unsafe fn early_setup(dt_ptr: u64) {
    static mut boot_paca: paca_struct = paca_struct::default();
    initialise_paca(&mut boot_paca, 0); fixup_boot_paca(&mut boot_paca); WARN_ON!(!local_paca.is_null()); setup_paca(&mut boot_paca);
    if IS_ENABLED!(CONFIG_PPC_BOOK3S_64) && (mfmsr() & MSR_HV != 0) { enable_machine_check(); }
    if !dt_cpu_ftrs_init(__va(dt_ptr)) { identify_cpu(0, mfspr(SPRN_PVR)); }
    udbg_early_init(); udbg_printf(c" -> %s(), dt_ptr: 0x%lx\n".as_ptr(), __func__, dt_ptr);
    early_init_devtree(__va(dt_ptr)); allocate_paca_ptrs(); allocate_paca(boot_cpuid); set_hard_smp_processor_id(boot_cpuid, boot_cpu_hwid);
    fixup_boot_paca(paca_ptrs[boot_cpuid]); setup_paca(paca_ptrs[boot_cpuid]);
    #[cfg(CONFIG_SMP)] { (*task_thread_info(current)).cpu = boot_cpuid; }
    configure_exceptions(); setup_kup(); apply_feature_fixups(); setup_feature_keys(); early_init_mmu(); early_ioremap_setup(); record_spr_defaults(); cpu_ready_for_interrupts(); this_cpu_enable_ftrace();
    udbg_printf(c" <- %s()\n".as_ptr(), __func__);
    #[cfg(CONFIG_PPC_EARLY_DEBUG_BOOTX)] { btext_map(); }
}

#[cfg(CONFIG_SMP)]
pub unsafe fn early_setup_secondary() { irq_soft_mask_set(IRQS_DISABLED); early_init_mmu_secondary(); setup_kup(); cpu_ready_for_interrupts(); }

pub unsafe fn panic_smp_self_stop() -> ! { hard_irq_disable(); spin_begin(); loop { spin_cpu_relax(); } }

#[cfg(any(CONFIG_SMP, CONFIG_KEXEC_CORE))]
unsafe fn use_spinloop() -> bool { if IS_ENABLED!(CONFIG_PPC_BOOK3S) { if firmware_has_feature(FW_FEATURE_OPAL) { return false; } return true; } of_property_read_bool(of_chosen, c"linux,booted-from-kexec".as_ptr()) }

#[cfg(any(CONFIG_SMP, CONFIG_KEXEC_CORE))]
pub unsafe fn smp_release_cpus() {
    if !use_spinloop() { return; }
    let ptr = ((&__secondary_hold_spinloop as *const _ as usize) - PHYSICAL_START as usize) as *mut u64;
    *ptr = ppc_function_entry(generic_secondary_smp_init) as u64;
    for _i in 0..100000 { mb(); HMT_low(); if spinning_secondaries == 0 { break; } udelay(1); }
    pr_debug!(c"spinning_secondaries = %d\n".as_ptr(), spinning_secondaries);
}

unsafe fn init_cache_info(info: *mut ppc_cache_info, size: u32, lsize: u32, bsize: u32, sets: u32) {
    (*info).size = size; (*info).sets = sets; (*info).line_size = lsize; (*info).block_size = bsize; (*info).log_block_size = __ilog2(bsize);
    (*info).blocks_per_page = if bsize != 0 { PAGE_SIZE / bsize } else { 0 }; (*info).assoc = if sets == 0 { 0xffff } else { size / (sets * lsize) };
}

unsafe fn parse_cache_info(np: *mut device_node, icache: bool, info: *mut ppc_cache_info) -> bool {
    let ipropnames = [c"i-cache-size".as_ptr(), c"i-cache-sets".as_ptr(), c"i-cache-block-size".as_ptr(), c"i-cache-line-size".as_ptr()];
    let dpropnames = [c"d-cache-size".as_ptr(), c"d-cache-sets".as_ptr(), c"d-cache-block-size".as_ptr(), c"d-cache-line-size".as_ptr()];
    let names = if icache { &ipropnames } else { &dpropnames }; let mut size = 0; let mut sets = u32::MAX; let mut lsize = (*cur_cpu_spec).dcache_bsize; let mut bsize = lsize;
    let sizep = of_get_property(np, names[0], core::ptr::null_mut()); if !sizep.is_null() { size = be32_to_cpu(*sizep); }
    let setsp = of_get_property(np, names[1], core::ptr::null_mut()); if !setsp.is_null() { sets = be32_to_cpu(*setsp); }
    let mut bsizep = of_get_property(np, names[2], core::ptr::null_mut()); let mut lsizep = of_get_property(np, names[3], core::ptr::null_mut());
    if bsizep.is_null() { bsizep = lsizep; } if lsizep.is_null() { lsizep = bsizep; }
    if !lsizep.is_null() { lsize = be32_to_cpu(*lsizep); } if !bsizep.is_null() { bsize = be32_to_cpu(*bsizep); }
    let success = !sizep.is_null() && !bsizep.is_null() && !lsizep.is_null(); if sets == 1 { sets = 0; } else if sets == 0 { sets = 1; }
    init_cache_info(info, size, lsize, bsize, sets); success
}

pub unsafe fn initialize_cache_info() {
    let mut cpu: *mut device_node = core::ptr::null_mut(); let mut l2: *mut device_node; let mut l3: *mut device_node = core::ptr::null_mut(); let pvr = PVR_VER(mfspr(SPRN_PVR));
    if pvr == PVR_POWER8 || pvr == PVR_POWER8E || pvr == PVR_POWER8NVL { init_cache_info(&mut ppc64_caches.l1i, 0x8000, 128, 128, 32); init_cache_info(&mut ppc64_caches.l1d, 0x10000, 128, 128, 64); init_cache_info(&mut ppc64_caches.l2, 0x80000, 128, 0, 512); init_cache_info(&mut ppc64_caches.l3, 0x800000, 128, 0, 8192); } else { cpu = of_find_node_by_type(core::ptr::null_mut(), c"cpu".as_ptr()); }
    if !cpu.is_null() { if !parse_cache_info(cpu, false, &mut ppc64_caches.l1d) { pr_warn!(c"Argh, can't find dcache properties !\n".as_ptr()); } if !parse_cache_info(cpu, true, &mut ppc64_caches.l1i) { pr_warn!(c"Argh, can't find icache properties !\n".as_ptr()); } l2 = of_find_next_cache_node(cpu); of_node_put(cpu); if !l2.is_null() { parse_cache_info(l2, false, &mut ppc64_caches.l2); l3 = of_find_next_cache_node(l2); of_node_put(l2); } if !l3.is_null() { parse_cache_info(l3, false, &mut ppc64_caches.l3); of_node_put(l3); } }
    dcache_bsize = ppc64_caches.l1d.block_size; icache_bsize = ppc64_caches.l1i.block_size; (*cur_cpu_spec).dcache_bsize = dcache_bsize; (*cur_cpu_spec).icache_bsize = icache_bsize;
}

pub unsafe fn ppc64_bolted_size() -> u64 {
    #[cfg(CONFIG_PPC_BOOK3E_64)] { return linear_map_top; }
    #[cfg(not(CONFIG_PPC_BOOK3E_64))] { if early_radix_enabled() { return u64::MAX; } if early_mmu_has_feature(MMU_FTR_1T_SEGMENT) { return 1u64 << SID_SHIFT_1T; } return 1u64 << SID_SHIFT; }
}

unsafe fn alloc_stack(limit: u64, cpu: i32) -> *mut u8 { BUILD_BUG_ON!(STACK_INT_FRAME_SIZE % 16 != 0); let ptr = memblock_alloc_try_nid(THREAD_SIZE, THREAD_ALIGN, MEMBLOCK_LOW_LIMIT, limit, early_cpu_to_node(cpu)); if ptr.is_null() { panic!("cannot allocate stacks"); } ptr as *mut u8 }

pub unsafe fn irqstack_early_init() { let limit = ppc64_bolted_size(); for_each_possible_cpu!(i) { softirq_ctx[i] = alloc_stack(limit, i); hardirq_ctx[i] = alloc_stack(limit, i); } }

#[cfg(CONFIG_PPC_BOOK3E_64)]
pub unsafe fn exc_lvl_early_init() { for_each_possible_cpu!(i) { let sp = alloc_stack(u64::MAX, i); critirq_ctx[i] = sp; (*paca_ptrs[i]).crit_kstack = sp.add(THREAD_SIZE); let sp = alloc_stack(u64::MAX, i); dbgirq_ctx[i] = sp; (*paca_ptrs[i]).dbg_kstack = sp.add(THREAD_SIZE); let sp = alloc_stack(u64::MAX, i); mcheckirq_ctx[i] = sp; (*paca_ptrs[i]).mc_kstack = sp.add(THREAD_SIZE); } if cpu_has_feature(CPU_FTR_DEBUG_LVL_EXC) { patch_exception(0x040, exc_debug_debug_book3e); } }

pub unsafe fn emergency_stack_init() { let mut limit = core::cmp::min(ppc64_bolted_size(), ppc64_rma_size); let mut mce_limit = limit; if firmware_has_feature(FW_FEATURE_LPAR) && mce_limit > SZ_4G { mce_limit = SZ_4G; } for_each_possible_cpu!(i) { (*paca_ptrs[i]).emergency_sp = alloc_stack(limit, i).add(THREAD_SIZE) as *mut _; #[cfg(CONFIG_PPC_BOOK3S_64)] { (*paca_ptrs[i]).nmi_emergency_sp = alloc_stack(limit, i).add(THREAD_SIZE) as *mut _; (*paca_ptrs[i]).mc_emergency_sp = alloc_stack(mce_limit, i).add(THREAD_SIZE) as *mut _; } } }

#[cfg(CONFIG_SMP)]
unsafe fn pcpu_cpu_distance(from: u32, to: u32) -> i32 { if early_cpu_to_node(from as i32) == early_cpu_to_node(to as i32) { LOCAL_DISTANCE } else { REMOTE_DISTANCE } }
#[cfg(CONFIG_SMP)] unsafe fn pcpu_cpu_to_node(cpu: i32) -> i32 { early_cpu_to_node(cpu) }
#[cfg(CONFIG_SMP)] pub static mut __per_cpu_offset: [u64; NR_CPUS] = [0; NR_CPUS];
#[cfg(CONFIG_SMP)] pub unsafe fn setup_per_cpu_areas() { let dyn_size = PERCPU_MODULE_RESERVE + PERCPU_DYNAMIC_RESERVE; let atom_size; let mut rc = -EINVAL; if IS_ENABLED!(CONFIG_PPC_BOOK3E_64) || radix_enabled() { atom_size = if IS_ENABLED!(CONFIG_PPC_BOOK3E_64) { SZ_1M } else { PAGE_SIZE }; } else { atom_size = if mmu_linear_psize == MMU_PAGE_4K { PAGE_SIZE } else { SZ_1M }; } if pcpu_chosen_fc != PCPU_FC_PAGE { rc = pcpu_embed_first_chunk(0, dyn_size, atom_size, pcpu_cpu_distance, pcpu_cpu_to_node); if rc != 0 { pr_warn!(c"PERCPU: allocator failed (%d), falling back to page size\n".as_ptr(), rc); } } if rc < 0 { rc = pcpu_page_first_chunk(0, pcpu_cpu_to_node); } if rc < 0 { panic!("cannot initialize percpu area (err={})", rc); } static_key_enable(&mut __percpu_first_chunk_is_paged.key); let delta = pcpu_base_addr as u64 - __per_cpu_start as u64; for_each_possible_cpu!(cpu) { __per_cpu_offset[cpu] = delta + pcpu_unit_offsets[cpu]; (*paca_ptrs[cpu]).data_offset = __per_cpu_offset[cpu]; } }

#[cfg(CONFIG_MEMORY_HOTPLUG)] pub unsafe fn memory_block_size_bytes() -> u64 { if let Some(f) = ppc_md.memory_block_size { return f(); } MIN_MEMORY_BLOCK_SIZE }
#[cfg(CONFIG_PPC_INDIRECT_PIO)] #[repr(C)] pub static mut ppc_pci_io: ppc_pci_io = ppc_pci_io::default();
#[cfg(CONFIG_HARDLOCKUP_DETECTOR_PERF)] pub unsafe fn hw_nmi_get_sample_period(watchdog_thresh: i32) -> u64 { ppc_proc_freq * watchdog_thresh as u64 }

unsafe fn disable_hardlockup_detector() -> i32 { #[cfg(CONFIG_HARDLOCKUP_DETECTOR_PERF)] { hardlockup_detector_disable(); } #[cfg(not(CONFIG_HARDLOCKUP_DETECTOR_PERF))] { if firmware_has_feature(FW_FEATURE_LPAR) { check_kvm_guest(); if is_kvm_guest() { hardlockup_detector_disable(); } } } 0 }
// early_initcall(disable_hardlockup_detector);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
