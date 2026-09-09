/*
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  Copyright (C) 2000, 2001, 2002 Andi Kleen, SuSE Labs
 */

static mut die_counter: i32 = 0;
static mut exec_summary_regs: pt_regs = unsafe { core::mem::zeroed() };

pub unsafe fn in_task_stack(stack: *mut usize, task: *mut task_struct,
                            info: *mut stack_info) -> bool {
    let begin = task_stack_page(task);
    let end = begin.add(THREAD_SIZE);
    if stack < begin || stack >= end { return false; }
    (*info).type_ = STACK_TYPE_TASK;
    (*info).begin = begin;
    (*info).end = end;
    (*info).next_sp = core::ptr::null_mut();
    true
}

/* Called from get_stack_info_noinstr - so must be noinstr too */
pub unsafe fn in_entry_stack(stack: *mut usize, info: *mut stack_info) -> bool {
    let ss = cpu_entry_stack(smp_processor_id());
    let begin = ss as *mut core::ffi::c_void;
    let end = ss.add(1) as *mut core::ffi::c_void;
    if (stack as *mut core::ffi::c_void) < begin || (stack as *mut core::ffi::c_void) >= end { return false; }
    (*info).type_ = STACK_TYPE_ENTRY;
    (*info).begin = begin as *mut usize;
    (*info).end = end as *mut usize;
    (*info).next_sp = core::ptr::null_mut();
    true
}

unsafe fn printk_stack_address(address: usize, reliable: i32, log_lvl: *const i8) {
    touch_nmi_watchdog();
    printk(b"%s %s%pBb\0".as_ptr() as *const i8, log_lvl,
           if reliable != 0 { b"\0".as_ptr() } else { b"? \0".as_ptr() }, address as *mut core::ffi::c_void);
}

unsafe fn copy_code(regs: *mut pt_regs, buf: *mut u8, src: usize, nbytes: u32) -> i32 {
    if !user_mode(regs) { return copy_from_kernel_nofault(buf, src as *mut u8, nbytes); }
    /* The user space code from other tasks cannot be accessed. */
    if regs != task_pt_regs(current) { return -EPERM; }
    /* Even if named copy_from_user_nmi() this can be invoked from other contexts. */
    copy_from_user_nmi(buf, src as *mut core::ffi::c_void, nbytes)
}

pub unsafe fn show_opcodes(regs: *mut pt_regs, loglvl: *const i8) {
    const PROLOGUE_SIZE: usize = 42;
    const EPILOGUE_SIZE: usize = 21;
    const OPCODE_BUFSIZE: usize = PROLOGUE_SIZE + 1 + EPILOGUE_SIZE;
    let mut opcodes = [0u8; OPCODE_BUFSIZE];
    let prologue = (*regs).ip.wrapping_sub(PROLOGUE_SIZE);
    match copy_code(regs, opcodes.as_mut_ptr(), prologue, core::mem::size_of_val(&opcodes) as u32) {
        0 => printk(b"%sCode: 42ph <%02x> 21ph\n\0".as_ptr() as *const i8, loglvl,
                    opcodes.as_ptr(), opcodes[PROLOGUE_SIZE], opcodes.as_ptr().add(PROLOGUE_SIZE + 1)),
        -EPERM => (),
        _ => printk(b"%sCode: Unable to access opcode bytes at 0x%lx.\n\0".as_ptr() as *const i8, loglvl, prologue),
    }
}

pub unsafe fn show_ip(regs: *mut pt_regs, loglvl: *const i8) {
    printk(b"%sRIP: %04x:%pS\n\0".as_ptr() as *const i8, loglvl, (*regs).cs as i32, (*regs).ip as *mut core::ffi::c_void);
    show_opcodes(regs, loglvl);
}

