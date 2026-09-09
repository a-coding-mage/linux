/* Rust translation of arch/xtensa/kernel/traps.c. C headers and build-time
 * configuration symbols are supplied by the surrounding kernel environment. */

use core::ffi::c_void;

const KRNL: i32 = 0x01;
const USER: i32 = 0x02;

#[repr(C)]
pub struct DispatchInitTable {
    pub cause: i32,
    pub fast: i32,
    pub handler: *mut c_void,
}

/* Configuration-dependent entries from the C dispatch table are retained
 * below as conditional Rust items in the same order. */
static mut DISPATCH_INIT_TABLE: &[DispatchInitTable] = &[
    #[cfg(CONFIG_USER_ABI_CALL0_PROBE)]
    DispatchInitTable { cause: EXCCAUSE_ILLEGAL_INSTRUCTION, fast: USER, handler: fast_illegal_instruction_user as *mut c_void },
    DispatchInitTable { cause: EXCCAUSE_ILLEGAL_INSTRUCTION, fast: 0, handler: do_illegal_instruction as *mut c_void },
    DispatchInitTable { cause: EXCCAUSE_SYSTEM_CALL, fast: USER, handler: fast_syscall_user as *mut c_void },
    DispatchInitTable { cause: EXCCAUSE_SYSTEM_CALL, fast: 0, handler: system_call as *mut c_void },
    #[cfg(CONFIG_XTENSA_LOAD_STORE)]
    DispatchInitTable { cause: EXCCAUSE_LOAD_STORE_ERROR, fast: USER | KRNL, handler: fast_load_store as *mut c_void },
    #[cfg(CONFIG_XTENSA_LOAD_STORE)]
    DispatchInitTable { cause: EXCCAUSE_LOAD_STORE_ERROR, fast: 0, handler: do_load_store as *mut c_void },
    DispatchInitTable { cause: EXCCAUSE_LEVEL1_INTERRUPT, fast: 0, handler: do_interrupt as *mut c_void },
    #[cfg(SUPPORT_WINDOWED)]
    DispatchInitTable { cause: EXCCAUSE_ALLOCA, fast: USER | KRNL, handler: fast_alloca as *mut c_void },
    DispatchInitTable { cause: EXCCAUSE_INTEGER_DIVIDE_BY_ZERO, fast: 0, handler: do_div0 as *mut c_void },
    DispatchInitTable { cause: EXCCAUSE_UNALIGNED, fast: 0, handler: do_unaligned_user as *mut c_void },
    #[cfg(CONFIG_MMU)]
    DispatchInitTable { cause: EXCCAUSE_ITLB_MISS, fast: 0, handler: do_page_fault as *mut c_void },
    #[cfg(CONFIG_MMU)]
    DispatchInitTable { cause: EXCCAUSE_ITLB_MISS, fast: USER | KRNL, handler: fast_second_level_miss as *mut c_void },
    #[cfg(CONFIG_MMU)]
    DispatchInitTable { cause: EXCCAUSE_DTLB_MISS, fast: USER | KRNL, handler: fast_second_level_miss as *mut c_void },
    #[cfg(CONFIG_MMU)]
    DispatchInitTable { cause: EXCCAUSE_DTLB_MISS, fast: 0, handler: do_page_fault as *mut c_void },
    #[cfg(CONFIG_MMU)]
    DispatchInitTable { cause: EXCCAUSE_STORE_CACHE_ATTRIBUTE, fast: USER | KRNL, handler: fast_store_prohibited as *mut c_void },
    DispatchInitTable { cause: EXCCAUSE_MAPPED_DEBUG, fast: 0, handler: do_debug as *mut c_void },
    DispatchInitTable { cause: -1, fast: -1, handler: core::ptr::null_mut() },
];

#[inline]
unsafe fn __die_if_kernel(str_: *const i8, regs: *mut pt_regs, err: i64) {
    if !user_mode(regs) { die(str_, regs, err); }
}

#[inline]
unsafe fn dump_user_code(_regs: *mut pt_regs) {}

pub unsafe fn do_unhandled(regs: *mut pt_regs) {
    __die_if_kernel(b"Caught unhandled exception - should not happen\0".as_ptr() as _, regs, SIGKILL as _);
    pr_info_ratelimited();
    dump_user_code(regs);
    force_sig(SIGILL);
}

unsafe fn do_multihit(regs: *mut pt_regs) {
    die(b"Caught multihit exception\0".as_ptr() as _, regs, SIGKILL as _);
}

