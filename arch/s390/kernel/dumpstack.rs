// SPDX-License-Identifier: GPL-2.0
/*
 * Stack dumping functions
 *
 *  Copyright IBM Corp. 1999, 2013
 */

// Kernel and architecture dependencies supplied by other translation units.

pub unsafe fn stack_type_name(type_: stack_type) -> *const core::ffi::c_char {
    match type_ {
        STACK_TYPE_TASK => b"task\0".as_ptr() as *const _,
        STACK_TYPE_IRQ => b"irq\0".as_ptr() as *const _,
        STACK_TYPE_NODAT => b"nodat\0".as_ptr() as *const _,
        STACK_TYPE_RESTART => b"restart\0".as_ptr() as *const _,
        _ => b"unknown\0".as_ptr() as *const _,
    }
}

// EXPORT_SYMBOL_GPL(stack_type_name);

#[inline]
unsafe fn in_stack(sp: c_ulong, info: *mut stack_info, type_: stack_type, stack: c_ulong) -> bool {
    if sp < stack || sp >= stack.wrapping_add(THREAD_SIZE) {
        return false;
    }
    (*info).type_ = type_;
    (*info).begin = stack;
    (*info).end = stack.wrapping_add(THREAD_SIZE);
    true
}

unsafe fn in_task_stack(sp: c_ulong, task: *mut task_struct, info: *mut stack_info) -> bool {
    let stack = task_stack_page(task) as c_ulong;
    in_stack(sp, info, STACK_TYPE_TASK, stack)
}

unsafe fn in_irq_stack(sp: c_ulong, info: *mut stack_info) -> bool {
    let stack = (*get_lowcore()).async_stack.wrapping_sub(STACK_INIT_OFFSET);
    in_stack(sp, info, STACK_TYPE_IRQ, stack)
}

unsafe fn in_nodat_stack(sp: c_ulong, info: *mut stack_info) -> bool {
    let stack = (*get_lowcore()).nodat_stack.wrapping_sub(STACK_INIT_OFFSET);
    in_stack(sp, info, STACK_TYPE_NODAT, stack)
}

unsafe fn in_mcck_stack(sp: c_ulong, info: *mut stack_info) -> bool {
    let stack = (*get_lowcore()).mcck_stack.wrapping_sub(STACK_INIT_OFFSET);
    in_stack(sp, info, STACK_TYPE_MCCK, stack)
}

unsafe fn in_restart_stack(sp: c_ulong, info: *mut stack_info) -> bool {
    let stack = (*get_lowcore()).restart_stack.wrapping_sub(STACK_INIT_OFFSET);
    in_stack(sp, info, STACK_TYPE_RESTART, stack)
}

pub unsafe fn get_stack_info(
    sp: c_ulong,
    task: *mut task_struct,
    info: *mut stack_info,
    visit_mask: *mut c_ulong,
) -> c_int {
    if sp == 0 || (sp & 0x7) != 0 {
        (*info).type_ = STACK_TYPE_UNKNOWN;
        return -EINVAL;
    }

    if in_task_stack(sp, task, info) {
        // recursion_check
    } else {
        if task != current {
            (*info).type_ = STACK_TYPE_UNKNOWN;
            return -EINVAL;
        }
        if !in_irq_stack(sp, info)
            && !in_nodat_stack(sp, info)
            && !in_restart_stack(sp, info)
            && !in_mcck_stack(sp, info)
        {
            (*info).type_ = STACK_TYPE_UNKNOWN;
            return -EINVAL;
        }
    }

    if (*visit_mask & (1 as c_ulong).wrapping_shl((*info).type_ as u32)) != 0 {
        (*info).type_ = STACK_TYPE_UNKNOWN;
        return -EINVAL;
    }
    *visit_mask |= (1 as c_ulong).wrapping_shl((*info).type_ as u32);
    0
}

pub unsafe fn show_stack(task: *mut task_struct, stack: *mut c_ulong, loglvl: *const c_char) {
    let mut state: unwind_state = core::mem::zeroed();
    printk(b"%sCall Trace:\n\0".as_ptr() as *const _, loglvl);
    unwind_for_each_frame(&mut state, task, core::ptr::null_mut(), stack as c_ulong) {
        printk(
            if state.reliable { b"%s [<%016lx>] %pSR \n\0" } else { b"%s([<%016lx>] %pSR)\n\0" }.as_ptr() as *const _,
            loglvl, state.ip, state.ip as *mut core::ffi::c_void,
        );
    }
    debug_show_held_locks(if !task.is_null() { task } else { current });
}

