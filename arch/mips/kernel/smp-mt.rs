// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2004, 05, 06 MIPS Technologies, Inc.
 *    Elizabeth Clarke (beth@mips.com)
 *    Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2006 Ralf Baechle (ralf@linux-mips.org)
 */

unsafe fn smvp_copy_vpe_config() {
    write_vpe_c0_status((read_c0_status() & !(ST0_IM | ST0_IE | ST0_KSU)) | ST0_CU0);

    /* set config to be the same as vpe0, particularly kseg0 coherency alg */
    write_vpe_c0_config(read_c0_config());

    /* make sure there are no software interrupts pending */
    write_vpe_c0_cause(0);

    /* Propagate Config7 */
    write_vpe_c0_config7(read_c0_config7());

    write_vpe_c0_count(read_c0_count());
}

unsafe fn smvp_vpe_init(mut tc: c_uint, mvpconf0: c_uint, mut ncpu: c_uint) -> c_uint {
    if tc >= smp_max_threads
        || (tc > ((mvpconf0 & MVPCONF0_PVPE) >> MVPCONF0_PVPE_SHIFT))
    {
        return ncpu;
    }

    /* Deactivate all but VPE 0 */
    if tc != 0 {
        let mut tmp = read_vpe_c0_vpeconf0();

        tmp &= !VPECONF0_VPA;

        /* master VPE */
        tmp |= VPECONF0_MVP;
        write_vpe_c0_vpeconf0(tmp);

        /* Record this as available CPU */
        set_cpu_possible(tc, true);
        set_cpu_present(tc, true);
        ncpu += 1;
        __cpu_number_map[tc as usize] = ncpu;
        __cpu_logical_map[ncpu as usize] = tc;
    }

    /* Disable multi-threading with TC's */
    write_vpe_c0_vpecontrol(read_vpe_c0_vpecontrol() & !VPECONTROL_TE);

    if tc != 0 {
        smvp_copy_vpe_config();
    }

    cpu_set_vpe_id(&mut cpu_data[ncpu as usize], tc);

    ncpu
}

unsafe fn smvp_tc_init(tc: c_uint, mvpconf0: c_uint) {
    let mut tmp: c_ulong;

    if tc == 0 {
        return;
    }

    /* bind a TC to each VPE, May as well put all excess TC's
       on the last VPE */
    if tc >= (((mvpconf0 & MVPCONF0_PVPE) >> MVPCONF0_PVPE_SHIFT) + 1) {
        write_tc_c0_tcbind(
            read_tc_c0_tcbind()
                | ((mvpconf0 & MVPCONF0_PVPE) >> MVPCONF0_PVPE_SHIFT),
        );
    } else {
        write_tc_c0_tcbind(read_tc_c0_tcbind() | tc);

        /* and set XTC */
        write_vpe_c0_vpeconf0(
            read_vpe_c0_vpeconf0() | (tc << VPECONF0_XTC_SHIFT),
        );
    }

    tmp = read_tc_c0_tcstatus();

    /* mark not allocated and not dynamically allocatable */
    tmp &= !(TCSTATUS_A | TCSTATUS_DA);
    tmp |= TCSTATUS_IXMT; /* interrupt exempt */
    write_tc_c0_tcstatus(tmp);

    write_tc_c0_tchalt(TCHALT_H);
}

unsafe fn vsmp_init_secondary() {
    /* This is Malta specific: IPI,performance and timer interrupts */
    if mips_gic_present() {
        change_c0_status(ST0_IM, STATUSF_IP2 | STATUSF_IP3 |
            STATUSF_IP4 | STATUSF_IP5 | STATUSF_IP6 | STATUSF_IP7);
    } else {
        change_c0_status(ST0_IM, STATUSF_IP0 | STATUSF_IP1 |
            STATUSF_IP6 | STATUSF_IP7);
    }
}

unsafe fn vsmp_smp_finish() {
    /* CDFIXME: remove this? */
    write_c0_compare(read_c0_count() + (8 * mips_hpt_frequency / HZ));

    // #ifdef CONFIG_MIPS_MT_FPAFF
    /* If we have an FPU, enroll ourselves in the FPU-full mask */
    if cpu_has_fpu {
        cpumask_set_cpu(smp_processor_id(), &mut mt_fpu_cpumask);
    }
    // #endif /* CONFIG_MIPS_MT_FPAFF */

    local_irq_enable();
}

