// SPDX-License-Identifier: GPL-2.0
/*
 * ip30-smp.c: SMP on IP30 architecture.
 * Based off of the original IP30 SMP code, with inspiration from ip27-smp.c
 * and smp-bmips.c.
 *
 * Copyright (C) 2005-2007 Stanislaw Skowronek <skylark@unaligned.org>
 *               2006-2007, 2014-2015 Joshua Kinard <linux@kumba.dev>
 *               2009 Johannes Dickgreber <tanzy@gmx.de>
 */

// Linux and architecture dependencies supplied by the surrounding kernel.

const MPCONF_MAGIC: u32 = 0xbaddeed2;
const MPCONF_ADDR: u64 = 0xa800000000000600;
const MPCONF_SIZE: u64 = 0x80;

#[inline]
const fn mpconf(x: u64) -> u64 {
    MPCONF_ADDR + x * MPCONF_SIZE
}

/* HEART can theoretically do 4 CPUs, but only 2 are physically possible */
const MP_NCPU: usize = 2;

#[repr(C)]
struct Mpconf {
    magic: u32,
    prid: u32,
    physid: u32,
    virtid: u32,
    scachesz: u32,
    fanloads: u16,
    res: u16,
    launch: *mut core::ffi::c_void,
    rendezvous: *mut core::ffi::c_void,
    res2: [u64; 3],
    stackaddr: *mut core::ffi::c_void,
    lnch_parm: *mut core::ffi::c_void,
    rndv_parm: *mut core::ffi::c_void,
    idleflag: u32,
}

unsafe fn ip30_smp_send_ipi_single(cpu: i32, action: u32) {
    let mut irq: i32;

    match action {
        SMP_RESCHEDULE_YOURSELF => {
            irq = HEART_L2_INT_RESCHED_CPU_0;
        }
        SMP_CALL_FUNCTION => {
            irq = HEART_L2_INT_CALL_CPU_0;
        }
        _ => {
            panic!("IP30: Unknown action value in ip30_smp_send_ipi_single!\n");
        }
    }

    irq += cpu;

    /* Poke the other CPU -- it's got mail! */
    unsafe {
        heart_write((1u64).wrapping_shl(irq as u32), &mut (*heart_regs).set_isr);
    }
}

unsafe fn ip30_smp_send_ipi_mask(mask: *const cpumask, action: u32) {
    let mut i: u32 = 0;

    for_each_cpu!(i, mask, {
        ip30_smp_send_ipi_single(i as i32, action);
    });
}

unsafe fn ip30_smp_setup() {
    let mut i: i32;
    let mut ncpu: i32 = 0;
    let mut mpc: *mut Mpconf;

    init_cpu_possible(cpumask_of(0));

    /* Scan the MPCONF structure and enumerate available CPUs. */
    i = 0;
    while i < MP_NCPU as i32 {
        mpc = mpconf(i as u64) as *mut Mpconf;
        if (*mpc).magic == MPCONF_MAGIC {
            set_cpu_possible(i as u32, true);
            __cpu_number_map[i as usize] = {
                ncpu += 1;
                ncpu
            };
            __cpu_logical_map[ncpu as usize] = i as u32;
            pr_info!(
                "IP30: Slot: {}, PrID: {:08x}, PhyID: {}, VirtID: {}\n",
                i, (*mpc).prid, (*mpc).physid, (*mpc).virtid
            );
        }
        i += 1;
    }
    pr_info!("IP30: Detected {} CPU(s) present.\n", ncpu);

    /*
     * Set the coherency algorithm to '5' (cacheable coherent
     * exclusive on write).  This is needed on IP30 SMP, especially
     * for R14000 CPUs, otherwise, instruction bus errors will
     * occur upon reaching userland.
     */
    change_c0_config(CONF_CM_CMASK, CONF_CM_CACHABLE_COW);
}

unsafe fn ip30_smp_prepare_cpus(_max_cpus: u32) {
    /* nothing to do here */
}

unsafe fn ip30_smp_boot_secondary(cpu: i32, idle: *mut task_struct) -> i32 {
    let mpc = mpconf(cpu as u64) as *mut Mpconf;

    /* Stack pointer (sp). */
    (*mpc).stackaddr = __KSTK_TOS(idle) as *mut core::ffi::c_void;

    /* Global pointer (gp). */
    (*mpc).lnch_parm = task_thread_info(idle) as *mut core::ffi::c_void;

    mb(); /* make sure stack and lparm are written */

    /* Boot CPUx. */
    (*mpc).launch = smp_bootstrap as *mut core::ffi::c_void;

    /* CPUx now executes smp_bootstrap, then ip30_smp_finish */
    0
}

unsafe fn ip30_smp_init_cpu() {
    ip30_per_cpu_init();
}

unsafe fn ip30_smp_finish() {
    enable_percpu_irq(get_c0_compare_int(), IRQ_TYPE_NONE);
    local_irq_enable();
}

#[repr(C)]
struct PlatSmpOps {
    send_ipi_single: Option<unsafe fn(i32, u32)>,
    send_ipi_mask: Option<unsafe fn(*const cpumask, u32)>,
    smp_setup: Option<unsafe fn()>,
    prepare_cpus: Option<unsafe fn(u32)>,
    boot_secondary: Option<unsafe fn(i32, *mut task_struct) -> i32>,
    init_secondary: Option<unsafe fn()>,
    smp_finish: Option<unsafe fn()>,
    prepare_boot_cpu: Option<unsafe fn()>,
}

#[no_mangle]
static mut ip30_smp_ops: PlatSmpOps = PlatSmpOps {
    send_ipi_single: Some(ip30_smp_send_ipi_single),
    send_ipi_mask: Some(ip30_smp_send_ipi_mask),
    smp_setup: Some(ip30_smp_setup),
    prepare_cpus: Some(ip30_smp_prepare_cpus),
    boot_secondary: Some(ip30_smp_boot_secondary),
    init_secondary: Some(ip30_smp_init_cpu),
    smp_finish: Some(ip30_smp_finish),
    prepare_boot_cpu: Some(ip30_smp_init_cpu),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
