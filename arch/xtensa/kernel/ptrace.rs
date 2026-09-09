/* Translated from xtensa/kernel/ptrace.c. */

unsafe fn gpr_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let regs = task_pt_regs(target);
    let mut newregs: user_pt_regs = core::mem::zeroed();
    newregs.pc = (*regs).pc;
    newregs.ps = (*regs).ps & !(1u32 << PS_EXCM_BIT);
    newregs.lbeg = (*regs).lbeg; newregs.lend = (*regs).lend;
    newregs.lcount = (*regs).lcount; newregs.sar = (*regs).sar;
    newregs.threadptr = (*regs).threadptr; newregs.windowbase = (*regs).windowbase;
    newregs.windowstart = (*regs).windowstart; newregs.syscall = (*regs).syscall;
    core::ptr::copy_nonoverlapping((*regs).areg.add(XCHAL_NUM_AREGS - (*regs).windowbase as usize * 4), newregs.a.as_mut_ptr(), (*regs).windowbase as usize * 16);
    core::ptr::copy_nonoverlapping((*regs).areg, newregs.a.as_mut_ptr().add((*regs).windowbase as usize * 4), (WSBITS - (*regs).windowbase as usize) * 16);
    membuf_write(&mut to, &newregs as *const _ as *const _, core::mem::size_of::<user_pt_regs>())
}

unsafe fn gpr_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let mut newregs: user_pt_regs = core::mem::zeroed();
    let ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, &mut newregs as *mut _ as *mut _, 0, -1i32 as usize);
    if ret != 0 { return ret; }
    if newregs.windowbase >= (XCHAL_NUM_AREGS / 4) as _ { return -EINVAL; }
    let regs = task_pt_regs(target);
    let ps_mask: u32 = PS_CALLINC_MASK | PS_OWB_MASK;
    (*regs).pc = newregs.pc; (*regs).ps = ((*regs).ps & !ps_mask) | (newregs.ps & ps_mask);
    (*regs).lbeg = newregs.lbeg; (*regs).lend = newregs.lend; (*regs).lcount = newregs.lcount;
    (*regs).sar = newregs.sar; (*regs).threadptr = newregs.threadptr;
    if newregs.syscall != 0 { (*regs).syscall = newregs.syscall; }
    if newregs.windowbase != (*regs).windowbase || newregs.windowstart != (*regs).windowstart {
        let rotws = (((newregs.windowstart | (newregs.windowstart << WSBITS)) >> newregs.windowbase) & ((1 << WSBITS) - 1)) & !1;
        let wmask = ((if rotws != 0 { WSBITS + 1 - ffs(rotws) } else { 0 }) << 4) | (rotws & 0xF) | 1;
        (*regs).windowbase = newregs.windowbase; (*regs).windowstart = newregs.windowstart; (*regs).wmask = wmask;
    }
    core::ptr::copy_nonoverlapping(newregs.a.as_ptr(), (*regs).areg.add(XCHAL_NUM_AREGS - newregs.windowbase as usize * 4), newregs.windowbase as usize * 4);
    core::ptr::copy_nonoverlapping(newregs.a.as_ptr().add(newregs.windowbase as usize * 4), (*regs).areg, (WSBITS - newregs.windowbase as usize) * 4);
    0
}

unsafe fn tie_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let regs = task_pt_regs(target); let ti = task_thread_info(target);
    let newregs = kzalloc_obj::<elf_xtregs_t>(); if newregs.is_null() { return -ENOMEM; }
    (*newregs).opt = (*regs).xtregs_opt; (*newregs).user = (*ti).xtregs_user;
    #[cfg(XTENSA_HAVE_COPROCESSORS)] { coprocessor_flush_all(ti); (*newregs).cp0 = (*ti).xtregs_cp.cp0; (*newregs).cp1 = (*ti).xtregs_cp.cp1; (*newregs).cp2 = (*ti).xtregs_cp.cp2; (*newregs).cp3 = (*ti).xtregs_cp.cp3; (*newregs).cp4 = (*ti).xtregs_cp.cp4; (*newregs).cp5 = (*ti).xtregs_cp.cp5; (*newregs).cp6 = (*ti).xtregs_cp.cp6; (*newregs).cp7 = (*ti).xtregs_cp.cp7; }
    let ret = membuf_write(&mut to, newregs as *const _, core::mem::size_of::<elf_xtregs_t>()); kfree(newregs); ret
}

