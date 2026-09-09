/*
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of this
 * archive for more details.
 *
 * Copyright (C) 2000 - 2001 by Kanoj Sarcar (kanoj@sgi.com)
 * Copyright (C) 2000 - 2001 by Silicon Graphics, Inc.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

const CPU_NONE: cpuid_t = -1i32 as cpuid_t;

static mut HUB_INIT_MASK: [usize; MAX_NUMNODES / (usize::BITS as usize)] =
    [0; MAX_NUMNODES / (usize::BITS as usize)];
static mut master_nasid: nasid_t = INVALID_NASID;

#[no_mangle]
pub static mut sn_cpu_info: [cpuinfo_ip27; NR_CPUS] = [cpuinfo_ip27::ZERO; NR_CPUS];

unsafe fn per_hub_init(nasid: nasid_t) {
    let hub: *mut hub_data = hub_data(nasid);

    cpumask_set_cpu(smp_processor_id(), &mut (*hub).h_cpus);

    if test_and_set_bit(nasid, HUB_INIT_MASK.as_mut_ptr()) != 0 {
        return;
    }
    /*
     * Set CRB timeout at 5ms, (< PI timeout of 10ms)
     */
    REMOTE_HUB_S(nasid, IIO_ICTP, 0x800);
    REMOTE_HUB_S(nasid, IIO_ICTO, 0xff);

    hub_rtc_init(nasid);

    if nasid != 0 {
        /* copy exception handlers from first node to current node */
        memcpy(
            NODE_OFFSET_TO_K0(nasid, 0) as *mut core::ffi::c_void,
            CKSEG0 as *const core::ffi::c_void,
            0x200,
        );
        __flush_cache_all();
        /* switch to node local exception handlers */
        REMOTE_HUB_S(nasid, PI_CALIAS_SIZE, PI_CALIAS_SIZE_8K);
    }
}

pub unsafe fn per_cpu_init() {
    let cpu = smp_processor_id();
    let nasid: nasid_t = get_nasid();

    clear_c0_status(ST0_IM);

    per_hub_init(nasid);

    pr_info!("CPU {} clock is {}MHz.\n", cpu, sn_cpu_info[cpu].p_speed);

    install_ipi();

    /* Install our NMI handler if symmon hasn't installed one. */
    install_cpu_nmi_handler(cputoslice(cpu));

    enable_percpu_irq(IP27_HUB_PEND0_IRQ, IRQ_TYPE_NONE);
    enable_percpu_irq(IP27_HUB_PEND1_IRQ, IRQ_TYPE_NONE);
}

pub unsafe fn plat_mem_setup() {
    let mut p: u64;
    let mut e: u64;
    let mut n_mode: u64;
    let nid: nasid_t;

    register_smp_ops(&ip27_smp_ops);

    ip27_reboot_setup();

    /*
     * hub_rtc init and cpu clock intr enabled for later calibrate_delay.
     */
    nid = get_nasid();
    printk!("IP27: Running on node {}.\n", nid);

    p = LOCAL_HUB_L(PI_CPU_PRESENT_A) & 1;
    e = LOCAL_HUB_L(PI_CPU_ENABLE_A) & 1;
    printk!(
        "Node {} has {} primary CPU{}.\n",
        nid,
        if p != 0 { "a" } else { "no" },
        if e != 0 { ", CPU is running" } else { "" },
    );

    p = LOCAL_HUB_L(PI_CPU_PRESENT_B) & 1;
    e = LOCAL_HUB_L(PI_CPU_ENABLE_B) & 1;
    printk!(
        "Node {} has {} secondary CPU{}.\n",
        nid,
        if p != 0 { "a" } else { "no" },
        if e != 0 { ", CPU is running" } else { "" },
    );

    /*
     * Try to catch kernel missconfigurations and give user an
     * indication what option to select.
     */
    n_mode = LOCAL_HUB_L(NI_STATUS_REV_ID) & NSRI_MORENODES_MASK;
    printk!("Machine is in {} mode.\n", if n_mode != 0 { 'N' } else { 'M' });
    #[cfg(CONFIG_SGI_SN_N_MODE)]
    if n_mode == 0 {
        panic!("Kernel compiled for M mode.");
    }

    #[cfg(not(CONFIG_SGI_SN_N_MODE))]
    if n_mode != 0 {
        panic!("Kernel compiled for N mode.");
    }

    ioport_resource.start = 0;
    ioport_resource.end = !0usize;
    set_io_port_base(IO_BASE);
}

pub unsafe fn get_system_type() -> *const core::ffi::c_char {
    b"SGI Origin\0".as_ptr() as *const core::ffi::c_char
}

pub unsafe fn prom_init() {
    prom_init_cmdline(fw_arg0, fw_arg1 as *mut LONG);
    prom_meminit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
