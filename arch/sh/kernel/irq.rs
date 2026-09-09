// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/kernel/irq.c
 *
 * Copyright (C) 1992, 1998 Linus Torvalds, Ingo Molnar
 *
 * SuperH version: Copyright (C) 1999 Niibe Yutaka
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, matching the original C includes.

extern "C" {
    static mut irq_err_count: atomic_t;
    fn printk(fmt: *const u8, ...);
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_read(v: *const atomic_t) -> u32;
}

#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

pub unsafe extern "C" fn ack_bad_irq(irq: u32) {
    atomic_inc(&mut irq_err_count);
    printk(b"unexpected IRQ trap at vector %02x\n\0".as_ptr(), irq);
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: i32) -> i32 {
    let mut j: i32;

    seq_printf(p, b"%*s:\0".as_ptr(), prec, b"NMI\0".as_ptr());
    for_each_online_cpu!(j, {
        seq_put_decimal_ull_width(p, b" \0".as_ptr(), per_cpu!(irq_stat.__nmi_count, j), 10);
    });
    seq_printf(p, b" Non-maskable interrupts\n\0".as_ptr());

    seq_printf(
        p,
        b"%*s: %10u\n\0".as_ptr(),
        prec,
        b"ERR\0".as_ptr(),
        atomic_read(&irq_err_count),
    );

    0
}

#[cfg(CONFIG_IRQSTACKS)]
#[repr(C)]
pub union irq_ctx {
    pub tinfo: thread_info,
    pub stack: [u32; THREAD_SIZE / core::mem::size_of::<u32>()],
}

#[cfg(CONFIG_IRQSTACKS)]
static mut hardirq_ctx: [*mut irq_ctx; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
#[cfg(CONFIG_IRQSTACKS)]
static mut softirq_ctx: [*mut irq_ctx; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
#[cfg(CONFIG_IRQSTACKS)]
static mut softirq_stack: [u8; NR_CPUS * THREAD_SIZE] = [0; NR_CPUS * THREAD_SIZE];
#[cfg(CONFIG_IRQSTACKS)]
static mut hardirq_stack: [u8; NR_CPUS * THREAD_SIZE] = [0; NR_CPUS * THREAD_SIZE];

#[cfg(CONFIG_IRQSTACKS)]
unsafe fn handle_one_irq(irq: u32) {
    let curctx = current_thread_info() as *mut irq_ctx;
    let irqctx = hardirq_ctx[smp_processor_id() as usize];

    if curctx != irqctx {
        let isp = (irqctx as *mut u8).add(core::mem::size_of::<irq_ctx>()) as *mut u32;
        (*irqctx).tinfo.task = (*curctx).tinfo.task;
        (*irqctx).tinfo.previous_sp = current_stack_pointer;
        (*irqctx).tinfo.preempt_count =
            ((*irqctx).tinfo.preempt_count & !SOFTIRQ_MASK) |
            ((*curctx).tinfo.preempt_count & SOFTIRQ_MASK);

        // Original SH inline assembly switches to the IRQ stack, invokes
        // generic_handle_irq, and restores the original stack.
        generic_handle_irq(irq);
        let _ = isp;
    } else {
        generic_handle_irq(irq);
    }
}

#[cfg(not(CONFIG_IRQSTACKS))]
unsafe fn handle_one_irq(irq: u32) {
    generic_handle_irq(irq);
}

#[cfg(CONFIG_IRQSTACKS)]
pub unsafe extern "C" fn irq_ctx_init(cpu: i32) {
    if !hardirq_ctx[cpu as usize].is_null() {
        return;
    }

    let mut irqctx = hardirq_stack.as_mut_ptr().add(cpu as usize * THREAD_SIZE) as *mut irq_ctx;
    (*irqctx).tinfo.task = core::ptr::null_mut();
    (*irqctx).tinfo.cpu = cpu;
    (*irqctx).tinfo.preempt_count = HARDIRQ_OFFSET;
    (*irqctx).tinfo.addr_limit = MAKE_MM_SEG(0);
    hardirq_ctx[cpu as usize] = irqctx;

    irqctx = softirq_stack.as_mut_ptr().add(cpu as usize * THREAD_SIZE) as *mut irq_ctx;
    (*irqctx).tinfo.task = core::ptr::null_mut();
    (*irqctx).tinfo.cpu = cpu;
    (*irqctx).tinfo.preempt_count = 0;
    (*irqctx).tinfo.addr_limit = MAKE_MM_SEG(0);
    softirq_ctx[cpu as usize] = irqctx;

    printk(b"CPU %u irqstacks, hard=%p soft=%p\n\0".as_ptr(), cpu, hardirq_ctx[cpu as usize], softirq_ctx[cpu as usize]);
}

#[cfg(CONFIG_IRQSTACKS)]
pub unsafe extern "C" fn irq_ctx_exit(cpu: i32) {
    hardirq_ctx[cpu as usize] = core::ptr::null_mut();
}

#[cfg(all(CONFIG_IRQSTACKS, CONFIG_SOFTIRQ_ON_OWN_STACK))]
pub unsafe extern "C" fn do_softirq_own_stack() {
    let curctx = current_thread_info();
    let irqctx = softirq_ctx[smp_processor_id() as usize];
    (*irqctx).tinfo.task = (*curctx).task;
    (*irqctx).tinfo.previous_sp = current_stack_pointer;
    let isp = (irqctx as *mut u8).add(core::mem::size_of::<irq_ctx>()) as *mut u32;

    // Original SH inline assembly switches to the softirq stack, invokes
    // __do_softirq, and restores the thread stack.
    __do_softirq();
    let _ = isp;
}

pub unsafe extern "C" fn do_IRQ(irq: u32, regs: *mut pt_regs) -> i32 {
    let old_regs = set_irq_regs(regs);
    irq_enter();
    let irq = irq_demux(irq_lookup(irq));

    if irq != NO_IRQ_IGNORE {
        handle_one_irq(irq);
        irq_finish(irq);
    }

    irq_exit();
    set_irq_regs(old_regs);
    IRQ_HANDLED
}

pub unsafe extern "C" fn init_IRQ() {
    plat_irq_setup();
    if !sh_mv.mv_init_irq.is_null() {
        ((*sh_mv.mv_init_irq)());
    }
    intc_finalize();
    irq_ctx_init(smp_processor_id());
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe extern "C" fn migrate_irqs() {
    let cpu = smp_processor_id();
    for_each_active_irq!(irq, {
        let data = irq_get_irq_data(irq);
        if irq_data_get_node(data) == cpu {
            let mask = irq_data_get_affinity_mask(data);
            let newcpu = cpumask_any_and(mask, cpu_online_mask);
            if newcpu >= nr_cpu_ids {
                pr_info_ratelimited!(b"IRQ%u no longer affine to CPU%u\n\0".as_ptr(), irq, cpu);
                irq_set_affinity(irq, cpu_all_mask);
            } else {
                irq_set_affinity(irq, mask);
            }
        }
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
