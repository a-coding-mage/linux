// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of arch/sparc64/kernel/process.c. */

// Dependencies supplied by the surrounding kernel translation unit are intentionally external.

pub unsafe fn arch_cpu_idle() {
    if tlb_type != hypervisor {
        touch_nmi_watchdog();
    } else {
        let mut pstate: usize;
        raw_local_irq_enable();
        // rdpr/andn/wrpr pstate, clearing PSTATE_IE
        asm!("rdpr %pstate, {0}; andn {0}, {1}, {0}; wrpr {0}, %g0, %pstate", out(reg) pstate, const PSTATE_IE);
        if !need_resched() && !cpu_is_offline(smp_processor_id()) {
            sun4v_cpu_yield();
            scheduler_poke();
        }
        // Re-enable interrupts.
        asm!("rdpr %pstate, {0}; or {0}, {1}, {0}; wrpr {0}, %g0, %pstate", out(reg) pstate, const PSTATE_IE);
        raw_local_irq_disable();
    }
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn arch_cpu_idle_dead() -> ! {
    sched_preempt_enable_no_resched();
    cpu_play_dead();
}

#[cfg(CONFIG_COMPAT)]
unsafe fn show_regwindow32(regs: *mut pt_regs) {
    let mut r_w: reg_window32 = core::mem::zeroed();
    asm!("flushw");
    let rw = compat_ptr((*regs).u_regs[14] as u32);
    if copy_from_user(&mut r_w as *mut _ as *mut _, rw, core::mem::size_of::<reg_window32>()) != 0 { return; }
    printk!("l0: {:08x} l1: {:08x} l2: {:08x} l3: {:08x} l4: {:08x} l5: {:08x} l6: {:08x} l7: {:08x}\n", r_w.locals[0],r_w.locals[1],r_w.locals[2],r_w.locals[3],r_w.locals[4],r_w.locals[5],r_w.locals[6],r_w.locals[7]);
    printk!("i0: {:08x} i1: {:08x} i2: {:08x} i3: {:08x} i4: {:08x} i5: {:08x} i6: {:08x} i7: {:08x}\n", r_w.ins[0],r_w.ins[1],r_w.ins[2],r_w.ins[3],r_w.ins[4],r_w.ins[5],r_w.ins[6],r_w.ins[7]);
}

#[cfg(not(CONFIG_COMPAT))]
unsafe fn show_regwindow32(_regs: *mut pt_regs) {}

unsafe fn show_regwindow(regs: *mut pt_regs) {
    let mut r_w: reg_window = core::mem::zeroed();
    let mut rwk: *mut reg_window;
    if (*regs).tstate & TSTATE_PRIV != 0 || !test_thread_flag(TIF_32BIT) {
        asm!("flushw");
        let rw = ((*regs).u_regs[14] + STACK_BIAS) as *mut reg_window;
        rwk = rw;
        if (*regs).tstate & TSTATE_PRIV == 0 {
            if copy_from_user(&mut r_w as *mut _ as *mut _, rw, core::mem::size_of::<reg_window>()) != 0 { return; }
            rwk = &mut r_w;
        }
    } else { show_regwindow32(regs); return; }
    printk!("l0: {:016x} l1: {:016x} l2: {:016x} l3: {:016x}\n",(*rwk).locals[0],(*rwk).locals[1],(*rwk).locals[2],(*rwk).locals[3]);
    printk!("l4: {:016x} l5: {:016x} l6: {:016x} l7: {:016x}\n",(*rwk).locals[4],(*rwk).locals[5],(*rwk).locals[6],(*rwk).locals[7]);
    printk!("i0: {:016x} i1: {:016x} i2: {:016x} i3: {:016x}\n",(*rwk).ins[0],(*rwk).ins[1],(*rwk).ins[2],(*rwk).ins[3]);
    printk!("i4: {:016x} i5: {:016x} i6: {:016x} i7: {:016x}\n",(*rwk).ins[4],(*rwk).ins[5],(*rwk).ins[6],(*rwk).ins[7]);
    if (*regs).tstate & TSTATE_PRIV != 0 { printk!("I7: <{:p}>\n", (*rwk).ins[7] as *const _); }
}

pub unsafe fn show_regs(regs: *mut pt_regs) {
    show_regs_print_info(KERN_DEFAULT);
    printk!("TSTATE: {:016x} TPC: {:016x} TNPC: {:016x} Y: {:08x}    {}\n",(*regs).tstate,(*regs).tpc,(*regs).tnpc,(*regs).y,print_tainted());
    printk!("TPC: <{:p}>\n",(*regs).tpc as *const _);
    for (name, start) in [("g",0usize),("o",8usize)] { printk!("{}0: {:016x} {}1: {:016x} {}2: {:016x} {}3: {:016x}\n",name,(*regs).u_regs[start],name,(*regs).u_regs[start+1],name,(*regs).u_regs[start+2],name,(*regs).u_regs[start+3]); }
    printk!("o4: {:016x} o5: {:016x} sp: {:016x} ret_pc: {:016x}\n",(*regs).u_regs[12],(*regs).u_regs[13],(*regs).u_regs[14],(*regs).u_regs[15]);
    printk!("RPC: <{:p}>\n",(*regs).u_regs[15] as *const _);
    show_regwindow(regs); show_stack(current, (*regs).u_regs[UREG_FP] as *mut _, KERN_DEFAULT);
}

// The remaining architecture routines retain the original control flow and ABI-facing operations.
pub unsafe fn exit_thread(tsk: *mut task_struct) { let t = task_thread_info(tsk); if (*t).utraps != core::ptr::null_mut() { if (*(*t).utraps) < 2 { kfree((*t).utraps as *mut _); } else { *(*t).utraps -= 1; } } }

pub unsafe fn flush_thread() { let t=current_thread_info(); let mm=(*t).task.mm; if !mm.is_null(){tsb_context_switch(mm);} set_thread_wsaved(0); (*t).fpsaved[0]=0; }

unsafe fn clone_stackframe(mut csp: usize, mut psp: usize) -> usize { let stack_64bit=test_thread_64bit_stack(psp); let fp; if stack_64bit {csp+=STACK_BIAS;psp+=STACK_BIAS; fp=(*(psp as *mut reg_window)).ins[6]+STACK_BIAS; } else {fp=(*(psp as *mut reg_window32)).ins[6] as usize;} let fp=if test_thread_flag(TIF_32BIT){fp&0xffff_ffff}else{fp}; csp&=!15; let distance=fp.wrapping_sub(psp); let rval=csp.wrapping_sub(distance); if raw_copy_in_user(rval as *mut _,psp as *mut _,distance)!=0 {0} else {rval} }

unsafe fn shift_window_buffer(first_win:i32,last_win:i32,t:*mut thread_info){for i in first_win..last_win{(*t).rwbuf_stkptrs[i as usize]=(*t).rwbuf_stkptrs[(i+1) as usize];(*t).reg_window[i as usize]=(*t).reg_window[(i+1) as usize];}}

pub unsafe fn synchronize_user_stack(){let t=current_thread_info();flush_user_windows();let mut window=get_thread_wsaved();if window!=0{window-=1;loop{let rwin=&(*t).reg_window[window as usize] as *const _;let mut sp=(*t).rwbuf_stkptrs[window as usize];let size=if test_thread_64bit_stack(sp){sp+=STACK_BIAS;core::mem::size_of::<reg_window>()}else{core::mem::size_of::<reg_window32>()};if copy_to_user(sp as *mut _,rwin,size)==0{shift_window_buffer(window,get_thread_wsaved()-1,t);set_thread_wsaved(get_thread_wsaved()-1);}if window==0{break}window-=1;}}}

unsafe fn stack_unaligned(sp:usize){force_sig_fault(SIGBUS,BUS_ADRALN,sp as *mut _);}
pub unsafe fn fault_in_user_windows(regs:*mut pt_regs){let t=current_thread_info();flush_user_windows();let mut window=get_thread_wsaved();if window!=0{window-=1;loop{let rwin=&(*t).reg_window[window as usize] as *const _;let mut sp=(*t).rwbuf_stkptrs[window as usize];let size=if test_thread_64bit_stack(sp){sp+=STACK_BIAS;core::mem::size_of::<reg_window>()}else{core::mem::size_of::<reg_window32>()};if sp&7!=0{stack_unaligned(sp);}if copy_to_user(sp as *mut _,rwin,size)!=0{force_sig(SIGSEGV);set_thread_wsaved(window+1);return;}if window==0{break}window-=1;}}set_thread_wsaved(0);}

pub unsafe fn copy_thread(p:*mut task_struct,args:*const kernel_clone_args)->i32 {
    let clone_flags=(*args).flags; let tls=(*args).tls; let t=task_thread_info(p); let regs=current_pt_regs();
    let mut sp=if (*args).stack!=0{(*args).stack}else{(*regs).u_regs[UREG_FP]};
    let child_stack_sz=STACKFRAME_SZ+TRACEREG_SZ; let child_trap_frame=(task_stack_page(p)+(THREAD_SIZE-child_stack_sz)) as *mut u8;
    (*t).new_child=1;(*t).ksp=child_trap_frame as usize-STACK_BIAS;(*t).kregs=child_trap_frame.add(core::mem::size_of::<sparc_stackf>()) as *mut pt_regs;(*t).fpsaved[0]=0;
    if (*args).fn_ != 0 { core::ptr::write_bytes(child_trap_frame,0,child_stack_sz); *__thread_flag_byte_ptr(t).add(TI_FLAG_BYTE_CWP)=((*current_pt_regs()).tstate+1)&TSTATE_CWP;(*(*t).kregs).u_regs[UREG_G1]=(*args).fn_ as usize;(*(*t).kregs).u_regs[UREG_G2]=(*args).fn_arg;return 0; }
    core::ptr::copy_nonoverlapping((regs as *mut sparc_stackf).offset(-1) as *const u8,child_trap_frame,child_stack_sz);
    if (*t).flags&_TIF_32BIT!=0{sp&=0xffff_ffff;(*regs).u_regs[UREG_FP]&=0xffff_ffff;}(*(*t).kregs).u_regs[UREG_FP]=sp;*__thread_flag_byte_ptr(t).add(TI_FLAG_BYTE_CWP)=((*regs).tstate+1)&TSTATE_CWP;
    if sp!=(*regs).u_regs[UREG_FP]{let csp=clone_stackframe(sp,(*regs).u_regs[UREG_FP]);if csp==0{return -EFAULT;}(*(*t).kregs).u_regs[UREG_FP]=csp;}if !(*t).utraps.is_null(){*(*t).utraps+=1;}
    if (*regs).u_regs[UREG_G1]==__NR_clone3{(*(*t).kregs).u_regs[UREG_I0]=0;(*(*t).kregs).u_regs[UREG_G1]=0;}else{(*(*t).kregs).u_regs[UREG_I0]=(*current).pid;(*(*t).kregs).u_regs[UREG_I1]=1;(*regs).u_regs[UREG_I1]=0;}if clone_flags&CLONE_SETTLS!=0{(*(*t).kregs).u_regs[UREG_G7]=tls;}0
}

pub unsafe fn arch_dup_task_struct(dst:*mut task_struct,src:*mut task_struct)->i32 { if adi_capable(){let tmp=read_mcdper();if tmp!=0{set_thread_flag(TIF_MCDPER)}else{clear_thread_flag(TIF_MCDPER)}};*dst=*src;0 }

pub unsafe fn __get_wchan(task:*mut task_struct)->usize {let tp=task_thread_info(task);let mut fp=(*tp).ksp+STACK_BIAS;let mut count=0;loop{if !kstack_valid(tp,fp){break}let rw=fp as *mut reg_window;let pc=(*rw).ins[7];if !in_sched_functions(pc){return pc}fp=(*rw).ins[6]+STACK_BIAS;count+=1;if count>=16{break}}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
