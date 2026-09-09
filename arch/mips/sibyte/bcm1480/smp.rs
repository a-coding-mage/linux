// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001,2002,2004 Broadcom Corporation
 */

// These are routines for dealing with the bcm1480 smp capabilities
// independent of board/firmware

static mut MAILBOX_0_SET_REGS: [*mut core::ffi::c_void; 4] = [
    IOADDR(A_BCM1480_IMR_CPU0_BASE + R_BCM1480_IMR_MAILBOX_0_SET_CPU),
    IOADDR(A_BCM1480_IMR_CPU1_BASE + R_BCM1480_IMR_MAILBOX_0_SET_CPU),
    IOADDR(A_BCM1480_IMR_CPU2_BASE + R_BCM1480_IMR_MAILBOX_0_SET_CPU),
    IOADDR(A_BCM1480_IMR_CPU3_BASE + R_BCM1480_IMR_MAILBOX_0_SET_CPU),
];

static mut MAILBOX_0_CLEAR_REGS: [*mut core::ffi::c_void; 4] = [
    IOADDR(A_BCM1480_IMR_CPU0_BASE + R_BCM1480_IMR_MAILBOX_0_CLR_CPU),
    IOADDR(A_BCM1480_IMR_CPU1_BASE + R_BCM1480_IMR_MAILBOX_0_CLR_CPU),
    IOADDR(A_BCM1480_IMR_CPU2_BASE + R_BCM1480_IMR_MAILBOX_0_CLR_CPU),
    IOADDR(A_BCM1480_IMR_CPU3_BASE + R_BCM1480_IMR_MAILBOX_0_CLR_CPU),
];

static mut MAILBOX_0_REGS: [*mut core::ffi::c_void; 4] = [
    IOADDR(A_BCM1480_IMR_CPU0_BASE + R_BCM1480_IMR_MAILBOX_0_CPU),
    IOADDR(A_BCM1480_IMR_CPU1_BASE + R_BCM1480_IMR_MAILBOX_0_CPU),
    IOADDR(A_BCM1480_IMR_CPU2_BASE + R_BCM1480_IMR_MAILBOX_0_CPU),
    IOADDR(A_BCM1480_IMR_CPU3_BASE + R_BCM1480_IMR_MAILBOX_0_CPU),
];

/* SMP init and finish on secondary CPUs */
pub unsafe fn bcm1480_smp_init() {
    let imask: u32 = STATUSF_IP4 | STATUSF_IP3 | STATUSF_IP2 | STATUSF_IP1 | STATUSF_IP0;

    /* Set interrupt mask, but don't enable */
    change_c0_status(ST0_IM, imask);
}

/*
 * These are routines for dealing with the sb1250 smp capabilities
 * independent of board/firmware
 */

/*
 * Simple enough; everything is set up, so just poke the appropriate mailbox
 * register, and we should be set
 */
unsafe fn bcm1480_send_ipi_single(cpu: i32, action: u32) {
    __raw_writeq((action as u64) << 48, MAILBOX_0_SET_REGS[cpu as usize]);
}

unsafe fn bcm1480_send_ipi_mask(mask: *const cpumask, action: u32) {
    let mut i: u32 = 0;
    for_each_cpu(i, mask, {
        bcm1480_send_ipi_single(i as i32, action);
    });
}

/* Code to run on secondary just after probing the CPU */
unsafe fn bcm1480_init_secondary() {
    bcm1480_smp_init();
}

/* Do any tidying up before marking online and running the idle loop */
unsafe fn bcm1480_smp_finish() {
    sb1480_clockevent_init();
    local_irq_enable();
}

/* Setup the PC, SP, and GP of a secondary processor and start it running! */
unsafe fn bcm1480_boot_secondary(cpu: i32, idle: *mut task_struct) -> i32 {
    let retval = cfe_cpu_start(
        cpu_logical_map(cpu),
        &smp_bootstrap,
        __KSTK_TOS(idle),
        task_thread_info(idle) as usize as u64,
        0,
    );
    if retval != 0 {
        printk("cfe_start_cpu(%i) returned %i\n", cpu, retval);
    }
    retval
}

/*
 * Use CFE to find out how many CPUs are available, setting up
 * cpu_possible_mask and the logical/physical mappings.
 * XXXKW will the boot CPU ever not be physical 0?
 *
 * Common setup before any secondaries are started
 */
unsafe fn bcm1480_smp_setup() {
    let mut num = 0;
    init_cpu_possible(cpumask_of(0));
    __cpu_number_map[0] = 0;
    __cpu_logical_map[0] = 0;

    for i in 1..NR_CPUS {
        if cfe_cpu_stop(i) == 0 {
            set_cpu_possible(i, true);
            num += 1;
            __cpu_number_map[i] = num;
            __cpu_logical_map[num] = i;
        }
    }
    printk(KERN_INFO "Detected %i available secondary CPU(s)\n", num);
}

unsafe fn bcm1480_prepare_cpus(_max_cpus: u32) {}

pub static bcm1480_smp_ops: plat_smp_ops = plat_smp_ops {
    send_ipi_single: Some(bcm1480_send_ipi_single),
    send_ipi_mask: Some(bcm1480_send_ipi_mask),
    init_secondary: Some(bcm1480_init_secondary),
    smp_finish: Some(bcm1480_smp_finish),
    boot_secondary: Some(bcm1480_boot_secondary),
    smp_setup: Some(bcm1480_smp_setup),
    prepare_cpus: Some(bcm1480_prepare_cpus),
};

pub unsafe fn bcm1480_mailbox_interrupt() {
    let cpu = smp_processor_id();
    let irq = K_BCM1480_INT_MBOX_0_0;
    let action = ((__raw_readq(MAILBOX_0_REGS[cpu as usize]) >> 48) & 0xffff) as u32;

    kstat_incr_irq_this_cpu(irq);
    /* Load the mailbox register to figure out what we're supposed to do */

    /* Clear the mailbox to clear the interrupt */
    __raw_writeq((action as u64) << 48, MAILBOX_0_CLEAR_REGS[cpu as usize]);

    if action & SMP_RESCHEDULE_YOURSELF != 0 {
        scheduler_ipi();
    }

    if action & SMP_CALL_FUNCTION != 0 {
        irq_enter();
        generic_smp_call_function_interrupt();
        irq_exit();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