unsafe fn tie_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let regs = task_pt_regs(target); let ti = task_thread_info(target); let newregs = kzalloc_obj::<elf_xtregs_t>(); if newregs.is_null() { return -ENOMEM; }
    let ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, newregs as *mut _, 0, -1i32 as usize);
    if ret == 0 { (*regs).xtregs_opt = (*newregs).opt; (*ti).xtregs_user = (*newregs).user;
        #[cfg(XTENSA_HAVE_COPROCESSORS)] { coprocessor_flush_release_all(ti); (*ti).xtregs_cp.cp0 = (*newregs).cp0; (*ti).xtregs_cp.cp1 = (*newregs).cp1; (*ti).xtregs_cp.cp2 = (*newregs).cp2; (*ti).xtregs_cp.cp3 = (*newregs).cp3; (*ti).xtregs_cp.cp4 = (*newregs).cp4; (*ti).xtregs_cp.cp5 = (*newregs).cp5; (*ti).xtregs_cp.cp6 = (*newregs).cp6; (*ti).xtregs_cp.cp7 = (*newregs).cp7; }
    } kfree(newregs); ret
}

#[repr(C)] enum xtensa_regset { REGSET_GPR, REGSET_TIE }
static xtensa_regsets: [user_regset; 2] = [
    user_regset { core::default::Default::default() }, user_regset { core::default::Default::default() }
];
static user_xtensa_view: user_regset_view = user_regset_view { name: "xtensa", e_machine: EM_XTENSA, regsets: xtensa_regsets.as_ptr(), n: 2 };

#[no_mangle] pub unsafe extern "C" fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view { &user_xtensa_view }
#[no_mangle] pub unsafe extern "C" fn user_enable_single_step(child: *mut task_struct) { set_tsk_thread_flag(child, TIF_SINGLESTEP); }
#[no_mangle] pub unsafe extern "C" fn user_disable_single_step(child: *mut task_struct) { clear_tsk_thread_flag(child, TIF_SINGLESTEP); }
#[no_mangle] pub unsafe extern "C" fn ptrace_disable(_child: *mut task_struct) { }

unsafe fn ptrace_getregs(c: *mut task_struct, p: *mut core::ffi::c_void) -> i32 { copy_regset_to_user(c, &user_xtensa_view, REGSET_GPR as _, 0, core::mem::size_of::<xtensa_gregset_t>(), p) }
unsafe fn ptrace_setregs(c: *mut task_struct, p: *mut core::ffi::c_void) -> i32 { copy_regset_from_user(c, &user_xtensa_view, REGSET_GPR as _, 0, core::mem::size_of::<xtensa_gregset_t>(), p) }
unsafe fn ptrace_getxregs(c: *mut task_struct, p: *mut core::ffi::c_void) -> i32 { copy_regset_to_user(c, &user_xtensa_view, REGSET_TIE as _, 0, core::mem::size_of::<elf_xtregs_t>(), p) }
unsafe fn ptrace_setxregs(c: *mut task_struct, p: *mut core::ffi::c_void) -> i32 { copy_regset_from_user(c, &user_xtensa_view, REGSET_TIE as _, 0, core::mem::size_of::<elf_xtregs_t>(), p) }

