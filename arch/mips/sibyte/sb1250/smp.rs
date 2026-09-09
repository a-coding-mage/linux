// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001, 2002, 2003 Broadcom Corporation
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

static mut MAILBOX_SET_REGS: [*mut core::ffi::c_void; 2] = [
    IOADDR(A_IMR_CPU0_BASE + R_IMR_MAILBOX_SET_CPU),
    IOADDR(A_IMR_CPU1_BASE + R_IMR_MAILBOX_SET_CPU),
];

static mut MAILBOX_CLEAR_REGS: [*mut core::ffi::c_void; 2] = [
    IOADDR(A_IMR_CPU0_BASE + R_IMR_MAILBOX_CLR_CPU),
    IOADDR(A_IMR_CPU1_BASE + R_IMR_MAILBOX_CLR_CPU),
];

static mut MAILBOX_REGS: [*mut core::ffi::c_void; 2] = [
    IOADDR(A_IMR_CPU0_BASE + R_IMR_MAILBOX_CPU),
    IOADDR(A_IMR_CPU1_BASE + R_IMR_MAILBOX_CPU),
];

/*
 * SMP init and finish on secondary CPUs
 */
pub unsafe fn sb1250_smp_init() {
    let imask: u32 = STATUSF_IP4 | STATUSF_IP3 | STATUSF_IP2 |
        STATUSF_IP1 | STATUSF_IP0;

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
unsafe fn sb1250_send_ipi_single(cpu: i32, action: u32) {
    __raw_writeq((action as u64) << 48, MAILBOX_SET_REGS[cpu as usize]);
}

unsafe fn sb1250_send_ipi_mask(mask: *const cpumask, action: u32) {
    let mut i: u32;

    for_each_cpu!(i, mask, {
        sb1250_send_ipi_single(i as i32, action);
    });
}

/*
 * Code to run on secondary just after probing the CPU
 */
unsafe fn sb1250_init_secondary() {
    sb1250_smp_init();
}

/*
 * Do any tidying up before marking online and running the idle
 * loop
 */
unsafe fn sb1250_smp_finish() {
    sb1250_clockevent_init();
    local_irq_enable();
}

/*
 * Setup the PC, SP, and GP of a secondary processor and start it
 * running!
 */
unsafe fn sb1250_boot_secondary(cpu: i32, idle: *mut task_struct) -> i32 {
    let retval = cfe_cpu_start(
        cpu_logical_map(cpu),
        &smp_bootstrap,
        __KSTK_TOS!(idle),
        task_thread_info(idle) as usize,
        0,
    );
    if retval != 0 {
        printk!("cfe_start_cpu(%i) returned %i\n", cpu, retval);
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
unsafe fn sb1250_smp_setup() {
    let mut num: i32 = 0;

    init_cpu_possible(cpumask_of(0));
    __cpu_number_map[0] = 0;
    __cpu_logical_map[0] = 0;

    for i in 1..NR_CPUS {
        if cfe_cpu_stop(i as i32) == 0 {
            set_cpu_possible(i as i32, true);
            num += 1;
            __cpu_number_map[i] = num;
            __cpu_logical_map[num as usize] = i as i32;
        }
    }
    printk!(KERN_INFO "Detected %i available secondary CPU(s)\n", num);
}

unsafe fn sb1250_prepare_cpus(_max_cpus: u32) {
}

pub static mut SB_SMP_OPS: plat_smp_ops = plat_smp_ops {
    send_ipi_single: Some(sb1250_send_ipi_single),
    send_ipi_mask: Some(sb1250_send_ipi_mask),
    init_secondary: Some(sb1250_init_secondary),
    smp_finish: Some(sb1250_smp_finish),
    boot_secondary: Some(sb1250_boot_secondary),
    smp_setup: Some(sb1250_smp_setup),
    prepare_cpus: Some(sb1250_prepare_cpus),
};

pub unsafe fn sb1250_mailbox_interrupt() {
    let cpu = smp_processor_id();
    let irq = K_INT_MBOX_0;
    let mut action: u32;

    kstat_incr_irq_this_cpu(irq);
    /* Load the mailbox register to figure out what we're supposed to do */
    action = (____raw_readq(MAILBOX_REGS[cpu as usize]) >> 48) as u32 & 0xffff;

    /* Clear the mailbox to clear the interrupt */
    ____raw_writeq((action as u64) << 48, MAILBOX_CLEAR_REGS[cpu as usize]);

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