#[cfg(XTENSA_FAKE_NMI)]
unsafe fn do_nmi(regs: *mut pt_regs) {
    let old_regs = set_irq_regs(regs);
    nmi_enter();
    *this_cpu_ptr(&mut nmi_count) += 1;
    check_valid_nmi();
    xtensa_pmu_irq_handler(0, core::ptr::null_mut());
    nmi_exit();
    set_irq_regs(old_regs);
}

unsafe fn do_interrupt(regs: *mut pt_regs) {
    let int_level_mask: [u32; 8] = [0, XCHAL_INTLEVEL1_MASK, XCHAL_INTLEVEL2_MASK,
        XCHAL_INTLEVEL3_MASK, XCHAL_INTLEVEL4_MASK, XCHAL_INTLEVEL5_MASK,
        XCHAL_INTLEVEL6_MASK, XCHAL_INTLEVEL7_MASK];
    let old_regs = set_irq_regs(regs);
    let mut unhandled = !0u32;
    irq_enter();
    loop {
        let intread = xtensa_get_sr(interrupt);
        let intenable = xtensa_get_sr(intenable);
        let mut int_at_level = intread & intenable;
        let mut level = LOCKLEVEL;
        while level > 0 {
            if int_at_level & int_level_mask[level as usize] != 0 {
                int_at_level &= int_level_mask[level as usize];
                if int_at_level & unhandled != 0 { int_at_level &= unhandled; }
                else { unhandled |= int_level_mask[level as usize]; }
                break;
            }
            level -= 1;
        }
        if level == 0 { break; }
        unhandled ^= int_at_level & int_at_level.wrapping_neg();
        do_IRQ(__ffs(int_at_level), regs);
    }
    irq_exit();
    set_irq_regs(old_regs);
}

unsafe fn check_div0(regs: *mut pt_regs) -> bool {
    let pattern = *b"DIV0";
    let mut buf = [0u8; 5];
    let p: *const u8;
    if user_mode(regs) {
        if copy_from_user(buf.as_mut_ptr() as _, ((*regs).pc + 2) as _, 5) != 0 { return false; }
        p = buf.as_ptr();
    } else { p = ((*regs).pc + 2) as *const u8; }
    core::slice::from_raw_parts(p, 4) == &pattern || core::slice::from_raw_parts(p.add(1), 4) == &pattern
}

unsafe fn do_illegal_instruction(regs: *mut pt_regs) {
    if check_div0(regs) { do_div0(regs); return; }
    __die_if_kernel(b"Illegal instruction in kernel\0".as_ptr() as _, regs, SIGKILL as _);
    force_sig(SIGILL);
}
unsafe fn do_div0(regs: *mut pt_regs) {
    __die_if_kernel(b"Unhandled division by 0 in kernel\0".as_ptr() as _, regs, SIGKILL as _);
    force_sig_fault(SIGFPE, FPE_INTDIV, (*regs).pc as _);
}
#[cfg(CONFIG_XTENSA_LOAD_STORE)]
unsafe fn do_load_store(regs: *mut pt_regs) {
    __die_if_kernel(b"Unhandled load/store exception in kernel\0".as_ptr() as _, regs, SIGKILL as _);
    force_sig_fault(SIGBUS, BUS_ADRERR, (*regs).excvaddr as _);
}
unsafe fn do_unaligned_user(regs: *mut pt_regs) {
    __die_if_kernel(b"Unhandled unaligned exception in kernel\0".as_ptr() as _, regs, SIGKILL as _);
    force_sig_fault(SIGBUS, BUS_ADRALN, (*regs).excvaddr as _);
}
#[cfg(XTENSA_HAVE_COPROCESSORS)]
unsafe fn do_coprocessor(_regs: *mut pt_regs) { coprocessor_flush_release_all(current_thread_info()); }
unsafe fn do_debug(regs: *mut pt_regs) {
    #[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
    { let ret = check_hw_breakpoint(regs); preempt_enable(); if ret == 0 { return; } }
    __die_if_kernel(b"Breakpoint in kernel\0".as_ptr() as _, regs, SIGKILL as _);
    force_sig(SIGTRAP);
}

