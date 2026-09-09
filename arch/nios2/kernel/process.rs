/*
 * Architecture-dependent parts of process handling.
 *
 * Copyright (C) 2013 Altera Corporation
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009 Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe extern "C" {
    fn ret_from_fork();
    fn ret_from_kernel_thread();
}

pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

pub unsafe extern "C" fn arch_cpu_idle() {}

/*
 * The development boards have no way to pull a board reset. Just jump to the
 * cpu reset address and let the boot loader or the code in head.S take care of
 * resetting peripherals.
 */
pub unsafe extern "C" fn machine_restart(__unused: *mut core::ffi::c_char) {
    pr_notice!("Machine restart (%08x)...\n", cpuinfo.reset_addr);
    local_irq_disable();
    core::arch::asm!("jmp {0}", in(reg) cpuinfo.reset_addr, clobber_abi("C"));
}

pub unsafe extern "C" fn machine_halt() {
    pr_notice!("Machine halt...\n");
    local_irq_disable();
    loop {}
}

/*
 * There is no way to power off the development boards. So just spin for now. If
 * we ever have a way of resetting a board using a GPIO we should add that here.
 */
pub unsafe extern "C" fn machine_power_off() {
    pr_notice!("Machine power off...\n");
    local_irq_disable();
    loop {}
}

pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) {
    pr_notice!("\n");
    show_regs_print_info(KERN_DEFAULT);
    pr_notice!("r1: %08lx r2: %08lx r3: %08lx r4: %08lx\n", (*regs).r1, (*regs).r2, (*regs).r3, (*regs).r4);
    pr_notice!("r5: %08lx r6: %08lx r7: %08lx r8: %08lx\n", (*regs).r5, (*regs).r6, (*regs).r7, (*regs).r8);
    pr_notice!("r9: %08lx r10: %08lx r11: %08lx r12: %08lx\n", (*regs).r9, (*regs).r10, (*regs).r11, (*regs).r12);
    pr_notice!("r13: %08lx r14: %08lx r15: %08lx\n", (*regs).r13, (*regs).r14, (*regs).r15);
    pr_notice!("ra: %08lx fp:  %08lx sp: %08lx gp: %08lx\n", (*regs).ra, (*regs).fp, (*regs).sp, (*regs).gp);
    pr_notice!("ea: %08lx estatus: %08lx\n", (*regs).ea, (*regs).estatus);
}

pub unsafe extern "C" fn flush_thread() {}

pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags: u64 = (*args).flags;
    let usp: usize = (*args).stack;
    let tls: usize = (*args).tls;
    let childregs: *mut pt_regs = task_pt_regs(p);
    let mut regs: *mut pt_regs;
    let mut stack: *mut switch_stack;
    let childstack: *mut switch_stack = (childregs as *mut switch_stack).sub(1);

    if unlikely((*args).fn_.is_some()) {
        core::ptr::write_bytes(childstack as *mut u8, 0, core::mem::size_of::<switch_stack>() + core::mem::size_of::<pt_regs>());
        (*childstack).r16 = (*args).fn_.unwrap() as usize;
        (*childstack).r17 = (*args).fn_arg as usize;
        (*childstack).ra = ret_from_kernel_thread as usize;
        (*childregs).estatus = STATUS_PIE;
        (*childregs).sp = childstack as usize;
        (*p).thread.ksp = childstack as usize;
        (*p).thread.kregs = childregs;
        return 0;
    }

    regs = current_pt_regs();
    *childregs = *regs;
    (*childregs).r2 = 0;
    (*childregs).r7 = 0;
    stack = (regs as *mut switch_stack).sub(1);
    *childstack = *stack;
    (*childstack).ra = ret_from_fork as usize;
    (*p).thread.kregs = childregs;
    (*p).thread.ksp = childstack as usize;
    if usp != 0 { (*childregs).sp = usp; }
    if clone_flags & CLONE_SETTLS != 0 { (*childstack).r23 = tls; }
    0
}