unsafe fn ptrace_peekusr(child: *mut task_struct, regno: i64, ret: *mut i64) -> i32 {
    let r = task_pt_regs(child); let mut tmp: usize = 0;
    if regno >= REG_AR_BASE && regno < REG_AR_BASE + XCHAL_NUM_AREGS as i64 { tmp = (*r).areg[(regno - REG_AR_BASE) as usize]; }
    else if regno >= REG_A_BASE && regno <= REG_A_BASE + 15 { tmp = (*r).areg[(regno - REG_A_BASE) as usize]; }
    else { match regno { REG_PC => tmp = (*r).pc, REG_PS => tmp = (*r).ps & !(1 << PS_EXCM_BIT), REG_WB => {}, REG_WS => { let wb=(*r).windowbase; let ws=(*r).windowstart; tmp=((ws>>wb)|(ws<<(WSBITS-wb)))&((1<<WSBITS)-1); }, REG_LBEG=>tmp=(*r).lbeg, REG_LEND=>tmp=(*r).lend, REG_LCOUNT=>tmp=(*r).lcount, REG_SAR=>tmp=(*r).sar, SYSCALL_NR=>tmp=(*r).syscall, _=>return -EIO } }
    put_user(tmp, ret)
}
unsafe fn ptrace_pokeusr(child: *mut task_struct, regno: i64, val: i64) -> i32 { let r=task_pt_regs(child); if regno>=REG_AR_BASE && regno<REG_AR_BASE+XCHAL_NUM_AREGS as i64 { (*r).areg[(regno-REG_AR_BASE) as usize]=val as _; } else if regno>=REG_A_BASE && regno<=REG_A_BASE+15 { (*r).areg[(regno-REG_A_BASE) as usize]=val as _; } else { match regno { REG_PC=>(*r).pc=val as _, SYSCALL_NR=>(*r).syscall=val as _, _=>return -EIO } } 0 }

#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
unsafe fn ptrace_hbptriggered(bp:*mut perf_event, _data:*mut perf_sample_data, _regs:*mut pt_regs) { let bkpt=counter_arch_bp(bp); let mut i: i32; if (*bp).attr.bp_type & HW_BREAKPOINT_X != 0 { i=0; while i < XCHAL_NUM_IBREAK as i32 && (*current).thread.ptrace_bp[i as usize] != bp { i+=1; } i<<=1; } else { i=0; while i < XCHAL_NUM_DBREAK as i32 && (*current).thread.ptrace_wp[i as usize] != bp { i+=1; } i=(i<<1)|1; } force_sig_ptrace_errno_trap(i,(*bkpt).address as *mut _); }
#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
unsafe fn ptrace_hbp_create(tsk:*mut task_struct, typ:i32)->*mut perf_event { let mut attr:perf_event_attr=core::mem::zeroed(); ptrace_breakpoint_init(&mut attr); attr.bp_addr=0; attr.bp_len=1; attr.bp_type=typ; attr.disabled=1; register_user_hw_breakpoint(&mut attr,ptrace_hbptriggered,core::ptr::null_mut(),tsk) }
#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
unsafe fn ptrace_gethbpregs(child:*mut task_struct, addr:i64, datap:*mut i64)->i64 { let dbreak=addr&1!=0; let idx=(addr>>1) as usize; if (!dbreak&&idx>=XCHAL_NUM_IBREAK)|| (dbreak&&idx>=XCHAL_NUM_DBREAK){return -EINVAL as _;} let bp=if dbreak{(*child).thread.ptrace_wp[idx]}else{(*child).thread.ptrace_bp[idx]}; let mut d=[0u32;2]; if !bp.is_null(){d[0]=(*bp).attr.bp_addr; d[1]=if (*bp).attr.disabled!=0{0}else{(*bp).attr.bp_len}; if dbreak {if (*bp).attr.bp_type&HW_BREAKPOINT_R!=0{d[1]|=DBREAKC_LOAD_MASK;} if (*bp).attr.bp_type&HW_BREAKPOINT_W!=0{d[1]|=DBREAKC_STOR_MASK;}}} if copy_to_user(datap,d.as_ptr() as _,8)!=0{-EFAULT as _}else{0} }
#[cfg(CONFIG_HAVE_HW_BREAKPOINT)]
unsafe fn ptrace_sethbpregs(child:*mut task_struct, addr:i64, datap:*mut i64)->i64 { let dbreak=addr&1!=0; let idx=(addr>>1) as usize; if (!dbreak&&idx>=XCHAL_NUM_IBREAK)||(dbreak&&idx>=XCHAL_NUM_DBREAK){return -EINVAL as _;} let mut d=[0u32;2]; if copy_from_user(d.as_mut_ptr() as _,datap,8)!=0{return -EFAULT as _;} let mut typ=0; let mut bp=if dbreak{if d[1]&DBREAKC_LOAD_MASK!=0{typ|=HW_BREAKPOINT_R;}if d[1]&DBREAKC_STOR_MASK!=0{typ|=HW_BREAKPOINT_W;}(*child).thread.ptrace_wp[idx]}else{typ=HW_BREAKPOINT_X;(*child).thread.ptrace_bp[idx]}; if bp.is_null(){bp=ptrace_hbp_create(child,if typ!=0{typ}else{HW_BREAKPOINT_RW});if IS_ERR(bp){return PTR_ERR(bp) as _;}if dbreak{(*child).thread.ptrace_wp[idx]=bp;}else{(*child).thread.ptrace_bp[idx]=bp;}} let mut attr=(*bp).attr;attr.bp_addr=d[0];attr.bp_len=d[1]&!(DBREAKC_LOAD_MASK|DBREAKC_STOR_MASK);attr.bp_type=typ;attr.disabled=if attr.bp_len==0{1}else{0};modify_user_hw_breakpoint(bp,&mut attr) }