pub unsafe fn trap_set_handler(cause: i32, handler: *mut c_void) -> *mut c_void {
    let previous = per_cpu(exc_table, 0).default_handler[cause as usize];
    for_each_possible_cpu(|cpu| { per_cpu(exc_table, cpu).default_handler[cause as usize] = handler; });
    previous
}
unsafe fn trap_init_excsave() { xtensa_set_sr(this_cpu_ptr(&exc_table), excsave1); }
unsafe fn trap_init_debug() {
    this_cpu_ptr(&mut debug_table).debug_exception = debug_exception;
    asm!("wsr {0}, excsave" , in(reg) this_cpu_ptr(&debug_table) as usize);
}
pub unsafe fn trap_init() {
    for i in 0..EXCCAUSE_N { set_handler(fast_user_handler, i, user_exception); set_handler(fast_kernel_handler, i, kernel_exception); set_handler(default_handler, i, do_unhandled as _); }
    let mut i = 0;
    while DISPATCH_INIT_TABLE[i].cause >= 0 {
        let e = &DISPATCH_INIT_TABLE[i];
        if e.fast == 0 { set_handler(default_handler, e.cause, e.handler); }
        if e.fast & USER != 0 { set_handler(fast_user_handler, e.cause, e.handler); }
        if e.fast & KRNL != 0 { set_handler(fast_kernel_handler, e.cause, e.handler); }
        i += 1;
    }
    trap_init_excsave(); trap_init_debug();
}
#[cfg(CONFIG_SMP)]
pub unsafe fn secondary_trap_init() { trap_init_excsave(); trap_init_debug(); }

pub unsafe fn show_regs(regs: *mut pt_regs) {
    show_regs_print_info(KERN_DEFAULT);
    for i in 0..16 { if i % 8 == 0 { pr_info(); } pr_cont(); }
    pr_cont(); pr_info();
    if user_mode(regs) { pr_cont(); }
}

unsafe fn show_trace_cb(frame: *mut stackframe, data: *mut c_void) -> i32 {
    if kernel_text_address((*frame).pc) { printk(data); }
    0
}
unsafe fn show_trace(task: *mut task_struct, mut sp: *mut usize, loglvl: *const i8) {
    if sp.is_null() { sp = stack_pointer(task); }
    printk(loglvl); walk_stackframe(sp, show_trace_cb, loglvl as _);
}

const STACK_DUMP_ENTRY_SIZE: usize = 4;
const STACK_DUMP_LINE_SIZE: usize = 16;
static mut KSTACK_DEPTH_TO_PRINT: usize = CONFIG_PRINT_STACK_DEPTH;
#[repr(C)] struct stack_fragment { len: usize, off: usize, sp: *mut u8, loglvl: *const i8 }
unsafe fn show_stack_fragment_cb(frame: *mut stackframe, data: *mut c_void) -> i32 {
    let sf = &mut *(data as *mut stack_fragment);
    while sf.off < sf.len { let line_len = (sf.len - sf.off).min(STACK_DUMP_LINE_SIZE); let arrow = sf.off == 0 || (!frame.is_null() && (*frame).sp == (sf.sp.add(sf.off) as usize)); let mut line = [0u8; STACK_DUMP_LINE_SIZE]; __memcpy(line.as_mut_ptr(), sf.sp.add(sf.off), line_len); print_hex_dump(sf.loglvl, arrow, line.as_ptr(), line_len); sf.off += STACK_DUMP_LINE_SIZE; if arrow { return 0; } }
    1
}
pub unsafe fn show_stack(task: *mut task_struct, mut sp: *mut usize, loglvl: *const i8) {
    if sp.is_null() { sp = stack_pointer(task); }
    let mut sf = stack_fragment { len: ((-(sp as isize) as usize) & (THREAD_SIZE - STACK_DUMP_ENTRY_SIZE)).min(KSTACK_DEPTH_TO_PRINT * STACK_DUMP_ENTRY_SIZE), off: 0, sp: sp as _, loglvl };
    printk(loglvl); walk_stackframe(sp, show_stack_fragment_cb, &mut sf as *mut _ as _); while sf.off < sf.len { show_stack_fragment_cb(core::ptr::null_mut(), &mut sf as *mut _ as _); } show_trace(task, sp, loglvl);
}
pub unsafe fn die(str_: *const i8, regs: *mut pt_regs, err: i64) -> ! {
    static mut DIE_COUNTER: i32 = 0;
    console_verbose(); spin_lock_irq(&die_lock); DIE_COUNTER += 1; pr_info(); show_regs(regs); if !user_mode(regs) { show_stack(core::ptr::null_mut(), (*regs).areg[1] as _, KERN_INFO); } add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE); spin_unlock_irq(&die_lock); if in_interrupt() { panic(); } if panic_on_oops { panic(); } make_task_dead(err);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
