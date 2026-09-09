// SPDX-License-Identifier: GPL-2.0-only
// Architecture-dependent process handling, translated from process_64.c.

#[repr(C)]
pub enum WhichSelector { FS, GS }

unsafe fn __rdgsbase_inactive() -> unsigned_long {
    let mut gsbase: unsigned_long = 0;
    lockdep_assert_irqs_disabled();
    if !cpu_feature_enabled(X86_FEATURE_FRED) && !cpu_feature_enabled(X86_FEATURE_XENPV) {
        native_swapgs(); gsbase = rdgsbase(); native_swapgs();
    } else { instrumentation_begin(); rdmsrq(MSR_KERNEL_GS_BASE, &mut gsbase); instrumentation_end(); }
    gsbase
}

unsafe fn __wrgsbase_inactive(gsbase: unsigned_long) {
    lockdep_assert_irqs_disabled();
    if !cpu_feature_enabled(X86_FEATURE_FRED) && !cpu_feature_enabled(X86_FEATURE_XENPV) {
        native_swapgs(); wrgsbase(gsbase); native_swapgs();
    } else { instrumentation_begin(); wrmsrq(MSR_KERNEL_GS_BASE, gsbase); instrumentation_end(); }
}

pub unsafe fn __show_regs(regs: *mut pt_regs, mode: show_regs_mode, log_lvl: *const c_char) {
    let (mut cr0, mut cr2, mut cr3, mut cr4, mut fs, mut gs, mut shadowgs) = (0,0,0,0,0,0,0);
    let (mut d0,mut d1,mut d2,mut d3,mut d6,mut d7)=(0,0,0,0,0,0);
    let (mut fsindex,mut gsindex,mut ds,mut es)=(0,0,0,0);
    show_iret_regs(regs, log_lvl);
    if (*regs).orig_ax != -1 { pr_cont(" ORIG_RAX: %016lx\n", (*regs).orig_ax); } else { pr_cont("\n"); }
    printk("%sRAX: %016lx RBX: %016lx RCX: %016lx\n",log_lvl,(*regs).ax,(*regs).bx,(*regs).cx);
    printk("%sRDX: %016lx RSI: %016lx RDI: %016lx\n",log_lvl,(*regs).dx,(*regs).si,(*regs).di);
    printk("%sRBP: %016lx R08: %016lx R09: %016lx\n",log_lvl,(*regs).bp,(*regs).r8,(*regs).r9);
    printk("%sR10: %016lx R11: %016lx R12: %016lx\n",log_lvl,(*regs).r10,(*regs).r11,(*regs).r12);
    printk("%sR13: %016lx R14: %016lx R15: %016lx\n",log_lvl,(*regs).r13,(*regs).r14,(*regs).r15);
    if mode == SHOW_REGS_SHORT { return; }
    if mode == SHOW_REGS_USER { rdmsrq(MSR_FS_BASE,&mut fs); rdmsrq(MSR_KERNEL_GS_BASE,&mut shadowgs); printk("%sFS:  %016lx GS:  %016lx\n",log_lvl,fs,shadowgs); return; }
    savesegment(ds,&mut ds); savesegment(es,&mut es); savesegment(fs,&mut fsindex); savesegment(gs,&mut gsindex);
    rdmsrq(MSR_FS_BASE,&mut fs); rdmsrq(MSR_GS_BASE,&mut gs); rdmsrq(MSR_KERNEL_GS_BASE,&mut shadowgs);
    cr0=read_cr0(); cr2=read_cr2(); cr3=__read_cr3(); cr4=__read_cr4();
    printk("%sFS:  %016lx(%04x) GS:%016lx(%04x) knlGS:%016lx\n",log_lvl,fs,fsindex,gs,gsindex,shadowgs);
    printk("%sCS:  %04x DS: %04x ES: %04x CR0: %016lx\n",log_lvl,(*regs).cs,ds,es,cr0);
    printk("%sCR2: %016lx CR3: %016lx CR4: %016lx\n",log_lvl,cr2,cr3,cr4);
    get_debugreg(&mut d0,0); get_debugreg(&mut d1,1); get_debugreg(&mut d2,2); get_debugreg(&mut d3,3); get_debugreg(&mut d6,6); get_debugreg(&mut d7,7);
    if !(d0==0&&d1==0&&d2==0&&d3==0&&d6==DR6_RESERVED&&d7==DR7_FIXED_1) { printk("%sDR0: %016lx DR1: %016lx DR2: %016lx\n",log_lvl,d0,d1,d2); printk("%sDR3: %016lx DR6: %016lx DR7: %016lx\n",log_lvl,d3,d6,d7); }
    if cr4 & X86_CR4_PKE != 0 { printk("%sPKRU: %08x\n",log_lvl,read_pkru()); }
}

pub unsafe fn release_thread(dead_task: *mut task_struct) { WARN_ON((*dead_task).mm); }