#[no_mangle] pub unsafe extern "C" fn arch_ptrace(child:*mut task_struct, request:i64, addr:usize, data:usize)->i32 { let p=data as *mut core::ffi::c_void; match request { PTRACE_PEEKUSR=>ptrace_peekusr(child,addr as _,p as _), PTRACE_POKEUSR=>ptrace_pokeusr(child,addr as _,data as _), PTRACE_GETREGS=>ptrace_getregs(child,p), PTRACE_SETREGS=>ptrace_setregs(child,p), PTRACE_GETXTREGS=>ptrace_getxregs(child,p), PTRACE_SETXTREGS=>ptrace_setxregs(child,p), _=>ptrace_request(child,request,addr,data) } }

#[no_mangle] pub unsafe extern "C" fn do_syscall_trace_enter(regs:*mut pt_regs)->i32 { if (*regs).syscall==NO_SYSCALL { (*regs).areg[2]=-ENOSYS as _; } if test_thread_flag(TIF_SYSCALL_TRACE)!=0 && ptrace_report_syscall_permit_entry(regs)==0 { (*regs).areg[2]=-ENOSYS as _; (*regs).syscall=NO_SYSCALL; return 0; } if (*regs).syscall==NO_SYSCALL || seccomp_permit_syscall()==0 { do_syscall_trace_leave(regs); return 0; } if test_thread_flag(TIF_SYSCALL_TRACEPOINT)!=0 { trace_sys_enter(regs,syscall_get_nr(current,regs)); } audit_syscall_entry((*regs).syscall,(*regs).areg[6],(*regs).areg[3],(*regs).areg[4],(*regs).areg[5]); 1 }
#[no_mangle] pub unsafe extern "C" fn do_syscall_trace_leave(regs:*mut pt_regs) { audit_syscall_exit(regs); if test_thread_flag(TIF_SYSCALL_TRACEPOINT)!=0 { trace_sys_exit(regs,regs_return_value(regs)); } let step=test_thread_flag(TIF_SINGLESTEP); if step!=0 || test_thread_flag(TIF_SYSCALL_TRACE)!=0 { ptrace_report_syscall_exit(regs,step); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