/*
 * Generic dumping code. Used for panic and debug.
 */
pub unsafe extern "C" fn dump(fp: *mut pt_regs) {
    let mut sp: *mut usize;
    let mut tp: *mut u8;
    let mut i: i32;
    pr_emerg!("\nCURRENT PROCESS:\n\n");
    pr_emerg!("COMM=%s PID=%d\n", current.comm, current.pid);
    if !current.mm.is_null() {
        pr_emerg!("TEXT=%08x-%08x DATA=%08x-%08x BSS=%08x-%08x\n", (*current.mm).start_code as i32, (*current.mm).end_code as i32, (*current.mm).start_data as i32, (*current.mm).end_data as i32, (*current.mm).end_data as i32, (*current.mm).brk as i32);
        pr_emerg!("USER-STACK=%08x  KERNEL-STACK=%08x\n\n", (*current.mm).start_stack as i32, ((&current as *const _) as usize + THREAD_SIZE) as i32);
    }
    pr_emerg!("PC: %08lx\n", (*fp).ea);
    pr_emerg!("SR: %08lx    SP: %08lx\n", (*fp).estatus as isize, fp as isize);
    pr_emerg!("r1: %08lx    r2: %08lx    r3: %08lx\n", (*fp).r1, (*fp).r2, (*fp).r3);
    pr_emerg!("r4: %08lx    r5: %08lx    r6: %08lx    r7: %08lx\n", (*fp).r4, (*fp).r5, (*fp).r6, (*fp).r7);
    pr_emerg!("r8: %08lx    r9: %08lx    r10: %08lx    r11: %08lx\n", (*fp).r8, (*fp).r9, (*fp).r10, (*fp).r11);
    pr_emerg!("r12: %08lx  r13: %08lx    r14: %08lx    r15: %08lx\n", (*fp).r12, (*fp).r13, (*fp).r14, (*fp).r15);
    pr_emerg!("or2: %08lx   ra: %08lx     fp: %08lx    sp: %08lx\n", (*fp).orig_r2, (*fp).ra, (*fp).fp, (*fp).sp);
    pr_emerg!("\nUSP: %08x   TRAPFRAME: %08x\n", (*fp).sp as u32, fp as u32);
    pr_emerg!("\nCODE:");
    tp = ((*fp).ea as *mut u8).sub(0x20);
    sp = tp as *mut usize; i = 0;
    while i < 0x40 { if i % 0x10 == 0 { pr_emerg!("\n%08x: ", tp.add(i as usize) as i32); } pr_emerg!("%08x ", *sp); sp = sp.add(1); i += 4; }
    pr_emerg!("\n\nKERNEL STACK:");
    tp = (fp as *mut u8).sub(0x40); sp = tp as *mut usize; i = 0;
    while i < 0xc0 { if i % 0x10 == 0 { pr_emerg!("\n%08x: ", tp.add(i as usize) as i32); } pr_emerg!("%08x ", *sp); sp = sp.add(1); i += 4; }
    pr_emerg!("\n\n\nUSER STACK:");
    tp = ((*fp).sp as *mut u8).sub(0x10); sp = tp as *mut usize; i = 0;
    while i < 0x80 { if i % 0x10 == 0 { pr_emerg!("\n%08x: ", tp.add(i as usize) as i32); } pr_emerg!("%08x ", *sp); sp = sp.add(1); i += 4; }
    pr_emerg!("\n\n");
}

pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> usize {
    let mut fp: usize;
    let mut pc: usize;
    let stack_page = p as usize;
    let mut count = 0;
    fp = (*( (*p).thread.ksp as *mut switch_stack)).fp;
    loop {
        if fp < stack_page + core::mem::size_of::<task_struct>() || fp >= 8184 + stack_page { return 0; }
        pc = *((fp as *mut usize).add(1));
        if !in_sched_functions(pc) { return pc; }
        fp = *(fp as *mut usize);
        count += 1;
        if count >= 16 { return 0; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