unsafe fn save_base_legacy(prev_p:*mut task_struct, selector:u16, which:WhichSelector) {
    if selector != 0 { if which==WhichSelector::FS { (*prev_p).thread.fsbase=0; } else { (*prev_p).thread.gsbase=0; } }
}
unsafe fn save_fsgs(task:*mut task_struct) { savesegment(fs,&mut (*task).thread.fsindex); savesegment(gs,&mut (*task).thread.gsindex); if cpu_feature_enabled(X86_FEATURE_FSGSBASE) { (*task).thread.fsbase=rdfsbase(); (*task).thread.gsbase=__rdgsbase_inactive(); } else { save_base_legacy(task,(*task).thread.fsindex,WhichSelector::FS); save_base_legacy(task,(*task).thread.gsindex,WhichSelector::GS); } }
pub unsafe fn current_save_fsgs() { let mut flags=0; local_irq_save(&mut flags); save_fsgs(current); local_irq_restore(flags); }
unsafe fn loadseg(which:WhichSelector, sel:u16) { if which==WhichSelector::FS { loadsegment(fs,sel); } else { load_gs_index(sel); } }
unsafe fn load_seg_legacy(prev_index:u16,prev_base:unsigned_long,next_index:u16,next_base:unsigned_long,which:WhichSelector) { if next_index<=3 { if next_base==0 { if static_cpu_has_bug(X86_BUG_NULL_SEG) { loadseg(which,__USER_DS); loadseg(which,next_index); } else if (prev_index as unsigned_long | next_index as unsigned_long | prev_base)!=0 { loadseg(which,next_index); } } else { if prev_index!=next_index { loadseg(which,next_index); } wrmsrq(if which==WhichSelector::FS {MSR_FS_BASE} else {MSR_KERNEL_GS_BASE},next_base); } } else { loadseg(which,next_index); } }
unsafe fn x86_pkru_load(prev:*mut thread_struct,next:*mut thread_struct) { if !cpu_feature_enabled(X86_FEATURE_OSPKE){return;} (*prev).pkru=rdpkru(); if (*prev).pkru!=(*next).pkru {wrpkru((*next).pkru);} }
unsafe fn x86_fsgsbase_load(prev:*mut thread_struct,next:*mut thread_struct) { if cpu_feature_enabled(X86_FEATURE_FSGSBASE) { if (*prev).fsindex!=0||(*next).fsindex!=0 {loadseg(WhichSelector::FS,(*next).fsindex);} if (*prev).gsindex!=0||(*next).gsindex!=0 {loadseg(WhichSelector::GS,(*next).gsindex);} wrfsbase((*next).fsbase); __wrgsbase_inactive((*next).gsbase); } else {load_seg_legacy((*prev).fsindex,(*prev).fsbase,(*next).fsindex,(*next).fsbase,WhichSelector::FS);load_seg_legacy((*prev).gsindex,(*prev).gsbase,(*next).gsindex,(*next).gsbase,WhichSelector::GS);} }

pub unsafe fn x86_gsbase_read_cpu_inactive()->unsigned_long { let mut x=0; if boot_cpu_has(X86_FEATURE_FSGSBASE){let mut f=0;local_irq_save(&mut f);x=__rdgsbase_inactive();local_irq_restore(f);}else{rdmsrq(MSR_KERNEL_GS_BASE,&mut x)} x }
pub unsafe fn x86_gsbase_write_cpu_inactive(x:unsigned_long){if boot_cpu_has(X86_FEATURE_FSGSBASE){let mut f=0;local_irq_save(&mut f);__wrgsbase_inactive(x);local_irq_restore(f);}else{wrmsrq(MSR_KERNEL_GS_BASE,x)}}
pub unsafe fn x86_fsbase_read_task(t:*mut task_struct)->unsigned_long{if t==current{x86_fsbase_read_cpu()}else if boot_cpu_has(X86_FEATURE_FSGSBASE)||(*t).thread.fsindex==0{(*t).thread.fsbase}else{x86_fsgsbase_read_task(t,(*t).thread.fsindex)}}
pub unsafe fn x86_gsbase_read_task(t:*mut task_struct)->unsigned_long{if t==current{x86_gsbase_read_cpu_inactive()}else if boot_cpu_has(X86_FEATURE_FSGSBASE)||(*t).thread.gsindex==0{(*t).thread.gsbase}else{x86_fsgsbase_read_task(t,(*t).thread.gsindex)}}
pub unsafe fn x86_fsbase_write_task(t:*mut task_struct,x:unsigned_long){WARN_ON_ONCE(t==current);(*t).thread.fsbase=x} pub unsafe fn x86_gsbase_write_task(t:*mut task_struct,x:unsigned_long){WARN_ON_ONCE(t==current);(*t).thread.gsbase=x}