unsafe fn show_last_breaking_event(regs: *mut pt_regs) {
    printk(b"Last Breaking-Event-Address:\n\0".as_ptr() as *const _);
    printk(b" [<%016lx>] \0".as_ptr() as *const _, (*regs).last_break);
    if user_mode(regs) {
        print_vma_addr(KERN_CONT, (*regs).last_break);
        pr_cont(b"\n\0".as_ptr() as *const _);
    } else {
        pr_cont(b"%pSR\n\0".as_ptr() as *const _, (*regs).last_break as *mut _);
    }
}

pub unsafe fn show_registers(regs: *mut pt_regs) {
    let psw = &mut psw_bits((*regs).psw);
    let mut pswaddr = (*regs).psw.addr;
    if test_pt_regs_flag(regs, PIF_PSW_ADDR_ADJUSTED) {
        pswaddr = __forward_psw((*regs).psw, (*regs).int_code >> 16);
    }
    let mode = if user_mode(regs) { b"User\0" } else { b"Krnl\0" };
    printk(b"%s PSW : %px %px\0".as_ptr() as *const _, mode.as_ptr(), (*regs).psw.mask as *mut _, pswaddr as *mut _);
    if !user_mode(regs) { pr_cont(b" (%pSR)\0".as_ptr() as *const _, pswaddr as *mut _); }
    pr_cont(b"\n\0".as_ptr() as *const _);
    printk(b"           R:%x T:%x IO:%x EX:%x Key:%x M:%x W:%x P:%x AS:%x CC:%x PM:%x\0".as_ptr() as *const _, psw.per, psw.dat, psw.io, psw.ext, psw.key, psw.mcheck, psw.wait, psw.pstate, psw.as_, psw.cc, psw.pm);
    pr_cont(b" RI:%x EA:%x\n\0".as_ptr() as *const _, psw.ri, psw.eaba);
    printk(b"%s GPRS: %016lx %016lx %016lx %016lx\n\0".as_ptr() as *const _, mode.as_ptr(), (*regs).gprs[0], (*regs).gprs[1], (*regs).gprs[2], (*regs).gprs[3]);
    printk(b"           %016lx %016lx %016lx %016lx\n\0".as_ptr() as *const _, (*regs).gprs[4], (*regs).gprs[5], (*regs).gprs[6], (*regs).gprs[7]);
    printk(b"           %016lx %016lx %016lx %016lx\n\0".as_ptr() as *const _, (*regs).gprs[8], (*regs).gprs[9], (*regs).gprs[10], (*regs).gprs[11]);
    printk(b"           %016lx %016lx %016lx %016lx\n\0".as_ptr() as *const _, (*regs).gprs[12], (*regs).gprs[13], (*regs).gprs[14], (*regs).gprs[15]);
    show_code(regs);
}

pub unsafe fn show_regs(regs: *mut pt_regs) {
    show_regs_print_info(KERN_DEFAULT);
    show_registers(regs);
    if !user_mode(regs) { show_stack(core::ptr::null_mut(), (*regs).gprs[15] as *mut c_ulong, KERN_DEFAULT); }
    show_last_breaking_event(regs);
}

static mut die_lock: spinlock_t = DEFINE_SPINLOCK();

pub unsafe fn die(regs: *mut pt_regs, str_: *const c_char) -> ! {
    static mut die_counter: c_int = 0;
    oops_enter(); lgr_info_log(); debug_stop_all(); console_verbose();
    spin_lock_irq(&mut die_lock); bust_spinlocks(1);
    die_counter += 1;
    printk(b"%s: %04x ilc:%d [#%d]\0".as_ptr() as *const _, str_, (*regs).int_code & 0xffff, (*regs).int_code >> 17, die_counter);
    pr_cont(b"SMP \0".as_ptr() as *const _);
    if debug_pagealloc_enabled() { pr_cont(b"DEBUG_PAGEALLOC\0".as_ptr() as *const _); }
    pr_cont(b"\n\0".as_ptr() as *const _);
    notify_die(DIE_OOPS, str_, regs, 0, (*regs).int_code & 0xffff, SIGSEGV);
    print_modules(); show_regs(regs); bust_spinlocks(0);
    add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE); spin_unlock_irq(&mut die_lock);
    if in_interrupt() { panic(b"Fatal exception in interrupt\0".as_ptr() as *const _); }
    if panic_on_oops != 0 { panic(b"Fatal exception: panic_on_oops\0".as_ptr() as *const _); }
    oops_exit(); make_task_dead(SIGSEGV);
    core::hint::unreachable_unchecked()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
