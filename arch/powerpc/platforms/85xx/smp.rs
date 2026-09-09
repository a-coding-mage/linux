// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Andy Fleming <afleming@freescale.com>
 *    Kumar Gala <galak@kernel.crashing.org>
 * Copyright 2006-2008, 2011-2012, 2015 Freescale Semiconductor Inc.
 */

#[repr(C)]
pub struct epapr_spin_table {
    pub addr_h: u32,
    pub addr_l: u32,
    pub r3_h: u32,
    pub r3_l: u32,
    pub reserved: u32,
    pub pir: u32,
}

static mut timebase: u64 = 0;
static mut tb_req: i32 = 0;
static mut tb_valid: i32 = 0;

unsafe fn mpc85xx_give_timebase() {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    hard_irq_disable();
    while tb_req == 0 { core::hint::spin_loop(); }
    tb_req = 0;
    qoriq_pm_ops.freeze_time_base(true);
    // CONFIG_PPC64 uses the manual mfspr/read-until-stable sequence.
    #[cfg(target_pointer_width = "64")]
    {
        let mut prev: u64;
        core::arch::asm!("mfspr {0}, {1}", out(reg) timebase, const SPRN_TBRL);
        loop {
            prev = timebase;
            core::arch::asm!("mfspr {0}, {1}", out(reg) timebase, const SPRN_TBRL);
            if prev == timebase { break; }
        }
    }
    #[cfg(not(target_pointer_width = "64"))]
    { timebase = get_tb(); }
    mb();
    tb_valid = 1;
    while tb_valid != 0 { core::hint::spin_loop(); }
    qoriq_pm_ops.freeze_time_base(false);
    local_irq_restore(flags);
}

unsafe fn mpc85xx_take_timebase() {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    hard_irq_disable();
    tb_req = 1;
    while tb_valid == 0 { core::hint::spin_loop(); }
    set_tb(timebase >> 32, timebase & 0xffff_ffff);
    isync();
    tb_valid = 0;
    local_irq_restore(flags);
}

#[cfg(feature = "hotplug_cpu")]
unsafe fn smp_85xx_cpu_offline_self() {
    let cpu = smp_processor_id();
    local_irq_disable(); hard_irq_disable();
    qoriq_pm_ops.irq_mask(cpu);
    idle_task_exit();
    mtspr(SPRN_TCR, 0); mtspr(SPRN_TSR, mfspr(SPRN_TSR));
    generic_set_cpu_dead(cpu);
    cur_cpu_spec.cpu_down_flush();
    qoriq_pm_ops.cpu_die(cpu);
    loop {}
}

#[cfg(feature = "hotplug_cpu")]
unsafe fn qoriq_cpu_kill(cpu: u32) {
    for _ in 0..500 {
        if is_cpu_dead(cpu) {
            #[cfg(target_pointer_width = "64")]
            { paca_ptrs[cpu as usize].cpu_start = 0; }
            return;
        }
        msleep(20);
    }
    pr_err!("CPU{} didn't die...\n", cpu);
}

unsafe fn flush_spin_table(spin_table: *mut core::ffi::c_void) {
    flush_dcache_range(spin_table as usize,
        spin_table as usize + core::mem::size_of::<epapr_spin_table>());
}

unsafe fn read_spin_table_addr_l(spin_table: *mut core::ffi::c_void) -> u32 {
    flush_spin_table(spin_table);
    in_be32(&(*(spin_table as *mut epapr_spin_table)).addr_l)
}

#[cfg(target_pointer_width = "64")]
unsafe fn wake_hw_thread(info: *mut core::ffi::c_void) {
    extern "C" { fn fsl_secondary_thread_init(); }
    let cpu = *(info as *const i32);
    let inia = ppc_function_entry(fsl_secondary_thread_init);
    book3e_start_thread(cpu_thread_in_core(cpu), inia);
}