unsafe fn start_thread_common(r:*mut pt_regs,ip:unsigned_long,sp:unsigned_long,cs:u16,ss:u16,ds:u16){
    WARN_ON_ONCE(r!=current_pt_regs());
    if cpu_feature_enabled(X86_BUG_NULL_SEG){loadsegment(fs,__USER_DS);load_gs_index(__USER_DS);}
    reset_thread_features(); loadsegment(fs,0);loadsegment(es,ds);loadsegment(ds,ds);load_gs_index(0);
    (*r).ip=ip;(*r).sp=sp;(*r).csx=cs;(*r).ssx=ss;
    if cpu_feature_enabled(X86_FEATURE_FRED){(*r).fred_ss.swevent=true;(*r).fred_ss.nmi=true;}
    (*r).flags=X86_EFLAGS_IF|X86_EFLAGS_FIXED;
}
pub unsafe fn start_thread(r:*mut pt_regs,ip:unsigned_long,sp:unsigned_long){start_thread_common(r,ip,sp,__USER_CS,__USER_DS,0)}
pub unsafe fn compat_start_thread(r:*mut pt_regs,ip:u32,sp:u32,x32:bool){start_thread_common(r,ip as unsigned_long,sp as unsigned_long,if x32{__USER_CS}else{__USER32_CS},__USER_DS,__USER_DS)}
pub unsafe fn set_personality_64bit(){clear_thread_flag(TIF_ADDR32);(*task_pt_regs(current)).orig_ax=__NR_execve;(*current_thread_info()).status &= !TS_COMPAT;if !(*current).mm.is_null(){__set_bit(MM_CONTEXT_HAS_VSYSCALL,&mut (*(*current).mm).context.flags);}(*current).personality &= !READ_IMPLIES_EXEC;}
unsafe fn __set_personality_x32(){(*task_pt_regs(current)).orig_ax=__NR_x32_execve|__X32_SYSCALL_BIT;(*current_thread_info()).status &= !TS_COMPAT;}
unsafe fn __set_personality_ia32(){(*current).personality|=force_personality32;(*task_pt_regs(current)).orig_ax=__NR_ia32_execve;(*current_thread_info()).status|=TS_COMPAT;}
pub unsafe fn set_personality_ia32(x32:bool){set_thread_flag(TIF_ADDR32);if x32{__set_personality_x32()}else{__set_personality_ia32()}}
pub unsafe fn __switch_to(prev_p:*mut task_struct,next_p:*mut task_struct)->*mut task_struct{let prev=&mut (*prev_p).thread;let next=&mut (*next_p).thread;let cpu=smp_processor_id();switch_fpu(prev_p,cpu);save_fsgs(prev_p);load_TLS(next_p,cpu);arch_end_context_switch(next_p);savesegment(es,&mut prev.es);if next.es|prev.es!=0{loadsegment(es,next.es)}savesegment(ds,&mut prev.ds);if next.ds|prev.ds!=0{loadsegment(ds,next.ds)}x86_fsgsbase_load(prev,next);x86_pkru_load(prev,next);raw_cpu_write(current_task,next_p);raw_cpu_write(cpu_current_top_of_stack,task_top_of_stack(next_p));update_task_stack(next_p);switch_to_extra(prev_p,next_p);resctrl_arch_sched_in(next_p);if cpu_feature_enabled(X86_FEATURE_AMD_WORKLOAD_CLASS){wrmsrq(MSR_AMD_WORKLOAD_HRST,1)}prev_p}
pub unsafe fn do_arch_prctl_64(task:*mut task_struct,option:c_int,arg2:unsigned_long)->c_long{match option{ARCH_SET_GS=>{if arg2>=TASK_SIZE_MAX{return -EPERM}preempt_disable();if task==current{loadseg(WhichSelector::GS,0);x86_gsbase_write_cpu_inactive(arg2);(*task).thread.gsbase=arg2}else{(*task).thread.gsindex=0;x86_gsbase_write_task(task,arg2)}preempt_enable();0},ARCH_SET_FS=>{if arg2>=TASK_SIZE_MAX{return -EPERM}preempt_disable();if task==current{loadseg(WhichSelector::FS,0);x86_fsbase_write_cpu(arg2);(*task).thread.fsbase=arg2}else{(*task).thread.fsindex=0;x86_fsbase_write_task(task,arg2)}preempt_enable();0},ARCH_GET_FS=>put_user(x86_fsbase_read_task(task),arg2 as *mut unsigned_long),ARCH_GET_GS=>put_user(x86_gsbase_read_task(task),arg2 as *mut unsigned_long),ARCH_SHSTK_ENABLE|ARCH_SHSTK_DISABLE|ARCH_SHSTK_LOCK|ARCH_SHSTK_UNLOCK|ARCH_SHSTK_STATUS=>shstk_prctl(task,option,arg2),_=>-EINVAL}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
