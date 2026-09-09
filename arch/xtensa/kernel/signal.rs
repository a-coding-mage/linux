/* Translated from arch/xtensa/kernel/signal.c. */

extern "C" {
    static mut coproc_owners: [*mut task_struct; 0];
}

#[repr(C)]
pub struct rt_sigframe {
    pub info: siginfo,
    pub uc: ucontext,
    pub xtregs: Xtregs,
    pub retcode: [u8; 6],
    pub window: [u32; 4],
}

#[repr(C)]
pub struct Xtregs {
    pub opt: xtregs_opt_t,
    pub user: xtregs_user_t,
    /* Present when XTENSA_HAVE_COPROCESSORS is enabled. */
    pub cp: xtregs_coprocessor_t,
}

#[cfg(any())]
const _XTENSA_HAVE_COPROCESSORS: bool = true;

#[cfg(any())]
unsafe fn flush_window_regs_user(regs: *mut pt_regs) -> i32 {
    let ws = (*regs).windowstart;
    let wb = (*regs).windowbase;
    let mut sp: c_ulong = 0;
    let wm: c_ulong;
    let mut err: i32 = 1;
    let mut base: i32;

    if (*regs).wmask == 1 { return 0; }
    wm = (ws >> wb) | (ws << (XCHAL_NUM_AREGS / 4 - wb));
    base = (XCHAL_NUM_AREGS / 4) - ((*regs).wmask >> 4) as i32;

    if ((*regs).wmask & 2) == 0 {
        if __get_user(&mut sp, ((*regs).areg.as_ptr().add((base * 4 + 1) as usize) as *const i32).offset(-3)) != 0 {
            return err;
        }
    }

    while base < XCHAL_NUM_AREGS / 4 {
        let m = wm >> base;
        let mut inc = 0;
        if (m & 2) != 0 {
            inc = 1;
        } else if (m & 4) != 0 {
            if copy_to_user(SPILL_SLOT_CALL8(sp, 4), (*regs).areg.as_ptr().add(((base + 1) * 4) as usize), 16) != 0 { return err; }
            inc = 2;
        } else if (m & 8) != 0 {
            if copy_to_user(SPILL_SLOT_CALL12(sp, 4), (*regs).areg.as_ptr().add(((base + 1) * 4) as usize), 32) != 0 { return err; }
            inc = 3;
        }
        sp = (*regs).areg[((base + inc) * 4 + 1) as usize % XCHAL_NUM_AREGS as usize];
        if copy_to_user(SPILL_SLOT(sp, 0), (*regs).areg.as_ptr().add((base * 4) as usize), 16) != 0 { return err; }
        sp = (*regs).areg[(base * 4 + 1) as usize];
        base += inc;
    }
    (*regs).wmask = 1;
    (*regs).windowstart = 1 << wb;
    0
}

#[cfg(not(any()))]
unsafe fn flush_window_regs_user(_regs: *mut pt_regs) -> i32 { 0 }

unsafe fn setup_sigcontext(frame: *mut rt_sigframe, regs: *mut pt_regs) -> i32 {
    let sc = &mut (*frame).uc.uc_mcontext as *mut sigcontext;
    let ti = current_thread_info();
    let mut err = 0;
    err |= __put_user((*regs).pc, &mut (*sc).sc_pc);
    err |= __put_user((*regs).ps, &mut (*sc).sc_ps);
    err |= __put_user((*regs).lbeg, &mut (*sc).sc_lbeg);
    err |= __put_user((*regs).lend, &mut (*sc).sc_lend);
    err |= __put_user((*regs).lcount, &mut (*sc).sc_lcount);
    err |= __put_user((*regs).sar, &mut (*sc).sc_sar);
    err |= flush_window_regs_user(regs);
    err |= __copy_to_user((*sc).sc_a.as_mut_ptr(), (*regs).areg.as_ptr(), 16 * 4);
    err |= __put_user(0, &mut (*sc).sc_xtregs);
    if err != 0 { return err; }
    /* XTENSA_HAVE_COPROCESSORS: flush and copy coprocessor state. */
    err |= __copy_to_user(&mut (*frame).xtregs.opt, &(*regs).xtregs_opt, core::mem::size_of::<xtregs_opt_t>());
    err |= __copy_to_user(&mut (*frame).xtregs.user, &(*ti).xtregs_user, core::mem::size_of::<xtregs_user_t>());
    err |= __put_user(if err != 0 { core::ptr::null_mut() } else { &mut (*frame).xtregs }, &mut (*sc).sc_xtregs);
    err
}

