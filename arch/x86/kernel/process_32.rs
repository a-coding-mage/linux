/*
 *  Copyright (C) 1995  Linus Torvalds
 *
 *  Pentium III FXSR, SSE support
 *	Gareth Hughes <gareth@valinux.com>, May 2000
 */

/* This file handles the architecture-dependent parts of process handling. */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn __show_regs(
    regs: *mut pt_regs,
    mode: show_regs_mode,
    log_lvl: *const core::ffi::c_char,
) {
    let mut cr0: c_ulong = 0;
    let mut cr2: c_ulong = 0;
    let mut cr3: c_ulong = 0;
    let mut cr4: c_ulong = 0;
    let mut d0: c_ulong;
    let mut d1: c_ulong;
    let mut d2: c_ulong;
    let mut d3: c_ulong;
    let mut d6: c_ulong;
    let mut d7: c_ulong;
    let mut gs: c_uint;

    savesegment(gs, gs);

    show_ip(regs, log_lvl);

    printk(
        c"%sEAX: %08lx EBX: %08lx ECX: %08lx EDX: %08lx\n",
        log_lvl, (*regs).ax, (*regs).bx, (*regs).cx, (*regs).dx,
    );
    printk(
        c"%sESI: %08lx EDI: %08lx EBP: %08lx ESP: %08lx\n",
        log_lvl, (*regs).si, (*regs).di, (*regs).bp, (*regs).sp,
    );
    printk(
        c"%sDS: %04x ES: %04x FS: %04x GS: %04x SS: %04x EFLAGS: %08lx\n",
        log_lvl, (*regs).ds as u16, (*regs).es as u16, (*regs).fs as u16,
        gs, (*regs).ss, (*regs).flags,
    );

    if mode != SHOW_REGS_ALL {
        return;
    }

    cr0 = read_cr0();
    cr2 = read_cr2();
    cr3 = __read_cr3();
    cr4 = __read_cr4();
    printk(c"%sCR0: %08lx CR2: %08lx CR3: %08lx CR4: %08lx\n", log_lvl, cr0, cr2, cr3, cr4);

    get_debugreg!(d0, 0);
    get_debugreg!(d1, 1);
    get_debugreg!(d2, 2);
    get_debugreg!(d3, 3);
    get_debugreg!(d6, 6);
    get_debugreg!(d7, 7);

    /* Only print out debug registers if they are in their non-default state. */
    if d0 == 0 && d1 == 0 && d2 == 0 && d3 == 0 && d6 == DR6_RESERVED && d7 == DR7_FIXED_1 {
        return;
    }

    printk(c"%sDR0: %08lx DR1: %08lx DR2: %08lx DR3: %08lx\n", log_lvl, d0, d1, d2, d3);
    printk(c"%sDR6: %08lx DR7: %08lx\n", log_lvl, d6, d7);
}

pub unsafe fn release_thread(dead_task: *mut task_struct) {
    BUG_ON!((*dead_task).mm);
    release_vm86_irqs(dead_task);
}

pub unsafe fn start_thread(regs: *mut pt_regs, new_ip: c_ulong, new_sp: c_ulong) {
    loadsegment!(gs, 0);
    (*regs).fs = 0;
    (*regs).ds = __USER_DS;
    (*regs).es = __USER_DS;
    (*regs).ss = __USER_DS;
    (*regs).cs = __USER_CS;
    (*regs).ip = new_ip;
    (*regs).sp = new_sp;
    (*regs).flags = X86_EFLAGS_IF;
}

// EXPORT_SYMBOL_GPL(start_thread)

/*
 * switch_to(x,y) should switch tasks from x to y.
 *
 * The return value (in %ax) is the "prev" task after the task-switch.
 */
pub unsafe fn __switch_to(
    prev_p: *mut task_struct,
    next_p: *mut task_struct,
) -> *mut task_struct {
    let prev: *mut thread_struct = &mut (*prev_p).thread;
    let next: *mut thread_struct = &mut (*next_p).thread;
    let cpu: c_int = smp_processor_id();

    /* never put a printk in __switch_to... printk() calls wake_up*() indirectly */
    switch_fpu(prev_p, cpu);

    /* Save away %gs. No need to save %fs, %es, or %ds here. */
    savesegment(gs, (*prev).gs);

    /* Load the per-thread Thread-Local Storage descriptor. */
    load_TLS(next, cpu);

    switch_to_extra(prev_p, next_p);

    /* Leave lazy mode, flushing any hypercalls made here. */
    arch_end_context_switch(next_p);

    /* Reload esp0 and cpu_current_top_of_stack. */
    update_task_stack(next_p);
    refresh_sysenter_cs(next);
    this_cpu_write!(
        cpu_current_top_of_stack,
        (task_stack_page(next_p) as c_ulong).wrapping_add(THREAD_SIZE),
    );

    /* Restore %gs if needed (which is common). */
    if (*prev).gs | (*next).gs != 0 {
        loadsegment!(gs, (*next).gs);
    }

    raw_cpu_write!(current_task, next_p);

    /* Load the Intel cache allocation PQR MSR. */
    resctrl_arch_sched_in(next_p);

    prev_p
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
