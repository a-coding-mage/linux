// SPDX-License-Identifier: GPL-2.0
/*
 * SH-X3 SMP
 *
 *  Copyright (C) 2007 - 2010  Paul Mundt
 *  Copyright (C) 2007  Magnus Damm
 */

// Dependencies supplied by the surrounding kernel translation.

const STBCR_MSTP: u32 = 0x00000001;
const STBCR_RESET: u32 = 0x00000002;
const STBCR_SLEEP: u32 = 0x00000004;
const STBCR_LTSLP: u32 = 0x80000000;

#[inline]
const fn stbcr_reg(phys_id: u32) -> usize {
    (0xfe400004u32 | (phys_id << 12)) as usize
}

#[inline]
const fn reset_reg(phys_id: u32) -> usize {
    (0xfe400008u32 | (phys_id << 12)) as usize
}

unsafe fn ipi_interrupt_handler(irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let message = arg as isize as u32;
    let cpu = hard_smp_processor_id();
    let offs = 4 * cpu;
    let mut x: u32;

    x = __raw_readl(0xfe410070usize + offs as usize); /* C0INITICI..CnINTICI */
    x &= 1u32 << (message << 2);
    __raw_writel(x, 0xfe410080usize + offs as usize); /* C0INTICICLR..CnINTICICLR */

    smp_message_recv(message);

    IRQ_HANDLED
}

unsafe fn shx3_smp_setup() {
    let cpu: u32 = 0;
    let mut i: i32;
    let mut num: i32;

    init_cpu_possible(cpumask_of(cpu));

    /* Enable light sleep for the boot CPU */
    __raw_writel(__raw_readl(stbcr_reg(cpu)) | STBCR_LTSLP, stbcr_reg(cpu));

    __cpu_number_map[0] = 0;
    __cpu_logical_map[0] = 0;

    /*
     * Do this stupidly for now.. we don't have an easy way to probe
     * for the total number of cores.
     */
    i = 1;
    num = 0;
    while i < NR_CPUS {
        set_cpu_possible(i as u32, true);
        num += 1;
        __cpu_number_map[i as usize] = num;
        __cpu_logical_map[num as usize] = i;
        i += 1;
    }

    printk(KERN_INFO, "Detected %i available secondary CPU(s)\n", num);
}

unsafe fn shx3_prepare_cpus(max_cpus: u32) {
    let mut i: i32;

    BUILD_BUG_ON(SMP_MSG_NR >= 8);

    i = 0;
    while i < SMP_MSG_NR {
        if request_irq(
            104 + i,
            Some(ipi_interrupt_handler),
            IRQF_PERCPU,
            c"IPI".as_ptr(),
            i as isize as *mut core::ffi::c_void,
        ) != 0 {
            pr_err("Failed to request irq %d\n", i);
        }
        i += 1;
    }

    i = 0;
    while (i as u32) < max_cpus {
        set_cpu_present(i as u32, true);
        i += 1;
    }
}

unsafe fn shx3_start_cpu(cpu: u32, entry_point: usize) {
    if __in_29bit_mode() {
        __raw_writel(entry_point as u32, reset_reg(cpu));
    } else {
        __raw_writel(virt_to_phys(entry_point), reset_reg(cpu));
    }

    if (__raw_readl(stbcr_reg(cpu)) & STBCR_MSTP) == 0 {
        __raw_writel(STBCR_MSTP, stbcr_reg(cpu));
    }

    while (__raw_readl(stbcr_reg(cpu)) & STBCR_MSTP) == 0 {
        cpu_relax();
    }

    /* Start up secondary processor by sending a reset */
    __raw_writel(STBCR_RESET | STBCR_LTSLP, stbcr_reg(cpu));
}

unsafe fn shx3_smp_processor_id() -> u32 {
    __raw_readl(0xff000048usize) /* CPIDR */
}

unsafe fn shx3_send_ipi(cpu: u32, message: u32) {
    let addr = 0xfe410070usize + (cpu * 4) as usize;

    BUG_ON(cpu >= 4);

    __raw_writel(1u32 << (message << 2), addr); /* C0INTICI..CnINTICI */
}

unsafe fn shx3_update_boot_vector(cpu: u32) {
    __raw_writel(STBCR_MSTP, stbcr_reg(cpu));
    while (__raw_readl(stbcr_reg(cpu)) & STBCR_MSTP) == 0 {
        cpu_relax();
    }
    __raw_writel(STBCR_RESET, stbcr_reg(cpu));
}

unsafe fn shx3_cpu_prepare(cpu: u32) -> i32 {
    shx3_update_boot_vector(cpu);
    0
}

unsafe fn register_shx3_cpu_notifier() -> i32 {
    cpuhp_setup_state_nocalls(
        CPUHP_SH_SH3X_PREPARE,
        c"sh/shx3:prepare".as_ptr(),
        Some(shx3_cpu_prepare),
        None,
    );
    0
}

late_initcall!(register_shx3_cpu_notifier);

static mut shx3_smp_ops: plat_smp_ops = plat_smp_ops {
    smp_setup: Some(shx3_smp_setup),
    prepare_cpus: Some(shx3_prepare_cpus),
    start_cpu: Some(shx3_start_cpu),
    smp_processor_id: Some(shx3_smp_processor_id),
    send_ipi: Some(shx3_send_ipi),
    cpu_die: Some(native_cpu_die),
    cpu_disable: Some(native_cpu_disable),
    play_dead: Some(native_play_dead),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