unsafe fn restore_sigcontext(regs: *mut pt_regs, frame: *mut rt_sigframe) -> u32 {
    let sc = &mut (*frame).uc.uc_mcontext as *mut sigcontext;
    let ti = current_thread_info();
    let mut err = 0u32;
    err |= __get_user(&mut (*regs).pc, &(*sc).sc_pc);
    err |= __get_user(&mut (*regs).lbeg, &(*sc).sc_lbeg);
    err |= __get_user(&mut (*regs).lend, &(*sc).sc_lend);
    err |= __get_user(&mut (*regs).lcount, &(*sc).sc_lcount);
    err |= __get_user(&mut (*regs).sar, &(*sc).sc_sar);
    (*regs).wmask = 1; (*regs).windowbase = 0; (*regs).windowstart = 1;
    (*regs).syscall = NO_SYSCALL;
    let mut ps = 0;
    err |= __get_user(&mut ps, &(*sc).sc_ps);
    (*regs).ps = ((*regs).ps & !PS_CALLINC_MASK) | (ps & PS_CALLINC_MASK);
    if (*regs).lcount > 0 && ((*regs).lbeg > TASK_SIZE || (*regs).lend > TASK_SIZE) { err = 1; }
    err |= __copy_from_user((*regs).areg.as_mut_ptr(), (*sc).sc_a.as_ptr(), 16 * 4);
    if err != 0 { return err; }
    /* XTENSA_HAVE_COPROCESSORS: release and restore coprocessor state. */
    err |= __copy_from_user(&mut (*ti).xtregs_user, &(*frame).xtregs.user, core::mem::size_of::<xtregs_user_t>());
    err |= __copy_from_user(&mut (*regs).xtregs_opt, &(*frame).xtregs.opt, core::mem::size_of::<xtregs_opt_t>());
    err
}

pub unsafe extern "C" fn xtensa_rt_sigreturn() -> c_long {
    let regs = current_pt_regs();
    current.restart_block.fn = do_no_restart_syscall;
    if (*regs).depc > 64 { panic!("rt_sigreturn in double exception!\n"); }
    let frame = (*regs).areg[1] as *mut rt_sigframe;
    if !access_ok(frame, core::mem::size_of::<rt_sigframe>()) { force_sig(SIGSEGV); return 0; }
    let mut set = core::mem::MaybeUninit::<sigset_t>::uninit();
    if __copy_from_user(set.as_mut_ptr(), &(*frame).uc.uc_sigmask, core::mem::size_of::<sigset_t>()) != 0 { force_sig(SIGSEGV); return 0; }
    set_current_blocked(set.assume_init_ref());
    if restore_sigcontext(regs, frame) != 0 || restore_altstack(&(*frame).uc.uc_stack) != 0 { force_sig(SIGSEGV); return 0; }
    (*regs).areg[2] as c_long
}

unsafe fn gen_return_code(codemem: *mut u8) -> i32 {
    let mut err = 0;
    /* Endianness is a build-time Xtensa condition; preserve both encodings. */
    #[cfg(target_endian = "big")]
    { err |= __put_user(0x22, codemem); err |= __put_user(0x0a, codemem.add(1)); err |= __put_user(__NR_rt_sigreturn, codemem.add(2)); err |= __put_user(0, codemem.add(3)); err |= __put_user(5, codemem.add(4)); err |= __put_user(0, codemem.add(5)); }
    #[cfg(target_endian = "little")]
    { err |= __put_user(0x22, codemem); err |= __put_user(0xa0, codemem.add(1)); err |= __put_user(__NR_rt_sigreturn, codemem.add(2)); err |= __put_user(0, codemem.add(3)); err |= __put_user(0x50, codemem.add(4)); err |= __put_user(0, codemem.add(5)); }
    if err == 0 { __invalidate_icache_range(codemem as c_ulong, 6); __flush_invalidate_dcache_range(codemem as c_ulong, 6); }
    err
}