pub unsafe fn show_iret_regs(regs: *mut pt_regs, log_lvl: *const i8) {
    show_ip(regs, log_lvl);
    printk(b"%sRSP: %04x:%016lx EFLAGS: %08lx\0".as_ptr() as *const i8, log_lvl, (*regs).ss as i32, (*regs).sp, (*regs).flags);
}

unsafe fn show_regs_if_on_stack(info: *mut stack_info, regs: *mut pt_regs, partial: bool, log_lvl: *const i8) {
    if !partial && on_stack(info, regs as *mut core::ffi::c_void, core::mem::size_of::<pt_regs>()) {
        __show_regs(regs, SHOW_REGS_SHORT, log_lvl);
    } else if partial && on_stack(info, (regs as *mut u8).add(IRET_FRAME_OFFSET) as *mut core::ffi::c_void, IRET_FRAME_SIZE) {
        show_iret_regs(regs, log_lvl);
    }
}

unsafe fn __show_trace_log_lvl(task: *mut task_struct, mut regs: *mut pt_regs, mut stack: *mut usize, log_lvl: *const i8) {
    let mut state = core::mem::MaybeUninit::<unwind_state>::uninit();
    let mut stack_info = core::mem::zeroed::<stack_info>();
    let mut visit_mask = 0usize;
    let mut graph_idx = 0i32;
    let mut partial = false;
    printk(b"%sCall Trace:\n\0".as_ptr() as *const i8, log_lvl);
    unwind_start(state.as_mut_ptr(), task, regs, stack);
    if stack.is_null() { stack = get_stack_pointer(task, regs); }
    regs = unwind_get_entry_regs(state.as_mut_ptr(), &mut partial);
    while !stack.is_null() {
        stack = ((stack as usize + core::mem::size_of::<usize>() - 1) & !(core::mem::size_of::<usize>() - 1)) as *mut usize;
        if get_stack_info(stack, task, &mut stack_info, &mut visit_mask) {
            stack = PAGE_ALIGN(stack as usize) as *mut usize;
            if get_stack_info(stack, task, &mut stack_info, &mut visit_mask) { break; }
        }
        let stack_name = stack_type_name(stack_info.type_);
        if !stack_name.is_null() { printk(b"%s <%s>\n\0".as_ptr() as *const i8, log_lvl, stack_name); }
        if !regs.is_null() { show_regs_if_on_stack(&mut stack_info, regs, partial, log_lvl); }
        while stack < stack_info.end {
            let addr = READ_ONCE_NOCHECK(*stack);
            let ret_addr_p = unwind_get_return_address_ptr(state.as_mut_ptr());
            if !__kernel_text_address(addr) { stack = stack.add(1); continue; }
            if !regs.is_null() && stack == &mut (*regs).ip { stack = stack.add(1); continue; }
            let reliable = if stack == ret_addr_p { 1 } else { 0 };
            let real_addr = ftrace_graph_ret_addr(task, &mut graph_idx, addr, stack);
            if real_addr != addr { printk_stack_address(addr, 0, log_lvl); }
            printk_stack_address(real_addr, reliable, log_lvl);
            if reliable != 0 {
                unwind_next_frame(state.as_mut_ptr());
                regs = unwind_get_entry_regs(state.as_mut_ptr(), &mut partial);
                if !regs.is_null() { show_regs_if_on_stack(&mut stack_info, regs, partial, log_lvl); }
            }
            stack = stack.add(1);
        }
        if !stack_name.is_null() { printk(b"%s </%s>\n\0".as_ptr() as *const i8, log_lvl, stack_name); }
        stack = stack_info.next_sp;
    }
}

unsafe fn show_trace_log_lvl(task: *mut task_struct, regs: *mut pt_regs, stack: *mut usize, log_lvl: *const i8) {
    let disable_kasan = !task.is_null() && task != current;
    if disable_kasan { kasan_disable_current(); }
    __show_trace_log_lvl(task, regs, stack, log_lvl);
    if disable_kasan { kasan_enable_current(); }
}

