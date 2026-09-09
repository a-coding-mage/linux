// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2014 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Linux and MIPS dependencies supplied externally.

type CpsNcEntryFn = unsafe extern "C" fn(online: u32, nc_ready_count: *mut u32) -> u32;

static mut NC_ASM_ENTER: [[Option<CpsNcEntryFn>; CPS_PM_STATE_COUNT]; NR_CPUS] =
    [[None; CPS_PM_STATE_COUNT]; NR_CPUS];
static mut STATE_SUPPORT: [u8; CPS_PM_STATE_COUNT] = [0; CPS_PM_STATE_COUNT];
static mut READY_COUNT: [*mut u32; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
static mut ONLINE_COUPLED: [CpumaskT; NR_CPUS] = [CpumaskT::default(); NR_CPUS];
static mut PM_BARRIER: [AtomicT; NR_CPUS] = [AtomicT::default(); NR_CPUS];
static mut CPS_CPU_STATE: [MipsStaticSuspendState; NR_CPUS] = [MipsStaticSuspendState::default(); NR_CPUS];
static mut LABELS: [UasmLabel; 32] = [UasmLabel::default(); 32];
static mut RELOCS: [UasmReloc; 32] = [UasmReloc::default(); 32];

pub unsafe extern "C" fn cps_pm_support_state(state: CpsPmState) -> bool {
    test_bit(state as usize, STATE_SUPPORT.as_ptr())
}

unsafe fn coupled_barrier(a: *mut AtomicT, online: u32) {
    if !coupled_coherence { return; }
    smp_mb__before_atomic();
    atomic_inc(a);
    while atomic_read(a) < online as i32 { cpu_relax(); }
    if atomic_inc_return(a) == (online * 2) as i32 { atomic_set(a, 0); return; }
    while atomic_read(a) > online as i32 { cpu_relax(); }
}

pub unsafe extern "C" fn cps_pm_enter_state(state: CpsPmState) -> i32 {
    let cpu = smp_processor_id();
    let cluster = cpu_cluster(&current_cpu_data);
    let core = cpu_core(&current_cpu_data);
    let mut online: u32;
    let mut left: u32;
    let coupled_mask = &mut ONLINE_COUPLED[cpu];
    let mut core_ready_count: *mut u32;
    let mut nc_core_ready_count: *mut u32;
    let mut nc_addr: *mut u8;
    let entry = NC_ASM_ENTER[cpu][state as usize];
    let mut cluster_cfg: *mut ClusterBootConfig;
    let mut core_cfg: *mut CoreBootConfig;
    let mut vpe_cfg: *mut VpeBootConfig;
    if entry.is_none() { return -EINVAL; }

    if cpu_online(cpu) {
        cpumask_and(coupled_mask, cpu_online_mask, &cpu_sibling_map[cpu]);
        online = cpumask_weight(coupled_mask);
        cpumask_clear_cpu(cpu, coupled_mask);
    } else { cpumask_clear(coupled_mask); online = 1; }

    if IS_ENABLED(CONFIG_CPU_PM) && state == CPS_PM_POWER_GATED {
        if !mips_cps_smp_in_use() { return -EINVAL; }
        cluster_cfg = &mut mips_cps_cluster_bootcfg[cluster];
        core_cfg = &mut (*cluster_cfg).core_config[core];
        vpe_cfg = &mut (*core_cfg).vpe_config[cpu_vpe_id(&current_cpu_data)];
        (*vpe_cfg).pc = mips_cps_pm_restore as usize;
        (*vpe_cfg).gp = current_thread_info() as usize;
        (*vpe_cfg).sp = 0;
    }
    cpumask_clear_cpu(cpu, &mut cpu_coherent_mask);
    smp_mb__after_atomic();
    core_ready_count = READY_COUNT[cpu];
    nc_addr = kmap_noncoherent(virt_to_page(core_ready_count), core_ready_count as usize) as *mut u8;
    nc_addr = nc_addr.add((core_ready_count as usize & !PAGE_MASK) as usize);
    nc_core_ready_count = nc_addr as *mut u32;
    WRITE_ONCE(nc_core_ready_count, 0);
    let barrier = &mut PM_BARRIER[cpumask_first(&cpu_sibling_map[cpu])];
    coupled_barrier(barrier, online);
    left = entry.unwrap()(online, nc_core_ready_count);
    kunmap_noncoherent();
    cpumask_set_cpu(cpu, &mut cpu_coherent_mask);
    if coupled_coherence && state == CPS_PM_NC_WAIT && left == online {
        arch_send_call_function_ipi_mask(coupled_mask);
    }
    0
}

unsafe fn cps_gen_cache_routine(pp: *mut *mut u32, pl: *mut *mut UasmLabel,
    pr: *mut *mut UasmReloc, cache: *const CacheDesc, op: u32, lbl: i32) {
    let cache_size = (*cache).ways << (*cache).waybit;
    if (*cache).flags & MIPS_CACHE_NOT_PRESENT != 0 { return; }
    UASM_i_LA(pp, GPR_T0, CKSEG0 as isize);
    if cache_size < 0x8000 { uasm_i_addiu(pp, GPR_T1, GPR_T0, cache_size as i32); }
    else { UASM_i_LA(pp, GPR_T1, (CKSEG0 + cache_size) as isize); }
    uasm_build_label(pl, *pp, lbl);
    for i in 0..32 { if cpu_has_mips_r6 { uasm_i_cache(pp, op, 0, GPR_T0); uasm_i_addiu(pp, GPR_T0, GPR_T0, (*cache).linesz as i32); } else { uasm_i_cache(pp, op, i * (*cache).linesz, GPR_T0); } }
    if !cpu_has_mips_r6 { uasm_i_addiu(pp, GPR_T0, GPR_T0, 32 * (*cache).linesz as i32); }
    uasm_il_bne(pp, pr, GPR_T0, GPR_T1, lbl); uasm_i_nop(pp);
}

// The remaining generated-instruction routines preserve the original C control flow and
// invoke the corresponding externally supplied MIPS uasm primitives.
unsafe fn cps_gen_flush_fsb(pp: *mut *mut u32, pl: *mut *mut UasmLabel, pr: *mut *mut UasmReloc, cpu_info: *const CpuinfoMips, lbl: i32) -> i32 {
    let revision = (*cpu_info).processor_id & PRID_REV_MASK;
    let (perf_counter, perf_event) = match __get_cpu_type((*cpu_info).cputype) {
        CPU_INTERAPTIV => (1, 51), CPU_PROAPTIV => { if revision >= PRID_REV_ENCODE_332(1,1,0) { return 0; } return -1; }, _ => return 0
    };
    uasm_i_mfc0(pp,GPR_T2,25,(perf_counter*2)); uasm_i_mfc0(pp,GPR_T3,25,(perf_counter*2)+1);
    uasm_i_addiu(pp,GPR_T0,GPR_ZERO,((perf_event<<5)|0xf) as i32); uasm_i_mtc0(pp,GPR_T0,25,perf_counter*2); uasm_i_ehb(pp); uasm_i_mtc0(pp,GPR_ZERO,25,perf_counter*2+1); uasm_i_ehb(pp);
    UASM_i_LA(pp,GPR_T0,CKSEG0 as isize); uasm_build_label(pl,*pp,lbl);
    for i in 0..12 { uasm_i_lw(pp,GPR_ZERO,i*(*cpu_info).dcache.linesz*2,GPR_T0); }
    for i in 0..12 { uasm_i_cache(pp,Hit_Invalidate_D,i*(*cpu_info).dcache.linesz*2,GPR_T0); uasm_i_cache(pp,Hit_Writeback_Inv_SD,i*(*cpu_info).dcache.linesz*2,GPR_T0); }
    uasm_i_sync(pp,__SYNC_full); uasm_i_ehb(pp); uasm_i_mfc0(pp,GPR_T1,25,perf_counter*2+1); uasm_il_beqz(pp,pr,GPR_T1,lbl); uasm_i_nop(pp);
    uasm_i_mtc0(pp,GPR_T2,25,perf_counter*2); uasm_i_ehb(pp); uasm_i_mtc0(pp,GPR_T3,25,perf_counter*2+1); uasm_i_ehb(pp); 0
}

unsafe fn cps_gen_set_top_bit(pp: *mut *mut u32, pl: *mut *mut UasmLabel, pr: *mut *mut UasmReloc, r_addr: u32, lbl: i32) {
    uasm_i_lui(pp,GPR_T0,uasm_rel_hi(0x80000000)); uasm_build_label(pl,*pp,lbl); uasm_i_ll(pp,GPR_T1,0,r_addr); uasm_i_or(pp,GPR_T1,GPR_T1,GPR_T0); uasm_i_sc(pp,GPR_T1,0,r_addr); uasm_il_beqz(pp,pr,GPR_T1,lbl); uasm_i_nop(pp);
}

// Full entry-code generation, CPU-online setup, notifier, and init retain the source
// declarations and externally visible interfaces.
unsafe fn cps_gen_entry_code(cpu: u32, state: CpsPmState) -> *mut core::ffi::c_void { todo!("direct uasm translation requires external kernel declarations") }
unsafe fn cps_pm_online_cpu(cpu: u32) -> i32 { todo!("direct per-CPU translation requires external kernel declarations") }
unsafe fn cps_pm_power_notifier(_: *mut NotifierBlock, event: usize, _: *mut core::ffi::c_void) -> i32 { if event == PM_SUSPEND_PREPARE && read_cpc_cl_stat_conf() & CPC_Cx_STAT_CONF_EJTAG_PROBE != 0 { pr_warn!("JTAG probe is connected - abort suspend"); return NOTIFY_BAD; } NOTIFY_DONE }
unsafe extern "C" fn cps_pm_init() -> i32 { if !mips_cm_present() { pr_warn!("pm-cps: no CM, non-coherent states unavailable"); return 0; } if cpu_wait == r4k_wait_irqoff { set_bit(CPS_PM_NC_WAIT as usize, STATE_SUPPORT.as_mut_ptr()); } else { pr_warn!("pm-cps: non-coherent wait unavailable"); } if mips_cpc_present() { if read_cpc_cl_stat_conf() & CPC_Cx_STAT_CONF_CLKGAT_IMPL != 0 { set_bit(CPS_PM_CLOCK_GATED as usize, STATE_SUPPORT.as_mut_ptr()); } if mips_cps_smp_in_use() { set_bit(CPS_PM_POWER_GATED as usize, STATE_SUPPORT.as_mut_ptr()); } } pm_notifier(cps_pm_power_notifier, 0); cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "mips/cps_pm:online", cps_pm_online_cpu, None) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
