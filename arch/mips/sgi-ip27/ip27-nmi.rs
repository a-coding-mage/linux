// SPDX-License-Identifier: GPL-2.0
// Translated from ip27-nmi.c. Kernel and SGI architecture dependencies are
// supplied by the surrounding translation environment.

type MachregT = ::core::ffi::c_ulong;

// #if 0: NODE_NUM_CPUS(n) is CNODE_NUM_CPUS(n); otherwise it is CPUS_PER_NODE.
const NODE_NUM_CPUS: usize = CPUS_PER_NODE;

static mut nmi_lock: arch_spinlock_t = __ARCH_SPIN_LOCK_UNLOCKED;

#[inline]
unsafe fn send_nmi(nasid: nasid_t, slice: i32) {
    REMOTE_HUB_S(nasid, PI_NMI_A + (slice * PI_NMI_OFFSET), 1);
}

pub unsafe fn install_cpu_nmi_handler(slice: i32) {
    let nmi_addr = NMI_ADDR(get_nasid(), slice) as *mut nmi_t;
    if (*nmi_addr).call_addr != 0 as *mut _ {
        return;
    }
    (*nmi_addr).magic = NMI_MAGIC;
    (*nmi_addr).call_addr = nmi_dump as *mut _;
    (*nmi_addr).call_addr_c = (!( (*nmi_addr).call_addr as usize)) as *mut _;
    (*nmi_addr).call_parm = 0;
}

/*
 * Copy the cpu registers which have been saved in the IP27prom format
 * into the eframe format for the node under consideration.
 */
unsafe fn nmi_cpu_eframe_save(nasid: nasid_t, slice: i32) {
    let nr = (TO_UNCAC(TO_NODE(nasid, IP27_NMI_KREGS_OFFSET))
        + slice as usize * IP27_NMI_KREGS_CPU_SIZE) as *mut reg_struct;

    pr_emerg!("NMI nasid %d: slice %d\n", nasid, slice);

    /* Saved main processor registers */
    let mut i = 0;
    while i < 32 {
        if i % 4 == 0 {
            pr_emerg!("$%2d   :", i);
        }
        pr_cont!(" %016lx", (*nr).gpr[i]);
        i += 1;
        if i % 4 == 0 {
            pr_cont!("\n");
        }
    }

    pr_emerg!("Hi    : (value lost)\n");
    pr_emerg!("Lo    : (value lost)\n");

    /* Saved cp0 registers */
    pr_emerg!("epc   : %016lx %pS\n", (*nr).epc, (*nr).epc as *mut _);
    pr_emerg!("%s\n", print_tainted());
    pr_emerg!("ErrEPC: %016lx %pS\n", (*nr).error_epc, (*nr).error_epc as *mut _);
    pr_emerg!("ra    : %016lx %pS\n", (*nr).gpr[31], (*nr).gpr[31] as *mut _);
    pr_emerg!("Status: %08lx      ", (*nr).sr);

    if (*nr).sr & ST0_KX != 0 { pr_cont!("KX "); }
    if (*nr).sr & ST0_SX != 0 { pr_cont!("SX "); }
    if (*nr).sr & ST0_UX != 0 { pr_cont!("UX "); }

    match (*nr).sr & ST0_KSU {
        KSU_USER => pr_cont!("USER "),
        KSU_SUPERVISOR => pr_cont!("SUPERVISOR "),
        KSU_KERNEL => pr_cont!("KERNEL "),
        _ => pr_cont!("BAD_MODE "),
    }
    if (*nr).sr & ST0_ERL != 0 { pr_cont!("ERL "); }
    if (*nr).sr & ST0_EXL != 0 { pr_cont!("EXL "); }
    if (*nr).sr & ST0_IE != 0 { pr_cont!("IE "); }
    pr_cont!("\n");

    pr_emerg!("Cause : %08lx\n", (*nr).cause);
    pr_emerg!("PrId  : %08x\n", read_c0_prid());
    pr_emerg!("BadVA : %016lx\n", (*nr).badva);
    pr_emerg!("CErr  : %016lx\n", (*nr).cache_err);
    pr_emerg!("NMI_SR: %016lx\n", (*nr).nmi_sr);
    pr_emerg!("\n");
}

unsafe fn nmi_dump_hub_irq(nasid: nasid_t, slice: i32) {
    let (mask0, mask1) = if slice == 0 {
        (REMOTE_HUB_L(nasid, PI_INT_MASK0_A), REMOTE_HUB_L(nasid, PI_INT_MASK1_A))
    } else {
        (REMOTE_HUB_L(nasid, PI_INT_MASK0_B), REMOTE_HUB_L(nasid, PI_INT_MASK1_B))
    };
    let pend0 = REMOTE_HUB_L(nasid, PI_INT_PEND0);
    let pend1 = REMOTE_HUB_L(nasid, PI_INT_PEND1);
    pr_emerg!("PI_INT_MASK0: %16llx PI_INT_MASK1: %16llx\n", mask0, mask1);
    pr_emerg!("PI_INT_PEND0: %16llx PI_INT_PEND1: %16llx\n", pend0, pend1);
    pr_emerg!("\n\n");
}

unsafe fn nmi_node_eframe_save(nasid: nasid_t) {
    if nasid == INVALID_NASID { return; }
    let mut slice = 0;
    while slice < NODE_NUM_CPUS as i32 {
        nmi_cpu_eframe_save(nasid, slice);
        nmi_dump_hub_irq(nasid, slice);
        slice += 1;
    }
}

unsafe fn nmi_eframes_save() {
    for_each_online_node!(nasid => nmi_node_eframe_save(nasid));
}

unsafe fn nmi_dump() {
    #[cfg(not(feature = "REAL_NMI_SIGNAL"))]
    {
        static mut NMIED_CPUS: atomic_t = ATOMIC_INIT(0);
        atomic_inc(&mut NMIED_CPUS);
    }

    arch_spin_lock(&mut nmi_lock);

    #[cfg(feature = "REAL_NMI_SIGNAL")]
    {
        // The original waits up to 15 seconds, then sends one additional NMI
        // to CPUs which have not responded.
        let mut i = 0;
        while i < 1500 {
            for_each_online_node!(node => {
                if NODEPDA(node).dump_count == 0 { break; }
            });
            if node == MAX_NUMNODES { break; }
            if i == 1000 {
                for_each_online_node!(node => {
                    if NODEPDA(node).dump_count == 0 {
                        let mut cpu = cpumask_first(cpumask_of_node(node));
                        let mut n = 0;
                        while n < CNODE_NUM_CPUS(node) {
                            CPUMASK_SETB(&mut NMIED_CPUS, cpu);
                            send_nmi(cputonasid(cpu), cputoslice(cpu));
                            cpu += 1;
                            n += 1;
                        }
                    }
                });
            }
            udelay(10000);
            i += 1;
        }
    }
    #[cfg(not(feature = "REAL_NMI_SIGNAL"))]
    while atomic_read(&NMIED_CPUS) != num_online_cpus() {}

    nmi_eframes_save();
    LOCAL_HUB_S(NI_PORT_RESET, NPR_PORTRESET | NPR_LOCALRESET);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