unsafe fn setup_frame(ksig: *mut ksignal, set: *mut sigset_t, regs: *mut pt_regs) -> i32 {
    let mut frame = (((*regs).areg[1] - core::mem::size_of::<rt_sigframe>() as c_ulong) & !15) as *mut rt_sigframe;
    let mut err = 0;
    let sig = (*ksig).sig;
    let mut handler = (*ksig).ka.sa.sa_handler as c_ulong;
    if (*regs).depc > 64 { panic!("Double exception sys_sigreturn\n"); }
    if !access_ok(frame, core::mem::size_of::<rt_sigframe>()) { return -EFAULT; }
    if ((*ksig).ka.sa.sa_flags & SA_SIGINFO) != 0 { err |= copy_siginfo_to_user(&mut (*frame).info, &(*ksig).info); }
    err |= __put_user(0, &mut (*frame).uc.uc_flags);
    err |= __put_user(0, &mut (*frame).uc.uc_link);
    err |= __save_altstack(&mut (*frame).uc.uc_stack, (*regs).areg[1]);
    err |= setup_sigcontext(frame, regs);
    err |= __copy_to_user(&mut (*frame).uc.uc_sigmask, set, core::mem::size_of::<sigset_t>());
    let ra = if ((*ksig).ka.sa.sa_flags & SA_RESTORER) != 0 { (*ksig).ka.sa.sa_restorer as c_ulong } else { err |= gen_return_code((*frame).retcode.as_mut_ptr()); (*frame).retcode.as_mut_ptr() as c_ulong };
    if err != 0 { return -EFAULT; }
    let tp = (*regs).threadptr; let ps0 = (*regs).ps;
    start_thread(regs, handler, frame as c_ulong);
    let base;
    if (ps0 & PS_WOE_MASK) != 0 { base = 4; (*regs).areg[base] = (ra & 0x3fffffff) | 0x40000000; (*regs).ps = (ps0 & !(PS_CALLINC_MASK | PS_OWB_MASK)) | (1 << PS_CALLINC_SHIFT); } else { base = 0; (*regs).areg[base] = ra; }
    (*regs).areg[base + 2] = sig as c_ulong;
    (*regs).areg[base + 3] = &mut (*frame).info as *mut _ as c_ulong;
    (*regs).areg[base + 4] = &mut (*frame).uc as *mut _ as c_ulong;
    (*regs).threadptr = tp;
    pr_debug!("SIG rt deliver ({}:{}): signal={} sp={:?} pc={:08x}\n", current.comm, current.pid, sig, frame, (*regs).pc);
    0
}

unsafe fn do_signal(regs: *mut pt_regs) {
    let mut ksig = core::mem::MaybeUninit::<ksignal>::uninit();
    (*task_pt_regs(current)).icountlevel = 0;
    if get_signal(ksig.as_mut_ptr()) {
        let ksig = ksig.assume_init_mut();
        if (*regs).syscall != NO_SYSCALL {
            match (*regs).areg[2] as c_long {
                -ERESTARTNOHAND | -ERESTART_RESTARTBLOCK => (*regs).areg[2] = -EINTR as c_ulong,
                -ERESTARTSYS if ((*ksig).ka.sa.sa_flags & SA_RESTART) == 0 => (*regs).areg[2] = -EINTR as c_ulong,
                -ERESTARTSYS | -ERESTARTNOINTR => { (*regs).areg[2] = (*regs).syscall; (*regs).pc -= 3; },
                _ => (),
            }
        }
        let ret = setup_frame(ksig, sigmask_to_save(), regs);
        signal_setup_done(ret, ksig, 0);
        if test_thread_flag(TIF_SINGLESTEP) { (*task_pt_regs(current)).icountlevel = 1; }
        return;
    }
    if (*regs).syscall != NO_SYSCALL { match (*regs).areg[2] as c_long { -ERESTARTNOHAND | -ERESTARTSYS | -ERESTARTNOINTR => { (*regs).areg[2] = (*regs).syscall; (*regs).pc -= 3; }, -ERESTART_RESTARTBLOCK => { (*regs).areg[2] = __NR_restart_syscall; (*regs).pc -= 3; }, _ => () } }
    restore_saved_sigmask();
    if test_thread_flag(TIF_SINGLESTEP) { (*task_pt_regs(current)).icountlevel = 1; }
}

pub unsafe fn do_notify_resume(regs: *mut pt_regs) {
    if test_thread_flag(TIF_SIGPENDING) || test_thread_flag(TIF_NOTIFY_SIGNAL) { do_signal(regs); }
    if test_thread_flag(TIF_NOTIFY_RESUME) { resume_user_mode_work(regs); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