unsafe fn smp_85xx_start_cpu(cpu: i32) -> i32 {
    let mut ret = 0;
    let np = of_get_cpu_node(cpu, core::ptr::null_mut());
    let cpu_rel_addr = of_get_property(np, c"cpu-release-addr".as_ptr(), core::ptr::null_mut());
    if cpu_rel_addr.is_null() { pr_err!("No cpu-release-addr for cpu {}\n", cpu); return -ENOENT; }
    let ioremappable = *cpu_rel_addr > virt_to_phys(high_memory - 1);
    let spin_table: *mut epapr_spin_table = if ioremappable {
        ioremap_coherent(*cpu_rel_addr, core::mem::size_of::<epapr_spin_table>()) as *mut _
    } else { phys_to_virt(*cpu_rel_addr) as *mut _ };
    let mut flags = 0usize;
    local_irq_save(&mut flags); hard_irq_disable();
    if !qoriq_pm_ops.is_null() && !(*qoriq_pm_ops).cpu_up_prepare.is_none() { (*qoriq_pm_ops).cpu_up_prepare(cpu); }
    if read_spin_table_addr_l(spin_table as *mut _) != 1 {
        mpic_reset_core(cpu);
        if !spin_event_timeout(read_spin_table_addr_l(spin_table as *mut _) == 1, 10000, 100) {
            pr_err!("timeout waiting for cpu {} to reset\n", get_hard_smp_processor_id(cpu));
            ret = -EAGAIN; local_irq_restore(flags);
            if ioremappable { iounmap(spin_table as *mut _); }
            return ret;
        }
    }
    flush_spin_table(spin_table as *mut _);
    out_be32(&mut (*spin_table).pir, get_hard_smp_processor_id(cpu) as u32);
    // PPC64 writes the function entry as a 64-bit address; PPC32 writes __early_start.
    #[cfg(target_pointer_width = "64")]
    out_be64((&mut (*spin_table).addr_h) as *mut u32 as *mut u64,
        __pa(ppc_function_entry(generic_secondary_smp_init)));
    #[cfg(not(target_pointer_width = "64"))]
    out_be32(&mut (*spin_table).addr_l, __pa(__early_start));
    flush_spin_table(spin_table as *mut _);
    local_irq_restore(flags);
    if ioremappable { iounmap(spin_table as *mut _); }
    ret
}

unsafe fn smp_85xx_kick_cpu(nr: i32) -> i32 {
    WARN_ON(nr < 0 || nr >= num_possible_cpus());
    pr_debug!("kick CPU #{}\n", nr);
    #[cfg(target_pointer_width = "64")]
    {
        let mut primary = nr;
        if threads_per_core == 2 {
            if WARN_ON_ONCE(!cpu_has_feature(CPU_FTR_SMT)) { return -ENOENT; }
            booting_thread_hwid = cpu_thread_in_core(nr); primary = cpu_first_thread_sibling(nr);
            if !qoriq_pm_ops.is_null() { (*qoriq_pm_ops).cpu_up_prepare(nr); }
            if cpu_online(primary) { smp_call_function_single(primary, wake_hw_thread, &nr as *const _ as *mut _, 1); }
            else if cpu_online(primary + 1) { smp_call_function_single(primary + 1, wake_hw_thread, &nr as *const _ as *mut _, 1); }
            else { let r = smp_85xx_start_cpu(primary); if r != 0 { return r; } }
        } else if threads_per_core == 1 { booting_thread_hwid = INVALID_THREAD_HWID; }
        else if threads_per_core > 2 { pr_err!("Do not support more than 2 threads per CPU."); return -EINVAL; }
        paca_ptrs[nr as usize].cpu_start = 1; generic_set_cpu_up(nr); return 0;
    }
    #[cfg(not(target_pointer_width = "64"))]
    { let ret = smp_85xx_start_cpu(nr); if ret != 0 { return ret; } generic_set_cpu_up(nr); ret }
}

pub static mut smp_85xx_ops: smp_ops_t = smp_ops_t {
    cause_nmi_ipi: None, kick_cpu: Some(smp_85xx_kick_cpu), cpu_bootable: Some(smp_generic_cpu_bootable),
    #[cfg(feature = "hotplug_cpu")] cpu_disable: Some(generic_cpu_disable),
    #[cfg(feature = "hotplug_cpu")] cpu_die: Some(generic_cpu_die),
};

unsafe fn smp_85xx_setup_cpu(_cpu_nr: i32) { mpic_setup_this_cpu(); }

#[no_mangle]
pub unsafe extern "C" fn mpc85xx_smp_init() {
    let np = of_find_node_by_type(core::ptr::null_mut(), c"open-pic".as_ptr());
    if !np.is_null() {
        smp_85xx_ops.probe = Some(smp_mpic_probe);
        smp_85xx_ops.setup_cpu = Some(smp_85xx_setup_cpu);
        smp_85xx_ops.message_pass = Some(smp_mpic_message_pass);
    } else { smp_85xx_ops.setup_cpu = None; }
    if cpu_has_feature(CPU_FTR_DBELL) {
        smp_85xx_ops.message_pass = None; smp_85xx_ops.cause_ipi = Some(doorbell_global_ipi); smp_85xx_ops.probe = None;
    }
    // CONFIG_FSL_CORENET_RCPM selects fsl_rcpm_init; otherwise mpc85xx_setup_pmc.
    #[cfg(feature = "fsl_corenet_rcpm")] fsl_rcpm_init();
    #[cfg(not(feature = "fsl_corenet_rcpm"))] mpc85xx_setup_pmc();
    if !qoriq_pm_ops.is_null() {
        smp_85xx_ops.give_timebase = Some(mpc85xx_give_timebase);
        smp_85xx_ops.take_timebase = Some(mpc85xx_take_timebase);
        #[cfg(feature = "hotplug_cpu")] { smp_85xx_ops.cpu_offline_self = Some(smp_85xx_cpu_offline_self); smp_85xx_ops.cpu_die = Some(qoriq_cpu_kill); }
    }
    smp_ops = &mut smp_85xx_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