pub unsafe fn show_stack(mut task: *mut task_struct, mut sp: *mut usize, loglvl: *const i8) {
    if task.is_null() { task = current; }
    if sp.is_null() && task == current { sp = get_stack_pointer(current, core::ptr::null_mut()); }
    show_trace_log_lvl(task, core::ptr::null_mut(), sp, loglvl);
}

pub unsafe fn show_stack_regs(regs: *mut pt_regs) { show_trace_log_lvl(current, regs, core::ptr::null_mut(), KERN_DEFAULT); }

static mut die_lock: arch_spinlock_t = __ARCH_SPIN_LOCK_UNLOCKED;
static mut die_owner: i32 = -1;
static mut die_nest_count: u32 = 0;

pub unsafe fn oops_begin() -> usize {
    let mut flags = 0usize;
    oops_enter(); raw_local_irq_save(&mut flags);
    let cpu = smp_processor_id();
    if !arch_spin_trylock(&mut die_lock) {
        if cpu != die_owner { arch_spin_lock(&mut die_lock); }
    }
    die_nest_count += 1; die_owner = cpu; console_verbose(); bust_spinlocks(1); flags
}

pub unsafe fn oops_end(flags: usize, regs: *mut pt_regs, signr: i32) {
    if !regs.is_null() && kexec_should_crash(current) { crash_kexec(regs); }
    bust_spinlocks(0); die_owner = -1; add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE); die_nest_count -= 1;
    if die_nest_count == 0 { arch_spin_unlock(&mut die_lock); }
    raw_local_irq_restore(flags); oops_exit();
    __show_regs(&mut exec_summary_regs, SHOW_REGS_ALL, KERN_DEFAULT);
    if signr == 0 { return; }
    if in_interrupt() { panic(b"Fatal exception in interrupt\0".as_ptr() as *const i8); }
    if panic_on_oops { panic(b"Fatal exception\0".as_ptr() as *const i8); }
    kasan_unpoison_task_stack(current); rewind_stack_and_make_dead(signr);
}

unsafe fn __die_header(str_: *const i8, regs: *mut pt_regs, err: isize) {
    if die_counter == 0 { exec_summary_regs = *regs; }
    die_counter += 1;
    printk(b"Oops: %s: %04lx [#%d]\n\0".as_ptr() as *const i8, str_, (err as usize) & 0xffff, die_counter);
}
unsafe fn __die_body(str_: *const i8, regs: *mut pt_regs, err: isize) -> i32 {
    show_regs(regs); print_modules();
    if notify_die(DIE_OOPS, str_, regs, err, (*current).thread.trap_nr, SIGSEGV) == NOTIFY_STOP { return 1; }
    0
}
pub unsafe fn __die(str_: *const i8, regs: *mut pt_regs, err: isize) -> i32 { __die_header(str_, regs, err); __die_body(str_, regs, err) }
pub unsafe fn die(str_: *const i8, regs: *mut pt_regs, err: isize) { let flags = oops_begin(); let sig = if __die(str_, regs, err) != 0 { 0 } else { SIGSEGV }; oops_end(flags, regs, sig); }
pub unsafe fn die_addr(str_: *const i8, regs: *mut pt_regs, err: isize, gp_addr: isize) { let flags = oops_begin(); __die_header(str_, regs, err); if gp_addr != 0 { kasan_non_canonical_hook(gp_addr); } let sig = if __die_body(str_, regs, err) != 0 { 0 } else { SIGSEGV }; oops_end(flags, regs, sig); }

pub unsafe fn show_regs(regs: *mut pt_regs) {
    show_regs_print_info(KERN_DEFAULT);
    let mode = if user_mode(regs) { SHOW_REGS_USER } else { SHOW_REGS_ALL };
    __show_regs(regs, mode, KERN_DEFAULT);
    if !user_mode(regs) { show_trace_log_lvl(current, regs, core::ptr::null_mut(), KERN_DEFAULT); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