/*
 * Setup the PC, SP, and GP of a secondary processor and start it
 * running!
 * smp_bootstrap is the place to resume from
 * __KSTK_TOS(idle) is apparently the stack pointer
 * (unsigned long)idle->thread_info the gp
 * assumes a 1:1 mapping of TC => VPE
 */
unsafe fn vsmp_boot_secondary(cpu: c_int, idle: *mut task_struct) -> c_int {
    let gp = task_thread_info(idle);
    dvpe();
    set_c0_mvpcontrol(MVPCONTROL_VPC);

    settc(cpu);

    /* restart */
    write_tc_c0_tcrestart(&smp_bootstrap as *const _ as c_ulong);

    /* enable the tc this vpe/cpu will be running */
    write_tc_c0_tcstatus((read_tc_c0_tcstatus() & !TCSTATUS_IXMT) | TCSTATUS_A);

    write_tc_c0_tchalt(0);

    /* enable the VPE */
    write_vpe_c0_vpeconf0(read_vpe_c0_vpeconf0() | VPECONF0_VPA);

    /* stack pointer */
    write_tc_gpr_sp(__KSTK_TOS(idle));

    /* global pointer */
    write_tc_gpr_gp(gp as c_ulong);

    flush_icache_range(
        gp as c_ulong,
        gp.add(core::mem::size_of::<thread_info>()) as c_ulong,
    );

    /* finally out of configuration and into chaos */
    clear_c0_mvpcontrol(MVPCONTROL_VPC);

    evpe(EVPE_ENABLE);

    0
}

/*
 * Common setup before any secondaries are started
 * Make sure all CPU's are in a sensible state before we boot any of the
 * secondaries
 */
unsafe fn vsmp_smp_setup() {
    let mut mvpconf0: c_uint;
    let mut ntc: c_uint;
    let mut tc: c_uint;
    let mut ncpu: c_uint = 0;
    let mut nvpe: c_uint;

    // #ifdef CONFIG_MIPS_MT_FPAFF
    /* If we have an FPU, enroll ourselves in the FPU-full mask */
    if cpu_has_fpu {
        cpumask_set_cpu(0, &mut mt_fpu_cpumask);
    }
    // #endif /* CONFIG_MIPS_MT_FPAFF */
    if !cpu_has_mipsmt {
        return;
    }

    /* disable MT so we can configure */
    dvpe();
    dmt();

    /* Put MVPE's into 'configuration state' */
    set_c0_mvpcontrol(MVPCONTROL_VPC);

    mvpconf0 = read_c0_mvpconf0();
    ntc = (mvpconf0 & MVPCONF0_PTC) >> MVPCONF0_PTC_SHIFT;

    nvpe = ((mvpconf0 & MVPCONF0_PVPE) >> MVPCONF0_PVPE_SHIFT) + 1;
    smp_num_siblings = nvpe;

    /* we'll always have more TC's than VPE's, so loop setting everything
       to a sensible state */
    tc = 0;
    while tc <= ntc {
        settc(tc);

        smvp_tc_init(tc, mvpconf0);
        ncpu = smvp_vpe_init(tc, mvpconf0, ncpu);
        tc += 1;
    }

    /* Release config state */
    clear_c0_mvpcontrol(MVPCONTROL_VPC);

    /* We'll wait until starting the secondaries before starting MVPE */

    printk!(KERN_INFO, "Detected %i available secondary CPU(s)\n", ncpu);
}

unsafe fn vsmp_prepare_cpus(_max_cpus: c_uint) {
    mips_mt_set_cpuoptions();
}

#[repr(C)]
pub static vsmp_smp_ops: plat_smp_ops = plat_smp_ops {
    send_ipi_single: Some(mips_smp_send_ipi_single),
    send_ipi_mask: Some(mips_smp_send_ipi_mask),
    init_secondary: Some(vsmp_init_secondary),
    smp_finish: Some(vsmp_smp_finish),
    boot_secondary: Some(vsmp_boot_secondary),
    smp_setup: Some(vsmp_smp_setup),
    prepare_cpus: Some(vsmp_prepare_cpus),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
