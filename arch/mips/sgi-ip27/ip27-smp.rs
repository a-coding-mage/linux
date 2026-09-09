/*
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of this
 * archive for more details.
 *
 * Copyright (C) 2000 - 2001 by Kanoj Sarcar (kanoj@sgi.com)
 * Copyright (C) 2000 - 2001 by Silicon Graphics, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn node_scan_cpus(nasid: nasid_t, mut highest: ::core::ffi::c_int) -> ::core::ffi::c_int {
    static mut CPUS_FOUND: ::core::ffi::c_int = 0;
    let mut brd: *mut lboard_t;
    let mut acpu: *mut klcpu_t;
    let cpuid: cpuid_t;

    brd = find_lboard(KL_CONFIG_INFO(nasid), KLTYPE_IP27);

    loop {
        acpu = find_first_component(brd, KLSTRUCT_CPU);
        while !acpu.is_null() {
            cpuid = (*acpu).cpu_info.virtid;
            /* Only let it join in if it's marked enabled */
            if ((*acpu).cpu_info.flags & KLINFO_ENABLE) != 0
                && CPUS_FOUND != NR_CPUS
            {
                if cpuid > highest {
                    highest = cpuid;
                }
                set_cpu_possible(cpuid, true);
                *cputonasid(CPUS_FOUND) = nasid;
                *cputoslice(CPUS_FOUND) = (*acpu).cpu_info.physid;
                (*sn_cpu_info.add(CPUS_FOUND as usize)).p_speed = (*acpu).cpu_speed;
                CPUS_FOUND += 1;
            }
            acpu = find_component(brd, acpu as *mut klinfo_t, KLSTRUCT_CPU);
        }
        brd = KLCF_NEXT(brd);
        if brd.is_null() {
            break;
        }
        brd = find_lboard(brd, KLTYPE_IP27);
        if brd.is_null() {
            break;
        }
    }

    highest
}

pub unsafe fn cpu_node_probe() {
    let mut highest: ::core::ffi::c_int = 0;
    let gdap: *mut gda_t = GDA;

    nodes_clear(&mut node_online_map);
    nodes_clear(&mut node_possible_map);
    for i in 0..MAX_NUMNODES {
        let nasid: nasid_t = (*gdap).g_nasidtable[i as usize];
        if nasid == INVALID_NASID {
            break;
        }
        node_set_online(nasid);
        node_set(nasid, &mut node_possible_map);
        highest = node_scan_cpus(nasid, highest);
    }

    printk!("Discovered %d cpus on %d nodes\n", highest + 1, num_online_nodes());
}

unsafe fn intr_clear_all(nasid: nasid_t) {
    REMOTE_HUB_S(nasid, PI_INT_MASK0_A, 0);
    REMOTE_HUB_S(nasid, PI_INT_MASK0_B, 0);
    REMOTE_HUB_S(nasid, PI_INT_MASK1_A, 0);
    REMOTE_HUB_S(nasid, PI_INT_MASK1_B, 0);

    for i in 0..128 {
        REMOTE_HUB_CLR_INTR(nasid, i);
    }
}

unsafe fn ip27_send_ipi_single(destid: ::core::ffi::c_int, action: ::core::ffi::c_uint) {
    let mut irq: ::core::ffi::c_int;

    irq = match action {
        SMP_RESCHEDULE_YOURSELF => CPU_RESCHED_A_IRQ,
        SMP_CALL_FUNCTION => CPU_CALL_A_IRQ,
        _ => panic!("sendintr"),
    };

    irq += *cputoslice(destid);

    /*
     * Set the interrupt bit associated with the CPU we want to
     * send the interrupt to.
     */
    REMOTE_HUB_SEND_INTR(cpu_to_node(destid), irq);
}

unsafe fn ip27_send_ipi_mask(mask: *const cpumask, action: ::core::ffi::c_uint) {
    for_each_cpu!(i, mask, {
        ip27_send_ipi_single(i, action);
    });
}

unsafe fn ip27_init_cpu() {
    per_cpu_init();
}

unsafe fn ip27_smp_finish() {
    hub_rt_clock_event_init();
    local_irq_enable();
}

/*
 * Launch a slave into smp_bootstrap().  It doesn't take an argument, and we
 * set sp to the kernel stack of the newly created idle process, gp to the proc
 * struct so that current_thread_info() will work.
 */
unsafe fn ip27_boot_secondary(cpu: ::core::ffi::c_int, idle: *mut task_struct) -> ::core::ffi::c_int {
    let gp = task_thread_info(idle) as usize;
    let sp = __KSTK_TOS(idle);

    LAUNCH_SLAVE(
        *cputonasid(cpu),
        *cputoslice(cpu),
        MAPPED_KERN_RW_TO_K0(smp_bootstrap),
        0,
        sp as *mut ::core::ffi::c_void,
        gp as *mut ::core::ffi::c_void,
    );
    0
}

unsafe fn ip27_smp_setup() {
    for_each_online_node!(nasid, {
        if nasid == 0 {
            continue;
        }
        intr_clear_all(nasid);
    });

    replicate_kernel_text();

    /*
     * PROM sets up system, that boot cpu is always first CPU on nasid 0
     */
    *cputonasid(0) = 0;
    *cputoslice(0) = LOCAL_HUB_L(PI_CPU_NUM);
}

unsafe fn ip27_prepare_cpus(_max_cpus: ::core::ffi::c_uint) {
    /* We already did everything necessary earlier */
}

#[no_mangle]
pub static mut ip27_smp_ops: plat_smp_ops = plat_smp_ops {
    send_ipi_single: Some(ip27_send_ipi_single),
    send_ipi_mask: Some(ip27_send_ipi_mask),
    init_secondary: Some(ip27_init_cpu),
    smp_finish: Some(ip27_smp_finish),
    boot_secondary: Some(ip27_boot_secondary),
    smp_setup: Some(ip27_smp_setup),
    prepare_cpus: Some(ip27_prepare_cpus),
    prepare_boot_cpu: Some(ip27_init_cpu),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
